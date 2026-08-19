#![windows_subsystem = "windows"]

use windows_reactor::{
    Button, Element, Icon, IconSymbol, NavigationItem, NavigationPaneDisplayMode, NavigationView,
    RenderCx, TextBlock,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let page = cx.use_state(|| 0_u64);
    let signed_out = cx.use_state(|| false);
    let current = page.value();
    let content = TextBlock::new(if signed_out.value() {
        "Signed out"
    } else if current == 1 {
        "Documents page"
    } else {
        "Home page"
    })
    .build();

    NavigationView::new(
        [
            NavigationItem::new(0, "Home").icon(Icon::symbol(IconSymbol::HOME)),
            NavigationItem::new(1, "Documents").icon(Icon::symbol(IconSymbol::DOCUMENT)),
        ],
        content,
        move |key| {
            if let Some(key) = key {
                page.set(key);
            }
        },
    )
    .selected_key(Some(current))
    .pane_display_mode(NavigationPaneDisplayMode::Left)
    .pane_title("Account")
    .open_pane_length(400.0)
    .pane_footer(
        Button::new("Sign out")
            .on_click(move || {
                signed_out.set(true);
            })
            .build(),
    )
    .settings_visible(false)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("NavigationView pane", app)
}
