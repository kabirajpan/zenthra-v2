use crate::state::ViewerState;
use crate::theme::{ThemeMode, ThemeColors};
use std::time::Instant;
use zenthra_core::{Color, ImageSource, ObjectFit};
use zenthra_widgets::Ui;
use zenthra_platform::event::PlatformEvent;

// Nerd Font Icon constants
pub const NF_FA_ZOOM_IN: &str = "\u{f00e}";
pub const NF_FA_ZOOM_OUT: &str = "\u{f010}";
pub const NF_FA_UNDO: &str = "\u{f0e2}";
pub const NF_FA_REDO: &str = "\u{f01e}";
pub const NF_FA_IMAGE: &str = "\u{f03e}";
pub const NF_FA_SLIDESHOW_PLAY: &str = "\u{f04b}";
pub const NF_FA_SLIDESHOW_PAUSE: &str = "\u{f04c}";
pub const NF_FA_RESET: &str = "\u{f021}";

// Menu items are now embedded inside draw_title_bar; this is kept for compatibility.
pub fn draw_menu_bar(_ui: &mut Ui, _state: &mut ViewerState) {}

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

            if state.images.is_empty() {
                ui.text("Open a folder to see images here.")
                    .color(colors.text_dim)
                    .size(10.5)
                    .show();
            } else {
                // Virtual list — only renders visible rows, O(viewport/item_h) widgets
                let avail_w = ui.available_width;
                let item_h  = 22.0_f32;
                let selected = state.selected_idx;
                let images   = state.images.as_slice();
                let mut clicked_idx: Option<usize> = None;

                ui.lazy_container()
                    .id("sidebar_file_list")
                    .column()
                    .item_size(avail_w - 8.0, item_h)
                    .count(images.len())
                    .gap(1.0)
                    .show(|ui, idx| {
                        let is_selected = idx == selected;
                        let path = &images[idx];
                        let filename = path.file_name()
                            .and_then(|f| f.to_str())
                            .unwrap_or("Unknown");

                        let display_name = if filename.len() > 24 {
                            let ext_len = filename.split('.').last().map(|s| s.len() + 1).unwrap_or(0);
                            let end_start = filename.len().saturating_sub(6 + ext_len);
                            format!("{}...{}", &filename[0..12], &filename[end_start..])
                        } else {
                            filename.to_string()
                        };

                        let label = format!("{} {}", NF_FA_IMAGE, display_name);

                        let mut btn = ui.button(&label)
                            .id(idx)
                            .radius(3.0, 3.0, 3.0, 3.0)
                            .padding(4.0, 8.0, 4.0, 8.0)
                            .size(11.0)
                            .fill_x()
                            .align(zenthra_core::Align::Left);

                        btn = if is_selected {
                            btn.bg(colors.bg_active)
                               .text_color(colors.accent)
                               .hover_bg(colors.bg_active)
                        } else {
                            btn.bg(Color::TRANSPARENT)
                               .text_color(colors.text_muted)
                               .hover_bg(colors.border)
                        };

                        if btn.show().clicked {
                            clicked_idx = Some(idx);
                        }
                    });

                if let Some(idx) = clicked_idx {
                    state.selected_idx = idx;
                    state.slideshow_active = false;
                }
            }
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
                // ── Welcome / launch screen ────────────────────────────────────
                ui.container()
                    .fill()
                    .align(zenthra::Align::Center)
                    .show(|ui| {
                        ui.column()
                            .gap(6.0)
                            .halign(zenthra_core::Align::Center)
                            .show(|ui| {
                                // App icon / title
                                ui.text("\u{f03e}") // image icon
                                    .size(42.0)
                                    .color(Color::rgb(40.0 / 255.0, 40.0 / 255.0, 40.0 / 255.0))
                                    .show();

                                ui.spacing(4.0);

                                ui.text("Zenthra \u{2014} Image Viewer")
                                    .size(18.0)
                                    .color(colors.text_muted)
                                    .show();

                                ui.text("Open an image or folder to get started")
                                    .size(11.5)
                                    .color(colors.text_dim)
                                    .show();

                                ui.spacing(16.0);

                                // Action buttons row
                                ui.container()
                                    .row()
                                    .gap(10.0)
                                    .halign(zenthra_core::Align::Center)
                                    .show(|ui| {
                                        if ui.button("  \u{f07c}  Open Image")
                                            .bg(colors.accent)
                                            .hover_bg(colors.accent)
                                            .hover_brightness(0.88)
                                            .text_color(Color::BLACK)
                                            .radius(5.0, 5.0, 5.0, 5.0)
                                            .padding(8.0, 18.0, 8.0, 18.0)
                                            .size(12.0)
                                            .show()
                                            .clicked
                                        {
                                            if let Some(path) = rfd::FileDialog::new()
                                                .add_filter("Images", &["jpg", "jpeg", "png", "webp"])
                                                .pick_file()
                                            {
                                                state.load_image_file(&path);
                                                ui.request_redraw();
                                            }
                                        }

                                        if ui.button("  \u{f07b}  Open Folder")
                                            .bg(Color::rgb(18.0 / 255.0, 18.0 / 255.0, 18.0 / 255.0))
                                            .hover_bg(Color::rgb(25.0 / 255.0, 25.0 / 255.0, 25.0 / 255.0))
                                            .text_color(colors.text_muted)
                                            .border(Color::rgb(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0), 1.0)
                                            .radius(5.0, 5.0, 5.0, 5.0)
                                            .padding(8.0, 18.0, 8.0, 18.0)
                                            .size(12.0)
                                            .show()
                                            .clicked
                                        {
                                            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                                                state.load_folder(&path);
                                                ui.request_redraw();
                                            }
                                        }
                                    });
                            });
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

                let container_builder = ui.container()
                    .row()
                    .fill_x()
                    .height(90.0)
                    .bg(colors.bg_sidebar)
                    .radius_all(10.0)
                    .border(colors.border, 1.0)
                    .padding(8.0, 8.0, 8.0, 8.0)
                    .gap(8.0)
                    .align(zenthra_core::Align::Center)
                    .halign(zenthra_core::Align::Center)
                    .id("filmstrip_window");

                container_builder.show(|ui| {
                    let total_images = state.images.len();
                    if total_images == 0 {
                        return;
                    }

                    let item_w = 100.0_f32;
                    let gap = 8.0_f32;
                    let step = item_w + gap;

                    // Calculate how many thumbnails can fit in the available width
                    let viewport_w = ui.available_width;
                    let padding = 16.0_f32;
                    let max_fit = ((viewport_w - padding + gap) / step).floor().max(1.0) as usize;
                    // Limit the window size to a maximum of 12 (or let it fit more if available)
                    let window_size = max_fit.min(12).min(total_images);

                    // 1. Shift the window minimally if selection is close to the bounds, center if far away
                    if state.selected_idx != state.last_selected_idx {
                        let end_idx = state.window_start_idx + window_size;
                        let left_bound = state.window_start_idx + 1;
                        let right_bound = end_idx.saturating_sub(1);

                        if state.selected_idx < left_bound {
                            let diff = (state.window_start_idx as isize - state.selected_idx as isize).abs();
                            if diff <= window_size as isize {
                                state.window_start_idx = state.selected_idx.saturating_sub(1);
                            } else {
                                let half = window_size / 2;
                                let mut s = (state.selected_idx as isize - half as isize).max(0) as usize;
                                if s + window_size > total_images {
                                    s = total_images.saturating_sub(window_size);
                                }
                                state.window_start_idx = s;
                            }
                        } else if state.selected_idx >= right_bound {
                            let diff = (state.selected_idx as isize - right_bound as isize).abs();
                            if diff < window_size as isize {
                                state.window_start_idx = (state.selected_idx + 2).saturating_sub(window_size);
                            } else {
                                let half = window_size / 2;
                                let mut s = (state.selected_idx as isize - half as isize).max(0) as usize;
                                if s + window_size > total_images {
                                    s = total_images.saturating_sub(window_size);
                                }
                                state.window_start_idx = s;
                            }
                        }
                        state.last_selected_idx = state.selected_idx;
                    }

                    // Ensure window_start_idx is in valid range
                    if state.window_start_idx + window_size > total_images {
                        state.window_start_idx = total_images.saturating_sub(window_size);
                    }

                    // 2. Intercept scroll events to shift the sliding window
                    let container_hover = ui.mouse_in_rect(ui.cursor_x, ui.cursor_y, viewport_w, 90.0);
                    if container_hover {
                        let events = std::mem::take(&mut ui.input_events);
                        let mut scroll_delta = 0.0f32;
                        let mut new_events = Vec::new();
                        for event in events {
                            if let PlatformEvent::MouseWheel { delta_y, delta_x, .. } = &event {
                                scroll_delta += if *delta_x != 0.0 { *delta_x } else { *delta_y };
                            } else {
                                new_events.push(event);
                            }
                        }
                        ui.input_events = new_events;

                        if scroll_delta != 0.0 {
                            if scroll_delta > 0.0 {
                                // Scroll up/left: shift window left
                                state.window_start_idx = state.window_start_idx.saturating_sub(1);
                                ui.request_redraw();
                            } else {
                                // Scroll down/right: shift window right
                                if state.window_start_idx + window_size < total_images {
                                    state.window_start_idx += 1;
                                }
                                ui.request_redraw();
                            }
                        }
                    }

                    // 3. Render the window
                    let end_idx = (state.window_start_idx + window_size).min(total_images);
                    let sel = state.selected_idx;

                    for idx in state.window_start_idx..end_idx {
                        let path = &state.images[idx];
                        let is_selected = idx == sel;

                        let (border_color, border_width, opacity) = if is_selected {
                            (colors.accent, 2.0, 1.0f32)
                        } else {
                            (colors.border, 1.0, 0.6f32)
                        };

                        let thumb = ui.image(ImageSource::Thumbnail(path.clone()))
                            .id(idx)
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
    let ww = 400.0;
    let wh = 280.0;

    let mut about_pos = [state.about_x, state.about_y];

    ui.window("About Zenthra", &mut state.show_about, &mut about_pos)
        .size(ww, wh)
        .bg(Color::rgb(8.0 / 255.0, 8.0 / 255.0, 8.0 / 255.0))
        .border(Color::rgb(30.0 / 255.0, 30.0 / 255.0, 30.0 / 255.0), 1.0)
        .radius_all(8.0)
        .header_bg(Color::rgb(12.0 / 255.0, 12.0 / 255.0, 12.0 / 255.0))
        .header_text_color(colors.text_muted)
        .header_height(38.0)
        .closable(true)
        .light_dismiss(true)
        .show(|ui| {
            ui.container()
                .full_width()
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

    state.about_x = about_pos[0];
    state.about_y = about_pos[1];
}

pub fn draw_title_bar(ui: &mut Ui, state: &mut ViewerState) {
    let colors = ThemeColors::get(state.theme);

    // The three window-control buttons are each 20px wide and 20px high.
    // Total right section width = 3 * 20 + 2 * 12px gaps + 12px right padding = 96px (fixed).
    const BTN_W: f32 = 20.0;
    const BAR_H: f32 = 36.0;
    const RIGHT_W: f32 = BTN_W * 3.0 + 36.0; // 96.0

    // Merged title + menu bar: single full-width row
    ui.container()
        .full_width()
        .height(BAR_H)
        .bg(colors.bg_panel)
        .row()
        .valign(zenthra_core::Align::Center)
        .no_wrap()
        .show(|ui| {
            let left_start = ui.cursor_x;

            // LEFT: compact icon  +  inline menu items
            // ─────────────────────────────────────────────────────
            ui.container()
                .row()
                .valign(zenthra_core::Align::Center)
                .padding_left(12.0) // Matching padding on the left edge
                .show(|ui| {
                    // App icon (non-circular)
                    ui.container()
                        .width(28.0)
                        .height(BAR_H)
                        .halign(zenthra_core::Align::Center)
                        .valign(zenthra_core::Align::Center)
                        .show(|ui| {
                            ui.text("\u{f03e}")
                                .size(13.0)
                                .color(colors.accent)
                                .show();
                        });

                    // ── File menu ──────────────────────────────
                    ui.menu("File").show(|ui| {
                        if ui.menu_item("Open Image…").shortcut("Ctrl+O").show().clicked {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("Images", &["jpg", "jpeg", "png", "webp"])
                                .pick_file()
                            {
                                state.load_image_file(&path);
                                ui.request_redraw();
                            }
                        }
                        if ui.menu_item("Open Folder…").shortcut("Ctrl+Shift+O").show().clicked {
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
                            ui.window_actions.push(zenthra_platform::app::WindowAction::Close);
                        }
                    });

                    // ── View menu ──────────────────────────────
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
                        let sidebar_label = if state.sidebar_visible { "Hide Sidebar" } else { "Show Sidebar" };
                        if ui.menu_item(sidebar_label).shortcut("Ctrl+B").show().clicked {
                            state.sidebar_visible = !state.sidebar_visible;
                            ui.request_redraw();
                        }
                        let theme_label = match state.theme {
                            ThemeMode::Dark  => "Use Light Theme",
                            ThemeMode::Light => "Use Dark Theme",
                        };
                        if ui.menu_item(theme_label).shortcut("Ctrl+T").show().clicked {
                            state.theme = match state.theme {
                                ThemeMode::Dark  => ThemeMode::Light,
                                ThemeMode::Light => ThemeMode::Dark,
                            };
                            ui.request_redraw();
                        }
                    });

                    // ── Image menu ─────────────────────────────
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

                    // ── About menu ─────────────────────────────
                    ui.menu("About").show(|ui| {
                        if ui.menu_item("About Zenthra").show().clicked {
                            state.show_about = !state.show_about;
                            ui.request_redraw();
                        }
                    });
                });

            let left_w = ui.cursor_x - left_start;
            let drag_w = (ui.available_width - left_w - RIGHT_W).max(0.0);

            // ─────────────────────────────────────────────────────
            // CENTER: drag zone — fills all remaining width
            // ─────────────────────────────────────────────────────
            let drag_resp = ui.container()
                .width(drag_w)
                .height(BAR_H)
                .halign(zenthra_core::Align::Center)
                .valign(zenthra_core::Align::Center)
                .show(|ui| {
                    let title = if let Some(path) = state.current_image() {
                        if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                            format!("{} \u{2014} Zenthra", filename)
                        } else {
                            "Zenthra Image Viewer".to_string()
                        }
                    } else {
                        "Zenthra Image Viewer".to_string()
                    };

                    ui.text(&title)
                        .size(11.0)
                        .color(colors.text_muted)
                        .show();
                });

            if drag_resp.pressed {
                ui.window_actions.push(zenthra_platform::app::WindowAction::Drag);
            }

            // ─────────────────────────────────────────────────────
            // RIGHT: fixed-width window controls [−] [□] [×]
            // ─────────────────────────────────────────────────────
            ui.container()
                .width(RIGHT_W)
                .height(BAR_H)
                .row()
                .valign(zenthra_core::Align::Center)
                .padding_right(12.0) // Space between close button and window edge
                .gap(12.0) // Space between buttons
                .show(|ui| {
                    // ── Minimize ───────────────────────────────
                    let min_key = zenthra_core::Id::from_u64(999999903);
                    let min_hov = ui.interaction_state.get(&min_key).copied().unwrap_or(0.0) > 0.5;
                    let min_resp = ui.container()
                        .width(BTN_W).height(BTN_W)
                        .radius_all(BTN_W / 2.0)
                        .bg(if min_hov { Color::rgba(1.0, 1.0, 1.0, 0.08) } else { Color::TRANSPARENT })
                        .halign(zenthra_core::Align::Center)
                        .valign(zenthra_core::Align::Center)
                        .show(|ui| {
                            ui.text("\u{f068}").size(8.0).color(colors.text_muted).show();
                        });
                    let h = min_resp.hovered;
                    let prev = ui.interaction_state.insert(min_key, if h { 1.0 } else { 0.0 });
                    if prev != Some(if h { 1.0 } else { 0.0 }) { ui.needs_redraw = true; }
                    if min_resp.clicked {
                        ui.window_actions.push(zenthra_platform::app::WindowAction::Minimize);
                    }

                    // ── Maximize ───────────────────────────────
                    let max_key = zenthra_core::Id::from_u64(999999904);
                    let max_hov = ui.interaction_state.get(&max_key).copied().unwrap_or(0.0) > 0.5;
                    let max_resp = ui.container()
                        .width(BTN_W).height(BTN_W)
                        .radius_all(BTN_W / 2.0)
                        .bg(if max_hov { Color::rgba(1.0, 1.0, 1.0, 0.08) } else { Color::TRANSPARENT })
                        .halign(zenthra_core::Align::Center)
                        .valign(zenthra_core::Align::Center)
                        .show(|ui| {
                            ui.text("\u{f0c8}").size(7.5).color(colors.text_muted).show();
                        });
                    let h = max_resp.hovered;
                    let prev = ui.interaction_state.insert(max_key, if h { 1.0 } else { 0.0 });
                    if prev != Some(if h { 1.0 } else { 0.0 }) { ui.needs_redraw = true; }
                    if max_resp.clicked {
                        ui.window_actions.push(zenthra_platform::app::WindowAction::Maximize);
                    }

                    // ── Close ──────────────────────────────────
                    let cls_key = zenthra_core::Id::from_u64(999999905);
                    let cls_hov = ui.interaction_state.get(&cls_key).copied().unwrap_or(0.0) > 0.5;
                    let cls_resp = ui.container()
                        .width(BTN_W).height(BTN_W)
                        .radius_all(BTN_W / 2.0)
                        .bg(if cls_hov { Color::rgb(0.85, 0.15, 0.15) } else { Color::TRANSPARENT })
                        .halign(zenthra_core::Align::Center)
                        .valign(zenthra_core::Align::Center)
                        .show(|ui| {
                            let fg = if cls_hov { Color::WHITE } else { colors.text_muted };
                            ui.text("\u{f00d}").size(8.0).color(fg).show();
                        });
                    let h = cls_resp.hovered;
                    let prev = ui.interaction_state.insert(cls_key, if h { 1.0 } else { 0.0 });
                    if prev != Some(if h { 1.0 } else { 0.0 }) { ui.needs_redraw = true; }
                    if cls_resp.clicked {
                        ui.window_actions.push(zenthra_platform::app::WindowAction::Close);
                    }
                });
        });

    // Light-dismiss menus when clicking outside them (replicated from MenuBarBuilder)
    {
        let active_menu_key = zenthra_core::Id::from_u64(999999900);
        let active_submenu_key = zenthra_core::Id::from_u64(999999901);
        let hover_flag_key = zenthra_core::Id::from_u64(999999902);
        let active_menu_id = ui.interaction_state.get(&active_menu_key).copied().map(|v| v as u64).unwrap_or(0);
        let hover_flag = ui.interaction_state.get(&hover_flag_key).copied().unwrap_or(0.0) > 0.5;
        if active_menu_id != 0 && ui.clicked && !hover_flag {
            ui.interaction_state.insert(active_menu_key, 0.0);
            ui.interaction_state.insert(active_submenu_key, 0.0);
            ui.needs_redraw = true;
        }
        // Reset hover flag for next frame
        ui.interaction_state.insert(hover_flag_key, 0.0);
    }
}
