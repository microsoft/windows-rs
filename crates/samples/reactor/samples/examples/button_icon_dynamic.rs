#![windows_subsystem = "windows"]

use windows_reactor::{
    Button, ButtonEmphasis, Element, Icon, IconSymbol, RenderCx, TextBlock, vstack,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let count = cx.use_state(|| 0_u32);
    let current = count.value();
    let increment_first = count.clone();

    vstack(
        12.0,
        [
            Button::new(format!("Clicked {current} times"))
                .icon(Icon::symbol(IconSymbol::FAVORITE))
                .on_click(move || {
                    increment_first.update(|value| *value += 1);
                })
                .build(),
            Button::new(if current == 0 { "Save" } else { "Saved!" })
                .icon(Icon::symbol(IconSymbol::SAVE))
                .emphasis(ButtonEmphasis::Accent)
                .on_click(move || {
                    count.update(|value| *value += 1);
                })
                .build(),
            TextBlock::new("Click the buttons - the icons should remain visible.")
                .opacity(0.6)
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ButtonIconDynamic", app)
}
