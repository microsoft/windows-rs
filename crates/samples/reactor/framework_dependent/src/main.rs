#![windows_subsystem = "windows"]

use windows_reactor::*;

fn app(_cx: &mut RenderCx) -> Element {
    text_block("Hello, world!").into()
}

fn main() -> Result<()> {
    bootstrap()?;
    App::new().title("Reactor app").render(app)
}
