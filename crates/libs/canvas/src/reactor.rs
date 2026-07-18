use super::*;
use std::cell::RefCell;
use std::rc::Rc;
use windows_reactor::*;

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
    _rendering: Rendering,
    _scale_revoker: Option<EventRevoker>,
}

/// DIP length to physical pixels for surface sizing, guarding against a zero
/// (a swap chain must be at least 1x1).
fn surface_pixels(dip: f32, scale: f32) -> u32 {
    ((dip * scale) as u32).max(1)
}

impl RenderState {
    fn rebuild(&mut self, pixel_width: u32, pixel_height: u32) -> bool {
        let Ok(device) = GpuDevice::new_or_warp() else {
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

/// Create an animated canvas that calls `draw` every frame.
///
/// Handles device creation, swap chain management, resize, and device-lost
/// recovery automatically.
///
/// ```ignore
/// animated_canvas(|ctx| {
///     ctx.clear(ColorF::CORNFLOWER_BLUE);
///     ctx.fill_ellipse(&ellipse, &brush);
/// })
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
    let unmount_state = state.clone();
    swap_chain_panel()
        .on_unmounted(move |_| {
            // Drop the render state in place: this revokes the
            // `CompositionTarget::Rendering` subscription and releases the swap
            // chain. Without it the state would leak forever, because its
            // rendering callback holds an `Rc` back to the cell that owns it.
            *unmount_state.borrow_mut() = None;
        })
        .on_mounted(move |panel| {
            let s = panel.composition_scale().map_or(1.0, |(x, _)| x);
            ready_scale.set(s);

            let (w, h) = ready_size.get();
            let pw = surface_pixels(w, s);
            let ph = surface_pixels(h, s);

            let Ok(device) = GpuDevice::new_or_warp() else {
                return;
            };
            let Ok(mut chain) = device.create_swap_chain(pw, ph) else {
                return;
            };
            let dpi = 96.0 * s;
            chain.set_dpi(dpi, dpi);
            chain.set_composition_scale(s, s);
            let _ = panel.set_swap_chain(chain.raw_swap_chain());

            // Listen for scale changes.
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
                            let pw = surface_pixels(w, rs.scale);
                            let ph = surface_pixels(h, rs.scale);
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
/// Where [`animated_canvas`] presents a new frame every vsync via a swap chain,
/// `CanvasImageSource` draws only when you ask — on a data change, a resize, or a
/// theme switch — into a `SurfaceImageSource`. It is the right tool for content
/// that is static between updates (charts, diagrams, a rendered document page)
/// where a continuous render loop would waste power.
///
/// Create it on the UI thread with a shared [`GpuDevice`], draw with the safe
/// canvas API through [`draw`](Self::draw), and display it by handing
/// [`image_source`](Self::image_source) to `Image::new`. This mirrors Win2D's
/// `CanvasImageSource`.
///
/// ```ignore
/// let surface = CanvasImageSource::new(&device, 256.0, 256.0, scale)?;
/// surface.draw(ColorF::CORNFLOWER_BLUE, |session| {
///     let brush = session.create_solid_brush(ColorF::WHITE).unwrap();
///     session.fill_ellipse(&Ellipse::circle(Vector2::new(128.0, 128.0), 96.0), &brush);
/// })?;
/// let image = Image::new(surface.image_source());
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct CanvasImageSource {
    source: SurfaceImageSource,
    pixel_width: i32,
    pixel_height: i32,
    dpi: f32,
    scale: f32,
}

impl CanvasImageSource {
    /// Create a surface `width`×`height` device-independent pixels in size,
    /// backed by `device`. `scale` is the host element's rasterization (DPI)
    /// scale — `1.0` at 96 DPI, `2.0` at 192 DPI — so the surface is allocated at
    /// physical-pixel resolution and stays crisp. Draw into it, then display it at
    /// the same DIP size.
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

    /// Redraw the surface: clear it to `clear`, run `f` to draw, and present.
    ///
    /// Coordinates in `f` are in device-independent pixels with the surface origin
    /// at `(0, 0)`; the DPI scale and the shared-atlas offset are handled for you.
    /// Returns `Ok(false)` if the GPU device was lost — recreate the device
    /// (e.g. [`GpuDevice::new_or_warp`]), call [`set_device`](Self::set_device),
    /// and draw again.
    pub fn draw(&self, clear: ColorF, f: impl FnOnce(&DrawingSession<'_>)) -> Result<bool> {
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

        unsafe { context.SetDpi(self.dpi, self.dpi) };

        // The atlas offset is in physical pixels; the context draws in DIPs, so
        // scale it back into DIP space before using it as the offset translation.
        let offset =
            Matrix3x2::translation(offset_x as f32 / self.scale, offset_y as f32 / self.scale);

        // Pair every successful `begin_draw` with an `end_draw`, even if `f`
        // panics, so the surface is never left mid-draw.
        let guard = EndDrawGuard(&self.source);
        {
            let session = DrawingSession::from_borrowed_context(&context, offset);
            session.clear(clear);
            f(&session);
        }
        std::mem::forget(guard);

        match self.source.end_draw() {
            Ok(()) => Ok(true),
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
