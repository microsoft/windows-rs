use crate::controls::*;
use windows_reactor::*;

pub fn menu_bar_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (status, set_status) = cx.use_state(String::from("Last clicked: (none)"));

    page_content(
        "MenuBar",
        "A horizontal bar hosting drop-down menus.",
        vec![sample_card(
            "Basic MenuBar",
            vstack((
                menu_bar(vec![
                    menu_bar_item(
                        "File",
                        vec![
                            MenuItemDef::Item { text: "New".into() },
                            MenuItemDef::Item {
                                text: "Open".into(),
                            },
                            MenuItemDef::Item {
                                text: "Save".into(),
                            },
                        ],
                    ),
                    menu_bar_item(
                        "Edit",
                        vec![
                            MenuItemDef::Item {
                                text: "Undo".into(),
                            },
                            MenuItemDef::Item { text: "Cut".into() },
                            MenuItemDef::Item {
                                text: "Copy".into(),
                            },
                            MenuItemDef::Item {
                                text: "Paste".into(),
                            },
                        ],
                    ),
                ])
                .on_item_click({
                    let set_status = set_status;
                    move |label| set_status.call(format!("Last clicked: {label}"))
                }),
                text_block(status).opacity(0.6),
            ))
            .spacing(8.0),
            r#"menu_bar(items)
    .on_item_click(|label| set_status.call(format!(\"Last clicked: {label}\")))"#,
        )],
    )
}
