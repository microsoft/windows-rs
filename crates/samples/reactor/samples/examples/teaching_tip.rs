#![windows_subsystem = "windows"]

use windows_reactor::{Button, Element, RenderCx, TeachingTip, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let open = cx.use_state(|| false);
    let status = cx.use_state(|| "(tip closed)".to_string());
    let current_open = open.value();
    let current_status = status.value();
    let show = open.clone();
    let close = open;
    let action_status = status.clone();
    let close_status = status;

    let owner = Button::new("Show Teaching Tip")
        .on_click(move || {
            show.set(true);
        })
        .build()
        .teaching_tip(
            TeachingTip::new("Welcome!")
                .subtitle("This teaching tip has action and close buttons.")
                .open(current_open)
                .light_dismiss(true)
                .action_button("Got it")
                .close_button("Dismiss")
                .on_action_button_click(move || {
                    action_status.set("Action button clicked".to_string());
                })
                .on_closed(move || {
                    close.set(false);
                    close_status.set("Tip was closed".to_string());
                }),
        );

    vstack(
        12.0,
        [
            owner,
            TextBlock::new(format!("Status: {current_status}"))
                .automation_id("teaching-tip-status")
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("TeachingTip", app)
}
