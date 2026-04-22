use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Container Test")
        .size(800, 600)
        .with_ui(|ui| {
            ui.container(Direction::Row, Wrap::RightToLeft, |ui| {
                // 1
                ui.container(Direction::Row, Wrap::NoWrap, |ui| {
                    ui.text("1").size(20.0).color(Color::WHITE).show();
                })
                .halign(HAlign::Center)
                .valign(VAlign::Center)
                .width(100.0)
                .height(60.0)
                .bg(Color::rgb(0.8, 0.2, 0.2))
                .radius(6.0)
                .show();

                // 2
                ui.container(Direction::Row, Wrap::NoWrap, |ui| {
                    ui.text("2").size(20.0).color(Color::WHITE).show();
                })
                .halign(HAlign::Center)
                .valign(VAlign::Center)
                .width(100.0)
                .height(60.0)
                .bg(Color::rgb(0.2, 0.8, 0.2))
                .radius(6.0)
                .show();

                // 3
                ui.container(Direction::Row, Wrap::NoWrap, |ui| {
                    ui.text("3").size(20.0).color(Color::WHITE).show();
                })
                .halign(HAlign::Center)
                .valign(VAlign::Center)
                .width(100.0)
                .height(60.0)
                .bg(Color::rgb(0.2, 0.2, 0.8))
                .radius(6.0)
                .show();

                // 4
                ui.container(Direction::Row, Wrap::NoWrap, |ui| {
                    ui.text("4").size(20.0).color(Color::WHITE).show();
                })
                .halign(HAlign::Center)
                .valign(VAlign::Center)
                .width(100.0)
                .height(60.0)
                .bg(Color::rgb(0.8, 0.8, 0.2))
                .radius(6.0)
                .show();

                // 5
                ui.container(Direction::Row, Wrap::NoWrap, |ui| {
                    ui.text("5").size(20.0).color(Color::WHITE).show();
                })
                .halign(HAlign::Center)
                .valign(VAlign::Center)
                .width(100.0)
                .height(60.0)
                .bg(Color::rgb(0.8, 0.2, 0.8))
                .radius(6.0)
                .show();

                // 6
                ui.container(Direction::Row, Wrap::NoWrap, |ui| {
                    ui.text("6").size(20.0).color(Color::WHITE).show();
                })
                .halign(HAlign::Center)
                .valign(VAlign::Center)
                .width(100.0)
                .height(60.0)
                .bg(Color::rgb(0.2, 0.8, 0.8))
                .radius(6.0)
                .show();

                // 7
                ui.container(Direction::Row, Wrap::NoWrap, |ui| {
                    ui.text("7").size(20.0).color(Color::WHITE).show();
                })
                .halign(HAlign::Center)
                .valign(VAlign::Center)
                .width(100.0)
                .height(60.0)
                .bg(Color::rgb(0.5, 0.5, 0.5))
                .radius(6.0)
                .show();
            })
            .fill()
            .gap(10.0)
            .bg(Color::rgb(0.05, 0.05, 0.07))
            .show();
        })
        .run();
}
