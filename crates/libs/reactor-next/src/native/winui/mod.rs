use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use super::*;
use windows_core::Interface;

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
    event_errors: Rc<RefCell<Vec<RuntimeError>>>,
    handles: HashMap<NodeId, Handle>,
    events: Rc<RefCell<Vec<QueuedEvent>>>,
    event_tick_scheduled: Rc<Cell<bool>>,
    feedback: Rc<RefCell<HashMap<(NodeId, EventId), EventPayload>>>,
    realizations: Rc<RefCell<Vec<RealizationRequest>>>,
    subscriptions: HashMap<(NodeId, EventId), windows_core::EventRevoker>,
    virtuals: HashMap<NodeId, element_factory::VirtualHandle>,
    windows: HashMap<NodeId, Window>,
}

impl WinUiRuntime {
    fn apply_one(&mut self, command: &Command) -> Result<(), RuntimeError> {
        match command {
            Command::CreateApplication { node } => {
                if self.contains(*node) || self.application.is_some() {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                self.application = Some((*node, create_application().map_err(native_error)?));
            }
            Command::CreateWindow { node } => {
                if self.contains(*node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                self.windows
                    .insert(*node, Window::new().map_err(native_error)?);
            }
            Command::ActivateWindow { node } => {
                self.windows
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?
                    .Activate()
                    .map_err(native_error)?;
            }
            Command::ResetWindowContent { window } => {
                let window = self
                    .windows
                    .get(window)
                    .ok_or(RuntimeError::MissingNode(*window))?;
                self.subscriptions.clear();
                window
                    .SetContent(None::<&UIElement>)
                    .map_err(native_error)?;
                self.handles.clear();
                self.virtuals.clear();
                self.feedback.borrow_mut().clear();
                self.realizations.borrow_mut().clear();
                self.events.borrow_mut().clear();
                self.event_errors.borrow_mut().clear();
                self.event_tick_scheduled.set(false);
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
                if let Some((event, value)) = &feedback {
                    self.feedback
                        .borrow_mut()
                        .insert((*node, *event), value.clone());
                }
                let result = set_property(handle, *property, value);
                if let Some((event, _)) = feedback {
                    self.feedback.borrow_mut().remove(&(*node, event));
                }
                result?;
            }
            Command::ClearProperty { node, property } => {
                let handle = self
                    .handles
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                let feedback = expected_feedback(*property, None);
                if let Some((event, value)) = &feedback {
                    self.feedback
                        .borrow_mut()
                        .insert((*node, *event), value.clone());
                }
                let result = clear_property(handle, *property);
                if let Some((event, _)) = feedback {
                    self.feedback.borrow_mut().remove(&(*node, event));
                }
                result?;
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
            scheduled: Rc::clone(&self.event_tick_scheduled),
            dispatcher: DispatcherQueue::GetForCurrentThread().map_err(native_error)?,
        })
    }
}

#[derive(Clone)]
pub struct EventSink {
    queue: Rc<RefCell<Vec<QueuedEvent>>>,
    errors: Rc<RefCell<Vec<RuntimeError>>>,
    feedback: Rc<RefCell<HashMap<(NodeId, EventId), EventPayload>>>,
    scheduled: Rc<Cell<bool>>,
    dispatcher: DispatcherQueue,
}

impl EventSink {
    pub fn enqueue(&self, node: NodeId, event: EventId, revision: u32, payload: EventPayload) {
        let expected = self
            .feedback
            .borrow()
            .get(&(node, event))
            .is_some_and(|expected| expected == &payload);
        if expected {
            self.feedback.borrow_mut().remove(&(node, event));
            return;
        }
        self.queue.borrow_mut().push(QueuedEvent {
            node,
            event,
            revision,
            payload,
        });
        self.schedule();
    }

    pub fn error(&self, error: RuntimeError) {
        self.errors.borrow_mut().push(error);
        self.schedule();
    }

    pub fn wake(&self) {
        self.schedule();
    }

    fn schedule(&self) {
        if !self.scheduled.replace(true) {
            let scheduled = Rc::clone(&self.scheduled);
            let handler = DispatcherQueueHandler::new(move || {
                scheduled.set(false);
                dispatch_native_events();
            });
            if self
                .dispatcher
                .TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &handler)
                != Ok(true)
            {
                self.scheduled.set(false);
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
    fn apply(&mut self, commands: &[Command]) -> CommitReceipt {
        let mut structural_failure = false;
        let outcomes = commands
            .iter()
            .map(|command| {
                if structural_failure {
                    return CommandOutcome::Skipped;
                }
                match self.apply_one(command) {
                    Ok(()) => CommandOutcome::Applied,
                    Err(error) => {
                        structural_failure = command.structural();
                        CommandOutcome::Failed(error)
                    }
                }
            })
            .collect();
        CommitReceipt { outcomes }
    }

    fn reset(&mut self) {
        self.subscriptions.clear();
        for window in self.windows.values() {
            _ = window.Close();
        }
        self.windows.clear();
        self.handles.clear();
        self.virtuals.clear();
        self.application = None;
        self.event_errors.borrow_mut().clear();
        self.events.borrow_mut().clear();
        self.feedback.borrow_mut().clear();
        self.realizations.borrow_mut().clear();
        self.event_tick_scheduled.set(false);
    }

    fn drain_events(&mut self) -> Vec<QueuedEvent> {
        self.events.borrow_mut().drain(..).collect()
    }

    fn drain_event_errors(&mut self) -> Vec<RuntimeError> {
        self.event_errors.borrow_mut().drain(..).collect()
    }

    fn drain_realizations(&mut self) -> Vec<RealizationRequest> {
        self.realizations.borrow_mut().drain(..).collect()
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

pub fn schedule_native_retry() -> windows_core::Result<()> {
    let dispatcher = DispatcherQueue::GetForCurrentThread()?;
    let handler = DispatcherQueueHandler::new(dispatch_native_events);
    if dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Low, &handler)? {
        Ok(())
    } else {
        Err(windows_core::Error::new(
            E_FAIL,
            "dispatcher rejected reactor retry",
        ))
    }
}
