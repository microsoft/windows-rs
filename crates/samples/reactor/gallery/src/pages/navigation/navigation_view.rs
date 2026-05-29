use crate::controls::*;
use windows_reactor::*;

pub fn navigation_view_page(_: &(), cx: &mut RenderCx) -> Element {
    let (selected, set_selected) = cx.use_state(String::from("page1"));
    let (top_selected, set_top_selected) = cx.use_state(String::from("home"));

    let items = [
        NavViewItem::new("Home")
            .tag("page1")
            .icon(SymbolGlyph::Home),
        NavViewItem::new("Browse")
            .tag("page2")
            .icon(SymbolGlyph::Find),
        NavViewItem::new("Settings")
            .tag("page3")
            .icon(SymbolGlyph::Setting),
    ];

    let body: Element = match selected.as_str() {
        "page2" => text_block("Browse page content").into(),
        "page3" => text_block("Settings page content").into(),
        _ => text_block("Home page content").into(),
    };

    let top_items = [
        NavViewItem::new("Home").tag("home").icon(SymbolGlyph::Home),
        NavViewItem::new("Documents")
            .tag("docs")
            .icon(SymbolGlyph::Edit),
        NavViewItem::new("Downloads")
            .tag("downloads")
            .icon(SymbolGlyph::Download),
    ];

    let top_body: Element = match top_selected.as_str() {
        "docs" => text_block("Documents area").into(),
        "downloads" => text_block("Downloads area").into(),
        _ => text_block("Home area").into(),
    };

    page_content(
        "NavigationView",
        "A side or top navigation pane for app-level navigation.",
        vec![
            sample_card(
                "Left-Pane NavigationView",
                NavigationView::new(items, body)
                    .selected_tag(&selected)
                    .on_selection_changed({
                        let set_selected = set_selected;
                        move |tag: String| set_selected.call(tag)
                    })
                    .pane_title("Nav Demo")
                    .settings_visible(false)
                    .height(300.0),
                r#"NavigationView::new(items, body)
    .selected_tag(&tag)
    .on_selection_changed(handler)
    .pane_title("Nav Demo")"#,
            ),
            sample_card(
                "Top-Mode NavigationView",
                NavigationView::new(top_items, top_body)
                    .pane_display_mode(NavViewPaneDisplayMode::Top)
                    .selected_tag(&top_selected)
                    .on_selection_changed(move |tag: String| set_top_selected.call(tag))
                    .settings_visible(false)
                    .height(200.0),
                r#"NavigationView::new(items, body)
    .pane_display_mode(NavViewPaneDisplayMode::Top)"#,
            ),
        ],
    )
}
