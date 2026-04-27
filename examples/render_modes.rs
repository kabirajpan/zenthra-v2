use zenthra::prelude::*;

fn main() {
    let mut frame_count: u64 = 0;
    let mut l1_snapshot: Option<u64> = None;
    let mut l3_snapshot: Option<u64> = None;

    App::new()
        .title("Zenthra - Render Mode Inheritance")
        .size(900, 700)
        .with_ui(move |ui: &mut Ui| {
            // LEVEL 1: Root (Static by default)
            ui.container()
                .fill()
                .column()
                .padding(40.0, 40.0, 40.0, 40.0)
                .gap(30.0)
                .bg(Color::rgb(0.08, 0.08, 0.1))
                .center_x()
                .show(|ui: &mut Ui| {
                    ui.h1("Render Mode Inheritance").full_width_bg(false).show();
                    
                    let l1_text = match l1_snapshot {
                        Some(val) => format!("L1 Snapshot (Static): {}", val),
                        None => "L1 Snapshot: <Click Button>".to_string(),
                    };
                    ui.h3(&l1_text).color(Color::rgb(0.4, 0.7, 1.0)).full_width_bg(false).show();
                    
                    // LEVEL 2: Continuous (forced rendering)
                    ui.container()
                        .render_mode(RenderMode::Continuous)
                        .width(600.0)
                        .height(250.0)
                        .bg(Color::rgb(0.15, 0.15, 0.2))
                        .radius(10.0, 10.0, 10.0, 10.0)
                        .padding(20.0, 20.0, 20.0, 20.0)
                        .center()
                        .show(|ui: &mut Ui| {
                            frame_count += 1;
                            
                            ui.column().center().show(|ui: &mut Ui| {
                                ui.h2("Level 2 (Continuous)").full_width_bg(false).show();
                                ui.text("This container renders every frame at 120 FPS.")
                                    .color(Color::rgb(0.7, 0.7, 0.7))
                                    .full_width_bg(false)
                                    .show();
                                ui.spacing(10.0);
                                ui.h1(&format!("{}", frame_count))
                                    .color(Color::rgb(0.3, 1.0, 0.5))
                                    .full_width_bg(false)
                                    .show();

                                // LEVEL 3: Explicitly Static (Inside Continuous)
                                ui.container()
                                    .render_mode(RenderMode::Static)
                                    .width(400.0)
                                    .padding(15.0, 15.0, 15.0, 15.0)
                                    .bg(Color::rgb(0.05, 0.05, 0.07))
                                    .radius(5.0, 5.0, 5.0, 5.0)
                                    .center()
                                    .show(|ui: &mut Ui| {
                                        let l3_text = match l3_snapshot {
                                            Some(val) => format!("L3 Snapshot (Static): {}", val),
                                            None => "L3 Snapshot: <Click Button>".to_string(),
                                        };
                                        ui.text(&l3_text).color(Color::rgb(1.0, 0.4, 0.4)).full_width_bg(false).show();
                                    });
                            });
                        });

                    // Controls at Bottom
                    ui.row()
                        .gap(15.0)
                        .padding(20.0, 20.0, 20.0, 20.0)
                        .show(|ui: &mut Ui| {
                            if ui.button("Capture at Level 1").width(220.0).show().clicked {
                                l1_snapshot = Some(frame_count);
                                ui.request_redraw();
                            }
                            
                            if ui.button("Capture at Level 3").width(220.0).show().clicked {
                                l3_snapshot = Some(frame_count);
                                ui.request_redraw();
                            }

                            if ui.button("Reset").width(100.0).bg(Color::rgb(0.4, 0.1, 0.1)).show().clicked {
                                frame_count = 0;
                                l1_snapshot = None;
                                l3_snapshot = None;
                                ui.request_redraw();
                            }
                        });
                });
        })
        .run();
}
