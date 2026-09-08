#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::*;

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::DARK_SLATE_BLUE);

    let brush = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;

    let w = ctx.width / 2.0;
    let h = ctx.height / 4.0;

    ctx.fill_rect(&Rect::new(10.0, 10.0, w, h - 5.0), &brush);

    brush.set_color(ColorF::rgb(0.9, 0.5, 0.1));
    ctx.fill_rect(&Rect::new(10.0, h + 5.0, w, h * 2.0 - 5.0), &brush);

    brush.set_color(ColorF::from_rgb8(128, 0, 255));
    ctx.fill_rect(&Rect::new(10.0, h * 2.0 + 5.0, w, h * 3.0 - 5.0), &brush);

    brush.set_color(ColorF::new(1.0, 1.0, 1.0, 0.5));
    ctx.fill_rect(
        &Rect::new(10.0, h * 3.0 + 5.0, w, ctx.height - 10.0),
        &brush,
    );
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
        context.window_title("Color");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        canvas(draw)
    }
}
