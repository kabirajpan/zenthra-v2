struct ScreenUniforms {
    screen_size: vec2<f32>,
    padding: vec2<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: ScreenUniforms;
@group(1) @binding(0) var t_diffuse: texture_2d<f32>;
@group(1) @binding(1) var s_diffuse: sampler;

struct ImageInstance {
    @location(0)  pos:           vec2<f32>,
    @location(1)  size:          vec2<f32>,
    @location(2)  radius:        vec4<f32>,
    @location(3)  border_width:  f32,
    @location(4)  border_color:  vec4<f32>,
    @location(5)  shadow_color:  vec4<f32>,
    @location(6)  shadow_offset: vec2<f32>,
    @location(7)  shadow_blur:   f32,
    @location(8)  clip_rect:     vec4<f32>,
    @location(9)  grayscale:     f32,
    @location(10) brightness:    f32,
    @location(11) opacity:       f32,
    @location(12) uv_rect:       vec4<f32>, // (u_start, v_start, u_size, v_size)
    @location(13) bg_color:      vec4<f32>,
    @location(14) rotation:      vec3<f32>,
    @location(15) flip:          vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0)  local_pos:     vec2<f32>,
    @location(1)  half_size:     vec2<f32>,
    @location(2)  radius:        vec4<f32>,
    @location(3)  border_width:  f32,
    @location(4)  border_color:  vec4<f32>,
    @location(5)  shadow_color:  vec4<f32>,
    @location(6)  shadow_offset: vec2<f32>,
    @location(7)  shadow_blur:   f32,
    @location(8)  clip_rect:     vec4<f32>,
    @location(9)  grayscale:     f32,
    @location(10) brightness:    f32,
    @location(11) opacity:       f32,
    @location(13) uv:            vec2<f32>,
    @location(14) bg_color:      vec4<f32>,
    @location(15) world_pos:     vec2<f32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
    instance: ImageInstance,
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
    let expansion = max(2.0, instance.shadow_blur * 3.0);
    
    // 1. Calculate Local Vertex Position (centred at 0,0 for rotation)
    // We add expansion to the quad size to accommodate the shadow blur
    let quad_size = instance.size + expansion * 2.0;
    let local_v_pos = (corner - 0.5) * quad_size;

    // 2. Apply 3D Rotation
    // rotation: (x_tilt, y_tilt, z_rot)
    let rot = instance.rotation;
    
    // Rotation Z (2D Rotation)
    let sz = sin(rot.z); let cz = cos(rot.z);
    let rxz = local_v_pos.x * cz - local_v_pos.y * sz;
    let ryz = local_v_pos.x * sz + local_v_pos.y * cz;
    var rotated_pos = vec3<f32>(rxz, ryz, 0.0);

    // Rotation X (Tilt forward/back)
    let sx = sin(rot.x); let cx = cos(rot.x);
    let ryx = rotated_pos.y * cx - rotated_pos.z * sx;
    let rzx = rotated_pos.y * sx + rotated_pos.z * cx;
    rotated_pos.y = ryx;
    rotated_pos.z = rzx;

    // Rotation Y (Tilt side-to-side)
    let sy = sin(rot.y); let cy = cos(rot.y);
    let rxy = rotated_pos.x * cy + rotated_pos.z * sy;
    let rzy = -rotated_pos.x * sy + rotated_pos.z * cy;
    rotated_pos.x = rxy;
    rotated_pos.z = rzy;

    // 3. Simple Perspective Projection
    // We move the camera back a bit to see the tilt effect
    let dist = 1000.0; 
    let perspective = dist / (dist - rotated_pos.z);
    let final_local_pos = rotated_pos.xy * perspective;

    // 4. Transform to Screen Coordinates
    let center_pos = instance.pos + instance.size * 0.5;
    let pixel_pos = center_pos + final_local_pos;

    let clip_x = (pixel_pos.x / uniforms.screen_size.x) * 2.0 - 1.0;
    let clip_y = 1.0 - (pixel_pos.y / uniforms.screen_size.y) * 2.0;

    out.clip_position = vec4<f32>(clip_x, clip_y, 0.0, 1.0);
    out.half_size     = instance.size * 0.5;
    
    // local_pos for SDF needs to be the UN-ROTATED, UN-EXPANDED position relative to image center
    // But we need to handle the scale/perspective too. 
    // For now, we'll use the corner-based mapping which is more robust for SDFs with rotation.
    out.local_pos     = (corner - 0.5) * quad_size; 

    out.radius        = instance.radius;
    out.border_width  = instance.border_width;
    out.border_color  = instance.border_color;
    out.shadow_color  = instance.shadow_color;
    out.shadow_offset = instance.shadow_offset;
    out.shadow_blur   = instance.shadow_blur;
    out.clip_rect     = instance.clip_rect;
    out.grayscale     = instance.grayscale;
    out.brightness    = instance.brightness;
    out.opacity       = instance.opacity;
    out.bg_color      = instance.bg_color;

    // 5. UV Mapping with Flipping
    // We only map UVs for the actual image rect, excluding expansion
    let img_norm = (out.local_pos / instance.size) + 0.5;
    let clamped_norm = clamp(img_norm, vec2<f32>(0.0), vec2<f32>(1.0));
    
    // Apply Flip
    let flipped_norm = (clamped_norm - 0.5) * instance.flip + 0.5;
    
    out.uv = vec2<f32>(
        instance.uv_rect.x + flipped_norm.x * instance.uv_rect.z,
        instance.uv_rect.y + flipped_norm.y * instance.uv_rect.w
    );
    out.world_pos = pixel_pos;

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

    // 1. Shadow
    var shadow_alpha = 0.0;
    if in.shadow_blur > 0.1 && in.shadow_color.a > 0.0 {
        let shadow_p = in.local_pos - in.shadow_offset;
        let shadow_d = sdf_rounded_box(shadow_p, in.half_size, r);
        let sigma    = in.shadow_blur * 0.5;
        shadow_alpha = gaussian_shadow(shadow_d, sigma);
    }
    let premul_shadow = vec4<f32>(
        in.shadow_color.rgb * in.shadow_color.a * shadow_alpha,
        in.shadow_color.a * shadow_alpha,
    );

    // 2. Rect SDF + AA
    let d        = sdf_rounded_box(in.local_pos, in.half_size, r);
    let aa_width = fwidth(d);
    let rect_alpha = 1.0 - smoothstep(-aa_width, aa_width, d);

    // Composite texture OVER background
    var tex_color = vec4<f32>(0.0);
    
    // Only sample if UVs are within the valid 0..1 range
    // (This allows us to shrink the image inside the box using app.rs logic)
    if in.uv.x >= 0.0 && in.uv.x <= 1.0 && in.uv.y >= 0.0 && in.uv.y <= 1.0 {
        tex_color = textureSample(t_diffuse, s_diffuse, in.uv);
    }
    
    let premul_tex = vec4<f32>(tex_color.rgb * tex_color.a, tex_color.a);
    let premul_bg  = vec4<f32>(in.bg_color.rgb * in.bg_color.a, in.bg_color.a);
    let premul_fill = premul_tex + premul_bg * (1.0 - premul_tex.a);

    let premul_border = vec4<f32>(in.border_color.rgb * in.border_color.a, in.border_color.a);

    var rect_body = premul_fill;
    if in.border_width > 0.1 {
        // Border exists from d = -border_width to d = 0
        let border_factor = smoothstep(-in.border_width - aa_width, -in.border_width + aa_width, d);
        rect_body = mix(premul_fill, premul_border, border_factor);
    }
    let premul_rect = rect_body * rect_alpha;

    // 3. Composite shadow + rect (premultiplied Over)
    let out_alpha = premul_rect.a + premul_shadow.a * (1.0 - premul_rect.a);
    let out_rgb   = premul_rect.rgb + premul_shadow.rgb * (1.0 - premul_rect.a);
    var final_color = vec4<f32>(out_rgb, out_alpha);

    // 4. Filters
    let gray    = dot(final_color.rgb, vec3<f32>(0.2126, 0.7152, 0.0722));
    final_color = vec4<f32>(mix(final_color.rgb, vec3<f32>(gray), in.grayscale), final_color.a);
    final_color = vec4<f32>(final_color.rgb * in.brightness, final_color.a * in.opacity);

    if final_color.a <= 0.001 { discard; }

    return final_color;
}
