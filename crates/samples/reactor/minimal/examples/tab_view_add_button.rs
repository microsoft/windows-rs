//! Sample for the `TabView` add-tab button.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (tabs, set_tabs) = cx.use_state(vec!["Tab 1".to_string(), "Tab 2".to_string()]);
    let (selected, set_selected) = cx.use_state(0i32);

    let items: Vec<TabItem> = tabs
        .iter()
        .enumerate()
        .map(|(i, header)| {
            TabItem::new(
                header.as_str(),
                text_block(format!("Content for {header}")).padding(Thickness::uniform(12.0)),
            )
            .with_key(format!("{i}"))
            .closable(true)
        })
        .collect();

    let tabs_for_add = tabs.clone();
    let set_tabs_add = set_tabs.clone();
    let set_selected_add = set_selected.clone();

    let tabs_for_close = tabs.clone();
    let selected_for_close = selected;
    let set_tabs_close = set_tabs;
    let set_selected_close = set_selected.clone();

    vstack((
        TabView::new(items)
            .selected_index(selected)
            .is_add_tab_button_visible(true)
            .on_selection_changed(move |i| set_selected.call(i))
            .on_add_tab_button_click(move |()| {
                let mut next = tabs_for_add.clone();
                let new_name = format!("Tab {}", next.len() + 1);
                next.push(new_name);
                let new_idx = next.len() as i32 - 1;
                set_tabs_add.call(next);
                set_selected_add.call(new_idx);
            })
            .on_close_requested(move |key: String| {
                let idx: usize = key.parse().unwrap_or(0);
                let next: Vec<_> = tabs_for_close
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != idx)
                    .map(|(_, h)| h.clone())
                    .collect();
                let max_index = next.len().saturating_sub(1) as i32;
                let clamped = selected_for_close.min(max_index).max(0);
                set_tabs_close.call(next);
                if clamped != selected_for_close {
                    set_selected_close.call(clamped);
                }
            }),
        text_block(format!(
            "selected = {selected}, total tabs = {}",
            tabs.len()
        )),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("TabView Add Button").render(app)
}
