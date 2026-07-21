use crate::controls::*;
use windows_reactor::*;

pub fn menu_bar_page(_: &(), cx: &mut RenderCx) -> Element {
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
                        vec![menu_item("New"), menu_item("Open"), menu_item("Save")],
                    ),
                    menu_bar_item(
                        "Edit",
                        vec![
                            menu_item("Undo"),
                            menu_item("Cut"),
                            menu_item("Copy"),
                            menu_item("Paste"),
                        ],
                    ),
                ])
                .on_item_clicked({
                    let set_status = set_status;
                    move |label| set_status.call(format!("Last clicked: {label}"))
                }),
                text_block(status).opacity(0.6),
            ))
            .spacing(8.0),
            r#"menu_bar(items)
    .on_item_clicked(|label| set_status.call(format!(\"Last clicked: {label}\")))"#,
        )],
    )
}
