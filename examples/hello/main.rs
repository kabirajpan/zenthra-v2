use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra")
        .size(800, 600)
        .with_ui(|ui| {
            ui.container(|ui| {
                ui.text("Hello Zenthra!")
                    .size(32.0)
                    .color(Color::WHITE)
                    .show();
            })
            .fill()
            .bg(Color::rgb(0.05, 0.05, 0.07))
            .show();
        })
        .run();
}
