#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("PersonPicture", || {
        StackPanel::new().spacing(8.0).children((
            TextBlock::new().text("Default (placeholder glyph)"),
            PersonPicture::new(),
            TextBlock::new().text("display_name (initials auto-derived)"),
            StackPanel::new()
                .orientation(Orientation::Horizontal)
                .spacing(12.0)
                .children((
                    PersonPicture::new().display_name("Ada Lovelace"),
                    PersonPicture::new().display_name("Grace Hopper"),
                    PersonPicture::new().display_name("Alan Turing"),
                )),
            TextBlock::new().text("initials (explicit)"),
            PersonPicture::new().initials("WR"),
        ))
    })
    .unwrap();
}
