use zenthra::prelude::*;

// ─────────────────────────────────────────────────────────────────────────────
// Colour palette helpers
// ─────────────────────────────────────────────────────────────────────────────

const BG: Color = Color { r: 0.06, g: 0.06, b: 0.08, a: 1.0 };
const PANEL: Color = Color { r: 0.10, g: 0.10, b: 0.14, a: 1.0 };
const ACCENT: Color = Color { r: 0.22, g: 0.42, b: 0.88, a: 1.0 };
const SUCCESS: Color = Color { r: 0.15, g: 0.72, b: 0.45, a: 1.0 };
const WARN: Color = Color { r: 0.92, g: 0.62, b: 0.18, a: 1.0 };
const DANGER: Color = Color { r: 0.85, g: 0.22, b: 0.28, a: 1.0 };
const MUTED: Color = Color { r: 0.55, g: 0.55, b: 0.62, a: 1.0 };

// A small set of pastel card colours indexed by position
fn card_color(i: usize) -> Color {
    let palette: &[(f32, f32, f32)] = &[
        (0.22, 0.38, 0.72),
        (0.15, 0.60, 0.52),
        (0.72, 0.30, 0.22),
        (0.55, 0.22, 0.72),
        (0.22, 0.62, 0.22),
        (0.72, 0.54, 0.12),
    ];
    let (r, g, b) = palette[i % palette.len()];
    Color::rgb(r, g, b)
}

// ─────────────────────────────────────────────────────────────────────────────
// Section label helper
// ─────────────────────────────────────────────────────────────────────────────

fn section_label(ui: &mut Ui, title: &str) {
    ui.spacing(6.0);
    ui.text(title)
        .size(13.0)
        .color(MUTED)
        .bold()
        .show();
    ui.spacing(4.0);
}

// ─────────────────────────────────────────────────────────────────────────────
// A small labelled box widget
// ─────────────────────────────────────────────────────────────────────────────

