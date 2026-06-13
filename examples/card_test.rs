// examples/card_test.rs

use zenthra::prelude::*;

fn main() {
    let mut click_counter_1 = 0;
    let mut click_counter_2 = 0;
    let mut slider_val = 0.5;
    let mut toggle_val = true;

    App::new()
        .title("Zenthra Card Widget Demonstration")
        .size(950, 750)
        .with_ui(move |ui: &mut Ui| {
            ui.container()
                .full_width()
                .full_height()
                .bg(Color::rgb(0.08, 0.08, 0.1))
                .padding_all(24.0)
                .gap(20.0)
                .show(|ui: &mut Ui| {
                    // Title section
                    ui.text("Zenthra Card Component Suite")
                        .size(28.0)
                        .weight(FontWeight::Bold)
                        .color(Color::WHITE)
                        .show();

                    ui.text("Demonstrating layout slots, dividers, actions, footers, and premium hover effects.")
                        .size(14.0)
                        .color(Color::rgb(0.5, 0.5, 0.6))
                        .show();

                    ui.spacing(10.0);

                    // Row layout with multiple cards
                    ui.container()
                        .full_width()
                        .row()
                        .gap(20.0)
                        .show(|ui: &mut Ui| {
                            // Card 1: Basic static information
                            ui.card()
                                .width(280.0)
                                .height(320.0)
                                .show(|ui: &mut Ui| {
                                    ui.card_header("System Status", "Live metrics from cluster", |_| {});
                                    
                                    ui.spacing(10.0);
                                    ui.text("CPU Usage: 42%").color(Color::rgb(0.8, 0.8, 0.8)).show();
                                    ui.spacing(5.0);
                                    ui.text("Memory: 3.4 GB / 8.0 GB").color(Color::rgb(0.8, 0.8, 0.8)).show();
                                    ui.spacing(5.0);
                                    ui.text("Disk read: 12.4 MB/s").color(Color::rgb(0.8, 0.8, 0.8)).show();
                                    
                                    ui.spacing(20.0);
                                    ui.text("All systems operating within normal parameters. Next check in 3 seconds.")
                                        .color(Color::rgb(0.5, 0.5, 0.6))
                                        .size(11.0)
                                        .show();
                                });

                            // Card 2: Interactive with header actions, slider controls, and footer
                            ui.card()
                                .width(320.0)
                                .height(320.0)
                                .show(|ui: &mut Ui| {
                                    ui.card_header("Audio Controller", "Mixer settings", |ui: &mut Ui| {
                                        // Status Badge on Header Right
                                        ui.container()
                                            .bg(Color::rgba(0.2, 0.8, 0.2, 0.15))
                                            .border(Color::rgb(0.2, 0.8, 0.2), 1.0)
                                            .radius_all(10.0)
                                            .padding(2.0, 8.0, 2.0, 8.0)
                                            .show(|ui: &mut Ui| {
                                                ui.text("ACTIVE").size(10.0).color(Color::rgb(0.2, 0.8, 0.2)).bold().show();
                                            });
                                    });

                                    ui.spacing(10.0);
                                    ui.text(&format!("Master Volume: {:.0}%", slider_val * 100.0))
                                        .color(Color::WHITE)
                                        .show();
                                    ui.spacing(5.0);
                                    ui.slider(&mut slider_val, "audio_vol_slider").show();

                                    ui.spacing(15.0);
                                    ui.container()
                                        .row()
                                        .show(|ui: &mut Ui| {
                                            ui.text("Enable Spatializer").color(Color::rgb(0.8, 0.8, 0.8)).show();
                                            ui.spacing(15.0);
                                            ui.toggle(&mut toggle_val, None).show();
                                        });

                                    ui.card_footer(|ui: &mut Ui| {
                                        if ui.button("Mute All").show().clicked {
                                            slider_val = 0.0;
                                        }
                                        ui.spacing(10.0);
                                        if ui.button("Max").show().clicked {
                                            slider_val = 1.0;
                                        }
                                    });
                                });

                            // Card 3: Premium Hover lift effect with interactive buttons
                            ui.card()
                                .width(280.0)
                                .height(320.0)
                                .hover_scale(1.03) // Lift card slightly
                                .hover_border_color(Color::rgb(0.3, 0.5, 1.0)) // Blue border on hover
                                .show(|ui: &mut Ui| {
                                    ui.card_header("Interactive Profile", "User information card", |_| {});

                                    ui.spacing(10.0);
                                    ui.text("Name: Kabir Panwar").bold().color(Color::WHITE).show();
                                    ui.spacing(4.0);
                                    ui.text("Role: Zenthra Architect").color(Color::rgb(0.6, 0.6, 0.7)).show();
                                    ui.spacing(15.0);

                                    ui.text(&format!("Followers: {}", click_counter_1))
                                        .size(12.0)
                                        .color(Color::rgb(0.8, 0.8, 0.8))
                                        .show();
                                    ui.spacing(4.0);
                                    ui.text(&format!("Messages sent: {}", click_counter_2))
                                        .size(12.0)
                                        .color(Color::rgb(0.8, 0.8, 0.8))
                                        .show();

                                    ui.card_footer(|ui: &mut Ui| {
                                        if ui.button("Follow").show().clicked {
                                            click_counter_1 += 1;
                                        }
                                        ui.spacing(10.0);
                                        if ui.button("Message").show().clicked {
                                            click_counter_2 += 1;
                                        }
                                    });
                                });
                        });
                });
        })
        .run();
}
