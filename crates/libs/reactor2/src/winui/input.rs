use super::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub(super) struct NativeInputState {
    pointer: Option<Box<PointerState>>,
    drop: Option<Box<DropState>>,
}

impl NativeInputState {
    pub(super) fn captures_pointer(&self) -> bool {
        self.pointer
            .as_deref()
            .is_some_and(|pointer| pointer.subscription.capture_on_press)
    }
}

struct PointerState {
    subscription: PointerSubscription,
    _pressed: Option<windows_core::EventRevoker>,
    _moved: Option<windows_core::EventRevoker>,
    _released: Option<windows_core::EventRevoker>,
    _entered: Option<windows_core::EventRevoker>,
    _exited: Option<windows_core::EventRevoker>,
    _capture_lost: Option<windows_core::EventRevoker>,
    _canceled: Option<windows_core::EventRevoker>,
    _tapped: Option<windows_core::EventRevoker>,
    _right_tapped: Option<windows_core::EventRevoker>,
}

#[cfg(test)]
pub(crate) struct PointerAttachmentProbe {
    pub subscription: PointerSubscription,
    pub pressed: bool,
    pub moved: bool,
    pub released: bool,
    pub entered: bool,
    pub exited: bool,
    pub capture_lost: bool,
    pub canceled: bool,
    pub tapped: bool,
    pub right_tapped: bool,
}

struct DropState {
    cancelled: Arc<AtomicBool>,
    _enter: windows_core::EventRevoker,
    _over: windows_core::EventRevoker,
    _drop: windows_core::EventRevoker,
}

impl Drop for DropState {
    fn drop(&mut self) {
        self.cancelled.store(true, Ordering::Release);
    }
}

impl WinUiRuntime {
    #[cfg(test)]
    pub(crate) fn pointer_attachment_probe(&self, id: NodeId) -> Option<PointerAttachmentProbe> {
        let pointer = self.node(id).ok()?.input.as_deref()?.pointer.as_deref()?;
        Some(PointerAttachmentProbe {
            subscription: pointer.subscription,
            pressed: pointer._pressed.is_some(),
            moved: pointer._moved.is_some(),
            released: pointer._released.is_some(),
            entered: pointer._entered.is_some(),
            exited: pointer._exited.is_some(),
            capture_lost: pointer._capture_lost.is_some(),
            canceled: pointer._canceled.is_some(),
            tapped: pointer._tapped.is_some(),
            right_tapped: pointer._right_tapped.is_some(),
        })
    }

    pub(super) fn apply_input_update(
        &mut self,
        id: NodeId,
        update: &InputUpdate,
    ) -> WindowsResult<()> {
        match update {
            InputUpdate::KeyboardAccelerators(accelerators) => {
                self.set_keyboard_accelerators(id, accelerators)
            }
            InputUpdate::Pointer(subscription) => self.set_pointer_subscription(id, *subscription),
            InputUpdate::Drop(target) => self.set_drop_target(id, *target),
        }
    }

