use zenthra::prelude::*;
use zenthra_core::EdgeInsets;

fn main() {
    let mut my_text = String::from("Type here...");

    App::new()
        .title("Zenthra Editor")
        .size(600, 400)
        .with_ui(move |ui| {
            ui.h1("Interactive Editor").show();

            ui.text("Click the box below to start typing:").show();

            ui.input(&mut my_text)
                .size(20.0) // Sets font size to 24
                .bg(Color::BLUE)
                .min_width(200.0) // Sets minimum width to 400
                .show();

            ui.text(&format!("Current buffer: {}", my_text)).show();
        })
        .run();
}
