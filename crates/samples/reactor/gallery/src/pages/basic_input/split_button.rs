use crate::controls::*;
use windows_reactor::*;

pub fn split_button_page(_: &(), cx: &mut RenderCx) -> Element {
    let (clicks, set_clicks) = cx.use_state(0_u32);

    page_content(
        "SplitButton",
        "A button with a primary action and a flyout menu.",
        vec![
            sample_card(
                "Basic SplitButton",
                vstack((
                    split_button("Paste").on_click({
                        let set_clicks = set_clicks;
                        move || set_clicks.call(clicks + 1)
                    }),
                    text_block(format!("Primary clicked: {clicks} times")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"split_button("Paste").on_click(handler)"#,
            ),
            sample_card(
                "Disabled SplitButton",
                split_button("Disabled Action").enabled(false),
                r#"split_button("Disabled").enabled(false)"#,
            ),
        ],
    )
}
