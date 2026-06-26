//! Sample for function components via `component(f, props)`.

use windows_reactor::*;

/// A function component — no struct needed.
fn counter(_: &(), cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_i32);

    let bump = move || set_count.call(count + 1);

    vstack((
        text_block(format!("count = {count}"))
            .font_size(24.0)
            .bold(),
        button("Increment").on_click(bump),
    ))
    .spacing(8.0)
    .into()
}

/// A function component with typed props.
#[derive(Clone, PartialEq)]
struct GreetingProps {
    name: String,
}

fn greeting(props: &GreetingProps, _cx: &mut RenderCx) -> Element {
    text_block(format!("Hello, {}!", props.name))
        .font_size(20.0)
        .bold()
        .into()
}

/// Root component that composes both function components.
fn app(cx: &mut RenderCx) -> Element {
    let (name, set_name) = cx.use_state(String::from("world"));

    vstack((
        component(greeting, GreetingProps { name: name.clone() }),
        text_box(name)
            .header("Your name")
            .placeholder_text("Type a name…")
            .on_text_changed(set_name),
        component(counter, ()),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("FunctionComponent", app)
}
