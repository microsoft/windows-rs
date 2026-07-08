//! Sample for setting the window icon via `App::icon`.
//!
//! WinUI 3 does not adopt the executable's embedded icon for the window, so the
//! title-bar and taskbar icon are set explicitly from a path to an `.ico` file.

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    text_block("Check the title bar and taskbar — the window uses icon.ico.")
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center)
        .into()
}

fn main() -> Result<()> {
    bootstrap()?;
    let icon = format!(
        "{}/examples/icon.ico",
        env!("CARGO_MANIFEST_DIR").replace('\\', "/"),
    );
    App::new()
        .title("Window Icon")
        .icon(icon)
        .inner_size(560.0, 240.0)
        .render(app)
}
