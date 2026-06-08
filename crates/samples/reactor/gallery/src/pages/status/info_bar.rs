use crate::controls::*;
use windows_reactor::*;

pub fn info_bar_page(_: &(), cx: &mut RenderCx) -> Element {
    let (visible, set_visible) = cx.use_state(true);

    page_content(
        "InfoBar",
        "A dismissible bar for app-level messages.",
        vec![
            sample_card(
                "Toggle InfoBar",
                vstack((
                    ToggleSwitch::new(visible)
                        .header("Show InfoBar")
                        .on_toggled(move |value: bool| set_visible.call(value)),
                    InfoBar::new("Update available")
                        .message("A new version is ready to install.")
                        .informational()
                        .is_open(visible),
                ))
                .spacing(8.0),
                r#"InfoBar::new("Title").message("...").is_open(visible)"#,
            ),
            sample_card(
                "Informational InfoBar",
                InfoBar::new("Update available")
                    .message("A new version is ready to install.")
                    .informational(),
                r#"InfoBar::new("Title").message("...").informational()"#,
            ),
            sample_card(
                "Severity Variants",
                vstack((
                    InfoBar::new("Success")
                        .message("Operation completed.")
                        .success(),
                    InfoBar::new("Warning")
                        .message("Check your input.")
                        .warning(),
                    InfoBar::new("Error")
                        .message("Something went wrong.")
                        .error(),
                ))
                .spacing(8.0),
                r#"InfoBar::new("Title").message("Msg").warning()"#,
            ),
        ],
    )
}
