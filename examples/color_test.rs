// examples/color_test.rs
//
// Run with:
//   cargo run --example color_test -p zenthra

use zenthra::prelude::*;

struct Swatch {
    name: &'static str,
    color: Color,
}

fn swatches() -> Vec<Swatch> {
    vec![
        Swatch { name: "bg_main\n(23,22,22)",       color: Color::from_hex(0x171616FF) },
        Swatch { name: "bg_button\n(26,25,25)",     color: Color::from_hex(0x1A1919FF) },
        Swatch { name: "bg_sidebar\n(10,10,10)",    color: Color::from_hex(0x0A0A0AFF) },
        Swatch { name: "bg_active\n(20,18,0)",      color: Color::from_hex(0x141200FF) },
        Swatch { name: "border\n(24,24,24)",        color: Color::from_hex(0x181818FF) },
        Swatch { name: "text_primary\n(224,224,224)", color: Color::from_hex(0xE0E0E0FF) },
        Swatch { name: "text_muted\n(102,102,102)", color: Color::from_hex(0x666666FF) },
        Swatch { name: "accent\n(200,169,110)",     color: Color::from_hex(0xC8A96EFF) },
        Swatch { name: "WHITE\n(255,255,255)",      color: Color::WHITE },
        Swatch { name: "BLACK\n(0,0,0)",            color: Color::BLACK },
    ]
}

// 20 black → near-white steps  (every 13 units in 0-255)
fn shade_steps() -> Vec<(u8, Color)> {
    (0u8..=20u8)
        .map(|i| {
            let v = (i as u16 * 13).min(255) as u8;
            (v, Color::rgb(v as f32 / 255.0, v as f32 / 255.0, v as f32 / 255.0))
        })
        .collect()
}

// Fine-grained: every single value from 0 to 13
fn fine_shade_steps() -> Vec<(u8, Color)> {
    (0u8..=13u8)
        .map(|v| {
            (v, Color::rgb(v as f32 / 255.0, v as f32 / 255.0, v as f32 / 255.0))
        })
        .collect()
}

fn main() {
    let sw   = swatches();
    let shd  = shade_steps();
    let fine = fine_shade_steps();

    App::new()
        .title("Zenthra – Color Palette")
        .size(960, 760)
        .with_ui(move |ui: &mut Ui| {
            ui.container()
                .fill()
                .bg(Color::from_hex(0x171616FF))
                .padding(24.0, 24.0, 24.0, 24.0)
                .gap(16.0)
                .show(|ui| {

                    // ── Title ─────────────────────────────────────────────────
                    ui.text("Design-System Color Tokens")
                        .size(18.0)
                        .color(Color::from_hex(0xE0E0E0FF))
                        .show();

                    ui.text("Dark theme — every token used in the UI")
                        .size(12.0)
                        .color(Color::from_hex(0x666666FF))
                        .show();

                    ui.spacing(4.0);

                    // ── Token swatches ────────────────────────────────────────
                    ui.container()
                        .fill_x()
                        .row()
                        .gap(10.0)
                        .wrap(Wrap::Wrap)
                        .show(|ui| {
                            for s in &sw {
                                ui.container()
                                    .width(170.0)
                                    .height(110.0)
                                    .bg(Color::from_hex(0x1A1919FF))
                                    .border(Color::from_hex(0x181818FF), 1.0)
                                    .radius_all(7.0)
                                    .gap(0.0)
                                    .show(|ui| {
                                        ui.container()
                                            .fill_x()
                                            .height(65.0)
                                            .bg(s.color)
                                            .radius(7.0, 7.0, 0.0, 0.0)
                                            .show(|_ui| {});

                                        ui.container()
                                            .fill_x()
                                            .height(45.0)
                                            .padding(8.0, 8.0, 6.0, 6.0)
                                            .show(|ui| {
                                                ui.text(s.name)
                                                    .size(10.5)
                                                    .color(Color::from_hex(0x888888FF))
                                                    .show();
                                            });
                                    });
                            }
                        });

                    ui.spacing(8.0);

                    // ── Black Shades Bar ──────────────────────────────────────
                    ui.text("Black Shades  (0 → 255, step 13)")
                        .size(12.0)
                        .color(Color::from_hex(0x666666FF))
                        .show();

                    // Shade cells in a single scrollable row
                    ui.container()
                        .fill_x()
                        .row()
                        .gap(3.0)
                        .show(|ui| {
                            for (v, col) in &shd {
                                ui.container()
                                    .width(40.0)
                                    .height(80.0)
                                    .gap(0.0)
                                    .radius_all(5.0)
                                    .show(|ui| {
                                        // Colour block
                                        ui.container()
                                            .fill_x()
                                            .height(52.0)
                                            .bg(*col)
                                            .border(Color::from_hex(0x2A2A2AFF), 1.0)
                                            .radius(5.0, 5.0, 0.0, 0.0)
                                            .show(|_| {});

                                        // Value label
                                        ui.container()
                                            .fill_x()
                                            .height(28.0)
                                            .align(Align::Center)
                                            .show(|ui| {
                                                ui.text(&format!("{}", v))
                                                    .size(9.5)
                                                    .color(Color::from_hex(0x666666FF))
                                                    .show();
                                            });
                                    });
                            }
                        });

                    ui.spacing(8.0);

                    // ── Fine Black Shades Bar (0 → 13, step 1) ───────────────
                    ui.text("Fine Black Shades  (0 → 13, step 1)  — near-black detail")
                        .size(12.0)
                        .color(Color::from_hex(0x666666FF))
                        .show();

                    ui.container()
                        .fill_x()
                        .row()
                        .gap(3.0)
                        .show(|ui| {
                            for (v, col) in &fine {
                                // Each cell is wider so you can actually see the difference
                                ui.container()
                                    .width(62.0)
                                    .height(90.0)
                                    .gap(0.0)
                                    .radius_all(5.0)
                                    .show(|ui| {
                                        ui.container()
                                            .fill_x()
                                            .height(60.0)
                                            .bg(*col)
                                            .border(Color::from_hex(0x2A2A2AFF), 1.0)
                                            .radius(5.0, 5.0, 0.0, 0.0)
                                            .show(|_| {});

                                        ui.container()
                                            .fill_x()
                                            .height(30.0)
                                            .align(Align::Center)
                                            .show(|ui| {
                                                ui.text(&format!("{}", v))
                                                    .size(10.5)
                                                    .color(Color::from_hex(0x888888FF))
                                                    .show();
                                            });
                                    });
                            }
                        });

                    ui.spacing(4.0);

                    // ── Text colour reference ─────────────────────────────────
                    ui.container()
                        .row()
                        .fill_x()
                        .gap(12.0)
                        .align(Align::Center)
                        .show(|ui| {
                            ui.text("accent →")
                                .size(12.0)
                                .color(Color::from_hex(0x444444FF))
                                .show();
                            ui.text("Hover / Active")
                                .size(12.0)
                                .color(Color::from_hex(0xC8A96EFF))
                                .show();

                            ui.spacing(16.0);

                            ui.text("text_primary →")
                                .size(12.0)
                                .color(Color::from_hex(0x444444FF))
                                .show();
                            ui.text("Normal text")
                                .size(12.0)
                                .color(Color::from_hex(0xE0E0E0FF))
                                .show();

                            ui.spacing(16.0);

                            ui.text("text_muted →")
                                .size(12.0)
                                .color(Color::from_hex(0x444444FF))
                                .show();
                            ui.text("Ctrl+S")
                                .size(12.0)
                                .color(Color::from_hex(0x666666FF))
                                .show();
                        });
                });
        })
        .run();
}
