use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Text API Showcase")
        .size(1050, 800)
        .with_ui(|ui: &mut Ui| {
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
                            ui.h1("Typography & Text API")
                                .color(Color::WHITE)
                                .show();

                            ui.text("Demonstrating font sizes, weights, styles, word-wrapping, badges, and selectable text.")
                                .size(14.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });

                    // ── SECTION 1: HEADING HIERARCHY ──────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("1. Heading Hierarchy")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.h1("H1 Header (40px Bold)")
                                .color(Color::WHITE)
                                .show();

                            ui.h2("H2 Header (32px Bold)")
                                .color(Color::rgb(0.90, 0.92, 0.98))
                                .show();

                            ui.h3("H3 Header (24px Bold)")
                                .color(Color::rgb(0.80, 0.84, 0.92))
                                .show();

                            ui.h4("H4 Header (20px Bold)")
                                .color(Color::rgb(0.70, 0.75, 0.85))
                                .show();

                            ui.text("Body regular text (16px Normal)")
                                .size(16.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });

                    // ── SECTION 2: FONT WEIGHTS & STYLES ──────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("2. Weights, Monospace & Italics")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.container()
                                .row()
                                .gap(24.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    ui.text("Regular Text")
                                        .size(15.0)
                                        .color(Color::WHITE)
                                        .show();

                                    ui.text("Bold Text")
                                        .size(15.0)
                                        .font_weight(FontWeight::Bold)
                                        .color(Color::WHITE)
                                        .show();

                                    ui.text("Italic Text")
                                        .size(15.0)
                                        .decoration(TextDecoration::Italic)
                                        .color(Color::rgb(0.80, 0.85, 0.95))
                                        .show();

                                    ui.text("Underline Text")
                                        .size(15.0)
                                        .decoration(TextDecoration::Underline)
                                        .color(Color::rgb(0.40, 0.85, 0.60))
                                        .show();

                                    ui.text("Strikethrough")
                                        .size(15.0)
                                        .decoration(TextDecoration::Strikethrough)
                                        .color(Color::rgb(0.65, 0.70, 0.80))
                                        .show();

                                    ui.text("Monospace Code")
                                        .size(15.0)
                                        .font_family("monospace")
                                        .color(Color::rgb(0.40, 0.80, 0.95))
                                        .show();
                                });
                        });

                    // ── SECTION 3: DECORATED BADGES & TAGS (BoxStyle on Text) ─
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("3. Badges, Tags & Pill Labels (BoxStyle on Text)")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.container()
                                .row()
                                .gap(12.0)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    // Primary Badge
                                    ui.text("PRIMARY")
                                        .size(11.0)
                                        .font_weight(FontWeight::Bold)
                                        .color(Color::WHITE)
                                        .bg(Color::rgb(0.20, 0.42, 0.88))
                                        .border((Color::rgb(0.35, 0.60, 1.0), 1.0))
                                        .radius_all(4.0)
                                        .padding(3.0, 8.0, 3.0, 8.0)
                                        .show();

                                    // Success Badge
                                    ui.text("ACTIVE")
                                        .size(11.0)
                                        .font_weight(FontWeight::Bold)
                                        .color(Color::rgb(0.20, 0.85, 0.45))
                                        .bg(Color::rgb(0.10, 0.25, 0.16))
                                        .border((Color::rgb(0.20, 0.60, 0.35), 1.0))
                                        .radius_all(4.0)
                                        .padding(3.0, 8.0, 3.0, 8.0)
                                        .show();

                                    // Warning Pill Tag
                                    ui.text("PENDING")
                                        .size(11.0)
                                        .font_weight(FontWeight::Bold)
                                        .color(Color::rgb(1.0, 0.75, 0.20))
                                        .bg(Color::rgb(0.25, 0.18, 0.08))
                                        .border((Color::rgb(0.65, 0.45, 0.15), 1.0))
                                        .radius_all(10.0) // pill shape
                                        .padding(3.0, 10.0, 3.0, 10.0)
                                        .show();

                                    // Error Badge
                                    ui.text("CRITICAL")
                                        .size(11.0)
                                        .font_weight(FontWeight::Bold)
                                        .color(Color::rgb(1.0, 0.35, 0.35))
                                        .bg(Color::rgb(0.26, 0.10, 0.10))
                                        .border((Color::rgb(0.65, 0.25, 0.25), 1.0))
                                        .radius_all(4.0)
                                        .padding(3.0, 8.0, 3.0, 8.0)
                                        .show();
                                });
                        });

                    // ── SECTION 4: TEXT WRAPPING & ELLIPSIS ────────────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("4. Text Wrapping & Ellipsis")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.container()
                                .row()
                                .gap(20.0)
                                .show(|ui: &mut Ui| {
                                    // Multiline Wrapped Text Box
                                    ui.container()
                                        .width(360.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.19))
                                        .border((Color::rgb(0.20, 0.24, 0.32), 1.0))
                                        .radius_all(8.0)
                                        .padding_all(14.0)
                                        .column()
                                        .gap(8.0)
                                        .show(|ui: &mut Ui| {
                                            ui.text("Multi-line Auto Wrap")
                                                .size(12.0)
                                                .font_weight(FontWeight::Bold)
                                                .color(Color::rgb(0.45, 0.65, 1.0))
                                                .show();

                                            ui.text("Zenthra computes typography layout with sub-pixel precision. When text exceeds available container width, it seamlessly breaks across word boundaries while maintaining consistent line spacing.")
                                                .size(14.0)
                                                .color(Color::rgb(0.80, 0.85, 0.92))
                                                .decoration(TextDecoration::LineHeight(20.0))
                                                .wrap(TextWrap::Word)
                                                .selectable(true)
                                                .show();
                                        });

                                    // Ellipsis Truncation Box
                                    ui.container()
                                        .width(280.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.19))
                                        .border((Color::rgb(0.20, 0.24, 0.32), 1.0))
                                        .radius_all(8.0)
                                        .padding_all(14.0)
                                        .column()
                                        .gap(8.0)
                                        .show(|ui: &mut Ui| {
                                            ui.text("Single-Line Ellipsis")
                                                .size(12.0)
                                                .font_weight(FontWeight::Bold)
                                                .color(Color::rgb(0.45, 0.65, 1.0))
                                                .show();

                                            ui.text("This very long single line of text will be neatly truncated with an ellipsis rather than overflowing its parent container boundary.")
                                                .size(14.0)
                                                .color(Color::rgb(0.80, 0.85, 0.92))
                                                .width(250.0)
                                                .wrap(TextWrap::None)
                                                .ellipsis(true)
                                                .show();
                                        });
                                });
                        });

                    // ── SECTION 5: INTERACTIVE & SELECTABLE TEXT ───────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .gap(12.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("5. Selectable Text")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .show();

                            ui.text("You can click and drag across this text to select it. Try selecting this sentence!")
                                .size(15.0)
                                .color(Color::rgb(0.90, 0.92, 0.98))
                                .selectable(true)
                                .cursor(CursorIcon::Text)
                                .show();
                        });

                    // ── SECTION 6: TYPOGRAPHY MARGINS & SPACING ───────────────
                    ui.container()
                        .fill_x()
                        .bg(Color::rgb(0.09, 0.11, 0.15))
                        .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                        .radius_all(10.0)
                        .padding_all(18.0)
                        .column()
                        .show(|ui: &mut Ui| {
                            ui.h3("6. Outer Margins on Text Elements")
                                .color(Color::rgb(0.35, 0.65, 1.0))
                                .margin_bottom(12.0)
                                .show();

                            ui.text("Text elements support .margin, .margin_top, and .margin_bottom, allowing typographic vertical rhythm without container wrapper nesting.")
                                .size(14.0)
                                .color(Color::rgb(0.70, 0.75, 0.85))
                                .margin_bottom(16.0)
                                .show();

                            ui.row().show(|ui: &mut Ui| {
                                ui.text("TAG 1")
                                    .size(11.0)
                                    .font_weight(FontWeight::Bold)
                                    .color(Color::WHITE)
                                    .bg(Color::rgb(0.20, 0.40, 0.80))
                                    .radius_all(4.0)
                                    .padding(3.0, 8.0, 3.0, 8.0)
                                    .margin_right(12.0)
                                    .show();

                                ui.text("TAG 2 (Margin Separated)")
                                    .size(11.0)
                                    .font_weight(FontWeight::Bold)
                                    .color(Color::WHITE)
                                    .bg(Color::rgb(0.30, 0.25, 0.60))
                                    .radius_all(4.0)
                                    .padding(3.0, 8.0, 3.0, 8.0)
                                    .margin_right(12.0)
                                    .show();

                                ui.text("TAG 3")
                                    .size(11.0)
                                    .font_weight(FontWeight::Bold)
                                    .color(Color::WHITE)
                                    .bg(Color::rgb(0.15, 0.50, 0.35))
                                    .radius_all(4.0)
                                    .padding(3.0, 8.0, 3.0, 8.0)
                                    .show();
                            });
                        });
                });
        })
        .run();
}
