use crate::controls::*;
use windows_reactor::*;

pub fn grid_view_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(-1_i32);
    let items: Vec<String> = (1..=12).map(|i| format!("Item {i}")).collect();

    page_content(
        "GridView",
        "Displays items in a horizontally wrapping grid.",
        vec![sample_card(
            "Selectable GridView",
            vstack((
                grid_view(items, |item, _idx| {
                    Border::new(text_block(item).bold())
                        .padding(16.0)
                        .corner_radius(4.0)
                })
                .on_selection_changed(move |idx| set_selected.call(idx))
                .height(300.0),
                text_block(if selected >= 0 {
                    format!("Selected: Item {}", selected + 1)
                } else {
                    "No selection".to_string()
                })
                .opacity(0.6),
            ))
            .spacing(8.0),
            r#"grid_view(items, |item, _| ...).on_selection_changed(move |idx| set.call(idx))"#,
        )],
    )
}
