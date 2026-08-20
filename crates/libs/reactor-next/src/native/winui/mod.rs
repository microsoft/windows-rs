use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use super::*;
use crate::{Orientation as ReactorOrientation, TextWrapping as ReactorTextWrapping};
use windows_core::Interface;

#[allow(
    clippy::upper_case_acronyms,
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
mod bindings;
pub use bindings::*;

enum Handle {
    Button(bindings::Button),
    StackPanel(bindings::StackPanel),
    TextBlock(bindings::TextBlock),
}

impl Handle {
    fn ui_element(&self) -> windows_core::Result<UIElement> {
        match self {
            Self::Button(value) => value.cast(),
            Self::StackPanel(value) => value.cast(),
            Self::TextBlock(value) => value.cast(),
        }
    }

    fn dependency_object(&self) -> windows_core::Result<IDependencyObject> {
        match self {
            Self::Button(value) => value.cast(),
            Self::StackPanel(value) => value.cast(),
            Self::TextBlock(value) => value.cast(),
        }
    }
}

#[derive(Default)]
pub struct WinUiRuntime {
    handles: HashMap<NodeId, Handle>,
    events: Rc<RefCell<Vec<QueuedEvent>>>,
    event_tick_scheduled: Rc<Cell<bool>>,
    subscriptions: HashMap<(NodeId, EventId), windows_core::EventRevoker>,
}

impl WinUiRuntime {
    pub fn ui_element(&self, node: NodeId) -> windows_core::Result<UIElement> {
        self.handles
            .get(&node)
            .ok_or_else(|| windows_core::Error::from_hresult(E_FAIL))?
            .ui_element()
    }

