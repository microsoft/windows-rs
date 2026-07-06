//! Sample for `NavigationView::open_pane_length` and `pane_footer` (issue 4669).
//!
//! What to look for:
//!   * `open_pane_length(400.0)` — the open navigation pane (left column with
//!     "Account", "Home", "Documents") is noticeably WIDE (400 DIPs) instead of
//!     the default 320. Try changing it to 200.0 and re-running to see the pane
//!     shrink.
//!   * `pane_footer(...)` — the "Sign out" button is pinned to the BOTTOM of the
//!     pane, separated from the menu items at the top. Toggle the pane with the
//!     hamburger button (top-left) and the footer stays anchored at the bottom.

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

    // Footer element rendered at the bottom of the pane.
    let footer = button("Sign out").on_click(|| println!("signed out"));

    NavigationView::new(menu_items, body)
        .selected_tag(page)
        .on_selection_changed(set_page)
        .pane_display_mode(NavigationViewPaneDisplayMode::Left)
        .pane_title("Account")
        .open_pane_length(400.0) // default is 320; 400 makes the change obvious
        .pane_footer(footer)
        .settings_visible(false)
        .into()
}

fn main() -> Result<()> {
    reactor_samples::run("NavigationView pane", app)
}
