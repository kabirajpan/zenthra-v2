use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

// ─────────────────────────────────────────────────────────────────────────────
// Uniform that drives every blur pass
// ─────────────────────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct BlurUniforms {
    pub texel_size: [f32; 2], // 1/src_w, 1/src_h
    pub offset:     f32,      // Kawase sample offset (radius-scaled)
    pub direction:  f32,      // box blur: 0.0=horizontal 1.0=vertical
}

// ─────────────────────────────────────────────────────────────────────────────
// A single pipeline object that can run all three blur entry-points:
//   downsample / upsample / box
// The caller selects the entry-point via `pass_kind`.
// ─────────────────────────────────────────────────────────────────────────────
pub enum BlurPassKind {
    Downsample,
    Upsample,
    Box { horizontal: bool },
}

pub struct BlurPipeline {
    pub downsample_pipeline: wgpu::RenderPipeline,
    pub upsample_pipeline:   wgpu::RenderPipeline,
    pub box_pipeline:        wgpu::RenderPipeline,
    pub uniform_bgl:         wgpu::BindGroupLayout,
    pub texture_bgl:         wgpu::BindGroupLayout,
    pub sampler:             wgpu::Sampler,
}

impl BlurPipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/blur.wgsl"));

        // ── Bind group layouts ────────────────────────────────────────────────
        let uniform_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Blur Uniform BGL"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let texture_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Blur Texture BGL"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Blur Pipeline Layout"),
            bind_group_layouts: &[Some(&uniform_bgl), Some(&texture_bgl)],
            immediate_size: 0,
        });

        // Shared blend: opaque (blur output is always opaque)
        let target = wgpu::ColorTargetState {
            format,
            blend: None, // fully replace
            write_mask: wgpu::ColorWrites::ALL,
        };

        let mk_pipeline = |label: &str, entry: &str| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some(label),
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some(entry),
                    targets: &[Some(target.clone())],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            })
        };

        let downsample_pipeline = mk_pipeline("Blur Downsample Pipeline", "fs_downsample");
        let upsample_pipeline   = mk_pipeline("Blur Upsample Pipeline",   "fs_upsample");
        let box_pipeline        = mk_pipeline("Blur Box Pipeline",         "fs_box");

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Blur Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            downsample_pipeline,
            upsample_pipeline,
            box_pipeline,
            uniform_bgl,
            texture_bgl,
            sampler,
        }
    }

    /// Run one blur pass.
    ///
    /// * `src_view`  — texture view to read from
    /// * `dst_view`  — texture view to write into (must NOT be the same texture)
    /// * `src_w/h`   — pixel dimensions of the SOURCE texture
    /// * `offset`    — Kawase offset (typically `(pass + 0.5) * radius / num_passes`)
    /// * `kind`      — which pass to run
    pub fn run_pass(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        src_view: &wgpu::TextureView,
        dst_view: &wgpu::TextureView,
        src_w: u32,
        src_h: u32,
        offset: f32,
        kind: BlurPassKind,
    ) {
        let direction = match &kind {
            BlurPassKind::Box { horizontal } => if *horizontal { 0.0 } else { 1.0 },
            _ => 0.0,
        };

        let uniforms = BlurUniforms {
            texel_size: [1.0 / src_w as f32, 1.0 / src_h as f32],
            offset,
            direction,
        };

        let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Blur Pass Uniform"),
            contents: bytemuck::bytes_of(&uniforms),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let uniform_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Blur Pass Uniform BG"),
            layout: &self.uniform_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buf.as_entire_binding(),
            }],
        });

        let texture_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Blur Pass Texture BG"),
            layout: &self.texture_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(src_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });

        let pipeline = match kind {
            BlurPassKind::Downsample   => &self.downsample_pipeline,
            BlurPassKind::Upsample     => &self.upsample_pipeline,
            BlurPassKind::Box { .. }   => &self.box_pipeline,
        };

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Blur Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: dst_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, &uniform_bg, &[]);
        pass.set_bind_group(1, &texture_bg, &[]);
        pass.draw(0..3, 0..1);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Scratch textures used during a blur sequence.
// Two ping-pong textures at full resolution + four downsampled mips.
// Allocated once and re-created only on window resize.
// ─────────────────────────────────────────────────────────────────────────────
pub struct BlurScratch {
    pub width:    u32,
    pub height:   u32,
    pub format:   wgpu::TextureFormat,
    // Full-resolution ping-pong pair used to snapshot scene + composite
    pub full_a:   wgpu::Texture,
    pub full_a_v: wgpu::TextureView,
    pub full_b:   wgpu::Texture,
    pub full_b_v: wgpu::TextureView,
    // 4 mip levels for the Kawase pyramid (half, quarter, eighth, sixteenth)
    pub mips:     [wgpu::Texture; 4],
    pub mips_v:   [wgpu::TextureView; 4],
    pub mip_w:    [u32; 4],
    pub mip_h:    [u32; 4],
}

