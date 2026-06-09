//! Hosts Direct2D content inside a reactor UI via `SwapChainPanel`, with WinUI
//! buttons to add/remove animated circles. Direct2D rendering runs on a
//! dedicated worker thread that presents into a composition swap chain.

use render::{RenderThread, SendSwap};
use windows_reactor::*;

/// Direct2D rendering, isolated on a dedicated worker thread. Only the handle
/// to that thread and the swap chain it hands back are visible to the UI.
mod render {
    use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
    use std::thread::{self, JoinHandle};
    use std::time::Duration;

    use windows::Win32::Foundation::DXGI_STATUS_OCCLUDED;
    use windows::Win32::Graphics::Direct2D::Common::*;
    use windows::Win32::Graphics::Direct2D::*;
    use windows::Win32::Graphics::Direct3D::*;
    use windows::Win32::Graphics::Direct3D11::*;
    use windows::Win32::Graphics::Dxgi::Common::*;
    use windows::Win32::Graphics::Dxgi::*;
    use windows::core::{Interface, Result};
    use windows_numerics::*;

    struct D2DState {
        target: ID2D1DeviceContext,
        swap_chain: IDXGISwapChain1,
        brush: ID2D1SolidColorBrush,
        frame: u64,
    }

    /// Messages sent from the UI thread to the render thread.
    enum RenderCommand {
        SetCircleCount(u32),
        Resize(u32, u32),
        Shutdown,
    }

    /// Result of presenting a frame. `Idle` means no frame was presented (the
    /// surface was occluded or had zero size), so the caller should throttle.
    enum FrameStatus {
        Presented,
        Idle,
    }

    /// Wraps an `IDXGISwapChain1` so it can be handed back to the UI thread for
    /// attachment via `SetSwapChain`. DXGI swap chains are agile (free-threaded),
    /// and only the render thread ever presents/resizes this one, so moving the
    /// reference across the thread boundary is sound.
    #[derive(Clone, PartialEq)]
    pub struct SendSwap(IDXGISwapChain1);

    unsafe impl Send for SendSwap {}

    impl SendSwap {
        /// The swap chain produced by the render thread, ready to be attached to a
        /// presentation surface on the UI thread.
        pub fn swap_chain(&self) -> &IDXGISwapChain1 {
            &self.0
        }
    }

    /// Owns the render thread and the channel used to talk to it. Created once the
    /// panel is ready. Dropping it asks the thread to stop and waits for it to
    /// finish, so a clean shutdown happens on any drop path.
    pub struct RenderThread {
        commands: Sender<RenderCommand>,
        worker: Option<JoinHandle<()>>,
    }

    impl RenderThread {
        /// Spawn the render thread. `initial_count` is the number of circles to
        /// draw before the first `set_circle_count`. `on_swap_chain` is invoked
        /// once, on the render thread, after the swap chain is created so the
        /// caller can attach it to its presentation surface.
        pub fn new(
            initial_count: u32,
            on_swap_chain: impl FnOnce(SendSwap) + Send + 'static,
        ) -> Self {
            let (commands, rx) = channel();
            let worker = thread::spawn(move || {
                if let Err(e) = render_loop(rx, initial_count, on_swap_chain) {
                    eprintln!("render thread failed: {e}");
                }
            });
            Self {
                commands,
                worker: Some(worker),
            }
        }

        /// Send a command to the render thread. No-op if it has already shut down.
        fn send(&self, command: RenderCommand) {
            let _ = self.commands.send(command);
        }

        /// Set the number of circles to animate.
        pub fn set_circle_count(&self, count: u32) {
            self.send(RenderCommand::SetCircleCount(count));
        }

        /// Resize the render surface.
        pub fn resize(&self, width: u32, height: u32) {
            self.send(RenderCommand::Resize(width, height));
        }
    }

    impl Drop for RenderThread {
        /// Ask the render thread to stop and wait for it to finish.
        fn drop(&mut self) {
            self.send(RenderCommand::Shutdown);
            if let Some(worker) = self.worker.take() {
                let _ = worker.join();
            }
        }
    }

    fn resize_swap_chain(state: &mut D2DState, width: u32, height: u32) -> Result<()> {
        if width == 0 || height == 0 {
            return Ok(());
        }
        unsafe {
            state.target.SetTarget(None);

            state.swap_chain.ResizeBuffers(
                0,
                width,
                height,
                DXGI_FORMAT_UNKNOWN,
                DXGI_SWAP_CHAIN_FLAG(0),
            )?;

            let surface: IDXGISurface = state.swap_chain.GetBuffer(0)?;
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
                .CreateBitmapFromDxgiSurface(&surface, Some(&props))?;
            state.target.SetTarget(&bitmap);
        }
        Ok(())
    }

