use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Container API Test")
        .size(1200, 900)
        .with_ui(|ui: &mut Ui| {
            // Root Container
            ui.container()
                .fill()
                .bg(Color::rgb(0.02, 0.02, 0.03))
                .padding(20.0, 20.0, 20.0, 20.0)
                .scroll_y(true)
                .gap(20.0)
                .show(|ui: &mut Ui| {
                    ui.h1("New Container API Test").color(Color::WHITE).show();

                    // --- SECTION 1: RADIUS TEST ---
                    ui.container()
                        .full_width()
                        .bg(Color::rgb(0.05, 0.05, 0.1))
                        .padding(15.0, 15.0, 15.0, 15.0)
                        .radius(15.0, 15.0, 30.0, 15.0)
                        .gap(15.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("1. Radius API (Individual & Group)").color(Color::rgb(0.6, 0.7, 1.0)).show();
                            
                            // Individual Corners
                            ui.row().gap(15.0).wrap(Wrap::Wrap).show(|ui: &mut Ui| {
                                // TL
                                ui.container().width(80.0).height(60.0).bg(Color::rgb(0.2, 0.4, 0.8)).radius_top_left(25.0).align(Align::Center).show(|ui: &mut Ui| { ui.text("TL").color(Color::WHITE).show(); });
                                // TR
                                ui.container().width(80.0).height(60.0).bg(Color::rgb(0.2, 0.4, 0.8)).radius_top_right(25.0).align(Align::Center).show(|ui: &mut Ui| { ui.text("TR").color(Color::WHITE).show(); });
                                // BR
                                ui.container().width(80.0).height(60.0).bg(Color::rgb(0.2, 0.4, 0.8)).radius_bottom_right(25.0).align(Align::Center).show(|ui: &mut Ui| { ui.text("BR").color(Color::WHITE).show(); });
                                // BL
                                ui.container().width(80.0).height(60.0).bg(Color::rgb(0.2, 0.4, 0.8)).radius_bottom_left(25.0).align(Align::Center).show(|ui: &mut Ui| { ui.text("BL").color(Color::WHITE).show(); });
                            });

                            // Group Radii
                            ui.row().gap(15.0).wrap(Wrap::Wrap).show(|ui: &mut Ui| {
                                // Top
                                ui.container().width(100.0).height(60.0).bg(Color::rgb(0.3, 0.5, 0.9)).radius_top(20.0).align(Align::Center).show(|ui: &mut Ui| { ui.text("Top").color(Color::WHITE).show(); });
                                // Bottom
                                ui.container().width(100.0).height(60.0).bg(Color::rgb(0.3, 0.5, 0.9)).radius_bottom(20.0).align(Align::Center).show(|ui: &mut Ui| { ui.text("Bottom").color(Color::WHITE).show(); });
                                // Left
                                ui.container().width(100.0).height(60.0).bg(Color::rgb(0.3, 0.5, 0.9)).radius_left(20.0).align(Align::Center).show(|ui: &mut Ui| { ui.text("Left").color(Color::WHITE).show(); });
                                // Right
                                ui.container().width(100.0).height(60.0).bg(Color::rgb(0.3, 0.5, 0.9)).radius_right(20.0).align(Align::Center).show(|ui: &mut Ui| { ui.text("Right").color(Color::WHITE).show(); });
                                // All
                                ui.container().width(100.0).height(60.0).bg(Color::rgb(0.3, 0.5, 0.9)).radius_all(20.0).align(Align::Center).show(|ui: &mut Ui| { ui.text("All").color(Color::WHITE).show(); });
                            });

                            // Shorthand
                            ui.row().gap(15.0).show(|ui: &mut Ui| {
                                ui.container()
                                    .width(200.0).height(60.0)
                                    .bg(Color::rgb(0.4, 0.6, 1.0))
                                    .radius(30.0, 0.0, 30.0, 0.0)
                                    .align(Align::Center)
                                    .show(|ui: &mut Ui| { ui.text("radius(30, 0, 30, 0)").size(12.0).color(Color::WHITE).show(); });
                            });
                        });

                    // --- SECTION 2: INTERACTIVE STATES ---
                    ui.container()
                        .full_width()
                        .bg(Color::rgb(0.05, 0.08, 0.05))
                        .padding(15.0, 15.0, 15.0, 15.0)
                        .radius_all(8.0)
                        .gap(10.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("2. Interactive States (Hover & Active)").color(Color::rgb(0.6, 1.0, 0.7)).show();
                            
                            ui.row().gap(20.0).show(|ui: &mut Ui| {
                                // Button-like Container
                                ui.container()
                                    .width(200.0).height(60.0)
                                    .bg(Color::rgb(0.1, 0.2, 0.1))
                                    .border(Color::rgb(0.2, 0.4, 0.2), 1.0)
                                    .radius_all(8.0)
                                    .align(Align::Center)
                                    .hover_bg(Color::rgb(0.15, 0.3, 0.15))
                                    .active_bg(Color::rgb(0.2, 0.5, 0.2))
                                    .active_scale(0.95)
                                    .show(|ui: &mut Ui| {
                                        ui.text("Press Me").color(Color::WHITE).show();
                                    });

                                // Border Active State
                                ui.container()
                                    .width(200.0).height(60.0)
                                    .bg(Color::rgb(0.1, 0.1, 0.1))
                                    .border(Color::rgb(0.3, 0.3, 0.3), 1.0)
                                    .radius_all(8.0)
                                    .align(Align::Center)
                                    .hover_border(Color::rgb(0.5, 0.5, 1.0), 2.0)
                                    .active_border(Color::rgb(0.0, 0.8, 1.0), 3.0)
                                    .show(|ui: &mut Ui| {
                                        ui.text("Border Feedback").color(Color::WHITE).show();
                                    });
                            });
                        });

                    // --- SECTION 3: LAYOUT & ALIGNMENT ---
                    ui.container()
                        .full_width()
                        .bg(Color::rgb(0.1, 0.05, 0.05))
                        .padding(15.0, 15.0, 15.0, 15.0)
                        .radius_all(8.0)
                        .gap(10.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("3. Explicit Alignment").color(Color::rgb(1.0, 0.6, 0.6)).show();

                            ui.container()
                                .full_width()
                                .height(150.0)
                                .bg(Color::rgb(0.15, 0.1, 0.1))
                                .align(Align::Center)
                                .show(|ui: &mut Ui| {
                                    ui.container()
                                        .width(80.0).height(80.0)
                                        .bg(Color::rgb(0.8, 0.4, 0.4))
                                        .radius_all(40.0)
                                        .show(|_| {});
                                    ui.text("Explicitly Centered").color(Color::WHITE).show();
                                });
                        });

                    // --- SECTION 4: RENDER MODES ---
                    ui.container()
                        .full_width()
                        .bg(Color::rgb(0.08, 0.08, 0.1))
                        .padding(15.0, 15.0, 15.0, 15.0)
                        .radius_all(8.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("4. Render Mode").color(Color::rgb(0.8, 0.8, 1.0)).show();
                            
                            ui.container()
                                .render_mode(RenderMode::Continuous)
                                .bg(Color::rgb(0.1, 0.1, 0.15))
                                .padding_all(10.0)
                                .show(|ui: &mut Ui| {
                                    ui.text(&format!("Elapsed Time: {:.2}s", ui.elapsed_time)).color(Color::rgb(0.0, 0.8, 1.0)).show();
                                    ui.text("(Continuous Render)").size(10.0).color(Color::rgb(0.5, 0.5, 0.5)).show();
                                });
                        });
                });
        })
        .run();
}
