use super::*;
use crate::reactor_bindings;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use windows_reactor::{
    Callback, Component, ComponentContext, ElementRef, IntegrationError, SwapChainPanel,
    SwapChainPanelEvent, View, ViewContext,
};

/// Per-frame drawing state for a Reactor canvas.
///
/// `width` and `height` are the current surface dimensions in DIPs.
pub struct DrawContext<'a> {
    session: DrawingSession<'a>,
    device: &'a GpuDevice,
    pub width: f32,
    pub height: f32,
    changed: bool,
}

impl DrawContext<'_> {
    /// Returns the device used by this frame.
    pub fn device(&self) -> &GpuDevice {
        self.device
    }

    /// Returns `true` on the first frame and after device-loss recovery or a surface rebuild.
    pub fn device_changed(&self) -> bool {
        self.changed
    }

    /// Clears the frame to `color`.
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

/// Shared repaint state for a demand-driven canvas.
///
/// A new invalidator starts invalidated. Calling [`invalidate`](Self::invalidate) coalesces with
/// any pending repaint.
#[derive(Clone)]
pub struct Invalidator(Rc<Cell<bool>>);

impl Invalidator {
    /// Creates invalidated repaint state.
    pub fn new() -> Self {
        Self(Rc::new(Cell::new(true)))
    }

    /// Requests a future draw callback.
    pub fn invalidate(&self) {
        self.0.set(true);
    }
}

impl Default for Invalidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Configures a Direct2D surface hosted in a Reactor tree.
pub struct Canvas {
    input: CanvasInput,
}

impl Canvas {
    /// Creates a canvas that draws on every composition rendering event.
    pub fn animated(draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static) -> Self {
        Self::new(
            Rc::new(GpuDevice::new_or_warp),
            Rc::new(draw),
            RenderMode::Continuous,
            Invalidator::new(),
        )
    }

    /// Creates a continuously rendered canvas using `device`.
    pub fn animated_with_device(
        device: GpuDevice,
        draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static,
    ) -> Self {
        Self::new(
            Rc::new(move || Ok(device.clone())),
            Rc::new(draw),
            RenderMode::Continuous,
            Invalidator::new(),
        )
    }

    /// Creates a canvas that draws only when `invalidator` requests a frame.
    pub fn invalidated(
        invalidator: &Invalidator,
        draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static,
    ) -> Self {
        Self::new(
            Rc::new(GpuDevice::new_or_warp),
            Rc::new(draw),
            RenderMode::Demand,
            invalidator.clone(),
        )
    }

    /// Replaces the default panic-on-error handler.
    pub fn on_error(mut self, handler: impl Fn(IntegrationError) + 'static) -> Self {
        self.input.on_error = Callback::new(handler);
        self
    }

