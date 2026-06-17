//! Sample for the `RadioButtons` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(0_i32);

    let update_selected = move |i: i32| set_selected.call(i);

    let options = ["Email", "SMS", "None"];
    let label = options.get(selected as usize).copied().unwrap_or("(none)");

    vstack((
        RadioButtons::new(options)
            .header("Notifications")
            .selected_index(selected)
            .max_columns(3)
            .on_selection_changed(update_selected),
        text_block(format!("selected_index = {selected} ({label})")),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("RadioButtons", app)
}
