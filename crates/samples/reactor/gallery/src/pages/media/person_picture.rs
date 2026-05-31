use crate::controls::*;
use windows_reactor::*;

pub fn person_picture_page(_: &(), cx: &mut RenderCx) -> Element {
    let (show_display_names, set_show_display_names) = cx.use_state(true);

    let pictures: Element = if show_display_names {
        hstack((
            PersonPicture::new().display_name("Alice Smith"),
            PersonPicture::new().display_name("Bob Johnson"),
            PersonPicture::new().display_name("Carol Lee"),
        ))
        .spacing(16.0)
        .into()
    } else {
        hstack((
            PersonPicture::new().initials("AS"),
            PersonPicture::new().initials("BJ"),
            PersonPicture::new().initials("CL"),
        ))
        .spacing(16.0)
        .into()
    };

    page_content(
        "PersonPicture",
        "A circular avatar for a person.",
        vec![sample_card(
            "Display Names or Initials",
            vstack((
                ToggleSwitch::new(show_display_names)
                    .header("Use display names")
                    .on_changed(move |value: bool| set_show_display_names.call(value)),
                pictures,
                text_block(if show_display_names {
                    "Initials are generated from the display names."
                } else {
                    "Initials can also be provided directly."
                })
                .opacity(0.6),
            ))
            .spacing(12.0),
            r#"ToggleSwitch::new(mode).on_changed(...)
PersonPicture::new().display_name("Alice Smith")
PersonPicture::new().initials("AS")"#,
        )],
    )
}
