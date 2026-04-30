use std::path::PathBuf;
use std::sync::Arc;
use zenthra::App;
use zenthra_core::{Color, ImageSource, ObjectFit};

fn main() {
    // We will use the absolute path to the generated image, but for a real project it should be relative.
    let image_path = PathBuf::from(
        "/home/kabir/Pictures/anime-girl-katana-tattoo-hd-wallpaper-uhdpaper.com-228@3@a.jpg",
    );

    // We can also load bytes and use ImageSource::Bytes for demonstration
    let image_bytes = std::fs::read(&image_path).unwrap_or_else(|_| {
        // Fallback dummy 1x1 image if generated one is moved
        vec![
            0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
            0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00,
            0x00, 0x1f, 0x15, 0xc4, 0x89, 0x00, 0x00, 0x00, 0x0b, 0x49, 0x44, 0x41, 0x54, 0x08,
            0xd7, 0x63, 0xf8, 0xcf, 0xc0, 0x00, 0x00, 0x03, 0x01, 0x01, 0x00, 0x18, 0xdd, 0x8d,
            0xb0, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
        ]
    });

    let shared_bytes: Arc<[u8]> = Arc::from(image_bytes.into_boxed_slice());

    App::new()
        .title("Zenthra Image Widget Test")
        .size(800, 800)
        .with_ui(move |ui| {
            ui.container()
                .width(800.0)
                .height(800.0)
                .bg(Color::rgb(0.1, 0.1, 0.12))
                .padding(40.0, 40.0, 40.0, 40.0)
                .show(|ui| {
                    ui.text("Zenthra Image Demonstration")
                        .size(24.0)
                        .weight(zenthra_widgets::text::FontWeight::Bold)
                        .color(Color::WHITE)
                        .show();

                    ui.spacing(30.0);

                    ui.lazy_container()
                        .row()
                        .wrap(zenthra_widgets::container::Wrap::Wrap)
                        .item_size(300.0, 300.0)
                        .count(4)
                        .gap(20.0)
                        .show(|ui, idx| {
                            let source = ImageSource::Bytes(shared_bytes.clone());

                            // Demonstrate different properties
                            match idx {
                                0 => {
                                    // Default image (Contain)
                                    ui.image(source)
                                        .size(300.0, 300.0)
                                        .fit(ObjectFit::Contain)
                                        .border_radius(12.0)
                                        .bg(Color::rgb(0.2, 0.2, 0.2))
                                        .show();
                                }
                                1 => {
                                    // Cover with heavy border and shadow
                                    ui.image(source)
                                        .size(300.0, 300.0)
                                        .fit(ObjectFit::Cover)
                                        .border_radius(24.0)
                                        .border(Color::rgb(0.4, 0.6, 1.0), 4.0)
                                        .shadow(Color::rgba(0.0, 0.0, 0.0, 0.8), 0.0, 10.0, 15.0)
                                        .show();
                                }
                                2 => {
                                    // Interactive image
                                    let resp = ui
                                        .image(source)
                                        .size(300.0, 300.0)
                                        .fit(ObjectFit::Fill)
                                        .border_radius(100.0) // Circle
                                        .grayscale(1.0) // Default grayscale
                                        .hover_grayscale(0.0) // Color on hover
                                        .hover_opacity(0.8)
                                        .active_opacity(0.5)
                                        .cursor(zenthra_widgets::text::CursorIcon::Pointer)
                                        .show();

                                    if resp.clicked {
                                        println!("Circular image clicked!");
                                    }
                                }
                                3 => {
                                    // ScaleDown with padding and custom border color on hover
                                    ui.image(source)
                                        .size(300.0, 300.0)
                                        .fit(ObjectFit::Contain)
                                        .padding(20.0, 20.0, 20.0, 20.0)
                                        .border_radius(12.0)
                                        .bg(Color::WHITE)
                                        .border(Color::TRANSPARENT, 3.0)
                                        .hover_border(Color::rgb(1.0, 0.4, 0.4))
                                        .show();
                                }
                                _ => {}
                            }
                        });
                });
        })
        .run();
}
