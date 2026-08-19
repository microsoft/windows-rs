//! The canvas bridge (feature `canvas`).
//!
//! Hosts [`windows-canvas`](windows_canvas) Direct2D content inside reactor without
//! making the canvas crate depend on reactor.

use super::*;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use windows_canvas::{
    ColorF, DrawingSession, GpuDevice, ID2D1DeviceContext, Matrix3x2, SwapChain, device_lost_error,
    is_device_lost,
};
use windows_core::EventRevoker;

/// Per-frame draw context.
pub struct DrawContext<'a> {
    session: DrawingSession<'a>,
    device: &'a GpuDevice,
    /// Width of the drawing surface, in device-independent pixels.
    pub width: f32,
    /// Height of the drawing surface, in device-independent pixels.
    pub height: f32,
    changed: bool,
}

impl<'a> DrawContext<'a> {
    fn finish(self) -> Result<()> {
        self.session.finish()
    }

    /// Returns the GPU device backing this context.
    pub fn device(&self) -> &GpuDevice {
        self.device
    }

    /// Returns `true` on the first frame after device loss or resize.
    pub fn device_changed(&self) -> bool {
        self.changed
    }

    /// Clears the surface to the given color.
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
    make_device: Rc<dyn Fn() -> Result<GpuDevice>>,
    _rendering: Rendering,
    _scale_revoker: Option<EventRevoker>,
}

fn surface_pixels(dip: f32, scale: f32) -> u32 {
    ((dip * scale) as u32).max(1)
}

impl RenderState {
    fn rebuild(&mut self, pixel_width: u32, pixel_height: u32) -> bool {
        let Ok(device) = (self.make_device)() else {
            return false;
        };
        let Ok(mut chain) = device.create_swap_chain(pixel_width, pixel_height) else {
            return false;
        };
        let dpi = 96.0 * self.scale;
        if chain
            .set_dpi(dpi, dpi)
            .and_then(|()| chain.set_composition_scale(self.scale, self.scale))
            .is_err()
        {
            return false;
        }
        let _ = self.panel.set_swap_chain(chain.raw_swap_chain());
        self.device = device;
        self.chain = chain;
        true
    }
}

/// Create an animated canvas that calls `draw` every frame.
///
/// Manages the device, swap chain, resizing, and device-loss recovery.
pub fn animated_canvas(draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static) -> SwapChainPanel {
    animated_canvas_impl(
        Rc::new(GpuDevice::new_or_warp),
        draw,
        RenderMode::Continuous,
        Rc::new(Cell::new(true)),
    )
}

/// Create an animated canvas that renders on a caller-provided [`GpuDevice`].
///
/// Use this to drive the canvas from a device the app already created - for
/// example a process-wide device shared across several surfaces. Because
/// `GpuDevice` is [`Clone`] and a clone shares the same underlying graphics
/// device, one device can back many surfaces. Each surface built by the loop
/// (including those rebuilt after a resize) uses a clone of `device`, so they
/// all share the same underlying graphics device.
///
/// Because the device is caller-owned, device-lost recovery reuses that same
/// device; if you need canvas to recreate the device on loss, use
/// [`animated_canvas`] (which owns its device) instead.
pub fn animated_canvas_with_device(
    device: GpuDevice,
    draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static,
) -> SwapChainPanel {
    animated_canvas_impl(
        Rc::new(move || Ok(device.clone())),
        draw,
        RenderMode::Continuous,
        Rc::new(Cell::new(true)),
    )
}

/// Demand-driven canvas that calls `draw` only when it needs to repaint.
///
/// Like [`animated_canvas`], but `draw` runs only on the first layout and on
/// resize or scale change - not every frame - so an idle window does no GPU
/// work. Use it for size-driven content such as text or a chart.
pub fn canvas(draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static) -> SwapChainPanel {
    animated_canvas_impl(
        Rc::new(GpuDevice::new_or_warp),
        draw,
        RenderMode::Demand,
        Rc::new(Cell::new(true)),
    )
}

