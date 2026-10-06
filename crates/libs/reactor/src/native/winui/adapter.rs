impl Runtime<WinUiAdapter> {
    pub fn set_native_event_wakers(
        &mut self,
        native: impl Fn() + 'static,
        imperative: impl Fn() + 'static,
    ) {
        self.adapter_mut().set_event_waker(native);
        self.set_imperative_waker(imperative);
    }
}

impl ComponentHost<WinUiAdapter> {
    pub fn set_native_event_wakers(
        &mut self,
        native: impl Fn() + 'static,
        imperative: impl Fn() + 'static,
    ) {
        self.runtime_mut_internal()
            .set_native_event_wakers(native, imperative);
    }
}

impl Adapter for WinUiAdapter {
    type Error = WinUiError;

    fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
        events.extend(
            self.event_queue
                .events
                .borrow()
                .iter()
                .filter_map(|event| self.native_event(event)),
        );
    }

    fn pop_native_event(&mut self) -> Option<NativeEvent> {
        loop {
            let queued = self.event_queue.events.borrow_mut().pop_front()?;
            if let Some(event) = self.native_event(&queued) {
                return Some(event);
            }
        }
    }

    fn take_error(&mut self) -> Option<Self::Error> {
        self.event_queue.errors.borrow_mut().pop_front()
    }

    fn validate(&self, _mutations: &[Mutation]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
        for mutation in mutations {
            match mutation {
                Mutation::Create { object, kind } => self.create(*object, *kind)?,
                Mutation::Replace { object, kind } => self.replace(*object, *kind)?,
                Mutation::SetProperties { object, set, clear } => {
                    self.set_properties(*object, set, clear)?;
                }
                Mutation::SetEvents { object, set, clear } => {
                    self.set_events(*object, set, clear)?;
                }
                Mutation::ClearWindowTitleBar { object } => {
                    self.clear_window_title_bar(*object)?;
                }
                Mutation::SetWindowTitleBar { object, height } => {
                    self.set_window_title_bar(*object, *height)?;
                }
                Mutation::SetTooltip {
                    target,
                    tooltip,
                    placement,
                } => self.set_tooltip(*target, *tooltip, *placement)?,
                Mutation::SetFlyout {
                    target,
                    content,
                    placement,
                } => self.set_flyout(*target, *content, *placement)?,
                Mutation::SetMenu {
                    target,
                    menu,
                    revision,
                } => self.set_menu(*target, menu.as_ref(), *revision)?,
                Mutation::SetCommandBarFlyout {
                    target,
                    flyout,
                    revision,
                } => self.set_command_bar_flyout(*target, flyout.as_ref(), *revision)?,
                Mutation::SetContentDialog { owner, dialog } => {
                    self.set_content_dialog(*owner, *dialog)?;
                }
                Mutation::SetContentDialogOpen { dialog, open } => {
                    self.set_content_dialog_open(*dialog, *open)?;
                }
                Mutation::SetVirtualSource {
                    object,
                    item_count,
                    source_revision,
                } => {
                    if let Some(items) = self.virtual_items.get(object) {
                        items.reset(*item_count, *source_revision)?;
                    } else {
                        let repeater = match self.handles.get(object) {
                            Some(Handle::Generated(GeneratedHandle::ItemsRepeater(repeater))) => {
                                repeater.clone()
                            }
                            Some(_) => return Err(WinUiError::InvalidObject(*object)),
                            None => return Err(WinUiError::MissingObject(*object)),
                        };
                        let items = NativeVirtualItems::new(
                            &repeater,
                            *object,
                            *item_count,
                            *source_revision,
                            Rc::clone(&self.event_queue),
                        )?;
                        self.virtual_items.insert(*object, items);
                    }
                }
                Mutation::Realize {
                    parent,
                    relation,
                    container,
                    child,
                    ..
                } => {
                    let content = self.ui_element(*child)?;
                    let items = self
                        .virtual_items
                        .get(parent)
                        .ok_or(WinUiError::InvalidObject(*parent))?;
                    items.shells.set_content(*container, Some(&content))?;
                    self.owners.insert(*child, (*parent, *relation));
                }
                Mutation::Recycle {
                    parent,
                    container,
                    child,
                    ..
                } => {
                    let items = self
                        .virtual_items
                        .get(parent)
                        .ok_or(WinUiError::InvalidObject(*parent))?;
                    items.shells.set_content(*container, None)?;
                    items.shells.acknowledge_recycle(*container)?;
                    if let Some(child) = child
                        && self
                            .owners
                            .get(child)
                            .is_some_and(|(owner, _)| owner == parent)
                    {
                        self.owners.remove(child);
                    }
                }
                Mutation::Attach {
                    parent,
                    relation,
                    child,
                } => self.attach(*parent, *relation, *child)?,
                Mutation::Detach {
                    parent,
                    relation,
                    child,
                } => self.detach(*parent, *relation, *child)?,
                Mutation::Insert {
                    parent,
                    relation,
                    child,
                    index,
                } => self.insert(*parent, *relation, *child, *index)?,
                Mutation::Remove {
                    parent,
                    relation,
                    child,
                    index,
                } => self.remove(*parent, *relation, *child, *index)?,
                Mutation::Reorder {
                    parent,
                    relation,
                    moves,
                    children,
                } => self.reorder(*parent, *relation, moves, children)?,
                Mutation::Retire {
                    root,
                    nodes,
                    parent,
                    relation,
                    duration,
                } => {
                    self.start_retirement(*root, nodes.clone(), *parent, *relation, *duration)?;
                }
                Mutation::CompleteRetirement { root, nodes } => {
                    if nodes.iter().any(|object| {
                        self.tooltips.contains_key(object)
                            || self.tooltip_owners.contains_key(object)
                            || self.flyouts.contains_key(object)
                            || self.flyout_owners.contains_key(object)
                            || self.menus.contains_key(object)
                            || self.command_bar_flyouts.contains_key(object)
                            || self.content_dialogs.contains_key(object)
                            || self.content_dialog_owners.contains_key(object)
                    }) {
                        return Err(WinUiError::StillOwned(*root));
                    }
                    self.complete_retirement(*root, nodes)?;
                }
                Mutation::Destroy { object } => {
                    if self
                        .window_title_bar
                        .is_some_and(|(current, _)| current == *object)
                    {
                        return Err(WinUiError::InvalidObject(*object));
                    }
                    if self.owners.contains_key(object) {
                        return Err(WinUiError::StillOwned(*object));
                    }
                    if self.tooltips.contains_key(object)
                        || self.tooltip_owners.contains_key(object)
                        || self.flyouts.contains_key(object)
                        || self.flyout_owners.contains_key(object)
                        || self.menus.contains_key(object)
                        || self.command_bar_flyouts.contains_key(object)
                        || self.content_dialogs.contains_key(object)
                        || self.content_dialog_owners.contains_key(object)
                    {
                        return Err(WinUiError::StillOwned(*object));
                    }
                    let handle = self
                        .handles
                        .remove(object)
                        .ok_or(WinUiError::MissingObject(*object))?;
                    self.encoded_image_failures.remove(object);
                    if let Handle::TreeNode(node) = &handle {
                        self.tree_node_texts
                            .borrow_mut()
                            .remove(&com_identity(&node.value)?);
                    }
                    self.resource_override_keys.remove(object);
                    self.style_states.remove(object);
                    if let Handle::Generated(GeneratedHandle::ContentDialog(dialog)) = handle {
                        self.event_queue
                            .content_dialogs
                            .borrow_mut()
                            .retire(*object, *dialog)?;
                    }
                    self.virtual_items.remove(object);
                    self.observations
                        .retain(|(observed, _), _| *observed != *object);
                    if let Some(initialization) =
                        self.webview_initializations.borrow_mut().remove(object)
                    {
                        for completion in initialization.completions {
                            completion.call(Err(IntegrationError::Unavailable));
                        }
                    }
                    self.event_queue
                        .feedback
                        .borrow_mut()
                        .remove_object(*object);
                    self.event_queue
                        .selection_items
                        .borrow_mut()
                        .retain(|item| item.object != *object && item.owner != *object);
                }
            }
        }
        self.sync_window_title_bars()
    }

    fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
        let element = self.ui_element(object)?;
        self.pending_focus_states
            .borrow_mut()
            .insert(object, ElementFocusState::Programmatic);
        match element.Focus(native::FocusState::Programmatic) {
            Ok(true) => {
                let pending_focus_states = Rc::clone(&self.pending_focus_states);
                let cleanup = native::DispatcherQueueHandler::new(move || {
                    pending_focus_states.borrow_mut().remove(&object);
                });
                let accepted = native::DispatcherQueue::GetForCurrentThread()?
                    .TryEnqueueWithPriority(native::DispatcherQueuePriority::Low, &cleanup)?;
                if !accepted {
                    self.pending_focus_states.borrow_mut().remove(&object);
                    return Err(windows_core::Error::new(
                        native::E_FAIL,
                        "dispatcher rejected focus cleanup",
                    )
                    .into());
                }
                Ok(true)
            }
            Ok(false) => {
                self.pending_focus_states.borrow_mut().remove(&object);
                Ok(false)
            }
            Err(error) => {
                self.pending_focus_states.borrow_mut().remove(&object);
                Err(error.into())
            }
        }
    }

    fn imperative(&mut self, request: ImperativeRequest) -> Result<(), Self::Error> {
        match request {
            ImperativeRequest::Focus { object, completion } => {
                completion.call(
                    self.focus(object)
                        .map_err(|error| integration_error(error.into())),
                );
            }
            ImperativeRequest::InitializeWebView2 { object, completion } => {
                let Some(Handle::Generated(GeneratedHandle::WebView2(control))) =
                    self.handles.get(&object)
                else {
                    completion.call(Err(IntegrationError::Unavailable));
                    return Ok(());
                };
                if let Ok(core) = control.CoreWebView2() {
                    completion.call(Ok(core.into()));
                    return Ok(());
                }
                if let Some(initialization) =
                    self.webview_initializations.borrow_mut().get_mut(&object)
                {
                    initialization.completions.push(completion);
                    return Ok(());
                }
                let setup = (|| {
                    let initializations = Rc::clone(&self.webview_initializations);
                    let initialized = control.CoreWebView2Initialized(move |sender, _| {
                        let result = sender
                            .as_ref()
                            .ok_or_else(windows_core::Error::empty)
                            .and_then(|control| control.CoreWebView2())
                            .map(Into::into)
                            .map_err(integration_error);
                        complete_webview_initialization(&initializations, object, result);
                    })?;
                    let framework = control.cast::<native::IFrameworkElement>()?;
                    let is_loaded = framework.IsLoaded()?;
                    Ok::<_, windows_core::Error>((initialized, framework, is_loaded))
                })();
                let (initialized, framework, is_loaded) = match setup {
                    Ok(setup) => setup,
                    Err(error) => {
                        completion.call(Err(integration_error(error)));
                        return Ok(());
                    }
                };
                let loaded = if is_loaded {
                    None
                } else {
                    let control = control.clone();
                    let initializations = Rc::clone(&self.webview_initializations);
                    let loaded = framework.Loaded(move |_, _| {
                        let result = control.EnsureCoreWebView2Async();
                        match result {
                            Ok(action) => {
                                if let Some(initialization) =
                                    initializations.borrow_mut().get_mut(&object)
                                {
                                    initialization._action = Some(action);
                                }
                            }
                            Err(error) => complete_webview_initialization(
                                &initializations,
                                object,
                                Err(integration_error(error)),
                            ),
                        }
                    });
                    match loaded {
                        Ok(loaded) => Some(loaded),
                        Err(error) => {
                            completion.call(Err(integration_error(error)));
                            return Ok(());
                        }
                    }
                };
                self.webview_initializations.borrow_mut().insert(
                    object,
                    WebViewInitialization {
                        _action: None,
                        _loaded: loaded,
                        _initialized: initialized,
                        completions: vec![completion],
                    },
                );
                if is_loaded {
                    match control.EnsureCoreWebView2Async() {
                        Ok(action) => {
                            if let Some(initialization) =
                                self.webview_initializations.borrow_mut().get_mut(&object)
                            {
                                initialization._action = Some(action);
                            }
                        }
                        Err(error) => complete_webview_initialization(
                            &self.webview_initializations,
                            object,
                            Err(integration_error(error)),
                        ),
                    }
                }
            }
            ImperativeRequest::ObserveSwapChainPanel {
                object,
                observation,
                binding,
                callback,
            } => {
                let Some(Handle::Generated(GeneratedHandle::SwapChainPanel(control))) =
                    self.handles.get(&object)
                else {
                    return Ok(());
                };
                let element = control.cast::<native::IFrameworkElement>()?;
                callback.call(SwapChainPanelEvent::Metrics {
                    binding: SwapChainPanelBinding::new(binding),
                    width: element.ActualWidth().unwrap_or(0.0),
                    height: element.ActualHeight().unwrap_or(0.0),
                    scale_x: control.CompositionScaleX().unwrap_or(1.0),
                    scale_y: control.CompositionScaleY().unwrap_or(1.0),
                });
                let size_callback = callback.clone();
                let size_control = control.clone();
                let size = element.SizeChanged(move |_, args| {
                    if let Some(args) = args.as_ref()
                        && let Ok(size) = args.NewSize()
                    {
                        size_callback.call(SwapChainPanelEvent::Metrics {
                            binding: SwapChainPanelBinding::new(binding),
                            width: f64::from(size.width),
                            height: f64::from(size.height),
                            scale_x: size_control.CompositionScaleX().unwrap_or(1.0),
                            scale_y: size_control.CompositionScaleY().unwrap_or(1.0),
                        });
                    }
                })?;
                let scale_callback = callback.clone();
                let scale_element = element;
                let scale = control.CompositionScaleChanged(move |sender, _| {
                    if let Some(sender) = sender.as_ref() {
                        scale_callback.call(SwapChainPanelEvent::Metrics {
                            binding: SwapChainPanelBinding::new(binding),
                            width: scale_element.ActualWidth().unwrap_or(0.0),
                            height: scale_element.ActualHeight().unwrap_or(0.0),
                            scale_x: sender.CompositionScaleX().unwrap_or(1.0),
                            scale_y: sender.CompositionScaleY().unwrap_or(1.0),
                        });
                    }
                })?;
                let rendering_callback = callback;
                let rendering = native::CompositionTarget::Rendering(move |_, _| {
                    rendering_callback.call(SwapChainPanelEvent::Rendering);
                })?;
                self.observations.insert(
                    (object, observation),
                    ObservationSubscription::SwapChainPanel {
                        _rendering: rendering,
                        _scale: scale,
                        _size: size,
                    },
                );
            }
            ImperativeRequest::RequestSwapChainPanelFrame { object, completion } => {
                if !matches!(
                    self.handles.get(&object),
                    Some(Handle::Generated(GeneratedHandle::SwapChainPanel(_)))
                ) {
                    completion.call(Err(IntegrationError::Unavailable));
                    return Ok(());
                }
                let queued_completion = completion.clone();
                let queued = native::DispatcherQueue::GetForCurrentThread().and_then(|queue| {
                    queue.TryEnqueueWithPriority(
                        native::DispatcherQueuePriority::Normal,
                        &native::DispatcherQueueHandler::new(move || {
                            queued_completion.call(Ok(()));
                        }),
                    )
                });
                match queued {
                    Ok(true) => {}
                    Ok(false) => completion.call(Err(IntegrationError::Unavailable)),
                    Err(error) => completion.call(Err(integration_error(error))),
                }
            }
            ImperativeRequest::SetSwapChain {
                object,
                swap_chain,
                completion,
            } => {
                let result = match self.handles.get(&object) {
                    Some(Handle::Generated(GeneratedHandle::SwapChainPanel(control))) => control
                        .cast::<native::ISwapChainPanelNative>()
                        .and_then(|panel| unsafe {
                            panel
                                .SetSwapChain(
                                    swap_chain
                                        .as_ref()
                                        .map_or(std::ptr::null_mut(), Interface::as_raw),
                                )
                                .ok()
                        }),
                    _ => Err(windows_core::Error::new(
                        HRESULT(0x8000000E_u32 as i32),
                        "swap-chain panel unavailable",
                    )),
                }
                .map_err(integration_error);
                completion.call(result);
            }
            ImperativeRequest::SetNativeImageSource {
                object,
                source,
                completion,
            } => {
                let result = match self.handles.get(&object) {
                    Some(Handle::Generated(GeneratedHandle::Image(control))) => source
                        .as_ref()
                        .map(|source| source.cast::<native::ImageSource>())
                        .transpose()
                        .and_then(|source| control.value.SetSource(source.as_ref())),
                    _ => Err(windows_core::Error::new(
                        HRESULT(0x8000000E_u32 as i32),
                        "image unavailable",
                    )),
                }
                .map_err(integration_error);
                completion.call(result);
            }
            ImperativeRequest::ObserveImageScale {
                object,
                observation,
                callback,
            } => {
                let Some(Handle::Generated(GeneratedHandle::Image(control))) =
                    self.handles.get(&object)
                else {
                    return Ok(());
                };
                let element = control.value.cast::<native::UIElement>()?;
                let framework = control.value.cast::<native::IFrameworkElement>()?;
                let changed = Rc::new(RefCell::new(None));
                observe_xaml_scale(&element, &changed, &self.event_queue, callback.clone())?;
                let loaded_element = element;
                let loaded_changed = Rc::clone(&changed);
                let loaded_errors = Rc::clone(&self.event_queue);
                let loaded = framework.Loaded(move |_, _| {
                    if let Err(error) = observe_xaml_scale(
                        &loaded_element,
                        &loaded_changed,
                        &loaded_errors,
                        callback.clone(),
                    ) {
                        loaded_errors.errors.borrow_mut().push_back(error.into());
                    }
                })?;
                self.observations.insert(
                    (object, observation),
                    ObservationSubscription::ImageScale {
                        _changed: changed,
                        _loaded: loaded,
                    },
                );
            }
            ImperativeRequest::ObserveCompositionHost {
                object,
                observation,
                callback,
            } => {
                let Some(Handle::Generated(GeneratedHandle::Grid(control))) =
                    self.handles.get(&object)
                else {
                    return Ok(());
                };
                let element = control.cast::<native::UIElement>()?;
                let framework = control.cast::<native::IFrameworkElement>()?;
                let visual = native::ElementCompositionPreview::GetElementVisual(&element)?;
                let compositor = visual.cast::<native::ICompositionObject>()?.Compositor()?;
                callback.call(CompositionHostEvent::Ready {
                    compositor: compositor.into(),
                    width: framework.ActualWidth().unwrap_or(0.0),
                    height: framework.ActualHeight().unwrap_or(0.0),
                    scale: xaml_scale(&element)?,
                });
                let size_callback = callback.clone();
                let size_element = element.clone();
                let size_errors = Rc::clone(&self.event_queue);
                let size = framework.SizeChanged(move |_, args| {
                    if let Some(args) = args.as_ref()
                        && let Ok(value) = args.NewSize()
                    {
                        match xaml_scale(&size_element) {
                            Ok(scale) => size_callback.call(CompositionHostEvent::Metrics {
                                width: f64::from(value.width),
                                height: f64::from(value.height),
                                scale,
                            }),
                            Err(error) => {
                                size_errors.errors.borrow_mut().push_back(error.into());
                            }
                        }
                    }
                })?;
                let changed = Rc::new(RefCell::new(None));
                let scale_callback = callback.clone();
                let scale_framework = framework.clone();
                observe_xaml_scale(
                    &element,
                    &changed,
                    &self.event_queue,
                    Callback::new(move |scale| {
                        scale_callback.call(CompositionHostEvent::Metrics {
                            width: scale_framework.ActualWidth().unwrap_or(0.0),
                            height: scale_framework.ActualHeight().unwrap_or(0.0),
                            scale,
                        });
                    }),
                )?;
                let loaded_element = element;
                let loaded_changed = Rc::clone(&changed);
                let loaded_errors = Rc::clone(&self.event_queue);
                let loaded_callback = callback;
                let loaded_framework = framework;
                let loaded = loaded_framework.clone().Loaded(move |_, _| {
                    let framework = loaded_framework.clone();
                    if let Err(error) = observe_xaml_scale(
                        &loaded_element,
                        &loaded_changed,
                        &loaded_errors,
                        Callback::new({
                            let callback = loaded_callback.clone();
                            move |scale| {
                                callback.call(CompositionHostEvent::Metrics {
                                    width: framework.ActualWidth().unwrap_or(0.0),
                                    height: framework.ActualHeight().unwrap_or(0.0),
                                    scale,
                                });
                            }
                        }),
                    ) {
                        loaded_errors.errors.borrow_mut().push_back(error.into());
                    }
                })?;
                self.observations.insert(
                    (object, observation),
                    ObservationSubscription::CompositionHost {
                        _changed: changed,
                        _loaded: loaded,
                        _size: size,
                    },
                );
            }
            ImperativeRequest::RevokeObservation {
                object,
                observation,
            } => {
                self.observations.remove(&(object, observation));
            }
            ImperativeRequest::SetCompositionChildVisual {
                object,
                visual,
                completion,
            } => {
                let result = match self.handles.get(&object) {
                    Some(Handle::Generated(GeneratedHandle::Grid(control))) => {
                        control.cast::<native::UIElement>().and_then(|element| {
                            visual
                                .as_ref()
                                .map(|visual| visual.cast::<native::Visual>())
                                .transpose()
                                .and_then(|visual| {
                                    native::ElementCompositionPreview::SetElementChildVisual(
                                        &element,
                                        visual.as_ref(),
                                    )
                                })
                        })
                    }
                    _ => Err(windows_core::Error::new(
                        HRESULT(0x8000000E_u32 as i32),
                        "composition host unavailable",
                    )),
                }
                .map_err(integration_error);
                completion.call(result);
            }
        }
        Ok(())
    }
}
