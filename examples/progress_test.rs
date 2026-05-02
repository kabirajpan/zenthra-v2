use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Progress Bar Test")
        .size(500, 400)
        .with_ui(move |ui: &mut Ui| {
            ui.container()
                .fill_x()
                .fill_y()
                .align(Align::Center)
                .padding_all(40.0)
                .gap(30.0)
                .show(|ui: &mut Ui| {
                    ui.text("Progress Bars").size(32.0).show();

                    // 1. Basic Progress Bar
                    ui.text("Basic").show();
                    ui.progress_bar(0.6)
                        .width(300.0)
                        .radius(0.0, 0.0, 0.0, 0.0)
                        .show();

                    // 2. Styled with Shimmer (The "Beauty" Version)
                    ui.text("Premium with Shimmer & Shadows").show();
                    ui.progress_bar((ui.elapsed_time * 0.2) % 1.0) // Animates over time
                        .track_size(300.0, 12.0)
                        .fill_color(Color::rgb(0.0, 0.8, 0.4)) // Emerald Green
                        .radius(6.0, 6.0, 6.0, 6.0)
                        .fill_shadow(Color::BLACK, 0.0, 2.0, 6.0) // <--- Fill Shadow
                        .fill_shadow_opacity(0.5)
                        .shimmer(true)
                        .show();

                    // 3. Thick Blocky Style (Semi-Transparent)
                    ui.text("Thick & Square (Transparent)").show();
                    ui.progress_bar(0.8)
                        .track_size(300.0, 24.0)
                        .fill_color(Color::rgb(1.0, 0.4, 0.0)) // Orange
                        .bg(Color::rgb(0.1, 0.1, 0.1))
                        .border(Color::rgb(0.3, 0.3, 0.3), 1.0)
                        .padding(4.0, 4.0, 4.0, 4.0)
                        .opacity(0.6) // <--- Transparency
                        .show();
                });
        })
        .run();
}
