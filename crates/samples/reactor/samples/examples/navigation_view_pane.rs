use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (page, set_page) = cx.use_state("home".to_string());

    let menu_items = [
        NavViewItem::new("Home").tag("home").icon(Symbol::Home),
        NavViewItem::new("Documents")
            .tag("docs")
            .icon(Symbol::Document),
    ];

    let body: Element = match page.as_str() {
        "docs" => text_block("Documents page").into(),
        _ => text_block("Home page").into(),
    };

    let footer = button("Sign out").on_click(|| println!("signed out"));

    NavigationView::new(menu_items, body)
        .selected_tag(page)
        .on_selection_changed(set_page)
        .pane_display_mode(NavigationViewPaneDisplayMode::Left)
        .pane_title("Account")
        .open_pane_length(400.0)
        .pane_footer(footer)
        .settings_visible(false)
        .into()
}

fn main() -> Result<()> {
    reactor_samples::run("NavigationView pane", app)
}
