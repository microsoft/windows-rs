//! Sample for the `Button` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (clicks, set_clicks) = cx.use_state(0_u32);

    let bump = set_clicks.setter(clicks + 1);

    vstack((
        button(format!("Clicked {clicks} times")).on_click(bump),
        button("Disabled").enabled(false),
        button("Accent (Primary Action)").accent(),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Button", app)
}
