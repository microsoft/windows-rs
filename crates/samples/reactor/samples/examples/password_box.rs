//! Sample for the `PasswordBox` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> Element {
    let (password, set_password) = cx.use_state(String::new());

    let update_password = move |v: String| set_password.call(v);

    vstack((
        PasswordBox::new()
            .value(password.clone())
            .header("Password")
            .placeholder_text("Type a password…")
            .on_password_changed(update_password),
        text_block(format!("captured length = {}", password.chars().count())),
        PasswordBox::new()
            .header("No reveal button")
            .placeholder_text("Reveal hidden")
            .password_reveal_mode(PasswordRevealMode::Hidden),
        PasswordBox::new().header("Disabled").enabled(false),
    ))
    .spacing(8.0)
    .max_width(320.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("PasswordBox", app)
}
