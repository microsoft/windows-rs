#![windows_subsystem = "windows"]

use windows_reactor::{
    Element, NavigationItem, NavigationPaneDisplayMode, NavigationView, RenderCx, TextBlock,
};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let page = cx.use_state(|| 0_u64);
    let current = page.value();
    let (name, content) = match current {
        1 => ("settings", "Settings page"),
        2 => ("about", "About page"),
        _ => ("home", "Home page"),
    };

    NavigationView::new(
        [
            NavigationItem::new(0, "Home"),
            NavigationItem::new(1, "Settings"),
            NavigationItem::new(2, "About"),
        ],
        TextBlock::new(content).build(),
        move |key| {
            if let Some(key) = key {
                page.set(key);
            }
        },
    )
    .selected_key(Some(current))
    .pane_display_mode(NavigationPaneDisplayMode::Left)
    .pane_title("Demo")
    .header(format!("page: {name}"))
    .settings_visible(false)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("NavigationView", app)
}
