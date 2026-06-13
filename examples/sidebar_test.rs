use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Sidebar Test")
        .size(900, 600)
        .with_ui(move |ui: &mut Ui| {
            // Full screen background
            ui.container()
                .fill()
                .bg(Color::rgb(0.04, 0.04, 0.06))
                .show(|ui| {
                    // Menu bar
                    ui.container()
                        .full_width()
                        .height(30.0)
                        .bg(Color::rgb(0.12, 0.12, 0.14))
                        .row()
                        .padding_left(8.0)
                        .align(Align::Center)
                        .show(|ui| {
                            ui.text("File").color(Color::WHITE).size(13.0).show();
                            ui.spacing(20.0);
                            ui.text("View").color(Color::WHITE).size(13.0).show();
                        });

                    // Two-column layout
                    ui.container()
                        .row()
                        .fill()
                        .padding(12.0, 12.0, 12.0, 12.0)
                        .gap(12.0)
                        .show(|ui| {
                            // LEFT SIDEBAR
                            ui.container()
                                .width(250.0)
                                .height(500.0)
                                .bg(Color::rgb(0.07, 0.07, 0.10))
                                .radius_all(12.0)
                                .border(Color::rgba(1.0, 1.0, 1.0, 0.05), 1.0)
                                .padding(16.0, 16.0, 16.0, 16.0)
                                .clip(true)
                                .show(|ui| {
                                    ui.text("Sidebar Title")
                                        .size(18.0)
                                        .color(Color::WHITE)
                                        .show();

                                    ui.spacing(10.0);

                                    ui.text("Items: 30")
                                        .size(13.0)
                                        .color(Color::rgba(1.0, 1.0, 1.0, 0.5))
                                        .show();

                                    ui.spacing(10.0);

                                    // Scrollable list
                                    ui.container()
                                        .fill_x()
                                        .height(400.0)
                                        .scroll_y(true)
                                        .gap(3.0)
                                        .show(|ui| {
                                            for i in 0..30 {
                                                let is_selected = i == 0;
                                                let bg = if is_selected {
                                                    Color::rgba(0.2, 0.4, 0.8, 0.15)
                                                } else {
                                                    Color::TRANSPARENT
                                                };
                                                let text_color = if is_selected {
                                                    Color::rgb(0.35, 0.65, 1.0)
                                                } else {
                                                    Color::rgba(1.0, 1.0, 1.0, 0.65)
                                                };

                                                ui.button(&format!("Item {}", i + 1))
                                                    .bg(bg)
                                                    .text_color(text_color)
                                                    .radius(4.0, 4.0, 4.0, 4.0)
                                                    .padding(8.0, 10.0, 8.0, 10.0)
                                                    .size(12.5)
                                                    .hover_bg(Color::rgba(1.0, 1.0, 1.0, 0.05))
                                                    .show();
                                            }
                                        });
                                });

                            // RIGHT CONTENT
                            ui.container()
                                .fill()
                                .bg(Color::rgb(0.06, 0.06, 0.08))
                                .radius_all(12.0)
                                .padding(20.0, 20.0, 20.0, 20.0)
                                .gap(10.0)
                                .clip(true)
                                .show(|ui| {
                                    ui.text("Main Content Area")
                                        .size(22.0)
                                        .color(Color::WHITE)
                                        .show();

                                    ui.text("This panel fills the remaining space.")
                                        .size(14.0)
                                        .color(Color::rgba(1.0, 1.0, 1.0, 0.5))
                                        .show();

                                    // A box to visualize the content area
                                    ui.container()
                                        .fill_x()
                                        .height(300.0)
                                        .bg(Color::rgb(0.02, 0.02, 0.03))
                                        .radius_all(8.0)
                                        .border(Color::rgba(1.0, 1.0, 1.0, 0.05), 1.0)
                                        .align(Align::Center)
                                        .show(|ui| {
                                            ui.text("Content Viewport")
                                                .size(16.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 0.3))
                                                .show();
                                        });

                                    // Bottom strip
                                    ui.container()
                                        .row()
                                        .fill_x()
                                        .height(70.0)
                                        .bg(Color::rgb(0.06, 0.06, 0.08))
                                        .radius_all(8.0)
                                        .border(Color::rgba(1.0, 1.0, 1.0, 0.03), 1.0)
                                        .padding(8.0, 8.0, 8.0, 8.0)
                                        .gap(8.0)
                                        .scroll_x(true)
                                        .show(|ui| {
                                            for i in 0..12 {
                                                ui.container()
                                                    .width(80.0)
                                                    .height(54.0)
                                                    .bg(Color::rgb(0.15, 0.15, 0.2))
                                                    .radius_all(6.0)
                                                    .align(Align::Center)
                                                    .show(|ui| {
                                                        ui.text(&format!("{}", i + 1))
                                                            .color(Color::WHITE)
                                                            .show();
                                                    });
                                            }
                                        });
                                });
                        });
                });
        })
        .run();
}
