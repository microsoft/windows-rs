#![windows_subsystem = "windows"]

use windows_reactor::{
    Border, CollectionSelection, Color, Element, RenderCx, TextBlock, Thickness, VirtualGrid,
    VirtualItemKeys, vstack,
};

const ITEMS: [(u64, &str, Color); 6] = [
    (10, "Red", Color::rgb(245, 205, 205)),
    (20, "Green", Color::rgb(205, 235, 205)),
    (30, "Blue", Color::rgb(205, 220, 245)),
    (40, "Yellow", Color::rgb(245, 235, 190)),
    (50, "Magenta", Color::rgb(235, 205, 235)),
    (60, "Cyan", Color::rgb(195, 235, 235)),
];

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let selection = cx.use_state(CollectionSelection::default);
    let current = selection.value();
    let label = current
        .as_slice()
        .first()
        .and_then(|key| ITEMS.iter().find(|item| item.0 == *key))
        .map_or_else(
            || "No selection".to_string(),
            |item| format!("Selected: {}", item.1),
        );
    vstack(
        8.0,
        [
            VirtualGrid::new(ITEMS.len(), 220.0, |index| {
                let (_, name, color) = ITEMS[index];
                Border::new(TextBlock::new(name).build())
                    .background(color)
                    .padding(Thickness::uniform(20.0))
                    .min_width(110.0)
                    .build()
            })
            .item_keys(VirtualItemKeys::new(ITEMS.iter().map(|item| item.0)))
            .selection(current, move |value| {
                selection.set(value);
            })
            .automation_name("Color grid")
            .build(),
            TextBlock::new(label).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("GridView", app)
}
