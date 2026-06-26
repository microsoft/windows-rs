//! Sample for the `ListBox` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(-1_i32);

    let on_sel = move |idx: i32| set_selected.call(idx);

    let label = if selected >= 0 {
        format!("Selected index: {selected}")
    } else {
        "No selection".to_string()
    };

    vstack((
        list_box()
            .items(["Apple", "Banana", "Cherry", "Date", "Elderberry"])
            .on_selection_changed(on_sel),
        text_block(label),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("ListBox", app)
}
