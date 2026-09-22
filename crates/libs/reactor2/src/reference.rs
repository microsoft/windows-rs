use super::*;
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicU64, Ordering};

const IMPERATIVE_QUEUE_CAPACITY: usize = 4_096;
static NEXT_BINDING_ID: AtomicU64 = AtomicU64::new(1);
static NEXT_OBSERVATION_ID: AtomicU64 = AtomicU64::new(1);

pub enum AnyElement {}

/// A declaration reference whose type limits control-specific integration APIs.
///
/// ```compile_fail
/// use windows_reactor2::{ElementRef, Grid, Image};
///
/// let image = ElementRef::<Image>::new();
/// let _ = Grid::new().element_ref(&image);
/// ```
pub struct ElementRef<T = AnyElement> {
    target: Rc<RefCell<ReferenceTarget>>,
    marker: PhantomData<fn() -> T>,
}

impl<T> ElementRef<T> {
    pub fn new() -> Self {
        Self {
            target: Rc::new(RefCell::new(ReferenceTarget::default())),
            marker: PhantomData,
        }
    }

    pub fn get(&self) -> Option<ObjectId> {
        self.target
            .borrow()
            .binding
            .as_ref()
            .map(|binding| binding.object)
            .or_else(|| self.target.borrow().direct)
    }

    pub(crate) fn bind(&self, endpoint: ReferenceEndpoint, object: ObjectId) {
        self.retire_current_binding();
        let binding = ReferenceBinding {
            id: NEXT_BINDING_ID.fetch_add(1, Ordering::Relaxed),
            endpoint,
            object,
        };
        let observations = {
            let mut target = self.target.borrow_mut();
            target.observations.retain(|observation| {
                observation
                    .upgrade()
                    .is_some_and(|observation| observation.active.get())
            });
            target.binding = Some(binding.clone());
            target
                .observations
                .iter()
                .filter_map(Weak::upgrade)
                .collect::<Vec<_>>()
        };
        for observation in observations {
            binding.endpoint.enqueue(QueuedImperative {
                binding: binding.id,
                object,
                target: Some(Rc::downgrade(&self.target)),
                request: observation.request(&self.target, &binding),
            });
        }
    }

    pub(crate) fn clear(&self, object: ObjectId) {
        if self.get() == Some(object) {
            self.retire_current_binding();
            let mut target = self.target.borrow_mut();
            target.binding = None;
            target.direct = None;
        }
    }

    pub(crate) fn set(&self, object: Option<ObjectId>) {
        let mut target = self.target.borrow_mut();
        target.binding = None;
        target.direct = object;
    }

    pub(crate) fn identity(&self) -> usize {
        Rc::as_ptr(&self.target) as usize
    }

    pub(crate) fn erased(&self) -> ElementRef {
        ElementRef {
            target: Rc::clone(&self.target),
            marker: PhantomData,
        }
    }

    fn binding(&self) -> Option<ReferenceBinding> {
        self.target.borrow().binding.clone()
    }

    fn retire_current_binding(&self) {
        let target = self.target.borrow();
        let Some(binding) = &target.binding else {
            return;
        };
        for observation in target.observations.iter().filter_map(Weak::upgrade) {
            binding.endpoint.enqueue(QueuedImperative {
                binding: binding.id,
                object: binding.object,
                target: None,
                request: ImperativeRequest::RevokeObservation {
                    object: binding.object,
                    observation: observation.id,
                },
            });
        }
    }

    fn enqueue(
        &self,
        request: impl FnOnce(ObjectId, u64, Weak<RefCell<ReferenceTarget>>) -> ImperativeRequest,
    ) -> bool {
        let Some(binding) = self.binding() else {
            return false;
        };
        binding.endpoint.enqueue(QueuedImperative {
            binding: binding.id,
            object: binding.object,
            target: Some(Rc::downgrade(&self.target)),
            request: request(binding.object, binding.id, Rc::downgrade(&self.target)),
        })
    }

