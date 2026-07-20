//! Bézier curves with `PathFigure`: a closed filled shape and an open stroke.

#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::DrawContext;

fn draw(ctx: &DrawContext) {
    ctx.clear(ColorF::DARK_SLATE_BLUE);

    if let Ok(path) = filled_blob(ctx)
        && let Ok(brush) = ctx.create_solid_brush(ColorF::new(1.0, 0.8, 0.0, 1.0))
    {
        ctx.fill_path(&path, &brush);
    }

    if let Ok(path) = open_wave(ctx)
        && let Ok(brush) = ctx.create_solid_brush(ColorF::WHITE)
    {
        ctx.draw_path(&path, &brush, 3.0);
    }
}

fn filled_blob(ctx: &DrawContext) -> Result<Path> {
    let cx = ctx.width / 2.0;
    let cy = ctx.height / 2.0;
    let r = cx.min(cy) * 0.5;

    PathBuilder::new(ctx.device())?
        .begin(Vector2::new(cx, cy - r))
        .bezier_to(
            Vector2::new(cx + r * 1.5, cy - r),
            Vector2::new(cx + r * 1.5, cy + r),
            Vector2::new(cx, cy + r),
        )
        .bezier_to(
            Vector2::new(cx - r * 1.5, cy + r),
            Vector2::new(cx - r * 1.5, cy - r),
            Vector2::new(cx, cy - r),
        )
        .close()
        .build()
}

fn open_wave(ctx: &DrawContext) -> Result<Path> {
    let y = ctx.height * 0.8;
    let x0 = ctx.width * 0.1;
    let span = ctx.width * 0.8;

    PathBuilder::new(ctx.device())?
        .begin_hollow(Vector2::new(x0, y))
        .bezier_to(
            Vector2::new(x0 + span * 0.25, y - 120.0),
            Vector2::new(x0 + span * 0.75, y + 120.0),
            Vector2::new(x0 + span, y),
        )
        .end_open()
        .build()
}

fn main() -> Result<()> {
    canvas_samples::run("Curves", draw)
}
