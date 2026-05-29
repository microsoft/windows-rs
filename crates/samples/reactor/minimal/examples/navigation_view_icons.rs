//! Minimal sample for `NavigationView` items with symbol icons.

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
            NavViewItem::new("Home").tag("home").icon(SymbolGlyph::Home),
            NavViewItem::new("Mail").tag("mail").icon(SymbolGlyph::Mail),
            NavViewItem::new("People")
                .tag("people")
                .icon(SymbolGlyph::People),
            NavViewItem::new("Settings")
                .tag("settings")
                .icon(SymbolGlyph::Setting),
        ],
        content,
    )
    .selected_tag(&*page)
    .on_selection_changed(move |tag: String| set_page.call(tag))
    .into()
}

fn main() -> Result<()> {
    App::new().title("NavigationView Icons Sample").render(app)
}
