//! Minimal sample for passing typed props to a child component.

#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, PartialEq)]
struct GreetingProps {
    name: String,
    clicks: u32,
}

fn greeting(props: &GreetingProps, _cx: &mut RenderCx) -> Element {
    let GreetingProps { name, clicks } = props;
    vstack((
        text_block(format!("Hello, {name}!")).bold().font_size(20.0),
        text_block(format!("You have clicked the button {clicks} times.")),
    ))
    .spacing(4.0)
    .into()
}

fn app(cx: &mut RenderCx) -> Element {
    let (name, _set_name) = cx.use_state(String::from("world"));
    let (clicks, set_clicks) = cx.use_state(0_u32);

    let bump = move || set_clicks.call(clicks + 1);

    vstack((
        TitleBar::new("windows_reactor — component props"),
        component(greeting, GreetingProps { name, clicks }),
        button("Click me").on_click(bump),
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new()
        .title("windows_reactor — component_props")
        .render(app)
}
