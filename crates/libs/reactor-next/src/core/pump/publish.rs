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
        self.validate_candidate_references(&candidate, &plan.reference_commits)?;
        match &mut changes {
            FrontendChanges::Component(changes) => self.prepare_component_effects(changes)?,
            FrontendChanges::Local { token, .. } => self.components.prepare_effects(*token)?,
            #[cfg(any(test, feature = "test"))]
            FrontendChanges::Element(_) => {}
        }

        self.apply_native_commands(&plan.commands)?;

        self.commit_candidate_properties(&mut candidate, &plan.commits)?;
        self.commit_candidate_references(&mut candidate, &plan.reference_commits)?;
        self.publish_frontend(candidate, changes, &plan.reference_commits)?;
        self.native_observation_pending = false;
        self.version = next_version;
        Ok(())
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
