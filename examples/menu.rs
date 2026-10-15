use zenthra::prelude::*;

fn main() {
    // ── Menu Action & Toggle States ──────────────────────────────────────────
    let mut show_grid = true;
    let mut show_minimap = true;
    let mut word_wrap = false;
    let mut dark_theme = true;

    let mut last_action = String::from("Ready. Click any menu item or submenu.");
    let mut action_counter = 0;

    App::new()
        .title("Zenthra - MenuBar & Cascading Menus Showcase")
        .size(1100, 850)
        .with_ui(move |ui: &mut Ui| {
            // Root layout: full-screen column
            ui.container()
                .fill()
                .bg(if dark_theme { Color::rgb(0.06, 0.07, 0.09) } else { Color::rgb(0.12, 0.14, 0.18) })
                .column()
                .show(|ui: &mut Ui| {

                    // ── 1. DESKTOP APPLICATION MENU BAR ─────────────────────────
                    ui.menu_bar()
                        .height(34.0)
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .padding_x(8.0)
                        .show(|ui: &mut Ui| {

                            // ── FILE MENU ─────────────────────────────────────
                            ui.menu("File")
                                .menu_width(220.0)
                                .show(|ui: &mut Ui| {
                                    if ui.menu_item("New Project").shortcut("Ctrl+N").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: File > New Project", action_counter);
                                    }
                                    if ui.menu_item("Open File...").shortcut("Ctrl+O").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: File > Open File...", action_counter);
                                    }

                                    // Cascading Submenu: Open Recent
                                    ui.menu("Open Recent")
                                        .menu_width(200.0)
                                        .show(|ui: &mut Ui| {
                                            if ui.menu_item("zenthra_app.rs").show().clicked {
                                                action_counter += 1;
                                                last_action = format!("#{}: Open Recent > zenthra_app.rs", action_counter);
                                            }
                                            if ui.menu_item("game_engine_ui.rs").show().clicked {
                                                action_counter += 1;
                                                last_action = format!("#{}: Open Recent > game_engine_ui.rs", action_counter);
                                            }
                                            if ui.menu_item("cyberpunk_hud.rs").show().clicked {
                                                action_counter += 1;
                                                last_action = format!("#{}: Open Recent > cyberpunk_hud.rs", action_counter);
                                            }
                                            ui.menu_separator();
                                            if ui.menu_item("Clear Recent List").show().clicked {
                                                action_counter += 1;
                                                last_action = format!("#{}: Open Recent > Clear Recent List", action_counter);
                                            }
                                        });

                                    ui.menu_separator();

                                    if ui.menu_item("Save").shortcut("Ctrl+S").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: File > Save", action_counter);
                                    }
                                    if ui.menu_item("Save As...").shortcut("Ctrl+Shift+S").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: File > Save As...", action_counter);
                                    }

                                    ui.menu_separator();

                                    // Cascading Submenu: Export
                                    ui.menu("Export")
                                        .menu_width(180.0)
                                        .show(|ui: &mut Ui| {
                                            if ui.menu_item("Export as PNG").shortcut("Ctrl+E").show().clicked {
                                                action_counter += 1;
                                                last_action = format!("#{}: Export > PNG Image", action_counter);
                                            }
                                            if ui.menu_item("Export as SVG").show().clicked {
                                                action_counter += 1;
                                                last_action = format!("#{}: Export > Vector SVG", action_counter);
                                            }
                                            if ui.menu_item("Export as PDF Document").show().clicked {
                                                action_counter += 1;
                                                last_action = format!("#{}: Export > PDF Document", action_counter);
                                            }
                                        });

                                    ui.menu_separator();

                                    if ui.menu_item("Exit Application").shortcut("Alt+F4").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: File > Exit Application", action_counter);
                                    }
                                });

                            // ── EDIT MENU ─────────────────────────────────────
                            ui.menu("Edit")
                                .menu_width(210.0)
                                .show(|ui: &mut Ui| {
                                    if ui.menu_item("Undo").shortcut("Ctrl+Z").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: Edit > Undo", action_counter);
                                    }
                                    if ui.menu_item("Redo").shortcut("Ctrl+Y").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: Edit > Redo", action_counter);
                                    }

                                    ui.menu_separator();

                                    if ui.menu_item("Cut").shortcut("Ctrl+X").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: Edit > Cut", action_counter);
                                    }
                                    if ui.menu_item("Copy").shortcut("Ctrl+C").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: Edit > Copy", action_counter);
                                    }
                                    if ui.menu_item("Paste").shortcut("Ctrl+V").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: Edit > Paste", action_counter);
                                    }

                                    ui.menu_separator();

                                    if ui.menu_item("Preferences...").shortcut("Ctrl+,").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: Edit > Preferences...", action_counter);
                                    }
                                });

                            // ── VIEW MENU ─────────────────────────────────────
                            ui.menu("View")
                                .menu_width(220.0)
                                .show(|ui: &mut Ui| {
                                    if ui.menu_item(if show_grid { "✓ Show Grid Canvas" } else { "   Show Grid Canvas" }).show().clicked {
                                        show_grid = !show_grid;
                                        action_counter += 1;
                                        last_action = format!("#{}: View > Toggle Grid ({})", action_counter, if show_grid { "ON" } else { "OFF" });
                                    }
                                    if ui.menu_item(if show_minimap { "✓ Show Minimap" } else { "   Show Minimap" }).show().clicked {
                                        show_minimap = !show_minimap;
                                        action_counter += 1;
                                        last_action = format!("#{}: View > Toggle Minimap ({})", action_counter, if show_minimap { "ON" } else { "OFF" });
                                    }
                                    if ui.menu_item(if word_wrap { "✓ Word Wrap Flow" } else { "   Word Wrap Flow" }).show().clicked {
                                        word_wrap = !word_wrap;
                                        action_counter += 1;
                                        last_action = format!("#{}: View > Toggle Word Wrap ({})", action_counter, if word_wrap { "ON" } else { "OFF" });
                                    }
                                    if ui.menu_item(if dark_theme { "✓ Dark Theme Mode" } else { "   Dark Theme Mode" }).show().clicked {
                                        dark_theme = !dark_theme;
                                        action_counter += 1;
                                        last_action = format!("#{}: View > Toggle Dark Theme ({})", action_counter, if dark_theme { "ON" } else { "OFF" });
                                    }

                                    ui.menu_separator();

                                    if ui.menu_item("Zoom In").shortcut("Ctrl++").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: View > Zoom In", action_counter);
                                    }
                                    if ui.menu_item("Zoom Out").shortcut("Ctrl+-").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: View > Zoom Out", action_counter);
                                    }
                                    if ui.menu_item("Reset Zoom (100%)").shortcut("Ctrl+0").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: View > Reset Zoom", action_counter);
                                    }

                                    ui.menu_separator();

                                    if ui.menu_item("Toggle Fullscreen").shortcut("F11").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: View > Toggle Fullscreen", action_counter);
                                    }
                                });

                            // ── HELP MENU ─────────────────────────────────────
                            ui.menu("Help")
                                .menu_width(200.0)
                                .show(|ui: &mut Ui| {
                                    if ui.menu_item("Documentation").shortcut("F1").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: Help > Documentation", action_counter);
                                    }
                                    if ui.menu_item("Release Notes").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: Help > Release Notes", action_counter);
                                    }
                                    ui.menu_separator();
                                    if ui.menu_item("Check for Updates...").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: Help > Check for Updates...", action_counter);
                                    }
                                    ui.menu_separator();
                                    if ui.menu_item("About Zenthra UI").show().clicked {
                                        action_counter += 1;
                                        last_action = format!("#{}: Help > About Zenthra UI", action_counter);
                                    }
                                });
                        });

                    // ── 2. MAIN APPLICATION WORKSPACE CONTENT ───────────────────
                    ui.container()
                        .fill()
                        .padding_all(28.0)
                        .scroll_y(true)
                        .gap(24.0)
                        .show(|ui: &mut Ui| {

                            // ── ACTION LOG BANNER ─────────────────────────────
                            ui.container()
                                .fill_x()
                                .bg(Color::rgb(0.09, 0.11, 0.16))
                                .border((Color::rgb(0.20, 0.25, 0.35), 1.0))
                                .radius_all(10.0)
                                .padding_all(18.0)
                                .row()
                                .gap(18.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    ui.container()
                                        .bg(Color::rgb(0.15, 0.35, 0.65))
                                        .radius_all(6.0)
                                        .padding(6.0, 12.0, 6.0, 12.0)
                                        .show(|ui: &mut Ui| {
                                            ui.text("EVENT LOG")
                                                .size(12.0)
                                                .font_weight(FontWeight::Bold)
                                                .color(Color::WHITE)
                                                .show();
                                        });

                                    ui.text(&last_action)
                                        .size(14.0)
                                        .font_weight(FontWeight::Bold)
                                        .color(Color::rgb(0.40, 0.85, 1.0))
                                        .show();
                                });

                            // ── ACTIVE TOGGLES DISPLAY ────────────────────────
                            ui.container()
                                .fill_x()
                                .bg(Color::rgb(0.08, 0.09, 0.13))
                                .border((Color::rgb(0.16, 0.20, 0.28), 1.0))
                                .radius_all(10.0)
                                .padding_all(20.0)
                                .column()
                                .gap(14.0)
                                .show(|ui: &mut Ui| {
                                    ui.text("Live Viewport Toggles (Controlled via View Menu)")
                                        .size(16.0)
                                        .font_weight(FontWeight::Bold)
                                        .color(Color::rgb(0.35, 0.65, 1.0))
                                        .show();

                                    ui.row().fill_x().gap(16.0).valign(Align::Center).show(|ui: &mut Ui| {
                                        // Grid Status Card
                                        ui.container()
                                            .bg(if show_grid { Color::rgb(0.10, 0.22, 0.15) } else { Color::rgb(0.14, 0.14, 0.18) })
                                            .border((if show_grid { Color::rgb(0.25, 0.65, 0.35) } else { Color::rgb(0.25, 0.25, 0.30) }, 1.0))
                                            .radius_all(8.0)
                                            .padding_all(14.0)
                                            .column()
                                            .gap(6.0)
                                            .width(220.0)
                                            .show(|ui: &mut Ui| {
                                                ui.text("Grid Canvas").size(12.0).color(Color::rgb(0.70, 0.75, 0.85)).show();
                                                ui.text(if show_grid { "● Active (Visible)" } else { "○ Inactive (Hidden)" })
                                                    .size(14.0)
                                                    .font_weight(FontWeight::Bold)
                                                    .color(if show_grid { Color::rgb(0.40, 0.90, 0.50) } else { Color::rgb(0.55, 0.55, 0.60) })
                                                    .show();
                                            });

                                        // Minimap Status Card
                                        ui.container()
                                            .bg(if show_minimap { Color::rgb(0.10, 0.18, 0.28) } else { Color::rgb(0.14, 0.14, 0.18) })
                                            .border((if show_minimap { Color::rgb(0.25, 0.50, 0.85) } else { Color::rgb(0.25, 0.25, 0.30) }, 1.0))
                                            .radius_all(8.0)
                                            .padding_all(14.0)
                                            .column()
                                            .gap(6.0)
                                            .width(220.0)
                                            .show(|ui: &mut Ui| {
                                                ui.text("Minimap Navigator").size(12.0).color(Color::rgb(0.70, 0.75, 0.85)).show();
                                                ui.text(if show_minimap { "● Enabled" } else { "○ Disabled" })
                                                    .size(14.0)
                                                    .font_weight(FontWeight::Bold)
                                                    .color(if show_minimap { Color::rgb(0.35, 0.75, 1.0) } else { Color::rgb(0.55, 0.55, 0.60) })
                                                    .show();
                                            });

                                        // Word Wrap Status Card
                                        ui.container()
                                            .bg(if word_wrap { Color::rgb(0.22, 0.16, 0.10) } else { Color::rgb(0.14, 0.14, 0.18) })
                                            .border((if word_wrap { Color::rgb(0.70, 0.50, 0.20) } else { Color::rgb(0.25, 0.25, 0.30) }, 1.0))
                                            .radius_all(8.0)
                                            .padding_all(14.0)
                                            .column()
                                            .gap(6.0)
                                            .width(220.0)
                                            .show(|ui: &mut Ui| {
                                                ui.text("Word Wrap").size(12.0).color(Color::rgb(0.70, 0.75, 0.85)).show();
                                                ui.text(if word_wrap { "● Word Boundary" } else { "○ Single Line" })
                                                    .size(14.0)
                                                    .font_weight(FontWeight::Bold)
                                                    .color(if word_wrap { Color::rgb(0.95, 0.75, 0.30) } else { Color::rgb(0.55, 0.55, 0.60) })
                                                    .show();
                                            });
                                    });
                                });

                            // ── ARCHITECTURE HIGHLIGHT CARDS ──────────────────
                            ui.container()
                                .fill_x()
                                .bg(Color::rgb(0.09, 0.11, 0.15))
                                .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                                .radius_all(10.0)
                                .padding_all(20.0)
                                .column()
                                .gap(16.0)
                                .show(|ui: &mut Ui| {
                                    ui.text("MenuBar & Menu Architecture Highlights")
                                        .size(16.0)
                                        .font_weight(FontWeight::Bold)
                                        .color(Color::WHITE)
                                        .show();

                                    ui.row().fill_x().gap(18.0).valign(Align::Top).show(|ui: &mut Ui| {
                                        // Card 1
                                        ui.container()
                                            .width(328.0)
                                            .bg(Color::rgb(0.11, 0.13, 0.18))
                                            .border((Color::rgb(0.20, 0.25, 0.35), 1.0))
                                            .radius_all(8.0)
                                            .padding_all(16.0)
                                            .column()
                                            .gap(8.0)
                                            .show(|ui: &mut Ui| {
                                                ui.text("Cascading Submenus")
                                                    .size(14.0)
                                                    .font_weight(FontWeight::Bold)
                                                    .color(Color::rgb(0.35, 0.75, 1.0))
                                                    .show();

                                                ui.text("Submenus fly out horizontally with automatic edge detection, arrow indicators, and outside-click dismissal across arbitrary nesting levels.")
                                                    .size(13.0)
                                                    .color(Color::rgb(0.65, 0.70, 0.80))
                                                    .show();
                                            });

                                        // Card 2
                                        ui.container()
                                            .width(328.0)
                                            .bg(Color::rgb(0.11, 0.13, 0.18))
                                            .border((Color::rgb(0.20, 0.25, 0.35), 1.0))
                                            .radius_all(8.0)
                                            .padding_all(16.0)
                                            .column()
                                            .gap(8.0)
                                            .show(|ui: &mut Ui| {
                                                ui.text("Top-Level Portal Overlay")
                                                    .size(14.0)
                                                    .font_weight(FontWeight::Bold)
                                                    .color(Color::rgb(0.40, 0.85, 0.50))
                                                    .show();

                                                ui.text("Flyouts render inside Zenthra's floating overlay pass, preventing clipping by parent containers, cards, scrollbars, or sibling layout nodes.")
                                                    .size(13.0)
                                                    .color(Color::rgb(0.65, 0.70, 0.80))
                                                    .show();
                                            });

                                        // Card 3
                                        ui.container()
                                            .width(328.0)
                                            .bg(Color::rgb(0.11, 0.13, 0.18))
                                            .border((Color::rgb(0.20, 0.25, 0.35), 1.0))
                                            .radius_all(8.0)
                                            .padding_all(16.0)
                                            .column()
                                            .gap(8.0)
                                            .show(|ui: &mut Ui| {
                                                ui.text("Single-Source BoxStyle")
                                                    .size(14.0)
                                                    .font_weight(FontWeight::Bold)
                                                    .color(Color::rgb(0.95, 0.75, 0.25))
                                                    .show();

                                                ui.text("MenuBarBuilder, MenuBuilder, and MenuItemBuilder all implement Styleable with canonical macros and zero-cost Sibling interaction states.")
                                                    .size(13.0)
                                                    .color(Color::rgb(0.65, 0.70, 0.80))
                                                    .show();
                                            });
                                    });
                                });
                        });
                });
        })
        .run();
}
