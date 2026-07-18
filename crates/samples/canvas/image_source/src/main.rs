//! On-demand Direct2D drawing with `windows-canvas` hosted in a reactor UI.
//!
//! Unlike [`animated_canvas`], which presents a new frame every vsync, a
//! [`CanvasImageSource`] repaints only when its data changes. Here the "+"/"−"
//! buttons change the number of dots; each change redraws the surface once. When
//! nothing changes, nothing is drawn and no GPU work happens.
//!
//! Compare with the continuous render loop in `crates/samples/canvas/clock`.

#![windows_subsystem = "windows"]

use std::f32::consts::TAU;
use windows_canvas::*;
use windows_reactor::*;

/// Surface size in device-independent pixels; also the displayed size.
const SIZE: f32 = 320.0;

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(6_u32);
    let (scale, set_scale) = cx.use_state(1.0_f64);
    let device = cx.use_ref::<Option<GpuDevice>>(None);
    let surface = cx.use_ref::<Option<CanvasImageSource>>(None);
    let scale_sub = cx.use_ref::<Option<windows_core::EventRevoker>>(None);
    let (image, set_image) = cx.use_state::<Option<ImageSource>>(None);

    // Redraw whenever `count` or the display `scale` changes (and once on mount).
    // The surface and its device are created lazily on the first draw, rebuilt at
    // the new resolution when the scale changes, and recreated if the GPU device
    // is lost.
    cx.use_effect((count, scale), move || {
        // Rebuild the surface if it was allocated at a different scale.
        if surface
            .borrow()
            .as_ref()
            .is_some_and(|s| s.scale() != scale as f32)
        {
            surface.set(None);
        }

        for attempt in 0..2 {
            if device.borrow().is_none() {
                match GpuDevice::new_or_warp() {
                    Ok(d) => device.set(Some(d)),
                    Err(e) => return eprintln!("failed to create device: {e}"),
                }
            }

            if surface.borrow().is_none() {
                let created = {
                    let device = device.borrow();
                    CanvasImageSource::new(device.as_ref().unwrap(), SIZE, SIZE, scale as f32)
                };
                match created {
                    Ok(s) => {
                        set_image.call(Some(s.image_source()));
                        surface.set(Some(s));
                    }
                    Err(e) => return eprintln!("failed to create surface: {e}"),
                }
            }

            let result = surface
                .borrow()
                .as_ref()
                .unwrap()
                .draw(ColorF::CORNFLOWER_BLUE, |session| draw_dial(session, count));

            match result {
                Ok(true) => return,
                // Device lost: drop the surface and device and try once more.
                Ok(false) if attempt == 0 => {
                    surface.set(None);
                    device.set(None);
                    set_image.call(None);
                }
                Ok(false) => return eprintln!("device lost during redraw"),
                Err(e) => return eprintln!("failed to redraw surface: {e}"),
            }
        }
    });

    let content: Element = match image {
        Some(source) => Image::new(source)
            .width(SIZE as f64)
            .height(SIZE as f64)
            .on_mounted(move |handle| {
                // Track the host DPI scale so the surface stays crisp when the
                // window moves to a monitor with different scaling. The revoker is
                // kept in hook state so the subscription lives with the control.
                let set_scale = set_scale.clone();
                if let Ok(revoker) =
                    handle.on_rasterization_scale_changed(move |s| set_scale.call(s))
                {
                    scale_sub.set(Some(revoker));
                }
            })
            .into(),
        None => text_block("Preparing surface\u{2026}").into(),
    };

    let controls = hstack((
        button("Add dot").on_click({
            let set_count = set_count.clone();
            move || set_count.call(count + 1)
        }),
        button("Remove dot").on_click(move || set_count.call(count.saturating_sub(1))),
        text_block(format!("{count} dots")),
    ))
    .spacing(8.0);

    vstack((
        text_block("On-demand surface \u{2014} redraws only when the count changes:"),
        content,
        controls,
    ))
    .spacing(12.0)
    .margin(Thickness::uniform(16.0))
    .into()
}

/// Draw a ring of `count` dots around a hub, in surface-local coordinates.
fn draw_dial(session: &DrawingSession, count: u32) {
    let center = Vector2::new(SIZE / 2.0, SIZE / 2.0);
    let radius = SIZE / 2.0 - 44.0;

    let hub = session.create_solid_brush(ColorF::WHITE).unwrap();
    session.fill_ellipse(&Ellipse::circle(center, 16.0), &hub);

    let count = count.max(1);
    for i in 0..count {
        let phase = i as f32 / count as f32;
        let angle = phase * TAU - TAU / 4.0;
        let position = Vector2::new(
            center.x + angle.cos() * radius,
            center.y + angle.sin() * radius,
        );
        let brush = session
            .create_solid_brush(ColorF::new(
                0.5 + 0.5 * (phase * TAU).cos(),
                0.5 + 0.5 * ((phase + 0.33) * TAU).cos(),
                0.5 + 0.5 * ((phase + 0.66) * TAU).cos(),
                1.0,
            ))
            .unwrap();
        session.fill_ellipse(&Ellipse::circle(position, 20.0), &brush);
    }
}

fn main() -> Result<()> {
    App::new()
        .title("Canvas Image Source")
        .backdrop(Backdrop::Mica)
        .render(app)
}
