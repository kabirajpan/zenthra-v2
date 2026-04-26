use zenthra::prelude::*;

fn main() {
    let mut input_text = String::from("Single line input");
    let mut area_text = String::from("Multi-line\nTextArea content");

    App::new()
        .title("Zenthra Test")
        .size(600, 400)
        .with_ui(move |ui| {
            ui.h1("Widget Parity Test").show();

            ui.text("Input (Horizontal Scroll):").show();
            ui.input(&mut input_text)
                .size(20.0)
                .text_bg(Color::BLUE)
                .text_padding(30.0)
                .text_bg_full_width(true)
                .bg(Color::RED)
                .padding(10.0)
                .full_width()
                .show();

            ui.text("TextArea (Vertical Grid):").show();
            ui.text_area(&mut area_text)
                .size(20.0)
                .bg(Color::RED)
                .text_bg(Color::BLUE)
                .highlight(Color::rgb(0.0, 0.5, 0.0))
                .text_bg_full_width(true)
                .text_padding(30.0)
                .padding(10.0)
                .full_width()
                .height(150.0)
                .scrollable(true)
                .show();
        })
        .run();
}
