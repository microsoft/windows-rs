use crate::controls::*;
use windows_reactor::*;

pub fn rich_edit_box_page(_: &(), cx: &mut RenderCx) -> Element {
    let (editor_text, set_editor_text) = cx.use_state(String::new());

    page_content(
        "RichEditBox",
        "A rich text editing control with formatting support.",
        vec![sample_card(
            "Basic RichEditBox",
            vstack((
                rich_edit_box(&editor_text)
                    .header("Document")
                    .placeholder("Start typing...")
                    .on_changed({
                        let set_editor_text = set_editor_text;
                        move |text: String| set_editor_text.call(text)
                    })
                    .height(200.0),
                text_block(if editor_text.is_empty() {
                    String::from("No changes yet")
                } else {
                    format!("Modified • {} characters", editor_text.chars().count())
                })
                .opacity(0.6),
            ))
            .spacing(8.0),
            r#"rich_edit_box(&editor_text)
    .header(\"Document\")
    .placeholder(\"Start typing...\")
    .on_changed(|text| set_editor_text.call(text))"#,
        )],
    )
    .into()
}
