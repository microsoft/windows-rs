//! Sample for the `DropDownButton` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (clicks, set_clicks) = cx.use_state(0_u32);

    let bump = move || set_clicks.call(clicks + 1);

    vstack((
        drop_down_button("Options").on_click(bump),
        text_block(format!("Clicked {clicks} time(s)")),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("DropDownButton", app)
}
