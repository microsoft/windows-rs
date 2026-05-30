#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> impl Into<Element> {
    text_block("Hello, world!")
}

fn main() -> Result<()> {
    let _bootstrap_handle = bootstrap::initialize()?;
    App::new().title("Reactor app").render(app)
}