    fn observe(&self, kind: ReferenceObservation) -> ElementObservation {
        let registration = Rc::new(ObservationRegistration {
            id: NEXT_OBSERVATION_ID.fetch_add(1, Ordering::Relaxed),
            active: Cell::new(true),
            kind,
            target: Rc::downgrade(&self.target),
        });
        let binding = {
            let mut target = self.target.borrow_mut();
            target.observations.push(Rc::downgrade(&registration));
            target.binding.clone()
        };
        if let Some(binding) = binding {
            binding.endpoint.enqueue(QueuedImperative {
                binding: binding.id,
                object: binding.object,
                target: Some(Rc::downgrade(&self.target)),
                request: registration.request(&self.target, &binding),
            });
        }
        ElementObservation(registration)
    }
}

impl<T> Clone for ElementRef<T> {
    fn clone(&self) -> Self {
        Self {
            target: Rc::clone(&self.target),
            marker: PhantomData,
        }
    }
}

impl<T> Default for ElementRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::fmt::Debug for ElementRef<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("ElementRef")
            .field(&self.get())
            .finish()
    }
}

impl<T> PartialEq for ElementRef<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.target, &other.target)
    }
}

impl<T> Eq for ElementRef<T> {}

impl ElementRef<WebView2> {
    #[must_use]
    pub fn request_core_web_view2(
        &self,
        completion: impl Fn(Result<windows_core::IUnknown, IntegrationError>) + 'static,
    ) -> bool {
        self.enqueue(
            move |object, binding, target| ImperativeRequest::InitializeWebView2 {
                object,
                completion: current_completion(target, binding, completion),
            },
        )
    }
}

impl ElementRef<SwapChainPanel> {
    #[must_use]
    pub fn request_set_swap_chain(
        &self,
        swap_chain: windows_core::IUnknown,
        completion: impl Fn(Result<(), IntegrationError>) + 'static,
    ) -> bool {
        self.request_swap_chain(Some(swap_chain), completion)
    }

    #[must_use]
    pub fn request_clear_swap_chain(
        &self,
        completion: impl Fn(Result<(), IntegrationError>) + 'static,
    ) -> bool {
        self.request_swap_chain(None, completion)
    }

    #[must_use]
    pub fn request_surface_frame(
        &self,
        completion: impl Fn(Result<(), IntegrationError>) + 'static,
    ) -> bool {
        self.enqueue(
            move |object, binding, target| ImperativeRequest::RequestSwapChainPanelFrame {
                object,
                completion: current_completion(target, binding, completion),
            },
        )
    }

    #[must_use]
    pub fn observe_surface(
        &self,
        callback: impl Fn(SwapChainPanelEvent) + 'static,
    ) -> ElementObservation {
        self.observe(ReferenceObservation::SwapChainPanel(Callback::new(
            callback,
        )))
    }

    fn request_swap_chain(
        &self,
        swap_chain: Option<windows_core::IUnknown>,
        completion: impl Fn(Result<(), IntegrationError>) + 'static,
    ) -> bool {
        self.enqueue(
            move |object, binding, target| ImperativeRequest::SetSwapChain {
                object,
                swap_chain,
                completion: current_completion(target, binding, completion),
            },
        )
    }
}

impl ElementRef<Image> {
    #[must_use]
    pub fn request_set_native_source(
        &self,
        source: Option<windows_core::IUnknown>,
        completion: impl Fn(Result<(), IntegrationError>) + 'static,
    ) -> bool {
        self.enqueue(
            move |object, binding, target| ImperativeRequest::SetNativeImageSource {
                object,
                source,
                completion: current_completion(target, binding, completion),
            },
        )
    }

    #[must_use]
    pub fn observe_rasterization_scale(
        &self,
        callback: impl Fn(f64) + 'static,
    ) -> ElementObservation {
        self.observe(ReferenceObservation::ImageScale(Callback::new(callback)))
    }
}

impl ElementRef<Grid> {
    #[must_use]
    pub fn observe_composition_host(
        &self,
        callback: impl Fn(CompositionHostEvent) + 'static,
    ) -> ElementObservation {
        self.observe(ReferenceObservation::CompositionHost(Callback::new(
            callback,
        )))
    }

