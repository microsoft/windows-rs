use super::*;

impl Runtime<WinUiAdapter> {
    pub fn set_native_event_waker(&mut self, waker: impl Fn() + 'static) {
        let waker = Rc::new(waker);
        let native = Rc::clone(&waker);
        self.set_native_event_wakers(move || native(), move || waker());
    }
}

impl ComponentHost<WinUiAdapter> {
    pub fn set_native_event_waker(&mut self, waker: impl Fn() + 'static) {
        self.runtime_mut_internal().set_native_event_waker(waker);
    }
}

impl WinUiAdapter {
    pub fn open_window_with_policy(
        &self,
        root: ObjectId,
        policy: &WindowPolicy,
    ) -> Result<NativeWindow, WinUiError> {
        let window = self.create_window_with_policy(root, policy)?;
        window.activate()?;
        Ok(window)
    }

    pub fn create_window(&self, root: ObjectId) -> Result<NativeWindow, WinUiError> {
        self.create_window_with_policy(root, &WindowPolicy::new())
    }

    pub fn open_window(&self, root: ObjectId) -> Result<NativeWindow, WinUiError> {
        let window = self.create_window(root)?;
        window.activate()?;
        Ok(window)
    }

    pub fn validate_graph(&self, graph: &RetainedGraph) -> Result<(), WinUiError> {
        let Some(root) = graph.root() else {
            return Ok(());
        };
        let mut pending = vec![root];
        let mut visited = HashSet::new();
        while let Some(object) = pending.pop() {
            if !visited.insert(object) {
                continue;
            }
            let kind = graph
                .kind(object)
                .ok_or(WinUiError::MissingObject(object))?;
            if self.kind(object)? != kind {
                return Err(WinUiError::StateMismatch(object));
            }
            if let Handle::Generated(GeneratedHandle::TextBlock(value)) = self
                .handles
                .get(&object)
                .ok_or(WinUiError::MissingObject(object))?
            {
                let expected = graph
                    .properties(object)
                    .unwrap_or_default()
                    .iter()
                    .find_map(|property| match (&property.id, &property.value) {
                        (PropertyId::Text, PropertyValue::String(value)) => Some(value.as_ref()),
                        _ => None,
                    })
                    .unwrap_or_default();
                if value.Text()? != expected {
                    return Err(WinUiError::StateMismatch(object));
                }
            }
            if let Handle::TextBox(value) = self
                .handles
                .get(&object)
                .ok_or(WinUiError::MissingObject(object))?
            {
                let expected = graph
                    .properties(object)
                    .unwrap_or_default()
                    .iter()
                    .find_map(|property| match (&property.id, &property.value) {
                        (PropertyId::Text, PropertyValue::String(value)) => Some(value.as_ref()),
                        _ => None,
                    })
                    .unwrap_or_default();
                if value.value.Text()? != expected {
                    return Err(WinUiError::StateMismatch(object));
                }
            }
            if let Handle::Data(value) = self
                .handles
                .get(&object)
                .ok_or(WinUiError::MissingObject(object))?
            {
                let expected = graph
                    .properties(object)
                    .unwrap_or_default()
                    .iter()
                    .find_map(|property| match (&property.id, &property.value) {
                        (PropertyId::Text, PropertyValue::String(value)) => Some(value.as_ref()),
                        _ => None,
                    })
                    .unwrap_or_default();
                let properties = value.cast::<IMap<HSTRING, IInspectable>>()?;
                let actual = properties.Lookup(&self.data_text_key)?;
                let actual = actual
                    .cast::<windows_reference::IReference<HSTRING>>()?
                    .Value()?;
                if actual != expected {
                    return Err(WinUiError::StateMismatch(object));
                }
            }
            if let Some(selection) = selection_contract(kind) {
                let selected = self.read_selected_item(object, selection)?;
                let selected = selected.as_ref().and_then(|selected| {
                    self.event_queue
                        .selection_items
                        .borrow()
                        .iter()
                        .find_map(|item| {
                            (item.owner == object && item.value == *selected).then_some(item.object)
                        })
                });
                for child in selection.relations.iter().flat_map(|relation| {
                    graph
                        .children(object, *relation)
                        .unwrap_or_default()
                        .iter()
                        .copied()
                }) {
                    if graph.kind(child) != Some(selection.item) {
                        continue;
                    }
                    let expected =
                        graph
                            .properties(child)
                            .unwrap_or_default()
                            .iter()
                            .find_map(|property| match property {
                                Property {
                                    id,
                                    value: PropertyValue::Bool(value),
                                } if *id == selection.selected_property => Some(*value),
                                _ => None,
                            });
                    if let Some(expected) = expected
                        && expected != (selected == Some(child))
                    {
                        return Err(WinUiError::StateMismatch(child));
                    }
                }
            }
            match graph.tooltip(object) {
                Some(tooltip) => {
                    if self.tooltips.get(&object).map(|(tooltip, _)| *tooltip) != Some(tooltip)
                        || self.tooltip_owners.get(&tooltip) != Some(&object)
                    {
                        return Err(WinUiError::StateMismatch(object));
                    }
                    pending.push(tooltip);
                }
                None if self.tooltips.contains_key(&object) => {
                    return Err(WinUiError::StateMismatch(object));
                }
                None => {}
            }
            match graph.flyout(object) {
                Some((content, placement)) => {
                    if self
                        .flyouts
                        .get(&object)
                        .map(|(_, current, current_placement)| (*current, *current_placement))
                        != Some((content, placement))
                        || self.flyout_owners.get(&content) != Some(&object)
                    {
                        return Err(WinUiError::StateMismatch(object));
                    }
                    pending.push(content);
                }
                None if self.flyouts.contains_key(&object) => {
                    return Err(WinUiError::StateMismatch(object));
                }
                None => {}
            }
            match graph.menu(object) {
                Some((menu, revision)) => {
                    if self
                        .menus
                        .get(&object)
                        .is_none_or(|current| current.revision != revision || &current.menu != menu)
                    {
                        return Err(WinUiError::StateMismatch(object));
                    }
                }
                None if self.menus.contains_key(&object) => {
                    return Err(WinUiError::StateMismatch(object));
                }
                None => {}
            }
            match graph.command_bar_flyout(object) {
                Some((flyout, revision)) => {
                    if self.command_bar_flyouts.get(&object).is_none_or(|current| {
                        current.revision != revision || &current.flyout != flyout
                    }) {
                        return Err(WinUiError::StateMismatch(object));
                    }
                }
                None if self.command_bar_flyouts.contains_key(&object) => {
                    return Err(WinUiError::StateMismatch(object));
                }
                None => {}
            }
            match graph.content_dialog(object) {
                Some((dialog, open)) => {
                    if self.content_dialogs.get(&object) != Some(&dialog)
                        || self.content_dialog_owners.get(&dialog) != Some(&object)
                        || self
                            .event_queue
                            .content_dialogs
                            .borrow()
                            .dialogs
                            .get(&dialog)
                            .is_none_or(|state| state.desired_open != open)
                    {
                        return Err(WinUiError::StateMismatch(object));
                    }
                    pending.push(dialog);
                }
                None if self.content_dialogs.contains_key(&object) => {
                    return Err(WinUiError::StateMismatch(object));
                }
                None => {}
            }
            for contract in relation_contracts(kind) {
                match contract.cardinality {
                    Cardinality::One => {
                        if let Some(child) = graph.child(object, contract.id) {
                            if self.owners.get(&child) != Some(&(object, contract.id)) {
                                return Err(WinUiError::StateMismatch(object));
                            }
                            pending.push(child);
                        }
                    }
                    Cardinality::Many => {
                        let children = graph.children(object, contract.id).unwrap_or_default();
                        for child in children {
                            if self.owners.get(child) != Some(&(object, contract.id)) {
                                return Err(WinUiError::StateMismatch(object));
                            }
                            pending.push(*child);
                        }
                        if contract.realization != Realization::Container {
                            self.validate_native_order(object, contract, children)?;
                        }
                    }
                }
            }
        }
        let retired = self
            .retirements
            .values()
            .flat_map(|retirement| retirement.nodes.iter().copied())
            .collect::<HashSet<_>>();
        if visited.len() + retired.len() != self.handles.len() {
            return Err(WinUiError::StateMismatch(root));
        }
        Ok(())
    }

