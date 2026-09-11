use zenthra::prelude::*;

struct AppTheme {
    primary_color: Color,
}

fn main() {
    // 1. Reactive Signals
    let count = Signal::new(0);
    let step = Signal::new(1);

    // 2. Computed Derived Reactive State
    let double_count = Computed::new({
        let count = count.clone();
        move || count.get() * 2
    });

    let description = Computed::new({
        let count = count.clone();
        let step = step.clone();
        move || format!("Current Count: {} (Step: {})", count.get(), step.get())
    });

    // 3. Side-effect runner
    let _effect = Effect::run({
        let count = count.clone();
        move || {
            log::info!("[Reactive Effect] Count changed to: {}", count.get());
        }
    });

    // 4. Provide reactive theme via Context Provider
    provide_context(AppTheme {
        primary_color: Color::rgb(0.2, 0.6, 0.9),
    });

    App::new()
        .title("Zenthra v2 - Reactive State Management Demo")
        .size(700, 500)
        .with_ui(move |ui| {
            let theme = use_context::<AppTheme>().expect("AppTheme missing");

            ui.container()
                .fill()
                .bg(Color::rgb(0.06, 0.07, 0.10))
                .padding_all(40.0)
                .show(|ui| {
                    ui.h1("Zenthra Dynamic Reactive State")
                        .color(theme.primary_color)
                        .show();
                    ui.spacing(15.0);

                    ui.text(&description.get())
                        .color(Color::WHITE)
                        .size(18.0)
                        .show();

                    ui.spacing(10.0);

                    ui.text(&format!("Double Count (Computed): {}", double_count.get()))
                        .color(Color::rgba(1.0, 1.0, 1.0, 0.7))
                        .size(16.0)
                        .show();

                    ui.spacing(25.0);

                    ui.container().row().gap(15.0).show(|ui| {
                        if ui.button(" + Step ")
                            .bg(Color::rgb(0.2, 0.4, 0.7))
                            .radius_all(6.0)
                            .padding_all(10.0)
                            .show()
                            .clicked
                        {
                            let s = step.get();
                            count.update(|c| c + s);
                        }

                        if ui.button(" - Step ")
                            .bg(Color::rgb(0.7, 0.3, 0.3))
                            .radius_all(6.0)
                            .padding_all(10.0)
                            .show()
                            .clicked
                        {
                            let s = step.get();
                            count.update(|c| c - s);
                        }

                        if ui.button("Batch Update (+10, Step=5)")
                            .bg(Color::rgb(0.3, 0.6, 0.4))
                            .radius_all(6.0)
                            .padding_all(10.0)
                            .show()
                            .clicked
                        {
                            let count = count.clone();
                            let step = step.clone();
                            batch(move || {
                                step.set(5);
                                count.update(|c| c + 10);
                            });
                        }

                        if ui.button("Reset")
                            .bg(Color::rgb(0.3, 0.3, 0.35))
                            .radius_all(6.0)
                            .padding_all(10.0)
                            .show()
                            .clicked
                        {
                            let count = count.clone();
                            let step = step.clone();
                            batch(move || {
                                count.set(0);
                                step.set(1);
                            });
                        }
                    });
                });
        })
        .run();
}
