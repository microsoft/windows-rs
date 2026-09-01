use super::*;

pub(super) const EVENT_WORK_BUDGET: usize = 64;
pub(super) const IMPERATIVE_WORK_BUDGET: usize = 64;
pub(super) const REALIZATION_WORK_BUDGET: usize = 32;

impl<R: NativeRuntime> Pump<R> {
    pub fn native_work_pending(&self) -> bool {
        !self.poisoned
            && (!self.events.is_empty()
                || !self.host_events.is_empty()
                || !self.imperative.is_empty()
                || !self.realizations.is_empty()
                || self.components.pending() != 0
                || !self.dirty_components.is_empty()
                || self.native_observation_pending)
    }

    /// Processes queued imperative element work after frontend publication.
    pub fn process_imperatives(&mut self) -> Result<usize, PumpError> {
        if self.poisoned {
            self.imperative.complete_unavailable();
            return Err(PumpError::Poisoned);
        }
        if !self.events.is_empty()
            || !self.host_events.is_empty()
            || self.components.pending() != 0
            || !self.dirty_components.is_empty()
            || self.native_observation_pending
        {
            return Ok(0);
        }
        let mut commands = Vec::new();
        for _ in 0..IMPERATIVE_WORK_BUDGET {
            let Some(queued) = self.imperative.pop_front() else {
                break;
            };
            if queued.identity != self.identity {
                queued.work.complete_unavailable();
                continue;
            }
            match queued.work {
                ImperativeRequest::Focus { node, completion } => {
                    if self.tree.try_native(node).is_none() {
                        _ = completion.call(Err(RuntimeError::MissingNode(node)));
                        continue;
                    }
                    commands.push(Command::Focus { node, completion });
                }
                ImperativeRequest::InitializeWebView2 { node, completion } => {
                    if self.tree.try_native(node).is_none() {
                        _ = completion.call(Err(RuntimeError::MissingNode(node)));
                        continue;
                    }
                    commands.push(Command::InitializeWebView2 { node, completion });
                }
                ImperativeRequest::ObserveSwapChainPanel {
                    node,
                    observation,
                    callback,
                } => {
                    if self.tree.try_native(node).is_none() {
                        continue;
                    }
                    commands.push(Command::ObserveSwapChainPanel {
                        node,
                        observation,
                        callback,
                    });
                }
                ImperativeRequest::SetSwapChain {
                    node,
                    swap_chain,
                    completion,
                } => {
                    if self.tree.try_native(node).is_none() {
                        _ = completion.call(Err(RuntimeError::MissingNode(node)));
                        continue;
                    }
                    commands.push(Command::SetSwapChain {
                        node,
                        swap_chain,
                        completion,
                    });
                }
                ImperativeRequest::SetNativeImageSource {
                    node,
                    source,
                    completion,
                } => {
                    if self.tree.try_native(node).is_none() {
                        _ = completion.call(Err(RuntimeError::MissingNode(node)));
                        continue;
                    }
                    commands.push(Command::SetNativeImageSource {
                        node,
                        source,
                        completion,
                    });
                }
                ImperativeRequest::ObserveImageScale {
                    node,
                    observation,
                    callback,
                } => {
                    if self.tree.try_native(node).is_none() {
                        continue;
                    }
                    commands.push(Command::ObserveImageScale {
                        node,
                        observation,
                        callback,
                    });
                }
                ImperativeRequest::ObserveCompositionHost {
                    node,
                    observation,
                    callback,
                } => {
                    if self.tree.try_native(node).is_none() {
                        continue;
                    }
                    commands.push(Command::ObserveCompositionHost {
                        node,
                        observation,
                        callback,
                    });
                }
                ImperativeRequest::RevokeObservation { node, observation } => {
                    commands.push(Command::RevokeObservation { node, observation });
                }
                ImperativeRequest::SetCompositionChildVisual {
                    node,
                    visual,
                    completion,
                } => {
                    if self.tree.try_native(node).is_none() {
                        _ = completion.call(Err(RuntimeError::MissingNode(node)));
                        continue;
                    }
                    commands.push(Command::SetCompositionChildVisual {
                        node,
                        visual,
                        completion,
                    });
                }
            }
        }
        if let Err(error) = self.apply_native_commands(&commands) {
            let PumpError::NativeApplyFailed(native) = error else {
                unreachable!();
            };
            for command in &commands[native.command..] {
                command.complete_unavailable();
            }
            self.imperative.complete_unavailable();
            return Err(PumpError::NativeApplyFailed(native));
        }
        Ok(commands.len())
    }

