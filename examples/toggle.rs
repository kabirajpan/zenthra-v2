use zenthra::prelude::*;

fn main() {
    // ── Toggle States ──────────────────────────────────────────────────────────
    let mut wifi_enabled = true;
    let mut bluetooth_enabled = false;
    let mut airplane_mode = false;
    let mut dark_theme = true;
    let mut notifications = true;
    let mut dev_mode = false;
    let mut hardware_accel = true;
    let mut auto_save = true;
    let mut square_toggle = false;
    let mut rounded_custom = true;
    let mut power = true;
    let mut standalone = true;
    let mut disabled_on = true;
    let mut disabled_off = false;

    let mut event_log = String::from("Interactive toggle events will appear here.");

    App::new()
        .title("Zenthra - Toggle API Showcase")
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
                            ui.h1("Toggle API & Unified Styling Showcase")
                                .color(Color::WHITE)
                                .show();

                            ui.text("Demonstrating immediate-mode boolean binding, spring sliding animations, inner status labels, glow effects, custom thumb shapes, and event listeners.")
                                .size(14.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });

                    // ── STATUS BAR & LIVE EVENT FEEDBACK ──────────────────────
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
                            ui.text(&format!("Wi-Fi: {}", if wifi_enabled { "ON" } else { "OFF" }))
                                .size(14.0)
                                .font_weight(FontWeight::Bold)
                                .color(if wifi_enabled { Color::rgb(0.20, 0.85, 0.50) } else { Color::rgb(0.50, 0.55, 0.65) })
                                .show();

                            ui.text(&format!("Dark Theme: {}", if dark_theme { "ACTIVE" } else { "LIGHT" }))
                                .size(14.0)
                                .font_weight(FontWeight::Bold)
                                .color(if dark_theme { Color::rgb(0.40, 0.75, 1.0) } else { Color::rgb(0.95, 0.75, 0.25) })
                                .show();

                            ui.text(&format!("Event: {}", event_log))
                                .size(13.0)
                                .color(Color::rgb(0.75, 0.80, 0.90))
                                .show();
                        });

                    // ── SECTION 1: DEFAULT & SIZED TOGGLES ─────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("1. Standard Sized Toggles with Right Labels")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.row()
                                .fill_x()
                                .gap(32.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    // Compact 42x20
                                    ui.toggle(&mut bluetooth_enabled, "Compact (42x20)")
                                        .track_size(42.0, 20.0)
                                        .size(13.0)
                                        .show();

                                    // Default 48x24
                                    let resp = ui.toggle(&mut wifi_enabled, "Standard (48x24)")
                                        .size(14.0)
                                        .show();
                                    if resp.changed {
                                        event_log = format!("Wi-Fi changed to {}", wifi_enabled);
                                    }

                                    // Large 58x28
                                    ui.toggle(&mut dark_theme, "Large Switch (58x28)")
                                        .track_size(58.0, 28.0)
                                        .size(15.0)
                                        .active(Text::color(Color::rgb(0.40, 0.75, 1.0)))
                                        .show();
                                });
                        });

                    // ── SECTION 2: PALETTES & GLOW AURA ────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("2. Vibrant Themes, Glow Effects & Micro-Interactions")
                                .color(Color::rgb(0.10, 0.80, 0.50))
                                .show();

                            ui.row()
                                .fill_x()
                                .gap(28.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    // Emerald Glow
                                    ui.toggle(&mut notifications, "Emerald Glow")
                                        .on_color(Color::rgb(0.10, 0.80, 0.50))
                                        .off_color(Color::rgb(0.18, 0.20, 0.26))
                                        .glow(true)
                                        .scaling(1.05, 0.95)
                                        .active(Text::color(Color::rgb(0.20, 0.90, 0.60)))
                                        .show();

                                    // Neon Violet
                                    ui.toggle(&mut hardware_accel, "Violet Neon")
                                        .on_color(Color::rgb(0.65, 0.35, 1.0))
                                        .off_color(Color::rgb(0.18, 0.18, 0.24))
                                        .thumb_color(Color::rgb(0.95, 0.90, 1.0))
                                        .glow(true)
                                        .scaling(1.05, 0.95)
                                        .active(Text::color(Color::rgb(0.75, 0.50, 1.0)))
                                        .show();

                                    // Crimson Danger
                                    ui.toggle(&mut airplane_mode, "Crimson Alert")
                                        .on_color(Color::rgb(0.95, 0.25, 0.35))
                                        .off_color(Color::rgb(0.22, 0.18, 0.20))
                                        .glow(true)
                                        .active(Text::color(Color::rgb(1.0, 0.40, 0.50)))
                                        .show();
                                });
                        });

                    // ── SECTION 3: INNER STATUS LABELS ("ON" / "OFF") ───────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("3. Embedded Track Labels (ON / OFF)")
                                .color(Color::rgb(0.95, 0.65, 0.25))
                                .show();

                            ui.row()
                                .fill_x()
                                .gap(32.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    // Track with ON / OFF text
                                    ui.toggle(&mut auto_save, "Automatic Sync")
                                        .track_size(60.0, 26.0)
                                        .inner_labels("ON", "OFF")
                                        .inner_size(9.0)
                                        .on_color(Color::rgb(0.95, 0.60, 0.20))
                                        .off_color(Color::rgb(0.16, 0.18, 0.24))
                                        .show();

                                    // Track with Yes / No
                                    ui.toggle(&mut dev_mode, "Developer Options")
                                        .track_size(62.0, 26.0)
                                        .inner_labels("YES", "NO")
                                        .inner_size(9.0)
                                        .on_color(Color::rgb(0.30, 0.65, 1.0))
                                        .off_color(Color::rgb(0.16, 0.18, 0.24))
                                        .show();
                                });
                        });

                    // ── SECTION 4: SHAPES & THUMB CUSTOMIZATION ────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("4. Track Shapes, Radii & Custom Thumbs")
                                .color(Color::rgb(0.70, 0.45, 1.0))
                                .show();

                            ui.row()
                                .fill_x()
                                .gap(32.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    // Rectangular / Square toggle
                                    ui.toggle(&mut square_toggle, "Square Switch")
                                        .square()
                                        .track_size(52.0, 24.0)
                                        .thumb_radius(0.0)
                                        .on_color(Color::rgb(0.20, 0.60, 0.90))
                                        .show();

                                    // Custom rounded corners with elongated thumb
                                    ui.toggle(&mut rounded_custom, "Capsule Slider")
                                        .track_size(56.0, 24.0)
                                        .radius_all(6.0)
                                        .thumb_radius(4.0)
                                        .thumb_width(16.0)
                                        .on_color(Color::rgb(0.85, 0.45, 0.90))
                                        .show();
                                });
                        });

                    // ── SECTION 5: LABEL POSITIONING & EVENT CALLBACKS ─────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("5. Left-Side Labels & Event Callbacks")
                                .color(Color::rgb(0.40, 0.85, 0.90))
                                .show();

                            ui.row()
                                .fill_x()
                                .gap(40.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    // Left label side
                                    ui.toggle(&mut power, "Main Engine Power")
                                        .label_left()
                                        .label_gap(14.0)
                                        .font_weight(FontWeight::Medium)
                                        .on_color(Color::rgb(0.15, 0.80, 0.60))
                                        .on_change(|val| {
                                            println!("Main Engine Power changed: {}", val);
                                        })
                                        .show();

                                    // Standalone toggle (no label)
                                    ui.toggle(&mut standalone, None)
                                        .track_size(50.0, 26.0)
                                        .glow(true)
                                        .show();
                                });
                        });

                    // ── SECTION 6: DISABLED STATES ─────────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("6. Disabled States")
                                .color(Color::rgb(0.65, 0.70, 0.80))
                                .show();

                            ui.row()
                                .fill_x()
                                .gap(32.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    ui.toggle(&mut disabled_on, "Disabled (Active)")
                                        .disabled(true)
                                        .show();

                                    ui.toggle(&mut disabled_off, "Disabled (Inactive)")
                                        .disabled(true)
                                        .show();
                                });
                        });
                });
        })
        .run();
}
