//! Sample for the `NavigationView` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (page, set_page) = cx.use_state("home".to_string());

    let menu_items = [
        NavViewItem::new("Home").tag("home"),
        NavViewItem::new("Settings").tag("settings"),
        NavViewItem::new("About").tag("about"),
    ];

    let body: Element = match page.as_str() {
        "settings" => text_block("Settings page").into(),
        "about" => text_block("About page").into(),
        _ => text_block("Home page").into(),
    };

    NavigationView::new(menu_items, body)
        .selected_tag(page.clone())
        .on_selection_changed(set_page)
        .pane_display_mode(NavigationViewPaneDisplayMode::Left)
        .pane_title("Demo")
        .header(format!("page: {page}"))
        .settings_visible(false)
        .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("NavigationView", app)
}
