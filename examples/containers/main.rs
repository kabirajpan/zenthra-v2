use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Container Test")
        .size(800, 600)
        .with_ui(|ui| {
            // Main Outer Scroll View (2D Scrolling enabled)
            ui.container()
                .row()
                .wrap(Wrap::Wrap)
                .fill()
                .scrollable(true, true) // BOTH Horizontal and Vertical!
                .gap(15.0)
                .bg(Color::rgb(0.05, 0.05, 0.07))
                .padding(30.0)
                .show(|ui| {
                    for i in 1..=100 {
                        // Rainbow colors
                        let r = 0.1 + (i as f32 * 0.1).sin().abs() * 0.05;
                        let g = 0.12 + (i as f32 * 0.2).sin().abs() * 0.05;
                        let b = 0.15 + (i as f32 * 0.3).sin().abs() * 0.1;

                        ui.container()
                            .id(i) // STABLE ID
                            .width(150.0) // Wider boxes to force horizontal wrapping
                            .height(100.0)
                            .bg(Color::rgb(r, g, b))
                            .radius(12.0)
                            .valign(Align::Center)
                            .show(|ui| {
                                ui.text(&format!("Box {}", i))
                                    .id(i) // STABLE ID
                                    .size(24.0)
                                    .color(Color::WHITE)
                                    .align_center()
                                    .show();
                            });
                    }
                });
        })
        .run();
}
