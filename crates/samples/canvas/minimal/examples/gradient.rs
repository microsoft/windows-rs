//! Demonstrates linear and radial gradient brushes.

#![windows_subsystem = "windows"]

use canvas_minimal::*;

fn draw(ctx: &DrawContext) {
    ctx.clear(Color::BLACK);

    let margin = 40.0;
    let half = ctx.height / 2.0;

    // Linear gradient: blue to orange across the full width.
    let Ok(linear) = ctx.create_linear_gradient(
        Vector2::new(margin, 0.0),
        Vector2::new(ctx.width - margin, 0.0),
        &[
            GradientStop::new(0.0, Color::CORNFLOWER_BLUE),
            GradientStop::new(1.0, Color::new(1.0, 0.5, 0.0, 1.0)),
        ],
    ) else {
        return;
    };

    ctx.fill_rounded_rect(
        &RoundedRect::uniform(
            Rect::new(margin, margin, ctx.width - margin, half - 10.0),
            20.0,
        ),
        &linear,
    );

    // Radial gradient: white center fading out.
    let cx = ctx.width / 2.0;
    let cy = half + (ctx.height - half) / 2.0;
    let r = (ctx.width - margin * 2.0).min(ctx.height - half - margin) / 2.0;

    let Ok(radial) = ctx.create_radial_gradient(
        Vector2::new(cx, cy),
        r,
        r,
        &[
            GradientStop::new(0.0, Color::WHITE),
            GradientStop::new(0.6, Color::CORNFLOWER_BLUE),
            GradientStop::new(1.0, Color::BLACK),
        ],
    ) else {
        return;
    };

    ctx.fill_ellipse(&Ellipse::circle(Vector2::new(cx, cy), r), &radial);
}

fn main() -> Result<()> {
    canvas_minimal::run("Gradient", draw)
}
