use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Nested Wrap Test")
        .size(800, 600)
        .with_ui(|ui| {
            ui.container()
                .column()
                .wrap(Wrap::Wrap)
                .fill()
                .gap(20.0)
                .bg(Color::rgb(0.05, 0.05, 0.07))
                .show(|ui| {
                    ui.text("Parent Container (Column Wrap)").size(24.0).color(Color::WHITE).show();
                    
                    // A nested row that should wrap its own children
                    ui.container()
                        .row()
                        .wrap(Wrap::Wrap)
                        .center()
                        .width(400.0) // Fixed width to force internal wrapping
                        .padding(10.0, 10.0, 10.0, 10.0)
                        .bg(Color::rgb(0.15, 0.15, 0.2))
                        .radius(8.0, 8.0, 8.0, 8.0)
                        .show(|ui| {
                            for i in 1..=15 {
                                ui.container()
                                    .row()
                                    .no_wrap()
                                    .center()
                                    .width(80.0)
                                    .height(50.0)
                                    .bg(Color::rgb(0.2, 0.4, 0.6))
                                    .radius(4.0, 4.0, 4.0, 4.0)
                                    .show(|ui| {
                                        ui.text(&format!("{}", i)).size(16.0).color(Color::WHITE).show();
                                    });
                            }
                        });

                    ui.text("Another item after wrapped container").size(18.0).color(Color::rgb(0.5, 0.5, 0.5)).show();
                });
        })
        .run();
}
