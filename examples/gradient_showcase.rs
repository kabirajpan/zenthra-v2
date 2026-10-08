use zenthra::prelude::*;

fn main() {
    let mut selected_tab: usize = 0;
    let mut angle: f32 = 68.5;
    let mut progress: f32 = 0.65;
    let mut btn_clicks: usize = 0;

    App::new()
        .title("Zenthra - GPU Gradient Engine Showcase (All 26 Examples)")
        .size(1240, 920)
        .with_ui(move |ui| {
            // Main App Background: subtle dark radial glow
            ui.container()
                .fill()
                .bg(Gradient::radial(
                    Align::Center,
                    [Color::rgb(0.06, 0.07, 0.11), Color::rgb(0.02, 0.02, 0.03)],
                ))
                .scroll_y(true)
                .padding_all(24.0)
                .column()
                .gap(20.0)
                .show(|ui| {
                    // Header Bar
                    ui.container()
                        .full_width()
                        .row()
                        .halign(Align::SpaceBetween)
                        .valign(Align::Center)
                        .show(|ui| {
                            ui.container().column().gap(4.0).show(|ui| {
                                ui.h2("Zenthra GPU Gradient Engine")
                                    .color(Color::WHITE)
                                    .show();
                                ui.text("All 26 Official Types & Techniques from docs/api/gradients.md")
                                    .size(13.0)
                                    .color(Color::rgb(0.55, 0.60, 0.75))
                                    .show();
                            });

                            // WebGPU Status Badge
                            ui.container()
                                .height(32.0)
                                .padding(0.0, 16.0, 0.0, 16.0)
                                .radius_all(16.0)
                                .bg(Gradient::linear(
                                    Direction::ToRight,
                                    [Color::rgba(0.2, 0.5, 1.0, 0.3), Color::rgba(0.8, 0.2, 0.9, 0.3)],
                                ))
                                .border(Color::rgba(0.5, 0.7, 1.0, 0.4), 1.0)
                                .row()
                                .valign(Align::Center)
                                .show(|ui| {
                                    ui.text("Hardware Shaders | 144+ FPS")
                                        .size(12.0)
                                        .weight(FontWeight::Bold)
                                        .color(Color::rgb(0.85, 0.92, 1.0))
                                        .show();
                                });
                        });

                    // Category Tabs Filter
                    ui.container()
                        .full_width()
                        .row()
                        .gap(10.0)
                        .valign(Align::Center)
                        .show(|ui| {
                            let tabs = [
                                "All (26 Examples)",
                                "1. Directional (6)",
                                "2. Angles (3)",
                                "3. Multi-Stop (6)",
                                "4. Radial (8)",
                                "5. Mesh (3)",
                                "Interactive & Timeline",
                            ];

                            for (idx, title) in tabs.iter().enumerate() {
                                let is_active = selected_tab == idx;
                                let btn = ui.button(*title)
                                    .padding(7.0, 14.0, 7.0, 14.0)
                                    .radius_all(8.0)
                                    .bg(if is_active {
                                        Background::from(Gradient::linear(
                                            Direction::ToRight,
                                            [Color::rgb(0.20, 0.45, 0.95), Color::rgb(0.65, 0.20, 0.85)],
                                        ))
                                    } else {
                                        Background::from(Color::rgba(0.10, 0.12, 0.18, 0.8))
                                    })
                                    .hover_bg(if is_active {
                                        Background::from(Gradient::linear(
                                            Direction::ToRight,
                                            [Color::rgb(0.30, 0.55, 1.00), Color::rgb(0.75, 0.30, 0.95)],
                                        ))
                                    } else {
                                        Background::from(Color::rgba(0.18, 0.20, 0.28, 0.9))
                                    })
                                    .show();

                                if btn.clicked {
                                    selected_tab = idx;
                                }
                            }
                        });

                    // ========================================================
                    // FAMILY 1: Linear Directional (Standard Edges & Flows)
                    // ========================================================
                    if selected_tab == 0 || selected_tab == 1 {
                        render_section_header(
                            ui,
                            "Family 1: Linear Directional (Standard Edges & Flows)",
                            "Evenly spaced directional ramps across compass axes: ToRight, ToLeft, ToBottom, ToTop, ToBottomRight, ToTopRight",
                        );

                        // Row 1: Items 1, 2, 3
                        ui.container().full_width().row().gap(16.0).show(|ui| {
                            render_gallery_card(
                                ui,
                                1,
                                "Left to Right (Horizontal Flow)",
                                "Gradient::linear(Direction::ToRight, [BLUE, RED])",
                                Gradient::linear(Direction::ToRight, [Color::BLUE, Color::RED]),
                                372.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                2,
                                "Right to Left (Reverse Flow)",
                                "Gradient::linear(Direction::ToLeft, [BLUE, RED])",
                                Gradient::linear(Direction::ToLeft, [Color::BLUE, Color::RED]),
                                372.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                3,
                                "Top to Bottom (Vertical Drop)",
                                "Gradient::linear(Direction::ToBottom, [slate, dark])",
                                Gradient::linear(
                                    Direction::ToBottom,
                                    [Color::rgb(0.12, 0.14, 0.20), Color::rgb(0.04, 0.05, 0.07)],
                                ),
                                372.0,
                                130.0,
                            );
                        });

                        // Row 2: Items 4, 5, 6
                        ui.container().full_width().row().gap(16.0).show(|ui| {
                            render_gallery_card(
                                ui,
                                4,
                                "Bottom to Top (Ground Up / Fog)",
                                "Gradient::linear(Direction::ToTop, [BLACK, TRANSPARENT])",
                                Gradient::linear(Direction::ToTop, [Color::BLACK, Color::TRANSPARENT]),
                                372.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                5,
                                "Top-Left to Bottom-Right (Diagonal Sheen)",
                                "Gradient::linear(Direction::ToBottomRight, [sky, violet])",
                                Gradient::linear(
                                    Direction::ToBottomRight,
                                    [Color::rgb(0.20, 0.50, 1.00), Color::rgb(0.80, 0.20, 0.80)],
                                ),
                                372.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                6,
                                "Bottom-Left to Top-Right (Deep Aurora)",
                                "Gradient::linear(Direction::ToTopRight, [dark, indigo])",
                                Gradient::linear(
                                    Direction::ToTopRight,
                                    [Color::rgb(0.05, 0.05, 0.08), Color::rgb(0.18, 0.22, 0.32)],
                                ),
                                372.0,
                                130.0,
                            );
                        });
                    }

                    // ========================================================
                    // FAMILY 2: Linear Angles (Exact Degrees)
                    // ========================================================
                    if selected_tab == 0 || selected_tab == 2 {
                        render_section_header(
                            ui,
                            "Family 2: Linear Angles (Exact Degrees)",
                            "Continuous angle rotations in degrees: 45.0°, 135.0°, 68.5° or any custom dynamic float",
                        );

                        // Row: Items 7, 8, 9
                        ui.container().full_width().row().gap(16.0).show(|ui| {
                            render_gallery_card(
                                ui,
                                7,
                                "45° Standard Card Angle",
                                "Gradient::linear(45.0, [BLUE, PURPLE])",
                                Gradient::linear(45.0, [Color::BLUE, Color::PURPLE]),
                                372.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                8,
                                "135° Modern Dark SaaS Card",
                                "Gradient::linear(135.0, [dark_gray, pitch_black])",
                                Gradient::linear(
                                    135.0,
                                    [Color::rgb(0.14, 0.15, 0.18), Color::rgb(0.06, 0.07, 0.09)],
                                ),
                                372.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                9,
                                "Precision Angle (68.5° Sunset)",
                                "Gradient::linear(68.5, [sunset_orange, plum])",
                                Gradient::linear(
                                    68.5,
                                    [Color::rgb(0.95, 0.35, 0.20), Color::rgb(0.40, 0.10, 0.50)],
                                ),
                                372.0,
                                130.0,
                            );
                        });

                        // Interactive Angle Control Panel
                        ui.container()
                            .full_width()
                            .padding_all(18.0)
                            .radius_all(12.0)
                            .bg(Color::rgb(0.08, 0.09, 0.14))
                            .border(Color::rgb(0.18, 0.20, 0.28), 1.0)
                            .row()
                            .gap(20.0)
                            .valign(Align::Center)
                            .show(|ui| {
                                ui.container()
                                    .width(180.0)
                                    .height(80.0)
                                    .radius_all(8.0)
                                    .bg(Gradient::linear(
                                        angle,
                                        [Color::hex("#ff007a"), Color::hex("#00dfd8")],
                                    ))
                                    .border(Color::rgba(1.0, 1.0, 1.0, 0.3), 1.0)
                                    .halign(Align::Center)
                                    .valign(Align::Center)
                                    .show(|ui| {
                                        ui.text(&format!("{:.1}°", angle))
                                            .size(18.0)
                                            .weight(FontWeight::Bold)
                                            .color(Color::WHITE)
                                            .show();
                                    });

                                ui.container().column().gap(6.0).show(|ui| {
                                    ui.text(&format!("Live Shader Rotation: Gradient::linear({:.1}, [hot_pink, cyan])", angle))
                                        .size(13.5)
                                        .weight(FontWeight::Bold)
                                        .color(Color::WHITE)
                                        .show();
                                    ui.text("Drag slider to inspect smooth vector rotation at any arbitrary degree.")
                                        .size(11.5)
                                        .color(Color::rgb(0.6, 0.65, 0.75))
                                        .show();

                                    ui.slider(&mut angle, "angle_slider")
                                        .range(0.0, 360.0)
                                        .width(520.0)
                                        .show();
                                });
                            });
                    }

                    // ========================================================
                    // FAMILY 3: Multi-Color & Custom Stop Positions
                    // ========================================================
                    if selected_tab == 0 || selected_tab == 3 {
                        render_section_header(
                            ui,
                            "Family 3: Multi-Color & Custom Stop Positions",
                            "Hardware stop positions for 3 and 4 color ramps: Rainbow, Audio VU Meter, Metallic Chrome, and Edge Darkening",
                        );

                        // Row 1: Items 10, 11, 12
                        ui.container().full_width().row().gap(16.0).show(|ui| {
                            render_gallery_card(
                                ui,
                                10,
                                "3 Colors Evenly Spaced",
                                "Gradient::linear(ToRight, [BLUE, WHITE, RED])",
                                Gradient::linear(Direction::ToRight, [Color::BLUE, Color::WHITE, Color::RED]),
                                372.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                11,
                                "4 Colors (Rainbow / Spectrum)",
                                "Gradient::linear(ToRight, [blue, teal, yellow, red])",
                                Gradient::linear(
                                    Direction::ToRight,
                                    [
                                        Color::rgb(0.20, 0.50, 1.00),
                                        Color::rgb(0.20, 0.80, 0.60),
                                        Color::rgb(0.90, 0.80, 0.20),
                                        Color::rgb(0.90, 0.30, 0.30),
                                    ],
                                ),
                                372.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                12,
                                "3 Colors: Subtle Edge Darkening",
                                "Gradient::linear(ToRight, [(0.08, 0.48, 0.32), (0.12, 0.68, 0.45), (0.08, 0.48, 0.32)])",
                                Gradient::linear(
                                    Direction::ToRight,
                                    [
                                        (0.08, 0.48, 0.32), // Left: green little bit dark
                                        (0.12, 0.68, 0.45), // Middle: base green
                                        (0.08, 0.48, 0.32), // Right: green little bit dark
                                    ],
                                ),
                                372.0,
                                130.0,
                            );
                        });

                        // Row 2: Items 13, 14, 15
                        ui.container().full_width().row().gap(16.0).show(|ui| {
                            render_gallery_card(
                                ui,
                                13,
                                "Audio VU Meter (Green → Yellow → Red)",
                                "Gradient::linear(ToRight, [(0.0, green), (0.7, green), (0.85, yellow), (1.0, red)])",
                                Gradient::linear(
                                    Direction::ToRight,
                                    [
                                        (0.00, Color::rgb(0.10, 0.80, 0.35)),
                                        (0.70, Color::rgb(0.10, 0.80, 0.35)),
                                        (0.85, Color::rgb(0.95, 0.80, 0.10)),
                                        (1.00, Color::rgb(0.95, 0.20, 0.20)),
                                    ],
                                ),
                                372.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                14,
                                "Metallic Chrome / Silver Glint",
                                "Gradient::linear(45.0, [(0.0, slate), (0.4, glint), (0.6, shadow), (1.0, silver)])",
                                Gradient::linear(
                                    45.0,
                                    [
                                        (0.00, Color::rgb(0.50, 0.50, 0.55)),
                                        (0.40, Color::rgb(0.95, 0.95, 1.00)),
                                        (0.60, Color::rgb(0.30, 0.30, 0.35)),
                                        (1.00, Color::rgb(0.70, 0.70, 0.75)),
                                    ],
                                ),
                                372.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                15,
                                "Opacity Fade-Out (Solid to 0%)",
                                "Gradient::linear(Direction::ToRight, [(0.0, RED), (1.0, TRANSPARENT)])",
                                Gradient::linear(
                                    Direction::ToRight,
                                    [
                                        (0.0, Color::RED),
                                        (1.0, Color::TRANSPARENT),
                                    ],
                                ),
                                372.0,
                                130.0,
                            );
                        });
                    }

                    // ========================================================
                    // FAMILY 4: Radial Gradients (Spotlights, Vignettes & Halos)
                    // ========================================================
                    if selected_tab == 0 || selected_tab == 4 {
                        render_section_header(
                            ui,
                            "Family 4: Radial Gradients (Spotlights, Vignettes & Halos)",
                            "Spherical radiation anchored at Center, Corners, Edges, Normalized Coordinates (x, y), or Off-Screen Light Sources",
                        );

                        // Row 1: Items 16, 17, 18, 19
                        ui.container().full_width().row().gap(16.0).show(|ui| {
                            render_gallery_card(
                                ui,
                                16,
                                "Centered Radial (Red to Green)",
                                "Gradient::radial(Align::Center, [RED, GREEN])",
                                Gradient::radial(Align::Center, [Color::RED, Color::GREEN]),
                                275.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                17,
                                "Center Dark Vignette",
                                "Gradient::radial(Align::Center, [gray, BLACK])",
                                Gradient::radial(
                                    Align::Center,
                                    [Color::rgb(0.15, 0.16, 0.20), Color::BLACK],
                                ),
                                275.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                18,
                                "Left-Edge Glow",
                                "Gradient::radial(Align::Left, [RED, BLUE])",
                                Gradient::radial(Align::Left, [Color::RED, Color::BLUE]),
                                275.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                19,
                                "Top-Left Corner Glow",
                                "Gradient::radial(Align::TopLeft, [glow, TRANSPARENT])",
                                Gradient::radial(
                                    Align::TopLeft,
                                    [Color::rgba(0.20, 0.50, 1.00, 0.40), Color::TRANSPARENT],
                                ),
                                275.0,
                                130.0,
                            );
                        });

                        // Row 2: Items 20, 21, 22, 23
                        ui.container().full_width().row().gap(16.0).show(|ui| {
                            render_gallery_card(
                                ui,
                                20,
                                "Bottom-Right Corner Spotlight",
                                "Gradient::radial(Align::BottomRight, [rose, BLACK])",
                                Gradient::radial(
                                    Align::BottomRight,
                                    [Color::rgb(0.90, 0.30, 0.50), Color::BLACK],
                                ),
                                275.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                21,
                                "Arbitrary Position (0.25, 0.80)",
                                "Gradient::radial((0.25, 0.80), [amber, TRANSPARENT])",
                                Gradient::radial(
                                    (0.25, 0.80),
                                    [Color::rgb(0.90, 0.50, 0.10), Color::TRANSPARENT],
                                ),
                                275.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                22,
                                "Off-Screen Light Source (-0.30, 0.50)",
                                "Gradient::radial((-0.30, 0.50), [crimson, TRANSPARENT])",
                                Gradient::radial(
                                    (-0.30, 0.50),
                                    [Color::rgba(0.90, 0.20, 0.40, 0.70), Color::TRANSPARENT],
                                ),
                                275.0,
                                130.0,
                            );

                            render_gallery_card(
                                ui,
                                23,
                                "Concentric Pulse Ring",
                                "Gradient::radial(Center, [(0.0, cyan), (0.3, glow), (0.7, cyan), (1.0, trans)])",
                                Gradient::radial(
                                    Align::Center,
                                    [
                                        (0.0, Color::CYAN),
                                        (0.3, Color::rgba(0.0, 0.8, 1.0, 0.20)),
                                        (0.7, Color::CYAN),
                                        (1.0, Color::TRANSPARENT),
                                    ],
                                ),
                                275.0,
                                130.0,
                            );
                        });
                    }

                    // ========================================================
                    // FAMILY 5: Freeform Mesh Gradients (Organic Apple/Stripe)
                    // ========================================================
                    if selected_tab == 0 || selected_tab == 5 {
                        render_section_header(
                            ui,
                            "Family 5: Freeform Mesh Gradients (Organic Apple/Stripe Cards)",
                            "Continuous bilinear 4-corner spatial interpolation and organic 5-point and 3-point color nodes",
                        );

                        ui.container().full_width().row().gap(16.0).show(|ui| {
                            // Item 24: 4-Corner Mesh
                            render_gallery_card(
                                ui,
                                24,
                                "4-Corner Mesh Card (Apple & Stripe)",
                                "Gradient::mesh([((0,0), yellow), ((1,0), green), ((0,1), red), ((1,1), blue)])",
                                Gradient::mesh([
                                    ((0.0, 0.0), Color::rgb(0.98, 0.88, 0.15)), // TL: Yellow
                                    ((1.0, 0.0), Color::rgb(0.15, 0.78, 0.42)), // TR: Green
                                    ((0.0, 1.0), Color::rgb(0.92, 0.22, 0.22)), // BL: Red
                                    ((1.0, 1.0), Color::rgb(0.15, 0.45, 0.95)), // BR: Blue
                                ]),
                                372.0,
                                155.0,
                            );

                            // Item 25: 5-Point Organic Mesh
                            render_gallery_card(
                                ui,
                                25,
                                "5-Point Organic Mesh (Warm Amber Center)",
                                "Gradient::mesh([corners + (0.5, 0.5) warm center])",
                                Gradient::mesh([
                                    ((0.10, 0.10), Color::YELLOW),
                                    ((0.90, 0.10), Color::GREEN),
                                    ((0.50, 0.50), Color::rgb(0.95, 0.50, 0.15)),
                                    ((0.10, 0.90), Color::RED),
                                    ((0.90, 0.90), Color::BLUE),
                                ]),
                                372.0,
                                155.0,
                            );

                            // Item 26: 3-Point Pastel Mesh
                            render_gallery_card(
                                ui,
                                26,
                                "3-Point Pastel Mesh (Minimalist Mood)",
                                "Gradient::mesh([soft_pink, soft_sky_blue, soft_mint])",
                                Gradient::mesh([
                                    ((0.20, 0.20), Color::rgb(0.95, 0.75, 0.85)),
                                    ((0.80, 0.30), Color::rgb(0.75, 0.85, 0.95)),
                                    ((0.50, 0.90), Color::rgb(0.85, 0.95, 0.80)),
                                ]),
                                372.0,
                                155.0,
                            );
                        });
                    }

                    // ========================================================
                    // INTERACTIVE STATE SUPPORT & TIMELINE CLIPS
                    // ========================================================
                    if selected_tab == 0 || selected_tab == 6 {
                        render_section_header(
                            ui,
                            "Interactive State Support & Timeline Clip Edge Darkening",
                            "Gradients on buttons (.bg, .hover_bg, .active_bg), progress bars, and zero-overlay timeline edge darkening",
                        );

                        // Buttons & Progress Bar Row
                        ui.container().full_width().row().gap(20.0).show(|ui| {
                            // Left: Interactive Buttons
                            ui.container()
                                .width(560.0)
                                .padding_all(18.0)
                                .radius_all(12.0)
                                .bg(Color::rgb(0.07, 0.08, 0.12))
                                .border(Color::rgb(0.18, 0.20, 0.28), 1.0)
                                .column()
                                .gap(12.0)
                                .show(|ui| {
                                    ui.text("Interactive Gradient Buttons (.bg, .hover_bg, .active_bg)")
                                        .size(13.5)
                                        .weight(FontWeight::Bold)
                                        .color(Color::WHITE)
                                        .show();

                                    ui.container().row().gap(12.0).show(|ui| {
                                        let btn1 = ui.button("⚡ Boost Turbo")
                                            .padding(10.0, 20.0, 10.0, 20.0)
                                            .radius_all(8.0)
                                            .bg(Gradient::linear(
                                                Direction::ToRight,
                                                [Color::hex("#7928ca"), Color::hex("#ff007a")],
                                            ))
                                            .hover_bg(Gradient::linear(
                                                Direction::ToRight,
                                                [Color::hex("#8a38db"), Color::hex("#ff2e8f")],
                                            ))
                                            .active_bg(Gradient::linear(
                                                Direction::ToRight,
                                                [Color::hex("#5f1b9f"), Color::hex("#cc0062")],
                                            ))
                                            .show();

                                        if btn1.clicked {
                                            btn_clicks += 1;
                                            progress = (progress + 0.1).min(1.0);
                                        }

                                        let btn2 = ui.button("🌿 Fresh Green")
                                            .padding(10.0, 20.0, 10.0, 20.0)
                                            .radius_all(8.0)
                                            .bg(Gradient::linear(
                                                Direction::ToRight,
                                                [Color::hex("#059669"), Color::hex("#0284c7")],
                                            ))
                                            .hover_bg(Gradient::linear(
                                                Direction::ToRight,
                                                [Color::hex("#10b981"), Color::hex("#0ea5e9")],
                                            ))
                                            .show();

                                        if btn2.clicked {
                                            progress = 0.20;
                                        }
                                    });

                                    if btn_clicks > 0 {
                                        ui.text(&format!("Turbo mode activated {} times!", btn_clicks))
                                            .size(11.5)
                                            .color(Color::rgb(0.7, 0.85, 1.0))
                                            .show();
                                    }
                                });

                            // Right: Progress Bar & Slider
                            ui.container()
                                .width(560.0)
                                .padding_all(18.0)
                                .radius_all(12.0)
                                .bg(Color::rgb(0.07, 0.08, 0.12))
                                .border(Color::rgb(0.18, 0.20, 0.28), 1.0)
                                .column()
                                .gap(10.0)
                                .show(|ui| {
                                    ui.text(&format!("Dynamic Gradient Progress Bar ({:.0}%)", progress * 100.0))
                                        .size(13.5)
                                        .weight(FontWeight::Bold)
                                        .color(Color::WHITE)
                                        .show();

                                    ui.progress_bar(progress)
                                        .track_height(10.0)
                                        .radius_all(5.0)
                                        .track_gradient(
                                            Color::rgb(0.10, 0.11, 0.15),
                                            Color::rgb(0.14, 0.16, 0.22),
                                            GradientDirection::Horizontal,
                                        )
                                        .fill_gradient(
                                            Color::rgb(0.0, 0.8, 1.0),
                                            Color::rgb(0.85, 0.2, 0.95),
                                            GradientDirection::Horizontal,
                                        )
                                        .fill_shadow(Color::rgb(0.0, 0.8, 1.0), 0.0, 2.0, 10.0)
                                        .show();

                                    ui.spacing(2.0);

                                    ui.slider(&mut progress, "progress_slider")
                                        .range(0.0, 1.0)
                                        .width(520.0)
                                        .show();
                                });
                        });

                        // Video Timeline Clips Panel
                        ui.container()
                            .full_width()
                            .padding_all(18.0)
                            .radius_all(12.0)
                            .bg(Color::rgb(0.05, 0.06, 0.08))
                            .border(Color::rgb(0.15, 0.16, 0.22), 1.0)
                            .column()
                            .gap(12.0)
                            .show(|ui| {
                                ui.text("Video Timeline Track Clips: Multi-Stop Edge Darkening (No Overlays, 1 Draw Call)")
                                    .size(13.5)
                                    .weight(FontWeight::Bold)
                                    .color(Color::rgb(0.85, 0.90, 1.0))
                                    .show();
                                ui.text("Gradient::linear(Direction::ToRight, [dark, base, dark]) with adjacent split cuts sitting back-to-back")
                                    .size(11.5)
                                    .color(Color::rgb(0.55, 0.60, 0.70))
                                    .show();

                                // Track 1: Video with split cuts
                                ui.container().row().gap(2.0).valign(Align::Center).show(|ui| {
                                    render_clip_pill(
                                        ui,
                                        "intro_part1.mp4",
                                        240.0,
                                        Color::rgb(0.20, 0.40, 0.85),
                                    );
                                    // Split cut right next to part 1:
                                    render_clip_pill(
                                        ui,
                                        "intro_part2_split.mp4",
                                        220.0,
                                        Color::rgb(0.14, 0.52, 0.88),
                                    );
                                    render_clip_pill(
                                        ui,
                                        "b_roll_drone.mp4",
                                        280.0,
                                        Color::rgb(0.25, 0.45, 0.92),
                                    );
                                });

                                // Track 2: Audio with split cuts
                                ui.container().row().gap(2.0).valign(Align::Center).show(|ui| {
                                    render_clip_pill(
                                        ui,
                                        "ambient_bed.wav",
                                        260.0,
                                        Color::rgb(0.10, 0.62, 0.40),
                                    );
                                    // Split cut right next to ambient bed:
                                    render_clip_pill(
                                        ui,
                                        "vocal_take_02_split.wav",
                                        270.0,
                                        Color::rgb(0.12, 0.70, 0.48),
                                    );
                                    render_clip_pill(
                                        ui,
                                        "outro_chime.wav",
                                        210.0,
                                        Color::rgb(0.15, 0.76, 0.52),
                                    );
                                });
                            });
                    }
                });
        })
        .run();
}

