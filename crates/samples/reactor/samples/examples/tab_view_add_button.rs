#![windows_subsystem = "windows"]

use windows_reactor::{Element, RenderCx, TabView, TabViewItem, TextBlock, Thickness, vstack};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let tabs = cx.use_state(|| vec![(1u64, "Tab 1".to_string()), (2, "Tab 2".to_string())]);
    let selected = cx.use_state(|| Some(0usize));
    let current_tabs = tabs.value();
    let current_selected = selected.value();
    let selected_text =
        current_selected.map_or_else(|| "none".to_string(), |index| index.to_string());

    let add_tabs = tabs.clone();
    let update_selection = selected.clone();
    let add_selection = selected.clone();
    let tabs_for_add = current_tabs.clone();
    let close_tabs = tabs;
    let close_selection = selected;
    let tabs_for_close = current_tabs.clone();

    vstack(
        8.0,
        [
            TabView::new(
                current_tabs.iter().map(|(key, header)| {
                    TabViewItem::new(
                        *key,
                        header,
                        TextBlock::new(format!("Content for {header}"))
                            .padding(Thickness::uniform(12.0))
                            .build(),
                    )
                }),
                move |index| {
                    update_selection.set(index);
                },
            )
            .selected_index(current_selected)
            .is_add_tab_button_visible(true)
            .on_add_tab_button_click(move || {
                let mut next = tabs_for_add.clone();
                let key = next.iter().map(|(key, _)| *key).max().unwrap_or(0) + 1;
                next.push((key, format!("Tab {}", next.len() + 1)));
                let index = Some(next.len() - 1);
                add_tabs.set(next);
                add_selection.set(index);
            })
            .on_close_requested(move |key| {
                let next = tabs_for_close
                    .iter()
                    .filter(|(candidate, _)| *candidate != key)
                    .cloned()
                    .collect::<Vec<_>>();
                let next_index = selected_after_removal(current_selected, next.len());
                close_tabs.set(next);
                close_selection.set(next_index);
            })
            .build(),
            TextBlock::new(format!(
                "selected = {selected_text}, total tabs = {}",
                current_tabs.len()
            ))
            .build(),
        ],
    )
}

fn selected_after_removal(selected: Option<usize>, count: usize) -> Option<usize> {
    if count == 0 {
        None
    } else {
        Some(selected.unwrap_or(0).min(count - 1))
    }
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("TabView Add Button", app)
}
