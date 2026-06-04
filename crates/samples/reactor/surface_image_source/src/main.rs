//! Minimal demo of displaying a `SurfaceImageSource` with the reactor `Image`
//! widget, drawing into it with Direct2D.

#![windows_subsystem = "windows"]

use windows::Win32::Graphics::Direct2D::Common::*;
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::*;
use windows::core::Interface;
use windows_numerics::{Matrix3x2, Vector2};
use windows_reactor::*;

/// Surface size, in physical pixels. Also used as the element's DIP size for a
/// 1:1 mapping at 96 DPI.
const SIZE: i32 = 256;

/// Create a Direct2D device backed by a hardware D3D11 device, suitable for
/// handing to a `SurfaceImageSource`.
fn create_d2d_device() -> windows::core::Result<ID2D1Device> {
    let mut d3d_device: Option<ID3D11Device> = None;
    unsafe {
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            None,
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            Some(&[D3D_FEATURE_LEVEL_11_0]),
            D3D11_SDK_VERSION,
            Some(&mut d3d_device),
            None,
            None,
        )?;
    }
    let d3d_device = d3d_device.unwrap();

    let d2d_factory: ID2D1Factory1 =
        unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)? };
    let dxgi_device: IDXGIDevice = d3d_device.cast()?;
    unsafe { d2d_factory.CreateDevice(&dxgi_device) }
}

/// Create a `SurfaceImageSource`, attach a Direct2D device, and draw a single
/// static frame into it. Runs on the UI thread, as required by
/// `SurfaceImageSource`.
fn build_surface() -> windows::core::Result<SurfaceImageSource> {
    let surface = SurfaceImageSource::new(SIZE, SIZE)?;

    let device = create_d2d_device()?;
    surface.set_device(&device)?;

    let (context, (offset_x, offset_y)) =
        surface.begin_draw::<ID2D1DeviceContext>(0, 0, SIZE, SIZE)?;

    unsafe {
        // Draw calls are relative to the surface origin; the offset positions
        // them within the underlying shared atlas.
        context.SetTransform(&Matrix3x2::translation(offset_x as f32, offset_y as f32));

        context.Clear(Some(&D2D1_COLOR_F {
            r: 0.39,
            g: 0.58,
            b: 0.93,
            a: 1.0,
        }));

        let brush = context.CreateSolidColorBrush(
            &D2D1_COLOR_F {
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
                    X: center,
                    Y: center,
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

fn app(cx: &mut RenderCx) -> Element {
    // Create and draw the surface once; it persists across re-renders.
    let surface = cx.use_ref::<Option<SurfaceImageSource>>(None);
    if surface.borrow().is_none() {
        match build_surface() {
            Ok(sis) => surface.set(Some(sis)),
            Err(e) => eprintln!("failed to build surface: {e}"),
        }
    }

    vstack((
        text_block("Image backed by a SurfaceImageSource:"),
        Image::new(surface.get_cloned().into())
            .width(SIZE as f64)
            .height(SIZE as f64),
    ))
    .spacing(8.0)
    .into()
}

fn main() -> Result<()> {
    App::new().title("SurfaceImageSource").render(app)
}
