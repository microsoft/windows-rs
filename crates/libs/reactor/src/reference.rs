use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::fmt;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::core::{NativeWork, NodeId, RuntimeError, WindowToken};
use crate::element::{Callback, View};

const IMPERATIVE_QUEUE_CAPACITY: usize = 4_096;
static NEXT_OBSERVATION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
pub(crate) enum HostRequest {
    CloseWindow { identity: WindowToken },
    OpenWindow { identity: WindowToken, root: View },
}

struct WindowRequestState {
    active: Option<ActiveWindowRequests>,
    lifecycle: WindowRequestLifecycle,
    staged_close: bool,
    staged_opens: Vec<View>,
}

#[derive(Default)]
struct ActiveWindowRequests {
    close: bool,
    opens: Vec<View>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum WindowRequestLifecycle {
    Open,
    CloseCommitted,
    Closed,
    ClosedCommitted,
}

#[derive(Clone)]
pub(crate) struct WindowEndpoint {
    identity: WindowToken,
    state: Rc<RefCell<WindowRequestState>>,
}

impl WindowEndpoint {
    pub(crate) fn new(identity: WindowToken) -> Self {
        Self {
            identity,
            state: Rc::new(RefCell::new(WindowRequestState {
                active: None,
                lifecycle: WindowRequestLifecycle::Open,
                staged_close: false,
                staged_opens: Vec::new(),
            })),
        }
    }

    pub(crate) fn begin(&self) {
        let mut state = self.state.borrow_mut();
        assert!(
            state.active.is_none(),
            "component lifecycle invocation reentered"
        );
        state.active = Some(ActiveWindowRequests::default());
    }

    pub(crate) fn finish(&self) {
        let mut state = self.state.borrow_mut();
        let active = state
            .active
            .take()
            .expect("component lifecycle invocation was not active");
        state.staged_close |= active.close;
        state.staged_opens.extend(active.opens);
    }

    pub(crate) fn take_requests(&self) -> Vec<HostRequest> {
        let mut state = self.state.borrow_mut();
        let mut requests = state
            .staged_opens
            .drain(..)
            .map(|root| HostRequest::OpenWindow {
                identity: self.identity,
                root,
            })
            .collect::<Vec<_>>();
        if state.staged_close {
            state.staged_close = false;
            requests.push(HostRequest::CloseWindow {
                identity: self.identity,
            });
        }
        requests
    }

    pub(crate) fn close(&self) {
        let mut state = self.state.borrow_mut();
        state.lifecycle = match state.lifecycle {
            WindowRequestLifecycle::Open | WindowRequestLifecycle::Closed => {
                WindowRequestLifecycle::Closed
            }
            WindowRequestLifecycle::CloseCommitted | WindowRequestLifecycle::ClosedCommitted => {
                WindowRequestLifecycle::ClosedCommitted
            }
        };
        state.active = None;
        state.staged_close = false;
        state.staged_opens.clear();
    }

    pub(crate) fn commit_close(&self) {
        let mut state = self.state.borrow_mut();
        state.lifecycle = match state.lifecycle {
            WindowRequestLifecycle::Open | WindowRequestLifecycle::CloseCommitted => {
                WindowRequestLifecycle::CloseCommitted
            }
            WindowRequestLifecycle::Closed | WindowRequestLifecycle::ClosedCommitted => {
                WindowRequestLifecycle::ClosedCommitted
            }
        };
    }

    pub(crate) fn reference(&self) -> WindowRef {
        WindowRef {
            endpoint: self.clone(),
        }
    }

    fn request_open(&self, root: View) -> bool {
        let mut state = self.state.borrow_mut();
        if state.lifecycle != WindowRequestLifecycle::Open {
            return false;
        }
        let Some(active) = state.active.as_mut() else {
            return false;
        };
        active.opens.push(root);
        true
    }
}

/// A token-bound capability for a component's owning window.
///
/// Requests are accepted only during `create`, `changed`, or `update` on the owning window. The
/// Pump commits an accepted request after the resulting candidate publishes.
#[derive(Clone)]
pub struct WindowRef {
    endpoint: WindowEndpoint,
}

impl WindowRef {
    /// Requests that the owning window close after the current component turn publishes.
    #[must_use = "false means there is no active component publication"]
    pub fn request_close(&self) -> bool {
        let mut state = self.endpoint.state.borrow_mut();
        if state.lifecycle != WindowRequestLifecycle::Open {
            return false;
        }
        let Some(active) = state.active.as_mut() else {
            return false;
        };
        active.close = true;
        true
    }

