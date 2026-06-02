//! Minimal sample for the `Flyout` modifier on `Button`.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0_u32);

    let bump = move || set_count.call(count + 1);

    vstack((
        button("Show Flyout").flyout("Hello from the flyout!"),
        button("Bottom Flyout")
            .flyout_with_placement(format!("Clicked {count} times"), FlyoutPlacement::Bottom),
        button("Increment").on_click(bump),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = windows_reactor::bootstrap::initialize()?;
    App::new().title("Sample").render(app)
}
