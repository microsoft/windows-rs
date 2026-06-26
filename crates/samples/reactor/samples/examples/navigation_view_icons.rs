//! Sample for `NavigationView` items with symbol icons.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (page, set_page) = cx.use_state(String::from("home"));

    let content = match page.as_str() {
        "home" => text_block("Welcome home!"),
        "settings" => text_block("Settings page"),
        "mail" => text_block("Mail inbox"),
        "people" => text_block("Contacts"),
        _ => text_block("Unknown page"),
    };

    NavigationView::new(
        [
            NavViewItem::new("Home").tag("home").icon(Symbol::Home),
            NavViewItem::new("Mail").tag("mail").icon(Symbol::Mail),
            NavViewItem::new("People")
                .tag("people")
                .icon(Symbol::People),
            NavViewItem::new("Settings")
                .tag("settings")
                .icon(Symbol::Setting),
        ],
        content,
    )
    .selected_tag(&*page)
    .on_selection_changed(move |tag: String| set_page.call(tag))
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("NavigationViewIcons", app)
}
