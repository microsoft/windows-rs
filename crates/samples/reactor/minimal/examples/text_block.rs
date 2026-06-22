//! Sample for the `TextBlock` element, including text selection support.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        text_block("Plain text"),
        text_block("Larger text").font_size(20.0),
        text_block("Bold + larger").bold().font_size(28.0),
        text_block("Selectable text — try selecting this with your mouse")
            .selectable(),
        text_block("Selectable + wrapped text that demonstrates both features working together on a TextBlock element")
            .selectable()
            .wrap(),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("Text Block", app)
}