    #[cfg(any(test, feature = "test"))]
    pub fn event_revision(&self, node: NodeId, event: EventId) -> Option<u32> {
        if matches!(
            event,
            EventId::OwnedCommandInvoked | EventId::OwnedMenuItemInvoked
        ) {
            self.tree.try_kind(node)?;
            return Some(self.tree.owned_revision(node));
        }
        self.tree
            .try_native(node)?
            .events
            .get(&event)
            .filter(|state| state.active)
            .map(|state| state.revision)
    }

    #[cfg(any(test, feature = "test"))]
    pub fn queue_event(&mut self, event: QueuedEvent) {
        self.events.push_back(NativeWork {
            identity: self.identity,
            work: event,
        });
    }

    pub fn window_token(&self) -> WindowToken {
        self.identity
    }

    pub fn dispatch_events(&mut self) -> Result<usize, PumpError> {
        self.events.extend(self.runtime.drain_events());
        self.host_events.extend(self.runtime.drain_host_events());
        if self.poisoned {
            self.events.clear();
            self.host_events.clear();
            self.realizations.clear();
            _ = self.runtime.drain_event_errors();
            _ = self.runtime.drain_realizations();
            return Ok(0);
        }
        self.process_realizations()?;
        let mut dispatched = 0;
        for _ in 0..EVENT_WORK_BUDGET {
            let Some(queued) = self.host_events.pop_front() else {
                break;
            };
            if queued.identity != self.identity {
                continue;
            }
            let callback_accepted = match &queued.work {
                HostEvent::WindowSize { observation, size } => {
                    let current = self
                        .tree
                        .validate_window_size_observation()
                        .map_err(|()| PumpError::DuplicateWindowSizeObservation)?;
                    let Some((_, callback)) = current.filter(|(current, _)| current == observation)
                    else {
                        continue;
                    };
                    callback.call(*size)
                }
                HostEvent::ColorScheme {
                    observation,
                    scheme,
                } => {
                    let current = self
                        .tree
                        .validate_color_scheme_observation()
                        .map_err(|()| PumpError::DuplicateColorSchemeObservation)?;
                    let Some((_, callback)) = current.filter(|(current, _)| current == observation)
                    else {
                        continue;
                    };
                    callback.call(*scheme)
                }
                HostEvent::ObservationError { observation, error } => {
                    let current_size = self
                        .tree
                        .validate_window_size_observation()
                        .map_err(|()| PumpError::DuplicateWindowSizeObservation)?
                        .is_some_and(|(current, _)| current == *observation);
                    let current_scheme = self
                        .tree
                        .validate_color_scheme_observation()
                        .map_err(|()| PumpError::DuplicateColorSchemeObservation)?
                        .is_some_and(|(current, _)| current == *observation);
                    if !current_size && !current_scheme {
                        continue;
                    }
                    self.events.clear();
                    self.host_events.clear();
                    return Err(PumpError::EventReadFailed(*error));
                }
                HostEvent::Error(error) => {
                    self.events.clear();
                    self.host_events.clear();
                    return Err(PumpError::EventReadFailed(*error));
                }
            };
            if callback_accepted {
                dispatched += 1;
            } else {
                self.host_events.push_front(queued);
                break;
            }
        }
        for queued in self.runtime.drain_event_errors() {
            if queued.identity != self.identity {
                continue;
            }
            let error = queued.work;
            let Some(native) = self.tree.try_native(error.node) else {
                continue;
            };
            let Some(state) = native.events.get(&error.event) else {
                continue;
            };
            if !state.active || state.revision != error.revision {
                continue;
            }
            self.events.clear();
            self.host_events.clear();
            return Err(PumpError::EventReadFailed(error.error));
        }
        for _ in 0..EVENT_WORK_BUDGET {
            let Some(queued) = self.events.pop_front() else {
                break;
            };
            let identity = queued.identity;
            if identity != self.identity {
                continue;
            }
            let event = queued.work;
            if matches!(
                event.event,
                EventId::OwnedCommandInvoked | EventId::OwnedMenuItemInvoked
            ) {
                let Some(kind) = self.tree.try_kind(event.node) else {
                    continue;
                };
                let expected_kind = match (event.event, kind) {
                    (EventId::OwnedCommandInvoked, NodeKind::CommandBarFlyout) => {
                        NodeKind::CommandBarFlyout
                    }
                    (EventId::OwnedMenuItemInvoked, NodeKind::Menu(kind)) => NodeKind::Menu(kind),
                    _ => continue,
                };
                if kind != expected_kind || self.tree.owned_revision(event.node) != event.revision {
                    continue;
                }
                let EventPayload::String(label) = &event.payload else {
                    continue;
                };
                if self.tree.owned_callback(event.node).call(label.clone()) {
                    dispatched += 1;
                } else {
                    self.events.push_front(NativeWork {
                        identity,
                        work: event,
                    });
                    break;
                }
                continue;
            }
            let observation = {
                let Some(native) = self.tree.try_native(event.node) else {
                    continue;
                };
                let Some(state) = native.events.get(&event.event) else {
                    continue;
                };
                if !state.active || state.revision != event.revision {
                    continue;
                }
                native.desired.observe_event(event.event, &event.payload)
            };
            let selection_observation = match &event.payload {
                EventPayload::SelectionChange(selected) => match selection_for_event(event.event) {
                    Some(selection) => self.observe_selection(event.node, selection, selected.item),
                    None => false,
                },
                _ => false,
            };
            let property_observation = observation.is_some();
            if let Some((property, value)) = observation {
                self.tree
                    .native_mut(event.node)
                    .properties
                    .insert(property, Some(value));
            }
            if (selection_observation || property_observation) && event.invokes_callback() {
                self.native_observation_pending = true;
                if self.trace_component_plans {
                    self.last_native_observation = Some((event.node, event.event));
                }
                let mut current = event.node;
                while let Some(parent) = self.tree.parent(current) {
                    current = parent;
                    if self.tree.kind(current) == NodeKind::Component {
                        let token = self.components.token(self.tree.component_scope(current));
                        self.dirty_components.insert(token);
                        break;
                    }
                }
            }
            if event.invokes_callback() {
                match self
                    .tree
                    .native(event.node)
                    .desired
                    .dispatch_event(event.event, &event.payload)
                {
                    Some(true) => dispatched += 1,
                    Some(false) => {
                        self.events.push_front(NativeWork {
                            identity,
                            work: event,
                        });
                        break;
                    }
                    None => {}
                }
            }
        }
        Ok(dispatched)
    }

