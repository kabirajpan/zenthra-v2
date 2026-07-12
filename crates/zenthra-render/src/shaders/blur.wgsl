// blur.wgsl — Dual Kawase + Separable Box Blur
// Used for glassmorphism backdrop blur in Zenthra.
// Three entry points:
//   fs_downsample — Kawase 4-tap downsample pass
//   fs_upsample   — Kawase 8-tap tent upsample pass
//   fs_box        — Separable 1D box blur (horizontal or vertical)

struct BlurUniforms {
    texel_size: vec2<f32>,    // 1/width, 1/height of SOURCE texture
    offset:     f32,          // Kawase offset multiplier
    direction:  f32,          // box blur: 0 = horizontal, 1 = vertical
}

@group(0) @binding(0) var<uniform> u: BlurUniforms;
@group(1) @binding(0) var t_src:    texture_2d<f32>;
@group(1) @binding(1) var s_src:    sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0)       uv:  vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
    // Fullscreen triangle (3 vertices cover the whole screen)
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    var uvs = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0),
    );
    var out: VsOut;
    out.pos = vec4<f32>(positions[vi], 0.0, 1.0);
    out.uv  = uvs[vi];
    return out;
}

// ------------------------------------------------------------
// Kawase Downsample  (4 diagonal taps around a centre tap)
// Equivalent to your OES_DOWNSAMPLE / DOWNSAMPLE passes
// ------------------------------------------------------------
@fragment
fn fs_downsample(in: VsOut) -> @location(0) vec4<f32> {
    let ts  = u.texel_size;
    let off = u.offset;
    var sum = textureSample(t_src, s_src, in.uv);
    sum += textureSample(t_src, s_src, in.uv + vec2<f32>(-off, -off) * ts);
    sum += textureSample(t_src, s_src, in.uv + vec2<f32>( off, -off) * ts);
    sum += textureSample(t_src, s_src, in.uv + vec2<f32>(-off,  off) * ts);
    sum += textureSample(t_src, s_src, in.uv + vec2<f32>( off,  off) * ts);
    return sum * 0.2; // average 5 taps
}

// ------------------------------------------------------------
// Kawase Upsample  (8 tent taps around center)
// Equivalent to your UPSAMPLE pass
// ------------------------------------------------------------
@fragment
fn fs_upsample(in: VsOut) -> @location(0) vec4<f32> {
    let ts  = u.texel_size;
    let off = u.offset;
    var sum = vec4<f32>(0.0);
    sum += textureSample(t_src, s_src, in.uv + vec2<f32>(-off * 2.0,  0.0        ) * ts);
    sum += textureSample(t_src, s_src, in.uv + vec2<f32>(-off,        off         ) * ts);
    sum += textureSample(t_src, s_src, in.uv + vec2<f32>( 0.0,        off * 2.0  ) * ts);
    sum += textureSample(t_src, s_src, in.uv + vec2<f32>( off,        off         ) * ts);
    sum += textureSample(t_src, s_src, in.uv + vec2<f32>( off * 2.0,  0.0        ) * ts);
    sum += textureSample(t_src, s_src, in.uv + vec2<f32>( off,       -off         ) * ts);
    sum += textureSample(t_src, s_src, in.uv + vec2<f32>( 0.0,       -off * 2.0  ) * ts);
    sum += textureSample(t_src, s_src, in.uv + vec2<f32>(-off,       -off         ) * ts);
    return sum / 8.0;
}

// ------------------------------------------------------------
// Separable Box Blur — 11 taps in one direction
// direction = 0: horizontal   direction = 1: vertical
// Equivalent to your BOX_BLUR_FRAGMENT_SHADER
// ------------------------------------------------------------
@fragment
fn fs_box(in: VsOut) -> @location(0) vec4<f32> {
    let ts  = u.texel_size;
    let dir = select(
        vec2<f32>(ts.x, 0.0),  // horizontal
        vec2<f32>(0.0, ts.y),  // vertical
        u.direction > 0.5
    );
    let stride = u.offset / 5.0;
    var sum = vec4<f32>(0.0);
    for (var i: i32 = -5; i <= 5; i++) {
        sum += textureSample(t_src, s_src, in.uv + dir * (f32(i) * stride));
    }
    return sum / 11.0;
}
