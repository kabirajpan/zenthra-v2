use zenthra::prelude::*;

struct ClipItem {
    id: usize,
    name: &'static str,
    start_sec: f32,
    dur_sec: f32,
}

struct TrackItem {
    name: &'static str,
    badge: &'static str,
    color: (f32, f32, f32), // Track-level uniform color
    clips: Vec<ClipItem>,
}

fn main() {
    let tracks = vec![
        TrackItem {
            name: "V1: Video A-Roll",
            badge: "VIDEO",
            color: (0.18, 0.40, 0.86), // All V1 clips share this EXACT same blue
            clips: vec![
                // 3-way split clips touching edge-to-edge (0 gap, same color):
                ClipItem { id: 1, name: "interview_take1_A.mp4", start_sec: 0.0, dur_sec: 2.5 },
                ClipItem { id: 2, name: "interview_take1_B.mp4", start_sec: 2.5, dur_sec: 2.8 },
                ClipItem { id: 3, name: "interview_take1_C.mp4", start_sec: 5.3, dur_sec: 2.2 },
                // Standalone clip later on track:
                ClipItem { id: 4, name: "broll_cutaway.mp4", start_sec: 8.0, dur_sec: 2.5 },
            ],
        },
        TrackItem {
            name: "A1: Master Audio",
            badge: "AUDIO",
            color: (0.12, 0.68, 0.45), // All A1 clips share this EXACT same green
            clips: vec![
                // Two split clips touching edge-to-edge (0 gap, same color):
                ClipItem { id: 5, name: "vocal_master_part1.wav", start_sec: 0.0, dur_sec: 4.2 },
                ClipItem { id: 6, name: "vocal_master_part2.wav", start_sec: 4.2, dur_sec: 3.4 },
                // Standalone clip:
                ClipItem { id: 7, name: "outro_ambient.wav", start_sec: 8.0, dur_sec: 2.5 },
            ],
        },
        TrackItem {
            name: "V2: Color Grade",
            badge: "GRADE",
            color: (0.60, 0.22, 0.82), // All V2 clips share this EXACT same purple
            clips: vec![
                // Split LUT adjustments touching edge-to-edge:
                ClipItem { id: 8, name: "lut_film_look_A.cube", start_sec: 1.0, dur_sec: 3.5 },
                ClipItem { id: 9, name: "lut_film_look_B.cube", start_sec: 4.5, dur_sec: 3.5 },
                ClipItem { id: 10, name: "vignette_heavy.cube", start_sec: 8.2, dur_sec: 2.2 },
            ],
        },
        TrackItem {
            name: "V3: Motion Titles",
            badge: "TITLES",
            color: (0.95, 0.42, 0.12), // All V3 clips share this EXACT same amber
            clips: vec![
                ClipItem { id: 11, name: "lower_third.svg", start_sec: 0.5, dur_sec: 3.0 },
                // Two split graphics touching edge-to-edge:
                ClipItem { id: 12, name: "quote_card_A.svg", start_sec: 4.0, dur_sec: 2.8 },
                ClipItem { id: 13, name: "quote_card_B.svg", start_sec: 6.8, dur_sec: 3.2 },
            ],
        },
    ];

    let mut selected_clip_id: usize = 2; // Default to selected split cut
    let time_scale: f32 = 88.0; // px per second

    App::new()
        .title("Zenthra - Timeline Multi-Clip & Split Cut Showcase")
        .size(1160, 680)
        .with_ui(move |ui| {
            ui.container()
                .fill()
                .bg(Color::rgb(0.04, 0.04, 0.06))
                .padding_all(24.0)
                .column()
                .gap(16.0)
                .show(|ui| {
                    // Header Bar
                    ui.container()
                        .full_width()
                        .row()
                        .halign(Align::SpaceBetween)
                        .valign(Align::Center)
                        .show(|ui| {
                            ui.container().column().gap(3.0).show(|ui| {
                                ui.h2("Timeline Clips: Perfectly Joint Split Cuts (Edge-to-Edge)")
                                    .color(Color::WHITE)
                                    .show();
                                ui.text("With static colors, split cuts of the same color blend into 1 blob. With [dark, base, dark], each clip forms a natural dark vignette seam at the cut!")
                                    .size(12.5)
                                    .color(Color::rgb(0.55, 0.60, 0.72))
                                    .show();
                            });

                            // Status Pill
                            ui.container()
                                .height(30.0)
                                .padding(0.0, 14.0, 0.0, 14.0)
                                .radius_all(15.0)
                                .bg(Gradient::linear(
                                    Direction::ToRight,
                                    [(0.12, 0.35, 0.75), (0.45, 0.15, 0.65)],
                                ))
                                .border(Color::rgba(1.0, 1.0, 1.0, 0.2), 1.0)
                                .row()
                                .valign(Align::Center)
                                .show(|ui| {
                                    ui.text("Same-Color Split Cuts Clearly Visible")
                                        .size(11.0)
                                        .weight(FontWeight::Bold)
                                        .color(Color::rgb(0.9, 0.95, 1.0))
                                        .show();
                                });
                        });

                    // Timeline Panel
                    ui.container()
                        .full_width()
                        .padding_all(16.0)
                        .radius_all(12.0)
                        .bg(Color::rgb(0.07, 0.07, 0.09))
                        .border(Color::rgb(0.16, 0.17, 0.22), 1.0)
                        .column()
                        .gap(8.0)
                        .show(|ui| {
                            // Timeline Ruler Header
                            ui.container()
                                .full_width()
                                .height(22.0)
                                .row()
                                .valign(Align::Center)
                                .padding_left(130.0)
                                .show(|ui| {
                                    for sec in 0..=10 {
                                        let tick_w = time_scale;
                                        ui.container().width(tick_w).row().gap(4.0).valign(Align::Center).show(|ui| {
                                            ui.container().width(1.0).height(8.0).bg(Color::rgb(0.3, 0.35, 0.45)).show(|_| {});
                                            ui.text(&format!("00:{:02}", sec))
                                                .size(10.0)
                                                .color(Color::rgb(0.45, 0.50, 0.60))
                                                .show();
                                        });
                                    }
                                });

                            // Track Rows
                            for track in tracks.iter() {
                                ui.container()
                                    .full_width()
                                    .height(42.0)
                                    .radius_all(6.0)
                                    .bg(Color::rgb(0.03, 0.03, 0.04))
                                    .border(Color::rgb(0.12, 0.13, 0.16), 1.0)
                                    .row()
                                    .gap(0.0)
                                    .valign(Align::Center)
                                    .padding_left(12.0)
                                    .show(|ui| {
                                        // Track Header / Name & Badge (Fixed width 115px)
                                        ui.container().width(115.0).row().gap(6.0).valign(Align::Center).show(|ui| {
                                            ui.container()
                                                .padding(2.0, 5.0, 2.0, 5.0)
                                                .radius_all(3.0)
                                                .bg(Color::rgba(1.0, 1.0, 1.0, 0.08))
                                                .show(|ui| {
                                                    ui.text(track.badge)
                                                        .size(9.0)
                                                        .weight(FontWeight::Bold)
                                                        .color(Color::rgb(0.7, 0.75, 0.85))
                                                        .show();
                                                });

                                            ui.text(track.name)
                                                .size(10.5)
                                                .weight(FontWeight::Bold)
                                                .color(Color::rgb(0.65, 0.70, 0.80))
                                                .wrap(TextWrap::None)
                                                .ellipsis(true)
                                                .show();
                                        });

                                        // Clips Lane Container (gap = 0.0 for seamless edge-to-edge joints)
                                        ui.container()
                                            .row()
                                            .gap(0.0)
                                            .valign(Align::Center)
                                            .show(|ui| {
                                                let mut cursor_time: f32 = 0.0;

                                                for (idx, clip) in track.clips.iter().enumerate() {
                                                    let is_selected = selected_clip_id == clip.id;

                                                    // Calculate gap from previous clip
                                                    let gap_sec = (clip.start_sec - cursor_time).max(0.0);
                                                    if gap_sec > 0.001 {
                                                        let spacer_w = (gap_sec * time_scale).round();
                                                        ui.container().width(spacer_w).height(30.0).show(|_| {});
                                                    }

                                                    let clip_w = (clip.dur_sec * time_scale).round().max(36.0);
                                                    cursor_time = clip.start_sec + clip.dur_sec;

                                                    // Detect if touching neighbor clips for seamless split corner joint
                                                    let is_touching_prev = idx > 0 && (track.clips[idx - 1].start_sec + track.clips[idx - 1].dur_sec - clip.start_sec).abs() < 0.01;
                                                    let is_touching_next = idx + 1 < track.clips.len() && (clip.start_sec + clip.dur_sec - track.clips[idx + 1].start_sec).abs() < 0.01;

                                                    // Perfectly flush joint: flat corners at the seam!
                                                    let (r_tl, r_tr, r_br, r_bl) = match (is_touching_prev, is_touching_next) {
                                                        (true, true) => (0.0, 0.0, 0.0, 0.0),    // Middle segment
                                                        (true, false) => (0.0, 3.0, 3.0, 0.0),   // Right side of cut: rounded right only
                                                        (false, true) => (3.0, 0.0, 0.0, 3.0),   // Left side of cut: rounded left only
                                                        (false, false) => (3.0, 3.0, 3.0, 3.0),  // Isolated clip
                                                    };

                                                    // 3 Colors: Left dark (76%), Center base (100%), Right dark (76%)
                                                    let base = track.color;
                                                    let dark = (base.0 * 0.76, base.1 * 0.76, base.2 * 0.76);

                                                    let border_c = if is_selected {
                                                        Color::WHITE
                                                    } else {
                                                        Color::rgba(1.0, 1.0, 1.0, 0.15)
                                                    };
                                                    let border_w = if is_selected { 1.5 } else { 0.5 };

                                                    let clip_resp = ui.container()
                                                        .width(clip_w)
                                                        .height(30.0)
                                                        .radius(r_tl, r_tr, r_br, r_bl)
                                                        .border(border_c, border_w)
                                                        .bg(Gradient::linear(
                                                            Direction::ToRight,
                                                            [dark, base, dark],
                                                        ))
                                                        .padding(0.0, 8.0, 0.0, 8.0)
                                                        .row()
                                                        .valign(Align::Center)
                                                        .show(|ui| {
                                                            ui.text(clip.name)
                                                                .size(10.5)
                                                                .weight(FontWeight::Bold)
                                                                .color(Color::WHITE)
                                                                .wrap(TextWrap::None)
                                                                .ellipsis(true)
                                                                .show();
                                                        });

                                                    if clip_resp.clicked {
                                                        selected_clip_id = clip.id;
                                                    }
                                                }
                                            });
                                    });
                            }
                        });

                    // Selected Clip Inspector
                    let mut found_clip = None;
                    let mut found_track_name = "";
                    let mut found_track_color = (0.0, 0.0, 0.0);

                    for track in tracks.iter() {
                        for clip in track.clips.iter() {
                            if clip.id == selected_clip_id {
                                found_clip = Some(clip);
                                found_track_name = track.name;
                                found_track_color = track.color;
                                break;
                            }
                        }
                    }

                    if let Some(clip) = found_clip {
                        ui.container()
                            .full_width()
                            .padding_all(16.0)
                            .radius_all(10.0)
                            .bg(Color::rgb(0.08, 0.09, 0.13))
                            .border(Color::rgb(0.18, 0.20, 0.28), 1.0)
                            .row()
                            .halign(Align::SpaceBetween)
                            .valign(Align::Center)
                            .show(|ui| {
                                ui.container().column().gap(4.0).show(|ui| {
                                    ui.row().gap(8.0).valign(Align::Center).show(|ui| {
                                        ui.text(&format!("Selected Clip: {}", clip.name))
                                            .size(14.0)
                                            .weight(FontWeight::Bold)
                                            .color(Color::WHITE)
                                            .show();

                                        ui.container()
                                            .padding(2.0, 8.0, 2.0, 8.0)
                                            .radius_all(4.0)
                                            .bg(Color::rgba(1.0, 1.0, 1.0, 0.1))
                                            .show(|ui| {
                                                ui.text(found_track_name)
                                                    .size(10.0)
                                                    .color(Color::rgb(0.75, 0.82, 0.95))
                                                    .show();
                                            });
                                    });

                                    ui.text(&format!(
                                        "Position: {:.1}s  |  Duration: {:.1}s  |  Ends at: {:.1}s  |  Cut Joint: Edge-to-Edge Flush with Vignette Seam",
                                        clip.start_sec, clip.dur_sec, clip.start_sec + clip.dur_sec
                                    ))
                                    .size(11.5)
                                    .color(Color::rgb(0.60, 0.65, 0.78))
                                    .show();
                                });

                                // Mini swatch preview
                                let base = found_track_color;
                                let dark = (base.0 * 0.76, base.1 * 0.76, base.2 * 0.76);
                                ui.container()
                                    .width(150.0)
                                    .height(34.0)
                                    .radius_all(4.0)
                                    .bg(Gradient::linear(
                                        Direction::ToRight,
                                        [dark, base, dark],
                                    ))
                                    .border(Color::WHITE, 1.0)
                                    .halign(Align::Center)
                                    .valign(Align::Center)
                                    .show(|ui| {
                                        ui.text("Split Cut Seam")
                                            .size(10.0)
                                            .weight(FontWeight::Bold)
                                            .color(Color::WHITE)
                                            .show();
                                    });
                            });
                    }
                });
        })
        .run();
}