    fn observe_selection(
        &mut self,
        owner: NodeId,
        selection: SelectionDescriptor,
        selected_item: Option<NodeId>,
    ) -> bool {
        let Some(slot) = self
            .tree
            .children(owner)
            .iter()
            .copied()
            .find(|child| self.tree.kind(*child) == NodeKind::NamedSlot(selection.slot))
        else {
            return false;
        };
        let mut items = Vec::new();
        for child in self.tree.children(slot).to_vec() {
            let mut roots = Vec::new();
            Self::collect_native_roots(&self.tree, child, &mut roots);
            if let [item] = roots.as_slice()
                && self.tree.kind(*item) == NodeKind::Native(selection.item)
            {
                items.push(*item);
            }
        }
        let mut changed = false;
        for item in items {
            let mut declared = false;
            self.tree
                .native(item)
                .desired
                .visit_properties(&mut |property, value| {
                    if property == selection.selected_property {
                        declared = value.is_some();
                    }
                });
            if !declared {
                continue;
            }
            let selected = selected_item == Some(item);
            let value = Some(PropertyValue::Bool(selected));
            let item_changed = self
                .tree
                .native_mut(item)
                .properties
                .insert(selection.selected_property, value.clone())
                != Some(value);
            changed |= item_changed;
            if item_changed {
                let mut current = item;
                while let Some(parent) = self.tree.parent(current) {
                    current = parent;
                    if self.tree.kind(current) == NodeKind::Component {
                        let token = self.components.token(self.tree.component_scope(current));
                        self.dirty_components.insert(token);
                        break;
                    }
                }
            }
        }
        changed
    }