    #[cfg(test)]
    pub(crate) fn close_committed(&self) -> bool {
        matches!(
            self.endpoint.state.borrow().lifecycle,
            WindowRequestLifecycle::CloseCommitted | WindowRequestLifecycle::ClosedCommitted
        )
    }

    pub(crate) fn request_open(&self, root: View) -> bool {
        self.endpoint.request_open(root)
    }
}

impl fmt::Debug for WindowRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = self.endpoint.state.borrow();
        formatter
            .debug_struct("WindowRef")
            .field("active", &state.active.is_some())
            .field(
                "close_committed",
                &matches!(
                    state.lifecycle,
                    WindowRequestLifecycle::CloseCommitted
                        | WindowRequestLifecycle::ClosedCommitted
                ),
            )
            .field(
                "open",
                &matches!(
                    state.lifecycle,
                    WindowRequestLifecycle::Open | WindowRequestLifecycle::CloseCommitted
                ),
            )
            .finish()
    }
}

pub(crate) mod sealed {
    pub trait Sealed {}
}

/// A native control that can be bound to an [`ElementRef`].
///
/// This trait is sealed and implemented only for controls whose generated schema supports native
/// references.
pub trait ReferenceControl: sealed::Sealed + 'static {}

/// A stable typed reference to a published native element.
///
/// Components normally store references as fields. A reference is unbound before its element is
/// published and after removal, shutdown, or window close. Imperative methods only enqueue work;
/// they do not expose or invoke the native WinUI object.
///
/// An accepted one-shot request completes exactly once. If its binding is retired before native
/// processing, its completion receives the typed `Unavailable` error.
///
/// References are control-specific:
///
/// ```compile_fail
/// use windows_reactor::{Button, ElementRef, TextBox};
///
/// let button = ElementRef::<Button>::new();
/// let _ = TextBox::new().element_ref(&button);
/// ```
///
/// ```compile_fail
/// use windows_reactor::ElementRef;
///
/// let _ = ElementRef::<u32>::new();
/// ```
pub struct ElementRef<T> {
    target: Rc<RefCell<ReferenceTarget>>,
    marker: PhantomData<fn() -> T>,
}

impl<T: ReferenceControl> ElementRef<T> {
    pub fn new() -> Self {
        Self {
            target: Rc::new(RefCell::new(ReferenceTarget::default())),
            marker: PhantomData,
        }
    }

    pub(crate) fn binding(&self) -> NativeElementRef {
        NativeElementRef(Rc::clone(&self.target))
    }
}

impl<T: FocusControl> ElementRef<T> {
    /// Enqueues programmatic focus for the currently published element.
    ///
    /// `true` means the request was accepted into the host queue. WinUI may later complete the
    /// request with `false` when focus does not move; that is not a host error.
    #[must_use = "false means the reference is currently unbound"]
    pub fn request_focus(&self) -> bool {
        self.request_focus_result(|_| {})
    }

    /// Enqueues programmatic focus and reports the native result asynchronously.
    #[must_use = "false means the reference is currently unbound"]
    pub fn request_focus_result(
        &self,
        completion: impl Fn(Result<bool, FocusError>) + 'static,
    ) -> bool {
        let Some(binding) = self.target.borrow().binding.clone() else {
            return false;
        };
        binding.endpoint.enqueue(NativeWork {
            identity: binding.identity,
            work: ImperativeRequest::Focus {
                node: binding.node,
                completion: Callback::new(move |result: Result<bool, RuntimeError>| {
                    completion(result.map_err(FocusError::from_runtime));
                }),
            },
        })
    }
}

