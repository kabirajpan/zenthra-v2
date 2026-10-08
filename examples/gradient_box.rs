use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Gradient Box (Blue to Red)")
        .size(600, 400)
        .with_ui(|ui| {
            // Background container filling the window
            ui.container()
                .fill()
                .bg(Color::rgb(0.06, 0.06, 0.08))
                .align(Align::Center)
                .show(|ui| {
                    // Simple gradient box: Blue on left, Red on right
                    ui.container()
                        .width(320.0)
                        .height(180.0)
                        .radius_all(16.0)
                        .bg(Gradient::linear(
                            Direction::ToRight,
                            [
                                Color::rgb(0.15, 0.40, 1.0), // Blue (Left)
                                Color::rgb(1.0, 0.20, 0.35), // Red (Right)
                            ],
                        ))
                        .align(Align::Center)
                        .show(|ui| {
                            ui.text("Blue → Red")
                                .size(20.0)
                                .weight(FontWeight::Bold)
                                .color(Color::WHITE)
                                .show();
                        });
                });
        })
        .run();
}
