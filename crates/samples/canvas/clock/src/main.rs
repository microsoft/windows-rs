#![windows_subsystem = "windows"]

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;
use windows_animation::*;
use windows_canvas::Brush;
use windows_canvas::*;
use windows_reactor::{
    Application, CanvasDrawContext, Window, WindowBackdrop, animated_canvas, component,
    run_reactor_winui_app,
};
use windows_time::DateTime;

struct Resources {
    style: StrokeStyle,
    brush: Brush,
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

fn startup() -> Startup {
    let manager = Manager::new().unwrap();
    let library = TransitionLibrary::new().unwrap();
    let variable = manager.create_variable(0.0).unwrap();
    let transition = library.accelerate_decelerate(5.0, 1.0, 0.2, 0.8).unwrap();
    manager
        .schedule_transition(&variable, &transition, 0.0)
        .unwrap();
    manager.update(0.0).unwrap();

    let (second, minute, hour) = angles_now();

    Startup {
        manager,
        variable,
        start: Instant::now(),
        second,
        minute,
        hour,
    }
}

fn draw_clock(
    ctx: &CanvasDrawContext<'_>,
    brush: &Brush,
    style: &StrokeStyle,
    swing: f32,
    startup: &Startup,
) {
    let radius = ctx.width.min(ctx.height).max(200.0) / 2.0 - 50.0;
    let translation = Matrix3x2::translation(ctx.width / 2.0, ctx.height / 2.0);

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

    ctx.with_transform(&(Matrix3x2::rotation(second) * translation), || {
        ctx.draw_line_styled(
            Vector2::zero(),
            Vector2::new(0.0, -(radius * 0.75)),
            brush,
            radius / 25.0,
            style,
        );
    });

    ctx.with_transform(&(Matrix3x2::rotation(minute) * translation), || {
        ctx.draw_line_styled(
            Vector2::zero(),
            Vector2::new(0.0, -(radius * 0.75)),
            brush,
            radius / 15.0,
            style,
        );
    });

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
    let resources = Rc::new(RefCell::new(None::<Resources>));
    let animation = Rc::new(RefCell::new(None::<Startup>));
    let root = component(move |cx| {
        let open = cx.use_state(|| true);
        if animation.borrow().is_none() {
            *animation.borrow_mut() = Some(startup());
        }
        let draw_resources = Rc::clone(&resources);
        let draw_animation = Rc::clone(&animation);

        let content = animated_canvas(move |ctx: &CanvasDrawContext<'_>| {
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

                *draw_resources.borrow_mut() = Some(Resources {
                    style,
                    brush,
                    target,
                    shadow,
                });
            }

            let resources = draw_resources.borrow();
            let resources = resources.as_ref().unwrap();
            let animation = draw_animation.borrow();
            let animation = animation.as_ref().unwrap();
            animation
                .manager
                .update(animation.start.elapsed().as_secs_f64())
                .unwrap();
            let swing = animation.variable.value().unwrap() as f32;

            ctx.with_target(&resources.target, || {
                ctx.clear(ColorF::TRANSPARENT);
                draw_clock(ctx, &resources.brush, &resources.style, swing, animation);
            });

            ctx.with_transform(&Matrix3x2::translation(5.0, 5.0), || {
                ctx.draw_effect(&resources.shadow);
            });

            ctx.draw_image(&resources.target);
            Ok(())
        })
        .build();

        let windows = if open.value() {
            vec![
                Window::new("Clock", content, move || {
                    open.set(false);
                })
                .backdrop(WindowBackdrop::Mica)
                .client_size(800.0, 600.0)
                .build(),
            ]
        } else {
            Vec::new()
        };
        Application::new(windows).build()
    });
    run_reactor_winui_app(root)
}