impl ElementRef<crate::WebView2> {
    /// Requests creation of the CoreWebView2 object for the currently published control.
    ///
    /// The returned COM object is the application-facing WebView2 core, not the XAML control
    /// owned by Reactor.
    #[must_use = "false means the reference is currently unbound"]
    pub fn request_core_web_view2(
        &self,
        completion: impl Fn(Result<windows_core::IUnknown, WebView2Error>) + 'static,
    ) -> bool {
        let Some(binding) = self.target.borrow().binding.clone() else {
            return false;
        };
        binding.endpoint.enqueue(NativeWork {
            identity: binding.identity,
            work: ImperativeRequest::InitializeWebView2 {
                node: binding.node,
                completion: Callback::new(
                    move |result: Result<windows_core::IUnknown, RuntimeError>| {
                        completion(result.map_err(WebView2Error::from_runtime));
                    },
                ),
            },
        })
    }
}

impl ElementRef<crate::SwapChainPanel> {
    /// Attaches an application-owned DXGI swap chain to the published panel.
    #[must_use = "false means the reference is currently unbound"]
    pub fn request_set_swap_chain(
        &self,
        swap_chain: windows_core::IUnknown,
        completion: impl Fn(Result<(), SwapChainPanelError>) + 'static,
    ) -> bool {
        self.request_swap_chain(Some(swap_chain), completion)
    }

    /// Detaches the current DXGI swap chain from the published panel.
    #[must_use = "false means the reference is currently unbound"]
    pub fn request_clear_swap_chain(
        &self,
        completion: impl Fn(Result<(), SwapChainPanelError>) + 'static,
    ) -> bool {
        self.request_swap_chain(None, completion)
    }

    /// Observes panel metrics and frame rendering across published panel bindings.
    ///
    /// Registration is accepted before publication. Each new binding creates a native
    /// subscription for that panel, and removal retires the old subscription.
    #[must_use = "the observation stops when the handle is dropped"]
    pub fn observe_surface(
        &self,
        callback: impl Fn(SwapChainPanelEvent) + 'static,
    ) -> ElementObservation {
        self.register_observation(ReferenceObservation::SwapChainPanel(Callback::new(
            callback,
        )))
    }

    fn request_swap_chain(
        &self,
        swap_chain: Option<windows_core::IUnknown>,
        completion: impl Fn(Result<(), SwapChainPanelError>) + 'static,
    ) -> bool {
        let Some(binding) = self.target.borrow().binding.clone() else {
            return false;
        };
        binding.endpoint.enqueue(NativeWork {
            identity: binding.identity,
            work: ImperativeRequest::SetSwapChain {
                node: binding.node,
                swap_chain,
                completion: Callback::new(move |result: Result<(), RuntimeError>| {
                    completion(result.map_err(SwapChainPanelError::from_runtime));
                }),
            },
        })
    }
}

impl ElementRef<crate::Image> {
    /// Assigns an application-owned native ImageSource to the published image.
    #[must_use = "false means the reference is currently unbound"]
    pub fn request_set_native_source(
        &self,
        source: Option<windows_core::IUnknown>,
        completion: impl Fn(Result<(), ImageSourceError>) + 'static,
    ) -> bool {
        let Some(binding) = self.target.borrow().binding.clone() else {
            return false;
        };
        binding.endpoint.enqueue(NativeWork {
            identity: binding.identity,
            work: ImperativeRequest::SetNativeImageSource {
                node: binding.node,
                source,
                completion: Callback::new(move |result: Result<(), RuntimeError>| {
                    completion(result.map_err(ImageSourceError::from_runtime));
                }),
            },
        })
    }

    /// Observes rasterization scale across the image's published bindings.
    ///
    /// Registration is accepted before publication. Each new binding creates a native
    /// subscription after the image enters a live XAML tree.
    #[must_use = "the observation stops when the handle is dropped"]
    pub fn observe_rasterization_scale(
        &self,
        callback: impl Fn(f64) + 'static,
    ) -> ElementObservation {
        self.register_observation(ReferenceObservation::ImageScale(Callback::new(callback)))
    }
}

