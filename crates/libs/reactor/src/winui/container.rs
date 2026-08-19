use super::*;

pub(super) struct SplitViewState {
    expected_open: Cell<bool>,
    feedback_enabled: Cell<bool>,
    suppressing: Cell<bool>,
}

struct ExpanderCallbackState {
    expected: Cell<bool>,
    feedback_enabled: Cell<bool>,
    suppressing: Cell<bool>,
}

pub(super) struct ExpanderState {
    pub(super) value: bindings::Expander,
    callback: Rc<ExpanderCallbackState>,
    _expanded_changed: [windows_core::EventRevoker; 2],
}

impl WinUiRuntime {
    pub(super) fn create_scroll_viewer(&self) -> WindowsResult<Handle> {
        Ok(Handle::ScrollViewer {
            value: bindings::ScrollViewer::new()?,
            view_changed: None,
        })
    }

    pub(super) fn create_scroll_view(&self) -> WindowsResult<Handle> {
        Ok(Handle::ScrollView {
            value: bindings::ScrollView::new()?,
            view_changed: None,
        })
    }

    pub(super) fn create_split_view(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::SplitView::new()?;
        let state = Rc::new(SplitViewState {
            expected_open: Cell::new(false),
            feedback_enabled: Cell::new(false),
            suppressing: Cell::new(false),
        });
        let object: bindings::DependencyObject = value.cast()?;
        let callback_state = Rc::clone(&state);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let callback =
            bindings::DependencyPropertyChangedCallback::new(move |sender, _property| {
                if callback_state.suppressing.get() {
                    return;
                }
                let control: bindings::SplitView = sender.as_ref().unwrap().cast().unwrap();
                let open = control.IsPaneOpen().unwrap();
                if callback_state.expected_open.get() == open {
                    return;
                }
                if open || !callback_state.feedback_enabled.get() {
                    callback_state.suppressing.set(true);
                    control
                        .SetIsPaneOpen(callback_state.expected_open.get())
                        .unwrap();
                    callback_state.suppressing.set(false);
                    return;
                }
                callback_state.expected_open.set(false);
                events
                    .borrow_mut()
                    .push_back(NativeEvent::PaneClosed { target: id });
                if let Some(wake) = waker.borrow().as_ref() {
                    wake();
                }
            });
        let property = bindings::SplitView::IsPaneOpenProperty()?;
        object.RegisterPropertyChangedCallback(&property, &callback)?;
        Ok(Handle::SplitView { value, state })
    }

    pub(super) fn create_expander(&self, id: NodeId) -> WindowsResult<Handle> {
        let value = bindings::Expander::new()?;
        let callback = Rc::new(ExpanderCallbackState {
            expected: Cell::new(false),
            feedback_enabled: Cell::new(false),
            suppressing: Cell::new(false),
        });
        let expanding_control = value.clone();
        let expanding_state = Rc::clone(&callback);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let expanding = value.Expanding(move |_sender, _args| {
            if expanding_state.suppressing.get() || expanding_state.expected.get() {
                return;
            }
            if !expanding_state.feedback_enabled.get() {
                expanding_state.suppressing.set(true);
                expanding_control.SetIsExpanded(false).unwrap();
                expanding_state.suppressing.set(false);
                return;
            }
            expanding_state.expected.set(true);
            queue_expanded_event(&events, &waker, id, true);
        })?;
        let collapsed_control = value.clone();
        let collapsed_state = Rc::clone(&callback);
        let events = Rc::clone(&self.events);
        let waker = Rc::clone(&self.waker);
        let collapsed = value.Collapsed(move |_sender, _args| {
            if collapsed_state.suppressing.get() || !collapsed_state.expected.get() {
                return;
            }
            if !collapsed_state.feedback_enabled.get() {
                collapsed_state.suppressing.set(true);
                collapsed_control.SetIsExpanded(true).unwrap();
                collapsed_state.suppressing.set(false);
                return;
            }
            collapsed_state.expected.set(false);
            queue_expanded_event(&events, &waker, id, false);
        })?;
        Ok(Handle::Expander(Box::new(ExpanderState {
            value,
            callback,
            _expanded_changed: [expanding, collapsed],
        })))
    }

    pub(super) fn apply_expander_update(
        &mut self,
        id: NodeId,
        update: ExpanderUpdate,
    ) -> WindowsResult<()> {
        match update {
            ExpanderUpdate::Expanded(expanded) => {
                let Handle::Expander(state) = &self.node(id)?.handle else {
                    panic!("expanded target is not an Expander");
                };
                state.callback.expected.set(expanded);
                state.callback.suppressing.set(true);
                let result = state.value.SetIsExpanded(expanded);
                state.callback.suppressing.set(false);
                result
            }
            ExpanderUpdate::ExpandedChanged(enabled) => {
                let Handle::Expander(state) = &self.node(id)?.handle else {
                    panic!("expanded-changed target is not an Expander");
                };
                state.callback.feedback_enabled.set(enabled);
                Ok(())
            }
        }
    }

