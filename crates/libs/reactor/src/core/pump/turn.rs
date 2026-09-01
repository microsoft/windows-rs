use super::*;

impl<R: NativeRuntime> Pump<R> {
    fn trace_component_plan(
        &self,
        components: impl IntoIterator<Item = ComponentToken>,
        plan: &UpdatePlan,
    ) {
        if !self.trace_component_plans || plan.commands.is_empty() {
            return;
        }
        let mut names = components
            .into_iter()
            .map(|token| self.components.type_name(token))
            .collect::<Vec<_>>();
        names.sort_unstable();
        names.dedup();

        let mut creates = 0;
        let mut destroys = 0;
        let mut property_sets = 0;
        let mut property_clears = 0;
        let mut subscriptions = 0;
        let mut topology = 0;
        for command in &plan.commands {
            match command {
                Command::Create { .. } | Command::CreateVirtualCollection { .. } => creates += 1,
                Command::Destroy { .. } | Command::RetireSubtree { .. } => destroys += 1,
                Command::SetProperty { .. } => property_sets += 1,
                Command::ClearProperty { .. } => property_clears += 1,
                Command::SubscribeEvent { .. } | Command::UnsubscribeEvent { .. } => {
                    subscriptions += 1;
                }
                Command::SetSlot { .. }
                | Command::InsertChild { .. }
                | Command::RemoveChild { .. }
                | Command::SynchronizeChildren { .. }
                | Command::MoveChild { .. } => topology += 1,
                _ => {}
            }
        }
        eprintln!(
            "windows-reactor trace: components={names:?} observation={:?} commands={} \
             create={creates} destroy={destroys} set_property={property_sets} \
             clear_property={property_clears} subscriptions={subscriptions} topology={topology}",
            self.last_native_observation,
            plan.commands.len(),
        );
    }

