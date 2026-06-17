//! Sample for the `TabView` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (tabs, set_tabs) = cx.use_state(vec![
        ("overview", "Overview"),
        ("badges", "Badges"),
        ("notice", "Notice"),
    ]);
    let (selected, set_selected) = cx.use_state(0i32);

    let items: Vec<TabItem> = tabs
        .iter()
        .map(|(key, header)| {
            let closable = *key != "overview";
            TabItem::new(
                *header,
                text_block(format!("Tab content — {header}")).padding(Thickness::uniform(12.0)),
            )
            .with_key(*key)
            .closable(closable)
        })
        .collect();

    let tabs_for_close = tabs.clone();
    let selected_for_close = selected;
    let set_tabs_close = set_tabs.clone();
    let set_selected_close = set_selected.clone();

    vstack((
        TabView::new(items)
            .selected_index(selected)
            .can_reorder_tabs(true)
            .on_selection_changed(move |i| set_selected.call(i))
            .on_close_requested(move |key| {
                let next: Vec<_> = tabs_for_close
                    .iter()
                    .filter(|(k, _)| *k != key)
                    .copied()
                    .collect();
                let max_index = next.len().saturating_sub(1) as i32;
                let clamped = selected_for_close.min(max_index).max(0);
                set_tabs_close.call(next);
                if clamped != selected_for_close {
                    set_selected_close.call(clamped);
                }
            }),
        text_block(format!(
            "selected_index = {selected}, tabs remaining = {}",
            tabs.len()
        )),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("TabView", app)
}
