#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    TextBlock::new("Hello from a framework-dependent Reactor app.").build()
}

fn main() -> windows_core::Result<()> {
    reactor_samples::run("Framework-dependent Reactor app", app)
}
