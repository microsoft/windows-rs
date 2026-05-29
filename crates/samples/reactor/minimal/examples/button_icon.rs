//! Minimal sample for `Button` with a symbol icon.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(0u32);

    vstack((
        button("Plain Button").on_click({
            let set_count = set_count.clone();
            move || set_count.call(count + 1)
        }),
        button("Add Item").icon(SymbolGlyph::Add).on_click({
            let set_count = set_count.clone();
            move || set_count.call(count + 1)
        }),
        button("Delete").icon(SymbolGlyph::Delete).on_click({
            let set_count = set_count;
            move || set_count.call(count.saturating_sub(1))
        }),
        button("Save").icon(SymbolGlyph::Save).accent(),
        text_block(format!("Count: {count}")),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("Button Icon Sample").render(app)
}
