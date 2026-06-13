// examples/panel_test.rs

use zenthra::prelude::*;

fn main() {
    let mut show_advanced_settings = false;
    let mut click_count = 0;
    let mut volume = 0.75;
    let mut brightness = 0.5;

    App::new()
        .title("Zenthra Collapsible Panel Demonstration")
        .size(1000, 800)
        .with_ui(move |ui: &mut Ui| {
            ui.container()
                .full_width()
                .full_height()
                .bg(Color::rgb(0.09, 0.09, 0.11))
                .padding_all(24.0)
                .scroll_y(true)
                .gap(20.0)
                .show(|ui: &mut Ui| {
                    // Header / Title section
                    ui.text("Zenthra Panel Layout Suite")
                        .size(28.0)
                        .weight(FontWeight::Bold)
                        .color(Color::WHITE)
                        .show();

                    ui.text("A versatile container widget supporting collapsible states, title bars, action slots, and nesting.")
                        .size(14.0)
                        .color(Color::rgb(0.5, 0.5, 0.6))
                        .show();

                    ui.spacing(15.0);

                    // Row layout with left sidebar and right content area
                    ui.container()
                        .full_width()
                        .row()
                        .gap(24.0)
                        .show(|ui: &mut Ui| {
                            // Left sidebar: Panel Controls (Non-collapsible)
                            ui.panel()
                                .width(280.0)
                                .title("Sidebar Controls")
                                .subtitle("Adjust dashboard settings")
                                .collapsible(false)
                                .show(|ui: &mut Ui| {
                                    ui.text("Dash Controls").bold().color(Color::WHITE).show();
                                    ui.spacing(10.0);
                                    
                                    ui.text(&format!("Click Counter: {}", click_count)).color(Color::rgb(0.8, 0.8, 0.8)).show();
                                    ui.spacing(5.0);
                                    if ui.button("Click Me").show().clicked {
                                        click_count += 1;
                                    }

                                    ui.spacing(20.0);
                                    ui.text("Sidebar state is permanent and does not collapse.").color(Color::rgb(0.5, 0.5, 0.6)).size(11.0).show();
                                });

                            // Right content: Accordion / Multiple panels stacked in Column
                            ui.container()
                                .width(640.0)
                                .column()
                                .gap(16.0)
                                .show(|ui: &mut Ui| {
                                    
                                    // Panel 1: Sound & Display Panel (Internally managed state)
                                    ui.panel()
                                        .title("Sound & Display")
                                        .subtitle("Configure media output levels")
                                        .show(|ui: &mut Ui| {
                                            ui.text(&format!("Output Volume: {:.0}%", volume * 100.0)).color(Color::WHITE).show();
                                            ui.spacing(6.0);
                                            ui.slider(&mut volume, "vol_slider").show();

                                            ui.spacing(15.0);

                                            ui.text(&format!("Screen Brightness: {:.0}%", brightness * 100.0)).color(Color::WHITE).show();
                                            ui.spacing(6.0);
                                            ui.slider(&mut brightness, "bright_slider").show();
                                        });

                                    // Panel 2: Advanced Custom Panel (Externally managed state)
                                    let btn_text = if show_advanced_settings { "Hide Advanced" } else { "Show Advanced" };
                                    if ui.button(btn_text).show().clicked {
                                        show_advanced_settings = !show_advanced_settings;
                                    }
                                    ui.spacing(5.0);

                                    ui.panel()
                                        .title("Advanced Settings")
                                        .subtitle("Externally controlled collapsible panel")
                                        .collapsed(&mut show_advanced_settings)
                                        .show(|ui: &mut Ui| {
                                            ui.text("Feature Toggles").bold().color(Color::WHITE).show();
                                            ui.spacing(10.0);
                                            
                                            ui.text("• Developer Mode: Enabled").color(Color::rgb(0.8, 0.8, 0.8)).show();
                                            ui.spacing(5.0);
                                            ui.text("• Hardware Acceleration: Active").color(Color::rgb(0.8, 0.8, 0.8)).show();
                                        });

                                    // Panel 3: System Overview (Nested Panels Demo)
                                    ui.panel()
                                        .title("System Diagnostics")
                                        .subtitle("Monitor server health metrics")
                                        .show(|ui: &mut Ui| {
                                            ui.text("Cluster Details").bold().color(Color::WHITE).show();
                                            ui.spacing(12.0);

                                            // Nested Panel 3a
                                            ui.panel()
                                                .title("Node 1: Web server")
                                                .subtitle("US-East Primary")
                                                .header_bg(Color::rgb(0.18, 0.18, 0.22))
                                                .bg(Color::rgb(0.14, 0.14, 0.17))
                                                .show(|ui: &mut Ui| {
                                                    ui.text("Status: Running smoothly").color(Color::rgb(0.3, 0.8, 0.3)).show();
                                                    ui.spacing(4.0);
                                                    ui.text("Response time: 42ms").color(Color::rgb(0.8, 0.8, 0.8)).show();
                                                });

                                            ui.spacing(12.0);

                                            // Panel 4: Emoji & Icon Support Demo
                                            ui.panel()
                                                .title("Emoji & UTF Icon Support 🎨🚀")
                                                .subtitle("Testing multi-color emoji rasterization and custom symbols")
                                                .show(|ui: &mut Ui| {
                                                    ui.text("Emoji check: 😀 😃 😄 😁 😆 😅 😂 🤣 😊 😇 🙂 🙃").size(16.0).show();
                                                    ui.spacing(10.0);
                                                    ui.text("Tech stack icons: 🦀 (Rust) ⚙️ (Engine) 🖥️ (Display) 🔋 (Power)").size(16.0).show();
                                                    ui.spacing(10.0);
                                                    ui.text("Nerd Font icons: \u{f015} Home  \u{f007} Profile  \u{f013} Settings  \u{f120} Terminal  \u{e7a8} Rust  \u{f09b} Github").size(16.0).show();
                                                    ui.spacing(10.0);
                                                    ui.text("Status indicators: 🟢 Online  🟡 Warning  🔴 Error  🔵 Information").size(16.0).show();
                                                });

                                            ui.spacing(12.0);

                                            // Nested Panel 3b
                                            ui.panel()
                                                .title("Node 2: Database")
                                                .subtitle("Secondary Replication")
                                                .header_bg(Color::rgb(0.18, 0.18, 0.22))
                                                .bg(Color::rgb(0.14, 0.14, 0.17))
                                                .show(|ui: &mut Ui| {
                                                    ui.text("Status: Syncing...").color(Color::rgb(0.9, 0.6, 0.2)).show();
                                                    ui.spacing(4.0);
                                                    ui.text("Lag: 0.8 seconds").color(Color::rgb(0.8, 0.8, 0.8)).show();
                                                });
                                        });
                                });
                        });
                });
        })
        .run();
}
