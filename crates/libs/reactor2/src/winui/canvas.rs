use super::*;

fn accept_invalidation_revision(current: &mut u64, revision: u64, source_changed: bool) -> bool {
    if source_changed || revision > *current {
        *current = revision;
        true
    } else {
        false
    }
}
use crate::canvas::{
    CanvasDrawCallback, CanvasDrawContext, SwapChainHostFactory, SwapChainHostFrame,
    SwapChainHostFrameCallback, SwapChainHostLayout, SwapChainHostLayoutCallback,
};

#[derive(Clone, Copy, PartialEq)]
pub(super) struct NativeCanvasLayout {
    pub(super) width: f32,
    pub(super) height: f32,
    pub(super) scale_x: f32,
    pub(super) scale_y: f32,
}

pub(super) struct SwapChainCanvasState {
    _value: bindings::SwapChainPanel,
    ui: bindings::UIElement,
    framework: bindings::FrameworkElement,
    native: bindings::ISwapChainPanelNative,
    metrics: Rc<Cell<NativeCanvasLayout>>,
    applied_metrics: NativeCanvasLayout,
    loaded: Rc<Cell<bool>>,
    xaml_root: Option<bindings::XamlRoot>,
    scale_revoker: Option<windows_core::EventRevoker>,
    draw: Option<CanvasDrawCallback>,
    device: Option<windows_canvas::GpuDevice>,
    swap_chain: Option<windows_canvas::SwapChain>,
    invalidation_revision: u64,
    continuous: bool,
    rendering: Option<windows_core::EventRevoker>,
    dirty: bool,
    frame_queued: Rc<Cell<bool>>,
    device_changed: bool,
    surface_changed: bool,
    _revokers: [windows_core::EventRevoker; 2],
}

pub(super) struct CanvasImageState {
    value: bindings::Image,
    ui: bindings::UIElement,
    framework: bindings::FrameworkElement,
    metrics: Rc<Cell<NativeCanvasLayout>>,
    applied_metrics: NativeCanvasLayout,
    loaded: Rc<Cell<bool>>,
    xaml_root: Option<bindings::XamlRoot>,
    scale_revoker: Option<windows_core::EventRevoker>,
    source: Option<bindings::SurfaceImageSource>,
    native: Option<bindings::ISurfaceImageSourceNativeWithD2D>,
    draw: Option<CanvasDrawCallback>,
    device: Option<windows_canvas::GpuDevice>,
    invalidation_revision: u64,
    dirty: bool,
    frame_queued: bool,
    device_changed: bool,
    surface_changed: bool,
    _revokers: [windows_core::EventRevoker; 2],
}

pub(super) struct SwapChainHostState {
    _value: bindings::SwapChainPanel,
    ui: bindings::UIElement,
    framework: bindings::FrameworkElement,
    native: bindings::ISwapChainPanelNative,
    metrics: Rc<Cell<SwapChainHostLayout>>,
    applied_layout: SwapChainHostLayout,
    loaded: Rc<Cell<bool>>,
    xaml_root: Option<bindings::XamlRoot>,
    scale_revoker: Option<windows_core::EventRevoker>,
    factory: Option<SwapChainHostFactory>,
    layout: Option<SwapChainHostLayoutCallback>,
    frame: Option<SwapChainHostFrameCallback>,
    content: Option<Box<dyn std::any::Any>>,
    swap_chain: Option<windows_canvas::SwapChain>,
    pending_actions: Vec<crate::canvas::SwapChainHostAction>,
    continuous: bool,
    rendering: Option<windows_core::EventRevoker>,
    frame_queued: Rc<Cell<bool>>,
    frame_revision: u64,
    dirty: bool,
    device_changed: bool,
    surface_changed: bool,
    _revokers: [windows_core::EventRevoker; 2],
}

impl CanvasImageState {
    pub(super) fn ui_element(&self) -> bindings::UIElement {
        self.ui.clone()
    }

    pub(super) fn framework_element(&self) -> bindings::FrameworkElement {
        self.framework.clone()
    }
}

impl SwapChainCanvasState {
    pub(super) fn ui_element(&self) -> bindings::UIElement {
        self.ui.clone()
    }

    pub(super) fn framework_element(&self) -> bindings::FrameworkElement {
        self.framework.clone()
    }

    pub(super) fn detach(&self) -> WindowsResult<()> {
        unsafe { self.native.SetSwapChain(core::ptr::null_mut()).ok() }
    }
}

impl SwapChainHostState {
    pub(super) fn ui_element(&self) -> bindings::UIElement {
        self.ui.clone()
    }

    pub(super) fn framework_element(&self) -> bindings::FrameworkElement {
        self.framework.clone()
    }

    pub(super) fn detach(&mut self) -> WindowsResult<()> {
        unsafe { self.native.SetSwapChain(core::ptr::null_mut()).ok()? };
        self.rendering = None;
        self.pending_actions.clear();
        self.content = None;
        self.swap_chain = None;
        Ok(())
    }
}

fn queue_canvas_layout(
    target: NodeId,
    metrics: NativeCanvasLayout,
    events: &RefCell<VecDeque<NativeEvent>>,
    waker: &RefCell<Option<Rc<dyn Fn()>>>,
) {
    queue_latest_event(
        events,
        NativeEvent::CanvasLayout {
            target,
            width: metrics.width,
            height: metrics.height,
            scale_x: metrics.scale_x,
            scale_y: metrics.scale_y,
        },
    );
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}

fn queue_swap_chain_host_layout(
    target: NodeId,
    layout: SwapChainHostLayout,
    events: &RefCell<VecDeque<NativeEvent>>,
    waker: &RefCell<Option<Rc<dyn Fn()>>>,
) {
    queue_latest_event(
        events,
        NativeEvent::SwapChainHostLayout {
            target,
            layout: Box::new(layout),
        },
    );
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}

fn queue_swap_chain_host_frame(
    target: NodeId,
    state: &SwapChainHostState,
    events: &RefCell<VecDeque<NativeEvent>>,
    waker: &RefCell<Option<Rc<dyn Fn()>>>,
) {
    if state.applied_layout.pixel_width == 0 || state.applied_layout.pixel_height == 0 {
        return;
    }
    if state.frame_queued.replace(true) {
        return;
    }
    events
        .borrow_mut()
        .push_back(NativeEvent::SwapChainHostFrame { target });
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}

