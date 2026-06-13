use crate::state::ViewerState;
use crate::theme::{ThemeMode, ThemeColors};
use std::time::Instant;
use zenthra_core::{Color, ImageSource, ObjectFit};
use zenthra_widgets::Ui;

// Nerd Font Icon constants
pub const NF_FA_ZOOM_IN: &str = "\u{f00e}";
pub const NF_FA_ZOOM_OUT: &str = "\u{f010}";
pub const NF_FA_UNDO: &str = "\u{f0e2}";
pub const NF_FA_REDO: &str = "\u{f01e}";
pub const NF_FA_IMAGE: &str = "\u{f03e}";
pub const NF_FA_SLIDESHOW_PLAY: &str = "\u{f04b}";
pub const NF_FA_SLIDESHOW_PAUSE: &str = "\u{f04c}";
pub const NF_FA_RESET: &str = "\u{f021}";

pub fn draw_menu_bar(ui: &mut Ui, state: &mut ViewerState) {
    ui.menu_bar().show(|ui| {
        ui.menu("File").show(|ui| {
            if ui.menu_item("Open Image...").shortcut("Ctrl+O").show().clicked {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Images", &["jpg", "jpeg", "png", "webp"])
                    .pick_file()
                {
                    state.load_image_file(&path);
                    ui.request_redraw();
                }
            }
            if ui.menu_item("Open Folder...").shortcut("Ctrl+Shift+O").show().clicked {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    state.load_folder(&path);
                    ui.request_redraw();
                }
            }
            let slide_label = if state.slideshow_active { "Pause Slideshow" } else { "Play Slideshow" };
            if ui.menu_item(slide_label).shortcut("Space").show().clicked {
                state.slideshow_active = !state.slideshow_active;
                state.last_slide_time = Instant::now();
            }
            if ui.menu_item("Exit").shortcut("Alt+F4").show().clicked {
                std::process::exit(0);
            }
        });
        ui.menu("View").show(|ui| {
            if ui.menu_item("Zoom In").shortcut("Ctrl++").show().clicked {
                state.zoom += 0.15;
            }
            if ui.menu_item("Zoom Out").shortcut("Ctrl+-").show().clicked {
                state.zoom = (state.zoom - 0.15).max(0.1);
            }
            if ui.menu_item("Reset View").shortcut("Ctrl+0").show().clicked {
                state.reset();
            }
            let sidebar_toggle_label = if state.sidebar_visible { "Hide Sidebar" } else { "Show Sidebar" };
            if ui.menu_item(sidebar_toggle_label).shortcut("Ctrl+B").show().clicked {
                state.sidebar_visible = !state.sidebar_visible;
                ui.request_redraw();
            }
            let theme_label = match state.theme {
                ThemeMode::Dark => "Use Light Theme",
                ThemeMode::Light => "Use Dark Theme",
            };
            if ui.menu_item(theme_label).shortcut("Ctrl+T").show().clicked {
                state.theme = match state.theme {
                    ThemeMode::Dark => ThemeMode::Light,
                    ThemeMode::Light => ThemeMode::Dark,
                };
                ui.request_redraw();
            }
        });
        ui.menu("Image").show(|ui| {
            if ui.menu_item("Rotate Clockwise").shortcut("Ctrl+R").show().clicked {
                state.rotation = (state.rotation + 90.0) % 360.0;
                ui.request_redraw();
            }
            if ui.menu_item("Rotate Counter-Clockwise").shortcut("Ctrl+Shift+R").show().clicked {
                state.rotation = (state.rotation - 90.0 + 360.0) % 360.0;
                ui.request_redraw();
            }
            let gray_label = if state.grayscale_val > 0.5 { "Disable Grayscale" } else { "Enable Grayscale" };
            if ui.menu_item(gray_label).shortcut("Ctrl+G").show().clicked {
                state.grayscale_val = if state.grayscale_val > 0.5 { 0.0 } else { 1.0 };
                ui.request_redraw();
            }
        });
        ui.menu("About").show(|ui| {
            if ui.menu_item("About Zenthra").show().clicked {
                state.show_about = !state.show_about;
                ui.request_redraw();
            }
        });
    });
}

