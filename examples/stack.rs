use zenthra::prelude::*;

fn main() {
    let mut selected_halign = Align::Center;
    let mut selected_valign = Align::Center;
    let mut clip_enabled = true;
    let mut notification_count = 3;
    let mut fab_clicked_count = 0;
    let mut status_msg = String::from("Ready. Interact with the layered stack widgets.");

    App::new()
        .title("Zenthra - Stack Layout & Multi-Layer Z-Index Showcase")
        .size(1120, 920)
        .with_ui(move |ui: &mut Ui| {
            let bg_canvas = Color::rgb(0.06, 0.07, 0.09);
            let bg_card = Color::rgb(0.10, 0.12, 0.17);
            let border_col = Color::rgb(0.18, 0.22, 0.30);
            let primary = Color::rgb(0.38, 0.44, 0.98); // Indigo
            let emerald = Color::rgb(0.12, 0.80, 0.54);

            ui.container()
                .fill()
                .bg(bg_canvas)
                .padding_all(32.0)
                .gap(24.0)
                .scroll_y(true)
                .column()
                .show(|ui: &mut Ui| {

                    // ── 1. HEADER BANNER ─────────────────────────────────────────
                    ui.card()
                        .fill_x()
                        .bg(bg_card)
                        .border((border_col, 1.0))
                        .radius_all(12.0)
                        .padding_all(20.0)
                        .gap(6.0)
                        .show(|ui: &mut Ui| {
                            ui.h2("Stack Layout Architecture (`ui.stack()`)")
                                .color(Color::WHITE)
                                .show();

                            ui.text("Multi-layer Z-axis layout container where children occupy overlapping spatial coordinates. Features fine-grained cross-axis alignment, clipping, and BoxStyle parity.")
                                .size(13.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });

                    // ── 2. RECIPES GRID ──────────────────────────────────────────
                    ui.container()
                        .fill_x()
                        .row()
                        .gap(20.0)
                        .wrap(Wrap::Wrap)
                        .show(|ui: &mut Ui| {

                            // ── Recipe 1: Avatar with Corner Status Indicator ────
                            ui.card()
                                .width(330.0)
                                .bg(bg_card)
                                .border((border_col, 1.0))
                                .radius_all(12.0)
                                .padding_all(18.0)
                                .gap(14.0)
                                .show(|ui: &mut Ui| {
                                    ui.h3("1. Avatar & Status Badge")
                                        .color(Color::WHITE)
                                        .show();

                                    ui.text("Overlaying a notification badge or online dot onto an avatar base.")
                                        .size(12.0)
                                        .color(Color::rgb(0.60, 0.65, 0.75))
                                        .show();

                                    // Stack: 64x64 bounds with online dot in bottom right
                                    ui.container()
                                        .fill_x()
                                        .align(Align::Center)
                                        .show(|ui: &mut Ui| {
                                            ui.stack()
                                                .width(72.0)
                                                .height(72.0)
                                                .show(|ui: &mut Ui| {
                                                    // Layer 0: Circular Avatar Base
                                                    ui.container()
                                                        .width(68.0)
                                                        .height(68.0)
                                                        .radius_all(34.0)
                                                        .bg(primary)
                                                        .border((Color::rgb(0.3, 0.35, 0.6), 2.0))
                                                        .align(Align::Center)
                                                        .valign(Align::Center)
                                                        .show(|ui: &mut Ui| {
                                                            ui.text("JD")
                                                                .size(24.0)
                                                                .font_weight(FontWeight::ExtraBold)
                                                                .color(Color::WHITE)
                                                                .show();
                                                        });

                                                    // Layer 1: Online Status Pill Indicator pinned to bottom-right
                                                    ui.container()
                                                        .margin_left(48.0)
                                                        .margin_top(48.0)
                                                        .width(18.0)
                                                        .height(18.0)
                                                        .radius_all(9.0)
                                                        .bg(emerald)
                                                        .border((Color::rgb(0.10, 0.12, 0.17), 2.5))
                                                        .show(|_| {});
                                                });
                                        });

                                    ui.container()
                                        .fill_x()
                                        .column()
                                        .align(Align::Center)
                                        .show(|ui: &mut Ui| {
                                            ui.text("John Doe (Active)")
                                                .size(14.0)
                                                .font_weight(FontWeight::Bold)
                                                .color(Color::WHITE)
                                                .show();
                                            ui.text("Senior Graphics Engineer")
                                                .size(12.0)
                                                .color(Color::rgb(0.55, 0.60, 0.70))
                                                .show();
                                        });
                                });

                            // ── Recipe 2: Card with Pinned "PRO" Tag & Shimmer ───
                            ui.card()
                                .width(330.0)
                                .bg(bg_card)
                                .border((border_col, 1.0))
                                .radius_all(12.0)
                                .padding_all(18.0)
                                .gap(14.0)
                                .show(|ui: &mut Ui| {
                                    ui.h3("2. Corner Tag & Badge")
                                        .color(Color::WHITE)
                                        .show();

                                    ui.text("Pinned absolute badges over complex content cards.")
                                        .size(12.0)
                                        .color(Color::rgb(0.60, 0.65, 0.75))
                                        .show();

                                    ui.stack()
                                        .fill_x()
                                        .height(140.0)
                                        .radius_all(10.0)
                                        .clip(true)
                                        .show(|ui: &mut Ui| {
                                            // Layer 0: Background card surface
                                            ui.container()
                                                .fill()
                                                .bg(Color::rgb(0.14, 0.16, 0.22))
                                                .border((border_col, 1.0))
                                                .radius_all(10.0)
                                                .padding_all(14.0)
                                                .column()
                                                .gap(6.0)
                                                .show(|ui: &mut Ui| {
                                                    ui.text("Enterprise Cloud Tier")
                                                        .size(15.0)
                                                        .font_weight(FontWeight::Bold)
                                                        .color(Color::WHITE)
                                                        .show();
                                                    ui.text("Unlimited compute nodes with dedicated 10Gbps mesh.")
                                                        .size(12.0)
                                                        .color(Color::rgb(0.60, 0.65, 0.75))
                                                        .show();
                                                    ui.spacing(4.0);
                                                    ui.text("$99 / month")
                                                        .size(16.0)
                                                        .font_weight(FontWeight::ExtraBold)
                                                        .color(Color::rgb(0.96, 0.65, 0.14))
                                                        .show();
                                                });

                                            // Layer 1: "PRO" Ribbon Badge pinned to Top-Right
                                            ui.container()
                                                .margin_left(220.0)
                                                .margin_top(10.0)
                                                .padding(4.0, 10.0, 4.0, 10.0)
                                                .radius_all(6.0)
                                                .bg(Color::rgb(0.96, 0.65, 0.14))
                                                .show(|ui: &mut Ui| {
                                                    ui.text("PRO")
                                                        .size(11.0)
                                                        .font_weight(FontWeight::ExtraBold)
                                                        .color(Color::BLACK)
                                                        .show();
                                                });
                                        });

                                    if ui.button("Select Plan").fill_x().padding(8.0, 12.0, 8.0, 12.0).radius_all(8.0).bg(primary).color(Color::WHITE).show().clicked {
                                        status_msg = "Enterprise tier selected".to_string();
                                    }
                                });

                            // ── Recipe 3: Floating Action Button (FAB) in Corner ─
                            ui.card()
                                .width(330.0)
                                .bg(bg_card)
                                .border((border_col, 1.0))
                                .radius_all(12.0)
                                .padding_all(18.0)
                                .gap(14.0)
                                .show(|ui: &mut Ui| {
                                    ui.h3("3. Floating Action Button")
                                        .color(Color::WHITE)
                                        .show();

                                    ui.text("Overlaying a circular action trigger at the bottom-right.")
                                        .size(12.0)
                                        .color(Color::rgb(0.60, 0.65, 0.75))
                                        .show();

                                    ui.stack()
                                        .fill_x()
                                        .height(140.0)
                                        .radius_all(10.0)
                                        .clip(true)
                                        .show(|ui: &mut Ui| {
                                            // Layer 0: Content Area
                                            ui.container()
                                                .fill()
                                                .bg(Color::rgb(0.12, 0.14, 0.19))
                                                .border((border_col, 1.0))
                                                .radius_all(10.0)
                                                .padding_all(12.0)
                                                .column()
                                                .gap(4.0)
                                                .show(|ui: &mut Ui| {
                                                    ui.text("Live Task Queue").size(14.0).font_weight(FontWeight::Bold).color(Color::WHITE).show();
                                                    ui.text(&format!("{} notifications pending", notification_count)).size(12.0).color(Color::rgb(0.6, 0.65, 0.75)).show();
                                                    ui.text(&format!("FAB clicked {} times", fab_clicked_count)).size(11.0).color(Color::rgb(0.4, 0.8, 1.0)).show();
                                                });

                                            // Layer 1: Floating Action Button (FAB)
                                            ui.container()
                                                .margin_left(235.0)
                                                .margin_top(75.0)
                                                .show(|ui: &mut Ui| {
                                                    if ui.button("+")
                                                        .width(44.0)
                                                        .height(44.0)
                                                        .radius_all(22.0)
                                                        .bg(primary)
                                                        .color(Color::WHITE)
                                                        .shadow(Color::rgba(0.0, 0.0, 0.0, 0.5), 0.0, 4.0, 10.0)
                                                        .hover(Style::bg(Color::rgb(0.48, 0.54, 1.0)))
                                                        .show()
                                                        .clicked
                                                    {
                                                        fab_clicked_count += 1;
                                                        notification_count += 1;
                                                        status_msg = format!("FAB clicked: new task #{}", fab_clicked_count);
                                                    }
                                                });
                                        });

                                    if ui.button("Clear Tasks").fill_x().padding(6.0, 10.0, 6.0, 10.0).radius_all(6.0).bg(Color::TRANSPARENT).color(Color::rgb(0.7, 0.75, 0.85)).show().clicked {
                                        notification_count = 0;
                                        status_msg = "Tasks cleared".to_string();
                                    }
                                });
                        });

                    // ── 3. HERO BANNER WITH GRADIENT & LAYERED CONTROLS ──────────
                    ui.card()
                        .fill_x()
                        .bg(bg_card)
                        .border((border_col, 1.0))
                        .radius_all(12.0)
                        .padding_all(20.0)
                        .gap(16.0)
                        .show(|ui: &mut Ui| {
                            ui.h3("4. Hero Banner with Gradient & Dynamic Alignment")
                                .color(Color::WHITE)
                                .show();

                            // Controls for dynamic stack alignment
                            ui.container()
                                .row()
                                .gap(16.0)
                                .wrap(Wrap::Wrap)
                                .valign(Align::Center)
                                .show(|ui: &mut Ui| {
                                    ui.text("Horizontal Alignment:").size(12.0).color(Color::rgb(0.7, 0.75, 0.85)).show();
                                    if ui.button("Left").padding(6.0, 10.0, 6.0, 10.0).radius_all(6.0).bg(if selected_halign == Align::Left { primary } else { Color::TRANSPARENT }).color(Color::WHITE).show().clicked {
                                        selected_halign = Align::Left;
                                    }
                                    if ui.button("Center").padding(6.0, 10.0, 6.0, 10.0).radius_all(6.0).bg(if selected_halign == Align::Center { primary } else { Color::TRANSPARENT }).color(Color::WHITE).show().clicked {
                                        selected_halign = Align::Center;
                                    }
                                    if ui.button("Right").padding(6.0, 10.0, 6.0, 10.0).radius_all(6.0).bg(if selected_halign == Align::Right { primary } else { Color::TRANSPARENT }).color(Color::WHITE).show().clicked {
                                        selected_halign = Align::Right;
                                    }

                                    ui.spacing(12.0);
                                    ui.text("Vertical Alignment:").size(12.0).color(Color::rgb(0.7, 0.75, 0.85)).show();
                                    if ui.button("Top").padding(6.0, 10.0, 6.0, 10.0).radius_all(6.0).bg(if selected_valign == Align::Top { primary } else { Color::TRANSPARENT }).color(Color::WHITE).show().clicked {
                                        selected_valign = Align::Top;
                                    }
                                    if ui.button("VCenter").padding(6.0, 10.0, 6.0, 10.0).radius_all(6.0).bg(if selected_valign == Align::Center { primary } else { Color::TRANSPARENT }).color(Color::WHITE).show().clicked {
                                        selected_valign = Align::Center;
                                    }
                                    if ui.button("Bottom").padding(6.0, 10.0, 6.0, 10.0).radius_all(6.0).bg(if selected_valign == Align::Bottom { primary } else { Color::TRANSPARENT }).color(Color::WHITE).show().clicked {
                                        selected_valign = Align::Bottom;
                                    }

                                    ui.spacing(12.0);
                                    ui.toggle(&mut clip_enabled, "Clip Overflow").on_color(primary).show();
                                });

                            // The Stack Hero Box
                            ui.stack()
                                .fill_x()
                                .height(200.0)
                                .radius_all(14.0)
                                .clip(clip_enabled)
                                .align(selected_halign, selected_valign)
                                .border((border_col, 1.5))
                                .show(|ui: &mut Ui| {
                                    // Layer 0: Multi-Stop Linear Gradient Surface
                                    ui.container()
                                        .fill()
                                        .radius_all(14.0)
                                        .bg(Gradient::linear(
                                            Direction::Angle(135.0),
                                            [
                                                Color::rgb(0.20, 0.10, 0.45),
                                                Color::rgb(0.08, 0.18, 0.35),
                                                Color::rgb(0.04, 0.08, 0.18),
                                            ],
                                        ))
                                        .show(|_| {});

                                    // Layer 1: Translucent Grid Background Pattern
                                    ui.container()
                                        .margin_left(20.0)
                                        .margin_top(20.0)
                                        .width(300.0)
                                        .height(160.0)
                                        .radius_all(10.0)
                                        .border((Color::rgba(1.0, 1.0, 1.0, 0.12), 1.0))
                                        .bg(Color::rgba(1.0, 1.0, 1.0, 0.04))
                                        .show(|_| {});

                                    // Layer 2: Foreground Typography & Call to Action
                                    ui.container()
                                        .padding_all(20.0)
                                        .column()
                                        .gap(8.0)
                                        .show(|ui: &mut Ui| {
                                            ui.text("Next-Gen Immediate-Mode Graphics")
                                                .size(20.0)
                                                .font_weight(FontWeight::ExtraBold)
                                                .color(Color::WHITE)
                                                .show();

                                            ui.text("Composing layers with zero layout jitter and hardware acceleration.")
                                                .size(13.0)
                                                .color(Color::rgb(0.75, 0.82, 0.95))
                                                .show();

                                            if ui.button("Launch Pipeline")
                                                .padding(8.0, 16.0, 8.0, 16.0)
                                                .radius_all(8.0)
                                                .bg(primary)
                                                .color(Color::WHITE)
                                                .hover(Style::bg(Color::rgb(0.48, 0.54, 1.0)))
                                                .show()
                                                .clicked
                                            {
                                                status_msg = "Pipeline launched from Stack layer".to_string();
                                            }
                                        });
                                });
                        });

                    // ── 4. STATUS BAR ────────────────────────────────────────────
                    ui.container()
                        .fill_x()
                        .bg(bg_card)
                        .border((border_col, 1.0))
                        .radius_all(8.0)
                        .padding_all(12.0)
                        .row()
                        .valign(Align::Center)
                        .show(|ui: &mut Ui| {
                            ui.text(&format!("Event Log: {}", status_msg))
                                .size(12.0)
                                .color(Color::rgb(0.60, 0.65, 0.75))
                                .show();
                        });
                });
        })
        .run();
}