fn colored_box(ui: &mut Ui, label: &str, w: f32, h: f32, color: Color, id: impl std::hash::Hash + Copy) {
    ui.container()
        .id(id)
        .width(w)
        .height(h)
        .bg(color)
        .radius(8.0, 8.0, 8.0, 8.0)
        .align(Align::Center)
        .show(|ui| {
            ui.text(label)
                .size(13.0)
                .color(Color::WHITE)
                .show();
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// DEMO 1 – Simple Row of boxes
// ─────────────────────────────────────────────────────────────────────────────

fn demo_row(ui: &mut Ui) {
    section_label(ui, "ROW CONTAINER  ─  row() + gap");

    ui.container()
        .id("demo_row")
        .row()
        .gap(10.0)
        .padding(12.0, 12.0, 12.0, 12.0)
        .bg(PANEL)
        .radius(10.0, 10.0, 10.0, 10.0)
        .show(|ui| {
            for i in 0..6 {
                colored_box(ui, &format!("Box {}", i + 1), 80.0, 55.0, card_color(i), ("row_box", i));
            }
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// DEMO 2 – Simple Column of boxes
// ─────────────────────────────────────────────────────────────────────────────

fn demo_column(ui: &mut Ui) {
    section_label(ui, "COLUMN CONTAINER  ─  column() + gap");

    ui.container()
        .id("demo_col")
        .column()
        .gap(8.0)
        .padding(12.0, 12.0, 12.0, 12.0)
        .bg(PANEL)
        .radius(10.0, 10.0, 10.0, 10.0)
        .show(|ui| {
            for i in 0..4 {
                ui.container()
                    .id(("col_item", i))
                    .row()
                    .width(260.0)
                    .height(40.0)
                    .bg(card_color(i))
                    .radius(6.0, 6.0, 6.0, 6.0)
                    .padding(8.0, 8.0, 8.0, 8.0)
                    .valign(Align::Center)
                    .show(|ui| {
                        ui.text(&format!("Item {}", i + 1))
                            .size(14.0)
                            .color(Color::WHITE)
                            .show();
                    });
            }
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// DEMO 3 – Wrapping row
// ─────────────────────────────────────────────────────────────────────────────

fn demo_wrap(ui: &mut Ui) {
    section_label(ui, "WRAP CONTAINER  ─  row() + wrap(Wrap::Wrap)");

    ui.container()
        .id("demo_wrap")
        .row()
        .wrap(Wrap::Wrap)
        .gap(8.0)
        .padding(12.0, 12.0, 12.0, 12.0)
        .width(520.0)
        .bg(PANEL)
        .radius(10.0, 10.0, 10.0, 10.0)
        .show(|ui| {
            for i in 0..14 {
                colored_box(ui, &format!("#{}", i + 1), 70.0, 45.0, card_color(i), ("wrap_box", i));
            }
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// DEMO 4 – Alignment showcase (Center / SpaceBetween / Right)
// ─────────────────────────────────────────────────────────────────────────────

fn demo_align(ui: &mut Ui) {
    section_label(ui, "ALIGNMENT  ─  Center / SpaceBetween / Right");

    let alignments: &[(&str, Align)] = &[
        ("Center", Align::Center),
        ("SpaceBetween", Align::SpaceBetween),
        ("Right", Align::Right),
    ];

    ui.container()
        .id("demo_align_outer")
        .column()
        .gap(8.0)
        .padding(12.0, 12.0, 12.0, 12.0)
        .bg(PANEL)
        .radius(10.0, 10.0, 10.0, 10.0)
        .show(|ui| {
            for (label, align) in alignments.iter() {
                ui.container()
                    .id(("align_row", *label))
                    .row()
                    .width(500.0)
                    .height(44.0)
                    .halign(*align)
                    .valign(Align::Center)
                    .gap(8.0)
                    .bg(Color::rgb(0.14, 0.14, 0.20))
                    .radius(6.0, 6.0, 6.0, 6.0)
                    .padding(0.0, 8.0, 0.0, 8.0)
                    .show(|ui| {
                        for i in 0..3 {
                            colored_box(ui, label, 80.0, 30.0, card_color(i), ("align_box", *label, i));
                        }
                    });
            }
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// DEMO 5 – Nested containers (3 levels)
// ─────────────────────────────────────────────────────────────────────────────

fn demo_nested(ui: &mut Ui) {
    section_label(ui, "NESTED CONTAINERS  ─  3 levels deep");

    ui.container()
        .id("demo_nested_outer")
        .row()
        .gap(10.0)
        .padding(12.0, 12.0, 12.0, 12.0)
        .bg(PANEL)
        .radius(10.0, 10.0, 10.0, 10.0)
        .show(|ui| {
            // Level 1 – two column panels side by side
            for panel in 0..2usize {
                ui.container()
                    .id(("nested_panel", panel))
                    .column()
                    .gap(8.0)
                    .padding(10.0, 10.0, 10.0, 10.0)
                    .bg(Color::rgb(0.14, 0.14, 0.20))
                    .radius(8.0, 8.0, 8.0, 8.0)
                    .show(|ui| {
                        ui.text(&format!("Panel {}", panel + 1))
                            .size(12.0)
                            .color(MUTED)
                            .show();
                        ui.spacing(4.0);

                        // Level 2 – row of chips
                        for row in 0..3usize {
                            ui.container()
                                .id(("nested_row", panel, row))
                                .row()
                                .gap(6.0)
                                .show(|ui| {
                                    // Level 3 – individual badges
                                    for col in 0..3usize {
                                        ui.container()
                                            .id(("nested_badge", panel, row, col))
                                            .width(50.0)
                                            .height(28.0)
                                            .bg(card_color(panel * 3 + col))
                                            .radius(14.0, 14.0, 14.0, 14.0)
                                            .align(Align::Center)
                                            .show(|ui| {
                                                ui.text(&format!("{}.{}.{}", panel + 1, row + 1, col + 1))
                                                    .size(10.0)
                                                    .color(Color::WHITE)
                                                    .show();
                                            });
                                    }
                                });
                        }
                    });
            }
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// DEMO 6 – Scrollable vertical container
// ─────────────────────────────────────────────────────────────────────────────

fn demo_scroll(ui: &mut Ui) {
    section_label(ui, "SCROLLABLE CONTAINER  ─  scroll_y(true)");

    ui.container()
        .id("demo_scroll")
        .column()
        .width(340.0)
        .height(180.0)
        .max_height(180.0)      // safety: never grow beyond fixed height
        .scroll_y(true)
        .gap(8.0)
        .padding(10.0, 10.0, 10.0, 10.0)
        .bg(PANEL)
        .radius(10.0, 10.0, 10.0, 10.0)
        .show(|ui| {
            for i in 0..20usize {
                ui.container()
                    .id(("scroll_item", i))
                    .row()
                    .full_width()   // fill the padded available width, no overflow
                    .height(36.0)
                    .bg(card_color(i))
                    .radius(6.0, 6.0, 6.0, 6.0)
                    .padding(8.0, 8.0, 8.0, 8.0)
                    .valign(Align::Center)
                    .show(|ui| {
                        ui.text(&format!("Scrollable Row #{}", i + 1))
                            .size(13.0)
                            .color(Color::WHITE)
                            .show();
                    });
            }
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// DEMO 7 – Bordered / shadowed containers
// ─────────────────────────────────────────────────────────────────────────────

fn demo_styled_boxes(ui: &mut Ui) {
    section_label(ui, "STYLED BOXES  ─  border / shadow / opacity");

    ui.container()
        .id("demo_styled_outer")
        .row()
        .gap(16.0)
        .padding(12.0, 12.0, 12.0, 12.0)
        .bg(PANEL)
        .radius(10.0, 10.0, 10.0, 10.0)
        .show(|ui| {
            // Bordered
            ui.container()
                .id("styled_border")
                .width(110.0)
                .height(70.0)
                .bg(Color::rgb(0.10, 0.10, 0.14))
                .border(ACCENT, 2.0)
                .radius(10.0, 10.0, 10.0, 10.0)
                .align(Align::Center)
                .show(|ui| {
                    ui.text("Border")
                        .size(13.0)
                        .color(Color::WHITE)
                        .show();
                });

            // Shadowed
            ui.container()
                .id("styled_shadow")
                .width(110.0)
                .height(70.0)
                .bg(ACCENT)
                .shadow(Color::rgb(0.0, 0.0, 0.0), 0.0, 4.0, 16.0)
                .shadow_opacity(0.6)
                .radius(10.0, 10.0, 10.0, 10.0)
                .align(Align::Center)
                .show(|ui| {
                    ui.text("Shadow")
                        .size(13.0)
                        .color(Color::WHITE)
                        .show();
                });

            // Success badge
            ui.container()
                .id("styled_success")
                .width(110.0)
                .height(70.0)
                .bg(SUCCESS)
                .radius(10.0, 10.0, 10.0, 10.0)
                .align(Align::Center)
                .show(|ui| {
                    ui.text("Success")
                        .size(13.0)
                        .color(Color::WHITE)
                        .show();
                });

            // Warning
            ui.container()
                .id("styled_warn")
                .width(110.0)
                .height(70.0)
                .bg(WARN)
                .radius(10.0, 10.0, 10.0, 10.0)
                .align(Align::Center)
                .show(|ui| {
                    ui.text("Warning")
                        .size(13.0)
                        .color(Color::rgb(0.1, 0.06, 0.0))
                        .show();
                });

            // Danger
            ui.container()
                .id("styled_danger")
                .width(110.0)
                .height(70.0)
                .bg(DANGER)
                .radius(10.0, 10.0, 10.0, 10.0)
                .align(Align::Center)
                .show(|ui| {
                    ui.text("Danger")
                        .size(13.0)
                        .color(Color::WHITE)
                        .show();
                });
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// DEMO 8 – Mixed-size items inside a wrapping container
// ─────────────────────────────────────────────────────────────────────────────

fn demo_mixed_wrap(ui: &mut Ui) {
    section_label(ui, "MIXED-SIZE WRAP  ─  varying widths in a wrapping row");

    ui.container()
        .id("demo_mixed_wrap")
        .row()
        .wrap(Wrap::Wrap)
        .gap(8.0)
        .padding(12.0, 12.0, 12.0, 12.0)
        .width(520.0)
        .bg(PANEL)
        .radius(10.0, 10.0, 10.0, 10.0)
        .show(|ui| {
            let widths = [60.0_f32, 120.0, 80.0, 100.0, 60.0, 140.0, 80.0, 60.0, 90.0, 120.0];
            for (i, &w) in widths.iter().enumerate() {
                ui.container()
                    .id(("mixed", i))
                    .width(w)
                    .height(40.0)
                    .bg(card_color(i))
                    .radius(6.0, 6.0, 6.0, 6.0)
                    .align(Align::Center)
                    .show(|ui| {
                        ui.text(&format!("{}px", w as u32))
                            .size(12.0)
                            .color(Color::WHITE)
                            .show();
                    });
            }
        });
}

// ─────────────────────────────────────────────────────────────────────────────
// Main
// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    App::new()
        .title("Zenthra – Wrapper System Test")
        .size(1100, 820)
        .with_ui(|ui| {
            // ── Root full-screen container ─────────────────────────────────
            ui.container()
                .id("root")
                .fill()
                .column()
                .scroll_y(true)
                .bg(BG)
                .padding(20.0, 20.0, 20.0, 20.0)
                .gap(0.0)
                .show(|ui| {
                    // Title
                    ui.h2("Wrapper System Test")
                        .color(Color::WHITE)
                        .show();
                    ui.text("Testing containers, nesting, wrapping, scrolling and alignment.")
                        .size(14.0)
                        .color(MUTED)
                        .show();
                    ui.spacing(20.0);

                    // ── Two-column layout for the demos ───────────────────
                    ui.container()
                        .id("two_col")
                        .row()
                        .gap(24.0)
                        .show(|ui| {
                            // LEFT column
                            ui.container()
                                .id("left_col")
                                .column()
                                .gap(16.0)
                                .show(|ui| {
                                    demo_row(ui);
                                    demo_wrap(ui);
                                    demo_styled_boxes(ui);
                                    demo_mixed_wrap(ui);
                                });

                            // RIGHT column
                            ui.container()
                                .id("right_col")
                                .column()
                                .gap(16.0)
                                .show(|ui| {
                                    demo_column(ui);
                                    demo_align(ui);
                                    demo_nested(ui);
                                    demo_scroll(ui);
                                });
                        });
                });
        })
        .run();
}