    pub fn dispatch_components(&mut self, budget: usize) -> Result<usize, PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        let mut dispatched = 0;
        for _ in 0..budget {
            if let Some(token) = self.components.next_pending_token()
                && self.has_dirty_component_ancestor(token)
            {
                let deferred = self
                    .components
                    .pending_tokens()
                    .into_iter()
                    .filter(|token| self.has_dirty_component_ancestor(*token))
                    .collect::<IdSet<_>>();
                self.compose_dirty_components(deferred)?;
            }
            let report = self.components.drain(1);
            let processed = report.dispatched + report.dropped;
            dispatched += report.dispatched;
            for token in report.dirty {
                self.dirty_components.insert(token);
            }
            if processed == 0 {
                break;
            }
        }
        if self.dirty_components.is_empty() {
            return Ok(dispatched);
        }
        self.compose_dirty_components(IdSet::default())?;
        Ok(dispatched)
    }

    fn compose_dirty_components(
        &mut self,
        deferred: IdSet<ComponentToken>,
    ) -> Result<(), PumpError> {
        let next_version = self.next_version();
        let mut staged_host_requests = self.components.take_host_requests();
        let mut composed_view = None;
        if self.dirty_components.len() == 1 {
            let Some(token) = self.dirty_components.iter().next().copied() else {
                return Ok(());
            };
            match self.try_local_component_update(token)? {
                LocalComponentUpdate::Plan(mut plan) => {
                    let window = self.window.ok_or(PumpError::NotMounted)?;
                    Self::plan_host_requests(window, &mut staged_host_requests, &mut plan.plan);
                    self.trace_component_plan([token], &plan.plan);
                    self.publish_candidate(
                        CandidateState::Native {
                            node: plan.node,
                            desired: plan.desired,
                            exit_transition: plan.exit_transition,
                            reference: plan.reference,
                        },
                        plan.plan,
                        FrontendChanges::Local {
                            context_reads: plan.context_reads,
                            token,
                        },
                        next_version,
                        PlanningFailure::Rearm,
                    )?;
                    self.planning_dirty.remove(&token);
                    self.dirty_components.clear();
                    return Ok(());
                }
                LocalComponentUpdate::Fallback(render) => composed_view = Some((token, render)),
                LocalComponentUpdate::Unavailable => {}
            }
        }

        let mut candidate = self.tree.clone();
        let mut plan = UpdatePlan {
            reconcile_observations: self.native_observation_pending,
            ..UpdatePlan::new(self.identity)
        };
        let mut changes = ComponentChanges {
            deferred,
            host_requests: staged_host_requests,
            recompose: self.planning_dirty.clone(),
            ..ComponentChanges::default()
        };
        let mut dirty = self
            .dirty_components
            .iter()
            .copied()
            .map(|token| {
                let depth = if let Some(node) = candidate.component_node(token.scope()) {
                    candidate.depth(node)
                } else {
                    usize::MAX
                };
                (depth, token)
            })
            .collect::<Vec<_>>();
        dirty.sort_unstable_by_key(|(depth, _)| *depth);
        for (_, token) in dirty {
            if changes.composed.contains(&token) {
                continue;
            }
            let Some(node) = candidate.component_node(token.scope()) else {
                if changes.retired.contains(&token) {
                    continue;
                }
                panic!("dirty component is missing from the candidate tree");
            };
            let result = if composed_view
                .as_ref()
                .is_some_and(|(cached, _)| *cached == token)
            {
                let (_, render) = composed_view.take().unwrap();
                changes.composed.insert(token);
                Self::reconcile_component_window_title(&mut candidate, token, render.window_title);
                Self::reconcile_component_window_visuals(
                    &mut candidate,
                    token,
                    render.window_visuals,
                );
                candidate
                    .set_window_size_observation(token.scope(), render.window_size_observation);
                candidate
                    .set_color_scheme_observation(token.scope(), render.color_scheme_observation);
                changes.context_reads.insert(token, render.dependencies);
                Self::recompose_component_view(
                    &mut candidate,
                    node,
                    render.view,
                    &mut self.components,
                    &mut changes,
                    &mut plan,
                )
            } else {
                Self::recompose_component(
                    &mut candidate,
                    node,
                    token,
                    &mut self.components,
                    &mut changes,
                    &mut plan,
                )
            };
            if let Err(error) = result {
                self.fail_component_candidate(&changes, PlanningFailure::Rearm);
                return Err(error);
            }
        }
        let root = self.root.ok_or(PumpError::NotMounted)?;
        self.trace_component_plan(changes.composed.iter().copied(), &plan);
        self.apply_component_candidate(candidate, root, plan, changes, next_version)?;
        self.dirty_components.clear();
        Ok(())
    }

    fn has_dirty_component_ancestor(&self, token: ComponentToken) -> bool {
        let Some(mut node) = self.tree.component_node(token.scope()) else {
            return false;
        };
        while let Some(parent) = self.tree.parent(node) {
            node = parent;
            if self.tree.kind(node) == NodeKind::Component {
                let ancestor = self.components.token(self.tree.component_scope(node));
                if self.dirty_components.contains(&ancestor) {
                    return true;
                }
            }
        }
        false
    }

    fn try_local_component_update(
        &mut self,
        token: ComponentToken,
    ) -> Result<LocalComponentUpdate, PumpError> {
        let Some(node) = self.tree.component_node(token.scope()) else {
            return Ok(LocalComponentUpdate::Unavailable);
        };
        let [slot] = self.tree.children(node) else {
            return Ok(LocalComponentUpdate::Unavailable);
        };
        let native = match self.tree.children(*slot) {
            [native] => *native,
            _ => return Ok(LocalComponentUpdate::Unavailable),
        };
        if !matches!(self.tree.kind(native), NodeKind::Native(_))
            || !self.tree.children(native).is_empty()
        {
            return Ok(LocalComponentUpdate::Unavailable);
        }
        let render = self
            .components
            .view(token, self.tree.context_snapshot(node))?;
        let title_matches = match self.tree.window_title() {
            Some(current) if current.owner != token.scope() => {
                if render.window_title.is_some() {
                    return Err(PumpError::DuplicateWindowTitle);
                }
                true
            }
            Some(current) => render.window_title.as_deref() == Some(current.title.as_ref()),
            None => render.window_title.is_none(),
        };
        if !title_matches {
            return Ok(LocalComponentUpdate::Fallback(render));
        }
        let visuals_match = match self.tree.window_visuals() {
            Some(current) if current.owner != token.scope() => {
                if render.window_visuals.is_some() {
                    return Err(PumpError::DuplicateWindowVisuals);
                }
                true
            }
            Some(current) => render.window_visuals == Some(current.visuals),
            None => render.window_visuals.is_none(),
        };
        if !visuals_match {
            return Ok(LocalComponentUpdate::Fallback(render));
        }
        let window_size_matches = match self
            .tree
            .validate_window_size_observation()
            .map_err(|()| PumpError::DuplicateWindowSizeObservation)?
        {
            Some((observation, _)) if observation.owner != token.scope() => {
                if render.window_size_observation.is_some() {
                    return Err(PumpError::DuplicateWindowSizeObservation);
                }
                true
            }
            Some((_, current)) => render.window_size_observation.as_ref() == Some(&current),
            None => render.window_size_observation.is_none(),
        };
        let color_scheme_matches = match self
            .tree
            .validate_color_scheme_observation()
            .map_err(|()| PumpError::DuplicateColorSchemeObservation)?
        {
            Some((observation, _)) if observation.owner != token.scope() => {
                if render.color_scheme_observation.is_some() {
                    return Err(PumpError::DuplicateColorSchemeObservation);
                }
                true
            }
            Some((_, current)) => render.color_scheme_observation.as_ref() == Some(&current),
            None => render.color_scheme_observation.is_none(),
        };
        if !window_size_matches || !color_scheme_matches {
            return Ok(LocalComponentUpdate::Fallback(render));
        }
        let ViewKind::Native(element) = render.view.as_kind() else {
            return Ok(LocalComponentUpdate::Fallback(render));
        };
        if self.tree.kind(native) != NodeKind::Native(element.kind())
            || !self.tree.children(native).is_empty()
            || !matches!(element.structure(), ElementStructureRef::None)
            || self.tree.node_window_title_bar(native).is_some()
            || element.window_title_bar().is_some()
        {
            return Ok(LocalComponentUpdate::Fallback(render));
        }
        let mut event_activity_matches = true;
        element.visit_events(&mut |event, active| {
            event_activity_matches &= self
                .tree
                .try_native(native)
                .and_then(|state| state.events.get(&event))
                .is_some_and(|state| state.active == active);
        });
        if !event_activity_matches {
            return Ok(LocalComponentUpdate::Fallback(render));
        }
        let ComponentRender {
            dependencies, view, ..
        } = render;
        let ViewKind::Native(element) = view.into_kind() else {
            unreachable!()
        };
        let mut plan = UpdatePlan {
            reconcile_observations: self.native_observation_pending,
            ..UpdatePlan::new(self.identity)
        };
        let (desired, exit_transition, reference) = Self::plan_local_native_state(
            self.tree.native(native),
            native,
            element.into_parts(),
            &mut plan,
        )?;
        Ok(LocalComponentUpdate::Plan(LocalCandidate {
            context_reads: dependencies,
            node: native,
            desired,
            exit_transition,
            reference,
            plan,
        }))
    }
}
