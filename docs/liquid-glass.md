# Liquid Glass: Modern Volumetric Lens & Refraction in Zenthra

Liquid Glass is a modern material system and post-processing technique for UI design. While traditional **glassmorphism** relies on static Gaussian or Kawase blurs with uniform semi-transparency, **Liquid Glass** treats UI elements as volumetric, physical lenses that dynamically bend and disperse light.

---

## 1. Core Principles: Liquid Glass vs. Glassmorphism

| Optical Property | Standard Glassmorphism | Liquid Glass |
| :--- | :--- | :--- |
| **Backdrop Filter** | Flat Gaussian / Kawase blur | Pre-filtered background sampled with non-linear UV displacement |
| **Edge Dynamics** | Flat border stroke (`1px solid`) | **Specular rim lighting & Fresnel glints** computed along curved surface normals |
| **Refraction** | None (direct pass-through) | **Volumetric lens refraction** (bending background content near container perimeters) |
| **Dispersion** | Uniform RGB tint | **Chromatic aberration** (RGB split along refraction vectors) |
| **Surface Motion** | Static | **Micro-fluid ripples & time-based wave equations** |

---

## 2. Architecture in Zenthra

Zenthra provides native custom post-processing container shaders via WebGPU/WGSL. The pipeline passes:
- **`t_src` & `s_src`**: The pre-blurred background texture (Kawase blur pass).
- **`BackdropUniforms`**:
  - `rect_pos: vec2<f32>` (Screen-space container position)
  - `rect_size: vec2<f32>` (Container dimensions)
  - `screen_size: vec2<f32>` (Viewport resolution)
  - `radius: vec4<f32>` (Per-corner radii `[TL, TR, BR, BL]`)
  - `time: f32` (Continuously running elapsed time in seconds)
  - `padding: vec2<f32>` (Parameters such as blur radius or refraction strength)

---

## 3. Implementing the Liquid Glass Shader (`liquid_glass.wgsl`)

```wgsl
// liquid_glass.wgsl
// Volumetric lens refraction, chromatic dispersion, and specular rim glints.

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

    // 1. Edge Normal & Bevel Lens Refraction
    let edge_dist = max(-d, 0.0);
    let bevel_width = 18.0; // Width of the refractive glass bevel
    let bevel_factor = 1.0 - clamp(edge_dist / bevel_width, 0.0, 1.0);
    let normal = normalize(in.local_pos / (in.half_size + 0.001));

    // Refraction displacement vector pointing inwards from bevel
    let refraction_strength = 0.018 * pow(bevel_factor, 1.8);
    let refract_vec = normal * refraction_strength;

    // 2. Micro-fluid wave ripples (optional subtle motion)
    let wave_offset = vec2<f32>(
        sin(in.local_pos.y * 0.03 + u.time * 1.5),
        cos(in.local_pos.x * 0.03 + u.time * 1.2)
    ) * (0.002 * (1.0 - bevel_factor));

    let base_uv = in.uv + refract_vec + wave_offset;

    // 3. Chromatic Dispersion (Prismatic RGB split along refraction)
    let dispersion = 0.0035 * bevel_factor;
    let col_r = textureSample(t_src, s_src, base_uv + normal * dispersion).r;
    let col_g = textureSample(t_src, s_src, base_uv).g;
    let col_b = textureSample(t_src, s_src, base_uv - normal * dispersion).b;
    var color = vec3<f32>(col_r, col_g, col_b);

    // 4. Specular Bevel Highlights & Directional Rim Glint
    let light_dir = normalize(vec2<f32>(-1.0, -1.0)); // Top-left primary light source
    let specular = pow(max(dot(normal, -light_dir), 0.0), 3.0) * bevel_factor;
    color += vec3<f32>(0.9, 0.95, 1.0) * (specular * 0.45);

    // 5. Subtle Frosting Tint
    color = mix(color, vec3<f32>(1.0, 1.0, 1.0), 0.04);

    return vec4<f32>(color * rect_alpha, rect_alpha);
}
```

---

## 4. Usage in Zenthra Apps

### Step 1: Register Shader on App Launch
```rust
App::new()
    .title("Zenthra App")
    .transparent(true)
    .blur(true)
    .register_custom_shader("liquid_glass", include_str!("shaders/liquid_glass.wgsl"))
    .with_ui(|ui| {
        // ...
    });
```

### Step 2: Apply to UI Containers / Modals
```rust
ui.container()
    .width(420.0)
    .height(260.0)
    .radius_all(14.0)
    .backdrop_blur(24.0)                     // Pre-blur background behind glass
    .post_process_shader("liquid_glass")     // Apply Liquid Glass volumetric shader
    .show(|ui| {
        ui.text("Floating Volumetric Liquid Glass Card").show();
    });
```

---

## 5. Design Guidelines & Performance

1. **Floating Elements Only**: Reserve Liquid Glass for elevated components (dialogs, context menus, dock bars, search palettes, and toolbars). Avoid applying it to full-screen base layers to maintain visual hierarchy.
2. **Text Contrast**: Provide a translucent solid underlay or darkened tint for content areas inside the glass to guarantee accessibility and high contrast.
3. **Hardware Acceleration**: Zenthra's Kawase blur and custom shader passes run entirely on the GPU, minimizing CPU overhead.
