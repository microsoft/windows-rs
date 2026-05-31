//! Minimal sample for the `PersonPicture` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        text_block("Default (placeholder glyph)"),
        PersonPicture::new(),
        text_block("display_name (initials auto-derived)"),
        hstack((
            PersonPicture::new().display_name("Ada Lovelace"),
            PersonPicture::new().display_name("Grace Hopper"),
            PersonPicture::new().display_name("Alan Turing"),
        ))
        .spacing(12.0),
        text_block("initials (explicit)"),
        PersonPicture::new().initials("WR"),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
