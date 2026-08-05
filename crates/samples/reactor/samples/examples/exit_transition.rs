#![windows_subsystem = "windows"]

use std::time::Duration;

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (visible, set_visible) = cx.use_state(true);
    let card: Element = if visible {
        border(
            text_block("This visual remains visible while its exit animation completes.")
                .font_size(18.0),
        )
        .padding(Thickness::uniform(24.0))
        .background(Color::rgb(32, 96, 160))
        .corner_radius(12.0)
        .transition(
            Some(AnimationConfig::fade_in(Duration::from_millis(300))),
            Some(AnimationConfig::fade_out(Duration::from_millis(600))),
        )
        .into()
    } else {
        Element::Empty
    };

    vstack((
        button(if visible { "Remove" } else { "Restore" })
            .on_click(move || set_visible.call(!visible)),
        card,
    ))
    .spacing(16.0)
    .padding(Thickness::uniform(24.0))
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Exit Transition", app)
}
