#![windows_subsystem = "windows"]

use windows_reactor::{Element, PersonPicture, RenderCx, TextBlock, hstack, vstack};

pub fn app(_cx: &mut RenderCx<'_>) -> Element {
    vstack(
        8.0,
        [
            TextBlock::new("Default (placeholder glyph)").build(),
            PersonPicture::new()
                .automation_name("Default person picture")
                .build(),
            TextBlock::new("Display name (initials derived by WinUI)").build(),
            hstack(
                12.0,
                [
                    PersonPicture::new()
                        .display_name("Ada Lovelace")
                        .automation_name("Ada Lovelace")
                        .build(),
                    PersonPicture::new()
                        .display_name("Grace Hopper")
                        .automation_name("Grace Hopper")
                        .build(),
                    PersonPicture::new()
                        .display_name("Alan Turing")
                        .automation_name("Alan Turing")
                        .build(),
                ],
            ),
            TextBlock::new("Explicit initials").build(),
            PersonPicture::new()
                .initials("WR")
                .automation_name("Initials WR")
                .build(),
        ],
    )
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("PersonPicture", app)
}
