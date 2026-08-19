#![windows_subsystem = "windows"]

use windows_reactor::{
    Button, ButtonEmphasis, Element, Icon, IconSymbol, RenderCx, TextBlock, vstack,
};

fn app(cx: &mut RenderCx<'_>) -> Element {
    let count = cx.use_state(|| 0_u32);
    let current = count.value();
    let increment_plain = count.clone();
    let increment_add = count.clone();

    vstack(
        8.0,
        [
            Button::new("Plain Button")
                .on_click(move || {
                    increment_plain.update(|value| *value += 1);
                })
                .build(),
            Button::new("Add Item")
                .icon(Icon::symbol(IconSymbol::ADD))
                .on_click(move || {
                    increment_add.update(|value| *value += 1);
                })
                .build(),
            Button::new("Delete")
                .icon(Icon::symbol(IconSymbol::DELETE))
                .on_click(move || {
                    count.update(|value| *value = value.saturating_sub(1));
                })
                .build(),
            Button::new("Save")
                .icon(Icon::symbol(IconSymbol::SAVE))
                .emphasis(ButtonEmphasis::Accent)
                .build(),
            TextBlock::new(format!("Count: {current}")).build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("ButtonIcon", app)
}
