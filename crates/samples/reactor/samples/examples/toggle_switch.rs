//! Sample for the `ToggleSwitch` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (on, set_on) = cx.use_state(true);

    let toggle = move |v| set_on.call(v);

    vstack((
        ToggleSwitch::new(on)
            .header("Notifications")
            .on_content("On")
            .off_content("Off")
            .on_toggled(toggle),
        text_block(if on {
            "Notifications enabled"
        } else {
            "Notifications muted"
        }),
        ToggleSwitch::new(true)
            .header("Disabled (always on)")
            .enabled(false),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("ToggleSwitch", app)
}