    #[must_use]
    pub fn request_set_child_visual(
        &self,
        visual: Option<windows_core::IUnknown>,
        completion: impl Fn(Result<(), IntegrationError>) + 'static,
    ) -> bool {
        self.enqueue(
            move |object, binding, target| ImperativeRequest::SetCompositionChildVisual {
                object,
                visual,
                completion: current_completion(target, binding, completion),
            },
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompositionHostEvent {
    Ready {
        compositor: windows_core::IUnknown,
        width: f64,
        height: f64,
        scale: f64,
    },
    Metrics {
        width: f64,
        height: f64,
        scale: f64,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegrationError {
    Native(i32),
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwapChainPanelEvent {
    Metrics {
        binding: u64,
        width: f64,
        height: f64,
        scale_x: f32,
        scale_y: f32,
    },
    Rendering,
}

pub struct ElementObservation(Rc<ObservationRegistration>);

impl Drop for ElementObservation {
    fn drop(&mut self) {
        self.0.active.set(false);
        let Some(target) = self.0.target.upgrade() else {
            return;
        };
        let binding = target.borrow().binding.clone();
        let Some(binding) = binding else {
            return;
        };
        binding.endpoint.enqueue(QueuedImperative {
            binding: binding.id,
            object: binding.object,
            target: None,
            request: ImperativeRequest::RevokeObservation {
                object: binding.object,
                observation: self.0.id,
            },
        });
    }
}

#[derive(Clone)]
struct ReferenceBinding {
    id: u64,
    endpoint: ReferenceEndpoint,
    object: ObjectId,
}

#[derive(Default)]
struct ReferenceTarget {
    binding: Option<ReferenceBinding>,
    direct: Option<ObjectId>,
    observations: Vec<Weak<ObservationRegistration>>,
}

struct ObservationRegistration {
    id: u64,
    active: Cell<bool>,
    kind: ReferenceObservation,
    target: Weak<RefCell<ReferenceTarget>>,
}

enum ReferenceObservation {
    SwapChainPanel(Callback<SwapChainPanelEvent>),
    ImageScale(Callback<f64>),
    CompositionHost(Callback<CompositionHostEvent>),
}

impl ObservationRegistration {
    fn request(
        &self,
        target: &Rc<RefCell<ReferenceTarget>>,
        binding: &ReferenceBinding,
    ) -> ImperativeRequest {
        match &self.kind {
            ReferenceObservation::SwapChainPanel(callback) => {
                ImperativeRequest::ObserveSwapChainPanel {
                    object: binding.object,
                    observation: self.id,
                    binding: binding.id,
                    callback: current_observation(
                        Rc::downgrade(target),
                        binding.id,
                        self.id,
                        callback.clone(),
                    ),
                }
            }
            ReferenceObservation::ImageScale(callback) => ImperativeRequest::ObserveImageScale {
                object: binding.object,
                observation: self.id,
                callback: current_observation(
                    Rc::downgrade(target),
                    binding.id,
                    self.id,
                    callback.clone(),
                ),
            },
            ReferenceObservation::CompositionHost(callback) => {
                ImperativeRequest::ObserveCompositionHost {
                    object: binding.object,
                    observation: self.id,
                    callback: current_observation(
                        Rc::downgrade(target),
                        binding.id,
                        self.id,
                        callback.clone(),
                    ),
                }
            }
        }
    }
}

fn current_completion<T: 'static>(
    target: Weak<RefCell<ReferenceTarget>>,
    binding: u64,
    completion: impl Fn(Result<T, IntegrationError>) + 'static,
) -> Callback<Result<T, IntegrationError>> {
    Callback::new(move |result| {
        let current = target.upgrade().is_some_and(|target| {
            target
                .borrow()
                .binding
                .as_ref()
                .is_some_and(|current| current.id == binding)
        });
        completion(if current {
            result
        } else {
            Err(IntegrationError::Unavailable)
        });
    })
}

fn current_observation<T: 'static>(
    target: Weak<RefCell<ReferenceTarget>>,
    binding: u64,
    observation: u64,
    callback: Callback<T>,
) -> Callback<T> {
    Callback::new(move |value| {
        let current = target.upgrade().is_some_and(|target| {
            let target = target.borrow();
            target
                .binding
                .as_ref()
                .is_some_and(|current| current.id == binding)
                && target
                    .observations
                    .iter()
                    .filter_map(Weak::upgrade)
                    .any(|registration| registration.id == observation && registration.active.get())
        });
        if current {
            callback.call(value);
        }
    })
}

#[derive(Clone)]
pub(crate) struct ReferenceEndpoint {
    queue: Rc<RefCell<VecDeque<QueuedImperative>>>,
    waker: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
}

impl ReferenceEndpoint {
    pub(crate) fn new() -> Self {
        Self {
            queue: Rc::new(RefCell::new(VecDeque::new())),
            waker: Rc::new(RefCell::new(None)),
        }
    }

    fn enqueue(&self, request: QueuedImperative) -> bool {
        let mut queue = self.queue.borrow_mut();
        if queue.len() >= IMPERATIVE_QUEUE_CAPACITY {
            drop(queue);
            request.request.complete_unavailable();
            return false;
        }
        queue.push_back(request);
        drop(queue);
        if let Some(waker) = self.waker.borrow().as_ref() {
            waker();
        }
        true
    }

    pub(crate) fn pop(&self) -> Option<QueuedImperative> {
        self.queue.borrow_mut().pop_front()
    }

    pub(crate) fn set_waker(&self, waker: impl Fn() + 'static) {
        *self.waker.borrow_mut() = Some(Rc::new(waker));
    }

    pub(crate) fn wake(&self) {
        let waker = self.waker.borrow().clone();
        if let Some(waker) = waker {
            waker();
        }
    }

    pub(crate) fn clear(&self) {
        let requests = std::mem::take(&mut *self.queue.borrow_mut());
        for request in requests {
            request.request.complete_unavailable();
        }
        self.waker.borrow_mut().take();
    }
}

pub(crate) struct QueuedImperative {
    pub binding: u64,
    pub object: ObjectId,
    target: Option<Weak<RefCell<ReferenceTarget>>>,
    pub request: ImperativeRequest,
}

impl QueuedImperative {
    pub(crate) fn is_current(&self) -> bool {
        self.target.as_ref().is_none_or(|target| {
            target.upgrade().is_some_and(|target| {
                target.borrow().binding.as_ref().is_some_and(|binding| {
                    binding.id == self.binding && binding.object == self.object
                })
            })
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ImperativeRequest {
    InitializeWebView2 {
        object: ObjectId,
        completion: Callback<Result<windows_core::IUnknown, IntegrationError>>,
    },
    ObserveSwapChainPanel {
        object: ObjectId,
        observation: u64,
        binding: u64,
        callback: Callback<SwapChainPanelEvent>,
    },
    RequestSwapChainPanelFrame {
        object: ObjectId,
        completion: Callback<Result<(), IntegrationError>>,
    },
    SetSwapChain {
        object: ObjectId,
        swap_chain: Option<windows_core::IUnknown>,
        completion: Callback<Result<(), IntegrationError>>,
    },
    SetNativeImageSource {
        object: ObjectId,
        source: Option<windows_core::IUnknown>,
        completion: Callback<Result<(), IntegrationError>>,
    },
    ObserveImageScale {
        object: ObjectId,
        observation: u64,
        callback: Callback<f64>,
    },
    ObserveCompositionHost {
        object: ObjectId,
        observation: u64,
        callback: Callback<CompositionHostEvent>,
    },
    RevokeObservation {
        object: ObjectId,
        observation: u64,
    },
    SetCompositionChildVisual {
        object: ObjectId,
        visual: Option<windows_core::IUnknown>,
        completion: Callback<Result<(), IntegrationError>>,
    },
}

impl ImperativeRequest {
    pub(crate) fn object(&self) -> ObjectId {
        match self {
            Self::InitializeWebView2 { object, .. }
            | Self::ObserveSwapChainPanel { object, .. }
            | Self::RequestSwapChainPanelFrame { object, .. }
            | Self::SetSwapChain { object, .. }
            | Self::SetNativeImageSource { object, .. }
            | Self::ObserveImageScale { object, .. }
            | Self::ObserveCompositionHost { object, .. }
            | Self::RevokeObservation { object, .. }
            | Self::SetCompositionChildVisual { object, .. } => *object,
        }
    }

    pub(crate) fn complete_unavailable(&self) {
        match self {
            Self::InitializeWebView2 { completion, .. } => {
                completion.call(Err(IntegrationError::Unavailable));
            }
            Self::RequestSwapChainPanelFrame { completion, .. }
            | Self::SetSwapChain { completion, .. }
            | Self::SetNativeImageSource { completion, .. }
            | Self::SetCompositionChildVisual { completion, .. } => {
                completion.call(Err(IntegrationError::Unavailable));
            }
            _ => {}
        }
    }
}
