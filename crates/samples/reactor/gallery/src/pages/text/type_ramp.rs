use crate::controls::*;
use windows_reactor::*;

pub fn type_ramp_page(_: &(), _cx: &mut RenderCx) -> Element {
    page_content(
        "Type Ramp",
        "Named text factories for the WinUI 3 type ramp.",
        vec![
            sample_card(
                "Type Ramp Factories",
                vstack((
                    title("Title — 28px Semibold"),
                    subtitle("Subtitle — 20px Semibold"),
                    body_large("BodyLarge — 18px Regular"),
                    body_strong("BodyStrong — 14px Semibold"),
                    body("Body — 14px Regular"),
                    caption("Caption — 12px Regular"),
                ))
                .spacing(8.0),
                r#"title("Title — 28px Semibold")
subtitle("Subtitle — 20px Semibold")
body_large("BodyLarge — 18px Regular")
body_strong("BodyStrong — 14px Semibold")
body("Body — 14px Regular")
caption("Caption — 12px Regular")"#,
            ),
            sample_card(
                "Custom Font Size",
                vstack((
                    text_block("Small text").font_size(10.0),
                    text_block("Normal text").font_size(14.0),
                    text_block("Large text").font_size(24.0),
                    text_block("Huge text").font_size(36.0),
                ))
                .spacing(4.0),
                r#"text_block("Small").font_size(10.0)
text_block("Normal").font_size(14.0)
text_block("Large").font_size(24.0)
text_block("Huge").font_size(36.0)"#,
            ),
            sample_card(
                "Composed Example — Article Card",
                border(
                    vstack((
                        title("Release Notes"),
                        subtitle("Version 2.5"),
                        body_strong("New update available"),
                        body("Version 2.5 ships performance improvements and several bug fixes."),
                        body("Restart the app to apply."),
                    ))
                    .spacing(8.0),
                )
                .padding(16.0)
                .corner_radius(8.0),
                r#"border(vstack((
    title("Release Notes"),
    subtitle("Version 2.5"),
    body_strong("New update available"),
    body("...description...")
)).spacing(8.0).into()).padding(16.0)"#,
            ),
        ],
    )
}
