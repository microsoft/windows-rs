#![windows_subsystem = "windows"]

use windows_reactor::{Element, PasswordBox, PasswordRevealMode, RenderCx, StackPanel, TextBlock};

pub fn app(cx: &mut RenderCx<'_>) -> Element {
    let password = cx.use_state(String::new);
    let hidden = cx.use_state(String::new);
    let current = password.value();
    let current_hidden = hidden.value();

    StackPanel::new([
        PasswordBox::new(current.clone(), move |value| {
            password.set(value);
        })
        .header("Password")
        .placeholder_text("Type a password...")
        .build(),
        TextBlock::new(format!("captured length = {}", current.chars().count())).build(),
        PasswordBox::new(current_hidden, move |value| {
            hidden.set(value);
        })
        .header("No reveal button")
        .placeholder_text("Reveal hidden")
        .password_reveal_mode(PasswordRevealMode::Hidden)
        .build(),
        PasswordBox::display("").header("Disabled").build(),
    ])
    .spacing(8.0)
    .max_width(320.0)
    .build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("PasswordBox", app)
}
