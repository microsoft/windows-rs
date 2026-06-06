//! Sample for the `CheckBox` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (checked, set_checked) = cx.use_state(false);

    let toggle = move |v| set_checked.call(v);

    vstack((
        check_box(checked)
            .content("I accept the terms")
            .on_changed(toggle),
        text_block(if checked {
            "Accepted ✓"
        } else {
            "Not yet accepted"
        }),
        check_box(true)
            .content("Disabled (always on)")
            .enabled(false),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("CheckBox", app)
}
