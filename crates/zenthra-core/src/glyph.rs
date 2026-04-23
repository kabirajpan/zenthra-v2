use bytemuck::{Pod, Zeroable};

/// A single instance of a glyph (or a solid background block) to be rendered on the GPU.
/// This is the common "language" between the text engine and the main renderer.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct GlyphInstance {
    /// Screen position [x, y].
    pub pos: [f32; 2],
    /// Screen size [width, height].
    pub size: [f32; 2],
    /// UV position [u, v].
    pub uv_pos: [f32; 2],
    /// UV size [u_width, v_height].
    pub uv_size: [f32; 2],
    /// Text color [r, g, b, a].
    pub color: [f32; 4],
    /// Background color [r, g, b, a].
    pub bg_color: [f32; 4],
    /// Clip rectangle [x, y, width, height].
    pub clip_rect: [f32; 4],
}

impl GlyphInstance {
    /// Returns the vertex buffer layout for this struct.
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GlyphInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute { offset: 0, shader_location: 0, format: wgpu::VertexFormat::Float32x2 }, // pos
                wgpu::VertexAttribute { offset: 8, shader_location: 1, format: wgpu::VertexFormat::Float32x2 }, // size
                wgpu::VertexAttribute { offset: 16, shader_location: 2, format: wgpu::VertexFormat::Float32x2 }, // uv_pos
                wgpu::VertexAttribute { offset: 24, shader_location: 3, format: wgpu::VertexFormat::Float32x2 }, // uv_size
                wgpu::VertexAttribute { offset: 32, shader_location: 4, format: wgpu::VertexFormat::Float32x4 }, // color
                wgpu::VertexAttribute { offset: 48, shader_location: 5, format: wgpu::VertexFormat::Float32x4 }, // bg_color
                wgpu::VertexAttribute { offset: 64, shader_location: 6, format: wgpu::VertexFormat::Float32x4 }, // clip_rect
            ],
        }
    }
}