pub fn draw_sidebar(ui: &mut Ui, state: &mut ViewerState) {
    let colors = ThemeColors::get(state.theme);

    ui.container()
        .width(280.0)
        .fill_y()
        .bg(colors.bg_sidebar)
        .radius_all(12.0)
        .border(colors.border, 1.0)
        .padding(20.0, 20.0, 20.0, 20.0)
        .clip(true)
        .show(|ui| {
            // Title & App Branding — centered horizontally
            ui.container()
                .fill_x()
                .halign(zenthra_core::Align::Center)
                .row()
                .gap(8.0)
                .valign(zenthra_core::Align::Center)
                .show(|ui| {
                ui.text("\u{f07b}") // Folder open icon
                    .size(16.0)
                    .color(Color::rgb(70.0 / 255.0, 140.0 / 255.0, 220.0 / 255.0))
                    .show();
                ui.text("Zenthra View")
                    .size(14.0)
                    .weight(zenthra_widgets::text::FontWeight::Bold)
                    .color(colors.text_primary)
                    .show();
            });

            ui.spacing(15.0);

            ui.text(&format!("Total Images: {}", state.images.len()))
                .color(colors.text_muted)
                .size(13.0)
                .show();

            ui.spacing(15.0);

            // Scrollable image file list
            ui.container()
                .fill_x()
                .fill_y()
                .scroll_y(true)
                .padding(0.0, 4.0, 0.0, 4.0)
                .gap(1.0)
                .show(|ui| {
                    if state.images.is_empty() {
                        ui.text("No images found in Pictures directory.")
                            .color(Color::rgb(0.8, 0.4, 0.4))
                            .show();
                    } else {
                        for (idx, path) in state.images.iter().enumerate() {
                            let is_selected = idx == state.selected_idx;
                            let filename = path.file_name()
                                .and_then(|f| f.to_str())
                                .unwrap_or("Unknown Image");

                            // Clean filename truncation to fit sidebar nicely
                            let display_name = if filename.len() > 24 {
                                let ext_len = filename.split('.').last().map(|s| s.len() + 1).unwrap_or(0);
                                let end_start = filename.len().saturating_sub(6 + ext_len);
                                format!("{}...{}", &filename[0..12], &filename[end_start..])
                            } else {
                                filename.to_string()
                            };

                            let btn_label = format!("{} {}", NF_FA_IMAGE, display_name);

                            let mut btn_builder = ui.button(&btn_label)
                                .radius(3.0, 3.0, 3.0, 3.0)
                                .padding(4.0, 8.0, 4.0, 8.0)
                                .size(11.0)
                                .fill_x()
                                .align(zenthra_core::Align::Left);

                            if is_selected {
                                btn_builder = btn_builder
                                    .bg(colors.bg_active)
                                    .text_color(colors.accent)
                                    .hover_bg(colors.bg_active);
                            } else {
                                btn_builder = btn_builder
                                    .bg(Color::TRANSPARENT)
                                    .text_color(colors.text_muted)
                                    .hover_bg(colors.border);
                            }

                            let btn = btn_builder.show();

                            if btn.clicked {
                                state.selected_idx = idx;
                                state.slideshow_active = false;
                            }
                        }
                    }
                });
        });
}