    fn create_d2d_state(width: u32, height: u32) -> Result<D2DState> {
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

        let swap_chain =
            unsafe { dxgi_factory.CreateSwapChainForComposition(&device, &desc, None)? };

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

    fn render_frame(state: &mut D2DState, count: u32, size: (u32, u32)) -> Result<FrameStatus> {
        let (w, h) = size;

        // While minimized or before the first layout the size can be (0, 0);
        // there is nothing to draw, so report it so the caller can idle.
        if w == 0 || h == 0 {
            return Ok(FrameStatus::Idle);
        }

        state.frame += 1;
        let t = state.frame as f32 * 0.02;
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

            state.target.EndDraw(None, None)?;

            // `DXGI_STATUS_OCCLUDED` is a success status, so it has to be
            // inspected before `ok()` discards it. The caller throttles on it.
            let present = state.swap_chain.Present(1, DXGI_PRESENT(0));
            if present == DXGI_STATUS_OCCLUDED {
                return Ok(FrameStatus::Idle);
            }
            present.ok()?;
        }

        Ok(FrameStatus::Presented)
    }

    /// Entry point for the dedicated render thread. Owns all D3D/D2D state, drains
    /// commands from the caller, and drives the animation loop. `Present(1)` paces
    /// the loop at the display refresh rate while the surface is visible.
    ///
    /// `on_swap_chain` is invoked once, after the swap chain is created, so the
    /// caller can attach it to its presentation surface.
    fn render_loop(
        rx: Receiver<RenderCommand>,
        initial_count: u32,
        on_swap_chain: impl FnOnce(SendSwap),
    ) -> Result<()> {
        let mut size = (400_u32, 300_u32);
        let mut count = initial_count;
        let mut state = create_d2d_state(size.0, size.1)?;

        // Hand the swap chain back to the caller to attach it to its surface.
        on_swap_chain(SendSwap(state.swap_chain.clone()));

        loop {
            // Drain all pending commands without blocking.
            loop {
                match rx.try_recv() {
                    Ok(RenderCommand::SetCircleCount(c)) => count = c,
                    Ok(RenderCommand::Resize(w, h)) => {
                        size = (w, h);
                        resize_swap_chain(&mut state, w, h)?;
                    }
                    Ok(RenderCommand::Shutdown) | Err(TryRecvError::Disconnected) => {
                        return Ok(());
                    }
                    Err(TryRecvError::Empty) => break,
                }
            }

            // `Present(1)` normally blocks to pace the loop at the refresh rate.
            // When no frame is presented (zero size or occluded) it returns
            // immediately, so idle briefly to avoid a busy loop.
            if let FrameStatus::Idle = render_frame(&mut state, count, size)? {
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

/// Sample page: animated Direct2D circles presented through a `SwapChainPanel`,
/// with buttons to add or remove circles.
pub fn swap_chain_sample(_: &(), cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(5_u32);
    let panel = cx.use_ref::<Option<SwapChainPanelHandle>>(None);
    let (swap_chain, set_swap_chain) = cx.use_async_state::<Option<SendSwap>>(None);
    let render_thread = cx.use_ref::<Option<RenderThread>>(None);

    // Attach the swap chain to the panel once the worker thread reports it.
    cx.use_effect(swap_chain.clone(), {
        let panel = panel.clone();
        move || {
            if let Some(swap) = swap_chain.as_ref()
                && let Some(panel) = panel.borrow().as_ref()
                && let Err(e) = panel.set_swap_chain(swap.swap_chain())
            {
                eprintln!("set_swap_chain failed: {e}");
            }
        }
    });

    // Push the current circle count to the render thread whenever it changes.
    cx.use_effect(count, {
        let render_thread = render_thread.clone();
        move || {
            if let Some(r) = render_thread.borrow().as_ref() {
                r.set_circle_count(count);
            }
        }
    });

    let add = {
        let set_count = set_count.clone();
        move || set_count.call(count + 1)
    };
    let remove = {
        move || {
            if count > 0 {
                set_count.call(count - 1);
            }
        }
    };

    let margin = 16.0;
    grid((
        Element::from(
            swap_chain_panel()
                .on_mounted({
                    let render_thread = render_thread.clone();
                    move |handle| {
                        panel.set(Some(handle));
                        let set_swap_chain = set_swap_chain.clone();
                        render_thread.set(Some(RenderThread::new(count, move |swap| {
                            set_swap_chain.call(Some(swap));
                        })));
                    }
                })
                .on_resize({
                    move |w, h| {
                        if let Some(r) = render_thread.borrow().as_ref() {
                            r.resize(w as u32, h as u32);
                        }
                    }
                })
                .margin(Thickness {
                    left: margin,
                    top: margin,
                    right: margin,
                    bottom: 0.0,
                }),
        )
        .grid_row(0),
        Element::from(
            hstack((
                button("Add circle").on_click(add),
                button("Remove circle").on_click(remove),
            ))
            .spacing(8.0)
            .margin(Thickness::uniform(margin)),
        )
        .grid_row(1),
    ))
    .rows([GridLength::STAR, GridLength::Auto])
    .into()
}