    fn new(
        make_device: Rc<dyn Fn() -> Result<GpuDevice>>,
        draw: Rc<dyn Fn(&DrawContext<'_>) -> Result<()>>,
        mode: RenderMode,
        invalidator: Invalidator,
    ) -> Self {
        Self {
            input: CanvasInput {
                make_device,
                draw,
                mode,
                invalidator,
                on_error: Callback::new(fail_fast),
            },
        }
    }
}

impl From<Canvas> for View {
    fn from(value: Canvas) -> Self {
        Self::component::<CanvasHost>(value.input)
    }
}

fn fail_fast(error: IntegrationError) {
    panic!("windows-canvas Reactor integration failed: {error:?}");
}

/// Creates a canvas that draws on every composition rendering event.
pub fn animated_canvas(draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static) -> View {
    Canvas::animated(draw).into()
}

/// Creates a continuously rendered canvas using `device`.
pub fn animated_canvas_with_device(
    device: GpuDevice,
    draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static,
) -> View {
    Canvas::animated_with_device(device, draw).into()
}

/// Creates a demand-driven canvas with initially invalidated repaint state.
pub fn canvas(draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static) -> View {
    canvas_invalidated(&Invalidator::new(), draw)
}

/// Creates a demand-driven canvas controlled by `invalidator`.
pub fn canvas_invalidated(
    invalidator: &Invalidator,
    draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static,
) -> View {
    Canvas::invalidated(invalidator, draw).into()
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum RenderMode {
    Continuous,
    Demand,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SurfaceMetrics {
    width: f32,
    height: f32,
    scale: f32,
}

impl SurfaceMetrics {
    fn new(width: f32, height: f32, scale: f32) -> Self {
        Self {
            width,
            height,
            scale: scale.max(f32::EPSILON),
        }
    }

    fn pixel_width(self) -> u32 {
        surface_pixels(self.width, self.scale)
    }

    fn pixel_height(self) -> u32 {
        surface_pixels(self.height, self.scale)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ContentState {
    Clean,
    NeedsResize,
    NeedsRebuild,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AttachmentState {
    Detached,
    Attaching {
        request: u64,
        panel: u64,
        chain: u64,
    },
    Attached {
        panel: u64,
        chain: u64,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SurfaceLifecycle {
    content: ContentState,
    attachment: AttachmentState,
    next_request: u64,
}

impl SurfaceLifecycle {
    fn new() -> Self {
        Self {
            content: ContentState::Clean,
            attachment: AttachmentState::Detached,
            next_request: 0,
        }
    }

    fn require_resize(&mut self) {
        if self.content != ContentState::NeedsRebuild {
            self.content = ContentState::NeedsResize;
        }
    }

    fn require_rebuild(&mut self) {
        self.content = ContentState::NeedsRebuild;
    }

    fn update_panel(&mut self, panel: u64) {
        if !matches!(
            self.attachment,
            AttachmentState::Attaching {
                panel: current, ..
            } | AttachmentState::Attached {
                panel: current, ..
            } if current == panel
        ) {
            self.attachment = AttachmentState::Detached;
        }
    }

    fn replace_chain(&mut self) {
        self.content = ContentState::Clean;
        self.attachment = AttachmentState::Detached;
    }

    fn is_attached(&self, panel: u64, chain: u64) -> bool {
        self.attachment == AttachmentState::Attached { panel, chain }
    }

    fn begin_attachment(&mut self, panel: u64, chain: u64) -> Option<u64> {
        if self.attachment != AttachmentState::Detached {
            return None;
        }
        self.next_request = self.next_request.checked_add(1).unwrap();
        self.attachment = AttachmentState::Attaching {
            request: self.next_request,
            panel,
            chain,
        };
        Some(self.next_request)
    }

    fn complete_attachment(&mut self, request: u64, panel: u64, chain: u64, success: bool) -> bool {
        if self.attachment
            != (AttachmentState::Attaching {
                request,
                panel,
                chain,
            })
        {
            return false;
        }
        self.attachment = if success {
            AttachmentState::Attached { panel, chain }
        } else {
            AttachmentState::Detached
        };
        true
    }
}

fn update_surface_metrics(
    current: &mut SurfaceMetrics,
    next: SurfaceMetrics,
    invalidator: &Invalidator,
) -> bool {
    if *current == next {
        return false;
    }
    *current = next;
    invalidator.invalidate();
    true
}

#[derive(Clone)]
struct CanvasInput {
    make_device: Rc<dyn Fn() -> Result<GpuDevice>>,
    draw: Rc<dyn Fn(&DrawContext<'_>) -> Result<()>>,
    mode: RenderMode,
    invalidator: Invalidator,
    on_error: Callback<IntegrationError>,
}

impl PartialEq for CanvasInput {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.make_device, &other.make_device)
            && Rc::ptr_eq(&self.draw, &other.draw)
            && self.mode == other.mode
            && Rc::ptr_eq(&self.invalidator.0, &other.invalidator.0)
            && self.on_error == other.on_error
    }
}

struct CanvasHost {
    runtime: Rc<CanvasRuntime>,
}

impl Component for CanvasHost {
    type Input = CanvasInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            runtime: Rc::new(CanvasRuntime {
                panel: ElementRef::new(),
                input: RefCell::new(input.clone()),
                surface: Cell::new(None),
                state: RefCell::new(None),
                error: Cell::new(None),
                frame_requested: Cell::new(false),
                frame_queued: Cell::new(false),
                frame_running: Cell::new(false),
                recovery_allowed: Cell::new(true),
            }),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.runtime.error.set(None);
        *self.runtime.input.borrow_mut() = input.clone();
        input.invalidator.invalidate();
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let runtime = Rc::clone(&self.runtime);
        let cleanup = Rc::clone(&self.runtime);
        context.use_effect("surface", (), move || {
            let callback_runtime = Rc::clone(&runtime);
            let observation = runtime.panel.observe_surface(move |event| {
                callback_runtime.handle_event(event);
            });
            Some(Box::new(move || {
                drop(observation);
                cleanup.state.borrow_mut().take();
                _ = cleanup.panel.request_clear_swap_chain(|_| {});
            }))
        });
        SwapChainPanel::new()
            .element_ref(&self.runtime.panel)
            .into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ObservedSurface {
    binding: u64,
    metrics: SurfaceMetrics,
}

struct CanvasRuntime {
    panel: ElementRef<SwapChainPanel>,
    input: RefCell<CanvasInput>,
    surface: Cell<Option<ObservedSurface>>,
    state: RefCell<Option<RenderState>>,
    error: Cell<Option<IntegrationError>>,
    frame_requested: Cell<bool>,
    frame_queued: Cell<bool>,
    frame_running: Cell<bool>,
    recovery_allowed: Cell<bool>,
}

struct RenderState {
    device: GpuDevice,
    chain: SwapChain,
    chain_generation: u64,
    metrics: SurfaceMetrics,
    changed: bool,
    lifecycle: SurfaceLifecycle,
}

impl CanvasRuntime {
    fn handle_event(self: &Rc<Self>, event: SwapChainPanelEvent) {
        match event {
            SwapChainPanelEvent::Metrics {
                binding,
                width,
                height,
                scale_x,
                ..
            } => {
                let metrics = SurfaceMetrics::new(width as f32, height as f32, scale_x);
                self.update_surface(ObservedSurface { binding, metrics });
                self.request_frame(true);
            }
            SwapChainPanelEvent::Rendering => {
                let input = self.input.borrow();
                let should_render =
                    input.mode == RenderMode::Continuous || input.invalidator.0.get();
                drop(input);
                if should_render {
                    self.recovery_allowed.set(true);
                    self.run_frame();
                }
            }
        }
    }

    fn update_surface(&self, surface: ObservedSurface) {
        self.surface.set(Some(surface));
        self.input.borrow().invalidator.invalidate();
    }

    fn request_frame(self: &Rc<Self>, external: bool) {
        self.frame_requested.set(true);
        if external {
            self.recovery_allowed.set(true);
        }
        if self.frame_queued.get() || self.frame_running.get() {
            return;
        }
        let binding = self.surface.get().map(|surface| surface.binding);
        self.frame_queued.set(true);
        let runtime = Rc::clone(self);
        let accepted = self.panel.request_surface_frame(move |result| {
            runtime.frame_queued.set(false);
            match result {
                Ok(()) => {
                    if runtime.frame_requested.get() {
                        runtime.run_frame();
                    }
                }
                Err(error) => {
                    let rebound = runtime.surface.get().map(|surface| surface.binding) != binding;
                    if rebound && runtime.frame_requested.get() {
                        runtime.request_frame(false);
                    } else {
                        runtime.frame_requested.set(false);
                        report_frame_error(&runtime.input, &runtime.error, error);
                    }
                }
            }
        });
        if !accepted {
            self.frame_queued.set(false);
        }
    }

    fn run_frame(self: &Rc<Self>) {
        if self.frame_running.replace(true) {
            return;
        }
        self.frame_requested.set(false);
        self.render_frame();
        self.frame_running.set(false);
        if self.frame_requested.get() {
            self.request_frame(false);
        }
    }
}

fn configure_surface(chain: &mut SwapChain, metrics: SurfaceMetrics) {
    let dpi = 96.0 * metrics.scale;
    chain.set_dpi(dpi, dpi);
    chain.set_composition_scale(metrics.scale, metrics.scale);
}

fn resize_surface(state: &mut RenderState) -> Result<()> {
    state
        .chain
        .resize(state.metrics.pixel_width(), state.metrics.pixel_height())?;
    configure_surface(&mut state.chain, state.metrics);
    Ok(())
}

fn classify_resize_failure(error: &Error) -> ContentState {
    if is_device_lost(error.code()) {
        ContentState::NeedsRebuild
    } else {
        ContentState::NeedsResize
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SurfacePreparation {
    Ready,
    Attach,
    Waiting,
}

impl CanvasRuntime {
    fn synchronize_surface(&self, state: &mut RenderState, surface: ObservedSurface) {
        state.lifecycle.update_panel(surface.binding);
        let invalidator = self.input.borrow().invalidator.clone();
        if update_surface_metrics(&mut state.metrics, surface.metrics, &invalidator) {
            state.changed = true;
            state.lifecycle.require_resize();
        }
    }

    fn prepare_surface(
        &self,
        state: &mut RenderState,
        surface: ObservedSurface,
    ) -> SurfacePreparation {
        if state.lifecycle.content == ContentState::NeedsResize {
            match resize_surface(state) {
                Ok(()) => state.lifecycle.content = ContentState::Clean,
                Err(error) => {
                    state.lifecycle.content = classify_resize_failure(&error);
                    if !is_device_lost(error.code()) {
                        report_error_ref(&self.input, &self.error, native_error(&error));
                    }
                }
            }
        }

        if state.lifecycle.content == ContentState::NeedsRebuild {
            let make_device = Rc::clone(&self.input.borrow().make_device);
            if let Err(error) = rebuild_surface(state, &make_device) {
                report_error_ref(&self.input, &self.error, native_error(&error));
            }
        }

        if state.lifecycle.content != ContentState::Clean {
            return SurfacePreparation::Waiting;
        }
        if state
            .lifecycle
            .is_attached(surface.binding, state.chain_generation)
        {
            SurfacePreparation::Ready
        } else if state.lifecycle.attachment == AttachmentState::Detached {
            SurfacePreparation::Attach
        } else {
            SurfacePreparation::Waiting
        }
    }

    fn ensure_surface(self: &Rc<Self>) -> bool {
        let Some(surface) = self.surface.get() else {
            return false;
        };
        if self.state.borrow().is_none() && !self.initialize_surface(surface.metrics) {
            return false;
        }
        let preparation = {
            let mut state_slot = self.state.borrow_mut();
            let Some(state) = state_slot.as_mut() else {
                return false;
            };
            self.synchronize_surface(state, surface);
            self.prepare_surface(state, surface)
        };
        match preparation {
            SurfacePreparation::Ready => true,
            SurfacePreparation::Attach => {
                self.request_surface_attachment(surface.binding);
                false
            }
            SurfacePreparation::Waiting => false,
        }
    }

    fn initialize_surface(&self, metrics: SurfaceMetrics) -> bool {
        let (make_device, invalidator) = {
            let input = self.input.borrow();
            (Rc::clone(&input.make_device), input.invalidator.clone())
        };
        let device = match make_device() {
            Ok(device) => device,
            Err(error) => {
                report_error_ref(&self.input, &self.error, native_error(&error));
                return false;
            }
        };
        let mut chain =
            match device.create_swap_chain(metrics.pixel_width(), metrics.pixel_height()) {
                Ok(chain) => chain,
                Err(error) => {
                    report_error_ref(&self.input, &self.error, native_error(&error));
                    return false;
                }
            };
        configure_surface(&mut chain, metrics);
        *self.state.borrow_mut() = Some(RenderState {
            device,
            chain,
            chain_generation: 1,
            metrics,
            changed: true,
            lifecycle: SurfaceLifecycle::new(),
        });
        invalidator.invalidate();
        true
    }

    fn request_surface_attachment(self: &Rc<Self>, panel: u64) {
        let request = {
            let mut state_slot = self.state.borrow_mut();
            let Some(state) = state_slot.as_mut() else {
                return;
            };
            let raw = match state.chain.raw_swap_chain().cast::<IUnknown>() {
                Ok(raw) => raw,
                Err(error) => {
                    report_error_ref(&self.input, &self.error, native_error(&error));
                    return;
                }
            };
            let chain = state.chain_generation;
            let Some(request) = state.lifecycle.begin_attachment(panel, chain) else {
                return;
            };
            (raw, request, chain)
        };
        let runtime = Rc::clone(self);
        let accepted = self.panel.request_set_swap_chain(request.0, move |result| {
            runtime.finish_surface_attachment(request.1, panel, request.2, result);
        });
        if !accepted {
            self.finish_surface_attachment(
                request.1,
                panel,
                request.2,
                Err(IntegrationError::Unavailable),
            );
        }
    }

    fn finish_surface_attachment(
        self: &Rc<Self>,
        request: u64,
        panel: u64,
        chain: u64,
        result: std::result::Result<(), IntegrationError>,
    ) {
        let completed = self.state.borrow_mut().as_mut().is_some_and(|state| {
            state
                .lifecycle
                .complete_attachment(request, panel, chain, result.is_ok())
        });
        if completed {
            match result {
                Ok(()) => {
                    self.error.set(None);
                    self.input.borrow().invalidator.invalidate();
                    self.request_frame(false);
                }
                Err(error) => report_error_ref(&self.input, &self.error, error),
            }
        }
    }

    fn render_frame(self: &Rc<Self>) {
        let (mode, invalidator, draw) = {
            let input = self.input.borrow();
            (
                input.mode,
                input.invalidator.clone(),
                Rc::clone(&input.draw),
            )
        };
        if mode == RenderMode::Demand && !invalidator.0.get() {
            return;
        }
        if !self.ensure_surface() {
            invalidator.invalidate();
            return;
        }

        let mut state_slot = self.state.borrow_mut();
        let Some(state) = state_slot.as_mut() else {
            return;
        };
        if state.metrics.width <= 0.0 || state.metrics.height <= 0.0 {
            return;
        }
        invalidator.0.set(false);
        let outcome = state.chain.begin_draw().and_then(|session| {
            let context = DrawContext {
                session,
                device: &state.device,
                width: state.metrics.width,
                height: state.metrics.height,
                changed: std::mem::replace(&mut state.changed, false),
            };
            let result = draw(&context);
            drop(context);
            result
        });
        let outcome = if state.chain.is_device_lost() {
            Ok(false)
        } else {
            outcome.and_then(|()| state.chain.present())
        };
        let needs_rebuild = matches!(outcome, Ok(false))
            || matches!(&outcome, Err(error) if is_device_lost(error.code()));
        if needs_rebuild {
            state.lifecycle.require_rebuild();
            invalidator.invalidate();
        } else {
            match outcome {
                Ok(true) => self.error.set(None),
                Ok(false) => {}
                Err(error) => {
                    report_error_ref(&self.input, &self.error, native_error(&error));
                }
            }
        }
        drop(state_slot);
        if needs_rebuild && self.recovery_allowed.replace(false) {
            self.request_frame(false);
        }
    }
}

fn rebuild_surface(
    state: &mut RenderState,
    make_device: &Rc<dyn Fn() -> Result<GpuDevice>>,
) -> Result<()> {
    let device = make_device()?;
    let mut chain =
        device.create_swap_chain(state.metrics.pixel_width(), state.metrics.pixel_height())?;
    configure_surface(&mut chain, state.metrics);
    state.device = device;
    state.chain = chain;
    state.chain_generation = state.chain_generation.checked_add(1).unwrap();
    state.changed = true;
    state.lifecycle.replace_chain();
    Ok(())
}

fn native_error(error: &Error) -> IntegrationError {
    IntegrationError::Native(error.code().0)
}

fn report_frame_error(
    input: &RefCell<CanvasInput>,
    state: &Cell<Option<IntegrationError>>,
    error: IntegrationError,
) {
    if matches!(error, IntegrationError::Native(_)) {
        report_error_ref(input, state, error);
    }
}

fn report_error_ref(
    input: &RefCell<CanvasInput>,
    state: &Cell<Option<IntegrationError>>,
    error: IntegrationError,
) {
    if state.replace(Some(error)) != Some(error) {
        let callback = input.borrow().on_error.clone();
        _ = callback.call(error);
    }
}

fn surface_pixels(dip: f32, scale: f32) -> u32 {
    ((dip * scale) as u32).max(1)
}

/// An on-demand Direct2D surface that can be assigned to a Reactor image.
#[derive(Clone, Debug, PartialEq)]
pub struct CanvasImageSource {
    source: reactor_bindings::SurfaceImageSource,
    native: reactor_bindings::ISurfaceImageSourceNativeWithD2D,
    pixel_width: i32,
    pixel_height: i32,
    dpi: f32,
    scale: f32,
}

impl CanvasImageSource {
    /// Creates an image surface with DIP dimensions `width` by `height` at `scale` pixels per DIP.
    ///
    /// Non-positive scales use 1.0.
    pub fn new(device: &GpuDevice, width: f32, height: f32, scale: f32) -> Result<Self> {
        let scale = if scale > 0.0 { scale } else { 1.0 };
        let pixel_width = ((width * scale).round() as i32).max(1);
        let pixel_height = ((height * scale).round() as i32).max(1);
        let source = reactor_bindings::SurfaceImageSource::CreateInstanceWithDimensions(
            pixel_width,
            pixel_height,
        )?;
        let native: reactor_bindings::ISurfaceImageSourceNativeWithD2D = source.cast()?;
        unsafe {
            native.SetDevice(device.d2d_device().as_raw()).ok()?;
        }
        Ok(Self {
            source,
            native,
            pixel_width,
            pixel_height,
            dpi: 96.0 * scale,
            scale,
        })
    }

    /// Runs one draw pass and returns `Ok(false)` if any stage reports device loss.
    pub fn draw(
        &self,
        clear: ColorF,
        draw: impl FnOnce(&DrawingSession<'_>) -> Result<()>,
    ) -> Result<bool> {
        let update = reactor_bindings::RECT {
            left: 0,
            top: 0,
            right: self.pixel_width,
            bottom: self.pixel_height,
        };
        let mut offset = reactor_bindings::POINT::default();
        let mut object = std::ptr::null_mut();
        let begin = unsafe {
            self.native
                .BeginDraw(&update, &ID2D1DeviceContext::IID, &mut object, &mut offset)
        };
        if is_device_lost(begin) {
            return Ok(false);
        }
        begin.ok()?;
        let context = unsafe { ID2D1DeviceContext::from_raw(object) };
        let guard = EndImageDraw(&self.native);
        let session = DrawingSession::from_borrowed_context_with_dpi(
            &context,
            Matrix3x2::translation(offset.x as f32 / self.scale, offset.y as f32 / self.scale),
            self.dpi,
        );
        session.clear(clear);
        let result = draw(&session);
        drop(session);
        std::mem::forget(guard);
        let end = unsafe { self.native.EndDraw() }.ok();
        device_lost::classify_draw_results(result, end)
    }

    #[must_use = "false means the image reference is currently unbound"]
    /// Attaches this surface to `image`.
    ///
    /// Returns `false` when the image reference is not currently bound.
    pub fn attach(&self, image: &ElementRef<windows_reactor::Image>) -> bool {
        self.attach_result(image, |result| {
            if let Err(error) = result {
                fail_fast(error);
            }
        })
    }

    #[must_use = "false means the image reference is currently unbound"]
    /// Attaches this surface and reports asynchronous native completion.
    ///
    /// Returns `false` when the image reference is not currently bound; in that case `completion`
    /// is not queued.
    pub fn attach_result(
        &self,
        image: &ElementRef<windows_reactor::Image>,
        completion: impl Fn(std::result::Result<(), IntegrationError>) + 'static,
    ) -> bool {
        image.request_set_native_source(Some(self.source.clone().into()), completion)
    }

    /// Returns the number of physical pixels per DIP.
    pub fn scale(&self) -> f32 {
        self.scale
    }
}

struct EndImageDraw<'a>(&'a reactor_bindings::ISurfaceImageSourceNativeWithD2D);

impl Drop for EndImageDraw<'_> {
    fn drop(&mut self) {
        unsafe {
            _ = self.0.EndDraw();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metric_changes_invalidate_demand_rendering() {
        let invalidator = Invalidator::new();
        let mut current = SurfaceMetrics::new(100.0, 80.0, 1.0);
        invalidator.0.set(false);

        assert!(!update_surface_metrics(
            &mut current,
            SurfaceMetrics::new(100.0, 80.0, 1.0),
            &invalidator
        ));
        assert!(!invalidator.0.get());

        for next in [
            SurfaceMetrics::new(120.0, 80.0, 1.0),
            SurfaceMetrics::new(120.0, 90.0, 1.0),
            SurfaceMetrics::new(120.0, 90.0, 1.5),
        ] {
            invalidator.0.set(false);
            assert!(update_surface_metrics(&mut current, next, &invalidator));
            assert!(invalidator.0.get());
        }
    }

    #[test]
    fn resize_device_loss_requires_rebuild() {
        assert_eq!(
            classify_resize_failure(&device_lost_error()),
            ContentState::NeedsRebuild
        );
        assert_eq!(
            classify_resize_failure(&Error::from_hresult(HRESULT(0x8007_0057_u32 as i32))),
            ContentState::NeedsResize
        );
    }

    #[test]
    fn attachment_requires_a_successful_completion() {
        let mut lifecycle = SurfaceLifecycle::new();
        assert_eq!(lifecycle.attachment, AttachmentState::Detached);

        let initial = lifecycle.begin_attachment(1, 1).unwrap();
        assert!(lifecycle.complete_attachment(initial, 1, 1, false));
        assert_eq!(lifecycle.attachment, AttachmentState::Detached);

        let retry = lifecycle.begin_attachment(1, 1).unwrap();
        assert_ne!(retry, initial);
        assert!(lifecycle.complete_attachment(retry, 1, 1, true));
        assert!(lifecycle.is_attached(1, 1));
    }

    #[test]
    fn stale_attachment_completion_cannot_ready_a_rebuilt_surface() {
        let mut lifecycle = SurfaceLifecycle::new();
        let stale = lifecycle.begin_attachment(1, 1).unwrap();
        lifecycle.require_rebuild();
        lifecycle.replace_chain();
        assert!(!lifecycle.complete_attachment(stale, 1, 1, true));

        let current = lifecycle.begin_attachment(1, 2).unwrap();
        assert_ne!(current, stale);
        assert!(!lifecycle.complete_attachment(stale, 1, 1, true));
        assert!(!lifecycle.is_attached(1, 2));

        assert!(lifecycle.complete_attachment(current, 1, 2, true));
        assert!(lifecycle.is_attached(1, 2));
    }

    #[test]
    fn resize_preserves_current_and_in_flight_attachments() {
        let mut attached = SurfaceLifecycle::new();
        let request = attached.begin_attachment(1, 1).unwrap();
        assert!(attached.complete_attachment(request, 1, 1, true));
        attached.require_resize();
        assert!(attached.is_attached(1, 1));

        let mut attaching = SurfaceLifecycle::new();
        let request = attaching.begin_attachment(1, 1).unwrap();
        attaching.require_resize();
        assert_eq!(
            attaching.attachment,
            AttachmentState::Attaching {
                request,
                panel: 1,
                chain: 1
            }
        );
    }

    #[test]
    fn a_new_panel_binding_requires_attachment() {
        let mut lifecycle = SurfaceLifecycle::new();
        let request = lifecycle.begin_attachment(1, 1).unwrap();
        assert!(lifecycle.complete_attachment(request, 1, 1, true));

        lifecycle.update_panel(1);
        assert!(lifecycle.is_attached(1, 1));
        lifecycle.update_panel(2);
        assert_eq!(lifecycle.attachment, AttachmentState::Detached);
    }

    #[test]
    fn repeated_integration_errors_are_reported_once_per_failure_episode() {
        let reported = Rc::new(RefCell::new(Vec::new()));
        let callback_reported = Rc::clone(&reported);
        let input = Canvas::animated(|_| Ok(()))
            .on_error(move |error| callback_reported.borrow_mut().push(error))
            .input;
        let input = RefCell::new(input);
        let state = Cell::new(None);

        report_error_ref(&input, &state, IntegrationError::Native(-1));
        report_error_ref(&input, &state, IntegrationError::Native(-1));
        report_error_ref(&input, &state, IntegrationError::Unavailable);
        state.set(None);
        report_error_ref(&input, &state, IntegrationError::Native(-1));

        assert_eq!(
            *reported.borrow(),
            [
                IntegrationError::Native(-1),
                IntegrationError::Unavailable,
                IntegrationError::Native(-1)
            ]
        );
    }

    #[test]
    fn retired_frame_requests_do_not_report_canvas_errors() {
        let reported = Rc::new(RefCell::new(Vec::new()));
        let callback_reported = Rc::clone(&reported);
        let input = Canvas::animated(|_| Ok(()))
            .on_error(move |error| callback_reported.borrow_mut().push(error))
            .input;
        let input = RefCell::new(input);
        let state = Cell::new(None);

        report_frame_error(&input, &state, IntegrationError::Unavailable);
        report_frame_error(&input, &state, IntegrationError::Native(-1));

        assert_eq!(*reported.borrow(), [IntegrationError::Native(-1)]);
    }

    #[test]
    fn failed_initialization_can_be_retried_without_new_metrics() {
        let attempts = Rc::new(Cell::new(0));
        let make_device_attempts = Rc::clone(&attempts);
        let input = Canvas::new(
            Rc::new(move || {
                make_device_attempts.set(make_device_attempts.get() + 1);
                Err(Error::from_hresult(HRESULT(0x8000_4005_u32 as i32)))
            }),
            Rc::new(|_| Ok(())),
            RenderMode::Demand,
            Invalidator::new(),
        )
        .on_error(|_| {})
        .input;
        let runtime = CanvasRuntime {
            panel: ElementRef::new(),
            input: RefCell::new(input),
            surface: Cell::new(None),
            state: RefCell::new(None),
            error: Cell::new(None),
            frame_requested: Cell::new(false),
            frame_queued: Cell::new(false),
            frame_running: Cell::new(false),
            recovery_allowed: Cell::new(true),
        };
        let metrics = SurfaceMetrics::new(100.0, 80.0, 1.0);

        assert!(!runtime.initialize_surface(metrics));
        assert!(!runtime.initialize_surface(metrics));
        assert_eq!(attempts.get(), 2);
    }
}
