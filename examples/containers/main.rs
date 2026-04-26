use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - High Performance Containers")
        .size(800, 600)
        .with_ui(|ui| {
            // Now we use lazy_container directly with styling!
            ui.lazy_container()
                .id("main_lazy_list")
                .bg(Color::rgb(0.05, 0.05, 0.07)) // Built-in background
                .padding(10.0) // Built-in padding
                .item_size(150.0, 100.0)
                .gap(15.0)
                .row()
                .wrap(Wrap::Wrap)
                .count(100)
                .show(|ui, i| {
                    let i = i + 1; // 1-indexed for display

                    let r = 0.1 + (i as f32 * 0.1).sin().abs() * 0.05;
                    let g = 0.12 + (i as f32 * 0.2).sin().abs() * 0.05;
                    let b = 0.15 + (i as f32 * 0.3).sin().abs() * 0.1;

                    ui.container()
                        .id(i)
                        .width(150.0)
                        .height(100.0)
                        .bg(Color::rgb(r, g, b))
                        .radius(4.0)
                        .align(Align::Center)
                        .show(|ui| {
                            ui.text(&format!("Box {}", i))
                                .id(i)
                                .size(24.0)
                                .color(Color::WHITE)
                                .show();
                        });
                });
        })
        .run();
}
