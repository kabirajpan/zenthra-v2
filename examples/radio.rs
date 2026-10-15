use zenthra::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThemeMode {
    Dark,
    Light,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccentColor {
    Emerald,
    Violet,
    Crimson,
    Amber,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LayoutDensity {
    Compact,
    Comfortable,
    Spacious,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StorageTier {
    Free,
    Pro,
    Enterprise,
}

fn main() {
    // ── Radio States ───────────────────────────────────────────────────────────
    let mut selected_theme = ThemeMode::Dark;
    let mut selected_accent = AccentColor::Violet;
    let mut selected_density = LayoutDensity::Comfortable;
    let mut selected_tier = StorageTier::Pro;

    let mut event_log = String::from("Interactive radio selections will appear here.");

    App::new()
        .title("Zenthra - Radio Button API Showcase")
        .size(1050, 920)
        .with_ui(move |ui: &mut Ui| {
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
                            ui.h1("Radio Button API & Unified Styling Showcase")
                                .color(Color::WHITE)
                                .show();

                            ui.text("Demonstrating generic enum state binding, animated inner dot expansion, custom ring & dot palettes, active label transitions, neon glow, and sizing.")
                                .size(14.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });

                    // ── STATUS BAR & LIVE SELECTION FEEDBACK ──────────────────
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
                            ui.text(&format!("Theme: {:?}", selected_theme))
                                .size(14.0)
                                .font_weight(FontWeight::Bold)
                                .color(Color::rgb(0.35, 0.75, 1.0))
                                .show();

                            ui.text(&format!("Accent: {:?}", selected_accent))
                                .size(14.0)
                                .font_weight(FontWeight::Bold)
                                .color(match selected_accent {
                                    AccentColor::Emerald => Color::rgb(0.20, 0.85, 0.50),
                                    AccentColor::Violet  => Color::rgb(0.70, 0.45, 1.00),
                                    AccentColor::Crimson => Color::rgb(0.95, 0.30, 0.40),
                                    AccentColor::Amber   => Color::rgb(0.95, 0.65, 0.20),
                                })
                                .show();

                            ui.text(&format!("Tier: {:?}", selected_tier))
                                .size(14.0)
                                .font_weight(FontWeight::Bold)
                                .color(Color::rgb(0.95, 0.80, 0.30))
                                .show();

                            ui.text(&format!("Event: {}", event_log))
                                .size(13.0)
                                .color(Color::rgb(0.75, 0.80, 0.90))
                                .show();
                        });

                    // ── SECTION 1: STANDARD SIZING & HORIZONTAL RADIO GROUP ───
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("1. Horizontal Radio Group (Theme Selection)")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.row()
                                .fill_x()
                                .gap(32.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    let resp1 = ui.radio(&mut selected_theme, ThemeMode::Dark, "Dark Mode")
                                        .size(18.0)
                                        .dot_color(Color::rgb(0.35, 0.65, 1.0))
                                        .active(Text::color(Color::rgb(0.40, 0.75, 1.0)))
                                        .show();
                                    if resp1.clicked {
                                        event_log = String::from("Selected Dark Mode");
                                    }

                                    let resp2 = ui.radio(&mut selected_theme, ThemeMode::Light, "Light Mode")
                                        .size(18.0)
                                        .dot_color(Color::rgb(0.35, 0.65, 1.0))
                                        .active(Text::color(Color::rgb(0.40, 0.75, 1.0)))
                                        .show();
                                    if resp2.clicked {
                                        event_log = String::from("Selected Light Mode");
                                    }

                                    let resp3 = ui.radio(&mut selected_theme, ThemeMode::System, "System Default")
                                        .size(18.0)
                                        .dot_color(Color::rgb(0.35, 0.65, 1.0))
                                        .active(Text::color(Color::rgb(0.40, 0.75, 1.0)))
                                        .show();
                                    if resp3.clicked {
                                        event_log = String::from("Selected System Default");
                                    }
                                });
                        });

                    // ── SECTION 2: PALETTES & NEON GLOW ────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("2. Vibrant Themes, Glow Effects & Micro-Interactions")
                                .color(Color::rgb(0.10, 0.80, 0.50))
                                .show();

                            ui.row()
                                .fill_x()
                                .gap(28.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    // Emerald
                                    ui.radio(&mut selected_accent, AccentColor::Emerald, "Emerald")
                                        .colors(Color::rgb(0.10, 0.80, 0.50), Color::rgb(0.10, 0.80, 0.50))
                                        .glow(true)
                                        .scaling(1.08, 0.95)
                                        .active(Text::color(Color::rgb(0.20, 0.90, 0.60)))
                                        .show();

                                    // Violet
                                    ui.radio(&mut selected_accent, AccentColor::Violet, "Violet")
                                        .colors(Color::rgb(0.65, 0.35, 1.00), Color::rgb(0.70, 0.45, 1.00))
                                        .glow(true)
                                        .scaling(1.08, 0.95)
                                        .active(Text::color(Color::rgb(0.75, 0.50, 1.0)))
                                        .show();

                                    // Crimson
                                    ui.radio(&mut selected_accent, AccentColor::Crimson, "Crimson")
                                        .colors(Color::rgb(0.95, 0.25, 0.35), Color::rgb(0.95, 0.25, 0.35))
                                        .glow(true)
                                        .scaling(1.08, 0.95)
                                        .active(Text::color(Color::rgb(1.0, 0.40, 0.50)))
                                        .show();

                                    // Amber
                                    ui.radio(&mut selected_accent, AccentColor::Amber, "Amber")
                                        .colors(Color::rgb(0.95, 0.65, 0.20), Color::rgb(0.95, 0.65, 0.20))
                                        .glow(true)
                                        .scaling(1.08, 0.95)
                                        .active(Text::color(Color::rgb(1.0, 0.75, 0.30)))
                                        .show();
                                });
                        });

                    // ── SECTION 3: SIZING & SCALING COMPARISON ─────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("3. Sizing Comparison (14px Compact, 18px Standard, 24px Hero)")
                                .color(Color::rgb(0.70, 0.45, 1.0))
                                .show();

                            ui.row()
                                .fill_x()
                                .gap(32.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    ui.radio(&mut selected_density, LayoutDensity::Compact, "Compact (14px)")
                                        .size(14.0)
                                        .gap(6.0)
                                        .label_size(12.0)
                                        .active(Text::color(Color::rgb(0.70, 0.45, 1.0)))
                                        .show();

                                    ui.radio(&mut selected_density, LayoutDensity::Comfortable, "Comfortable (18px)")
                                        .size(18.0)
                                        .gap(10.0)
                                        .label_size(14.0)
                                        .active(Text::color(Color::rgb(0.70, 0.45, 1.0)))
                                        .show();

                                    ui.radio(&mut selected_density, LayoutDensity::Spacious, "Spacious (24px)")
                                        .size(24.0)
                                        .gap(12.0)
                                        .label_size(15.0)
                                        .active(Text::color(Color::rgb(0.70, 0.45, 1.0)))
                                        .show();
                                });
                        });

                    // ── SECTION 4: VERTICAL RADIO LIST (PRICING TIERS) ─────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(14.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("4. Vertical Radio List with Custom Callbacks")
                                .color(Color::rgb(0.95, 0.65, 0.25))
                                .show();

                            ui.radio(&mut selected_tier, StorageTier::Free, "Starter Tier (5 GB Cloud Storage)")
                                .active(Text::color(Color::rgb(0.95, 0.80, 0.30)))
                                .on_change(|tier| {
                                    println!("Selected tier changed to: {:?}", tier);
                                })
                                .show();

                            ui.radio(&mut selected_tier, StorageTier::Pro, "Pro Developer (1 TB NVMe Storage + GPU Accel)")
                                .active(Text::color(Color::rgb(0.95, 0.80, 0.30)))
                                .font_weight(FontWeight::Medium)
                                .on_change(|tier| {
                                    println!("Selected tier changed to: {:?}", tier);
                                })
                                .show();

                            ui.radio(&mut selected_tier, StorageTier::Enterprise, "Enterprise Dedicated Cluster (Unlimited Storage)")
                                .active(Text::color(Color::rgb(0.95, 0.80, 0.30)))
                                .on_change(|tier| {
                                    println!("Selected tier changed to: {:?}", tier);
                                })
                                .show();
                        });

                    // ── SECTION 5: DISABLED RADIO BUTTONS ─────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("5. Disabled State Handling")
                                .color(Color::rgb(0.65, 0.70, 0.80))
                                .show();

                            ui.row()
                                .fill_x()
                                .gap(32.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    let mut dummy_active = true;
                                    ui.radio(&mut dummy_active, true, "Disabled (Selected)")
                                        .disabled(true)
                                        .show();

                                    let mut dummy_inactive = true;
                                    ui.radio(&mut dummy_inactive, false, "Disabled (Unselected)")
                                        .disabled(true)
                                        .show();
                                });
                        });
                });
        })
        .run();
}
