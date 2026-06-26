//! Sample for the `Expander` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        Expander::new(
            vstack((
                text_block("Hidden details live inside the expander."),
                text_block("Click the chevron to collapse this panel.").opacity(0.7),
            ))
            .spacing(4.0),
        )
        .header("Details")
        .expanded(true),
        Expander::new(text_block("Collapsed by default."))
            .header("More")
            .expanded(false),
        // Complex header: an element tree instead of plain text.
        Expander::new(text_block("Body content for the rich header expander."))
            .header_content(
                hstack((
                    text_block("⚙").font_size(18.0),
                    text_block("Settings").bold(),
                ))
                .spacing(8.0),
            )
            .expanded(true),
    ))
    .spacing(8.0)
    .max_width(400.0)
    .into()
}

fn main() -> Result<()> {
    reactor_samples::run("Expander", app)
}
