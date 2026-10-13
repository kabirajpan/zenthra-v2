use zenthra::prelude::*;

fn main() {
    let mut click_count = 0;
    let mut last_action = String::from("No action taken yet.");

    App::new()
        .title("Zenthra - Button API Showcase")
        .size(1050, 900)
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
                            ui.h1("Button API & Styling Showcase")
                                .color(Color::WHITE)
                                .show();

                            ui.text("Demonstrating unified BoxStyle properties, interactive states (hover/press), gradients, borders, shadows, and click responses.")
                                .font_size(14.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });

                    // ── STATUS BAR & LIVE INTERACTION ──────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.10, 0.12, 0.17))
                        .border((Color::rgb(0.20, 0.25, 0.35), 1.0))
                        .radius_all(8.0)
                        .padding_all(16.0)
                        .row()
                        .gap(20.0)
                        .valign(Align::Center)
                        .show(|ui: &mut Ui| {
                            ui.text(&format!("Counter: {}", click_count))
                                .font_size(16.0)
                                .weight(FontWeight::Bold)
                                .color(Color::rgb(0.35, 0.75, 1.0))
                                .show();

                            ui.text(&format!("Last Action: {}", last_action))
                                .font_size(14.0)
                                .color(Color::rgb(0.75, 0.80, 0.90))
                                .show();

                            if ui.button("Reset Counter")
                                .bg(Color::rgb(0.18, 0.20, 0.26))
                                .hover_bg(Color::rgb(0.25, 0.28, 0.36))
                                .border((Color::rgb(0.30, 0.35, 0.45), 1.0))
                                .radius_all(6.0)
                                .padding(6.0, 14.0, 6.0, 14.0)
                                .font_size(13.0)
                                .show()
                                .clicked
                            {
                                click_count = 0;
                                last_action = String::from("Counter reset to 0");
                            }
                        });

                    // ── SECTION 1: BUTTON VARIANTS & SEMANTIC STYLES ─────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(14.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("1. Common Semantic Variants")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.row().gap(12.0).show(|ui: &mut Ui| {
                                // Primary Accent Button
                                if ui.button("Primary Accent")
                                    .bg(Color::rgb(0.15, 0.45, 0.95))
                                    .hover_bg(Color::rgb(0.25, 0.55, 1.0))
                                    .active_bg(Color::rgb(0.10, 0.35, 0.80))
                                    .radius_all(6.0)
                                    .padding(8.0, 18.0, 8.0, 18.0)
                                    .font_size(14.0)
                                    .weight(FontWeight::Bold)
                                    .show()
                                    .clicked
                                {
                                    click_count += 1;
                                    last_action = String::from("Clicked 'Primary Accent'");
                                }

                                // Secondary / Neutral
                                if ui.button("Secondary")
                                    .bg(Color::rgb(0.16, 0.19, 0.25))
                                    .hover_bg(Color::rgb(0.22, 0.26, 0.34))
                                    .active_bg(Color::rgb(0.12, 0.14, 0.19))
                                    .border((Color::rgb(0.26, 0.31, 0.40), 1.0))
                                    .radius_all(6.0)
                                    .padding(8.0, 18.0, 8.0, 18.0)
                                    .font_size(14.0)
                                    .show()
                                    .clicked
                                {
                                    click_count += 1;
                                    last_action = String::from("Clicked 'Secondary'");
                                }

                                // Success / Confirm
                                if ui.button("Confirm Changes")
                                    .bg(Color::rgb(0.12, 0.58, 0.34))
                                    .hover_bg(Color::rgb(0.16, 0.70, 0.40))
                                    .active_bg(Color::rgb(0.09, 0.48, 0.28))
                                    .radius_all(6.0)
                                    .padding(8.0, 18.0, 8.0, 18.0)
                                    .font_size(14.0)
                                    .show()
                                    .clicked
                                {
                                    click_count += 1;
                                    last_action = String::from("Clicked 'Confirm Changes'");
                                }

                                // Danger / Destructive
                                if ui.button("Delete Resource")
                                    .bg(Color::rgb(0.75, 0.18, 0.18))
                                    .hover_bg(Color::rgb(0.88, 0.22, 0.22))
                                    .active_bg(Color::rgb(0.60, 0.14, 0.14))
                                    .radius_all(6.0)
                                    .padding(8.0, 18.0, 8.0, 18.0)
                                    .font_size(14.0)
                                    .show()
                                    .clicked
                                {
                                    click_count += 1;
                                    last_action = String::from("Clicked 'Delete Resource'");
                                }

                                // Ghost / Transparent
                                if ui.button("Ghost Link")
                                    .bg(None)
                                    .hover_bg(Color::rgba(1.0, 1.0, 1.0, 0.08))
                                    .active_bg(Color::rgba(1.0, 1.0, 1.0, 0.04))
                                    .text_color(Color::rgb(0.70, 0.85, 1.0))
                                    .radius_all(6.0)
                                    .padding(8.0, 18.0, 8.0, 18.0)
                                    .font_size(14.0)
                                    .show()
                                    .clicked
                                {
                                    click_count += 1;
                                    last_action = String::from("Clicked 'Ghost Link'");
                                }
                            });
                        });

                    // ── SECTION 2: GRADIENTS & DROP SHADOWS ───────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(14.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("2. Gradients & Drop Shadows")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.row().gap(16.0).show(|ui: &mut Ui| {
                                // Purple/Cyan Neon Gradient
                                let neon_grad = Gradient::linear(
                                    Direction::ToRight,
                                    [
                                        (0.0, Color::rgb(0.65, 0.15, 0.90)),
                                        (1.0, Color::rgb(0.10, 0.70, 0.95)),
                                    ],
                                );
                                let neon_hover_grad = Gradient::linear(
                                    Direction::ToRight,
                                    [
                                        (0.0, Color::rgb(0.75, 0.25, 1.00)),
                                        (1.0, Color::rgb(0.20, 0.80, 1.00)),
                                    ],
                                );

                                if ui.button("Vibrant Gradient")
                                    .bg(neon_grad)
                                    .hover_bg(neon_hover_grad)
                                    .radius_all(8.0)
                                    .padding(10.0, 24.0, 10.0, 24.0)
                                    .font_size(15.0)
                                    .weight(FontWeight::Bold)
                                    .shadow(Color::rgb(0.50, 0.15, 0.85), 0.0, 4.0, 16.0)
                                    .shadow_opacity(0.40)
                                    .show()
                                    .clicked
                                {
                                    click_count += 1;
                                    last_action = String::from("Clicked 'Vibrant Gradient'");
                                }

                                // Amber/Orange Sunset Gradient
                                let sunset_grad = Gradient::linear(
                                    Direction::ToBottomRight,
                                    [
                                        (0.0, Color::rgb(0.95, 0.55, 0.10)),
                                        (1.0, Color::rgb(0.85, 0.20, 0.20)),
                                    ],
                                );

                                if ui.button("Sunset Glow")
                                    .bg(sunset_grad)
                                    .radius_all(20.0) // Pill shape
                                    .padding(10.0, 24.0, 10.0, 24.0)
                                    .font_size(15.0)
                                    .weight(FontWeight::Bold)
                                    .shadow(Color::rgb(0.85, 0.30, 0.10), 0.0, 4.0, 14.0)
                                    .shadow_opacity(0.45)
                                    .show()
                                    .clicked
                                {
                                    click_count += 1;
                                    last_action = String::from("Clicked 'Sunset Glow'");
                                }
                            });
                        });

                    // ── SECTION 3: INTERACTIVE BORDERS & MICRO-INTERACTIONS ───
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(14.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("3. State-Driven Borders & Hover Scaling")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.row().gap(16.0).show(|ui: &mut Ui| {
                                // Outline button with interactive border change
                                if ui.button("Outline Accent")
                                    .bg(None)
                                    .border((Color::rgb(0.35, 0.65, 1.0), 1.5))
                                    .hover_border((Color::rgb(0.60, 0.85, 1.0), 2.0))
                                    .active_border((Color::rgb(0.20, 0.50, 0.90), 2.0))
                                    .hover_bg(Color::rgba(0.35, 0.65, 1.0, 0.12))
                                    .radius_all(6.0)
                                    .padding(8.0, 20.0, 8.0, 20.0)
                                    .text_color(Color::rgb(0.85, 0.92, 1.0))
                                    .show()
                                    .clicked
                                {
                                    click_count += 1;
                                    last_action = String::from("Clicked 'Outline Accent'");
                                }

                                // Button with subtle hover scale micro-interaction
                                if ui.button("Hover Scale 1.05x")
                                    .bg(Color::rgb(0.22, 0.26, 0.34))
                                    .hover_bg(Color::rgb(0.28, 0.34, 0.44))
                                    .hover_scale(1.05)
                                    .radius_all(6.0)
                                    .padding(8.0, 20.0, 8.0, 20.0)
                                    .show()
                                    .clicked
                                {
                                    click_count += 1;
                                    last_action = String::from("Clicked 'Hover Scale 1.05x'");
                                }
                            });
                        });

                    // ── SECTION 4: SIZING, PILL SHAPES & ALIGNMENT ────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(14.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("4. Explicit Sizing & Alignment")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.row().gap(12.0).show(|ui: &mut Ui| {
                                // Fixed square button (e.g. icon/tool button)
                                if ui.button("✕")
                                    .width(36.0)
                                    .height(36.0)
                                    .bg(Color::rgb(0.18, 0.20, 0.26))
                                    .hover_bg(Color::rgb(0.75, 0.20, 0.20))
                                    .radius_all(6.0)
                                    .font_size(14.0)
                                    .show()
                                    .clicked
                                {
                                    click_count += 1;
                                    last_action = String::from("Clicked Close [✕] Button");
                                }

                                // Start aligned fixed button
                                if ui.button("Left Aligned")
                                    .width(180.0)
                                    .height(36.0)
                                    .align(Align::Left)
                                    .padding_x(12.0)
                                    .bg(Color::rgb(0.16, 0.19, 0.25))
                                    .hover_bg(Color::rgb(0.22, 0.26, 0.34))
                                    .radius_all(6.0)
                                    .show()
                                    .clicked
                                {
                                    click_count += 1;
                                    last_action = String::from("Clicked 'Left Aligned'");
                                }

                                // End aligned fixed button
                                if ui.button("Right Aligned")
                                    .width(180.0)
                                    .height(36.0)
                                    .align(Align::Right)
                                    .padding_x(12.0)
                                    .bg(Color::rgb(0.16, 0.19, 0.25))
                                    .hover_bg(Color::rgb(0.22, 0.26, 0.34))
                                    .radius_all(6.0)
                                    .show()
                                    .clicked
                                {
                                    click_count += 1;
                                    last_action = String::from("Clicked 'Right Aligned'");
                                }
                            });

                            // Full width block button
                            if ui.button("Full Width Block Button (fill_x)")
                                .fill_x()
                                .height(42.0)
                                .bg(Color::rgb(0.14, 0.18, 0.25))
                                .hover_bg(Color::rgb(0.20, 0.26, 0.36))
                                .border((Color::rgb(0.25, 0.32, 0.45), 1.0))
                                .radius_all(8.0)
                                .font_size(15.0)
                                .weight(FontWeight::Bold)
                                .show()
                                .clicked
                            {
                                click_count += 1;
                                last_action = String::from("Clicked 'Full Width Block Button'");
                            }
                        });
                });
        })
        .run();
}
