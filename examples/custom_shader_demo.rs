// examples/custom_shader_demo.rs
//
// Demonstrates Zenthra's new extensible shader post-processing container API.
// Run with: cargo run --example custom_shader_demo

use zenthra::prelude::*;

fn main() {
    let mut active_tab: usize = 0;
    let mut slider_val: f32 = 0.3; // blur radius parameter
    let mut toggle = true;

    App::new()
        .title("Zenthra — Custom Container Shader Demo")
        .size(1100, 750)
        // ── 1. Register our custom WGSL shader ─────────────────────────────────
        .register_custom_shader("liquid", include_str!("liquid_distortion.wgsl"))
        .with_ui(move |ui: &mut Ui| {
            // ── Root background ───────────────────────────────────────────────
            ui.container()
                .full_width()
                .full_height()
                .bg(Color::BLACK)
                .bg_opacity(0.1)
                .border(Color::rgba(1.0, 1.0, 1.0, 0.12), 1.0)
                .show(|ui| {
                    // ── Colourful blobs behind the glass (things to distort) ──
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

                    // ── Full-screen frosted glass layer ──────
                    ui.container()
                        .absolute(0.0, 0.0)
                        .full_width()
                        .full_height()
                        .backdrop_blur(28.0)
                        .bg(Color::WHITE)
                        .bg_opacity(0.04)
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
                                .width(8.0)
                                .height(38.0)
                                .bg(Color::rgba(0.0, 220.0 / 255.0, 255.0 / 255.0, 1.0))
                                .radius_all(4.0)
                                .show(|_| {});
                            ui.container()
                                .column()
                                .gap(2.0)
                                .show(|ui| {
                                    ui.text("Zenthra Shader Extensibility")
                                        .size(24.0)
                                        .weight(FontWeight::Bold)
                                        .color(Color::WHITE)
                                        .show();
                                    ui.text("Containers rendered with developer-registered custom WGSL fragment shaders")
                                        .size(15.0)
                                        .color(Color::rgba(1.0, 1.0, 1.0, 160.0 / 255.0))
                                        .show();
                                });
                        });

                    // ── Custom Liquid PostProcess Card ───────────────────────
                    ui.container()
                        .absolute(60.0, 140.0)
                        .width(480.0)
                        .height(460.0)
                        // This applies our custom post-processing shader:
                        .post_process_shader("liquid")
                        // Optional blur underneath the distortion:
                        .backdrop_blur(if toggle { slider_val * 40.0 } else { 0.0 })
                        .bg(Color::WHITE)
                        .bg_opacity(0.02)
                        .border(Color::rgba(1.0, 1.0, 1.0, 60.0 / 255.0), 1.0)
                        .radius_all(24.0)
                        .shadow(Color::rgba(0.0, 0.0, 0.0, 80.0 / 255.0), 0.0, 16.0, 48.0)
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
                                        .bg(Color::rgba(0.0, 200.0 / 255.0, 255.0 / 255.0, 0.2))
                                        .border(Color::rgba(0.0, 200.0 / 255.0, 255.0 / 255.0, 1.0), 1.5)
                                        .radius_all(22.0)
                                        .show(|ui| {
                                            ui.text("〰")
                                                .size(20.0)
                                                .color(Color::rgba(0.0, 200.0 / 255.0, 255.0 / 255.0, 1.0))
                                                .show();
                                        });
                                    ui.container()
                                        .column()
                                        .gap(2.0)
                                        .show(|ui| {
                                            ui.text("Liquid Distortion")
                                                .size(18.0)
                                                .weight(FontWeight::SemiBold)
                                                .color(Color::WHITE)
                                                .show();
                                            ui.text(&format!("blur: {:.0} px  •  warp: {:.1} px", if toggle { slider_val * 40.0 } else { 0.0 }, if toggle { slider_val * 40.0 * 0.6 } else { 0.0 }))
                                                 .size(12.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 140.0 / 255.0))
                                                .show();
                                        });
                                });

                            // Separator
                            ui.container()
                                .full_width()
                                .height(1.0)
                                .bg(Color::rgba(1.0, 1.0, 1.0, 30.0 / 255.0))
                                .show(|_| {});

                            // Description
                            ui.text("This container uses a custom post-processing fragment shader compiled dynamically on the fly. It samples the scene behind it and displaces coordinates based on screen-space sine waves over time to create a liquid water warp effect.")
                                .size(14.0)
                                .color(Color::rgba(1.0, 1.0, 1.0, 200.0 / 255.0))
                                .show();

                            ui.text("You can register any WGSL fragment shader that complies with the standard vertex output and uniform layouts, unlocking endless layout possibilities.")
                                .size(14.0)
                                .color(Color::rgba(1.0, 1.0, 1.0, 150.0 / 255.0))
                                .show();
                        });

                    // ── Settings control panel ────────────────────────────────
                    ui.container()
                        .absolute(580.0, 140.0)
                        .width(460.0)
                        .height(460.0)
                        .backdrop_blur(20.0)
                        .bg(Color::WHITE)
                        .bg_opacity(0.05)
                        .border(Color::rgba(1.0, 1.0, 1.0, 45.0 / 255.0), 1.0)
                        .radius_all(24.0)
                        .shadow(Color::rgba(0.0, 0.0, 0.0, 60.0 / 255.0), 0.0, 12.0, 32.0)
                        .padding_all(28.0)
                        .gap(20.0)
                        .show(|ui| {
                            // Tab switcher
                            ui.container()
                                .full_width()
                                .row()
                                .gap(8.0)
                                .show(|ui| {
                                    for (i, title) in ["Shader Info", "Interactive Params"].iter().enumerate() {
                                        let active = active_tab == i;
                                        let btn = ui.button(*title)
                                            .width(180.0)
                                            .padding_y(10.0)
                                            .bg(if active { Color::rgba(1.0, 1.0, 1.0, 30.0 / 255.0) } else { Color::TRANSPARENT })
                                            .hover_bg(Color::rgba(1.0, 1.0, 1.0, 20.0 / 255.0))
                                            .radius_all(12.0)
                                            .text_color(if active { Color::WHITE } else { Color::rgba(1.0, 1.0, 1.0, 150.0 / 255.0) })
                                            .size(13.0)
                                            .show();
                                        if btn.clicked {
                                            active_tab = i;
                                        }
                                    }
                                });

                            // Separator
                            ui.container()
                                .full_width()
                                .height(1.0)
                                .bg(Color::rgba(1.0, 1.0, 1.0, 30.0 / 255.0))
                                .show(|_| {});

                            match active_tab {
                                0 => {
                                    ui.container()
                                        .column()
                                        .gap(12.0)
                                        .show(|ui| {
                                            ui.text("Registered Custom Shaders:")
                                                .size(15.0)
                                                .weight(FontWeight::SemiBold)
                                                .color(Color::WHITE)
                                                .show();
                                            ui.text("  - ID: 'liquid'\n  - File: 'liquid_distortion.wgsl'\n  - Inputs: BackdropUniforms (rect_pos, rect_size, time)\n  - Rendering: Procedural coordinate warp")
                                                .size(13.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 170.0 / 255.0))
                                                .show();
                                            ui.text("Extending Zenthra:")
                                                .size(15.0)
                                                .weight(FontWeight::SemiBold)
                                                .color(Color::WHITE)
                                                .show();
                                            ui.text("To create custom post-processors, call App::register_custom_shader() with a fragment module containing a fn fs_main(in: BackdropVsOut) -> vec4<f32>.")
                                                .size(13.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 140.0 / 255.0))
                                                .show();
                                        });
                                }
                                1 => {
                                    ui.container()
                                        .column()
                                        .full_width()
                                        .gap(14.0)
                                        .show(|ui| {
                                             ui.text("Distortion Warp & Blur Radius")
                                                 .size(14.0)
                                                 .color(Color::rgba(1.0, 1.0, 1.0, 180.0 / 255.0))
                                                 .show();
                                             ui.slider(&mut slider_val, "blur_slider").width(400.0).show();
                                             ui.text(&format!("blur: {:.0} px  •  warp: {:.1} px", slider_val * 40.0, slider_val * 40.0 * 0.6))
                                                 .size(13.0)
                                                 .color(Color::rgba(1.0, 1.0, 1.0, 140.0 / 255.0))
                                                 .show();

                                            ui.text("Backdrop Blur Underlay")
                                                .size(14.0)
                                                .color(Color::rgba(1.0, 1.0, 1.0, 180.0 / 255.0))
                                                .show();
                                            let btn = ui.button(if toggle { "● Blur Underlay Enabled" } else { "○ Blur Underlay Disabled" })
                                                .width(400.0)
                                                .padding_y(10.0)
                                                .bg(if toggle { Color::rgba(0.0, 200.0 / 255.0, 255.0 / 255.0, 180.0 / 255.0) } else { Color::rgba(1.0, 1.0, 1.0, 20.0 / 255.0) })
                                                .hover_bg(if toggle { Color::rgba(0.0, 200.0 / 255.0, 255.0 / 255.0, 220.0 / 255.0) } else { Color::rgba(1.0, 1.0, 1.0, 40.0 / 255.0) })
                                                .radius_all(12.0)
                                                .text_color(Color::WHITE)
                                                .size(14.0)
                                                .show();
                                            if btn.clicked {
                                                toggle = !toggle;
                                            }
                                        });
                                }
                                _ => {}
                            }
                        });
                });
        })
        .run();
}
