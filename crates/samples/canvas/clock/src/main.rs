//! Analog clock with drop shadow rendered with `windows-canvas`.
//!
//! Compare with `crates/samples/windows/direct2d/src/main.rs`.

// #![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::*;
use windows_time::DateTime;

struct Resources {
    style: StrokeStyle,
    brush: windows_canvas::Brush,
    target: Bitmap,
    shadow: Effect,
}

fn app(cx: &mut RenderCx) -> Element {
    
    let res = cx.use_ref::<Option<Resources>>(None);

    animated_canvas(move |ctx| {
        ctx.clear(ColorF::WHITE);

        // Recreate cached resources on device loss or resize.
        if ctx.device_changed() {
            let Ok(style) = ctx.device().create_stroke_style(
                &StrokeStyleBuilder::new()
                    .start_cap(CapStyle::Round)
                    .end_cap(CapStyle::Triangle),
            ) else {
                return;
            };
            let Ok(brush) = ctx.create_solid_brush(ColorF::new(0.92, 0.38, 0.208, 0.8)) else {
                return;
            };
            let Ok(target) = ctx.create_bitmap_target() else {
                return;
            };
            let Ok(shadow) = ctx.create_shadow(&target) else {
                return;
            };
            *res.borrow_mut() = Some(Resources {
                style,
                brush,
                target,
                shadow,
            });
        }

        let r = res.borrow();
        let r = r.as_ref().unwrap();

        ctx.with_target(&r.target, || {
            ctx.clear(ColorF::TRANSPARENT);
            draw_clock(ctx, &r.brush, &r.style);
        });

        ctx.with_transform(&Matrix3x2::translation(5.0, 5.0), || {
            ctx.draw_effect(&r.shadow);
        });

        ctx.draw_image(&r.target);
    })
    .into()
}

fn draw_clock(ctx: &DrawContext, brush: &windows_canvas::Brush, style: &StrokeStyle) {
    let radius = ctx.width.min(ctx.height).max(200.0) / 2.0 - 50.0;
    let translation = Matrix3x2::translation(ctx.width / 2.0, ctx.height / 2.0);

    // Clock face.
    ctx.with_transform(&translation, || {
        ctx.draw_ellipse(
            &Ellipse::circle(Vector2::zero(), radius),
            brush,
            radius / 20.0,
        );
    });

    let t = DateTime::now().to_local();
    let second_deg = (t.second() as f32 + t.milliseconds() as f32 / 1000.0) * 6.0;
    let minute_deg = t.minute() as f32 * 6.0 + second_deg / 60.0;
    let hour_deg = (t.hour() % 12) as f32 * 30.0 + minute_deg / 12.0;

    // Second hand.
    ctx.with_transform(&(Matrix3x2::rotation(second_deg) * translation), || {
        ctx.draw_line_styled(
            Vector2::zero(),
            Vector2::new(0.0, -(radius * 0.75)),
            brush,
            radius / 25.0,
            style,
        );
    });

    // Minute hand.
    ctx.with_transform(&(Matrix3x2::rotation(minute_deg) * translation), || {
        ctx.draw_line_styled(
            Vector2::zero(),
            Vector2::new(0.0, -(radius * 0.75)),
            brush,
            radius / 15.0,
            style,
        );
    });

    // Hour hand.
    ctx.with_transform(&(Matrix3x2::rotation(hour_deg) * translation), || {
        ctx.draw_line_styled(
            Vector2::zero(),
            Vector2::new(0.0, -(radius * 0.5)),
            brush,
            radius / 10.0,
            style,
        );
    });
}

fn main() -> Result<()> {
    App::new()
        .title("Clock")
        .backdrop(Backdrop::Mica)
        .render(app)
}
