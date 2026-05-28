use crate::controls::*;
use windows_reactor::*;

pub fn pivot_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (tab_idx, set_tab) = cx.use_state(0_i32);

    page_content(
        "Pivot",
        "A tabbed interface for switching between content sections.",
        vec![
            sample_card(
                "Basic Pivot",
                Pivot::new([
                    PivotItem::new("Tab 1", text_block("Content of tab 1")),
                    PivotItem::new("Tab 2", text_block("Content of tab 2")),
                    PivotItem::new("Tab 3", text_block("Content of tab 3")),
                ]),
                r#"Pivot::new([PivotItem::new("Tab", content), ...])"#,
            ),
            sample_card(
                "Pivot with Selection Tracking",
                vstack((
                    Pivot::new([
                        PivotItem::new("Overview", text_block("Overview content")),
                        PivotItem::new("Details", text_block("Detail content")),
                        PivotItem::new("History", text_block("History content")),
                    ])
                    .selected_index(tab_idx)
                    .on_selection_changed(move |i: i32| set_tab.call(i)),
                    text_block(format!("Active tab index: {tab_idx}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"Pivot::new(items).selected_index(idx).on_selection_changed(handler)"#,
            ),
        ],
    )
}
