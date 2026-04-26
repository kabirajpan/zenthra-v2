use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra")
        .size(600, 600)
        .with_ui(|ui| {
            ui.h1("This is H1 Headerj").bg(Color::RED).show();
            ui.h2("This is H2 Header").bg(Color::BLUE).show();
            ui.h3("This is H3 Header").bg(Color::BLUE).show();
            ui.h4("This is H4 Header").bg(Color::BLUE).show();
            ui.text("And this is standard body text.")
                .bg(Color::GREEN)
                .highlight(Color::RED)
                .size(30.0)
                .show();
            ui.text("And this is standard body text. And this is standard body text. And this is standard body text. And this is standard body text. And this is standard body text. And this is standard body text.")
                
                .bg(Color::BLUE)
                // .padding_x(30.0)
                .highlight(Color::RED)
                .show();
        })
        .run();
}
