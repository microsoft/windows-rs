#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> impl Into<Element> {
    text_block("Hello, world!")
}

fn main() -> Result<()> {
    App::new().title("Reactor app").render(app)
}
