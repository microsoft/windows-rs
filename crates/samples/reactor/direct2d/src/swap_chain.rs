//! Hosts Direct2D content inside a reactor UI via `SwapChainPanel`, with WinUI
//! buttons to add/remove animated circles. Direct2D rendering runs on a
//! dedicated worker thread that presents into a composition swap chain.
//!
//! The D3D/D2D device is not created here; it is the app-wide shared `Device`
//! taken from the `Gpu` context and handed to the worker thread as an owned
//! `SharedDevice` snapshot.

use crate::device::{Device, Gpu, gpu_context};
use render::{RenderThread, SendSwap};
use std::rc::Rc;
use windows_reactor::*;

/// Direct2D rendering, isolated on a dedicated worker thread. Only the handle
/// to that thread and the swap chain it hands back are visible to the UI.
mod render {
    use std::sync::mpsc::{Receiver, Sender, TryRecvError, channel};
    use std::thread::{self, JoinHandle};
    use std::time::Duration;

    use crate::device::{SharedDevice, is_device_lost};
    use windows::Win32::Foundation::DXGI_STATUS_OCCLUDED;
    use windows::Win32::Graphics::Direct2D::Common::*;
    use windows::Win32::Graphics::Direct2D::*;
    use windows::Win32::Graphics::Dxgi::Common::*;
    use windows::Win32::Graphics::Dxgi::*;
    use windows::core::Result;
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

    /// Wraps `IDXGISwapChain1` so it can be handed back to the UI thread. Sound
    /// because swap chains are agile and only the render thread presents/resizes
    /// this one.
    #[derive(Clone, PartialEq)]
    pub struct SendSwap(IDXGISwapChain1);

    impl SendSwap {
        /// The swap chain, for attaching to the UI-thread presentation surface.
        pub fn swap_chain(&self) -> &IDXGISwapChain1 {
            &self.0
        }
    }

    /// Owns the render thread and the channel to it. Dropping it stops and joins
    /// the thread.
    pub struct RenderThread {
        commands: Sender<RenderCommand>,
        worker: Option<JoinHandle<()>>,
    }

