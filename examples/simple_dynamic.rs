use zenthra::prelude::*;

fn main() {
    let mut show_box_1 = false;
    let mut show_box_2 = false;

    App::new()
        .title("Zenthra - Dynamic Containers")
        .size(800, 600)
        .with_ui(move |ui| {
            ui.container()
                .fill()
                .bg(Color::rgb(0.05, 0.05, 0.08))
                .padding_all(40.0)
                .show(|ui| {
                    ui.h1("Dynamic Content Test").color(Color::WHITE).show();
                    ui.spacing(20.0);

                    ui.container().row().gap(20.0).show(|ui| {
                        if ui.button("Toggle Box 1")
                            .bg(if show_box_1 { Color::rgb(0.2, 0.6, 0.4) } else { Color::rgb(0.2, 0.2, 0.25) })
                            .padding(15.0, 15.0, 15.0, 15.0)
                            .radius_all(8.0)
                            .show()
                            .clicked 
                        {
                            show_box_1 = !show_box_1;
                        }

                        if ui.button("Toggle Box 2")
                            .bg(if show_box_2 { Color::rgb(0.6, 0.2, 0.4) } else { Color::rgb(0.2, 0.2, 0.25) })
                            .padding(15.0, 15.0, 15.0, 15.0)
                            .radius_all(8.0)
                            .show()
                            .clicked 
                        {
                            show_box_2 = !show_box_2;
                        }
                    });

                    ui.spacing(30.0);

                    ui.container().row().gap(20.0).show(|ui| {
                        if show_box_1 {
                            ui.container()
                                .width(200.0)
                                .height(150.0)
                                .bg(Color::rgb(0.15, 0.2, 0.3))
                                .radius_all(12.0)
                                .border(Color::rgb(0.3, 0.5, 0.8), 2.0)
                                .padding_all(20.0)
                                .show(|ui| {
                                    ui.h3("Container 1").color(Color::WHITE).show();
                                    ui.text("This is dynamic content triggered by button 1.")
                                        .color(Color::rgba(1.0, 1.0, 1.0, 0.7))
                                        .show();
                                });
                        }

                        if show_box_2 {
                            ui.container()
                                .width(200.0)
                                .height(150.0)
                                .bg(Color::rgb(0.3, 0.15, 0.2))
                                .radius_all(12.0)
                                .border(Color::rgb(0.8, 0.3, 0.5), 2.0)
                                .padding_all(20.0)
                                .show(|ui| {
                                    ui.h3("Container 2").color(Color::WHITE).show();
                                    ui.text("This is dynamic content triggered by button 2.")
                                        .color(Color::rgba(1.0, 1.0, 1.0, 0.7))
                                        .show();
                                });
                        }
                    });
                });
        })
        .run();
}
