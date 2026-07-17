#![windows_subsystem = "windows"]

use windows_reactor::*;

mod controls;
mod pages;
mod registry;
mod router;
mod shell;

fn main() {
    let _ = App::new()
        .title("Reactor WinUI Gallery (Rust)")
        .inner_size(1400.0, 900.0)
        .backdrop(Backdrop::Mica)
        .render(shell::gallery_shell);
}