    pub fn simulate_text_input(
        &self,
        object: ObjectId,
        text: &str,
        selection_start: i32,
        selection_length: i32,
    ) -> Result<(), WinUiError> {
        let Some(Handle::TextBox(value)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        value.value.SetText(text)?;
        value.value.SetSelectionStart(selection_start)?;
        value.value.SetSelectionLength(selection_length)?;
        Self::dispatch_text_changed(
            &value.event,
            &value.observed_text,
            &self.event_queue,
            object,
            Rc::from(value.value.Text()?),
        );
        Ok(())
    }

    pub fn hide_content_dialog(&self, dialog: ObjectId) -> Result<(), WinUiError> {
        let value = self
            .event_queue
            .content_dialogs
            .borrow()
            .dialogs
            .get(&dialog)
            .ok_or(WinUiError::MissingObject(dialog))?
            .value
            .clone();
        value.Hide().map_err(Into::into)
    }

    pub fn simulate_rich_edit_input(&self, object: ObjectId, text: &str) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::RichEditBox(value))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        value
            .value
            .Document()?
            .SetText(native::TextSetOptions::None, text)?;
        let text = read_rich_edit_text(&value.value).map_err(WinUiError::from)?;
        Self::dispatch_string(
            &value.text_changed,
            &self.event_queue,
            object,
            EventId::TextChanged,
            Some(Observation::SetProperty {
                object,
                property: Property {
                    id: PropertyId::Document,
                    value: PropertyValue::String(Rc::clone(&text)),
                },
            }),
            text,
        );
        Ok(())
    }

    pub fn rich_edit_state(&self, object: ObjectId) -> Result<(String, bool), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::RichEditBox(value))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok((
            read_rich_edit_text(&value.value)
                .map_err(WinUiError::from)?
                .to_string(),
            value.value.IsReadOnly()?,
        ))
    }

    pub fn grid_definition_counts(&self, object: ObjectId) -> Result<(u32, u32), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Grid(value))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok((
            value.RowDefinitions()?.Size()?,
            value.ColumnDefinitions()?.Size()?,
        ))
    }

    pub fn realize_virtual_item(&self, object: ObjectId, index: usize) -> Result<(), WinUiError> {
        let index = i32::try_from(index).map_err(|_| WinUiError::IndexOverflow(index))?;
        match self.handles.get(&object) {
            Some(Handle::Generated(GeneratedHandle::ItemsRepeater(repeater))) => {
                repeater.GetOrCreateElement(index)?;
                Ok(())
            }
            Some(_) => Err(WinUiError::InvalidObject(object)),
            None => Err(WinUiError::MissingObject(object)),
        }
    }

    pub fn virtual_shell_count(&self, object: ObjectId) -> Result<usize, WinUiError> {
        let items = self
            .virtual_items
            .get(&object)
            .ok_or(WinUiError::InvalidObject(object))?;
        Ok(items.shells.pool.borrow().shells.len())
    }

    pub fn queued_event_count(&self) -> usize {
        self.event_queue.events.borrow().len()
    }

    pub fn total_virtual_shell_count(&self) -> usize {
        self.virtual_items
            .values()
            .map(|items| items.shells.pool.borrow().shells.len())
            .sum()
    }

    pub fn virtual_shell_counts(&self) -> (usize, usize) {
        self.virtual_items
            .values()
            .fold((0, 0), |(live, retired), items| {
                let pool = items.shells.pool.borrow();
                (live + pool.shells.len(), retired + pool.retired.len())
            })
    }

    pub fn focus_text_box_deferred(
        &self,
        object: ObjectId,
        completion: impl Fn(bool) + 'static,
    ) -> Result<(), WinUiError> {
        let Some(Handle::TextBox(value)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        let value = value.value.clone();
        let timer = native::DispatcherQueue::GetForCurrentThread()?.CreateTimer()?;
        timer.SetInterval(TimeSpan::from_millis(10))?;
        timer.SetIsRepeating(true)?;
        let state = Rc::new(RefCell::new(
            None::<(
                native::DispatcherQueueTimer,
                windows_core::EventRevoker,
                usize,
            )>,
        ));
        let event_state = Rc::clone(&state);
        let event_timer = timer.clone();
        let revoker = timer.Tick(move |_, _| {
            let focused = value
                .cast::<native::IUIElement>()
                .and_then(|value| value.Focus(native::FocusState::Programmatic))
                .unwrap_or(false);
            let mut state = event_state.borrow_mut();
            let (_, _, attempts) = state.as_mut().unwrap();
            *attempts += 1;
            if focused || *attempts == 100 {
                _ = event_timer.Stop();
                state.take();
                completion(focused);
            }
        });
        *state.borrow_mut() = Some((timer.clone(), revoker?, 0));
        timer.Start()?;
        Ok(())
    }

    pub fn text_box_set_count(&self, object: ObjectId) -> Result<usize, WinUiError> {
        let Some(Handle::TextBox(value)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok(value.set_count.get())
    }

    pub fn retirement_count(&self) -> usize {
        self.retirements.len()
    }

    pub fn owned_physical_children(
        &self,
        parent: ObjectId,
        relation: RelationId,
    ) -> Result<Vec<ObjectId>, WinUiError> {
        let identities = self
            .owners
            .iter()
            .filter(|(_, owner)| **owner == (parent, relation))
            .map(|(object, _)| {
                self.ui_element(*object)
                    .and_then(|value| com_identity(&value))
                    .map(|identity| (identity, *object))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        let values = self.owned_collection(parent, relation)?;
        (0..values.size()?)
            .map(|index| {
                let value = values.get_at(index)?;
                identities
                    .get(&com_identity(&value)?)
                    .copied()
                    .ok_or(WinUiError::StateMismatch(parent))
            })
            .collect()
    }

    pub fn contains_object(&self, object: ObjectId) -> bool {
        self.handles.contains_key(&object)
    }

    pub fn is_attached(&self, object: ObjectId) -> Result<bool, WinUiError> {
        Ok(self.ui_element(object)?.XamlRoot().is_ok())
    }

    pub fn opacity(&self, object: ObjectId) -> Result<f64, WinUiError> {
        self.ui_element(object)?.Opacity().map_err(Into::into)
    }

    pub fn simulate_click(&self, object: ObjectId) -> Result<(), WinUiError> {
        let Some(Handle::Generated(handle)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        let Some(event) = handle.unit_event(EventId::Click) else {
            return Err(WinUiError::InvalidObject(object));
        };
        Self::dispatch_unit(event, &self.event_queue, object, EventId::Click, None);
        Ok(())
    }

    pub fn simulate_is_checked(
        &self,
        object: ObjectId,
        checked: Option<bool>,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(handle)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        match handle {
            GeneratedHandle::CheckBox(value) => {
                value
                    .value
                    .cast::<native::IToggleButton>()?
                    .SetIsChecked(checked)?;
            }
            GeneratedHandle::ToggleButton(value) => {
                value.value.SetIsChecked(checked)?;
            }
            _ => return Err(WinUiError::InvalidObject(object)),
        }
        Ok(())
    }

    pub fn simulate_is_expanded(&self, object: ObjectId, value: bool) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Expander(handle))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        handle.value.SetIsExpanded(value)?;
        Ok(())
    }

    pub fn simulate_is_pane_open(&self, object: ObjectId, value: bool) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::NavigationView(handle))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        handle.value.SetIsPaneOpen(value)?;
        Ok(())
    }

    pub fn simulate_calendar_date(
        &self,
        object: ObjectId,
        value: DateTime,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::CalendarDatePicker(handle))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        handle.value.SetDate(Some(value))?;
        Ok(())
    }

    pub fn simulate_pointer_released(
        &self,
        object: ObjectId,
        value: PointerEventInfo,
    ) -> Result<(), WinUiError> {
        self.simulate_pointer_event(object, EventId::PointerReleased, value)
    }

    pub fn pointer_policy(&self, object: ObjectId) -> Result<(bool, bool), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Border(border))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok((
            border.capture_pointer_on_press.get(),
            border.focus_on_pointer_release.get(),
        ))
    }

    pub fn drop_policy(&self, object: ObjectId) -> Result<Option<Rc<DragDropPolicy>>, WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Border(border))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        let value = border.drop_policy.borrow().clone();
        Ok(value)
    }

    pub fn simulate_drag_kind(
        &self,
        object: ObjectId,
        event: EventId,
        value: DragKind,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Border(border))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        let native_event = match event {
            EventId::DragEnter => &border.drag_enter,
            EventId::DragOver => &border.drag_over,
            _ => return Err(WinUiError::InvalidEvent(object, event)),
        };
        Self::dispatch_drag_kind(native_event, &self.event_queue, object, event, None, value);
        Ok(())
    }

    pub fn simulate_dropped_data(
        &self,
        object: ObjectId,
        value: DroppedData,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Border(border))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Self::queue_dropped_data(
            &border.drop,
            &self.event_queue,
            object,
            EventId::Drop,
            None,
            value,
        );
        Self::schedule_event_wake(&self.event_queue);
        Ok(())
    }

    pub fn simulate_item_tags(
        &self,
        object: ObjectId,
        value: Vec<String>,
    ) -> Result<(), WinUiError> {
        let event = match self.handles.get(&object) {
            Some(Handle::Generated(GeneratedHandle::GridView(grid))) => &grid.drag_items_completed,
            Some(Handle::ListView(list)) => &list.drag_items_completed,
            _ => return Err(WinUiError::InvalidObject(object)),
        };
        Self::dispatch_string_list(
            event,
            &self.event_queue,
            object,
            EventId::DragItemsCompleted,
            None,
            value,
        );
        Ok(())
    }

    pub fn simulate_list_selection(
        &self,
        object: ObjectId,
        value: Option<usize>,
    ) -> Result<(), WinUiError> {
        let Some(Handle::ListView(list)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        let queued = self.event_queue.events.borrow().len();
        list.value
            .cast::<native::ISelector>()?
            .SetSelectedIndex(native_selection_index(value)?)?;
        if self.event_queue.events.borrow().len() == queued {
            Self::dispatch_selection_index(
                &list.selection_changed,
                &self.event_queue,
                object,
                EventId::SelectionChanged,
                Some(Observation::SetProperty {
                    object,
                    property: Property {
                        id: PropertyId::SelectedIndex,
                        value: PropertyValue::SelectionIndex(value),
                    },
                }),
                value,
            );
        }
        Ok(())
    }

    pub fn list_view_state(
        &self,
        object: ObjectId,
    ) -> Result<(Option<usize>, ListViewSelectionMode, bool, bool, bool), WinUiError> {
        let Some(Handle::ListView(list)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        let selector = list.value.cast::<native::ISelector>()?;
        let list_base = list.value.cast::<native::IListViewBase>()?;
        let element = list.value.cast::<native::IUIElement>()?;
        let selection_mode = match list_base.SelectionMode()? {
            native::ListViewSelectionMode::None => ListViewSelectionMode::None,
            native::ListViewSelectionMode::Single => ListViewSelectionMode::Single,
            native::ListViewSelectionMode::Multiple => ListViewSelectionMode::Multiple,
            native::ListViewSelectionMode::Extended => ListViewSelectionMode::Extended,
            _ => unreachable!("unknown ListViewSelectionMode"),
        };
        Ok((
            selection_index(selector.SelectedIndex()?)?,
            selection_mode,
            list_base.CanDragItems()?,
            list_base.CanReorderItems()?,
            element.AllowDrop()?,
        ))
    }

    pub fn tree_view_selection_mode(
        &self,
        object: ObjectId,
    ) -> Result<TreeViewSelectionMode, WinUiError> {
        let Some(Handle::TreeView(tree)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        match tree.value.SelectionMode()? {
            native::TreeViewSelectionMode::None => Ok(TreeViewSelectionMode::None),
            native::TreeViewSelectionMode::Single => Ok(TreeViewSelectionMode::Single),
            native::TreeViewSelectionMode::Multiple => Ok(TreeViewSelectionMode::Multiple),
            _ => unreachable!("unknown TreeViewSelectionMode"),
        }
    }

    pub fn simulate_tree_item_invoked(
        &self,
        object: ObjectId,
        value: impl Into<Rc<str>>,
    ) -> Result<(), WinUiError> {
        let Some(Handle::TreeView(tree)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        Self::dispatch_string(
            &tree.item_invoked,
            &self.event_queue,
            object,
            EventId::ItemInvoked,
            None,
            value.into(),
        );
        Ok(())
    }

    pub fn simulate_string_event(
        &self,
        object: ObjectId,
        event: EventId,
        value: impl Into<Rc<str>>,
    ) -> Result<(), WinUiError> {
        let native_event = match (self.handles.get(&object), event) {
            (
                Some(Handle::Generated(GeneratedHandle::BreadcrumbBar(value))),
                EventId::ItemClicked,
            ) => &value.item_clicked,
            (
                Some(Handle::Generated(GeneratedHandle::AutoSuggestBox(value))),
                EventId::SuggestionChosen,
            ) => &value.suggestion_chosen,
            (
                Some(Handle::Generated(GeneratedHandle::TabView(value))),
                EventId::TabCloseRequested,
            ) => &value.tab_close_requested,
            _ => return Err(WinUiError::InvalidEvent(object, event)),
        };
        Self::dispatch_string(
            native_event,
            &self.event_queue,
            object,
            event,
            None,
            value.into(),
        );
        Ok(())
    }

    pub fn simulate_string_list_event(
        &self,
        object: ObjectId,
        event: EventId,
        value: impl IntoIterator<Item = impl Into<Rc<str>>>,
    ) -> Result<(), WinUiError> {
        let native_event = match (self.handles.get(&object), event) {
            (
                Some(Handle::Generated(GeneratedHandle::TabView(value))),
                EventId::TabItemsChanged,
            ) => &value.tab_items_changed,
            _ => return Err(WinUiError::InvalidEvent(object, event)),
        };
        Self::dispatch_string_list(
            native_event,
            &self.event_queue,
            object,
            event,
            None,
            value
                .into_iter()
                .map(|value| value.into().to_string())
                .collect(),
        );
        Ok(())
    }

    pub fn simulate_color_changed(&self, object: ObjectId, value: Color) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::ColorPicker(control))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        let queued = self.event_queue.events.borrow().len();
        control.value.SetColor(to_native_color(value))?;
        if self.event_queue.events.borrow().len() != queued {
            return Ok(());
        }
        let observation = Observation::SetProperty {
            object,
            property: Property {
                id: PropertyId::Color,
                value: PropertyValue::Color(value),
            },
        };
        if !self
            .event_queue
            .observe(object, EventId::ColorChanged, observation.clone())
        {
            return Ok(());
        }
        Self::dispatch_color(
            &control.color_changed,
            &self.event_queue,
            object,
            EventId::ColorChanged,
            Some(observation),
            value,
        );
        Ok(())
    }

    pub fn navigation_view_flags(&self, object: ObjectId) -> Result<(bool, bool), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::NavigationView(control))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        let control = control.value.cast::<native::INavigationView2>()?;
        Ok((control.IsBackEnabled()?, control.IsPaneVisible()?))
    }

    pub fn navigation_view_pane_header_matches(
        &self,
        object: ObjectId,
        expected: Option<ObjectId>,
    ) -> Result<bool, WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::NavigationView(control))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        let actual = match control
            .value
            .cast::<native::INavigationView2>()?
            .PaneHeader()
        {
            Ok(header) => Some(com_identity(&header)?),
            Err(error) if error.code().is_ok() => None,
            Err(error) => return Err(error.into()),
        };
        let expected = expected
            .map(|object| {
                self.ui_element(object)
                    .and_then(|element| com_identity(&element))
            })
            .transpose()?;
        Ok(actual == expected)
    }

    pub fn simulate_navigation_display_mode_changed(
        &self,
        object: ObjectId,
        value: NavigationViewDisplayMode,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::NavigationView(control))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Self::dispatch_navigation_view_display_mode(
            &control.display_mode_changed,
            &self.event_queue,
            object,
            EventId::DisplayModeChanged,
            None,
            value,
        );
        Ok(())
    }

    pub fn simulate_selected_date_changed(
        &self,
        object: ObjectId,
        value: Option<DateTime>,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::DatePicker(control))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Self::dispatch_optional_date_time(
            &control.selected_date_changed,
            &self.event_queue,
            object,
            EventId::SelectedDateChanged,
            None,
            value,
        );
        Ok(())
    }

    pub fn simulate_selected_time_changed(
        &self,
        object: ObjectId,
        value: Option<TimeSpan>,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::TimePicker(control))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Self::dispatch_optional_time_span(
            &control.selected_time_changed,
            &self.event_queue,
            object,
            EventId::SelectedTimeChanged,
            None,
            value,
        );
        Ok(())
    }

    pub fn color_picker_color(&self, object: ObjectId) -> Result<Color, WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::ColorPicker(control))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok(from_native_color(control.value.Color()?))
    }

    pub fn image_has_source(&self, object: ObjectId) -> Result<bool, WinUiError> {
        let result = match self.handles.get(&object) {
            Some(Handle::Generated(GeneratedHandle::Image(control))) => control.value.Source(),
            Some(Handle::Generated(GeneratedHandle::ImageIcon(control))) => control.Source(),
            _ => return Err(WinUiError::InvalidObject(object)),
        };
        match result {
            Ok(_) => Ok(true),
            Err(error) if error.code().is_ok() => Ok(false),
            Err(error) => Err(error.into()),
        }
    }

    pub fn image_has_decode_failure_subscription(&self, object: ObjectId) -> bool {
        self.encoded_image_failures.contains_key(&object)
    }

    pub fn image_source_is_svg(&self, object: ObjectId) -> Result<bool, WinUiError> {
        let source = match self.handles.get(&object) {
            Some(Handle::Generated(GeneratedHandle::Image(control))) => control.value.Source()?,
            Some(Handle::Generated(GeneratedHandle::ImageIcon(control))) => control.Source()?,
            _ => return Err(WinUiError::InvalidObject(object)),
        };
        Ok(source.cast::<native::ISvgImageSource>().is_ok())
    }

    pub fn show_flyout(&self, target: ObjectId) -> Result<bool, WinUiError> {
        let (flyout, _, _) = self
            .flyouts
            .get(&target)
            .ok_or(WinUiError::InvalidObject(target))?;
        let target = self
            .ui_element(target)?
            .cast::<native::FrameworkElement>()?;
        let flyout = flyout.cast::<native::IFlyoutBase>()?;
        flyout.ShowAt(&target)?;
        Ok(flyout.IsOpen()?)
    }

    pub fn is_flyout_open(&self, target: ObjectId) -> Result<bool, WinUiError> {
        let (flyout, _, _) = self
            .flyouts
            .get(&target)
            .ok_or(WinUiError::InvalidObject(target))?;
        Ok(flyout.cast::<native::IFlyoutBase>()?.IsOpen()?)
    }

    pub fn hide_flyout(&self, target: ObjectId) -> Result<(), WinUiError> {
        let (flyout, _, _) = self
            .flyouts
            .get(&target)
            .ok_or(WinUiError::InvalidObject(target))?;
        flyout.cast::<native::IFlyoutBase>()?.Hide()?;
        Ok(())
    }

    pub fn show_menu(&self, target: ObjectId) -> Result<bool, WinUiError> {
        let menu = self
            .menus
            .get(&target)
            .ok_or(WinUiError::InvalidObject(target))?;
        let flyout = menu
            ._flyout
            .as_ref()
            .ok_or(WinUiError::InvalidObject(target))?
            .cast::<native::IFlyoutBase>()?;
        let target = self
            .ui_element(target)?
            .cast::<native::FrameworkElement>()?;
        flyout.ShowAt(&target)?;
        Ok(flyout.IsOpen()?)
    }

    pub fn is_menu_open(&self, target: ObjectId) -> Result<bool, WinUiError> {
        let menu = self
            .menus
            .get(&target)
            .ok_or(WinUiError::InvalidObject(target))?;
        Ok(menu
            ._flyout
            .as_ref()
            .ok_or(WinUiError::InvalidObject(target))?
            .cast::<native::IFlyoutBase>()?
            .IsOpen()?)
    }

    pub fn hide_menu(&self, target: ObjectId) -> Result<(), WinUiError> {
        let menu = self
            .menus
            .get(&target)
            .ok_or(WinUiError::InvalidObject(target))?;
        menu._flyout
            .as_ref()
            .ok_or(WinUiError::InvalidObject(target))?
            .cast::<native::IFlyoutBase>()?
            .Hide()?;
        Ok(())
    }

    pub fn show_command_bar_flyout(&self, target: ObjectId) -> Result<bool, WinUiError> {
        let flyout = self
            .command_bar_flyouts
            .get(&target)
            .ok_or(WinUiError::InvalidObject(target))?
            ._native
            .cast::<native::IFlyoutBase>()?;
        let target = self
            .ui_element(target)?
            .cast::<native::FrameworkElement>()?;
        flyout.ShowAt(&target)?;
        Ok(flyout.IsOpen()?)
    }

    pub fn is_command_bar_flyout_open(&self, target: ObjectId) -> Result<bool, WinUiError> {
        Ok(self
            .command_bar_flyouts
            .get(&target)
            .ok_or(WinUiError::InvalidObject(target))?
            ._native
            .cast::<native::IFlyoutBase>()?
            .IsOpen()?)
    }

    pub fn hide_command_bar_flyout(&self, target: ObjectId) -> Result<(), WinUiError> {
        self.command_bar_flyouts
            .get(&target)
            .ok_or(WinUiError::InvalidObject(target))?
            ._native
            .cast::<native::IFlyoutBase>()?
            .Hide()?;
        Ok(())
    }

    pub fn rich_text_shape(&self, object: ObjectId) -> Result<Vec<u32>, WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::RichTextBlock(control))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        let blocks = control.Blocks()?;
        let mut shape = Vec::with_capacity(blocks.Size()? as usize);
        for index in 0..blocks.Size()? {
            let paragraph = blocks.GetAt(index)?.cast::<native::IParagraph>()?;
            let inlines = paragraph.Inlines()?;
            shape.push(inlines.Size()?);
        }
        Ok(shape)
    }

    pub fn simulate_key_event(
        &self,
        object: ObjectId,
        event: EventId,
        value: KeyEventInfo,
    ) -> Result<bool, WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Border(border))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        let native_event = match event {
            EventId::PreviewKeyDown => &border.preview_key_down,
            EventId::KeyUp => &border.key_up,
            _ => return Err(WinUiError::InvalidEvent(object, event)),
        };
        Ok(native_event
            .borrow()
            .callback
            .as_ref()
            .is_some_and(|callback| callback.call(value)))
    }

    pub fn simulate_character_event(
        &self,
        object: ObjectId,
        value: CharacterEventInfo,
    ) -> Result<bool, WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Border(border))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok(border
            .character_received
            .borrow()
            .callback
            .as_ref()
            .is_some_and(|callback| callback.call(value)))
    }

    pub fn simulate_focus_event(
        &self,
        object: ObjectId,
        event: EventId,
        value: FocusEventInfo,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Border(border))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        let native_event = match event {
            EventId::GotFocus => &border.got_focus,
            EventId::LostFocus => &border.lost_focus,
            _ => return Err(WinUiError::InvalidEvent(object, event)),
        };
        Self::dispatch_focus_event_info(
            native_event,
            &self.event_queue,
            object,
            event,
            None,
            value,
        );
        Ok(())
    }

    pub fn simulate_pointer_event(
        &self,
        object: ObjectId,
        event: EventId,
        value: PointerEventInfo,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Border(border))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        let native_event = match event {
            EventId::PointerPressed => &border.pointer_pressed,
            EventId::PointerMoved => &border.pointer_moved,
            EventId::PointerEntered => &border.pointer_entered,
            EventId::PointerExited => &border.pointer_exited,
            EventId::PointerReleased => &border.pointer_released,
            _ => return Err(WinUiError::InvalidEvent(object, event)),
        };
        Self::dispatch_pointer_event_info(
            native_event,
            &self.event_queue,
            object,
            event,
            None,
            value,
        );
        Ok(())
    }

    pub fn set_slider_value(&self, object: ObjectId, value: f64) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Slider(slider))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        slider.value.cast::<native::IRangeBase>()?.SetValue(value)?;
        Ok(())
    }

    pub fn slider_state(&self, object: ObjectId) -> Result<(f64, f64, f64), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::Slider(slider))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        let slider = slider.value.cast::<native::IRangeBase>()?;
        Ok((slider.Minimum()?, slider.Maximum()?, slider.Value()?))
    }

    pub fn set_toggle_switch_is_on(&self, object: ObjectId, value: bool) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::ToggleSwitch(toggle))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        toggle.value.SetIsOn(value)?;
        Ok(())
    }

    pub fn toggle_switch_state(&self, object: ObjectId) -> Result<bool, WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::ToggleSwitch(toggle))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok(toggle.value.IsOn()?)
    }

    pub fn set_password(&self, object: ObjectId, value: &str) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::PasswordBox(password_box))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        self.event_queue.feedback.borrow_mut().begin(
            object,
            EventId::PasswordChanged,
            FeedbackExpectation::Suppressed,
        );
        let result = password_box.value.SetPassword(value);
        self.event_queue
            .feedback
            .borrow_mut()
            .finish(object, EventId::PasswordChanged);
        result?;
        let value = password_box.value.Password().map(Rc::<str>::from)?;
        let observation = Observation::SetProperty {
            object,
            property: Property {
                id: PropertyId::Password,
                value: PropertyValue::String(Rc::clone(&value)),
            },
        };
        let event = password_box.password_changed.borrow();
        self.event_queue.queue(
            Some(observation),
            event.callback.is_some().then(|| QueuedEvent {
                object,
                event: EventId::PasswordChanged,
                revision: event.revision,
                payload: EventPayload::String(value),
            }),
        );
        Self::schedule_event_wake(&self.event_queue);
        Ok(())
    }

    pub fn set_rating_value(&self, object: ObjectId, value: f64) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::RatingControl(rating))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        self.event_queue.feedback.borrow_mut().begin(
            object,
            EventId::ValueChanged,
            FeedbackExpectation::Suppressed,
        );
        let result = rating.value.SetValue(value);
        self.event_queue
            .feedback
            .borrow_mut()
            .finish(object, EventId::ValueChanged);
        result?;
        let value = rating.value.Value().map(rating_value)?;
        let observation = Observation::SetProperty {
            object,
            property: Property {
                id: PropertyId::Value,
                value: PropertyValue::OptionalF64(value),
            },
        };
        let event = rating.value_changed.borrow();
        self.event_queue.queue(
            Some(observation),
            event.callback.is_some().then(|| QueuedEvent {
                object,
                event: EventId::ValueChanged,
                revision: event.revision,
                payload: EventPayload::OptionalF64(value),
            }),
        );
        Self::schedule_event_wake(&self.event_queue);
        Ok(())
    }

    pub fn set_combo_box_selected_index(
        &self,
        object: ObjectId,
        value: Option<usize>,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::ComboBox(combo_box))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        combo_box
            .value
            .cast::<native::ISelector>()?
            .SetSelectedIndex(native_selection_index(value)?)?;
        Ok(())
    }

    pub fn select_item(
        &self,
        object: ObjectId,
        selected: Option<ObjectId>,
    ) -> Result<(), WinUiError> {
        let selection =
            selection_contract(self.kind(object)?).ok_or(WinUiError::InvalidObject(object))?;
        let selected = selected
            .map(|selected| {
                self.event_queue
                    .selection_items
                    .borrow()
                    .iter()
                    .find_map(|item| {
                        (item.owner == object && item.object == selected)
                            .then(|| item.value.clone())
                    })
                    .ok_or(WinUiError::ChildNotFound(selected))
            })
            .transpose()?;
        self.write_selected_item(object, selection, selected.as_ref())
    }

    pub fn selected_item(&self, object: ObjectId) -> Result<Option<ObjectId>, WinUiError> {
        let selection =
            selection_contract(self.kind(object)?).ok_or(WinUiError::InvalidObject(object))?;
        let selected = self.read_selected_item(object, selection)?;
        Ok(selected.as_ref().and_then(|selected| {
            self.event_queue
                .selection_items
                .borrow()
                .iter()
                .find_map(|item| {
                    (item.owner == object && item.value == *selected).then_some(item.object)
                })
        }))
    }

    pub fn check_box_state(&self, object: ObjectId) -> Result<bool, WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::CheckBox(check_box))) =
            self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok(check_box
            .value
            .cast::<native::IToggleButton>()?
            .IsChecked()?)
    }

    pub fn stack_panel_state(&self, object: ObjectId) -> Result<(f64, bool), WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::StackPanel(panel))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok((
            panel.Spacing()?,
            panel.Orientation()? == native::Orientation::Horizontal,
        ))
    }

    pub fn canvas_position(&self, object: ObjectId) -> Result<(f64, f64), WinUiError> {
        let element = self
            .ui_element(object)?
            .cast::<native::FrameworkElement>()?;
        Ok((
            native::Canvas::GetLeft(&element)?,
            native::Canvas::GetTop(&element)?,
        ))
    }

    pub fn capability_state(
        &self,
        object: ObjectId,
    ) -> Result<(f64, f64, i32, bool, String, bool), WinUiError> {
        let element = self
            .ui_element(object)?
            .cast::<native::FrameworkElement>()?;
        Ok((
            element.MinWidth()?,
            element.MaxHeight()?,
            native::Grid::GetRow(&element)?,
            native::RelativePanel::GetAlignLeftWithPanel(&element)?,
            native::AutomationProperties::GetName(&element)?,
            element.cast::<native::IControl>()?.IsEnabled()?,
        ))
    }

    pub fn text_block_font_weight(&self, object: ObjectId) -> Result<u16, WinUiError> {
        let Some(Handle::Generated(GeneratedHandle::TextBlock(value))) = self.handles.get(&object)
        else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok(value.FontWeight()?.weight)
    }

    pub fn text_box_state(&self, object: ObjectId) -> Result<(String, i32, i32), WinUiError> {
        let Some(Handle::TextBox(value)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        Ok((
            value.value.Text()?,
            value.value.SelectionStart()?,
            value.value.SelectionLength()?,
        ))
    }

    pub fn text_box_appearance(
        &self,
        object: ObjectId,
    ) -> Result<(String, bool, TextWrapping), WinUiError> {
        let Some(Handle::TextBox(value)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        let wrapping = match value.value.TextWrapping()? {
            native::TextWrapping::NoWrap => TextWrapping::NoWrap,
            native::TextWrapping::Wrap => TextWrapping::Wrap,
            native::TextWrapping::WrapWholeWords => TextWrapping::WrapWholeWords,
            _ => return Err(WinUiError::StateMismatch(object)),
        };
        Ok((
            value.value.PlaceholderText()?,
            value.value.AcceptsReturn()?,
            wrapping,
        ))
    }

    fn validate_native_order(
        &self,
        parent: ObjectId,
        contract: &RelationContract,
        children: &[ObjectId],
    ) -> Result<(), WinUiError> {
        let matches = match contract.realization {
            Realization::Owned => {
                let values = self.owned_collection(parent, contract.id)?;
                let retired = self
                    .retirements
                    .iter()
                    .filter_map(|(root, retirement)| {
                        (retirement.parent == parent && retirement.relation == contract.id)
                            .then_some(*root)
                    })
                    .map(|root| self.ui_element(root).and_then(|value| com_identity(&value)))
                    .collect::<Result<HashSet<_>, _>>()?;
                let mut active = Vec::new();
                for index in 0..values.size()? {
                    let value = values.get_at(index)?;
                    let identity = com_identity(&value)?;
                    if !retired.contains(&identity) {
                        active.push(identity);
                    }
                }
                let desired = children
                    .iter()
                    .map(|child| {
                        self.ui_element(*child)
                            .and_then(|value| com_identity(&value))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                active == desired
            }
            Realization::Structural => {
                let values = self.tree_nodes(parent)?;
                if values.Size()? as usize != children.len() {
                    false
                } else {
                    children.iter().enumerate().all(|(index, child)| {
                        values.GetAt(index as u32).ok().as_ref()
                            == self.tree_node(*child).ok().as_ref()
                    })
                }
            }
            Realization::Container => {
                let values = self.items(parent)?;
                if values.Size()? as usize != children.len() {
                    false
                } else {
                    children.iter().enumerate().all(|(index, child)| {
                        values.GetAt(index as u32).ok().as_ref()
                            == self.inspectable(*child).ok().as_ref()
                    })
                }
            }
        };
        if matches {
            Ok(())
        } else {
            Err(WinUiError::StateMismatch(parent))
        }
    }
}
