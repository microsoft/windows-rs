#![windows_subsystem = "windows"]

use std::time::Duration;

use windows_reactor::{
    Border, Button, Color, CornerRadius, Element, RenderCx, TextBlock, Thickness, fade_transition,
    fragment, vstack,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let visible = cx.use_state(|| true);
    let current = visible.value();
    let toggle = visible;
    let card = if current {
        Border::new(
            TextBlock::new("This visual remains visible while its exit animation completes.")
                .font_size(18.0)
                .foreground(Color::rgb(255, 255, 255))
                .build(),
        )
        .padding(Thickness::uniform(24.0))
        .background(Color::rgb(32, 96, 160))
        .corner_radius(CornerRadius::uniform(12.0))
        .build()
    } else {
        fragment([])
    };

    Border::new(vstack(
        16.0,
        [
            Button::new(if current { "Remove" } else { "Restore" })
                .on_click(move || {
                    toggle.set(!current);
                })
                .build(),
            fade_transition(
                card,
                Some(Duration::from_millis(300)),
                Some(Duration::from_millis(600)),
            ),
        ],
    ))
    .padding(Thickness::uniform(24.0))
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Exit Transition", app)
}
