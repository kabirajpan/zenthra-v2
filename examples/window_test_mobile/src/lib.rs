use zenthra::prelude::*;

#[cfg(target_os = "android")]
#[no_mangle]
pub fn android_main(app: android_activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .with_android_app(app)
        .build()
        .unwrap();
    
    run_with_event_loop(event_loop);
}

#[allow(dead_code)]
fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    run_with_event_loop(event_loop);
}

fn run_with_event_loop(event_loop: winit::event_loop::EventLoop<()>) {
    let mut win1_open = true;
    let mut win1_pos = [100.0, 100.0];
    
    let mut win2_open = false;
    let mut win2_pos = [200.0, 200.0];
    
    let mut counter = 0;

    App::new()
        .title("Zenthra - Mobile Test")
        .size(1200, 800)
        .with_ui(move |ui| {
            ui.container()
                .fill()
                .bg(Color::rgb(0.05, 0.05, 0.1))
                .padding_all(50.0)
                .show(|ui| {
                    ui.h1("Zenthra Mobile").color(Color::WHITE).show();
                    ui.text("Running natively on Android with Touch support.")
                        .color(Color::rgb(0.7, 0.7, 0.8))
                        .show();
                    
                    ui.spacing(30.0);
                    
                    if ui.button("Toggle Window 2")
                        .bg(Color::rgb(0.2, 0.4, 0.8))
                        .show()
                        .clicked 
                    {
                        win2_open = !win2_open;
                        ui.request_redraw();
                    }

                    ui.window("Controls", &mut win1_open, &mut win1_pos)
                        .size(350.0, 300.0)
                        .show(|ui| {
                            ui.text("Touch-ready floating window.").show();
                            ui.spacing(20.0);
                            
                            ui.row().gap(10.0).show(|ui| {
                                ui.text(&format!("Counter: {}", counter)).size(20.0).show();
                                if ui.button("+").show().clicked {
                                    counter += 1;
                                    ui.request_redraw();
                                }
                                if ui.button("-").show().clicked {
                                    counter -= 1;
                                    ui.request_redraw();
                                }
                            });
                        });

                    ui.window("Status", &mut win2_open, &mut win2_pos)
                        .size(300.0, 200.0)
                        .show(|ui| {
                            ui.text("Mobile performance is key.").show();
                        });
                });
        })
        .run_with_event_loop(event_loop);
}
