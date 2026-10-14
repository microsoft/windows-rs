#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    let hash = "d41d8cd98f00b204e9800998ecf8427e";
    App::run(
        Border::new().padding(Thickness::uniform(16.0)).content(
            StackPanel::new().spacing(8.0).children((
                "Select and copy the text. Typing cannot change it.",
                TextBox::new(hash).is_read_only(true),
                "Resize the window: this field stretches and its text stays centered.",
                TextBox::new(hash)
                    .is_read_only(true)
                    .text_alignment(TextAlignment::Center),
            )),
        ),
    )
    .unwrap();
}