    pub(super) fn apply_split_view_update(
        &mut self,
        id: NodeId,
        update: SplitViewUpdate,
    ) -> WindowsResult<()> {
        match update {
            SplitViewUpdate::DisplayMode(value) => {
                let Handle::SplitView { value: control, .. } = &self.node(id)?.handle else {
                    panic!("display mode target is not a SplitView");
                };
                control.SetDisplayMode(native_split_view_display_mode(value))
            }
            SplitViewUpdate::IsPaneOpen(open) => {
                let Handle::SplitView { value, state, .. } = &self.node(id)?.handle else {
                    panic!("open-state target is not a SplitView");
                };
                state.expected_open.set(open);
                state.suppressing.set(true);
                let result = value.SetIsPaneOpen(open);
                state.suppressing.set(false);
                result
            }
            SplitViewUpdate::OpenPaneLength(length) => {
                let Handle::SplitView { value, .. } = &self.node(id)?.handle else {
                    panic!("pane length target is not a SplitView");
                };
                value.SetOpenPaneLength(length)
            }
            SplitViewUpdate::CompactPaneLength(length) => {
                let Handle::SplitView { value, .. } = &self.node(id)?.handle else {
                    panic!("pane length target is not a SplitView");
                };
                value.SetCompactPaneLength(length)
            }
            SplitViewUpdate::PaneClosed(enabled) => {
                let Handle::SplitView { state, .. } = &self.node(id)?.handle else {
                    panic!("pane-closed target is not a SplitView");
                };
                state.feedback_enabled.set(enabled);
                Ok(())
            }
        }
    }

    pub(super) fn apply_scroll_viewer_update(
        &mut self,
        id: NodeId,
        update: ScrollViewerUpdate,
    ) -> WindowsResult<()> {
        match update {
            ScrollViewerUpdate::HorizontalScrollBarVisibility(value) => {
                let Handle::ScrollViewer { value: control, .. } = &self.node(id)?.handle else {
                    panic!("scroll bar target is not a ScrollViewer");
                };
                control.SetHorizontalScrollBarVisibility(native_scroll_bar_visibility(value))
            }
            ScrollViewerUpdate::VerticalScrollBarVisibility(value) => {
                let Handle::ScrollViewer { value: control, .. } = &self.node(id)?.handle else {
                    panic!("scroll bar target is not a ScrollViewer");
                };
                control.SetVerticalScrollBarVisibility(native_scroll_bar_visibility(value))
            }
            ScrollViewerUpdate::ViewChanged(enabled) => {
                let previous = match &mut self.node_mut(id)?.handle {
                    Handle::ScrollViewer { view_changed, .. } => view_changed.take(),
                    _ => {
                        panic!("view-changed target is not a ScrollViewer");
                    }
                };
                drop(previous);
                if !enabled {
                    return Ok(());
                }
                let control = match &self.node(id)?.handle {
                    Handle::ScrollViewer { value, .. } => value.clone(),
                    _ => unreachable!(),
                };
                let callback_control = control.clone();
                let events = Rc::clone(&self.events);
                let waker = Rc::clone(&self.waker);
                let revoker = control.ViewChanged(move |_sender, args| {
                    let activity = if args.as_ref().unwrap().IsIntermediate().unwrap() {
                        ScrollActivity::Intermediate
                    } else {
                        ScrollActivity::Idle
                    };
                    let event = scroll_viewer_event(&callback_control, activity);
                    queue_scroll_event(&events, &waker, id, event);
                })?;
                let Handle::ScrollViewer { view_changed, .. } = &mut self.node_mut(id)?.handle
                else {
                    unreachable!()
                };
                *view_changed = Some(revoker);
                Ok(())
            }
        }
    }

    pub(super) fn apply_scroll_view_update(
        &mut self,
        id: NodeId,
        update: ScrollViewUpdate,
    ) -> WindowsResult<()> {
        match update {
            ScrollViewUpdate::HorizontalScrollBarVisibility(value) => {
                let Handle::ScrollView { value: control, .. } = &self.node(id)?.handle else {
                    panic!("scroll bar target is not a ScrollView");
                };
                control.SetHorizontalScrollBarVisibility(native_scroll_view_bar_visibility(value))
            }
            ScrollViewUpdate::VerticalScrollBarVisibility(value) => {
                let Handle::ScrollView { value: control, .. } = &self.node(id)?.handle else {
                    panic!("scroll bar target is not a ScrollView");
                };
                control.SetVerticalScrollBarVisibility(native_scroll_view_bar_visibility(value))
            }
            ScrollViewUpdate::ContentOrientation(value) => {
                let Handle::ScrollView { value: control, .. } = &self.node(id)?.handle else {
                    panic!("orientation target is not a ScrollView");
                };
                control.SetContentOrientation(native_scroll_orientation(value))
            }
            ScrollViewUpdate::ViewChanged(enabled) => {
                let previous = match &mut self.node_mut(id)?.handle {
                    Handle::ScrollView { view_changed, .. } => view_changed.take(),
                    _ => {
                        panic!("view-changed target is not a ScrollView");
                    }
                };
                drop(previous);
                if !enabled {
                    return Ok(());
                }
                let control = match &self.node(id)?.handle {
                    Handle::ScrollView { value, .. } => value.clone(),
                    _ => unreachable!(),
                };
                let callback_control = control.clone();
                let events = Rc::clone(&self.events);
                let waker = Rc::clone(&self.waker);
                let revoker = control.ViewChanged(move |_sender, _args| {
                    let event = scroll_view_event(&callback_control);
                    queue_scroll_event(&events, &waker, id, event);
                })?;
                let Handle::ScrollView { view_changed, .. } = &mut self.node_mut(id)?.handle else {
                    unreachable!()
                };
                *view_changed = Some(revoker);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
#[path = "../../testing/private/winui/container_access.rs"]
pub(super) mod tests;
