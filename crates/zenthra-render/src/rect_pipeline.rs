use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

/// One rectangle — uploaded as a GPU instance.
/// Field order matches shader @location order exactly.
///
/// offset  0  loc 0  pos           [f32;2]
/// offset  8  loc 1  size          [f32;2]
/// offset 16  loc 2  color         [f32;4]
/// offset 32  loc 3  radius        f32
/// offset 36  loc 4  border_width  f32
/// offset 40  loc 5  border_color  [f32;4]
/// offset 56  loc 6  shadow_color  [f32;4]
/// offset 72  loc 7  shadow_offset [f32;2]
/// offset 80  loc 8  shadow_blur   f32
/// offset 84  loc 9  clip_rect     [f32;4]
/// offset 100 loc 10 grayscale     f32
/// offset 104 loc 11 brightness    f32
/// offset 108 loc 12 opacity       f32
/// total: 112 bytes
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct RectInstance {
    pub pos: [f32; 2],           // offset 0
    pub size: [f32; 2],          // offset 8
    pub color: [f32; 4],         // offset 16
    pub radius: f32,             // offset 32
    pub border_width: f32,       // offset 36
    pub border_color: [f32; 4],  // offset 40
    pub shadow_color: [f32; 4],  // offset 56
    pub shadow_offset: [f32; 2], // offset 72
    pub shadow_blur: f32,        // offset 80
    pub clip_rect: [f32; 4],     // offset 84
    pub grayscale: f32,          // offset 100
    pub brightness: f32,         // offset 104
    pub opacity: f32,            // offset 108
}

impl Default for RectInstance {
    fn default() -> Self {
        Self {
            pos: [0.0; 2],
            size: [100.0, 100.0],
            color: [1.0, 1.0, 1.0, 1.0],
            radius: 0.0,
            border_width: 0.0,
            border_color: [0.0; 4],
            shadow_color: [0.0; 4],
            shadow_offset: [0.0; 2],
            shadow_blur: 0.0,
            clip_rect: [0.0, 0.0, 9999.0, 9999.0],
            grayscale: 0.0,
            brightness: 1.0,
            opacity: 1.0,
        }
    }
}

impl RectInstance {
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RectInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 16,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: 36,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: 40,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 56,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 72,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: 80,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: 84,
                    shader_location: 9,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: 100,
                    shader_location: 10,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: 104,
                    shader_location: 11,
                    format: wgpu::VertexFormat::Float32,
                },
                wgpu::VertexAttribute {
                    offset: 108,
                    shader_location: 12,
                    format: wgpu::VertexFormat::Float32,
                },
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

pub struct RectPipeline {
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    uniform_bg: wgpu::BindGroup,
    instance_buffer: Option<wgpu::Buffer>,
    instance_count: u32,
}

impl RectPipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/rect.wgsl"));

        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Rect Uniform Buffer"),
            size: std::mem::size_of::<ScreenUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Rect BGL"),
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
            label: Some("Rect Uniform BG"),
            layout: &bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Rect Pipeline Layout"),
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Rect Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[RectInstance::desc()],
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
            instance_buffer: None,
            instance_count: 0,
        }
    }

    pub fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        width: u32,
        height: u32,
        rects: &[RectInstance],
    ) {
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::bytes_of(&ScreenUniforms {
                screen_size: [width as f32, height as f32],
                _padding: [0.0; 2],
            }),
        );

        if rects.is_empty() {
            self.instance_count = 0;
            return;
        }

        self.instance_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Rect Instance Buffer"),
                contents: bytemuck::cast_slice(rects),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );
        self.instance_count = rects.len() as u32;
    }

    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        if self.instance_count == 0 {
            return;
        }
        if let Some(buf) = &self.instance_buffer {
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.uniform_bg, &[]);
            pass.set_vertex_buffer(0, buf.slice(..));
            pass.draw(0..6, 0..self.instance_count);
        }
    }
}
