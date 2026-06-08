//! Sample for the `TeachingTip` widget.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (is_open, set_is_open) = cx.use_state(false);
    let (status, set_status) = cx.use_state(String::from("(tip closed)"));

    vstack((
        button("Show Teaching Tip").on_click({
            let set_is_open = set_is_open.clone();
            move || set_is_open.call(true)
        }),
        text_block(format!("Status: {status}")),
        teaching_tip("Welcome!")
            .subtitle("This is a teaching tip with action and close buttons.")
            .is_open(is_open)
            .light_dismiss()
            .preferred_placement(TeachingTipPlacementMode::Bottom)
            .action_button("Got it")
            .close_button("Dismiss")
            .on_action_button_click({
                let set_status = set_status.clone();
                move || {
                    set_status.call(String::from("Action button clicked!"));
                }
            })
            .on_closed({
                let set_status = set_status;
                let set_is_open = set_is_open;
                move || {
                    set_is_open.call(false);
                    set_status.call(String::from("Tip was closed/dismissed"));
                }
            }),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("TeachingTip", app)
}
