use crate::controls::*;
use windows_reactor::*;

pub fn radio_button_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(0_i32);

    let options = ["Option A", "Option B", "Option C"];
    let label = options.get(selected as usize).copied().unwrap_or("?");

    page_content(
        "RadioButton",
        "Select one option from a group.",
        vec![
            sample_card(
                "Basic RadioButtons",
                vstack((
                    RadioButtons::new(options)
                        .header("Pick one")
                        .selected_index(selected)
                        .on_selection_changed({
                            let set_selected = set_selected;
                            move |i| set_selected.call(i)
                        }),
                    text_block(format!("Selected: {label}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"RadioButtons::new(["A", "B", "C"]).selected_index(idx).on_selection_changed(handler)"#,
            ),
            sample_card(
                "RadioButtons with Header",
                RadioButtons::new(["Small", "Medium", "Large", "Extra Large"])
                    .header("T-shirt size")
                    .selected_index(1),
                r#"RadioButtons::new(sizes).header("T-shirt size").selected_index(1)"#,
            ),
        ],
    )
}
