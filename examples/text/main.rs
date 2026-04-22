use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra")
        .size(600, 600)
        .with_ui(|ui| {
            ui.h1("This is H1 Header").bg(Color::BLUE).show();
            ui.h2("This is H2 Header").bg(Color::BLUE).show();
            ui.h3("This is H3 Header").bg(Color::BLUE).show();
            ui.h4("This is H4 Header").bg(Color::BLUE).show();
            ui.text("And this is standard body text.")
                .bg(Color::BLUE)
                .show();
            ui.text("And this is standard body text.")
                .bg(Color::BLUE)
                .show();
        })
        .run();
}
