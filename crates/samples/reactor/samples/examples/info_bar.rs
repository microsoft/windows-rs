#![windows_subsystem = "windows"]

use windows_reactor::*;

fn main() {
    sample_reactor_controls::run("InfoBar", || {
        StackPanel::new().spacing(8.0).max_width(360.0).children((
            InfoBar::new()
                .title("Did you know?")
                .message("This is an informational notice.")
                .severity(InfoBarSeverity::Informational)
                .is_open(true),
            InfoBar::new()
                .title("Saved")
                .message("Your changes have been saved.")
                .severity(InfoBarSeverity::Success)
                .is_open(true),
            InfoBar::new()
                .title("Heads up")
                .message("Check before proceeding.")
                .severity(InfoBarSeverity::Warning)
                .is_open(true),
            InfoBar::new()
                .title("Something went wrong")
                .message("The operation could not be completed.")
                .severity(InfoBarSeverity::Error)
                .is_open(true),
            InfoBar::new()
                .title("Sticky")
                .message("This bar hides its close button.")
                .severity(InfoBarSeverity::Informational)
                .is_closable(false)
                .is_open(true),
            InfoBar::new()
                .title("Hidden")
                .message("This bar is not currently open.")
                .severity(InfoBarSeverity::Informational)
                .is_open(false),
        ))
    })
    .unwrap();
}
