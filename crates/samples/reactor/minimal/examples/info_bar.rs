//! Sample for the `InfoBar` element.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    vstack((
        InfoBar::new("Did you know?")
            .message("This is an informational notice.")
            .informational(),
        InfoBar::new("Saved")
            .message("Your changes have been saved.")
            .success(),
        InfoBar::new("Heads up")
            .message("Check before proceeding.")
            .warning(),
        InfoBar::new("Something went wrong")
            .message("The operation could not be completed.")
            .error(),
        InfoBar::new("Sticky")
            .message("This bar hides its close button.")
            .informational()
            .is_closable(false),
        InfoBar::new("Hidden")
            .message("This bar is not currently open.")
            .informational()
            .is_open(false),
    ))
    .spacing(8.0)
    .max_width(360.0)
    .into()
}

fn main() -> Result<()> {
    reactor_minimal::run("InfoBar", app)
}
