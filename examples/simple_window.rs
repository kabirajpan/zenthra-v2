// examples/simple_window.rs
//
// A simple container window example showing backdrop filter blur.
// Run with: cargo run --example simple_window

use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra — Simple Window")
        .size(800, 600)
        .with_ui(|ui: &mut Ui| {
            ui.container()
                .full_width()
                .full_height()
                .bg(Color::rgba(25.0 / 255.0, 28.0 / 255.0, 36.0 / 255.0, 1.0))
                .show(|ui| {
                    // Draw colorful blobs behind the card to demonstrate blur
                    // Top-left purple blob
                    ui.container()
                        .absolute(120.0, 100.0)
                        .width(280.0)
                        .height(280.0)
                        .bg(Color::rgba(180.0 / 255.0, 80.0 / 255.0, 250.0 / 255.0, 0.8))
                        .radius_all(140.0)
                        .show(|_| {});

                    // Bottom-right cyan blob
                    ui.container()
                        .absolute(400.0, 220.0)
                        .width(300.0)
                        .height(300.0)
                        .bg(Color::rgba(0.0, 200.0 / 255.0, 255.0 / 255.0, 0.7))
                        .radius_all(150.0)
                        .show(|_| {});

                    // Center glassmorphic card on top of the blobs
                    ui.container()
                        .absolute(200.0, 150.0)
                        .width(400.0)
                        .height(300.0)
                        .bg(Color::WHITE)
                        .bg_opacity(0.08) // semi-transparent white tint
                        .backdrop_filter(
                            BackdropFilter::new()
                                .blur(20.0, style::blur::Type::Glassmorphism)
                        )
                        .radius_all(16.0) // Rounded corners
                        .border(Color::rgba(1.0, 1.0, 1.0, 0.12), 1.0) // Subtle border
                        .shadow(Color::rgba(0.0, 0.0, 0.0, 0.4), 0.0, 8.0, 24.0) // Deep drop shadow
                        .padding_all(24.0)
                        .halign(Align::Center)
                        .valign(Align::Center)
                        .gap(12.0)
                        .show(|ui| {
                            // Centered title text
                            ui.text("Zenthra")
                                .size(24.0)
                                .weight(FontWeight::Bold)
                                .color(Color::WHITE)
                                .show();
                            
                            ui.text("A high-performance immediate-mode UI framework.")
                                .size(13.0)
                                .color(Color::rgba(1.0, 1.0, 1.0, 0.6))
                                .show();
                        });
                });
        })
        .run();
}
