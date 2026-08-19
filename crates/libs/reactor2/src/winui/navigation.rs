use super::*;

pub(super) struct NavigationCallbackState {
    pub(super) suppressing: Cell<bool>,
    suppressing_state: Cell<bool>,
    pub(super) expected: Cell<Option<u64>>,
    selection_feedback: Cell<bool>,
    expected_pane_open: Cell<bool>,
    pane_feedback: Cell<bool>,
}

pub(super) struct NavigationViewState {
    pub(super) value: bindings::NavigationView,
    pub(super) items: Rc<RefCell<Vec<(u64, bindings::NavigationViewItem)>>>,
    pub(super) callback: Rc<NavigationCallbackState>,
    _revoker: windows_core::EventRevoker,
}

pub(super) struct NavigationViewItemState {
    pub(super) value: bindings::NavigationViewItem,
    pub(super) key: Cell<Option<u64>>,
}

impl WinUiRuntime {
    pub(super) fn create_navigation_view(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::NavigationView::new()?;
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let items = Rc::new(RefCell::new(
            Vec::<(u64, bindings::NavigationViewItem)>::new(),
        ));
        let callback = Rc::new(NavigationCallbackState {
            suppressing: Cell::new(false),
            suppressing_state: Cell::new(false),
            expected: Cell::new(None),
            selection_feedback: Cell::new(false),
            expected_pane_open: Cell::new(true),
            pane_feedback: Cell::new(false),
        });
        let callback_items = Rc::clone(&items);
        let selection_state = Rc::clone(&callback);
        let revoker = value.SelectionChanged(move |sender, _args| {
            if selection_state.suppressing.get() {
                return;
            }
            let Some(sender) = sender.as_ref() else {
                return;
            };
            let selected = sender.SelectedItem().ok();
            let key = selected.as_ref().and_then(|selected| {
                callback_items.borrow().iter().find_map(|(key, item)| {
                    item.cast::<windows_core::IInspectable>()
                        .ok()
                        .filter(|candidate| candidate == selected)
                        .map(|_| *key)
                })
            });
            if selected.is_some() && key.is_none() {
                let controlled = selection_state
                    .expected
                    .get()
                    .and_then(|expected| {
                        callback_items
                            .borrow()
                            .iter()
                            .find(|(key, _)| *key == expected)
                            .map(|(_, item)| item.clone())
                    })
                    .map(|item| item.cast::<windows_core::IInspectable>())
                    .transpose()
                    .unwrap();
                selection_state.suppressing.set(true);
                let result = sender.SetSelectedItem(controlled.as_ref());
                selection_state.suppressing.set(false);
                result.unwrap();
                return;
            }
            if selection_state.expected.get() == key {
                return;
            }
            if !selection_state.selection_feedback.get() {
                let controlled = selection_state
                    .expected
                    .get()
                    .and_then(|expected| {
                        callback_items
                            .borrow()
                            .iter()
                            .find(|(key, _)| *key == expected)
                            .map(|(_, item)| item.clone())
                    })
                    .map(|item| item.cast::<windows_core::IInspectable>())
                    .transpose()
                    .unwrap();
                selection_state.suppressing.set(true);
                let result = sender.SetSelectedItem(controlled.as_ref());
                selection_state.suppressing.set(false);
                result.unwrap();
                return;
            }
            selection_state.expected.set(key);
            queue_latest_event(&events, NativeEvent::SelectedKeyChanged { target: id, key });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        let object: bindings::DependencyObject = value.cast()?;
        let pane_state = Rc::clone(&callback);
        let pane_events = Rc::clone(&self.events);
        let pane_waker = Rc::clone(&self.waker);
        let pane_callback =
            bindings::DependencyPropertyChangedCallback::new(move |sender, _property| {
                if pane_state.suppressing_state.get() {
                    return;
                }
                let value: bindings::NavigationView = sender.as_ref().unwrap().cast().unwrap();
                let open = value.IsPaneOpen().unwrap();
                if pane_state.expected_pane_open.get() == open {
                    return;
                }
                if !pane_state.pane_feedback.get() {
                    pane_state.suppressing_state.set(true);
                    value
                        .SetIsPaneOpen(pane_state.expected_pane_open.get())
                        .unwrap();
                    pane_state.suppressing_state.set(false);
                    return;
                }
                pane_state.expected_pane_open.set(open);
                pane_events
                    .borrow_mut()
                    .push_back(NativeEvent::NavigationPaneOpenChanged { target: id, open });
                if let Some(wake) = pane_waker.borrow().as_ref() {
                    wake();
                }
            });
        let pane_property = bindings::NavigationView::IsPaneOpenProperty()?;
        object.RegisterPropertyChangedCallback(&pane_property, &pane_callback)?;
        let mode_state = Rc::clone(&callback);
        let mode_events = Rc::clone(&self.events);
        let mode_waker = Rc::clone(&self.waker);
        let mode_callback =
            bindings::DependencyPropertyChangedCallback::new(move |sender, _property| {
                if mode_state.suppressing_state.get() {
                    return;
                }
                let value: bindings::NavigationView = sender.as_ref().unwrap().cast().unwrap();
                let mode = native_display_mode(value.DisplayMode().unwrap());
                mode_events
                    .borrow_mut()
                    .push_back(NativeEvent::NavigationDisplayModeChanged { target: id, mode });
                if let Some(wake) = mode_waker.borrow().as_ref() {
                    wake();
                }
            });
        let mode_property = bindings::NavigationView::DisplayModeProperty()?;
        object.RegisterPropertyChangedCallback(&mode_property, &mode_callback)?;
        Ok(Handle::NavigationView(Box::new(NavigationViewState {
            value,
            items,
            callback,
            _revoker: revoker,
        })))
    }

    pub(super) fn create_navigation_view_item(&self) -> WindowsResult<Handle> {
        Ok(Handle::NavigationViewItem(Box::new(
            NavigationViewItemState {
                value: bindings::NavigationViewItem::new()?,
                key: Cell::new(None),
            },
        )))
    }

    pub(super) fn apply_navigation_update(
        &self,
        id: NodeId,
        update: &NavigationUpdate,
    ) -> WindowsResult<()> {
        match update {
            NavigationUpdate::Properties(value) => self.apply_navigation_view_update(id, value),
            NavigationUpdate::Selection(value) => self.apply_navigation_selection(id, *value),
        }
    }

    pub(super) fn apply_navigation_view_update(
        &self,
        id: NodeId,
        update: &NavigationViewUpdate,
    ) -> WindowsResult<()> {
        let Handle::NavigationView(state) = &self.node(id)?.handle else {
            panic!("NavigationView update target is invalid");
        };
        let header = update
            .header
            .as_ref()
            .map(|header| {
                let value = bindings::TextBlock::new()?;
                value.SetText(header)?;
                value.cast::<windows_core::IInspectable>()
            })
            .transpose()?;
        state.callback.suppressing_state.set(true);
        state.callback.expected_pane_open.set(update.pane_open);
        state
            .callback
            .selection_feedback
            .set(update.selection_feedback);
        state.callback.pane_feedback.set(update.pane_feedback);
        let result = (|| {
            state.value.SetHeader(header.as_ref())?;
            let navigation2: bindings::INavigationView2 = state.value.cast()?;
            navigation2.SetPaneTitle(update.pane_title.as_deref().unwrap_or(""))?;
            state.value.SetIsSettingsVisible(update.settings_visible)?;
            state
                .value
                .SetIsPaneToggleButtonVisible(update.pane_toggle_visible)?;
            state.value.SetIsPaneOpen(update.pane_open)?;
            state.value.SetOpenPaneLength(update.open_pane_length)?;
            navigation2.SetPaneDisplayMode(native_pane_display_mode(update.pane_display_mode))
        })();
        state.callback.suppressing_state.set(false);
        result
    }

    pub(super) fn apply_navigation_selection(
        &self,
        id: NodeId,
        selected_key: Option<u64>,
    ) -> WindowsResult<()> {
        let Handle::NavigationView(state) = &self.node(id)?.handle else {
            panic!("NavigationView selection target is invalid");
        };
        remove_queued_event(&self.events, id, LatestEventSlot::SelectedKeyChanged);
        let selected = selected_key
            .and_then(|key| {
                state
                    .items
                    .borrow()
                    .iter()
                    .find(|(candidate, _)| *candidate == key)
                    .map(|(_, item)| item.clone())
            })
            .map(|item| item.cast::<windows_core::IInspectable>())
            .transpose()?;
        state.callback.expected.set(selected_key);
        state.callback.suppressing.set(true);
        let result = state.value.SetSelectedItem(selected.as_ref());
        state.callback.suppressing.set(false);
        result?;
        Ok(())
    }

    pub(super) fn apply_navigation_view_item_update(
        &self,
        id: NodeId,
        update: &NavigationViewItemUpdate,
    ) -> WindowsResult<()> {
        let icon = media::create_icon(update.icon.as_ref())?;
        let Handle::NavigationViewItem(state) = &self.node(id)?.handle else {
            panic!("NavigationViewItem update target is invalid");
        };
        let text = bindings::TextBlock::new()?;
        text.SetText(&update.label)?;
        let content: bindings::IContentControl = state.value.cast()?;
        content.SetContent(&text)?;
        state.value.SetIcon(icon.as_ref())?;
        state.key.set(Some(update.item_key));
        Ok(())
    }
}

fn native_pane_display_mode(
    value: NavigationPaneDisplayMode,
) -> bindings::NavigationViewPaneDisplayMode {
    match value {
        NavigationPaneDisplayMode::Auto => bindings::NavigationViewPaneDisplayMode::Auto,
        NavigationPaneDisplayMode::Left => bindings::NavigationViewPaneDisplayMode::Left,
        NavigationPaneDisplayMode::Top => bindings::NavigationViewPaneDisplayMode::Top,
        NavigationPaneDisplayMode::LeftCompact => {
            bindings::NavigationViewPaneDisplayMode::LeftCompact
        }
        NavigationPaneDisplayMode::LeftMinimal => {
            bindings::NavigationViewPaneDisplayMode::LeftMinimal
        }
    }
}

fn native_display_mode(value: bindings::NavigationViewDisplayMode) -> NavigationDisplayMode {
    match value {
        bindings::NavigationViewDisplayMode::Minimal => NavigationDisplayMode::Minimal,
        bindings::NavigationViewDisplayMode::Compact => NavigationDisplayMode::Compact,
        bindings::NavigationViewDisplayMode::Expanded => NavigationDisplayMode::Expanded,
        _ => panic!("unexpected NavigationView display mode"),
    }
}
