use super::*;

impl<R: NativeRuntime> Engine<R> {
    pub(crate) fn start_timer(&mut self, timer: TimerSpec) -> Result<(), EngineError> {
        let node = self
            .arena
            .get(timer.owner)
            .ok_or(EngineError::InvalidNode(timer.owner))?;
        if !matches!(
            node.mounted.as_ref().map(|mounted| &mounted.kind),
            Some(MountedKind::Component { .. } | MountedKind::FadeTransition { .. })
        ) {
            return Err(EngineError::TimerOwnerNotComponent(timer.owner));
        }
        self.pending.push(Command::StartTimer(timer));
        Ok(())
    }

    pub(crate) fn stop_timer(&mut self, owner: NodeId, slot: u32, revision: u64) {
        self.pending.push(Command::StopTimer {
            owner,
            slot,
            revision,
        });
    }

    #[cfg(feature = "canvas")]
    pub(crate) fn drain_committed_canvas_frames(&mut self, target: &mut Vec<NodeId>) {
        for id in self.committed_canvas_frames.drain(..) {
            if !target.contains(&id) {
                target.push(id);
            }
        }
    }

    pub fn process_events<F>(
        &mut self,
        rows: &mut F,
    ) -> Result<(Vec<NativeEvent>, bool), EngineError>
    where
        F: RowFactory<R> + ?Sized,
    {
        let events = self.runtime.drain_events();
        let had_events = !events.is_empty();
        let recycled: BTreeSet<_> = events
            .iter()
            .filter_map(|event| match event {
                NativeEvent::Recycle { host, index, lease } => Some((*host, *index, *lease)),
                _ => None,
            })
            .collect();
        let mut application = Vec::new();
        for event in events {
            let target = event.target();
            if !self.arena.contains(target) {
                continue;
            }
            self.ensure_event_compatible(&event)?;
            if self.restore_display_feedback(&event)? {
                self.commit()?;
                continue;
            }
            match event.class() {
                NativeEventClass::ControlledFeedback
                | NativeEventClass::ClosureFailureSynchronization => {
                    self.sync_mounted_state(&event);
                }
                NativeEventClass::NotificationOnly
                | NativeEventClass::CancelableRequest
                | NativeEventClass::InternalRuntime => {}
            }
            let result = match event {
                #[cfg(feature = "canvas")]
                NativeEvent::CanvasImageLayout {
                    target,
                    width,
                    height,
                    scale,
                } => {
                    self.pending.push(Command::ApplyCanvasImageLayout {
                        target,
                        width,
                        height,
                        scale,
                    });
                    Ok(())
                }
                #[cfg(feature = "canvas")]
                NativeEvent::CanvasImageFrame { target } => {
                    self.pending.push(Command::RunCanvasImageFrame { target });
                    Ok(())
                }
                #[cfg(feature = "canvas")]
                NativeEvent::CanvasLayout {
                    target,
                    width,
                    height,
                    scale_x,
                    scale_y,
                } => {
                    self.pending.push(Command::ApplyCanvasLayout {
                        target,
                        width,
                        height,
                        scale_x,
                        scale_y,
                    });
                    Ok(())
                }
                #[cfg(feature = "canvas")]
                NativeEvent::CanvasFrame { target } => {
                    self.pending.push(Command::RunCanvasFrame { target });
                    Ok(())
                }
                #[cfg(feature = "canvas")]
                NativeEvent::SwapChainHostLayout { target, layout } => {
                    self.pending
                        .push(Command::ApplySwapChainHostLayout { target, layout });
                    Ok(())
                }
                #[cfg(feature = "canvas")]
                NativeEvent::SwapChainHostFrame { target } => {
                    self.pending.push(Command::RunSwapChainHostFrame { target });
                    Ok(())
                }
                NativeEvent::CompositionLayout {
                    target,
                    width,
                    height,
                    rasterization_scale,
                } => {
                    self.pending.push(Command::ApplyCompositionLayout {
                        target,
                        width,
                        height,
                        rasterization_scale,
                    });
                    Ok(())
                }
                #[cfg(feature = "webview")]
                NativeEvent::WebViewInitializationReady { target, revision } => {
                    self.pending
                        .push(Command::FinishWebViewInitialization { target, revision });
                    Ok(())
                }
                NativeEvent::TimerFired { .. }
                | NativeEvent::WindowCloseRequested { .. }
                | NativeEvent::WindowSizeChanged { .. }
                | NativeEvent::WindowColorSchemeChanged { .. }
                | NativeEvent::Click { .. }
                | NativeEvent::MenuItemClick { .. }
                | NativeEvent::TextChanged { .. }
                | NativeEvent::PasswordChanged { .. }
                | NativeEvent::Toggled { .. }
                | NativeEvent::ValueChanged { .. }
                | NativeEvent::OptionalValueChanged { .. }
                | NativeEvent::ColorChanged { .. }
                | NativeEvent::DateChanged { .. }
                | NativeEvent::TimeChanged { .. }
                | NativeEvent::DatesChanged { .. }
                | NativeEvent::KeyboardAcceleratorInvoked { .. }
                | NativeEvent::Pointer { .. }
                | NativeEvent::Tapped { .. }
                | NativeEvent::RightTapped { .. }
                | NativeEvent::Drop { .. }
                | NativeEvent::Scroll { .. }
                | NativeEvent::PaneClosed { .. }
                | NativeEvent::NavigationPaneOpenChanged { .. }
                | NativeEvent::NavigationDisplayModeChanged { .. }
                | NativeEvent::ExpandedChanged { .. }
                | NativeEvent::TreeNodeExpandedChanged { .. }
                | NativeEvent::TeachingTipClosed { .. }
                | NativeEvent::TeachingTipAction { .. }
                | NativeEvent::InfoBarCloseRequested { .. }
                | NativeEvent::TitleBarBackRequested { .. }
                | NativeEvent::TitleBarPaneRequested { .. }
                | NativeEvent::FlyoutOpened { .. }
                | NativeEvent::FlyoutClosed { .. }
                | NativeEvent::ContentDialogClosed { .. }
                | NativeEvent::ImageLoad { .. }
                | NativeEvent::ItemInvoked { .. }
                | NativeEvent::QuerySubmitted { .. }
                | NativeEvent::SelectionChanged { .. }
                | NativeEvent::IndexChanged { .. }
                | NativeEvent::TabCloseRequested { .. }
                | NativeEvent::AddTabButtonClick { .. }
                | NativeEvent::TabsReordered { .. }
                | NativeEvent::ItemsReordered { .. } => {
                    application.push(event);
                    continue;
                }
                #[cfg(feature = "webview")]
                NativeEvent::WebViewCreated { .. }
                | NativeEvent::WebViewNavigationCompleted { .. } => {
                    application.push(event);
                    continue;
                }
                NativeEvent::SelectedKeyChanged { .. } => {
                    application.push(event);
                    continue;
                }
                NativeEvent::Realize { host, index, lease } => {
                    if recycled.contains(&(host, index, lease)) {
                        continue;
                    }

                    self.realize(host, index, lease, rows)
                }
                NativeEvent::DeferredReady {
                    target,
                    revision,
                    action,
                } => {
                    self.pending.push(Command::RunDeferred {
                        target,
                        window: self.owning_window(target),
                        revision,
                        action,
                    });
                    Ok(())
                }
                NativeEvent::Recycle { host, index, lease } => self.recycle(host, index, lease),
            };
            result?;
            self.commit()?;
        }
        Ok((application, had_events))
    }

