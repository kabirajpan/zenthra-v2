use zenthra::prelude::*;

fn main() {
    let mut show_profile = false;
    let mut show_settings = false;
    let mut show_palette = false;

    let mut volume = 72.0f32;
    let mut wifi_enabled = true;
    let mut dark_mode = true;
    let mut selected_accent = Color::rgb(0.38, 0.44, 0.98); // Indigo
    let mut status_msg = String::from("Click any button to open its anchored popover.");

    App::new()
        .title("Zenthra - Popover Architecture & Positioning Showcase")
        .size(1120, 880)
        .with_ui(move |ui: &mut Ui| {
            let bg_canvas = if dark_mode {
                Color::rgb(0.06, 0.07, 0.09)
            } else {
                Color::rgb(0.95, 0.96, 0.98)
            };

            let bg_card = if dark_mode {
                Color::rgb(0.10, 0.12, 0.17)
            } else {
                Color::rgb(1.0, 1.0, 1.0)
            };

            let border_col = if dark_mode {
                Color::rgb(0.18, 0.22, 0.30)
            } else {
                Color::rgb(0.85, 0.88, 0.94)
            };

            let text_main = if dark_mode { Color::WHITE } else { Color::rgb(0.08, 0.10, 0.15) };
            let text_muted = if dark_mode { Color::rgb(0.60, 0.65, 0.75) } else { Color::rgb(0.45, 0.50, 0.60) };

            ui.container()
                .fill()
                .bg(bg_canvas)
                .padding_all(32.0)
                .gap(24.0)
                .scroll_y(true)
                .column()
                .show(|ui: &mut Ui| {

                    // ── 1. HEADER BANNER ─────────────────────────────────────────
                    ui.card()
                        .fill_x()
                        .bg(bg_card)
                        .border((border_col, 1.0))
                        .radius_all(12.0)
                        .padding_all(20.0)
                        .gap(6.0)
                        .show(|ui: &mut Ui| {
                            ui.h2("Popover Widget Architecture")
                                .color(text_main)
                                .show();

                            ui.text("Floating contextual overlays dynamically anchored to trigger widgets with auto-flipping, edge detection, and unified BoxStyle parity.")
                                .size(13.0)
                                .color(text_muted)
                                .show();
                        });

                    // ── 2. INTERACTIVE TRIGGER BAR ───────────────────────────────
                    ui.card()
                        .fill_x()
                        .bg(bg_card)
                        .border((border_col, 1.0))
                        .radius_all(12.0)
                        .padding_all(20.0)
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("Anchored Triggers & Directions")
                                .color(text_main)
                                .show();

                            ui.text("Click each trigger to open its floating card. Notice how Popovers clamp to viewport bounds and auto-flip when space is constrained.")
                                .size(13.0)
                                .color(text_muted)
                                .show();

                            ui.container()
                                .row()
                                .gap(16.0)
                                .wrap(Wrap::Wrap)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {

                                    // Trigger 1: User Profile Flyout (Bottom-Start)
                                    let profile_clicked = ui.button("👤 User Profile")
                                        .id("btn_profile_anchor")
                                        .padding(10.0, 16.0, 10.0, 16.0)
                                        .radius_all(8.0)
                                        .bg(selected_accent)
                                        .color(Color::WHITE)
                                        .hover(Style::bg(selected_accent.with_alpha(0.85)))
                                        .show()
                                        .clicked;

                                    if profile_clicked {
                                        show_profile = !show_profile;
                                        status_msg = format!("Toggled User Profile Popover (State: {})", show_profile);
                                    }

                                    // Trigger 2: Quick Settings (Bottom-Center)
                                    let settings_clicked = ui.button("⚙️ Quick Settings")
                                        .id("btn_settings_anchor")
                                        .padding(10.0, 16.0, 10.0, 16.0)
                                        .radius_all(8.0)
                                        .bg(Color::rgb(0.18, 0.22, 0.30))
                                        .color(Color::WHITE)
                                        .hover(Style::bg(Color::rgb(0.25, 0.30, 0.40)))
                                        .show()
                                        .clicked;

                                    if settings_clicked {
                                        show_settings = !show_settings;
                                        status_msg = format!("Toggled Quick Settings Popover (State: {})", show_settings);
                                    }

                                    // Trigger 3: Theme Palette (Right / Top Placement)
                                    let palette_clicked = ui.button("🎨 Theme Swatches")
                                        .id("btn_palette_anchor")
                                        .padding(10.0, 16.0, 10.0, 16.0)
                                        .radius_all(8.0)
                                        .bg(Color::rgb(0.12, 0.80, 0.54))
                                        .color(Color::WHITE)
                                        .hover(Style::bg(Color::rgb(0.18, 0.88, 0.62)))
                                        .show()
                                        .clicked;

                                    if palette_clicked {
                                        show_palette = !show_palette;
                                        status_msg = format!("Toggled Palette Swatches Popover (State: {})", show_palette);
                                    }
                                });

                            // ── RENDER ACTIVE POPOVERS ───────────────────────────

                            // Popover 1: User Profile Card
                            if show_profile {
                                let profile_btn_id = ui.id_from("btn_profile_anchor");
                                ui.popover(profile_btn_id)
                                    .placement(Placement::Bottom)
                                    .align(PopoverAlign::Start)
                                    .gap(8.0)
                                    .auto_flip(true)
                                    .width(260.0)
                                    .bg(Color::rgb(0.10, 0.12, 0.17))
                                    .border((Color::rgb(0.24, 0.28, 0.38), 1.0))
                                    .radius_all(10.0)
                                    .shadow(Color::rgba(0.0, 0.0, 0.0, 0.6), 0.0, 8.0, 20.0)
                                    .padding_all(16.0)
                                    .show(|ui: &mut Ui| {
                                        ui.container()
                                            .column()
                                            .gap(10.0)
                                            .show(|ui: &mut Ui| {
                                                // User row
                                                ui.container()
                                                    .row()
                                                    .gap(10.0)
                                                    .valign(Align::Center)
                                                    .show(|ui: &mut Ui| {
                                                        ui.container()
                                                            .width(36.0)
                                                            .height(36.0)
                                                            .radius_all(18.0)
                                                            .bg(selected_accent)
                                                            .align(Align::Center)
                                                            .valign(Align::Center)
                                                            .show(|ui: &mut Ui| {
                                                                ui.text("Z").size(18.0).font_weight(FontWeight::Bold).color(Color::WHITE).show();
                                                            });

                                                        ui.container()
                                                            .column()
                                                            .show(|ui: &mut Ui| {
                                                                ui.text("Zenthra Developer").size(14.0).font_weight(FontWeight::Bold).color(Color::WHITE).show();
                                                                ui.text("dev@zenthralabs.dev").size(11.0).color(Color::rgb(0.6, 0.65, 0.75)).show();
                                                            });
                                                    });

                                                ui.card_divider();

                                                if ui.button("View Full Profile").fill_x().padding(6.0, 10.0, 6.0, 10.0).radius_all(6.0).bg(Color::TRANSPARENT).color(Color::WHITE).show().clicked {
                                                    status_msg = "Profile clicked".to_string();
                                                    show_profile = false;
                                                }

                                                if ui.button("Sign Out").fill_x().padding(6.0, 10.0, 6.0, 10.0).radius_all(6.0).bg(Color::rgba(1.0, 0.2, 0.2, 0.15)).color(Color::rgb(1.0, 0.4, 0.4)).show().clicked {
                                                    status_msg = "User signed out".to_string();
                                                    show_profile = false;
                                                }
                                            });
                                    });
                            }

                            // Popover 2: Quick Settings
                            if show_settings {
                                let settings_btn_id = ui.id_from("btn_settings_anchor");
                                ui.popover(settings_btn_id)
                                    .placement(Placement::Bottom)
                                    .align(PopoverAlign::Center)
                                    .gap(8.0)
                                    .auto_flip(true)
                                    .width(280.0)
                                    .bg(Color::rgb(0.10, 0.12, 0.17))
                                    .border((Color::rgb(0.24, 0.28, 0.38), 1.0))
                                    .radius_all(10.0)
                                    .shadow(Color::rgba(0.0, 0.0, 0.0, 0.6), 0.0, 8.0, 20.0)
                                    .padding_all(16.0)
                                    .show(|ui: &mut Ui| {
                                        ui.container()
                                            .column()
                                            .gap(12.0)
                                            .show(|ui: &mut Ui| {
                                                ui.text("Quick Preferences")
                                                    .size(14.0)
                                                    .font_weight(FontWeight::Bold)
                                                    .color(Color::WHITE)
                                                    .show();

                                                ui.toggle(&mut wifi_enabled, "Wireless Adapter")
                                                    .on_color(selected_accent)
                                                    .show();

                                                ui.toggle(&mut dark_mode, "Dark Appearance")
                                                    .on_color(selected_accent)
                                                    .show();

                                                ui.container()
                                                    .fill_x()
                                                    .column()
                                                    .gap(4.0)
                                                    .show(|ui: &mut Ui| {
                                                        ui.text(&format!("Output Volume: {:.0}%", volume))
                                                            .size(12.0)
                                                            .color(Color::rgb(0.7, 0.75, 0.85))
                                                            .show();
                                                        ui.slider(&mut volume, "pop_volume")
                                                            .range(0.0, 100.0)
                                                            .thumb_color(selected_accent)
                                                            .show();
                                                    });

                                                if ui.button("Close").fill_x().padding(6.0, 10.0, 6.0, 10.0).radius_all(6.0).bg(selected_accent).color(Color::WHITE).show().clicked {
                                                    show_settings = false;
                                                }
                                            });
                                    });
                            }

                            // Popover 3: Palette Swatches
                            if show_palette {
                                let palette_btn_id = ui.id_from("btn_palette_anchor");
                                ui.popover(palette_btn_id)
                                    .placement(Placement::Right)
                                    .align(PopoverAlign::Start)
                                    .gap(8.0)
                                    .auto_flip(true)
                                    .width(220.0)
                                    .bg(Color::rgb(0.10, 0.12, 0.17))
                                    .border((Color::rgb(0.24, 0.28, 0.38), 1.0))
                                    .radius_all(10.0)
                                    .shadow(Color::rgba(0.0, 0.0, 0.0, 0.6), 0.0, 8.0, 20.0)
                                    .padding_all(14.0)
                                    .show(|ui: &mut Ui| {
                                        ui.container()
                                            .column()
                                            .gap(10.0)
                                            .show(|ui: &mut Ui| {
                                                ui.text("Pick Accent Color")
                                                    .size(13.0)
                                                    .font_weight(FontWeight::Bold)
                                                    .color(Color::WHITE)
                                                    .show();

                                                let swatches = [
                                                    (Color::rgb(0.38, 0.44, 0.98), "Indigo"),
                                                    (Color::rgb(0.12, 0.80, 0.54), "Emerald"),
                                                    (Color::rgb(0.96, 0.65, 0.14), "Amber"),
                                                    (Color::rgb(0.95, 0.28, 0.45), "Rose"),
                                                    (Color::rgb(0.65, 0.30, 0.95), "Violet"),
                                                ];

                                                for (col, name) in swatches {
                                                    if ui.button(name)
                                                        .fill_x()
                                                        .padding(6.0, 10.0, 6.0, 10.0)
                                                        .radius_all(6.0)
                                                        .bg(col)
                                                        .color(Color::WHITE)
                                                        .show()
                                                        .clicked
                                                    {
                                                        selected_accent = col;
                                                        status_msg = format!("Selected accent: {}", name);
                                                        show_palette = false;
                                                    }
                                                }
                                            });
                                    });
                            }
                        });

                    // ── 3. STATUS BAR ────────────────────────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(bg_card)
                        .border((border_col, 1.0))
                        .radius_all(8.0)
                        .padding_all(12.0)
                        .row()
                        .valign(Align::Center)
                        .show(|ui: &mut Ui| {
                            ui.text(&format!("Event Log: {}", status_msg))
                                .size(12.0)
                                .color(text_muted)
                                .show();
                        });
                });
        })
        .run();
}
