use crate::controls::*;
use windows_reactor::*;

pub fn list_box_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (selected, set_selected) = cx.use_state(-1_i32);

    let fruits = ["Apple", "Banana", "Cherry", "Date", "Elderberry"];
    let label = if selected >= 0 {
        fruits.get(selected as usize).copied().unwrap_or("?")
    } else {
        "(none)"
    };

    page_content(
        "ListBox",
        "A list of selectable items presented inline.",
        vec![
            sample_card(
                "Basic ListBox",
                vstack((
                    list_box()
                        .items(fruits)
                        .selected_index(selected)
                        .on_selection_changed(move |i: i32| set_selected.call(i)),
                    text_block(format!("Selected: {label}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"list_box().items(["Apple", ...]).selected_index(idx).on_selection_changed(handler)"#,
            ),
            sample_card(
                "Disabled ListBox",
                list_box()
                    .items(["Read", "Only", "Items"])
                    .selected_index(0)
                    .enabled(false),
                r#"list_box().items(items).enabled(false)"#,
            ),
        ],
    )
}
