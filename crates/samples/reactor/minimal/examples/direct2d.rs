//! Hosts Direct2D content inside a reactor UI via `SwapChainPanel`, with WinUI
//! buttons to add/remove animated circles.

#![windows_subsystem = "windows"]

use std::cell::{Cell, RefCell};

use windows::Win32::Graphics::Direct2D::Common::*;
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::Direct3D::*;
use windows::Win32::Graphics::Direct3D11::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Graphics::Dxgi::*;
use windows_numerics::*;
use windows_reactor::*;

struct D2DState {
    target: ID2D1DeviceContext,
    swap_chain: IDXGISwapChain1,
    brush: ID2D1SolidColorBrush,
    frame: u64,
}

thread_local! {
    static D2D: RefCell<Option<D2DState>> = const { RefCell::new(None) };
    static CIRCLE_COUNT: Cell<u32> = const { Cell::new(5) };
    static PANEL_SIZE: Cell<(u32, u32)> = const { Cell::new((400, 300)) };
}

fn resize_swap_chain(state: &mut D2DState, width: u32, height: u32) {
    if width == 0 || height == 0 {
        return;
    }
    unsafe {
        state.target.SetTarget(None);

        if state
            .swap_chain
            .ResizeBuffers(
                0,
                width,
                height,
                DXGI_FORMAT_UNKNOWN,
                DXGI_SWAP_CHAIN_FLAG(0),
            )
            .is_ok()
        {
            let surface: IDXGISurface = state.swap_chain.GetBuffer(0).unwrap();
            let props = D2D1_BITMAP_PROPERTIES1 {
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: 96.0,
                dpiY: 96.0,
                bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
                ..Default::default()
            };
            let bitmap = state
                .target
                .CreateBitmapFromDxgiSurface(&surface, Some(&props))
                .unwrap();
            state.target.SetTarget(&bitmap);
        }
    }
}

fn create_d2d_state(panel: &SwapChainPanelHandle, width: u32, height: u32) -> Result<D2DState> {
    let mut device: Option<ID3D11Device> = None;
    unsafe {
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            None,
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            Some(&[D3D_FEATURE_LEVEL_11_0]),
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            None,
        )?;
    }
    let device = device.unwrap();

    let d2d_factory: ID2D1Factory1 =
        unsafe { D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)? };
    let dxgi_device: IDXGIDevice = device.cast()?;
    let d2d_device = unsafe { d2d_factory.CreateDevice(&dxgi_device)? };
    let target = unsafe { d2d_device.CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)? };

    let dxgi_adapter = unsafe { dxgi_device.GetAdapter()? };
    let dxgi_factory: IDXGIFactory2 = unsafe { dxgi_adapter.GetParent()? };

    let desc = DXGI_SWAP_CHAIN_DESC1 {
        Width: width,
        Height: height,
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 2,
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
        AlphaMode: DXGI_ALPHA_MODE_PREMULTIPLIED,
        ..Default::default()
    };

    let swap_chain = unsafe { dxgi_factory.CreateSwapChainForComposition(&device, &desc, None)? };

    panel.set_swap_chain(&swap_chain)?;

    let surface: IDXGISurface = unsafe { swap_chain.GetBuffer(0)? };
    let bitmap_props = D2D1_BITMAP_PROPERTIES1 {
        pixelFormat: D2D1_PIXEL_FORMAT {
            format: DXGI_FORMAT_B8G8R8A8_UNORM,
            alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
        },
        dpiX: 96.0,
        dpiY: 96.0,
        bitmapOptions: D2D1_BITMAP_OPTIONS_TARGET | D2D1_BITMAP_OPTIONS_CANNOT_DRAW,
        ..Default::default()
    };
    let bitmap = unsafe { target.CreateBitmapFromDxgiSurface(&surface, Some(&bitmap_props))? };
    unsafe { target.SetTarget(&bitmap) };

    let brush = unsafe {
        target.CreateSolidColorBrush(
            &D2D1_COLOR_F {
                r: 0.3,
                g: 0.6,
                b: 0.9,
                a: 1.0,
            },
            None,
        )?
    };

    Ok(D2DState {
        target,
        swap_chain,
        brush,
        frame: 0,
    })
}

fn render_frame(state: &mut D2DState) {
    state.frame += 1;
    let t = state.frame as f32 * 0.02;
    let count = CIRCLE_COUNT.with(|c| c.get());
    let (w, h) = PANEL_SIZE.with(|c| c.get());
    let cx = w as f32 / 2.0;
    let cy = h as f32 / 2.0;
    let orbit = cx.min(cy) * 0.5;

    unsafe {
        state.target.BeginDraw();

        state.target.Clear(Some(&D2D1_COLOR_F {
            r: 0.05,
            g: 0.05,
            b: 0.1,
            a: 1.0,
        }));

        for i in 0..count {
            let phase = t + i as f32 * 1.2;
            let x = cx + phase.cos() * orbit;
            let y = cy + phase.sin() * (orbit * 0.7);
            let radius = 20.0 + (phase * 0.7).sin().abs() * 30.0;

            let color = D2D1_COLOR_F {
                r: 0.3 + (phase * 0.3).sin().abs() * 0.7,
                g: 0.4 + (phase * 0.5).cos().abs() * 0.5,
                b: 0.8,
                a: 0.85,
            };
            state.brush.SetColor(&color);

            let ellipse = D2D1_ELLIPSE {
                point: Vector2::new(x, y),
                radiusX: radius,
                radiusY: radius,
            };
            state.target.FillEllipse(&ellipse, &state.brush);
        }

        _ = state.target.EndDraw(None, None);
        _ = state.swap_chain.Present(1, DXGI_PRESENT(0));
    }
}

fn app(cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(5_u32);
    let size = cx.use_inner_size();

    let panel_width = (size.width - 32.0).max(100.0);
    let panel_height = (panel_width * 0.6).min(size.height - 140.0).max(100.0);
    let pw = panel_width as u32;
    let ph = panel_height as u32;

    CIRCLE_COUNT.with(|c| c.set(count));
    let prev_size = PANEL_SIZE.with(|c| c.get());
    if prev_size != (pw, ph) {
        PANEL_SIZE.with(|c| c.set((pw, ph)));
        D2D.with(|cell| {
            if let Some(state) = cell.borrow_mut().as_mut() {
                resize_swap_chain(state, pw, ph);
            }
        });
    }

    let rendering = cx.use_ref::<Option<Rendering>>(None);
    cx.use_effect((), {
        #[allow(clippy::redundant_clone)]
        let rendering = rendering.clone();
        move || {
            if let Ok(r) = on_rendering(|| {
                D2D.with(|cell| {
                    if let Some(state) = cell.borrow_mut().as_mut() {
                        render_frame(state);
                    }
                });
            }) {
                rendering.set(Some(r));
            }
        }
    });

    let add = {
        let s = set_count.clone();
        move || s.call(count + 1)
    };
    let remove = {
        let s = set_count;
        move || {
            if count > 0 {
                s.call(count - 1);
            }
        }
    };

    vstack((
        swap_chain_panel()
            .width(panel_width)
            .height(panel_height)
            .on_ready(move |panel| match create_d2d_state(&panel, pw, ph) {
                Ok(state) => D2D.with(|cell| *cell.borrow_mut() = Some(state)),
                Err(e) => eprintln!("D2D init failed: {e}"),
            }),
        hstack((
            button("Add circle").on_click(add),
            button("Remove circle").on_click(remove),
        ))
        .spacing(8.0),
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    App::new().title("Direct2D SwapChainPanel").render(app)
}