    fn restore_display_feedback(&mut self, event: &NativeEvent) -> Result<bool, EngineError> {
        let NativeEvent::TreeNodeExpandedChanged { target, .. } = event else {
            return Ok(false);
        };
        let Some(MountedKind::TreeView(props)) = self
            .arena
            .get(*target)
            .and_then(|node| node.mounted.as_ref())
            .map(|mounted| &mounted.kind)
        else {
            return Ok(false);
        };
        if props.on_expanded_changed.is_some() {
            return Ok(false);
        }
        let nodes = Rc::clone(&props.nodes);
        self.queue_control_update(
            *target,
            ControlUpdate::TreeView(Box::new(TreeViewUpdate::Nodes(nodes))),
        )?;
        Ok(true)
    }

    fn sync_mounted_state(&mut self, event: &NativeEvent) {
        if let NativeEvent::TabsReordered { target, keys } = event {
            self.sync_keyed_children(*target, keys);
            return;
        }
        let Some(mounted) = self
            .arena
            .get_mut(event.target())
            .and_then(|node| node.mounted.as_mut())
        else {
            return;
        };
        match (event, &mut mounted.kind) {
            (NativeEvent::TextChanged { value, .. }, MountedKind::TextBox(props))
                if props.on_change.is_some() =>
            {
                props.text.clone_from(value);
            }
            (NativeEvent::TextChanged { value, .. }, MountedKind::RichEditBox(props))
                if props.on_change.is_some() =>
            {
                props.text.clone_from(value);
            }
            (NativeEvent::TextChanged { value, .. }, MountedKind::AutoSuggestBox(props))
                if props.on_text_changed.is_some() =>
            {
                props.text.clone_from(value);
            }
            (NativeEvent::PasswordChanged { value, .. }, MountedKind::PasswordBox(props))
                if props.on_change.is_some() =>
            {
                props.password.clone_from(value);
            }
            (NativeEvent::Toggled { value, .. }, MountedKind::CheckBox(props))
                if props.on_toggle.is_some() =>
            {
                props.checked = *value;
            }
            (NativeEvent::Toggled { value, .. }, MountedKind::RadioButton(props))
                if props.on_toggle.is_some() =>
            {
                props.checked = *value;
            }
            (NativeEvent::Toggled { value, .. }, MountedKind::ToggleButton(props))
                if props.on_toggle.is_some() =>
            {
                props.checked = *value;
            }
            (NativeEvent::Toggled { value, .. }, MountedKind::ToggleSwitch(props))
                if props.on_toggle.is_some() =>
            {
                props.on = *value;
            }
            (NativeEvent::Toggled { value, .. }, MountedKind::AppBarToggleButton(props)) => {
                props.checked = *value;
            }
            (NativeEvent::ValueChanged { value, .. }, MountedKind::Slider(props))
                if props.on_change.is_some() =>
            {
                props.value = *value;
            }
            (NativeEvent::OptionalValueChanged { value, .. }, MountedKind::NumberBox(props))
                if props.on_change.is_some() =>
            {
                props.value = *value;
            }
            (
                NativeEvent::OptionalValueChanged { value, .. },
                MountedKind::RatingControl(props),
            ) if props.on_change.is_some() => {
                props.value = *value;
            }
            (NativeEvent::ColorChanged { value, .. }, MountedKind::ColorPicker(props))
                if props.on_change.is_some() =>
            {
                props.color = *value;
            }
            (NativeEvent::DateChanged { value, .. }, MountedKind::DatePicker(props))
                if props.on_change.is_some() =>
            {
                props.date = *value;
            }
            (NativeEvent::DateChanged { value, .. }, MountedKind::CalendarDatePicker(props))
                if props.on_change.is_some() =>
            {
                props.date = *value;
            }
            (NativeEvent::TimeChanged { value, .. }, MountedKind::TimePicker(props))
                if props.on_change.is_some() =>
            {
                props.time = *value;
            }
            (NativeEvent::DatesChanged { value, .. }, MountedKind::CalendarView(props))
                if props.on_change.is_some() =>
            {
                props.selected_dates = value.clone().into();
            }
            (NativeEvent::PaneClosed { .. }, MountedKind::SplitView(props))
                if props.on_pane_closed.is_some() =>
            {
                props.is_pane_open = false;
            }
            (
                NativeEvent::NavigationPaneOpenChanged { open, .. },
                MountedKind::NavigationView(props),
            ) if props.on_pane_open_changed.is_some() => {
                props.pane_open = *open;
            }
            (NativeEvent::ExpandedChanged { expanded, .. }, MountedKind::Expander(props))
                if props.on_expanded_changed.is_some() =>
            {
                props.expanded = *expanded;
            }
            (
                NativeEvent::TreeNodeExpandedChanged { key, expanded, .. },
                MountedKind::TreeView(props),
            ) if props.on_expanded_changed.is_some() => {
                assert!(
                    sync_tree_node_expansion(&mut props.nodes, *key, *expanded),
                    "native TreeView expansion key is unknown"
                );
            }
            (NativeEvent::TeachingTipClosed { .. }, MountedKind::TeachingTip(props)) => {
                props.open = false;
            }
            (NativeEvent::ContentDialogClosed { .. }, MountedKind::ContentDialog(props)) => {
                props.open = false;
            }
            (
                NativeEvent::SelectionChanged { selection, .. },
                MountedKind::VirtualCollection(props),
            ) if props.on_selection_changed.is_some() => {
                props.selection.clone_from(selection);
            }
            (NativeEvent::SelectionChanged { selection, .. }, MountedKind::ListBox(props))
                if props.on_selection_changed.is_some() =>
            {
                props.selection.clone_from(selection);
            }
            (NativeEvent::IndexChanged { index, .. }, MountedKind::FlipView(props))
                if props.on_selection_changed.is_some() =>
            {
                props.selected_index = *index;
            }
            (NativeEvent::IndexChanged { index, .. }, MountedKind::TabView(props))
                if props.on_selection_changed.is_some() =>
            {
                props.selected_index = *index;
            }
            (NativeEvent::IndexChanged { index, .. }, MountedKind::Pivot(props))
                if props.on_selection_changed.is_some() =>
            {
                props.selected_index = *index;
            }
            (NativeEvent::SelectedKeyChanged { key, .. }, MountedKind::SelectorBar(props))
                if props.on_selection_changed.is_some() =>
            {
                props.selected_key = *key;
            }
            (NativeEvent::SelectedKeyChanged { key, .. }, MountedKind::ComboBox(props))
                if props.on_selection_changed.is_some() =>
            {
                props.selected_key = *key;
            }
            (NativeEvent::SelectedKeyChanged { key, .. }, MountedKind::RadioButtons(props))
                if props.on_selection_changed.is_some() =>
            {
                props.selected_key = *key;
            }
            (NativeEvent::SelectedKeyChanged { key, .. }, MountedKind::NavigationView(props))
                if props.on_selection_changed.is_some() =>
            {
                props.selected_key = *key;
            }
            (NativeEvent::ItemsReordered { keys, .. }, MountedKind::VirtualCollection(props))
                if props.on_items_reordered.is_some() =>
            {
                props.items = crate::element::props::VirtualCollectionItems::Keyed(
                    crate::element::VirtualItemKeys::new(keys.clone()),
                );
            }
            _ => {}
        }
    }

