#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::*;

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::BLACK);

    let format = TextFormat::new("Segoe UI", 32.0)?
        .with_alignment(TextAlignment::Center)
        .with_paragraph_alignment(ParagraphAlignment::Center);

    let brush = ctx.create_solid_brush(ColorF::WHITE)?;

    let rect = Rect::new(0.0, 0.0, ctx.width, ctx.height);
    ctx.draw_text("Hello, Canvas!", &format, &rect, &brush);
    Ok(())
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

struct Sample;

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Draw Text");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        canvas(draw)
    }
}
