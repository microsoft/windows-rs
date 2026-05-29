use crate::controls::*;
use windows_reactor::*;

pub fn flyout_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (deleted, set_deleted) = cx.use_state(false);

    page_content(
        "Flyout",
        "A lightweight popup for contextual information.",
        vec![
            sample_card(
                "Confirmation Flyout",
                vstack((
                    button("Delete item")
                        .icon(SymbolGlyph::Delete)
                        .flyout("Are you sure? This cannot be undone.")
                        .on_click({
                            let set_deleted = set_deleted;
                            move || set_deleted.call(true)
                        }),
                    text_block(if deleted {
                        "Item deleted!"
                    } else {
                        "No action taken"
                    })
                    .opacity(0.6),
                ))
                .spacing(8.0),
                r#"button("Delete").flyout("Are you sure?").on_click(|| set.call(true))"#,
            ),
            sample_card(
                "Button with Flyout",
                button("Click for flyout").flyout("This is a flyout!"),
                r#"button("Click").flyout("This is a flyout!")"#,
            ),
            sample_card(
                "Multiple Flyout Buttons",
                hstack((
                    button("Help").flyout("Press F1 for more help."),
                    button("Info")
                        .flyout("This operation cannot be undone.")
                        .icon(SymbolGlyph::Flag),
                ))
                .spacing(8.0),
                r#"button("Help").flyout("message"), button("Info").flyout("...")"#,
            ),
        ],
    )
}
