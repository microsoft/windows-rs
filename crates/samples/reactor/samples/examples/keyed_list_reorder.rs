#![windows_subsystem = "windows"]

use windows_reactor::{
    Button, Element, RenderCx, TextBlock, VirtualItemKeys, VirtualList, component, hstack, vstack,
};

fn item_name(key: u64) -> &'static str {
    match key {
        10 => "Alpha",
        20 => "Beta",
        30 => "Gamma",
        _ => unreachable!(),
    }
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let keys = cx.use_state(|| VirtualItemKeys::new([10, 20, 30]));
    let current = keys.value();
    let order = current
        .as_slice()
        .iter()
        .map(|key| item_name(*key))
        .collect::<Vec<_>>()
        .join(", ");
    let row_keys = current.clone();

    vstack(
        8.0,
        [
            TextBlock::new(format!("Order: {order}")).build(),
            Button::new("Rotate rows")
                .on_click(move || {
                    keys.update(|keys| {
                        let mut values = keys.as_slice().to_vec();
                        values.rotate_left(1);
                        *keys = VirtualItemKeys::new(values);
                    });
                })
                .build(),
            VirtualList::new(current.len(), 240.0, move |index| {
                let key = row_keys.as_slice()[index];
                component(move |cx| {
                    let clicks = cx.use_state(|| 0u32);
                    let count = clicks.value();
                    let name = item_name(key);
                    hstack(
                        8.0,
                        [
                            TextBlock::new(format!("{name}: {count}")).build(),
                            Button::new(format!("Increment {name}"))
                                .on_click(move || {
                                    clicks.update(|value| *value += 1);
                                })
                                .build(),
                        ],
                    )
                })
            })
            .item_keys(current)
            .automation_name("Keyed rows")
            .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("KeyedListReorder", app)
}
