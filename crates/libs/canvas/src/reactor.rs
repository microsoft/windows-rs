use super::*;
use crate::reactor_bindings;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use windows_reactor::{
    Callback, Component, ComponentContext, ElementRef, IntegrationError, SwapChainPanel,
    SwapChainPanelEvent, View, ViewContext,
};

/// Per-frame draw context.
pub struct DrawContext<'a> {
    session: DrawingSession<'a>,
    device: &'a GpuDevice,
    pub width: f32,
    pub height: f32,
    changed: bool,
}

impl DrawContext<'_> {
    pub fn device(&self) -> &GpuDevice {
        self.device
    }

    pub fn device_changed(&self) -> bool {
        self.changed
    }

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

/// Requests a repaint from a demand-driven canvas.
#[derive(Clone)]
pub struct Invalidator(Rc<Cell<bool>>);

impl Invalidator {
    pub fn new() -> Self {
        Self(Rc::new(Cell::new(true)))
    }

    pub fn invalidate(&self) {
        self.0.set(true);
    }
}

impl Default for Invalidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Configures a Canvas surface hosted in a Reactor tree.
pub struct Canvas {
    input: CanvasInput,
}

impl Canvas {
    pub fn animated(draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static) -> Self {
        Self::new(
            Rc::new(GpuDevice::new_or_warp),
            Rc::new(draw),
            RenderMode::Continuous,
            Invalidator::new(),
        )
    }

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

pub fn animated_canvas(draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static) -> View {
    Canvas::animated(draw).into()
}

pub fn animated_canvas_with_device(
    device: GpuDevice,
    draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static,
) -> View {
    Canvas::animated_with_device(device, draw).into()
}

pub fn canvas(draw: impl Fn(&DrawContext<'_>) -> Result<()> + 'static) -> View {
    canvas_invalidated(&Invalidator::new(), draw)
}

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
enum SurfaceState {
    NeedsResize,
    NeedsRebuild,
    Unattached,
    Attaching(u64),
    Ready,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SurfaceLifecycle {
    state: SurfaceState,
    generation: u64,
}

impl SurfaceLifecycle {
    fn new() -> Self {
        Self {
            state: SurfaceState::Unattached,
            generation: 0,
        }
    }

    fn require_resize(&mut self) {
        if self.state != SurfaceState::NeedsRebuild {
            self.state = SurfaceState::NeedsResize;
        }
    }

    fn require_rebuild(&mut self) {
        self.state = SurfaceState::NeedsRebuild;
    }

    fn require_reattachment(&mut self) {
        if !matches!(
            self.state,
            SurfaceState::NeedsResize | SurfaceState::NeedsRebuild
        ) {
            self.state = SurfaceState::Unattached;
        }
    }

    fn require_attachment(&mut self) {
        self.state = SurfaceState::Unattached;
    }

    fn begin_attachment(&mut self) -> Option<u64> {
        if self.state != SurfaceState::Unattached {
            return None;
        }
        self.generation = self.generation.checked_add(1).unwrap();
        self.state = SurfaceState::Attaching(self.generation);
        Some(self.generation)
    }

    fn complete_attachment(&mut self, generation: u64, success: bool) -> bool {
        if self.state != SurfaceState::Attaching(generation) {
            return false;
        }
        self.state = if success {
            SurfaceState::Ready
        } else {
            SurfaceState::Unattached
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
    panel: ElementRef<SwapChainPanel>,
    input: Rc<RefCell<CanvasInput>>,
    metrics: Rc<Cell<Option<SurfaceMetrics>>>,
    state: Rc<RefCell<Option<RenderState>>>,
    error: Rc<Cell<Option<IntegrationError>>>,
}

impl Component for CanvasHost {
    type Input = CanvasInput;
    type Message = ();

    fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self {
            panel: ElementRef::new(),
            input: Rc::new(RefCell::new(input.clone())),
            metrics: Rc::new(Cell::new(None)),
            state: Rc::new(RefCell::new(None)),
            error: Rc::new(Cell::new(None)),
        }
    }

    fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
        self.error.set(None);
        *self.input.borrow_mut() = input.clone();
        input.invalidator.invalidate();
    }

    fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
        let panel = self.panel.clone();
        let effect_panel = panel.clone();
        let metrics = Rc::clone(&self.metrics);
        let state = Rc::clone(&self.state);
        let input = Rc::clone(&self.input);
        let error = Rc::clone(&self.error);
        context.use_effect("surface", (), move || {
            let callback_panel = effect_panel.clone();
            let cleanup_state = Rc::clone(&state);
            let observation = effect_panel.observe_surface(move |event| {
                handle_surface_event(&callback_panel, &metrics, &state, &input, &error, event);
            });
            Some(Box::new(move || {
                drop(observation);
                cleanup_state.borrow_mut().take();
                _ = panel.request_clear_swap_chain(|_| {});
            }))
        });
        SwapChainPanel::new().element_ref(&self.panel).into()
    }
}

struct RenderState {
    device: GpuDevice,
    chain: SwapChain,
    metrics: SurfaceMetrics,
    changed: bool,
    lifecycle: SurfaceLifecycle,
}

fn handle_surface_event(
    panel: &ElementRef<SwapChainPanel>,
    metrics: &Rc<Cell<Option<SurfaceMetrics>>>,
    state: &Rc<RefCell<Option<RenderState>>>,
    input: &Rc<RefCell<CanvasInput>>,
    error: &Rc<Cell<Option<IntegrationError>>>,
    event: SwapChainPanelEvent,
) {
    match event {
        SwapChainPanelEvent::Metrics {
            width,
            height,
            scale_x,
            ..
        } => {
            let next = SurfaceMetrics::new(width as f32, height as f32, scale_x);
            metrics.set(Some(next));
            update_surface(panel, metrics, state, input, error, next);
        }
        SwapChainPanelEvent::Rendering => render_frame(panel, metrics, state, input, error),
    }
}

fn update_surface(
    panel: &ElementRef<SwapChainPanel>,
    metrics_state: &Rc<Cell<Option<SurfaceMetrics>>>,
    state: &Rc<RefCell<Option<RenderState>>>,
    input: &Rc<RefCell<CanvasInput>>,
    error_state: &Rc<Cell<Option<IntegrationError>>>,
    metrics: SurfaceMetrics,
) {
    let input_value = input.borrow();
    let mut state_slot = state.borrow_mut();
    if let Some(render_state) = state_slot.as_mut() {
        if update_surface_metrics(&mut render_state.metrics, metrics, &input_value.invalidator) {
            render_state.changed = true;
            render_state.lifecycle.require_resize();
        } else {
            render_state.lifecycle.require_reattachment();
        }
        input_value.invalidator.invalidate();
    }
    drop(state_slot);
    drop(input_value);
    _ = ensure_surface(panel, metrics_state, state, input, error_state);
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

fn classify_resize_failure(error: &Error) -> SurfaceState {
    if is_device_lost(error.code()) {
        SurfaceState::NeedsRebuild
    } else {
        SurfaceState::NeedsResize
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SurfacePreparation {
    Ready,
    Attach,
    Waiting,
}

fn prepare_surface(
    state: &mut RenderState,
    make_device: &Rc<dyn Fn() -> Result<GpuDevice>>,
    input: &Rc<RefCell<CanvasInput>>,
    error_state: &Rc<Cell<Option<IntegrationError>>>,
) -> SurfacePreparation {
    if state.lifecycle.state == SurfaceState::NeedsResize {
        match resize_surface(state) {
            Ok(()) => state.lifecycle.require_attachment(),
            Err(error) => {
                state.lifecycle.state = classify_resize_failure(&error);
                if !is_device_lost(error.code()) {
                    report_error(input, error_state, native_error(&error));
                }
            }
        }
    }

    if state.lifecycle.state == SurfaceState::NeedsRebuild
        && let Err(error) = rebuild_surface(state, make_device)
    {
        report_error(input, error_state, native_error(&error));
    }

    match state.lifecycle.state {
        SurfaceState::Ready => SurfacePreparation::Ready,
        SurfaceState::Unattached => SurfacePreparation::Attach,
        SurfaceState::NeedsResize | SurfaceState::NeedsRebuild | SurfaceState::Attaching(_) => {
            SurfacePreparation::Waiting
        }
    }
}

fn ensure_surface(
    panel: &ElementRef<SwapChainPanel>,
    metrics: &Cell<Option<SurfaceMetrics>>,
    state: &Rc<RefCell<Option<RenderState>>>,
    input: &Rc<RefCell<CanvasInput>>,
    error_state: &Rc<Cell<Option<IntegrationError>>>,
) -> bool {
    if state.borrow().is_none() {
        let Some(metrics) = metrics.get() else {
            return false;
        };
        if !initialize_surface(state, input, error_state, metrics) {
            return false;
        }
    }
    let make_device = Rc::clone(&input.borrow().make_device);
    let preparation = {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else {
            return false;
        };
        prepare_surface(state, &make_device, input, error_state)
    };

    match preparation {
        SurfacePreparation::Ready => true,
        SurfacePreparation::Attach => {
            request_surface_attachment(panel, state, input, error_state);
            false
        }
        SurfacePreparation::Waiting => false,
    }
}

fn initialize_surface(
    state: &RefCell<Option<RenderState>>,
    input: &RefCell<CanvasInput>,
    error_state: &Cell<Option<IntegrationError>>,
    metrics: SurfaceMetrics,
) -> bool {
    let (make_device, invalidator) = {
        let input = input.borrow();
        (Rc::clone(&input.make_device), input.invalidator.clone())
    };
    let device = match make_device() {
        Ok(device) => device,
        Err(error) => {
            report_error_ref(input, error_state, native_error(&error));
            return false;
        }
    };
    let mut chain = match device.create_swap_chain(metrics.pixel_width(), metrics.pixel_height()) {
        Ok(chain) => chain,
        Err(error) => {
            report_error_ref(input, error_state, native_error(&error));
            return false;
        }
    };
    configure_surface(&mut chain, metrics);
    *state.borrow_mut() = Some(RenderState {
        device,
        chain,
        metrics,
        changed: true,
        lifecycle: SurfaceLifecycle::new(),
    });
    invalidator.invalidate();
    true
}

fn request_surface_attachment(
    panel: &ElementRef<SwapChainPanel>,
    state: &Rc<RefCell<Option<RenderState>>>,
    input: &Rc<RefCell<CanvasInput>>,
    error_state: &Rc<Cell<Option<IntegrationError>>>,
) {
    let request = {
        let mut state = state.borrow_mut();
        let Some(state) = state.as_mut() else {
            return;
        };
        let raw = match state.chain.raw_swap_chain().cast::<IUnknown>() {
            Ok(raw) => raw,
            Err(error) => {
                report_error(input, error_state, native_error(&error));
                return;
            }
        };
        let Some(generation) = state.lifecycle.begin_attachment() else {
            return;
        };
        (raw, generation)
    };

    let callback_state = Rc::clone(state);
    let callback_input = Rc::clone(input);
    let callback_error = Rc::clone(error_state);
    let generation = request.1;
    let accepted = panel.request_set_swap_chain(request.0, move |result| {
        finish_surface_attachment(
            &callback_state,
            &callback_input,
            &callback_error,
            generation,
            result,
        );
    });
    if !accepted {
        finish_surface_attachment(
            state,
            input,
            error_state,
            generation,
            Err(IntegrationError::Unavailable),
        );
    }
}

fn finish_surface_attachment(
    state: &RefCell<Option<RenderState>>,
    input: &RefCell<CanvasInput>,
    error_state: &Cell<Option<IntegrationError>>,
    generation: u64,
    result: std::result::Result<(), IntegrationError>,
) {
    let completed = state.borrow_mut().as_mut().is_some_and(|state| {
        state
            .lifecycle
            .complete_attachment(generation, result.is_ok())
    });
    if completed {
        match result {
            Ok(()) => error_state.set(None),
            Err(error) => report_error_ref(input, error_state, error),
        }
        input.borrow().invalidator.invalidate();
    }
}

fn render_frame(
    panel: &ElementRef<SwapChainPanel>,
    metrics: &Cell<Option<SurfaceMetrics>>,
    state: &Rc<RefCell<Option<RenderState>>>,
    input: &Rc<RefCell<CanvasInput>>,
    error_state: &Rc<Cell<Option<IntegrationError>>>,
) {
    let (mode, invalidator, draw) = {
        let input = input.borrow();
        (
            input.mode,
            input.invalidator.clone(),
            Rc::clone(&input.draw),
        )
    };
    if mode == RenderMode::Demand && !invalidator.0.get() {
        return;
    }
    if !ensure_surface(panel, metrics, state, input, error_state) {
        invalidator.invalidate();
        return;
    }

    let mut state_slot = state.borrow_mut();
    let Some(render_state) = state_slot.as_mut() else {
        return;
    };
    if render_state.metrics.width <= 0.0 || render_state.metrics.height <= 0.0 {
        return;
    }
    invalidator.0.set(false);
    let outcome = render_state.chain.begin_draw().and_then(|session| {
        let context = DrawContext {
            session,
            device: &render_state.device,
            width: render_state.metrics.width,
            height: render_state.metrics.height,
            changed: std::mem::replace(&mut render_state.changed, false),
        };
        let result = draw(&context);
        drop(context);
        result
    });
    let outcome = if render_state.chain.is_device_lost() {
        Ok(false)
    } else {
        outcome.and_then(|()| render_state.chain.present())
    };
    let needs_rebuild = matches!(outcome, Ok(false))
        || matches!(&outcome, Err(error) if is_device_lost(error.code()));
    if needs_rebuild {
        render_state.lifecycle.require_rebuild();
        invalidator.invalidate();
    } else {
        match outcome {
            Ok(true) => error_state.set(None),
            Ok(false) => {}
            Err(error) => report_error(input, error_state, native_error(&error)),
        }
    }
    drop(state_slot);
    if needs_rebuild {
        _ = ensure_surface(panel, metrics, state, input, error_state);
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
    state.changed = true;
    state.lifecycle.require_attachment();
    Ok(())
}

fn native_error(error: &Error) -> IntegrationError {
    IntegrationError::Native(error.code().0)
}

fn report_error(
    input: &Rc<RefCell<CanvasInput>>,
    state: &Rc<Cell<Option<IntegrationError>>>,
    error: IntegrationError,
) {
    report_error_ref(input, state, error);
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

    /// Draws the surface and returns `Ok(false)` if any stage reports device loss.
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
    pub fn attach(&self, image: &ElementRef<windows_reactor::Image>) -> bool {
        self.attach_result(image, |result| {
            if let Err(error) = result {
                fail_fast(error);
            }
        })
    }

    #[must_use = "false means the image reference is currently unbound"]
    pub fn attach_result(
        &self,
        image: &ElementRef<windows_reactor::Image>,
        completion: impl Fn(std::result::Result<(), IntegrationError>) + 'static,
    ) -> bool {
        image.request_set_native_source(Some(self.source.clone().into()), completion)
    }

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
            SurfaceState::NeedsRebuild
        );
        assert_eq!(
            classify_resize_failure(&Error::from_hresult(HRESULT(0x8007_0057_u32 as i32))),
            SurfaceState::NeedsResize
        );
    }

    #[test]
    fn attachment_requires_a_successful_completion() {
        let mut lifecycle = SurfaceLifecycle::new();
        assert_eq!(lifecycle.state, SurfaceState::Unattached);

        let initial = lifecycle.begin_attachment().unwrap();
        assert_eq!(lifecycle.state, SurfaceState::Attaching(initial));
        assert!(lifecycle.complete_attachment(initial, false));
        assert_eq!(lifecycle.state, SurfaceState::Unattached);

        let retry = lifecycle.begin_attachment().unwrap();
        assert_ne!(retry, initial);
        assert!(lifecycle.complete_attachment(retry, true));
        assert_eq!(lifecycle.state, SurfaceState::Ready);

        lifecycle.require_reattachment();
        let reattachment = lifecycle.begin_attachment().unwrap();
        assert_eq!(lifecycle.state, SurfaceState::Attaching(reattachment));
        assert!(lifecycle.complete_attachment(reattachment, true));
        assert_eq!(lifecycle.state, SurfaceState::Ready);
    }

    #[test]
    fn stale_attachment_completion_cannot_ready_a_rebuilt_surface() {
        let mut lifecycle = SurfaceLifecycle::new();
        let stale = lifecycle.begin_attachment().unwrap();
        lifecycle.require_rebuild();
        assert!(!lifecycle.complete_attachment(stale, true));
        assert_eq!(lifecycle.state, SurfaceState::NeedsRebuild);

        lifecycle.require_attachment();
        let current = lifecycle.begin_attachment().unwrap();
        assert_ne!(current, stale);
        assert!(!lifecycle.complete_attachment(stale, true));
        assert_eq!(lifecycle.state, SurfaceState::Attaching(current));

        assert!(lifecycle.complete_attachment(current, true));
        assert_eq!(lifecycle.state, SurfaceState::Ready);
    }

    #[test]
    fn repeated_integration_errors_are_reported_once_per_failure_episode() {
        let reported = Rc::new(RefCell::new(Vec::new()));
        let callback_reported = Rc::clone(&reported);
        let input = Canvas::animated(|_| Ok(()))
            .on_error(move |error| callback_reported.borrow_mut().push(error))
            .input;
        let input = Rc::new(RefCell::new(input));
        let state = Rc::new(Cell::new(None));

        report_error(&input, &state, IntegrationError::Native(-1));
        report_error(&input, &state, IntegrationError::Native(-1));
        report_error(&input, &state, IntegrationError::Unavailable);
        state.set(None);
        report_error(&input, &state, IntegrationError::Native(-1));

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
        let state = RefCell::new(None);
        let error = Cell::new(None);
        let metrics = SurfaceMetrics::new(100.0, 80.0, 1.0);

        assert!(!initialize_surface(
            &state,
            &RefCell::new(input.clone()),
            &error,
            metrics
        ));
        assert!(!initialize_surface(
            &state,
            &RefCell::new(input),
            &error,
            metrics
        ));
        assert_eq!(attempts.get(), 2);
    }
}
