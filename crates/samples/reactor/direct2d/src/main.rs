//! A hub of Direct2D samples hosted in a reactor UI. Each sample lives in its
//! own module and is selected from a `NavigationView` in [`shell`].

#![windows_subsystem = "windows"]

use windows_reactor::*;

mod device;
mod shell;
mod surface_image_source;
mod swap_chain;

fn main() -> Result<()> {
    App::new()
        .title("Direct2D Samples")
        .backdrop(Backdrop::Mica)
        .render(shell::shell)
}
