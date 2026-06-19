mod state;
mod theme;
mod ui_components;

use state::ViewerState;
use theme::{ThemeMode, ThemeColors};
use zenthra::prelude::*;

fn main() {
    env_logger::init();

    // Initialize the modular state
    let mut state = ViewerState::new();
    if let Ok(entries) = std::fs::read_dir("/home/kabir/Downloads/pvt") {
        let mut paths: Vec<_> = entries.filter_map(Result::ok).map(|e| e.path()).collect();
        paths.sort();
        for path in paths {
            if path.is_file() {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                if ext == "jpg" || ext == "jpeg" || ext == "png" || ext == "webp" {
                    state.images.push(path);
                }
            }
        }
    }

    App::new()
        .title("Zenthra — Image Viewer")
        .size(1200, 660)
        .decorations(false)
        .with_ui(move |ui| {
            // Update slideshow progression
            if state.update_slideshow() {
                ui.request_redraw();
            }
            if state.slideshow_active && !state.images.is_empty() {
                let elapsed = state.last_slide_time.elapsed();
                let duration = std::time::Duration::from_secs_f32(3.0).saturating_sub(elapsed);
                ui.request_redraw_after(duration);
            }

            // Expose the active theme flag to interaction_state so widgets can read it
            let theme_val = if state.theme == ThemeMode::Light { 1.0 } else { 0.0 };
            ui.interaction_state.insert(zenthra_core::Id::from_u64(999999999), theme_val);

            let colors = ThemeColors::get(state.theme);

            // Main Background Container
            ui.container()
                .fill()
                .bg(colors.bg_base) // Theme-specific window background
                .show(|ui| {
                    // Draw Custom Title Bar
                    ui_components::draw_title_bar(ui, &mut state);

                    // Draw top menu bar
                    ui_components::draw_menu_bar(ui, &mut state);

                    // Two-column layout: Left Sidebar, Right Content - floating with padding and gaps
                    ui.container()
                        .row()
                        .fill()
                        .padding(12.0, 12.0, 12.0, 12.0)
                        .gap(12.0)
                        .show(|ui| {
                            // Draw Sidebar conditionally
                            if state.sidebar_visible {
                                ui_components::draw_sidebar(ui, &mut state);
                            }

                            // Draw Main Image Viewer
                            ui_components::draw_viewer(ui, &mut state);
                        });

                    // Draw About floating window (overlay, always on top)
                    ui_components::draw_about_window(ui, &mut state);
                });
        })
        .run();
}
