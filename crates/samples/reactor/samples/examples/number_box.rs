#![windows_subsystem = "windows"]

use windows_reactor::{Element, NumberBox, RenderCx, StackPanel, TextBlock};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let quantity = cx.use_state(|| 3.0_f64);
    let current = quantity.value();

    StackPanel::new([
        NumberBox::new(current, move |value| {
            if let Some(value) = value {
                quantity.set(value);
            }
        })
        .range(0.0, 10.0)
        .header("Quantity")
        .build(),
        TextBlock::new(format!("Quantity = {current:.0}")).build(),
        NumberBox::display(42.0).header("Disabled").build(),
    ])
    .spacing(8.0)
    .max_width(320.0)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("NumberBox", app)
}
