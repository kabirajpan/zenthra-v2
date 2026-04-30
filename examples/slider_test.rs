use zenthra::prelude::*;

fn main() {
    let mut val = 50.0;

    App::new()
        .title("Slider Test")
        .size(400, 300)
        .with_ui(move |ui| {
            ui.container()
                .fill()
                .center()
                .padding(20.0, 20.0, 20.0, 20.0)
                .gap(20.0)
                .show(|ui| {
                    ui.text(&format!("Value: {:.2}", val)).size(24.0).show();
                    let prev_val = val;

                    ui.slider(&mut val, "main-slider")
                        .range(0.0, 100.0)
                        // 1. Root Style
                        .size(300.0, 100.0)
                        .bg(Color::rgb(0.1, 0.1, 0.1))
                        .border(Color::rgb(0.3, 0.3, 0.3), 1.0)
                        .radius(8.0, 8.0, 8.0, 8.0)
                        .padding(0.0, 20.0, 0.0, 20.0)
                        .shadow(Color::BLACK, 0.0, 2.0, 5.0)
                        // 2. Track Style
                        .track_size(260.0, 6.0)
                        .track_radius(3.0, 3.0, 3.0, 3.0)
                        .track_color(Color::rgb(0.2, 0.2, 0.2))
                        .track_shadow(Color::BLACK, 0.0, 2.0, 5.0)
                        // 3. Thumb Style
                        .thumb_size(24.0, 20.0)
                        .thumb_radius(0.0, 0.0, 0.0, 0.0)
                        .thumb_color(Color::RED)
                        .thumb_shadow(Color::BLACK, 0.0, 2.0, 5.0)
                        .show();

                    if (val - prev_val).abs() > 0.001 {
                        println!("Slider value changed: {}", val);
                    }

                    if ui.button("Reset").show().clicked {
                        val = 50.0;
                    }
                });
        })
        .run();
}
