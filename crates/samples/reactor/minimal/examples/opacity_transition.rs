//! Minimal sample for `with_opacity_transition`.
//!
//! Registers a Composition transition so opacity changes tween smoothly.

use std::time::Duration;

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (visible, set_visible) = cx.use_state(true);

    let toggle = move || set_visible.call(!visible);

    let swatch = border(
        text_block("Animated content")
            .font_size(18.0)
            .foreground(Color::rgb(255, 255, 255))
            .padding(Thickness::uniform(20.0)),
    )
    .background(Color::rgb(70, 130, 200))
    .opacity(if visible { 1.0 } else { 0.2 })
    .with_opacity_transition(Duration::from_millis(1000))
    .max_width(280.0);

    vstack((
        text_block("Toggle to drive opacity through an implicit transition."),
        button(if visible { "Fade out" } else { "Fade in" }).on_click(toggle),
        swatch,
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new().title("Opacity transition").render(app)
}
