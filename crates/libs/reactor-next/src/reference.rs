use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use crate::core::{NativeWork, NodeId, WindowToken};
use crate::element::View;

const IMPERATIVE_QUEUE_CAPACITY: usize = 4_096;

#[derive(Debug)]
pub(crate) enum HostRequest {
    CloseWindow { identity: WindowToken },
    OpenWindow { identity: WindowToken, root: View },
}

struct WindowRequestState {
    active: bool,
    close_committed: bool,
    close_requested: bool,
    open: bool,
    open_requests: Vec<View>,
    staged_close: bool,
    staged_opens: Vec<View>,
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
                active: false,
                close_committed: false,
                close_requested: false,
                open: true,
                open_requests: Vec::new(),
                staged_close: false,
                staged_opens: Vec::new(),
            })),
        }
    }

    pub(crate) fn begin(&self) {
        let mut state = self.state.borrow_mut();
        assert!(!state.active, "component lifecycle invocation reentered");
        state.active = true;
        state.close_requested = false;
        state.open_requests.clear();
    }

    pub(crate) fn finish(&self) {
        let mut state = self.state.borrow_mut();
        assert!(
            state.active,
            "component lifecycle invocation was not active"
        );
        state.active = false;
        state.staged_close |= state.close_requested;
        state.close_requested = false;
        let requests = std::mem::take(&mut state.open_requests);
        state.staged_opens.extend(requests);
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
        state.open = false;
        state.active = false;
        state.close_requested = false;
        state.open_requests.clear();
        state.staged_close = false;
        state.staged_opens.clear();
    }

    pub(crate) fn commit_close(&self) {
        self.state.borrow_mut().close_committed = true;
    }

    pub(crate) fn reference(&self) -> WindowRef {
        WindowRef {
            endpoint: self.clone(),
        }
    }

    fn request_open(&self, root: View) -> bool {
        let mut state = self.state.borrow_mut();
        if !state.open || !state.active || state.close_committed {
            return false;
        }
        state.open_requests.push(root);
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
        if !state.open || !state.active || state.close_committed {
            return false;
        }
        state.close_requested = true;
        true
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
            .field("active", &state.active)
            .field("close_committed", &state.close_committed)
            .field("open", &state.open)
            .finish()
    }
}

#[doc(hidden)]
pub trait ReferenceType: sealed::Sealed + 'static {}

pub(crate) mod sealed {
    pub trait Sealed {}
}

/// A stable typed reference to a published native element.
///
/// Components normally store references as fields. A reference is unbound before its element is
/// published and after removal, shutdown, or window close. Imperative methods only enqueue work;
/// they do not expose or invoke the native WinUI object.
///
/// References are control-specific:
///
/// ```compile_fail
/// use windows_reactor_next::{Button, ElementRef, TextBox};
///
/// let button = ElementRef::<Button>::new();
/// let _ = TextBox::new().element_ref(&button);
/// ```
pub struct ElementRef<T> {
    target: Rc<RefCell<Option<ReferenceBinding>>>,
    marker: PhantomData<fn() -> T>,
}

impl<T: ReferenceType> ElementRef<T> {
    pub fn new() -> Self {
        Self {
            target: Rc::new(RefCell::new(None)),
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
        let Some(binding) = self.target.borrow().clone() else {
            return false;
        };
        binding.endpoint.enqueue(NativeWork {
            identity: binding.identity,
            work: ImperativeRequest::Focus { node: binding.node },
        })
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

impl<T: ReferenceType> Default for ElementRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Debug for ElementRef<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ElementRef")
            .field("bound", &self.target.borrow().is_some())
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
pub trait FocusControl: ReferenceType {}

#[derive(Clone)]
pub(crate) struct NativeElementRef(Rc<RefCell<Option<ReferenceBinding>>>);

impl NativeElementRef {
    pub(crate) fn identity(&self) -> usize {
        Rc::as_ptr(&self.0) as usize
    }

    pub(crate) fn bind(&self, endpoint: ImperativeEndpoint, identity: WindowToken, node: NodeId) {
        *self.0.borrow_mut() = Some(ReferenceBinding {
            endpoint,
            identity,
            node,
        });
    }

    pub(crate) fn unbind(&self, identity: WindowToken, node: NodeId) {
        let clear = self
            .0
            .borrow()
            .as_ref()
            .is_some_and(|binding| binding.identity == identity && binding.node == node);
        if clear {
            *self.0.borrow_mut() = None;
        }
    }

    pub(crate) fn binding_target(&self) -> Option<(WindowToken, NodeId)> {
        self.0
            .borrow()
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ImperativeRequest {
    Focus { node: NodeId },
}

#[derive(Clone)]
pub(crate) struct ImperativeEndpoint {
    queue: Rc<RefCell<VecDeque<NativeWork<ImperativeRequest>>>>,
    wake: Option<Rc<dyn Fn()>>,
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
        queue.push_back(request);
        drop(queue);
        if let Some(wake) = &self.wake {
            wake();
        }
        true
    }

    pub(crate) fn pop_front(&self) -> Option<NativeWork<ImperativeRequest>> {
        self.queue.borrow_mut().pop_front()
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.queue.borrow().is_empty()
    }

    pub(crate) fn clear(&self) {
        self.queue.borrow_mut().clear();
    }
}
