#![windows_subsystem = "windows"]

use windows_reactor::{
    Button, Element, FontWeight, RenderCx, StackPanel, TextBlock, Thickness, memo_component, vstack,
};

#[derive(Clone, PartialEq)]
struct GreetingProps {
    name: String,
    clicks: u32,
}

fn greeting(props: &GreetingProps) -> Element {
    vstack(
        4.0,
        [
            TextBlock::new(format!("Hello, {}!", props.name))
                .font_weight(FontWeight::BOLD)
                .font_size(20.0)
                .build(),
            TextBlock::new(format!(
                "You have clicked the button {} times.",
                props.clicks
            ))
            .build(),
        ],
    )
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let clicks = cx.use_state(|| 0_u32);
    let current = clicks.value();
    let props = GreetingProps {
        name: "world".to_string(),
        clicks: current,
    };
    let render_props = props.clone();

    StackPanel::new([
        memo_component(props, move |_| greeting(&render_props)),
        Button::new("Click me")
            .on_click(move || {
                clicks.update(|value| *value += 1);
            })
            .build(),
    ])
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ComponentProps", app)
}
