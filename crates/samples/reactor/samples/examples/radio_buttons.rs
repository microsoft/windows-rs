#![windows_subsystem = "windows"]

use windows_reactor::{Element, RadioButtons, RenderCx, TextBlock, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let selected = cx.use_state(|| 0_u64);
    let current = selected.value();
    let options = ["Email", "SMS", "None"];
    let label = options.get(current as usize).copied().unwrap_or("(none)");

    let selection = selected;
    vstack(
        8.0,
        [
            RadioButtons::new([(0, "Email"), (1, "SMS"), (2, "None")], move |value| {
                if let Some(key) = value {
                    selection.set(key);
                }
            })
            .header("Notifications")
            .selected_key(Some(current))
            .max_columns(3)
            .build(),
            TextBlock::new(format!("selected = {current} ({label})")).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("RadioButtons", app)
}