impl ElementRef<crate::Grid> {
    /// Observes an application-owned lifted Composition host across published bindings.
    ///
    /// Registration is accepted before publication. Each new binding creates a native
    /// subscription for that host, and removal retires the old subscription.
    #[must_use = "the observation stops when the handle is dropped"]
    pub fn observe_composition_host(
        &self,
        callback: impl Fn(CompositionHostEvent) + 'static,
    ) -> ElementObservation {
        self.register_observation(ReferenceObservation::CompositionHost(Callback::new(
            callback,
        )))
    }

    /// Assigns an application-owned lifted Composition visual to the host element.
    #[must_use = "false means the reference is currently unbound"]
    pub fn request_set_child_visual(
        &self,
        visual: Option<windows_core::IUnknown>,
        completion: impl Fn(Result<(), CompositionHostError>) + 'static,
    ) -> bool {
        let Some(binding) = self.target.borrow().binding.clone() else {
            return false;
        };
        binding.endpoint.enqueue(NativeWork {
            identity: binding.identity,
            work: ImperativeRequest::SetCompositionChildVisual {
                node: binding.node,
                visual,
                completion: Callback::new(move |result: Result<(), RuntimeError>| {
                    completion(result.map_err(CompositionHostError::from_runtime));
                }),
            },
        })
    }
}

/// Layout and compositor events from a composition host.
///
/// Width and height are in device-independent pixels (DIPs). Scale converts DIPs to pixels.
#[derive(Clone, Debug, PartialEq)]
pub enum CompositionHostEvent {
    /// Provides the application-safe compositor capability for a newly bound host.
    Ready {
        compositor: windows_core::IUnknown,
        width: f64,
        height: f64,
        scale: f64,
    },
    /// Reports updated layout metrics for the current host.
    Metrics { width: f64, height: f64, scale: f64 },
}

/// Failure reported by an asynchronous native integration request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntegrationError {
    /// A native operation failed with the given HRESULT value.
    Native(i32),
    /// The target is not currently bound or cannot accept the request.
    Unavailable,
}

impl IntegrationError {
    fn from_runtime(error: RuntimeError) -> Self {
        match error {
            RuntimeError::Native(code) => Self::Native(code),
            _ => Self::Unavailable,
        }
    }
}

/// Errors from composition-host requests.
pub type CompositionHostError = IntegrationError;
/// Errors from focus requests.
pub type FocusError = IntegrationError;
/// Errors from image-source requests.
pub type ImageSourceError = IntegrationError;
/// Errors from swap-chain panel requests.
pub type SwapChainPanelError = IntegrationError;
/// Errors from WebView2 integration requests.
pub type WebView2Error = IntegrationError;
/// Rendering and layout events from a swap-chain panel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwapChainPanelEvent {
    /// Reports panel dimensions in DIPs and the pixel scale for each axis.
    Metrics {
        width: f64,
        height: f64,
        scale_x: f32,
        scale_y: f32,
    },
    /// Requests a frame during the composition rendering pass.
    Rendering,
}

impl<T> Clone for ElementRef<T> {
    fn clone(&self) -> Self {
        Self {
            target: Rc::clone(&self.target),
            marker: PhantomData,
        }
    }
}

impl<T: ReferenceControl> Default for ElementRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Debug for ElementRef<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ElementRef")
            .field("bound", &self.target.borrow().binding.is_some())
            .finish()
    }
}

impl<T> PartialEq for ElementRef<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.target, &other.target)
    }
}

impl<T> Eq for ElementRef<T> {}

/// Marks generated controls that accept programmatic focus.
///
/// This trait is sealed and implemented only for controls whose generated schema declares focus
/// support.
pub trait FocusControl: ReferenceControl {}

#[derive(Clone)]
pub(crate) struct NativeElementRef(Rc<RefCell<ReferenceTarget>>);

impl NativeElementRef {
    pub(crate) fn identity(&self) -> usize {
        Rc::as_ptr(&self.0) as usize
    }

