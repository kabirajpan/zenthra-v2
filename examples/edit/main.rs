use zenthra::prelude::*;

fn main() {
    let mut input_text = String::from("Single line input");
    let mut area_text = String::from("Multi-line\nTextArea content");
    let mut slider_val = 0.5;

    App::new()
        .title("Zenthra Test")
        .size(800, 600)
        .with_ui(move |ui| {
            ui.h1("Widget Parity Test").show();

            ui.text("Input (Horizontal Scroll):").show();
            ui.input(&mut input_text, "main-input")
                .size(20.0)
                .text_bg(Color::BLUE)
                .text_bg_full_width(true)
                .bg(Color::RED)
                .padding(10.0, 10.0, 10.0, 10.0)
                .full_width()
                .show();

            ui.spacing(20.0);
            let prev_val = slider_val;
            ui.text(&format!("Slider Value: {:.2}", slider_val)).show();
            ui.slider(&mut slider_val, "main-slider")
                .range(0.0, 1.0)
                .width(400.0)
                .show();

            if (slider_val - prev_val).abs() > 0.001 {
                println!("Slider value changed: {}", slider_val);
            }

            ui.spacing(20.0);
            ui.text("TextArea (Vertical Grid):").show();
            ui.text_area(&mut area_text, "main-area")
                .size(20.0)
                .bg(Color::RED)
                .text_bg(Color::BLUE)
                .highlight(Color::rgb(0.0, 0.5, 0.0))
                .text_bg_full_width(true)
                .text_padding(30.0, 30.0, 30.0, 30.0)
                .padding(10.0, 10.0, 10.0, 10.0)
                .full_width()
                .height(150.0)
                .scrollable(true)
                .show();
        })
        .run();
}
