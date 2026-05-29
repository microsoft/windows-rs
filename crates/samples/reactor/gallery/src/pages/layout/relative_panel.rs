use crate::controls::*;
use windows_reactor::*;

pub fn relative_panel_page(_: &(), cx: &mut RenderCx) -> Element {
    let (show_alt_layout, set_show_alt_layout) = cx.use_state(false);

    let dynamic_items: Vec<Element> = if show_alt_layout {
        vec![
            text_block("Bottom Left")
                .relative_align_left()
                .relative_align_bottom()
                .into(),
            text_block("Bottom Right")
                .relative_align_right()
                .relative_align_bottom()
                .into(),
            text_block("Center")
                .relative_align_h_center()
                .relative_align_v_center()
                .into(),
        ]
    } else {
        vec![
            text_block("Top Left").into(),
            text_block("Top Right")
                .relative_align_right()
                .relative_align_top()
                .into(),
            text_block("Center")
                .relative_align_h_center()
                .relative_align_v_center()
                .into(),
        ]
    };

    page_content(
        "RelativePanel",
        "Positions children relative to each other.",
        vec![
            sample_card(
                "Switch Layouts",
                vstack((
                    ToggleSwitch::new(show_alt_layout)
                        .header("Show bottom corners")
                        .on_changed(move |value: bool| set_show_alt_layout.call(value)),
                    border(relative_panel(dynamic_items).height(200.0))
                        .padding(Thickness::uniform(16.0)),
                ))
                .spacing(12.0),
                r#"relative_panel(items).height(200.0) // items swap based on state"#,
            ),
            sample_card(
                "Basic RelativePanel",
                relative_panel([
                    text_block("Top Left"),
                    text_block("Top Right")
                        .relative_align_right()
                        .relative_align_top(),
                    text_block("Center")
                        .relative_align_h_center()
                        .relative_align_v_center(),
                ])
                .height(200.0),
                r#"relative_panel([el.relative_align_right(), ...]).height(200.0)"#,
            ),
        ],
    )
}
