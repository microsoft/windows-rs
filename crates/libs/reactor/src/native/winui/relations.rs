impl WinUiAdapter {
    fn attach(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    ) -> Result<(), WinUiError> {
        if self.owners.contains_key(&child) {
            return Err(WinUiError::StillOwned(child));
        }
        if matches!(
            self.kind(child)?,
            ObjectType::ToolTip | ObjectType::ContentDialog
        ) {
            return Err(WinUiError::InvalidObject(child));
        }
        let child_element = self.ui_element(child)?;
        let data_content_key = self.data_content_key.clone();
        match (self.handle(parent)?, relation) {
            (Handle::Generated(value), relation) => {
                value
                    .set_content(relation, Some(&child_element))
                    .ok_or(WinUiError::InvalidRelation(parent, relation))??;
            }
            (Handle::TreeNode(parent), RelationId::Content) => {
                parent.value.SetContent(&child_element)?;
                parent.content = Some(child);
            }
            (Handle::Data(parent), RelationId::Content) => {
                let properties =
                    parent.cast::<IMap<HSTRING, IInspectable>>()?;
                let content: IInspectable = child_element.cast()?;
                properties.Insert(&data_content_key, &content)?;
                properties.Insert(&self.data_text_visibility_key, &data_string("Collapsed"))?;
            }
            _ => return Err(WinUiError::InvalidRelation(parent, relation)),
        }
        self.owners.insert(child, (parent, relation));
        Ok(())
    }

    fn detach(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    ) -> Result<(), WinUiError> {
        if self.owners.get(&child) != Some(&(parent, relation)) {
            return Err(WinUiError::ChildNotFound(child));
        }
        let data_content_key = self.data_content_key.clone();
        match (self.handle(parent)?, relation) {
            (Handle::Generated(value), relation) => {
                value
                    .set_content(relation, None)
                    .ok_or(WinUiError::InvalidRelation(parent, relation))??;
            }
            (Handle::TreeNode(parent), RelationId::Content) => {
                let content: IInspectable =
                    windows_reference::IReference::from(parent.text.clone()).into();
                parent.value.SetContent(&content)?;
                parent.content = None;
            }
            (Handle::Data(parent), RelationId::Content) => {
                let properties =
                    parent.cast::<IMap<HSTRING, IInspectable>>()?;
                properties.Insert(&data_content_key, None)?;
                properties.Insert(&self.data_text_visibility_key, &data_string("Visible"))?;
            }
            _ => return Err(WinUiError::InvalidRelation(parent, relation)),
        }
        self.owners.remove(&child);
        Ok(())
    }

    fn has_retirements(&self, parent: ObjectId, relation: RelationId) -> bool {
        self.retirements
            .values()
            .any(|retirement| retirement.parent == parent && retirement.relation == relation)
    }

    fn active_native_index(
        &self,
        parent: ObjectId,
        relation: RelationId,
        active_index: usize,
    ) -> Result<usize, WinUiError> {
        let retired = self
            .retirements
            .iter()
            .filter_map(|(root, retirement)| {
                (retirement.parent == parent && retirement.relation == relation).then_some(*root)
            })
            .collect::<Vec<_>>();
        if retired.is_empty() {
            return Ok(active_index);
        }
        let values = self.owned_collection(parent, relation)?;
        let retired = retired
            .into_iter()
            .map(|root| self.ui_element(root))
            .collect::<Result<Vec<_>, _>>()?;
        let mut active = 0;
        for index in 0..values.size()? {
            if retired.contains(&values.get_at(index)?) {
                continue;
            }
            if active == active_index {
                return Ok(index as usize);
            }
            active += 1;
        }
        if active == active_index {
            Ok(values.size()? as usize)
        } else {
            Err(WinUiError::InvalidMutation(relation))
        }
    }

    fn insert(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
        index: usize,
    ) -> Result<(), WinUiError> {
        if self.owners.contains_key(&child) {
            return Err(WinUiError::StillOwned(child));
        }
        if matches!(
            self.kind(child)?,
            ObjectType::ToolTip | ObjectType::ContentDialog
        ) {
            return Err(WinUiError::InvalidObject(child));
        }
        let index = self.active_native_index(parent, relation, index)?;
        let selection = selection_for_relation(self.kind(parent)?, relation);
        let selected = selection
            .map(|selection| self.read_selected_item(parent, selection))
            .transpose()?
            .flatten();
        let selection_item =
            selection.filter(|selection| self.kind(child).is_ok_and(|kind| kind == selection.item));
        let child_value = selection_item
            .map(|_| {
                self.ui_element(child)
                    .and_then(|value| value.cast::<IInspectable>().map_err(Into::into))
            })
            .transpose()?;
        let child_selected = selection_item
            .map(|selection| self.selection_item_is_selected(child, selection))
            .transpose()?
            .unwrap_or(false);
        let tracked = selection_item.is_some()
            || reorder_event(self.kind(parent)?, relation).is_some();
        let identity = tracked
            .then(|| self.item_identity(parent, relation, child))
            .transpose()?;
        let feedback = self.suppress_selection(parent, relation, selection)?;
        let result = (|| {
            match relation_contract(self.kind(parent)?, relation)?.realization {
                Realization::Owned => {
                    self.owned_collection(parent, relation)?
                        .insert_at(index32(index)?, &self.ui_element(child)?)?;
                }
                Realization::Structural => {
                    self.tree_nodes(parent)?
                        .InsertAt(index32(index)?, &self.tree_node(child)?)?;
                }
                Realization::Container => {
                    self.items(parent)?
                        .InsertAt(index32(index)?, &self.inspectable(child)?)?;
                }
            }
            self.owners.insert(child, (parent, relation));
            if let Some(identity) = identity {
                self.event_queue.items.borrow_mut().insert(
                    child,
                    NativeItem {
                        owner: parent,
                        identity,
                    },
                );
            }
            if let (Some(selection), Some(value)) = (selection_item, child_value.as_ref()) {
                if child_selected {
                    self.write_selected_item(parent, selection, Some(value))?;
                } else if let Some(selected) = selected.as_ref() {
                    self.write_selected_item(parent, selection, Some(selected))?;
                }
            }
            Ok(())
        })();
        self.release_selection(parent, feedback);
        result
    }

    fn remove(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
        index: usize,
    ) -> Result<(), WinUiError> {
        if self.owners.get(&child) != Some(&(parent, relation)) {
            return Err(WinUiError::ChildNotFound(child));
        }
        let index = self.active_native_index(parent, relation, index)?;
        let selection = selection_for_relation(self.kind(parent)?, relation);
        let selected = selection
            .map(|selection| self.read_selected_item(parent, selection))
            .transpose()?
            .flatten();
        let selection_item =
            selection.filter(|selection| self.kind(child).is_ok_and(|kind| kind == selection.item));
        let child_value = selection_item
            .map(|_| {
                self.ui_element(child)
                    .and_then(|value| value.cast::<IInspectable>().map_err(Into::into))
            })
            .transpose()?;
        let feedback = self.suppress_selection(parent, relation, selection)?;
        let result = (|| {
            match relation_contract(self.kind(parent)?, relation)?.realization {
                Realization::Owned => {
                    let values = self.owned_collection(parent, relation)?;
                    if values.get_at(index32(index)?)? != self.ui_element(child)? {
                        return Err(WinUiError::ChildNotFound(child));
                    }
                    values.remove_at(index32(index)?)?;
                }
                Realization::Structural => {
                    let values = self.tree_nodes(parent)?;
                    if values.GetAt(index32(index)?)? != self.tree_node(child)? {
                        return Err(WinUiError::ChildNotFound(child));
                    }
                    values.RemoveAt(index32(index)?)?;
                }
                Realization::Container => {
                    let values = self.items(parent)?;
                    if values.GetAt(index32(index)?)? != self.inspectable(child)? {
                        return Err(WinUiError::ChildNotFound(child));
                    }
                    values.RemoveAt(index32(index)?)?;
                }
            }
            self.owners.remove(&child);
            self.event_queue.items.borrow_mut().remove(&child);
            if let Some(selection) = selection
                && let Some(selected) = selected.as_ref()
                && child_value.as_ref() != Some(selected)
            {
                self.write_selected_item(parent, selection, Some(selected))?;
            }
            Ok(())
        })();
        self.release_selection(parent, feedback);
        result
    }

    fn reorder(
        &mut self,
        parent: ObjectId,
        relation: RelationId,
        moves: &[Move],
        children: &[ObjectId],
    ) -> Result<(), WinUiError> {
        let selection = selection_for_relation(self.kind(parent)?, relation);
        let selected = selection
            .map(|selection| self.read_selected_item(parent, selection))
            .transpose()?
            .flatten();
        let feedback = self.suppress_selection(parent, relation, selection)?;
        let result = (|| {
            match relation_contract(self.kind(parent)?, relation)?.realization {
                Realization::Owned if !self.has_retirements(parent, relation) => {
                    // Without retiring children, native order matches active order, so the
                    // reconciler's moves apply directly and unmoved children stay in place.
                    let values = self.owned_collection(parent, relation)?;
                    for movement in moves {
                        let child = self.ui_element(movement.child)?;
                        let previous = values
                            .index_of(&child)?
                            .ok_or(WinUiError::ChildNotFound(movement.child))?;
                        values.remove_at(previous)?;
                        let index = if let Some(before) = movement.before {
                            values
                                .index_of(&self.ui_element(before)?)?
                                .ok_or(WinUiError::ChildNotFound(before))?
                        } else {
                            values.size()?
                        };
                        values.insert_at(index, &child)?;
                    }
                }
                Realization::Owned => {
                    let values = self.owned_collection(parent, relation)?;
                    let mut current = (0..values.size()?)
                        .map(|index| {
                            let value = values.get_at(index)?;
                            Ok((com_identity(&value)?, value))
                        })
                        .collect::<Result<Vec<_>, WinUiError>>()?;
                    let desired = children
                        .iter()
                        .map(|child| {
                            self.ui_element(*child)
                                .and_then(|value| com_identity(&value))
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    let active = desired.iter().copied().collect::<HashSet<_>>();
                    let swaps = simulate_active_slot_reorder(&mut current, &active, &desired)
                        .ok_or(WinUiError::InvalidMutation(relation))?;
                    for (left, right, left_value, right_value) in swaps {
                        values.remove_at(index32(right)?)?;
                        values.remove_at(index32(left)?)?;
                        values.insert_at(index32(left)?, &left_value)?;
                        values.insert_at(index32(right)?, &right_value)?;
                    }
                }
                Realization::Structural => {
                    let values = self.tree_nodes(parent)?;
                    for movement in moves {
                        let child = self.tree_node(movement.child)?;
                        let mut previous = 0;
                        if !values.IndexOf(&child, &mut previous)? {
                            return Err(WinUiError::ChildNotFound(movement.child));
                        }
                        values.RemoveAt(previous)?;
                        let index = if let Some(before) = movement.before {
                            let before = self.tree_node(before)?;
                            let mut index = 0;
                            if !values.IndexOf(&before, &mut index)? {
                                return Err(WinUiError::ChildNotFound(movement.child));
                            }
                            index
                        } else {
                            values.Size()?
                        };
                        values.InsertAt(index, &child)?;
                    }
                }
                Realization::Container => {
                    let values = self.items(parent)?;
                    if moves.len().saturating_add(1) >= children.len() {
                        let children = children
                            .iter()
                            .map(|child| self.inspectable(*child).map(Some))
                            .collect::<Result<Vec<_>, _>>()?;
                        values.ReplaceAll(&children)?;
                    } else {
                        for movement in moves {
                            let child = self.inspectable(movement.child)?;
                            let mut previous = 0;
                            if !values.IndexOf(&child, &mut previous)? {
                                return Err(WinUiError::ChildNotFound(movement.child));
                            }
                            values.RemoveAt(previous)?;
                            let index = if let Some(before) = movement.before {
                                let before = self.inspectable(before)?;
                                let mut index = 0;
                                if !values.IndexOf(&before, &mut index)? {
                                    return Err(WinUiError::ChildNotFound(movement.child));
                                }
                                index
                            } else {
                                values.Size()?
                            };
                            values.InsertAt(index, &child)?;
                        }
                    }
                }
            }
            if let (Some(selection), Some(selected)) = (selection, selected.as_ref()) {
                self.write_selected_item(parent, selection, Some(selected))?;
            }
            Ok(())
        })();
        self.release_selection(parent, feedback);
        result?;
        let active = children.iter().copied().collect::<HashSet<_>>();
        debug_assert_eq!(
            children.len(),
            self.owners
                .iter()
                .filter(|(object, owner)| **owner == (parent, relation) && active.contains(object))
                .count()
        );
        Ok(())
    }

    fn selection_feedback(
        &self,
        object: ObjectId,
        property: PropertyId,
    ) -> Result<Option<(ObjectId, EventId)>, WinUiError> {
        let Some((owner, relation)) = self.owners.get(&object).copied() else {
            return Ok(None);
        };
        Ok(
            selection_for_item_property(self.kind(owner)?, relation, self.kind(object)?, property)
                .map(|selection| (owner, selection.event)),
        )
    }

    /// Drops the owner's selection notifications while Reactor mutates a relation whose children
    /// carry the selection, or whose index the same batch writes afterwards.
    fn suppress_selection(
        &self,
        owner: ObjectId,
        relation: RelationId,
        selection: Option<SelectionContract>,
    ) -> Result<Option<EventId>, WinUiError> {
        let event = match selection {
            Some(selection) => Some(selection.event),
            None => self.index_event(owner, IndexItems::Relation(relation))?,
        };
        if let Some(event) = event {
            self.event_queue
                .feedback
                .borrow_mut()
                .suppress(owner, event);
        }
        Ok(event)
    }

    /// The owner's index event when the batch being applied writes the index after `items`.
    pub(super) fn index_event(
        &self,
        owner: ObjectId,
        items: IndexItems,
    ) -> Result<Option<EventId>, WinUiError> {
        Ok(index_for_items(self.kind(owner)?, items)
            .filter(|contract| self.index_writes.contains(&(owner, contract.property)))
            .map(|contract| contract.event))
    }

    fn release_selection(&self, owner: ObjectId, event: Option<EventId>) {
        if let Some(event) = event {
            self.event_queue.feedback.borrow_mut().release(owner, event);
        }
    }

    fn item_identity(
        &self,
        parent: ObjectId,
        relation: RelationId,
        child: ObjectId,
    ) -> Result<usize, WinUiError> {
        match relation_contract(self.kind(parent)?, relation)?.realization {
            Realization::Owned => com_identity(&self.ui_element(child)?),
            Realization::Structural => com_identity(&self.tree_node(child)?),
            Realization::Container => com_identity(&self.inspectable(child)?),
        }
    }

    fn read_selected_item(
        &self,
        object: ObjectId,
        selection: SelectionContract,
    ) -> Result<Option<IInspectable>, WinUiError> {
        let Some(Handle::Generated(handle)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        handle
            .selected_item(selection.event)
            .unwrap_or(Err(WinUiError::InvalidObject(object)))
    }

    fn write_selected_item(
        &self,
        object: ObjectId,
        selection: SelectionContract,
        selected: Option<&IInspectable>,
    ) -> Result<(), WinUiError> {
        let Some(Handle::Generated(handle)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        handle
            .set_selected_item(selection.event, selected)
            .unwrap_or(Err(WinUiError::InvalidObject(object)))
    }

    fn selection_item_is_selected(
        &self,
        object: ObjectId,
        selection: SelectionContract,
    ) -> Result<bool, WinUiError> {
        let Some(Handle::Generated(handle)) = self.handles.get(&object) else {
            return Err(WinUiError::InvalidObject(object));
        };
        handle
            .selection_item_is_selected(selection.selected_property)
            .unwrap_or(Err(WinUiError::InvalidObject(object)))
    }

    fn handle(&mut self, object: ObjectId) -> Result<&mut Handle, WinUiError> {
        self.handles
            .get_mut(&object)
            .ok_or(WinUiError::MissingObject(object))
    }

    fn kind(&self, object: ObjectId) -> Result<ObjectType, WinUiError> {
        Ok(
            match self
                .handles
                .get(&object)
                .ok_or(WinUiError::MissingObject(object))?
            {
                Handle::Generated(value) => value.kind(),
                Handle::TreeView(_) => ObjectType::TreeView,
                Handle::TreeNode(_) => ObjectType::TreeNode,
                Handle::Data(_) => ObjectType::DataItem,
            },
        )
    }

    fn ui_element(&self, object: ObjectId) -> Result<native::UIElement, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::Generated(value) => value.ui_element(),
            Handle::TreeView(value) => Ok(value.value.cast()?),
            Handle::TreeNode(_) | Handle::Data(_) => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn inspectable(&self, object: ObjectId) -> Result<IInspectable, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::Data(value) => Ok(value.cast()?),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn tree_node(&self, object: ObjectId) -> Result<native::TreeViewNode, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::TreeNode(value) => Ok(value.value.clone()),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn owned_collection(
        &self,
        object: ObjectId,
        relation: RelationId,
    ) -> Result<GeneratedCollection, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::Generated(value) => value
                .owned_collection(relation)
                .unwrap_or(Err(WinUiError::InvalidObject(object))),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn tree_nodes(
        &self,
        object: ObjectId,
    ) -> Result<IVector<native::TreeViewNode>, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::TreeView(value) => Ok(value.value.RootNodes()?),
            Handle::TreeNode(value) => Ok(value.value.Children()?),
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn items(&self, object: ObjectId) -> Result<native::ItemCollection, WinUiError> {
        match self
            .handles
            .get(&object)
            .ok_or(WinUiError::MissingObject(object))?
        {
            Handle::Generated(GeneratedHandle::ListView(value)) => {
                Ok(value.value.cast::<native::IItemsControl>()?.Items()?)
            }
            _ => Err(WinUiError::InvalidObject(object)),
        }
    }

    fn set_data_text(&mut self, object: ObjectId, text: &str) -> Result<(), WinUiError> {
        let text_key = self.data_text_key.clone();
        let Handle::Data(value) = self.handle(object)? else {
            return Err(WinUiError::InvalidObject(object));
        };
        let properties = value.cast::<IMap<HSTRING, IInspectable>>()?;
        properties.Insert(&text_key, &data_string(text))?;
        Ok(())
    }

    fn tree_template(&mut self) -> Result<native::DataTemplate, WinUiError> {
        if let Some(template) = &self.tree_template {
            return Ok(template.clone());
        }
        // A ContentPresenter root would take its text content as its own DataContext, so the
        // binding would stop following the node: renamed or recycled items kept stale text.
        let template = native::XamlReader::Load(
            "<DataTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation'>\
             <ContentControl IsTabStop='False' Content='{Binding Content}'/>\
             </DataTemplate>",
        )?
        .cast::<native::DataTemplate>()?;
        self.tree_template = Some(template.clone());
        Ok(template)
    }

    fn list_template(&mut self) -> Result<native::DataTemplate, WinUiError> {
        if let Some(template) = &self.list_template {
            return Ok(template.clone());
        }
        let template = native::XamlReader::Load(
            "<DataTemplate xmlns='http://schemas.microsoft.com/winfx/2006/xaml/presentation'>\
             <Grid>\
             <TextBlock HorizontalAlignment='Left' Text='{Binding [Text]}' \
             Visibility='{Binding [TextVisibility]}'/>\
             <ContentPresenter Content='{Binding [Content]}'/>\
             </Grid>\
             </DataTemplate>",
        )?
        .cast::<native::DataTemplate>()?;
        self.list_template = Some(template.clone());
        Ok(template)
    }

    fn start_retirement(
        &mut self,
        root: ObjectId,
        nodes: Vec<ObjectId>,
        parent: ObjectId,
        relation: RelationId,
        duration: Duration,
    ) -> Result<(), WinUiError> {
        if self.retirements.contains_key(&root)
            || self.owners.get(&root) != Some(&(parent, relation))
            || nodes.iter().any(|node| !self.handles.contains_key(node))
        {
            return Err(WinUiError::MissingObject(root));
        }
        let duration =
            TimeSpan::try_from(duration).map_err(|_| WinUiError::InvalidDuration)?;
        let root_element = self.ui_element(root)?;
        let transition = native::ScalarTransition::new()?;
        transition.SetDuration(duration)?;
        root_element.SetOpacityTransition(&transition)?;
        let timer = native::DispatcherQueue::GetForCurrentThread()?.CreateTimer()?;
        timer.SetInterval(duration)?;
        timer.SetIsRepeating(false)?;
        let event_queue = Rc::clone(&self.event_queue);
        let tick = timer.Tick(move |_, _| {
            event_queue
                .events
                .borrow_mut()
                .push_back(QueuedNativeEvent {
                    observation: None,
                    event: None,
                    retirement: Some(RetirementCompletion { root }),
                    realization: None,
                });
            Self::schedule_event_wake(&event_queue);
        })?;
        self.retirements.insert(
            root,
            NativeRetirement {
                nodes,
                parent,
                relation,
                timer: timer.clone(),
                _tick: tick,
            },
        );
        if let Err(error) = root_element.SetOpacity(0.0) {
            self.retirements.remove(&root);
            return Err(error.into());
        }
        if let Err(error) = timer.Start() {
            self.retirements.remove(&root);
            return Err(error.into());
        }
        Ok(())
    }

    fn complete_retirement(
        &mut self,
        root: ObjectId,
        nodes: &[ObjectId],
    ) -> Result<(), WinUiError> {
        let retirement = self
            .retirements
            .get(&root)
            .ok_or(WinUiError::MissingObject(root))?;
        if retirement.nodes != nodes {
            return Err(WinUiError::InvalidObject(root));
        }
        retirement.timer.Stop()?;
        let parent = retirement.parent;
        let relation = retirement.relation;
        let values = self.owned_collection(parent, relation)?;
        let root_value = self.ui_element(root)?;
        let mut index = 0;
        while index < values.size()? && values.get_at(index)? != root_value {
            index += 1;
        }
        if index == values.size()? {
            return Err(WinUiError::ChildNotFound(root));
        }
        values.remove_at(index)?;
        self.retirements.remove(&root);
        for object in nodes {
            self.owners.remove(object);
            let handle = self
                .handles
                .remove(object)
                .ok_or(WinUiError::MissingObject(*object))?;
            self.release_object(*object, handle)?;
        }
        Ok(())
    }

    fn native_event(&self, queued: &QueuedNativeEvent) -> Option<NativeEvent> {
        if let Some(completion) = queued.retirement {
            return Some(NativeEvent::retirement(completion));
        }
        if let Some(request) = queued.realization {
            return Some(NativeEvent::realization(request));
        }
        // Native handlers queue reorders without an order; reading it here keeps queued
        // notifications from replaying an order that a later render has already replaced.
        let mut observation = queued.observation.clone();
        let mut order = None;
        if let Some(Observation::Reorder {
            object,
            relation,
            children,
        }) = &mut observation
        {
            if !self.handles.contains_key(object) {
                return None;
            }
            let native = self.native_children(*object, *relation).unwrap();
            children.clone_from(&native);
            order = Some(EventPayload::Items(native));
        }
        let event = queued.event.as_ref().and_then(|queued| {
            let callback = match (&queued.handler, queued.event) {
                (QueuedHandler::Presentation(revision), EventId::MenuItemInvoked) => {
                    let menu = self.menus.get(&queued.object)?;
                    (menu.revision == *revision).then(|| EventValue::Key(menu.menu.on_click.clone()))
                }
                (QueuedHandler::Presentation(revision), EventId::CommandInvoked) => {
                    let flyout = self.command_bar_flyouts.get(&queued.object)?;
                    (flyout.revision == *revision)
                        .then(|| EventValue::Key(flyout.flyout.on_click.clone()))
                }
                (QueuedHandler::Callback { callback, live }, _) => {
                    live.get().then(|| callback.clone())
                }
                (QueuedHandler::Presentation(_), event) => {
                    unreachable!("{event:?} has no presentation")
                }
            }?;
            Some(EventDispatch::new(
                queued.object,
                queued.event,
                callback,
                order.unwrap_or_else(|| queued.payload.clone()),
            ))
        });
        Some(NativeEvent::new(observation, event))
    }

    /// Returns the registered children of a reorderable relation in native order.
    fn native_children(
        &self,
        object: ObjectId,
        relation: RelationId,
    ) -> Result<Vec<ObjectId>, WinUiError> {
        let identities = match relation_contract(self.kind(object)?, relation)?.realization {
            Realization::Owned => {
                let values = self.owned_collection(object, relation)?;
                (0..values.size()?)
                    .map(|index| com_identity(&values.get_at(index)?))
                    .collect::<Result<Vec<_>, _>>()?
            }
            Realization::Container => {
                let values = self.items(object)?;
                (0..values.Size()?)
                    .map(|index| com_identity(&values.GetAt(index)?))
                    .collect::<Result<Vec<_>, _>>()?
            }
            Realization::Structural => return Err(WinUiError::InvalidRelation(object, relation)),
        };
        let items = self.event_queue.items.borrow();
        let registered = items
            .iter()
            .filter(|(_, item)| item.owner == object)
            .map(|(child, item)| (item.identity, *child))
            .collect::<HashMap<_, _>>();
        Ok(identities
            .iter()
            .filter_map(|identity| registered.get(identity).copied())
            .collect())
    }
}

fn data_string(value: &str) -> IInspectable {
    windows_reference::IReference::from(HSTRING::from(value)).into()
}
