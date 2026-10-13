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
                                .font_size(14.0)
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
                                .font_size(16.0)
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
                                        .font_size(15.0)
                                        .color(Color::WHITE)
                                        .show();

                                    ui.text("Bold Text")
                                        .font_size(15.0)
                                        .bold()
                                        .color(Color::WHITE)
                                        .show();

                                    ui.text("Italic Text")
                                        .font_size(15.0)
                                        .italic()
                                        .color(Color::rgb(0.80, 0.85, 0.95))
                                        .show();

                                    ui.text("Underline Text")
                                        .font_size(15.0)
                                        .underline(true)
                                        .color(Color::rgb(0.40, 0.85, 0.60))
                                        .show();

                                    ui.text("Strikethrough")
                                        .font_size(15.0)
                                        .strikethrough(true)
                                        .color(Color::rgb(0.65, 0.70, 0.80))
                                        .show();

                                    ui.text("Monospace Code")
                                        .font_size(15.0)
                                        .monospace()
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
                                        .font_size(11.0)
                                        .bold()
                                        .color(Color::WHITE)
                                        .bg(Color::rgb(0.20, 0.42, 0.88))
                                        .border((Color::rgb(0.35, 0.60, 1.0), 1.0))
                                        .radius_all(4.0)
                                        .padding(3.0, 8.0, 3.0, 8.0)
                                        .show();

                                    // Success Badge
                                    ui.text("ACTIVE")
                                        .font_size(11.0)
                                        .bold()
                                        .color(Color::rgb(0.20, 0.85, 0.45))
                                        .bg(Color::rgb(0.10, 0.25, 0.16))
                                        .border((Color::rgb(0.20, 0.60, 0.35), 1.0))
                                        .radius_all(4.0)
                                        .padding(3.0, 8.0, 3.0, 8.0)
                                        .show();

                                    // Warning Pill Tag
                                    ui.text("PENDING")
                                        .font_size(11.0)
                                        .bold()
                                        .color(Color::rgb(1.0, 0.75, 0.20))
                                        .bg(Color::rgb(0.25, 0.18, 0.08))
                                        .border((Color::rgb(0.65, 0.45, 0.15), 1.0))
                                        .radius_all(10.0) // pill shape
                                        .padding(3.0, 10.0, 3.0, 10.0)
                                        .show();

                                    // Error Badge
                                    ui.text("CRITICAL")
                                        .font_size(11.0)
                                        .bold()
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
                                                .font_size(12.0)
                                                .bold()
                                                .color(Color::rgb(0.45, 0.65, 1.0))
                                                .show();

                                            ui.text("Zenthra computes typography layout with sub-pixel precision. When text exceeds available container width, it seamlessly breaks across word boundaries while maintaining consistent line spacing.")
                                                .font_size(14.0)
                                                .color(Color::rgb(0.80, 0.85, 0.92))
                                                .line_height(20.0)
                                                .auto_wrap()
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
                                                .font_size(12.0)
                                                .bold()
                                                .color(Color::rgb(0.45, 0.65, 1.0))
                                                .show();

                                            ui.text("This very long single line of text will be neatly truncated with an ellipsis rather than overflowing its parent container boundary.")
                                                .font_size(14.0)
                                                .color(Color::rgb(0.80, 0.85, 0.92))
                                                .width(250.0)
                                                .no_wrap()
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
                                .font_size(15.0)
                                .color(Color::rgb(0.90, 0.92, 0.98))
                                .selectable(true)
                                .cursor(CursorIcon::Text)
                                .show();
                        });
                });
        })
        .run();
}
