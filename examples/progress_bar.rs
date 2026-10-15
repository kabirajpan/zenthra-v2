use zenthra::prelude::*;

fn main() {
    let mut progress_val: f32 = 0.65;
    let mut download_progress: f32 = 0.42;
    let mut is_animating = true;

    App::new()
        .title("Zenthra - Progress Bar API Showcase")
        .size(1050, 920)
        .with_ui(move |ui: &mut Ui| {
            // Animate progress smoothly when active
            if is_animating {
                progress_val = (progress_val + 0.002) % 1.0;
                download_progress = (download_progress + 0.001) % 1.0;
                ui.request_redraw();
            }

            // Root scrollable background container
            ui.container()
                .fill()
                .bg(Color::rgb(0.06, 0.07, 0.09))
                .padding_all(24.0)
                .scroll_y(true)
                .gap(24.0)
                .show(|ui: &mut Ui| {

                    // ── HEADER ────────────────────────────────────────────────
                    ui.container()
                        .fill_x()
                        .column()
                        .gap(6.0)
                        .show(|ui: &mut Ui| {
                            ui.h1("Progress Bar API & Styling Showcase")
                                .color(Color::WHITE)
                                .show();

                            ui.text("Demonstrating smooth progress indicators, gradient fills, animated shimmer highlights, track padding, customizable corner radii, and glow effects.")
                                .size(14.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });

                    // ── STATUS BAR & ANIMATION CONTROLS ───────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.10, 0.12, 0.17))
                        .border((Color::rgb(0.20, 0.25, 0.35), 1.0))
                        .radius_all(8.0)
                        .padding_all(16.0)
                        .row()
                        .gap(24.0)
                        .valign(Align::Center)
                        .show(|ui: &mut Ui| {
                            ui.text(&format!("Progress: {:.0}%", progress_val * 100.0))
                                .size(15.0)
                                .font_weight(FontWeight::Bold)
                                .color(Color::rgb(0.35, 0.75, 1.0))
                                .show();

                            ui.text(&format!("Download: {:.1}%", download_progress * 100.0))
                                .size(14.0)
                                .font_weight(FontWeight::Medium)
                                .color(Color::rgb(0.20, 0.85, 0.50))
                                .show();

                            ui.toggle(&mut is_animating, "Auto-Simulate Progress")
                                .size(13.0)
                                .active(Text::color(Color::rgb(0.40, 0.85, 1.0)))
                                .show();

                            if ui.button("Reset (0%)")
                                .bg(Color::rgb(0.18, 0.20, 0.26))
                                .hover(Style::bg(Color::rgb(0.25, 0.28, 0.36)))
                                .padding(6.0, 12.0, 6.0, 12.0)
                                .radius_all(6.0)
                                .show()
                                .clicked
                            {
                                progress_val = 0.0;
                                download_progress = 0.0;
                            }

                            if ui.button("Complete (100%)")
                                .bg(Color::rgb(0.18, 0.20, 0.26))
                                .hover(Style::bg(Color::rgb(0.25, 0.28, 0.36)))
                                .padding(6.0, 12.0, 6.0, 12.0)
                                .radius_all(6.0)
                                .show()
                                .clicked
                            {
                                progress_val = 1.0;
                                download_progress = 1.0;
                            }
                        });

                    // ── SECTION 1: DEFAULT & SIZED PROGRESS BARS ──────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(18.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("1. Standard Progress Indicators with Percentage Labels")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            // Slim 4px Bar
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                    ui.text("Slim Track (4px)").size(13.0).color(Color::rgb(0.70, 0.75, 0.85)).show();
                                    ui.text(&format!("{:.0}%", progress_val * 100.0)).size(12.0).color(Color::rgb(0.50, 0.55, 0.65)).show();
                                });
                                ui.progress_bar(progress_val)
                                    .track_height(4.0)
                                    .fill_color(Color::rgb(0.35, 0.65, 1.0))
                                    .track_color(Color::rgb(0.14, 0.16, 0.22))
                                    .radius_all(2.0)
                                    .show();
                            });

                            // Standard 8px Bar
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                    ui.text("Standard Track (8px)").size(13.0).color(Color::rgb(0.70, 0.75, 0.85)).show();
                                    ui.text(&format!("{:.0}%", progress_val * 100.0)).size(12.0).color(Color::rgb(0.50, 0.55, 0.65)).show();
                                });
                                ui.progress_bar(progress_val)
                                    .track_height(8.0)
                                    .fill_color(Color::rgb(0.30, 0.70, 1.0))
                                    .track_color(Color::rgb(0.14, 0.16, 0.22))
                                    .radius_all(4.0)
                                    .show();
                            });

                            // Thick 16px Hero Bar
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                    ui.text("Thick Hero Track (16px)").size(13.0).color(Color::rgb(0.70, 0.75, 0.85)).show();
                                    ui.text(&format!("{:.0}%", progress_val * 100.0)).size(12.0).color(Color::rgb(0.50, 0.55, 0.65)).show();
                                });
                                ui.progress_bar(progress_val)
                                    .track_height(16.0)
                                    .fill_color(Color::rgb(0.20, 0.80, 0.50))
                                    .track_color(Color::rgb(0.12, 0.14, 0.18))
                                    .radius_all(8.0)
                                    .show();
                            });
                        });

                    // ── SECTION 2: VIBRANT GRADIENT FILLS & NEON GLOW ──────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(18.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("2. Vibrant Gradients & Fill Shadows")
                                .color(Color::rgb(0.10, 0.80, 0.50))
                                .show();

                            // Ocean Cyan to Blue Gradient
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.text("Ocean Breeze (Horizontal Gradient)").size(13.0).color(Color::rgb(0.70, 0.75, 0.85)).show();
                                ui.progress_bar(progress_val)
                                    .track_height(10.0)
                                    .fill_gradient(Color::rgb(0.10, 0.85, 0.80), Color::rgb(0.25, 0.50, 1.0), GradientDirection::Horizontal)
                                    .track_color(Color::rgb(0.12, 0.14, 0.19))
                                    .fill_shadow(Color::rgba(0.20, 0.60, 1.0, 0.40), 0.0, 2.0, 8.0)
                                    .radius_all(5.0)
                                    .show();
                            });

                            // Sunset Orange to Purple Gradient
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.text("Sunset Glow (Amber to Crimson)").size(13.0).color(Color::rgb(0.70, 0.75, 0.85)).show();
                                ui.progress_bar(download_progress)
                                    .track_height(10.0)
                                    .fill_gradient(Color::rgb(0.95, 0.65, 0.20), Color::rgb(0.95, 0.25, 0.45), GradientDirection::Horizontal)
                                    .track_color(Color::rgb(0.12, 0.14, 0.19))
                                    .fill_shadow(Color::rgba(0.95, 0.40, 0.30, 0.40), 0.0, 2.0, 8.0)
                                    .radius_all(5.0)
                                    .show();
                            });

                            // Neon Cyberpunk Violet
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.text("Cyberpunk Neon (Violet with Glow)").size(13.0).color(Color::rgb(0.70, 0.75, 0.85)).show();
                                ui.progress_bar(progress_val)
                                    .track_height(10.0)
                                    .fill_gradient(Color::rgb(0.55, 0.20, 0.95), Color::rgb(0.85, 0.35, 1.0), GradientDirection::Horizontal)
                                    .track_color(Color::rgb(0.12, 0.14, 0.19))
                                    .fill_shadow(Color::rgba(0.70, 0.30, 1.0, 0.50), 0.0, 2.0, 10.0)
                                    .radius_all(5.0)
                                    .show();
                            });
                        });

                    // ── SECTION 3: ANIMATED SHIMMER EFFECT ────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(18.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("3. Animated Shimmer Light Highlights (.shimmer(true))")
                                .color(Color::rgb(0.95, 0.65, 0.25))
                                .show();

                            ui.text("The shimmer effect renders a high-brightness light beam that continuously sweeps across the filled portion of the bar.")
                                .size(13.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();

                            // Emerald with Shimmer
                            ui.progress_bar(progress_val)
                                .track_height(12.0)
                                .fill_color(Color::rgb(0.15, 0.80, 0.50))
                                .track_color(Color::rgb(0.12, 0.15, 0.20))
                                .fill_shadow(Color::rgba(0.15, 0.80, 0.50, 0.35), 0.0, 2.0, 8.0)
                                .radius_all(6.0)
                                .shimmer(true)
                                .show();

                            // Blue Gradient with Shimmer
                            ui.progress_bar(download_progress)
                                .track_height(14.0)
                                .fill_gradient(Color::rgb(0.20, 0.55, 1.0), Color::rgb(0.40, 0.75, 1.0), GradientDirection::Horizontal)
                                .track_color(Color::rgb(0.12, 0.14, 0.20))
                                .fill_shadow(Color::rgba(0.30, 0.65, 1.0, 0.40), 0.0, 2.0, 8.0)
                                .radius_all(7.0)
                                .shimmer(true)
                                .show();
                        });

                    // ── SECTION 4: CONTAINER CARD & EMBEDDED PROGRESS ──────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("4. Card Layout: Real-Time Download Manager")
                                .color(Color::rgb(0.70, 0.45, 1.0))
                                .show();

                            // Card container
                            ui.container()
                                .fill_x()
                                .bg(Color::rgb(0.12, 0.14, 0.19))
                                .border((Color::rgb(0.22, 0.25, 0.35), 1.0))
                                .radius_all(8.0)
                                .padding_all(16.0)
                                .column()
                                .gap(12.0)
                                .show(|ui: &mut Ui| {
                                    ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                        ui.text("zenthra_v2_release_amd64.tar.gz")
                                            .size(14.0)
                                            .font_weight(FontWeight::Medium)
                                            .color(Color::WHITE)
                                            .show();

                                        ui.text(&format!("{:.1} MB / 148.0 MB", download_progress * 148.0))
                                            .size(12.0)
                                            .color(Color::rgb(0.60, 0.65, 0.75))
                                            .show();
                                    });

                                    ui.progress_bar(download_progress)
                                        .track_height(8.0)
                                        .fill_gradient(Color::rgb(0.20, 0.60, 1.0), Color::rgb(0.40, 0.85, 0.90), GradientDirection::Horizontal)
                                        .track_color(Color::rgb(0.18, 0.20, 0.26))
                                        .radius_all(4.0)
                                        .shimmer(true)
                                        .show();

                                    ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                        ui.text("Speed: 14.2 MB/s • ETA: 4s")
                                            .size(12.0)
                                            .color(Color::rgb(0.50, 0.55, 0.65))
                                            .show();

                                        ui.text(&format!("{:.0}%", download_progress * 100.0))
                                            .size(13.0)
                                            .font_weight(FontWeight::Bold)
                                            .color(Color::rgb(0.35, 0.75, 1.0))
                                            .show();
                                    });
                                });
                        });

                    // ── SECTION 5: SHAPES & CORNER RADII ───────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("5. Corner Radii: Flat, Rounded, and Full Capsule")
                                .color(Color::rgb(0.40, 0.85, 0.90))
                                .show();

                            // Sharp Flat Rect (0px)
                            ui.column().gap(4.0).show(|ui: &mut Ui| {
                                ui.text("Sharp Corners (0px radius)").size(12.0).color(Color::rgb(0.60, 0.65, 0.75)).show();
                                ui.progress_bar(progress_val)
                                    .track_height(10.0)
                                    .radius_all(0.0)
                                    .fill_color(Color::rgb(0.95, 0.60, 0.20))
                                    .track_color(Color::rgb(0.14, 0.16, 0.22))
                                    .show();
                            });

                            // Smooth 4px
                            ui.column().gap(4.0).show(|ui: &mut Ui| {
                                ui.text("Slight Rounding (4px radius)").size(12.0).color(Color::rgb(0.60, 0.65, 0.75)).show();
                                ui.progress_bar(progress_val)
                                    .track_height(10.0)
                                    .radius_all(4.0)
                                    .fill_color(Color::rgb(0.25, 0.70, 0.95))
                                    .track_color(Color::rgb(0.14, 0.16, 0.22))
                                    .show();
                            });

                            // Full Capsule Pill (5px on 10px track)
                            ui.column().gap(4.0).show(|ui: &mut Ui| {
                                ui.text("Full Capsule Pill (height / 2 radius)").size(12.0).color(Color::rgb(0.60, 0.65, 0.75)).show();
                                ui.progress_bar(progress_val)
                                    .track_height(10.0)
                                    .radius_all(5.0)
                                    .fill_color(Color::rgb(0.85, 0.40, 0.90))
                                    .track_color(Color::rgb(0.14, 0.16, 0.22))
                                    .show();
                            });
                        });
                });
        })
        .run();
}
