//! Minimal sample for the `Button` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (clicks, set_clicks) = cx.use_state(0_u32);

    let bump = move || set_clicks.call(clicks + 1);

    vstack((
        button(format!("Clicked {clicks} times")).on_click(bump),
        button("Disabled").enabled(false),
        button("Accent (Primary Action)").accent(),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
