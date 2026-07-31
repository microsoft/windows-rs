#![windows_subsystem = "windows"]

use windows_reactor::*;

#[allow(nonstandard_style, unused, clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

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
