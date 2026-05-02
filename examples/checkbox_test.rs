use zenthra::prelude::*;

fn main() {
    let mut check1 = false;
    let mut check2 = true;
    let mut check3 = false;

    App::new()
        .title("Checkbox Test")
        .size(400, 400)
        .with_ui(move |ui: &mut Ui| {
            ui.container()
                .fill_x()
                .fill_y()
                .align(Align::Center)
                .bg(Color::rgb(0.05, 0.05, 0.07))
                .padding_all(40.0)
                .gap(20.0)
                .show(|ui: &mut Ui| {
                    ui.text("Checkboxes")
                        .size(32.0)
                        .color(Color::WHITE)
                        .show();

                    // 1. Default Style
                    ui.checkbox(&mut check1, "Default jjg Checkbox").show();

                    // 2. Custom Sizing and Colors
                    ui.checkbox(&mut check2, "Custom Style")
                        .size(24.0)
                        .radius_all(0.0)
                        .gap(12.0)
                        .check_bg(Color::rgb(0.2, 0.8, 0.4))
                        .check_color(Color::rgb(0.0, 0.2, 0.0))
                        .label_size(18.0)
                        .label_color(Color::rgb(0.8, 0.8, 1.0))
                        .show();

                    // 3. Outlined Style
                    ui.checkbox(&mut check3, "Outlined")
                        .bg(Color::TRANSPARENT)
                        .border(Color::rgb(1.0, 0.5, 0.0), 2.0)
                        .check_bg(Color::rgb(1.0, 0.5, 0.0))
                        .show();

                    ui.spacing(20.0);

                    if check1 {
                        ui.text("Option 1 is enabled!")
                            .color(Color::rgb(0.5, 1.0, 0.5))
                            .show();
                    }
                });
        })
        .run();
}