/// Requests repaints of a demand-driven [`canvas_invalidated`].
///
/// Get one from [`RenderCx::use_invalidator`], keep drawing state in a
/// [`use_ref`](RenderCx::use_ref), and call [`invalidate`](Self::invalidate)
/// after mutating that state.
#[derive(Clone)]
pub struct Invalidator(Rc<Cell<bool>>);

impl Invalidator {
    /// Creates an invalidator whose first frame paints on mount.
    pub fn new() -> Self {
        Self(Rc::new(Cell::new(true)))
    }

    /// Schedules a repaint on the next frame.
    pub fn invalidate(&self) {
        self.0.set(true);
    }
}

impl Default for Invalidator {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderCx {
    /// Returns a stable [`Invalidator`] for [`canvas_invalidated`], the same one
    /// every render so it can be cloned into event handlers.
    pub fn use_invalidator(&mut self) -> Invalidator {
        self.use_ref(Invalidator::new()).borrow().clone()
    }
}

/// Demand-driven [`canvas()`] that also repaints whenever `inv` is invalidated.
///
/// Keep drawing state in a [`use_ref`](RenderCx::use_ref), mutate it in an event
/// handler, then call [`Invalidator::invalidate`]. Mutating a `use_ref` does not
/// reconcile the tree, so nothing runs between changes.
pub fn canvas_invalidated(
    inv: &Invalidator,
    draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static,
) -> SwapChainPanel {
    animated_canvas_impl(
        Rc::new(GpuDevice::new_or_warp),
        draw,
        RenderMode::Demand,
        inv.0.clone(),
    )
}

/// Whether a canvas repaints every frame or only when something changes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RenderMode {
    Continuous,
    Demand,
}

