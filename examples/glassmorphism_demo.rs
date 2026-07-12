// examples/glassmorphism_demo.rs
//
// Demonstrates the new `.backdrop_blur()` API added to ContainerBuilder.
// Run with: cargo run --example glassmorphism_demo

use zenthra::prelude::*;

fn main() {
    let mut active_tab: usize = 0;
    let mut blur_strength: f32 = 24.0; // default blur strength (px)
    let mut opacity: f32 = 0.05; // default glass opacity (5%)
    let mut style_idx: usize = 0; // 0 = Light, 1 = Dark, 2 = Amethyst
    let toggle = true;
    let mut shadow_depth: f32 = 16.0; // ~16px default shadow depth

    App::new()
        .title("Zenthra — Glassmorphism Demo")
        .size(1100, 750)
        .with_ui(move |ui: &mut Ui| {
            // ── Root: gradient background ─────────────────────────────────────
            // We draw a vivid gradient via stacked rects + use images to simulate
            // a rich background that the glass effect will blur.
            ui.container()
                .full_width()
                .full_height()
                
                .bg_opacity(0.5) // Highly transparent (~4% opaque)

                .border(Color::rgba(1.0, 1.0, 1.0, 0.12), 1.0)
                .show(|ui| {
                    // ── Colourful blobs behind the glass (things to blur) ──────
                    // Top-left purple blob
                    ui.container()
                        .absolute(-80.0, -80.0)
                        .width(420.0)
                        .height(420.0)
                        .bg(Color::rgba(120.0 / 255.0, 40.0 / 255.0, 220.0 / 255.0, 180.0 / 255.0))
                        .radius_all(210.0)
                        .show(|_| {});

                    // Centre-right cyan blob
                    ui.container()
                        .absolute(600.0, 80.0)
                        .width(360.0)
                        .height(360.0)
                        .bg(Color::rgba(0.0, 200.0 / 255.0, 1.0, 150.0 / 255.0))
                        .radius_all(180.0)
                        .show(|_| {});

                    // Bottom-centre pink blob
                    ui.container()
                        .absolute(280.0, 480.0)
                        .width(500.0)
                        .height(280.0)
                        .bg(Color::rgba(1.0, 50.0 / 255.0, 160.0 / 255.0, 130.0 / 255.0))
                        .radius_all(140.0)
                        .show(|_| {});

                    // Bottom-left teal blob
                    ui.container()
                        .absolute(-60.0, 460.0)
                        .width(300.0)
                        .height(300.0)
                        .bg(Color::rgba(20.0 / 255.0, 220.0 / 255.0, 180.0 / 255.0, 140.0 / 255.0))
                        .radius_all(150.0)
                        .show(|_| {});

                    // ── Full-screen backdrop blur frosted glass layer ──────
                    ui.container()
                        .absolute(0.0, 0.0)
                        .full_width()
                        .full_height()
                        .backdrop_blur(35.0)
                        .bg(Color::rgba(0.05, 0.04, 0.12, 0.5))
                        .show(|_| {});

                    // ── Page title ────────────────────────────────────────────
                    ui.container()
                        .absolute(0.0, 0.0)
                        .full_width()
                        .padding_top(48.0)
                        .gap(8.0)
                        .row()
                        .show(|ui| {
                            ui.container()
                                .full_width()
                                .gap(6.0)
                                .show(|ui| {
                                    ui.text("✦  Glassmorphism")
                                        .size(38.0)
                                        .weight(FontWeight::Bold)
                                        .color(Color::WHITE)
                                        .show();
                                    ui.text("Backdrop blur powered by a Dual-Kawase GPU pyramid")
                                        .size(15.0)
                                        .color(Color::rgba(1.0, 1.0, 1.0, 160.0 / 255.0))
                                        .show();
                                });
                        });

                    // ── Main glass card (backdrop_filter = dynamic) ─────────────
                    let bg_color = match style_idx {
                        0 => Color::WHITE,
                        1 => Color::TRANSPARENT,
                        2 => Color::rgba(180.0 / 255.0, 120.0 / 255.0, 1.0, 1.0),
                        _ => Color::WHITE,
                    };
                    let blur_type = match style_idx {
                        0 => style::blur::Type::Glassmorphism,
                        1 => style::blur::Type::Frosted,
                        2 => style::blur::Type::OpaqueGlass,
                        _ => style::blur::Type::Normal,
                    };
                    let filter = BackdropFilter::new()
                        .blur(if toggle { blur_strength } else { 0.0 }, blur_type)
                        .brightness(if style_idx == 1 { 0.85 } else { 1.12 })
                        .saturate(if style_idx == 1 { 0.90 } else { 1.20 });

                    ui.container()
                        .absolute(60.0, 140.0)
                        .width(480.0)
                        .height(460.0)
                        .backdrop_filter(filter)
                        .bg(bg_color)
                        .bg_opacity(opacity)
                        .border(Color::rgba(1.0, 1.0, 1.0, 60.0 / 255.0), 1.0)
                        .radius_all(24.0)
                        .shadow(Color::rgba(0.0, 0.0, 0.0, 100.0 / 255.0), 0.0, shadow_depth * 0.33, shadow_depth)
                        .padding_all(28.0)
                        .gap(18.0)
                        .show(|ui| {
                            // Card header
                            ui.container()
                                .full_width()
                                .row()
                                .gap(12.0)
                                .show(|ui| {
                                    ui.container()
                                        .width(44.0)
                                        .height(44.0)
                                        .bg(Color::rgba(180.0 / 255.0, 120.0 / 255.0, 1.0, 200.0 / 255.0))
                                        .radius_all(12.0)
                                        .show(|_| {});
                                    ui.container()
                                        .gap(3.0)
                                        .show(|ui| {
                                            ui.text("Main Glass Card")
                                                .size(18.0)
                                                .weight(FontWeight::SemiBold)
                                                .color(Color::WHITE)
                                                .show();
                                            let style_name = match style_idx {
                                                0 => "Light",
                                                1 => "Transparent",
                                                2 => "Amethyst",
                                                _ => "Light",
                                            };
                                            ui.text(&format!("blur: {:.0} px  •  opacity: {:.0}%  •  style: {}", if toggle { blur_strength } else { 0.0 }, opacity * 100.0, style_name))
                                                .size(12.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 140.0 / 255.0))
                                                .show();
                                        });
                                });

                            // Divider
                            ui.container()
                                .full_width()
                                .height(1.0)
                                .bg(Color::rgba(1.0, 1.0, 1.0, 40.0 / 255.0))
                                .show(|_| {});

                            // Tab pills
                            ui.container()
                                .full_width()
                                .row()
                                .gap(8.0)
                                .show(|ui| {
                                    for (i, label) in ["Overview", "Settings", "About"].iter().enumerate() {
                                        let is_active = active_tab == i;
                                        let btn = ui.button(label)
                                            .padding_x(16.0)
                                            .padding_y(7.0)
                                            .bg(if is_active { Color::rgba(180.0 / 255.0, 120.0 / 255.0, 1.0, 200.0 / 255.0) } else { Color::rgba(1.0, 1.0, 1.0, 20.0 / 255.0) })
                                            .hover_bg(if is_active { Color::rgba(180.0 / 255.0, 120.0 / 255.0, 1.0, 220.0 / 255.0) } else { Color::rgba(1.0, 1.0, 1.0, 40.0 / 255.0) })
                                            .border(Color::rgba(1.0, 1.0, 1.0, if is_active { 0.0 } else { 40.0 / 255.0 }), 1.0)
                                            .radius_all(20.0)
                                            .text_color(Color::WHITE)
                                            .size(13.0)
                                            .show();
                                        if btn.clicked { active_tab = i; }
                                    }
                                });

                            // Tab content
                            match active_tab {
                                0 => {
                                    // Overview
                                    ui.container()
                                        .full_width()
                                        .gap(10.0)
                                        .show(|ui| {
                                            ui.text("How it works")
                                                .size(15.0)
                                                .weight(FontWeight::SemiBold)
                                                .color(Color::WHITE)
                                                .show();
                                            ui.text("When a container has .backdrop_blur(radius), Zenthra:\n  1. Flushes all pending draw batches\n  2. Runs a Dual-Kawase 4-level pyramid downsample + upsample over the scene\n  3. Blits the blurred result back to the offscreen texture\n  4. Draws the tinted glass rect on top")
                                                .size(13.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 180.0 / 255.0))
                                                .show();
                                        });
                                }
                                1 => {
                                    // Settings
                                    ui.container()
                                        .full_width()
                                        .gap(12.0)
                                        .show(|ui| {
                                            // 1. Backdrop Blur Strength
                                            ui.text("Backdrop Blur Strength")
                                                .size(14.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 180.0 / 255.0))
                                                .show();
                                            let mut blur_slider = blur_strength / 60.0;
                                            ui.slider(&mut blur_slider, "blur_slider").width(400.0).show();
                                            blur_strength = blur_slider * 60.0;
                                            ui.text(&format!("{:.0} px", blur_strength))
                                                .size(12.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 140.0 / 255.0))
                                                .show();

                                            // 2. Glass Opacity Tint
                                            ui.text("Glass Opacity Tint")
                                                .size(14.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 180.0 / 255.0))
                                                .show();
                                            let mut opacity_slider = opacity / 0.50;
                                            ui.slider(&mut opacity_slider, "opacity_slider").width(400.0).show();
                                            opacity = opacity_slider * 0.50;
                                            ui.text(&format!("{:.1}%", opacity * 100.0))
                                                .size(12.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 140.0 / 255.0))
                                                .show();

                                            // 3. Glass Panel Blur Type (style::blur::type)
                                            ui.text("Glass Panel Blur Type")
                                                .size(14.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 180.0 / 255.0))
                                                .show();
                                            ui.container()
                                                .full_width()
                                                .row()
                                                .gap(8.0)
                                                .show(|ui| {
                                                    for (i, name) in ["Light (White)", "Transparent", "Amethyst (Colored)"].iter().enumerate() {
                                                        let active = style_idx == i;
                                                        let btn = ui.button(*name)
                                                            .width(128.0)
                                                            .padding_y(8.0)
                                                            .bg(if active { Color::rgba(1.0, 1.0, 1.0, 30.0 / 255.0) } else { Color::rgba(1.0, 1.0, 1.0, 10.0 / 255.0) })
                                                            .hover_bg(Color::rgba(1.0, 1.0, 1.0, 20.0 / 255.0))
                                                            .radius_all(10.0)
                                                            .text_color(if active { Color::WHITE } else { Color::rgba(1.0, 1.0, 1.0, 150.0 / 255.0) })
                                                            .size(11.0)
                                                            .show();
                                                        if btn.clicked {
                                                            style_idx = i;
                                                        }
                                                    }
                                                });

                                            // 4. Shadow Depth
                                            ui.text("Shadow Depth (Visual Depth)")
                                                .size(14.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 180.0 / 255.0))
                                                .show();
                                            let mut shadow_slider = shadow_depth / 36.0;
                                            ui.slider(&mut shadow_slider, "shadow_slider").width(400.0).show();
                                            shadow_depth = shadow_slider * 36.0;
                                            ui.text(&format!("{:.0} px", shadow_depth))
                                                .size(12.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 140.0 / 255.0))
                                                .show();
                                        });
                                }
                                _ => {
                                    ui.text("Zenthra Glassmorphism\nAdded by Dual-Kawase blur pipeline.\nSame technique as the after-motion mobile app's Gaussian FBO pyramid, ported to WGSL / wgpu.")
                                        .size(13.0)
                                        .color(Color::rgba(1.0, 1.0, 1.0, 180.0 / 255.0))
                                        .show();
                                }
                            }
                        });

                    // ── Side stats cards ──────────────────────────────────────
                    glass_stat(ui, 580.0, 140.0, "⚡", "GPU Passes", "4 downsample + 3 upsample", Color::rgba(100.0 / 255.0, 180.0 / 255.0, 1.0, 200.0 / 255.0));
                    glass_stat(ui, 580.0, 266.0, "🎯", "Blur Quality", "Dual-Kawase pyramid", Color::rgba(160.0 / 255.0, 100.0 / 255.0, 1.0, 200.0 / 255.0));
                    glass_stat(ui, 580.0, 392.0, "🔲", "Clipping", "Per-corner SDF rounding", Color::rgba(60.0 / 255.0, 200.0 / 255.0, 180.0 / 255.0, 200.0 / 255.0));

                    // ── Mini softly-blurred notification ─────────────────────
                    ui.container()
                        .absolute(60.0, 620.0)
                        .width(480.0)
                        .height(72.0)
                        .backdrop_blur(12.0)
                        .bg(Color::rgba(1.0, 1.0, 1.0, 20.0 / 255.0))
                        .border(Color::rgba(1.0, 1.0, 1.0, 50.0 / 255.0), 1.0)
                        .radius_all(16.0)
                        .padding_x(20.0)
                        .padding_y(12.0)
                        .row()
                        .gap(12.0)
                        .show(|ui| {
                            ui.container()
                                .width(8.0)
                                .height(8.0)
                                .bg(Color::rgba(80.0 / 255.0, 220.0 / 255.0, 140.0 / 255.0, 1.0))
                                .radius_all(4.0)
                                .show(|_| {});
                            ui.text("backdrop_blur(12.0) — light notification bar")
                                .size(13.0)
                                .color(Color::rgba(1.0, 1.0, 1.0, 200.0 / 255.0))
                                .show();
                        });
                });
        })
        .run();
}