pub fn draw_viewer(ui: &mut Ui, state: &mut ViewerState) {
    let colors = ThemeColors::get(state.theme);

    ui.container()
        .fill()
        .padding(8.0, 8.0, 8.0, 8.0)
        .gap(5.0)
        .clip(true)
        .show(|ui| {
            if state.images.is_empty() {
                ui.container()
                    .fill()
                    .align(zenthra::Align::Center)
                    .show(|ui| {
                        ui.text("Please add JPG or PNG images to /home/kabir/Pictures to preview them.")
                            .color(colors.text_muted)
                            .family("Space Grotesk")
                            .size(16.0)
                            .show();
                    });
            } else {
                let current_path = state.images[state.selected_idx % state.images.len()].clone();
                let filename = current_path.file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or("Unknown File");

                // Image Header bar
                ui.container().row().fill_x().gap(8.0).valign(zenthra_core::Align::Center).show(|ui| {
                    // Sidebar Toggle Button
                    let toggle_icon = if state.sidebar_visible {
                        "\u{ebf3}"
                    } else {
                        "\u{ec02}"
                    };
                    if ui.button(toggle_icon)
                        .bg(colors.bg_panel)
                        .hover_bg(colors.border)
                        .text_color(colors.text_muted)
                        .radius(4.0, 4.0, 4.0, 4.0)
                        .padding(5.0, 8.0, 5.0, 8.0)
                        .size(12.0)
                        .show()
                        .clicked
                    {
                        state.sidebar_visible = !state.sidebar_visible;
                    }

                    ui.column().gap(2.0).show(|ui| {
                        ui.text(filename)
                            .size(13.0)
                            .weight(zenthra_widgets::text::FontWeight::Bold)
                            .color(colors.text_primary)
                            .show();
                        ui.text(&format!("Path: {}", current_path.to_string_lossy()))
                            .size(10.0)
                            .color(colors.text_dim)
                            .monospace()
                            .show();
                    });
                });

                // Floating Toolbar controls
                ui.container()
                    .fill_x()
                    .bg(colors.bg_panel)
                    .radius_all(8.0)
                    .border(colors.border, 1.0)
                    .padding(5.0, 8.0, 5.0, 8.0)
                    .show(|ui| {
                        ui.container()
                            .row()
                            .fill_x()
                            .wrap(zenthra_widgets::container::Wrap::Wrap)
                            .valign(zenthra_core::Align::Center)
                            .gap(6.0)
                            .show(|ui| {
                                // 1. Zoom controls
                                if ui.button(NF_FA_ZOOM_IN)
                                    .bg(Color::TRANSPARENT)
                                    .hover_bg(colors.border)
                                    .text_color(colors.text_muted)
                                    .radius(4.0, 4.0, 4.0, 4.0)
                                    .padding(5.0, 7.0, 5.0, 7.0)
                                    .size(11.0)
                                    .show()
                                    .clicked 
                                {
                                    state.zoom = (state.zoom + 0.1).min(3.0);
                                }

                                if ui.button(NF_FA_ZOOM_OUT)
                                    .bg(Color::TRANSPARENT)
                                    .hover_bg(colors.border)
                                    .text_color(colors.text_muted)
                                    .radius(4.0, 4.0, 4.0, 4.0)
                                    .padding(5.0, 7.0, 5.0, 7.0)
                                    .size(11.0)
                                    .show()
                                    .clicked 
                                {
                                    state.zoom = (state.zoom - 0.1).max(0.2);
                                }

                                ui.text(&format!("Zoom: {:.0}%", state.zoom * 100.0))
                                    .color(colors.text_muted)
                                    .size(11.0)
                                    .monospace()
                                    .show();

                                ui.spacing(3.0);

                                // 2. Rotation controls
                                if ui.button(NF_FA_UNDO)
                                    .bg(Color::TRANSPARENT)
                                    .hover_bg(colors.border)
                                    .text_color(colors.text_muted)
                                    .radius(4.0, 4.0, 4.0, 4.0)
                                    .padding(5.0, 7.0, 5.0, 7.0)
                                    .size(11.0)
                                    .show()
                                    .clicked 
                                {
                                    state.rotation -= 90.0;
                                }

                                if ui.button(NF_FA_REDO)
                                    .bg(Color::TRANSPARENT)
                                    .hover_bg(colors.border)
                                    .text_color(colors.text_muted)
                                    .radius(4.0, 4.0, 4.0, 4.0)
                                    .padding(5.0, 7.0, 5.0, 7.0)
                                    .size(11.0)
                                    .show()
                                    .clicked 
                                {
                                    state.rotation += 90.0;
                                }

                                ui.text(&format!("{}°", state.rotation as i32 % 360))
                                    .color(colors.text_muted)
                                    .size(11.0)
                                    .monospace()
                                    .show();

                                ui.spacing(3.0);

                                // 3. Grayscale toggle
                                let is_grayscale = state.grayscale_val > 0.5;
                                let gray_btn_text = if is_grayscale { "Color" } else { "Mono" };
                                if ui.button(gray_btn_text)
                                    .bg(if is_grayscale { colors.bg_active } else { Color::TRANSPARENT })
                                    .hover_bg(colors.border)
                                    .text_color(if is_grayscale { colors.text_primary } else { colors.text_muted })
                                    .radius(4.0, 4.0, 4.0, 4.0)
                                    .padding(5.0, 8.0, 5.0, 8.0)
                                    .size(11.0)
                                    .show()
                                    .clicked 
                                {
                                    state.grayscale_val = if is_grayscale { 0.0 } else { 1.0 };
                                }

                                // 4. Fit mode toggle
                                let fit_label = match state.fit_mode {
                                    ObjectFit::Contain => "Contain",
                                    ObjectFit::Cover => "Cover",
                                    ObjectFit::Fill => "Stretch",
                                    _ => "Auto",
                                };
                                if ui.button(fit_label)
                                    .bg(Color::TRANSPARENT)
                                    .hover_bg(colors.border)
                                    .text_color(colors.text_muted)
                                    .radius(4.0, 4.0, 4.0, 4.0)
                                    .padding(5.0, 8.0, 5.0, 8.0)
                                    .size(11.0)
                                    .show()
                                    .clicked 
                                {
                                    state.fit_mode = match state.fit_mode {
                                        ObjectFit::Contain => ObjectFit::Cover,
                                        ObjectFit::Cover => ObjectFit::Fill,
                                        ObjectFit::Fill => ObjectFit::Contain,
                                        _ => ObjectFit::Contain,
                                    };
                                }

                                ui.spacing(3.0);

                                // 5. Slideshow toggle
                                let slide_icon = if state.slideshow_active { NF_FA_SLIDESHOW_PAUSE } else { NF_FA_SLIDESHOW_PLAY };
                                let slide_label = if state.slideshow_active { "Pause" } else { "Slideshow" };
                                if ui.button(&format!("{} {}", slide_icon, slide_label))
                                    .bg(if state.slideshow_active { colors.bg_active } else { Color::TRANSPARENT })
                                    .hover_bg(colors.border)
                                    .text_color(if state.slideshow_active { colors.text_primary } else { colors.text_muted })
                                    .radius(4.0, 4.0, 4.0, 4.0)
                                    .padding(5.0, 8.0, 5.0, 8.0)
                                    .size(11.0)
                                    .show()
                                    .clicked
                                {
                                    state.slideshow_active = !state.slideshow_active;
                                    state.last_slide_time = Instant::now();
                                }

                                // 6. Reset
                                if ui.button(&format!("{} Reset", NF_FA_RESET))
                                    .bg(colors.accent)
                                    .hover_bg(colors.accent)
                                    .hover_brightness(0.9)
                                    .text_color(colors.bg_base)
                                    .radius(4.0, 4.0, 4.0, 4.0)
                                    .padding(5.0, 8.0, 5.0, 8.0)
                                    .size(11.0)
                                    .show()
                                    .clicked 
                                {
                                    state.reset();
                                }
                            });
                    });

                // Large Image viewport
                let viewport_height = (ui.max_y - ui.cursor_y - 100.0).max(100.0);
                ui.container()
                    .fill_x()
                    .height(viewport_height)
                    .bg(colors.bg_base)
                    .radius_all(12.0)
                    .border(colors.border, 1.0)
                    .align(zenthra::Align::Center)
                    .clip(true)
                    .show(|ui| {
                        let view_w = ui.available_width;
                        let view_h = ui.max_y - ui.cursor_y;
                        ui.image(ImageSource::Path(current_path.clone()))
                            .size(view_w, view_h)
                            .fit(state.fit_mode)
                            .scale(state.zoom)
                            .rotate_z(state.rotation)
                            .grayscale(state.grayscale_val)
                            .show();
                    });

                // Horizontal Image List Roller (Thumbnail Filmstrip)
                ui.container()
                    .row()
                    .fill_x()
                    .height(90.0)
                    .bg(colors.bg_sidebar)
                    .radius_all(10.0)
                    .border(colors.border, 1.0)
                    .padding(8.0, 8.0, 8.0, 8.0)
                    .scroll_x(true)
                    .gap(8.0)
                    .show(|ui| {
                        for (idx, path) in state.images.iter().enumerate() {
                            let is_selected = idx == state.selected_idx;

                            let (border_color, border_width, opacity) = if is_selected {
                                (colors.accent, 2.0, 1.0f32)
                            } else {
                                (
                                    colors.border,
                                    1.0,
                                    0.6f32
                                )
                            };

                            let thumb = ui.image(ImageSource::Path(path.clone()))
                                .size(100.0, 70.0)
                                .fit(ObjectFit::Cover)
                                .radius_all(6.0)
                                .border(border_color, border_width)
                                .opacity(opacity)
                                .hover_opacity(1.0)
                                .hover_border(colors.accent)
                                .show();

                            if thumb.clicked {
                                state.selected_idx = idx;
                                state.slideshow_active = false;
                            }
                        }
                    });
            }
        });
}

