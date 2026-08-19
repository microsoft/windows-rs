use std::any::{Any, TypeId};
use std::cell::{Cell, RefCell};
use std::collections::BTreeSet;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use windows_canvas::{DrawingSession, GpuDevice, SwapChain};

use crate::element::tree::ElementKind;
use crate::element::{Element, Framework, RenderCx};
use crate::framework_properties::FrameworkProps;
use crate::hooks::SchedulerRef;
use crate::id::NodeId;
use crate::references::{ElementRef, NativeElementRef};

pub struct CanvasDrawContext<'a> {
    session: DrawingSession<'a>,
    device: &'a GpuDevice,
    pub width: f32,
    pub height: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    device_changed: bool,
    surface_changed: bool,
}

impl<'a> CanvasDrawContext<'a> {
    pub(crate) fn new(
        session: DrawingSession<'a>,
        device: &'a GpuDevice,
        metrics: (f32, f32, f32, f32),
        device_changed: bool,
        surface_changed: bool,
    ) -> Self {
        let (width, height, scale_x, scale_y) = metrics;
        Self {
            session,
            device,
            width,
            height,
            scale_x,
            scale_y,
            device_changed,
            surface_changed,
        }
    }

    pub fn device(&self) -> &GpuDevice {
        self.device
    }

    pub const fn device_changed(&self) -> bool {
        self.device_changed
    }

    pub const fn surface_changed(&self) -> bool {
        self.surface_changed
    }

    pub(crate) fn finish(self) -> windows_core::Result<()> {
        self.session.finish()
    }
}

impl<'a> Deref for CanvasDrawContext<'a> {
    type Target = DrawingSession<'a>;

    fn deref(&self) -> &Self::Target {
        &self.session
    }
}

#[derive(Clone)]
pub struct CanvasDrawCallback(
    Rc<dyn for<'a> Fn(&CanvasDrawContext<'a>) -> windows_core::Result<()>>,
);

impl CanvasDrawCallback {
    pub(crate) fn new(
        callback: impl for<'a> Fn(&CanvasDrawContext<'a>) -> windows_core::Result<()> + 'static,
    ) -> Self {
        Self(Rc::new(callback))
    }

    pub(crate) fn call(&self, context: &CanvasDrawContext<'_>) -> windows_core::Result<()> {
        (self.0)(context)
    }
}

impl fmt::Debug for CanvasDrawCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CanvasDrawCallback")
    }
}

impl PartialEq for CanvasDrawCallback {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for CanvasDrawCallback {}

#[derive(Clone)]
pub struct CanvasInvalidator {
    targets: Rc<RefCell<BTreeSet<NodeId>>>,
    revision: Rc<Cell<u64>>,
    scheduler: SchedulerRef,
}

impl CanvasInvalidator {
    fn new(scheduler: SchedulerRef) -> Self {
        Self {
            targets: Rc::new(RefCell::new(BTreeSet::new())),
            revision: Rc::new(Cell::new(0)),
            scheduler,
        }
    }

    pub fn invalidate(&self) {
        let revision = self.revision.get().wrapping_add(1);
        self.revision.set(revision);
        for target in self.targets.borrow().iter().copied() {
            self.scheduler.invalidate_canvas(target, revision);
        }
    }

    pub(crate) fn revision(&self) -> u64 {
        self.revision.get()
    }

    pub(crate) fn bind(&self, target: NodeId) {
        self.targets.borrow_mut().insert(target);
    }

    pub(crate) fn unbind(&self, target: NodeId) {
        self.targets.borrow_mut().remove(&target);
    }

    pub(crate) fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.targets, &other.targets)
    }
}

pub(crate) struct SwapChainCanvasProps {
    pub draw: CanvasDrawCallback,
    pub invalidator: Option<CanvasInvalidator>,
    pub continuous: bool,
    pub framework: FrameworkProps,
}

