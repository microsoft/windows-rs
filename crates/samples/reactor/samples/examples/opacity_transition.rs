#![windows_subsystem = "windows"]

use std::time::Duration;

use windows_reactor::{Border, Button, Color, Element, RenderCx, TextBlock, Thickness, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let visible = cx.use_state(|| true);
    let current = visible.value();
    let toggle = visible;

    let swatch = Border::new(
        TextBlock::new("Animated content")
            .padding(Thickness::uniform(20.0))
            .font_size(18.0)
            .foreground(Color::rgb(255, 255, 255))
            .build(),
    )
    .background(Color::rgb(70, 130, 200))
    .opacity(if current { 1.0 } else { 0.2 })
    .opacity_transition(Some(Duration::from_millis(1000)))
    .max_width(280.0)
    .build();

    vstack(
        12.0,
        [
            TextBlock::new("Toggle to drive opacity through an implicit transition.").build(),
            Button::new(if current { "Fade out" } else { "Fade in" })
                .on_click(move || {
                    toggle.set(!current);
                })
                .build(),
            swatch,
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Opacity Transition", app)
}
