//! Button icon preserved when its label changes dynamically.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0u32);
    let label = format!("Clicked {count} times");

    vstack((
        Button::new(&*label).icon(SymbolGlyph::Favorite).on_click({
            let set_count = set_count.clone();
            move || set_count.call(count + 1)
        }),
        Button::new(if count == 0 { "Save" } else { "Saved!" })
            .icon(SymbolGlyph::Save)
            .accent()
            .on_click({
                let set_count = set_count;
                move || set_count.call(count + 1)
            }),
        text_block("Click the buttons — the icons should remain visible.").opacity(0.6),
    ))
    .spacing(12.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("ButtonIconDynamic", app)
}
