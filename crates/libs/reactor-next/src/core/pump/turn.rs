use super::*;

impl<R: NativeRuntime> Pump<R> {
    pub fn dispatch_components(&mut self, budget: usize) -> Result<usize, PumpError> {
        if self.poisoned {
            return Err(PumpError::Poisoned);
        }
        let mut dispatched = 0;
        for _ in 0..budget {
            let report = self.components.drain(1)?;
            let processed = report.dispatched + report.dropped;
            dispatched += report.dispatched;
            for token in report.dirty {
                self.dirty_components.insert(token);
            }
            if processed == 0 {
                break;
            }

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
            if !deferred.is_empty() {
                self.compose_dirty_components(deferred)?;
            }
        }
        if self.dirty_components.is_empty() && self.retry_pending {
            let root = self.root.ok_or(PumpError::NotMounted)?;
            for node in self.tree.subtree_postorder(root)? {
                if self.tree.kind(node)? == NodeKind::Component {
                    self.dirty_components
                        .insert(self.components.token(self.tree.component_scope(node)?)?);
                }
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
        let mut composed_view = None;
        if self.dirty_components.len() == 1 {
            let Some(token) = self.dirty_components.iter().next().copied() else {
                return Ok(());
            };
            match self.try_local_component_update(token)? {
                LocalComponentUpdate::Plan(plan) => {
                    self.publish_candidate(
                        CandidateState::Native {
                            node: plan.node,
                            desired: plan.desired,
                        },
                        plan.plan,
                        FrontendChanges::Local(token),
                        next_version,
                    )?;
                    self.dirty_components.clear();
                    return Ok(());
                }
                LocalComponentUpdate::Fallback(view) => composed_view = Some((token, view)),
                LocalComponentUpdate::Unavailable => {}
            }
        }

        let mut candidate = self.tree.clone();
        let mut plan = UpdatePlan {
            retry_properties: self.retry_pending,
            ..UpdatePlan::new(self.identity)
        };
        let mut changes = ComponentChanges {
            deferred,
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
                Self::remove_reservations(&mut self.components, &changes.reserved);
                return Err(PumpError::StructureUnsupported);
            };
            let result = if composed_view
                .as_ref()
                .is_some_and(|(cached, _)| *cached == token)
            {
                let (_, view) = composed_view.take().unwrap();
                changes.composed.insert(token);
                Self::recompose_component_view(
                    &mut candidate,
                    node,
                    view,
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
                Self::remove_reservations(&mut self.components, &changes.reserved);
                return Err(error);
            }
        }
        let root = self.root.ok_or(PumpError::NotMounted)?;
        match self.apply_component_candidate(candidate, root, plan, changes, next_version) {
            Ok(_) => {
                self.dirty_components.clear();
                Ok(())
            }
            Err(error @ PumpError::RecoveredStructure(_)) => {
                self.dirty_components.clear();
                Err(error)
            }
            Err(error) => Err(error),
        }
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
        let view = self.components.view(token)?;
        let View::Native(element) = view else {
            return Ok(LocalComponentUpdate::Fallback(view));
        };
        if self.tree.kind(native)? != NodeKind::Native(element.kind())
            || !self.tree.children(native)?.is_empty()
            || !matches!(element.structure(), ElementStructureRef::None)
        {
            return Ok(LocalComponentUpdate::Fallback(View::Native(element)));
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
            return Ok(LocalComponentUpdate::Fallback(View::Native(element)));
        }
        let mut plan = UpdatePlan {
            retry_properties: self.retry_pending,
            ..UpdatePlan::new(self.identity)
        };
        let desired = Self::plan_local_native_state(
            self.tree.native(native)?,
            native,
            element.into_parts(),
            &mut plan,
        )?;
        debug_assert!(plan.commands.iter().all(|command| !command.structural()));
        Ok(LocalComponentUpdate::Plan(LocalCandidate {
            node: native,
            desired,
            plan,
        }))
    }
}
