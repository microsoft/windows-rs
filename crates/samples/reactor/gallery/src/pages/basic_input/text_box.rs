use crate::controls::*;
use windows_reactor::*;

pub fn text_box_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (text, set_text) = cx.use_state(String::new());
    let (multi_text, set_multi) = cx.use_state(String::from("Hello\nWorld"));

    page_content(
        "TextBox",
        "A single-line or multi-line text input field.",
        vec![
            sample_card(
                "Basic TextBox",
                vstack((
                    text_box(&text)
                        .header("Name")
                        .placeholder("Type here...")
                        .on_changed({
                            let set_text = set_text;
                            move |s: String| set_text.call(s)
                        }),
                    text_block(format!("Characters: {}", text.len())).opacity(0.6),
                ))
                .spacing(8.0),
                r#"text_box(&text).header("Name").placeholder("Type here...").on_changed(handler)"#,
            ),
            sample_card(
                "Multi-line TextBox",
                text_box(&multi_text)
                    .header("Notes")
                    .multiline()
                    .on_changed(move |s: String| set_multi.call(s))
                    .height(120.0),
                r#"text_box(&text).header("Notes").multiline()"#,
            ),
            sample_card(
                "Disabled TextBox",
                text_box("Read-only content")
                    .header("Status")
                    .enabled(false),
                r#"text_box("content").enabled(false)"#,
            ),
        ],
    )
}