    fn collect_native_roots(tree: &Tree, node: NodeId, roots: &mut Vec<NodeId>) {
        match tree.kind(node) {
            NodeKind::Native(_) | NodeKind::VirtualCollection => roots.push(node),
            _ => {
                for child in tree.children(node) {
                    Self::collect_native_roots(tree, *child, roots);
                }
            }
        }
    }

    pub fn process_realizations(&mut self) -> Result<Vec<RealizationOutcome>, PumpError> {
        if self.poisoned {
            self.realizations.clear();
            _ = self.runtime.drain_realizations();
            return Err(PumpError::Poisoned);
        }
        self.realizations.extend(self.runtime.drain_realizations());
        let mut outcomes = Vec::with_capacity(self.realizations.len().min(REALIZATION_WORK_BUDGET));
        let mut candidate = self.tree.clone();
        let mut plan = UpdatePlan::new(self.identity);
        let mut changes = ComponentChanges::default();
        let mut consumed = Vec::new();
        let mut pending_recycles = self
            .realizations
            .iter()
            .filter(|queued| queued.identity == self.identity)
            .filter_map(|queued| match queued.work {
                RealizationRequest::Recycle {
                    collection,
                    container,
                    ..
                } => Some((collection, container)),
                _ => None,
            })
            .fold(HashMap::new(), |mut counts, key| {
                *counts.entry(key).or_insert(0usize) += 1;
                counts
            });
        let planning = (|| {
            let mut processed = 0;
            while processed < REALIZATION_WORK_BUDGET {
                let Some(queued) = self.realizations.pop_front() else {
                    break;
                };
                let request = queued.work;
                let current_identity = queued.identity == self.identity;
                consumed.push(queued);
                if current_identity
                    && let RealizationRequest::Recycle {
                        collection,
                        container,
                        ..
                    } = request
                    && let Some(count) = pending_recycles.get_mut(&(collection, container))
                {
                    *count -= 1;
                }
                if current_identity
                    && let RealizationRequest::Realize {
                        collection,
                        container,
                        ..
                    } = request
                    && pending_recycles
                        .get(&(collection, container))
                        .is_some_and(|count| *count != 0)
                {
                    outcomes.push(RealizationOutcome::Rejected(request));
                    continue;
                }
                processed += 1;
                if !current_identity {
                    outcomes.push(RealizationOutcome::Rejected(request));
                    continue;
                }
                let outcome = match request {
                    RealizationRequest::Realize {
                        collection,
                        container,
                        index,
                        source_revision,
                    } => {
                        let Some(model) = candidate.try_virtual_model(collection) else {
                            outcomes.push(RealizationOutcome::Rejected(request));
                            continue;
                        };
                        if model.source_revision() != source_revision {
                            outcomes.push(RealizationOutcome::Rejected(request));
                            continue;
                        }
                        let Some(model) = candidate.try_virtual_model_mut(collection) else {
                            outcomes.push(RealizationOutcome::Rejected(request));
                            continue;
                        };
                        let Some(lease) = model.realize(index, container) else {
                            outcomes.push(RealizationOutcome::Rejected(request));
                            continue;
                        };
                        let view = candidate.virtual_view_at(collection, index);
                        let stale = candidate
                            .children(collection)
                            .iter()
                            .copied()
                            .filter(|logical_root| {
                                candidate.key(*logical_root) == Some(&lease.key)
                                    || candidate
                                        .realized(collection, container)
                                        .is_some_and(|row| row.logical_root == *logical_root)
                            })
                            .collect::<Vec<_>>();
                        for old in stale {
                            Self::collect_retired_components(
                                &mut candidate,
                                old,
                                &self.components,
                                &mut changes,
                            );
                            Self::retire_planned_subtree(&mut candidate, old, &mut plan)?;
                        }
                        let (logical_root, _) = Self::mount_planned_view(
                            &mut candidate,
                            Some(collection),
                            Some(lease.key.clone()),
                            view,
                            &mut self.components,
                            &mut changes,
                            &mut plan,
                        )?;
                        candidate.set_realized(collection, container, index, logical_root, None);
                        Self::refresh_virtual_row_attachment(
                            &mut candidate,
                            collection,
                            container,
                            &mut plan,
                        )?;
                        RealizationOutcome::Realized(lease)
                    }
                    RealizationRequest::Recycle {
                        collection,
                        container,
                        source_revision,
                    } => {
                        let Some(model) = candidate.try_virtual_model(collection) else {
                            outcomes.push(RealizationOutcome::Rejected(request));
                            continue;
                        };
                        if model.source_revision() != source_revision {
                            plan.push(Command::AcknowledgeRecycle {
                                collection,
                                container,
                            });
                            outcomes.push(RealizationOutcome::Rejected(request));
                            continue;
                        }
                        let Some(row) = candidate.realized(collection, container) else {
                            plan.push(Command::AcknowledgeRecycle {
                                collection,
                                container,
                            });
                            outcomes.push(RealizationOutcome::Rejected(request));
                            continue;
                        };
                        let Some(lease) = candidate
                            .try_virtual_model_mut(collection)
                            .and_then(|model| model.recycle_container(container))
                        else {
                            plan.push(Command::AcknowledgeRecycle {
                                collection,
                                container,
                            });
                            outcomes.push(RealizationOutcome::Rejected(request));
                            continue;
                        };
                        Self::collect_retired_components(
                            &mut candidate,
                            row.logical_root,
                            &self.components,
                            &mut changes,
                        );
                        Self::retire_planned_subtree(&mut candidate, row.logical_root, &mut plan)?;
                        RealizationOutcome::Recycled(lease)
                    }
                };
                outcomes.push(outcome);
            }
            Ok::<(), PumpError>(())
        })();
        if let Err(error) = planning {
            for queued in consumed.into_iter().rev() {
                self.realizations.push_front(queued);
            }
            self.fail_component_candidate(&changes, PlanningFailure::Discard);
            return Err(error);
        }
        let has_work = !plan.commands.is_empty()
            || !plan.diagnostics.is_empty()
            || !changes.reserved.is_empty()
            || !changes.retired.is_empty()
            || !changes.composed.is_empty()
            || outcomes.iter().any(|outcome| {
                matches!(
                    outcome,
                    RealizationOutcome::Realized(_) | RealizationOutcome::Recycled(_)
                )
            });
        if has_work {
            match self.apply_realization(candidate, plan, changes) {
                Ok(()) => {}
                Err(error) => {
                    if !self.poisoned {
                        for queued in consumed.into_iter().rev() {
                            self.realizations.push_front(queued);
                        }
                    }
                    return Err(error);
                }
            }
        }
        Ok(outcomes)
    }

    fn apply_realization(
        &mut self,
        candidate: Tree,
        plan: UpdatePlan,
        changes: ComponentChanges,
    ) -> Result<(), PumpError> {
        let root = self.root.ok_or(PumpError::NotMounted)?;
        let window = self.window.ok_or(PumpError::NotMounted)?;
        self.finalize_component_candidate(ComponentCandidate {
            activate_window: false,
            changes,
            next_version: self.version,
            plan,
            planning_failure: PlanningFailure::Discard,
            root,
            tree: candidate,
            window,
        })
    }
}
