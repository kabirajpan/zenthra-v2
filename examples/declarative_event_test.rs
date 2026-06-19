use std::cell::RefCell;
use std::rc::Rc;
use zenthra::prelude::*;

struct AppState {
    count: i32,
    hover_msg: String,
    text_val: String,
    checked: bool,
    slider_val: f32,
}

fn main() {
    let state = Rc::new(RefCell::new(AppState {
        count: 0,
        hover_msg: "Not Hovering".to_string(),
        text_val: "Hello".to_string(),
        checked: false,
        slider_val: 50.0,
    }));

    App::new()
        .title("Zenthra - Declarative Event Handling")
        .size(800, 600)
        .with_ui(move |ui: &mut Ui| {
            let s1 = state.clone();
            let s2 = state.clone();
            let s3 = state.clone();
            let s4 = state.clone();
            let s5 = state.clone();

            let count = state.borrow().count;
            let hover_msg = state.borrow().hover_msg.clone();
            let checked_val = state.borrow().checked;
            let slider_val = state.borrow().slider_val;
            let text_val = state.borrow().text_val.clone();

            ui.container()
                .fill_x()
                .fill_y()
                .bg(Color::rgb(0.05, 0.05, 0.08))
                .halign(Align::Center)
                .valign(Align::Center)
                .show(|ui| {
                    ui.container()
                        .column()
                        .width(600.0)
                        .gap(20.0)
                        .padding_all(40.0)
                        .bg(Color::rgb(0.1, 0.1, 0.12))
                        .radius_all(20.0)
                        .halign(Align::Center)
                        .show(|ui| {
                            ui.h1("Declarative Event Handling").color(Color::WHITE).show();

                            // 1. Buttons
                            ui.container().row().gap(20.0).show(|ui| {
                                let s = s1.clone();
                                ui.button(&format!("Count: {}", count))
                                    .bg(Color::rgb(0.2, 0.4, 0.8))
                                    .radius_all(8.0)
                                    .on_click(move || {
                                        s.borrow_mut().count += 1;
                                    })
                                    .on_hover(move |hovered| {
                                        if hovered {
                                            s1.borrow_mut().hover_msg = "Hovering Increment!".to_string();
                                        } else {
                                            s1.borrow_mut().hover_msg = "Not Hovering".to_string();
                                        }
                                    })
                                    .show();

                                let s = s2.clone();
                                ui.button("Reset")
                                    .bg(Color::rgb(0.8, 0.2, 0.2))
                                    .radius_all(8.0)
                                    .on_click(move || {
                                        s.borrow_mut().count = 0;
                                    })
                                    .show();
                            });

                            ui.text(&format!("Hover Status: {}", hover_msg))
                                .color(Color::rgb(0.7, 0.7, 0.7))
                                .show();

                            // 2. Checkbox
                            let mut temp_checked = checked_val;
                            let s = s3.clone();
                            ui.checkbox(&mut temp_checked, "Toggle Message")
                                .on_change(move |val| {
                                    s.borrow_mut().checked = val;
                                })
                                .show();

                            if checked_val {
                                ui.text("Checkbox is checked! Yay!")
                                    .color(Color::rgb(0.2, 0.8, 0.2))
                                    .show();
                            }

                            // 3. Slider
                            ui.container().row().gap(10.0).show(|ui| {
                                ui.text(&format!("Slider value: {:.2}", slider_val))
                                    .color(Color::WHITE)
                                    .show();
                                let s = s4.clone();
                                let mut temp_slider = slider_val;
                                ui.slider(&mut temp_slider, "declarative-slider")
                                    .range(0.0, 100.0)
                                    .width(200.0)
                                    .on_change(move |val| {
                                        s.borrow_mut().slider_val = val;
                                    })
                                    .show();
                            });

                            // 4. Input
                            ui.container().row().gap(10.0).show(|ui| {
                                ui.text("Type something: ").color(Color::WHITE).show();
                                let s = s5.clone();
                                let mut temp_text = text_val.clone();
                                ui.input(&mut temp_text, "declarative-input")
                                    .width(200.0)
                                    .on_change(move |val| {
                                        s.borrow_mut().text_val = val;
                                    })
                                    .show();
                            });

                            ui.text(&format!("Current input content: {}", text_val))
                                .color(Color::rgb(0.8, 0.8, 0.8))
                                .show();
                        });
                });
        })
        .run();
}