    pub(crate) fn bind(&self, endpoint: ImperativeEndpoint, identity: WindowToken, node: NodeId) {
        let reference = self.identity();
        let previous = self.0.borrow().binding.clone();
        let observations = {
            let mut target = self.0.borrow_mut();
            target.observations.retain(|observation| {
                observation
                    .upgrade()
                    .is_some_and(|observation| observation.active.get())
            });
            target
                .observations
                .iter()
                .filter_map(Weak::upgrade)
                .collect::<Vec<_>>()
        };
        if let Some(previous) = previous {
            previous.endpoint.retire_observations(reference);
            for observation in &observations {
                previous.endpoint.enqueue_observation_revocation(
                    previous.identity,
                    previous.node,
                    observation.id,
                );
            }
        }
        let binding = ReferenceBinding {
            endpoint,
            identity,
            node,
        };
        self.0.borrow_mut().binding = Some(binding.clone());
        let requests = observations
            .iter()
            .map(|observation| {
                (
                    observation.id,
                    ObservationRegistration::request(observation, &self.0, &binding),
                )
            })
            .collect();
        binding.endpoint.replace_observations(reference, requests);
    }

    pub(crate) fn unbind(&self, identity: WindowToken, node: NodeId) {
        let binding = self.0.borrow().binding.clone();
        if let Some(binding) = binding
            && binding.identity == identity
            && binding.node == node
        {
            let reference = self.identity();
            binding.endpoint.retire_observations(reference);
            for observation in self
                .0
                .borrow()
                .observations
                .iter()
                .filter_map(Weak::upgrade)
                .filter(|observation| observation.active.get())
            {
                binding.endpoint.enqueue_observation_revocation(
                    binding.identity,
                    binding.node,
                    observation.id,
                );
            }
            self.0.borrow_mut().binding = None;
        }
    }

    pub(crate) fn binding_target(&self) -> Option<(WindowToken, NodeId)> {
        self.0
            .borrow()
            .binding
            .as_ref()
            .map(|binding| (binding.identity, binding.node))
    }
}

impl fmt::Debug for NativeElementRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("NativeElementRef")
    }
}

impl PartialEq for NativeElementRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for NativeElementRef {}

#[derive(Clone)]
struct ReferenceBinding {
    endpoint: ImperativeEndpoint,
    identity: WindowToken,
    node: NodeId,
}

#[derive(Default)]
struct ReferenceTarget {
    binding: Option<ReferenceBinding>,
    observations: Vec<Weak<ObservationRegistration>>,
}

#[derive(Clone)]
enum ReferenceObservation {
    SwapChainPanel(Callback<SwapChainPanelEvent>),
    ImageScale(Callback<f64>),
    CompositionHost(Callback<CompositionHostEvent>),
}

struct ObservationRegistration {
    active: Cell<bool>,
    id: u64,
    observation: ReferenceObservation,
}

impl ObservationRegistration {
    fn request(
        this: &Rc<Self>,
        target: &Rc<RefCell<ReferenceTarget>>,
        binding: &ReferenceBinding,
    ) -> NativeWork<ImperativeRequest> {
        let identity = binding.identity;
        let node = binding.node;
        let target = Rc::downgrade(target);
        let registration = Rc::downgrade(this);
        let work = match &this.observation {
            ReferenceObservation::SwapChainPanel(callback) => {
                ImperativeRequest::ObserveSwapChainPanel {
                    node,
                    observation: this.id,
                    callback: current_binding_callback(
                        target,
                        registration,
                        identity,
                        node,
                        callback.clone(),
                    ),
                }
            }
            ReferenceObservation::ImageScale(callback) => ImperativeRequest::ObserveImageScale {
                node,
                observation: this.id,
                callback: current_binding_callback(
                    target,
                    registration,
                    identity,
                    node,
                    callback.clone(),
                ),
            },
            ReferenceObservation::CompositionHost(callback) => {
                ImperativeRequest::ObserveCompositionHost {
                    node,
                    observation: this.id,
                    callback: current_binding_callback(
                        target,
                        registration,
                        identity,
                        node,
                        callback.clone(),
                    ),
                }
            }
        };
        NativeWork { identity, work }
    }
}

