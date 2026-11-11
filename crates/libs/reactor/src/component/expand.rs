use super::*;
use std::borrow::Cow;

fn merge_attachments(
    current: &mut Option<Box<DeclaredAttachments>>,
    additional: Option<Box<DeclaredAttachments>>,
) {
    let Some(additional) = additional else {
        return;
    };
    let current = current.get_or_insert_with(Default::default);
    if additional.tooltip.is_some() {
        current.tooltip = additional.tooltip;
    }
    if additional.content_dialog.is_some() {
        current.content_dialog = additional.content_dialog;
    }
}

struct PreparedComponentExpansion {
    attachments: Option<Box<DeclaredAttachments>>,
    declaration_key: Key,
    id: ComponentId,
    view: Option<View>,
}

struct ComponentExpansion {
    attachments: Option<Box<DeclaredAttachments>>,
    node: ComponentNode,
    relation_key: Option<RelationKey>,
}

pub(super) struct ExpansionLocation<'a> {
    pub(super) depth: usize,
    pub(super) path: &'a mut Vec<ComponentPathSegment>,
}

#[derive(Default)]
pub(super) struct ExpansionState {
    objects: usize,
    pub(super) pending: Vec<PendingScopeRender>,
    pub(super) seen: HashMap<ComponentId, HashSet<Key>>,
}

impl ExpansionState {
    pub(super) fn push_render(
        &mut self,
        id: ComponentId,
        render: ComponentRender,
        environment: OwnedContextEnvironment,
    ) -> View {
        let ComponentRender {
            dependencies,
            effects,
            view,
            window,
        } = render;
        self.pending.push(PendingScopeRender {
            dependencies,
            environment: Box::new(environment),
            effects,
            id,
            window,
        });
        view
    }
}

impl<A: Adapter> ComponentHost<A> {
    pub(super) fn expand_view(
        &mut self,
        owner: ComponentId,
        view: View,
        environment: &ContextEnvironment<'_>,
        depth: usize,
        expansion: &mut ExpansionState,
    ) -> View {
        expansion.seen.entry(owner).or_default();
        let mut path = Vec::new();
        View(DeclaredNode::Object(self.expand_root(
            owner,
            view.0,
            environment,
            ExpansionLocation {
                depth,
                path: &mut path,
            },
            expansion,
        )))
    }

    // A root that is another component shares its root object with `owner`.
    fn expand_root(
        &mut self,
        owner: ComponentId,
        node: DeclaredNode,
        environment: &ContextEnvironment<'_>,
        location: ExpansionLocation<'_>,
        expansion: &mut ExpansionState,
    ) -> Declaration {
        match node {
            DeclaredNode::Object(declaration) => {
                self.scope_mut(owner).unwrap().root_child = None;
                self.expand_declaration(
                    owner,
                    Cow::Owned(declaration),
                    environment,
                    true,
                    location,
                    expansion,
                )
                .into_owned()
            }
            DeclaredNode::Provider(provider) => {
                let mut contexts = environment.contexts.clone();
                contexts.insert(provider.provision.id, provider.provision.value);
                let mut provided = environment.provided.clone();
                provided.insert(provider.provision.id);
                self.expand_root(
                    owner,
                    provider.child,
                    &ContextEnvironment {
                        contexts: &contexts,
                        provided: &provided,
                    },
                    location,
                    expansion,
                )
            }
            DeclaredNode::Component {
                node,
                relation_key,
                attachments,
            } => {
                validate_limits(location.depth + 1, expansion.objects);
                let (child, declaration) = self.expand_component(
                    owner,
                    ComponentExpansion {
                        attachments,
                        node,
                        relation_key,
                    },
                    environment,
                    location.depth + 1,
                    expansion,
                    location.path,
                );
                self.scope_mut(owner).unwrap().root_child = Some(child);
                declaration
            }
        }
    }

