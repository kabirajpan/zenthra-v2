use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Container Test")
        .size(800, 600)
        .with_ui(|ui| {
            ui.container(Direction::Row, Wrap::Wrap, |ui| {
                ui.container(Direction::Row, Wrap::Wrap, |ui| {
                    ui.text("Kabiraj Pan").bg(Color::GREEN).show();
                })
                .halign(HAlign::Center)
                .valign(VAlign::Center)
                .height(300.0)
                .width(400.0)
                .bg(Color::WHITE)
                .show();
            })
            .fill()
            .gap(10.0)
            .bg(Color::BLUE)
            .show();
        })
        .run();
}
