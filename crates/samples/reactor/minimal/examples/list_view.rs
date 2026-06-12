//! Sample for the `list_view` templated list.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(-1_i32);
    let (mode_idx, set_mode_idx) = cx.use_state(1_i32); // default = Single

    let items: Vec<String> = ["Red", "Green", "Blue", "Yellow", "Magenta"]
        .iter()
        .map(|s| (*s).to_string())
        .collect();

    let modes = [
        SelectionMode::None,
        SelectionMode::Single,
        SelectionMode::Multiple,
        SelectionMode::Extended,
    ];
    let mode = modes[mode_idx as usize];

    let label = if selected >= 0 {
        items
            .get(selected as usize)
            .cloned()
            .unwrap_or_else(|| "(out of range)".into())
    } else {
        "(none)".into()
    };

    let mode_items: Vec<String> = ["None", "Single", "Multiple", "Extended"]
        .iter()
        .map(|s| (*s).to_string())
        .collect();

    vstack((
        text_block("Selection Mode:").bold(),
        list_view(mode_items, |s, _idx| {
            text_block(s.clone()).padding(Thickness::xy(12.0, 4.0))
        })
        .with_key_selector(|s| s.clone())
        .selected_index(mode_idx)
        .on_selection_changed(move |i| set_mode_idx.call(i))
        .height(120.0),
        text_block("Items:").bold(),
        list_view(items, |s, _idx| {
            text_block(s.clone()).padding(Thickness::xy(12.0, 6.0))
        })
        .with_key_selector(|s| s.clone())
        .selected_index(selected)
        .selection_mode(mode)
        .on_selection_changed(move |i| set_selected.call(i))
        .height(180.0),
        text_block(format!(
            "selected_index = {selected} ({label}) | mode = {mode:?}"
        )),
    ))
    .spacing(8.0)
    .max_width(320.0)
    .into()
}

fn main() -> Result<()> {
    let _bootstrap_handle = bootstrap::initialize()?;
    App::new()
        .title("Sample")
        .eager_templated_realization(true)
        .render(app)
}