    fn set_pointer_subscription(
        &mut self,
        id: NodeId,
        subscription: PointerSubscription,
    ) -> WindowsResult<()> {
        let element = self.node(id)?.handle.ui_element()?;
        let previous = self
            .node_mut(id)?
            .input
            .as_deref_mut()
            .and_then(|input| input.pointer.take());
        if previous
            .as_deref()
            .is_some_and(|pointer| pointer.subscription.capture_on_press)
        {
            element.ReleasePointerCaptures()?;
        }
        drop(previous);
        if subscription.is_empty() {
            self.compact_native_input(id)?;
            return Ok(());
        }

        let pressed = if subscription.events.contains(PointerEvents::PRESSED) {
            let element = element.clone();
            let events = Rc::clone(&self.events);
            let waker = Rc::clone(&self.waker);
            Some(element.clone().PointerPressed(move |_sender, args| {
                let capture_succeeded = if subscription.capture_on_press {
                    let args = args.as_ref().unwrap();
                    let pointer = args.Pointer().unwrap();
                    element.CapturePointer(&pointer).unwrap()
                } else {
                    false
                };
                queue_pointer_event(
                    &events,
                    &waker,
                    id,
                    PointerEventKind::Pressed,
                    pointer_event(&element, args, capture_succeeded),
                );
            })?)
        } else {
            None
        };
        let moved = if subscription.events.contains(PointerEvents::MOVED) {
            let element = element.clone();
            let events = Rc::clone(&self.events);
            let waker = Rc::clone(&self.waker);
            Some(element.clone().PointerMoved(move |_sender, args| {
                queue_pointer_event(
                    &events,
                    &waker,
                    id,
                    PointerEventKind::Moved,
                    pointer_event(&element, args, false),
                );
            })?)
        } else {
            None
        };
        let released = if subscription.events.contains(PointerEvents::RELEASED) {
            let element = element.clone();
            let events = Rc::clone(&self.events);
            let waker = Rc::clone(&self.waker);
            Some(element.clone().PointerReleased(move |_sender, args| {
                let pointer = if subscription.capture_on_press {
                    Some(args.as_ref().unwrap().Pointer().unwrap())
                } else {
                    None
                };
                queue_pointer_event(
                    &events,
                    &waker,
                    id,
                    PointerEventKind::Released,
                    pointer_event(&element, args, false),
                );
                if let Some(pointer) = pointer {
                    element.ReleasePointerCapture(&pointer).unwrap();
                }
            })?)
        } else {
            None
        };
        let entered = if subscription.events.contains(PointerEvents::ENTERED) {
            let element = element.clone();
            let events = Rc::clone(&self.events);
            let waker = Rc::clone(&self.waker);
            Some(element.clone().PointerEntered(move |_sender, args| {
                queue_pointer_event(
                    &events,
                    &waker,
                    id,
                    PointerEventKind::Entered,
                    pointer_event(&element, args, false),
                );
            })?)
        } else {
            None
        };
        let exited = if subscription.events.contains(PointerEvents::EXITED) {
            let element = element.clone();
            let events = Rc::clone(&self.events);
            let waker = Rc::clone(&self.waker);
            Some(element.clone().PointerExited(move |_sender, args| {
                queue_pointer_event(
                    &events,
                    &waker,
                    id,
                    PointerEventKind::Exited,
                    pointer_event(&element, args, false),
                );
            })?)
        } else {
            None
        };
        let capture_lost = if subscription.events.contains(PointerEvents::CAPTURE_LOST) {
            let element = element.clone();
            let events = Rc::clone(&self.events);
            let waker = Rc::clone(&self.waker);
            Some(element.clone().PointerCaptureLost(move |_sender, args| {
                queue_pointer_event(
                    &events,
                    &waker,
                    id,
                    PointerEventKind::CaptureLost,
                    pointer_event(&element, args, false),
                );
            })?)
        } else {
            None
        };
        let canceled = if subscription.events.contains(PointerEvents::CANCELED) {
            let callback_element = element.clone();
            let events = Rc::clone(&self.events);
            let waker = Rc::clone(&self.waker);
            Some(element.PointerCanceled(move |_sender, args| {
                let pointer = if subscription.capture_on_press {
                    Some(args.as_ref().unwrap().Pointer().unwrap())
                } else {
                    None
                };
                queue_pointer_event(
                    &events,
                    &waker,
                    id,
                    PointerEventKind::Canceled,
                    pointer_event(&callback_element, args, false),
                );
                if let Some(pointer) = pointer {
                    callback_element.ReleasePointerCapture(&pointer).unwrap();
                }
            })?)
        } else {
            None
        };
        let tapped = if subscription.events.contains(PointerEvents::TAPPED) {
            let events = Rc::clone(&self.events);
            let waker = Rc::clone(&self.waker);
            Some(element.Tapped(move |_sender, _args| {
                events
                    .borrow_mut()
                    .push_back(NativeEvent::Tapped { target: id });
                if let Some(wake) = waker.borrow().as_ref() {
                    wake();
                }
            })?)
        } else {
            None
        };
        let right_tapped = if subscription.events.contains(PointerEvents::RIGHT_TAPPED) {
            let events = Rc::clone(&self.events);
            let waker = Rc::clone(&self.waker);
            Some(element.RightTapped(move |_sender, _args| {
                events
                    .borrow_mut()
                    .push_back(NativeEvent::RightTapped { target: id });
                if let Some(wake) = waker.borrow().as_ref() {
                    wake();
                }
            })?)
        } else {
            None
        };
        self.node_mut(id)?
            .input
            .get_or_insert_with(|| Box::new(NativeInputState::default()))
            .pointer = Some(Box::new(PointerState {
            subscription,
            _pressed: pressed,
            _moved: moved,
            _released: released,
            _entered: entered,
            _exited: exited,
            _capture_lost: capture_lost,
            _canceled: canceled,
            _tapped: tapped,
            _right_tapped: right_tapped,
        }));
        Ok(())
    }