/// Helper to render clean section headers with description
fn render_section_header(ui: &mut Ui, title: &str, description: &str) {
    ui.spacing(6.0);
    ui.container().column().gap(3.0).show(|ui| {
        ui.text(title)
            .size(15.5)
            .weight(FontWeight::Bold)
            .color(Color::WHITE)
            .show();
        ui.text(description)
            .size(11.5)
            .color(Color::rgb(0.55, 0.60, 0.72))
            .show();
    });
}

/// Helper to render gallery cards with full gradient background and glass floating badges
fn render_gallery_card(
    ui: &mut Ui,
    num: usize,
    title: &str,
    code_desc: &str,
    bg: Gradient,
    width: f32,
    height: f32,
) {
    ui.container()
        .width(width)
        .height(height)
        .radius_all(14.0)
        .bg(bg)
        .border(Color::rgba(1.0, 1.0, 1.0, 0.25), 1.0)
        .shadow(Color::BLACK, 0.0, 4.0, 16.0)
        .shadow_opacity(0.35)
        .padding_all(12.0)
        .column()
        .valign(Align::SpaceBetween)
        .show(|ui| {
            // Top Badge with number and title
            ui.container()
                .padding(3.0, 8.0, 3.0, 8.0)
                .radius_all(6.0)
                .bg(Color::rgba(0.0, 0.0, 0.0, 0.60))
                .border(Color::rgba(1.0, 1.0, 1.0, 0.15), 1.0)
                .show(|ui| {
                    ui.text(&format!("#{num}. {title}"))
                        .size(11.0)
                        .weight(FontWeight::Bold)
                        .color(Color::WHITE)
                        .show();
                });

            // Bottom Code Badge
            ui.container()
                .full_width()
                .padding(4.0, 8.0, 4.0, 8.0)
                .radius_all(6.0)
                .bg(Color::rgba(0.0, 0.0, 0.0, 0.65))
                .show(|ui| {
                    ui.text(code_desc)
                        .size(9.5)
                        .color(Color::rgb(0.85, 0.90, 1.0))
                        .wrap(TextWrap::None)
                        .ellipsis(true)
                        .show();
                });
        });
}

/// Helper to render timeline clip pills (3 colors: left dark, middle base, right dark)
fn render_clip_pill(ui: &mut Ui, label: &str, clip_w: f32, base: Color) {
    let dark = Color::rgb(base.r * 0.76, base.g * 0.76, base.b * 0.76);
    ui.container()
        .width(clip_w)
        .height(30.0)
        .radius_all(4.0)
        .bg(Gradient::linear(
            Direction::ToRight,
            [dark, base, dark],
        ))
        .border(Color::rgba(1.0, 1.0, 1.0, 0.15), 1.0)
        .padding(0.0, 10.0, 0.0, 10.0)
        .row()
        .valign(Align::Center)
        .show(|ui| {
            ui.text(label)
                .size(11.5)
                .weight(FontWeight::Bold)
                .color(Color::WHITE)
                .wrap(TextWrap::None)
                .ellipsis(true)
                .show();
        });
}
