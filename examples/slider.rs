use zenthra::prelude::*;

fn main() {
    // ── Slider States ──────────────────────────────────────────────────────────
    let mut volume: f32 = 0.72;
    let mut brightness: f32 = 0.85;
    let mut blur_radius: f32 = 16.0;
    let mut corner_radius: f32 = 12.0;
    let mut audio_pan: f32 = 0.0;
    let mut scale_factor: f32 = 1.0;

    let mut event_log = String::from("Interactive slider events will appear here.");

    App::new()
        .title("Zenthra - Slider API Showcase")
        .size(1050, 920)
        .with_ui(move |ui: &mut Ui| {
            // Root scrollable background container
            ui.container()
                .fill()
                .bg(Color::rgb(0.06, 0.07, 0.09))
                .padding_all(24.0)
                .scroll_y(true)
                .gap(24.0)
                .show(|ui: &mut Ui| {

                    // ── HEADER ────────────────────────────────────────────────
                    ui.container()
                        .fill_x()
                        .column()
                        .gap(6.0)
                        .show(|ui: &mut Ui| {
                            ui.h1("Slider API & Styling Showcase")
                                .color(Color::WHITE)
                                .show();

                            ui.text("Demonstrating immediate-mode floating-point binding, continuous and stepped ranges, custom track rails, gradient thumb knobs, and event listeners.")
                                .size(14.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });

                    // ── STATUS BAR & LIVE READOUTS ────────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.10, 0.12, 0.17))
                        .border((Color::rgb(0.20, 0.25, 0.35), 1.0))
                        .radius_all(8.0)
                        .padding_all(16.0)
                        .row()
                        .gap(24.0)
                        .valign(Align::Center)
                        .show(|ui: &mut Ui| {
                            ui.text(&format!("Volume: {:.0}%", volume * 100.0))
                                .size(14.0)
                                .font_weight(FontWeight::Bold)
                                .color(Color::rgb(0.35, 0.75, 1.0))
                                .show();

                            ui.text(&format!("Brightness: {:.0}%", brightness * 100.0))
                                .size(14.0)
                                .font_weight(FontWeight::Bold)
                                .color(Color::rgb(0.95, 0.80, 0.25))
                                .show();

                            ui.text(&format!("Blur: {:.1}px", blur_radius))
                                .size(14.0)
                                .font_weight(FontWeight::Bold)
                                .color(Color::rgb(0.75, 0.45, 1.0))
                                .show();

                            ui.text(&format!("Event: {}", event_log))
                                .size(13.0)
                                .color(Color::rgb(0.75, 0.80, 0.90))
                                .show();
                        });

                    // ── SECTION 1: STANDARD SLIDERS (CONTINUOUS 0.0 - 1.0) ─────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(18.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("1. Standard Continuous Sliders (0.0 to 1.0)")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            // Volume Slider
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                    ui.text("Master Output Volume").size(13.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    ui.text(&format!("{:.0}%", volume * 100.0)).size(13.0).font_weight(FontWeight::Bold).color(Color::rgb(0.35, 0.75, 1.0)).show();
                                });

                                let resp = ui.slider(&mut volume, "vol_slider")
                                    .range(0.0, 1.0)
                                    .fill_x()
                                    .track_height(6.0)
                                    .track_color(Color::rgb(0.15, 0.18, 0.24))
                                    .thumb_size(18.0, 18.0)
                                    .thumb_color(Color::rgb(0.35, 0.70, 1.0))
                                    .thumb_shadow(Color::rgba(0.20, 0.50, 1.0, 0.40), 0.0, 2.0, 6.0)
                                    .show();
                                if resp.changed {
                                    event_log = format!("Volume adjusted: {:.2}", volume);
                                }
                            });

                            // Display Brightness Slider
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                    ui.text("Display Brightness").size(13.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    ui.text(&format!("{:.0}%", brightness * 100.0)).size(13.0).font_weight(FontWeight::Bold).color(Color::rgb(0.95, 0.80, 0.25)).show();
                                });

                                ui.slider(&mut brightness, "bright_slider")
                                    .range(0.0, 1.0)
                                    .fill_x()
                                    .track_height(6.0)
                                    .track_color(Color::rgb(0.15, 0.18, 0.24))
                                    .thumb_size(18.0, 18.0)
                                    .thumb_color(Color::rgb(0.95, 0.75, 0.20))
                                    .thumb_shadow(Color::rgba(0.95, 0.70, 0.20, 0.40), 0.0, 2.0, 6.0)
                                    .show();
                            });
                        });

                    // ── SECTION 2: STEPPED RANGES & NUMERIC VALUES ─────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(18.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("2. Custom Min/Max Ranges & Stepped Snapping (.step(...))")
                                .color(Color::rgb(0.10, 0.80, 0.50))
                                .show();

                            // Blur Radius: 0.0 to 64.0, step 2.0
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                    ui.text("Backdrop Blur (0px to 64px, step 2.0)").size(13.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    ui.text(&format!("{:.0} px", blur_radius)).size(13.0).font_weight(FontWeight::Bold).color(Color::rgb(0.20, 0.85, 0.50)).show();
                                });

                                ui.slider(&mut blur_radius, "blur_slider")
                                    .range(0.0, 64.0)
                                    .step(2.0)
                                    .fill_x()
                                    .track_height(6.0)
                                    .track_color(Color::rgb(0.15, 0.18, 0.24))
                                    .thumb_size(18.0, 18.0)
                                    .thumb_color(Color::rgb(0.20, 0.85, 0.50))
                                    .thumb_shadow(Color::rgba(0.20, 0.85, 0.50, 0.40), 0.0, 2.0, 6.0)
                                    .show();
                            });

                            // Corner Radius: 0.0 to 32.0, step 4.0
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                    ui.text("Box Corner Radius (0px to 32px, step 4.0)").size(13.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    ui.text(&format!("{:.0} px", corner_radius)).size(13.0).font_weight(FontWeight::Bold).color(Color::rgb(0.20, 0.85, 0.50)).show();
                                });

                                ui.slider(&mut corner_radius, "radius_slider")
                                    .range(0.0, 32.0)
                                    .step(4.0)
                                    .fill_x()
                                    .track_height(6.0)
                                    .track_color(Color::rgb(0.15, 0.18, 0.24))
                                    .thumb_size(18.0, 18.0)
                                    .thumb_color(Color::rgb(0.20, 0.85, 0.50))
                                    .thumb_shadow(Color::rgba(0.20, 0.85, 0.50, 0.40), 0.0, 2.0, 6.0)
                                    .show();
                            });
                        });

                    // ── SECTION 3: CENTERED BIPOLAR & MULTI-RANGE SLIDERS ──────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(18.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("3. Bipolar Range (-1.0 to +1.0) & Zoom Multiplier")
                                .color(Color::rgb(0.95, 0.65, 0.25))
                                .show();

                            // Stereo Audio Pan: -1.0 (Left) to +1.0 (Right)
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                    ui.text("Stereo Audio Pan Balance").size(13.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    let pan_desc = if audio_pan < -0.05 {
                                        format!("{:.0}% Left", audio_pan.abs() * 100.0)
                                    } else if audio_pan > 0.05 {
                                        format!("{:.0}% Right", audio_pan * 100.0)
                                    } else {
                                        String::from("Center (0.0)")
                                    };
                                    ui.text(&pan_desc).size(13.0).font_weight(FontWeight::Bold).color(Color::rgb(0.95, 0.65, 0.25)).show();
                                });

                                ui.slider(&mut audio_pan, "pan_slider")
                                    .range(-1.0, 1.0)
                                    .step(0.05)
                                    .fill_x()
                                    .track_height(6.0)
                                    .track_color(Color::rgb(0.15, 0.18, 0.24))
                                    .thumb_size(18.0, 18.0)
                                    .thumb_color(Color::rgb(0.95, 0.65, 0.25))
                                    .thumb_shadow(Color::rgba(0.95, 0.60, 0.20, 0.40), 0.0, 2.0, 6.0)
                                    .show();
                            });

                            // Zoom / Scale Factor: 0.5x to 3.0x
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                    ui.text("Canvas Zoom Scale (0.5x to 3.0x)").size(13.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    ui.text(&format!("{:.2}x", scale_factor)).size(13.0).font_weight(FontWeight::Bold).color(Color::rgb(0.95, 0.65, 0.25)).show();
                                });

                                ui.slider(&mut scale_factor, "scale_slider")
                                    .range(0.5, 3.0)
                                    .step(0.1)
                                    .fill_x()
                                    .track_height(6.0)
                                    .track_color(Color::rgb(0.15, 0.18, 0.24))
                                    .thumb_size(18.0, 18.0)
                                    .thumb_color(Color::rgb(0.95, 0.65, 0.25))
                                    .thumb_shadow(Color::rgba(0.95, 0.60, 0.20, 0.40), 0.0, 2.0, 6.0)
                                    .show();
                            });
                        });

                    // ── SECTION 4: GRADIENTS & THUMB CUSTOMIZATION ────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(18.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("4. Gradient Tracks & Capsule Pill Thumbs")
                                .color(Color::rgb(0.70, 0.45, 1.0))
                                .show();

                            // Cyberpunk Neon Violet Slider
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.text("Gradient Rail with Glowing Thumb").size(13.0).color(Color::rgb(0.75, 0.80, 0.90)).show();

                                ui.slider(&mut volume, "gradient_slider")
                                    .range(0.0, 1.0)
                                    .fill_x()
                                    .track_height(8.0)
                                    .track_gradient(Color::rgb(0.20, 0.10, 0.35), Color::rgb(0.65, 0.30, 0.95), GradientDirection::Horizontal)
                                    .thumb_size(22.0, 22.0)
                                    .thumb_gradient(Color::rgb(0.75, 0.40, 1.0), Color::rgb(0.95, 0.60, 1.0), GradientDirection::Vertical)
                                    .thumb_shadow(Color::rgba(0.70, 0.30, 1.0, 0.50), 0.0, 3.0, 10.0)
                                    .show();
                            });

                            // Elongated Capsule Thumb (10x24)
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.text("Elongated Capsule Knob (Fader Style)").size(13.0).color(Color::rgb(0.75, 0.80, 0.90)).show();

                                ui.slider(&mut brightness, "capsule_slider")
                                    .range(0.0, 1.0)
                                    .fill_x()
                                    .track_height(4.0)
                                    .track_radius_all(2.0)
                                    .track_color(Color::rgb(0.14, 0.16, 0.22))
                                    .thumb_size(12.0, 24.0)
                                    .thumb_radius_all(6.0)
                                    .thumb_color(Color::rgb(0.40, 0.85, 0.90))
                                    .thumb_shadow(Color::rgba(0.30, 0.80, 0.90, 0.40), 0.0, 2.0, 6.0)
                                    .show();
                            });
                        });

                    // ── SECTION 5: CONTAINED SLIDER WITH SIBLING HOVER & ACTIVE ──
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(18.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("5. Contained Surface & Sibling Hover States (.hover(...))")
                                .color(Color::rgb(0.35, 0.85, 0.75))
                                .show();

                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.text("Enclosed slider card with interactive hover/active surface transitions")
                                    .size(13.0)
                                    .color(Color::rgb(0.75, 0.80, 0.90))
                                    .show();

                                ui.slider(&mut volume, "contained_slider")
                                    .range(0.0, 1.0)
                                    .fill_x()
                                    .bg(Color::rgb(0.12, 0.14, 0.20))
                                    .border((Color::rgb(0.22, 0.26, 0.36), 1.0))
                                    .radius_all(8.0)
                                    .padding_x(14.0)
                                    .padding_y(12.0)
                                    .hover(Style::bg(Color::rgb(0.16, 0.19, 0.28)))
                                    .active(Style::bg(Color::rgb(0.18, 0.22, 0.32)))
                                    .track_height(6.0)
                                    .track_radius_all(3.0)
                                    .track_color(Color::rgb(0.22, 0.26, 0.36))
                                    .thumb_size(18.0, 18.0)
                                    .thumb_radius_all(9.0)
                                    .thumb_color(Color::rgb(0.35, 0.85, 0.75))
                                    .thumb_shadow(Color::rgba(0.20, 0.75, 0.65, 0.40), 0.0, 2.0, 6.0)
                                    .show();
                            });
                        });
                });
        })
        .run();
}
