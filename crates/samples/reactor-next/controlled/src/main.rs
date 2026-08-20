use windows_reactor_next::*;

fn main() {
    bootstrap().unwrap();
    App::run(|hooks| {
        let text = hooks.use_state(String::new);
        let changed = text.clone();
        StackPanel::new()
            .spacing(8.0)
            .child(
                "input",
                TextBox::new()
                    .text(text.get())
                    .placeholder_text("Type here")
                    .on_text_changed(move |value| changed.set(value)),
            )
            .child("value", TextBlock::new().text(text.get()))
            .into()
    })
    .unwrap();
}
