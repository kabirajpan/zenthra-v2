use zenthra::prelude::*;

fn main() {
    let mut sidebar_w = 260.0f32;
    let mut terminal_h = 160.0f32;
    let mut logs_w = 220.0f32;

    App::new()
        .title("Zenthra - Splitter & Resizable Layout Demo")
        .size(1080, 720)
        .with_ui(move |ui: &mut Ui| {
            // Main Container filling entire window
            ui.container()
                .fill()
                .bg(Color::rgb(0.08, 0.09, 0.12))
                .column()
                .show(|ui: &mut Ui| {
                    // ── Header Bar ──────────────────────────────────────────
                    ui.container()
                        .fill_x()
                        .height(44.0)
                        .padding(0.0, 16.0, 0.0, 16.0)
                        .bg(Color::rgb(0.12, 0.13, 0.17))
                        .border((Color::rgb(0.18, 0.20, 0.26), 1.0))
                        .row()
                        .valign(zenthra::Align::Center)
                        .halign(zenthra::Align::SpaceBetween)
                        .show(|ui: &mut Ui| {
                            ui.container().row().gap(8.0).valign(zenthra::Align::Center).show(|ui: &mut Ui| {
                                ui.text("⚡")
                                    .size(18.0)
                                    .color(Color::rgb(0.95, 0.80, 0.30))
                                    .show();
                                ui.text("Zenthra Studio")
                                    .size(15.0)
                                    .font_weight(FontWeight::Bold)
                                    .color(Color::WHITE)
                                    .show();
                                ui.text("v0.2.4")
                                    .size(10.5)
                                    .color(Color::rgb(0.40, 0.65, 1.0))
                                    .padding(2.0, 6.0, 2.0, 6.0)
                                    .bg(Color::rgb(0.16, 0.22, 0.35))
                                    .radius_all(4.0)
                                    .show();
                            });

                            ui.container().row().gap(12.0).valign(zenthra::Align::Center).show(|ui: &mut Ui| {
                                ui.text(&format!("Sidebar: {:.0}px", sidebar_w))
                                    .size(11.0)
                                    .color(Color::rgb(0.60, 0.65, 0.75))
                                    .show();
                                ui.text(&format!("Terminal: {:.0}px", terminal_h))
                                    .size(11.0)
                                    .color(Color::rgb(0.60, 0.65, 0.75))
                                    .show();
                            });
                        });

                    // ── Body Layout (Horizontal Split: Sidebar | Editor | Logs) ──
                    ui.container()
                        .fill()
                        .row()
                        .show(|ui: &mut Ui| {
                            // 1. Left Panel (File Explorer / Sidebar)
                            ui.container()
                                .width(sidebar_w)
                                .fill_y()
                                .bg(Color::rgb(0.09, 0.10, 0.14))
                                .padding(12.0, 14.0, 12.0, 14.0)
                                .column()
                                .gap(8.0)
                                .show(|ui: &mut Ui| {
                                    ui.text("PROJECT EXPLORER")
                                        .size(11.0)
                                        .font_weight(FontWeight::Bold)
                                        .color(Color::rgb(0.50, 0.55, 0.65))
                                        .show();

                                    let files = [
                                        ("📁 src", true),
                                        ("  📄 main.rs", false),
                                        ("  📄 lib.rs", false),
                                        ("  📄 splitter.rs", false),
                                        ("📁 examples", true),
                                        ("  📄 card.rs", false),
                                        ("  📄 slider.rs", false),
                                        ("📦 Cargo.toml", false),
                                        ("📝 README.md", false),
                                    ];

                                    for (item, is_dir) in files {
                                        ui.container()
                                            .fill_x()
                                            .padding(4.0, 8.0, 4.0, 8.0)
                                            .radius_all(4.0)
                                            .hover(Style::bg(Color::rgb(0.14, 0.16, 0.22)))
                                            .show(|ui: &mut Ui| {
                                                ui.text(item)
                                                    .size(12.5)
                                                    .color(if is_dir {
                                                        Color::rgb(0.90, 0.75, 0.40)
                                                    } else {
                                                        Color::rgb(0.80, 0.85, 0.92)
                                                    })
                                                    .show();
                                            });
                                    }
                                });

                            // ── Vertical Splitter 1 (Left ↔ Middle) ──
                            ui.splitter(&mut sidebar_w, 160.0..=480.0)
                                .width(4.0)
                                .fill_y()
                                .bg(Color::rgb(0.15, 0.17, 0.23))
                                .hover(Style::bg(Color::rgb(0.35, 0.60, 1.0)))
                                .active(Style::bg(Color::rgb(0.50, 0.75, 1.0)))
                                .show();

                            // 2. Middle Panel (Code Editor + Terminal with Vertical Split)
                            ui.container()
                                .fill()
                                .column()
                                .show(|ui: &mut Ui| {
                                    // Upper: Code Editor
                                    ui.container()
                                        .fill()
                                        .bg(Color::rgb(0.06, 0.07, 0.10))
                                        .padding(16.0, 20.0, 16.0, 20.0)
                                        .column()
                                        .gap(8.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container().row().gap(8.0).show(|ui: &mut Ui| {
                                                ui.text("splitter.rs")
                                                    .size(12.0)
                                                    .font_weight(FontWeight::Medium)
                                                    .color(Color::WHITE)
                                                    .padding(4.0, 10.0, 4.0, 10.0)
                                                    .bg(Color::rgb(0.14, 0.16, 0.22))
                                                    .radius_all(4.0)
                                                    .show();
                                            });

                                            ui.text("// Try dragging the splitter dividers with your mouse or trackpad!")
                                                .size(12.5)
                                                .color(Color::rgb(0.40, 0.50, 0.60))
                                                .show();

                                            ui.text("ui.splitter(&mut sidebar_w, 160.0..=480.0)")
                                                .size(13.0)
                                                .font_weight(FontWeight::SemiBold)
                                                .color(Color::rgb(0.45, 0.85, 0.60))
                                                .show();
                                            ui.text("    .width(4.0).fill_y()")
                                                .size(13.0)
                                                .color(Color::rgb(0.70, 0.80, 0.95))
                                                .show();
                                            ui.text("    .bg(Color::rgb(0.15, 0.17, 0.23))")
                                                .size(13.0)
                                                .color(Color::rgb(0.70, 0.80, 0.95))
                                                .show();
                                            ui.text("    .hover(Style::bg(Color::rgb(0.35, 0.60, 1.0)))")
                                                .size(13.0)
                                                .color(Color::rgb(0.70, 0.80, 0.95))
                                                .show();
                                            ui.text("    .show();")
                                                .size(13.0)
                                                .color(Color::rgb(0.70, 0.80, 0.95))
                                                .show();
                                        });

                                    // ── Horizontal Splitter (Editor ↕ Terminal) ──
                                    ui.splitter(&mut terminal_h, 80.0..=450.0)
                                        .fill_x()
                                        .height(4.0)
                                        .bg(Color::rgb(0.15, 0.17, 0.23))
                                        .hover(Style::bg(Color::rgb(0.35, 0.60, 1.0)))
                                        .active(Style::bg(Color::rgb(0.50, 0.75, 1.0)))
                                        .invert(true) // Dragging up expands bottom panel!
                                        .show();

                                    // Lower: Terminal
                                    ui.container()
                                        .fill_x()
                                        .height(terminal_h)
                                        .bg(Color::rgb(0.04, 0.05, 0.07))
                                        .padding(12.0, 16.0, 12.0, 16.0)
                                        .column()
                                        .gap(6.0)
                                        .show(|ui: &mut Ui| {
                                            ui.text("TERMINAL")
                                                .size(10.5)
                                                .font_weight(FontWeight::Bold)
                                                .color(Color::rgb(0.45, 0.50, 0.60))
                                                .show();

                                            ui.text("kabir@pop-os:~/projects/zenthra-v2$ cargo check --workspace")
                                                .size(12.0)
                                                .color(Color::rgb(0.60, 0.90, 0.60))
                                                .show();
                                            ui.text("   Compiling zenthra v0.2.4 (clean, 0 warnings)")
                                                .size(12.0)
                                                .color(Color::rgb(0.75, 0.80, 0.85))
                                                .show();
                                            ui.text("   Finished `dev` profile in 0.84s")
                                                .size(12.0)
                                                .color(Color::rgb(0.50, 0.75, 1.0))
                                                .show();
                                        });
                                });

                            // ── Vertical Splitter 2 (Middle ↔ Right Logs Panel) ──
                            ui.splitter(&mut logs_w, 140.0..=400.0)
                                .width(4.0)
                                .fill_y()
                                .bg(Color::rgb(0.15, 0.17, 0.23))
                                .hover(Style::bg(Color::rgb(0.35, 0.60, 1.0)))
                                .active(Style::bg(Color::rgb(0.50, 0.75, 1.0)))
                                .invert(true) // Dragging left expands right panel!
                                .show();

                            // 3. Right Panel (Output / Telemetry)
                            ui.container()
                                .width(logs_w)
                                .fill_y()
                                .bg(Color::rgb(0.09, 0.10, 0.14))
                                .padding(12.0, 14.0, 12.0, 14.0)
                                .column()
                                .gap(8.0)
                                .show(|ui: &mut Ui| {
                                    ui.text("DEBUG METRICS")
                                        .size(11.0)
                                        .font_weight(FontWeight::Bold)
                                        .color(Color::rgb(0.50, 0.55, 0.65))
                                        .show();

                                    ui.text(&format!("FPS: {:.0}", 60.0))
                                        .size(12.0)
                                        .color(Color::rgb(0.40, 0.85, 0.60))
                                        .show();
                                    ui.text(&format!("Draw Calls: 4"))
                                        .size(12.0)
                                        .color(Color::rgb(0.70, 0.75, 0.85))
                                        .show();
                                    ui.text(&format!("Mouse: ({:.0}, {:.0})", ui.mouse_x, ui.mouse_y))
                                        .size(12.0)
                                        .color(Color::rgb(0.70, 0.75, 0.85))
                                        .show();
                                });
                        });
                });
        })
        .run();
}
