use std::sync::Arc;
use zenthra::prelude::*;

// Embed real project assets from the workspace assets directory
const ZENFILE_PNG: &[u8] = include_bytes!("../assets/zenfile.png");
const ZENTHRA_VIEW_JPEG: &[u8] = include_bytes!("../assets/zenthra_view.jpeg");
const GLASSMORPHISM_PNG: &[u8] = include_bytes!("../assets/zenfile_glassmorphism2.png");

fn main() {
    let zenfile_texture: Arc<[u8]> = Arc::from(ZENFILE_PNG);
    let view_texture: Arc<[u8]> = Arc::from(ZENTHRA_VIEW_JPEG);
    let glass_texture: Arc<[u8]> = Arc::from(GLASSMORPHISM_PNG);

    // Interactive State Variables
    let mut selected_image_idx = 0usize; // 0: ZenFile UI, 1: Zenthra View (JPEG), 2: Glassmorphism
    let mut current_fit = ObjectFit::Cover;
    let mut grayscale = 0.0f32;
    let mut opacity = 1.0f32;
    let mut zoom_level = 1.0f32;
    let mut rot_x = 0.0f32;
    let mut rot_y = 0.0f32;
    let mut rot_z = 0.0f32;
    let mut flip_h = false;
    let mut flip_v = false;
    let mut corner_radius = 16.0f32;
    let mut click_count = 0usize;
    let mut status_msg = String::from("Interact with controls on the left to transform the image in real-time.");

    App::new()
        .title("Zenthra - Hardware-Accelerated Image Widget Showcase")
        .size(1240, 920)
        .with_ui(move |ui: &mut Ui| {
            let active_texture = match selected_image_idx {
                1 => view_texture.clone(),
                2 => glass_texture.clone(),
                _ => zenfile_texture.clone(),
            };

            let bg_canvas = Color::rgb(0.06, 0.07, 0.09);
            let bg_card = Color::rgb(0.10, 0.12, 0.16);
            let bg_card_alt = Color::rgb(0.13, 0.15, 0.20);
            let border_col = Color::rgb(0.19, 0.23, 0.31);
            let accent_col = Color::rgb(0.38, 0.44, 0.98);
            let text_main = Color::WHITE;
            let text_muted = Color::rgb(0.60, 0.65, 0.75);

            ui.container()
                .fill()
                .bg(bg_canvas)
                .padding_all(24.0)
                .gap(20.0)
                .scroll_y(true)
                .column()
                .show(|ui: &mut Ui| {

                    // ── 1. HEADER BANNER ─────────────────────────────────────────
                    ui.card()
                        .fill_x()
                        .bg(bg_card)
                        .border((border_col, 1.0))
                        .radius_all(14.0)
                        .padding_all(20.0)
                        .gap(6.0)
                        .show(|ui: &mut Ui| {
                            ui.h2("Hardware-Accelerated Image Widget")
                                .color(text_main)
                                .show();

                            ui.text("Immediate-mode GPU image rendering with dynamic ObjectFit, UV scaling, 3D perspective rotation, grayscale filters, and SDF clipping.")
                                .color(text_muted)
                                .size(13.5)
                                .show();

                            ui.container()
                                .row()
                                .gap(10.0)
                                .padding_top(4.0)
                                .show(|ui: &mut Ui| {
                                    ui.container()
                                        .bg(Color::rgba(0.38, 0.44, 0.98, 0.18))
                                        .border((accent_col, 1.0))
                                        .radius_all(6.0)
                                        .padding_x(8.0)
                                        .padding_y(4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.text("WGPU Shaders").size(11.0).color(Color::rgb(0.65, 0.72, 1.0)).show();
                                        });

                                    ui.container()
                                        .bg(Color::rgba(0.16, 0.75, 0.50, 0.18))
                                        .border((Color::rgb(0.16, 0.75, 0.50), 1.0))
                                        .radius_all(6.0)
                                        .padding_x(8.0)
                                        .padding_y(4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.text("PNG & JPEG Decoding").size(11.0).color(Color::rgb(0.40, 0.90, 0.65)).show();
                                        });

                                    ui.container()
                                        .bg(Color::rgba(0.95, 0.60, 0.15, 0.18))
                                        .border((Color::rgb(0.95, 0.60, 0.15), 1.0))
                                        .radius_all(6.0)
                                        .padding_x(8.0)
                                        .padding_y(4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.text("SDF Anti-Aliased Corners").size(11.0).color(Color::rgb(1.0, 0.75, 0.35)).show();
                                        });
                                });
                        });

                    // ── 2. MAIN WORKSPACE: CONTROLS (LEFT) + LIVE HERO (RIGHT) ───
                    ui.container()
                        .fill_x()
                        .row()
                        .gap(20.0)
                        .show(|ui: &mut Ui| {

                            // LEFT CONTROLS PANEL
                            ui.container()
                                .width(380.0)
                                .bg(bg_card)
                                .border((border_col, 1.0))
                                .radius_all(14.0)
                                .padding_all(20.0)
                                .gap(14.0)
                                .column()
                                .show(|ui: &mut Ui| {
                                    ui.h3("Image Parameters").color(text_main).show();

                                    // Asset Selector
                                    ui.text("Select Source Asset:").color(text_muted).size(12.5).show();
                                    ui.container()
                                        .row()
                                        .gap(6.0)
                                        .show(|ui: &mut Ui| {
                                            let assets = [
                                                (0, "ZenFile (PNG)"),
                                                (1, "View (JPEG)"),
                                                (2, "Glass (PNG)"),
                                            ];
                                            for (idx, label) in assets {
                                                let is_active = selected_image_idx == idx;
                                                let btn_bg = if is_active { accent_col } else { bg_card_alt };
                                                let btn_text = if is_active { Color::WHITE } else { text_muted };

                                                if ui.button(label)
                                                    .bg(btn_bg)
                                                    .color(btn_text)
                                                    .radius_all(6.0)
                                                    .padding_x(10.0)
                                                    .padding_y(6.0)
                                                    .show()
                                                    .clicked
                                                {
                                                    selected_image_idx = idx;
                                                    status_msg = format!("Loaded asset: {}", label);
                                                }
                                            }
                                        });

                                    // ObjectFit selector
                                    ui.text("Object Fit Mode:").color(text_muted).size(12.5).show();
                                    ui.container()
                                        .row()
                                        .gap(6.0)
                                        .wrap(Wrap::Wrap)
                                        .show(|ui: &mut Ui| {
                                            let modes = [
                                                (ObjectFit::Cover, "Cover"),
                                                (ObjectFit::Contain, "Contain"),
                                                (ObjectFit::Fill, "Fill"),
                                                (ObjectFit::ScaleDown, "ScaleDown"),
                                            ];

                                            for (mode, label) in modes {
                                                let is_active = current_fit == mode;
                                                let btn_bg = if is_active { accent_col } else { bg_card_alt };
                                                let btn_text = if is_active { Color::WHITE } else { text_muted };

                                                if ui.button(label)
                                                    .bg(btn_bg)
                                                    .color(btn_text)
                                                    .radius_all(6.0)
                                                    .padding_x(10.0)
                                                    .padding_y(6.0)
                                                    .show()
                                                    .clicked
                                                {
                                                    current_fit = mode;
                                                    status_msg = format!("Set ObjectFit to {:?}", mode);
                                                }
                                            }
                                        });

                                    // Grayscale slider
                                    ui.container()
                                        .fill_x()
                                        .column()
                                        .gap(4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container()
                                                .fill_x()
                                                .row()
                                                .show(|ui: &mut Ui| {
                                                    ui.text("Grayscale Filter").color(text_muted).size(12.0).show();
                                                    ui.text(&format!("{:.0}%", grayscale * 100.0))
                                                        .color(accent_col)
                                                        .size(12.0)
                                                        .show();
                                                });

                                            ui.slider(&mut grayscale, "slider_grayscale")
                                                .range(0.0, 1.0)
                                                .fill_x()
                                                .show();
                                        });

                                    // Opacity slider
                                    ui.container()
                                        .fill_x()
                                        .column()
                                        .gap(4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container()
                                                .fill_x()
                                                .row()
                                                .show(|ui: &mut Ui| {
                                                    ui.text("Opacity / Transparency").color(text_muted).size(12.0).show();
                                                    ui.text(&format!("{:.0}%", opacity * 100.0))
                                                        .color(accent_col)
                                                        .size(12.0)
                                                        .show();
                                                });

                                            ui.slider(&mut opacity, "slider_opacity")
                                                .range(0.1, 1.0)
                                                .fill_x()
                                                .show();
                                        });

                                    // Zoom / Scale slider
                                    ui.container()
                                        .fill_x()
                                        .column()
                                        .gap(4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container()
                                                .fill_x()
                                                .row()
                                                .show(|ui: &mut Ui| {
                                                    ui.text("Internal Zoom (UV Scale)").color(text_muted).size(12.0).show();
                                                    ui.text(&format!("{:.2}x", zoom_level))
                                                        .color(accent_col)
                                                        .size(12.0)
                                                        .show();
                                                });

                                            ui.slider(&mut zoom_level, "slider_zoom")
                                                .range(0.5, 3.0)
                                                .fill_x()
                                                .show();
                                        });

                                    // Corner Radius slider
                                    ui.container()
                                        .fill_x()
                                        .column()
                                        .gap(4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container()
                                                .fill_x()
                                                .row()
                                                .show(|ui: &mut Ui| {
                                                    ui.text("Corner Radius (SDF)").color(text_muted).size(12.0).show();
                                                    ui.text(&format!("{:.0}px", corner_radius))
                                                        .color(accent_col)
                                                        .size(12.0)
                                                        .show();
                                                });

                                            ui.slider(&mut corner_radius, "slider_radius")
                                                .range(0.0, 120.0)
                                                .fill_x()
                                                .show();
                                        });

                                    // 3D Perspective Rotation
                                    ui.text("3D Perspective Tilt:").color(text_muted).size(12.5).show();

                                    ui.container()
                                        .fill_x()
                                        .column()
                                        .gap(4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container()
                                                .fill_x()
                                                .row()
                                                .show(|ui: &mut Ui| {
                                                    ui.text("Pitch (X-Axis)").color(text_muted).size(11.5).show();
                                                    ui.text(&format!("{:.0}°", rot_x)).color(text_main).size(11.5).show();
                                                });
                                            ui.slider(&mut rot_x, "slider_rot_x").range(-45.0, 45.0).fill_x().show();
                                        });

                                    ui.container()
                                        .fill_x()
                                        .column()
                                        .gap(4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container()
                                                .fill_x()
                                                .row()
                                                .show(|ui: &mut Ui| {
                                                    ui.text("Yaw (Y-Axis)").color(text_muted).size(11.5).show();
                                                    ui.text(&format!("{:.0}°", rot_y)).color(text_main).size(11.5).show();
                                                });
                                            ui.slider(&mut rot_y, "slider_rot_y").range(-45.0, 45.0).fill_x().show();
                                        });

                                    ui.container()
                                        .fill_x()
                                        .column()
                                        .gap(4.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container()
                                                .fill_x()
                                                .row()
                                                .show(|ui: &mut Ui| {
                                                    ui.text("Roll (Z-Axis)").color(text_muted).size(11.5).show();
                                                    ui.text(&format!("{:.0}°", rot_z)).color(text_main).size(11.5).show();
                                                });
                                            ui.slider(&mut rot_z, "slider_rot_z").range(-45.0, 45.0).fill_x().show();
                                        });

                                    // Flip toggles
                                    ui.container()
                                        .fill_x()
                                        .row()
                                        .gap(16.0)
                                        .show(|ui: &mut Ui| {
                                            ui.toggle(&mut flip_h, "Flip H").show();
                                            ui.toggle(&mut flip_v, "Flip V").show();
                                        });

                                    // Reset Button
                                    if ui.button("Reset All Transforms")
                                        .bg(bg_card_alt)
                                        .color(text_main)
                                        .border((border_col, 1.0))
                                        .radius_all(8.0)
                                        .padding_y(8.0)
                                        .fill_x()
                                        .show()
                                        .clicked
                                    {
                                        current_fit = ObjectFit::Cover;
                                        grayscale = 0.0;
                                        opacity = 1.0;
                                        zoom_level = 1.0;
                                        rot_x = 0.0;
                                        rot_y = 0.0;
                                        rot_z = 0.0;
                                        flip_h = false;
                                        flip_v = false;
                                        corner_radius = 16.0;
                                        status_msg = String::from("Transforms reset to default.");
                                    }
                                });

                            // RIGHT LIVE HERO PREVIEW STAGE
                            ui.container()
                                .fill_x()
                                .bg(bg_card)
                                .border((border_col, 1.0))
                                .radius_all(14.0)
                                .padding_all(24.0)
                                .gap(18.0)
                                .column()
                                .show(|ui: &mut Ui| {
                                    ui.container()
                                        .fill_x()
                                        .row()
                                        .show(|ui: &mut Ui| {
                                            ui.h3("Interactive Viewport").color(text_main).show();

                                            ui.text(&format!("Clicks: {} | Mode: {:?}", click_count, current_fit))
                                                .color(text_muted)
                                                .size(12.5)
                                                .show();
                                        });

                                    // Hero Image Canvas with hover border & interactive response
                                    ui.container()
                                        .fill_x()
                                        .height(340.0)
                                        .bg(Color::rgb(0.04, 0.05, 0.07))
                                        .radius_all(12.0)
                                        .border((border_col, 1.0))
                                        .align(Align::Center)
                                        .show(|ui: &mut Ui| {
                                            let resp = ui.image(ImageSource::Bytes(active_texture.clone()))
                                                .width(460.0)
                                                .height(290.0)
                                                .fit(current_fit)
                                                .grayscale(grayscale)
                                                .opacity(opacity)
                                                .zoom(zoom_level)
                                                .rotate(rot_x, rot_y, rot_z)
                                                .flip_h(flip_h)
                                                .flip_v(flip_v)
                                                .radius_all(corner_radius)
                                                .border((accent_col, 2.0))
                                                .shadow(Color::rgba(0.0, 0.0, 0.0, 0.6), 0.0, 8.0, 24.0)
                                                .hover([
                                                    Style::border((Color::rgb(0.55, 0.65, 1.0), 2.5)),
                                                    Style::shadow(Color::rgba(0.38, 0.44, 0.98, 0.4), 0.0, 12.0, 32.0),
                                                ])
                                                .show();

                                            if resp.clicked {
                                                click_count += 1;
                                                status_msg = format!("Clicked Hero Image! Total clicks: {}", click_count);
                                            }
                                        });

                                    // Status feedback bar
                                    ui.container()
                                        .fill_x()
                                        .bg(bg_card_alt)
                                        .radius_all(8.0)
                                        .padding_all(12.0)
                                        .row()
                                        .gap(8.0)
                                        .show(|ui: &mut Ui| {
                                            ui.text("Status:")
                                                .color(accent_col)
                                                .size(12.5)
                                                .font_weight(FontWeight::Bold)
                                                .show();
                                            ui.text(&status_msg).color(text_main).size(12.5).show();
                                        });
                                });
                        });

                    // ── 3. OBJECT-FIT MODES COMPARISON GALLERY ────────────────────
                    ui.card()
                        .fill_x()
                        .bg(bg_card)
                        .border((border_col, 1.0))
                        .radius_all(14.0)
                        .padding_all(20.0)
                        .gap(14.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("ObjectFit Scaling Comparison Gallery").color(text_main).show();
                            ui.text("Comparing all ObjectFit modes in fixed 220x130 containers with real assets:")
                                .color(text_muted)
                                .size(12.5)
                                .show();

                            ui.container()
                                .fill_x()
                                .row()
                                .gap(16.0)
                                .wrap(Wrap::Wrap)
                                .show(|ui: &mut Ui| {
                                    let comparisons = [
                                        (ObjectFit::Cover, "Cover", "Fills viewport, clips overflow"),
                                        (ObjectFit::Contain, "Contain", "Letterboxes to fit entirely"),
                                        (ObjectFit::Fill, "Fill", "Stretches to fill bounds"),
                                        (ObjectFit::ScaleDown, "ScaleDown", "Like contain, but never upscales"),
                                    ];

                                    for (fit_mode, title, desc) in comparisons {
                                        ui.container()
                                            .width(260.0)
                                            .bg(bg_card_alt)
                                            .border((border_col, 1.0))
                                            .radius_all(10.0)
                                            .padding_all(10.0)
                                            .gap(8.0)
                                            .column()
                                            .show(|ui: &mut Ui| {
                                                ui.container()
                                                    .fill_x()
                                                    .height(130.0)
                                                    .bg(Color::rgb(0.05, 0.06, 0.08))
                                                    .radius_all(8.0)
                                                    .align(Align::Center)
                                                    .show(|ui: &mut Ui| {
                                                        ui.image(ImageSource::Bytes(zenfile_texture.clone()))
                                                            .width(240.0)
                                                            .height(130.0)
                                                            .fit(fit_mode)
                                                            .radius_all(6.0)
                                                            .show();
                                                    });

                                                ui.h4(title).color(text_main).show();
                                                ui.text(desc).color(text_muted).size(11.0).show();
                                            });
                                    }
                                });
                        });

                    // ── 4. AVATAR, PROFILE & CIRCULAR SDF CLIPPING ─────────────────
                    ui.card()
                        .fill_x()
                        .bg(bg_card)
                        .border((border_col, 1.0))
                        .radius_all(14.0)
                        .padding_all(20.0)
                        .gap(14.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("Circular Avatars & Profile Cards (SDF Radius)").color(text_main).show();

                            ui.container()
                                .fill_x()
                                .row()
                                .gap(24.0)
                                .wrap(Wrap::Wrap)
                                .show(|ui: &mut Ui| {
                                    // 1. Fully Circular Avatar (radius = width / 2)
                                    ui.container()
                                        .row()
                                        .gap(14.0)
                                        .align(Align::Center)
                                        .show(|ui: &mut Ui| {
                                            ui.image(ImageSource::Bytes(view_texture.clone()))
                                                .width(72.0)
                                                .height(72.0)
                                                .radius_all(36.0) // Perfect circle
                                                .border((accent_col, 2.5))
                                                .fit(ObjectFit::Cover)
                                                .show();

                                            ui.container()
                                                .column()
                                                .gap(3.0)
                                                .show(|ui: &mut Ui| {
                                                    ui.h4("Zenthra Panoramic View").color(text_main).show();
                                                    ui.text("radius_all(36.0) - Perfect Circle").color(text_muted).size(12.0).show();
                                                    ui.text("JPEG Format Asset").color(Color::rgb(0.35, 0.85, 0.55)).size(11.5).show();
                                                });
                                        });

                                    // 2. Rounded Square Avatar
                                    ui.container()
                                        .row()
                                        .gap(14.0)
                                        .align(Align::Center)
                                        .show(|ui: &mut Ui| {
                                            ui.image(ImageSource::Bytes(glass_texture.clone()))
                                                .width(72.0)
                                                .height(72.0)
                                                .radius_all(14.0) // Squircle
                                                .border((Color::rgb(0.95, 0.65, 0.20), 2.0))
                                                .fit(ObjectFit::Cover)
                                                .show();

                                            ui.container()
                                                .column()
                                                .gap(3.0)
                                                .show(|ui: &mut Ui| {
                                                    ui.h4("ZenFile Glassmorphism").color(text_main).show();
                                                    ui.text("radius_all(14.0) - Squircle").color(text_muted).size(12.0).show();
                                                    ui.text("PNG High-Res Asset").color(Color::rgb(0.95, 0.65, 0.20)).size(11.5).show();
                                                });
                                        });

                                    // 3. Aspect Ratio Card Preview
                                    ui.container()
                                        .row()
                                        .gap(14.0)
                                        .align(Align::Center)
                                        .show(|ui: &mut Ui| {
                                            ui.image(ImageSource::Bytes(zenfile_texture.clone()))
                                                .width(128.0)
                                                .height(72.0) // 16:9 ratio
                                                .radius_all(8.0)
                                                .fit(ObjectFit::Cover)
                                                .border((border_col, 1.0))
                                                .show();

                                            ui.container()
                                                .column()
                                                .gap(3.0)
                                                .show(|ui: &mut Ui| {
                                                    ui.h4("16:9 Thumbnail").color(text_main).show();
                                                    ui.text("Aspect ratio constrained").color(text_muted).size(12.0).show();
                                                    ui.text("128 × 72 px").color(text_muted).size(11.5).show();
                                                });
                                        });
                                });
                        });
                });
        })
        .run();
}
