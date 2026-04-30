use zenthra::prelude::*;

fn main() {
    let mut selected_fruit = "Apple".to_string();
    let fruits = vec!["Apple".to_string(), "Banana".to_string(), "Cherry".to_string(), "Date".to_string(), "Elderberry".to_string()];

    let mut selected_theme = "Dark".to_string();
    let themes = vec!["Dark".to_string(), "Light".to_string(), "System".to_string(), "High Contrast".to_string()];

    App::new()
        .title("Zenthra - Dropdowns")
        .size(800, 600)
        .with_ui(move |ui| {
            ui.container()
                .fill()
                .bg(Color::rgb(0.05, 0.05, 0.07))
                .center()
                .show(|ui| {
                    ui.container()
                        .width(400.0)
                        .padding(40.0, 40.0, 40.0, 40.0)
                        .bg(Color::rgb(0.1, 0.1, 0.12))
                        .radius(20.0, 20.0, 20.0, 20.0)
                        .gap(40.0)
                        .show(|ui| {
                            ui.h1("Dropdown Menus").color(Color::WHITE).show();

                            // --- 1. Standard Dropdown ---
                            ui.column().gap(10.0).show(|ui| {
                                ui.h4("Select Fruit").color(Color::rgb(0.5, 0.5, 0.5)).show();
                                ui.dropdown(&mut selected_fruit, fruits.clone())
                                    .width(320.0)
                                    .show();
                            });

                            // --- 2. Custom Styled ---
                            ui.column().gap(10.0).show(|ui| {
                                ui.h4("App Theme").color(Color::rgb(0.5, 0.5, 0.5)).show();
                                ui.dropdown(&mut selected_theme, themes.clone())
                                    .width(320.0)
                                    .colors(Color::rgb(0.05, 0.05, 0.05), Color::rgb(0.2, 0.5, 1.0))
                                    .show();
                            });

                            // --- 3. Normal Button Below (to test overlap) ---
                            ui.button("Regular Button (Should be behind menu)").show();
                        });
                });
        })
        .run();
}
