use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Container Showcase")
        .size(800, 600)
        .with_ui(|ui| {
            // Root container
            ui.container()
                .fill()
                .gap(20.0)
                .bg(Color::rgb(0.05, 0.05, 0.07))
                .center()
                .show(|ui| {
                    
                    // A centered row
                    ui.row()
                        .gap(20.0)
                        .padding(30.0)
                        .bg(Color::rgb(0.1, 0.1, 0.15))
                        .radius(15.0)
                        .show(|ui| {
                            
                            // A stylized child container
                            ui.container()
                                .width(200.0)
                                .height(200.0)
                                .bg(Color::rgb(0.2, 0.6, 0.3))
                                .radius(10.0)
                                .center()
                                .show(|ui| {
                                    ui.text("Centered").color(Color::WHITE).show();
                                });

                            ui.container()
                                .width(200.0)
                                .height(200.0)
                                .bg(Color::rgb(0.6, 0.2, 0.3))
                                .radius(10.0)
                                .center()
                                .show(|ui| {
                                    ui.text("Box 2").color(Color::WHITE).show();
                                });
                        });
                });
        })
        .run();
}
