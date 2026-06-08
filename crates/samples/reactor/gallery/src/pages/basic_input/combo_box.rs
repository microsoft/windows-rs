use crate::controls::*;
use windows_reactor::*;

pub fn combo_box_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(-1_i32);

    let colors = ["Red", "Green", "Blue", "Yellow"];
    let label = if selected >= 0 {
        colors.get(selected as usize).copied().unwrap_or("?")
    } else {
        "(none)"
    };

    page_content(
        "ComboBox",
        "A drop-down list of items a user can select from.",
        vec![
            sample_card(
                "Basic ComboBox",
                vstack((
                    ComboBox::new(colors)
                        .header("Color")
                        .placeholder_text("Pick a color")
                        .selected_index(selected)
                        .on_selection_changed(move |i| set_selected.call(i)),
                    text_block(format!("Selected: {label}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"ComboBox::new(colors).header("Color").on_selection_changed(handler)"#,
            ),
            sample_card(
                "Editable ComboBox",
                ComboBox::new(["Cat", "Dog", "Fox"])
                    .header("Animal")
                    .placeholder_text("Type or pick")
                    .editable(true),
                r#"ComboBox::new(items).editable(true)"#,
            ),
        ],
    )
}
