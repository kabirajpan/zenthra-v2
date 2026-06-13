use zenthra::prelude::*;

fn main() {
    let mut last_action = "No action yet".to_string();

    App::new()
        .title("Zenthra - Menu Bar & Sub-menus")
        .size(800, 600)
        .with_ui(move |ui: &mut Ui| {
            // Main Window Container
            ui.container()
                .fill_x()
                .fill_y()
                .bg(Color::rgb(0.06, 0.06, 0.08))
                .show(|ui: &mut Ui| {
                    // --- 1. Top Menu Bar ---
                    ui.menu_bar().show(|ui: &mut Ui| {
                        // File Menu
                        ui.menu("File").show(|ui: &mut Ui| {
                            if ui.menu_item("New").shortcut("Ctrl+N").show().clicked {
                                last_action = "File -> New".to_string();
                            }
                            if ui.menu_item("Open...").shortcut("Ctrl+O").show().clicked {
                                last_action = "File -> Open...".to_string();
                            }
                            
                            // Nested Open Recent Submenu
                            ui.sub_menu("Open Recent").show(|ui: &mut Ui| {
                                if ui.menu_item("Project Alpha").show().clicked {
                                    last_action = "File -> Open Recent -> Project Alpha".to_string();
                                }
                                if ui.menu_item("Project Beta").show().clicked {
                                    last_action = "File -> Open Recent -> Project Beta".to_string();
                                }
                                if ui.menu_item("Clear History").show().clicked {
                                    last_action = "File -> Open Recent -> Clear History".to_string();
                                }
                            });

                            if ui.menu_item("Save").shortcut("Ctrl+S").show().clicked {
                                last_action = "File -> Save".to_string();
                            }
                            
                            ui.spacing(4.0); // Separator spacing
                            
                            if ui.menu_item("Exit").shortcut("Alt+F4").show().clicked {
                                std::process::exit(0);
                            }
                        });

                        // Edit Menu
                        ui.menu("Edit").show(|ui: &mut Ui| {
                            if ui.menu_item("Cut").shortcut("Ctrl+X").show().clicked {
                                last_action = "Edit -> Cut".to_string();
                            }
                            if ui.menu_item("Copy").shortcut("Ctrl+C").show().clicked {
                                last_action = "Edit -> Copy".to_string();
                            }
                            if ui.menu_item("Paste").shortcut("Ctrl+V").show().clicked {
                                last_action = "Edit -> Paste".to_string();
                            }
                        });

                        // Help Menu
                        ui.menu("Help").show(|ui: &mut Ui| {
                            if ui.menu_item("Documentation").show().clicked {
                                last_action = "Help -> Documentation".to_string();
                            }
                            if ui.menu_item("About Zenthra").show().clicked {
                                last_action = "Help -> About Zenthra".to_string();
                            }
                        });
                    });

                    // --- 2. Body / Content Area ---
                    ui.container()
                        .fill_x()
                        .fill_y()
                        .align(Align::Center)
                        .show(|ui: &mut Ui| {
                            ui.container()
                                .width(450.0)
                                .padding_all(30.0)
                                .bg(Color::rgb(0.1, 0.1, 0.13))
                                .radius_all(12.0)
                                .border(Color::rgb(0.2, 0.2, 0.25), 1.0)
                                .gap(20.0)
                                .show(|ui: &mut Ui| {
                                    ui.text("Menu Bar Demonstration")
                                        .size(24.0)
                                        .color(Color::WHITE)
                                        .show();

                                    ui.text("Click any item in the menu bar at the top to trigger actions. Hovering over 'Open Recent' will show the nested sub-menus.")
                                        .size(14.0)
                                        .color(Color::rgb(0.6, 0.6, 0.7))
                                        .show();

                                    ui.spacing(10.0);

                                    // Display the last triggered action
                                    ui.column().gap(8.0).show(|ui: &mut Ui| {
                                        ui.text("Last Triggered Action:")
                                            .size(12.0)
                                            .color(Color::rgb(0.4, 0.7, 1.0))
                                            .show();

                                        ui.text(&last_action)
                                            .size(16.0)
                                            .color(Color::rgb(0.2, 0.8, 0.4))
                                            .show();
                                    });

                                    // Add a regular button to ensure occlusion logic doesn't bleed through
                                    if ui.button("Interactive Test Button").show().clicked {
                                        last_action = "Test Button Clicked".to_string();
                                    }
                                });
                        });
                });
        })
        .run();
}
