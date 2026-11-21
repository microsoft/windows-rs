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
        contexts: &ContextEnvironment,
    ) -> Option<ComponentRender>;
    fn component_type(&self) -> TypeId;
    fn cancel_tasks(&self);
    fn dispatch(
        &mut self,
        message: Box<dyn Any>,
        reference: ElementRef,
        contexts: &ContextEnvironment,
    ) -> ComponentRender;
    fn render_view(
        &mut self,
        reference: ElementRef,
        contexts: &ContextEnvironment,
    ) -> ComponentRender;
    fn cached_view(&self) -> View;
}

pub(super) struct TypedScope<C: Component> {
    pub(super) component: C,
    pub(super) input: C::Input,
    pub(super) view: Option<View>,
    pub(super) local_sender: LocalSender<C::Message>,
    pub(super) services: Arc<ComponentServices>,
    pub(super) tasks: Rc<RefCell<Vec<SyncWeak<TaskControl>>>>,
    pub(super) ui_services: Rc<WindowServices>,
}

impl<C: Component> TypedScope<C> {
    fn context<'a>(&self, input: &'a C::Input, reference: ElementRef) -> ComponentContext<'a, C> {
        ComponentContext {
            input,
            reference,
            local_sender: self.local_sender.clone(),
            services: Arc::clone(&self.services),
            tasks: Rc::clone(&self.tasks),
            ui_services: Rc::clone(&self.ui_services),
        }
    }

    fn render_view(
        &mut self,
        reference: ElementRef,
        contexts: &ContextEnvironment,
    ) -> ComponentRender {
        let mut context: ViewContext<C> = ViewContext {
            reference,
            local_sender: self.local_sender.clone(),
            contexts: Rc::clone(contexts),
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
        contexts: &ContextEnvironment,
    ) -> Option<ComponentRender> {
        let input = input.downcast_ref::<C::Input>().unwrap();
        if self.input == *input {
            return None;
        }
        self.input = input.clone();
        let context = self.context(&self.input, reference.clone());
        self.component.input_changed(&self.input, &context);
        Some(self.render_view(reference, contexts))
    }

    fn component_type(&self) -> TypeId {
        TypeId::of::<C>()
    }

    fn cancel_tasks(&self) {
        for task in self.tasks.borrow_mut().drain(..) {
            if let Some(task) = task.upgrade() {
                task.cancel();
            }
        }
    }

    fn dispatch(
        &mut self,
        message: Box<dyn Any>,
        reference: ElementRef,
        contexts: &ContextEnvironment,
    ) -> ComponentRender {
        let context = self.context(&self.input, reference.clone());
        self.component
            .update(*message.downcast::<C::Message>().unwrap(), &context);
        self.render_view(reference, contexts)
    }

    fn render_view(
        &mut self,
        reference: ElementRef,
        contexts: &ContextEnvironment,
    ) -> ComponentRender {
        self.render_view(reference, contexts)
    }

    fn cached_view(&self) -> View {
        self.view.clone().unwrap()
    }
}

pub(super) struct Scope {
    pub(super) children: HashMap<Key, ComponentId>,
    pub(super) component: Box<dyn ErasedComponent>,
    pub(super) contexts: ContextEnvironment,
    pub(super) dependencies: HashSet<ContextId>,
    pub(super) effects: Vec<EffectSlot>,
    pub(super) key: Key,
    pub(super) parent: Option<ComponentId>,
    pub(super) reference: ElementRef,
    pub(super) root: Option<ObjectId>,
    // The child component whose root is also this component's root.
    pub(super) root_child: Option<ComponentId>,
    pub(super) window: Option<Box<WindowPublication>>,
}

impl Scope {
    pub(super) fn dependencies_changed(&self, contexts: &ContextEnvironment) -> bool {
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
    pub(super) environment: ContextEnvironment,
    pub(super) effects: EffectDraft,
    pub(super) id: ComponentId,
    pub(super) window: Option<Box<WindowPublication>>,
}

impl ComponentHost {
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
        contexts: ContextEnvironment,
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
            reference,
            root: None,
            root_child: None,
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

    /// Plans a component render and completes its structural host work. Effect setup waits for
    /// `complete_batch` so it runs after the native application.
    pub(super) fn plan_render(&mut self, id: ComponentId, render: ComponentRender) -> NativeBatch {
        let contexts = Rc::clone(&self.scope(id).unwrap().contexts);
        self.begin_batch();
        self.mark_busy_ancestors(id);
        let root = self.scope(id).unwrap().root.unwrap();
        let mut expansion = ExpansionState::default();
        let view = expansion.push_render(id, render, Rc::clone(&contexts));
        let view = self.expand_view(id, view, &contexts, 0, &mut expansion);
        let retired = self.unseen_scopes(&expansion.seen);
        let publication = self.resolve_window_publication(&expansion.pending, &retired);
        let batch = self.runtime.plan_subtree_update(root, view.clone());
        self.prepare_pending(expansion.pending);
        self.prepare_scope_retirement(&retired);
        self.refresh_roots_from(view.0.as_object(), root);
        self.retire_unseen(&expansion.seen);
        self.sweep_virtual_rows();
        self.publish_committed_window(publication);
        batch
    }

    /// Prepares the effects of rendered scopes, running the cleanups of changed and removed
    /// effects, and commits the rest of each scope's render state.
    pub(super) fn prepare_pending(&mut self, pending: Vec<PendingScopeRender>) {
        for pending in pending {
            let scope = self.scope_mut(pending.id).unwrap();
            let effects = pending.effects.prepare(&mut scope.effects);
            scope.contexts = pending.environment;
            scope.window = pending.window;
            scope.dependencies = pending.dependencies;
            self.prepared_effects.push((pending.id, effects));
        }
    }

    pub(super) fn retire_unseen(&mut self, seen: &HashMap<ComponentId, HashSet<Key>>) {
        let mut retired = Vec::new();
        for (parent, keys) in seen {
            let scope = self.scope_mut(*parent).unwrap();
            scope.children.retain(|key, id| {
                if keys.contains(key) {
                    true
                } else {
                    retired.push(*id);
                    false
                }
            });
        }
        for id in retired {
            self.retire_scope(id);
        }
    }

    pub(super) fn retire_scope(&mut self, id: ComponentId) {
        if let Some(scope) = self.scope_mut(id)
            && !scope.children.is_empty()
        {
            let children = std::mem::take(&mut scope.children);
            for &child in children.values() {
                self.retire_scope(child);
            }
            // Drop child keys with the parent scope, before its component.
            self.scope_mut(id).unwrap().children = children;
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
        self.discard_prepared_effects(id);
    }

    /// Cancels the tasks and runs the effect cleanups of scopes that the batch being planned
    /// retires, so cleanups observe roots, references, and native objects before the batch
    /// applies. This includes retained effects that an in-flight batch prepared.
    pub(super) fn prepare_scope_retirement(&mut self, roots: &[ComponentId]) {
        let mut pending = roots.to_vec();
        while let Some(id) = pending.pop() {
            let Some(scope) = self.scope_mut(id) else {
                continue;
            };
            pending.extend(scope.children.values().copied());
            scope.component.cancel_tasks();
            cleanup_effects(&mut scope.effects);
            self.discard_prepared_effects(id);
        }
    }

    /// Runs the retained cleanups of a retiring scope whose effects an in-flight batch prepared.
    fn discard_prepared_effects(&mut self, id: ComponentId) {
        for index in (0..self.prepared_effects.len()).rev() {
            if self.prepared_effects[index].0 == id {
                self.prepared_effects.remove(index).1.discard();
            }
        }
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
