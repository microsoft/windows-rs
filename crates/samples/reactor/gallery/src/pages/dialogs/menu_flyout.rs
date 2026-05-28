use crate::controls::*;
use windows_reactor::*;

pub fn menu_flyout_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (last_action, set_action) = cx.use_state(String::from("(none)"));

    page_content(
        "MenuFlyout",
        "A flyout that displays a list of menu commands attached to a button.",
        vec![
            sample_card(
                "Basic MenuFlyout",
                vstack((
                    drop_down_button("Open Menu")
                        .menu_flyout(vec![
                            MenuItemDef::Item { text: "Cut".into() },
                            MenuItemDef::Item {
                                text: "Copy".into(),
                            },
                            MenuItemDef::Item {
                                text: "Paste".into(),
                            },
                        ])
                        .on_flyout_item_click({
                            let set_action = set_action.clone();
                            move |s| set_action.call(s)
                        }),
                    text_block(format!("Last action: {last_action}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"drop_down_button("Open Menu").menu_flyout(vec![
    MenuItemDef::Item { text: "Cut".into() },
    MenuItemDef::Item { text: "Copy".into() },
    MenuItemDef::Item { text: "Paste".into() },
]).on_flyout_item_click(handler)"#,
            ),
            sample_card(
                "MenuFlyout with Separators",
                vstack((
                    drop_down_button("Format")
                        .menu_flyout(vec![
                            MenuItemDef::Item {
                                text: "Bold".into(),
                            },
                            MenuItemDef::Item {
                                text: "Italic".into(),
                            },
                            MenuItemDef::Separator,
                            MenuItemDef::Item {
                                text: "Underline".into(),
                            },
                            MenuItemDef::Item {
                                text: "Strikethrough".into(),
                            },
                        ])
                        .on_flyout_item_click(move |s| set_action.call(s)),
                    text_block(format!("Last format: {last_action}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"drop_down_button("Format").menu_flyout(vec![
    MenuItemDef::Item { text: "Bold".into() },
    MenuItemDef::Separator,
    MenuItemDef::Item { text: "Underline".into() },
])"#,
            ),
        ],
    )
}
