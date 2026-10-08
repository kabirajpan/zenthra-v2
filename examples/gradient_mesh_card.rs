use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Unified Gradient & Mesh Card Showcase")
        .size(960, 620)
        .with_ui(|ui| {
            ui.container()
                .fill()
                .bg(Color::rgb(0.04, 0.04, 0.06))
                .padding_all(28.0)
                .column()
                .gap(20.0)
                .show(|ui| {
                    // Header
                    ui.h2("Zenthra Unified Gradient System")
                        .color(Color::WHITE)
                        .show();
                    ui.text("Hardware-accelerated Linear, Radial, and Freeform Mesh Gradients via a single unified .bg(...) API.")
                        .size(13.0)
                        .color(Color::rgb(0.55, 0.60, 0.72))
                        .show();

                    ui.spacing(8.0);

                    // Row with cards
                    ui.container()
                        .full_width()
                        .row()
                        .gap(24.0)
                        .show(|ui| {
                            // Card 1: 4-Corner Mesh Gradient (Matching Reference Image)
                            ui.container()
                                .width(420.0)
                                .height(260.0)
                                .radius_all(20.0)
                                .bg(Gradient::corners(
                                    Color::rgb(0.98, 0.88, 0.15), // Top-Left:     Yellow
                                    Color::rgb(0.15, 0.78, 0.42), // Top-Right:    Green
                                    Color::rgb(0.92, 0.22, 0.22), // Bottom-Left:  Red
                                    Color::rgb(0.15, 0.45, 0.95), // Bottom-Right: Blue
                                ))
                                .border(Color::rgba(1.0, 1.0, 1.0, 0.20), 1.0)
                                .shadow(Color::rgba(0.0, 0.0, 0.0, 0.4), 0.0, 8.0, 20.0)
                                .padding_all(20.0)
                                .column()
                                .halign(Align::SpaceBetween)
                                .show(|ui| {
                                    ui.container().row().halign(Align::SpaceBetween).show(|ui| {
                                        ui.text("4-Corner Mesh Card")
                                            .size(18.0)
                                            .weight(FontWeight::Bold)
                                            .color(Color::WHITE)
                                            .show();
                                        ui.container()
                                            .padding(3.0, 8.0, 3.0, 8.0)
                                            .radius_all(10.0)
                                            .bg(Color::rgba(0.0, 0.0, 0.0, 0.35))
                                            .show(|ui| {
                                                ui.text("GPU Mesh").size(11.0).color(Color::WHITE).show();
                                            });
                                    });

                                    ui.text("Apple/Stripe Bilinear Mesh Gradient calculated natively on WebGPU.")
                                        .size(12.0)
                                        .color(Color::rgba(1.0, 1.0, 1.0, 0.85))
                                        .show();
                                });

                            // Card 2: Linear Directional & Radial Showcase
                            ui.container()
                                .width(420.0)
                                .height(260.0)
                                .column()
                                .gap(16.0)
                                .show(|ui| {
                                    // 2A: Linear Left to Right
                                    ui.container()
                                        .full_width()
                                        .height(115.0)
                                        .radius_all(14.0)
                                        .bg(Gradient::linear(
                                            Direction::ToRight,
                                            [Color::rgb(0.15, 0.45, 1.0), Color::rgb(0.95, 0.25, 0.40)],
                                        ))
                                        .padding_all(16.0)
                                        .align(Align::Center)
                                        .show(|ui| {
                                            ui.text("Linear: ToRight (Blue → Red)")
                                                .size(15.0)
                                                .weight(FontWeight::Bold)
                                                .color(Color::WHITE)
                                                .show();
                                        });

                                    // 2B: Radial Left-Edge Spotlight
                                    ui.container()
                                        .full_width()
                                        .height(115.0)
                                        .radius_all(14.0)
                                        .bg(Gradient::radial(
                                            Align::Left,
                                            [Color::rgb(0.95, 0.35, 0.20), Color::rgb(0.12, 0.15, 0.30)],
                                        ))
                                        .padding_all(16.0)
                                        .align(Align::Center)
                                        .show(|ui| {
                                            ui.text("Radial: Left-Edge Spotlight")
                                                .size(15.0)
                                                .weight(FontWeight::Bold)
                                                .color(Color::WHITE)
                                                .show();
                                        });
                                });
                        });
                });
        })
        .run();
}
