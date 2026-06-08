use std::cell::{Cell, RefCell};
use std::rc::Rc;

use windows_reactor::{
    Rendering, SwapChainPanel, SwapChainPanelHandle, on_rendering, swap_chain_panel,
};

use crate::color::ColorF;
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
    changed: bool,
}

impl<'a> DrawContext<'a> {
    /// Access the GPU device (for creating paths, resources, etc.).
    pub fn device(&self) -> &GpuDevice {
        self.device
    }

    /// Returns `true` on the first frame after device loss or resize.
    ///
    /// Use this to recreate cached device-dependent resources (brushes,
    /// bitmap targets, effects).
    pub fn device_changed(&self) -> bool {
        self.changed
    }

    /// Clear the render target.
    pub fn clear(&self, color: ColorF) {
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
    scale: f32,
    _rendering: Rendering,
    _scale_revoker: Option<windows_core::EventRevoker>,
}

impl RenderState {
    fn rebuild(&mut self, pixel_width: u32, pixel_height: u32) -> bool {
        let Ok(device) = GpuDevice::new() else {
            return false;
        };
        let Ok(mut chain) = device.create_swap_chain(pixel_width, pixel_height) else {
            return false;
        };
        let dpi = 96.0 * self.scale;
        chain.set_dpi(dpi, dpi);
        chain.set_composition_scale(self.scale, self.scale);
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
/// The swap chain is created at the display's native pixel resolution for
/// crisp rendering. The `DrawContext` reports dimensions in DIPs so draw
/// code is resolution-independent.
///
/// ```ignore
/// animated_canvas(|ctx| {
///     ctx.clear(ColorF::CORNFLOWER_BLUE);
///     ctx.fill_ellipse(&ellipse, &brush);
/// })
/// .margin(16.0)
/// ```
pub fn animated_canvas(draw: impl Fn(&DrawContext<'_>) + 'static) -> SwapChainPanel {
    let state: Rc<RefCell<Option<RenderState>>> = Rc::new(RefCell::new(None));
    let size: Rc<Cell<(f32, f32)>> = Rc::new(Cell::new((0.0, 0.0)));
    let scale: Rc<Cell<f32>> = Rc::new(Cell::new(1.0));
    let changed: Rc<Cell<bool>> = Rc::new(Cell::new(true));
    let draw = Rc::new(draw);

    let ready_state = state.clone();
    let ready_size = size.clone();
    let ready_scale = scale.clone();
    let ready_changed = changed.clone();
    swap_chain_panel()
        .on_ready(move |panel| {
            let s = panel.composition_scale().map_or(1.0, |(x, _)| x);
            ready_scale.set(s);

            let (w, h) = ready_size.get();
            let pw = ((w * s) as u32).max(1);
            let ph = ((h * s) as u32).max(1);

            let Ok(device) = GpuDevice::new() else {
                return;
            };
            let Ok(mut chain) = device.create_swap_chain(pw, ph) else {
                return;
            };
            let dpi = 96.0 * s;
            chain.set_dpi(dpi, dpi);
            chain.set_composition_scale(s, s);
            let _ = panel.set_swap_chain(chain.raw_swap_chain());

            // Listen for scale changes (e.g., window moved to different monitor).
            let sc_size = ready_size.clone();
            let sc_scale = ready_scale.clone();
            let sc_state = ready_state.clone();
            let sc_gen = ready_changed.clone();
            let scale_revoker = panel
                .on_composition_scale_changed(move |new_s, _| {
                    sc_scale.set(new_s);
                    let (w, h) = sc_size.get();
                    let pw = ((w * new_s) as u32).max(1);
                    let ph = ((h * new_s) as u32).max(1);
                    let mut borrow = sc_state.borrow_mut();
                    if let Some(rs) = borrow.as_mut() {
                        rs.scale = new_s;
                        let _ = rs.chain.resize(pw, ph);
                        let dpi = 96.0 * new_s;
                        rs.chain.set_dpi(dpi, dpi);
                        rs.chain.set_composition_scale(new_s, new_s);
                        sc_gen.set(true);
                    }
                })
                .ok();

            let render_state = ready_state.clone();
            let render_size = ready_size.clone();
            let render_draw = draw.clone();
            let render_changed = ready_changed.clone();
            let Ok(rendering) = on_rendering(move || {
                let mut borrow = render_state.borrow_mut();
                if let Some(rs) = borrow.as_mut() {
                    let (w, h) = render_size.get();
                    if w <= 0.0 || h <= 0.0 {
                        return;
                    }
                    let Ok(session) = rs.chain.begin_draw() else {
                        return;
                    };
                    let ctx = DrawContext {
                        session,
                        device: &rs.device,
                        width: w,
                        height: h,
                        changed: render_changed.replace(false),
                    };
                    render_draw(&ctx);
                    drop(ctx);

                    match rs.chain.present() {
                        Ok(true) => {}
                        Ok(false) => {
                            let pw = ((w * rs.scale) as u32).max(1);
                            let ph = ((h * rs.scale) as u32).max(1);
                            if rs.rebuild(pw, ph) {
                                render_changed.set(true);
                            }
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
                scale: s,
                _rendering: rendering,
                _scale_revoker: scale_revoker,
            });
        })
        .on_resize(move |w, h| {
            size.set((w as f32, h as f32));
            let s = scale.get();
            let pw = ((w as f32 * s) as u32).max(1);
            let ph = ((h as f32 * s) as u32).max(1);
            let mut borrow = state.borrow_mut();
            if let Some(rs) = borrow.as_mut() {
                let _ = rs.chain.resize(pw, ph);
                changed.set(true);
            }
        })
}
