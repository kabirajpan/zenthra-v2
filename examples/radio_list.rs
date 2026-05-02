use zenthra::prelude::*;

fn main() {
    let mut selected_theme = 0; // 0: Dark, 1: Light, 2: System
    let mut difficulty = "Normal".to_string();
    let mut color_choice = 0;

    App::new()
        .title("Zenthra - Radio Buttons")
        .size(800, 600)
        .with_ui(move |ui: &mut Ui| {
            ui.container()
                .fill_x()
                .fill_y()
                .bg(Color::rgb(0.05, 0.05, 0.07)) // Deep Dark Background
                .align(Align::Center)
                .show(|ui: &mut Ui| {
                    ui.container()
                        .width(450.0)
                        .padding_all(40.0)
                        .bg(Color::rgb(0.1, 0.1, 0.12))
                        .radius_all(20.0)
                        .border(Color::rgba(1.0, 1.0, 1.0, 0.05), 1.0)
                        .gap(30.0)
                        .show(|ui: &mut Ui| {
                            ui.text("Radio Groups")
                                .size(32.0)
                                .color(Color::WHITE)
                                .show();

                            // --- 1. Theme Selection ---
                            ui.column().gap(12.0).show(|ui: &mut Ui| {
                                ui.text("Select Theme")
                                    .size(14.0)
                                    .color(Color::rgb(0.5, 0.5, 0.5))
                                    .show();
                                ui.radio(&mut selected_theme, 0, "Dark Mode").show();
                                ui.radio(&mut selected_theme, 1, "Light Mode").show();
                                ui.radio(&mut selected_theme, 2, "System Default").show();
                            });

                            // --- 2. String/Enum Based Selection ---
                            ui.column().gap(12.0).show(|ui: &mut Ui| {
                                ui.text("Difficulty")
                                    .size(14.0)
                                    .color(Color::rgb(0.5, 0.5, 0.5))
                                    .show();
                                ui.row().gap(25.0).show(|ui: &mut Ui| {
                                    ui.radio(&mut difficulty, "Easy".to_string(), "Easy").show();
                                    ui.radio(&mut difficulty, "Normal".to_string(), "Normal").show();
                                    ui.radio(&mut difficulty, "Hard".to_string(), "Hard").show();
                                });
                            });

                            // --- 3. Custom Colors ---
                            ui.column().gap(12.0).show(|ui: &mut Ui| {
                                ui.text("Accent Color")
                                    .size(14.0)
                                    .color(Color::rgb(0.5, 0.5, 0.5))
                                    .show();
                                ui.row().gap(25.0).show(|ui: &mut Ui| {
                                    ui.radio(&mut color_choice, 0, "Ocean")
                                        .colors(Color::rgb(0.3, 0.3, 0.3), Color::rgb(0.2, 0.6, 1.0))
                                        .show();
                                    ui.radio(&mut color_choice, 1, "Forest")
                                        .colors(Color::rgb(0.3, 0.3, 0.3), Color::rgb(0.2, 0.8, 0.4))
                                        .show();
                                    ui.radio(&mut color_choice, 2, "Sunset")
                                        .colors(Color::rgb(0.3, 0.3, 0.3), Color::rgb(1.0, 0.4, 0.2))
                                        .show();
                                });
                            });

                            // --- 4. Premium Effects (Opt-in) ---
                            ui.column().gap(12.0).show(|ui: &mut Ui| {
                                ui.text("Premium Effects")
                                    .size(14.0)
                                    .color(Color::rgb(0.5, 0.5, 0.5))
                                    .show();
                                ui.row().gap(25.0).show(|ui: &mut Ui| {
                                    ui.radio(&mut color_choice, 0, "Glow")
                                        .glow(true)
                                        .colors(Color::rgb(0.3, 0.3, 0.3), Color::rgb(0.0, 1.0, 0.8))
                                        .show();
                                    
                                    ui.radio(&mut color_choice, 1, "Zoom")
                                        .scaling(1.15, 0.9)
                                        .show();

                                    ui.radio(&mut color_choice, 2, "Both")
                                        .glow(true)
                                        .scaling(1.15, 0.9)
                                        .colors(Color::rgb(0.3, 0.3, 0.3), Color::rgb(1.0, 0.2, 0.5))
                                        .show();
                                });
                            });

                            // --- 5. Disabled State ---
                            ui.column().gap(12.0).show(|ui: &mut Ui| {
                                ui.text("Status")
                                    .size(14.0)
                                    .color(Color::rgb(0.5, 0.5, 0.5))
                                    .show();
                                // We use a dummy local state for disabled example
                                let mut dummy = 1;
                                ui.radio(&mut dummy, 1, "Connected (Read-only)")
                                    .disabled(true)
                                    .show();
                            });
                        });
                });
        })
        .run();
}
