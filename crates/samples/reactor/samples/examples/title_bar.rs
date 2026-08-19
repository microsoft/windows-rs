#![windows_subsystem = "windows"]

use windows_reactor::{
    Application, Button, Element, RenderCx, TitleBar, TitleBarHeight, Window, text_block, vstack,
};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let open = cx.use_state(|| true);
    let back_clicks = cx.use_state(|| 0);
    let pane_clicks = cx.use_state(|| 0);
    let custom = cx.use_state(|| true);
    let windows = if open.value() {
        let update_back = back_clicks.clone();
        let update_pane = pane_clicks.clone();
        let toggle_custom = custom.clone();
        let window = Window::new(
            "TitleBar",
            vstack(
                8.0,
                [
                    text_block(format!(
                        "back_clicks = {}, pane_toggle_clicks = {}",
                        back_clicks.value(),
                        pane_clicks.value()
                    )),
                    Button::new(if custom.value() {
                        "Use system title bar"
                    } else {
                        "Use custom title bar"
                    })
                    .on_click(move || {
                        toggle_custom.set(!toggle_custom.value());
                    })
                    .build(),
                ],
            ),
            move || {
                open.set(false);
            },
        );
        let window = if custom.value() {
            window.title_bar(
                TitleBar::custom("windows-reactor - title_bar sample")
                    .subtitle(Some("Window-owned custom chrome".to_string()))
                    .back_button_visible(true)
                    .back_button_enabled(true)
                    .pane_toggle_button_visible(true)
                    .height(TitleBarHeight::Tall)
                    .on_back_requested(move || {
                        update_back.set(update_back.value() + 1);
                    })
                    .on_pane_requested(move || {
                        update_pane.set(update_pane.value() + 1);
                    }),
            )
        } else {
            window
        };
        vec![window.client_size(640.0, 360.0).build().key(0)]
    } else {
        Vec::new()
    };
    Application::new(windows).build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run_application(app)
}
