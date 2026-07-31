#![windows_subsystem = "windows"]

use windows_canvas::*;
use windows_reactor::*;

const COLS: usize = 4;
const ROWS: usize = 3;
const TILES: usize = COLS * ROWS;
const TILE: f32 = 132.0;

fn app(cx: &mut RenderCx) -> Element {
    let device = cx.use_ref::<Option<GpuDevice>>(None);
    let surfaces = cx.use_ref::<Vec<CanvasImageSource>>(Vec::new());
    let (images, set_images) = cx.use_state::<Vec<ImageSource>>(Vec::new());
    let (scale, set_scale) = cx.use_state(1.0_f64);
    let scale_sub = cx.use_ref::<Option<windows_core::EventRevoker>>(None);

    let scale_sub_effect = scale_sub.clone();
    cx.use_effect((scale,), move || {
        let stale = surfaces
            .borrow()
            .first()
            .is_some_and(|s| s.scale() != scale as f32);
        if stale {
            surfaces.set(Vec::new());
        }

        for attempt in 0..2 {
            if device.borrow().is_none() {
                match GpuDevice::new_or_warp() {
                    Ok(d) => device.set(Some(d)),
                    Err(e) => return eprintln!("failed to create shared device: {e}"),
                }
            }

            if surfaces.borrow().is_empty() {
                let mut built = Vec::new();
                let mut sources = Vec::new();
                let mut lost = false;
                {
                    let device = device.borrow();
                    let device = device.as_ref().unwrap();
                    for i in 0..TILES {
                        let surface = match CanvasImageSource::new(device, TILE, TILE, scale as f32)
                        {
                            Ok(s) => s,
                            Err(e) => return eprintln!("failed to create surface: {e}"),
                        };
                        match surface.draw(background(i), |session| draw_tile(session, i)) {
                            Ok(true) => {}
                            Ok(false) => {
                                lost = true;
                                break;
                            }
                            Err(e) => return eprintln!("failed to draw surface: {e}"),
                        }
                        sources.push(surface.image_source());
                        built.push(surface);
                    }
                }

                if lost {
                    device.set(None);
                    surfaces.set(Vec::new());
                    set_images.call(Vec::new());
                    scale_sub_effect.set(None);
                    if attempt == 0 {
                        continue;
                    }
                    return eprintln!("device lost while building surfaces");
                }

                set_images.call(sources);
                surfaces.set(built);
            }
            return;
        }
    });

    let content: Element = if images.is_empty() {
        text_block("Preparing surfaces\u{2026}").into()
    } else {
        let mut rows: Vec<Element> = Vec::new();
        for (r, chunk) in images.chunks(COLS).enumerate() {
            let mut tiles: Vec<Element> = Vec::new();
            for (c, source) in chunk.iter().enumerate() {
                let mut tile = Image::new(source.clone())
                    .width(TILE as f64)
                    .height(TILE as f64);
                if r == 0 && c == 0 {
                    let set_scale = set_scale.clone();
                    let scale_sub = scale_sub.clone();
                    tile = tile.on_mounted(move |handle| {
                        let set_scale = set_scale.clone();
                        if let Ok(revoker) =
                            handle.on_rasterization_scale_changed(move |s| set_scale.call(s))
                        {
                            scale_sub.set(Some(revoker));
                        }
                    });
                }
                tiles.push(tile.into());
            }
            rows.push(hstack(tiles).spacing(8.0).into());
        }
        vstack(rows).spacing(8.0).into()
    };

    vstack((
        text_block(format!(
            "{TILES} on-demand surfaces \u{2014} all sharing one GpuDevice:"
        )),
        content,
    ))
    .spacing(12.0)
    .margin(Thickness::uniform(16.0))
    .into()
}

fn background(i: usize) -> ColorF {
    let t = i as f32 / TILES as f32;
    ColorF::new(0.12 + 0.10 * t, 0.14, 0.30 - 0.12 * t, 1.0)
}

fn draw_tile(session: &DrawingSession, i: usize) -> Result<()> {
    let center = Vector2::new(TILE / 2.0, TILE / 2.0);
    let radius = TILE * 0.28;
    let brush = session.create_solid_brush(ColorF::WHITE)?;

    match i % 4 {
        0 => session.fill_ellipse(&Ellipse::circle(center, radius), &brush),
        1 => session.fill_rect(
            &Rect::new(
                center.x - radius,
                center.y - radius,
                center.x + radius,
                center.y + radius,
            ),
            &brush,
        ),
        2 => session.draw_ellipse(&Ellipse::circle(center, radius), &brush, 8.0),
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
                &brush,
            );
            session.fill_rect(
                &Rect::new(
                    center.x - thick,
                    center.y - arm,
                    center.x + thick,
                    center.y + arm,
                ),
                &brush,
            );
        }
    }

    let format = TextFormat::new("Segoe UI", 16.0)?.with_alignment(TextAlignment::Center);
    session.draw_text(
        &format!("{i}"),
        &format,
        &Rect::new(0.0, TILE - 28.0, TILE, TILE),
        &brush,
    );
    Ok(())
}

fn main() -> Result<()> {
    App::new()
        .title("Canvas Shared Device")
        .backdrop(Backdrop::Mica)
        .render(app)
}
