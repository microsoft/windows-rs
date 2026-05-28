use crate::controls::*;
use windows_reactor::*;

pub fn check_box_page(_: &(), cx: &mut RenderCx) -> impl Into<Element> {
    let (accepted, set_accepted) = cx.use_state(false);
    let (email_opt_in, set_email) = cx.use_state(true);
    let (sms_opt_in, set_sms) = cx.use_state(false);
    let (push_opt_in, set_push) = cx.use_state(true);

    let active_count = [email_opt_in, sms_opt_in, push_opt_in]
        .iter()
        .filter(|&&v| v)
        .count();

    page_content(
        "CheckBox",
        "A control that a user can select or clear.",
        vec![
            sample_card(
                "Basic CheckBox",
                vstack((
                    check_box(accepted)
                        .label("I accept the terms and conditions")
                        .on_changed({
                            let set_accepted = set_accepted;
                            move |v| set_accepted.call(v)
                        }),
                    text_block(if accepted {
                        "Accepted ✓"
                    } else {
                        "Not yet accepted"
                    })
                    .opacity(0.6),
                ))
                .spacing(8.0),
                r#"check_box(checked).label("I accept").on_changed(move |v| set(v))"#,
            ),
            sample_card(
                "Notification Preferences",
                vstack((
                    text_block("Choose your notification channels:").bold(),
                    check_box(email_opt_in)
                        .label("Email notifications")
                        .on_changed({
                            let set_email = set_email;
                            move |v| set_email.call(v)
                        }),
                    check_box(sms_opt_in)
                        .label("SMS notifications")
                        .on_changed({
                            let set_sms = set_sms;
                            move |v| set_sms.call(v)
                        }),
                    check_box(push_opt_in)
                        .label("Push notifications")
                        .on_changed({
                            let set_push = set_push;
                            move |v| set_push.call(v)
                        }),
                    text_block(format!("{active_count} channel(s) active")).opacity(0.6),
                ))
                .spacing(6.0),
                r#"check_box(email).label("Email").on_changed(handler)
check_box(sms).label("SMS").on_changed(handler)
check_box(push).label("Push").on_changed(handler)"#,
            ),
            sample_card(
                "Disabled States",
                vstack((
                    check_box(true)
                        .label("Always enabled (checked, disabled)")
                        .enabled(false),
                    check_box(false)
                        .label("Always disabled (unchecked, disabled)")
                        .enabled(false),
                ))
                .spacing(6.0),
                r#"check_box(true).label("Locked on").enabled(false)"#,
            ),
        ],
    )
}
