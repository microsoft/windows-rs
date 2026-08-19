#![windows_subsystem = "windows"]

use windows_reactor::{ComboBox, Element, RenderCx, StackPanel, TextBlock};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let selected = cx.use_state(|| None::<u64>);
    let animal = cx.use_state(|| None::<u64>);
    let current = selected.value();
    let current_animal = animal.value();
    let label = match current {
        Some(1) => "Red",
        Some(2) => "Green",
        Some(3) => "Blue",
        _ => "(none)",
    };

    StackPanel::new([
        ComboBox::new([(1, "Red"), (2, "Green"), (3, "Blue")], move |value| {
            selected.set(value);
        })
        .header("Color")
        .placeholder_text("Pick a color")
        .selected_key(current)
        .build(),
        TextBlock::new(format!("selected = {current:?} ({label})")).build(),
        ComboBox::new([(10, "Cat"), (20, "Dog"), (30, "Fox")], move |value| {
            animal.set(value);
        })
        .header("Editable")
        .placeholder_text("Type or pick an animal")
        .editable(true)
        .selected_key(current_animal)
        .build(),
        ComboBox::display([(100, "A"), (200, "B"), (300, "C")])
            .header("Disabled")
            .selected_key(Some(100))
            .enabled(false)
            .build(),
    ])
    .spacing(8.0)
    .max_width(320.0)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ComboBox", app)
}
