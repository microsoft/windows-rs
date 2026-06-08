//! Sample for the `ComboBox` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(-1_i32);

    let update_selected = move |i: i32| set_selected.call(i);

    let colors = ["Red", "Green", "Blue"];
    let label = if selected >= 0 {
        colors
            .get(selected as usize)
            .copied()
            .unwrap_or("(out of range)")
    } else {
        "(none)"
    };

    vstack((
        ComboBox::new(colors)
            .header("Color")
            .placeholder_text("Pick a color")
            .selected_index(selected)
            .on_selection_changed(update_selected),
        text_block(format!("selected_index = {selected} ({label})")),
        ComboBox::new(["Cat", "Dog", "Fox"])
            .header("Editable")
            .placeholder_text("Type or pick an animal")
            .editable(true),
        ComboBox::new(["A", "B", "C"])
            .header("Disabled")
            .selected_index(0)
            .enabled(false),
    ))
    .spacing(8.0)
    .max_width(320.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("ComboBox", app)
}