fn queue_canvas_frame(
    target: NodeId,
    state: &mut SwapChainCanvasState,
    events: &RefCell<VecDeque<NativeEvent>>,
    waker: &RefCell<Option<Rc<dyn Fn()>>>,
) {
    let metrics = state.applied_metrics;
    if state.frame_queued.get() || metrics.width <= 0.0 || metrics.height <= 0.0 {
        return;
    }
    state.frame_queued.set(true);
    events
        .borrow_mut()
        .push_back(NativeEvent::CanvasFrame { target });
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}

fn queue_canvas_image_layout(
    target: NodeId,
    metrics: NativeCanvasLayout,
    events: &RefCell<VecDeque<NativeEvent>>,
    waker: &RefCell<Option<Rc<dyn Fn()>>>,
) {
    queue_latest_event(
        events,
        NativeEvent::CanvasImageLayout {
            target,
            width: metrics.width,
            height: metrics.height,
            scale: metrics.scale_x,
        },
    );
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}

fn queue_canvas_image_frame(
    target: NodeId,
    state: &mut CanvasImageState,
    events: &RefCell<VecDeque<NativeEvent>>,
    waker: &RefCell<Option<Rc<dyn Fn()>>>,
) {
    let metrics = state.applied_metrics;
    if state.frame_queued || metrics.width <= 0.0 || metrics.height <= 0.0 {
        return;
    }
    state.frame_queued = true;
    events
        .borrow_mut()
        .push_back(NativeEvent::CanvasImageFrame { target });
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}

struct EndSurfaceDrawGuard<'a>(&'a bindings::ISurfaceImageSourceNativeWithD2D);

impl Drop for EndSurfaceDrawGuard<'_> {
    fn drop(&mut self) {
        unsafe {
            _ = self.0.EndDraw();
        }
    }
}

impl WinUiRuntime {
    pub(super) fn create_canvas_image(&mut self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::Image::new()?;
        value.SetStretch(bindings::Stretch::Fill)?;
        let ui = value.cast()?;
        let framework: bindings::FrameworkElement = value.cast()?;
        let metrics = Rc::new(Cell::new(NativeCanvasLayout {
            width: 0.0,
            height: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }));
        let loaded = Rc::new(Cell::new(false));

        let size_metrics = Rc::clone(&metrics);
        let size_events = Rc::clone(&self.events);
        let size_waker = Rc::clone(&self.waker);
        let size_changed = framework.SizeChanged(move |_sender, args| {
            let args = args.as_ref().unwrap();
            let size = args.NewSize().unwrap();
            let mut current = size_metrics.get();
            current.width = size.width;
            current.height = size.height;
            size_metrics.set(current);
            queue_canvas_image_layout(id, current, &size_events, &size_waker);
        })?;

        let loaded_metrics = Rc::clone(&metrics);
        let loaded_state = Rc::clone(&loaded);
        let loaded_events = Rc::clone(&self.events);
        let loaded_waker = Rc::clone(&self.waker);
        let loaded_revoker = framework.Loaded(move |_sender, _args| {
            loaded_state.set(true);
            queue_canvas_image_layout(id, loaded_metrics.get(), &loaded_events, &loaded_waker);
        })?;

        Ok(Handle::CanvasImage(Box::new(CanvasImageState {
            value,
            ui,
            framework,
            metrics,
            applied_metrics: NativeCanvasLayout {
                width: 0.0,
                height: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
            },
            loaded,
            xaml_root: None,
            scale_revoker: None,
            source: None,
            native: None,
            draw: None,
            device: None,
            invalidation_revision: 0,
            dirty: true,
            frame_queued: false,
            device_changed: true,
            surface_changed: true,
            _revokers: [size_changed, loaded_revoker],
        })))
    }

    pub(super) fn apply_canvas_image_update(
        &mut self,
        id: NodeId,
        update: &CanvasUpdate,
    ) -> WindowsResult<()> {
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let Handle::CanvasImage(state) = &mut self.node_mut(id)?.handle else {
            panic!("CanvasImage update target is not a CanvasImage");
        };
        match update {
            CanvasUpdate::Props {
                draw,
                invalidation_revision,
            } => {
                if state.draw.as_ref() != Some(draw) {
                    state.draw = Some(draw.clone());
                }
                if accept_invalidation_revision(
                    &mut state.invalidation_revision,
                    *invalidation_revision,
                    false,
                ) {
                    state.dirty = true;
                }
            }
            CanvasUpdate::Rebind {
                draw,
                invalidation_revision,
            } => {
                state.draw = Some(draw.clone());
                accept_invalidation_revision(
                    &mut state.invalidation_revision,
                    *invalidation_revision,
                    true,
                );
                state.dirty = true;
            }
            CanvasUpdate::Invalidate(revision) => {
                if accept_invalidation_revision(&mut state.invalidation_revision, *revision, false)
                {
                    state.dirty = true;
                }
            }
        }
        if state.dirty && state.native.is_some() {
            queue_canvas_image_frame(id, state, &events, &waker);
        }
        Ok(())
    }

