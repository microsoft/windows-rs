#![windows_subsystem = "windows"]

use windows_reactor::{Border, Color, Element, RenderCx, TextBlock, Thickness, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let position = cx.use_state(|| None::<(f32, f32)>);
    let current = position.value();
    let label = match current {
        Some((x, y)) => format!("Pressed at ({x:.0}, {y:.0})"),
        None => "Click anywhere in the box".to_string(),
    };

    vstack(
        12.0,
        [
            TextBlock::new(label)
                .font_size(20.0)
                .automation_id("pointer-label")
                .build(),
            Border::new(
                TextBlock::new("Click to read the pointer position")
                    .foreground(Color::rgb(255, 255, 255))
                    .build(),
            )
            .background(Color::rgb(40, 120, 200))
            .padding(Thickness::uniform(40.0))
            .width(360.0)
            .height(240.0)
            .on_pointer_pressed(move |event| {
                position.set(Some((event.x, event.y)));
            })
            .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Pointer Position", app)
}
