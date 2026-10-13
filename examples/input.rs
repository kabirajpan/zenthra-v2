use zenthra::prelude::*;

fn main() {
    let mut username = String::from("Alex Mercer");
    let mut email = String::new();
    let mut search_query = String::new();
    let mut code_snippet = String::from("fn main() {\n    println!(\"Zenthra\");\n}");
    let mut inline_rename = String::from("Project_Alpha_v2");
    let mut submitted_msg = String::new();

    App::new()
        .title("Zenthra - Input API Showcase")
        .size(1050, 850)
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
                            ui.h1("Text Input API Showcase")
                                .color(Color::WHITE)
                                .show();

                            ui.text("Demonstrating unified BoxStyle borders, focus rings, typography, placeholders, and event handling.")
                                .size(14.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });

                    // ── SECTION 1: FORM INPUTS & FOCUS BORDERS ────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(14.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("1. Standard Form Inputs & Focus Rings")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.row().gap(20.0).show(|ui: &mut Ui| {
                                // Field A: Username
                                ui.column().gap(6.0).show(|ui: &mut Ui| {
                                    ui.text("Username (Default Accent Focus)")
                                        .size(13.0)
                                        .color(Color::rgb(0.70, 0.75, 0.85))
                                        .show();

                                    ui.input(&mut username)
                                        .width(300.0)
                                        .size(14.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.19))
                                        .border((Color::rgb(0.22, 0.25, 0.35), 1.0))
                                        .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.5))
                                        .radius_all(6.0)
                                        .show();
                                });

                                // Field B: Email with custom emerald focus
                                ui.column().gap(6.0).show(|ui: &mut Ui| {
                                    ui.text("Email (Emerald Focus + Custom Cursor)")
                                        .size(13.0)
                                        .color(Color::rgb(0.70, 0.75, 0.85))
                                        .show();

                                    ui.input(&mut email)
                                        .width(320.0)
                                        .size(14.0)
                                        .placeholder("name@company.com")
                                        .placeholder_color(Color::rgb(0.40, 0.44, 0.52))
                                        .bg(Color::rgb(0.12, 0.14, 0.19))
                                        .border((Color::rgb(0.22, 0.25, 0.35), 1.0))
                                        .focus_border((Color::rgb(0.10, 0.80, 0.50), 1.5))
                                        .cursor_color(Color::rgb(0.10, 0.80, 0.50))
                                        .radius_all(6.0)
                                        .show();
                                });
                            });
                        });

                    // ── SECTION 2: PILL SEARCH BAR & SHADOWS ───────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(25.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("2. Pill Search Field with Drop Shadow & Submit")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            let res = ui.input(&mut search_query)
                                .fill_x()
                                .height(40.0)
                                .size(15.0)
                                .placeholder("Type search query and press Enter...")
                                .placeholder_color(Color::rgb(0.42, 0.46, 0.56))
                                .bg(Color::rgb(0.13, 0.15, 0.21))
                                .border((Color::rgb(0.22, 0.26, 0.36), 1.0))
                                .focus_border((Color::rgb(0.40, 0.70, 1.0), 1.5))
                                .radius_all(20.0)
                                .padding(10.0, 20.0, 10.0, 20.0)
                                .shadow(Color::rgba(0.0, 0.0, 0.0, 0.35), 0.0, 4.0, 10.0)
                                .show();

                            if res.submitted {
                                submitted_msg = format!("Submitted search: \"{}\"", search_query);
                            }

                            if !submitted_msg.is_empty() {
                                ui.text(&submitted_msg)
                                    .size(13.0)
                                    .color(Color::rgb(0.30, 0.85, 0.55))
                                    .show();
                            }
                        });

                    // ── SECTION 3: INLINE SEAMLESS RENAME ─────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("3. Seamless Inline Rename (Transparent Surface)")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.row().gap(12.0).valign(Align::Center).show(|ui: &mut Ui| {
                                ui.text("File:")
                                    .size(14.0)
                                    .color(Color::rgb(0.60, 0.65, 0.75))
                                    .show();

                                ui.input(&mut inline_rename)
                                    .width(260.0)
                                    .size(14.0)
                                    .bg(None)
                                    .border(None)
                                    .focus_border((Color::rgb(0.35, 0.65, 1.0), 1.0))
                                    .radius_all(4.0)
                                    .padding(4.0, 6.0, 4.0, 6.0)
                                    .show();

                                ui.text("(Click text to edit inline with subtle border)")
                                    .size(12.0)
                                    .color(Color::rgb(0.45, 0.50, 0.60))
                                    .show();
                            });
                        });

                    // ── SECTION 4: MONOSPACE & TYPOGRAPHY ─────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("4. Monospace Typography & Selection Highlight")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.input(&mut code_snippet)
                                .fill_x()
                                .font_family("monospace")
                                .size(13.0)
                                .color(Color::rgb(0.90, 0.92, 0.98))
                                .bg(Color::rgb(0.08, 0.09, 0.12))
                                .border((Color::rgb(0.20, 0.24, 0.32), 1.0))
                                .focus_border((Color::rgb(0.70, 0.45, 1.0), 1.5))
                                .selection_color(Color::rgba(0.55, 0.30, 0.90, 0.40))
                                .radius_all(6.0)
                                .padding(8.0, 12.0, 8.0, 12.0)
                                .show();
                        });

                    // ── SECTION 5: INPUT MARGINS & FORM SPACING ───────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("5. Outer Margins & Independent Spacing")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.text("Inputs support .margin, .margin_x, .margin_top/bottom to establish outer spacing independently.")
                                .size(13.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();

                            ui.row().show(|ui: &mut Ui| {
                                ui.input(&mut username)
                                    .width(240.0)
                                    .size(14.0)
                                    .bg(Color::rgb(0.12, 0.14, 0.19))
                                    .border((Color::rgb(0.22, 0.25, 0.35), 1.0))
                                    .radius_all(6.0)
                                    .margin_right(16.0)
                                    .show();

                                ui.input(&mut search_query)
                                    .width(240.0)
                                    .size(14.0)
                                    .placeholder("Quick filter...")
                                    .bg(Color::rgb(0.12, 0.14, 0.19))
                                    .border((Color::rgb(0.22, 0.25, 0.35), 1.0))
                                    .radius_all(6.0)
                                    .margin_left(8.0)
                                    .show();
                            });
                        });
                });
        })
        .run();
}
