//! Sample for the `MenuBar` and `DropDownButton` menu flyout.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (last_click, set_last_click) = cx.use_state(String::from("(none)"));

    let on_item = {
        let set_last_click = set_last_click;
        move |text: String| set_last_click.call(text)
    };

    vstack((
        menu_bar(vec![
            menu_bar_item(
                "File",
                vec![
                    menu_item("New"),
                    menu_item("Open"),
                    menu_separator(),
                    menu_sub_item("Recent", vec![menu_item("doc1.txt"), menu_item("doc2.txt")]),
                    menu_separator(),
                    menu_item("Exit"),
                ],
            ),
            menu_bar_item(
                "Edit",
                vec![menu_item("Cut"), menu_item("Copy"), menu_item("Paste")],
            ),
            menu_bar_item("Help", vec![menu_item("About")]),
        ])
        .on_item_clicked(on_item.clone()),
        drop_down_button("Actions")
            .menu_flyout(vec![
                menu_item("Action A"),
                menu_item("Action B"),
                menu_separator(),
                menu_sub_item("More", vec![menu_item("Action C"), menu_item("Action D")]),
            ])
            .on_item_clicked(on_item),
        text_block(format!("Last clicked: {last_click}")),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("MenuBar", app)
}
