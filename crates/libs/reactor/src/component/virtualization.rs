use super::*;

struct VirtualRow;

impl Component for VirtualRow {
    type Input = View;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        input.clone()
    }
}

impl<A: Adapter> ComponentHost<A> {
    /// Plans the realization at the front of the native queue, unless it targets a row whose batch
    /// is being applied or renders a row whose effects are not yet committed.
    pub(crate) fn next_realization_batch(&mut self) -> Option<NativeBatch> {
        let pending = self.pending_rows();
        let work = self.runtime.next_realization(&self.busy_rows, &pending)?;
        Some(self.plan_virtual_work(work))
    }

    pub(super) fn plan_virtual_work(&mut self, work: VirtualWork) -> NativeBatch {
        self.begin_batch();
        match work {
            VirtualWork::Realize {
                lease,
                index,
                view,
                owner,
            } => {
                let view = *view;
                if let Some(owner) = owner
                    && self.scope(owner).is_none()
                {
                    return NativeBatch::default();
                }
                let row_key = (lease.collection, lease.key.clone());
                if let Some(row) = self
                    .runtime
                    .graph()
                    .virtual_row_key(lease.collection, lease.container)
                {
                    self.busy_rows.push((lease.collection, row));
                }
                self.busy_rows.push(row_key.clone());
                if let Some(owner) = owner {
                    self.mark_busy_ancestors(owner);
                }
                let existing = self.virtual_rows.get(&row_key).copied();
                let contexts = owner.map_or_else(
                    || self.contexts.clone(),
                    |owner| self.scope(owner).unwrap().contexts.clone(),
                );
                let provided_contexts = owner.map_or_else(HashSet::new, |owner| {
                    self.scope(owner).unwrap().provided_contexts.clone()
                });
                let mut expansion = ExpansionState::default();
                let (id, view) = if let Some(id) = existing {
                    let reference = self.scope(id).unwrap().reference.clone();
                    let view = {
                        let scope = self.scope_mut(id).unwrap();
                        let render =
                            scope
                                .component
                                .apply_input(&view, reference.clone(), &contexts);
                        let render = match render {
                            Some(render) => Some(render),
                            None if scope.dependencies_changed(&contexts) => {
                                Some(scope.component.render_view(reference, &contexts))
                            }
                            None => None,
                        };
                        if let Some(render) = render {
                            expansion.push_render(
                                id,
                                render,
                                OwnedContextEnvironment {
                                    contexts: contexts.clone(),
                                    provided: provided_contexts.clone(),
                                },
                            )
                        } else {
                            scope.component.cached_view()
                        }
                    };
                    (id, view)
                } else {
                    let node = component::<VirtualRow>(lease.key.clone(), view);
                    let (id, render) =
                        self.create_scope(owner, node, contexts.clone(), provided_contexts.clone());
                    let view = expansion.push_render(
                        id,
                        render,
                        OwnedContextEnvironment {
                            contexts: contexts.clone(),
                            provided: provided_contexts.clone(),
                        },
                    );
                    (id, view)
                };
                self.scope_mut(id).unwrap().contexts = contexts;
                self.scope_mut(id).unwrap().provided_contexts = provided_contexts;
                let declaration = self.expand_virtual_view(id, view, &mut expansion);
                let retired = self.unseen_scopes(&expansion.seen);
                let publication = self.resolve_window_publication(&expansion.pending, &retired);
                let batch = self
                    .runtime
                    .plan_realize_virtual(&lease, index, declaration.clone());
                let root = self
                    .runtime
                    .graph()
                    .virtual_realization(lease.collection, lease.container)
                    .unwrap();
                self.virtual_rows.insert(row_key, id);
                self.refresh_roots_from(declaration.0.as_object(), root);
                self.prepare_pending(expansion.pending);
                self.prepare_scope_retirement(&retired);
                self.retire_unseen(&expansion.seen);
                self.sweep_virtual_rows();
                self.publish_committed_window(publication);
                batch
            }
            VirtualWork::Recycle { lease } => {
                let row_key = (lease.collection, lease.key.clone());
                if let Some(row) = self.virtual_rows.get(&row_key).copied() {
                    self.prepare_scope_retirement(&[row]);
                }
                self.busy_rows.push(row_key);
                let batch = self.runtime.plan_recycle_virtual(&lease);
                self.sweep_virtual_rows();
                self.publish_window();
                batch
            }
            VirtualWork::Cancel {
                collection,
                relation,
                container,
            } => self
                .runtime
                .plan_cancel_virtual(collection, relation, container),
        }
    }

    pub(super) fn sweep_virtual_rows(&mut self) {
        let retired = self
            .virtual_rows
            .iter()
            .filter_map(|((collection, key), id)| {
                (!self.runtime.virtual_lease_active(*collection, key))
                    .then_some(((*collection, key.clone()), *id))
            })
            .collect::<Vec<_>>();
        for (key, id) in retired {
            self.virtual_rows.remove(&key);
            self.prepare_scope_retirement(&[id]);
            self.retire_scope(id);
        }
    }

    fn expand_virtual_view(
        &mut self,
        owner: ComponentId,
        view: View,
        expansion: &mut ExpansionState,
    ) -> View {
        expansion.seen.entry(owner).or_default();
        let contexts = self.scope(owner).unwrap().contexts.clone();
        let provided_contexts = self.scope(owner).unwrap().provided_contexts.clone();
        let environment = ContextEnvironment {
            contexts: &contexts,
            provided: &provided_contexts,
        };
        let mut path = Vec::new();
        let declaration = match view.0 {
            DeclaredNode::Object(declaration) => self.expand_declaration(
                owner,
                declaration,
                &environment,
                true,
                ExpansionLocation {
                    depth: 0,
                    path: &mut path,
                },
                expansion,
            ),
            node => {
                path.push(ComponentPathSegment::VirtualRoot);
                self.expand_node(owner, node, &environment, 0, expansion, &mut path)
            }
        };
        View(DeclaredNode::Object(declaration))
    }
}