    fn sync_keyed_children(&mut self, parent: NodeId, keys: &[u64]) {
        let children = self.arena.get(parent).unwrap().children.clone();
        assert_eq!(
            children.len(),
            keys.len(),
            "native keyed reorder changed the child count"
        );
        let reordered: Vec<NodeId> = keys
            .iter()
            .map(|key| {
                children
                    .iter()
                    .copied()
                    .find(|child| {
                        self.arena
                            .get(*child)
                            .and_then(|node| node.mounted.as_ref())
                            .and_then(|mounted| mounted.key)
                            == Some(*key)
                    })
                    .unwrap()
            })
            .collect();
        self.arena.get_mut(parent).unwrap().children = reordered;
    }

    fn ensure_event_compatible(&self, event: &NativeEvent) -> Result<(), EngineError> {
        let target = event.target();
        let node = self
            .arena
            .get(target)
            .ok_or(EngineError::InvalidNode(target))?;
        let compatible = native_event_compatible(event, node);
        if compatible {
            Ok(())
        } else {
            Err(EngineError::IncompatibleEvent {
                target,
                event: event.name(),
            })
        }
    }

    fn owning_window(&self, mut id: NodeId) -> Option<NodeId> {
        loop {
            let node = self.arena.get(id)?;
            if matches!(node.kind, NodeKind::Window) {
                return Some(id);
            }
            id = node.parent?;
        }
    }
}