/// Helper: small glassmorphism stat card
fn glass_stat(ui: &mut Ui, x: f32, y: f32, icon: &str, label: &str, value: &str, accent: Color) {
    ui.container()
        .absolute(x, y)
        .width(460.0)
        .height(108.0)
        .backdrop_blur(18.0)
        .bg(Color::WHITE)
        .bg_opacity(0.04)
        .border(Color::rgba(1.0, 1.0, 1.0, 50.0 / 255.0), 1.0)
        .radius_all(18.0)
        .shadow(Color::rgba(0.0, 0.0, 0.0, 80.0 / 255.0), 0.0, 8.0, 24.0)
        .padding_all(18.0)
        .row()
        .gap(16.0)
        .show(|ui| {
            ui.container()
                .width(52.0)
                .height(52.0)
                .bg(accent)
                .radius_all(14.0)
                .show(|ui| {
                    ui.text(icon)
                        .size(24.0)
                        .color(Color::WHITE)
                        .show();
                });
            ui.container()
                .gap(4.0)
                .show(|ui| {
                    ui.text(label)
                        .size(13.0)
                        .color(Color::rgba(1.0, 1.0, 1.0, 150.0 / 255.0))
                        .show();
                    ui.text(value)
                        .size(15.0)
                        .weight(FontWeight::SemiBold)
                        .color(Color::WHITE)
                        .show();
                });
        });
}
