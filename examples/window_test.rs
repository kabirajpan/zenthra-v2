use zenthra::prelude::*;
use zenthra_core::Id;


fn main() {
    let mut win1_open = true;
    let mut win1_pos = [100.0, 150.0];
    
    let mut win2_open = false;
    let mut win2_pos = [500.0, 200.0];

    let mut win_modal_open = false;
    let mut win_modal_pos = [420.0, 260.0];

    let mut win_dismiss_open = false;
    let mut win_dismiss_pos = [300.0, 450.0];
    
    let mut modal_action_status = String::from("No action taken yet.");
    let mut counter = 0;

    App::new()
        .title("Zenthra - Logical Window Flow")
        .size(1200, 800)
        .with_ui(move |ui| {
            // State Update Keys for cleaner communication and preventing borrow checker issues
            let close_win1_key = Id::from_u64(1001);
            let open_win2_key = Id::from_u64(1002);
            let close_win2_key = Id::from_u64(1003);
            let open_modal_key = Id::from_u64(1004);
            let close_modal_key = Id::from_u64(1005);
            let open_dismiss_key = Id::from_u64(1006);
            let close_dismiss_key = Id::from_u64(1007);

            // Process State Transitions
            if ui.interaction_state.remove(&close_win1_key).unwrap_or(0.0) > 0.5 {
                win1_open = false;
                ui.needs_redraw = true;
            }
            if ui.interaction_state.remove(&open_win2_key).unwrap_or(0.0) > 0.5 {
                win2_open = true;
                ui.needs_redraw = true;
            }
            if ui.interaction_state.remove(&close_win2_key).unwrap_or(0.0) > 0.5 {
                win2_open = false;
                ui.needs_redraw = true;
            }
            if ui.interaction_state.remove(&open_modal_key).unwrap_or(0.0) > 0.5 {
                win_modal_open = true;
                ui.needs_redraw = true;
            }
            if ui.interaction_state.remove(&close_modal_key).unwrap_or(0.0) > 0.5 {
                win_modal_open = false;
                ui.needs_redraw = true;
            }
            if ui.interaction_state.remove(&open_dismiss_key).unwrap_or(0.0) > 0.5 {
                win_dismiss_open = true;
                ui.needs_redraw = true;
            }
            if ui.interaction_state.remove(&close_dismiss_key).unwrap_or(0.0) > 0.5 {
                win_dismiss_open = false;
                ui.needs_redraw = true;
            }

            // Background
            ui.container()
                .fill()
                .bg(Color::rgb(0.05, 0.05, 0.1))
                .padding_all(50.0)
                .show(|ui| {
                    ui.h1("Logical Window Flow").color(Color::WHITE).show();
                    ui.text("Test the connection, focus, and modal flows of Zenthra windows.")
                        .color(Color::rgb(0.7, 0.7, 0.8))
                        .show();
                    
                    ui.spacing(15.0);
                    ui.text(&format!("Status: {}", modal_action_status))
                        .color(Color::rgb(0.9, 0.9, 0.6))
                        .show();
                    
                    ui.spacing(20.0);

                    // Background Controls
                    ui.row().gap(15.0).show(|ui| {
                        if ui.button("Open Main Window")
                            .bg(Color::rgb(0.2, 0.5, 0.9))
                            .show()
                            .clicked 
                        {
                            win1_open = true;
                            ui.request_redraw();
                        }

                        if ui.button("Reset Environment")
                            .bg(Color::rgb(0.3, 0.3, 0.35))
                            .show()
                            .clicked 
                        {
                            win1_open = true;
                            win1_pos = [100.0, 150.0];
                            win2_open = false;
                            win2_pos = [500.0, 200.0];
                            win_modal_open = false;
                            win_modal_pos = [420.0, 260.0];
                            win_dismiss_open = false;
                            win_dismiss_pos = [300.0, 450.0];
                            modal_action_status = String::from("Environment reset.");
                            ui.request_redraw();
                        }
                    });
                    
                    ui.spacing(30.0);
                    
                    // --- Window 1: Primary Controls ---
                    let mut temp_win1_open = win1_open;
                    ui.window("Primary Control Center", &mut temp_win1_open, &mut win1_pos)
                        .size(360.0, 320.0)
                        .show(|ui| {
                            ui.text("This is the main orchestrator panel.").show();
                            ui.spacing(15.0);
                            
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
                            
                            ui.spacing(20.0);
                            ui.text("Flow Triggers:").bold().show();
                            ui.spacing(10.0);
                            
                            ui.row().gap(10.0).show(|ui| {
                                if ui.button(if win2_open { "Close Info Panel" } else { "Open Info Panel" })
                                    .bg(Color::rgb(0.2, 0.6, 0.8))
                                    .show()
                                    .clicked
                                {
                                    if win2_open {
                                        ui.interaction_state.insert(close_win2_key, 1.0);
                                    } else {
                                        ui.interaction_state.insert(open_win2_key, 1.0);
                                    }
                                    ui.request_redraw();
                                }
                                
                                if ui.button("Quick Menu")
                                    .bg(Color::rgb(0.2, 0.7, 0.4))
                                    .show()
                                    .clicked
                                {
                                    ui.interaction_state.insert(open_dismiss_key, 1.0);
                                    ui.request_redraw();
                                }
                            });
                            
                            ui.spacing(25.0);
                            if ui.button("Close This Window")
                                .bg(Color::rgb(0.6, 0.2, 0.2))
                                .fill_x()
                                .show()
                                .clicked
                            {
                                ui.interaction_state.insert(close_win1_key, 1.0);
                                ui.request_redraw();
                            }
                        });
                    win1_open = temp_win1_open;

                    // --- Window 2: Secondary Information Panel ---
                    let mut temp_win2_open = win2_open;
                    ui.window("Information Panel", &mut temp_win2_open, &mut win2_pos)
                        .size(340.0, 240.0)
                        .show(|ui| {
                            ui.text("This panel displays system analytics.").show();
                            ui.spacing(15.0);
                            ui.text("To perform a critical action, trigger the modal.").show();
                            ui.spacing(25.0);
                            
                            ui.row().gap(10.0).show(|ui| {
                                if ui.button("Trigger Modal Dialog")
                                    .bg(Color::rgb(0.8, 0.2, 0.4))
                                    .show()
                                    .clicked
                                {
                                    ui.interaction_state.insert(open_modal_key, 1.0);
                                    ui.request_redraw();
                                }

                                if ui.button("Close Panel")
                                    .bg(Color::rgb(0.4, 0.4, 0.45))
                                    .show()
                                    .clicked
                                {
                                    ui.interaction_state.insert(close_win2_key, 1.0);
                                    ui.request_redraw();
                                }
                            });
                        });
                    win2_open = temp_win2_open;

                    // --- Window 3: Modal Alert dialog ---
                    let mut temp_modal_open = win_modal_open;
                    ui.window("Confirm Action", &mut temp_modal_open, &mut win_modal_pos)
                        .size(340.0, 220.0)
                        .modal(true)
                        .show(|ui| {
                            ui.text("⚠️ Critical Action Required").bold().color(Color::rgb(1.0, 0.3, 0.3)).show();
                            ui.spacing(10.0);
                            ui.text("This modal locks interaction. Are you sure you want to perform this action?").show();
                            ui.spacing(25.0);
                            
                            ui.row().gap(15.0).show(|ui| {
                                if ui.button("Confirm")
                                    .bg(Color::rgb(0.8, 0.2, 0.2))
                                    .show()
                                    .clicked
                                {
                                    modal_action_status = String::from("Action confirmed successfully!");
                                    ui.interaction_state.insert(close_modal_key, 1.0);
                                    ui.request_redraw();
                                }

                                if ui.button("Cancel")
                                    .bg(Color::rgb(0.4, 0.4, 0.4))
                                    .show()
                                    .clicked
                                {
                                    modal_action_status = String::from("Action canceled.");
                                    ui.interaction_state.insert(close_modal_key, 1.0);
                                    ui.request_redraw();
                                }
                            });
                        });
                    win_modal_open = temp_modal_open;

                    // --- Window 4: Light-Dismiss Popup ---
                    let mut temp_dismiss_open = win_dismiss_open;
                    ui.window("Quick Settings", &mut temp_dismiss_open, &mut win_dismiss_pos)
                        .size(280.0, 200.0)
                        .light_dismiss(true)
                        .show(|ui| {
                            ui.text("Click outside to dismiss this panel.").show();
                            ui.spacing(15.0);
                            
                            if ui.button("Select Option A").fill_x().show().clicked {
                                modal_action_status = String::from("Option A selected via Popup.");
                                ui.interaction_state.insert(close_dismiss_key, 1.0);
                                ui.request_redraw();
                            }
                            ui.spacing(5.0);
                            if ui.button("Select Option B").fill_x().show().clicked {
                                modal_action_status = String::from("Option B selected via Popup.");
                                ui.interaction_state.insert(close_dismiss_key, 1.0);
                                ui.request_redraw();
                            }
                            ui.spacing(10.0);
                            if ui.button("Close Menu")
                                .bg(Color::rgb(0.4, 0.4, 0.4))
                                .fill_x()
                                .show()
                                .clicked
                            {
                                ui.interaction_state.insert(close_dismiss_key, 1.0);
                                ui.request_redraw();
                            }
                        });
                    win_dismiss_open = temp_dismiss_open;
                });
        })
        .run();
}
