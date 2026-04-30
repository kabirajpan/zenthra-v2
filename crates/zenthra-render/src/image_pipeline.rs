use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ImageInstance {
    pub pos: [f32; 2],
    pub size: [f32; 2],
    pub radius: [f32; 4],
    pub border_width: f32,
    pub border_color: [f32; 4],
    pub shadow_color: [f32; 4],
    pub shadow_offset: [f32; 2],
    pub shadow_blur: f32,
    pub clip_rect: [f32; 4],
    pub grayscale: f32,
    pub brightness: f32,
    pub opacity: f32,
    pub uv_rect: [f32; 4],
    pub bg_color: [f32; 4],
    pub rotation: [f32; 3], // (x, y, z) in radians
    pub flip: [f32; 2],     // (horizontal, vertical) - 1.0 for normal, -1.0 for flipped
}

impl Default for ImageInstance {
    fn default() -> Self {
        Self {
            pos: [0.0; 2],
            size: [100.0, 100.0],
            radius: [0.0; 4],
            border_width: 0.0,
            border_color: [0.0; 4],
            shadow_color: [0.0; 4],
            shadow_offset: [0.0; 2],
            shadow_blur: 0.0,
            clip_rect: [0.0, 0.0, 9999.0, 9999.0],
            grayscale: 0.0,
            brightness: 1.0,
            opacity: 1.0,
            uv_rect: [0.0, 0.0, 1.0, 1.0],
            bg_color: [0.0; 4],
            rotation: [0.0; 3],
            flip: [1.0, 1.0],
        }
    }
}

impl ImageInstance {
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<ImageInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 },
                wgpu::VertexAttribute { offset: 8, shader_location: 1, format: wgpu::VertexFormat::Float32x2 },
                wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
                wgpu::VertexAttribute { offset: 32, shader_location: 3, format: wgpu::VertexFormat::Float32 },
                wgpu::VertexAttribute { offset: 36, shader_location: 4, format: wgpu::VertexFormat::Float32x4 },
                wgpu::VertexAttribute { offset: 52, shader_location: 5, format: wgpu::VertexFormat::Float32x4 },
                wgpu::VertexAttribute { offset: 68, shader_location: 6, format: wgpu::VertexFormat::Float32x2 },
                wgpu::VertexAttribute { offset: 76, shader_location: 7, format: wgpu::VertexFormat::Float32 },
                wgpu::VertexAttribute { offset: 80, shader_location: 8, format: wgpu::VertexFormat::Float32x4 },
                wgpu::VertexAttribute { offset: 96, shader_location: 9, format: wgpu::VertexFormat::Float32 },
                wgpu::VertexAttribute { offset: 100, shader_location: 10, format: wgpu::VertexFormat::Float32 },
                wgpu::VertexAttribute { offset: 104, shader_location: 11, format: wgpu::VertexFormat::Float32 },
                wgpu::VertexAttribute { offset: 108, shader_location: 12, format: wgpu::VertexFormat::Float32x4 },
                wgpu::VertexAttribute { offset: 124, shader_location: 13, format: wgpu::VertexFormat::Float32x4 },
                wgpu::VertexAttribute { offset: 140, shader_location: 14, format: wgpu::VertexFormat::Float32x3 },
                wgpu::VertexAttribute { offset: 152, shader_location: 15, format: wgpu::VertexFormat::Float32x2 },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct ScreenUniforms {
    screen_size: [f32; 2],
    _padding: [f32; 2],
}

pub struct ImagePipeline {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bg: wgpu::BindGroup,
    pub texture_bgl: wgpu::BindGroupLayout,
}

impl ImagePipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/image.wgsl"));

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Image Uniform Buffer"),
            size: std::mem::size_of::<ScreenUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Image Uniform BGL"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let uniform_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Image Uniform BG"),
            layout: &uniform_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let texture_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Image Texture BGL"),
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
            label: Some("Image Pipeline Layout"),
            bind_group_layouts: &[Some(&uniform_bgl), Some(&texture_bgl)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Image Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[ImageInstance::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        Self {
            pipeline,
            uniform_buffer,
            uniform_bg,
            texture_bgl,
        }
    }

    pub fn prepare(
        &self,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
    ) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::bytes_of(&ScreenUniforms {
                screen_size: [width as f32, height as f32],
                _padding: [0.0; 2],
            }),
        );
    }

    pub fn draw<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        texture_bind_group: &'a wgpu::BindGroup,
        instance_buffer: &'a wgpu::Buffer,
        instance_count: u32,
    ) {
        if instance_count == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.uniform_bg, &[]);
        pass.set_bind_group(1, texture_bind_group, &[]);
        pass.set_vertex_buffer(0, instance_buffer.slice(..));
        pass.draw(0..6, 0..instance_count);
    }
}
