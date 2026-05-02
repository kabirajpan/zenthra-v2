use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Scroll Test")
        .size(600, 500)
        .with_ui(move |ui: &mut Ui| {
            ui.container()
                .fill_x()
                .fill_y()
                .bg(Color::rgb(0.05, 0.05, 0.07))
                .align(Align::Center)
                .padding_all(40.0)
                .gap(40.0)
                .show(|ui: &mut Ui| {
                    ui.text("Scrollbar Testing")
                        .size(32.0)
                        .color(Color::WHITE)
                        .show();

                    ui.row().gap(40.0).bg(Color::RED).show(|ui: &mut Ui| {
                        // 1. Vertical Scroll Test
                        ui.column().gap(10.0).bg(Color::GREEN).show(|ui: &mut Ui| {
                            ui.text("Vertical Scroll")
                                .color(Color::rgb(0.6, 0.6, 0.6))
                                .show();
                            ui.container()
                                .width(200.0)
                                .height(250.0)
                                .border(Color::rgba(1.0, 1.0, 1.0, 0.2), 3.0)
                                .border_alignment(BorderAlignment::Outside)
                                .bg(Color::rgb(0.1, 0.1, 0.12))
                                .radius(8.0, 8.0, 8.0, 8.0)
                                .padding(10.0, 10.0, 10.0, 10.0)
                                .scroll_y(true)
                                .gap(8.0)
                                .show(|ui: &mut Ui| {
                                    for i in 0..20 {
                                        ui.container()
                                            .fill_x()
                                            .height(40.0)
                                            .bg(Color::rgb(0.15, 0.15, 0.2))
                                            .radius_all(4.0)
                                            .align(Align::Center)
                                            .show(|ui: &mut Ui| {
                                                ui.text(&format!("Item {}", i + 1))
                                                    .color(Color::WHITE)
                                                    .show();
                                            });
                                    }
                                });
                        });

                        // 2. Horizontal Scroll Test
                        ui.column().gap(10.0).show(|ui: &mut Ui| {
                            ui.text("Horizontal Scroll")
                                .color(Color::rgb(0.6, 0.6, 0.6))
                                .show();
                            ui.container()
                                .width(250.0)
                                // .border(Color::rgba(1.0, 1.0, 1.0, 0.2), 3.0)
                                // .border_alignment(BorderAlignment::Center)
                                .bg(Color::rgb(0.1, 0.1, 0.12))
                                .radius(8.0, 8.0, 8.0, 8.0)
                                .padding_y(10.0)
                                .scroll_x(true)
                                .row()
                                .gap(8.0)
                                .show(|ui: &mut Ui| {
                                    for i in 0..15 {
                                        ui.container()
                                            .width(80.0)
                                            .height(60.0)
                                            .bg(Color::rgb(0.2, 0.15, 0.25))
                                            .radius_all(4.0)
                                            .align(Align::Center)
                                            .show(|ui: &mut Ui| {
                                                ui.text(&format!("Box {}", i + 1))
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
