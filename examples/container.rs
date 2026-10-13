use zenthra::prelude::*;

fn main() {
    App::new()
        .title("Zenthra - Container Dashboard Showcase")
        .size(1200, 820)
        .with_ui(|ui: &mut Ui| {
            // Root Application Container (Horizontal split: Sidebar + Main Content)
            ui.container()
                .fill()
                .bg(Color::rgb(0.06, 0.07, 0.09))
                .row()
                .show(|ui: &mut Ui| {

                    // ── LEFT SIDEBAR ──────────────────────────────────────────
                    ui.container()
                        .width(240.0)
                        .fill_y()
                        .bg(Color::rgb(0.08, 0.09, 0.12))
                        .border((Color::rgb(0.15, 0.17, 0.22), 1.0))
                        .padding_all(16.0)
                        .column()
                        .gap(10.0)
                        .show(|ui: &mut Ui| {

                            // Brand Header Block
                            ui.container()
                                .fill_x()
                                .height(46.0)
                                .bg(Color::rgb(0.12, 0.14, 0.19))
                                .border((Color::rgb(0.18, 0.22, 0.30), 1.0))
                                .radius_all(8.0)
                                .padding_x(12.0)
                                .row()
                                .valign(Align::Center)
                                .gap(10.0)
                                .show(|ui: &mut Ui| {
                                    // Logo badge
                                    ui.container()
                                        .width(24.0)
                                        .height(24.0)
                                        .bg_gradient(
                                            Color::rgb(0.30, 0.55, 1.0),
                                            Color::rgb(0.55, 0.25, 0.95),
                                            GradientDirection::Diagonal,
                                        )
                                        .radius_all(6.0)
                                        .show(|_| {});

                                    // Brand Title Bar
                                    ui.container()
                                        .width(110.0)
                                        .height(12.0)
                                        .bg(Color::rgb(0.85, 0.88, 0.95))
                                        .radius_all(3.0)
                                        .show(|_| {});
                                });

                            // Spacing
                            ui.container().height(8.0).show(|_| {});

                            // ── NAV BUTTON 1 (Active) ──
                            ui.container()
                                .fill_x()
                                .height(40.0)
                                .bg(Color::rgb(0.20, 0.42, 0.88))
                                .radius_all(6.0)
                                .padding_x(12.0)
                                .row()
                                .valign(Align::Center)
                                .gap(10.0)
                                .show(|ui: &mut Ui| {
                                    // Nav icon block
                                    ui.container()
                                        .width(16.0)
                                        .height(16.0)
                                        .bg(Color::WHITE)
                                        .radius_all(3.0)
                                        .show(|_| {});

                                    // Nav label bar
                                    ui.container()
                                        .width(85.0)
                                        .height(10.0)
                                        .bg(Color::WHITE)
                                        .radius_all(2.0)
                                        .show(|_| {});
                                });

                            // ── NAV BUTTON 2 ──
                            ui.container()
                                .fill_x()
                                .height(40.0)
                                .bg(Color::rgb(0.10, 0.12, 0.16))
                                .hover_bg(Color::rgb(0.15, 0.18, 0.24))
                                .hover_border((Color::rgb(0.25, 0.30, 0.40), 1.0))
                                .radius_all(6.0)
                                .padding_x(12.0)
                                .row()
                                .valign(Align::Center)
                                .gap(10.0)
                                .show(|ui: &mut Ui| {
                                    ui.container()
                                        .width(16.0)
                                        .height(16.0)
                                        .bg(Color::rgb(0.45, 0.50, 0.60))
                                        .radius_all(3.0)
                                        .show(|_| {});

                                    ui.container()
                                        .width(70.0)
                                        .height(10.0)
                                        .bg(Color::rgb(0.60, 0.65, 0.75))
                                        .radius_all(2.0)
                                        .show(|_| {});
                                });

                            // ── NAV BUTTON 3 ──
                            ui.container()
                                .fill_x()
                                .height(40.0)
                                .bg(Color::rgb(0.10, 0.12, 0.16))
                                .hover_bg(Color::rgb(0.15, 0.18, 0.24))
                                .hover_border((Color::rgb(0.25, 0.30, 0.40), 1.0))
                                .radius_all(6.0)
                                .padding_x(12.0)
                                .row()
                                .valign(Align::Center)
                                .gap(10.0)
                                .show(|ui: &mut Ui| {
                                    ui.container()
                                        .width(16.0)
                                        .height(16.0)
                                        .bg(Color::rgb(0.45, 0.50, 0.60))
                                        .radius_all(3.0)
                                        .show(|_| {});

                                    ui.container()
                                        .width(95.0)
                                        .height(10.0)
                                        .bg(Color::rgb(0.60, 0.65, 0.75))
                                        .radius_all(2.0)
                                        .show(|_| {});
                                });

                            // ── NAV BUTTON 4 ──
                            ui.container()
                                .fill_x()
                                .height(40.0)
                                .bg(Color::rgb(0.10, 0.12, 0.16))
                                .hover_bg(Color::rgb(0.15, 0.18, 0.24))
                                .hover_border((Color::rgb(0.25, 0.30, 0.40), 1.0))
                                .radius_all(6.0)
                                .padding_x(12.0)
                                .row()
                                .valign(Align::Center)
                                .gap(10.0)
                                .show(|ui: &mut Ui| {
                                    ui.container()
                                        .width(16.0)
                                        .height(16.0)
                                        .bg(Color::rgb(0.45, 0.50, 0.60))
                                        .radius_all(3.0)
                                        .show(|_| {});

                                    ui.container()
                                        .width(60.0)
                                        .height(10.0)
                                        .bg(Color::rgb(0.60, 0.65, 0.75))
                                        .radius_all(2.0)
                                        .show(|_| {});
                                });

                            // ── NAV BUTTON 5 (Settings) ──
                            ui.container()
                                .fill_x()
                                .height(40.0)
                                .bg(Color::rgb(0.10, 0.12, 0.16))
                                .hover_bg(Color::rgb(0.15, 0.18, 0.24))
                                .hover_border((Color::rgb(0.25, 0.30, 0.40), 1.0))
                                .radius_all(6.0)
                                .padding_x(12.0)
                                .row()
                                .valign(Align::Center)
                                .gap(10.0)
                                .show(|ui: &mut Ui| {
                                    ui.container()
                                        .width(16.0)
                                        .height(16.0)
                                        .bg(Color::rgb(0.45, 0.50, 0.60))
                                        .radius_all(3.0)
                                        .show(|_| {});

                                    ui.container()
                                        .width(78.0)
                                        .height(10.0)
                                        .bg(Color::rgb(0.60, 0.65, 0.75))
                                        .radius_all(2.0)
                                        .show(|_| {});
                                });

                            // Expand vertical space pushing profile card to bottom
                            ui.container().fill_y().show(|_| {});

                            // Bottom Profile Card
                            ui.container()
                                .fill_x()
                                .height(52.0)
                                .bg(Color::rgb(0.11, 0.13, 0.17))
                                .border((Color::rgb(0.18, 0.22, 0.28), 1.0))
                                .radius_all(8.0)
                                .padding_all(8.0)
                                .row()
                                .valign(Align::Center)
                                .gap(10.0)
                                .show(|ui: &mut Ui| {
                                    // Avatar circle
                                    ui.container()
                                        .width(32.0)
                                        .height(32.0)
                                        .bg(Color::rgb(0.25, 0.40, 0.65))
                                        .radius_all(16.0)
                                        .show(|_| {});

                                    // User meta
                                    ui.container()
                                        .column()
                                        .gap(6.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container()
                                                .width(70.0)
                                                .height(10.0)
                                                .bg(Color::rgb(0.80, 0.85, 0.92))
                                                .radius_all(2.0)
                                                .show(|_| {});
                                            ui.container()
                                                .width(50.0)
                                                .height(8.0)
                                                .bg(Color::rgb(0.45, 0.50, 0.60))
                                                .radius_all(2.0)
                                                .show(|_| {});
                                        });
                                });
                        });

                    // ── RIGHT MAIN DASHBOARD CONTENT AREA ──────────────────────
                    ui.container()
                        .fill()
                        .column()
                        .padding_all(20.0)
                        .gap(18.0)
                        .scroll(true) // Unified X & Y scrolling with clipping!
                        .show(|ui: &mut Ui| {

                            // Top Header Bar
                            ui.container()
                                .fill_x()
                                .height(50.0)
                                .bg(Color::rgb(0.08, 0.10, 0.13))
                                .border((Color::rgb(0.15, 0.18, 0.24), 1.0))
                                .radius_all(8.0)
                                .padding_x(16.0)
                                .row()
                                .valign(Align::Center)
                                .halign(Align::SpaceBetween)
                                .show(|ui: &mut Ui| {
                                    // Search Bar Container
                                    ui.container()
                                        .width(260.0)
                                        .height(32.0)
                                        .bg(Color::rgb(0.12, 0.14, 0.19))
                                        .border((Color::rgb(0.18, 0.22, 0.28), 1.0))
                                        .radius_all(6.0)
                                        .padding_x(10.0)
                                        .row()
                                        .valign(Align::Center)
                                        .gap(8.0)
                                        .show(|ui: &mut Ui| {
                                            // Search icon placeholder
                                            ui.container()
                                                .width(12.0)
                                                .height(12.0)
                                                .bg(Color::rgb(0.40, 0.45, 0.55))
                                                .radius_all(2.0)
                                                .show(|_| {});
                                            // Placeholder line
                                            ui.container()
                                                .width(90.0)
                                                .height(8.0)
                                                .bg(Color::rgb(0.35, 0.40, 0.50))
                                                .radius_all(2.0)
                                                .show(|_| {});
                                        });

                                    // Right Header Action Icons
                                    ui.container()
                                        .row()
                                        .gap(10.0)
                                        .valign(Align::Center)
                                        .show(|ui: &mut Ui| {
                                            // Notification icon box
                                            ui.container()
                                                .width(34.0)
                                                .height(34.0)
                                                .bg(Color::rgb(0.12, 0.14, 0.19))
                                                .border((Color::rgb(0.18, 0.22, 0.28), 1.0))
                                                .hover_bg(Color::rgb(0.16, 0.20, 0.28))
                                                .radius_all(6.0)
                                                .show(|_| {});

                                            // Action CTA button box
                                            ui.container()
                                                .width(110.0)
                                                .height(34.0)
                                                .bg(Color::rgb(0.20, 0.42, 0.88))
                                                .hover_bg(Color::rgb(0.25, 0.48, 0.98))
                                                .radius_all(6.0)
                                                .padding_x(12.0)
                                                .row()
                                                .valign(Align::Center)
                                                .halign(Align::Center)
                                                .show(|ui: &mut Ui| {
                                                    ui.container()
                                                        .width(60.0)
                                                        .height(10.0)
                                                        .bg(Color::WHITE)
                                                        .radius_all(2.0)
                                                        .show(|_| {});
                                                });
                                        });
                                });

                            // ── 4 METRIC STAT CARDS ROW ────────────────────────
                            ui.container()
                                .fill_x()
                                .row()
                                .gap(14.0)
                                .show(|ui: &mut Ui| {
                                    // Stat Card 1 (Total Revenue)
                                    ui.container()
                                        .width(215.0)
                                        .height(105.0)
                                        .bg(Color::rgb(0.09, 0.11, 0.15))
                                        .border((Color::rgb(0.16, 0.20, 0.28), 1.0))
                                        .hover_border((Color::rgb(0.30, 0.55, 1.0), 1.5))
                                        .hover_scale(1.02)
                                        .radius_all(10.0)
                                        .padding_all(14.0)
                                        .column()
                                        .gap(10.0)
                                        .valign(Align::SpaceBetween)
                                        .shadow(Color::rgba(0.0, 0.0, 0.0, 0.35), 0.0, 4.0, 10.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container().width(70.0).height(10.0).bg(Color::rgb(0.50, 0.55, 0.65)).radius_all(2.0).show(|_| {});
                                            ui.container().width(110.0).height(20.0).bg(Color::rgb(0.25, 0.55, 1.0)).radius_all(4.0).show(|_| {});
                                            ui.container().width(85.0).height(8.0).bg(Color::rgb(0.20, 0.70, 0.40)).radius_all(2.0).show(|_| {});
                                        });

                                    // Stat Card 2 (Active Users)
                                    ui.container()
                                        .width(215.0)
                                        .height(105.0)
                                        .bg(Color::rgb(0.09, 0.11, 0.15))
                                        .border((Color::rgb(0.16, 0.20, 0.28), 1.0))
                                        .hover_border((Color::rgb(0.25, 0.70, 0.75), 1.5))
                                        .hover_scale(1.02)
                                        .radius_all(10.0)
                                        .padding_all(14.0)
                                        .column()
                                        .gap(10.0)
                                        .valign(Align::SpaceBetween)
                                        .shadow(Color::rgba(0.0, 0.0, 0.0, 0.35), 0.0, 4.0, 10.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container().width(65.0).height(10.0).bg(Color::rgb(0.50, 0.55, 0.65)).radius_all(2.0).show(|_| {});
                                            ui.container().width(95.0).height(20.0).bg(Color::rgb(0.25, 0.70, 0.75)).radius_all(4.0).show(|_| {});
                                            ui.container().width(75.0).height(8.0).bg(Color::rgb(0.20, 0.70, 0.40)).radius_all(2.0).show(|_| {});
                                        });

                                    // Stat Card 3 (New Orders)
                                    ui.container()
                                        .width(215.0)
                                        .height(105.0)
                                        .bg(Color::rgb(0.09, 0.11, 0.15))
                                        .border((Color::rgb(0.16, 0.20, 0.28), 1.0))
                                        .hover_border((Color::rgb(0.60, 0.35, 0.90), 1.5))
                                        .hover_scale(1.02)
                                        .radius_all(10.0)
                                        .padding_all(14.0)
                                        .column()
                                        .gap(10.0)
                                        .valign(Align::SpaceBetween)
                                        .shadow(Color::rgba(0.0, 0.0, 0.0, 0.35), 0.0, 4.0, 10.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container().width(60.0).height(10.0).bg(Color::rgb(0.50, 0.55, 0.65)).radius_all(2.0).show(|_| {});
                                            ui.container().width(105.0).height(20.0).bg(Color::rgb(0.60, 0.35, 0.90)).radius_all(4.0).show(|_| {});
                                            ui.container().width(65.0).height(8.0).bg(Color::rgb(0.85, 0.40, 0.30)).radius_all(2.0).show(|_| {});
                                        });

                                    // Stat Card 4 (Conversion Rate)
                                    ui.container()
                                        .width(215.0)
                                        .height(105.0)
                                        .bg(Color::rgb(0.09, 0.11, 0.15))
                                        .border((Color::rgb(0.16, 0.20, 0.28), 1.0))
                                        .hover_border((Color::rgb(0.20, 0.75, 0.45), 1.5))
                                        .hover_scale(1.02)
                                        .radius_all(10.0)
                                        .padding_all(14.0)
                                        .column()
                                        .gap(10.0)
                                        .valign(Align::SpaceBetween)
                                        .shadow(Color::rgba(0.0, 0.0, 0.0, 0.35), 0.0, 4.0, 10.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container().width(75.0).height(10.0).bg(Color::rgb(0.50, 0.55, 0.65)).radius_all(2.0).show(|_| {});
                                            ui.container().width(80.0).height(20.0).bg(Color::rgb(0.20, 0.75, 0.45)).radius_all(4.0).show(|_| {});
                                            ui.container().width(90.0).height(8.0).bg(Color::rgb(0.20, 0.70, 0.40)).radius_all(2.0).show(|_| {});
                                        });
                                });

                            // ── LARGE ANALYTICS CHART CONTAINER ────────────────
                            ui.container()
                                .fill_x()
                                .height(250.0)
                                .bg(Color::rgb(0.08, 0.10, 0.14))
                                .border((Color::rgb(0.16, 0.19, 0.26), 1.0))
                                .radius_all(10.0)
                                .padding_all(18.0)
                                .column()
                                .halign(Align::SpaceBetween)
                                .shadow(Color::rgba(0.0, 0.0, 0.0, 0.4), 0.0, 6.0, 16.0)
                                .show(|ui: &mut Ui| {

                                    // Chart Header row
                                    ui.container()
                                        .fill_x()
                                        .row()
                                        .valign(Align::Center)
                                        .halign(Align::SpaceBetween)
                                        .show(|ui: &mut Ui| {
                                            // Chart title & sub bars
                                            ui.container()
                                                .column()
                                                .gap(6.0)
                                                .show(|ui: &mut Ui| {
                                                    ui.container().width(120.0).height(12.0).bg(Color::rgb(0.85, 0.88, 0.95)).radius_all(3.0).show(|_| {});
                                                    ui.container().width(80.0).height(8.0).bg(Color::rgb(0.40, 0.45, 0.55)).radius_all(2.0).show(|_| {});
                                                });

                                            // Chart Legend pill
                                            ui.container()
                                                .width(90.0)
                                                .height(26.0)
                                                .bg(Color::rgb(0.12, 0.15, 0.21))
                                                .border((Color::rgb(0.20, 0.25, 0.35), 1.0))
                                                .radius_all(13.0)
                                                .row()
                                                .valign(Align::Center)
                                                .halign(Align::Center)
                                                .gap(6.0)
                                                .show(|ui: &mut Ui| {
                                                    ui.container().width(8.0).height(8.0).bg(Color::rgb(0.25, 0.55, 1.0)).radius_all(4.0).show(|_| {});
                                                    ui.container().width(45.0).height(6.0).bg(Color::rgb(0.70, 0.75, 0.85)).radius_all(2.0).show(|_| {});
                                                });
                                        });

                                    // Chart Bars Row (visual graph built purely from containers)
                                    ui.container()
                                        .fill_x()
                                        .height(150.0)
                                        .row()
                                        .valign(Align::Bottom)
                                        .halign(Align::SpaceBetween)
                                        .padding_top(10.0)
                                        .show(|ui: &mut Ui| {
                                            let heights = [45.0, 75.0, 110.0, 65.0, 130.0, 95.0, 140.0, 115.0, 85.0, 125.0, 100.0, 145.0];
                                            for &h in &heights {
                                                ui.container()
                                                    .width(42.0)
                                                    .height(h)
                                                    .bg_gradient(
                                                        Color::rgb(0.22, 0.48, 0.95),
                                                        Color::rgb(0.12, 0.22, 0.55),
                                                        GradientDirection::Vertical,
                                                    )
                                                    .hover_bg(Color::rgb(0.35, 0.65, 1.0))
                                                    .radius_top(4.0)
                                                    .show(|_| {});
                                            }
                                        });
                                });

                            // ── BOTTOM SECTION: 2 SPLIT CARDS ──────────────────
                            ui.container()
                                .fill_x()
                                .row()
                                .gap(14.0)
                                .show(|ui: &mut Ui| {

                                    // Left Card: Activity Feed (Table rows)
                                    ui.container()
                                        .width(580.0)
                                        .height(170.0)
                                        .bg(Color::rgb(0.08, 0.10, 0.14))
                                        .border((Color::rgb(0.16, 0.19, 0.26), 1.0))
                                        .radius_all(10.0)
                                        .padding_all(16.0)
                                        .column()
                                        .gap(10.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container().width(110.0).height(12.0).bg(Color::rgb(0.85, 0.88, 0.95)).radius_all(3.0).show(|_| {});

                                            // 3 Activity row items
                                            for _ in 0..3 {
                                                ui.container()
                                                    .fill_x()
                                                    .height(32.0)
                                                    .bg(Color::rgb(0.11, 0.13, 0.18))
                                                    .border((Color::rgb(0.16, 0.20, 0.26), 1.0))
                                                    .radius_all(6.0)
                                                    .padding_x(10.0)
                                                    .row()
                                                    .valign(Align::Center)
                                                    .halign(Align::SpaceBetween)
                                                    .show(|ui: &mut Ui| {
                                                        ui.container()
                                                            .row()
                                                            .valign(Align::Center)
                                                            .gap(8.0)
                                                            .show(|ui: &mut Ui| {
                                                                ui.container().width(10.0).height(10.0).bg(Color::rgb(0.20, 0.70, 0.40)).radius_all(5.0).show(|_| {});
                                                                ui.container().width(130.0).height(8.0).bg(Color::rgb(0.75, 0.80, 0.90)).radius_all(2.0).show(|_| {});
                                                            });
                                                        ui.container().width(45.0).height(8.0).bg(Color::rgb(0.40, 0.45, 0.55)).radius_all(2.0).show(|_| {});
                                                    });
                                            }
                                        });

                                    // Right Card: Quick Actions
                                    ui.container()
                                        .width(310.0)
                                        .height(170.0)
                                        .bg(Color::rgb(0.08, 0.10, 0.14))
                                        .border((Color::rgb(0.16, 0.19, 0.26), 1.0))
                                        .radius_all(10.0)
                                        .padding_all(16.0)
                                        .column()
                                        .gap(10.0)
                                        .show(|ui: &mut Ui| {
                                            ui.container().width(90.0).height(12.0).bg(Color::rgb(0.85, 0.88, 0.95)).radius_all(3.0).show(|_| {});

                                            // 2 Action button cards
                                            ui.container()
                                                .fill_x()
                                                .height(44.0)
                                                .bg(Color::rgb(0.12, 0.15, 0.22))
                                                .border((Color::rgb(0.20, 0.26, 0.38), 1.0))
                                                .hover_bg(Color::rgb(0.16, 0.20, 0.30))
                                                .radius_all(6.0)
                                                .padding_x(12.0)
                                                .row()
                                                .valign(Align::Center)
                                                .gap(10.0)
                                                .show(|ui: &mut Ui| {
                                                    ui.container().width(14.0).height(14.0).bg(Color::rgb(0.30, 0.60, 1.0)).radius_all(3.0).show(|_| {});
                                                    ui.container().width(110.0).height(10.0).bg(Color::rgb(0.80, 0.85, 0.95)).radius_all(2.0).show(|_| {});
                                                });

                                            ui.container()
                                                .fill_x()
                                                .height(44.0)
                                                .bg(Color::rgb(0.12, 0.15, 0.22))
                                                .border((Color::rgb(0.20, 0.26, 0.38), 1.0))
                                                .hover_bg(Color::rgb(0.16, 0.20, 0.30))
                                                .radius_all(6.0)
                                                .padding_x(12.0)
                                                .row()
                                                .valign(Align::Center)
                                                .gap(10.0)
                                                .show(|ui: &mut Ui| {
                                                    ui.container().width(14.0).height(14.0).bg(Color::rgb(0.65, 0.35, 0.95)).radius_all(3.0).show(|_| {});
                                                    ui.container().width(90.0).height(10.0).bg(Color::rgb(0.80, 0.85, 0.95)).radius_all(2.0).show(|_| {});
                                                });
                                        });
                                });
                        });
                });
        })
        .run();
}
