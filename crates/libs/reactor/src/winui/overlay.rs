use super::*;

pub(super) struct OpenCallbackState {
    pub(super) expected: Cell<bool>,
    pub(super) native: Cell<bool>,
}

pub(super) struct TeachingTipState {
    pub(super) open: Rc<OpenCallbackState>,
    pub(super) current_target: Rc<RefCell<Option<bindings::FrameworkElement>>>,
    pub(super) deferred_open: deferred::DeferredUpdate,
    _closed: windows_core::EventRevoker,
    action_button_click: Option<windows_core::EventRevoker>,
}

pub(super) struct ContentDialogState {
    ui: bindings::UIElement,
    content: bindings::IContentControl,
    pub(super) open: Rc<OpenCallbackState>,
    pub(super) operation:
        Rc<RefCell<Option<windows_future::IAsyncOperation<bindings::ContentDialogResult>>>>,
    pub(super) deferred_open: deferred::DeferredUpdate,
    _closed: windows_core::EventRevoker,
}

impl ContentDialogState {
    pub(super) fn ui_element(&self) -> bindings::UIElement {
        self.ui.clone()
    }

    pub(super) fn content_control(&self) -> bindings::IContentControl {
        self.content.clone()
    }

    pub(super) fn destroy(
        &self,
        value: &bindings::ContentDialog,
        id: NodeId,
        active_content_dialogs: &RefCell<BTreeMap<NodeId, NodeId>>,
    ) -> WindowsResult<()> {
        active_content_dialogs
            .borrow_mut()
            .retain(|_, active| *active != id);
        self.open.expected.set(false);
        self.deferred_open
            .revision
            .set(self.deferred_open.revision.get().wrapping_add(1));
        if self.open.native.replace(false) {
            value.Hide()
        } else {
            Ok(())
        }
    }

    pub(super) fn shutdown(&self, value: &bindings::ContentDialog) {
        self.open.expected.set(false);
        self.deferred_open.active.set(false);
        self.deferred_open
            .revision
            .set(self.deferred_open.revision.get().wrapping_add(1));
        if self.open.native.replace(false) {
            _ = value.Hide();
        }
    }
}

pub(super) fn subscribe_flyout(
    flyout: &bindings::IFlyoutBase,
    target: NodeId,
    events: Rc<RefCell<VecDeque<NativeEvent>>>,
    waker: Rc<RefCell<Option<Rc<dyn Fn()>>>>,
) -> WindowsResult<[windows_core::EventRevoker; 2]> {
    let opened_events = Rc::clone(&events);
    let opened_waker = Rc::clone(&waker);
    let opened = flyout.Opened(move |_sender, _args| {
        opened_events
            .borrow_mut()
            .push_back(NativeEvent::FlyoutOpened { target });
        if let Some(wake) = opened_waker.borrow().as_ref() {
            wake();
        }
    })?;
    let closed = flyout.Closed(move |_sender, _args| {
        events
            .borrow_mut()
            .push_back(NativeEvent::FlyoutClosed { target });
        if let Some(wake) = waker.borrow().as_ref() {
            wake();
        }
    })?;
    Ok([opened, closed])
}

impl WinUiRuntime {
    pub(super) fn create_flyout(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::Flyout::new()?;
        let flyout: bindings::IFlyoutBase = value.cast()?;
        let revokers =
            subscribe_flyout(&flyout, id, Rc::clone(&self.events), Rc::clone(&self.waker))?;
        Ok(Handle::Flyout {
            _revokers: revokers,
            value,
        })
    }

