use super::*;

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
        let declaration = match view.0 {
            DeclaredNode::Object(declaration) => declaration,
            DeclaredNode::Component { .. } => {
                panic!("a component view cannot be another component")
            }
            DeclaredNode::Provider(provider) => {
                return self.expand_provider_view(
                    owner,
                    provider.provision,
                    provider.child,
                    environment,
                    ExpansionLocation {
                        depth,
                        path: &mut path,
                    },
                    expansion,
                );
            }
        };
        View(DeclaredNode::Object(self.expand_declaration(
            owner,
            declaration,
            environment,
            true,
            ExpansionLocation {
                depth,
                path: &mut path,
            },
            expansion,
        )))
    }

    fn expand_provider_view(
        &mut self,
        owner: ComponentId,
        provision: ContextProvision,
        child: DeclaredNode,
        environment: &ContextEnvironment<'_>,
        location: ExpansionLocation<'_>,
        expansion: &mut ExpansionState,
    ) -> View {
        let mut contexts = environment.contexts.clone();
        contexts.insert(provision.id, provision.value);
        let mut provided = environment.provided.clone();
        provided.insert(provision.id);
        match child {
            DeclaredNode::Object(declaration) => {
                View(DeclaredNode::Object(self.expand_declaration(
                    owner,
                    declaration,
                    &ContextEnvironment {
                        contexts: &contexts,
                        provided: &provided,
                    },
                    true,
                    location,
                    expansion,
                )))
            }
            _ => panic!("a context provider in a component view must wrap an element"),
        }
    }

    pub(super) fn expand_declaration(
        &mut self,
        owner: ComponentId,
        mut declaration: Declaration,
        environment: &ContextEnvironment<'_>,
        scope_root: bool,
        location: ExpansionLocation<'_>,
        expansion: &mut ExpansionState,
    ) -> Declaration {
        self.expand_declaration_in_place(
            owner,
            &mut declaration,
            environment,
            scope_root,
            location,
            expansion,
        );
        declaration
    }

    fn expand_declaration_in_place(
        &mut self,
        owner: ComponentId,
        declaration: &mut Declaration,
        environment: &ContextEnvironment<'_>,
        scope_root: bool,
        location: ExpansionLocation<'_>,
        expansion: &mut ExpansionState,
    ) {
        validate_limits(location.depth, expansion.objects);
        expansion.objects += 1;
        if scope_root {
            declaration.component = Some(owner);
        }
        if let Some(items) = declaration.virtual_items.as_mut() {
            items.owner = Some(owner);
        }
        declaration.attachments = self.expand_attachments(
            owner,
            declaration.attachments.take(),
            environment,
            location.depth,
            expansion,
            location.path,
        );
        let mut relations = std::mem::take(&mut declaration.relations);
        let relation_slice = match &mut relations {
            SharedList::Empty => &mut [],
            SharedList::One(relation) => std::slice::from_mut(relation),
            SharedList::Many(relations) => Rc::make_mut(relations),
        };
        for relation in relation_slice {
            let value = std::mem::replace(&mut relation.value, RelationValue::One(None));
            relation.value = match value {
                RelationValue::One(Some(child)) => {
                    let mut retained = Some(child);
                    location.path.push(ComponentPathSegment::Relation(
                        relation.id,
                        retained.as_ref().unwrap().relation_identity(0),
                    ));
                    let result = if let Some(DeclaredNode::Object(declaration)) =
                        Rc::get_mut(retained.as_mut().unwrap())
                    {
                        self.expand_declaration_in_place(
                            owner,
                            declaration,
                            environment,
                            false,
                            ExpansionLocation {
                                depth: location.depth + 1,
                                path: location.path,
                            },
                            expansion,
                        );
                        None
                    } else {
                        let child = Rc::try_unwrap(retained.take().unwrap())
                            .unwrap_or_else(|child| child.as_ref().clone());
                        Some(self.expand_node(
                            owner,
                            child,
                            environment,
                            location.depth + 1,
                            expansion,
                            location.path,
                        ))
                    };
                    location.path.pop();
                    match result {
                        Some(expanded) => {
                            RelationValue::One(Some(Rc::new(DeclaredNode::Object(expanded))))
                        }
                        None => RelationValue::One(retained),
                    }
                }
                RelationValue::Many(mut children) => {
                    for (index, child) in Rc::make_mut(&mut children).iter_mut().enumerate() {
                        let identity = child.relation_identity(index);
                        location
                            .path
                            .push(ComponentPathSegment::Relation(relation.id, identity));
                        let result = match child {
                            DeclaredNode::Object(declaration) => {
                                self.expand_declaration_in_place(
                                    owner,
                                    declaration,
                                    environment,
                                    false,
                                    ExpansionLocation {
                                        depth: location.depth + 1,
                                        path: location.path,
                                    },
                                    expansion,
                                );
                                None
                            }
                            _ => Some(self.expand_node(
                                owner,
                                child.clone(),
                                environment,
                                location.depth + 1,
                                expansion,
                                location.path,
                            )),
                        };
                        location.path.pop();
                        if let Some(expanded) = result {
                            *child = DeclaredNode::Object(expanded);
                        }
                    }
                    RelationValue::Many(children)
                }
                RelationValue::One(None) => RelationValue::One(None),
            };
        }
        declaration.relations = relations;
    }

    pub(super) fn expand_node(
        &mut self,
        owner: ComponentId,
        node: DeclaredNode,
        environment: &ContextEnvironment<'_>,
        depth: usize,
        expansion: &mut ExpansionState,
        path: &mut Vec<ComponentPathSegment>,
    ) -> Declaration {
        match node {
            DeclaredNode::Object(declaration) => self.expand_declaration(
                owner,
                declaration,
                environment,
                false,
                ExpansionLocation { depth, path },
                expansion,
            ),
            DeclaredNode::Provider(provider) => self.expand_provider_node(
                owner,
                provider.provision,
                provider.child,
                environment,
                ExpansionLocation { depth, path },
                expansion,
            ),
            DeclaredNode::Component {
                node,
                relation_key,
                attachments,
            } => self.expand_component(
                owner,
                ComponentExpansion {
                    attachments,
                    node,
                    relation_key,
                },
                environment,
                depth,
                expansion,
                path,
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
    ) -> Declaration {
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
        let attachments = self.expand_attachments(
            owner,
            prepared.attachments.take(),
            environment,
            depth,
            expansion,
            path,
        );
        merge_attachments(&mut declaration.attachments, attachments);
        declaration
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
        provision: ContextProvision,
        child: DeclaredNode,
        environment: &ContextEnvironment<'_>,
        location: ExpansionLocation<'_>,
        expansion: &mut ExpansionState,
    ) -> Declaration {
        let mut contexts = environment.contexts.clone();
        contexts.insert(provision.id, provision.value);
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
    }

    fn expand_attachments(
        &mut self,
        owner: ComponentId,
        attachments: Option<Box<DeclaredAttachments>>,
        environment: &ContextEnvironment<'_>,
        depth: usize,
        expansion: &mut ExpansionState,
        path: &mut Vec<ComponentPathSegment>,
    ) -> Option<Box<DeclaredAttachments>> {
        let mut attachments = attachments?;
        if let Some(mut tooltip) = attachments.tooltip.take() {
            validate_limits(depth, expansion.objects);
            expansion.objects += 1;
            path.push(ComponentPathSegment::Tooltip);
            let result = self.expand_node(
                owner,
                *tooltip.content,
                environment,
                depth + 2,
                expansion,
                path,
            );
            path.pop();
            let expanded = result;
            tooltip.content = Box::new(DeclaredNode::Object(expanded));
            attachments.tooltip = Some(tooltip);
        }
        if let Some(mut flyout) = attachments.flyout.take() {
            validate_limits(depth, expansion.objects);
            expansion.objects += 1;
            path.push(ComponentPathSegment::Flyout);
            let result = self.expand_node(
                owner,
                *flyout.content,
                environment,
                depth + 2,
                expansion,
                path,
            );
            path.pop();
            let expanded = result;
            flyout.content = Box::new(DeclaredNode::Object(expanded));
            attachments.flyout = Some(flyout);
        }
        if let Some(mut dialog) = attachments.content_dialog.take() {
            path.push(ComponentPathSegment::ContentDialog);
            let result = self.expand_declaration(
                owner,
                dialog.declaration,
                environment,
                false,
                ExpansionLocation {
                    depth: depth + 1,
                    path,
                },
                expansion,
            );
            path.pop();
            dialog.declaration = result;
            attachments.content_dialog = Some(dialog);
        }
        Some(attachments)
    }

    pub(super) fn refresh_roots(&mut self, root: &View) {
        let object = self.runtime.graph().root().unwrap();
        let declaration = root.0.as_object();
        self.refresh_roots_from(declaration, object);
    }

    pub(super) fn refresh_roots_from(&mut self, declaration: &Declaration, object: ObjectId) {
        if let Some(id) = declaration.component {
            let scope = self.scope_mut(id).unwrap();
            scope.root = Some(object);
            scope.reference.set(Some(object));
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
