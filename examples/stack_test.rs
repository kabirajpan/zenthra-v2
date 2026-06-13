// examples/stack_test.rs

use zenthra::prelude::*;

fn main() {
    let mut hover_count = 0;
    let mut badge_clicked = false;

    App::new()
        .title("Zenthra Stack Layout Demonstration")
        .size(900, 700)
        .with_ui(move |ui: &mut Ui| {
            ui.container()
                .full_width()
                .full_height()
                .bg(Color::rgb(0.06, 0.06, 0.08))
                .padding_all(24.0)
                .gap(20.0)
                .show(|ui: &mut Ui| {
                    // Header
                    ui.text("Zenthra Stack Layout Suite")
                        .size(28.0)
                        .weight(FontWeight::Bold)
                        .color(Color::WHITE)
                        .show();

                    ui.text("Demonstrating overlapping widgets, layer ordering, and alignment within stack bounds.")
                        .size(14.0)
                        .color(Color::rgb(0.5, 0.5, 0.6))
                        .show();

                    ui.spacing(15.0);

                    // Row showing different stacked elements
                    ui.container()
                        .full_width()
                        .row()
                        .gap(25.0)
                        .show(|ui: &mut Ui| {
                            // Stack 1: Profile card with corner badge overlay
                            ui.stack()
                                .width(300.0)
                                .height(380.0)
                                .bg(Color::rgb(0.12, 0.12, 0.15))
                                .border(Color::rgb(0.2, 0.2, 0.25), 1.0)
                                .radius_all(12.0)
                                .padding_all(16.0)
                                .shadow(Color::rgba(0.0, 0.0, 0.0, 0.4), 0.0, 6.0, 15.0)
                                .show(|ui: &mut Ui| {
                                    // Layer 1 (Bottom): Profile details aligned column
                                    ui.container()
                                        .full_width()
                                        .full_height()
                                        .column()
                                        .valign(Align::Center)
                                        .halign(Align::Center)
                                        .show(|ui: &mut Ui| {
                                            ui.text("Olivia Vance")
                                                .size(20.0)
                                                .weight(FontWeight::Bold)
                                                .color(Color::WHITE)
                                                .show();
                                            ui.spacing(4.0);
                                            ui.text("Senior DevOps Engineer")
                                                .size(13.0)
                                                .color(Color::rgb(0.6, 0.6, 0.7))
                                                .show();

                                            ui.spacing(25.0);

                                            if ui.button("View Profile").show().hovered {
                                                hover_count += 1;
                                            }
                                        });

                                    // Layer 2 (Top-Right): "PRO" Badge
                                    ui.stack()
                                        .full_width()
                                        .full_height()
                                        .halign(Align::Right)
                                        .valign(Align::Top)
                                        .show(|ui: &mut Ui| {
                                            let badge_color = if badge_clicked {
                                                Color::rgb(0.9, 0.6, 0.1) // Gold
                                            } else {
                                                Color::rgb(0.3, 0.5, 1.0) // Blue
                                            };

                                            let resp = ui.container()
                                                .bg(badge_color)
                                                .radius_all(6.0)
                                                .padding(4.0, 10.0, 4.0, 10.0)
                                                .show(|ui: &mut Ui| {
                                                    ui.text("PRO")
                                                        .size(10.0)
                                                        .weight(FontWeight::Bold)
                                                        .color(Color::WHITE)
                                                        .show();
                                                });

                                            if resp.clicked {
                                                badge_clicked = !badge_clicked;
                                            }
                                        });
                                });

                            // Stack 2: Overlay controls with custom alignments
                            ui.stack()
                                .width(320.0)
                                .height(380.0)
                                .bg(Color::rgb(0.1, 0.1, 0.12))
                                .border(Color::rgb(0.22, 0.22, 0.28), 1.0)
                                .radius_all(16.0)
                                .padding_all(12.0)
                                .show(|ui: &mut Ui| {
                                    // Background design lines
                                    ui.container()
                                        .full_width()
                                        .full_height()
                                        .bg(Color::rgba(0.2, 0.2, 0.25, 0.05))
                                        .show(|_| {});

                                    // Top-Left Alignment
                                    ui.stack()
                                        .full_width()
                                        .full_height()
                                        .halign(Align::Left)
                                        .valign(Align::Top)
                                        .show(|ui: &mut Ui| {
                                            ui.text("Top Left").size(12.0).color(Color::rgb(0.5, 0.5, 0.6)).show();
                                        });

                                    // Center Alignment
                                    ui.stack()
                                        .full_width()
                                        .full_height()
                                        .halign(Align::Center)
                                        .valign(Align::Center)
                                        .show(|ui: &mut Ui| {
                                            ui.container()
                                                .bg(Color::rgb(0.15, 0.15, 0.18))
                                                .border(Color::rgb(0.3, 0.3, 0.35), 1.0)
                                                .radius_all(20.0)
                                                .padding(10.0, 20.0, 10.0, 20.0)
                                                .show(|ui: &mut Ui| {
                                                    ui.text("Centered Content")
                                                        .size(14.0)
                                                        .color(Color::WHITE)
                                                        .show();
                                                });
                                        });

                                    // Bottom-Right Alignment
                                    ui.stack()
                                        .full_width()
                                        .full_height()
                                        .halign(Align::Right)
                                        .valign(Align::Bottom)
                                        .show(|ui: &mut Ui| {
                                            ui.text(&format!("Hover count: {}", hover_count))
                                                .size(11.0)
                                                .color(Color::rgb(0.4, 0.8, 0.4))
                                                .show();
                                        });
                                });
                        });
                });
        })
        .run();
}
