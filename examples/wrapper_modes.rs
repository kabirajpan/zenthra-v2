use zenthra::prelude::*;

const BG: Color = Color {
    r: 0.08,
    g: 0.08,
    b: 0.10,
    a: 1.0,
};
const PANEL: Color = Color {
    r: 0.14,
    g: 0.14,
    b: 0.18,
    a: 1.0,
};
const BOX_COLOR: Color = Color {
    r: 0.25,
    g: 0.55,
    b: 0.85,
    a: 1.0,
};

fn main() {
    App::new()
        .title("Zenthra – Wrapping Playground")
        .size(800, 600)
        .with_ui(|ui| {
            // Root application background
            ui.container()
                .id("root")
                .fill()
                .bg(BG)
                // We use Align::Center to keep our playground box in the exact middle of the screen
                .align(Align::Center)
                .show(|ui| {
                    // ============================================
                    // THE PLAYGROUND
                    // Tweak any of these properties to experiment!
                    // ============================================
                    ui.container()
                        .id("playground")
                        .width(400.0)
                        .height(300.0)
                        .scroll_y(true)
                        .padding(16.0, 16.0, 16.0, 16.0)
                        .gap(12.0)
                        .bg(PANEL)
                        .radius(8.0, 8.0, 8.0, 8.0)
                        // 1. Try changing between `.row()` and `.column()`
                        .column()
                        // 2. Try changing the wrap strategy:
                        // Options: Wrap::Wrap, Wrap::NoWrap, Wrap::WrapReverse, Wrap::RightToLeft, Wrap::RightToLeftReverse
                        .wrap(Wrap::Wrap)
                        // 3. Try uncommenting scrolling! (Make sure the content spills past the width/height limits first)
                        // .scroll_y(true)
                        // .scroll_x(true)
                        // 4. Try chaining alignment modifiers!
                        // .align_top()
                        // .align_center()
                        // .align_right()
                        .show(|ui| {
                            // Let's spawn 15 boxes inside to see how they wrap around the container
                            for i in 1..=15 {
                                ui.container()
                                    .id(("box", i))
                                    .bg(BOX_COLOR)
                                    // .radius(6.0, 6.0, 6.0, 6.0)
                                    .align(Align::Center)
                                    .show(|ui| {
                                        ui.text(&format!("{}", i))
                                            .size(16.0)
                                            .color(Color::WHITE)
                                            .bold()
                                            .show();
                                    });
                            }
                        });
                });
        })
        .run();
}
