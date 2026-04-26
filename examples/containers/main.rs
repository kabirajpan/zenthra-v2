use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Container Test")
        .size(800, 600)
        .with_ui(|ui| {
            // Main Outer Container configured as a Row
            ui.container()
                .column()
                .wrap(Wrap::Wrap)
                .fill()
                .gap(10.0)
                .bg(Color::rgb(0.05, 0.05, 0.07))
                .padding(20.0)
                .show(|ui| {
                    // Box 1 - Red
                    ui.container()
                        .width(100.0)
                        .height(60.0)
                        .bg(Color::rgb(0.8, 0.2, 0.2))
                        .radius(6.0)
                        .align(Align::Center)
                        .show(|ui| {
                            ui.text("1").size(20.0).color(Color::WHITE).show();
                        });

                    // Box 2 - Green
                    ui.container()
                        .width(100.0)
                        .height(60.0)
                        .bg(Color::rgb(0.2, 0.8, 0.2))
                        .radius(6.0)
                        .align(Align::Center)
                        .show(|ui| {
                            ui.text("2").size(20.0).color(Color::WHITE).show();
                        });

                    // Box 3 - Blue
                    ui.container()
                        .width(100.0)
                        .height(60.0)
                        .bg(Color::rgb(0.2, 0.2, 0.8))
                        .radius(6.0)
                        .align(Align::Center)
                        .show(|ui| {
                            ui.text("3").size(20.0).color(Color::WHITE).show();
                        });

                    // Box 4 - Yellow
                    ui.container()
                        .width(100.0)
                        .height(60.0)
                        .bg(Color::rgb(0.8, 0.8, 0.2))
                        .radius(6.0)
                        .align(Align::Center)
                        .show(|ui| {
                            ui.text("4").size(20.0).color(Color::WHITE).show();
                        });

                    // Box 5 - Purple
                    ui.container()
                        .width(100.0)
                        .height(60.0)
                        .bg(Color::rgb(0.8, 0.2, 0.8))
                        .radius(6.0)
                        .align(Align::Center)
                        .show(|ui| {
                            ui.text("5").size(20.0).color(Color::WHITE).show();
                        });

                    // Box 6 - Cyan
                    ui.container()
                        .width(100.0)
                        .height(60.0)
                        .bg(Color::rgb(0.2, 0.8, 0.8))
                        .radius(6.0)
                        .align(Align::Center)
                        .show(|ui| {
                            ui.text("6").size(20.0).color(Color::WHITE).show();
                        });

                    // Box 7 - Grey
                    ui.container()
                        .width(100.0)
                        .height(60.0)
                        .bg(Color::rgb(0.5, 0.5, 0.5))
                        .radius(6.0)
                        .align(Align::Center)
                        .show(|ui| {
                            ui.text("7").size(20.0).color(Color::WHITE).show();
                        });
                });
        })
        .run();
}
