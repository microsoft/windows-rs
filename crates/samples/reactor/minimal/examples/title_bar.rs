//! Minimal sample for the `TitleBar` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (back_clicks, set_back) = cx.use_state(0i32);
    let (pane_clicks, set_pane) = cx.use_state(0i32);

    vstack((
        TitleBar::new("windows_reactor — title_bar sample")
            .subtitle("Minimal demo")
            .back_button_visible(true)
            .back_button_enabled(true)
            .pane_toggle_button_visible(true)
            .on_back_requested(move || set_back.call(back_clicks + 1))
            .on_pane_toggle_requested(move || set_pane.call(pane_clicks + 1)),
        text_block(format!(
            "back_clicks = {back_clicks}, pane_toggle_clicks = {pane_clicks}"
        )),
    ))
    .spacing(8.0)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
