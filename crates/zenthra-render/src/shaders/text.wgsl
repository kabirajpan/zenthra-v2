struct Uniforms {
    screen_size: vec2<f32>,
    _padding:    vec2<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(0) var atlas_texture: texture_2d<f32>;
@group(1) @binding(1) var atlas_sampler: sampler;

struct GlyphInstance {
    @location(0) pos:      vec2<f32>,
    @location(1) size:     vec2<f32>,
    @location(2) uv_pos:   vec2<f32>,
    @location(3) uv_size:  vec2<f32>,
    @location(4) color:    vec4<f32>,
    @location(5) bg_color: vec4<f32>,
    @location(6) clip_rect: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) uv:        vec2<f32>,
    @location(1) color:     vec4<f32>,
    @location(2) bg_color:  vec4<f32>,
    @location(3) uv_size:   vec2<f32>,
    @location(4) clip_rect: vec4<f32>,
    @location(5) screen_pos: vec2<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) vi: u32,
    g: GlyphInstance,
) -> VertexOutput {
    var corners_6 = array<vec2<f32>, 6>(
        vec2(0.0, 0.0), vec2(1.0, 0.0), vec2(0.0, 1.0),
        vec2(0.0, 1.0), vec2(1.0, 0.0), vec2(1.0, 1.0)
    );

    let c = corners_6[vi % 6u];
    let pixel  = g.pos + c * g.size;
    let clip_x = (pixel.x / uniforms.screen_size.x) * 2.0 - 1.0;
    let clip_y = 1.0 - (pixel.y / uniforms.screen_size.y) * 2.0;

    var out: VertexOutput;
    out.clip_pos   = vec4(clip_x, clip_y, 0.0, 1.0);
    out.uv         = g.uv_pos + c * g.uv_size;
    out.color      = g.color;
    out.bg_color   = g.bg_color;
    out.uv_size    = g.uv_size;
    out.clip_rect  = g.clip_rect;
    out.screen_pos = pixel;
    return out;
}

@fragment
fn fs_main(@builtin(position) sc_pos: vec4<f32>, in: VertexOutput) -> @location(0) vec4<f32> {
    // Hardware clipping
    if (sc_pos.x < in.clip_rect.x || sc_pos.x > in.clip_rect.x + in.clip_rect.z ||
        sc_pos.y < in.clip_rect.y || sc_pos.y > in.clip_rect.y + in.clip_rect.w) {
        discard;
    }

    // Detect solid color mode (used for backgrounds)
    if (in.uv_size.x < 0.00001 && in.uv_size.y < 0.00001) {
        return in.bg_color;
    }

    let alpha       = textureSample(atlas_texture, atlas_sampler, in.uv).r;
    let color       = mix(in.bg_color, in.color, alpha);
    let final_alpha = max(in.bg_color.a, in.color.a * alpha);
    
    return vec4(color.rgb, final_alpha);
}
