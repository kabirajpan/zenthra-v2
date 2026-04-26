use zenthra::prelude::*;

fn main() {
    let mut count = 0;

    App::new()
        .title("Zenthra - Interactive Buttons")
        .size(800, 600)
        .with_ui(move |ui| {
            ui.container()
                .fill()
                .bg(Color::rgb(0.05, 0.05, 0.08))
                .halign(Align::Center)
                .valign(Align::Center)
                .show(|ui| {
                    ui.container()
                        .column()
                        .width(520.0)
                        .gap(30.0)
                        .padding(40.0)
                        .bg(Color::rgb(0.1, 0.1, 0.12))
                        .radius(20.0)
                        .halign(Align::Center)
                        .show(|ui| {
                            ui.h1("Interaction Counter").color(Color::WHITE).show();

                            ui.container().row().gap(20.0).show(|ui| {
                                let btn = ui
                                    .button(&format!("Count: {}", count))
                                    .bg(Color::rgb(0.2, 0.4, 0.8))
                                    .radius(8.0, 8.0, 8.0, 8.0)
                                    .padding(15.0, 20.0, 15.0, 20.0)
                                    .show();

                                if btn.clicked {
                                    count += 1;
                                }

                                if ui
                                    .button("Reset")
                                    .bg(Color::rgb(0.8, 0.2, 0.2))
                                    .radius(8.0, 8.0, 8.0, 8.0)
                                    .padding(15.0, 20.0, 15.0, 20.0)
                                    .show()
                                    .clicked
                                {
                                    count = 0;
                                }
                            });

                            if count > 0 {
                                ui.text("Nice! You clicked the button.")
                                    .color(Color::rgba(1.0, 1.0, 1.0, 0.6))
                                    .show();
                            }
                        });
                });
        })
        .run();
}
