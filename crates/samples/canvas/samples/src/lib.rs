use windows_canvas::*;
use windows_reactor::*;

pub fn run(title: &'static str, draw: fn(&DrawContext) -> Result<()>) -> Result<()> {
    bootstrap()?;
    App::new()
        .title(title)
        .backdrop(Backdrop::Mica)
        .render(move |_cx: &mut RenderCx| -> Element { canvas(draw).into() })
}

pub fn run_animated(title: &'static str, draw: fn(&DrawContext) -> Result<()>) -> Result<()> {
    bootstrap()?;
    App::new()
        .title(title)
        .backdrop(Backdrop::Mica)
        .render(move |_cx: &mut RenderCx| -> Element { animated_canvas(draw).into() })
}
