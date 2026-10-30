use super::*;

impl WinUiAdapter {
    pub(super) fn dispatch<T: EventArg>(
        event: &Rc<RefCell<NativeValueEvent<T>>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: T::Native,
    ) {
        let queued = event.borrow().handler(value).map(|handler| QueuedEvent {
            object,
            event: event_id,
            handler,
        });
        event_queue.queue(observation, queued);
    }

    pub(super) fn drag_kind(
        args: Ref<native::DragEventArgs>,
        policy: &Rc<RefCell<Option<Rc<DragDropPolicy>>>>,
    ) -> Result<DragKind, WinUiError> {
        let args = args.unwrap();
        let data = args.DataView()?;
        let kind = if data.Contains("Shell IDList Array")? {
            DragKind::StorageItems
        } else if data.Contains("Text")? {
            DragKind::Text
        } else {
            DragKind::Unsupported
        };
        let policy = policy.borrow();
        let action = policy.as_ref().and_then(|policy| policy.action(kind));
        args.SetAcceptedOperation(action.map_or(native::DataPackageOperation::None, |action| {
            Self::native_drag_operation(action.operation)
        }))?;
        let ui = args.DragUIOverride()?;
        if let Some(caption) = action.and_then(|action| action.caption.as_deref()) {
            ui.SetCaption(caption)?;
            ui.SetIsCaptionVisible(true)?;
        } else {
            ui.SetIsCaptionVisible(false)?;
        }
        Ok(action.map_or(DragKind::Unsupported, |_| kind))
    }

    pub(super) fn dispatch_dropped_data(
        event: &Rc<RefCell<NativeDroppedDataEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        args: Ref<native::DragEventArgs>,
        policy: &Rc<RefCell<Option<Rc<DragDropPolicy>>>>,
    ) -> Result<(), WinUiError> {
        let args = args.unwrap();
        let deferral = args.GetDeferral()?;
        let data = args.DataView()?;
        let kind = if data.Contains("Shell IDList Array")? {
            DragKind::StorageItems
        } else if data.Contains("Text")? {
            DragKind::Text
        } else {
            DragKind::Unsupported
        };
        let action = policy
            .borrow()
            .as_ref()
            .and_then(|policy| policy.action(kind))
            .cloned();
        let Some(action) = action else {
            args.SetAcceptedOperation(native::DataPackageOperation::None)?;
            deferral.Complete()?;
            Self::dispatch(
                event,
                event_queue,
                object,
                event_id,
                observation,
                DroppedData::Unsupported,
            );
            return Ok(());
        };
        args.SetAcceptedOperation(Self::native_drag_operation(action.operation))?;
        let subscription = event.borrow().subscription.clone();
        let result = Arc::new(Mutex::new(None::<windows_core::Result<DroppedData>>));
        let result_for_handler = Arc::clone(&result);
        let event_queue = Rc::clone(event_queue);
        let deferral_for_handler = deferral.clone();
        let callback = UiCallback::once(move || {
            // The drag source owns the data and can fail or disconnect before it is read.
            let value = result_for_handler
                .lock()
                .unwrap()
                .take()
                .unwrap()
                .unwrap_or(DroppedData::Unsupported);
            if let Some(subscription) = subscription {
                event_queue.queue(
                    observation,
                    Some(QueuedEvent {
                        object,
                        event: event_id,
                        handler: subscription.handler(value),
                    }),
                );
            }
            _ = deferral_for_handler.Complete();
        })?;
        match kind {
            DragKind::StorageItems => {
                let operation = data.GetStorageItemsAsync()?;
                let completion = callback.clone();
                let completion_deferral = deferral.clone();
                if let Err(error) = operation.when(move |items| {
                    let value = items.and_then(|items| {
                        let mut dropped = Vec::with_capacity(items.Size()? as usize);
                        for index in 0..items.Size()? {
                            let item = items.GetAt(index)?;
                            dropped.push(DroppedStorageItem {
                                name: item.Name()?,
                                path: item.Path()?,
                            });
                        }
                        Ok(DroppedData::StorageItems(dropped))
                    });
                    *result.lock().unwrap() = Some(value);
                    if !matches!(completion.try_wake(), Ok(true)) {
                        _ = completion_deferral.Complete();
                    }
                }) {
                    callback.cancel();
                    deferral.Complete()?;
                    return Err(error.into());
                }
            }
            DragKind::Text => {
                let operation = data.GetTextAsync()?;
                let completion = callback.clone();
                let completion_deferral = deferral.clone();
                if let Err(error) = operation.when(move |value| {
                    *result.lock().unwrap() =
                        Some(value.map(|value| DroppedData::Text(value.to_string_lossy())));
                    if !matches!(completion.try_wake(), Ok(true)) {
                        _ = completion_deferral.Complete();
                    }
                }) {
                    callback.cancel();
                    deferral.Complete()?;
                    return Err(error.into());
                }
            }
            DragKind::Unsupported => unreachable!(),
        }
        Ok(())
    }

