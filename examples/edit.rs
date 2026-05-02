use zenthra::prelude::*;

fn main() {
    let mut my_text = String::from("Type here...");

    App::new()
        .title("Zenthra Editor")
        .size(600, 400)
        .with_ui(move |ui: &mut Ui| {
            ui.text("Interactive Editor").size(32.0).bold().show();

            ui.text("Click the box below to start typing:").show();

            ui.input(&mut my_text)
                .size(24.0) // Sets font size to 24
                .min_width(400.0) // Sets minimum width to 400
                .padding_x(10.0) // Adds 10px horizontal padding
                .show();

            ui.text(format!("Current buffer: {}", my_text)).show();
        })
        .run();
}
