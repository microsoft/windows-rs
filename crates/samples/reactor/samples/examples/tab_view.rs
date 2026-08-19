#![windows_subsystem = "windows"]

use windows_reactor::{Element, RenderCx, TabView, TabViewItem, TextBlock, Thickness, vstack};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let tabs = cx.use_state(|| {
        vec![
            (1u64, "Overview".to_string()),
            (2, "Badges".to_string()),
            (3, "Notice".to_string()),
        ]
    });
    let selected = cx.use_state(|| Some(0usize));
    let current_tabs = tabs.value();
    let current_selected = selected.value();
    let selected_text =
        current_selected.map_or_else(|| "none".to_string(), |index| index.to_string());

    let update_selection = selected.clone();
    let close_tabs = tabs.clone();
    let close_selection = selected;
    let tabs_for_close = current_tabs.clone();
    let reorder_tabs = tabs;
    let tabs_for_reorder = current_tabs.clone();

    vstack(
        8.0,
        [
            TabView::new(
                current_tabs.iter().map(|(key, header)| {
                    TabViewItem::new(
                        *key,
                        header,
                        TextBlock::new(format!("Tab content - {header}"))
                            .padding(Thickness::uniform(12.0))
                            .build(),
                    )
                    .closable(*key != 1)
                }),
                move |index| {
                    update_selection.set(index);
                },
            )
            .selected_index(current_selected)
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
            .reorderable(move |keys| {
                let next = keys
                    .into_iter()
                    .filter_map(|key| {
                        tabs_for_reorder
                            .iter()
                            .find(|(candidate, _)| *candidate == key)
                            .cloned()
                    })
                    .collect::<Vec<_>>();
                if next.len() == tabs_for_reorder.len() {
                    reorder_tabs.set(next);
                }
            })
            .build(),
            TextBlock::new(format!(
                "selected_index = {selected_text}, tabs remaining = {}",
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
    reactor_samples::run("TabView", app)
}
