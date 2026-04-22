struct Uniforms {
    screen_size: vec2<f32>,
    _padding: vec2<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(0) var atlas_texture: texture_2d<f32>;
@group(1) @binding(1) var atlas_sampler: sampler;

struct GlyphInstance {
    @location(0) pos:   vec2<f32>,  // screen position
    @location(1) size:  vec2<f32>,  // glyph size in pixels
    @location(2) uv0:   vec2<f32>,  // atlas uv top-left
    @location(3) uv1:   vec2<f32>,  // atlas uv bottom-right
    @location(4) color: vec4<f32>,  // text color
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) uv:    vec2<f32>,
    @location(1) color: vec4<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) vi: u32,
    g: GlyphInstance,
) -> VertexOutput {
    var corners = array<vec2<f32>, 6>(
        vec2(0.0, 0.0),
        vec2(1.0, 0.0),
        vec2(0.0, 1.0),
        vec2(0.0, 1.0),
        vec2(1.0, 0.0),
        vec2(1.0, 1.0),
    );
    let c = corners[vi];

    let pixel = g.pos + c * g.size;
    let clip_x = (pixel.x / uniforms.screen_size.x) * 2.0 - 1.0;
    let clip_y = 1.0 - (pixel.y / uniforms.screen_size.y) * 2.0;

    var out: VertexOutput;
    out.clip_pos = vec4(clip_x, clip_y, 0.0, 1.0);
    out.uv       = g.uv0 + c * (g.uv1 - g.uv0);
    out.color    = g.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let mask = textureSample(atlas_texture, atlas_sampler, in.uv).r;
    if mask < 0.001 { discard; }
    return vec4(in.color.rgb, in.color.a * mask);
}
