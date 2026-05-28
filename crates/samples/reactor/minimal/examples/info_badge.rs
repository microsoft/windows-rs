//! Minimal sample for the `InfoBadge` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> impl Into<Element> {
    vstack((
        text_block("Dot (attention indicator)"),
        InfoBadge::dot(),
        text_block("Numeric (small / large counts)"),
        hstack((
            InfoBadge::numeric(1),
            InfoBadge::numeric(9),
            InfoBadge::numeric(42),
            InfoBadge::numeric(999),
        ))
        .spacing(12.0),
    ))
    .spacing(8.0)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
