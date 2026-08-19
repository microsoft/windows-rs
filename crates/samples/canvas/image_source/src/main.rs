#![windows_subsystem = "windows"]

use std::f32::consts::TAU;
use windows_canvas::*;
use windows_reactor::{
    Application, Element, RenderCx, StackPanel, Thickness, Window, WindowBackdrop, button,
    canvas_image_invalidated, component, hstack, run_reactor_winui_app, text_block,
};

const SIZE: f32 = 320.0;

struct Resources {
    scale: f32,
    hub: Brush,
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let count = cx.use_state(|| 6_u32);
    let current_count = count.value();
    let invalidator = cx.use_canvas_invalidator();
    let invalidate = invalidator.clone();
    cx.use_effect(current_count, move || {
        invalidate.invalidate();
    });

    let resources = cx.use_ref(|| None::<Resources>);
    StackPanel::new([
        text_block("On-demand surface - redraws only when the count changes:"),
        canvas_image_invalidated(&invalidator, move |ctx| {
            let stale = ctx.device_changed()
                || ctx.surface_changed()
                || resources
                    .with(|current| {
                        current
                            .as_ref()
                            .is_none_or(|current| current.scale != ctx.scale_x)
                    })
                    .unwrap_or(true);
            if stale {
                resources.set(Some(Resources {
                    scale: ctx.scale_x,
                    hub: ctx.create_solid_brush(ColorF::WHITE)?,
                }));
            }
            ctx.clear(ColorF::CORNFLOWER_BLUE);
            resources
                .with(|resources| draw_dial(ctx, &resources.as_ref().unwrap().hub, current_count))
                .unwrap()
        })
        .width(SIZE as f64)
        .height(SIZE as f64)
        .build(),
        hstack(
            8.0,
            [
                button("Add dot", {
                    let count = count.clone();
                    move || {
                        count.update(|value| *value += 1);
                    }
                }),
                button("Remove dot", move || {
                    count.update(|value| *value = value.saturating_sub(1));
                }),
                text_block(format!("{current_count} dots")),
            ],
        ),
    ])
    .spacing(12.0)
    .margin(Thickness::uniform(16.0))
    .build()
}

fn draw_dial(session: &DrawingSession<'_>, hub: &Brush, count: u32) -> Result<()> {
    let center = Vector2::new(SIZE / 2.0, SIZE / 2.0);
    let radius = SIZE / 2.0 - 44.0;

    session.fill_ellipse(&Ellipse::circle(center, 16.0), hub);

    let count = count.max(1);
    for i in 0..count {
        let phase = i as f32 / count as f32;
        let angle = phase * TAU - TAU / 4.0;
        let position = Vector2::new(
            center.x + angle.cos() * radius,
            center.y + angle.sin() * radius,
        );
        let brush = session.create_solid_brush(ColorF::new(
            0.5 + 0.5 * (phase * TAU).cos(),
            0.5 + 0.5 * ((phase + 0.33) * TAU).cos(),
            0.5 + 0.5 * ((phase + 0.66) * TAU).cos(),
            1.0,
        ))?;
        session.fill_ellipse(&Ellipse::circle(position, 20.0), &brush);
    }
    Ok(())
}

fn main() -> Result<()> {
    let root = component(|cx| {
        let open = cx.use_state(|| true);
        let windows = if open.value() {
            vec![
                Window::new("Canvas Image Source", component(app), move || {
                    open.set(false);
                })
                .backdrop(WindowBackdrop::Mica)
                .client_size(480.0, 480.0)
                .build()
                .key(0),
            ]
        } else {
            Vec::new()
        };
        Application::new(windows).build()
    });
    run_reactor_winui_app(root)
}
