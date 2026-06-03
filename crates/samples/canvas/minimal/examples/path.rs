//! Path geometry using the typestate `PathBuilder`.

#![windows_subsystem = "windows"]

use windows_canvas::*;

fn draw(ctx: &DrawContext) {
    ctx.clear(Color::DARK_SLATE_BLUE);

    let cx = ctx.width / 2.0;
    let cy = ctx.height / 2.0;
    let r = cx.min(cy) * 0.8;

    let Ok(path) = (|| -> Result<Path> {
        let mut builder = PathBuilder::new(ctx.device())?.begin(star_point(cx, cy, r, 0));
        for i in 1..10 {
            let radius = if i % 2 == 0 { r } else { r * 0.5 };
            builder = builder.line_to(star_point(cx, cy, radius, i));
        }
        builder.close().build()
    })() else {
        return;
    };

    let Ok(brush) = ctx.create_solid_brush(Color::new(1.0, 0.8, 0.0, 1.0)) else {
        return;
    };
    ctx.fill_path(&path, &brush);
}

fn star_point(cx: f32, cy: f32, r: f32, i: u32) -> Vector2 {
    let angle = std::f32::consts::PI / 5.0 * i as f32 - std::f32::consts::FRAC_PI_2;
    Vector2::new(cx + r * angle.cos(), cy + r * angle.sin())
}

fn main() -> Result<()> {
    canvas_minimal::run("Path", draw)
}
