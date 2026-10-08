struct ScreenUniforms {
    screen_size: vec2<f32>,
    padding: vec2<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: ScreenUniforms;

struct RectInstance {
    @location(0)  rect_bounds:   vec4<f32>, // pos in xy, size in zw
    @location(1)  color:         vec4<f32>,
    @location(2)  radius:        vec4<f32>,
    @location(3)  border_width:  f32,
    @location(4)  border_color:  vec4<f32>,
    @location(5)  shadow_color:  vec4<f32>,
    @location(6)  shadow_offset: vec2<f32>,
    @location(7)  shadow_blur:   f32,
    @location(8)  clip_rect:     vec4<f32>,
    @location(9)  filter_params: vec4<f32>, // grayscale, brightness, opacity, border_alignment
    @location(10) color2:        vec4<f32>,
    @location(11) color3:        vec4<f32>,
    @location(12) color4:        vec4<f32>,
    @location(13) gradient_params: vec4<f32>, // [type, angle_or_count, anchor_x, anchor_y]
    @location(14) gradient_stops:  vec4<f32>, // [s0, s1, s2, s3]
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0)  color:           vec4<f32>,
    @location(1)  local_pos:       vec2<f32>,
    @location(2)  half_size:       vec2<f32>,
    @location(3)  radius:          vec4<f32>,
    @location(4)  border_color:    vec4<f32>,
    @location(5)  shadow_color:    vec4<f32>,
    @location(6)  clip_rect:       vec4<f32>,
    @location(7)  world_pos:       vec2<f32>,
    @location(8)  filter_params:   vec4<f32>,
    @location(9)  shadow_params:   vec4<f32>, // [shadow_offset.x, shadow_offset.y, shadow_blur, border_width]
    @location(10) color2:          vec4<f32>,
    @location(11) color3:          vec4<f32>,
    @location(12) color4:          vec4<f32>,
    @location(13) gradient_params: vec4<f32>,
    @location(14) gradient_stops:  vec4<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    instance: RectInstance,
) -> VertexOutput {
    var out: VertexOutput;

    var corners = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 0.0),
        vec2<f32>(1.0, 1.0),
    );
    let corner = corners[in_vertex_index];

    let pos  = instance.rect_bounds.xy;
    let size = instance.rect_bounds.zw;

    let expansion     = max(2.0, instance.shadow_blur * 3.0);
    let expanded_size = size + expansion * 2.0;
    let expanded_pos  = pos  - expansion;

    let pixel_pos = expanded_pos + corner * expanded_size;

    let clip_x = (pixel_pos.x / uniforms.screen_size.x) * 2.0 - 1.0;
    let clip_y = 1.0 - (pixel_pos.y / uniforms.screen_size.y) * 2.0;

    out.clip_position = vec4<f32>(clip_x, clip_y, 0.0, 1.0);
    out.half_size     = size * 0.5;
    out.local_pos     = pixel_pos - (pos + out.half_size);

    out.color         = instance.color;
    out.radius        = instance.radius;
    out.border_color  = instance.border_color;
    out.shadow_color  = instance.shadow_color;
    out.clip_rect     = instance.clip_rect;
    out.world_pos     = pixel_pos;
    out.filter_params = instance.filter_params;
    out.shadow_params = vec4<f32>(instance.shadow_offset.x, instance.shadow_offset.y, instance.shadow_blur, instance.border_width);
    out.color2        = instance.color2;
    out.color3        = instance.color3;
    out.color4        = instance.color4;
    out.gradient_params = instance.gradient_params;
    out.gradient_stops  = instance.gradient_stops;

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

