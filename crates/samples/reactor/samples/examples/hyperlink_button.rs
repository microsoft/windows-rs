#![windows_subsystem = "windows"]

use windows_reactor::{Element, HyperlinkButton, RenderCx, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let clicks = cx.use_state(|| 0_u32);
    let current = clicks.value();

    vstack(
        8.0,
        [
            HyperlinkButton::new("Open Microsoft Docs")
                .navigate_uri("https://learn.microsoft.com/windows/apps/")
                .build(),
            HyperlinkButton::new(format!("Clicked {current} times"))
                .on_click(move || {
                    clicks.update(|value| *value += 1);
                })
                .build(),
            HyperlinkButton::new("Disabled")
                .navigate_uri("https://example.com/")
                .enabled(false)
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("HyperlinkButton", app)
}
