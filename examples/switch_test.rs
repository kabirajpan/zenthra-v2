use zenthra::prelude::*;

fn main() {
    let mut on1 = true;
    let mut on2 = false;
    let mut on3 = true;

    App::new()
        .title("Switch Test")
        .size(400, 300)
        .with_ui(move |ui| {
            ui.container()
                .fill()
                .center()
                .padding(40.0, 40.0, 40.0, 40.0)
                .gap(20.0)
                .show(|ui| {
                    ui.text("Toggle Switches").size(32.0).show();

                    // 1. Square Track with Circular Thumb (Premium Shadow)
                    ui.container().row().gap(10.0).show(|ui| {
                        ui.text("Bluetooth").show();
                        ui.switch(&mut on1, "sw1")
                            .width(60.0)
                            .height(30.0)
                            .padding(5.0)
                            .thumb_size(20.0, 20.0)
                            .thumb_radius_full()
                            .thumb_shadow(Color::BLACK, 0.0, 2.0, 6.0)
                            .thumb_shadow_opacity(0.5)
                            .show();
                    });

                    // 2. Semi-Transparent Switch
                    ui.container().row().gap(10.0).show(|ui| {
                        ui.text("Dark Mode").show();
                        ui.switch(&mut on2, "sw2")
                            .width(50.0)
                            .height(20.0)
                            .padding(2.0)
                            .opacity(0.7)
                            .colors(
                                Color::rgb(0.4, 0.2, 1.0),
                                Color::rgb(0.1, 0.1, 0.1),
                                Color::rgb(0.8, 0.8, 0.8),
                            )
                            .thumb_radius_full()
                            .show();
                    });

                    // 3. Neumorphic Rounded Switch
                    ui.container().row().gap(10.0).show(|ui| {
                        ui.text("Notifications").show();
                        ui.switch(&mut on3, "sw3")
                            .width(44.0)
                            .height(24.0)
                            .radius(12.0, 12.0, 12.0, 12.0)
                            .thumb_radius_full()
                            .shadow(Color::BLACK, 0.0, 4.0, 10.0)
                            .shadow_opacity(0.3)
                            .show();
                    });
                });
        })
        .run();
}
