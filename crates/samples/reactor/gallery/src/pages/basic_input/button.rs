use crate::controls::*;
use windows_reactor::*;

pub fn button_page(_: &(), cx: &mut RenderCx) -> Element {
    let (basic_output, set_basic_output) = cx.use_state(String::from("Ready"));
    let (accent_output, set_accent_output) = cx.use_state(String::from("Ready"));
    let (subtle_output, set_subtle_output) = cx.use_state(String::from("Ready"));
    let (link_output, set_link_output) = cx.use_state(String::from("Ready"));

    page_content(
        "Button",
        "A button that responds to user clicks and pointer input.",
        vec![
            sample_card(
                "Basic Button — .on_click(handler)",
                vstack((
                    button("Save").on_click({
                        let set = set_basic_output;
                        move || set.call(String::from("Saved!"))
                    }),
                    text_block(&*basic_output).foreground(ThemeRef::SecondaryText),
                ))
                .spacing(8.0),
                r#"button("Save").on_click(move || set_output.call("Saved!".into()))"#,
            ),
            sample_card(
                "Disabled Button",
                button("Can't Click").enabled(false),
                r#"button("Can't Click").enabled(false)"#,
            ),
            sample_card(
                "Accent style — .accent()",
                vstack((
                    button("Confirm")
                        .on_click({
                            let set = set_accent_output;
                            move || set.call(String::from("Confirmed!"))
                        })
                        .accent(),
                    text_block(&*accent_output).foreground(ThemeRef::SecondaryText),
                ))
                .spacing(8.0),
                r#"button("Confirm").on_click(...).accent()"#,
            ),
            sample_card(
                "Subtle style — .subtle()",
                vstack((
                    button("Cancel")
                        .on_click({
                            let set = set_subtle_output;
                            move || set.call(String::from("Cancelled."))
                        })
                        .subtle(),
                    text_block(&*subtle_output).foreground(ThemeRef::SecondaryText),
                ))
                .spacing(8.0),
                r#"button("Cancel").on_click(...).subtle()"#,
            ),
            sample_card(
                "Text-link style — .text_link()",
                vstack((
                    hstack((
                        text_block("Need help?").foreground(ThemeRef::SecondaryText),
                        button("Learn more")
                            .on_click({
                                let set = set_link_output;
                                move || set.call(String::from("Link clicked."))
                            })
                            .text_link(),
                    ))
                    .spacing(4.0),
                    text_block(&*link_output).foreground(ThemeRef::SecondaryText),
                ))
                .spacing(8.0),
                r#"button("Learn more").on_click(...).text_link()"#,
            ),
        ],
    )
    .into()
}
