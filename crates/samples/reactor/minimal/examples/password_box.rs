//! Minimal sample for the `PasswordBox` element.

use windows_reactor::*;

fn app(cx: &mut RenderCx) -> impl Into<Element> {
    let (password, set_password) = cx.use_state(String::new());

    let update_password = move |v: String| set_password.call(v);

    vstack((
        PasswordBox::new()
            .value(password.clone())
            .header("Password")
            .placeholder("Type a password…")
            .on_changed(update_password),
        // Echo the captured length without leaking the value.
        text_block(format!("captured length = {}", password.chars().count())),
        PasswordBox::new()
            .header("No reveal button")
            .placeholder("Reveal hidden")
            .reveal_mode(PasswordRevealMode::Hidden),
        PasswordBox::new().header("Disabled").enabled(false),
    ))
    .spacing(8.0)
    .max_width(320.0)
}

fn main() -> Result<()> {
    App::new().title("Sample").render(app)
}
