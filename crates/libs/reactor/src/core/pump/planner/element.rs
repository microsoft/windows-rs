//! Planning for plain [`Element`] trees: native/virtual reconciliation and
//! mounting, independent of [`View`]/component structure.

use super::super::*;

impl<R: NativeRuntime> Pump<R> {
    fn visit_element_properties(
        props: &MountedProps,
        element_state: Option<&ElementState>,
        visit: &mut dyn FnMut(PropertyId, Option<PropertyValueRef<'_>>),
    ) {
        props.visit_properties(visit);
        visit_element_state(element_state, visit);
    }

    pub(in super::super) fn native_properties_match(
        native: &NativeState,
        props: &MountedProps,
        element_state: Option<&ElementState>,
    ) -> bool {
        let mut matches = true;
        Self::visit_element_properties(props, element_state, &mut |property, value| {
            matches &= native.properties.get(&property).map_or_else(
                || value.is_none(),
                |current| match (current.as_ref(), value) {
                    (Some(current), Some(value)) => value.equals_owned(current),
                    (None, None) => true,
                    _ => false,
                },
            );
        });
        matches
    }

    fn plan_native_properties(
        native: &NativeState,
        node: NodeId,
        props: &MountedProps,
        element_state: Option<&ElementState>,
        plan: &mut UpdatePlan,
    ) {
        Self::visit_element_properties(props, element_state, &mut |property, value| {
            let changed = native.properties.get(&property).map_or_else(
                || value.is_some(),
                |current| match (current.as_ref(), value) {
                    (Some(current), Some(value)) => !value.equals_owned(current),
                    (None, None) => false,
                    _ => true,
                },
            );
            if !changed {
                return;
            }
            let value = value.map(PropertyValueRef::into_owned);
            let command = match &value {
                Some(value) => Command::SetProperty {
                    node,
                    property,
                    value: value.clone(),
                },
                None => Command::ClearProperty { node, property },
            };
            plan.push(command);
            plan.commits.push(PropertyCommit {
                node,
                property,
                value,
            });
        });
        Self::plan_theme_style(native, node, props, plan);
    }

    fn plan_theme_style(
        native: &NativeState,
        node: NodeId,
        props: &MountedProps,
        plan: &mut UpdatePlan,
    ) {
        let style = props.theme_style();
        if native.desired.theme_style() != style {
            plan.push(Command::SetThemeStyle { node, style });
        }
    }

    pub(super) fn element_structure_is_empty(element: &Element) -> bool {
        match element.structure() {
            ElementStructureRef::None | ElementStructureRef::Content(None) => true,
            ElementStructureRef::Children(children) => children.is_empty(),
            ElementStructureRef::Virtual(_) => false,
            ElementStructureRef::Content(Some(_)) => false,
        }
    }

    pub(in super::super) fn plan_local_native_state(
        native: &NativeState,
        node: NodeId,
        parts: ElementParts,
        plan: &mut UpdatePlan,
    ) -> Result<
        (
            MountedProps,
            Option<ExitTransition>,
            Option<NativeElementRef>,
        ),
        PumpError,
    > {
        let exit_transition = parts
            .element_state
            .as_deref()
            .and_then(ElementState::exit_transition);
        Self::plan_native_properties(
            native,
            node,
            &parts.props,
            parts.element_state.as_deref(),
            plan,
        );
        Self::plan_reference(native, node, parts.reference.clone(), plan);
        Ok((parts.props, exit_transition, parts.reference))
    }

    pub(in super::super) fn reconcile_node(
        tree: &mut Tree,
        node: NodeId,
        element: Element,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        let desired_kind = element.kind();
        let compatible = match tree.kind(node) {
            NodeKind::Native(kind) => kind == desired_kind,
            NodeKind::VirtualCollection => desired_kind == MountedKind::ItemsRepeater,
            _ => false,
        };
        if !compatible {
            return Self::replace_planned_node(tree, node, element, plan);
        }
        if !plan.reconcile_observations && Self::node_matches_element(tree, node, &element)? {
            return Ok(node);
        }

        let parts = element.into_parts();
        if tree.kind(node) == NodeKind::VirtualCollection {
            let ElementStructure::Virtual(items) = parts.structure else {
                return Err(PumpError::StructureUnsupported);
            };
            let changed_keys =
                items.changed_keys(tree.virtual_items(node), tree.virtual_model(node).keys());
            if let Some(keys) = changed_keys {
                for child in tree.children(node).to_vec() {
                    Self::retire_planned_subtree(tree, child, plan)?;
                }
                let source_revision = {
                    let model = tree.virtual_model_mut(node);
                    model
                        .update(keys)
                        .map_err(|DuplicateKeyError(key)| PumpError::DuplicateKey(key))?;
                    model.source_revision()
                };
                tree.update_virtual_items(node, items);
                tree.virtual_model_mut(node).clear();
                plan.push(Command::ResetVirtualCollection {
                    node,
                    item_count: tree.virtual_items(node).len(),
                    source_revision,
                });
            } else {
                tree.update_virtual_items(node, items);
                if !tree.children(node).is_empty() {
                    return Err(PumpError::StructureUnsupported);
                }
            }
            return Ok(node);
        }
        let NodeKind::Native(kind) = tree.kind(node) else {
            return Err(PumpError::NotMounted);
        };
        debug_assert_eq!(kind, parts.kind);

        let props_changed = tree.native(node).desired != parts.props;
        let desired_reference = parts.reference.clone();
        Self::plan_native_properties(
            tree.native(node),
            node,
            &parts.props,
            parts.element_state.as_deref(),
            plan,
        );
        if props_changed {
            Self::update_event_states(tree.native_mut(node), node, &parts.props, plan)?;
            tree.native_mut(node).desired = parts.props;
        }
        Self::plan_reference(tree.native(node), node, desired_reference, plan);
        tree.set_window_title_bar(node, parts.window_title_bar);

        let current_children = tree.children(node).to_vec();
        match parts.structure {
            ElementStructure::None => {
                if !current_children.is_empty() {
                    return Err(PumpError::StructureUnsupported);
                }
            }
            ElementStructure::Content(content) => match (current_children.as_slice(), content) {
                ([], None) => {}
                ([], Some(content)) => {
                    let child = Self::mount_planned_element(tree, Some(node), None, content, plan)?;
                    plan.push(Command::InsertChild {
                        parent: node,
                        slot: None,
                        child,
                        index: 0,
                    });
                }
                ([child], None) => {
                    Self::retire_planned_subtree(tree, *child, plan)?;
                }
                ([child], Some(content)) => {
                    Self::reconcile_node(tree, *child, content, plan)?;
                }
                _ => return Err(PumpError::StructureUnsupported),
            },
            ElementStructure::Children(children) => {
                if current_children.len() == children.len()
                    && current_children
                        .iter()
                        .zip(children.iter())
                        .all(|(child, desired)| tree.key(*child) == Some(desired.key()))
                {
                    let mut replacements = Vec::new();
                    for (index, (child, desired)) in current_children
                        .iter()
                        .copied()
                        .zip(children.iter())
                        .enumerate()
                    {
                        if !plan.reconcile_observations
                            && Self::node_matches_element(tree, child, desired.element())?
                        {
                        } else {
                            let reconciled =
                                Self::reconcile_node(tree, child, desired.element().clone(), plan)?;
                            if reconciled != child {
                                replacements.push((index, reconciled));
                            }
                        }
                    }
                    if !replacements.is_empty() {
                        let mut children = current_children;
                        for (index, replacement) in replacements {
                            children[index] = replacement;
                        }
                        tree.set_children(node, children);
                    }
                    return Ok(node);
                }

                let old_native = Self::native_children(tree, node)?;
                let old_keys = current_children
                    .iter()
                    .map(|child| {
                        tree.key(*child)
                            .cloned()
                            .ok_or(PumpError::StructureUnsupported)
                    })
                    .collect::<Result<Vec<_>, PumpError>>()?;
                let new_keys = children
                    .iter()
                    .map(|child| child.key().clone())
                    .collect::<Vec<_>>();
                let operations = diff(&old_keys, &new_keys)
                    .map_err(|DuplicateKeyError(key)| PumpError::DuplicateKey(key))?;
                let new_key_set = new_keys.iter().cloned().collect::<HashSet<_>>();
                let mut nodes = old_keys
                    .iter()
                    .cloned()
                    .zip(current_children.iter().copied())
                    .collect::<HashMap<_, _>>();
                for (key, child) in old_keys.iter().zip(current_children.iter().copied()) {
                    if !new_key_set.contains(key) {
                        Self::retire_planned_subtree(tree, child, plan)?;
                    }
                }
                for child in children.iter() {
                    if let Some(child_node) = nodes.get(child.key()).copied() {
                        let reconciled = if !plan.reconcile_observations
                            && Self::node_matches_element(tree, child_node, child.element())?
                        {
                            child_node
                        } else {
                            Self::reconcile_node(tree, child_node, child.element().clone(), plan)?
                        };
                        if reconciled != child_node {
                            nodes.insert(child.key().clone(), reconciled);
                        }
                    }
                }
                for child in children.iter() {
                    if !nodes.contains_key(child.key()) {
                        let child_node = Self::mount_planned_element(
                            tree,
                            Some(node),
                            Some(child.key().clone()),
                            child.element().clone(),
                            plan,
                        )?;
                        nodes.insert(child.key().clone(), child_node);
                    }
                }
                let order = new_keys
                    .iter()
                    .map(|key| {
                        nodes
                            .get(key)
                            .copied()
                            .ok_or(PumpError::StructureUnsupported)
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                tree.set_children(node, order);
                let new_native = Self::native_children(tree, node)?;
                if super::is_dense_keyed_update(&operations) && old_native != new_native {
                    plan.synchronize_children(node, None, new_native);
                } else {
                    Self::replay_keyed_child_list(
                        tree, node, None, old_keys, operations, &nodes, plan,
                    )?;
                }
            }
            ElementStructure::Virtual(_) => return Err(PumpError::StructureUnsupported),
        }
        Ok(node)
    }

    pub(in super::super) fn node_matches_element(
        tree: &Tree,
        node: NodeId,
        element: &Element,
    ) -> Result<bool, PumpError> {
        let kind = tree.kind(node);
        let compatible = match kind {
            NodeKind::Native(mounted) => mounted == element.kind(),
            NodeKind::VirtualCollection => element.kind() == MountedKind::ItemsRepeater,
            _ => false,
        };
        if !compatible {
            return Ok(false);
        }
        if let NodeKind::Native(_) = kind
            && tree.native(node).reference.as_ref() != element.reference()
        {
            return Ok(false);
        }
        if let NodeKind::Native(_) = kind
            && tree.node_window_title_bar(node) != element.window_title_bar()
        {
            return Ok(false);
        }
        if kind == NodeKind::VirtualCollection {
            let ElementStructureRef::Virtual(items) = element.structure() else {
                return Ok(false);
            };
            return Ok(tree.virtual_items(node) == items);
        }
        if !element.props_match(&tree.native(node).desired) {
            return Ok(false);
        }
        let native = tree.native(node);
        if tree.exit_transition(node)
            != element
                .element_state()
                .and_then(ElementState::exit_transition)
        {
            return Ok(false);
        }
        if !Self::native_properties_match(native, &native.desired, element.element_state()) {
            return Ok(false);
        }

        let children = tree.children(node);
        match element.structure() {
            ElementStructureRef::None => Ok(children.is_empty()),
            ElementStructureRef::Content(content) => match (children, content) {
                ([], None) => Ok(true),
                ([child], Some(content)) => Self::node_matches_element(tree, *child, content),
                _ => Ok(false),
            },
            ElementStructureRef::Children(desired) => {
                if children.len() != desired.len() {
                    return Ok(false);
                }
                for (child, desired) in children.iter().zip(desired) {
                    if tree.key(*child) != Some(desired.key())
                        || !Self::node_matches_element(tree, *child, desired.element())?
                    {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            ElementStructureRef::Virtual(_) => Ok(false),
        }
    }

    fn replace_planned_node(
        tree: &mut Tree,
        node: NodeId,
        element: Element,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        let parent = tree.parent(node).ok_or(PumpError::StructureUnsupported)?;
        let key = tree.key(node).cloned();
        let realized = if tree.kind(parent) == NodeKind::VirtualCollection {
            let container = tree
                .realized_container_for_logical(parent, node)
                .ok_or(PumpError::StructureUnsupported)?;
            Some((
                container,
                tree.realized(parent, container)
                    .ok_or(PumpError::StructureUnsupported)?
                    .index,
            ))
        } else {
            None
        };
        let index = tree
            .children(parent)
            .iter()
            .position(|child| *child == node)
            .ok_or(PumpError::StructureUnsupported)?;
        Self::retire_planned_subtree(tree, node, plan)?;
        let replacement = Self::mount_planned_element(tree, Some(parent), key, element, plan)?;
        let mut children = tree.children(parent).to_vec();
        let appended = children
            .iter()
            .position(|child| *child == replacement)
            .ok_or(PumpError::StructureUnsupported)?;
        children.remove(appended);
        children.insert(index, replacement);
        tree.set_children(parent, children);
        if let Some((container, source_index)) = realized {
            tree.set_realized(parent, container, source_index, replacement, None);
            Self::refresh_virtual_row_attachment(tree, parent, container, plan)?;
        } else {
            plan.push(Command::InsertChild {
                parent,
                slot: None,
                child: replacement,
                index,
            });
        }
        Ok(replacement)
    }

    fn update_event_states(
        native: &mut NativeState,
        node: NodeId,
        desired: &MountedProps,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let mut desired_events = Vec::new();
        desired.visit_events(&mut |event, active| {
            desired_events.push((event, active));
        });
        for (event, active) in desired_events {
            let state = native.events.entry(event).or_insert(EventState {
                revision: 0,
                active: false,
            });
            if state.active != active {
                state.revision = state.revision.checked_add(1).unwrap();
                state.active = active;
                if active {
                    plan.push(Command::SubscribeEvent {
                        node,
                        event,
                        revision: state.revision,
                    });
                } else {
                    plan.push(Command::UnsubscribeEvent { node, event });
                }
            }
        }
        Ok(())
    }

    pub(super) fn reconcile_shallow_control(
        tree: &mut Tree,
        node: NodeId,
        control: Element,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        if !Self::element_structure_is_empty(&control) {
            return Err(PumpError::StructureUnsupported);
        }
        let parts = control.into_parts();
        Self::reconcile_shallow_parts(tree, node, parts, plan)
    }

    fn reconcile_shallow_parts(
        tree: &mut Tree,
        node: NodeId,
        parts: ElementParts,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        if tree.kind(node) != NodeKind::Native(parts.kind) {
            return Err(PumpError::StructureUnsupported);
        }
        let native = tree.native(node);
        let exit_transition = parts
            .element_state
            .as_deref()
            .and_then(ElementState::exit_transition);
        if !plan.reconcile_observations
            && native.desired == parts.props
            && native.reference == parts.reference
            && tree.node_window_title_bar(node) == parts.window_title_bar
            && tree.exit_transition(node) == exit_transition
            && Self::native_properties_match(native, &parts.props, parts.element_state.as_deref())
        {
            return Ok(());
        }
        tree.set_exit_transition(node, exit_transition);
        tree.set_window_title_bar(node, parts.window_title_bar);
        Self::reconcile_native_state(
            tree.native_mut(node),
            node,
            parts.props,
            parts.reference,
            parts.element_state,
            plan,
        )
    }

    fn reconcile_native_state(
        native: &mut NativeState,
        node: NodeId,
        props: MountedProps,
        reference: Option<NativeElementRef>,
        element_state: Option<Rc<ElementState>>,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let props_changed = native.desired != props;
        Self::plan_native_properties(native, node, &props, element_state.as_deref(), plan);
        if props_changed {
            Self::update_event_states(native, node, &props, plan)?;
            native.desired = props;
        }
        Self::plan_reference(native, node, reference, plan);
        Ok(())
    }

    fn plan_reference(
        native: &NativeState,
        node: NodeId,
        desired: Option<NativeElementRef>,
        plan: &mut UpdatePlan,
    ) {
        if native.reference != desired {
            plan.reference_commits.push(ReferenceCommit {
                node,
                old: native.reference.clone(),
                new: desired,
            });
        }
    }

    fn plan_initial_native_state(
        tree: &Tree,
        node: NodeId,
        props: &MountedProps,
        reference: &Option<NativeElementRef>,
        element_state: Option<&ElementState>,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        if reference.is_some() {
            plan.reference_commits.push(ReferenceCommit {
                node,
                old: None,
                new: reference.clone(),
            });
        }
        Self::visit_element_properties(props, element_state, &mut |property, value| {
            if let Some(value) = value {
                let value = value.into_owned();
                plan.push(Command::SetProperty {
                    node,
                    property,
                    value: value.clone(),
                });
                plan.commits.push(PropertyCommit {
                    node,
                    property,
                    value: Some(value),
                });
            }
        });
        let style = props.theme_style();
        if !style.is_empty() {
            plan.push(Command::SetThemeStyle { node, style });
        }
        for (event, state) in &tree.native(node).events {
            if state.active {
                plan.push(Command::SubscribeEvent {
                    node,
                    event: *event,
                    revision: state.revision,
                });
            }
        }
        Ok(())
    }

    pub(in super::super) fn reconcile_native_values(
        tree: &mut Tree,
        node: NodeId,
        props: MountedProps,
        reference: Option<NativeElementRef>,
        element_state: Option<Rc<ElementState>>,
        window_title_bar: Option<WindowTitleBarHeight>,
        plan: &mut UpdatePlan,
    ) -> Result<(), PumpError> {
        let native = tree.native(node);
        let exit_transition = element_state
            .as_deref()
            .and_then(ElementState::exit_transition);
        if !plan.reconcile_observations
            && native.desired == props
            && native.reference == reference
            && tree.node_window_title_bar(node) == window_title_bar
            && tree.exit_transition(node) == exit_transition
            && Self::native_properties_match(native, &props, element_state.as_deref())
        {
            return Ok(());
        }
        tree.set_exit_transition(node, exit_transition);
        tree.set_window_title_bar(node, window_title_bar);
        Self::reconcile_native_state(
            tree.native_mut(node),
            node,
            props,
            reference,
            element_state,
            plan,
        )
    }

    pub(in super::super) fn mount_planned_element(
        tree: &mut Tree,
        parent: Option<NodeId>,
        key: Option<Key>,
        element: Element,
        plan: &mut UpdatePlan,
    ) -> Result<NodeId, PumpError> {
        let parts = element.into_parts();
        if let ElementStructure::Virtual(items) = parts.structure {
            if parts.kind != MountedKind::ItemsRepeater {
                return Err(PumpError::StructureUnsupported);
            }
            let item_count = items.len();
            let node = tree
                .insert_virtual_items(plan.identity, parent, key, parts.props.clone(), items)
                .map_err(|DuplicateKeyError(key)| PumpError::DuplicateKey(key))?;
            tree.set_exit_transition(
                node,
                parts
                    .element_state
                    .as_deref()
                    .and_then(ElementState::exit_transition),
            );
            plan.push(Command::CreateVirtualCollection {
                node,
                item_count,
                source_revision: 0,
            });
            Self::plan_initial_native_state(
                tree,
                node,
                &parts.props,
                &parts.reference,
                parts.element_state.as_deref(),
                plan,
            )?;
            return Ok(node);
        }
        let node = tree.insert_native(
            parent,
            parts.kind,
            key,
            parts.props.clone(),
            parts.window_title_bar,
        );
        tree.set_exit_transition(
            node,
            parts
                .element_state
                .as_deref()
                .and_then(ElementState::exit_transition),
        );
        plan.push(Command::Create {
            node,
            kind: parts.kind,
        });
        Self::plan_initial_native_state(
            tree,
            node,
            &parts.props,
            &parts.reference,
            parts.element_state.as_deref(),
            plan,
        )?;

        match parts.structure {
            ElementStructure::None => {}
            ElementStructure::Content(content) => {
                if let Some(content) = content {
                    let child = Self::mount_planned_element(tree, Some(node), None, content, plan)?;
                    plan.push(Command::InsertChild {
                        parent: node,
                        slot: None,
                        child,
                        index: 0,
                    });
                }
            }
            ElementStructure::Children(children) => {
                let children = Rc::unwrap_or_clone(children);
                let keys = children
                    .iter()
                    .map(|child| child.key().clone())
                    .collect::<Vec<_>>();
                diff(&[], &keys).map_err(|DuplicateKeyError(key)| PumpError::DuplicateKey(key))?;
                for (index, child) in children.into_iter().enumerate() {
                    let (key, child) = child.into_parts();
                    let child =
                        Self::mount_planned_element(tree, Some(node), Some(key), child, plan)?;
                    plan.push(Command::InsertChild {
                        parent: node,
                        slot: None,
                        child,
                        index,
                    });
                }
            }
            ElementStructure::Virtual(_) => return Err(PumpError::StructureUnsupported),
        }
        Ok(node)
    }
}
