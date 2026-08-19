#![windows_subsystem = "windows"]

use windows_reactor::{Element, Orientation, RenderCx, Slider, StackPanel, TextBlock};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let volume = cx.use_state(|| 35.0_f64);
    let vertical = cx.use_state(|| 50.0_f64);
    let current = volume.value();
    let current_vertical = vertical.value();

    StackPanel::new([
        Slider::new(current, move |value| {
            volume.set(value);
        })
        .range(0.0, 100.0)
        .step(1.0)
        .header("Volume")
        .build(),
        TextBlock::new(format!("Volume = {current:.0}")).build(),
        Slider::new(current_vertical, move |value| {
            vertical.set(value);
        })
        .range(0.0, 100.0)
        .header("Vertical")
        .orientation(Orientation::Vertical)
        .height(120.0)
        .build(),
        Slider::display(50.0)
            .range(0.0, 100.0)
            .header("Disabled")
            .build(),
    ])
    .spacing(8.0)
    .max_width(320.0)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Slider", app)
}