    fn set_drop_target(&mut self, id: NodeId, target: Option<DropTarget>) -> WindowsResult<()> {
        let element = self.node(id)?.handle.ui_element()?;
        let previous = self
            .node_mut(id)?
            .input
            .as_deref_mut()
            .and_then(|input| input.drop.take());
        drop(previous);
        element.SetAllowDrop(target.is_some())?;
        let Some(target) = target else {
            self.compact_native_input(id)?;
            return Ok(());
        };

        let enter = element.DragEnter(move |_sender, args| {
            accept_drop(args.as_ref(), target).unwrap();
        })?;
        let over = element.DragOver(move |_sender, args| {
            accept_drop(args.as_ref(), target).unwrap();
        })?;

        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let dispatcher = self.dispatcher.clone();
        let cancelled = Arc::new(AtomicBool::new(false));
        let callback_cancelled = Arc::clone(&cancelled);
        let dropped = element.Drop(move |_sender, args| {
            if callback_cancelled.load(Ordering::Acquire) {
                return;
            }
            let args = args.as_ref().unwrap();
            let data = match args.DataView() {
                Ok(data) => data,
                Err(error) => {
                    queue_drop_event(&events, &waker, id, Err(error));
                    return;
                }
            };
            let formats = match read_drop_formats(&data) {
                Ok(formats) => formats,
                Err(error) => {
                    queue_drop_event(&events, &waker, id, Err(error));
                    return;
                }
            };
            if !formats.intersects(target.formats()) {
                if let Err(error) = args.SetAcceptedOperation(bindings::DataPackageOperation::None)
                {
                    queue_drop_event(&events, &waker, id, Err(error));
                }
                return;
            }
            if let Err(error) = args.SetAcceptedOperation(drop_operation(target.operation())) {
                queue_drop_event(&events, &waker, id, Err(error));
                return;
            }
            let deferral = match args.GetDeferral() {
                Ok(deferral) => deferral,
                Err(error) => {
                    queue_drop_event(&events, &waker, id, Err(error));
                    return;
                }
            };
            let data = match windows_core::AgileReference::new(&data) {
                Ok(data) => data,
                Err(error) => {
                    let mut error = error;
                    if let Err(complete_error) = deferral.Complete() {
                        error = complete_error;
                    }
                    queue_drop_event(&events, &waker, id, Err(error));
                    return;
                }
            };
            let result = Arc::new(Mutex::new(None));
            let callback_result = Arc::clone(&result);
            let completion_cancelled = Arc::clone(&callback_cancelled);
            let completion_events = Rc::clone(&events);
            let completion_waker = Rc::clone(&waker);
            let completion_deferral = deferral.clone();
            let completion = bindings::DispatcherQueueHandler::new(move || {
                let mut result = callback_result.lock().unwrap().take();
                if let Err(error) = completion_deferral.Complete() {
                    result = Some(Err(error));
                }
                if !completion_cancelled.load(Ordering::Acquire)
                    && let Some(result) = result
                {
                    queue_drop_event(&completion_events, &completion_waker, id, result);
                }
            });
            let completion = match windows_core::AgileReference::new(&completion) {
                Ok(completion) => completion,
                Err(error) => {
                    let mut error = error;
                    if let Err(complete_error) = deferral.Complete() {
                        error = complete_error;
                    }
                    queue_drop_event(&events, &waker, id, Err(error));
                    return;
                }
            };
            let load_dispatcher = dispatcher.clone();
            windows_threading::submit(move || {
                let loaded = data
                    .resolve()
                    .and_then(|data| load_drop_event(&data, formats, target.formats()));
                *result.lock().unwrap() = Some(loaded);
                let completion = completion.resolve().unwrap();
                assert!(
                    load_dispatcher
                        .TryEnqueueWithPriority(
                            bindings::DispatcherQueuePriority::Normal,
                            &completion,
                        )
                        .unwrap(),
                    "failed to enqueue drop completion"
                );
            });
        })?;

        self.node_mut(id)?
            .input
            .get_or_insert_with(|| Box::new(NativeInputState::default()))
            .drop = Some(Box::new(DropState {
            cancelled,
            _enter: enter,
            _over: over,
            _drop: dropped,
        }));
        Ok(())
    }

    fn compact_native_input(&mut self, id: NodeId) -> WindowsResult<()> {
        let node = self.node_mut(id)?;
        if node
            .input
            .as_deref()
            .is_some_and(|input| input.pointer.is_none() && input.drop.is_none())
        {
            node.input = None;
        }
        Ok(())
    }

    fn set_keyboard_accelerators(
        &mut self,
        id: NodeId,
        accelerators: &[KeyboardAcceleratorSpec],
    ) -> WindowsResult<()> {
        let element: bindings::IUIElement = self.node(id)?.handle.ui_element()?.cast()?;
        let values = element.KeyboardAccelerators()?;
        values.Clear()?;
        element.SetKeyboardAcceleratorPlacementMode(
            bindings::KeyboardAcceleratorPlacementMode::Hidden,
        )?;
        for accelerator in accelerators {
            let value = bindings::KeyboardAccelerator::new()?;
            value.SetKey(bindings::VirtualKey(accelerator.key.code()))?;
            value.SetModifiers(bindings::VirtualKeyModifiers(accelerator.modifiers.bits()))?;
            let events = Rc::clone(&self.events);
            let waker = Rc::clone(&self.waker);
            let accelerator = *accelerator;
            let revoker = value.Invoked(move |_sender, args| {
                let args = args.as_ref().unwrap();
                args.SetHandled(true).unwrap();
                events
                    .borrow_mut()
                    .push_back(NativeEvent::KeyboardAcceleratorInvoked {
                        target: id,
                        accelerator,
                    });
                if let Some(wake) = waker.borrow().as_ref() {
                    wake();
                }
            })?;
            _ = revoker.into_token();
            values.Append(&value)?;
        }
        Ok(())
    }
}