    fn apply_one(&mut self, command: &Command) -> Result<(), RuntimeError> {
        match command {
            Command::Create { node, kind } => {
                if self.handles.contains_key(node) {
                    return Err(RuntimeError::DuplicateNode(*node));
                }
                let handle = match kind {
                    MountedKind::Button => {
                        Handle::Button(bindings::Button::new().map_err(native_error)?)
                    }
                    MountedKind::StackPanel => {
                        Handle::StackPanel(bindings::StackPanel::new().map_err(native_error)?)
                    }
                    MountedKind::TextBlock => {
                        Handle::TextBlock(bindings::TextBlock::new().map_err(native_error)?)
                    }
                    _ => return Err(RuntimeError::UnsupportedKind),
                };
                self.handles.insert(*node, handle);
            }
            Command::Destroy { node } => {
                if !self.handles.contains_key(node) {
                    return Err(RuntimeError::MissingNode(*node));
                }
                self.subscriptions
                    .retain(|(subscription_node, _), _| subscription_node != node);
                drop(self.handles.remove(node));
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
                match (handle, property, value) {
                    (
                        Handle::Button(button),
                        PropertyId::ButtonIsEnabled,
                        PropertyValue::Bool(value),
                    ) => button
                        .cast::<IControl>()
                        .map_err(native_error)?
                        .SetIsEnabled(*value)
                        .map_err(native_error)?,
                    (
                        Handle::StackPanel(panel),
                        PropertyId::StackPanelOrientation,
                        PropertyValue::Orientation(value),
                    ) => panel
                        .SetOrientation(match value {
                            ReactorOrientation::Horizontal => bindings::Orientation::Horizontal,
                            ReactorOrientation::Vertical => bindings::Orientation::Vertical,
                        })
                        .map_err(native_error)?,
                    (
                        Handle::StackPanel(panel),
                        PropertyId::StackPanelSpacing,
                        PropertyValue::F64(value),
                    ) => panel.SetSpacing(*value).map_err(native_error)?,
                    (
                        Handle::TextBlock(text),
                        PropertyId::TextBlockText,
                        PropertyValue::Str(value),
                    ) => text.SetText(value).map_err(native_error)?,
                    (
                        Handle::TextBlock(text),
                        PropertyId::TextBlockTextWrapping,
                        PropertyValue::TextWrapping(value),
                    ) => text
                        .SetTextWrapping(match value {
                            ReactorTextWrapping::NoWrap => bindings::TextWrapping::NoWrap,
                            ReactorTextWrapping::Wrap => bindings::TextWrapping::Wrap,
                            ReactorTextWrapping::WrapWholeWords => {
                                bindings::TextWrapping::WrapWholeWords
                            }
                        })
                        .map_err(native_error)?,
                    _ => return Err(RuntimeError::UnsupportedKind),
                }
            }
            Command::ClearProperty { node, property } => {
                let handle = self
                    .handles
                    .get(node)
                    .ok_or(RuntimeError::MissingNode(*node))?;
                match (handle, property) {
                    (Handle::TextBlock(_), PropertyId::TextBlockText) => {
                        handle
                            .dependency_object()
                            .map_err(native_error)?
                            .ClearValue(&bindings::TextBlock::TextProperty().map_err(native_error)?)
                            .map_err(native_error)?;
                    }
                    (Handle::TextBlock(_), PropertyId::TextBlockTextWrapping) => handle
                        .dependency_object()
                        .map_err(native_error)?
                        .ClearValue(
                            &bindings::TextBlock::TextWrappingProperty().map_err(native_error)?,
                        )
                        .map_err(native_error)?,
                    (Handle::Button(_), PropertyId::ButtonIsEnabled) => handle
                        .dependency_object()
                        .map_err(native_error)?
                        .ClearValue(&Control::IsEnabledProperty().map_err(native_error)?)
                        .map_err(native_error)?,
                    (Handle::StackPanel(_), PropertyId::StackPanelOrientation) => handle
                        .dependency_object()
                        .map_err(native_error)?
                        .ClearValue(
                            &bindings::StackPanel::OrientationProperty().map_err(native_error)?,
                        )
                        .map_err(native_error)?,
                    (Handle::StackPanel(_), PropertyId::StackPanelSpacing) => handle
                        .dependency_object()
                        .map_err(native_error)?
                        .ClearValue(&bindings::StackPanel::SpacingProperty().map_err(native_error)?)
                        .map_err(native_error)?,
                    _ => return Err(RuntimeError::UnsupportedKind),
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
                let queue = Rc::clone(&self.events);
                let scheduled = Rc::clone(&self.event_tick_scheduled);
                let dispatcher = DispatcherQueue::GetForCurrentThread().map_err(native_error)?;
                let revoker = match (handle, event) {
                    (Handle::Button(button), EventId::ButtonClick) => button
                        .cast::<ButtonBase>()
                        .map_err(native_error)?
                        .Click({
                            let node = *node;
                            let revision = *revision;
                            move |_, _| {
                                queue.borrow_mut().push(QueuedEvent {
                                    node,
                                    event: EventId::ButtonClick,
                                    revision,
                                    payload: EventPayload::Unit,
                                });
                                if !scheduled.replace(true) {
                                    let scheduled = Rc::clone(&scheduled);
                                    let handler = DispatcherQueueHandler::new(move || {
                                        scheduled.set(false);
                                        dispatch_native_events();
                                    });
                                    assert!(
                                        dispatcher
                                            .TryEnqueueWithPriority(
                                                DispatcherQueuePriority::Normal,
                                                &handler,
                                            )
                                            .unwrap(),
                                        "live WinUI dispatcher rejected an event tick"
                                    );
                                }
                            }
                        })
                        .map_err(native_error)?,
                    _ => return Err(RuntimeError::UnsupportedKind),
                };
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

    fn insert_child(
        &self,
        parent: NodeId,
        child: NodeId,
        index: usize,
    ) -> Result<(), RuntimeError> {
        let parent = self
            .handles
            .get(&parent)
            .ok_or(RuntimeError::MissingNode(parent))?;
        let child = self
            .handles
            .get(&child)
            .ok_or(RuntimeError::MissingNode(child))?
            .ui_element()
            .map_err(native_error)?;
        match parent {
            Handle::Button(button) if index == 0 => button
                .cast::<IContentControl>()
                .map_err(native_error)?
                .SetContent(&child)
                .map_err(native_error),
            Handle::StackPanel(panel) => panel_children(panel)?
                .InsertAt(index32(index)?, &child)
                .map_err(native_error),
            _ => Err(RuntimeError::IndexOutOfBounds),
        }
    }

    fn remove_child(&self, parent: NodeId, child: NodeId) -> Result<(), RuntimeError> {
        let child_id = child;
        let parent = self
            .handles
            .get(&parent)
            .ok_or(RuntimeError::MissingNode(parent))?;
        let child = self
            .handles
            .get(&child)
            .ok_or(RuntimeError::MissingNode(child))?
            .ui_element()
            .map_err(native_error)?;
        match parent {
            Handle::Button(button) => button
                .cast::<IContentControl>()
                .map_err(native_error)?
                .SetContent(None::<&windows_core::IInspectable>)
                .map_err(native_error),
            Handle::StackPanel(panel) => {
                let children = panel_children(panel)?;
                let index = child_index(&children, child_id, &child)?;
                children.RemoveAt(index).map_err(native_error)
            }
            _ => Err(RuntimeError::UnsupportedKind),
        }
    }

    fn move_child(&self, parent: NodeId, child: NodeId, index: usize) -> Result<(), RuntimeError> {
        let child_id = child;
        let parent = self
            .handles
            .get(&parent)
            .ok_or(RuntimeError::MissingNode(parent))?;
        let child = self
            .handles
            .get(&child)
            .ok_or(RuntimeError::MissingNode(child))?
            .ui_element()
            .map_err(native_error)?;
        match parent {
            Handle::Button(_) if index == 0 => Ok(()),
            Handle::StackPanel(panel) => {
                let children = panel_children(panel)?;
                let from = child_index(&children, child_id, &child)?;
                children.RemoveAt(from).map_err(native_error)?;
                children
                    .InsertAt(index32(index)?, &child)
                    .map_err(native_error)
            }
            _ => Err(RuntimeError::IndexOutOfBounds),
        }
    }
}

fn panel_children(panel: &bindings::StackPanel) -> Result<UIElementCollection, RuntimeError> {
    panel
        .cast::<IPanel>()
        .map_err(native_error)?
        .Children()
        .map_err(native_error)
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
        self.handles.clear();
        self.events.borrow_mut().clear();
        self.event_tick_scheduled.set(false);
    }

    fn drain_events(&mut self) -> Vec<QueuedEvent> {
        self.events.borrow_mut().drain(..).collect()
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
