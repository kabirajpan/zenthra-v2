use zenthra::prelude::*;

fn main() {
    // ── Checkbox States ────────────────────────────────────────────────────────
    let mut auto_save = true;
    let mut format_on_save = false;
    let mut lint_on_type = true;

    let mut emerald_glow = true;
    let mut violet_neon = false;
    let mut crimson_alert = true;

    let mut terms_agreed = false;
    let mut newsletter = true;
    let mut telemetry = false;

    let mut compact_opt1 = true;
    let mut compact_opt2 = false;
    let mut compact_opt3 = true;

    let mut circular_cb = true;
    let mut square_cb = false;
    let mut custom_styled = true;

    let mut event_log = String::from("Interactive checkbox events will appear here.");

    App::new()
        .title("Zenthra - Checkbox API Showcase")
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
                            ui.h1("Checkbox API & Unified Styling Showcase")
                                .color(Color::WHITE)
                                .show();

                            ui.text("Demonstrating immediate-mode boolean binding, checkmark glyph animations, active label transitions, neon glow, custom corner radii, and sizing.")
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
                            ui.text(&format!("Auto-Save: {}", if auto_save { "ENABLED" } else { "DISABLED" }))
                                .size(14.0)
                                .font_weight(FontWeight::Bold)
                                .color(if auto_save { Color::rgb(0.20, 0.85, 0.50) } else { Color::rgb(0.50, 0.55, 0.65) })
                                .show();

                            ui.text(&format!("Terms: {}", if terms_agreed { "AGREED" } else { "PENDING" }))
                                .size(14.0)
                                .font_weight(FontWeight::Bold)
                                .color(if terms_agreed { Color::rgb(0.35, 0.75, 1.0) } else { Color::rgb(0.95, 0.40, 0.40) })
                                .show();

                            ui.text(&format!("Event: {}", event_log))
                                .size(13.0)
                                .color(Color::rgb(0.75, 0.80, 0.90))
                                .show();
                        });

                    // ── SECTION 1: STANDARD SIZES & BASIC BINDING ──────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("1. Standard Sized Checkboxes & Labels")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.row()
                                .fill_x()
                                .gap(32.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    // Compact 14px
                                    ui.checkbox(&mut auto_save, "Compact (14px)")
                                        .box_size(14.0)
                                        .size(13.0)
                                        .gap(6.0)
                                        .show();

                                    // Default 18px
                                    let resp = ui.checkbox(&mut format_on_save, "Standard (18px)")
                                        .size(14.0)
                                        .show();
                                    if resp.changed {
                                        event_log = format!("Format on Save changed to {}", format_on_save);
                                    }

                                    // Large 24px
                                    ui.checkbox(&mut lint_on_type, "Large Box (24px)")
                                        .box_size(24.0)
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
                                .gap(32.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    // Emerald Glow
                                    ui.checkbox(&mut emerald_glow, "Emerald Glow")
                                        .check_bg(Color::rgb(0.10, 0.80, 0.50))
                                        .glow(true)
                                        .scaling(1.08, 0.95)
                                        .active(Text::color(Color::rgb(0.20, 0.90, 0.60)))
                                        .show();

                                    // Neon Violet
                                    ui.checkbox(&mut violet_neon, "Violet Neon")
                                        .check_bg(Color::rgb(0.65, 0.35, 1.0))
                                        .check_color(Color::rgb(0.95, 0.90, 1.0))
                                        .glow(true)
                                        .scaling(1.08, 0.95)
                                        .active(Text::color(Color::rgb(0.75, 0.50, 1.0)))
                                        .show();

                                    // Crimson Danger
                                    ui.checkbox(&mut crimson_alert, "Crimson Alert")
                                        .check_bg(Color::rgb(0.95, 0.25, 0.35))
                                        .glow(true)
                                        .active(Text::color(Color::rgb(1.0, 0.40, 0.50)))
                                        .show();
                                });
                        });

                    // ── SECTION 3: CORNER RADII & CUSTOM CHECK SHAPES ──────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("3. Shapes & Radii (Circle, Square, Rounded)")
                                .color(Color::rgb(0.70, 0.45, 1.0))
                                .show();

                            ui.row()
                                .fill_x()
                                .gap(32.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    // Circular checkbox (pill radius)
                                    ui.checkbox(&mut circular_cb, "Circular Checkbox")
                                        .box_size(20.0)
                                        .radius_all(10.0)
                                        .check_bg(Color::rgb(0.35, 0.65, 1.0))
                                        .show();

                                    // Sharp Square (0.0 radius)
                                    ui.checkbox(&mut square_cb, "Sharp Square (0px)")
                                        .box_size(18.0)
                                        .radius_all(0.0)
                                        .check_bg(Color::rgb(0.95, 0.60, 0.20))
                                        .show();

                                    // Custom Rounded Border
                                    ui.checkbox(&mut custom_styled, "Smooth 6px Radius")
                                        .box_size(20.0)
                                        .radius_all(6.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.19))
                                        .border((Color::rgb(0.30, 0.35, 0.45), 1.5))
                                        .check_bg(Color::rgb(0.85, 0.45, 0.90))
                                        .show();
                                });
                        });

                    // ── SECTION 4: VERTICAL LIST / FORM GROUP ──────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(14.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("4. Vertical Preferences List with Active Label Color")
                                .color(Color::rgb(0.95, 0.65, 0.25))
                                .show();

                            ui.checkbox(&mut terms_agreed, "I agree to the Terms of Service & Privacy Policy")
                                .active(Text::color(Color::rgb(0.35, 0.75, 1.0)))
                                .font_weight(FontWeight::Medium)
                                .on_change(|checked| {
                                    println!("Terms of Service toggled: {}", checked);
                                })
                                .show();

                            ui.checkbox(&mut newsletter, "Receive monthly engineering newsletter and product updates")
                                .active(Text::color(Color::rgb(0.40, 0.85, 0.90)))
                                .show();

                            ui.checkbox(&mut telemetry, "Send anonymous crash diagnostics and performance reports")
                                .active(Text::color(Color::rgb(0.75, 0.80, 0.95)))
                                .show();
                        });

                    // ── SECTION 5: COMPACT DENSE SELECTION LIST ────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("5. Dense Settings Matrix")
                                .color(Color::rgb(0.40, 0.85, 0.90))
                                .show();

                            ui.row()
                                .fill_x()
                                .gap(28.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    ui.checkbox(&mut compact_opt1, "Show Line Numbers")
                                        .box_size(15.0)
                                        .size(12.0)
                                        .gap(6.0)
                                        .show();

                                    ui.checkbox(&mut compact_opt2, "Highlight Whitespace")
                                        .box_size(15.0)
                                        .size(12.0)
                                        .gap(6.0)
                                        .show();

                                    ui.checkbox(&mut compact_opt3, "Code Folding Controls")
                                        .box_size(15.0)
                                        .size(12.0)
                                        .gap(6.0)
                                        .show();
                                });
                        });
                });
        })
        .run();
}
