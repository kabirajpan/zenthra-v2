use zenthra::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Environment {
    Development,
    Staging,
    Production,
    Testing,
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Development => write!(f, "Development (Local)"),
            Environment::Staging => write!(f, "Staging (Pre-Release)"),
            Environment::Production => write!(f, "Production (Live)"),
            Environment::Testing => write!(f, "Testing (CI/CD)"),
        }
    }
}

fn main() {
    // Dropdown state variables
    let mut selected_theme = "Dark Velvet";
    let mut selected_env = Environment::Production;
    let mut selected_log_level = "Info";

    let mut selected_region = "us-east-1 (N. Virginia)";
    let mut selected_lang = "Rust 1.82";

    let mut selected_font = "JetBrains Mono";
    let mut selected_audio_output = "Studio Headphones (USB Audio)";

    let mut selected_preset = "Cyberpunk Violet";
    let mut selected_capsule = "High Performance Mode";

    let mut event_log = String::from("Ready - select an option from any dropdown.");

    App::new()
        .title("Zenthra - Dropdown API & Styling Showcase")
        .size(1080, 950)
        .with_ui(move |ui: &mut Ui| {
            if ui.clicked {
                eprintln!("\n=== [FRAME CLICK] mouse=({:.1}, {:.1}) mouse_down={} ===", ui.mouse_x, ui.mouse_y, ui.mouse_down);
            }

            // Root scrollable background container
            ui.container()
                .fill()
                .bg(Color::rgb(0.06, 0.07, 0.09))
                .padding_all(24.0)
                .scroll_y(true)
                .gap(24.0)
                .show(|ui: &mut Ui| {
                    // ── HEADER CARD ─────────────────────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(12.0)
                        .padding_all(20.0)
                        .column()
                        .gap(10.0)
                        .show(|ui: &mut Ui| {
                            ui.h1("Dropdown API & Styling Showcase")
                                .color(Color::WHITE)
                                .font_weight(FontWeight::Bold)
                                .show();

                            ui.text("Demonstrating popup menus, custom borders, menu height limits, directional placement, glow effects, and typed enum bindings.")
                                .size(14.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();

                            // Live Status Bar & Reset
                            ui.row()
                                .fill_x()
                                .valign(Align::Center)
                                .gap(16.0)
                                .padding_all(12.0)
                                .bg(Color::rgb(0.07, 0.08, 0.11))
                                .border((Color::rgb(0.14, 0.17, 0.23), 1.0))
                                .radius_all(8.0)
                                .show(|ui: &mut Ui| {
                                    ui.row().valign(Align::Center).gap(8.0).show(|ui: &mut Ui| {
                                        ui.text("Active Theme:")
                                            .size(13.0)
                                            .color(Color::rgb(0.50, 0.55, 0.65))
                                            .show();
                                        ui.text(selected_theme)
                                            .size(13.0)
                                            .font_weight(FontWeight::Bold)
                                            .color(Color::rgb(0.35, 0.75, 1.0))
                                            .show();
                                    });

                                    ui.text("•").size(14.0).color(Color::rgb(0.30, 0.35, 0.45)).show();

                                    ui.row().valign(Align::Center).gap(8.0).show(|ui: &mut Ui| {
                                        ui.text("Env:")
                                            .size(13.0)
                                            .color(Color::rgb(0.50, 0.55, 0.65))
                                            .show();
                                        ui.text(&selected_env.to_string())
                                            .size(13.0)
                                            .font_weight(FontWeight::Bold)
                                            .color(Color::rgb(0.20, 0.85, 0.50))
                                            .show();
                                    });

                                    ui.text("•").size(14.0).color(Color::rgb(0.30, 0.35, 0.45)).show();

                                    ui.text(&event_log)
                                        .size(12.0)
                                        .color(Color::rgb(0.95, 0.75, 0.25))
                                        .show();
                                });
                        });

                    // ── SECTION 1: STANDARD DROPDOWNS & TYPED ENUMS ─────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(18.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("1. Standard Dropdowns & Typed Enums")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.row().fill_x().gap(24.0).show(|ui: &mut Ui| {
                                // Theme Selector
                                ui.column().gap(6.0).show(|ui: &mut Ui| {
                                    ui.text("Color Theme").size(13.0).color(Color::rgb(0.70, 0.75, 0.85)).show();

                                    let themes = vec![
                                        "Dark Velvet",
                                        "Midnight Blue",
                                        "Cyberpunk Neon",
                                        "Emerald Forest",
                                        "Solar Flare",
                                    ];

                                    let resp = ui.dropdown(&mut selected_theme, themes)
                                        .id("theme_dropdown")
                                        .width(220.0)
                                        .height(36.0)
                                        .radius_all(6.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.19))
                                        .border((Color::rgb(0.22, 0.26, 0.35), 1.0))
                                        .hover(Style::bg(Color::rgb(0.16, 0.19, 0.26)))
                                        .show();

                                    if ui.clicked {
                                        eprintln!("  -> [theme_dropdown] hovered={} changed={}", resp.hovered, resp.changed);
                                    }

                                    if resp.changed {
                                        event_log = format!("Theme switched to: {}", selected_theme);
                                    }
                                });

                                // Environment (Typed Enum)
                                ui.column().gap(6.0).show(|ui: &mut Ui| {
                                    ui.text("Target Environment (Enum)").size(13.0).color(Color::rgb(0.70, 0.75, 0.85)).show();

                                    let envs = vec![
                                        Environment::Development,
                                        Environment::Staging,
                                        Environment::Production,
                                        Environment::Testing,
                                    ];

                                    let resp = ui.dropdown(&mut selected_env, envs)
                                        .id("env_dropdown")
                                        .width(240.0)
                                        .height(36.0)
                                        .radius_all(6.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.19))
                                        .border((Color::rgb(0.22, 0.26, 0.35), 1.0))
                                        .hover(Style::bg(Color::rgb(0.16, 0.19, 0.26)))
                                        .show();

                                    if ui.clicked {
                                        eprintln!("  -> [env_dropdown] hovered={} changed={}", resp.hovered, resp.changed);
                                    }

                                    if resp.changed {
                                        event_log = format!("Environment changed to: {}", selected_env);
                                    }
                                });

                                // Log Level
                                ui.column().gap(6.0).show(|ui: &mut Ui| {
                                    ui.text("Log Severity").size(13.0).color(Color::rgb(0.70, 0.75, 0.85)).show();

                                    let levels = vec!["Trace", "Debug", "Info", "Warn", "Error"];

                                    let resp = ui.dropdown(&mut selected_log_level, levels)
                                        .id("log_dropdown")
                                        .width(160.0)
                                        .height(36.0)
                                        .radius_all(6.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.19))
                                        .border((Color::rgb(0.22, 0.26, 0.35), 1.0))
                                        .hover(Style::bg(Color::rgb(0.16, 0.19, 0.26)))
                                        .show();

                                    if ui.clicked {
                                        eprintln!("  -> [log_dropdown] hovered={} changed={}", resp.hovered, resp.changed);
                                    }

                                    if resp.changed {
                                        event_log = format!("Log Level set to: {}", selected_log_level);
                                    }
                                });
                            });
                        });

                    // ── SECTION 2: FULL-WIDTH FORM CONTROLS (.fill_x()) ──────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(18.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("2. Full-Width Responsive Form Controls (.fill_x())")
                                .color(Color::rgb(0.10, 0.80, 0.50))
                                .show();

                            // Cloud Datacenter Region
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                    ui.text("Primary Compute Region").size(13.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    ui.text("Required").size(11.0).color(Color::rgb(0.45, 0.50, 0.60)).show();
                                });

                                let regions = vec![
                                    "us-east-1 (N. Virginia)",
                                    "us-west-2 (Oregon)",
                                    "eu-central-1 (Frankfurt)",
                                    "eu-west-1 (Ireland)",
                                    "ap-southeast-1 (Singapore)",
                                    "ap-northeast-1 (Tokyo)",
                                ];

                                let resp = ui.dropdown(&mut selected_region, regions)
                                    .id("region_dropdown")
                                    .fill_x()
                                    .height(40.0)
                                    .padding_x(14.0)
                                    .radius_all(8.0)
                                    .bg(Color::rgb(0.12, 0.15, 0.20))
                                    .border((Color::rgb(0.20, 0.25, 0.35), 1.0))
                                    .hover(Style::bg(Color::rgb(0.16, 0.20, 0.28)))
                                    .show();

                                if ui.clicked {
                                    eprintln!("  -> [region_dropdown] hovered={} changed={}", resp.hovered, resp.changed);
                                }

                                if resp.changed {
                                    event_log = format!("Deployment region: {}", selected_region);
                                }
                            });

                            // Primary Programming Language Runtime
                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.row().fill_x().halign(Align::SpaceBetween).valign(Align::Center).show(|ui: &mut Ui| {
                                    ui.text("Project Target Language").size(13.0).color(Color::rgb(0.75, 0.80, 0.90)).show();
                                    ui.text("Recommended").size(11.0).color(Color::rgb(0.20, 0.80, 0.50)).show();
                                });

                                let langs = vec![
                                    "Rust 1.82",
                                    "TypeScript 5.6",
                                    "Go 1.23",
                                    "Python 3.12",
                                    "C++ 23",
                                    "Kotlin 2.0",
                                ];

                                let resp = ui.dropdown(&mut selected_lang, langs)
                                    .id("lang_dropdown")
                                    .fill_x()
                                    .height(40.0)
                                    .padding_x(14.0)
                                    .radius_all(8.0)
                                    .bg(Color::rgb(0.12, 0.15, 0.20))
                                    .border((Color::rgb(0.20, 0.25, 0.35), 1.0))
                                    .hover(Style::bg(Color::rgb(0.16, 0.20, 0.28)))
                                    .show();

                                if resp.changed {
                                    event_log = format!("Target language: {}", selected_lang);
                                }
                            });
                        });

                    // ── SECTION 3: CUSTOM POPUP MENU STYLING & CLAMPED HEIGHT ───
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(18.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("3. Clamped Popover Height & Custom Menu Styles")
                                .color(Color::rgb(0.95, 0.65, 0.25))
                                .show();

                            ui.text("Menus with many choices can be clamped to a maximum height using .menu_max_height(...), with custom borders and background colors.")
                                .size(13.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();

                            ui.column().gap(6.0).show(|ui: &mut Ui| {
                                ui.text("Monospace Editor Font (10+ Items, 160px Max Height)")
                                    .size(13.0)
                                    .color(Color::rgb(0.70, 0.75, 0.85))
                                    .show();

                                let fonts = vec![
                                    "JetBrains Mono",
                                    "Fira Code",
                                    "Cascadia Code",
                                    "Source Code Pro",
                                    "Hack",
                                    "Ubuntu Mono",
                                    "Inconsolata",
                                    "DejaVu Sans Mono",
                                    "Monaco",
                                    "Consolas",
                                ];

                                let resp = ui.dropdown(&mut selected_font, fonts)
                                    .id("font_dropdown")
                                    .width(320.0)
                                    .height(38.0)
                                    .radius_all(8.0)
                                    .bg(Color::rgb(0.11, 0.13, 0.18))
                                    .border((Color::rgb(0.24, 0.28, 0.38), 1.0))
                                    .hover(Style::bg(Color::rgb(0.16, 0.20, 0.28)))
                                    .menu_max_height(160.0)
                                    .menu_bg(Color::rgb(0.08, 0.10, 0.14))
                                    .menu_border((Color::rgb(0.35, 0.40, 0.55), 1.0))
                                    .menu_radius_all(8.0)
                                    .show();

                                if resp.changed {
                                    event_log = format!("Font family: {}", selected_font);
                                }
                            });
                        });

                    // ── SECTION 4: PLACEMENT DIRECTION (TOP VS BOTTOM) ─────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(18.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("4. Directional Placement (.placement(Placement::Top))")
                                .color(Color::rgb(0.70, 0.45, 1.0))
                                .show();

                            ui.text("When placed near the bottom edge of a window or inside a dock/toolbar, dropdowns can open upwards to avoid clipping.")
                                .size(13.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();

                            // Bottom dock simulation card
                            ui.container()
                                .fill_x()
                                .bg(Color::rgb(0.12, 0.14, 0.20))
                                .border((Color::rgb(0.22, 0.26, 0.38), 1.0))
                                .radius_all(8.0)
                                .padding_all(16.0)
                                .row()
                                .fill_x()
                                .halign(Align::SpaceBetween)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    ui.column().gap(4.0).show(|ui: &mut Ui| {
                                        ui.text("Audio Output Device")
                                            .size(14.0)
                                            .font_weight(FontWeight::Medium)
                                            .color(Color::WHITE)
                                            .show();
                                        ui.text("Opens upward via .placement(Placement::Top)")
                                            .size(11.0)
                                            .color(Color::rgb(0.50, 0.55, 0.65))
                                            .show();
                                    });

                                    let audio_devices = vec![
                                        "Studio Headphones (USB Audio)",
                                        "MacBook Pro Speakers",
                                        "External Monitor HDMI",
                                        "AirPods Pro (Bluetooth)",
                                    ];

                                    let resp = ui.dropdown(&mut selected_audio_output, audio_devices)
                                        .id("audio_dropdown")
                                        .placement(Placement::Top)
                                        .gap(6.0)
                                        .width(300.0)
                                        .height(38.0)
                                        .radius_all(6.0)
                                        .bg(Color::rgb(0.16, 0.19, 0.28))
                                        .border((Color::rgb(0.30, 0.38, 0.55), 1.0))
                                        .hover(Style::bg(Color::rgb(0.22, 0.26, 0.38)))
                                        .menu_bg(Color::rgb(0.10, 0.12, 0.18))
                                        .menu_border((Color::rgb(0.35, 0.45, 0.65), 1.0))
                                        .menu_radius_all(6.0)
                                        .show();

                                    if resp.changed {
                                        event_log = format!("Audio device switched to: {}", selected_audio_output);
                                    }
                                });
                        });

                    // ── SECTION 5: GLOW EFFECTS, ELEVATION & CAPSULE SHAPES ────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(18.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("5. Glow Highlights, Elevation Shadows & Capsule Pills")
                                .color(Color::rgb(0.30, 0.85, 0.90))
                                .show();

                            ui.row().fill_x().gap(24.0).show(|ui: &mut Ui| {
                                // Cyberpunk Glow Dropdown
                                ui.column().gap(6.0).show(|ui: &mut Ui| {
                                    ui.text("Neon Glow Highlight (.glow(true))")
                                        .size(13.0)
                                        .color(Color::rgb(0.70, 0.75, 0.85))
                                        .show();

                                    let presets = vec![
                                        "Cyberpunk Violet",
                                        "Matrix Green",
                                        "Solar Flare Amber",
                                        "Iceberg Cyan",
                                    ];

                                    let resp = ui.dropdown(&mut selected_preset, presets)
                                        .id("glow_dropdown")
                                        .width(260.0)
                                        .height(40.0)
                                        .glow(true)
                                        .radius_all(8.0)
                                        .bg(Color::rgb(0.14, 0.12, 0.22))
                                        .border((Color::rgb(0.45, 0.25, 0.75), 1.5))
                                        .hover(Style::bg(Color::rgb(0.20, 0.16, 0.32)))
                                        .menu_bg(Color::rgb(0.12, 0.10, 0.18))
                                        .menu_border((Color::rgb(0.55, 0.30, 0.90), 1.5))
                                        .menu_radius_all(8.0)
                                        .show();

                                    if resp.changed {
                                        event_log = format!("Neon theme preset: {}", selected_preset);
                                    }
                                });

                                // Pill Capsule Dropdown with Shadow
                                ui.column().gap(6.0).show(|ui: &mut Ui| {
                                    ui.text("Pill Capsule with Drop Shadow")
                                        .size(13.0)
                                        .color(Color::rgb(0.70, 0.75, 0.85))
                                        .show();

                                    let modes = vec![
                                        "High Performance Mode",
                                        "Balanced Eco Profile",
                                        "Battery Saver Mode",
                                    ];

                                    let resp = ui.dropdown(&mut selected_capsule, modes)
                                        .id("capsule_dropdown")
                                        .width(280.0)
                                        .height(40.0)
                                        .padding_x(18.0)
                                        .radius_all(20.0)
                                        .shadow(Color::rgba(0.0, 0.0, 0.0, 0.50), 0.0, 4.0, 12.0)
                                        .bg(Color::rgb(0.15, 0.18, 0.24))
                                        .border((Color::rgb(0.25, 0.32, 0.44), 1.0))
                                        .hover(Style::bg(Color::rgb(0.20, 0.24, 0.32)))
                                        .menu_bg(Color::rgb(0.11, 0.13, 0.18))
                                        .menu_border((Color::rgb(0.30, 0.38, 0.50), 1.0))
                                        .menu_radius_all(14.0)
                                        .show();

                                    if resp.changed {
                                        event_log = format!("Power mode: {}", selected_capsule);
                                    }
                                });
                            });
                        });
                });
        })
        .run();
}
