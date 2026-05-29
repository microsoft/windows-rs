//! Minimal sample for the `SplitButton` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (clicks, set_clicks) = cx.use_state(0_u32);

    let bump = move || set_clicks.call(clicks + 1);

    vstack((
        split_button(format!("Primary action ({clicks})")).on_click(bump),
        split_button("Disabled").enabled(false),
    ))
    .spacing(8.0)
}

fn main() -> Result<()> {
    App::new().title("SplitButton Sample").render(app)
}
