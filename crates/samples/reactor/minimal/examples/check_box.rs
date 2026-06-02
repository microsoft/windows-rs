//! Minimal sample for the `CheckBox` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (checked, set_checked) = cx.use_state(false);

    let toggle = move |v| set_checked.call(v);

    vstack((
        check_box(checked)
            .label("I accept the terms")
            .on_changed(toggle),
        text_block(if checked {
            "Accepted ✓"
        } else {
            "Not yet accepted"
        }),
        check_box(true).label("Disabled (always on)").enabled(false),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new().title("Sample").render(app)
}
