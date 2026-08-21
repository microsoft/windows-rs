use super::*;

impl<R: NativeRuntime> Pump<R> {
    pub(super) fn apply_native_commands(&mut self, commands: &[Command]) -> Result<(), PumpError> {
        if commands.is_empty() {
            return Ok(());
        }
        if let Err(error) = self.runtime.apply(commands) {
            self.poisoned = true;
            self.events.clear();
            self.realizations.clear();
            return Err(PumpError::NativeApplyFailed(error));
        }
        Ok(())
    }

    pub(super) fn publish_candidate(
        &mut self,
        mut candidate: CandidateState,
        plan: UpdatePlan,
        mut changes: FrontendChanges,
        next_version: u64,
    ) -> Result<(), PumpError> {
        let commits_window_close = plan
            .post_publish_commands
            .iter()
            .any(|command| matches!(command, Command::CloseWindow { .. }));
        if let Err(error) = self.validate_candidate_references(&candidate, &plan.reference_commits)
        {
            if let FrontendChanges::Component(changes) = &changes {
                self.planning_dirty.extend(changes.touched.iter().copied());
                Self::remove_reservations(&mut self.components, &changes.reserved);
            }
            return Err(error);
        }
        let prepared = match &mut changes {
            FrontendChanges::Component(changes) => self.prepare_component_effects(changes),
            FrontendChanges::Local { token, .. } => {
                self.components.prepare_effects(*token).map_err(Into::into)
            }
            #[cfg(any(test, feature = "test"))]
            FrontendChanges::Element(_) => Ok(()),
        };
        if let Err(error) = prepared {
            self.poisoned = true;
            if let FrontendChanges::Component(changes) = &changes {
                Self::remove_reservations(&mut self.components, &changes.reserved);
            }
            return Err(error);
        }

        self.apply_native_commands(&plan.commands)?;

        self.commit_candidate_properties(&mut candidate, &plan.commits)?;
        self.commit_candidate_references(&mut candidate, &plan.reference_commits)?;
        self.publish_frontend(candidate, changes, &plan.reference_commits)?;
        self.diagnostics.extend(plan.diagnostics);
        self.native_observation_pending = false;
        self.version = next_version;
        if commits_window_close {
            self.components.commit_window_close();
        }
        self.apply_native_commands(&plan.post_publish_commands)
    }

    fn publish_frontend(
        &mut self,
        candidate: CandidateState,
        changes: FrontendChanges,
        reference_commits: &[ReferenceCommit],
    ) -> Result<(), PumpError> {
        if let FrontendChanges::Component(changes) = &changes {
            self.finalize_component_changes(changes)?;
        }
        match candidate {
            CandidateState::Tree { tree, root } => {
                self.tree = tree;
                self.root = Some(root);
            }
            CandidateState::Native {
                node,
                desired,
                reference,
            } => {
                let native = self.tree.native_mut(node)?;
                native.desired = desired;
                native.reference = reference;
            }
        }
        self.apply_reference_bindings(reference_commits);
        match changes {
            #[cfg(any(test, feature = "test"))]
            FrontendChanges::Element(element) => self.element = Some(element),
            FrontendChanges::Component(changes) => self.commit_component_effects(&changes)?,
            FrontendChanges::Local {
                context_reads,
                token,
            } => {
                self.components
                    .set_context_dependencies(token, context_reads)?;
                self.components.commit_effects(token)?;
            }
        }
        Ok(())
    }
}
