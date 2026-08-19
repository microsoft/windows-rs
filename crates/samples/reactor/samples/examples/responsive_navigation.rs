#![windows_subsystem = "windows"]

use windows_reactor::{
    Button, Element, Icon, IconSymbol, NavigationDisplayMode, NavigationItem,
    NavigationPaneDisplayMode, NavigationView, RenderCx, StackPanel, TextBlock, Thickness,
};

fn display_mode_name(mode: NavigationDisplayMode) -> &'static str {
    match mode {
        NavigationDisplayMode::Minimal => "minimal",
        NavigationDisplayMode::Compact => "compact",
        NavigationDisplayMode::Expanded => "expanded",
    }
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let pane_open = cx.use_state(|| true);
    let display_mode = cx.use_state(|| NavigationDisplayMode::Expanded);
    let selected = cx.use_state(|| None::<u64>);
    let open = pane_open.value();
    let mode = display_mode.value();
    let current_selected = selected.value();
    let footer = if mode == NavigationDisplayMode::Expanded {
        "Signed in: Ada"
    } else {
        "AD"
    };

    NavigationView::new(
        [
            NavigationItem::new(0, "Home").icon(Icon::symbol(IconSymbol::HOME)),
            NavigationItem::new(1, "Documents").icon(Icon::symbol(IconSymbol::DOCUMENT)),
        ],
        StackPanel::new([
            TextBlock::new(format!("Actual display mode: {}", display_mode_name(mode))).build(),
            TextBlock::new(if open {
                "Pane is open"
            } else {
                "Pane is closed"
            })
            .build(),
            Button::new("Toggle pane")
                .on_click({
                    let pane_open = pane_open.clone();
                    move || {
                        pane_open.set(!open);
                    }
                })
                .build(),
            TextBlock::new("Resize the window to cross compact and minimal thresholds.").build(),
        ])
        .spacing(12.0)
        .padding(Thickness::uniform(16.0))
        .build(),
        move |key| {
            selected.set(key);
        },
    )
    .selected_key(current_selected)
    .pane_open(open, move |value| {
        pane_open.set(value);
    })
    .pane_display_mode(NavigationPaneDisplayMode::Auto)
    .on_display_mode_changed(move |value| {
        display_mode.set(value);
    })
    .pane_title("Responsive navigation")
    .pane_footer(TextBlock::new(footer).build())
    .settings_visible(false)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Responsive Navigation", app)
}
