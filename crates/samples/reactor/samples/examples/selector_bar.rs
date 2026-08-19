#![windows_subsystem = "windows"]

use windows_reactor::{
    Element, Icon, IconSymbol, RenderCx, SelectorBar, SelectorBarItem, TextBlock, vstack,
};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let selected = cx.use_state(|| Some(1u64));
    let current = selected.value();
    let update = selected;
    let label = match current {
        Some(1) => "Recent",
        Some(2) => "Shared",
        Some(3) => "Favorites",
        _ => "<none>",
    };

    vstack(
        12.0,
        [
            SelectorBar::new(
                [
                    SelectorBarItem::new(1, "Recent"),
                    SelectorBarItem::new(2, "Shared").icon(Icon::symbol(IconSymbol::PEOPLE)),
                    SelectorBarItem::new(3, "Favorites").icon(Icon::symbol(IconSymbol::FAVORITE)),
                ],
                move |key| {
                    update.set(key);
                },
            )
            .selected_key(current)
            .build(),
            TextBlock::new(format!("Selected: {label}")).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("SelectorBar", app)
}
