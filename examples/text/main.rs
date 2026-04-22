use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra")
        .size(600, 600)
        .with_ui(|ui| {
            ui.text("The renderer was trying to expand the background box "outwards" from the text position. When the text was at the top-left (0,0), the padding was being drawn at negative coordinates (like -20), which made it appear hidden off-screen.")
                .padding_top(20.0)
                .bg(Color::BLUE)
                .show();
        })
        .run();
}
