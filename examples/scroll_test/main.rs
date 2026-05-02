use zenthra::prelude::*;
use zenthra_widgets::prelude::*;

fn main() {
    let mut app = App::new("Zenthra Scroll Test", 800, 600);

    app.run(|ui| {
        ui.container()
            .fill_x()
            .height(600.0)
            .padding_all(20.0)
            .bg(Color::rgb(0.05, 0.05, 0.07))
            .show(|ui| {
                ui.text("Zenthra Scroll Implementation Test")
                    .size(24.0)
                    .color(Color::WHITE)
                    .show();

                ui.spacer(20.0);

                // 1. Vertical Scroll Test
                ui.text("Vertical Scroll (30 Items)").size(18.0).color(Color::rgb(0.7, 0.7, 0.7)).show();
                ui.container()
                    .width(300.0)
                    .height(200.0)
                    .bg(Color::rgb(0.1, 0.1, 0.12))
                    .radius_all(8.0)
                    .border(Color::rgb(0.3, 0.3, 0.3), 1.0)
                    .scrollable(false, true)
                    .padding_all(10.0)
                    .show(|ui| {
                        for i in 1..=30 {
                            ui.container()
                                .fill_x()
                                .height(40.0)
                                .bg(if i % 2 == 0 { Color::rgb(0.15, 0.15, 0.17) } else { Color::rgb(0.12, 0.12, 0.14) })
                                .radius_all(4.0)
                                .margin_bottom(5.0)
                                .align(Align::Center)
                                .show(|ui| {
                                    ui.text(&format!("Item #{}", i)).show();
                                });
                        }
                    });

                ui.spacer(30.0);

                // 2. Horizontal Scroll Test
                ui.text("Horizontal Scroll (10 Cards)").size(18.0).color(Color::rgb(0.7, 0.7, 0.7)).show();
                ui.container()
                    .fill_x()
                    .height(120.0)
                    .bg(Color::rgb(0.1, 0.1, 0.12))
                    .radius_all(8.0)
                    .border(Color::rgb(0.3, 0.3, 0.3), 1.0)
                    .scrollable(true, false)
                    .padding_all(10.0)
                    .show(|ui| {
                        ui.row(|ui| {
                            for i in 1..=10 {
                                ui.container()
                                    .width(150.0)
                                    .height(80.0)
                                    .bg(Color::rgb(0.2, 0.2, 0.25))
                                    .radius_all(6.0)
                                    .margin_right(10.0)
                                    .shadow(Color::BLACK, 0.0, 4.0, 8.0)
                                    .align(Align::Center)
                                    .show(|ui| {
                                        ui.text(&format!("Card #{}", i)).show();
                                    });
                            }
                        });
                    });

                ui.spacer(30.0);

                // 3. Bidirectional Scroll Test
                ui.text("Bidirectional Scroll (Large Grid)").size(18.0).color(Color::rgb(0.7, 0.7, 0.7)).show();
                ui.container()
                    .width(400.0)
                    .height(200.0)
                    .bg(Color::rgb(0.08, 0.08, 0.1))
                    .radius_all(12.0)
                    .border(Color::rgb(0.4, 0.4, 0.5), 1.0)
                    .scrollable(true, true)
                    .show(|ui| {
                        // Create a 1000x1000 content area
                        ui.container()
                            .width(1000.0)
                            .height(1000.0)
                            .bg(Color::rgba(0.2, 0.2, 0.3, 0.1))
                            .show(|ui| {
                                for y in 0..10 {
                                    ui.row(|ui| {
                                        for x in 0..10 {
                                            ui.container()
                                                .width(90.0)
                                                .height(90.0)
                                                .margin_all(5.0)
                                                .bg(Color::rgb(0.3, 0.3, 0.4))
                                                .radius_all(4.0)
                                                .align(Align::Center)
                                                .show(|ui| {
                                                    ui.text(&format!("{},{}", x, y)).show();
                                                });
                                        }
                                    });
                                }
                            });
                    });
            });
    });
}
