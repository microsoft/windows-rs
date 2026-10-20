#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    let hash = "d41d8cd98f00b204e9800998ecf8427e";
    App::run(
        Border::new().padding(Thickness::uniform(16.0)).content(
            StackPanel::new().spacing(8.0).children((
                "Read-only: select and copy, but not edit.",
                TextBox::new().text(hash).is_read_only(true),
                "Centered text: resize the window.",
                TextBox::new()
                    .text(hash)
                    .is_read_only(true)
                    .text_alignment(TextAlignment::Center),
            )),
        ),
    )
    .unwrap();
}
