#![windows_subsystem = "windows"]

use std::time::Instant;
use windows_canvas::*;
use windows_reactor::*;

fn draw(ctx: &DrawContext, t: f32) -> Result<()> {
    ctx.clear(ColorF::BLACK);

    let brush = ctx.create_solid_brush(ColorF::CORNFLOWER_BLUE)?;

    let cx = ctx.width / 2.0;
    let cy = ctx.height / 2.0;
    let size = cx.min(cy) * 0.25;

    for i in 0..6 {
        let angle = t.to_degrees() + i as f32 * 60.0;
        let transform = Matrix3x2::rotation(angle) * Matrix3x2::translation(cx, cy);

        let frac = i as f32 / 6.0;
        brush.set_color(ColorF::new(0.3 + frac * 0.7, 0.5, 1.0 - frac * 0.5, 0.8));

        ctx.with_transform(&transform, || {
            ctx.fill_rect(&Rect::new(-size, -size, size, size), &brush);
        });
    }
    Ok(())
}

fn main() -> Result<()> {
    App::run_component::<Sample>(())
}

struct Sample {
    start: Instant,
}

impl Component for Sample {
    type Input = ();
    type Message = ();

    fn create(_input: &(), _context: &ComponentContext<Self>) -> Self {
        Self {
            start: Instant::now(),
        }
    }

    fn view(&self, _input: &(), context: &mut ViewContext<Self>) -> View {
        context.window_title("Transform");
        context.window_visuals(WindowVisuals::new().backdrop(WindowBackdrop::Mica));
        let start = self.start;
        animated_canvas(move |ctx| draw(ctx, start.elapsed().as_secs_f32() * 0.6))
    }
}
