#![windows_subsystem = "windows"]

use windows_reactor::*;

#[derive(Clone, PartialEq)]
struct RowProps {
    name: String,
}

fn row(props: &RowProps, cx: &mut RenderCx) -> Element {
    let (clicks, set_clicks) = cx.use_state(0_u32);

    hstack((
        text_block(format!("{}: {clicks}", props.name)).width(120.0),
        button(format!("Increment {}", props.name)).on_click(move || set_clicks.call(clicks + 1)),
    ))
    .spacing(8.0)
    .padding(Thickness::uniform(6.0))
    .into()
}

fn app(cx: &mut RenderCx) -> Element {
    let (items, set_items) = cx.use_state(vec![
        "Alpha".to_string(),
        "Beta".to_string(),
        "Gamma".to_string(),
        "Delta".to_string(),
    ]);
    let shuffled = {
        let mut items = items.clone();
        items.rotate_left(1);
        items
    };

    vstack((
        text_block("Increment a row, then rotate the list. The count stays with its name."),
        button("Rotate").on_click(move || set_items.call(shuffled.clone())),
        list_view(items, |name, _| {
            component(row, RowProps { name: name.clone() })
        })
        .with_key_selector(|name| name.clone())
        .height(240.0),
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("KeyedListReorder", app)
}