fn sync_tree_node_expansion(
    nodes: &mut Rc<[crate::element::TreeNode]>,
    key: u64,
    expanded: bool,
) -> bool {
    for node in Rc::make_mut(nodes) {
        if node.key == key {
            node.expanded = expanded;
            return true;
        }
        if sync_tree_node_expansion(&mut node.children, key, expanded) {
            return true;
        }
    }
    false
}

fn native_event_compatible(event: &NativeEvent, node: &Node) -> bool {
    let mounted = node.mounted.as_ref().map(|mounted| &mounted.kind);
    match event.compatibility() {
        NativeEventCompatibility::TimerOwner => {
            matches!(
                mounted,
                Some(MountedKind::Component { .. } | MountedKind::FadeTransition { .. })
            )
        }
        NativeEventCompatibility::Window => {
            matches!(mounted, Some(MountedKind::Window(_)))
        }
        NativeEventCompatibility::Click => matches!(
            mounted,
            Some(
                MountedKind::Button(_)
                    | MountedKind::ButtonEvent(_)
                    | MountedKind::SplitButton(_)
                    | MountedKind::SplitButtonEvent(_)
                    | MountedKind::HyperlinkButton(_)
                    | MountedKind::RepeatButton(_)
                    | MountedKind::ToggleButton(_)
                    | MountedKind::AppBarButton(_)
            )
        ),
        NativeEventCompatibility::Menu => {
            matches!(
                mounted,
                Some(MountedKind::MenuBar(_) | MountedKind::MenuFlyout(_))
            )
        }
        NativeEventCompatibility::Text => matches!(
            mounted,
            Some(
                MountedKind::TextBox(_)
                    | MountedKind::RichEditBox(_)
                    | MountedKind::AutoSuggestBox(_)
            )
        ),
        NativeEventCompatibility::Password => {
            matches!(mounted, Some(MountedKind::PasswordBox(_)))
        }
        NativeEventCompatibility::Toggle => matches!(
            mounted,
            Some(
                MountedKind::CheckBox(_)
                    | MountedKind::RadioButton(_)
                    | MountedKind::ToggleButton(_)
                    | MountedKind::ToggleSwitch(_)
                    | MountedKind::AppBarToggleButton(_)
            )
        ),
        NativeEventCompatibility::Slider => {
            matches!(mounted, Some(MountedKind::Slider(_)))
        }
        NativeEventCompatibility::OptionalValue => matches!(
            mounted,
            Some(MountedKind::NumberBox(_) | MountedKind::RatingControl(_))
        ),
        NativeEventCompatibility::Color => {
            matches!(mounted, Some(MountedKind::ColorPicker(_)))
        }
        NativeEventCompatibility::Date => matches!(
            mounted,
            Some(MountedKind::DatePicker(_) | MountedKind::CalendarDatePicker(_))
        ),
        NativeEventCompatibility::Time => {
            matches!(mounted, Some(MountedKind::TimePicker(_)))
        }
        NativeEventCompatibility::Dates => {
            matches!(mounted, Some(MountedKind::CalendarView(_)))
        }
        NativeEventCompatibility::Framework => {
            mounted.is_some_and(|kind| kind.framework_props().is_some())
        }
        NativeEventCompatibility::Scroll => matches!(
            mounted,
            Some(MountedKind::ScrollViewer(_) | MountedKind::ScrollView(_))
        ),
        NativeEventCompatibility::SplitView => {
            matches!(mounted, Some(MountedKind::SplitView(_)))
        }
        NativeEventCompatibility::NavigationView => {
            matches!(mounted, Some(MountedKind::NavigationView(_)))
        }
        NativeEventCompatibility::Expander => {
            matches!(mounted, Some(MountedKind::Expander(_)))
        }
        NativeEventCompatibility::TreeView => {
            matches!(mounted, Some(MountedKind::TreeView(_)))
        }
        NativeEventCompatibility::TeachingTip => {
            matches!(mounted, Some(MountedKind::TeachingTip(_)))
        }
        NativeEventCompatibility::InfoBar => {
            matches!(mounted, Some(MountedKind::InfoBar(_)))
        }
        NativeEventCompatibility::TitleBar => {
            matches!(mounted, Some(MountedKind::TitleBar(_)))
        }
        NativeEventCompatibility::Flyout => {
            matches!(
                mounted,
                Some(
                    MountedKind::Flyout(_)
                        | MountedKind::MenuFlyout(_)
                        | MountedKind::CommandBarFlyout(_)
                )
            )
        }
        NativeEventCompatibility::ContentDialog => {
            matches!(mounted, Some(MountedKind::ContentDialog(_)))
        }
        NativeEventCompatibility::Image => {
            matches!(mounted, Some(MountedKind::Image { .. }))
        }
        NativeEventCompatibility::CompositionHost => {
            matches!(mounted, Some(MountedKind::CompositionHost(_)))
        }
        #[cfg(feature = "webview")]
        NativeEventCompatibility::WebViewHost => {
            matches!(mounted, Some(MountedKind::WebViewHost(_)))
        }
        #[cfg(feature = "canvas")]
        NativeEventCompatibility::CanvasImage => {
            matches!(mounted, Some(MountedKind::CanvasImage(_)))
        }
        #[cfg(feature = "canvas")]
        NativeEventCompatibility::SwapChainCanvas => {
            matches!(mounted, Some(MountedKind::SwapChainCanvas(_)))
        }
        #[cfg(feature = "canvas")]
        NativeEventCompatibility::SwapChainHost => {
            matches!(mounted, Some(MountedKind::SwapChainHost(_)))
        }
        NativeEventCompatibility::DeferredContentDialog => {
            node.native_kind == Some(NativeKind::ContentDialog)
        }
        NativeEventCompatibility::DeferredTeachingTip => {
            node.native_kind == Some(NativeKind::TeachingTip)
        }
        NativeEventCompatibility::DeferredRadioButtons => {
            node.native_kind == Some(NativeKind::RadioButtons)
        }
        NativeEventCompatibility::ItemInvocation => {
            matches!(
                mounted,
                Some(
                    MountedKind::VirtualCollection(_)
                        | MountedKind::BreadcrumbBar(_)
                        | MountedKind::AutoSuggestBox(_)
                        | MountedKind::TreeView(_)
                )
            )
        }
        NativeEventCompatibility::AutoSuggestBox => {
            matches!(mounted, Some(MountedKind::AutoSuggestBox(_)))
        }
        NativeEventCompatibility::CollectionSelection => matches!(
            mounted,
            Some(MountedKind::VirtualCollection(_) | MountedKind::ListBox(_))
        ),
        NativeEventCompatibility::IndexSelection => matches!(
            mounted,
            Some(MountedKind::FlipView(_) | MountedKind::TabView(_) | MountedKind::Pivot(_))
        ),
        NativeEventCompatibility::TabView => matches!(mounted, Some(MountedKind::TabView(_))),
        NativeEventCompatibility::VirtualCollection => {
            matches!(mounted, Some(MountedKind::VirtualCollection(_)))
        }
        NativeEventCompatibility::SingleSelection => {
            matches!(
                mounted,
                Some(
                    MountedKind::SelectorBar(_)
                        | MountedKind::NavigationView(_)
                        | MountedKind::ComboBox(_)
                        | MountedKind::RadioButtons(_)
                )
            )
        }
        NativeEventCompatibility::VirtualHost => {
            matches!(node.kind, NodeKind::VirtualHost { .. })
        }
    }
}
