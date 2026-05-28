use crate::controls::*;
use windows_reactor::*;

pub fn content_dialog_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (open, set_open) = cx.use_state(false);
    let (result_text, set_result) = cx.use_state(String::from("(none)"));
    let (open2, set_open2) = cx.use_state(false);

    let close = set_open.clone();
    let close2 = set_open2.clone();

    page_content(
        "ContentDialog",
        "A modal dialog box with content and action buttons.",
        vec![
            sample_card(
                "Basic Confirmation Dialog",
                vstack((
                    button("Show Dialog").on_click(move || set_open.call(true)),
                    ContentDialog::new("Confirm Action")
                        .content("Are you sure you want to proceed?")
                        .primary_button_text("Yes")
                        .close_button_text("No")
                        .is_open(open)
                        .on_closed({
                            let set_result = set_result;
                            move |r| {
                                close.call(false);
                                let s = match r {
                                    ContentDialogResult::Primary => "Primary (Yes)",
                                    ContentDialogResult::Secondary => "Secondary",
                                    _ => "Closed (No)",
                                };
                                set_result.call(s.to_string());
                            }
                        }),
                    text_block(format!("Last result: {result_text}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"ContentDialog::new("Title").content("...").primary_button_text("Yes").is_open(open)"#,
            ),
            sample_card(
                "Three-Button Dialog",
                vstack((
                    button("Show Three-Button").on_click(move || set_open2.call(true)),
                    ContentDialog::new("Save Changes?")
                        .content("You have unsaved changes. What would you like to do?")
                        .primary_button_text("Save")
                        .secondary_button_text("Don't Save")
                        .close_button_text("Cancel")
                        .is_open(open2)
                        .on_closed(move |_| close2.call(false)),
                ))
                .spacing(8.0),
                r#"ContentDialog::new("Save?").primary_button_text("Save").secondary_button_text("Don't Save")"#,
            ),
        ],
    )
}
