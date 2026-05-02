use zenthra::prelude::*;

fn main() {
    let mut states = [true, false, true, true, true, true, true, true];

    App::new()
        .title("Zenthra Checkbox List")
        .size(450, 600)
        .with_ui(move |ui: &mut Ui| {
            ui.container()
                .fill_x()
                .fill_y()
                .bg(Color::rgb(0.05, 0.05, 0.07)) // Deep Dark
                .padding_y(40.0)
                .padding_left(60.0)
                .gap(15.0)
                .show(|ui: &mut Ui| {
                    // Style 1: Simple Border
                    ui.checkbox(&mut states[0], "A simple checkbox")
                        .bg(Color::WHITE)
                        .border(Color::rgb(0.7, 0.7, 0.7), 1.0)
                        .check_bg(Color::WHITE)
                        .check_color(Color::rgb(0.3, 0.3, 0.3))
                        .label_color(Color::WHITE)
                        .radius_all(3.0)
                        .show();

                    // Style 2: Unchecked Simple
                    ui.checkbox(&mut states[1], "Style 2")
                        .bg(Color::WHITE)
                        .border(Color::rgb(0.7, 0.7, 0.7), 1.0)
                        .check_bg(Color::WHITE)
                        .check_color(Color::rgb(0.3, 0.3, 0.3))
                        .label_color(Color::WHITE)
                        .radius_all(3.0)
                        .show();

                    // Style 3: Blue
                    ui.checkbox(&mut states[2], "Style 3")
                        .check_bg(Color::rgb(0.2, 0.45, 0.75)) // Professional Blue
                        .check_color(Color::WHITE)
                        .label_color(Color::WHITE)
                        .radius_all(4.0)
                        .show();

                    // Style 4: Cyan
                    ui.checkbox(&mut states[3], "Style 4")
                        .check_bg(Color::rgb(0.35, 0.75, 0.85)) // Cyan
                        .check_color(Color::WHITE)
                        .label_color(Color::WHITE)
                        .radius_all(4.0)
                        .show();

                    // Style 5: Green
                    ui.checkbox(&mut states[4], "Style 5")
                        .check_bg(Color::rgb(0.35, 0.7, 0.35)) // Green
                        .check_color(Color::WHITE)
                        .label_color(Color::WHITE)
                        .radius_all(4.0)
                        .show();

                    // Style 6: Red
                    ui.checkbox(&mut states[5], "Style 6")
                        .check_bg(Color::rgb(0.85, 0.35, 0.35)) // Red
                        .check_color(Color::WHITE)
                        .label_color(Color::WHITE)
                        .radius_all(4.0)
                        .show();

                    // Style 7: Rounded Classic
                    ui.checkbox(&mut states[6], "Style 7 (Rounded)")
                        .bg(Color::WHITE)
                        .border(Color::rgb(0.7, 0.7, 0.7), 1.0)
                        .check_bg(Color::WHITE)
                        .check_color(Color::rgb(0.3, 0.3, 0.3))
                        .label_color(Color::WHITE)
                        .radius_all(10.0) // Circular
                        .show();

                    // Style 8: Rounded Cyan
                    ui.checkbox(&mut states[7], "Style 8 (Rounded)")
                        .check_bg(Color::rgb(0.35, 0.75, 0.85))
                        .check_color(Color::WHITE)
                        .label_color(Color::rgb(0.2, 0.2, 0.2))
                        .radius_all(10.0) // Circular
                        .show();
                });
        })
        .run();
}
