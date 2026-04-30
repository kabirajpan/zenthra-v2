use zenthra::prelude::*;

fn main() {
    let mut states = [true; 15];

    App::new()
        .title("Zenthra Toggle Switchy")
        .size(800, 600)
        .with_ui(move |ui| {
            ui.container()
                .fill()
                .bg(Color::rgb(0.05, 0.05, 0.07))
                .padding(40.0, 40.0, 40.0, 40.0)
                .gap(30.0)
                .show(|ui| {
                    ui.h1("Toggle Switchy").color(Color::WHITE).show();

                    // --- 1. Default (Square-ish) ---
                    ui.row().gap(40.0).show(|ui| {
                        ui.h3("Default").color(Color::rgb(0.8, 0.8, 0.8)).show();
                        ui.toggle(&mut states[0], None).inner_labels("ON", "OFF").radius(3.0).show();
                        ui.toggle(&mut states[1], None).inner_labels("ON", "OFF").radius(3.0).size(40.0, 20.0).show();
                        ui.toggle(&mut states[2], None).inner_labels("ON", "OFF").radius(3.0).size(34.0, 18.0).show();
                    });

                    // --- 2. Rounded ---
                    ui.row().gap(40.0).show(|ui| {
                        ui.h3("Rounded").color(Color::rgb(0.8, 0.8, 0.8)).show();
                        ui.toggle(&mut states[3], None).inner_labels("ON", "OFF").pill().show();
                        ui.toggle(&mut states[4], None).inner_labels("ON", "OFF").pill().size(40.0, 20.0).show();
                        ui.toggle(&mut states[5], None).inner_labels("ON", "OFF").pill().size(34.0, 18.0).show();
                    });

                    // --- 3. No Text ---
                    ui.row().gap(40.0).show(|ui| {
                        ui.h3("No Text").color(Color::rgb(0.8, 0.8, 0.8)).show();
                        ui.toggle(&mut states[6], None).pill().show();
                        ui.toggle(&mut states[7], None).pill().size(40.0, 20.0).show();
                        ui.toggle(&mut states[8], None).pill().size(34.0, 18.0).show();
                    });

                    // --- 4. Colors ---
                    ui.row().gap(40.0).show(|ui| {
                        ui.h3("Colors").color(Color::rgb(0.8, 0.8, 0.8)).show();
                        ui.toggle(&mut states[9], None).inner_labels("ON", "").pill().colors(Color::rgb(0.8, 0.2, 0.2), Color::rgb(0.2, 0.2, 0.2), Color::WHITE).show();
                        ui.toggle(&mut states[10], None).inner_labels("ON", "").pill().colors(Color::rgb(0.9, 0.5, 0.1), Color::rgb(0.2, 0.2, 0.2), Color::WHITE).show();
                        ui.toggle(&mut states[11], None).inner_labels("ON", "").pill().colors(Color::rgb(0.9, 0.8, 0.1), Color::rgb(0.2, 0.2, 0.2), Color::WHITE).show();
                        ui.toggle(&mut states[12], None).inner_labels("ON", "").pill().colors(Color::rgb(0.2, 0.7, 0.3), Color::rgb(0.2, 0.2, 0.2), Color::WHITE).show();
                    });

                    // --- 5. Labels & Disabled ---
                    ui.row().gap(40.0).show(|ui| {
                        ui.toggle(&mut states[13], "Label on right").pill().show();
                        ui.toggle(&mut states[14], "Label on left").pill().label_left().show();
                    });

                    // --- 6. Advanced Customization ---
                    ui.row().gap(40.0).show(|ui| {
                        ui.h3("Advanced").color(Color::rgb(0.8, 0.8, 0.8)).show();
                        
                        ui.toggle(&mut states[10], "Slow Motion")
                            .animation_speed(3.0)
                            .inner_labels("ON", "OFF")
                            .inner_color(Color::rgb(1.0, 0.9, 0.0))
                            .pill()
                            .show();

                        ui.toggle(&mut states[11], "Large Text")
                            .inner_labels("RUN", "STOP")
                            .inner_size(12.0)
                            .size(70.0, 28.0)
                            .pill()
                            .show();
                            
                        ui.toggle(&mut states[12], "Disabled State")
                            .pill()
                            .disabled(true)
                            .show();
                    });
                });
        })
        .run();
}
