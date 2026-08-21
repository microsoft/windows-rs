use super::*;

pub(super) const EVENT_WORK_BUDGET: usize = 64;
pub(super) const REALIZATION_WORK_BUDGET: usize = 32;

impl<R: NativeRuntime> Pump<R> {
    pub fn native_work_pending(&self) -> bool {
        !self.events.is_empty()
            || !self.imperative.is_empty()
            || !self.realizations.is_empty()
            || self.components.pending() != 0
            || !self.dirty_components.is_empty()
            || self.native_observation_pending
    }

    /// Processes queued imperative element work after frontend publication.
    pub fn process_imperatives(&mut self) -> Result<usize, PumpError> {
        if self.poisoned {
            self.imperative.clear();
            return Err(PumpError::Poisoned);
        }
        if !self.events.is_empty()
            || self.components.pending() != 0
            || !self.dirty_components.is_empty()
            || self.native_observation_pending
        {
            return Ok(0);
        }
        let mut processed = 0;
        while let Some(queued) = self.imperative.pop_front() {
            if queued.identity != self.identity {
                continue;
            }
            match queued.work {
                ImperativeRequest::Focus { node } => {
                    if self.tree.native(node).is_err() {
                        continue;
                    }
                    self.apply_native_commands(&[Command::Focus { node }])?;
                    processed += 1;
                }
            }
        }
        Ok(processed)
    }

    #[cfg(any(test, feature = "test"))]
    pub fn event_revision(&self, node: NodeId, event: EventId) -> Option<u32> {
        self.tree
            .native(node)
            .ok()?
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
        if self.poisoned {
            self.events.clear();
            self.realizations.clear();
            _ = self.runtime.drain_event_errors();
            _ = self.runtime.drain_realizations();
            return Ok(0);
        }
        self.process_realizations()?;
        for queued in self.runtime.drain_event_errors() {
            if queued.identity != self.identity {
                continue;
            }
            let error = queued.work;
            let Ok(native) = self.tree.native(error.node) else {
                continue;
            };
            let Some(state) = native.events.get(&error.event) else {
                continue;
            };
            if !state.active || state.revision != error.revision {
                continue;
            }
            self.events.clear();
            return Err(PumpError::EventReadFailed(error.error));
        }
        let mut dispatched = 0;
        for _ in 0..EVENT_WORK_BUDGET {
            let Some(queued) = self.events.pop_front() else {
                break;
            };
            if queued.identity != self.identity {
                continue;
            }
            let event = queued.work;
            let Ok(native) = self.tree.native(event.node) else {
                continue;
            };
            let Some(state) = native.events.get(&event.event) else {
                continue;
            };
            if !state.active || state.revision != event.revision {
                continue;
            }
            let observation = native.desired.observe_event(event.event, &event.payload);
            if let Some((property, value)) = observation {
                self.tree
                    .native_mut(event.node)?
                    .properties
                    .insert(property, Some(value));
                if event.invokes_callback() {
                    self.native_observation_pending = true;
                    let mut current = event.node;
                    while let Some(parent) = self.tree.parent(current)? {
                        current = parent;
                        if self.tree.kind(current)? == NodeKind::Component {
                            let token =
                                self.components.token(self.tree.component_scope(current)?)?;
                            self.dirty_components.insert(token);
                            break;
                        }
                    }
                }
            }
            if event.invokes_callback() {
                match self
                    .tree
                    .native(event.node)?
                    .desired
                    .dispatch_event(event.event, &event.payload)
                {
                    Some(true) => dispatched += 1,
                    Some(false) => {
                        self.events.clear();
                        return Err(PumpError::EventCallbackRejected {
                            node: event.node,
                            event: event.event,
                        });
                    }
                    None => {}
                }
            }
        }
        Ok(dispatched)
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
        let planning = (|| {
            for _ in 0..REALIZATION_WORK_BUDGET {
                let Some(queued) = self.realizations.pop_front() else {
                    break;
                };
                let request = queued.work;
                let current_identity = queued.identity == self.identity;
                consumed.push(queued);
                if !current_identity {
                    outcomes.push(RealizationOutcome::Rejected(request));
                    continue;
                }
                let outcome = match request {
                    RealizationRequest::Realize {
                        collection,
                        container,
                        index,
                    } => {
                        let Ok(lease) = candidate.virtual_model_mut(collection).and_then(|model| {
                            model.realize(index, container).map_err(TreeError::from)
                        }) else {
                            outcomes.push(RealizationOutcome::Rejected(request));
                            continue;
                        };
                        let view = candidate.virtual_item(collection, &lease.key)?.clone();
                        let stale = candidate
                            .children(collection)?
                            .iter()
                            .copied()
                            .filter(|logical_root| {
                                candidate.key(*logical_root).ok().flatten() == Some(&lease.key)
                                    || candidate
                                        .realized(collection, container)
                                        .ok()
                                        .flatten()
                                        .is_some_and(|row| row.logical_root == *logical_root)
                            })
                            .collect::<Vec<_>>();
                        for old in stale {
                            Self::collect_retired_components(
                                &candidate,
                                old,
                                &self.components,
                                &mut changes,
                            )?;
                            Self::retire_planned_subtree(&mut candidate, old, &mut plan)?;
                        }
                        let (logical_root, native) = Self::mount_planned_view(
                            &mut candidate,
                            Some(collection),
                            Some(lease.key.clone()),
                            view,
                            &mut self.components,
                            &mut changes,
                            &mut plan,
                        )?;
                        let [native_root] = native.as_slice() else {
                            return Err(PumpError::StructureUnsupported);
                        };
                        candidate.set_realized(
                            collection,
                            container,
                            logical_root,
                            *native_root,
                        )?;
                        plan.push(Command::AttachRealized {
                            collection,
                            container,
                            child: *native_root,
                        });
                        RealizationOutcome::Realized(lease)
                    }
                    RealizationRequest::Recycle {
                        collection,
                        container,
                    } => {
                        let Some(row) = candidate.realized(collection, container)? else {
                            outcomes.push(RealizationOutcome::Rejected(request));
                            continue;
                        };
                        let Some(lease) = candidate
                            .virtual_model_mut(collection)
                            .ok()
                            .and_then(|model| model.recycle_container(container))
                        else {
                            outcomes.push(RealizationOutcome::Rejected(request));
                            continue;
                        };
                        Self::collect_retired_components(
                            &candidate,
                            row.logical_root,
                            &self.components,
                            &mut changes,
                        )?;
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
            Self::remove_reservations(&mut self.components, &changes.reserved);
            return Err(error);
        }
        if !plan.commands.is_empty()
            || !changes.reserved.is_empty()
            || !changes.retired.is_empty()
            || !changes.composed.is_empty()
        {
            self.apply_realization(candidate, plan, changes)?;
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
        self.publish_candidate(
            CandidateState::Tree {
                tree: candidate,
                root,
            },
            plan,
            FrontendChanges::Component(changes),
            self.version,
        )
    }
}