fn current_binding_callback<T: 'static>(
    target: Weak<RefCell<ReferenceTarget>>,
    registration: Weak<ObservationRegistration>,
    identity: WindowToken,
    node: NodeId,
    callback: Callback<T>,
) -> Callback<T> {
    Callback::new_with_acceptance(move |value| {
        let active = registration
            .upgrade()
            .is_some_and(|registration| registration.active.get());
        let current = active
            && target.upgrade().is_some_and(|target| {
                target
                    .borrow()
                    .binding
                    .as_ref()
                    .is_some_and(|binding| binding.identity == identity && binding.node == node)
            });
        current && callback.call(value)
    })
}

impl<T> ElementRef<T> {
    fn register_observation(&self, observation: ReferenceObservation) -> ElementObservation {
        let id = NEXT_OBSERVATION_ID
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(1))
            .unwrap_or_else(|_| panic!("element observation identity exhausted"));
        let registration = Rc::new(ObservationRegistration {
            active: Cell::new(true),
            id,
            observation,
        });
        let mut target = self.target.borrow_mut();
        target.observations.push(Rc::downgrade(&registration));
        if let Some(binding) = &target.binding {
            let request = ObservationRegistration::request(&registration, &self.target, binding);
            binding
                .endpoint
                .enqueue_observation(self.target_identity(), registration.id, request);
        }
        ElementObservation {
            reference: self.target_identity(),
            registration,
            target: Rc::downgrade(&self.target),
        }
    }

    fn target_identity(&self) -> usize {
        Rc::as_ptr(&self.target) as usize
    }
}

/// Keeps an `ElementRef` observation registered.
///
/// Dropping the handle stops callback delivery and prevents the observation from following later
/// bindings. The observed element reference may be captured by the callback without creating an
/// ownership cycle.
pub struct ElementObservation {
    reference: usize,
    registration: Rc<ObservationRegistration>,
    target: Weak<RefCell<ReferenceTarget>>,
}

