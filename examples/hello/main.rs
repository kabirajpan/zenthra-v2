use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Text Test")
        .size(800, 600)
        .with_ui(|ui| {
            // bg fill
            ui.container(|_ui| {})
                .fill()
                .bg(Color::rgb(0.05, 0.05, 0.07))
                .show();

            // plain text
            ui.text("Hello Zenthra!")
                .size(32.0)
                .color(Color::WHITE)
                .pos(40.0, 40.0)
                .show();

            // highlighted text (like code editor)
            ui.text("highlighted")
                .size(20.0)
                .color(Color::BLACK)
                .bg(Color::rgb(1.0, 0.85, 0.0))
                .padding(4.0)
                .pos(40.0, 100.0)
                .show();

            // monospace bold
            ui.text("fn main() {")
                .size(18.0)
                .color(Color::rgb(0.4, 0.8, 1.0))
                .monospace()
                .bold()
                .pos(40.0, 160.0)
                .show();

            // italic
            ui.text("// comment")
                .size(16.0)
                .color(Color::rgb(0.5, 0.5, 0.5))
                .italic()
                .monospace()
                .pos(40.0, 200.0)
                .show();

            // hover test
            ui.text("hover over me")
                .size(20.0)
                .color(Color::WHITE)
                .hover_bg(Color::rgba(1.0, 1.0, 1.0, 0.1))
                .cursor_pointer()
                .pos(40.0, 260.0)
                .show();
        })
        .run();
}