    fn native_drag_operation(value: DragDropOperation) -> native::DataPackageOperation {
        match value {
            DragDropOperation::Copy => native::DataPackageOperation::Copy,
            DragDropOperation::Move => native::DataPackageOperation::Move,
            DragDropOperation::Link => native::DataPackageOperation::Link,
        }
    }

    pub(super) fn handle_selection_changed(
        event: &Rc<RefCell<NativeSelectionEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        selected: windows_core::Result<IInspectable>,
        payload_property: PropertyId,
    ) {
        let selected = match selected {
            Ok(selected) => Some(selected),
            Err(error) if error.code().is_ok() => None,
            Err(error) => panic!("{error:?}"),
        };
        let selected_object = selected
            .as_ref()
            .and_then(|selected| event_queue.item_object(object, selected));
        let observation = Observation::SetSelection {
            object,
            selected: match (&selected, selected_object) {
                (_, Some(item)) => ObservedSelection::Item(item),
                (Some(_), None) => ObservedSelection::Untracked,
                (None, None) => ObservedSelection::Cleared,
            },
        };
        if !event_queue.reports(object, event_id) {
            return;
        }
        let value = selected.as_ref().and_then(|selected| {
            GeneratedHandle::selection_payload(payload_property, selected).unwrap()
        });
        let handler = event.borrow().handler(SelectionChange {
            item: selected_object,
            value,
        });
        event_queue.queue(
            Some(observation),
            handler.map(|handler| QueuedEvent {
                object,
                event: event_id,
                handler,
            }),
        );
    }

    pub(super) fn pointer_event_info(
        object: ObjectId,
        element: &native::UIElement,
        args: Ref<native::PointerRoutedEventArgs>,
        capture_on_press: Option<bool>,
        release_capture: bool,
        focus_on_release: bool,
        pending_focus_states: &Rc<RefCell<HashMap<ObjectId, ElementFocusState>>>,
    ) -> Result<PointerEventInfo, WinUiError> {
        let args = args.unwrap();
        let local = args.GetCurrentPoint(element)?;
        let local_position = local.Position()?;
        let window = args.GetCurrentPoint(None::<&native::UIElement>)?;
        let window_position = window.Position()?;
        let properties = local.Properties()?;
        let pointer = args.Pointer()?;
        let capture_succeeded = if capture_on_press == Some(true) {
            Some(element.CapturePointer(&pointer)?)
        } else {
            capture_on_press.map(|_| false)
        };
        let mut capture_index = 0;
        let is_captured = element
            .PointerCaptures()
            .ok()
            .and_then(|captures| captures.IndexOf(&pointer, &mut capture_index).ok())
            .unwrap_or(false);
        if release_capture {
            element.ReleasePointerCapture(&pointer)?;
        }

        if focus_on_release {
            let element = element.clone();
            let pending_focus_states = Rc::clone(pending_focus_states);
            let handler = native::DispatcherQueueHandler::new(move || {
                pending_focus_states
                    .borrow_mut()
                    .insert(object, ElementFocusState::Pointer);
                if !element.Focus(native::FocusState::Pointer).unwrap() {
                    pending_focus_states.borrow_mut().remove(&object);
                    return;
                }
                let cleanup_states = Rc::clone(&pending_focus_states);
                let cleanup = native::DispatcherQueueHandler::new(move || {
                    cleanup_states.borrow_mut().remove(&object);
                });
                // The dispatcher rejects work only once it is shutting down.
                if !native::DispatcherQueue::GetForCurrentThread()
                    .unwrap()
                    .TryEnqueueWithPriority(native::DispatcherQueuePriority::Low, &cleanup)
                    .unwrap()
                {
                    pending_focus_states.borrow_mut().remove(&object);
                }
            });
            // The focus request is dropped if the dispatcher is shutting down.
            native::DispatcherQueue::GetForCurrentThread()?
                .TryEnqueueWithPriority(native::DispatcherQueuePriority::Normal, &handler)?;
        }
        Ok(PointerEventInfo {
            x: f64::from(local_position.x),
            y: f64::from(local_position.y),
            window_x: f64::from(window_position.x),
            window_y: f64::from(window_position.y),
            pointer_id: local.PointerId()?,
            modifiers: Self::input_modifiers_from_virtual_keys(args.KeyModifiers()?),
            capture_succeeded,
            is_captured,
            is_left_button_pressed: properties.IsLeftButtonPressed()?,
            is_right_button_pressed: properties.IsRightButtonPressed()?,
            is_middle_button_pressed: properties.IsMiddleButtonPressed()?,
        })
    }