pub fn draw_about_window(ui: &mut Ui, state: &mut ViewerState) {
    if !state.show_about {
        return;
    }

    let colors = ThemeColors::get(state.theme);

    // Window dimensions and position (centered-ish)
    let wx = 390.0;
    let wy = 150.0;
    let ww = 400.0;
    let wh = 260.0;

    // Click outside → close
    if ui.clicked {
        let mx = ui.mouse_x;
        let my = ui.mouse_y;
        if mx < wx || mx > wx + ww || my < wy || my > wy + wh {
            state.show_about = false;
            ui.request_redraw();
            return;
        }
    }

    ui.container()
        .absolute(wx, wy)
        .width(ww)
        .bg(Color::rgb(8.0 / 255.0, 8.0 / 255.0, 8.0 / 255.0))
        .border(Color::rgb(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0), 1.0)
        .radius_all(8.0)
        .clip(true)
        .show(|ui| {
            // ── Title bar ──────────────────────────────────────────────────────
            ui.container()
                .full_width()
                .height(38.0)
                .bg(Color::rgb(12.0 / 255.0, 12.0 / 255.0, 12.0 / 255.0))
                .row()
                .padding(12.0, 8.0, 12.0, 16.0)
                .valign(zenthra_core::Align::Center)
                .show(|ui| {
                    // Title text fills remaining space
                    ui.container()
                        .fill()
                        .valign(zenthra_core::Align::Center)
                        .show(|ui| {
                            ui.text("About")
                                .size(11.5)
                                .color(colors.text_muted)
                                .show();
                        });

                    // Close button — plain text "X" is reliable
                    if ui.button("X")
                        .bg(Color::TRANSPARENT)
                        .hover_bg(Color::rgb(40.0 / 255.0, 40.0 / 255.0, 40.0 / 255.0))
                        .text_color(colors.text_muted)
                        .radius(4.0, 4.0, 4.0, 4.0)
                        .padding(4.0, 10.0, 4.0, 10.0)
                        .size(12.0)
                        .show()
                        .clicked
                    {
                        state.show_about = false;
                        ui.request_redraw();
                    }
                });

            // ── Body ───────────────────────────────────────────────────────────
            ui.container()
                .full_width()
                .padding(20.0, 20.0, 20.0, 20.0)
                .gap(8.0)
                .show(|ui| {
                    ui.text("Zenthra \u{2014} Image Viewer")
                        .size(16.0)
                        .color(colors.text_primary)
                        .show();

                    ui.text("A minimal, fast image viewer built with the Zenthra UI framework.")
                        .size(11.5)
                        .color(colors.text_muted)
                        .show();

                    ui.spacing(8.0);

                    // Divider
                    ui.container()
                        .full_width()
                        .height(1.0)
                        .bg(Color::rgb(20.0 / 255.0, 20.0 / 255.0, 20.0 / 255.0))
                        .show(|_| {});

                    ui.spacing(4.0);

                    // Info rows
                    for (label, value, use_accent) in &[
                        ("Developer", "kabirajpan",            true),
                        ("GitHub",    "github.com/kabirajpan", false),
                        ("Website",   "zenthralabs.dev",        true),
                    ] {
                        ui.container()
                            .row()
                            .fill_x()
                            .gap(12.0)
                            .valign(zenthra_core::Align::Center)
                            .show(|ui| {
                                ui.container().width(60.0).show(|ui| {
                                    ui.text(label)
                                        .size(10.5)
                                        .color(colors.text_dim)
                                        .show();
                                });
                                let col = if *use_accent { colors.accent } else { colors.text_primary };
                                ui.text(value)
                                    .size(11.5)
                                    .color(col)
                                    .monospace()
                                    .show();
                            });
                    }

                    ui.spacing(8.0);

                    ui.text("Built with Zenthra UI \u{00b7} Rust \u{00b7} wgpu")
                        .size(10.0)
                        .color(colors.text_dim)
                        .show();
                });
        });
}
