use crate::controls::*;
use windows_reactor::*;

pub fn password_box_page(_: &(), cx: &mut RenderCx) -> Element {
    let (password, set_password) = cx.use_state(String::new());

    page_content(
        "PasswordBox",
        "A text input that conceals typed characters for secure entry.",
        vec![
            sample_card(
                "Basic PasswordBox",
                vstack((
                    PasswordBox::new()
                        .value(password.clone())
                        .header("Password")
                        .placeholder("Enter password")
                        .on_changed({
                            let set_password = set_password;
                            move |s: String| set_password.call(s)
                        }),
                    text_block(format!("Length: {} chars", password.len())).opacity(0.6),
                ))
                .spacing(8.0),
                r#"PasswordBox::new().header("Password").placeholder("...").on_changed(handler)"#,
            ),
            sample_card(
                "PasswordBox with Reveal Button",
                PasswordBox::new()
                    .header("Secret Key")
                    .placeholder("Enter secret")
                    .reveal_button_enabled(true),
                r#"PasswordBox::new().reveal_button_enabled(true)"#,
            ),
            sample_card(
                "Disabled PasswordBox",
                PasswordBox::new()
                    .value("hunter2")
                    .header("Saved")
                    .enabled(false),
                r#"PasswordBox::new().value("***").enabled(false)"#,
            ),
        ],
    )
}
