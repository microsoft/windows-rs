//! Demo of displaying a `SurfaceImageSource` with the reactor `Image` widget,
//! drawing into it once with Direct2D using the app-wide shared device.

use crate::bindings::*;
use crate::device::{Device, Gpu, gpu_context, is_device_lost};
use windows_numerics::{Matrix3x2, Vector2};
use windows_reactor::*;

/// Surface size, in physical pixels. Also used as the element's DIP size for a
/// 1:1 mapping at 96 DPI.
const SIZE: i32 = 256;

/// Create a `SurfaceImageSource`, attach the shared Direct2D device, and draw a
/// single static frame into it. Runs on the UI thread, as required by
/// `SurfaceImageSource`.
fn build_surface(device: &Device) -> Result<SurfaceImageSource> {
    let surface = SurfaceImageSource::new(SIZE, SIZE)?;
    surface.set_device(device.d2d_device())?;

    let (context, (offset_x, offset_y)) =
        surface.begin_draw::<ID2D1DeviceContext>(0, 0, SIZE, SIZE)?;

    unsafe {
        // Draw calls are relative to the surface origin; the offset positions
        // them within the underlying shared atlas.
        context.SetTransform(&Matrix3x2::translation(offset_x as f32, offset_y as f32));

        context.Clear(Some(&D2D_COLOR_F {
            r: 0.39,
            g: 0.58,
            b: 0.93,
            a: 1.0,
        }));

        let brush = context.CreateSolidColorBrush(
            &D2D_COLOR_F {
                r: 1.0,
                g: 0.78,
                b: 0.0,
                a: 1.0,
            },
            None,
        )?;

        let center = SIZE as f32 / 2.0;
        context.FillEllipse(
            &D2D1_ELLIPSE {
                point: Vector2 {
                    x: center,
                    y: center,
                },
                radiusX: center - 24.0,
                radiusY: center - 24.0,
            },
            &brush,
        );
    }

    surface.end_draw()?;
    Ok(surface)
}

/// Sample page: a static Direct2D drawing rendered into a `SurfaceImageSource`
/// and displayed with the reactor `Image` widget.
pub fn surface_image_source_sample(_: &(), cx: &mut RenderCx) -> Element {
    let gpu = cx.use_context(&gpu_context());
    let device = gpu.as_ref().and_then(Gpu::device);
    let (surface, set_surface) = cx.use_state::<Option<SurfaceImageSource>>(None);

    // (Re)build the surface whenever the shared device appears or changes (e.g.
    // after recovery). On device loss, ask the root to recreate the device.
    cx.use_effect(device.clone(), {
        move || match device.as_ref() {
            Some(dev) => match build_surface(dev) {
                Ok(sis) => set_surface.call(Some(sis)),
                Err(e) if is_device_lost(e.code()) => {
                    if let Some(gpu) = gpu.as_ref() {
                        gpu.request_recovery();
                    }
                }
                Err(e) => eprintln!("failed to build surface: {e}"),
            },
            None => set_surface.call(None),
        }
    });

    let image: Element = match surface {
        Some(sis) => Image::new(sis.into())
            .width(SIZE as f64)
            .height(SIZE as f64)
            .into(),
        None => text_block("Creating shared device\u{2026}").into(),
    };

    vstack((text_block("Image backed by a SurfaceImageSource:"), image))
        .spacing(8.0)
        .margin(Thickness::uniform(16.0))
        .into()
}