fn pointer_event(
    element: &bindings::UIElement,
    args: windows_core::Ref<bindings::PointerRoutedEventArgs>,
    capture_succeeded: bool,
) -> WindowsResult<PointerEvent> {
    let args = args
        .as_ref()
        .unwrap_or_else(|| panic!("pointer event supplied no event arguments"));
    let point = args.GetCurrentPoint(element)?;
    let position = point.Position()?;
    let properties = point.Properties()?;
    let window_position = args
        .GetCurrentPoint(None::<&bindings::UIElement>)?
        .Position()?;
    Ok(PointerEvent {
        pointer_id: point.PointerId()?,
        x: position.x,
        y: position.y,
        window_x: window_position.x,
        window_y: window_position.y,
        capture_succeeded,
        is_left_button_pressed: properties.IsLeftButtonPressed()?,
        is_right_button_pressed: properties.IsRightButtonPressed()?,
        is_middle_button_pressed: properties.IsMiddleButtonPressed()?,
    })
}

fn queue_pointer_event(
    events: &Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: &Rc<RefCell<Option<Rc<dyn Fn()>>>>,
    target: NodeId,
    kind: PointerEventKind,
    event: WindowsResult<PointerEvent>,
) {
    events.borrow_mut().push_back(NativeEvent::Pointer {
        target,
        kind,
        event: event.unwrap(),
    });
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}

fn accept_drop(args: Option<&bindings::DragEventArgs>, target: DropTarget) -> WindowsResult<()> {
    let args = args.unwrap_or_else(|| panic!("drag event supplied no event arguments"));
    let data = args.DataView()?;
    let formats = read_drop_formats(&data).unwrap_or_else(|error| panic!("{error}"));
    let operation = if formats.intersects(target.formats()) {
        drop_operation(target.operation())
    } else {
        bindings::DataPackageOperation::None
    };
    args.SetAcceptedOperation(operation)
}

fn drop_operation(operation: DropOperation) -> bindings::DataPackageOperation {
    match operation {
        DropOperation::Copy => bindings::DataPackageOperation::Copy,
        DropOperation::Move => bindings::DataPackageOperation::Move,
        DropOperation::Link => bindings::DataPackageOperation::Link,
    }
}

fn read_drop_formats(data: &bindings::DataPackageView) -> WindowsResult<DropFormats> {
    const TEXT: &str = "Text";
    const STORAGE_ITEMS: &str = "Shell IDList Array";

    let mut result = DropFormats::default();
    let formats = data.AvailableFormats()?;
    for format in &formats {
        match format.to_string_lossy().as_str() {
            TEXT => result |= DropFormats::TEXT,
            STORAGE_ITEMS => result |= DropFormats::STORAGE_ITEMS,
            _ => {}
        }
    }
    Ok(result)
}

fn load_drop_event(
    data: &bindings::DataPackageView,
    available: DropFormats,
    requested: DropFormats,
) -> WindowsResult<DropEvent> {
    let text = if available.contains(DropFormats::TEXT) && requested.contains(DropFormats::TEXT) {
        Some(data.GetTextAsync()?.join()?.to_string_lossy())
    } else {
        None
    };
    let storage_items = if available.contains(DropFormats::STORAGE_ITEMS)
        && requested.contains(DropFormats::STORAGE_ITEMS)
    {
        let items = data.GetStorageItemsAsync()?.join()?;
        let mut result = Vec::with_capacity(items.Size()? as usize);
        for index in 0..items.Size()? {
            let item = items.GetAt(index)?;
            let attributes = item.Attributes()?;
            result.push(DroppedItem {
                path: item.Path()?,
                name: item.Name()?,
                is_folder: attributes.0 & bindings::FileAttributes::Directory.0 != 0,
            });
        }
        result.into_boxed_slice()
    } else {
        Box::default()
    };
    Ok(DropEvent {
        formats: available,
        text,
        storage_items,
    })
}

fn queue_drop_event(
    events: &Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: &Rc<RefCell<Option<Rc<dyn Fn()>>>>,
    target: NodeId,
    result: WindowsResult<DropEvent>,
) {
    events.borrow_mut().push_back(NativeEvent::Drop {
        target,
        result: Box::new(result),
    });
    if let Some(wake) = waker.borrow().as_ref() {
        wake();
    }
}
