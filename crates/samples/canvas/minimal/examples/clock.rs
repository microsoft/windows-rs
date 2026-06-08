//! Analog clock with drop shadow rendered with `windows-canvas`.
//!
//! Compare with `crates/samples/windows/direct2d/src/main.rs`.

//#![windows_subsystem = "windows"]

use std::cell::OnceCell;
use windows_canvas::*;
use windows_time::DateTime;

thread_local! {
    static HAND_STYLE: OnceCell<StrokeStyle> = const { OnceCell::new() };
}

fn draw(ctx: &DrawContext) {
    ctx.clear(ColorF::WHITE);

    let size_x = ctx.width;
    let size_y = ctx.height;

    let Ok(target) = ctx.create_bitmap_target() else {
        return;
    };
    let Ok(shadow) = ctx.create_shadow(&target) else {
        return;
    };

    ctx.with_target(&target, || {
        ctx.clear(ColorF::TRANSPARENT);
        draw_clock(ctx, size_x, size_y);
    });

    // Shadow offset.
    ctx.with_transform(&Matrix3x2::translation(5.0, 5.0), || {
        ctx.draw_effect(&shadow);
    });

    ctx.draw_image(&target);
}

fn draw_clock(ctx: &DrawContext, size_x: f32, size_y: f32) {
    let radius = size_x.min(size_y).max(200.0) / 2.0 - 50.0;

    let Ok(brush) = ctx.create_solid_brush(ColorF::new(0.92, 0.38, 0.208, 0.8)) else {
        return;
    };

    let style = HAND_STYLE.with(|cell| {
        cell.get_or_init(|| {
            ctx.device()
                .create_stroke_style(
                    &StrokeStyleBuilder::new()
                        .start_cap(CapStyle::Round)
                        .end_cap(CapStyle::Triangle),
                )
                .unwrap()
        })
        .clone()
    });

    let translation = Matrix3x2::translation(size_x / 2.0, size_y / 2.0);

    // Clock face.
    ctx.with_transform(&translation, || {
        ctx.draw_ellipse(
            &Ellipse::circle(Vector2::zero(), radius),
            &brush,
            radius / 20.0,
        );
    });

    // Time → angles in degrees.
    let t = DateTime::now().to_local();
    let second_deg = (t.second() as f32 + t.milliseconds() as f32 / 1000.0) * 6.0;
    let minute_deg = t.minute() as f32 * 6.0 + second_deg / 60.0;
    let hour_deg = (t.hour() % 12) as f32 * 30.0 + minute_deg / 12.0;

    // Second hand.
    ctx.with_transform(&(Matrix3x2::rotation(second_deg) * translation), || {
        ctx.draw_line_styled(
            Vector2::zero(),
            Vector2::new(0.0, -(radius * 0.75)),
            &brush,
            radius / 25.0,
            &style,
        );
    });

    // Minute hand.
    ctx.with_transform(&(Matrix3x2::rotation(minute_deg) * translation), || {
        ctx.draw_line_styled(
            Vector2::zero(),
            Vector2::new(0.0, -(radius * 0.75)),
            &brush,
            radius / 15.0,
            &style,
        );
    });

    // Hour hand.
    ctx.with_transform(&(Matrix3x2::rotation(hour_deg) * translation), || {
        ctx.draw_line_styled(
            Vector2::zero(),
            Vector2::new(0.0, -(radius * 0.5)),
            &brush,
            radius / 10.0,
            &style,
        );
    });
}

fn main() -> Result<()> {
    canvas_minimal::run("Clock", draw)
}
