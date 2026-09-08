#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::*;

fn draw(ctx: &DrawContext) -> Result<()> {
    ctx.clear(ColorF::new(0.1, 0.1, 0.1, 1.0));

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("sample.png");

    let bitmap = ctx.load_bitmap(&path)?;

    let w = bitmap.width();
    let h = bitmap.height();
    ctx.draw_bitmap(&bitmap, &Rect::new(20.0, 20.0, 20.0 + w, 20.0 + h), 1.0);

    ctx.draw_bitmap(
        &bitmap,
        &Rect::new(120.0, 20.0, ctx.width - 20.0, ctx.height - 20.0),
        0.8,
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
        context.window_title("Bitmap");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        canvas(draw)
    }
}
