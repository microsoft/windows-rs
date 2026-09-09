use super::*;

impl<R: NativeRuntime> Pump<R> {
    pub(super) fn fail_stop(&mut self) {
        self.poisoned = true;
        self.events.clear();
        self.host_events.clear();
        self.realizations.clear();
        self.window_operation = None;
    }

    pub(super) fn fail_component_candidate(
        &mut self,
        changes: &ComponentChanges,
        failure: PlanningFailure,
    ) {
        if matches!(failure, PlanningFailure::Rearm) {
            self.planning_dirty.extend(changes.touched.iter().copied());
        }
        Self::remove_reservations(&mut self.components, &changes.reserved);
    }

    fn fail_frontend_planning(&mut self, changes: &FrontendChanges, failure: PlanningFailure) {
        if let FrontendChanges::Component(changes) = changes {
            self.fail_component_candidate(changes, failure);
        }
    }

    pub(super) fn apply_native_commands(&mut self, commands: &[Command]) -> Result<(), PumpError> {
        if commands.is_empty() {
            return Ok(());
        }
        if let Err(error) = self.runtime.apply(commands) {
            self.fail_stop();
            return Err(PumpError::NativeApplyFailed(error));
        }
        Ok(())
    }

    fn apply_window_opens(&mut self, roots: Vec<View>) {
        if roots.is_empty() {
            return;
        }
        if let Err(error) = self.runtime.open_windows(roots) {
            self.diagnostics
                .push_back(PumpDiagnostic::WindowOpenRejected { error });
        }
    }

    pub(crate) fn process_window_operations(&mut self) -> Result<usize, PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        let Some((window, operation)) = self.window_operation.take() else {
            return Ok(0);
        };
        if !self.components.contains(operation.owner()) {
            return Ok(0);
        }
        let hwnd = match self.runtime.window_handle(window) {
            Ok(hwnd) => hwnd,
            Err(error) => {
                let error = PumpError::WindowHandleFailed(error);
                self.fail_stop();
                return Err(error);
            }
        };
        operation.run(hwnd);
        Ok(1)
    }

    fn queue_window_operation(&mut self, operation: Option<(NodeId, WindowOperation)>) {
        if let Some(operation) = operation {
            if !self.components.contains(operation.1.owner()) {
                return;
            }
            assert!(self.window_operation.is_none());
            self.window_operation = Some(operation);
        }
    }

    fn discard_window_operation(&mut self) {
        self.window_operation = None;
    }

    fn apply_post_publish_commands(
        &mut self,
        commands: &[Command],
        operation: Option<(NodeId, WindowOperation)>,
    ) -> Result<(), PumpError> {
        self.apply_native_commands(commands)?;
        self.queue_window_operation(operation);
        Ok(())
    }

    pub(super) fn publish_candidate(
        &mut self,
        mut candidate: CandidateState,
        plan: UpdatePlan,
        mut changes: FrontendChanges,
        next_version: u64,
        planning_failure: PlanningFailure,
    ) -> Result<(), PumpError> {
        let commits_window_close = plan
            .post_publish_commands
            .iter()
            .any(|command| matches!(command, Command::CloseWindow { .. }));
        if let Err(error) = self.validate_candidate_references(&candidate, &plan.reference_commits)
        {
            self.fail_frontend_planning(&changes, planning_failure);
            return Err(error);
        }
        match &mut changes {
            FrontendChanges::Component(changes) => self.prepare_component_effects(changes),
            FrontendChanges::Local { token, .. } => {
                self.components.prepare_effects(*token);
            }
            #[cfg(test)]
            FrontendChanges::Element(_) => {}
        }

        if let Err(error) = self.apply_native_commands(&plan.commands) {
            self.fail_frontend_planning(&changes, PlanningFailure::Discard);
            return Err(error);
        }

        self.commit_candidate_properties(&mut candidate, &plan.commits);
        self.commit_candidate_references(&mut candidate, &plan.reference_commits);
        self.publish_frontend(candidate, &changes, &plan.reference_commits);
        self.diagnostics.extend(plan.diagnostics);
        self.native_observation_pending = false;
        self.last_native_observation = None;
        self.version = next_version;
        self.apply_window_opens(plan.post_publish_windows);
        if commits_window_close {
            self.components.commit_window_close();
            self.discard_window_operation();
        }
        self.apply_post_publish_commands(
            &plan.post_publish_commands,
            plan.post_publish_window_operation,
        )
    }

    fn publish_frontend(
        &mut self,
        candidate: CandidateState,
        changes: &FrontendChanges,
        reference_commits: &[ReferenceCommit],
    ) {
        if let FrontendChanges::Component(changes) = changes {
            self.finalize_component_changes(changes);
        }
        match candidate {
            CandidateState::Tree { tree, root } => {
                self.tree = tree;
                self.root = Some(root);
            }
            CandidateState::Native {
                node,
                desired,
                exit_transition,
                reference,
            } => {
                let native = self.tree.native_mut(node);
                native.desired = desired;
                native.reference = reference;
                self.tree.set_exit_transition(node, exit_transition);
            }
        }
        self.apply_reference_bindings(reference_commits);
        match changes {
            #[cfg(test)]
            FrontendChanges::Element(element) => self.element = Some(element.clone()),
            FrontendChanges::Component(changes) => self.commit_component_effects(changes),
            FrontendChanges::Local {
                context_reads,
                token,
            } => {
                self.components
                    .set_context_dependencies(*token, context_reads.clone());
                self.components.commit_effects(*token);
            }
        }
    }
}
