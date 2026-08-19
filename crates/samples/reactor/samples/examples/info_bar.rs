#![windows_subsystem = "windows"]

use windows_reactor::{Button, Element, InfoBar, RenderCx, TextBlock, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let open = cx.use_state(|| true);
    let status = cx.use_state(|| "Open".to_string());
    let current_open = open.value();
    let current_status = status.value();
    let show = open.clone();
    let show_status = status.clone();
    let close = open;
    let close_status = status;

    vstack(
        8.0,
        [
            Button::new("Show InfoBar")
                .on_click(move || {
                    show.set(true);
                    show_status.set("Open".to_string());
                })
                .automation_id("show-info-bar")
                .build(),
            InfoBar::new("Did you know?")
                .message("This close button requests a declarative state change.")
                .open(current_open)
                .on_close_requested(move || {
                    close.set(false);
                    close_status.set("Close requested".to_string());
                })
                .automation_id("controlled-info-bar")
                .build(),
            TextBlock::new(format!("Status: {current_status}"))
                .automation_id("info-bar-status")
                .build(),
            InfoBar::new("Saved")
                .message("Your changes have been saved.")
                .success()
                .closable(false)
                .build(),
            InfoBar::new("Heads up")
                .message("Check before proceeding.")
                .warning()
                .closable(false)
                .build(),
            InfoBar::new("Something went wrong")
                .message("The operation could not be completed.")
                .error()
                .closable(false)
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("InfoBar", app)
}
