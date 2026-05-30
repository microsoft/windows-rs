//! Hosts a Direct3D11 swap chain inside a reactor UI via `SwapChainPanel`.

#![windows_subsystem = "windows"]

use std::cell::{Cell, RefCell};

use windows::Win32::Graphics::{Direct3D::*, Direct3D11::*, Dxgi::Common::*, Dxgi::*};
use windows_reactor::*;

struct D3DState {
    swap_chain: IDXGISwapChain1,
    device_context: ID3D11DeviceContext,
    frame: u64,
}

thread_local! {
    static D3D: RefCell<Option<D3DState>> = const { RefCell::new(None) };
    static PANEL_SIZE: Cell<(u32, u32)> = const { Cell::new((400, 300)) };
}


fn create_d3d_swap_chain(panel: &SwapChainPanel, width: u32, height: u32) -> Result<D3DState> {
    let mut device: Option<ID3D11Device> = None;
    let mut context: Option<ID3D11DeviceContext> = None;

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
            Some(&mut context),
        )?;
    }

    let device = device.unwrap();
    let context = context.unwrap();

    let dxgi_device: IDXGIDevice = device.cast()?;
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

    let native: ISwapChainPanelNative = panel.cast()?;
    unsafe { native.SetSwapChain(swap_chain.as_raw())? };

    Ok(D3DState {
        swap_chain,
        device_context: context,
        frame: 0,
    })
}

fn render_frame(state: &mut D3DState) {
    state.frame += 1;
    let t = (state.frame as f64 * 0.01).sin().abs() as f32;

    unsafe {
        let backbuffer: ID3D11Texture2D = state.swap_chain.GetBuffer(0).unwrap();
        let device: ID3D11Device = state.device_context.GetDevice().unwrap();
        let mut rtv = None;
        device
            .CreateRenderTargetView(&backbuffer, None, Some(&mut rtv))
            .unwrap();
        let rtv = rtv.unwrap();

        let color = [0.1_f32, 0.2 + t * 0.3, 0.5 + t * 0.4, 1.0];
        state.device_context.ClearRenderTargetView(&rtv, &color);
        state.swap_chain.Present(1, DXGI_PRESENT(0)).unwrap();
    }
}

fn app(cx: &mut RenderCx) -> Element {
    let size = cx.use_inner_size();

    let prev_size = PANEL_SIZE.with(|c| c.get());
    if prev_size != (pw, ph) {
        PANEL_SIZE.with(|c| c.set((pw, ph)));
        D3D.with(|cell| {
            if let Some(state) = cell.borrow_mut().as_mut() {
                    if !(size.width == 0 || size.height == 0) {
    unsafe {
        _ = state.swap_chain.ResizeBuffers(
            0,
            width,
            height,
            DXGI_FORMAT_UNKNOWN,
            DXGI_SWAP_CHAIN_FLAG(0),
        );
    }
}
 
            }
        });
    }

    let rendering = cx.use_ref::<Option<Rendering>>(None);
    cx.use_effect((), {
        #[allow(clippy::redundant_clone)]
        let rendering = rendering.clone();
        move || {
            if let Ok(r) = on_rendering(|| {
                D3D.with(|cell| {
                    if let Some(state) = cell.borrow_mut().as_mut() {
                        render_frame(state);
                    }
                });
            }) {
                rendering.set(Some(r));
            }
        }
    });

    vstack((
        swap_chain_panel()
            .width(size.width)
            .height(size.height)
            .on_mounted(move |native| {
                let panel: SwapChainPanel = native.cast().unwrap();
                match create_d3d_swap_chain(&panel, pw, ph) {
                    Ok(state) => D3D.with(|cell| *cell.borrow_mut() = Some(state)),
                    Err(e) => eprintln!("D3D init failed: {e}"),
                }
            }),
    ))
    .spacing(12.0)
    .padding(Thickness::uniform(16.0))
    .into()
}

fn main() -> Result<()> {
    App::new()
        .title("SwapChainPanel")
        .render(app)
}
