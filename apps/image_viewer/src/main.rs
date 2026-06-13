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

    App::new()
        .title("Zenthra — Image Viewer")
        .size(1200, 660)
        .with_ui(move |ui| {
            // Update slideshow progression
            if state.update_slideshow() {
                ui.request_redraw();
            } else if state.slideshow_active {
                ui.request_redraw();
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
