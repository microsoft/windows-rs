use crate::controls::*;
use windows_reactor::*;

pub fn menu_flyout_page(_: &(), cx: &mut RenderCx) -> Element {
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
                            menu_item("Cut"),
                            menu_item("Copy"),
                            menu_item("Paste"),
                        ])
                        .on_item_clicked({
                            let set_action = set_action.clone();
                            move |s| set_action.call(s)
                        }),
                    text_block(format!("Last action: {last_action}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"drop_down_button("Open Menu").menu_flyout(vec![
    menu_item("Cut"),
    menu_item("Copy"),
    menu_item("Paste"),
]).on_item_clicked(handler)"#,
            ),
            sample_card(
                "MenuFlyout with Separators",
                vstack((
                    drop_down_button("Format")
                        .menu_flyout(vec![
                            menu_item("Bold"),
                            menu_item("Italic"),
                            MenuItemDef::Separator,
                            menu_item("Underline"),
                            menu_item("Strikethrough"),
                        ])
                        .on_item_clicked(set_action),
                    text_block(format!("Last format: {last_action}")).opacity(0.6),
                ))
                .spacing(8.0),
                r#"drop_down_button("Format").menu_flyout(vec![
    menu_item("Bold"),
    MenuItemDef::Separator,
    menu_item("Underline"),
])"#,
            ),
        ],
    )
}
