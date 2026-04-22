use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra")
        .size(600, 600)
        .with_ui(|ui| {
            ui.text("Hello Kabiraj").bg(Color::BLUE).show();
        })
        .run();
}