pub(crate) struct CanvasImageProps {
    pub draw: CanvasDrawCallback,
    pub invalidator: Option<CanvasInvalidator>,
    pub framework: FrameworkProps,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SwapChainHostLayout {
    pub width: f32,
    pub height: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub revision: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SwapChainHostFrame {
    pub layout: SwapChainHostLayout,
    pub revision: u64,
    pub device_changed: bool,
    pub surface_changed: bool,
}

pub struct SwapChainHostContent<T> {
    state: T,
    swap_chain: SwapChain,
}

impl<T> SwapChainHostContent<T> {
    pub fn new(state: T, swap_chain: SwapChain) -> Self {
        Self { state, swap_chain }
    }
}

#[derive(Clone)]
pub(crate) struct SwapChainHostFactory {
    state_type: TypeId,
    callback: Rc<dyn Fn(SwapChainHostLayout) -> windows_core::Result<(Box<dyn Any>, SwapChain)>>,
}

impl SwapChainHostFactory {
    pub(crate) fn new<T: 'static>(
        callback: impl Fn(SwapChainHostLayout) -> windows_core::Result<SwapChainHostContent<T>>
        + 'static,
    ) -> Self {
        Self {
            state_type: TypeId::of::<T>(),
            callback: Rc::new(move |layout| {
                let content = callback(layout)?;
                Ok((Box::new(content.state), content.swap_chain))
            }),
        }
    }

    pub(crate) fn create(
        &self,
        layout: SwapChainHostLayout,
    ) -> windows_core::Result<(Box<dyn Any>, SwapChain)> {
        (self.callback)(layout)
    }

    pub(crate) fn state_type(&self) -> TypeId {
        self.state_type
    }
}

impl fmt::Debug for SwapChainHostFactory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SwapChainHostFactory")
    }
}

impl PartialEq for SwapChainHostFactory {
    fn eq(&self, other: &Self) -> bool {
        self.state_type == other.state_type && Rc::ptr_eq(&self.callback, &other.callback)
    }
}

#[derive(Clone)]
pub(crate) struct SwapChainHostLayoutCallback {
    state_type: TypeId,
    callback:
        Rc<dyn Fn(&mut dyn Any, &mut SwapChain, SwapChainHostLayout) -> windows_core::Result<()>>,
}

impl SwapChainHostLayoutCallback {
    pub(crate) fn new<T: 'static>(
        callback: impl Fn(&mut T, &mut SwapChain, SwapChainHostLayout) -> windows_core::Result<()>
        + 'static,
    ) -> Self {
        Self {
            state_type: TypeId::of::<T>(),
            callback: Rc::new(move |state, swap_chain, layout| {
                callback(state.downcast_mut::<T>().unwrap(), swap_chain, layout)
            }),
        }
    }

    pub(crate) fn call(
        &self,
        state: &mut dyn Any,
        swap_chain: &mut SwapChain,
        layout: SwapChainHostLayout,
    ) -> windows_core::Result<()> {
        (self.callback)(state, swap_chain, layout)
    }

    pub(crate) fn state_type(&self) -> TypeId {
        self.state_type
    }
}

impl fmt::Debug for SwapChainHostLayoutCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SwapChainHostLayoutCallback")
    }
}

impl PartialEq for SwapChainHostLayoutCallback {
    fn eq(&self, other: &Self) -> bool {
        self.state_type == other.state_type && Rc::ptr_eq(&self.callback, &other.callback)
    }
}

#[derive(Clone)]
pub(crate) struct SwapChainHostFrameCallback {
    state_type: TypeId,
    callback:
        Rc<dyn Fn(&mut dyn Any, &mut SwapChain, SwapChainHostFrame) -> windows_core::Result<()>>,
}

impl SwapChainHostFrameCallback {
    pub(crate) fn new<T: 'static>(
        callback: impl Fn(&mut T, &mut SwapChain, SwapChainHostFrame) -> windows_core::Result<()>
        + 'static,
    ) -> Self {
        Self {
            state_type: TypeId::of::<T>(),
            callback: Rc::new(move |state, swap_chain, frame| {
                callback(state.downcast_mut::<T>().unwrap(), swap_chain, frame)
            }),
        }
    }

    pub(crate) fn call(
        &self,
        state: &mut dyn Any,
        swap_chain: &mut SwapChain,
        frame: SwapChainHostFrame,
    ) -> windows_core::Result<()> {
        (self.callback)(state, swap_chain, frame)
    }

    pub(crate) fn state_type(&self) -> TypeId {
        self.state_type
    }
}

