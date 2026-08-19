#![windows_subsystem = "windows"]

use windows_reactor::{CollectionSelection, Element, ListBox, RenderCx, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let selected = cx.use_state(|| None::<u64>);
    let current = selected.value();
    let label = current.map_or_else(
        || "No selection".to_string(),
        |key| format!("Selected key: {key}"),
    );

    vstack(
        8.0,
        [
            ListBox::new(
                [
                    (10, "Apple"),
                    (20, "Banana"),
                    (30, "Cherry"),
                    (40, "Date"),
                    (50, "Elderberry"),
                ],
                move |value| {
                    selected.set(value.as_slice().first().copied());
                },
            )
            .selection(CollectionSelection::new(current))
            .build(),
            TextBlock::new(label).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ListBox", app)
}
