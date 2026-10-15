use zenthra::prelude::*;

fn main() {
    let mut card_click_counter = 0;
    let mut active_card = 1;

    App::new()
        .title("Zenthra - Card Architecture & Styling Showcase")
        .size(1080, 950)
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
                    ui.card()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(12.0)
                        .padding_all(20.0)
                        .column()
                        .gap(6.0)
                        .show(|ui: &mut Ui| {
                            ui.text("Card Widget Architecture")
                                .size(24.0)
                                .font_weight(zenthra_text::prelude::FontWeight::Bold)
                                .color(Color::WHITE)
                                .show();

                            ui.text("An elevated surface container widget featuring unified BoxStyle methods, hover animations, interactive states, and backdrop filters.")
                                .size(13.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });

                    // ── 1. BASIC INFORMATION CARD ─────────────────────────────
                    ui.text("1. Basic Information Card")
                        .size(16.0)
                        .font_weight(zenthra_text::prelude::FontWeight::Bold)
                        .color(Color::WHITE)
                        .show();

                    ui.card()
                        .fill_x()
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.card_header("Cloud Node Alpha", "Status: Online and Healthy", |ui| {
                                ui.text("US-EAST").size(12.0).color(Color::rgb(0.3, 0.85, 0.5)).show();
                            });

                            ui.container()
                                .row()
                                .gap(24.0)
                                .show(|ui: &mut Ui| {
                                    ui.container().column().gap(4.0).show(|ui| {
                                        ui.text("CPU Cores").size(11.0).color(Color::rgb(0.5, 0.55, 0.65)).show();
                                        ui.text("32 Cores").size(14.0).color(Color::WHITE).show();
                                    });
                                    ui.container().column().gap(4.0).show(|ui| {
                                        ui.text("Memory").size(11.0).color(Color::rgb(0.5, 0.55, 0.65)).show();
                                        ui.text("128 GB ECC").size(14.0).color(Color::WHITE).show();
                                    });
                                    ui.container().column().gap(4.0).show(|ui| {
                                        ui.text("Network").size(11.0).color(Color::rgb(0.5, 0.55, 0.65)).show();
                                        ui.text("10 Gbps SFP+").size(14.0).color(Color::WHITE).show();
                                    });
                                });
                        });

                    // ── 2. INTERACTIVE CLICKABLE CARDS ────────────────────────
                    ui.text("2. Interactive Clickable Cards (Response & Hover Scale)")
                        .size(16.0)
                        .font_weight(zenthra_text::prelude::FontWeight::Bold)
                        .color(Color::WHITE)
                        .show();

                    ui.container()
                        .row()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            // Interactive Card 1
                            let resp1 = ui.card()
                                .id("click_card_1")
                                .width(280.0)
                                .height(130.0)
                                .padding_all(16.0)
                                .radius_all(10.0)
                                .bg(if active_card == 1 { Color::rgb(0.12, 0.18, 0.30) } else { Color::rgb(0.09, 0.11, 0.15) })
                                .border(if active_card == 1 {
                                    (Color::rgb(0.35, 0.65, 1.0), 1.5)
                                } else {
                                    (Color::rgb(0.18, 0.22, 0.30), 1.0)
                                })
                                .hover([
                                    Style::bg(Color::rgb(0.14, 0.20, 0.34)),
                                    Transform::scale(1.03),
                                    Cursor::Pointer,
                                ])
                                .column()
                                .gap(8.0)
                                .show(|ui: &mut Ui| {
                                    ui.text("Standard Tier")
                                        .size(15.0)
                                        .font_weight(zenthra_text::prelude::FontWeight::Bold)
                                        .color(Color::WHITE)
                                        .show();
                                    ui.text("$29 / month - Essential compute resources for light workloads.")
                                        .size(11.0)
                                        .wrap(true)
                                        .color(Color::rgb(0.65, 0.70, 0.80))
                                        .show();
                                });

                            if resp1.clicked {
                                active_card = 1;
                                card_click_counter += 1;
                            }

                            // Interactive Card 2
                            let resp2 = ui.card()
                                .id("click_card_2")
                                .width(280.0)
                                .height(130.0)
                                .padding_all(16.0)
                                .radius_all(10.0)
                                .bg(if active_card == 2 { Color::rgb(0.12, 0.18, 0.30) } else { Color::rgb(0.09, 0.11, 0.15) })
                                .border(if active_card == 2 {
                                    (Color::rgb(0.35, 0.65, 1.0), 1.5)
                                } else {
                                    (Color::rgb(0.18, 0.22, 0.30), 1.0)
                                })
                                .hover([
                                    Style::bg(Color::rgb(0.14, 0.20, 0.34)),
                                    Transform::scale(1.03),
                                    Cursor::Pointer,
                                ])
                                .column()
                                .gap(8.0)
                                .show(|ui: &mut Ui| {
                                    ui.text("Pro Cluster")
                                        .size(15.0)
                                        .font_weight(zenthra_text::prelude::FontWeight::Bold)
                                        .color(Color::WHITE)
                                        .show();
                                    ui.text("$99 / month - Dedicated high-frequency instances with NVMe.")
                                        .size(11.0)
                                        .wrap(true)
                                        .color(Color::rgb(0.65, 0.70, 0.80))
                                        .show();
                                });

                            if resp2.clicked {
                                active_card = 2;
                                card_click_counter += 1;
                            }
                        });

                    ui.text(&format!("Total card clicks registered: {} (Active tier: #{})", card_click_counter, active_card))
                        .size(12.0)
                        .color(Color::rgb(0.55, 0.60, 0.70))
                        .show();

                    // ── 3. VIBRANT GRADIENT & SHADOW CARDS ────────────────────
                    ui.text("3. Elevated Custom Box Styling (Gradients & Shadows)")
                        .size(16.0)
                        .font_weight(zenthra_text::prelude::FontWeight::Bold)
                        .color(Color::WHITE)
                        .show();

                    ui.card()
                        .fill_x()
                        .bg(Gradient::linear(
                            Direction::ToBottomRight,
                            [
                                Color::rgb(0.12, 0.14, 0.25),
                                Color::rgb(0.06, 0.08, 0.15),
                            ],
                        ))
                        .border((Color::rgb(0.35, 0.45, 0.85), 1.5))
                        .radius_all(14.0)
                        .shadow(Color::rgba(0.20, 0.35, 0.85, 0.25), 0.0, 8.0, 24.0)
                        .padding_all(22.0)
                        .column()
                        .gap(10.0)
                        .show(|ui: &mut Ui| {
                            ui.text("Custom BoxStyle Parity")
                                .size(16.0)
                                .font_weight(zenthra_text::prelude::FontWeight::Bold)
                                .color(Color::WHITE)
                                .show();

                            ui.text("CardBuilder implements Styleable and impl_box_style_methods!, giving it exact parity with Container and Panel without non-standard helper methods.")
                                .size(13.0)
                                .color(Color::rgb(0.75, 0.82, 0.95))
                                .show();
                        });

                    // ── 4. SIBLING ARCHITECTURE (STYLE, TRANSFORM, CURSOR, TOOLTIP, ANIMATION)
                    ui.text("4. Full Sibling Interaction Architecture")
                        .size(16.0)
                        .font_weight(zenthra_text::prelude::FontWeight::Bold)
                        .color(Color::WHITE)
                        .show();

                    ui.card()
                        .fill_x()
                        .bg(Color::rgb(0.08, 0.10, 0.14))
                        .border((Color::rgb(0.20, 0.24, 0.32), 1.0))
                        .radius_all(12.0)
                        .padding_all(20.0)
                        .hover([
                            Style::bg(Color::rgb(0.12, 0.16, 0.24)),
                            Style::border((Color::rgb(0.40, 0.60, 1.0), 1.5)),
                            Transform::scale(1.02),
                            Cursor::Pointer,
                            Tooltip::new("Interactive card demonstrating full sibling array dispatch"),
                            Animation::duration(150).ease_out(),
                        ])
                        .active([
                            Style::bg(Color::rgb(0.06, 0.08, 0.12)),
                            Transform::scale(0.98),
                        ])
                        .column()
                        .gap(8.0)
                        .show(|ui: &mut Ui| {
                            ui.text("Complete Sibling Array Dispatch")
                                .size(15.0)
                                .font_weight(zenthra_text::prelude::FontWeight::Bold)
                                .color(Color::WHITE)
                                .show();

                            ui.text("Hover over this card to see smooth scale, border highlight, custom cursor, and tooltip simultaneously. Press mouse to see active state.")
                                .size(12.0)
                                .color(Color::rgb(0.65, 0.72, 0.85))
                                .show();
                        });
                });
        })
        .run();
}
