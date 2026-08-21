use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::*;
use windows_core::Interface;

#[cfg(feature = "test")]
thread_local! {
    static LIVE_TEST_CLEANUP: Cell<u8> = const { Cell::new(0) };
}

#[allow(
    clippy::missing_transmute_annotations,
    clippy::upper_case_acronyms,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
mod bindings;
pub use bindings::*;
mod app_shim;
pub use app_shim::*;
mod element_factory;
#[allow(unused_qualifications)]
mod generated;
pub use generated::*;

#[derive(Default)]
pub struct WinUiRuntime {
    application: Option<(NodeId, Application)>,
    event_errors: Rc<RefCell<Vec<NativeWork<QueuedEventError>>>>,
    handles: HashMap<NodeId, Handle>,
    events: Rc<RefCell<Vec<NativeWork<QueuedEvent>>>>,
    feedback: Rc<RefCell<HashMap<(NodeId, EventId), FeedbackExpectation>>>,
    identity: Rc<Cell<Option<WindowToken>>>,
    realizations: Rc<RefCell<Vec<NativeWork<RealizationRequest>>>>,
    scheduler: Rc<RefCell<SchedulerState>>,
    subscriptions: HashMap<(NodeId, EventId), windows_core::EventRevoker>,
    virtuals: HashMap<NodeId, element_factory::VirtualHandle>,
    window_closed: Rc<Cell<bool>>,
    window_subscriptions: HashMap<NodeId, windows_core::EventRevoker>,
    windows: HashMap<NodeId, Window>,
    pending_application: Option<Application>,
    #[cfg(feature = "test")]
    reject_next_enqueue: Rc<Cell<bool>>,
}

impl WinUiRuntime {
    pub fn with_application(application: Application) -> Self {
        Self {
            pending_application: Some(application),
            ..Default::default()
        }
    }

    #[cfg(feature = "test")]
    pub fn live_set_text(&self, node: NodeId, value: &str) -> Result<(), RuntimeError> {
        let Some(Handle::TextBox(text_box)) = self.handles.get(&node) else {
            return Err(RuntimeError::UnsupportedKind);
        };
        text_box.SetText(value).map_err(native_error)
    }

    #[cfg(feature = "test")]
    pub fn live_text(&self, node: NodeId) -> Result<String, RuntimeError> {
        let Some(Handle::TextBox(text_box)) = self.handles.get(&node) else {
            return Err(RuntimeError::UnsupportedKind);
        };
        text_box.Text().map_err(native_error)
    }

    #[cfg(feature = "test")]
    pub fn live_range_value(&self, node: NodeId) -> Result<f64, RuntimeError> {
        match self.handles.get(&node) {
            Some(Handle::NumberBox(number_box)) => number_box.Value().map_err(native_error),
            Some(Handle::Slider(slider)) => slider
                .cast::<IRangeBase>()
                .and_then(|slider| slider.Value())
                .map_err(native_error),
            _ => Err(RuntimeError::UnsupportedKind),
        }
    }

    #[cfg(feature = "test")]
    pub fn live_reject_next_enqueue(&self) {
        self.reject_next_enqueue.set(true);
    }

