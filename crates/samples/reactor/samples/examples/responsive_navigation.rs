#![windows_subsystem = "windows"]

use windows_reactor::*;

fn display_mode_name(mode: NavigationViewDisplayMode) -> &'static str {
    match mode {
        NavigationViewDisplayMode::Minimal => "minimal",
        NavigationViewDisplayMode::Compact => "compact",
        NavigationViewDisplayMode::Expanded => "expanded",
        _ => "unknown",
    }
}

fn app(cx: &mut RenderCx) -> Element {
    let (pane_open, set_pane_open) = cx.use_state(true);
    let (display_mode, set_display_mode) = cx.use_state(NavigationViewDisplayMode::Expanded);
    let footer = if display_mode == NavigationViewDisplayMode::Expanded {
        "Signed in: Ada"
    } else {
        "AD"
    };

    NavigationView::new(
        [
            NavViewItem::new("Home").tag("home").icon(Symbol::Home),
            NavViewItem::new("Documents")
                .tag("documents")
                .icon(Symbol::Document),
        ],
        vstack((
            text_block(format!(
                "Actual display mode: {}",
                display_mode_name(display_mode)
            )),
            text_block(if pane_open {
                "Pane is open"
            } else {
                "Pane is closed"
            }),
            button("Toggle pane").on_click({
                let set_pane_open = set_pane_open.clone();
                move || set_pane_open.call(!pane_open)
            }),
            text_block("Resize the window to cross compact and minimal thresholds."),
        ))
        .spacing(12.0)
        .padding(Thickness::uniform(16.0)),
    )
    .pane_open(pane_open)
    .on_pane_open_changed(set_pane_open)
    .pane_display_mode(NavigationViewPaneDisplayMode::Auto)
    .on_display_mode_changed(set_display_mode)
    .pane_title("Responsive navigation")
    .pane_footer(text_block(footer))
    .settings_visible(false)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Responsive Navigation", app)
}
