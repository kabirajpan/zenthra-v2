use zenthra::prelude::*;

fn main() {
    let mut on1 = true;
    let mut on2 = false;
    let mut on3 = true;

    App::new()
        .title("Switch Test")
        .size(400, 300)
        .with_ui(move |ui: &mut Ui| {
            ui.container()
                .fill_x()
                .fill_y()
                .align(Align::Center)
                .padding_all(40.0)
                .gap(20.0)
                .show(|ui: &mut Ui| {
                    ui.text("Toggle Switches").size(32.0).show();

                    // 1. Square Track with Circular Thumb (Premium Shadow)
                    ui.container().row().gap(10.0).show(|ui: &mut Ui| {
                        ui.text("Bluetooth").show();
                        ui.toggle(&mut on1, None)
                            .width(60.0)
                            .height(30.0)
                            .padding(5.0)
                            .shadow(Color::BLACK, 0.0, 2.0, 6.0)
                            .shadow_opacity(0.5)
                            .show();
                    });

                    // 2. Semi-Transparent Switch
                    ui.container().row().gap(10.0).show(|ui: &mut Ui| {
                        ui.text("Dark Mode").show();
                        ui.toggle(&mut on2, None)
                            .width(50.0)
                            .height(20.0)
                            .padding(2.0)
                            .opacity(0.7)
                            .colors(
                                Color::rgb(0.4, 0.2, 1.0),
                                Color::rgb(0.1, 0.1, 0.1),
                                Color::rgb(0.8, 0.8, 0.8),
                            )
                            .pill()
                            .show();
                    });

                    // 3. Neumorphic Rounded Switch
                    ui.container().row().gap(10.0).show(|ui: &mut Ui| {
                        ui.text("Notifications").show();
                        ui.toggle(&mut on3, None)
                            .width(44.0)
                            .height(24.0)
                            .radius_all(12.0)
                            .shadow(Color::BLACK, 0.0, 4.0, 10.0)
                            .shadow_opacity(0.3)
                            .show();
                    });
                });
        })
        .run();
}