fn gaussian_shadow(d: f32, sigma: f32) -> f32 {
    if d > 0.0 {
        return exp(-0.5 * (d * d) / (sigma * sigma));
    }
    return 1.0;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 0. Clip
    if in.world_pos.x < in.clip_rect.x ||
       in.world_pos.x > (in.clip_rect.x + in.clip_rect.z) ||
       in.world_pos.y < in.clip_rect.y ||
       in.world_pos.y > (in.clip_rect.y + in.clip_rect.w) {
        discard;
    }

    let r = in.radius;
    let shadow_offset = in.shadow_params.xy;
    let shadow_blur   = in.shadow_params.z;
    let border_width  = in.shadow_params.w;
    let border_alignment = in.filter_params.w;

    // 1. Shadow
    var shadow_alpha = 0.0;
    if shadow_blur > 0.1 && in.shadow_color.a > 0.0 {
        let shadow_p = in.local_pos - shadow_offset;
        let shadow_d = sdf_rounded_box(shadow_p, in.half_size, r);
        let sigma    = shadow_blur * 0.5;
        shadow_alpha = gaussian_shadow(shadow_d, sigma);
    }
    let premul_shadow = vec4<f32>(
        in.shadow_color.rgb * in.shadow_color.a * shadow_alpha,
        in.shadow_color.a * shadow_alpha,
    );

    // 2. Rect SDF + AA
    let border_offset = border_alignment * border_width;
    let d             = sdf_rounded_box(in.local_pos, in.half_size, r);
    let d_shifted     = d - border_offset;
    
    let aa_width      = max(0.5 * fwidth(d), 0.5);
    let rect_alpha    = 1.0 - smoothstep(-aa_width, aa_width, d_shifted);

    // Fill calculation (Solid, Linear, Radial, or Freeform Mesh)
    var fill_color = in.color;
    let g_type = in.gradient_params.x;

    if g_type > 0.5 {
        let half_w = max(in.half_size.x, 0.001);
        let half_h = max(in.half_size.y, 0.001);

        let norm_x = clamp((in.local_pos.x / half_w) * 0.5 + 0.5, 0.0, 1.0);
        let norm_y = clamp((in.local_pos.y / half_h) * 0.5 + 0.5, 0.0, 1.0);

        if g_type > 2.5 {
            // Mesh 4-Corner / Freeform Gradient (Type 3.0)
            // in.color  = Top-Left
            // in.color2 = Top-Right
            // in.color3 = Bottom-Left
            // in.color4 = Bottom-Right
            let u = norm_x;
            let v = norm_y;
            let top = mix(in.color, in.color2, u);
            let bottom = mix(in.color3, in.color4, u);
            fill_color = mix(top, bottom, v);
        } else if g_type > 1.5 {
            // Radial Gradient (Type 2.0)
            let anchor = vec2<f32>(in.gradient_params.z, in.gradient_params.w);
            let d_vec = vec2<f32>(norm_x - anchor.x, norm_y - anchor.y);
            let radius = max(in.gradient_stops.w, 0.5);
            let t = clamp(length(d_vec) / radius, 0.0, 1.0);

            let count = in.gradient_params.y;
            if count > 3.5 {
                if t <= in.gradient_stops.y {
                    let f = clamp((t - in.gradient_stops.x) / max(in.gradient_stops.y - in.gradient_stops.x, 0.0001), 0.0, 1.0);
                    fill_color = mix(in.color, in.color2, f);
                } else if t <= in.gradient_stops.z {
                    let f = clamp((t - in.gradient_stops.y) / max(in.gradient_stops.z - in.gradient_stops.y, 0.0001), 0.0, 1.0);
                    fill_color = mix(in.color2, in.color3, f);
                } else {
                    let f = clamp((t - in.gradient_stops.z) / max(in.gradient_stops.w - in.gradient_stops.z, 0.0001), 0.0, 1.0);
                    fill_color = mix(in.color3, in.color4, f);
                }
            } else if count > 2.5 {
                if t <= in.gradient_stops.y {
                    let f = clamp((t - in.gradient_stops.x) / max(in.gradient_stops.y - in.gradient_stops.x, 0.0001), 0.0, 1.0);
                    fill_color = mix(in.color, in.color2, f);
                } else {
                    let f = clamp((t - in.gradient_stops.y) / max(in.gradient_stops.z - in.gradient_stops.y, 0.0001), 0.0, 1.0);
                    fill_color = mix(in.color2, in.color3, f);
                }
            } else {
                let f = clamp((t - in.gradient_stops.x) / max(in.gradient_stops.y - in.gradient_stops.x, 0.0001), 0.0, 1.0);
                fill_color = mix(in.color, in.color2, f);
            }
        } else {
            // Linear Gradient (Type 1.0)
            let angle = in.gradient_params.y;
            let cos_a = cos(angle);
            let sin_a = sin(angle);
            let cx = norm_x - 0.5;
            let cy = norm_y - 0.5;
            let t = clamp((cx * cos_a + cy * sin_a) + 0.5, 0.0, 1.0);

            let count = in.gradient_params.z;
            if count > 3.5 {
                if t <= in.gradient_stops.y {
                    let f = clamp((t - in.gradient_stops.x) / max(in.gradient_stops.y - in.gradient_stops.x, 0.0001), 0.0, 1.0);
                    fill_color = mix(in.color, in.color2, f);
                } else if t <= in.gradient_stops.z {
                    let f = clamp((t - in.gradient_stops.y) / max(in.gradient_stops.z - in.gradient_stops.y, 0.0001), 0.0, 1.0);
                    fill_color = mix(in.color2, in.color3, f);
                } else {
                    let f = clamp((t - in.gradient_stops.z) / max(in.gradient_stops.w - in.gradient_stops.z, 0.0001), 0.0, 1.0);
                    fill_color = mix(in.color3, in.color4, f);
                }
            } else if count > 2.5 {
                if t <= in.gradient_stops.y {
                    let f = clamp((t - in.gradient_stops.x) / max(in.gradient_stops.y - in.gradient_stops.x, 0.0001), 0.0, 1.0);
                    fill_color = mix(in.color, in.color2, f);
                } else {
                    let f = clamp((t - in.gradient_stops.y) / max(in.gradient_stops.z - in.gradient_stops.y, 0.0001), 0.0, 1.0);
                    fill_color = mix(in.color2, in.color3, f);
                }
            } else {
                let f = clamp((t - in.gradient_stops.x) / max(in.gradient_stops.y - in.gradient_stops.x, 0.0001), 0.0, 1.0);
                fill_color = mix(in.color, in.color2, f);
            }
        }
    }

    let premul_fill   = vec4<f32>(fill_color.rgb * fill_color.a, fill_color.a);
    let premul_border = vec4<f32>(in.border_color.rgb * in.border_color.a, in.border_color.a);

    var rect_body = premul_fill;
    if border_width > 0.1 {
        let border_factor = smoothstep(-border_width - aa_width, -border_width + aa_width, d_shifted);
        rect_body = mix(premul_fill, premul_border, border_factor);
    }
    let premul_rect = rect_body * rect_alpha;

    // 3. Composite shadow + rect (premultiplied Over)
    let out_alpha = premul_rect.a + premul_shadow.a * (1.0 - premul_rect.a);
    let out_rgb   = premul_rect.rgb + premul_shadow.rgb * (1.0 - premul_rect.a);
    var final_color = vec4<f32>(out_rgb, out_alpha);

    // 4. Filters
    let grayscale  = in.filter_params.x;
    let brightness = in.filter_params.y;
    let opacity    = in.filter_params.z;

    let gray    = dot(final_color.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    final_color = vec4<f32>(mix(final_color.rgb, vec3<f32>(gray), grayscale), final_color.a);
    final_color = vec4<f32>(final_color.rgb * brightness, final_color.a * opacity);

    if final_color.a <= 0.001 { discard; }

    return final_color;
}
