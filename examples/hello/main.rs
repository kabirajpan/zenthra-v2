use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra")
        .size(800, 600)
        .with_ui(|ui| {
            ui.row(|ui| {
                ui.text("Line 1").size(20.0).color(Color::WHITE).show();
                ui.text("Line 2").size(20.0).color(Color::GREEN).show();
                ui.text("Line 3").size(20.0).color(Color::BLUE).show();
            })
            .bg(Color::rgb(0.1, 0.1, 0.15))
            .show();
        })
        .run();
}