    pub(super) fn expand_declaration<'a>(
        &mut self,
        owner: ComponentId,
        mut declaration: Cow<'a, Declaration>,
        environment: &ContextEnvironment<'_>,
        scope_root: bool,
        location: ExpansionLocation<'_>,
        expansion: &mut ExpansionState,
    ) -> Cow<'a, Declaration> {
        #[cfg(any(test, feature = "test"))]
        work::record(|work| work.expanded += 1);
        validate_limits(location.depth, expansion.objects);
        expansion.objects += 1;
        if scope_root && declaration.component != Some(owner) {
            declaration.to_mut().component = Some(owner);
        }
        if declaration
            .virtual_items
            .as_ref()
            .is_some_and(|items| items.owner != Some(owner))
        {
            declaration.to_mut().virtual_items.as_mut().unwrap().owner = Some(owner);
        }
        let attachments = match &mut declaration {
            Cow::Borrowed(current) => current.attachments.as_ref().map(Cow::Borrowed),
            Cow::Owned(current) => current.attachments.take().map(Cow::Owned),
        };
        if let Some(Cow::Owned(attachments)) = self.expand_attachments(
            owner,
            attachments,
            environment,
            location.depth,
            expansion,
            location.path,
        ) {
            let attachments = Some(attachments);
            match &mut declaration {
                Cow::Borrowed(current) => {
                    declaration = Cow::Owned(current.clone_with_attachments(attachments));
                }
                Cow::Owned(current) => current.attachments = attachments,
            }
        }
        for relation_index in 0..declaration.relations.as_slice().len() {
            let relation = &declaration.relations.as_slice()[relation_index];
            let value = match &relation.value {
                RelationValue::One(Some(child)) => {
                    location.path.push(ComponentPathSegment::Relation(
                        relation.id,
                        child.relation_identity(0),
                    ));
                    let result = self.expand_node(
                        owner,
                        child,
                        environment,
                        location.depth + 1,
                        expansion,
                        location.path,
                    );
                    location.path.pop();
                    match result {
                        Cow::Owned(expanded) => Some(RelationValue::One(Some(Rc::new(
                            DeclaredNode::Object(expanded),
                        )))),
                        Cow::Borrowed(_) => None,
                    }
                }
                RelationValue::Many(children) => {
                    let mut next = None;
                    for (index, child) in children.iter().enumerate() {
                        let identity = child.relation_identity(index);
                        location
                            .path
                            .push(ComponentPathSegment::Relation(relation.id, identity));
                        let result = self.expand_node(
                            owner,
                            child,
                            environment,
                            location.depth + 1,
                            expansion,
                            location.path,
                        );
                        location.path.pop();
                        match result {
                            Cow::Owned(expanded) => {
                                let next = next.get_or_insert_with(|| {
                                    let mut next = Vec::with_capacity(children.len());
                                    next.extend_from_slice(&children[..index]);
                                    next
                                });
                                next.push(DeclaredNode::Object(expanded));
                            }
                            Cow::Borrowed(_) => {
                                if let Some(next) = &mut next {
                                    next.push(child.clone());
                                }
                            }
                        }
                    }
                    next.map(|next| RelationValue::Many(Rc::new(next)))
                }
                RelationValue::One(None) => None,
            };
            if let Some(value) = value {
                let relation = match &mut declaration.to_mut().relations {
                    SharedList::One(relation) => relation,
                    SharedList::Many(relations) => &mut Rc::make_mut(relations)[relation_index],
                    SharedList::Empty => unreachable!(),
                };
                relation.value = value;
            }
        }
        declaration
    }

    pub(super) fn expand_node<'a>(
        &mut self,
        owner: ComponentId,
        node: &'a DeclaredNode,
        environment: &ContextEnvironment<'_>,
        depth: usize,
        expansion: &mut ExpansionState,
        path: &mut Vec<ComponentPathSegment>,
    ) -> Cow<'a, Declaration> {
        match node {
            DeclaredNode::Object(declaration) => self.expand_declaration(
                owner,
                Cow::Borrowed(declaration),
                environment,
                false,
                ExpansionLocation { depth, path },
                expansion,
            ),
            DeclaredNode::Provider(provider) => Cow::Owned(self.expand_provider_node(
                owner,
                &provider.provision,
                &provider.child,
                environment,
                ExpansionLocation { depth, path },
                expansion,
            )),
            DeclaredNode::Component {
                node,
                relation_key,
                attachments,
            } => Cow::Owned(
                self.expand_component(
                    owner,
                    ComponentExpansion {
                        attachments: attachments.clone(),
                        node: node.clone(),
                        relation_key: relation_key.clone(),
                    },
                    environment,
                    depth,
                    expansion,
                    path,
                )
                .1,
            ),
        }
    }

    fn expand_component(
        &mut self,
        owner: ComponentId,
        component: ComponentExpansion,
        environment: &ContextEnvironment<'_>,
        depth: usize,
        expansion: &mut ExpansionState,
        path: &mut Vec<ComponentPathSegment>,
    ) -> (ComponentId, Declaration) {
        let mut prepared = self.prepare_component(owner, component, environment, expansion, path);
        let expanded = self.expand_view(
            prepared.id,
            prepared.view.take().unwrap(),
            environment,
            depth,
            expansion,
        );
        let mut declaration = expanded.0.object();
        declaration.key = Some(prepared.declaration_key);
        let attachments = self
            .expand_attachments(
                owner,
                prepared.attachments.take().map(Cow::Owned),
                environment,
                depth,
                expansion,
                path,
            )
            .map(Cow::into_owned);
        merge_attachments(&mut declaration.attachments, attachments);
        (prepared.id, declaration)
    }

    fn prepare_component(
        &mut self,
        owner: ComponentId,
        component: ComponentExpansion,
        environment: &ContextEnvironment<'_>,
        expansion: &mut ExpansionState,
        path: &[ComponentPathSegment],
    ) -> Box<PreparedComponentExpansion> {
        let ComponentExpansion {
            attachments,
            mut node,
            relation_key,
        } = component;
        if let Some(component_key) = node.key.as_ref()
            && let Some(RelationKey::Explicit(relation_key)) = &relation_key
            && relation_key != component_key
        {
            panic!("component key {component_key:?} does not match relation key {relation_key:?}");
        }
        let component_key = node.key.clone().unwrap_or_else(|| {
            let mut path = path.to_vec();
            path.push(ComponentPathSegment::Component(
                node.factory.component_type(),
            ));
            Key::component_path(&path)
        });
        let declaration_key = relation_key
            .as_ref()
            .map_or(&component_key, RelationKey::key)
            .clone();
        node.key = Some(component_key.clone());
        let seen = expansion.seen.entry(owner).or_default();
        assert!(
            seen.insert(component_key.clone()),
            "duplicate key {component_key:?}"
        );
        let existing = self
            .scope(owner)
            .and_then(|scope| scope.children.get(&component_key).copied());
        let (id, view) = if let Some(id) = existing {
            assert!(
                self.scope(id).unwrap().component.component_type() == node.factory.component_type(),
                "component {component_key:?} changed type"
            );
            let scope = self.scope_mut(id).unwrap();
            let reference = scope.reference.clone();
            let render = scope.component.apply_input(
                node.factory.input(),
                reference.clone(),
                environment.contexts,
            );
            let render = match render {
                Some(render) => Some(render),
                None if scope.dependencies_changed(environment.contexts) => {
                    Some(scope.component.render_view(reference, environment.contexts))
                }
                None => None,
            };
            let view = if let Some(render) = render {
                expansion.push_render(
                    id,
                    render,
                    OwnedContextEnvironment {
                        contexts: environment.contexts.clone(),
                        provided: environment.provided.clone(),
                    },
                )
            } else {
                scope.contexts.clone_from(environment.contexts);
                scope.provided_contexts.clone_from(environment.provided);
                scope.component.cached_view()
            };
            (id, view)
        } else {
            let created = self.create_scope(
                Some(owner),
                node,
                environment.contexts.clone(),
                environment.provided.clone(),
            );
            self.scope_mut(owner)
                .unwrap()
                .children
                .insert(component_key, created.0);
            let view = expansion.push_render(
                created.0,
                created.1,
                OwnedContextEnvironment {
                    contexts: environment.contexts.clone(),
                    provided: environment.provided.clone(),
                },
            );
            (created.0, view)
        };
        Box::new(PreparedComponentExpansion {
            attachments,
            declaration_key,
            id,
            view: Some(view),
        })
    }

    fn expand_provider_node(
        &mut self,
        owner: ComponentId,
        provision: &ContextProvision,
        child: &DeclaredNode,
        environment: &ContextEnvironment<'_>,
        location: ExpansionLocation<'_>,
        expansion: &mut ExpansionState,
    ) -> Declaration {
        let mut contexts = environment.contexts.clone();
        contexts.insert(provision.id, provision.value.clone());
        let mut provided = environment.provided.clone();
        provided.insert(provision.id);
        self.expand_node(
            owner,
            child,
            &ContextEnvironment {
                contexts: &contexts,
                provided: &provided,
            },
            location.depth,
            expansion,
            location.path,
        )
        .into_owned()
    }

    fn expand_attachments<'a>(
        &mut self,
        owner: ComponentId,
        attachments: Option<Cow<'a, Box<DeclaredAttachments>>>,
        environment: &ContextEnvironment<'_>,
        depth: usize,
        expansion: &mut ExpansionState,
        path: &mut Vec<ComponentPathSegment>,
    ) -> Option<Cow<'a, Box<DeclaredAttachments>>> {
        let mut attachments = attachments?;
        if let Some(tooltip) = &attachments.tooltip {
            validate_limits(depth, expansion.objects);
            expansion.objects += 1;
            path.push(ComponentPathSegment::Tooltip);
            let result = self.expand_node(
                owner,
                &tooltip.content,
                environment,
                depth + 2,
                expansion,
                path,
            );
            path.pop();
            if let Cow::Owned(content) = result {
                *attachments.to_mut().tooltip.as_mut().unwrap().content =
                    DeclaredNode::Object(content);
            }
        }
        if let Some(flyout) = &attachments.flyout {
            validate_limits(depth, expansion.objects);
            expansion.objects += 1;
            path.push(ComponentPathSegment::Flyout);
            let result = self.expand_node(
                owner,
                &flyout.content,
                environment,
                depth + 2,
                expansion,
                path,
            );
            path.pop();
            if let Cow::Owned(content) = result {
                *attachments.to_mut().flyout.as_mut().unwrap().content =
                    DeclaredNode::Object(content);
            }
        }
        if let Some(dialog) = &attachments.content_dialog {
            path.push(ComponentPathSegment::ContentDialog);
            let result = self.expand_declaration(
                owner,
                Cow::Borrowed(&dialog.declaration),
                environment,
                false,
                ExpansionLocation {
                    depth: depth + 1,
                    path,
                },
                expansion,
            );
            path.pop();
            if let Cow::Owned(declaration) = result {
                attachments
                    .to_mut()
                    .content_dialog
                    .as_mut()
                    .unwrap()
                    .declaration = declaration;
            }
        }
        Some(attachments)
    }

    pub(super) fn refresh_roots(&mut self, root: &View) {
        let object = self.runtime.graph().root().unwrap();
        let declaration = root.0.as_object();
        self.refresh_roots_from(declaration, object);
    }

    pub(super) fn refresh_roots_from(&mut self, declaration: &Declaration, object: ObjectId) {
        #[cfg(any(test, feature = "test"))]
        work::record(|work| work.refreshed += 1);
        // The declaring component and each ancestor whose view is that component share the root.
        let mut component = declaration.component;
        while let Some(id) = component {
            let scope = self.scope_mut(id).unwrap();
            scope.root = Some(object);
            scope.reference.set(Some(object));
            component = scope
                .parent
                .filter(|&parent| self.scope(parent).unwrap().root_child == Some(id));
        }
        if let Some(attachments) = &declaration.attachments {
            if let Some(tooltip) = &attachments.tooltip {
                let tooltip_object = self.runtime.graph().tooltip(object).unwrap();
                let content_object = self
                    .runtime
                    .graph()
                    .child(tooltip_object, RelationId::Content)
                    .unwrap();
                self.refresh_roots_from(tooltip.content.as_object(), content_object);
            }
            if let Some(flyout) = &attachments.flyout {
                let content_object = self.runtime.graph().flyout(object).unwrap().0;
                self.refresh_roots_from(flyout.content.as_object(), content_object);
            }
            if let Some(dialog) = &attachments.content_dialog {
                let dialog_object = self.runtime.graph().content_dialog(object).unwrap().0;
                self.refresh_roots_from(&dialog.declaration, dialog_object);
            }
        }
        for relation in declaration.relations.iter() {
            match &relation.value {
                RelationValue::One(Some(child)) => {
                    let object = self.runtime.graph().child(object, relation.id).unwrap();
                    self.refresh_roots_from(child.as_object(), object);
                }
                RelationValue::Many(children) => {
                    for (index, child) in children.iter().enumerate() {
                        let object =
                            self.runtime.graph().children(object, relation.id).unwrap()[index];
                        self.refresh_roots_from(child.as_object(), object);
                    }
                }
                RelationValue::One(None) => {}
            }
        }
    }
}