impl Drop for ElementObservation {
    fn drop(&mut self) {
        self.registration.active.set(false);
        let Some(target) = self.target.upgrade() else {
            return;
        };
        let registration = self.registration.id;
        let mut target = target.borrow_mut();
        target.observations.retain(|observation| {
            observation
                .upgrade()
                .is_some_and(|observation| observation.id != registration)
        });
        if let Some(binding) = &target.binding {
            binding.endpoint.retire_observation(
                self.reference,
                registration,
                binding.identity,
                binding.node,
            );
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ImperativeRequest {
    Focus {
        node: NodeId,
        completion: Callback<Result<bool, RuntimeError>>,
    },
    InitializeWebView2 {
        node: NodeId,
        completion: Callback<Result<windows_core::IUnknown, RuntimeError>>,
    },
    ObserveSwapChainPanel {
        node: NodeId,
        observation: u64,
        callback: Callback<SwapChainPanelEvent>,
    },
    SetSwapChain {
        node: NodeId,
        swap_chain: Option<windows_core::IUnknown>,
        completion: Callback<Result<(), RuntimeError>>,
    },
    SetNativeImageSource {
        node: NodeId,
        source: Option<windows_core::IUnknown>,
        completion: Callback<Result<(), RuntimeError>>,
    },
    ObserveImageScale {
        node: NodeId,
        observation: u64,
        callback: Callback<f64>,
    },
    ObserveCompositionHost {
        node: NodeId,
        observation: u64,
        callback: Callback<CompositionHostEvent>,
    },
    RevokeObservation {
        node: NodeId,
        observation: u64,
    },
    SetCompositionChildVisual {
        node: NodeId,
        visual: Option<windows_core::IUnknown>,
        completion: Callback<Result<(), RuntimeError>>,
    },
}

impl ImperativeRequest {
    pub(crate) fn complete_unavailable(self) {
        match self {
            Self::Focus { node, completion } => {
                _ = completion.call(Err(RuntimeError::MissingNode(node)));
            }
            Self::InitializeWebView2 { node, completion } => {
                _ = completion.call(Err(RuntimeError::MissingNode(node)));
            }
            Self::SetSwapChain {
                node, completion, ..
            }
            | Self::SetNativeImageSource {
                node, completion, ..
            }
            | Self::SetCompositionChildVisual {
                node, completion, ..
            } => {
                _ = completion.call(Err(RuntimeError::MissingNode(node)));
            }
            Self::ObserveSwapChainPanel { .. }
            | Self::ObserveImageScale { .. }
            | Self::ObserveCompositionHost { .. }
            | Self::RevokeObservation { .. } => {}
        }
    }
}

#[derive(Clone)]
pub(crate) struct ImperativeEndpoint {
    queue: Rc<RefCell<VecDeque<QueuedImperative>>>,
    wake: Option<Rc<dyn Fn()>>,
}

enum QueuedImperative {
    OneShot(NativeWork<ImperativeRequest>),
    Observation {
        reference: usize,
        registration: u64,
        request: NativeWork<ImperativeRequest>,
    },
}

impl ImperativeEndpoint {
    pub(crate) fn new(wake: Option<Rc<dyn Fn()>>) -> Self {
        Self {
            queue: Rc::new(RefCell::new(VecDeque::new())),
            wake,
        }
    }

    fn enqueue(&self, request: NativeWork<ImperativeRequest>) -> bool {
        let mut queue = self.queue.borrow_mut();
        if queue.len() >= IMPERATIVE_QUEUE_CAPACITY {
            return false;
        }
        queue.push_back(QueuedImperative::OneShot(request));
        drop(queue);
        self.wake();
        true
    }

    fn enqueue_observation(
        &self,
        reference: usize,
        registration: u64,
        request: NativeWork<ImperativeRequest>,
    ) {
        self.queue
            .borrow_mut()
            .push_back(QueuedImperative::Observation {
                reference,
                registration,
                request,
            });
        self.wake();
    }

    fn replace_observations(
        &self,
        reference: usize,
        requests: Vec<(u64, NativeWork<ImperativeRequest>)>,
    ) {
        let mut queue = self.queue.borrow_mut();
        queue.retain(|queued| {
            !matches!(
                queued,
                QueuedImperative::Observation {
                    reference: queued,
                    ..
                } if *queued == reference
            )
        });
        queue.extend(requests.into_iter().map(|(registration, request)| {
            QueuedImperative::Observation {
                reference,
                registration,
                request,
            }
        }));
        drop(queue);
        self.wake();
    }

    fn retire_observations(&self, reference: usize) {
        self.queue.borrow_mut().retain(|queued| {
            !matches!(
                queued,
                QueuedImperative::Observation {
                    reference: queued,
                    ..
                } if *queued == reference
            )
        });
    }

    fn retire_observation(
        &self,
        reference: usize,
        registration: u64,
        identity: WindowToken,
        node: NodeId,
    ) {
        self.queue.borrow_mut().retain(|queued| {
            !matches!(
                queued,
                QueuedImperative::Observation {
                    reference: queued_reference,
                    registration: queued_registration,
                    ..
                } if *queued_reference == reference && *queued_registration == registration
            )
        });
        self.enqueue_observation_revocation(identity, node, registration);
    }

    fn enqueue_observation_revocation(
        &self,
        identity: WindowToken,
        node: NodeId,
        observation: u64,
    ) {
        self.queue
            .borrow_mut()
            .push_back(QueuedImperative::OneShot(NativeWork {
                identity,
                work: ImperativeRequest::RevokeObservation { node, observation },
            }));
        self.wake();
    }

    pub(crate) fn pop_front(&self) -> Option<NativeWork<ImperativeRequest>> {
        self.queue
            .borrow_mut()
            .pop_front()
            .map(|queued| match queued {
                QueuedImperative::OneShot(request)
                | QueuedImperative::Observation { request, .. } => request,
            })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.queue.borrow().is_empty()
    }

    pub(crate) fn clear(&self) {
        self.queue.borrow_mut().clear();
    }

    pub(crate) fn complete_unavailable(&self) {
        let queued = std::mem::take(&mut *self.queue.borrow_mut());
        for request in queued {
            match request {
                QueuedImperative::OneShot(request)
                | QueuedImperative::Observation { request, .. } => {
                    request.work.complete_unavailable();
                }
            }
        }
    }

    fn wake(&self) {
        if let Some(wake) = &self.wake {
            wake();
        }
    }
}
