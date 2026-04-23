use zenthra::prelude::*;

fn main() {
    let mut my_text = String::from("Type here...");

    App::new()
        .title("Zenthra Editor")
        .size(600, 400)
        .with_ui(move |ui| {
            ui.h1("Interactive Editor").show();

            ui.text("Click the box below to start typing:").show();

            ui.text_area(&mut my_text)
                .size(20.0)
                .bg(Color::RED)
                .text_bg(Color::BLUE)
                .padding(20.0)
                .text_bg_full_width(true)
                .full_width()
                .height(200.0)
                .scrollable(true)
                .show();
        })
        .run();
}
