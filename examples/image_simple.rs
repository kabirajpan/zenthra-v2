use std::path::PathBuf;
use zenthra::App;
use zenthra_core::{Color, ImageSource, ObjectFit};

fn main() {
    let image_path = PathBuf::from(
        "/home/kabir/Pictures/anime-girl-red-eye-tattoo-hd-wallpaper-uhdpaper.com-733@3@a.jpg",
    );

    App::new()
        .title("Zenthra Simple Image Test")
        .size(1000, 800)
        .with_ui(move |ui| {
            ui.container()
                .fill()
                .align(zenthra::Align::Center)
                .bg(Color::BLUE)
                .show(|ui| {
                    ui.image(ImageSource::Path(image_path.clone()))
                        .height(1080.0 / 2.0)
                        .fit(ObjectFit::Contain) // 👈 This tells Zenthra: "Don't stretch or squeeze anything"
                        .bg(Color::GREEN)
                        .shadow(Color::rgba(0.0, 0.0, 0.0, 0.5), 0.0, 10.0, 20.0)
                        .show();
                });
        })
        .run();
}