    fn input_modifiers() -> Result<InputModifiers, WinUiError> {
        let mut keys = [0u8; 256];
        if !unsafe { native::GetKeyboardState(keys.as_mut_ptr()) }.as_bool() {
            return Err(windows_core::Error::from_thread().into());
        }
        let mut modifiers = InputModifiers::NONE;
        if keys[0x10] & 0x80 != 0 {
            modifiers |= InputModifiers::SHIFT;
        }
        if keys[0x11] & 0x80 != 0 {
            modifiers |= InputModifiers::CONTROL;
        }
        if keys[0x12] & 0x80 != 0 {
            modifiers |= InputModifiers::ALT;
        }
        if keys[0x5b] & 0x80 != 0 || keys[0x5c] & 0x80 != 0 {
            modifiers |= InputModifiers::WINDOWS;
        }
        Ok(modifiers)
    }

    fn physical_key_status(value: native::CorePhysicalKeyStatus) -> PhysicalKeyStatus {
        PhysicalKeyStatus {
            repeat_count: value.repeat_count,
            scan_code: value.scan_code,
            is_extended: value.is_extended_key,
            is_menu_down: value.is_menu_key_down,
            was_down: value.was_key_down,
            is_released: value.is_key_released,
        }
    }

    pub(super) fn input_modifiers_from_virtual_keys(
        value: native::VirtualKeyModifiers,
    ) -> InputModifiers {
        let mut modifiers = InputModifiers::NONE;
        if value.contains(native::VirtualKeyModifiers::Shift) {
            modifiers |= InputModifiers::SHIFT;
        }
        if value.contains(native::VirtualKeyModifiers::Control) {
            modifiers |= InputModifiers::CONTROL;
        }
        if value.contains(native::VirtualKeyModifiers::Menu) {
            modifiers |= InputModifiers::ALT;
        }
        if value.contains(native::VirtualKeyModifiers::Windows) {
            modifiers |= InputModifiers::WINDOWS;
        }
        modifiers
    }

    pub(super) fn key_event_info(
        args: &native::KeyRoutedEventArgs,
    ) -> Result<KeyEventInfo, WinUiError> {
        Ok(KeyEventInfo {
            key: VirtualKey(args.Key()?.0 as u32),
            original_key: VirtualKey(args.OriginalKey()?.0 as u32),
            status: Self::physical_key_status(args.KeyStatus()?),
            modifiers: Self::input_modifiers()?,
        })
    }

    pub(super) fn character_event_info(
        args: &native::CharacterReceivedRoutedEventArgs,
    ) -> Result<CharacterEventInfo, WinUiError> {
        Ok(CharacterEventInfo {
            character: args.Character()?,
            status: Self::physical_key_status(args.KeyStatus()?),
            modifiers: Self::input_modifiers()?,
        })
    }

    pub(super) fn focus_event_info(
        object: ObjectId,
        element: &native::UIElement,
        args: Ref<native::RoutedEventArgs>,
        got_focus: bool,
        pending_focus_states: &Rc<RefCell<HashMap<ObjectId, ElementFocusState>>>,
    ) -> Result<FocusEventInfo, WinUiError> {
        let args = args.unwrap();
        let original = args.OriginalSource()?;
        let element_identity: &windows_core::IUnknown = element.into();
        let original_identity: &windows_core::IUnknown = (&original).into();
        let is_direct = element_identity == original_identity;
        let state = if got_focus {
            pending_focus_states.borrow_mut().remove(&object).unwrap_or(
                match element.FocusState()? {
                    native::FocusState::Pointer => ElementFocusState::Pointer,
                    native::FocusState::Keyboard => ElementFocusState::Keyboard,
                    native::FocusState::Programmatic => ElementFocusState::Programmatic,
                    _ => ElementFocusState::Unfocused,
                },
            )
        } else {
            ElementFocusState::Unfocused
        };
        Ok(FocusEventInfo { state, is_direct })
    }

    /// Records a native change Reactor caused through its own write, without a callback.
    pub(super) fn record(event_queue: &Rc<NativeEventQueue>, observation: Observation) {
        event_queue.push(NativeEvent::Record(observation));
    }

    pub(super) fn set_events(
        &mut self,
        object: ObjectId,
        set: &[Event],
        clear: &[EventId],
    ) -> Result<(), WinUiError> {
        let context = EventContext {
            event_queue: &self.event_queue,
            pending_focus_states: &self.pending_focus_states,
        };
        match self
            .handles
            .get_mut(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::Generated(value) => {
                return value
                    .set_events(object, set, clear, context)
                    .unwrap_or(Err(WinUiError::InvalidObject(object)));
            }
            Handle::TreeView(value) => {
                let mut native_event = value.item_invoked.borrow_mut();
                if clear.contains(&EventId::ItemInvoked) {
                    native_event.set(None);
                }
                for event in set {
                    match (event.id, &event.value) {
                        (EventId::ItemInvoked, EventValue::String(callback)) => {
                            native_event.set(Some(callback.clone()));
                        }
                        _ => return Err(WinUiError::InvalidEvent(object, event.id)),
                    }
                }
            }
            _ => return Err(WinUiError::InvalidObject(object)),
        }
        Ok(())
    }
}