fn animated_canvas_impl(
    make_device: Rc<dyn Fn() -> Result<GpuDevice>>,
    draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static,
    mode: RenderMode,
    changed: Rc<Cell<bool>>,
) -> SwapChainPanel {
    let state: Rc<RefCell<Option<RenderState>>> = Rc::new(RefCell::new(None));
    let size: Rc<Cell<(f32, f32)>> = Rc::new(Cell::new((0.0, 0.0)));
    let scale: Rc<Cell<f32>> = Rc::new(Cell::new(1.0));
    let draw = Rc::new(draw);

    let ready_state = state.clone();
    let ready_size = size.clone();
    let ready_scale = scale.clone();
    let ready_changed = changed.clone();
    let ready_make_device = make_device.clone();
    let unmount_state = state.clone();
    swap_chain_panel()
        .on_unmounted(move |_| {
            // Break the Rendering callback cycle on unmount.
            *unmount_state.borrow_mut() = None;
        })
        .on_mounted(move |panel| {
            let s = panel.composition_scale().map_or(1.0, |(x, _)| x);
            ready_scale.set(s);

            let (w, h) = ready_size.get();
            let pw = surface_pixels(w, s);
            let ph = surface_pixels(h, s);

            let Ok(device) = (ready_make_device)() else {
                return;
            };
            let Ok(mut chain) = device.create_swap_chain(pw, ph) else {
                return;
            };
            let dpi = 96.0 * s;
            if chain
                .set_dpi(dpi, dpi)
                .and_then(|()| chain.set_composition_scale(s, s))
                .is_err()
            {
                return;
            }
            let _ = panel.set_swap_chain(chain.raw_swap_chain());

            let sc_size = ready_size.clone();
            let sc_scale = ready_scale.clone();
            let sc_state = ready_state.clone();
            let sc_gen = ready_changed.clone();
            let scale_revoker = panel
                .on_composition_scale_changed(move |new_s, _| {
                    sc_scale.set(new_s);
                    let (w, h) = sc_size.get();
                    let pw = surface_pixels(w, new_s);
                    let ph = surface_pixels(h, new_s);
                    let mut borrow = sc_state.borrow_mut();
                    if let Some(rs) = borrow.as_mut() {
                        rs.scale = new_s;
                        let dpi = 96.0 * new_s;
                        _ = rs
                            .chain
                            .resize_with_dpi(pw, ph, dpi, dpi)
                            .and_then(|()| rs.chain.set_composition_scale(new_s, new_s));
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
                    // In demand mode, skip the frame unless something changed
                    // since the last paint. The vsync callback still fires, but
                    // no GPU work happens while the canvas is idle.
                    if mode == RenderMode::Demand && !render_changed.get() {
                        return;
                    }
                    // Consume `changed` only once a frame actually starts. A
                    // device lost at `begin_draw`, mid-draw, or at `present` all
                    // surface as an `Err`/`Ok(false)` that rebuilds the surface.
                    let outcome = rs
                        .chain
                        .begin_draw()
                        .and_then(|session| {
                            let ctx = DrawContext {
                                session,
                                device: &rs.device,
                                width: w,
                                height: h,
                                changed: render_changed.replace(false),
                            };
                            let draw_result = render_draw(&ctx);
                            let end_result = ctx.finish();
                            draw_result.and(end_result)
                        })
                        .and_then(|()| rs.chain.present());

                    if matches!(outcome, Ok(false))
                        || matches!(&outcome, Err(e) if is_device_lost(e.code()))
                    {
                        let pw = surface_pixels(w, rs.scale);
                        let ph = surface_pixels(h, rs.scale);
                        if rs.rebuild(pw, ph) {
                            render_changed.set(true);
                        }
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
                make_device: ready_make_device.clone(),
                _rendering: rendering,
                _scale_revoker: scale_revoker,
            });
        })
        .on_resize(move |w, h| {
            size.set((w as f32, h as f32));
            let s = scale.get();
            let pw = surface_pixels(w as f32, s);
            let ph = surface_pixels(h as f32, s);
            let mut borrow = state.borrow_mut();
            if let Some(rs) = borrow.as_mut() {
                let _ = rs.chain.resize(pw, ph);
                changed.set(true);
            }
        })
}

/// An on-demand Direct2D drawing surface hosted in a reactor UI.
///
/// Draws on demand into a `SurfaceImageSource`, for content that is static between
/// updates. Create it on the UI thread with a shared [`GpuDevice`].
#[derive(Clone, PartialEq, Debug)]
pub struct CanvasImageSource {
    source: SurfaceImageSource,
    pixel_width: i32,
    pixel_height: i32,
    dpi: f32,
    scale: f32,
}

impl CanvasImageSource {
    /// Creates a `width`x`height` DIP surface backed by `device`.
    pub fn new(device: &GpuDevice, width: f32, height: f32, scale: f32) -> Result<Self> {
        let scale = if scale > 0.0 { scale } else { 1.0 };
        let pixel_width = ((width * scale).round() as i32).max(1);
        let pixel_height = ((height * scale).round() as i32).max(1);
        let source = SurfaceImageSource::new(pixel_width, pixel_height)?;
        source.set_device(device.d2d_device())?;
        Ok(Self {
            source,
            pixel_width,
            pixel_height,
            dpi: 96.0 * scale,
            scale,
        })
    }

    /// Redraw and present. Returns `Ok(false)` after device loss.
    pub fn draw(
        &self,
        clear: ColorF,
        f: impl FnOnce(&DrawingSession<'_>) -> Result<()>,
    ) -> Result<bool> {
        let (context, (offset_x, offset_y)) = match self.source.begin_draw::<ID2D1DeviceContext>(
            0,
            0,
            self.pixel_width,
            self.pixel_height,
        ) {
            Ok(v) => v,
            Err(e) if is_device_lost(e.code()) => return Ok(false),
            Err(e) => return Err(e),
        };

        // The atlas offset is in physical pixels; convert it to DIPs.
        let offset =
            Matrix3x2::translation(offset_x as f32 / self.scale, offset_y as f32 / self.scale);

        // Pair every successful `begin_draw` with `end_draw`, even if `f` panics.
        let guard = EndDrawGuard(&self.source);
        let draw_result = {
            let session =
                DrawingSession::from_borrowed_context_with_dpi(&context, offset, self.dpi);
            session.clear(clear);
            f(&session)
        };
        std::mem::forget(guard);

        match self.source.end_draw() {
            Ok(()) => match draw_result {
                Ok(()) => Ok(true),
                Err(e) if is_device_lost(e.code()) => Ok(false),
                Err(e) => Err(e),
            },
            Err(e) if is_device_lost(e.code()) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Associate a new device after device loss, then redraw with
    /// [`draw`](Self::draw).
    pub fn set_device(&self, device: &GpuDevice) -> Result<()> {
        self.source.set_device(device.d2d_device())
    }

    /// The image source to display, for `Image::new`.
    pub fn image_source(&self) -> ImageSource {
        self.source.clone().into()
    }

    /// The rasterization (DPI) scale the surface was allocated at.
    pub fn scale(&self) -> f32 {
        self.scale
    }
}

/// Ends the `SurfaceImageSource` draw on the panic path so a successful
/// `begin_draw` is never left unpaired. On the normal path the guard is
/// forgotten and `draw` calls `end_draw` itself to observe the result.
struct EndDrawGuard<'a>(&'a SurfaceImageSource);

impl Drop for EndDrawGuard<'_> {
    fn drop(&mut self) {
        let _ = self.0.end_draw();
    }
}

struct SwapChainState {
    device: GpuDevice,
    chain: SwapChain,
    panel: SwapChainPanelHandle,
    /// Surface size in device-independent pixels.
    width: f32,
    height: f32,
    /// Rasterization (DPI) scale the swap chain is currently allocated at.
    scale: f32,
    make_device: Rc<dyn Fn() -> Result<GpuDevice>>,
}

impl SwapChainState {
    /// Draws and presents a single frame.
    ///
    /// Returns an [`Err`] whose code satisfies [`is_device_lost`] if the GPU
    /// device was lost (the caller should [`rebuild`](Self::rebuild) and retry);
    /// any other `Err` is a hard failure that should be propagated as-is.
    fn present_frame(&mut self, f: &dyn Fn(&DrawContext<'_>) -> Result<()>) -> Result<()> {
        if self.width <= 0.0 || self.height <= 0.0 {
            return Ok(());
        }
        // Device loss surfaces here as an `Err` whose code `is_device_lost`.
        let session = self.chain.begin_draw()?;
        let ctx = DrawContext {
            session,
            device: &self.device,
            width: self.width,
            height: self.height,
            changed: false,
        };
        let draw_result = f(&ctx);
        let end_result = ctx.finish();
        draw_result?;
        end_result?;
        match self.chain.present() {
            Ok(true) => Ok(()),
            // `SwapChain::present` reports device loss as `Ok(false)` and does not
            // surface the original code, so use the canonical device-lost error.
            Ok(false) => Err(device_lost_error()),
            Err(e) => Err(e),
        }
    }

    /// Recreates the swap chain (on a fresh device from `make_device`) after
    /// device loss and re-attaches it to the panel. Returns `false` on failure.
    fn rebuild(&mut self) -> bool {
        let Ok(device) = (self.make_device)() else {
            return false;
        };
        let pixel_width = surface_pixels(self.width, self.scale);
        let pixel_height = surface_pixels(self.height, self.scale);
        let Ok(mut chain) = device.create_swap_chain(pixel_width, pixel_height) else {
            return false;
        };
        let dpi = 96.0 * self.scale;
        if chain
            .set_dpi(dpi, dpi)
            .and_then(|()| chain.set_composition_scale(self.scale, self.scale))
            .is_err()
        {
            return false;
        }
        if self.panel.set_swap_chain(chain.raw_swap_chain()).is_err() {
            return false;
        }
        self.device = device;
        self.chain = chain;
        true
    }
}

/// An on-demand swap-chain surface hosted on a reactor [`SwapChainPanel`].
///
/// Create it inside [`SwapChainPanel::on_mounted`] so the native control exists
/// before the swap chain is attached.
#[derive(Clone)]
pub struct CanvasSwapChain {
    inner: Rc<RefCell<SwapChainState>>,
}

impl CanvasSwapChain {
    /// Creates a `width`x`height` DIP surface backed by a canvas-owned device.
    pub fn new(panel: &SwapChainPanelHandle, width: f32, height: f32, scale: f32) -> Result<Self> {
        Self::build(panel, Rc::new(GpuDevice::new_or_warp), width, height, scale)
    }

    /// Creates a surface on `panel` backed by a caller-provided [`GpuDevice`].
    ///
    /// Reuses the caller's device, including after device loss.
    pub fn with_device(
        panel: &SwapChainPanelHandle,
        device: &GpuDevice,
        width: f32,
        height: f32,
        scale: f32,
    ) -> Result<Self> {
        let device = device.clone();
        Self::build(
            panel,
            Rc::new(move || Ok(device.clone())),
            width,
            height,
            scale,
        )
    }

    fn build(
        panel: &SwapChainPanelHandle,
        make_device: Rc<dyn Fn() -> Result<GpuDevice>>,
        width: f32,
        height: f32,
        scale: f32,
    ) -> Result<Self> {
        let scale = if scale > 0.0 { scale } else { 1.0 };
        let device = make_device()?;
        let pixel_width = surface_pixels(width, scale);
        let pixel_height = surface_pixels(height, scale);
        let mut chain = device.create_swap_chain(pixel_width, pixel_height)?;
        let dpi = 96.0 * scale;
        chain.set_dpi(dpi, dpi)?;
        chain.set_composition_scale(scale, scale)?;
        panel.set_swap_chain(chain.raw_swap_chain())?;
        Ok(Self {
            inner: Rc::new(RefCell::new(SwapChainState {
                device,
                chain,
                panel: panel.clone(),
                width,
                height,
                scale,
                make_device,
            })),
        })
    }

    /// Draws and presents one frame, retrying once after device loss.
    pub fn draw(&self, f: impl Fn(&DrawContext<'_>) -> Result<()>) -> Result<()> {
        let mut state = self.inner.borrow_mut();
        match state.present_frame(&f) {
            Ok(()) => Ok(()),
            Err(e) if is_device_lost(e.code()) => {
                // Rebuild once, then propagate the retry result.
                if state.rebuild() {
                    state.present_frame(&f)
                } else {
                    Err(e)
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Resizes the surface to `width`x`height` device-independent pixels. A
    /// no-op if the size is unchanged. Redraw with [`draw`](Self::draw) after.
    pub fn resize(&self, width: f32, height: f32) -> Result<()> {
        let mut state = self.inner.borrow_mut();
        if state.width == width && state.height == height {
            return Ok(());
        }
        let pixel_width = surface_pixels(width, state.scale);
        let pixel_height = surface_pixels(height, state.scale);
        state.chain.resize(pixel_width, pixel_height)?;
        state.width = width;
        state.height = height;
        Ok(())
    }

    /// Updates the rasterization (DPI) scale (for example after the window moves
    /// to a monitor with different scaling). A no-op if unchanged. Redraw with
    /// [`draw`](Self::draw) after.
    pub fn set_scale(&self, scale: f32) -> Result<()> {
        let scale = if scale > 0.0 { scale } else { 1.0 };
        let mut state = self.inner.borrow_mut();
        if state.scale == scale {
            return Ok(());
        }
        let pixel_width = surface_pixels(state.width, scale);
        let pixel_height = surface_pixels(state.height, scale);
        let dpi = 96.0 * scale;
        state
            .chain
            .resize_with_dpi(pixel_width, pixel_height, dpi, dpi)?;
        state.chain.set_composition_scale(scale, scale)?;
        state.scale = scale;
        Ok(())
    }

    /// The rasterization (DPI) scale the surface is currently allocated at.
    pub fn scale(&self) -> f32 {
        self.inner.borrow().scale
    }
}
