//! Sample for the `HyperlinkButton` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (clicks, set_clicks) = cx.use_state(0_u32);

    let bump = move || set_clicks.call(clicks + 1);

    vstack((
        HyperlinkButton::new("Open Microsoft Docs")
            .navigate_uri("https://learn.microsoft.com/windows/apps/"),
        HyperlinkButton::new(format!("Clicked {clicks} times")).on_click(bump),
        HyperlinkButton::new("Disabled")
            .navigate_uri("https://example.com/")
            .enabled(false),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("HyperlinkButton", app)
}