    impl RenderThread {
        /// Spawn the render thread, rendering with the shared `device`.
        /// `on_swap_chain` runs once the swap chain is created, so the caller can
        /// attach it. `on_device_lost` runs if rendering fails from device loss,
        /// after which the worker exits.
        pub fn new(
            device: SharedDevice,
            initial_count: u32,
            on_swap_chain: impl FnOnce(SendSwap) + Send + 'static,
            on_device_lost: impl FnOnce() + Send + 'static,
        ) -> Self {
            let (commands, rx) = channel();
            let worker = thread::spawn(move || {
                match render_loop(rx, device, initial_count, on_swap_chain) {
                    Ok(()) => {}
                    Err(e) if is_device_lost(e.code()) => on_device_lost(),
                    Err(e) => eprintln!("render thread failed: {e}"),
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
        /// Stop and join the worker so it has released its device and swap-chain
        /// references before we return. Joining prevents an orphaned worker from
        /// racing a freshly spawned one when the sample is switched away and back.
        ///
        /// Safe on the UI thread: the worker only blocks in `Present(1)`, which
        /// returns at vblank regardless of the message pump, so it promptly sees
        /// `Shutdown`.
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

    fn create_d2d_state(device: &SharedDevice, width: u32, height: u32) -> Result<D2DState> {
        // Per-thread device context from the shared D2D device.
        let target = unsafe {
            device
                .d2d_device()
                .CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE)?
        };

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

        // Swap-chain creation and the initial bitmap binding are D3D/DXGI interop
        // on the shared device.
        let swap_chain = unsafe {
            device
                .dxgi_factory()
                .CreateSwapChainForComposition(device.d3d_device(), &desc, None)?
        };

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

            // `Present(1)` blocks until vblank to pace the loop. DXGI_STATUS_OCCLUDED
            // is a success status, so check it before `ok()` discards it.
            let present = state.swap_chain.Present(1, DXGI_PRESENT(0));
            if present == DXGI_STATUS_OCCLUDED {
                return Ok(FrameStatus::Idle);
            }
            present.ok()?;
        }

        Ok(FrameStatus::Presented)
    }

    /// Render-thread entry point: drives the animation loop on the shared
    /// `device`, draining commands. `on_swap_chain` runs once after the swap
    /// chain is created.
    fn render_loop(
        rx: Receiver<RenderCommand>,
        device: SharedDevice,
        initial_count: u32,
        on_swap_chain: impl FnOnce(SendSwap),
    ) -> Result<()> {
        let mut size = (400_u32, 300_u32);
        let mut count = initial_count;
        let mut state = create_d2d_state(&device, size.0, size.1)?;

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

/// Hosts the Direct2D render worker behind a `SwapChainPanel`: (re)spawns it for
/// the shared device, attaches the swap chain it produces, pushes circle-count
/// changes, and routes device loss back to the root for recovery.
#[derive(Clone)]
struct RenderHost {
    panel: HookRef<Option<SwapChainPanelHandle>>,
    render_thread: HookRef<Option<RenderThread>>,
    try_start: Rc<dyn Fn()>,
}

impl RenderHost {
    /// Store the panel handle and start the worker once the panel mounts.
    fn mount(&self, handle: SwapChainPanelHandle) {
        self.panel.set(Some(handle));
        (self.try_start)();
    }

    /// Stop and join the worker while the panel and its swap chain still exist,
    /// so it never presents into a destroyed panel.
    fn unmount(&self) {
        // Release the borrow before the worker is dropped, so the join does not
        // run while the ref is held.
        let thread = self.render_thread.borrow_mut().take();
        drop(thread);
    }

    fn resize(&self, width: u32, height: u32) {
        if let Some(r) = self.render_thread.borrow().as_ref() {
            r.resize(width, height);
        }
    }
}

/// Drive the render worker for the shared device, returning a [`RenderHost`] to
/// wire into the `SwapChainPanel`'s lifecycle callbacks.
fn use_render_host(cx: &mut RenderCx, gpu: Option<Gpu>, count: u32) -> RenderHost {
    let device = gpu.as_ref().and_then(Gpu::device);
    let panel = cx.use_ref::<Option<SwapChainPanelHandle>>(None);
    let (swap_chain, set_swap_chain) = cx.use_async_state::<Option<SendSwap>>(None);
    let render_thread = cx.use_ref::<Option<RenderThread>>(None);
    // Which device the live worker was started with, to avoid respawning it for
    // the same device.
    let started_for = cx.use_ref::<Option<Device>>(None);
    // Token marshalled back from the worker on device loss. A fresh token per
    // worker means a new loss always changes the value, firing the recovery
    // effect below.
    let (lost_token, set_lost_token) = cx.use_async_state::<u64>(0);
    let next_token = cx.use_ref::<u64>(0);

    let try_start: Rc<dyn Fn()> = {
        let panel = panel.clone();
        let render_thread = render_thread.clone();
        let device = device.clone();
        Rc::new(move || {
            let Some(dev) = device.clone() else {
                return;
            };
            if panel.borrow().is_none() || started_for.borrow().as_ref() == Some(&dev) {
                return;
            }

            let token = {
                let mut t = next_token.borrow_mut();
                *t += 1;
                *t
            };
            let set_swap_chain = set_swap_chain.clone();
            let set_lost_token = set_lost_token.clone();
            let thread = RenderThread::new(
                dev.to_send(),
                count,
                move |swap| set_swap_chain.call(Some(swap)),
                move || set_lost_token.call(token),
            );
            render_thread.set(Some(thread));
            started_for.set(Some(dev));
        })
    };

    // Attach the swap chain to the panel once the worker reports it.
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

    // (Re)start the worker when the shared device appears or changes.
    cx.use_effect(device, {
        let try_start = try_start.clone();
        move || try_start()
    });

    // On device loss the worker marshals a token here; ask the root to recreate
    // the device. Token 0 means nothing was lost.
    cx.use_effect(lost_token, move || {
        if lost_token != 0
            && let Some(gpu) = gpu.as_ref()
        {
            gpu.request_recovery();
        }
    });

    // Push circle-count changes to the worker.
    cx.use_effect(count, {
        let render_thread = render_thread.clone();
        move || {
            if let Some(r) = render_thread.borrow().as_ref() {
                r.set_circle_count(count);
            }
        }
    });

    RenderHost {
        panel,
        render_thread,
        try_start,
    }
}

/// Sample page: animated Direct2D circles presented through a `SwapChainPanel`,
/// with buttons to add or remove circles.
pub fn swap_chain_sample(_: &(), cx: &mut RenderCx) -> Element {
    let (count, set_count) = cx.use_state(5_u32);
    let gpu = cx.use_context(&gpu_context());
    let host = use_render_host(cx, gpu, count);

    let add = {
        let set_count = set_count.clone();
        move || set_count.call(count + 1)
    };
    let remove = move || {
        if count > 0 {
            set_count.call(count - 1);
        }
    };

    let margin = 16.0;
    grid((
        Element::from(
            swap_chain_panel()
                .on_mounted({
                    let host = host.clone();
                    move |handle| host.mount(handle)
                })
                .on_unmounted({
                    let host = host.clone();
                    move |_handle| host.unmount()
                })
                .on_resize(move |w, h| host.resize(w as u32, h as u32))
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
