//! Analog clock with drop shadow rendered with `windows-canvas`.
//!
//! Compare with `crates/samples/windows/direct2d/src/main.rs`.

#![windows_subsystem = "windows"]

use std::time::Instant;
use windows_animation::*;
use windows_canvas::*;
use windows_reactor::*;
use windows_time::DateTime;

struct Resources {
    style: StrokeStyle,
    brush: windows_canvas::Brush,
    target: Bitmap,
    shadow: Effect,
}

struct Startup {
    manager: Manager,
    variable: Variable,
    start: Instant,
    second: f32,
    minute: f32,
    hour: f32,
}

fn app(cx: &mut RenderCx) -> Element {
    let res = cx.use_ref::<Option<Resources>>(None);
    let anim = cx.use_ref::<Option<Startup>>(None);

    if anim.borrow().is_none() {
        let manager = Manager::new().unwrap();
        let library = TransitionLibrary::new().unwrap();
        let variable = manager.create_variable(0.0).unwrap();
        let transition = library.accelerate_decelerate(5.0, 1.0, 0.2, 0.8).unwrap();
        manager
            .schedule_transition(&variable, &transition, 0.0)
            .unwrap();
        manager.update(0.0).unwrap();

        let (second, minute, hour) = angles_now();

        *anim.borrow_mut() = Some(Startup {
            manager,
            variable,
            start: Instant::now(),
            second,
            minute,
            hour,
        });
    }

    animated_canvas(move |ctx| {
        ctx.clear(ColorF::WHITE);

        if ctx.device_changed() {
            let style = ctx
                .device()
                .create_stroke_style(
                    &StrokeStyleBuilder::new()
                        .start_cap(CapStyle::Round)
                        .end_cap(CapStyle::Triangle),
                )
                .unwrap();

            let brush = ctx
                .create_solid_brush(ColorF::new(0.92, 0.38, 0.208, 0.8))
                .unwrap();

            let target = ctx.create_bitmap_target().unwrap();
            let shadow = ctx.create_shadow(&target).unwrap();

            *res.borrow_mut() = Some(Resources {
                style,
                brush,
                target,
                shadow,
            });
        }

        let r = res.borrow();
        let r = r.as_ref().unwrap();

        let a = anim.borrow();
        let a = a.as_ref().unwrap();
        a.manager.update(a.start.elapsed().as_secs_f64()).unwrap();
        let swing = a.variable.value().unwrap() as f32;

        ctx.with_target(&r.target, || {
            ctx.clear(ColorF::TRANSPARENT);
            draw_clock(ctx, &r.brush, &r.style, swing, a);
        });

        ctx.with_transform(&Matrix3x2::translation(5.0, 5.0), || {
            ctx.draw_effect(&r.shadow);
        });

        ctx.draw_image(&r.target);
    })
    .into()
}

fn draw_clock(
    ctx: &DrawContext,
    brush: &windows_canvas::Brush,
    style: &StrokeStyle,
    swing: f32,
    startup: &Startup,
) {
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

    let (mut second, mut minute, mut hour) = angles_now();

    if swing < 1.0 {
        if startup.second > second {
            second += 360.0;
        }
        if startup.minute > minute {
            minute += 360.0;
        }
        if startup.hour > hour {
            hour += 360.0;
        }

        second *= swing;
        minute *= swing;
        hour *= swing;
    }

    // Second hand.
    ctx.with_transform(&(Matrix3x2::rotation(second) * translation), || {
        ctx.draw_line_styled(
            Vector2::zero(),
            Vector2::new(0.0, -(radius * 0.75)),
            brush,
            radius / 25.0,
            style,
        );
    });

    // Minute hand.
    ctx.with_transform(&(Matrix3x2::rotation(minute) * translation), || {
        ctx.draw_line_styled(
            Vector2::zero(),
            Vector2::new(0.0, -(radius * 0.75)),
            brush,
            radius / 15.0,
            style,
        );
    });

    // Hour hand.
    ctx.with_transform(&(Matrix3x2::rotation(hour) * translation), || {
        ctx.draw_line_styled(
            Vector2::zero(),
            Vector2::new(0.0, -(radius * 0.5)),
            brush,
            radius / 10.0,
            style,
        );
    });
}

fn angles_now() -> (f32, f32, f32) {
    let t = DateTime::now().to_local();
    let second = (t.second() as f32 + t.milliseconds() as f32 / 1000.0) * 6.0;
    let minute = t.minute() as f32 * 6.0 + second / 60.0;
    let hour = (t.hour() % 12) as f32 * 30.0 + minute / 12.0;
    (second, minute, hour)
}

fn main() -> Result<()> {
    App::new()
        .title("Clock")
        .backdrop(Backdrop::Mica)
        .render(app)
}
