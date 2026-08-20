use super::*;

pub(super) const EVENT_WORK_BUDGET: usize = 64;
pub(super) const REALIZATION_WORK_BUDGET: usize = 32;

impl<R: NativeRuntime> Pump<R> {
    pub fn native_work_pending(&self) -> bool {
        self.pending_recovery.is_some()
            || !self.events.is_empty()
            || !self.realizations.is_empty()
            || self.components.pending() != 0
            || !self.dirty_components.is_empty()
            || self.retry_pending
    }

    pub fn event_revision(&self, node: NodeId, event: EventId) -> Option<u32> {
        self.tree
            .native(node)
            .ok()?
            .events
            .get(&event)
            .filter(|state| state.active)
            .map(|state| state.revision)
    }

    pub fn queue_event(&mut self, event: QueuedEvent) {
        self.events.push_back(NativeWork {
            identity: self.identity,
            work: event,
        });
    }

    pub fn native_identity(&self) -> NativeIdentity {
        self.identity
    }

    pub(super) fn queue_event_with_identity(
        &mut self,
        identity: NativeIdentity,
        event: QueuedEvent,
    ) {
        self.events.push_back(NativeWork {
            identity,
            work: event,
        });
    }

    pub fn dispatch_events(&mut self) -> Result<usize, PumpError> {
        self.resume_recovery()?;
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
                    .insert(property, NativePropertyState::Known(Some(value)));
                self.retry_pending = true;
            }
            if self
                .tree
                .native(event.node)?
                .desired
                .dispatch_event(event.event, &event.payload)
            {
                dispatched += 1;
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
        for _ in 0..REALIZATION_WORK_BUDGET {
            let Some(queued) = self.realizations.pop_front() else {
                break;
            };
            let request = queued.work;
            if queued.identity != self.identity {
                outcomes.push(RealizationOutcome::Rejected(request));
                continue;
            }
            let outcome = match request {
                RealizationRequest::Realize {
                    collection,
                    container,
                    index,
                } => {
                    let Ok(lease) = candidate
                        .virtual_model_mut(collection)
                        .and_then(|model| model.realize(index, container).map_err(TreeError::from))
                    else {
                        outcomes.push(RealizationOutcome::Rejected(request));
                        continue;
                    };
                    let element = candidate.virtual_item(collection, &lease.key)?.clone();
                    let stale = candidate
                        .children(collection)?
                        .iter()
                        .copied()
                        .filter(|child| {
                            candidate.key(*child).ok().flatten() == Some(&lease.key)
                                || candidate.realized(collection, container).ok().flatten()
                                    == Some(*child)
                        })
                        .collect::<Vec<_>>();
                    for old in stale {
                        Self::retire_planned_subtree(&mut candidate, old, &mut plan)?;
                    }
                    let child = Self::mount_planned_element(
                        &mut candidate,
                        Some(collection),
                        Some(lease.key.clone()),
                        element,
                        &mut plan,
                    )?;
                    candidate.set_realized(collection, container, child)?;
                    plan.push(Command::AttachRealized {
                        collection,
                        container,
                        child,
                    });
                    RealizationOutcome::Realized(lease)
                }
                RealizationRequest::Recycle {
                    collection,
                    container,
                } => {
                    let Some(child) = candidate.realized(collection, container)? else {
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
                    Self::retire_planned_subtree(&mut candidate, child, &mut plan)?;
                    RealizationOutcome::Recycled(lease)
                }
            };
            outcomes.push(outcome);
        }
        if !plan.commands.is_empty() {
            self.apply_realization(candidate, &plan)?;
        }
        Ok(outcomes)
    }

    fn apply_realization(
        &mut self,
        mut candidate: Tree,
        plan: &UpdatePlan,
    ) -> Result<(), PumpError> {
        let receipt = self.runtime.apply(&plan.commands);
        if receipt.outcomes.len() != plan.commands.len() {
            self.poisoned = true;
            return Err(PumpError::ApplyReceiptMismatch);
        }
        if plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, _)| !receipt.applied(index))
        {
            self.poisoned = true;
            return Err(PumpError::StructuralApplyFailed(receipt));
        }
        Self::commit_tree_properties(&mut candidate, &plan.commits, &receipt)?;
        self.tree = candidate;
        Ok(())
    }
}