    fn subscribe_canvas_image_scale(&mut self, id: NodeId) -> WindowsResult<Option<f32>> {
        let (loaded, ui) = {
            let Handle::CanvasImage(state) = &self.node(id)?.handle else {
                panic!("canvas image scale target is not a CanvasImage");
            };
            (state.loaded.get(), state.ui.clone())
        };
        if !loaded {
            return Ok(None);
        }

        let xaml_root = ui.XamlRoot()?;
        let subscribed_scale = {
            let Handle::CanvasImage(state) = &self.node(id)?.handle else {
                unreachable!()
            };
            (state.scale_revoker.is_some() && state.xaml_root.as_ref() == Some(&xaml_root))
                .then(|| state.metrics.get().scale_x)
        };
        if let Some(scale) = subscribed_scale {
            return Ok(Some(scale));
        }

        let scale = xaml_root.RasterizationScale()? as f32;
        let metrics = {
            let Handle::CanvasImage(state) = &self.node(id)?.handle else {
                unreachable!()
            };
            Rc::clone(&state.metrics)
        };
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let changed = xaml_root.Changed(move |sender, _args| {
            let sender = sender.as_ref().unwrap();
            let scale = sender.RasterizationScale().unwrap() as f32;
            let mut current = metrics.get();
            current.scale_x = scale;
            current.scale_y = scale;
            metrics.set(current);
            queue_canvas_image_layout(id, current, &events, &waker);
        })?;

        let Handle::CanvasImage(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        let mut current = state.metrics.get();
        current.scale_x = scale;
        current.scale_y = scale;
        state.metrics.set(current);
        state.xaml_root = Some(xaml_root);
        state.scale_revoker = Some(changed);
        Ok(Some(scale))
    }

    fn rebuild_canvas_image(&mut self, id: NodeId, rebuild_device: bool) -> WindowsResult<()> {
        let metrics = match &self.node(id)?.handle {
            Handle::CanvasImage(state) => state.applied_metrics,
            _ => {
                panic!("canvas image rebuild target is not a CanvasImage");
            }
        };
        let (width, height) = canvas_pixel_size(metrics)?;
        if width == 0 || height == 0 {
            return Ok(());
        }
        let width = i32::try_from(width)
            .unwrap_or_else(|_| panic!("canvas image pixel width is too large"));
        let height = i32::try_from(height)
            .unwrap_or_else(|_| panic!("canvas image pixel height is too large"));

        let device = if rebuild_device {
            Some(windows_canvas::GpuDevice::new_or_warp()?)
        } else {
            None
        };
        let device_ref = if let Some(device) = device.as_ref() {
            device
        } else {
            let Handle::CanvasImage(state) = &self.node(id)?.handle else {
                unreachable!()
            };
            state
                .device
                .as_ref()
                .unwrap_or_else(|| panic!("canvas image rebuild has no existing GPU device"))
        };
        let source = bindings::SurfaceImageSource::CreateInstanceWithDimensions(width, height)?;
        let native: bindings::ISurfaceImageSourceNativeWithD2D = source.cast()?;
        unsafe {
            native
                .SetDevice(Interface::as_raw(device_ref.d2d_device()))
                .ok()?;
        }
        let image_source: bindings::ImageSource = source.cast()?;

        let Handle::CanvasImage(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        state.value.SetSource(&image_source)?;
        if let Some(device) = device {
            state.device = Some(device);
        }
        state.source = Some(source);
        state.native = Some(native);
        state.dirty = true;
        state.device_changed |= rebuild_device;
        state.surface_changed = true;
        Ok(())
    }

    pub(super) fn apply_canvas_image_layout(
        &mut self,
        id: NodeId,
        width: f32,
        height: f32,
        event_scale: f32,
    ) -> WindowsResult<()> {
        let scale = self
            .subscribe_canvas_image_scale(id)?
            .unwrap_or(event_scale);
        let metrics = NativeCanvasLayout {
            width,
            height,
            scale_x: scale,
            scale_y: scale,
        };
        canvas_pixel_size(metrics)?;
        let old_metrics = {
            let Handle::CanvasImage(state) = &mut self.node_mut(id)?.handle else {
                panic!("canvas image metrics target is not a CanvasImage");
            };
            let old = state.applied_metrics;
            state.metrics.set(metrics);
            state.applied_metrics = metrics;
            old
        };
        if old_metrics == metrics {
            return Ok(());
        }

        let (pixel_width, pixel_height) = canvas_pixel_size(metrics)?;
        if pixel_width == 0 || pixel_height == 0 {
            let Handle::CanvasImage(state) = &mut self.node_mut(id)?.handle else {
                unreachable!()
            };
            if state.source.is_some() {
                state.value.SetSource(None::<&bindings::ImageSource>)?;
            }
            state.source = None;
            state.native = None;
            state.frame_queued = false;
            state.dirty = true;
            state.surface_changed = true;
            return Ok(());
        }

        let rebuild_device = {
            let Handle::CanvasImage(state) = &self.node(id)?.handle else {
                unreachable!()
            };
            state.device.is_none()
        };
        self.rebuild_canvas_image(id, rebuild_device)?;
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let Handle::CanvasImage(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        queue_canvas_image_frame(id, state, &events, &waker);
        Ok(())
    }

    fn recover_canvas_image(&mut self, id: NodeId) -> WindowsResult<()> {
        {
            let Handle::CanvasImage(state) = &mut self.node_mut(id)?.handle else {
                panic!("canvas image recovery target is not a CanvasImage");
            };
            state.frame_queued = false;
            state.source = None;
            state.native = None;
            state.device = None;
            state.dirty = true;
        }
        self.rebuild_canvas_image(id, true)?;
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let Handle::CanvasImage(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        queue_canvas_image_frame(id, state, &events, &waker);
        Ok(())
    }

    pub(super) fn run_canvas_image_frame(&mut self, id: NodeId) -> WindowsResult<()> {
        enum FrameDraw {
            DeviceLost,
            Complete(WindowsResult<()>),
        }

        let outcome = {
            let Handle::CanvasImage(state) = &mut self.node_mut(id)?.handle else {
                panic!("canvas image frame target is not a CanvasImage");
            };
            state.frame_queued = false;
            if !state.dirty {
                return Ok(());
            }
            let metrics = state.applied_metrics;
            let (width, height) = canvas_pixel_size(metrics)?;
            if width == 0 || height == 0 {
                return Ok(());
            }
            let Some(draw) = state.draw.clone() else {
                return Ok(());
            };
            let device = state
                .device
                .as_ref()
                .unwrap_or_else(|| panic!("canvas image frame has no GPU device"));
            let native = state
                .native
                .as_ref()
                .unwrap_or_else(|| panic!("canvas image frame has no SurfaceImageSource"));
            let rect = bindings::RECT {
                left: 0,
                top: 0,
                right: i32::try_from(width)
                    .unwrap_or_else(|_| panic!("canvas image width is too large")),
                bottom: i32::try_from(height)
                    .unwrap_or_else(|_| panic!("canvas image height is too large")),
            };
            let mut offset = bindings::POINT::default();
            let mut object = core::ptr::null_mut();
            let begin = unsafe {
                native.BeginDraw(
                    &rect,
                    &windows_canvas::ID2D1DeviceContext::IID,
                    &mut object,
                    &mut offset,
                )
            };
            if windows_canvas::is_device_lost(begin) {
                FrameDraw::DeviceLost
            } else {
                begin.ok()?;
                let context = unsafe { windows_canvas::ID2D1DeviceContext::from_raw(object) };
                let guard = EndSurfaceDrawGuard(native);
                let scale = metrics.scale_x;
                let session = windows_canvas::DrawingSession::from_borrowed_context_with_dpi(
                    &context,
                    windows_canvas::Matrix3x2::translation(
                        offset.x as f32 / scale,
                        offset.y as f32 / scale,
                    ),
                    96.0 * scale,
                );
                let context = CanvasDrawContext::new(
                    session,
                    device,
                    (
                        metrics.width,
                        metrics.height,
                        metrics.scale_x,
                        metrics.scale_y,
                    ),
                    state.device_changed,
                    state.surface_changed,
                );
                let draw_result = draw.call(&context);
                let session_result = context.finish();
                core::mem::forget(guard);
                let end_result = unsafe { native.EndDraw() }.ok();
                if session_result
                    .as_ref()
                    .is_err_and(|error| windows_canvas::is_device_lost(error.code()))
                    || end_result
                        .as_ref()
                        .is_err_and(|error| windows_canvas::is_device_lost(error.code()))
                    || draw_result
                        .as_ref()
                        .is_err_and(|error| windows_canvas::is_device_lost(error.code()))
                {
                    FrameDraw::DeviceLost
                } else if let Err(error) = session_result {
                    FrameDraw::Complete(Err(error))
                } else if let Err(error) = end_result {
                    FrameDraw::Complete(Err(error))
                } else {
                    FrameDraw::Complete(draw_result)
                }
            }
        };

        let FrameDraw::Complete(result) = outcome else {
            return self.recover_canvas_image(id);
        };
        result?;
        let Handle::CanvasImage(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        state.dirty = false;
        state.device_changed = false;
        state.surface_changed = false;
        Ok(())
    }

    pub(super) fn create_swap_chain_canvas(&mut self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::SwapChainPanel::new()?;
        let ui = value.cast()?;
        let framework: bindings::FrameworkElement = value.cast()?;
        let native = value.cast()?;
        let metrics = Rc::new(Cell::new(NativeCanvasLayout {
            width: 0.0,
            height: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
        }));
        let loaded = Rc::new(Cell::new(false));

        let size_metrics = Rc::clone(&metrics);
        let size_events = Rc::clone(&self.events);
        let size_waker = Rc::clone(&self.waker);
        let size_changed = framework.SizeChanged(move |_sender, args| {
            let args = args.as_ref().unwrap();
            let size = args.NewSize().unwrap();
            let mut current = size_metrics.get();
            current.width = size.width;
            current.height = size.height;
            size_metrics.set(current);
            queue_canvas_layout(id, current, &size_events, &size_waker);
        })?;

        let loaded_metrics = Rc::clone(&metrics);
        let loaded_state = Rc::clone(&loaded);
        let loaded_events = Rc::clone(&self.events);
        let loaded_waker = Rc::clone(&self.waker);
        let loaded_revoker = framework.Loaded(move |_sender, _args| {
            loaded_state.set(true);
            queue_canvas_layout(id, loaded_metrics.get(), &loaded_events, &loaded_waker);
        })?;

        Ok(Handle::SwapChainCanvas(Box::new(SwapChainCanvasState {
            _value: value,
            ui,
            framework,
            native,
            metrics,
            applied_metrics: NativeCanvasLayout {
                width: 0.0,
                height: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
            },
            loaded,
            xaml_root: None,
            scale_revoker: None,
            draw: None,
            device: None,
            swap_chain: None,
            invalidation_revision: 0,
            continuous: false,
            rendering: None,
            dirty: true,
            frame_queued: Rc::new(Cell::new(false)),
            device_changed: true,
            surface_changed: true,
            _revokers: [size_changed, loaded_revoker],
        })))
    }

    fn subscribe_swap_chain_canvas_scale(&mut self, id: NodeId) -> WindowsResult<Option<f32>> {
        let (loaded, ui) = {
            let Handle::SwapChainCanvas(state) = &self.node(id)?.handle else {
                panic!("canvas scale target is not a SwapChainCanvas");
            };
            (state.loaded.get(), state.ui.clone())
        };
        if !loaded {
            return Ok(None);
        }

        let xaml_root = ui.XamlRoot()?;
        let subscribed_scale = {
            let Handle::SwapChainCanvas(state) = &self.node(id)?.handle else {
                unreachable!()
            };
            (state.scale_revoker.is_some() && state.xaml_root.as_ref() == Some(&xaml_root))
                .then(|| state.metrics.get().scale_x)
        };
        if let Some(scale) = subscribed_scale {
            return Ok(Some(scale));
        }

        let scale = xaml_root.RasterizationScale()? as f32;
        let metrics = {
            let Handle::SwapChainCanvas(state) = &self.node(id)?.handle else {
                unreachable!()
            };
            Rc::clone(&state.metrics)
        };
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let changed = xaml_root.Changed(move |sender, _args| {
            let sender = sender.as_ref().unwrap();
            let scale = sender.RasterizationScale().unwrap() as f32;
            let mut current = metrics.get();
            current.scale_x = scale;
            current.scale_y = scale;
            metrics.set(current);
            queue_canvas_layout(id, current, &events, &waker);
        })?;

        let Handle::SwapChainCanvas(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        let mut current = state.metrics.get();
        current.scale_x = scale;
        current.scale_y = scale;
        state.metrics.set(current);
        state.xaml_root = Some(xaml_root);
        state.scale_revoker = Some(changed);
        Ok(Some(scale))
    }

    pub(super) fn apply_swap_chain_canvas_update(
        &mut self,
        id: NodeId,
        update: &CanvasUpdate,
    ) -> WindowsResult<()> {
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let Handle::SwapChainCanvas(state) = &mut self.node_mut(id)?.handle else {
            panic!("SwapChainCanvas update target is not a SwapChainCanvas");
        };
        match update {
            CanvasUpdate::Props {
                draw,
                invalidation_revision,
            } => {
                if state.draw.as_ref() != Some(draw) {
                    state.draw = Some(draw.clone());
                }
                if accept_invalidation_revision(
                    &mut state.invalidation_revision,
                    *invalidation_revision,
                    false,
                ) {
                    state.dirty = true;
                }
            }
            CanvasUpdate::Rebind {
                draw,
                invalidation_revision,
            } => {
                state.draw = Some(draw.clone());
                accept_invalidation_revision(
                    &mut state.invalidation_revision,
                    *invalidation_revision,
                    true,
                );
                state.dirty = true;
            }
            CanvasUpdate::Invalidate(revision) => {
                if accept_invalidation_revision(&mut state.invalidation_revision, *revision, false)
                {
                    state.dirty = true;
                }
            }
        }
        if state.dirty {
            queue_canvas_frame(id, state, &events, &waker);
        }
        Ok(())
    }

    pub(super) fn apply_swap_chain_canvas_control_update(
        &mut self,
        id: NodeId,
        update: &SwapChainCanvasUpdate,
    ) -> WindowsResult<()> {
        match update {
            SwapChainCanvasUpdate::Canvas(value) => self.apply_swap_chain_canvas_update(id, value),
            SwapChainCanvasUpdate::Continuous(value) => self.apply_canvas_continuous(id, *value),
        }
    }

    pub(super) fn apply_canvas_continuous(
        &mut self,
        id: NodeId,
        continuous: bool,
    ) -> WindowsResult<()> {
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let Handle::SwapChainCanvas(state) = &mut self.node_mut(id)?.handle else {
            panic!("render-mode target is not a SwapChainCanvas");
        };
        if state.continuous == continuous {
            return Ok(());
        }
        state.continuous = continuous;
        state.rendering = None;
        if continuous {
            let metrics = Rc::clone(&state.metrics);
            let frame_queued = Rc::clone(&state.frame_queued);
            state.rendering = Some(bindings::CompositionTarget::Rendering(
                move |_sender, _args| {
                    let metrics = metrics.get();
                    if metrics.width <= 0.0 || metrics.height <= 0.0 {
                        return;
                    }
                    if frame_queued.replace(true) {
                        return;
                    }
                    events
                        .borrow_mut()
                        .push_back(NativeEvent::CanvasFrame { target: id });
                    if let Some(wake) = waker.borrow().as_ref() {
                        wake();
                    }
                },
            )?);
        }
        Ok(())
    }

    fn rebuild_swap_chain_canvas(&mut self, id: NodeId) -> WindowsResult<()> {
        let metrics = match &self.node(id)?.handle {
            Handle::SwapChainCanvas(state) => state.applied_metrics,
            _ => {
                panic!("canvas rebuild target is not a SwapChainCanvas");
            }
        };
        let (width, height) = canvas_pixel_size(metrics)?;
        if width == 0 || height == 0 {
            return Ok(());
        }

        let device = windows_canvas::GpuDevice::new_or_warp()?;
        let mut swap_chain = device.create_swap_chain(width, height)?;
        swap_chain.set_dpi(96.0 * metrics.scale_x, 96.0 * metrics.scale_y)?;
        swap_chain.set_composition_scale(metrics.scale_x, metrics.scale_y)?;

        let Handle::SwapChainCanvas(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        unsafe {
            state
                .native
                .SetSwapChain(Interface::as_raw(swap_chain.raw_swap_chain()))
                .ok()?;
        }
        state.device = Some(device);
        state.swap_chain = Some(swap_chain);
        state.dirty = true;
        state.device_changed = true;
        state.surface_changed = true;
        Ok(())
    }

    pub(super) fn apply_canvas_layout(
        &mut self,
        id: NodeId,
        mut metrics: NativeCanvasLayout,
    ) -> WindowsResult<()> {
        if let Some(scale) = self.subscribe_swap_chain_canvas_scale(id)? {
            metrics.scale_x = scale;
            metrics.scale_y = scale;
        }
        #[cfg(test)]
        let metrics = self.apply_forced_canvas_scale(metrics);
        canvas_pixel_size(metrics)?;
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let (old_metrics, has_surface) = {
            let Handle::SwapChainCanvas(state) = &mut self.node_mut(id)?.handle else {
                panic!("canvas metrics target is not a SwapChainCanvas");
            };
            let old = state.applied_metrics;
            state.metrics.set(metrics);
            state.applied_metrics = metrics;
            (old, state.swap_chain.is_some())
        };
        if old_metrics == metrics {
            return Ok(());
        }

        let (width, height) = canvas_pixel_size(metrics)?;
        if width == 0 || height == 0 {
            let Handle::SwapChainCanvas(state) = &mut self.node_mut(id)?.handle else {
                unreachable!()
            };
            state.dirty = true;
            state.surface_changed = true;
            return Ok(());
        }

        if !has_surface {
            self.rebuild_swap_chain_canvas(id)?;
        } else {
            let (old_width, old_height) = canvas_pixel_size(old_metrics)?;
            let scale_changed = old_metrics.scale_x.to_bits() != metrics.scale_x.to_bits()
                || old_metrics.scale_y.to_bits() != metrics.scale_y.to_bits();
            let resize_result = {
                let Handle::SwapChainCanvas(state) = &mut self.node_mut(id)?.handle else {
                    unreachable!()
                };
                let swap_chain = state.swap_chain.as_mut().unwrap();
                let result = if (old_width, old_height) != (width, height) {
                    swap_chain.resize_with_dpi(
                        width,
                        height,
                        96.0 * metrics.scale_x,
                        96.0 * metrics.scale_y,
                    )
                } else if scale_changed {
                    swap_chain.set_dpi(96.0 * metrics.scale_x, 96.0 * metrics.scale_y)
                } else {
                    Ok(())
                };
                result.and_then(|()| {
                    if scale_changed {
                        swap_chain.set_composition_scale(metrics.scale_x, metrics.scale_y)
                    } else {
                        Ok(())
                    }
                })
            };
            if let Err(error) = resize_result {
                if windows_canvas::is_device_lost(error.code()) {
                    self.rebuild_swap_chain_canvas(id)?;
                } else {
                    return Err(error);
                }
            }
        }

        let Handle::SwapChainCanvas(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        state.dirty = true;
        state.surface_changed = true;
        queue_canvas_frame(id, state, &events, &waker);
        Ok(())
    }

    fn recover_swap_chain_canvas(&mut self, id: NodeId) -> WindowsResult<()> {
        {
            let Handle::SwapChainCanvas(state) = &mut self.node_mut(id)?.handle else {
                panic!("canvas recovery target is not a SwapChainCanvas");
            };
            state.frame_queued.set(false);
            state.swap_chain = None;
            state.device = None;
            state.dirty = true;
        }
        self.rebuild_swap_chain_canvas(id)?;
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let Handle::SwapChainCanvas(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        queue_canvas_frame(id, state, &events, &waker);
        Ok(())
    }

    pub(super) fn run_canvas_frame(&mut self, id: NodeId) -> WindowsResult<()> {
        enum FrameDraw {
            DeviceLost,
            Complete(WindowsResult<()>),
        }

        let draw_result = {
            let Handle::SwapChainCanvas(state) = &mut self.node_mut(id)?.handle else {
                panic!("canvas frame target is not a SwapChainCanvas");
            };
            state.frame_queued.set(false);
            if state.continuous {
                state.dirty = true;
            }
            if !state.dirty {
                return Ok(());
            }
            let metrics = state.applied_metrics;
            let (width, height) = canvas_pixel_size(metrics)?;
            if width == 0 || height == 0 {
                return Ok(());
            }
            let Some(draw) = state.draw.clone() else {
                return Ok(());
            };
            let device = state
                .device
                .as_ref()
                .unwrap_or_else(|| panic!("canvas frame has no GPU device"));
            let swap_chain = state
                .swap_chain
                .as_mut()
                .unwrap_or_else(|| panic!("canvas frame has no swap chain"));
            let outcome = match swap_chain.begin_draw() {
                Err(error) if windows_canvas::is_device_lost(error.code()) => FrameDraw::DeviceLost,
                Err(error) => return Err(error),
                Ok(session) => {
                    let context = CanvasDrawContext::new(
                        session,
                        device,
                        (
                            metrics.width,
                            metrics.height,
                            metrics.scale_x,
                            metrics.scale_y,
                        ),
                        state.device_changed,
                        state.surface_changed,
                    );
                    let draw_result = draw.call(&context);
                    let end_result = context.finish();
                    if end_result
                        .as_ref()
                        .is_err_and(|error| windows_canvas::is_device_lost(error.code()))
                        || draw_result
                            .as_ref()
                            .is_err_and(|error| windows_canvas::is_device_lost(error.code()))
                    {
                        FrameDraw::DeviceLost
                    } else if let Err(error) = end_result {
                        FrameDraw::Complete(Err(error))
                    } else {
                        FrameDraw::Complete(draw_result)
                    }
                }
            };
            if swap_chain.is_device_lost() {
                FrameDraw::DeviceLost
            } else {
                outcome
            }
        };

        let FrameDraw::Complete(draw_result) = draw_result else {
            return self.recover_swap_chain_canvas(id);
        };
        draw_result?;

        #[cfg(test)]
        if self.take_forced_canvas_present_loss() {
            return self.recover_swap_chain_canvas(id);
        }
        let Handle::SwapChainCanvas(state) = &self.node(id)?.handle else {
            unreachable!()
        };
        let present = state.swap_chain.as_ref().unwrap().present()?;
        if !present {
            return self.recover_swap_chain_canvas(id);
        }

        let Handle::SwapChainCanvas(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        state.dirty = false;
        state.device_changed = false;
        state.surface_changed = false;
        Ok(())
    }
}

#[cfg(test)]
#[path = "../../testing/private/winui/canvas_support.rs"]
mod testing;

fn canvas_pixel_size(metrics: NativeCanvasLayout) -> WindowsResult<(u32, u32)> {
    if !metrics.width.is_finite()
        || !metrics.height.is_finite()
        || !metrics.scale_x.is_finite()
        || !metrics.scale_y.is_finite()
        || metrics.width < 0.0
        || metrics.height < 0.0
        || metrics.scale_x <= 0.0
        || metrics.scale_y <= 0.0
    {
        panic!("canvas metrics are invalid");
    }
    let width = (metrics.width * metrics.scale_x).ceil();
    let height = (metrics.height * metrics.scale_y).ceil();
    assert!(
        !(width > u32::MAX as f32 || height > u32::MAX as f32),
        "canvas pixel size is too large"
    );
    Ok((width as u32, height as u32))
}

fn host_pixel_size(layout: SwapChainHostLayout) -> WindowsResult<(u32, u32)> {
    canvas_pixel_size(NativeCanvasLayout {
        width: layout.width,
        height: layout.height,
        scale_x: layout.scale_x,
        scale_y: layout.scale_y,
    })
}

fn update_host_pixels(layout: &mut SwapChainHostLayout) {
    let (width, height) = host_pixel_size(*layout).unwrap();
    layout.pixel_width = width;
    layout.pixel_height = height;
}

impl WinUiRuntime {
    pub(super) fn create_swap_chain_host(&mut self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::SwapChainPanel::new()?;
        let ui = value.cast()?;
        let framework: bindings::FrameworkElement = value.cast()?;
        let native = value.cast()?;
        let metrics = Rc::new(Cell::new(SwapChainHostLayout {
            scale_x: 1.0,
            scale_y: 1.0,
            ..Default::default()
        }));
        let loaded = Rc::new(Cell::new(false));

        let size_metrics = Rc::clone(&metrics);
        let size_events = Rc::clone(&self.events);
        let size_waker = Rc::clone(&self.waker);
        let size_changed = framework.SizeChanged(move |_sender, args| {
            let size = args.as_ref().unwrap().NewSize().unwrap();
            let mut current = size_metrics.get();
            current.width = size.width;
            current.height = size.height;
            current.revision = current.revision.wrapping_add(1);
            update_host_pixels(&mut current);
            size_metrics.set(current);
            queue_swap_chain_host_layout(id, current, &size_events, &size_waker);
        })?;

        let loaded_metrics = Rc::clone(&metrics);
        let loaded_state = Rc::clone(&loaded);
        let loaded_events = Rc::clone(&self.events);
        let loaded_waker = Rc::clone(&self.waker);
        let loaded_revoker = framework.Loaded(move |_sender, _args| {
            loaded_state.set(true);
            let mut current = loaded_metrics.get();
            current.revision = current.revision.wrapping_add(1);
            update_host_pixels(&mut current);
            loaded_metrics.set(current);
            queue_swap_chain_host_layout(id, current, &loaded_events, &loaded_waker);
        })?;

        Ok(Handle::SwapChainHost(Box::new(SwapChainHostState {
            _value: value,
            ui,
            framework,
            native,
            metrics,
            applied_layout: SwapChainHostLayout::default(),
            loaded,
            xaml_root: None,
            scale_revoker: None,
            factory: None,
            layout: None,
            frame: None,
            content: None,
            swap_chain: None,
            pending_actions: Vec::new(),
            continuous: false,
            rendering: None,
            frame_queued: Rc::new(Cell::new(false)),
            frame_revision: 0,
            dirty: true,
            device_changed: true,
            surface_changed: true,
            _revokers: [size_changed, loaded_revoker],
        })))
    }

    fn subscribe_swap_chain_host_scale(&mut self, id: NodeId) -> WindowsResult<()> {
        let (loaded, ui) = {
            let Handle::SwapChainHost(state) = &self.node(id)?.handle else {
                panic!("scale target is not a SwapChainHost");
            };
            (state.loaded.get(), state.ui.clone())
        };
        if !loaded {
            return Ok(());
        }

        let xaml_root = ui.XamlRoot()?;
        let subscribed_to_root = {
            let Handle::SwapChainHost(state) = &self.node(id)?.handle else {
                unreachable!()
            };
            state.scale_revoker.is_some() && state.xaml_root.as_ref() == Some(&xaml_root)
        };
        if subscribed_to_root {
            return Ok(());
        }

        let scale = xaml_root.RasterizationScale()? as f32;
        let metrics = {
            let Handle::SwapChainHost(state) = &self.node(id)?.handle else {
                unreachable!()
            };
            Rc::clone(&state.metrics)
        };
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let changed = xaml_root.Changed(move |sender, _args| {
            let scale = sender.as_ref().unwrap().RasterizationScale().unwrap() as f32;
            let mut current = metrics.get();
            current.scale_x = scale;
            current.scale_y = scale;
            current.revision = current.revision.wrapping_add(1);
            update_host_pixels(&mut current);
            metrics.set(current);
            queue_swap_chain_host_layout(id, current, &events, &waker);
        })?;

        let Handle::SwapChainHost(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        let mut current = state.metrics.get();
        current.scale_x = scale;
        current.scale_y = scale;
        update_host_pixels(&mut current);
        state.metrics.set(current);
        state.xaml_root = Some(xaml_root);
        state.scale_revoker = Some(changed);
        Ok(())
    }

    fn set_swap_chain_host_continuous(
        &mut self,
        id: NodeId,
        continuous: bool,
    ) -> WindowsResult<()> {
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let Handle::SwapChainHost(state) = &mut self.node_mut(id)?.handle else {
            panic!("render-mode target is not a SwapChainHost");
        };
        if state.continuous == continuous {
            return Ok(());
        }
        state.continuous = continuous;
        state.rendering = None;
        if continuous {
            let metrics = Rc::clone(&state.metrics);
            let frame_queued = Rc::clone(&state.frame_queued);
            state.rendering = Some(bindings::CompositionTarget::Rendering(
                move |_sender, _args| {
                    let layout = metrics.get();
                    if layout.pixel_width == 0 || layout.pixel_height == 0 {
                        return;
                    }
                    if frame_queued.replace(true) {
                        return;
                    }
                    events
                        .borrow_mut()
                        .push_back(NativeEvent::SwapChainHostFrame { target: id });
                    if let Some(wake) = waker.borrow().as_ref() {
                        wake();
                    }
                },
            )?);
        }
        Ok(())
    }

    fn rebuild_swap_chain_host(&mut self, id: NodeId) -> WindowsResult<()> {
        let (factory, layout) = {
            let Handle::SwapChainHost(state) = &self.node(id)?.handle else {
                panic!("rebuild target is not a SwapChainHost");
            };
            (
                state
                    .factory
                    .clone()
                    .expect("SwapChainHost rebuild ran before initialization"),
                state.applied_layout,
            )
        };
        if layout.pixel_width == 0 || layout.pixel_height == 0 {
            return Ok(());
        }
        let (mut content, swap_chain) = factory.create(layout)?;
        let Handle::SwapChainHost(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        for action in state.pending_actions.drain(..) {
            action.call(content.as_mut())?;
        }
        unsafe {
            state
                .native
                .SetSwapChain(Interface::as_raw(swap_chain.raw_swap_chain()))
                .ok()?;
        }
        state.content = Some(content);
        state.swap_chain = Some(swap_chain);
        state.dirty = true;
        state.device_changed = true;
        state.surface_changed = true;
        Ok(())
    }

    pub(super) fn apply_swap_chain_host_update(
        &mut self,
        id: NodeId,
        update: &SwapChainHostUpdate,
    ) -> WindowsResult<()> {
        match update {
            SwapChainHostUpdate::Initialize {
                factory,
                layout,
                frame,
                continuous,
            } => {
                let Handle::SwapChainHost(state) = &mut self.node_mut(id)?.handle else {
                    panic!("SwapChainHost update target is not a SwapChainHost");
                };
                state.factory = Some(factory.clone());
                state.layout = Some(layout.clone());
                state.frame = Some(frame.clone());
                state.dirty = true;
                self.set_swap_chain_host_continuous(id, *continuous)
            }
            SwapChainHostUpdate::Props {
                layout,
                frame,
                continuous,
            } => {
                let events = Rc::clone(&self.events);
                let waker = Rc::clone(&self.waker);
                {
                    let Handle::SwapChainHost(state) = &mut self.node_mut(id)?.handle else {
                        panic!("SwapChainHost update target is not a SwapChainHost");
                    };
                    state.layout = Some(layout.clone());
                    state.frame = Some(frame.clone());
                    state.dirty = true;
                }
                self.set_swap_chain_host_continuous(id, *continuous)?;
                let Handle::SwapChainHost(state) = &self.node(id)?.handle else {
                    unreachable!()
                };
                queue_swap_chain_host_frame(id, state, &events, &waker);
                Ok(())
            }
            SwapChainHostUpdate::Action(action) => {
                let events = Rc::clone(&self.events);
                let waker = Rc::clone(&self.waker);
                let Handle::SwapChainHost(state) = &mut self.node_mut(id)?.handle else {
                    panic!("SwapChainHost update target is not a SwapChainHost");
                };
                if let Some(content) = state.content.as_mut() {
                    action.call(content.as_mut())?;
                } else {
                    state.pending_actions.push(action.clone());
                }
                state.dirty = true;
                queue_swap_chain_host_frame(id, state, &events, &waker);
                Ok(())
            }
        }
    }

    pub(super) fn apply_swap_chain_host_layout(
        &mut self,
        id: NodeId,
        mut layout: SwapChainHostLayout,
    ) -> WindowsResult<()> {
        self.subscribe_swap_chain_host_scale(id)?;
        let current = {
            let Handle::SwapChainHost(state) = &self.node(id)?.handle else {
                panic!("layout target is not a SwapChainHost");
            };
            state.metrics.get()
        };
        if current.revision >= layout.revision {
            layout = current;
        }
        host_pixel_size(layout)?;
        let has_content = {
            let Handle::SwapChainHost(state) = &mut self.node_mut(id)?.handle else {
                unreachable!()
            };
            if layout.revision <= state.applied_layout.revision {
                return Ok(());
            }
            state.metrics.set(layout);
            state.applied_layout = layout;
            state.content.is_some()
        };
        if layout.pixel_width == 0 || layout.pixel_height == 0 {
            return Ok(());
        }

        if !has_content {
            self.rebuild_swap_chain_host(id)?;
        } else {
            let result = {
                let Handle::SwapChainHost(state) = &mut self.node_mut(id)?.handle else {
                    unreachable!()
                };
                let callback = state.layout.clone().unwrap();
                callback.call(
                    state.content.as_mut().unwrap().as_mut(),
                    state.swap_chain.as_mut().unwrap(),
                    layout,
                )
            };
            if let Err(error) = result {
                if windows_canvas::is_device_lost(error.code()) {
                    self.recover_swap_chain_host(id)?;
                    return Ok(());
                }
                return Err(error);
            }
            let Handle::SwapChainHost(state) = &mut self.node_mut(id)?.handle else {
                unreachable!()
            };
            if state.swap_chain.as_ref().unwrap().is_device_lost() {
                self.recover_swap_chain_host(id)?;
                return Ok(());
            }
            state.dirty = true;
            state.surface_changed = true;
        }

        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let Handle::SwapChainHost(state) = &self.node(id)?.handle else {
            unreachable!()
        };
        queue_swap_chain_host_frame(id, state, &events, &waker);
        Ok(())
    }

    fn recover_swap_chain_host(&mut self, id: NodeId) -> WindowsResult<()> {
        {
            let Handle::SwapChainHost(state) = &mut self.node_mut(id)?.handle else {
                panic!("recovery target is not a SwapChainHost");
            };
            unsafe { state.native.SetSwapChain(core::ptr::null_mut()).ok()? };
            state.frame_queued.set(false);
            state.content = None;
            state.swap_chain = None;
            state.dirty = true;
        }
        self.rebuild_swap_chain_host(id)?;
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let Handle::SwapChainHost(state) = &self.node(id)?.handle else {
            unreachable!()
        };
        queue_swap_chain_host_frame(id, state, &events, &waker);
        Ok(())
    }

    pub(super) fn run_swap_chain_host_frame(&mut self, id: NodeId) -> WindowsResult<()> {
        let result = {
            let Handle::SwapChainHost(state) = &mut self.node_mut(id)?.handle else {
                panic!("frame target is not a SwapChainHost");
            };
            state.frame_queued.set(false);
            if !state.continuous && !state.dirty {
                return Ok(());
            }
            if state.content.is_none()
                || state.applied_layout.pixel_width == 0
                || state.applied_layout.pixel_height == 0
            {
                return Ok(());
            }
            state.frame_revision = state.frame_revision.wrapping_add(1);
            let frame = SwapChainHostFrame {
                layout: state.applied_layout,
                revision: state.frame_revision,
                device_changed: state.device_changed,
                surface_changed: state.surface_changed,
            };
            state.frame.clone().unwrap().call(
                state.content.as_mut().unwrap().as_mut(),
                state.swap_chain.as_mut().unwrap(),
                frame,
            )
        };
        if let Err(error) = result {
            if windows_canvas::is_device_lost(error.code()) {
                return self.recover_swap_chain_host(id);
            }
            return Err(error);
        }
        let Handle::SwapChainHost(state) = &mut self.node_mut(id)?.handle else {
            unreachable!()
        };
        if state.swap_chain.as_ref().unwrap().is_device_lost() {
            return self.recover_swap_chain_host(id);
        }
        state.dirty = false;
        state.device_changed = false;
        state.surface_changed = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::accept_invalidation_revision;

    #[test]
    fn changed_invalidation_source_resets_revision() {
        let mut revision = 10;
        assert!(accept_invalidation_revision(&mut revision, 0, true));
        assert_eq!(revision, 0);
        assert!(accept_invalidation_revision(&mut revision, 1, false));
        assert_eq!(revision, 1);
    }
}
