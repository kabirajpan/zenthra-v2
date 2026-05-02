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
                .fill_x()
                .fill_y()
                .column()
                .padding_all(40.0)
                .gap(30.0)
                .bg(Color::rgb(0.08, 0.08, 0.1))
                .halign(Align::Center)
                .show(|ui: &mut Ui| {
                    ui.text("Render Mode Inheritance")
                        .size(32.0)
                        .fill_x(false)
                        .show();
                    
                    let l1_text = match l1_snapshot {
                        Some(val) => format!("L1 Snapshot (Static): {}", val),
                        None => "L1 Snapshot: <Click Button>".to_string(),
                    };
                    
                    ui.text(&l1_text)
                        .size(18.0)
                        .color(Color::rgb(0.4, 0.7, 1.0))
                        .fill_x(false)
                        .show();
                    
                    // LEVEL 2: Continuous (forced rendering)
                    ui.container()
                        .render_mode(RenderMode::Continuous)
                        .width(600.0)
                        .height(250.0)
                        .bg(Color::rgb(0.15, 0.15, 0.2))
                        .radius_all(10.0)
                        .padding_all(20.0)
                        .align(Align::Center)
                        .show(|ui: &mut Ui| {
                            frame_count += 1;
                            
                            ui.column().align(Align::Center).show(|ui: &mut Ui| {
                                ui.text("Level 2 (Continuous)")
                                    .size(24.0)
                                    .fill_x(false)
                                    .show();
                                ui.text("This container renders every frame at 120 FPS.")
                                    .color(Color::rgb(0.7, 0.7, 0.7))
                                    .fill_x(false)
                                    .show();
                                ui.spacing(10.0);
                                ui.text(&format!("{}", frame_count))
                                    .size(32.0)
                                    .color(Color::rgb(0.3, 1.0, 0.5))
                                    .fill_x(false)
                                    .show();

                                // LEVEL 3: Explicitly Static (Inside Continuous)
                                ui.container()
                                    .render_mode(RenderMode::Static)
                                    .width(400.0)
                                    .padding_all(15.0)
                                    .bg(Color::rgb(0.05, 0.05, 0.07))
                                    .radius_all(5.0)
                                    .align(Align::Center)
                                    .show(|ui: &mut Ui| {
                                        let l3_text = match l3_snapshot {
                                            Some(val) => format!("L3 Snapshot (Static): {}", val),
                                            None => "L3 Snapshot: <Click Button>".to_string(),
                                        };
                                        ui.text(&l3_text)
                                            .color(Color::rgb(1.0, 0.4, 0.4))
                                            .fill_x(false)
                                            .show();
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
