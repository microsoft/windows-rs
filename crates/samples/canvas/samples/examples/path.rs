//! Path geometry using the typestate `PathBuilder`.

#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::DrawContext;

fn draw(ctx: &DrawContext) {
    ctx.clear(ColorF::DARK_SLATE_BLUE);

    let cx = ctx.width / 2.0;
    let cy = ctx.height / 2.0;
    let r = cx.min(cy) * 0.8;

    let Ok(path) = (|| -> Result<Path> {
        let points = (0..10).map(|i| {
            let radius = if i % 2 == 0 { r } else { r * 0.5 };
            star_point(cx, cy, radius, i)
        });
        PathBuilder::new(ctx.device())?.polygon(points)
    })() else {
        return;
    };

    let Ok(brush) = ctx.create_solid_brush(ColorF::new(1.0, 0.8, 0.0, 1.0)) else {
        return;
    };
    ctx.fill_path(&path, &brush);
}

fn star_point(cx: f32, cy: f32, r: f32, i: u32) -> Vector2 {
    let angle = std::f32::consts::PI / 5.0 * i as f32 - std::f32::consts::FRAC_PI_2;
    Vector2::new(cx + r * angle.cos(), cy + r * angle.sin())
}

fn main() -> Result<()> {
    canvas_samples::run("Path", draw)
}
