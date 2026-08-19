#![windows_subsystem = "windows"]

use windows_reactor::{
    Element, HorizontalAlignment, RenderCx, TextBlock, VerticalAlignment, WindowIcon,
};

fn app(_cx: &mut RenderCx<'_>) -> Element {
    TextBlock::new("Check the title bar and taskbar - the window uses icon.ico.")
        .horizontal_alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Center)
        .build()
}

fn main() -> windows_core::Result<()> {
    let icon = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("icon.ico");
    reactor_samples::run_with_window(
        "Window Icon",
        move |window| window.icon(WindowIcon::file(icon.to_string_lossy())),
        app,
    )
}
