use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Minimal Button")
        .size(800, 600)
        .with_ui(|ui| {
            // Minimal setup as requested: just a button, no padding, "nothing" extra
            ui.container()
                .fill()
                .bg(Color::rgb(0.02, 0.02, 0.03))
                .halign(Align::Center)
                .valign(Align::Center)
                .show(|ui| {
                    // The "nothing" button
                    ui.button("Default Button").show();

                    ui.spacing(20.0);

                    // The "nothing" button
                    ui.button("Simple Button")
                        .shadow(Color::BLACK, 2.0, 2.0, 5.0)
                        .shadow_opacity(0.5)
                        .padding(0.0, 0.0, 0.0, 0.0)
                        .radius(0.0, 0.0, 0.0, 0.0)
                        .show();
                });
        })
        .run();
}
