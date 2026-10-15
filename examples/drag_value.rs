use zenthra::prelude::*;

fn main() {
    // ── 3D Transform States ──────────────────────────────────────────────────
    let mut pos_x: f32 = 0.0;
    let mut pos_y: f32 = 0.0;
    let mut pos_z: f32 = 0.0;
    let mut rot_deg: f32 = 0.0;
    let mut scale_pct: f32 = 100.0;

    // ── Camera & Rendering States ────────────────────────────────────────────
    let mut fov: f32 = 65.0;
    let mut exposure_ev: f32 = 0.0;
    let mut sample_count: u32 = 32;
    let mut quality: u8 = 85;
    let mut gain_db: i32 = -6;

    // ── Advanced / Custom Styled States ──────────────────────────────────────
    let mut blur_radius: f32 = 18.5;
    let mut corner_radius: f32 = 14.0;
    let mut opacity_pct: f32 = 90.0;

    let mut last_event = String::from("Scrub or click any DragValue to interact.");

    App::new()
        .title("Zenthra - DragValue API & Interactive Inspector")
        .size(1120, 960)
        .with_ui(move |ui: &mut Ui| {
            // Root scrollable background container
            ui.container()
                .fill()
                .bg(Color::rgb(0.06, 0.07, 0.09))
                .padding_all(28.0)
                .scroll_y(true)
                .gap(24.0)
                .show(|ui: &mut Ui| {

                    // ── HEADER BANNER ─────────────────────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(12.0)
                        .padding_all(20.0)
                        .column()
                        .gap(8.0)
                        .show(|ui: &mut Ui| {
                            ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                ui.text("DragValue Widget Showcase")
                                    .size(24.0)
                                    .font_weight(FontWeight::Bold)
                                    .color(Color::WHITE)
                                    .show();

                                ui.container()
                                    .bg(Color::rgb(0.15, 0.30, 0.55))
                                    .border((Color::rgb(0.25, 0.45, 0.85), 1.0))
                                    .radius_all(6.0)
                                    .padding(4.0, 10.0, 4.0, 10.0)
                                    .show(|ui: &mut Ui| {
                                        ui.text("Immediate-Mode Generic Scrubbing")
                                            .size(12.0)
                                            .font_weight(FontWeight::Bold)
                                            .color(Color::rgb(0.70, 0.88, 1.0))
                                            .show();
                                    });
                            });

                            ui.text("Horizontal scrub dragging with 6px deadzone detection, modifier acceleration (Shift = 0.1x, Ctrl = 10x), in-place text editing on click, and full BoxStyle/Sibling parity.")
                                .size(14.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });

                    // ── LIVE STATUS BAR & SUMMARY ─────────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.16))
                        .border((Color::rgb(0.20, 0.25, 0.35), 1.0))
                        .radius_all(10.0)
                        .padding_all(16.0)
                        .row()
                        .gap(20.0)
                        .valign(Align::Center)
                        .show(|ui: &mut Ui| {
                            // Position Chip
                            ui.container()
                                .bg(Color::rgb(0.12, 0.15, 0.22))
                                .radius_all(6.0)
                                .padding(6.0, 12.0, 6.0, 12.0)
                                .row()
                                .gap(8.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    ui.text("Position:").size(12.0).color(Color::rgb(0.55, 0.60, 0.70)).show();
                                    ui.text(&format!("({:.1}, {:.1}, {:.1})", pos_x, pos_y, pos_z))
                                        .size(13.0)
                                        .font_weight(FontWeight::Bold)
                                        .color(Color::rgb(0.35, 0.75, 1.0))
                                        .show();
                                });

                            // Rotation Chip
                            ui.container()
                                .bg(Color::rgb(0.12, 0.15, 0.22))
                                .radius_all(6.0)
                                .padding(6.0, 12.0, 6.0, 12.0)
                                .row()
                                .gap(8.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    ui.text("Rotation:").size(12.0).color(Color::rgb(0.55, 0.60, 0.70)).show();
                                    ui.text(&format!("{:.1}°", rot_deg))
                                        .size(13.0)
                                        .font_weight(FontWeight::Bold)
                                        .color(Color::rgb(0.95, 0.75, 0.25))
                                        .show();
                                });

                            // Scale Chip
                            ui.container()
                                .bg(Color::rgb(0.12, 0.15, 0.22))
                                .radius_all(6.0)
                                .padding(6.0, 12.0, 6.0, 12.0)
                                .row()
                                .gap(8.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    ui.text("Scale:").size(12.0).color(Color::rgb(0.55, 0.60, 0.70)).show();
                                    ui.text(&format!("{:.0}%", scale_pct))
                                        .size(13.0)
                                        .font_weight(FontWeight::Bold)
                                        .color(Color::rgb(0.40, 0.85, 0.50))
                                        .show();
                                });

                            // Event log
                            ui.text(&format!("Status: {}", last_event))
                                .size(13.0)
                                .color(Color::rgb(0.70, 0.75, 0.85))
                                .show();
                        });

                    // ── INTERACTIVE LIVE PREVIEW CANVAS ───────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.08, 0.09, 0.13))
                        .border((Color::rgb(0.16, 0.20, 0.28), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                ui.text("Live Canvas Preview (Reactive to DragValues)")
                                    .size(15.0)
                                    .font_weight(FontWeight::Bold)
                                    .color(Color::rgb(0.35, 0.65, 1.0))
                                    .show();

                                ui.text("Canvas Origin: (0, 0) Center Crosshair")
                                    .size(12.0)
                                    .color(Color::rgb(0.50, 0.55, 0.65))
                                    .show();
                            });

                            // Preview surface showing transformed element with coordinates and crosshairs
                            let canvas_h = 220.0;
                            ui.container()
                                .fill_x()
                                .height(canvas_h)
                                .bg(Color::rgb(0.04, 0.05, 0.07))
                                .border((Color::rgb(0.14, 0.18, 0.24), 1.0))
                                .radius_all(8.0)
                                .clip(true)
                                .show(|ui: &mut Ui| {
                                    let c_start_x = ui.cursor_x;
                                    let c_start_y = ui.cursor_y;
                                    let c_w = (ui.max_x - c_start_x).max(400.0);
                                    let center_x = c_start_x + c_w / 2.0;
                                    let center_y = c_start_y + canvas_h / 2.0;

                                    // X Axis Line (Faint Red)
                                    ui.container()
                                        .pos(c_start_x, center_y)
                                        .width(c_w)
                                        .height(1.0)
                                        .bg(Color::rgba(0.9, 0.3, 0.3, 0.30))
                                        .show(|_| {});

                                    // Y Axis Line (Faint Green)
                                    ui.container()
                                        .pos(center_x, c_start_y)
                                        .width(1.0)
                                        .height(canvas_h)
                                        .bg(Color::rgba(0.3, 0.9, 0.3, 0.30))
                                        .show(|_| {});

                                    // Origin Marker Dot
                                    ui.container()
                                        .pos(center_x - 3.0, center_y - 3.0)
                                        .width(6.0)
                                        .height(6.0)
                                        .radius_all(3.0)
                                        .bg(Color::rgba(1.0, 1.0, 1.0, 0.5))
                                        .show(|_| {});

                                    ui.container()
                                        .pos(center_x + 8.0, center_y + 4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.text("(0, 0)")
                                                .size(10.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 0.35))
                                                .show();
                                        });

                                    // Dynamic Depth (Z) & Sizing
                                    let z_depth = (1.0 + pos_z / 150.0).clamp(0.4, 2.0);
                                    let dynamic_w = (140.0 * (scale_pct / 100.0) * z_depth).clamp(40.0, 450.0);
                                    let dynamic_h = (55.0 * (scale_pct / 100.0) * z_depth).clamp(25.0, 160.0);
                                    let dynamic_alpha = (opacity_pct / 100.0).clamp(0.1, 1.0);

                                    let box_cx = center_x + pos_x;
                                    let box_cy = center_y + pos_y;
                                    let box_x = box_cx - dynamic_w / 2.0;
                                    let box_y = box_cy - dynamic_h / 2.0;

                                    let z_elev = (pos_z / 5.0).clamp(-10.0, 30.0);
                                    let eff_blur = (blur_radius + z_elev * 1.5).max(0.0);
                                    let eff_shadow_y = (4.0 + z_elev).max(0.0);

                                    // Directional Compass Arrow & Trigonometry
                                    let norm_rot = ((rot_deg % 360.0) + 360.0) % 360.0;
                                    let rad = norm_rot.to_radians();
                                    let cos_r = rad.cos();
                                    let sin_r = rad.sin();

                                    let rot_arrow = match ((norm_rot + 22.5) / 45.0) as usize % 8 {
                                        0 => "→ (East)",
                                        1 => "↘ (SE)",
                                        2 => "↓ (South)",
                                        3 => "↙ (SW)",
                                        4 => "← (West)",
                                        5 => "↖ (NW)",
                                        6 => "↑ (North)",
                                        7 => "↗ (NE)",
                                        _ => "→ (East)",
                                    };

                                    // ── ROTATIONAL GIMBAL / ORBIT RING ───────────────────
                                    let gimbal_radius = (dynamic_w.hypot(dynamic_h) * 0.5 + 24.0).clamp(45.0, 300.0);
                                    ui.container()
                                        .pos(box_cx - gimbal_radius, box_cy - gimbal_radius)
                                        .width(gimbal_radius * 2.0)
                                        .height(gimbal_radius * 2.0)
                                        .radius_all(gimbal_radius)
                                        .border((Color::rgba(0.35, 0.65, 1.0, 0.18), 1.0))
                                        .show(|_| {});

                                    // Cardinal Ticks on Gimbal Ring (0°, 90°, 180°, 270°)
                                    let cardinal_angles = [0.0f32, 90.0, 180.0, 270.0];
                                    for card_deg in cardinal_angles {
                                        let card_rad = card_deg.to_radians();
                                        let tx = box_cx + gimbal_radius * card_rad.cos();
                                        let ty = box_cy + gimbal_radius * card_rad.sin();
                                        ui.container()
                                            .pos(tx - 2.5, ty - 2.5)
                                            .width(5.0)
                                            .height(5.0)
                                            .radius_all(2.5)
                                            .bg(Color::rgba(0.40, 0.70, 1.0, 0.40))
                                            .show(|_| {});
                                    }

                                    // ── ROTATING HEADING VECTOR & ORBITAL SATELLITE ───────
                                    // Intermediate beam pips connecting box center to orbital bead
                                    for step in 1..=4 {
                                        let fraction = step as f32 / 4.0;
                                        let px = box_cx + (gimbal_radius * fraction) * cos_r;
                                        let py = box_cy + (gimbal_radius * fraction) * sin_r;
                                        let pip_size = 2.0 + fraction * 2.5;
                                        ui.container()
                                            .pos(px - pip_size / 2.0, py - pip_size / 2.0)
                                            .width(pip_size)
                                            .height(pip_size)
                                            .radius_all(pip_size / 2.0)
                                            .bg(Color::rgba(0.98, 0.85, 0.25, 0.35 + fraction * 0.55))
                                            .show(|_| {});
                                    }

                                    // Orbital Heading Bead (Orbits at exact angle norm_rot)
                                    let bead_x = box_cx + gimbal_radius * cos_r;
                                    let bead_y = box_cy + gimbal_radius * sin_r;
                                    ui.container()
                                        .pos(bead_x - 7.0, bead_y - 7.0)
                                        .width(14.0)
                                        .height(14.0)
                                        .radius_all(7.0)
                                        .bg(Color::rgb(0.98, 0.85, 0.25))
                                        .border((Color::WHITE, 1.5))
                                        .shadow(Color::rgba(0.98, 0.85, 0.25, 0.75), 0.0, 0.0, 8.0)
                                        .show(|_| {});

                                    // ── ROTATED BOUNDING WIREFRAME / VERTEX PINS ──────────
                                    // 4 vertices of the box physically rotated by norm_rot
                                    let hw = dynamic_w / 2.0;
                                    let hh = dynamic_h / 2.0;
                                    let local_corners = [
                                        (-hw, -hh), // Top-Left
                                        ( hw, -hh), // Top-Right
                                        ( hw,  hh), // Bottom-Right
                                        (-hw,  hh), // Bottom-Left
                                    ];

                                    for i in 0..4 {
                                        let (lx, ly) = local_corners[i];
                                        // 2D Rotation: rx = lx*cos - ly*sin, ry = lx*sin + ly*cos
                                        let rx = lx * cos_r - ly * sin_r;
                                        let ry = lx * sin_r + ly * cos_r;
                                        let vx = box_cx + rx;
                                        let vy = box_cy + ry;

                                        // Glowing Rotated Vertex Pin
                                        ui.container()
                                            .pos(vx - 4.5, vy - 4.5)
                                            .width(9.0)
                                            .height(9.0)
                                            .radius_all(4.5)
                                            .bg(Color::rgb(0.35, 0.85, 1.0))
                                            .border((Color::WHITE, 1.0))
                                            .shadow(Color::rgba(0.35, 0.85, 1.0, 0.60), 0.0, 0.0, 6.0)
                                            .show(|_| {});

                                        // Midpoint pin along edge (i -> next)
                                        let (nlx, nly) = local_corners[(i + 1) % 4];
                                        let mx = (lx + nlx) * 0.5;
                                        let my = (ly + nly) * 0.5;
                                        let rmx = mx * cos_r - my * sin_r;
                                        let rmy = mx * sin_r + my * cos_r;
                                        ui.container()
                                            .pos(box_cx + rmx - 2.5, box_cy + rmy - 2.5)
                                            .width(5.0)
                                            .height(5.0)
                                            .radius_all(2.5)
                                            .bg(Color::rgba(0.40, 0.75, 1.0, 0.50))
                                            .show(|_| {});
                                    }

                                    // ── TRANSFORMED INTERACTIVE BOX ──────────────────────
                                    // Features dynamic linear gradient physically rotating on GPU with rot_deg
                                    ui.container()
                                        .pos(box_x, box_y)
                                        .width(dynamic_w)
                                        .height(dynamic_h)
                                        .bg(Gradient::linear(
                                            Direction::Angle(norm_rot),
                                            [
                                                (0.0, Color::rgba(0.28, 0.62, 1.0, dynamic_alpha)),
                                                (0.5, Color::rgba(0.16, 0.38, 0.85, dynamic_alpha)),
                                                (1.0, Color::rgba(0.06, 0.14, 0.32, dynamic_alpha)),
                                            ],
                                        ))
                                        .border((Color::rgb(0.50, 0.75, 1.0), 2.0))
                                        .radius_all(corner_radius.clamp(0.0, 30.0))
                                        .shadow(Color::rgba(0.0, 0.0, 0.0, 0.55), 0.0, eff_shadow_y, eff_blur)
                                        .valign(Align::Center)
                                        .halign(Align::Center)
                                        .padding_all(6.0)
                                        .column()
                                        .gap(2.0)
                                        .show(|ui: &mut Ui| {
                                            ui.row().gap(6.0).valign(Align::Center).show(|ui: &mut Ui| {
                                                ui.text(&format!("{:.1}°", norm_rot))
                                                    .size(13.0)
                                                    .font_weight(FontWeight::Bold)
                                                    .color(Color::rgb(0.98, 0.85, 0.25))
                                                    .show();

                                                ui.text(rot_arrow)
                                                    .size(11.0)
                                                    .font_weight(FontWeight::Medium)
                                                    .color(Color::WHITE)
                                                    .show();
                                            });

                                            ui.text(&format!("{:.0}x{:.0} • ({:.0}, {:.0})", dynamic_w, dynamic_h, pos_x, pos_y))
                                                .size(10.5)
                                                .color(Color::rgb(0.80, 0.90, 1.0))
                                                .show();
                                        });
                                });
                        });

                    // ── SECTION 1: 3D TRANSFORM INSPECTOR (BLENDER STYLE) ─────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(20.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.text("1. 3D Spatial Transform Inspector")
                                .size(16.0)
                                .font_weight(FontWeight::Bold)
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.text("Click and drag horizontally across any coordinate field to scrub. Hold Shift for 0.1x fine tuning, Ctrl for 10x coarse scrubbing, or click once to type directly.")
                                .size(13.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();

                            // Position Row (X / Y / Z)
                            ui.row().fill_x().wrap(Wrap::Wrap).gap(16.0).valign(Align::Center).show(|ui: &mut Ui| {
                                // X axis (Red)
                                ui.column().gap(4.0).show(|ui: &mut Ui| {
                                    ui.text("Position X").size(12.0).font_weight(FontWeight::Bold).color(Color::rgb(0.95, 0.35, 0.35)).show();
                                    let resp = ui.drag_value(&mut pos_x, "pos_x")
                                        .prefix("X: ")
                                        .suffix(" px")
                                        .speed(0.5)
                                        .range(-1000.0, 1000.0)
                                        .precision(1)
                                        .width(150.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.18))
                                        .border((Color::rgb(0.35, 0.18, 0.18), 1.0))
                                        .focus_border((Color::rgb(0.95, 0.35, 0.35), 1.5))
                                        .radius_all(6.0)
                                        .hover([Style::bg(Color::rgb(0.16, 0.18, 0.24))])
                                        .show();
                                    if resp.changed { last_event = format!("Position X changed to {:.1} px", pos_x); }
                                });

                                // Y axis (Green)
                                ui.column().gap(4.0).show(|ui: &mut Ui| {
                                    ui.text("Position Y").size(12.0).font_weight(FontWeight::Bold).color(Color::rgb(0.40, 0.85, 0.50)).show();
                                    let resp = ui.drag_value(&mut pos_y, "pos_y")
                                        .prefix("Y: ")
                                        .suffix(" px")
                                        .speed(0.5)
                                        .range(-1000.0, 1000.0)
                                        .precision(1)
                                        .width(150.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.18))
                                        .border((Color::rgb(0.18, 0.32, 0.22), 1.0))
                                        .focus_border((Color::rgb(0.40, 0.85, 0.50), 1.5))
                                        .radius_all(6.0)
                                        .hover([Style::bg(Color::rgb(0.16, 0.18, 0.24))])
                                        .show();
                                    if resp.changed { last_event = format!("Position Y changed to {:.1} px", pos_y); }
                                });

                                // Z axis (Blue)
                                ui.column().gap(4.0).show(|ui: &mut Ui| {
                                    ui.text("Position Z").size(12.0).font_weight(FontWeight::Bold).color(Color::rgb(0.35, 0.70, 1.0)).show();
                                    let resp = ui.drag_value(&mut pos_z, "pos_z")
                                        .prefix("Z: ")
                                        .suffix(" px")
                                        .speed(0.5)
                                        .range(-1000.0, 1000.0)
                                        .precision(1)
                                        .width(150.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.18))
                                        .border((Color::rgb(0.18, 0.26, 0.40), 1.0))
                                        .focus_border((Color::rgb(0.35, 0.70, 1.0), 1.5))
                                        .radius_all(6.0)
                                        .hover([Style::bg(Color::rgb(0.16, 0.18, 0.24))])
                                        .show();
                                    if resp.changed { last_event = format!("Position Z changed to {:.1} px", pos_z); }
                                });
                            });

                            // Rotation & Scale Row
                            ui.row().fill_x().wrap(Wrap::Wrap).gap(16.0).valign(Align::Center).show(|ui: &mut Ui| {
                                // Rotation with smooth continuous scrubbing
                                ui.column().gap(4.0).show(|ui: &mut Ui| {
                                    ui.text("Rotation Angle (Continuous Scrubbing)").size(12.0).font_weight(FontWeight::Bold).color(Color::rgb(0.95, 0.85, 0.25)).show();
                                    let resp = ui.drag_value(&mut rot_deg, "rot_deg")
                                        .prefix("Rot: ")
                                        .suffix("°")
                                        .speed(0.5)
                                        .range(-360.0, 360.0)
                                        .precision(1)
                                        .width(220.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.18))
                                        .border((Color::rgb(0.35, 0.32, 0.15), 1.0))
                                        .focus_border((Color::rgb(0.95, 0.85, 0.25), 1.5))
                                        .radius_all(6.0)
                                        .hover([Style::bg(Color::rgb(0.18, 0.20, 0.28))])
                                        .show();
                                    if resp.changed { last_event = format!("Rotation changed to {:.1}°", rot_deg); }
                                });

                                // Scale percentage
                                ui.column().gap(4.0).show(|ui: &mut Ui| {
                                    ui.text("Uniform Scale (Min 10%, Max 500%)").size(12.0).font_weight(FontWeight::Bold).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    let resp = ui.drag_value(&mut scale_pct, "scale_pct")
                                        .prefix("Scale: ")
                                        .suffix("%")
                                        .speed(0.5)
                                        .range(10.0, 500.0)
                                        .precision(0)
                                        .width(220.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.18))
                                        .border((Color::rgb(0.24, 0.28, 0.36), 1.0))
                                        .focus_border((Color::rgb(0.40, 0.85, 0.50), 1.5))
                                        .radius_all(6.0)
                                        .hover([Style::bg(Color::rgb(0.16, 0.19, 0.26))])
                                        .show();
                                    if resp.changed { last_event = format!("Scale changed to {:.0}%", scale_pct); }
                                });
                            });
                        });

                    // ── SECTION 2: GENERIC NUMERIC TYPES (f32, u32, u8, i32) ──
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(20.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.text("2. Generic Numeric Binding & Precision Formats")
                                .size(16.0)
                                .font_weight(FontWeight::Bold)
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.text("DragValue binds generically to all numeric primitives (f32, f64, u8, u16, u32, u64, i8, i16, i32, i64) with automatic clamping and custom unit suffixes.")
                                .size(13.0)
                                .wrap(TextWrap::Word)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();

                            ui.row().fill_x().wrap(Wrap::Wrap).gap(12.0).valign(Align::Center).show(|ui: &mut Ui| {
                                // Camera FOV (f32)
                                ui.column().gap(4.0).show(|ui: &mut Ui| {
                                    ui.text("Camera FOV (f32)").size(12.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    let resp = ui.drag_value(&mut fov, "cam_fov")
                                        .suffix("°")
                                        .speed(0.2)
                                        .range(15.0, 120.0)
                                        .precision(1)
                                        .width(125.0)
                                        .bg(Color::rgb(0.11, 0.13, 0.18))
                                        .border((Color::rgb(0.22, 0.26, 0.34), 1.0))
                                        .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.5))
                                        .radius_all(6.0)
                                        .show();
                                    if resp.changed { last_event = format!("Camera FOV changed to {:.1}°", fov); }
                                });

                                // Exposure EV (f32)
                                ui.column().gap(4.0).show(|ui: &mut Ui| {
                                    ui.text("Exposure (f32)").size(12.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    let resp = ui.drag_value(&mut exposure_ev, "exp_ev")
                                        .suffix(" EV")
                                        .speed(0.05)
                                        .range(-6.0, 6.0)
                                        .precision(2)
                                        .width(125.0)
                                        .bg(Color::rgb(0.11, 0.13, 0.18))
                                        .border((Color::rgb(0.22, 0.26, 0.34), 1.0))
                                        .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.5))
                                        .radius_all(6.0)
                                        .show();
                                    if resp.changed { last_event = format!("Exposure changed to {:.2} EV", exposure_ev); }
                                });

                                // Sample Count (u32 integer)
                                ui.column().gap(4.0).show(|ui: &mut Ui| {
                                    ui.text("Samples (u32)").size(12.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    let resp = ui.drag_value(&mut sample_count, "samples")
                                        .suffix(" spp")
                                        .speed(1.0)
                                        .range(1, 1024)
                                        .width(125.0)
                                        .bg(Color::rgb(0.11, 0.13, 0.18))
                                        .border((Color::rgb(0.22, 0.26, 0.34), 1.0))
                                        .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.5))
                                        .radius_all(6.0)
                                        .show();
                                    if resp.changed { last_event = format!("Samples changed to {} spp", sample_count); }
                                });

                                // Audio Gain (i32 signed integer)
                                ui.column().gap(4.0).show(|ui: &mut Ui| {
                                    ui.text("Gain (i32)").size(12.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    let resp = ui.drag_value(&mut gain_db, "gain")
                                        .suffix(" dB")
                                        .speed(1.0)
                                        .range(-60, 12)
                                        .width(125.0)
                                        .bg(Color::rgb(0.11, 0.13, 0.18))
                                        .border((Color::rgb(0.22, 0.26, 0.34), 1.0))
                                        .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.5))
                                        .radius_all(6.0)
                                        .show();
                                    if resp.changed { last_event = format!("Gain changed to {} dB", gain_db); }
                                });

                                // Quality (u8 byte)
                                ui.column().gap(4.0).show(|ui: &mut Ui| {
                                    ui.text("Quality (u8)").size(12.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    let resp = ui.drag_value(&mut quality, "quality")
                                        .suffix("%")
                                        .speed(1.0)
                                        .range(1, 100)
                                        .width(120.0)
                                        .bg(Color::rgb(0.11, 0.13, 0.18))
                                        .border((Color::rgb(0.22, 0.26, 0.34), 1.0))
                                        .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.5))
                                        .radius_all(6.0)
                                        .show();
                                    if resp.changed { last_event = format!("Quality changed to {}%", quality); }
                                });
                            });
                        });

                    // ── SECTION 3: SIBLING INTERACTION & FOCUS THEMING ────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(20.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.text("3. Sibling Interaction States & Focus Theming")
                                .size(16.0)
                                .font_weight(FontWeight::Bold)
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.text("Demonstrating unified Sibling arrays (.hover([...]), .active([...])) and focus styling overrides with smooth GPU quad blending.")
                                .size(13.0)
                                .wrap(TextWrap::Word)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();

                            ui.row().fill_x().wrap(Wrap::Wrap).gap(16.0).valign(Align::Center).show(|ui: &mut Ui| {
                                // Blur Radius
                                ui.column().gap(4.0).show(|ui: &mut Ui| {
                                    ui.text("Shadow Blur Radius").size(12.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    let resp = ui.drag_value(&mut blur_radius, "blur_radius")
                                        .prefix("Blur: ")
                                        .suffix(" px")
                                        .speed(0.25)
                                        .range(0.0, 50.0)
                                        .precision(1)
                                        .width(190.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.20))
                                        .border((Color::rgb(0.25, 0.30, 0.42), 1.0))
                                        .focus_border((Color::rgb(0.60, 0.40, 1.0), 2.0))
                                        .radius_all(8.0)
                                        .hover([
                                            Style::bg(Color::rgb(0.16, 0.19, 0.28)),
                                            Style::border((Color::rgb(0.45, 0.55, 0.80), 1.5)),
                                            Transform::scale(1.02),
                                        ])
                                        .active([
                                            Transform::scale(0.98),
                                        ])
                                        .show();
                                    if resp.changed { last_event = format!("Blur changed to {:.1} px", blur_radius); }
                                });

                                // Corner Radius
                                ui.column().gap(4.0).show(|ui: &mut Ui| {
                                    ui.text("Corner Radius").size(12.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    let resp = ui.drag_value(&mut corner_radius, "corner_radius")
                                        .prefix("Radius: ")
                                        .suffix(" px")
                                        .speed(0.25)
                                        .range(0.0, 30.0)
                                        .precision(1)
                                        .width(190.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.20))
                                        .border((Color::rgb(0.25, 0.30, 0.42), 1.0))
                                        .focus_border((Color::rgb(0.40, 0.85, 0.50), 2.0))
                                        .radius_all(8.0)
                                        .hover([
                                            Style::bg(Color::rgb(0.16, 0.19, 0.28)),
                                            Style::border((Color::rgb(0.45, 0.55, 0.80), 1.5)),
                                            Transform::scale(1.02),
                                        ])
                                        .active([
                                            Transform::scale(0.98),
                                        ])
                                        .show();
                                    if resp.changed { last_event = format!("Corner radius changed to {:.1} px", corner_radius); }
                                });

                                // Opacity
                                ui.column().gap(4.0).show(|ui: &mut Ui| {
                                    ui.text("Preview Opacity").size(12.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    let resp = ui.drag_value(&mut opacity_pct, "opacity_pct")
                                        .prefix("Alpha: ")
                                        .suffix("%")
                                        .speed(0.5)
                                        .range(10.0, 100.0)
                                        .precision(0)
                                        .width(190.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.20))
                                        .border((Color::rgb(0.25, 0.30, 0.42), 1.0))
                                        .focus_border((Color::rgb(0.95, 0.45, 0.30), 2.0))
                                        .radius_all(8.0)
                                        .hover([
                                            Style::bg(Color::rgb(0.16, 0.19, 0.28)),
                                            Style::border((Color::rgb(0.50, 0.60, 0.80), 1.5)),
                                            Transform::scale(1.02),
                                        ])
                                        .active([
                                            Transform::scale(0.98),
                                        ])
                                        .show();
                                    if resp.changed { last_event = format!("Opacity changed to {:.0}%", opacity_pct); }
                                });
                            });
                        });

                    // ── FOOTER / SHORTCUTS CHEATSHEET ─────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.08, 0.09, 0.12))
                        .border((Color::rgb(0.15, 0.18, 0.25), 1.0))
                        .radius_all(8.0)
                        .padding_all(14.0)
                        .column()
                        .gap(8.0)
                        .show(|ui: &mut Ui| {
                            ui.text("💡 Interaction Shortcuts").size(13.0).font_weight(FontWeight::Bold).color(Color::rgb(0.35, 0.65, 1.0)).show();
                            ui.row().fill_x().wrap(Wrap::Wrap).gap(20.0).valign(Align::Center).show(|ui: &mut Ui| {
                                ui.text("• Drag: Scrub value").size(12.0).color(Color::rgb(0.70, 0.75, 0.85)).show();
                                ui.text("• Shift + Drag: Fine (0.1x)").size(12.0).color(Color::rgb(0.70, 0.75, 0.85)).show();
                                ui.text("• Ctrl + Drag: Coarse (10x)").size(12.0).color(Color::rgb(0.70, 0.75, 0.85)).show();
                                ui.text("• Click: Type directly (Enter / Esc)").size(12.0).color(Color::rgb(0.70, 0.75, 0.85)).show();
                            });
                        });
                });
        }).run();
}
