use crate::controls::*;
use windows_reactor::*;

pub fn tab_view_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(0_i32);
    let (tab_count, set_tab_count) = cx.use_state(3_i32);

    let dynamic_tabs: Vec<TabItem> = (1..=tab_count)
        .map(|i| {
            TabItem::new(
                format!("Tab {i}"),
                text_block(format!("Content of Tab {i}")),
            )
        })
        .collect();

    page_content(
        "TabView",
        "A control that displays closable, rearrangeable tabs.",
        vec![
            sample_card(
                "Basic TabView",
                TabView::new([
                    TabItem::new("Home", text_block("Home content")),
                    TabItem::new("Document", text_block("Document content")),
                    TabItem::new("Settings", text_block("Settings content")),
                ])
                .selected_index(selected)
                .on_selection_changed({
                    let set_selected = set_selected.clone();
                    move |i: i32| set_selected.call(i)
                })
                .height(200.0),
                r#"TabView::new([
    TabItem::new("Home", content),
    TabItem::new("Document", content),
]).selected_index(idx).on_selection_changed(handler)"#,
            ),
            sample_card(
                "Dynamic Tabs",
                vstack((
                    TabView::new(dynamic_tabs)
                        .selected_index(selected.min(tab_count - 1))
                        .on_selection_changed({
                            let set_selected = set_selected;
                            move |i: i32| set_selected.call(i)
                        })
                        .height(180.0),
                    hstack((
                        button("Add Tab").on_click({
                            let set_tab_count = set_tab_count.clone();
                            move || set_tab_count.call(tab_count + 1)
                        }),
                        button("Remove Tab").enabled(tab_count > 1).on_click({
                            let set_tab_count = set_tab_count;
                            move || set_tab_count.call((tab_count - 1).max(1))
                        }),
                    ))
                    .spacing(8.0),
                ))
                .spacing(8.0),
                r#"let tabs: Vec<TabItem> = (1..=count)
    .map(|i| TabItem::new(format!("Tab {i}"), content))
    .collect();
TabView::new(tabs)"#,
            ),
            sample_card(
                "Non-closable Tabs",
                TabView::new([
                    TabItem::new("Fixed A", text_block("Always present")).closable(false),
                    TabItem::new("Fixed B", text_block("Cannot close")).closable(false),
                ])
                .height(150.0),
                r#"TabItem::new("Tab", content).closable(false)"#,
            ),
        ],
    )
}
