use crate::controls::*;
use windows_reactor::*;

pub fn hyperlink_button_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (clicks, set_clicks) = cx.use_state(0_i32);

    page_content(
        "HyperlinkButton",
        "A button that appears as a hyperlink and navigates to a URI.",
        vec![
            sample_card(
                "Navigate to URI",
                HyperlinkButton::new("Visit Microsoft").navigate_uri("https://www.microsoft.com"),
                r#"HyperlinkButton::new("Visit").navigate_uri("https://...")"#,
            ),
            sample_card(
                "HyperlinkButton with Click Handler",
                vstack((
                    HyperlinkButton::new("Click me (no navigation)")
                        .on_click(move || set_clicks.call(clicks + 1)),
                    text_block(format!("Clicked: {clicks} times")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"HyperlinkButton::new("Click me").on_click(handler)"#,
            ),
        ],
    )
}
