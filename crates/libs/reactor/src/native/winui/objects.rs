fn is_implicit_transition(id: PropertyId) -> bool {
    matches!(
        id,
        PropertyId::OpacityTransition
            | PropertyId::ScaleTransition
            | PropertyId::TranslationTransition
    )
}

impl WinUiAdapter {
    fn create(&mut self, object: ObjectId, kind: ObjectType) -> Result<(), WinUiError> {
        if self.handles.contains_key(&object) {
            return Err(WinUiError::DuplicateObject(object));
        }
        let handle = if let Some(handle) =
            GeneratedHandle::create(kind, object, &self.event_queue)?
        {
            if let GeneratedHandle::ListView(list) = &handle {
                list.value
                    .cast::<native::IItemsControl>()?
                    .SetItemTemplate(&self.list_template()?)?;
            }
            Handle::Generated(handle)
        } else {
            match kind {
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
                        let node = args
                            .InvokedItem()
                            .and_then(|value| value.cast::<native::ITreeViewNode>())
                            .unwrap();
                        let identity = com_identity(&node).unwrap();
                        let value = item_texts.borrow().get(&identity).cloned().unwrap();
                        Self::dispatch(
                            &item_event,
                            &item_queue,
                            object,
                            EventId::ItemInvoked,
                            None,
                            value,
                        );
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
                ObjectType::DataItem => {
                    let values = BTreeMap::from([
                        (self.data_text_key.clone(), Some(data_string(""))),
                        (self.data_content_key.clone(), None),
                        (self.data_text_visibility_key.clone(), Some(data_string("Visible"))),
                    ]);
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

    fn set_transition_properties(
        &self,
        object: ObjectId,
        set: &[Property],
        clear: &[PropertyId],
    ) -> Result<(), WinUiError> {
        for (id, value) in clear
            .iter()
            .map(|id| (*id, None))
            .chain(set.iter().map(|property| (property.id, Some(&property.value))))
            .filter(|(id, _)| is_implicit_transition(*id))
        {
            let Some(Handle::Generated(handle)) = self.handles.get(&object) else {
                return Err(WinUiError::InvalidObject(object));
            };
            handle
                .set_property(id, value, None)
                .ok_or(WinUiError::InvalidObject(object))??;
        }
        Ok(())
    }

    fn set_properties(
        &mut self,
        object: ObjectId,
        set: &[Property],
        clear: &[PropertyId],
        initializing: bool,
    ) -> Result<(), WinUiError> {
        let kind = self.kind(object)?;
        let element = self.ui_element(object).ok();
        if let Some(Handle::Generated(handle)) = self.handles.get_mut(&object) {
            let context = EventContext {
                event_queue: &self.event_queue,
                pending_focus_states: &self.pending_focus_states,
            };
            for property in set {
                handle.subscribe_property_events(object, property.id, context)?;
            }
        }
        // Native setters choose their animation immediately. Apply the new transition policy
        // before target changes, but establish initial values before installing transitions.
        if !initializing {
            self.set_transition_properties(object, set, clear)?;
        }
        if !clear.is_empty()
            && let Some(subscription) = self.image_subscriptions.get(&object)
            && clear.contains(&subscription.property)
        {
            self.release_image(object);
        }
        let properties = clear
            .iter()
            .map(|id| (*id, None))
            .chain(set.iter().map(|property| (property.id, Some(&property.value))))
            .filter(|(id, _)| !is_implicit_transition(*id));
        for (property, value) in properties {
            let feedback = GeneratedHandle::feedback_expectation(kind, property, value);
            if let Some((event, expectation)) = feedback.clone() {
                self.event_queue
                    .feedback
                    .borrow_mut()
                    .begin(object, event, expectation);
            }
            let suppressed = match self.selection_feedback(object, property)? {
                Some(feedback) => Some(feedback),
                None => self
                    .index_event(object, IndexItems::Property(property))?
                    .map(|event| (object, event)),
            };
            if let Some((owner, event)) = suppressed {
                self.event_queue
                    .feedback
                    .borrow_mut()
                    .suppress(owner, event);
            }
            if let Some(PropertyValue::SelectionIndex(Some(_))) = value
                && index_contract(kind)
                    .is_some_and(|contract| contract.property == property && contract.realize_items)
                && let Some(element) = element.as_ref()
            {
                element.UpdateLayout()?;
            }
            let result = self.apply_property(object, kind, element.as_ref(), property, value);
            let observation = feedback.and_then(|(event, _)| {
                self.event_queue.feedback.borrow_mut().finish(object, event)
            });
            if let Some((owner, event)) = suppressed {
                self.event_queue.feedback.borrow_mut().release(owner, event);
            }
            result?;
            if let Some(observation) = observation {
                self.event_queue.queue(Some(observation), None);
                Self::schedule_event_wake(&self.event_queue);
            }
        }
        if initializing {
            self.set_transition_properties(object, set, clear)?;
        }
        Ok(())
    }

    /// Sets one property, or restores its default when `value` is `None`.
    fn apply_property(
        &mut self,
        object: ObjectId,
        kind: ObjectType,
        element: Option<&native::UIElement>,
        property: PropertyId,
        value: Option<&PropertyValue>,
    ) -> Result<(), WinUiError> {
        if property == PropertyId::Source
            && value.is_none()
            && let Some(Handle::Generated(GeneratedHandle::Image(control))) =
                self.handles.get(&object)
        {
            self.event_queue.discard_events(object, &IMAGE_SOURCE_EVENTS);
            control.source.set(false);
        }
        match (property, value) {
            (PropertyId::Resources, None) => {
                return self.set_resource_overrides(object, &ResourceOverrides::default());
            }
            (PropertyId::Resources, Some(PropertyValue::ResourceOverrides(value))) => {
                return self.set_resource_overrides(object, value);
            }
            (PropertyId::KeyboardAccelerators, None) => {
                return self.set_key_accelerators(object, &KeyAccelerators::default());
            }
            (PropertyId::KeyboardAccelerators, Some(PropertyValue::KeyAccelerators(value))) => {
                return self.set_key_accelerators(object, value);
            }
            (PropertyId::Style, None) if kind == ObjectType::Button => {
                return self.set_button_style(object, None);
            }
            (PropertyId::Style, Some(PropertyValue::ButtonStyle(value)))
                if kind == ObjectType::Button =>
            {
                return self.set_button_style(object, Some(*value));
            }
            _ => {}
        }
        if theme_style_info(kind).is_some_and(|(_, properties)| {
            properties
                .iter()
                .any(|(candidate, _)| *candidate == property)
        }) {
            match value {
                Some(PropertyValue::Brush(Brush::Theme(value))) => {
                    let Some(result) =
                        element.and_then(|element| set_dependency_property(element, kind, property, None))
                    else {
                        return Err(WinUiError::InvalidObject(object));
                    };
                    result?;
                    return self.set_theme_brush(object, property, Some(*value));
                }
                None | Some(PropertyValue::Brush(Brush::Solid(_))) => {
                    self.set_theme_brush(object, property, None)?;
                }
                _ => {}
            }
        }
        if let Some(element) = element
            && let Some(result) = set_dependency_property(element, kind, property, value)
        {
            return result;
        }
        if property == PropertyId::Source
            && let Some(PropertyValue::ImageSource(source)) = value
        {
            return self.set_image_source(object, source);
        }
        let image = if let Some(PropertyValue::Icon(icon)) = value {
            self.shared_icon_image(icon)?
        } else {
            None
        };
        if let Some(Handle::Generated(handle)) = self.handles.get(&object)
            && let Some(result) =
                handle.set_property(property, value, image.as_ref().map(|image| &image.value))
        {
            result?;
            if let Some(PropertyValue::Icon(_)) = value {
                self.release_image(object);
                if let Some(image) = image {
                    self.image_subscriptions.insert(
                        object,
                        ImageSubscription {
                            property,
                            image,
                            _callback: None,
                        },
                    );
                }
            }
            return Ok(());
        }
        if property == PropertyId::Text && matches!(self.handles.get(&object), Some(Handle::Data(_)))
        {
            match value {
                None => return self.set_data_text(object, ""),
                Some(PropertyValue::String(text)) => return self.set_data_text(object, text),
                _ => {}
            }
        }
        let tree_node_texts = Rc::clone(&self.tree_node_texts);
        match (self.handle(object)?, property, value) {
            (Handle::TreeNode(node), PropertyId::Text, None | Some(PropertyValue::String(_))) => {
                if let Some(PropertyValue::String(text)) = value {
                    node.text = HSTRING::from(text.as_ref());
                    tree_node_texts
                        .borrow_mut()
                        .insert(com_identity(&node.value)?, Rc::clone(text));
                } else {
                    node.text = HSTRING::new();
                }
                if node.content.is_none() {
                    let content: IInspectable =
                        windows_reference::IReference::from(node.text.clone()).into();
                    node.value.SetContent(&content)?;
                }
                Ok(())
            }
            (Handle::TreeNode(node), PropertyId::Expanded, None) => {
                node.value.SetIsExpanded(false).map_err(Into::into)
            }
            (Handle::TreeNode(node), PropertyId::Expanded, Some(PropertyValue::Bool(expanded))) => {
                node.value.SetIsExpanded(*expanded).map_err(Into::into)
            }
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn shared_image(
        &mut self,
        source: &ImageSource,
    ) -> Result<Rc<NativeImageSource>, WinUiError> {
        let key = ImageKey(source.value().clone());
        if let Some(image) = self.images.get(&key).and_then(Weak::upgrade) {
            return Ok(image);
        }
        let image = Rc::new(NativeImageSource::new(source.value())?);
        self.images.insert(key, Rc::downgrade(&image));
        Ok(image)
    }

    fn shared_icon_image(
        &mut self,
        icon: &Icon,
    ) -> Result<Option<Rc<NativeImageSource>>, WinUiError> {
        match icon.value() {
            IconValue::Image(source) => self.shared_image(source).map(Some),
            _ => Ok(None),
        }
    }

    fn release_image(&mut self, object: ObjectId) {
        // Keep released images until the batch completes so replacement controls can share them.
        if let Some(subscription) = self.image_subscriptions.remove(&object) {
            self.released_images.push(subscription.image);
        }
    }

    /// Drops all adapter state for an object whose handle was just removed. Destroy, retirement
    /// completion, and replacement all end an object's native lifetime through this path.
    fn release_object(&mut self, object: ObjectId, handle: Handle) -> Result<(), WinUiError> {
        self.release_image(object);
        self.live_regions.remove(&object);
        self.resource_override_keys.remove(&object);
        self.style_states.remove(&object);
        self.virtual_items.remove(&object);
        self.observations
            .retain(|(observed, _), _| *observed != object);
        if let Some(initialization) = self.webview_initializations.borrow_mut().remove(&object) {
            for completion in initialization.completions {
                completion.call(Err(IntegrationError::Unavailable));
            }
        }
        self.event_queue
            .feedback
            .borrow_mut()
            .remove_object(object);
        self.event_queue.items.borrow_mut().remove(&object);
        match handle {
            Handle::TreeNode(node) => {
                self.tree_node_texts
                    .borrow_mut()
                    .remove(&com_identity(&node.value)?);
            }
            Handle::Generated(GeneratedHandle::ContentDialog(dialog)) => {
                self.event_queue
                    .content_dialogs
                    .borrow_mut()
                    .retire(object, *dialog)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn set_image_source(&mut self, object: ObjectId, source: &ImageSource) -> Result<(), WinUiError> {
        let image = self.shared_image(source)?;
        let callback = match self.handles.get(&object) {
            Some(Handle::Generated(GeneratedHandle::Image(control))) => {
                control.value.SetSource(&image.value)?;
                self.event_queue.discard_events(object, &IMAGE_SOURCE_EVENTS);
                control.source.set(true);
                let opened = Rc::clone(&control.image_opened);
                let failed = Rc::clone(&control.image_failed);
                let event_queue = Rc::clone(&self.event_queue);
                let callback: Rc<dyn Fn(bool)> = Rc::new(move |success| {
                    let (event, id) = if success {
                        (&opened, EventId::ImageOpened)
                    } else {
                        (&failed, EventId::ImageFailed)
                    };
                    Self::dispatch(event, &event_queue, object, id, None, ());
                });
                Some(callback)
            }
            Some(Handle::Generated(GeneratedHandle::ImageIcon(control))) => {
                control.SetSource(&image.value)?;
                None
            }
            _ => return Err(WinUiError::InvalidObject(object)),
        };
        self.release_image(object);
        let result = callback
            .as_ref()
            .and_then(|callback| image.state.subscribe(callback));
        let replay = callback.as_ref().map(Rc::downgrade);
        self.image_subscriptions.insert(
            object,
            ImageSubscription {
                property: PropertyId::Source,
                image,
                _callback: callback,
            },
        );
        if let (Some(result), Some(replay)) = (result, replay) {
            // Replay a completed load after the current batch applies event handlers.
            let handler = native::DispatcherQueueHandler::new(move || {
                if let Some(callback) = replay.upgrade() {
                    callback(result);
                }
            });
            // The replay is dropped if the dispatcher is shutting down.
            native::DispatcherQueue::GetForCurrentThread()?
                .TryEnqueueWithPriority(native::DispatcherQueuePriority::Normal, &handler)?;
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
        let item = self
            .event_queue
            .items
            .borrow()
            .get(&object)
            .map(|item| item.owner);
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
        let handle = self
            .handles
            .remove(&object)
            .ok_or(WinUiError::MissingObject(object))?;
        self.release_object(object, handle)?;
        self.create(object, kind)?;
        let replacement = self.ui_element(object)?;
        if let Some(index) = index {
            self.owned_collection(parent, relation)?
                .insert_at(index, &replacement)?;
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
        if let Some(owner) = item {
            let identity = com_identity(&self.ui_element(object)?)?;
            self.event_queue
                .items
                .borrow_mut()
                .insert(object, NativeItem { owner, identity });
        }
        self.event_queue.discard_object(object);
        Ok(())
    }
}
