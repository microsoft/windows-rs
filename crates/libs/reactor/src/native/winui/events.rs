impl WinUiAdapter {
    fn dispatch_text_changed(
        event: &Rc<RefCell<NativeTextEvent>>,
        observed_text: &Rc<RefCell<Rc<str>>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        text: Rc<str>,
    ) {
        {
            let mut observed = observed_text.borrow_mut();
            if *observed == text {
                return;
            }
            *observed = Rc::clone(&text);
        }
        let observation = Observation::SetProperty {
            object,
            property: Property {
                id: PropertyId::Text,
                value: PropertyValue::String(Rc::clone(&text)),
            },
        };
        if !event_queue.observe(object, EventId::TextChanged, observation.clone()) {
            return;
        }
        let event = event.borrow();
        event_queue.queue(
            Some(observation),
            event.callback.is_some().then(|| QueuedEvent {
                object,
                event: EventId::TextChanged,
                revision: event.revision,
                payload: EventPayload::String(text),
            }),
        );
        Self::schedule_event_wake(event_queue);
    }

    fn dispatch_unit(
        event: &Rc<RefCell<NativeUnitEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::Unit,
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_bool(
        event: &Rc<RefCell<NativeBoolEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: bool,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::Bool(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_content_dialog_result(
        event: &Rc<RefCell<NativeContentDialogResultEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: ContentDialogResult,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::ContentDialogResult(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_f64(
        event: &Rc<RefCell<NativeF64Event>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: f64,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::F64(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_color(
        event: &Rc<RefCell<NativeColorEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: Color,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::Color(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_optional_bool(
        event: &Rc<RefCell<NativeOptionalBoolEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: Option<bool>,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::OptionalBool(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_optional_f64(
        event: &Rc<RefCell<NativeOptionalF64Event>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: Option<f64>,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::OptionalF64(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_optional_date_time(
        event: &Rc<RefCell<NativeOptionalDateTimeEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: Option<DateTime>,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::OptionalDateTime(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_optional_time_span(
        event: &Rc<RefCell<NativeOptionalTimeSpanEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: Option<TimeSpan>,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::OptionalTimeSpan(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_navigation_view_display_mode(
        event: &Rc<RefCell<NativeNavigationViewDisplayModeEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: NavigationViewDisplayMode,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::NavigationViewDisplayMode(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_selection_index(
        event: &Rc<RefCell<NativeSelectionIndexEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: Option<usize>,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::SelectionIndex(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_string(
        event: &Rc<RefCell<NativeTextEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: Rc<str>,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::String(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_string_list(
        event: &Rc<RefCell<NativeStringListEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: Vec<String>,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::StringList(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn item_tags(source: &native::IItemsControl) -> Result<Vec<String>, WinUiError> {
        let items = source.Items()?;
        let mut tags = Vec::with_capacity(items.Size()? as usize);
        for index in 0..items.Size()? {
            let tag = items
                .GetAt(index)?
                .cast::<native::IFrameworkElement>()?
                .Tag()?
                .cast::<windows_reference::IReference<HSTRING>>()?
                .Value()?;
            tags.push(tag.to_string_lossy());
        }
        Ok(tags)
    }

    fn tab_item_tags(source: &native::ITabView) -> Result<Vec<String>, WinUiError> {
        let items = source.TabItems()?;
        let mut tags = Vec::with_capacity(items.Size()? as usize);
        for index in 0..items.Size()? {
            let tag = items
                .GetAt(index)?
                .cast::<native::IFrameworkElement>()?
                .Tag()?
                .cast::<windows_reference::IReference<HSTRING>>()?
                .Value()?;
            tags.push(tag.to_string_lossy());
        }
        Ok(tags)
    }

    fn dispatch_pointer_event_info(
        event: &Rc<RefCell<NativePointerEventInfoEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: PointerEventInfo,
    ) {
        if Self::queue_pointer_event_info(event, event_queue, object, event_id, observation, value)
        {
            Self::schedule_event_wake(event_queue);
        }
    }

    fn dispatch_drag_kind(
        event: &Rc<RefCell<NativeDragKindEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: DragKind,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::DragKind(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn queue_dropped_data(
        event: &Rc<RefCell<NativeDroppedDataEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: DroppedData,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::DroppedData(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
        }
    }

    fn drag_kind(
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

    fn dispatch_dropped_data(
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
            Self::queue_dropped_data(
                event,
                event_queue,
                object,
                event_id,
                observation,
                DroppedData::Unsupported,
            );
            Self::schedule_event_wake(event_queue);
            return Ok(());
        };
        args.SetAcceptedOperation(Self::native_drag_operation(action.operation))?;
        let revision = event.borrow().revision;
        let subscribed = event.borrow().callback.is_some();
        let result = Arc::new(Mutex::new(None));
        let result_for_handler = Arc::clone(&result);
        let event_queue = Rc::clone(event_queue);
        let deferral_for_handler = deferral.clone();
        let callback = AppContext::ui_callback_once(move || {
            let result = result_for_handler.lock().unwrap().take().unwrap();
            match result {
                Ok(value) if subscribed => {
                    event_queue.queue(
                        observation,
                        Some(QueuedEvent {
                            object,
                            event: event_id,
                            revision,
                            payload: EventPayload::DroppedData(value),
                        }),
                    );
                    Self::schedule_event_wake(&event_queue);
                }
                Ok(_) => {}
                Err(error) => report_error(error),
            }
            if let Err(error) = deferral_for_handler.Complete() {
                report_error(error);
            }
            Ok(())
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
                    if completion.invoke().is_err() {
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
                    if completion.invoke().is_err() {
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

    fn queue_pointer_event_info(
        event: &Rc<RefCell<NativePointerEventInfoEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: PointerEventInfo,
    ) -> bool {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::PointerEventInfo(value),
        });
        let dispatch = observation.is_some() || queued.is_some();
        if dispatch {
            event_queue.queue(observation, queued);
        }
        dispatch
    }

    fn handle_selection_changed(
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
            Err(error) => {
                report_error(error);
                return;
            }
        };
        let selected_object = selected.as_ref().and_then(|selected| {
            event_queue
                .selection_items
                .borrow()
                .iter()
                .find_map(|item| {
                    (item.owner == object && item.value == *selected).then_some(item.object)
                })
        });
        let observation = Observation::SetSelection {
            object,
            selected: selected_object,
        };
        if !event_queue.observe(object, event_id, observation.clone()) {
            return;
        }
        let value = match selected.as_ref() {
            Some(selected) => {
                match GeneratedHandle::selection_payload(payload_property, selected) {
                    Ok(value) => value,
                    Err(error) => {
                        report_error(error.into());
                        return;
                    }
                }
            }
            None => None,
        };
        let event = event.borrow();
        event_queue.queue(
            Some(observation),
            event.callback.is_some().then(|| QueuedEvent {
                object,
                event: event_id,
                revision: event.revision,
                payload: EventPayload::Selection(SelectionChange {
                    item: selected_object,
                    value,
                }),
            }),
        );
        Self::schedule_event_wake(event_queue);
    }

    fn pointer_event_info(
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
                match element.Focus(native::FocusState::Pointer) {
                    Ok(true) => {
                        let cleanup_states = Rc::clone(&pending_focus_states);
                        let cleanup = native::DispatcherQueueHandler::new(move || {
                            cleanup_states.borrow_mut().remove(&object);
                        });
                        match native::DispatcherQueue::GetForCurrentThread().and_then(|queue| {
                            queue.TryEnqueueWithPriority(
                                native::DispatcherQueuePriority::Low,
                                &cleanup,
                            )
                        }) {
                            Ok(true) => {}
                            Ok(false) => {
                                pending_focus_states.borrow_mut().remove(&object);
                                report_error(windows_core::Error::new(
                                    native::E_FAIL,
                                    "dispatcher rejected pointer focus cleanup",
                                ));
                            }
                            Err(error) => {
                                pending_focus_states.borrow_mut().remove(&object);
                                report_error(error);
                            }
                        }
                    }
                    Ok(false) => {
                        pending_focus_states.borrow_mut().remove(&object);
                    }
                    Err(error) => {
                        pending_focus_states.borrow_mut().remove(&object);
                        report_error(error);
                    }
                }
            });
            let accepted = native::DispatcherQueue::GetForCurrentThread()?
                .TryEnqueueWithPriority(native::DispatcherQueuePriority::Normal, &handler)?;
            if !accepted {
                return Err(windows_core::Error::new(
                    native::E_FAIL,
                    "dispatcher rejected pointer focus",
                )
                .into());
            }
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

    fn input_modifiers_from_virtual_keys(
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

    fn key_event_info(
        args: &native::KeyRoutedEventArgs,
    ) -> Result<KeyEventInfo, WinUiError> {
        Ok(KeyEventInfo {
            key: VirtualKey(args.Key()?.0 as u32),
            original_key: VirtualKey(args.OriginalKey()?.0 as u32),
            status: Self::physical_key_status(args.KeyStatus()?),
            modifiers: Self::input_modifiers()?,
        })
    }

    fn character_event_info(
        args: &native::CharacterReceivedRoutedEventArgs,
    ) -> Result<CharacterEventInfo, WinUiError> {
        Ok(CharacterEventInfo {
            character: args.Character()?,
            status: Self::physical_key_status(args.KeyStatus()?),
            modifiers: Self::input_modifiers()?,
        })
    }

    fn focus_event_info(
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

    fn dispatch_focus_event_info(
        event: &Rc<RefCell<NativeFocusEventInfoEvent>>,
        event_queue: &Rc<NativeEventQueue>,
        object: ObjectId,
        event_id: EventId,
        observation: Option<Observation>,
        value: FocusEventInfo,
    ) {
        let event = event.borrow();
        let queued = event.callback.is_some().then(|| QueuedEvent {
            object,
            event: event_id,
            revision: event.revision,
            payload: EventPayload::FocusEventInfo(value),
        });
        if observation.is_some() || queued.is_some() {
            event_queue.queue(observation, queued);
            Self::schedule_event_wake(event_queue);
        }
    }

    fn schedule_event_wake(event_queue: &Rc<NativeEventQueue>) {
        if event_queue.waker.borrow().is_none() {
            return;
        }
        if event_queue.wake_pending.replace(true) {
            return;
        }
        let event_queue = Rc::clone(event_queue);
        let queue = match native::DispatcherQueue::GetForCurrentThread() {
            Ok(queue) => queue,
            Err(error) => {
                event_queue.wake_pending.set(false);
                report_error(error);
                return;
            }
        };
        let event_queue_for_handler = Rc::clone(&event_queue);
        let handler = native::DispatcherQueueHandler::new(move || {
            event_queue_for_handler.wake_pending.set(false);
            let waker = event_queue_for_handler.waker.borrow().clone();
            if let Some(waker) = waker {
                waker();
            }
        });
        match queue.TryEnqueueWithPriority(native::DispatcherQueuePriority::Normal, &handler) {
            Ok(true) => {}
            Ok(false) => {
                event_queue.wake_pending.set(false);
                report_error(windows_core::Error::new(
                    native::E_FAIL,
                    "DispatcherQueue rejected the Reactor event wake",
                ));
            }
            Err(error) => {
                event_queue.wake_pending.set(false);
                report_error(error);
            }
        }
    }

    fn set_events(
        &mut self,
        object: ObjectId,
        set: &[Event],
        clear: &[EventId],
    ) -> Result<(), WinUiError> {
        match self.handle(object)? {
            Handle::Generated(value) => {
                return value
                    .set_events(object, set, clear)
                    .unwrap_or(Err(WinUiError::InvalidObject(object)));
            }
            Handle::TextBox(value) => {
                let mut native_event = value.event.borrow_mut();
                if clear.contains(&EventId::TextChanged) {
                    native_event.revision = native_event.revision.wrapping_add(1);
                    native_event.callback = None;
                }
                for event in set {
                    match (event.id, &event.value) {
                        (EventId::TextChanged, EventValue::String(callback)) => {
                            native_event.revision = native_event.revision.wrapping_add(1);
                            native_event.callback = Some(callback.clone());
                        }
                        _ => return Err(WinUiError::InvalidObject(object)),
                    }
                }
            }
            Handle::TreeView(value) => {
                let mut native_event = value.item_invoked.borrow_mut();
                if clear.contains(&EventId::ItemInvoked) {
                    native_event.revision = native_event.revision.wrapping_add(1);
                    native_event.callback = None;
                }
                for event in set {
                    match (event.id, &event.value) {
                        (EventId::ItemInvoked, EventValue::String(callback)) => {
                            native_event.revision = native_event.revision.wrapping_add(1);
                            native_event.callback = Some(callback.clone());
                        }
                        _ => return Err(WinUiError::InvalidEvent(object, event.id)),
                    }
                }
            }
            Handle::ListView(value) => {
                for event in clear {
                    match event {
                        EventId::SelectionChanged => {
                            let mut native_event = value.selection_changed.borrow_mut();
                            native_event.revision = native_event.revision.wrapping_add(1);
                            native_event.callback = None;
                        }
                        EventId::DragItemsCompleted => {
                            let mut native_event = value.drag_items_completed.borrow_mut();
                            native_event.revision = native_event.revision.wrapping_add(1);
                            native_event.callback = None;
                        }
                        _ => return Err(WinUiError::InvalidEvent(object, *event)),
                    }
                }
                for event in set {
                    match (event.id, &event.value) {
                        (EventId::SelectionChanged, EventValue::SelectionIndex(callback)) => {
                            let mut native_event = value.selection_changed.borrow_mut();
                            native_event.revision = native_event.revision.wrapping_add(1);
                            native_event.callback = Some(callback.clone());
                        }
                        (EventId::DragItemsCompleted, EventValue::StringList(callback)) => {
                            let mut native_event = value.drag_items_completed.borrow_mut();
                            native_event.revision = native_event.revision.wrapping_add(1);
                            native_event.callback = Some(callback.clone());
                        }
                        _ => return Err(WinUiError::InvalidEvent(object, event.id)),
                    }
                }
            }
            _ => return Err(WinUiError::InvalidObject(object)),
        }
        Ok(())
    }

    fn set_text_box_text(value: &NativeTextBox, text: &str) -> Result<(), WinUiError> {
        if value.value.Text()? == text {
            return Ok(());
        }
        let selection_start = value.value.SelectionStart()?;
        let selection_length = value.value.SelectionLength()?;
        value.value.SetText(text)?;
        *value.observed_text.borrow_mut() = Rc::from(text);
        value.set_count.set(value.set_count.get() + 1);
        let text_length = i32::try_from(text.encode_utf16().count()).unwrap_or(i32::MAX);
        let selection_start = selection_start.clamp(0, text_length);
        let selection_length = selection_length.clamp(0, text_length - selection_start);
        value.value.SetSelectionStart(selection_start)?;
        value.value.SetSelectionLength(selection_length)?;
        Ok(())
    }
}
