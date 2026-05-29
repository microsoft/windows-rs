//! Minimal sample for `with_scale_transition`.
//!
//! Registers a Composition `Vector3Transition` on scale so subsequent
//! scale changes tween smoothly. Scale has no direct setter on the DSL;
//! it is driven by a one-shot property animation via `.animate(..)`.

use std::time::Duration;

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (big, set_big) = cx.use_state(false);

    let toggle = move || set_big.call(!big);

    let target_scale = if big { 1.3 } else { 1.0 };

    let swatch = border(
        text_block("Animated content")
            .font_size(18.0)
            .foreground(Color::rgb(255, 255, 255))
            .padding(Thickness::uniform(20.0)),
    )
    .background(Color::rgb(70, 130, 200))
    .with_scale_transition(Duration::from_millis(1000))
    .animate(AnimationConfig {
        scale: Some(target_scale),
        duration: Duration::from_millis(1000),
        easing: Easing::EaseOut,
        ..AnimationConfig::default()
    })
    .max_width(280.0);

    vstack((
        text_block("Toggle the button to drive scale changes through an implicit transition."),
        button(if big { "Scale down" } else { "Scale up" }).on_click(toggle),
        swatch,
    ))
    .spacing(12.0)
}

fn main() -> Result<()> {
    App::new().title("Scale transition").render(app)
}
