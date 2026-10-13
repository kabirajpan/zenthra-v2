use zenthra::prelude::*;

fn main() {
    let mut code_content = String::from(
        "// Welcome to Zenthra TextArea!\n\
         fn calculate_metrics(count: usize) -> f64 {\n\
             let factor = 1.6180339887;\n\
             (count as f64) * factor\n\
         }\n\n\
         // Immediate mode UI with zero lag cursor\n\
         let result = calculate_metrics(42);\n\
         println!(\"Result: {}\", result);\n"
    );

    let mut notes_content = String::new();
    let mut commit_msg = String::from("feat(widgets): standardize TextArea builder with unified BoxStyle padding and font wrap options");
    let mut log_output = String::from("Ready for input...\n");

    App::new()
        .title("Zenthra - TextArea API Showcase")
        .size(1050, 880)
        .with_ui(move |ui: &mut Ui| {
            // Root scrollable container
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
                            ui.h1("TextArea Widget API Showcase")
                                .color(Color::WHITE)
                                .show();

                            ui.text("Demonstrating unified BoxStyle padding, typography, wrap modes, selection highlights, custom cursor colors, and event callbacks.")
                                .size(14.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });

                    // ── SECTION 1: CODE EDITOR STYLE ──────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.row()
                                .fill_x()
                                .gap(12.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    ui.h3("1. Monospace Code Editor")
                                        .color(Color::rgb(0.35, 0.65, 1.0))
                                        .show();

                                    let info = format!("Chars: {} | Lines: {}", code_content.len(), code_content.lines().count());
                                    ui.text(&info)
                                        .size(12.0)
                                        .color(Color::rgb(0.50, 0.55, 0.65))
                                        .show();
                                });

                            ui.text_area(&mut code_content)
                                .fill_x()
                                .height(180.0)
                                .size(13.0)
                                .color(Color::rgb(0.85, 0.90, 0.98))
                                .bg(Color::rgb(0.05, 0.06, 0.08))
                                .border((Color::rgb(0.20, 0.24, 0.32), 1.0))
                                .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.5))
                                .radius_all(6.0)
                                .padding_all(12.0)
                                .cursor_color(Color::rgb(0.35, 0.65, 1.0))
                                .scrollable(true)
                                .show();
                        });

                    // ── SECTION 2: AUTO-WRAPPING NOTE / FORM INPUT ─────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("2. Form Notes with Custom Emerald Focus & Placeholder")
                                .color(Color::rgb(0.10, 0.80, 0.50))
                                .show();

                            ui.text_area(&mut notes_content)
                                .fill_x()
                                .height(110.0)
                                .size(14.0)
                                .placeholder("Write your notes or documentation comments here...")
                                .placeholder_color(Color::rgb(0.40, 0.45, 0.55))
                                .bg(Color::rgb(0.12, 0.14, 0.19))
                                .border((Color::rgb(0.22, 0.25, 0.35), 1.0))
                                .focus_border((Color::rgb(0.10, 0.80, 0.50), 1.5))
                                .cursor_color(Color::rgb(0.10, 0.80, 0.50))
                                .selection_color(Color::rgba(0.10, 0.80, 0.50, 0.30))
                                .radius_all(8.0)
                                .padding_all(12.0)
                                .wrap(TextWrap::Word)
                                .scrollable(true)
                                .show();
                        });

                    // ── SECTION 3: COMMIT MESSAGE & EVENT HANDLERS ────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("3. Commit Message Box with on_change Event Tracking")
                                .color(Color::rgb(0.95, 0.65, 0.25))
                                .show();

                            let resp = ui.text_area(&mut commit_msg)
                                .fill_x()
                                .height(85.0)
                                .size(13.0)
                                .bg(Color::rgb(0.12, 0.14, 0.19))
                                .border((Color::rgb(0.22, 0.25, 0.35), 1.0))
                                .focus_border((Color::rgb(0.95, 0.65, 0.25), 1.5))
                                .cursor_color(Color::rgb(0.95, 0.65, 0.25))
                                .selection_color(Color::rgba(0.95, 0.65, 0.25, 0.35))
                                .radius_all(6.0)
                                .padding_all(10.0)
                                .wrap(TextWrap::Word)
                                .show();

                            if resp.changed {
                                log_output = format!("Last edited: {} chars (modified)", commit_msg.len());
                            }

                            ui.row().gap(8.0).show(|ui: &mut Ui| {
                                ui.text("Status:")
                                    .size(12.0)
                                    .color(Color::rgb(0.50, 0.55, 0.65))
                                    .show();

                                ui.text(&log_output)
                                    .size(12.0)
                                    .color(Color::rgb(0.80, 0.85, 0.95))
                                    .show();
                            });
                        });

                    // ── SECTION 4: TEXT AREA WITH MARGINS & SPACING ───────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .show(|ui: &mut Ui| {
                            ui.h3("4. TextArea Outer Margins (.margin, .margin_bottom)")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .margin_bottom(10.0)
                                .show();

                            ui.text("TextArea supports .margin, .margin_top, and .margin_bottom to naturally separate the editor from surrounding form headers and action toolbars.")
                                .size(13.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .margin_bottom(12.0)
                                .show();

                            ui.text_area(&mut notes_content)
                                .fill_x()
                                .height(90.0)
                                .size(13.0)
                                .placeholder("Additional feedback or observations...")
                                .bg(Color::rgb(0.12, 0.14, 0.19))
                                .border((Color::rgb(0.22, 0.25, 0.35), 1.0))
                                .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.5))
                                .radius_all(6.0)
                                .padding_all(10.0)
                                .margin_bottom(10.0)
                                .show();

                            ui.row().show(|ui: &mut Ui| {
                                ui.text("Saved draft automatically")
                                    .size(12.0)
                                    .color(Color::rgb(0.40, 0.75, 0.50))
                                    .show();
                            });
                        });
                });
        })
        .run();
}
