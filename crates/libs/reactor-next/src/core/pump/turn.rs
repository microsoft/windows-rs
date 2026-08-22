use super::*;

impl<R: NativeRuntime> Pump<R> {
    pub fn dispatch_components(&mut self, budget: usize) -> Result<usize, PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        let mut dispatched = 0;
        for _ in 0..budget {
            if let Some(token) = self.components.next_pending_token()
                && self.has_dirty_component_ancestor(token)?.is_some()
            {
                let deferred = self
                    .components
                    .pending_tokens()
                    .into_iter()
                    .filter_map(|token| {
                        self.has_dirty_component_ancestor(token)
                            .transpose()
                            .map(|result| result.map(|()| token))
                    })
                    .collect::<Result<HashSet<_>, PumpError>>()?;
                self.compose_dirty_components(deferred)?;
            }
            let report = self.components.drain(1)?;
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
        self.compose_dirty_components(HashSet::new())?;
        Ok(dispatched)
    }

    fn compose_dirty_components(
        &mut self,
        deferred: HashSet<ComponentToken>,
    ) -> Result<(), PumpError> {
        let next_version = self.next_version()?;
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
                    self.publish_candidate(
                        CandidateState::Native {
                            node: plan.node,
                            desired: plan.desired,
                            reference: plan.reference,
                        },
                        plan.plan,
                        FrontendChanges::Local {
                            context_reads: plan.context_reads,
                            token,
                        },
                        next_version,
                        CandidateFailureStage::PlanningRetry,
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
            retry: self.planning_dirty.clone(),
            ..ComponentChanges::default()
        };
        let mut dirty = self
            .dirty_components
            .iter()
            .copied()
            .map(|token| {
                let depth = if let Some(node) = candidate.component_node(token.scope())? {
                    candidate.depth(node)?
                } else {
                    usize::MAX
                };
                Ok((depth, token))
            })
            .collect::<Result<Vec<_>, PumpError>>()?;
        dirty.sort_unstable_by_key(|(depth, _)| *depth);
        for (_, token) in dirty {
            if changes.composed.contains(&token) {
                continue;
            }
            let Some(node) = candidate.component_node(token.scope())? else {
                if changes.retired.contains(&token) {
                    continue;
                }
                self.fail_component_candidate(&changes, CandidateFailureStage::PlanningRetry);
                return Err(PumpError::StructureUnsupported);
            };
            let result = if composed_view
                .as_ref()
                .is_some_and(|(cached, _)| *cached == token)
            {
                let (_, render) = composed_view.take().unwrap();
                changes.composed.insert(token);
                Self::reconcile_component_window_title(
                    &mut candidate,
                    token,
                    render.duplicate_window_title,
                    render.window_title,
                )?;
                Self::reconcile_component_window_visuals(
                    &mut candidate,
                    token,
                    render.duplicate_window_visuals,
                    render.window_visuals,
                )?;
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
                self.fail_component_candidate(&changes, CandidateFailureStage::PlanningRetry);
                return Err(error);
            }
        }
        let root = self.root.ok_or(PumpError::NotMounted)?;
        self.apply_component_candidate(candidate, root, plan, changes, next_version)?;
        self.dirty_components.clear();
        Ok(())
    }

    fn has_dirty_component_ancestor(&self, token: ComponentToken) -> Result<Option<()>, PumpError> {
        let Some(mut node) = self.tree.component_node(token.scope())? else {
            return Ok(None);
        };
        while let Some(parent) = self.tree.parent(node)? {
            node = parent;
            if self.tree.kind(node)? == NodeKind::Component {
                let ancestor = self.components.token(self.tree.component_scope(node)?)?;
                if self.dirty_components.contains(&ancestor) {
                    return Ok(Some(()));
                }
            }
        }
        Ok(None)
    }

    fn try_local_component_update(
        &mut self,
        token: ComponentToken,
    ) -> Result<LocalComponentUpdate, PumpError> {
        let Some(node) = self.tree.component_node(token.scope())? else {
            return Ok(LocalComponentUpdate::Unavailable);
        };
        let [slot] = self.tree.children(node)? else {
            return Ok(LocalComponentUpdate::Unavailable);
        };
        let native = match self.tree.children(*slot)? {
            [native] => *native,
            _ => return Ok(LocalComponentUpdate::Unavailable),
        };
        if !matches!(self.tree.kind(native)?, NodeKind::Native(_))
            || !self.tree.children(native)?.is_empty()
        {
            return Ok(LocalComponentUpdate::Unavailable);
        }
        let render = self
            .components
            .view(token, self.tree.context_snapshot(node)?)?;
        if render.duplicate_window_title {
            return Err(PumpError::DuplicateWindowTitle);
        }
        if render.duplicate_window_visuals {
            return Err(PumpError::DuplicateWindowVisuals);
        }
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
        let ComponentRender {
            dependencies,
            duplicate_window_title,
            duplicate_window_visuals,
            view,
            window_title,
            window_visuals,
        } = render;
        let element = match view.into_kind() {
            ViewKind::Native(element) => element,
            kind => {
                return Ok(LocalComponentUpdate::Fallback(ComponentRender {
                    dependencies,
                    duplicate_window_title,
                    duplicate_window_visuals,
                    view: View::from_kind(kind),
                    window_title,
                    window_visuals,
                }));
            }
        };
        if self.tree.kind(native)? != NodeKind::Native(element.kind())
            || !self.tree.children(native)?.is_empty()
            || !matches!(element.structure(), ElementStructureRef::None)
        {
            return Ok(LocalComponentUpdate::Fallback(ComponentRender {
                dependencies,
                duplicate_window_title,
                duplicate_window_visuals,
                view: View::native(element),
                window_title,
                window_visuals,
            }));
        }
        let mut event_activity_matches = true;
        element.visit_events(&mut |event, active| {
            event_activity_matches &= self
                .tree
                .native(native)
                .ok()
                .and_then(|state| state.events.get(&event))
                .is_some_and(|state| state.active == active);
        });
        if !event_activity_matches {
            return Ok(LocalComponentUpdate::Fallback(ComponentRender {
                dependencies,
                duplicate_window_title,
                duplicate_window_visuals,
                view: View::native(element),
                window_title,
                window_visuals,
            }));
        }
        let mut plan = UpdatePlan {
            reconcile_observations: self.native_observation_pending,
            ..UpdatePlan::new(self.identity)
        };
        let (desired, reference) = Self::plan_local_native_state(
            self.tree.native(native)?,
            native,
            element.into_parts(),
            &mut plan,
        )?;
        Ok(LocalComponentUpdate::Plan(LocalCandidate {
            context_reads: dependencies,
            node: native,
            desired,
            reference,
            plan,
        }))
    }
}
