use super::*;
use crate::reactor_bindings;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use windows_reactor::{
    Callback, Component, ComponentContext, ElementRef, IntegrationError, SwapChainPanel,
    SwapChainPanelBinding, SwapChainPanelEvent, View, ViewContext,
};

/// Per-frame drawing state for a Reactor canvas.
///
/// `width` and `height` are the current surface dimensions in DIPs.
pub struct DrawContext<'a> {
    session: DrawingSession<'a>,
    device: &'a GpuDevice,
    pub width: f32,
    pub height: f32,
    scale: f32,
    changed: bool,
}

impl DrawContext<'_> {
    /// Returns the device used by this frame.
    pub fn device(&self) -> &GpuDevice {
        self.device
    }

    /// Returns the number of physical pixels per DIP.
    pub fn scale(&self) -> f32 {
        self.scale
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

/// Shared repaint requests for demand-driven canvases.
///
/// Each canvas draws an initial frame. Calling [`invalidate`](Self::invalidate) coalesces with
/// any pending repaint for each attached canvas.
#[derive(Clone)]
pub struct Invalidator(Rc<RefCell<Vec<Weak<CanvasRuntime>>>>);

impl Invalidator {
    /// Creates shared repaint requests.
    pub fn new() -> Self {
        Self(Rc::default())
    }

    /// Requests a future draw callback.
    pub fn invalidate(&self) {
        for runtime in self.0.borrow().iter().filter_map(Weak::upgrade) {
            runtime.invalidate();
        }
    }

    fn attach(&self, runtime: &Rc<CanvasRuntime>) {
        let mut attached = self.0.borrow_mut();
        attached.retain(|runtime| runtime.strong_count() > 0);
        attached.push(Rc::downgrade(runtime));
    }

    fn detach(&self, runtime: &Rc<CanvasRuntime>) {
        let target = Rc::downgrade(runtime);
        self.0
            .borrow_mut()
            .retain(|runtime| !runtime.ptr_eq(&target));
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

    /// Selects the device factory used at surface creation and after device loss.
    ///
    /// For example, pass [`GpuDevice::new_warp`] to use software rendering. The factory must
    /// return a usable device on every call, not keep cloning a removed device. A factory that
    /// shares devices between canvases owns their coordinated replacement.
    pub fn device_factory(mut self, make_device: impl Fn() -> Result<GpuDevice> + 'static) -> Self {
        self.input.make_device = Rc::new(make_device);
        self
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
// Generic so lifecycle tests can use simple values without making binding tokens constructible.
enum AttachmentState<P = SwapChainPanelBinding> {
    Detached,
    Attaching { request: u64, panel: P, chain: u64 },
    Attached { panel: P, chain: u64 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SurfaceLifecycle<P = SwapChainPanelBinding> {
    content: ContentState,
    attachment: AttachmentState<P>,
    next_request: u64,
}

impl<P: Copy + Eq> SurfaceLifecycle<P> {
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

    fn update_panel(&mut self, panel: P) {
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

    fn is_attached(&self, panel: P, chain: u64) -> bool {
        self.attachment == AttachmentState::Attached { panel, chain }
    }

    fn begin_attachment(&mut self, panel: P, chain: u64) -> Option<u64> {
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

    fn complete_attachment(&mut self, request: u64, panel: P, chain: u64, success: bool) -> bool {
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
    dirty: &Cell<bool>,
) -> bool {
    if *current == next {
        return false;
    }
    *current = next;
    dirty.set(true);
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
            runtime: CanvasRuntime::new(input),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.runtime.set_input(input);
    }

    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let runtime = Rc::clone(&self.runtime);
        context.use_effect("lifetime", (), move || {
            runtime.activate();
            Some(Box::new(move || runtime.deactivate()))
        });
        let runtime = Rc::clone(&self.runtime);
        let callback_runtime = Rc::clone(&runtime);
        let mode = input.mode;
        context.use_effect_guard("surface", mode, move || {
            let callback = move |event| callback_runtime.handle_event(event);
            match mode {
                RenderMode::Continuous => runtime.panel.observe_surface(callback),
                RenderMode::Demand => runtime.panel.observe_surface_metrics(callback),
            }
        });
        SwapChainPanel::new()
            .element_ref(&self.runtime.panel)
            .into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ObservedSurface {
    binding: SwapChainPanelBinding,
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
    active: Cell<bool>,
    dirty: Cell<bool>,
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
    fn new(input: &CanvasInput) -> Rc<Self> {
        Rc::new(Self {
            panel: ElementRef::new(),
            input: RefCell::new(input.clone()),
            surface: Cell::new(None),
            state: RefCell::new(None),
            error: Cell::new(None),
            frame_requested: Cell::new(false),
            frame_queued: Cell::new(false),
            frame_running: Cell::new(false),
            recovery_allowed: Cell::new(true),
            active: Cell::new(false),
            dirty: Cell::new(true),
        })
    }

    fn activate(self: &Rc<Self>) {
        self.active.set(true);
        self.input.borrow().invalidator.attach(self);
    }

    fn deactivate(self: &Rc<Self>) {
        self.active.set(false);
        self.input.borrow().invalidator.detach(self);
        self.frame_requested.set(false);
        self.surface.set(None);
        self.state.borrow_mut().take();
        _ = self.panel.request_clear_swap_chain(|_| {});
    }

    fn set_input(self: &Rc<Self>, input: &CanvasInput) {
        let previous = self.input.replace(input.clone());
        if self.active.get() && !Rc::ptr_eq(&previous.invalidator.0, &input.invalidator.0) {
            previous.invalidator.detach(self);
            input.invalidator.attach(self);
        }
        self.error.set(None);
        self.invalidate();
    }

    fn invalidate(self: &Rc<Self>) {
        self.dirty.set(true);
        self.request_frame(true);
    }

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
                let continuous = self.input.borrow().mode == RenderMode::Continuous;
                if continuous {
                    self.recovery_allowed.set(true);
                    self.run_frame();
                }
            }
        }
    }

    fn update_surface(&self, surface: ObservedSurface) {
        self.surface.set(Some(surface));
        self.dirty.set(true);
    }

    fn request_frame(self: &Rc<Self>, external: bool) {
        if !self.active.get() {
            return;
        }
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
        if !self.active.get() || self.frame_running.replace(true) {
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
        if update_surface_metrics(&mut state.metrics, surface.metrics, &self.dirty) {
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
        let make_device = Rc::clone(&self.input.borrow().make_device);
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
        self.dirty.set(true);
        true
    }

    fn request_surface_attachment(self: &Rc<Self>, panel: SwapChainPanelBinding) {
        let request = {
            let mut state_slot = self.state.borrow_mut();
            let Some(state) = state_slot.as_mut() else {
                return;
            };
            let raw: IUnknown = state.chain.swap_chain.clone().into();
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
        panel: SwapChainPanelBinding,
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
                    self.dirty.set(true);
                    self.request_frame(false);
                }
                Err(error) => report_error_ref(&self.input, &self.error, error),
            }
        }
    }

    fn render_frame(self: &Rc<Self>) {
        let (mode, draw) = {
            let input = self.input.borrow();
            (input.mode, Rc::clone(&input.draw))
        };
        if mode == RenderMode::Demand && !self.dirty.get() {
            return;
        }
        if !self.ensure_surface() {
            self.dirty.set(true);
            return;
        }

        let mut state_slot = self.state.borrow_mut();
        let Some(state) = state_slot.as_mut() else {
            return;
        };
        if state.metrics.width <= 0.0 || state.metrics.height <= 0.0 {
            return;
        }
        self.dirty.set(false);
        let outcome = state.chain.begin_draw().and_then(|session| {
            let context = DrawContext {
                session,
                device: &state.device,
                width: state.metrics.width,
                height: state.metrics.height,
                scale: state.metrics.scale,
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
            self.dirty.set(true);
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
        callback.call(error);
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
            native.SetDevice(device.d2d_device()).ok()?;
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
        let begin = unsafe {
            self.native
                .BeginDraw::<ID2D1DeviceContext>(&update, &mut offset)
        };
        if check_device_lost(&begin) {
            return Ok(false);
        }
        let context = begin?;
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
    fn invalidation_reaches_each_live_canvas_without_retaining_retired_canvases() {
        let invalidator = Invalidator::new();
        invalidator.invalidate();
        let input = Canvas::invalidated(&invalidator, |_| Ok(())).input;
        let first = CanvasRuntime::new(&input);
        let second = CanvasRuntime::new(&input);
        assert!(first.dirty.get());
        first.activate();
        second.activate();
        first.dirty.set(false);
        second.dirty.set(false);

        invalidator.invalidate();
        assert!(first.dirty.get());
        assert!(second.dirty.get());
        assert!(first.frame_requested.get());
        assert!(second.frame_requested.get());
        first.dirty.set(false);
        assert!(second.dirty.get());

        first.deactivate();
        invalidator.invalidate();
        assert!(!first.dirty.get());
        assert!(!first.frame_requested.get());
        let weak = Rc::downgrade(&first);
        drop(first);
        assert!(weak.upgrade().is_none());
        second.deactivate();
        assert!(invalidator.0.borrow().is_empty());
    }

    #[test]
    fn replacing_an_invalidator_disconnects_the_previous_controller() {
        let previous = Invalidator::new();
        let next = Invalidator::new();
        let input = Canvas::invalidated(&previous, |_| Ok(())).input;
        let runtime = CanvasRuntime::new(&input);
        runtime.activate();
        runtime.set_input(&Canvas::invalidated(&next, |_| Ok(())).input);
        runtime.dirty.set(false);
        runtime.frame_requested.set(false);

        previous.invalidate();
        assert!(!runtime.dirty.get());
        assert!(!runtime.frame_requested.get());
        next.invalidate();
        assert!(runtime.dirty.get());
        assert!(runtime.frame_requested.get());

        runtime.deactivate();
        assert!(previous.0.borrow().is_empty());
        assert!(next.0.borrow().is_empty());
    }

    #[test]
    fn invalidating_during_a_frame_preserves_the_next_frame_request() {
        let invalidator = Invalidator::new();
        let input = Canvas::invalidated(&invalidator, |_| Ok(())).input;
        let runtime = CanvasRuntime::new(&input);
        runtime.activate();
        runtime.frame_running.set(true);
        runtime.dirty.set(false);

        invalidator.invalidate();
        invalidator.invalidate();
        assert!(runtime.dirty.get());
        assert!(runtime.frame_requested.get());
        assert!(!runtime.frame_queued.get());

        runtime.deactivate();
    }

    #[test]
    fn custom_factory_replaces_the_device_on_rebuild() {
        let attempts = Rc::new(Cell::new(0));
        let created = Rc::clone(&attempts);
        let input = Canvas::invalidated(&Invalidator::new(), |_| Ok(()))
            .device_factory(move || {
                created.set(created.get() + 1);
                GpuDevice::new_warp()
            })
            .input;
        let device = (input.make_device)().unwrap();
        let original = device.d3d_device().clone();
        let chain = device.create_swap_chain(32, 32).unwrap();
        let mut state = RenderState {
            device,
            chain,
            chain_generation: 1,
            metrics: SurfaceMetrics::new(32.0, 32.0, 1.0),
            changed: false,
            lifecycle: SurfaceLifecycle::new(),
        };
        state.lifecycle.require_rebuild();

        rebuild_surface(&mut state, &input.make_device).unwrap();

        assert_eq!(attempts.get(), 2);
        assert_ne!(&original, state.device.d3d_device());
        assert_eq!(state.chain_generation, 2);
        assert!(state.changed);
        assert_eq!(state.lifecycle.content, ContentState::Clean);
        assert_eq!(state.lifecycle.attachment, AttachmentState::Detached);
        state
            .chain
            .begin_draw()
            .unwrap()
            .clear(ColorF::CORNFLOWER_BLUE);
        assert!(!state.chain.is_device_lost());
    }

    #[test]
    fn metric_changes_invalidate_demand_rendering() {
        let dirty = Cell::new(false);
        let mut current = SurfaceMetrics::new(100.0, 80.0, 1.0);

        assert!(!update_surface_metrics(
            &mut current,
            SurfaceMetrics::new(100.0, 80.0, 1.0),
            &dirty
        ));
        assert!(!dirty.get());

        for next in [
            SurfaceMetrics::new(120.0, 80.0, 1.0),
            SurfaceMetrics::new(120.0, 90.0, 1.0),
            SurfaceMetrics::new(120.0, 90.0, 1.5),
        ] {
            dirty.set(false);
            assert!(update_surface_metrics(&mut current, next, &dirty));
            assert!(dirty.get());
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
        let runtime = CanvasRuntime::new(&input);
        let metrics = SurfaceMetrics::new(100.0, 80.0, 1.0);

        assert!(!runtime.initialize_surface(metrics));
        assert!(!runtime.initialize_surface(metrics));
        assert_eq!(attempts.get(), 2);
    }
}
