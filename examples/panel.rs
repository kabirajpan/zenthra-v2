use zenthra::prelude::*;

fn main() {
    // State for collapsible panels and controls
    let mut preferences_collapsed = false;
    let mut controlled_panel_collapsed = false;
    let mut custom_styled_collapsed = false;
    let mut notifications_enabled = true;
    let mut dark_mode = true;
    let mut hardware_accel = true;
    let mut active_accordion = 0; // 0 = general, 1 = network, 2 = security

    App::new()
        .title("Zenthra - Panel Showcase & Architecture")
        .size(1080, 950)
        .with_ui(move |ui: &mut Ui| {
            // Root scrollable background container
            ui.container()
                .fill()
                .bg(Color::rgb(0.06, 0.07, 0.09))
                .padding_all(28.0)
                .scroll_y(true)
                .gap(24.0)
                .show(|ui: &mut Ui| {
                    // ── HEADER BANNER ─────────────────────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(12.0)
                        .padding_all(20.0)
                        .column()
                        .gap(6.0)
                        .show(|ui: &mut Ui| {
                            ui.text("Panel Widget Architecture")
                                .size(24.0)
                                .font_weight(zenthra_text::prelude::FontWeight::Bold)
                                .color(Color::WHITE)
                                .show();

                            ui.text("A clean, structured container featuring an optional title, subtitle, flush header, and collapsible body. Built strictly on Zenthra's unified BoxStyle architecture.")
                                .size(13.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });

                    // ── 1. BASIC PANEL (STATIC, NOT COLLAPSIBLE) ──────────────
                    ui.panel()
                        .id("basic_panel")
                        .title("System Information")
                        .subtitle("Read-only cluster overview and environment telemetry (static panel)")
                        .fill_x()
                        .show(|ui: &mut Ui| {
                            ui.container()
                                .row()
                                .gap(20.0)
                                .show(|ui: &mut Ui| {
                                    // Stat Item 1
                                    ui.container()
                                        .column()
                                        .gap(4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.text("Status").size(11.0).color(Color::rgb(0.5, 0.55, 0.65)).show();
                                            ui.text("ONLINE").size(13.0).color(Color::rgb(0.3, 0.85, 0.5)).font_weight(zenthra_text::prelude::FontWeight::Bold).show();
                                        });

                                    // Stat Item 2
                                    ui.container()
                                        .column()
                                        .gap(4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.text("Uptime").size(11.0).color(Color::rgb(0.5, 0.55, 0.65)).show();
                                            ui.text("14d 6h 32m").size(13.0).color(Color::WHITE).show();
                                        });

                                    // Stat Item 3
                                    ui.container()
                                        .column()
                                        .gap(4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.text("Engine").size(11.0).color(Color::rgb(0.5, 0.55, 0.65)).show();
                                            ui.text("Zenthra v0.2.4 (WGPU)").size(13.0).color(Color::WHITE).show();
                                        });
                                });
                        });

                    // ── 2. COLLAPSIBLE PANEL (.collapsed(&mut bool)) ───────────
                    ui.panel()
                        .id("preferences_panel")
                        .title("Collapsible Preferences")
                        .subtitle("Click header or chevron icon to expand / fold away content.")
                        .collapsed(&mut preferences_collapsed)
                        .fill_x()
                        .show(|ui: &mut Ui| {
                            ui.container()
                                .column()
                                .gap(12.0)
                                .show(|ui: &mut Ui| {
                                    ui.text("When collapsed, the body is completely skipped, saving layout and render passes.")
                                        .size(13.0)
                                        .color(Color::rgb(0.75, 0.80, 0.88))
                                        .show();

                                    ui.container()
                                        .row()
                                        .gap(12.0)
                                        .show(|ui: &mut Ui| {
                                            ui.checkbox(&mut notifications_enabled, "Enable Notifications").show();
                                            ui.checkbox(&mut dark_mode, "Dark Mode Aesthetics").show();
                                        });
                                });
                        });

                    // ── 3. CONTROLLED PANEL (EXTERNAL BUTTON / VARIABLE) ──────
                    ui.container()
                        .fill_x()
                        .row()
                        .gap(12.0)
                        .valign(Align::Center)
                        .show(|ui: &mut Ui| {
                            ui.text("External Control:").size(13.0).color(Color::WHITE).show();
                            let toggle_text = if controlled_panel_collapsed { "Expand Panel" } else { "Collapse Panel" };
                            if ui.button(toggle_text).padding_all(8.0).show().clicked {
                                controlled_panel_collapsed = !controlled_panel_collapsed;
                            }
                        });

                    ui.panel()
                        .id("controlled_panel")
                        .title("Controlled Panel State")
                        .subtitle(format!("Current state: {}", if controlled_panel_collapsed { "COLLAPSED" } else { "EXPANDED" }))
                        .collapsed(&mut controlled_panel_collapsed)
                        .fill_x()
                        .show(|ui: &mut Ui| {
                            ui.container()
                                .column()
                                .gap(10.0)
                                .show(|ui: &mut Ui| {
                                    ui.text("This panel's collapsed state is bound to `&mut controlled_panel_collapsed`.")
                                        .size(13.0)
                                        .color(Color::rgb(0.75, 0.80, 0.88))
                                        .show();
                                    ui.toggle(&mut hardware_accel, "Hardware Acceleration").show();
                                });
                        });

                    // ── 4. CUSTOM STYLING (BOX STYLE MACROS) ─────────────────
                    ui.panel()
                        .id("custom_styled_panel")
                        .title("Vibrant Custom Box Styling")
                        .subtitle("Demonstrating .bg(), .border(), .radius_all(), .shadow(), and .header_bg()")
                        .bg(Color::rgb(0.08, 0.12, 0.20))
                        .border((Color::rgb(0.25, 0.45, 0.85), 1.5))
                        .radius_all(16.0)
                        .shadow(Color::rgba(0.15, 0.35, 0.75, 0.25), 0.0, 8.0, 20.0)
                        .header_bg(Color::rgb(0.12, 0.18, 0.30))
                        .header_padding(14.0, 18.0, 14.0, 18.0)
                        .padding_all(20.0)
                        .collapsed(&mut custom_styled_collapsed)
                        .fill_x()
                        .show(|ui: &mut Ui| {
                            ui.text("Every BoxStyle method works natively on PanelBuilder thanks to `impl_box_style_methods!`. No wrapper overhead, no non-standard helper methods.")
                                .size(13.0)
                                .color(Color::rgb(0.80, 0.88, 1.0))
                                .show();
                        });

                    // ── 5. ACCORDION GROUP ────────────────────────────────────
                    ui.text("Accordion Group (Exclusive Collapse)")
                        .size(18.0)
                        .font_weight(zenthra_text::prelude::FontWeight::Bold)
                        .color(Color::WHITE)
                        .show();

                    // Accordion Item 1
                    let mut is_gen_collapsed = active_accordion != 0;
                    let resp1 = ui.panel()
                        .id("accordion_general")
                        .title("General Settings")
                        .subtitle("Application startup and workspace defaults")
                        .collapsed(&mut is_gen_collapsed)
                        .fill_x()
                        .show(|ui: &mut Ui| {
                            ui.text("General settings options and configurations are displayed here.")
                                .size(13.0)
                                .color(Color::rgb(0.7, 0.75, 0.85))
                                .show();
                        });
                    if resp1.clicked {
                        active_accordion = if active_accordion == 0 { 99 } else { 0 };
                    }

                    // Accordion Item 2
                    let mut is_net_collapsed = active_accordion != 1;
                    let resp2 = ui.panel()
                        .id("accordion_network")
                        .title("Network & Proxies")
                        .subtitle("Port forwarding, DNS resolvers, and proxy rules")
                        .collapsed(&mut is_net_collapsed)
                        .fill_x()
                        .show(|ui: &mut Ui| {
                            ui.text("Configure outbound HTTP/HTTPS proxies and custom DNS servers.")
                                .size(13.0)
                                .color(Color::rgb(0.7, 0.75, 0.85))
                                .show();
                        });
                    if resp2.clicked {
                        active_accordion = if active_accordion == 1 { 99 } else { 1 };
                    }

                    // Accordion Item 3
                    let mut is_sec_collapsed = active_accordion != 2;
                    let resp3 = ui.panel()
                        .id("accordion_security")
                        .title("Security & Credentials")
                        .subtitle("API tokens, TLS certificates, and session encryption")
                        .collapsed(&mut is_sec_collapsed)
                        .fill_x()
                        .show(|ui: &mut Ui| {
                            ui.text("Manage keyrings, private keys, and automated key rotation policies.")
                                .size(13.0)
                                .color(Color::rgb(0.7, 0.75, 0.85))
                                .show();
                        });
                    if resp3.clicked {
                        active_accordion = if active_accordion == 2 { 99 } else { 2 };
                    }
                });
        })
        .run();
}