    fn apply_one(&mut self, command: &Command) -> Result<(), RuntimeError> {
        match command {
            Command::CreateApplication { node } => {
                if self.contains(*node) || self.application.is_some() {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                let application = self
                    .pending_application
                    .take()
                    .ok_or(RuntimeError::MissingApplication)?;
                self.application = Some((*node, application));
            }
            Command::CreateWindow { node } => {
                if self.contains(*node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                let window = Window::new().map_err(native_error)?;
                let closed = Rc::clone(&self.window_closed);
                let identity = self.identity.get().unwrap();
                let subscription = window
                    .Closed(move |_, _| {
                        closed.set(true);
                        dispatch_window_closed(identity);
                    })
                    .map_err(native_error)?;
                self.window_subscriptions.insert(*node, subscription);
                self.windows.insert(*node, window);
            }
            Command::ActivateWindow { node } => {
                self.windows
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?
                    .Activate()
                    .map_err(native_error)?;
            }
            Command::Create { node, kind } => {
                if self.contains(*node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                let handle = Handle::create(*kind)?;
                self.handles.insert(*node, handle);
            }
            Command::CreateVirtualCollection { node, item_count } => {
                if self.contains(*node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                let handle = element_factory::VirtualHandle::create(
                    self.identity.get().unwrap(),
                    *node,
                    *item_count,
                    Rc::clone(&self.realizations),
                    self.event_sink()?,
                )
                .map_err(native_error)?;
                self.virtuals.insert(*node, handle);
            }
            Command::ResetVirtualCollection { node, item_count } => {
                self.virtuals
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?
                    .reset(*item_count)
                    .map_err(native_error)?;
            }
            Command::AttachRealized {
                collection,
                container,
                child,
            } => {
                let child = self.ui_element(*child)?;
                self.virtuals
                    .get(collection)
                    .ok_or(RuntimeError::MissingNode(*collection))?
                    .set_content(*container, Some(&child))
                    .map_err(native_error)?;
            }
            Command::DetachRealized {
                collection,
                container,
                ..
            } => {
                self.virtuals
                    .get(collection)
                    .ok_or(RuntimeError::MissingNode(*collection))?
                    .set_content(*container, None)
                    .map_err(native_error)?;
            }
            Command::Destroy { node } => {
                if !self.contains(*node) {
                    return Err(RuntimeError::MissingNode(*node));
                }
                self.subscriptions
                    .retain(|(subscription_node, _), _| subscription_node != node);
                self.window_subscriptions.remove(node);
                if self.handles.remove(node).is_some()
                    || self.virtuals.remove(node).is_some()
                    || self.windows.remove(node).is_some()
                {
                    return Ok(());
                }
                if self
                    .application
                    .as_ref()
                    .is_some_and(|(application, _)| application == node)
                {
                    self.application = None;
                }
            }
            Command::SetProperty {
                node,
                property,
                value,
            } => {
                let handle = self
                    .handles
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                let feedback = expected_feedback(*property, Some(value));
                let feedback_event = feedback.as_ref().map(|(event, _)| *event);
                if let Some((event, expectation)) = feedback {
                    self.feedback
                        .borrow_mut()
                        .insert((*node, event), expectation);
                }
                let result = set_property(handle, *property, value);
                let observation = feedback_event.and_then(|event| {
                    self.feedback
                        .borrow_mut()
                        .remove(&(*node, event))
                        .and_then(|expectation| match expectation {
                            FeedbackExpectation::Normalized { observation } => observation,
                            FeedbackExpectation::Exact(_) => None,
                        })
                });
                result?;
                if let Some(observation) = observation {
                    self.events.borrow_mut().push(NativeWork {
                        identity: self.identity.get().unwrap(),
                        work: observation,
                    });
                    self.schedule_dispatch()?;
                }
            }
            Command::ClearProperty { node, property } => {
                let handle = self
                    .handles
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                let feedback = expected_feedback(*property, None);
                let feedback_event = feedback.as_ref().map(|(event, _)| *event);
                if let Some((event, expectation)) = feedback {
                    self.feedback
                        .borrow_mut()
                        .insert((*node, event), expectation);
                }
                let result = clear_property(handle, *property);
                let observation = feedback_event.and_then(|event| {
                    self.feedback
                        .borrow_mut()
                        .remove(&(*node, event))
                        .and_then(|expectation| match expectation {
                            FeedbackExpectation::Normalized { observation } => observation,
                            FeedbackExpectation::Exact(_) => None,
                        })
                });
                result?;
                if let Some(observation) = observation {
                    self.events.borrow_mut().push(NativeWork {
                        identity: self.identity.get().unwrap(),
                        work: observation,
                    });
                    self.schedule_dispatch()?;
                }
            }
            Command::SubscribeEvent {
                node,
                event,
                revision,
            } => {
                if self.subscriptions.contains_key(&(*node, *event)) {
                    return Err(RuntimeError::DuplicateEvent(*node, *event));
                }
                let handle = self
                    .handles
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                let sink = self.event_sink()?;
                let revoker = subscribe_event(handle, *node, *event, *revision, sink)?;
                self.subscriptions.insert((*node, *event), revoker);
            }
            Command::UnsubscribeEvent { node, event } => {
                let revoker = self
                    .subscriptions
                    .remove(&(*node, *event))
                    .ok_or(RuntimeError::MissingSubscription(*node, *event))?;
                drop(revoker);
            }
            Command::InsertChild {
                parent,
                child,
                index,
            } => self.insert_child(*parent, *child, *index)?,
            Command::RemoveChild { parent, child } => self.remove_child(*parent, *child)?,
            Command::ResetChildren { parent } => self.reset_children(*parent)?,
            Command::SynchronizeChildren { parent, children } => {
                self.reset_children(*parent)?;
                for (index, child) in children.iter().copied().enumerate() {
                    self.insert_child(*parent, child, index)?;
                }
            }
            Command::MoveChild {
                parent,
                child,
                index,
            } => self.move_child(*parent, *child, *index)?,
        }
        Ok(())
    }

    fn contains(&self, node: NodeId) -> bool {
        self.handles.contains_key(&node)
            || self.virtuals.contains_key(&node)
            || self.windows.contains_key(&node)
            || self
                .application
                .as_ref()
                .is_some_and(|(application, _)| *application == node)
    }

    fn insert_child(
        &self,
        parent: NodeId,
        child: NodeId,
        index: usize,
    ) -> Result<(), RuntimeError> {
        let child = self.ui_element(child)?;
        if let Some(window) = self.windows.get(&parent) {
            if index != 0 {
                return Err(RuntimeError::IndexOutOfBounds);
            }
            window.SetContent(&child).map_err(native_error)
        } else {
            let parent = self
                .handles
                .get(&parent)
                .ok_or(RuntimeError::MissingNode(parent))?;
            if let Some(content) = parent.content_control()? {
                if index != 0 {
                    return Err(RuntimeError::IndexOutOfBounds);
                }
                content.SetContent(&child).map_err(native_error)
            } else if let Some(children) = parent.child_collection()? {
                children
                    .InsertAt(index32(index)?, &child)
                    .map_err(native_error)
            } else {
                Err(RuntimeError::UnsupportedKind)
            }
        }
    }

    fn remove_child(&self, parent: NodeId, child: NodeId) -> Result<(), RuntimeError> {
        let child_id = child;
        let child = self.ui_element(child)?;
        if let Some(window) = self.windows.get(&parent) {
            window.SetContent(None::<&UIElement>).map_err(native_error)
        } else {
            let parent = self
                .handles
                .get(&parent)
                .ok_or(RuntimeError::MissingNode(parent))?;
            if let Some(content) = parent.content_control()? {
                content
                    .SetContent(None::<&windows_core::IInspectable>)
                    .map_err(native_error)
            } else if let Some(children) = parent.child_collection()? {
                let index = child_index(&children, child_id, &child)?;
                children.RemoveAt(index).map_err(native_error)
            } else {
                Err(RuntimeError::UnsupportedKind)
            }
        }
    }

    fn reset_children(&self, parent: NodeId) -> Result<(), RuntimeError> {
        if let Some(window) = self.windows.get(&parent) {
            window.SetContent(None::<&UIElement>).map_err(native_error)
        } else {
            let parent = self
                .handles
                .get(&parent)
                .ok_or(RuntimeError::MissingNode(parent))?;
            if let Some(content) = parent.content_control()? {
                content
                    .SetContent(None::<&windows_core::IInspectable>)
                    .map_err(native_error)
            } else if let Some(children) = parent.child_collection()? {
                children.Clear().map_err(native_error)
            } else {
                Err(RuntimeError::UnsupportedKind)
            }
        }
    }

    fn move_child(&self, parent: NodeId, child: NodeId, index: usize) -> Result<(), RuntimeError> {
        let child_id = child;
        let child = self.ui_element(child)?;
        if self.windows.contains_key(&parent) {
            if index != 0 {
                return Err(RuntimeError::IndexOutOfBounds);
            }
            Ok(())
        } else {
            let parent = self
                .handles
                .get(&parent)
                .ok_or(RuntimeError::MissingNode(parent))?;
            if parent.content_control()?.is_some() {
                if index == 0 {
                    Ok(())
                } else {
                    Err(RuntimeError::IndexOutOfBounds)
                }
            } else if let Some(children) = parent.child_collection()? {
                let from = child_index(&children, child_id, &child)?;
                children.RemoveAt(from).map_err(native_error)?;
                children
                    .InsertAt(index32(index)?, &child)
                    .map_err(native_error)
            } else {
                Err(RuntimeError::UnsupportedKind)
            }
        }
    }

    fn ui_element(&self, node: NodeId) -> Result<UIElement, RuntimeError> {
        if let Some(handle) = self.handles.get(&node) {
            handle.ui_element().map_err(native_error)
        } else if let Some(handle) = self.virtuals.get(&node) {
            handle.ui_element().map_err(native_error)
        } else {
            Err(RuntimeError::MissingNode(node))
        }
    }

    fn event_sink(&self) -> Result<EventSink, RuntimeError> {
        Ok(EventSink {
            queue: Rc::clone(&self.events),
            errors: Rc::clone(&self.event_errors),
            feedback: Rc::clone(&self.feedback),
            dispatcher: DispatcherQueue::GetForCurrentThread().map_err(native_error)?,
            identity: self.identity.get().unwrap(),
            current_identity: Rc::clone(&self.identity),
            scheduler: Rc::clone(&self.scheduler),
            #[cfg(feature = "test")]
            reject_next_enqueue: Rc::clone(&self.reject_next_enqueue),
        })
    }

    pub fn schedule_dispatch(&self) -> Result<(), RuntimeError> {
        self.event_sink()?.request(WorkPriority::Low)
    }

    pub fn close_scheduler(&self) {
        self.scheduler.borrow_mut().close();
    }
}

#[derive(Clone)]
pub struct EventSink {
    queue: Rc<RefCell<Vec<NativeWork<QueuedEvent>>>>,
    errors: Rc<RefCell<Vec<NativeWork<QueuedEventError>>>>,
    feedback: Rc<RefCell<HashMap<(NodeId, EventId), FeedbackExpectation>>>,
    dispatcher: DispatcherQueue,
    identity: WindowToken,
    current_identity: Rc<Cell<Option<WindowToken>>>,
    scheduler: Rc<RefCell<SchedulerState>>,
    #[cfg(feature = "test")]
    reject_next_enqueue: Rc<Cell<bool>>,
}

impl EventSink {
    pub fn enqueue(&self, node: NodeId, event: EventId, revision: u32, payload: EventPayload) {
        {
            let mut feedback = self.feedback.borrow_mut();
            if let Some(expected) = feedback.get_mut(&(node, event)) {
                match expected {
                    // Keep the expectation active until the setter returns so every synchronous
                    // echo from the same native mutation is covered.
                    FeedbackExpectation::Exact(expected) if expected == &payload => return,
                    FeedbackExpectation::Normalized { observation } => {
                        *observation =
                            Some(QueuedEvent::observation(node, event, revision, payload));
                        return;
                    }
                    FeedbackExpectation::Exact(_) => {}
                }
            }
        }
        self.queue.borrow_mut().push(NativeWork {
            identity: self.identity,
            work: QueuedEvent::new(node, event, revision, payload),
        });
        self.schedule();
    }

    pub fn error(&self, node: NodeId, event: EventId, revision: u32, error: RuntimeError) {
        self.errors.borrow_mut().push(NativeWork {
            identity: self.identity,
            work: QueuedEventError {
                node,
                event,
                revision,
                error,
            },
        });
        self.schedule();
    }

    pub fn wake(&self) {
        self.schedule();
    }

    fn schedule(&self) {
        match self.request(WorkPriority::Normal) {
            Ok(()) | Err(RuntimeError::SchedulerClosed) => {}
            Err(error) => fail_native_scheduler(error),
        }
    }

    fn request(&self, priority: WorkPriority) -> Result<(), RuntimeError> {
        let action = self.scheduler.borrow_mut().request(priority);
        Self::perform(
            action,
            self.identity,
            &self.current_identity,
            &self.scheduler,
            &self.dispatcher,
            #[cfg(feature = "test")]
            &self.reject_next_enqueue,
        )
    }

    fn perform(
        action: ScheduleAction,
        identity: WindowToken,
        current_identity: &Rc<Cell<Option<WindowToken>>>,
        scheduler: &Rc<RefCell<SchedulerState>>,
        dispatcher: &DispatcherQueue,
        #[cfg(feature = "test")] reject_next_enqueue: &Rc<Cell<bool>>,
    ) -> Result<(), RuntimeError> {
        let ScheduleAction::Enqueue(ticket) = action else {
            return match action {
                ScheduleAction::Closed => Err(RuntimeError::SchedulerClosed),
                _ => Ok(()),
            };
        };
        #[cfg(feature = "test")]
        if reject_next_enqueue.replace(false) {
            scheduler.borrow_mut().enqueue_failed(ticket);
            return Err(RuntimeError::DispatcherRejected);
        }
        let current_identity_capture = Rc::clone(current_identity);
        let scheduler_capture = Rc::clone(scheduler);
        let dispatcher_capture = dispatcher.clone();
        #[cfg(feature = "test")]
        let reject_next_enqueue_capture = Rc::clone(reject_next_enqueue);
        let handler = DispatcherQueueHandler::new(move || {
            if current_identity_capture.get() != Some(identity) {
                let action = {
                    let mut scheduler = scheduler_capture.borrow_mut();
                    if !scheduler.begin_dispatch(ticket) {
                        return;
                    }
                    _ = scheduler.request(WorkPriority::Normal);
                    scheduler.finish_dispatch()
                };
                if let Some(identity) = current_identity_capture.get()
                    && let Err(error) = Self::perform(
                        action,
                        identity,
                        &current_identity_capture,
                        &scheduler_capture,
                        &dispatcher_capture,
                        #[cfg(feature = "test")]
                        &reject_next_enqueue_capture,
                    )
                    && error != RuntimeError::SchedulerClosed
                {
                    fail_native_scheduler(error);
                }
                return;
            }
            if !scheduler_capture.borrow_mut().begin_dispatch(ticket) {
                return;
            }
            dispatch_native_events(identity);
            let action = scheduler_capture.borrow_mut().finish_dispatch();
            if let Some(identity) = current_identity_capture.get()
                && let Err(error) = Self::perform(
                    action,
                    identity,
                    &current_identity_capture,
                    &scheduler_capture,
                    &dispatcher_capture,
                    #[cfg(feature = "test")]
                    &reject_next_enqueue_capture,
                )
                && error != RuntimeError::SchedulerClosed
            {
                fail_native_scheduler(error);
            }
        });
        let priority = match ticket.priority {
            WorkPriority::Low => DispatcherQueuePriority::Low,
            WorkPriority::Normal => DispatcherQueuePriority::Normal,
        };
        match dispatcher.TryEnqueueWithPriority(priority, &handler) {
            Ok(true) => Ok(()),
            Ok(false) => {
                scheduler.borrow_mut().enqueue_failed(ticket);
                Err(RuntimeError::DispatcherRejected)
            }
            Err(error) => {
                scheduler.borrow_mut().enqueue_failed(ticket);
                Err(native_error(error))
            }
        }
    }
}

fn child_index(
    children: &UIElementCollection,
    child_id: NodeId,
    child: &UIElement,
) -> Result<u32, RuntimeError> {
    let size = children.Size().map_err(native_error)?;
    (0..size)
        .find(|index| children.GetAt(*index).as_ref() == Ok(child))
        .ok_or(RuntimeError::ChildNotFound(child_id))
}

fn index32(index: usize) -> Result<u32, RuntimeError> {
    index.try_into().map_err(|_| RuntimeError::IndexOutOfBounds)
}

impl NativeRuntime for WinUiRuntime {
    fn apply(&mut self, commands: &[Command]) -> Result<(), NativeApplyError> {
        for (index, command) in commands.iter().enumerate() {
            self.apply_one(command).map_err(|error| NativeApplyError {
                command: index,
                error,
            })?;
        }
        Ok(())
    }

    fn reset(&mut self) {
        self.subscriptions.clear();
        if self.window_closed.get() {
            for (_, subscription) in self.window_subscriptions.drain() {
                subscription.into_token();
            }
        } else {
            self.window_subscriptions.clear();
            for window in self.windows.values() {
                _ = window.Close();
            }
        }
        self.windows.clear();
        self.handles.clear();
        self.virtuals.clear();
        self.application = None;
        self.pending_application = None;
        self.event_errors.borrow_mut().clear();
        self.events.borrow_mut().clear();
        self.feedback.borrow_mut().clear();
        self.realizations.borrow_mut().clear();
        self.window_closed.set(false);
    }

    fn native_window_closed(&mut self) {
        for (_, subscription) in self.window_subscriptions.drain() {
            subscription.into_token();
        }
    }

    fn component_waker(&self) -> Option<Rc<dyn Fn()>> {
        let sink = self.event_sink().ok()?;
        Some(Rc::new(move || sink.wake()))
    }

    fn component_background_waker(&self) -> Option<Arc<dyn Fn() -> bool + Send + Sync>> {
        let dispatcher = DispatcherQueue::GetForCurrentThread().ok()?;
        let identity = self.identity.get()?;
        let queued = Arc::new(AtomicBool::new(false));
        Some(Arc::new(move || {
            if queued.swap(true, Ordering::AcqRel) {
                return true;
            }
            let handler_queued = Arc::clone(&queued);
            let handler = DispatcherQueueHandler::new(move || {
                handler_queued.store(false, Ordering::Release);
                dispatch_native_events(identity);
            });
            if matches!(
                dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &handler),
                Ok(true)
            ) {
                true
            } else {
                queued.store(false, Ordering::Release);
                false
            }
        }))
    }

    fn set_identity(&mut self, identity: WindowToken) {
        if self
            .identity
            .get()
            .is_none_or(|current| current != identity)
        {
            self.scheduler.borrow_mut().open();
        }
        self.identity.set(Some(identity));
    }

    fn drain_events(&mut self) -> Vec<NativeWork<QueuedEvent>> {
        self.events.borrow_mut().drain(..).collect()
    }

    fn drain_event_errors(&mut self) -> Vec<NativeWork<QueuedEventError>> {
        self.event_errors.borrow_mut().drain(..).collect()
    }

    fn drain_realizations(&mut self) -> Vec<NativeWork<RealizationRequest>> {
        self.realizations.borrow_mut().drain(..).collect()
    }
}

impl WinUiRuntime {
    #[cfg(feature = "test")]
    pub(crate) fn live_window(&self) -> Result<Window, RuntimeError> {
        self.windows
            .values()
            .next()
            .cloned()
            .ok_or(RuntimeError::MissingApplication)
    }
}

fn native_error(error: windows_core::Error) -> RuntimeError {
    RuntimeError::Native(error.code().0)
}

pub fn bootstrap_runtime() -> windows_core::Result<()> {
    unsafe {
        MddBootstrapInitialize2(
            WINDOWSAPPSDK_RELEASE_MAJORMINOR as u32,
            WINDOWSAPPSDK_RELEASE_VERSION_TAG_W.as_ptr(),
            PACKAGE_VERSION {
                Anonymous: PACKAGE_VERSION_0 {
                    Version: WINDOWSAPPSDK_RUNTIME_VERSION_UINT64,
                },
            },
            MddBootstrapInitializeOptions_OnNoMatch_ShowUI
                | MddBootstrapInitializeOptions_OnPackageIdentity_NOOP,
        )
        .ok()
    }
}

pub fn initialize_ui_thread() -> windows_core::Result<()> {
    unsafe {
        _ = SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }
    let result = unsafe { CoInitializeEx(std::ptr::null(), COINIT_APARTMENTTHREADED as u32) };
    if result == RPC_E_CHANGED_MODE {
        return Err(windows_core::Error::new(
            RPC_E_CHANGED_MODE,
            "WinUI requires an STA thread",
        ));
    }
    result.ok()
}

pub fn exit_ui_thread() {
    unsafe {
        PostQuitMessage(0);
    }
}

#[cfg(feature = "test")]
pub fn mark_live_test_cleanup() {
    LIVE_TEST_CLEANUP.with(|cleanup| cleanup.set(cleanup.get().saturating_add(1)));
}

#[cfg(feature = "test")]
pub(crate) fn live_test_cleanup_count() -> u8 {
    LIVE_TEST_CLEANUP.with(Cell::get)
}

#[cfg(feature = "test")]
pub fn live_resources_installed() -> windows_core::Result<bool> {
    Ok(Application::Current()?
        .Resources()?
        .MergedDictionaries()?
        .Size()?
        != 0)
}

#[cfg(feature = "test")]
pub fn schedule_live_test_exit(success: bool) -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let handler = DispatcherQueueHandler::new(move || {
        std::process::exit(i32::from(!success));
    });
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &handler)? {
        Ok(())
    } else {
        std::process::exit(1);
    }
}