    pub(super) fn create_content_dialog(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::ContentDialog::new()?;
        let ui: bindings::UIElement = value.cast()?;
        let content: bindings::IContentControl = value.cast()?;
        let open = Rc::new(OpenCallbackState {
            expected: Cell::new(false),
            native: Cell::new(false),
        });
        let operation = Rc::new(RefCell::new(None));
        let closed_state = Rc::clone(&open);
        let closed_operation = Rc::clone(&operation);
        let closed_active_dialogs = Rc::clone(&self.active_content_dialogs);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let closed = value.Closed(move |_sender, args| {
            closed_state.native.set(false);
            closed_operation.borrow_mut().take();
            closed_active_dialogs
                .borrow_mut()
                .retain(|_, active| *active != id);
            if !closed_state.expected.replace(false) {
                return;
            }
            let args = args.as_ref().unwrap();
            let result = match args.Result().unwrap() {
                bindings::ContentDialogResult::Primary => ContentDialogResult::Primary,
                bindings::ContentDialogResult::Secondary => ContentDialogResult::Secondary,
                _ => ContentDialogResult::None,
            };
            events
                .borrow_mut()
                .push_back(NativeEvent::ContentDialogClosed { target: id, result });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::ContentDialog {
            value,
            state: Box::new(ContentDialogState {
                ui,
                content,
                open,
                operation,
                deferred_open: deferred::DeferredUpdate::new(),
                _closed: closed,
            }),
        })
    }

    pub(super) fn create_teaching_tip(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::TeachingTip::new()?;
        let open = Rc::new(OpenCallbackState {
            expected: Cell::new(false),
            native: Cell::new(false),
        });
        let closed_state = Rc::clone(&open);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let closed = value.Closed(move |_sender, _args| {
            closed_state.native.set(false);
            if !closed_state.expected.replace(false) {
                return;
            }
            events
                .borrow_mut()
                .push_back(NativeEvent::TeachingTipClosed { target: id });
            if let Some(wake) = waker.borrow().as_ref() {
                wake();
            }
        })?;
        Ok(Handle::TeachingTip {
            value,
            state: Box::new(TeachingTipState {
                open,
                current_target: Rc::new(RefCell::new(None)),
                deferred_open: deferred::DeferredUpdate::new(),
                _closed: closed,
                action_button_click: None,
            }),
        })
    }

    pub(super) fn apply_teaching_tip_update(
        &mut self,
        id: NodeId,
        update: &TeachingTipUpdate,
    ) -> WindowsResult<()> {
        match update {
            TeachingTipUpdate::Title(value) => {
                let Handle::TeachingTip { value: control, .. } = &self.node(id)?.handle else {
                    panic!("TeachingTip update target is not a TeachingTip");
                };
                control.SetTitle(value)
            }
            TeachingTipUpdate::Subtitle(value) => {
                let Handle::TeachingTip { value: control, .. } = &self.node(id)?.handle else {
                    panic!("TeachingTip update target is not a TeachingTip");
                };
                control.SetSubtitle(value)
            }
            TeachingTipUpdate::Open(open) => {
                let (value, revision) = {
                    let Handle::TeachingTip { value, state } = &self.node(id)?.handle else {
                        panic!("TeachingTip update target is not a TeachingTip");
                    };
                    state.open.expected.set(*open);
                    let revision = state.deferred_open.revision.get().wrapping_add(1);
                    state.deferred_open.revision.set(revision);
                    (value.clone(), revision)
                };
                if !open {
                    let Handle::TeachingTip { state, .. } = &self.node(id)?.handle else {
                        unreachable!()
                    };
                    state.open.native.set(false);
                    return value.SetIsOpen(false);
                }
                match &self.node(id)?.handle {
                    Handle::TeachingTip { state, .. } => state
                        .current_target
                        .borrow()
                        .clone()
                        .unwrap_or_else(|| panic!("TeachingTip has no target owner")),
                    _ => unreachable!(),
                };
                self.enqueue_teaching_tip_open(id, revision)
            }
            TeachingTipUpdate::LightDismiss(enabled) => {
                let Handle::TeachingTip { value, .. } = &self.node(id)?.handle else {
                    panic!("TeachingTip update target is not a TeachingTip");
                };
                value.SetIsLightDismissEnabled(*enabled)
            }
            TeachingTipUpdate::ActionButton(content) => {
                let Handle::TeachingTip { value, .. } = &self.node(id)?.handle else {
                    panic!("TeachingTip update target is not a TeachingTip");
                };
                let content = content
                    .as_deref()
                    .map(|text| {
                        windows_reference::IReference::<windows_core::HSTRING>::from(
                            windows_core::HSTRING::from(text),
                        )
                        .cast::<windows_core::IInspectable>()
                    })
                    .transpose()?;
                value.SetActionButtonContent(content.as_ref())
            }
            TeachingTipUpdate::CloseButton(content) => {
                let Handle::TeachingTip { value, .. } = &self.node(id)?.handle else {
                    panic!("TeachingTip update target is not a TeachingTip");
                };
                let content = content
                    .as_deref()
                    .map(|text| {
                        windows_reference::IReference::<windows_core::HSTRING>::from(
                            windows_core::HSTRING::from(text),
                        )
                        .cast::<windows_core::IInspectable>()
                    })
                    .transpose()?;
                value.SetCloseButtonContent(content.as_ref())
            }
            TeachingTipUpdate::ActionButtonClick(enabled) => {
                let previous = match &mut self.node_mut(id)?.handle {
                    Handle::TeachingTip { state, .. } => state.action_button_click.take(),
                    _ => {
                        panic!("TeachingTip update target is not a TeachingTip");
                    }
                };
                drop(previous);
                if !enabled {
                    return Ok(());
                }
                let control = match &self.node(id)?.handle {
                    Handle::TeachingTip { value, .. } => value.clone(),
                    _ => unreachable!(),
                };
                let events = Rc::clone(&self.events);
                let waker = Rc::clone(&self.waker);
                let revoker = control.ActionButtonClick(move |_sender, _args| {
                    events
                        .borrow_mut()
                        .push_back(NativeEvent::TeachingTipAction { target: id });
                    if let Some(wake) = waker.borrow().as_ref() {
                        wake();
                    }
                })?;
                let Handle::TeachingTip { state, .. } = &mut self.node_mut(id)?.handle else {
                    unreachable!()
                };
                state.action_button_click = Some(revoker);
                Ok(())
            }
        }
    }

    pub(super) fn apply_content_dialog_update(
        &self,
        id: NodeId,
        update: &ContentDialogUpdate,
    ) -> WindowsResult<()> {
        let Handle::ContentDialog { value, state } = &self.node(id)?.handle else {
            panic!("ContentDialog update target is not a ContentDialog");
        };
        let open_changed = state.open.expected.get() != update.open;
        value.SetPrimaryButtonText(&update.primary_button_text)?;
        value.SetSecondaryButtonText(&update.secondary_button_text)?;
        value.SetCloseButtonText(&update.close_button_text)?;
        value.SetIsPrimaryButtonEnabled(update.primary_button_enabled)?;
        value.SetIsSecondaryButtonEnabled(update.secondary_button_enabled)?;
        if !open_changed {
            return Ok(());
        }
        if !update.open {
            self.active_content_dialogs
                .borrow_mut()
                .retain(|_, active| *active != id);
        }
        state.open.expected.set(update.open);
        let revision = state.deferred_open.revision.get().wrapping_add(1);
        state.deferred_open.revision.set(revision);
        if update.open {
            self.enqueue_content_dialog_open(id, revision)
        } else if state.open.native.replace(false) {
            value.Hide()
        } else {
            Ok(())
        }
    }

    fn enqueue_content_dialog_open(&self, id: NodeId, revision: u64) -> WindowsResult<()> {
        let Handle::ContentDialog { state, .. } = &self.node(id)?.handle else {
            panic!("ContentDialog update target is not a ContentDialog");
        };
        self.enqueue_deferred_ready(
            id,
            revision,
            DeferredAction::ContentDialogOpen,
            Rc::clone(&state.deferred_open.active),
            Rc::clone(&state.deferred_open.revision),
            "dispatcher rejected ContentDialog open update",
        )
    }

    pub(super) fn defer_teaching_tip_open(&self, id: NodeId) -> WindowsResult<()> {
        let revision = {
            let Handle::TeachingTip { state, .. } = &self.node(id)?.handle else {
                panic!("TeachingTip update target is not a TeachingTip");
            };
            let revision = state.deferred_open.revision.get().wrapping_add(1);
            state.deferred_open.revision.set(revision);
            revision
        };
        self.enqueue_teaching_tip_open(id, revision)
    }

    fn enqueue_teaching_tip_open(&self, id: NodeId, revision: u64) -> WindowsResult<()> {
        let Handle::TeachingTip { state, .. } = &self.node(id)?.handle else {
            panic!("TeachingTip update target is not a TeachingTip");
        };
        self.enqueue_deferred_ready(
            id,
            revision,
            DeferredAction::TeachingTipOpen,
            Rc::clone(&state.deferred_open.active),
            Rc::clone(&state.deferred_open.revision),
            "dispatcher rejected TeachingTip open update",
        )
    }

    pub(super) fn run_overlay_deferred(
        &self,
        target: NodeId,
        window: Option<NodeId>,
        revision: u64,
        action: DeferredAction,
    ) -> WindowsResult<()> {
        match action {
            DeferredAction::ContentDialogOpen => {
                let Handle::ContentDialog { value, state } = &self.node(target)?.handle else {
                    panic!("deferred ContentDialog target is invalid");
                };
                if !state.deferred_open.active.get()
                    || state.deferred_open.revision.get() != revision
                    || !state.open.expected.get()
                {
                    return Ok(());
                }
                let window =
                    window.unwrap_or_else(|| panic!("ContentDialog window is unavailable"));
                if self
                    .active_content_dialogs
                    .borrow()
                    .get(&window)
                    .is_some_and(|active| *active != target)
                {
                    panic!("only one ContentDialog may be open per window");
                }
                let root = self
                    .windows
                    .get(&window)
                    .and_then(|window| window.root.as_ref())
                    .unwrap_or_else(|| panic!("window root is unavailable"));
                let xaml_root = root.XamlRoot()?;
                state.ui.SetXamlRoot(&xaml_root)?;
                let operation = value.ShowAsync()?;
                state.operation.borrow_mut().replace(operation);
                state.open.native.set(true);
                self.active_content_dialogs
                    .borrow_mut()
                    .insert(window, target);
                Ok(())
            }
            DeferredAction::TeachingTipOpen => {
                let Handle::TeachingTip { value, state } = &self.node(target)?.handle else {
                    panic!("deferred TeachingTip target is invalid");
                };
                if !state.deferred_open.active.get()
                    || state.deferred_open.revision.get() != revision
                    || !state.open.expected.get()
                {
                    return Ok(());
                }
                let owner = state
                    .current_target
                    .borrow()
                    .clone()
                    .unwrap_or_else(|| panic!("TeachingTip has no target owner"));
                value.SetTarget(&owner)?;
                value.SetIsOpen(true)?;
                state.open.native.set(true);
                Ok(())
            }
            DeferredAction::RadioButtonsSelection => {
                panic!("deferred action is not an overlay action")
            }
        }
    }
}

#[cfg(test)]
mod callback_state_tests {
    use super::*;

    #[test]
    fn overlay_state_distinguishes_controlled_and_native_close_reentry() {
        let state = Rc::new(OpenCallbackState {
            expected: Cell::new(true),
            native: Cell::new(true),
        });
        let callback_state = Rc::clone(&state);
        let published = Cell::new(0);
        let closed = || {
            callback_state.native.set(false);
            if callback_state.expected.replace(false) {
                published.set(published.get() + 1);
            }
        };

        state.expected.set(false);
        closed();
        assert!(!state.native.get());
        assert_eq!(published.get(), 0);

        state.expected.set(true);
        state.native.set(true);
        closed();
        assert!(!state.expected.get());
        assert!(!state.native.get());
        assert_eq!(published.get(), 1);
    }
}
