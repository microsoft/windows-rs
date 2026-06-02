use std::cell::{Cell, RefCell};
use std::rc::Rc;

use windows_reactor::{
    Rendering, SwapChainPanelHandle, SwapChainPanelWidget, on_rendering, swap_chain_panel,
};

use crate::color::Color;
use crate::device::GpuDevice;
use crate::session::DrawingSession;
use crate::swap_chain::SwapChain;

/// Per-frame draw context passed to the user's closure.
pub struct DrawContext<'a> {
    session: DrawingSession<'a>,
    device: &'a GpuDevice,
    /// Width of the render target in DIPs.
    pub width: f32,
    /// Height of the render target in DIPs.
    pub height: f32,
}

impl<'a> DrawContext<'a> {
    /// Access the GPU device (for creating paths, resources, etc.).
    pub fn device(&self) -> &GpuDevice {
        self.device
    }

    /// Clear the render target.
    pub fn clear(&self, color: Color) {
        self.session.clear(color);
    }
}

impl<'a> std::ops::Deref for DrawContext<'a> {
    type Target = DrawingSession<'a>;
    fn deref(&self) -> &Self::Target {
        &self.session
    }
}

struct RenderState {
    device: GpuDevice,
    chain: SwapChain,
    panel: SwapChainPanelHandle,
    _rendering: Rendering,
}

impl RenderState {
    /// Attempt to rebuild the device and swap chain after device-lost.
    fn rebuild(&mut self, width: u32, height: u32) -> bool {
        let Ok(device) = GpuDevice::new() else {
            return false;
        };
        let Ok(chain) = device.create_swap_chain(width, height) else {
            return false;
        };
        let _ = self.panel.set_swap_chain(chain.raw_swap_chain());
        self.device = device;
        self.chain = chain;
        true
    }
}

/// Create an animated canvas widget that calls `draw` every frame.
///
/// Handles device creation, swap chain management, resize, and the
/// rendering loop automatically. If the GPU device is lost (driver update,
/// sleep/wake, etc.), the widget silently recreates the device and resumes.
///
/// The closure receives a [`DrawContext`] with the active drawing session
/// and current dimensions.
///
/// ```ignore
/// animated_canvas(|ctx| {
///     ctx.clear(Color::CORNFLOWER_BLUE);
///     ctx.fill_ellipse(&ellipse, &brush);
/// })
/// .margin(16.0)
/// ```
pub fn animated_canvas(draw: impl Fn(&DrawContext<'_>) + 'static) -> SwapChainPanelWidget {
    let state: Rc<RefCell<Option<RenderState>>> = Rc::new(RefCell::new(None));
    let size: Rc<Cell<(f32, f32)>> = Rc::new(Cell::new((0.0, 0.0)));
    let draw = Rc::new(draw);

    let ready_state = state.clone();
    let ready_size = size.clone();
    swap_chain_panel()
        .on_ready(move |panel| {
            let (w, h) = ready_size.get();
            let pw = (w as u32).max(1);
            let ph = (h as u32).max(1);

            let Ok(device) = GpuDevice::new() else {
                return;
            };
            let Ok(chain) = device.create_swap_chain(pw, ph) else {
                return;
            };
            let _ = panel.set_swap_chain(chain.raw_swap_chain());

            let render_state = ready_state.clone();
            let render_size = ready_size.clone();
            let render_draw = draw.clone();
            let Ok(rendering) = on_rendering(move || {
                let mut borrow = render_state.borrow_mut();
                if let Some(s) = borrow.as_mut() {
                    let (w, h) = render_size.get();
                    if w <= 0.0 || h <= 0.0 {
                        return;
                    }
                    let Ok(session) = s.chain.begin_draw() else {
                        return;
                    };
                    let ctx = DrawContext {
                        session,
                        device: &s.device,
                        width: w,
                        height: h,
                    };
                    render_draw(&ctx);
                    drop(ctx);

                    // present() returns Ok(false) on device-lost.
                    match s.chain.present() {
                        Ok(true) => {}
                        Ok(false) => {
                            // Device lost — rebuild and continue next frame.
                            let pw = (w as u32).max(1);
                            let ph = (h as u32).max(1);
                            s.rebuild(pw, ph);
                        }
                        Err(_) => {}
                    }
                }
            }) else {
                return;
            };

            *ready_state.borrow_mut() = Some(RenderState {
                device,
                chain,
                panel,
                _rendering: rendering,
            });
        })
        .on_resize(move |w, h| {
            size.set((w as f32, h as f32));
            let mut borrow = state.borrow_mut();
            if let Some(s) = borrow.as_mut() {
                let _ = s.chain.resize(w as u32, h as u32);
            }
        })
}
