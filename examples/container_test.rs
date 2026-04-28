use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Container Test")
        .size(800, 600)
        .with_ui(|ui| {
            // Root background
            ui.container()
                .fill()
                .bg(Color::rgb(0.94, 0.94, 0.94))
                .center()
                .show(|ui| {
                    // This is the simple box matching the HTML/CSS
                    ui.container()
                        .id("simple-box")
                        .bg(Color::WHITE)
                        .height(100.0)
                        .width(200.0)
                        .border(Color::rgb(0.2, 0.2, 0.2), 2.0)
                        .radius(8.0, 8.0, 8.0, 8.0)
                        .shadow(Color::rgb(0.0, 0.0, 0.0), 4.0, 4.0, 10.0)
                        .shadow_opacity(0.15)
                        .center()
                        .show(|ui| {
                            ui.text("Hello, Worljd!")
                                .color(Color::rgb(0.2, 0.2, 0.2))
                                .size(18.0)
                                .weight(600)
                                .show();
                        });
                });
        })
        .run();
}
