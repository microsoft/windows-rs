#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    TextBlock::new("Hello from a self-contained Reactor app.").build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Self-contained Reactor app", app)
}
