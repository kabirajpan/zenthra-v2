// examples/liquid_distortion.wgsl
// Custom fragment shader for Zenthra container post-processing.
// Simulates a liquid/water ripple refraction effect inside the card.

@group(0) @binding(0) var t_src: texture_2d<f32>;
@group(0) @binding(1) var s_src: sampler;

struct BackdropUniforms {
    radius: vec4<f32>,
    rect_pos: vec2<f32>,
    rect_size: vec2<f32>,
    screen_size: vec2<f32>,
    time: f32,
    brightness: f32,
    saturation: f32,
    contrast: f32,
    blur_type: f32,
    opacity: f32,
    padding: vec2<f32>,
    _end_padding: vec2<f32>,
}

@group(1) @binding(0) var<uniform> u: BackdropUniforms;

struct BackdropVsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0)       uv:  vec2<f32>,
    @location(1)       local_pos: vec2<f32>,
    @location(2)       half_size: vec2<f32>,
    @location(3)       radius: vec4<f32>,
}

fn sdf_rounded_box(p: vec2<f32>, b: vec2<f32>, r: vec4<f32>) -> f32 {
    let corner = select(
        select(r.w, r.z, p.x > 0.0),
        select(r.x, r.y, p.x > 0.0),
        p.y > 0.0
    );
    let q = abs(p) - b + corner;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - corner;
}

@fragment
fn fs_main(in: BackdropVsOut) -> @location(0) vec4<f32> {
    let d = sdf_rounded_box(in.local_pos, in.half_size, in.radius);
    let aa_width = fwidth(d);
    let rect_alpha = 1.0 - smoothstep(-aa_width, aa_width, d);

    if (rect_alpha < 0.01) {
        discard;
    }

    // ── Liquid Wave Distortion Refraction ────────────────────────────────────
    // Displace the sampling UVs using a sine/cosine wave based on position and time
    let warp_factor = u.padding.x * 0.6;
    let wave_x = sin(in.local_pos.y * 0.04 + u.time * 3.5) * warp_factor;
    let wave_y = cos(in.local_pos.x * 0.04 + u.time * 3.0) * warp_factor;
    let distorted_uv = in.uv + vec2<f32>(wave_x, wave_y) / u.screen_size;

    var color = textureSample(t_src, s_src, distorted_uv);

    // ── Specular Highlights ──────────────────────────────────────────────────
    let edge_dist = abs(d);
    if (d <= 0.0 && edge_dist < 2.0) {
        let len = max(length(in.local_pos), 0.0001);
        let normal = in.local_pos / len;
        let tl_dir = normalize(vec2<f32>(-1.0, -1.0));
        let alignment = dot(normal, tl_dir);
        let rim_strength = smoothstep(0.0, 2.0, 2.0 - edge_dist);
        let rim_color = mix(vec3<f32>(0.2), vec3<f32>(1.0), smoothstep(-0.5, 0.5, alignment));
        color = vec4<f32>(mix(color.rgb, rim_color, rim_strength * 0.4), color.a);
    }

    return vec4<f32>(color.rgb * rect_alpha, color.a * rect_alpha);
}
