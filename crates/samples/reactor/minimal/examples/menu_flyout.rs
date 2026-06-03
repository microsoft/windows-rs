//! Sample for `MenuFlyout` on a `Button`.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (last_action, set_action) = cx.use_state("(none)".to_string());

    let on_item = move |text: String| set_action.call(text);

    vstack((
        button("Open Menu")
            .menu_flyout(vec![
                menu_item("Cut"),
                menu_item("Copy"),
                menu_item("Paste"),
                menu_separator(),
                menu_sub_item(
                    "Font Size",
                    vec![menu_item("Small"), menu_item("Medium"), menu_item("Large")],
                ),
            ])
            .on_menu_item_clicked(on_item),
        text_block(format!("Last action: {last_action}")),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("MenuFlyout", app)
}
