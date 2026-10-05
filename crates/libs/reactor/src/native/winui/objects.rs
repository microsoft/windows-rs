impl WinUiAdapter {
    fn create(&mut self, object: ObjectId, kind: ObjectType) -> Result<(), WinUiError> {
        if self.handles.contains_key(&object) {
            return Err(WinUiError::DuplicateObject(object));
        }
        let handle = if let Some(handle) =
            GeneratedHandle::create(kind, object, &self.event_queue, &self.pending_focus_states)?
        {
            Handle::Generated(handle)
        } else {
            match kind {
                ObjectType::TextBox => {
                    let value = native::TextBox::new()?;
                    let event = Rc::new(RefCell::new(NativeTextEvent::default()));
                    let event_for_callback = Rc::clone(&event);
                    let observed_text = Rc::new(RefCell::new(Rc::<str>::from("")));
                    let observed_text_for_event = Rc::clone(&observed_text);
                    let event_queue = Rc::clone(&self.event_queue);
                    let source = value.clone();
                    let text_changed = value.TextChanged(move |_, _| {
                        let text = match source.Text() {
                            Ok(text) => text,
                            Err(error) => {
                                report_error(error);
                                return;
                            }
                        };
                        Self::dispatch_text_changed(
                            &event_for_callback,
                            &observed_text_for_event,
                            &event_queue,
                            object,
                            Rc::from(text),
                        );
                    })?;
                    Handle::TextBox(NativeTextBox {
                        value,
                        event,
                        observed_text,
                        set_count: Cell::new(0),
                        _text_changed: text_changed,
                    })
                }
                ObjectType::TreeView => {
                    let value = native::TreeView::new()?;
                    let template = self.tree_template()?;
                    value
                        .cast::<native::ITreeView2>()?
                        .SetItemTemplate(&template)?;
                    let item_invoked = Rc::new(RefCell::new(NativeStringEvent::default()));
                    let item_event = Rc::clone(&item_invoked);
                    let item_queue = Rc::clone(&self.event_queue);
                    let item_texts = Rc::clone(&self.tree_node_texts);
                    let _item_invoked = value.ItemInvoked(move |_, args| {
                        let args = args.unwrap();
                        let value = args
                            .InvokedItem()
                            .map_err(Into::into)
                            .and_then(|value| {
                                value.cast::<native::ITreeViewNode>().map_err(Into::into)
                            })
                            .and_then(|value| com_identity(&value))
                            .and_then(|identity| {
                                item_texts
                                    .borrow()
                                    .get(&identity)
                                    .cloned()
                                    .ok_or(WinUiError::UnknownTreeNode(identity))
                            });
                        match value {
                            Ok(value) => Self::dispatch_string(
                                &item_event,
                                &item_queue,
                                object,
                                EventId::ItemInvoked,
                                None,
                                value,
                            ),
                            Err(error) => report_error(error.into()),
                        }
                    })?;
                    Handle::TreeView(NativeTreeView {
                        value,
                        item_invoked,
                        _item_invoked,
                    })
                }
                ObjectType::TreeNode => {
                    let value = native::TreeViewNode::new()?;
                    self.tree_node_texts
                        .borrow_mut()
                        .insert(com_identity(&value)?, Rc::from(""));
                    Handle::TreeNode(NativeTreeNode {
                        value,
                        text: HSTRING::new(),
                        content: None,
                    })
                }
                ObjectType::ListView => {
                    let value = native::ListView::new()?;
                    value
                        .cast::<native::IItemsControl>()?
                        .SetItemTemplate(&self.list_template()?)?;
                    let selection_changed =
                        Rc::new(RefCell::new(NativeSelectionIndexEvent::default()));
                    let selection_event = Rc::clone(&selection_changed);
                    let selection_queue = Rc::clone(&self.event_queue);
                    let selection_source = value.cast::<native::ISelector>()?;
                    let selection_source_for_callback = selection_source.clone();
                    let _selection_changed = selection_source.SelectionChanged(move |_, _| {
                        let value = match selection_source_for_callback
                            .SelectedIndex()
                            .and_then(selection_index)
                        {
                            Ok(value) => value,
                            Err(error) => {
                                report_error(error);
                                return;
                            }
                        };
                        let observation = Observation::SetProperty {
                            object,
                            property: Property {
                                id: PropertyId::SelectedIndex,
                                value: PropertyValue::SelectionIndex(value),
                            },
                        };
                        let dispatch = selection_queue.observe(
                            object,
                            EventId::SelectionChanged,
                            observation.clone(),
                        );
                        if dispatch {
                            Self::dispatch_selection_index(
                                &selection_event,
                                &selection_queue,
                                object,
                                EventId::SelectionChanged,
                                Some(observation),
                                value,
                            );
                        }
                    })?;
                    let drag_items_completed =
                        Rc::new(RefCell::new(NativeStringListEvent::default()));
                    let drag_event = Rc::clone(&drag_items_completed);
                    let drag_queue = Rc::clone(&self.event_queue);
                    let drag_source = value.cast::<native::IItemsControl>()?;
                    let _drag_items_completed = value
                        .cast::<native::IListViewBase>()?
                        .DragItemsCompleted(move |_, _| match Self::item_tags(&drag_source) {
                            Ok(value) => Self::dispatch_string_list(
                                &drag_event,
                                &drag_queue,
                                object,
                                EventId::DragItemsCompleted,
                                None,
                                value,
                            ),
                            Err(error) => report_error(error.into()),
                        })?;
                    Handle::ListView(NativeListView {
                        value,
                        selection_changed,
                        drag_items_completed,
                        _selection_changed,
                        _drag_items_completed,
                    })
                }
                ObjectType::DataItem => {
                    let values = BTreeMap::<HSTRING, Option<IInspectable>>::new();
                    Handle::Data(values.into())
                }
                _ => unreachable!("generated object was not created"),
            }
        };
        self.handles.insert(object, handle);
        if kind == ObjectType::ContentDialog {
            let Some(Handle::Generated(GeneratedHandle::ContentDialog(value))) =
                self.handles.get(&object)
            else {
                unreachable!()
            };
            self.event_queue
                .content_dialogs
                .borrow_mut()
                .create(object, value.value.clone());
        }
        Ok(())
    }

    fn set_properties(
        &mut self,
        object: ObjectId,
        set: &[Property],
        clear: &[PropertyId],
    ) -> Result<(), WinUiError> {
        let kind = self.kind(object)?;
        let sorted_clear = (kind == ObjectType::ListView).then(|| {
            let mut clear = clear.to_vec();
            clear.sort_by_key(|property| *property == PropertyId::SelectedIndex);
            clear
        });
        let sorted_set = (kind == ObjectType::ListView).then(|| {
            let mut set = set.to_vec();
            set.sort_by_key(|property| property.id == PropertyId::SelectedIndex);
            set
        });
        let clear = sorted_clear.as_deref().unwrap_or(clear);
        let set = sorted_set.as_deref().unwrap_or(set);
        for property in clear.iter().copied() {
            if property == PropertyId::Source {
                self.encoded_image_failures.remove(&object);
            }
            let feedback = GeneratedHandle::feedback_expectation(kind, property, None);
            if let Some((event, expectation)) = feedback.clone() {
                self.event_queue
                    .feedback
                    .borrow_mut()
                    .begin(object, event, expectation);
            }
            let selection_feedback = self.selection_feedback(object, property)?;
            if let Some((owner, event)) = selection_feedback {
                self.event_queue.feedback.borrow_mut().begin(
                    owner,
                    event,
                    FeedbackExpectation::Suppressed,
                );
            }
            let result = 'apply: {
                match property {
                    PropertyId::Resources => {
                        break 'apply self
                            .set_resource_overrides(object, &ResourceOverrides::default());
                    }
                    PropertyId::KeyboardAccelerators => {
                        break 'apply self
                            .set_key_accelerators(object, &KeyAccelerators::default());
                    }
                    PropertyId::Style if kind == ObjectType::Button => {
                        break 'apply self.set_button_style(object, None);
                    }
                    _ => {}
                }
                if theme_style_info(kind).is_some_and(|(_, properties)| {
                    properties
                        .iter()
                        .any(|(candidate, _)| *candidate == property)
                }) {
                    self.set_theme_brush(object, property, None)?;
                }
                if let Ok(element) = self.ui_element(object)
                    && let Some(result) =
                        GeneratedHandle::set_attached_property(&element, property, None)
                {
                    break 'apply result;
                }
                if let Ok(element) = self.ui_element(object)
                    && let Some(result) =
                        GeneratedHandle::set_visual_property(&element, property, None)
                {
                    break 'apply result;
                }
                if let Ok(element) = self.ui_element(object)
                    && let Some(result) =
                        GeneratedHandle::set_handwritten_property(kind, &element, property, None)
                {
                    break 'apply result;
                }
                if let Some(Handle::Generated(handle)) = self.handles.get(&object)
                    && let Some(result) = handle.set_property(property, None)
                {
                    break 'apply result;
                }
                if property == PropertyId::Text
                    && matches!(self.handles.get(&object), Some(Handle::Data(_)))
                {
                    break 'apply self.set_data_text(object, "");
                }
                break 'apply match (self.handle(object)?, property) {
                    (Handle::TextBox(value), PropertyId::Text) => {
                        Self::set_text_box_text(value, "")
                    }
                    (Handle::TreeNode(value), PropertyId::Text) => {
                        value.text = HSTRING::new();
                        if value.content.is_none() {
                            let content: IInspectable =
                                windows_reference::IReference::from(value.text.clone()).into();
                            value.value.SetContent(&content)?;
                        }
                        Ok(())
                    }
                    (Handle::TreeNode(value), PropertyId::Expanded) => {
                        value.value.SetIsExpanded(false).map_err(Into::into)
                    }
                    (Handle::ListView(value), PropertyId::SelectedIndex) => value
                        .value
                        .cast::<native::ISelector>()?
                        .SetSelectedIndex(-1)
                        .map_err(Into::into),
                    _ => Err(WinUiError::InvalidObject(object)),
                };
            };
            let observation = feedback.and_then(|(event, _)| {
                self.event_queue.feedback.borrow_mut().finish(object, event)
            });
            if let Some((owner, event)) = selection_feedback {
                self.event_queue.feedback.borrow_mut().finish(owner, event);
            }
            result?;
            if let Some(observation) = observation {
                self.event_queue.queue(Some(observation), None);
                Self::schedule_event_wake(&self.event_queue);
            }
        }
        for property in set {
            if property.id == PropertyId::Source {
                self.encoded_image_failures.remove(&object);
            }
            let feedback =
                GeneratedHandle::feedback_expectation(kind, property.id, Some(&property.value));
            if let Some((event, expectation)) = feedback.clone() {
                self.event_queue
                    .feedback
                    .borrow_mut()
                    .begin(object, event, expectation);
            }
            let selection_feedback = self.selection_feedback(object, property.id)?;
            if let Some((owner, event)) = selection_feedback {
                self.event_queue.feedback.borrow_mut().begin(
                    owner,
                    event,
                    FeedbackExpectation::Suppressed,
                );
            }
            let result = 'apply: {
                match (property.id, &property.value) {
                    (PropertyId::Resources, PropertyValue::ResourceOverrides(value)) => {
                        break 'apply self.set_resource_overrides(object, value);
                    }
                    (PropertyId::KeyboardAccelerators, PropertyValue::KeyAccelerators(value)) => {
                        break 'apply self.set_key_accelerators(object, value);
                    }
                    (PropertyId::Style, PropertyValue::ButtonStyle(value))
                        if kind == ObjectType::Button =>
                    {
                        break 'apply self.set_button_style(object, Some(*value));
                    }
                    (id, PropertyValue::Brush(Brush::Theme(value)))
                        if theme_style_info(kind).is_some_and(|(_, properties)| {
                            properties.iter().any(|(candidate, _)| *candidate == id)
                        }) =>
                    {
                        if let Ok(element) = self.ui_element(object)
                            && let Some(result) =
                                GeneratedHandle::set_handwritten_property(kind, &element, id, None)
                        {
                            result?;
                        } else if let Some(Handle::Generated(handle)) = self.handles.get(&object)
                            && let Some(result) = handle.set_property(id, None)
                        {
                            result?;
                        } else {
                            break 'apply Err(WinUiError::InvalidObject(object));
                        }
                        break 'apply self.set_theme_brush(object, id, Some(*value));
                    }
                    (id, PropertyValue::Brush(Brush::Solid(_)))
                        if theme_style_info(kind).is_some_and(|(_, properties)| {
                            properties.iter().any(|(candidate, _)| *candidate == id)
                        }) =>
                    {
                        self.set_theme_brush(object, id, None)?;
                    }
                    _ => {}
                }
                if let Ok(element) = self.ui_element(object)
                    && let Some(result) = GeneratedHandle::set_attached_property(
                        &element,
                        property.id,
                        Some(&property.value),
                    )
                {
                    break 'apply result;
                }
                if let Ok(element) = self.ui_element(object)
                    && let Some(result) = GeneratedHandle::set_visual_property(
                        &element,
                        property.id,
                        Some(&property.value),
                    )
                {
                    break 'apply result;
                }
                if let Ok(element) = self.ui_element(object)
                    && let Some(result) = GeneratedHandle::set_handwritten_property(
                        kind,
                        &element,
                        property.id,
                        Some(&property.value),
                    )
                {
                    break 'apply result;
                }
                if property.id == PropertyId::Source
                    && let PropertyValue::ImageSource(source) = &property.value
                    && let ImageSourceValue::Encoded(value) = source.value()
                {
                    let failed = match self.handles.get(&object) {
                        Some(Handle::Generated(GeneratedHandle::Image(control))) => {
                            let event = Rc::clone(&control.image_failed);
                            let event_queue = Rc::clone(&self.event_queue);
                            Some(Rc::new(move || {
                                Self::dispatch_unit(
                                    &event,
                                    &event_queue,
                                    object,
                                    EventId::ImageFailed,
                                    None,
                                );
                            }) as Rc<dyn Fn()>)
                        }
                        _ => None,
                    };
                    let (image, failed) = encoded_bitmap_image(value, failed)?;
                    let image = image.cast::<native::ImageSource>()?;
                    match self.handles.get(&object) {
                        Some(Handle::Generated(GeneratedHandle::Image(control))) => {
                            control.value.SetSource(&image).map_err(Into::into)
                        }
                        Some(Handle::Generated(GeneratedHandle::ImageIcon(control))) => {
                            control.SetSource(&image).map_err(Into::into)
                        }
                        _ => Err(WinUiError::InvalidObject(object)),
                    }?;
                    if let Some(failed) = failed {
                        self.encoded_image_failures.insert(object, failed);
                    }
                    break 'apply Ok(());
                }
                if let Some(Handle::Generated(handle)) = self.handles.get(&object)
                    && let Some(result) = handle.set_property(property.id, Some(&property.value))
                {
                    break 'apply result;
                }
                if property.id == PropertyId::Text
                    && matches!(self.handles.get(&object), Some(Handle::Data(_)))
                    && let PropertyValue::String(text) = &property.value
                {
                    break 'apply self.set_data_text(object, text);
                }
                let tree_node_texts = Rc::clone(&self.tree_node_texts);
                break 'apply match (self.handle(object)?, property.id, &property.value) {
                    (Handle::TextBox(value), PropertyId::Text, PropertyValue::String(text)) => {
                        Self::set_text_box_text(value, text)
                    }
                    (Handle::TreeNode(value), PropertyId::Text, PropertyValue::String(text)) => {
                        value.text = HSTRING::from(text.as_ref());
                        tree_node_texts
                            .borrow_mut()
                            .insert(com_identity(&value.value)?, Rc::clone(text));
                        if value.content.is_none() {
                            let content: IInspectable =
                                windows_reference::IReference::from(value.text.clone()).into();
                            value.value.SetContent(&content)?;
                        }
                        Ok(())
                    }
                    (
                        Handle::TreeNode(value),
                        PropertyId::Expanded,
                        PropertyValue::Bool(expanded),
                    ) => value.value.SetIsExpanded(*expanded).map_err(Into::into),
                    (
                        Handle::ListView(value),
                        PropertyId::SelectedIndex,
                        PropertyValue::SelectionIndex(selected),
                    ) => value
                        .value
                        .cast::<native::ISelector>()?
                        .SetSelectedIndex(native_selection_index(*selected)?)
                        .map_err(Into::into),
                    _ => Err(WinUiError::InvalidObject(object)),
                };
            };
            let observation = feedback.and_then(|(event, _)| {
                self.event_queue.feedback.borrow_mut().finish(object, event)
            });
            if let Some((owner, event)) = selection_feedback {
                self.event_queue.feedback.borrow_mut().finish(owner, event);
            }
            result?;
            if let Some(observation) = observation {
                self.event_queue.queue(Some(observation), None);
                Self::schedule_event_wake(&self.event_queue);
            }
        }
        Ok(())
    }

    fn replace(&mut self, object: ObjectId, kind: ObjectType) -> Result<(), WinUiError> {
        if self.tooltips.contains_key(&object)
            || self.tooltip_owners.contains_key(&object)
            || self.flyouts.contains_key(&object)
            || self.flyout_owners.contains_key(&object)
            || self.menus.contains_key(&object)
            || self.command_bar_flyouts.contains_key(&object)
            || self.content_dialogs.contains_key(&object)
            || self.content_dialog_owners.contains_key(&object)
        {
            return Err(WinUiError::StillOwned(object));
        }
        let (parent, relation) = self
            .owners
            .get(&object)
            .copied()
            .ok_or(WinUiError::InvalidReplacement(object))?;
        let contract = relation_contract(self.kind(parent)?, relation)?;
        if contract.realization != Realization::Owned
            || contract.child != object_category(kind)
        {
            return Err(WinUiError::InvalidReplacement(object));
        }
        let previous = self.ui_element(object)?;
        let index = match contract.cardinality {
            Cardinality::One => {
                match (self.handle(parent)?, relation) {
                    (Handle::Generated(parent_handle), relation) => {
                        parent_handle
                            .set_content(relation, None)
                            .ok_or(WinUiError::InvalidRelation(parent, relation))??;
                    }
                    (Handle::TreeNode(parent), RelationId::Content) => {
                        let content: IInspectable =
                            windows_reference::IReference::from(parent.text.clone()).into();
                        parent.value.SetContent(&content)?;
                        parent.content = None;
                    }
                    _ => return Err(WinUiError::InvalidRelation(parent, relation)),
                }
                None
            }
            Cardinality::Many => {
                let values = self.owned_collection(parent, relation)?;
                let previous: IInspectable = previous.cast()?;
                let mut index = 0;
                while index < values.size()? && values.get_at(index)? != previous {
                    index += 1;
                }
                if index == values.size()? {
                    return Err(WinUiError::ChildNotFound(object));
                }
                values.remove_at(index)?;
                Some(index)
            }
        };
        self.handles
            .remove(&object)
            .ok_or(WinUiError::MissingObject(object))?;
        self.encoded_image_failures.remove(&object);
        self.resource_override_keys.remove(&object);
        self.style_states.remove(&object);
        self.create(object, kind)?;
        let replacement = self.ui_element(object)?;
        if let Some(index) = index {
            self.owned_collection(parent, relation)?
                .insert_at(index, &replacement.cast()?)?;
        } else {
            match (self.handle(parent)?, relation) {
                (Handle::Generated(parent_handle), relation) => {
                    parent_handle
                        .set_content(relation, Some(&replacement))
                        .ok_or(WinUiError::InvalidRelation(parent, relation))??;
                }
                (Handle::TreeNode(parent), RelationId::Content) => {
                    parent.value.SetContent(&replacement)?;
                    parent.content = Some(object);
                }
                _ => return Err(WinUiError::InvalidRelation(parent, relation)),
            }
        }
        self.event_queue.events.borrow_mut().retain(|event| {
            event
                .event
                .as_ref()
                .is_none_or(|event| event.object != object)
                && event.observation.as_ref().is_none_or(|observation| {
                    !matches!(
                        observation,
                        Observation::SetProperty {
                            object: observed,
                            ..
                        } | Observation::SetSelection {
                            object: observed,
                            ..
                        } if *observed == object
                    )
                })
        });
        self.event_queue.feedback.borrow_mut().remove_object(object);
        Ok(())
    }
}