impl fmt::Debug for SwapChainHostFrameCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SwapChainHostFrameCallback")
    }
}

impl PartialEq for SwapChainHostFrameCallback {
    fn eq(&self, other: &Self) -> bool {
        self.state_type == other.state_type && Rc::ptr_eq(&self.callback, &other.callback)
    }
}

#[derive(Clone)]
pub(crate) enum SwapChainHostAction {
    Update {
        state_type: TypeId,
        callback: Rc<dyn Fn(&mut dyn Any) -> windows_core::Result<()>>,
    },
    Invalidate,
}

impl SwapChainHostAction {
    pub(crate) fn update<T: 'static>(
        callback: impl Fn(&mut T) -> windows_core::Result<()> + 'static,
    ) -> Self {
        Self::Update {
            state_type: TypeId::of::<T>(),
            callback: Rc::new(move |state| callback(state.downcast_mut::<T>().unwrap())),
        }
    }

    pub(crate) fn state_type(&self) -> Option<TypeId> {
        match self {
            Self::Update { state_type, .. } => Some(*state_type),
            Self::Invalidate => None,
        }
    }

    pub(crate) fn call(&self, state: &mut dyn Any) -> windows_core::Result<()> {
        match self {
            Self::Update { callback, .. } => callback(state),
            Self::Invalidate => Ok(()),
        }
    }
}

impl fmt::Debug for SwapChainHostAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Update { .. } => f.write_str("SwapChainHostAction::Update"),
            Self::Invalidate => f.write_str("SwapChainHostAction::Invalidate"),
        }
    }
}

impl PartialEq for SwapChainHostAction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Update {
                    state_type: left_type,
                    callback: left,
                },
                Self::Update {
                    state_type: right_type,
                    callback: right,
                },
            ) => left_type == right_type && Rc::ptr_eq(left, right),
            (Self::Invalidate, Self::Invalidate) => true,
            _ => false,
        }
    }
}

pub(crate) struct SwapChainHostProps {
    pub factory: SwapChainHostFactory,
    pub layout: SwapChainHostLayoutCallback,
    pub frame: SwapChainHostFrameCallback,
    pub continuous: bool,
    pub framework: FrameworkProps,
}

pub struct SwapChainHost {
    props: SwapChainHostProps,
    reference: NativeElementRef,
}

pub struct SwapChainCanvas {
    props: SwapChainCanvasProps,
}

pub struct CanvasImage {
    props: CanvasImageProps,
}

impl SwapChainCanvas {
    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        Element::new(ElementKind::SwapChainCanvas(self.props))
    }
}

impl SwapChainHost {
    pub fn new<T: 'static>(
        reference: &SwapChainHostRef<T>,
        factory: impl Fn(SwapChainHostLayout) -> windows_core::Result<SwapChainHostContent<T>> + 'static,
        layout: impl Fn(&mut T, &mut SwapChain, SwapChainHostLayout) -> windows_core::Result<()>
        + 'static,
        frame: impl Fn(&mut T, &mut SwapChain, SwapChainHostFrame) -> windows_core::Result<()> + 'static,
    ) -> Framework<Self> {
        Framework::new(Self {
            props: SwapChainHostProps {
                factory: SwapChainHostFactory::new(factory),
                layout: SwapChainHostLayoutCallback::new(layout),
                frame: SwapChainHostFrameCallback::new(frame),
                continuous: false,
                framework: FrameworkProps::default(),
            },
            reference: reference.reference.binding(),
        })
    }

    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        let element = Element::new(ElementKind::SwapChainHost(Box::new(self.props)));
        Element::new(ElementKind::Reference {
            reference: self.reference,
            child: Box::new(element),
        })
    }
}

impl Framework<SwapChainHost> {
    pub fn continuous(mut self, value: bool) -> Self {
        self.control.props.continuous = value;
        self
    }
}

impl CanvasImage {
    pub(crate) fn build_with_framework(mut self, framework: FrameworkProps) -> Element {
        self.props.framework = framework;
        Element::new(ElementKind::CanvasImage(self.props))
    }
}

