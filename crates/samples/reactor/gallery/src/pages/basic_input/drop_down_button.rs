use crate::controls::*;
use windows_reactor::*;

pub fn drop_down_button_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(String::from("(none)"));

    page_content(
        "DropDownButton",
        "A button that displays a flyout of choices when clicked.",
        vec![
            sample_card(
                "DropDownButton with Menu",
                vstack((
                    drop_down_button("File Actions")
                        .menu_flyout(vec![
                            MenuItemDef::Item {
                                text: "Open".into(),
                            },
                            MenuItemDef::Item {
                                text: "Save".into(),
                            },
                            MenuItemDef::Separator,
                            MenuItemDef::Item {
                                text: "Exit".into(),
                            },
                        ])
                        .on_flyout_item_click(move |s| set_selected.call(s)),
                    text_block(format!("Last action: {selected}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"drop_down_button("File").menu_flyout(items).on_flyout_item_click(handler)"#,
            ),
            sample_card(
                "Disabled DropDownButton",
                drop_down_button("Disabled").enabled(false),
                r#"drop_down_button("Disabled").enabled(false)"#,
            ),
        ],
    )
}
