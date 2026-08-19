#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::{
    Application, Element, RenderCx, StackPanel, Thickness, Window, WindowBackdrop, canvas_image,
    component, run_reactor_winui_app, text_block,
};

const COLS: usize = 4;
const ROWS: usize = 3;
const TILES: usize = COLS * ROWS;
const TILE: f32 = 132.0;
const GAP: f32 = 8.0;
const WIDTH: f32 = COLS as f32 * TILE + (COLS - 1) as f32 * GAP;
const HEIGHT: f32 = ROWS as f32 * TILE + (ROWS - 1) as f32 * GAP;

struct Resources {
    scale: f32,
    white: Brush,
}

fn app(cx: &mut RenderCx<'_>) -> Element {
    let resources = cx.use_ref(|| None::<Resources>);

    StackPanel::new([
        text_block(format!(
            "{TILES} on-demand tiles - all rendered with one GpuDevice:"
        )),
        canvas_image(move |ctx| {
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
                    white: ctx.create_solid_brush(ColorF::WHITE)?,
                }));
            }

            ctx.clear(ColorF::TRANSPARENT);
            resources
                .with(|resources| {
                    let resources = resources.as_ref().unwrap();
                    for i in 0..TILES {
                        draw_tile(ctx, &resources.white, i)?;
                    }
                    Ok(())
                })
                .unwrap()
        })
        .width(WIDTH as f64)
        .height(HEIGHT as f64)
        .build(),
    ])
    .spacing(12.0)
    .margin(Thickness::uniform(16.0))
    .build()
}

fn background(i: usize) -> ColorF {
    let t = i as f32 / TILES as f32;
    ColorF::new(0.12 + 0.10 * t, 0.14, 0.30 - 0.12 * t, 1.0)
}

fn draw_tile(session: &DrawingSession<'_>, brush: &Brush, i: usize) -> Result<()> {
    let col = i % COLS;
    let row = i / COLS;
    let left = col as f32 * (TILE + GAP);
    let top = row as f32 * (TILE + GAP);
    let right = left + TILE;
    let bottom = top + TILE;
    let background = session.create_solid_brush(background(i))?;
    session.fill_rect(&Rect::new(left, top, right, bottom), &background);

    let center = Vector2::new(left + TILE / 2.0, top + TILE / 2.0);
    let radius = TILE * 0.28;

    match i % 4 {
        0 => session.fill_ellipse(&Ellipse::circle(center, radius), brush),
        1 => session.fill_rect(
            &Rect::new(
                center.x - radius,
                center.y - radius,
                center.x + radius,
                center.y + radius,
            ),
            brush,
        ),
        2 => session.draw_ellipse(&Ellipse::circle(center, radius), brush, 8.0),
        _ => {
            let arm = radius;
            let thick = radius * 0.34;
            session.fill_rect(
                &Rect::new(
                    center.x - arm,
                    center.y - thick,
                    center.x + arm,
                    center.y + thick,
                ),
                brush,
            );
            session.fill_rect(
                &Rect::new(
                    center.x - thick,
                    center.y - arm,
                    center.x + thick,
                    center.y + arm,
                ),
                brush,
            );
        }
    }

    let format = TextFormat::new("Segoe UI", 16.0)?.with_alignment(TextAlignment::Center);
    session.draw_text(
        &format!("{i}"),
        &format,
        &Rect::new(left, bottom - 28.0, right, bottom),
        brush,
    );
    Ok(())
}

fn main() -> Result<()> {
    let root = component(|cx| {
        let open = cx.use_state(|| true);
        let windows = if open.value() {
            vec![
                Window::new("Canvas Shared Device", component(app), move || {
                    open.set(false);
                })
                .backdrop(WindowBackdrop::Mica)
                .client_size(640.0, 520.0)
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
