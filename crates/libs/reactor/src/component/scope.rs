use super::*;

pub(super) struct ComponentRender {
    pub(super) dependencies: HashSet<ContextId>,
    pub(super) effects: EffectDraft,
    pub(super) view: View,
    pub(super) window: Option<Box<WindowPublication>>,
}

pub(super) trait ErasedComponent {
    fn apply_input(
        &mut self,
        input: &dyn Any,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Option<ComponentRender>;
    fn component_type(&self) -> TypeId;
    fn cancel_tasks(&self);
    fn dispatch(
        &mut self,
        message: Box<dyn Any>,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> ComponentRender;
    fn render_view(
        &mut self,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> ComponentRender;
    fn cached_view(&self) -> View;
    #[cfg(any(test, feature = "test"))]
    fn sender(&self) -> &dyn Any;
    #[cfg(any(test, feature = "test"))]
    fn tracked_tasks(&self) -> usize;
}

pub(super) struct TypedScope<C: Component> {
    pub(super) component: C,
    pub(super) input: C::Input,
    pub(super) view: Option<View>,
    pub(super) local_sender: LocalSender<C::Message>,
    pub(super) sender: ComponentSender<C::Message>,
    pub(super) services: Arc<dyn ComponentServices>,
    pub(super) tasks: Arc<Mutex<Vec<SyncWeak<TaskControl>>>>,
    pub(super) ui_services: Rc<dyn ComponentUiServices>,
}

impl<C: Component> TypedScope<C> {
    fn context(&self, reference: ElementRef) -> ComponentContext<C> {
        ComponentContext {
            reference,
            local_sender: self.local_sender.clone(),
            sender: self.sender.clone(),
            services: Arc::clone(&self.services),
            tasks: Arc::clone(&self.tasks),
            ui_services: Rc::clone(&self.ui_services),
        }
    }

    fn render_view(
        &mut self,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> ComponentRender {
        let mut context: ViewContext<'_, C> = ViewContext {
            context: self.context(reference),
            contexts,
            dependencies: HashSet::new(),
            effects: EffectDraft::default(),
            window: WindowPublication::default(),
        };
        let view = self.component.view(&self.input, &mut context);
        self.view = Some(view.clone());
        ComponentRender {
            dependencies: context.dependencies,
            effects: context.effects,
            view,
            window: (context.window != WindowPublication::default())
                .then(|| Box::new(context.window)),
        }
    }
}

impl<C: Component> ErasedComponent for TypedScope<C> {
    fn apply_input(
        &mut self,
        input: &dyn Any,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Option<ComponentRender> {
        let input = input.downcast_ref::<C::Input>().unwrap();
        if self.input == *input {
            return None;
        }
        self.input = input.clone();
        let context = self.context(reference.clone());
        self.component.input_changed(&self.input, &context);
        Some(self.render_view(reference, contexts))
    }

    fn component_type(&self) -> TypeId {
        TypeId::of::<C>()
    }

    fn cancel_tasks(&self) {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.retain(|task| {
            if let Some(task) = task.upgrade() {
                task.cancel();
                false
            } else {
                false
            }
        });
    }

    fn dispatch(
        &mut self,
        message: Box<dyn Any>,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> ComponentRender {
        let context = self.context(reference.clone());
        self.component
            .update(*message.downcast::<C::Message>().unwrap(), &context);
        self.render_view(reference, contexts)
    }

    fn render_view(
        &mut self,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> ComponentRender {
        self.render_view(reference, contexts)
    }

    fn cached_view(&self) -> View {
        self.view.clone().unwrap()
    }

    #[cfg(any(test, feature = "test"))]
    fn sender(&self) -> &dyn Any {
        &self.sender
    }

    #[cfg(any(test, feature = "test"))]
    fn tracked_tasks(&self) -> usize {
        self.tasks
            .lock()
            .unwrap()
            .iter()
            .filter(|task| task.strong_count() != 0)
            .count()
    }
}

pub(super) struct Scope {
    pub(super) children: HashMap<Key, ComponentId>,
    pub(super) component: Box<dyn ErasedComponent>,
    pub(super) contexts: HashMap<ContextId, ContextValue>,
    pub(super) dependencies: HashSet<ContextId>,
    pub(super) effects: Vec<EffectSlot>,
    pub(super) key: Key,
    pub(super) parent: Option<ComponentId>,
    pub(super) provided_contexts: HashSet<ContextId>,
    pub(super) reference: ElementRef,
    pub(super) root: Option<ObjectId>,
    pub(super) window: Option<Box<WindowPublication>>,
}

impl Scope {
    pub(super) fn dependencies_changed(&self, contexts: &HashMap<ContextId, ContextValue>) -> bool {
        self.dependencies
            .iter()
            .any(|id| match (self.contexts.get(id), contexts.get(id)) {
                (None, None) => false,
                (Some(left), Some(right)) => {
                    !Rc::ptr_eq(&left.value, &right.value)
                        && !(left.equals)(left.value.as_ref(), right.value.as_ref())
                }
                _ => true,
            })
    }
}

pub(super) struct ScopeSlot {
    pub(super) generation: u32,
    pub(super) scope: Option<Scope>,
}

pub(super) struct PendingScopeRender {
    pub(super) dependencies: HashSet<ContextId>,
    pub(super) environment: Box<OwnedContextEnvironment>,
    pub(super) effects: EffectDraft,
    pub(super) id: ComponentId,
    pub(super) window: Option<Box<WindowPublication>>,
}

impl<A: Adapter> ComponentHost<A> {
    pub(super) fn owns_scope(&self, owner: ComponentId, mut child: ComponentId) -> bool {
        loop {
            if owner == child {
                return true;
            }
            let Some(parent) = self.scope(child).and_then(|scope| scope.parent) else {
                return false;
            };
            child = parent;
        }
    }

    pub(super) fn create_scope(
        &mut self,
        parent: Option<ComponentId>,
        node: ComponentNode,
        contexts: HashMap<ContextId, ContextValue>,
        provided_contexts: HashSet<ContextId>,
    ) -> (ComponentId, ComponentRender) {
        let reused = self.free_scopes.pop();
        let id = if let Some(index) = reused {
            ComponentId {
                index,
                generation: self.scopes[index as usize].generation,
            }
        } else {
            ComponentId {
                index: u32::try_from(self.scopes.len()).unwrap(),
                generation: 0,
            }
        };
        let reference = ElementRef::default();
        let key = node
            .key
            .clone()
            .expect("component scope identity must be resolved before creation");
        let (component, render) = node.factory.create(
            id,
            ComponentQueues {
                shared: Arc::clone(&self.queue),
                local: Rc::clone(&self.local_queue),
            },
            reference.clone(),
            Arc::clone(&self.services),
            Rc::clone(&self.ui_services),
            &contexts,
        );
        let scope = Scope {
            children: HashMap::new(),
            component,
            contexts,
            dependencies: HashSet::new(),
            effects: Vec::new(),
            key,
            parent,
            provided_contexts,
            reference,
            root: None,
            window: None,
        };
        if id.index as usize == self.scopes.len() {
            self.scopes.push(ScopeSlot {
                generation: id.generation,
                scope: Some(scope),
            });
        } else {
            self.scopes[id.index as usize].scope = Some(scope);
        }
        (id, render)
    }

    pub(super) fn apply_render(&mut self, id: ComponentId, render: ComponentRender) -> UpdateStats {
        let contexts = self.scope(id).unwrap().contexts.clone();
        let provided_contexts = self.scope(id).unwrap().provided_contexts.clone();
        self.apply_render_with_contexts(id, render, contexts, provided_contexts)
    }

    pub(super) fn apply_render_with_contexts(
        &mut self,
        id: ComponentId,
        render: ComponentRender,
        contexts: HashMap<ContextId, ContextValue>,
        provided_contexts: HashSet<ContextId>,
    ) -> UpdateStats {
        let root = self.scope(id).unwrap().root.unwrap();
        let mut expansion = ExpansionState::default();
        let view = expansion.push_render(
            id,
            render,
            OwnedContextEnvironment {
                contexts: contexts.clone(),
                provided: provided_contexts.clone(),
            },
        );
        let view = self.expand_view(
            id,
            view,
            &ContextEnvironment {
                contexts: &contexts,
                provided: &provided_contexts,
            },
            0,
            &mut expansion,
        );
        let retired = self.unseen_scopes(&expansion.seen);
        let publication = self.resolve_window_publication(&expansion.pending, &retired);
        let mut pending = Some(expansion.pending);
        let mut prepared = Vec::new();
        let (runtime, scopes) = (&mut self.runtime, &mut self.scopes);
        let mutations = runtime.update_subtree_before_apply(root, view.clone(), || {
            for pending in pending.take().unwrap() {
                let slots = &mut scopes[pending.id.index as usize]
                    .scope
                    .as_mut()
                    .unwrap()
                    .effects;
                prepared.push((
                    pending.id,
                    pending.effects.prepare(slots),
                    pending.environment,
                    pending.dependencies,
                    pending.window,
                ));
            }
            prepare_scope_retirement(scopes, &retired);
        });
        self.refresh_roots_from(view.0.as_object(), root);
        self.retire_unseen(&expansion.seen);
        for (id, prepared, environment, dependencies, window) in prepared {
            let environment = *environment;
            prepared.commit(&mut self.scope_mut(id).unwrap().effects);
            self.scope_mut(id).unwrap().contexts = environment.contexts;
            self.replace_dependencies(id, dependencies);
            self.scope_mut(id).unwrap().provided_contexts = environment.provided;
            self.scope_mut(id).unwrap().window = window;
        }
        self.sweep_virtual_rows();
        self.publish_committed_window(publication);
        mutations
    }

    pub(super) fn retire_unseen(&mut self, seen: &HashMap<ComponentId, HashSet<Key>>) {
        let mut retired = Vec::new();
        for (parent, keys) in seen {
            let scope = self.scope_mut(*parent).unwrap();
            let removed = scope
                .children
                .iter()
                .filter(|(key, _)| !keys.contains(*key))
                .map(|(key, id)| (key.clone(), *id))
                .collect::<Vec<_>>();
            for (key, id) in removed {
                scope.children.remove(&key);
                retired.push(id);
            }
        }
        for id in retired {
            self.retire_scope(id);
        }
    }

    pub(super) fn retire_scope(&mut self, id: ComponentId) {
        let children = self
            .scope(id)
            .map(|scope| scope.children.values().copied().collect::<Vec<_>>())
            .unwrap_or_default();
        for child in children {
            self.retire_scope(child);
        }
        for consumers in self.context_consumers.values_mut() {
            consumers.remove(&id);
        }
        let slot = &mut self.scopes[id.index as usize];
        let Some(mut scope) = slot.scope.take() else {
            return;
        };
        scope.reference.set(None);
        scope.component.cancel_tasks();
        cleanup_effects(&mut scope.effects);
        slot.generation = slot.generation.wrapping_add(1);
        self.free_scopes.push(id.index);
    }

    pub(super) fn unseen_scopes(
        &self,
        seen: &HashMap<ComponentId, HashSet<Key>>,
    ) -> Vec<ComponentId> {
        seen.iter()
            .flat_map(|(parent, keys)| {
                self.scope(*parent)
                    .into_iter()
                    .flat_map(|scope| scope.children.iter())
                    .filter(|(key, _)| !keys.contains(*key))
                    .map(|(_, id)| *id)
            })
            .collect()
    }

    pub(super) fn scope(&self, id: ComponentId) -> Option<&Scope> {
        let slot = self.scopes.get(id.index as usize)?;
        (slot.generation == id.generation)
            .then_some(slot.scope.as_ref())
            .flatten()
    }

    pub(super) fn scope_mut(&mut self, id: ComponentId) -> Option<&mut Scope> {
        let slot = self.scopes.get_mut(id.index as usize)?;
        (slot.generation == id.generation)
            .then_some(slot.scope.as_mut())
            .flatten()
    }
}

pub(super) fn prepare_scope_retirement(scopes: &mut [ScopeSlot], roots: &[ComponentId]) {
    let mut pending = roots.to_vec();
    while let Some(id) = pending.pop() {
        let Some(slot) = scopes.get_mut(id.index as usize) else {
            continue;
        };
        if slot.generation != id.generation {
            continue;
        }
        let Some(scope) = slot.scope.as_mut() else {
            continue;
        };
        pending.extend(scope.children.values().copied());
        scope.component.cancel_tasks();
        cleanup_effects(&mut scope.effects);
    }
}
