use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Hello")
        .size(800, 600)
        .with_ui(|ui| {
            ui.container()
                .fill()
                .bg(Color::rgb(0.05, 0.05, 0.08))
                .valign(Align::Center)
                .halign(Align::Center)
                .show(|ui| {
                    ui.container().row().gap(16.0).show(|ui| {
                        ui.text("Hello,").size(32.0).color(Color::WHITE).show();
                        ui.text("Zenthra!")
                            .size(32.0)
                            .color(Color::rgb(0.4, 0.6, 1.0))
                            .show();
                    });
                });
        })
        .run();
}
