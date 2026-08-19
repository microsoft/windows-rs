#![windows_subsystem = "windows"]

use windows_reactor::{
    Application, Element, FontWeight, StackPanel, TextBlock, Thickness, Window, button, component,
    hstack,
};

fn counter(heading: String) -> Element {
    component(move |cx| {
        let count = cx.use_state(|| 0_i32);
        let current = count.value();
        let decrement = count.clone();
        let increment = count;

        StackPanel::new([
            TextBlock::new(heading.clone())
                .font_weight(FontWeight::BOLD)
                .font_size(20.0)
                .build(),
            TextBlock::new(format!("Count: {current}"))
                .font_size(28.0)
                .build(),
            hstack(
                8.0,
                [
                    button("-", move || {
                        decrement.update(|value| *value -= 1);
                    }),
                    button("+", move || {
                        increment.update(|value| *value += 1);
                    }),
                ],
            ),
        ])
        .spacing(12.0)
        .padding(Thickness::uniform(24.0))
        .build()
    })
}

fn app(cx: &mut windows_reactor::RenderCx<'_>) -> Element {
    let window_ids = cx.use_state(|| vec![0_u64]);
    let opened = cx.use_state(|| 0_u64);
    let opened_count = opened.value();

    let windows = window_ids
        .value()
        .into_iter()
        .map(|id| {
            let content = if id == 0 {
                let ids_for_open = window_ids.clone();
                let opened_for_open = opened.clone();
                StackPanel::new([
                    TextBlock::new("Each window owns independent component state.").build(),
                    TextBlock::new("Closing the last remaining window exits the app.")
                        .opacity(0.75)
                        .build(),
                    button("Open counter window", move || {
                        let Some(next) = opened_for_open.try_value().map(|value| value + 1) else {
                            return;
                        };
                        opened_for_open.set(next);
                        ids_for_open.update(|ids| ids.push(next));
                    }),
                    TextBlock::new(format!("Windows opened: {opened_count}"))
                        .opacity(0.6)
                        .build(),
                ])
                .spacing(12.0)
                .padding(Thickness::uniform(24.0))
                .build()
            } else {
                counter(format!("Independent counter #{id}"))
            };

            let ids_for_close = window_ids.clone();
            Window::new(
                if id == 0 {
                    "Secondary windows".to_string()
                } else {
                    format!("Counter window #{id}")
                },
                content,
                move || {
                    ids_for_close.update(|ids| ids.retain(|candidate| *candidate != id));
                },
            )
            .client_size(420.0, 260.0)
            .build()
            .key(id)
        })
        .collect::<Vec<_>>();

    Application::new(windows).build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run_application(app)
}