impl BlurScratch {
    pub fn new(device: &wgpu::Device, width: u32, height: u32, format: wgpu::TextureFormat) -> Self {
        let mk = |w: u32, h: u32, label: &str| {
            device.create_texture(&wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d { width: w, height: h, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING
                    | wgpu::TextureUsages::COPY_SRC
                    | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            })
        };

        let full_a = mk(width, height, "Blur Full A");
        let full_a_v = full_a.create_view(&wgpu::TextureViewDescriptor::default());
        let full_b = mk(width, height, "Blur Full B");
        let full_b_v = full_b.create_view(&wgpu::TextureViewDescriptor::default());

        let mut mip_w = [0u32; 4];
        let mut mip_h = [0u32; 4];
        let mut mips_raw: Vec<wgpu::Texture> = Vec::with_capacity(4);
        for i in 0..4 {
            let scale = 2u32.pow(i as u32 + 1);
            let w = (width / scale).max(1);
            let h = (height / scale).max(1);
            mip_w[i] = w;
            mip_h[i] = h;
            mips_raw.push(mk(w, h, &format!("Blur Mip {}", i)));
        }

        // SAFETY: exactly 4 elements
        let mips: [wgpu::Texture; 4] = mips_raw.try_into().unwrap();
        let mips_v: [wgpu::TextureView; 4] = std::array::from_fn(|i| {
            mips[i].create_view(&wgpu::TextureViewDescriptor::default())
        });

        Self { width, height, format, full_a, full_a_v, full_b, full_b_v, mips, mips_v, mip_w, mip_h }
    }

    /// Returns true if the scratch buffers need to be recreated.
    pub fn needs_resize(&self, width: u32, height: u32) -> bool {
        self.width != width || self.height != height
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// High-level helper: run the full Dual Kawase blur on a region.
//
// Reads from `scene_view` (the offscreen render target snapshot),
// writes the blurred result into `out_view`.
// ─────────────────────────────────────────────────────────────────────────────
pub fn run_kawase_blur(
    pipeline: &BlurPipeline,
    scratch: &BlurScratch,
    device: &wgpu::Device,
    encoder: &mut wgpu::CommandEncoder,
    scene_view: &wgpu::TextureView, // full-res source
    out_view:   &wgpu::TextureView, // full-res output
    radius:     f32,                // blur radius in pixels (logical)
) {
    if radius <= 0.0 {
        // Just copy scene_view to out_view if radius is 0
        pipeline.run_pass(
            device, encoder,
            scene_view, out_view,
            scratch.width, scratch.height,
            0.0, BlurPassKind::Upsample,
        );
        return;
    }

    // Dynamically choose pass count based on radius size.
    // Small blurs require fewer downsamples to avoid looking instantly fully blurred.
    if radius < 5.0 {
        // Full resolution single-pass blur (0 downsampling). Very crisp and subtle!
        pipeline.run_pass(
            device, encoder,
            scene_view, out_view,
            scratch.width, scratch.height,
            radius, BlurPassKind::Downsample,
        );
        return;
    }

    let num_passes = if radius < 12.0 {
        1usize
    } else if radius < 25.0 {
        2usize
    } else if radius < 40.0 {
        3usize
    } else {
        4usize
    };

    // ── 1. Downsample chain ───────────────────────────────────────────────
    // Pass 0: scene → mip[0]  (half size)
    {
        let offset = 0.5 * radius / num_passes as f32;
        pipeline.run_pass(
            device, encoder,
            scene_view, &scratch.mips_v[0],
            scratch.width, scratch.height,
            offset, BlurPassKind::Downsample,
        );
    }
    // Passes 1 to num_passes-1: mip[i-1] → mip[i]
    for i in 1..num_passes {
        let offset = (i as f32 + 0.5) * radius / num_passes as f32;
        pipeline.run_pass(
            device, encoder,
            &scratch.mips_v[i - 1], &scratch.mips_v[i],
            scratch.mip_w[i - 1], scratch.mip_h[i - 1],
            offset, BlurPassKind::Downsample,
        );
    }

    // ── 2. Upsample chain ─────────────────────────────────────────────────
    // Passes num_passes-2 down to 0: mip[i+1] → mip[i]
    for i in (0..num_passes - 1).rev() {
        let offset = (i as f32 + 0.5) * radius / num_passes as f32;
        pipeline.run_pass(
            device, encoder,
            &scratch.mips_v[i + 1], &scratch.mips_v[i],
            scratch.mip_w[i + 1], scratch.mip_h[i + 1],
            offset, BlurPassKind::Upsample,
        );
    }

    // ── 3. Final upsample mip[0] → out_view (full res) ───────────────────
    {
        let offset = 0.5 * radius / num_passes as f32;
        pipeline.run_pass(
            device, encoder,
            &scratch.mips_v[0], out_view,
            scratch.mip_w[0], scratch.mip_h[0],
            offset, BlurPassKind::Upsample,
        );
    }
}

pub fn run_box_blur(
    pipeline: &BlurPipeline,
    scratch: &BlurScratch,
    device: &wgpu::Device,
    encoder: &mut wgpu::CommandEncoder,
    scene_view: &wgpu::TextureView, // full-res source
    out_view:   &wgpu::TextureView, // full-res output
    radius:     f32,                // blur radius in pixels (logical)
) {
    // Pass 1: Horizontal box blur (scene -> mip[0])
    pipeline.run_pass(
        device, encoder,
        scene_view, &scratch.mips_v[0],
        scratch.width, scratch.height,
        radius, BlurPassKind::Box { horizontal: true },
    );
    // Pass 2: Vertical box blur (mip[0] -> out_view)
    pipeline.run_pass(
        device, encoder,
        &scratch.mips_v[0], out_view,
        scratch.mip_w[0], scratch.mip_h[0],
        radius, BlurPassKind::Box { horizontal: false },
    );
}
