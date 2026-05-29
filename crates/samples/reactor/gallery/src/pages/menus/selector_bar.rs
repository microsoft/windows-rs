use crate::controls::*;
use windows_reactor::*;

pub fn selector_bar_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (selected, set_selected) = cx.use_state(String::from("Recent"));

    let cb = set_selected;
    page_content(
        "SelectorBar",
        "Switch between different views or modes.",
        vec![sample_card(
            "Basic SelectorBar",
            vstack((
                selector_bar(vec![
                    selector_bar_item("Recent"),
                    selector_bar_item("Shared"),
                    selector_bar_item("Favorites"),
                ])
                .on_selection_changed(move |tag: String| cb.call(tag)),
                text_block(format!("Selected: {selected}")).opacity(0.6),
            ))
            .spacing(8.0),
            r#"selector_bar(vec![selector_bar_item("Recent"), ...]).on_selection_changed(h)"#,
        )],
    )
}
