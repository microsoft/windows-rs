#![windows_subsystem = "windows"]

use windows_reactor::{
    CollectionSelection, Element, FontWeight, RenderCx, SelectionMode, StackPanel, TextBlock,
    Thickness, VirtualItemKeys, VirtualList,
};

#[derive(Clone)]
struct ColorItem {
    key: u64,
    name: &'static str,
}

const MODES: [(u64, &str, SelectionMode); 4] = [
    (100, "None", SelectionMode::None),
    (200, "Single", SelectionMode::Single),
    (300, "Multiple", SelectionMode::Multiple),
    (400, "Extended", SelectionMode::Extended),
];

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let mode = cx.use_state(|| SelectionMode::Single);
    let selection = cx.use_state(CollectionSelection::default);
    let items = cx.use_state(|| {
        vec![
            ColorItem {
                key: 10,
                name: "Red",
            },
            ColorItem {
                key: 20,
                name: "Green",
            },
            ColorItem {
                key: 30,
                name: "Blue",
            },
            ColorItem {
                key: 40,
                name: "Yellow",
            },
            ColorItem {
                key: 50,
                name: "Magenta",
            },
        ]
    });

    let current_mode = mode.value();
    let current_selection = selection.value();
    let current_items = items.value();
    let selected_names = current_items
        .iter()
        .filter(|item| current_selection.as_slice().contains(&item.key))
        .map(|item| item.name)
        .collect::<Vec<_>>();
    let selected_label = if selected_names.is_empty() {
        "(none)".to_string()
    } else {
        selected_names.join(", ")
    };
    let order = current_items
        .iter()
        .map(|item| item.name)
        .collect::<Vec<_>>()
        .join(", ");
    let mode_key = MODES
        .iter()
        .find(|entry| entry.2 == current_mode)
        .unwrap()
        .0;
    let mode_selection = mode;
    let controlled_selection = selection.clone();
    let rows = current_items.clone();
    let reordered_items = items;

    StackPanel::new([
        TextBlock::new("Selection Mode:")
            .font_weight(FontWeight::BOLD)
            .build(),
        VirtualList::new(MODES.len(), 120.0, |index| {
            TextBlock::new(MODES[index].1)
                .margin(Thickness::xy(12.0, 4.0))
                .build()
        })
        .item_keys(VirtualItemKeys::new(MODES.iter().map(|entry| entry.0)))
        .selection(CollectionSelection::new([mode_key]), move |value| {
            let Some(key) = value.as_slice().first() else {
                return;
            };
            let next = MODES.iter().find(|entry| entry.0 == *key).unwrap().2;
            mode_selection.set(next);
            controlled_selection.update(|selection| {
                if next == SelectionMode::None {
                    *selection = CollectionSelection::default();
                } else if next == SelectionMode::Single && selection.len() > 1 {
                    *selection = CollectionSelection::new(selection.as_slice().first().copied());
                }
            });
        })
        .automation_name("Selection modes")
        .build(),
        TextBlock::new("Items (drag to reorder):")
            .font_weight(FontWeight::BOLD)
            .build(),
        VirtualList::new(current_items.len(), 220.0, move |index| {
            let item = rows[index].clone();
            TextBlock::new(item.name)
                .margin(Thickness::xy(12.0, 6.0))
                .build()
        })
        .item_keys(VirtualItemKeys::new(
            current_items.iter().map(|item| item.key),
        ))
        .selection_mode(current_mode)
        .selection(current_selection, move |value| {
            selection.set(value);
        })
        .reorderable(move |keys| {
            reordered_items.update(|items| {
                assert_eq!(keys.len(), items.len());
                *items = keys
                    .into_iter()
                    .map(|key| items.iter().find(|item| item.key == key).unwrap().clone())
                    .collect();
            });
        })
        .automation_name("Reorderable colors")
        .build(),
        TextBlock::new(format!(
            "Selected: {selected_label} | Mode: {current_mode:?}"
        ))
        .build(),
        TextBlock::new(format!("Order: {order}")).build(),
    ])
    .spacing(8.0)
    .max_width(420.0)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ListView", app)
}
