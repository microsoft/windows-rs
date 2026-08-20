use super::*;

impl<R: NativeRuntime> Pump<R> {
    pub(super) fn publish_candidate(
        &mut self,
        mut candidate: CandidateState,
        plan: UpdatePlan,
        changes: FrontendChanges,
        next_version: u64,
    ) -> Result<CommitReceipt, PumpError> {
        match &changes {
            FrontendChanges::Component(changes) => self.prepare_component_effects(changes)?,
            FrontendChanges::Local(token) => self.components.prepare_effects(*token)?,
            FrontendChanges::Element(_) => {}
        }
        if plan.commands.is_empty() {
            self.publish_frontend(candidate, &changes)?;
            self.retry_pending = false;
            self.version = next_version;
            return Ok(CommitReceipt {
                outcomes: Vec::new(),
            });
        }

        let receipt = self.runtime.apply(&plan.commands);
        if receipt.outcomes.len() != plan.commands.len() {
            self.abandon_frontend(&changes);
            self.events.clear();
            self.realizations.clear();
            self.poisoned = true;
            self.retry_pending = false;
            return Err(PumpError::ApplyReceiptMismatch);
        }
        if plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, command)| command.structural() && !receipt.applied(index))
        {
            return self.recover_candidate_structure(candidate, receipt, changes, next_version);
        }
        let retries_exhausted =
            match self.commit_candidate_properties(&mut candidate, &plan.commits, &receipt) {
                Ok(retries_exhausted) => retries_exhausted,
                Err(error) => {
                    self.runtime.reset();
                    self.abandon_frontend(&changes);
                    self.poisoned = true;
                    self.retry_pending = false;
                    return Err(error);
                }
            };
        self.publish_frontend(candidate, &changes)?;
        if plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, _)| !receipt.applied(index))
        {
            self.retry_pending = true;
            return Err(if retries_exhausted {
                PumpError::PropertyRetriesExhausted(receipt)
            } else {
                PumpError::PropertyApplyFailed(receipt)
            });
        }
        self.retry_pending = false;
        self.version = next_version;
        Ok(receipt)
    }

    fn publish_frontend(
        &mut self,
        candidate: CandidateState,
        changes: &FrontendChanges,
    ) -> Result<(), PumpError> {
        if let FrontendChanges::Component(changes) = changes {
            self.finalize_component_changes(changes)?;
        }
        match candidate {
            CandidateState::Tree { tree, root } => {
                self.tree = tree;
                self.root = Some(root);
            }
            CandidateState::Native { node, desired } => {
                self.tree.native_mut(node)?.desired = desired;
            }
        }
        match changes {
            FrontendChanges::Element(element) => self.element = Some(element.clone()),
            FrontendChanges::Component(changes) => self.commit_component_effects(changes)?,
            FrontendChanges::Local(token) => self.components.commit_effects(*token)?,
        }
        Ok(())
    }

    fn abandon_frontend(&mut self, changes: &FrontendChanges) {
        match changes {
            FrontendChanges::Component(changes) => {
                Self::remove_reservations(&mut self.components, &changes.reserved);
            }
            FrontendChanges::Element(_) | FrontendChanges::Local(_) => {}
        }
    }

    fn recover_candidate_structure(
        &mut self,
        candidate: CandidateState,
        failure: CommitReceipt,
        changes: FrontendChanges,
        next_version: u64,
    ) -> Result<CommitReceipt, PumpError> {
        let CandidateState::Tree {
            mut tree,
            root: candidate_root,
        } = candidate
        else {
            self.poisoned = true;
            return Err(PumpError::StructuralApplyFailed(failure));
        };
        let window = self.window.ok_or(PumpError::NotMounted)?;
        let recovery_identity = self
            .identity
            .next_realization()
            .ok_or(PumpError::RevisionExhausted)?;
        let mut plan = UpdatePlan::new(recovery_identity);
        plan.push(Command::ResetWindowContent { window });
        let recovery_root = match &changes {
            FrontendChanges::Element(element) => {
                tree.retire_subtree(candidate_root)?;
                Self::mount_planned_element(
                    &mut tree,
                    Some(window),
                    None,
                    element.clone(),
                    &mut plan,
                )?
            }
            FrontendChanges::Component(_) => candidate_root,
            FrontendChanges::Local(_) => unreachable!(),
        };
        let native_roots = match &changes {
            FrontendChanges::Element(_) => vec![recovery_root],
            FrontendChanges::Component(_) => {
                match Self::plan_existing_subtree(&tree, recovery_root, &mut plan) {
                    Ok(native_roots) => native_roots,
                    Err(error) => {
                        self.runtime.reset();
                        self.abandon_frontend(&changes);
                        self.poisoned = true;
                        return Err(error);
                    }
                }
            }
            FrontendChanges::Local(_) => unreachable!(),
        };
        match native_roots.as_slice() {
            [] => {}
            [native_root] => {
                plan.push(Command::InsertChild {
                    parent: window,
                    child: *native_root,
                    index: 0,
                });
            }
            _ => {
                self.abandon_frontend(&changes);
                return Err(PumpError::StructureUnsupported);
            }
        }

        self.events.clear();
        self.realizations.clear();
        self.identity = recovery_identity;
        self.runtime.set_identity(recovery_identity);
        self.refresh_component_waker();

        // If the recovery plan fits within one budget, apply it immediately and
        // preserve existing single-turn RecoveredStructure behavior.
        if plan.commands.len() <= RECOVERY_COMMAND_BUDGET {
            return self.apply_recovery_immediate(
                tree,
                recovery_root,
                plan,
                failure,
                changes,
                next_version,
            );
        }

        // Large recovery: apply the first chunk and store a continuation.
        let total = plan.commands.len();
        let chunk_end = RECOVERY_COMMAND_BUDGET.min(total);
        let chunk = &plan.commands[..chunk_end];
        let recovery = self.runtime.apply(chunk);
        if recovery.outcomes.len() != chunk_end {
            self.abandon_frontend(&changes);
            self.poisoned = true;
            self.retry_pending = false;
            return Err(PumpError::RecoveryFailed(Box::new(StructuralRecovery {
                failure,
                recovery,
                root: recovery_root,
            })));
        }
        if plan.commands[..chunk_end]
            .iter()
            .enumerate()
            .any(|(index, command)| command.structural() && !recovery.applied(index))
        {
            self.abandon_frontend(&changes);
            self.poisoned = true;
            self.retry_pending = false;
            return Err(PumpError::RecoveryFailed(Box::new(StructuralRecovery {
                failure,
                recovery,
                root: recovery_root,
            })));
        }

        self.pending_recovery = Some(PendingRecovery {
            candidate: CandidateState::Tree {
                tree,
                root: recovery_root,
            },
            changes,
            plan,
            failure,
            next_version,
            commands_applied: chunk_end,
            outcomes: recovery.outcomes,
            recovery_root,
        });
        Err(PumpError::RecoveryPending)
    }

    /// Apply a recovery plan that fits within one budget chunk. This preserves
    /// the existing single-turn `RecoveredStructure` behavior.
    fn apply_recovery_immediate(
        &mut self,
        tree: Tree,
        recovery_root: NodeId,
        plan: UpdatePlan,
        failure: CommitReceipt,
        changes: FrontendChanges,
        next_version: u64,
    ) -> Result<CommitReceipt, PumpError> {
        let recovery = self.runtime.apply(&plan.commands);
        let attempt = |recovery| {
            Box::new(StructuralRecovery {
                failure: failure.clone(),
                recovery,
                root: recovery_root,
            })
        };
        if recovery.outcomes.len() != plan.commands.len()
            || plan
                .commands
                .iter()
                .enumerate()
                .any(|(index, command)| command.structural() && !recovery.applied(index))
        {
            self.abandon_frontend(&changes);
            self.poisoned = true;
            self.retry_pending = false;
            return Err(PumpError::RecoveryFailed(attempt(recovery)));
        }

        let mut candidate = CandidateState::Tree {
            tree,
            root: recovery_root,
        };
        self.commit_candidate_properties(&mut candidate, &plan.commits, &recovery)?;
        self.publish_frontend(candidate, &changes)?;
        self.retry_pending = plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, command)| !command.structural() && !recovery.applied(index));
        if !self.retry_pending {
            self.version = next_version;
        }
        Err(PumpError::RecoveredStructure(attempt(recovery)))
    }

    /// Resume a pending multi-turn recovery. Called at the start of each
    /// dispatcher turn before ordinary events, messages, or reconciliation.
    pub(super) fn resume_recovery(&mut self) -> Result<(), PumpError> {
        let Some(mut recovery) = self.pending_recovery.take() else {
            return Ok(());
        };

        let remaining = recovery.remaining_commands();
        if remaining.is_empty() {
            return self.finalize_recovery(recovery);
        }

        let chunk_end = RECOVERY_COMMAND_BUDGET.min(remaining.len());
        let chunk = &remaining[..chunk_end];
        let receipt = self.runtime.apply(chunk);
        if receipt.outcomes.len() != chunk_end {
            self.abandon_frontend(&recovery.changes);
            self.poisoned = true;
            self.retry_pending = false;
            return Err(PumpError::RecoveryFailed(Box::new(StructuralRecovery {
                failure: recovery.failure,
                recovery: CommitReceipt {
                    outcomes: {
                        let mut all = recovery.outcomes;
                        all.extend(receipt.outcomes);
                        all
                    },
                },
                root: recovery.recovery_root,
            })));
        }

        // Check for structural failure in this chunk. Command indices are
        // relative to the chunk, not the full plan.
        let global_offset = recovery.commands_applied;
        if recovery.plan.commands[global_offset..global_offset + chunk_end]
            .iter()
            .enumerate()
            .any(|(index, command)| command.structural() && !receipt.applied(index))
        {
            self.abandon_frontend(&recovery.changes);
            self.poisoned = true;
            self.retry_pending = false;
            return Err(PumpError::RecoveryFailed(Box::new(StructuralRecovery {
                failure: recovery.failure,
                recovery: CommitReceipt {
                    outcomes: {
                        let mut all = recovery.outcomes;
                        all.extend(receipt.outcomes);
                        all
                    },
                },
                root: recovery.recovery_root,
            })));
        }

        recovery.outcomes.extend(receipt.outcomes);
        recovery.commands_applied += chunk_end;

        if recovery.is_complete() {
            return self.finalize_recovery(recovery);
        }

        // More chunks remain - put the continuation back.
        self.pending_recovery = Some(recovery);
        Err(PumpError::RecoveryPending)
    }

    /// Complete a multi-turn recovery by committing properties, publishing
    /// the frontend, and reporting the aggregate result.
    fn finalize_recovery(&mut self, recovery: PendingRecovery) -> Result<(), PumpError> {
        let aggregate = CommitReceipt {
            outcomes: recovery.outcomes,
        };
        let attempt = Box::new(StructuralRecovery {
            failure: recovery.failure,
            recovery: aggregate.clone(),
            root: recovery.recovery_root,
        });

        // Translate property commits: their command indices are absolute
        // within the full plan and match the aggregate outcome indices.
        let mut candidate = recovery.candidate;
        if self
            .commit_candidate_properties(&mut candidate, &recovery.plan.commits, &aggregate)
            .is_err()
        {
            self.abandon_frontend(&recovery.changes);
            self.poisoned = true;
            self.retry_pending = false;
            return Err(PumpError::RecoveryFailed(attempt));
        }
        if self.publish_frontend(candidate, &recovery.changes).is_err() {
            self.poisoned = true;
            self.retry_pending = false;
            return Err(PumpError::RecoveryFailed(attempt));
        }
        self.retry_pending = recovery
            .plan
            .commands
            .iter()
            .enumerate()
            .any(|(index, command)| !command.structural() && !aggregate.applied(index));
        if !self.retry_pending {
            self.version = recovery.next_version;
        }
        // Report recovery as a recoverable error so the host can schedule a
        // retry if property failures remain.
        Err(PumpError::RecoveredStructure(attempt))
    }
}