pub fn swap_chain_canvas(
    draw: impl for<'a> Fn(&CanvasDrawContext<'a>) -> windows_core::Result<()> + 'static,
) -> Framework<SwapChainCanvas> {
    Framework::new(SwapChainCanvas {
        props: SwapChainCanvasProps {
            draw: CanvasDrawCallback::new(draw),
            invalidator: None,
            continuous: false,
            framework: FrameworkProps::default(),
        },
    })
}

pub fn swap_chain_canvas_invalidated(
    invalidator: &CanvasInvalidator,
    draw: impl for<'a> Fn(&CanvasDrawContext<'a>) -> windows_core::Result<()> + 'static,
) -> Framework<SwapChainCanvas> {
    Framework::new(SwapChainCanvas {
        props: SwapChainCanvasProps {
            draw: CanvasDrawCallback::new(draw),
            invalidator: Some(invalidator.clone()),
            continuous: false,
            framework: FrameworkProps::default(),
        },
    })
}

pub fn animated_canvas(
    draw: impl for<'a> Fn(&CanvasDrawContext<'a>) -> windows_core::Result<()> + 'static,
) -> Framework<SwapChainCanvas> {
    Framework::new(SwapChainCanvas {
        props: SwapChainCanvasProps {
            draw: CanvasDrawCallback::new(draw),
            invalidator: None,
            continuous: true,
            framework: FrameworkProps::default(),
        },
    })
}

pub fn canvas_image(
    draw: impl for<'a> Fn(&CanvasDrawContext<'a>) -> windows_core::Result<()> + 'static,
) -> Framework<CanvasImage> {
    Framework::new(CanvasImage {
        props: CanvasImageProps {
            draw: CanvasDrawCallback::new(draw),
            invalidator: None,
            framework: FrameworkProps::default(),
        },
    })
}

pub fn canvas_image_invalidated(
    invalidator: &CanvasInvalidator,
    draw: impl for<'a> Fn(&CanvasDrawContext<'a>) -> windows_core::Result<()> + 'static,
) -> Framework<CanvasImage> {
    Framework::new(CanvasImage {
        props: CanvasImageProps {
            draw: CanvasDrawCallback::new(draw),
            invalidator: Some(invalidator.clone()),
            framework: FrameworkProps::default(),
        },
    })
}

impl RenderCx<'_> {
    pub fn use_canvas_invalidator(&mut self) -> CanvasInvalidator {
        let scheduler = Rc::clone(&self.scheduler);
        self.use_ref(move || CanvasInvalidator::new(scheduler))
            .get()
            .unwrap()
    }

    pub fn use_swap_chain_host_ref<T: 'static>(&mut self) -> SwapChainHostRef<T> {
        SwapChainHostRef {
            reference: self.use_element_ref(),
            marker: std::marker::PhantomData,
        }
    }
}

pub struct SwapChainHostRef<T> {
    reference: ElementRef<SwapChainHost>,
    marker: std::marker::PhantomData<fn(&mut T)>,
}

impl<T: 'static> SwapChainHostRef<T> {
    pub fn new() -> Self {
        Self {
            reference: ElementRef::new(),
            marker: std::marker::PhantomData,
        }
    }

    pub fn is_mounted(&self) -> bool {
        self.reference.is_mounted()
    }

    pub fn update(&self, callback: impl Fn(&mut T) -> windows_core::Result<()> + 'static) -> bool {
        self.reference.schedule(move |scheduler, node| {
            scheduler.run_swap_chain_host_action(node, SwapChainHostAction::update(callback));
        })
    }

    pub fn invalidate(&self) -> bool {
        self.reference.schedule(move |scheduler, node| {
            scheduler.run_swap_chain_host_action(node, SwapChainHostAction::Invalidate);
        })
    }
}

impl<T: 'static> Default for SwapChainHostRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for SwapChainHostRef<T> {
    fn clone(&self) -> Self {
        Self {
            reference: self.reference.clone(),
            marker: std::marker::PhantomData,
        }
    }
}

impl<T: 'static> fmt::Debug for SwapChainHostRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SwapChainHostRef")
            .field("mounted", &self.is_mounted())
            .finish()
    }
}
