use crate::controls::*;
use windows_reactor::*;

pub fn toggle_button_page(_: &(), cx: &mut RenderCx) -> Element {
    let (bold, set_bold) = cx.use_state(false);
    let (italic, set_italic) = cx.use_state(false);

    page_content(
        "ToggleButton",
        "A button that toggles between two states.",
        vec![
            sample_card(
                "Text Formatting Toggles",
                vstack((
                    hstack((
                        toggle_button("Bold", bold).on_changed({
                            let set_bold = set_bold;
                            move |v: bool| set_bold.call(v)
                        }),
                        toggle_button("Italic", italic)
                            .on_changed(move |v: bool| set_italic.call(v)),
                    ))
                    .spacing(8.0),
                    text_block(format!(
                        "Bold: {}, Italic: {}",
                        if bold { "ON" } else { "OFF" },
                        if italic { "ON" } else { "OFF" }
                    ))
                    .opacity(0.6),
                ))
                .spacing(8.0),
                r#"toggle_button("Bold", toggled).on_changed(handler)"#,
            ),
            sample_card(
                "Disabled ToggleButton",
                toggle_button("Locked", true).enabled(false),
                r#"toggle_button("Locked", true).enabled(false)"#,
            ),
        ],
    )
}
