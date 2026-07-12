// blit.wgsl — Fullscreen texture blit
// Blits an offscreen texture onto the surface (used for the final present pass
// and for sampling the scene behind a backdrop-blur element).

@group(0) @binding(0) var t_src: texture_2d<f32>;
@group(0) @binding(1) var s_src: sampler;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0)       uv:  vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> VsOut {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    // wgpu: y=0 is top in NDC texture coords
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

@fragment
fn fs_main(in: VsOut) -> @location(0) vec4<f32> {
    return textureSample(t_src, s_src, in.uv);
}

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

@vertex
fn vs_backdrop(@builtin(vertex_index) vi: u32) -> BackdropVsOut {
    var quad = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 1.0),
    );
    let corner = quad[vi];
    let pixel_pos = u.rect_pos + corner * u.rect_size;

    let clip_x = (pixel_pos.x / u.screen_size.x) * 2.0 - 1.0;
    let clip_y = 1.0 - (pixel_pos.y / u.screen_size.y) * 2.0;

    var out: BackdropVsOut;
    out.pos = vec4<f32>(clip_x, clip_y, 0.0, 1.0);
    out.uv  = pixel_pos / u.screen_size;
    out.half_size = u.rect_size * 0.5;
    out.local_pos = pixel_pos - (u.rect_pos + out.half_size);
    out.radius = u.radius;
    return out;
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

fn rand(co: vec2<f32>) -> f32 {
    return fract(sin(dot(co, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

@fragment
fn fs_backdrop(in: BackdropVsOut) -> @location(0) vec4<f32> {
    let d = sdf_rounded_box(in.local_pos, in.half_size, in.radius);
    let aa_width = fwidth(d);
    let rect_alpha = 1.0 - smoothstep(-aa_width, aa_width, d);

    if (rect_alpha < 0.01) {
        discard;
    }

    var color = textureSample(t_src, s_src, in.uv);

    // ── CSS Backdrop Filters ─────────────────────────────────────────────────
    // 1. Brightness
    color = vec4<f32>(color.rgb * u.brightness, color.a);
    // 2. Saturation
    let luma = dot(color.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    color = vec4<f32>(mix(vec3<f32>(luma), color.rgb, u.saturation), color.a);
    // 3. Contrast
    color = vec4<f32>((color.rgb - 0.5) * u.contrast + 0.5, color.a);
    // 4. Opacity
    color = vec4<f32>(color.rgb * u.opacity, color.a * u.opacity);

    // ── Apply Preset styles based on u.blur_type ──────────────────────────────
    // 0.0: Normal (raw blur) -> do nothing extra
    // 1.0: Frosted -> add grain
    // 2.0: Glassmorphism -> add rim highlight + grain
    // 3.0: OpaqueGlass -> add white tint + rim highlight + grain
    
    if (u.blur_type >= 3.0) {
        color = vec4<f32>(mix(color.rgb, vec3<f32>(1.0), 0.15), color.a);
    }
    
    if (u.blur_type >= 2.0) {
        // Specular Rim Highlight
        let edge_dist = abs(d);
        if (d <= 0.0 && edge_dist < 1.5) {
            let len = max(length(in.local_pos), 0.0001);
            let normal = in.local_pos / len;
            let tl_dir = normalize(vec2<f32>(-1.0, -1.0));
            let alignment = dot(normal, tl_dir);
            let rim_strength = smoothstep(0.0, 1.5, 1.5 - edge_dist);
            let rim_color = mix(vec3<f32>(0.2), vec3<f32>(1.0), smoothstep(-0.5, 0.5, alignment));
            color = vec4<f32>(mix(color.rgb, rim_color, rim_strength * 0.35), color.a);
        }
    }
    
    if (u.blur_type >= 1.0) {
        // Frosted Grain/Noise Overlay
        let n = rand(in.local_pos * 1.83);
        let grain = (n - 0.5) * 0.025;
        color = vec4<f32>(color.rgb + vec3<f32>(grain), color.a);
    }

    return vec4<f32>(color.rgb * rect_alpha, color.a * rect_alpha);
}
