use zenthra::prelude::*;

fn main() {
    let mut is_open = false;
    let mut window_pos = [400.0, 250.0];
    let mut is_dragging = false;
    let mut drag_offset = [0.0, 0.0];

    App::new()
        .title("Zenthra - Floating Menu")
        .size(1200, 900)
        .with_ui(move |ui| {
                // Bright Workspace Background (Deep Indigo/Violet)
                ui.container()
                    .fill()
                    .bg(Color::rgb(0.08, 0.05, 0.15))
                    .padding_all(80.0)
                    .show(|ui| {
                        ui.h1("Neon Menu System").color(Color::rgb(1.0, 0.4, 0.8)).show(); // Neon Pink
                        ui.text("Drag the Action Panel to test the coordinate system.")
                            .color(Color::rgb(0.4, 0.9, 1.0)) // Bright Cyan
                            .show();
                        
                        ui.spacing(30.0);

                        // Main Trigger (Vibrant Cyan/Magenta)
                        if ui.button(if is_open { "CLOSE" } else { "OPEN MENU" })
                            .bg(if is_open { Color::rgb(1.0, 0.1, 0.5) } else { Color::rgb(0.0, 0.8, 1.0) })
                            .radius_all(15.0)
                            .padding(14.0, 32.0, 14.0, 32.0)
                            .text_color(Color::WHITE)
                            .hover_scale(1.08)
                            .show()
                            .clicked 
                        {
                            is_open = !is_open;
                            ui.request_redraw();
                        }

                        // Minimalist Floating Menu (Sharp & High Contrast)
                        if is_open {
                            ui.container()
                                .absolute(window_pos[0], window_pos[1])
                                .width(320.0) 
                                .bg(Color::WHITE)
                                .radius_all(0.0) // SHARP
                                .border(Color::rgb(0.1, 0.1, 0.1), 1.5)
                                .padding_bottom(10.0) 
                                .clip(true)
                                .show(|ui| {
                                    // Header (Dark)
                                    let header = ui.container()
                                        .full_width()
                                        .padding_y(15.0)
                                        .bg(Color::rgb(0.1, 0.1, 0.1))
                                        .halign(Align::Center)
                                        .valign(Align::Center)
                                        .show(|ui| {
                                            ui.text("ACTION MENU")
                                                .wrap(false)
                                                .bold()
                                                .color(Color::WHITE)
                                                .size(16.0)
                                                .show();
                                        });

                                    // Dragging Logic
                                    if header.pressed {
                                        if !is_dragging {
                                            is_dragging = true;
                                            drag_offset = [ui.mouse_x - window_pos[0], ui.mouse_y - window_pos[1]];
                                        }
                                    }
                                    if is_dragging {
                                        if ui.mouse_down {
                                            window_pos[0] = ui.mouse_x - drag_offset[0];
                                            window_pos[1] = ui.mouse_y - drag_offset[1];
                                            ui.request_redraw();
                                        } else { is_dragging = false; }
                                    }

                                    // Menu Items (Text-based List)
                                    ui.container()
                                        .full_width()
                                        .padding_y(10.0)
                                        .halign(Align::Center)
                                        .gap(2.0)
                                        .show(|ui| {
                                            let items = [
                                                "CREATE MAGIC",
                                                "LAUNCH PROJECT",
                                                "EDIT CANVAS",
                                                "EXPLORE DATA",
                                                "PREFERENCES",
                                                "TERMINATE",
                                            ];

                                            for label in items {
                                                if ui.button(label)
                                                    .fill_x()
                                                    .bg(Color::rgb(0.1, 0.1, 0.1))
                                                    .radius_all(0.0)
                                                    .padding_y(15.0)
                                                    .text_color(Color::WHITE)
                                                    .hover_bg(Color::rgb(0.2, 0.2, 0.2))
                                                    .show()
                                                    .clicked 
                                                {
                                                    println!("Action: {}", label);
                                                    is_open = false; 
                                                    ui.request_redraw();
                                                }
                                            }
                                        });
                                });
                        }
                });
        })
        .run();
}
