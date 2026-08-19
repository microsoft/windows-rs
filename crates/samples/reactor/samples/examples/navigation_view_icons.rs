#![windows_subsystem = "windows"]

use windows_reactor::{
    Element, Icon, IconSymbol, NavigationItem, NavigationView, RenderCx, TextBlock,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let page = cx.use_state(|| 0_u64);
    let current = page.value();
    let content = match current {
        1 => "Mail inbox",
        2 => "Contacts",
        3 => "Settings page",
        _ => "Welcome home!",
    };

    NavigationView::new(
        [
            NavigationItem::new(0, "Home").icon(Icon::symbol(IconSymbol::HOME)),
            NavigationItem::new(1, "Mail").icon(Icon::symbol(IconSymbol::MAIL)),
            NavigationItem::new(2, "People").icon(Icon::symbol(IconSymbol::PEOPLE)),
            NavigationItem::new(3, "Settings").icon(Icon::symbol(IconSymbol::SETTINGS)),
        ],
        TextBlock::new(content).build(),
        move |key| {
            if let Some(key) = key {
                page.set(key);
            }
        },
    )
    .selected_key(Some(current))
    .settings_visible(false)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("NavigationViewIcons", app)
}
