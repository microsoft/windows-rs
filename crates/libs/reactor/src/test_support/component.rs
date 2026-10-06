use super::*;
use std::sync::Condvar;

impl ComponentTask {
    pub fn status(&self) -> ComponentTaskStatus {
        self.control.status()
    }
}

impl ComponentTimer {
    pub fn status(&self) -> ComponentTaskStatus {
        self.task.status()
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ComponentHostState {
    pub live_scopes: usize,
    pub scope_slots: usize,
    pub free_scopes: usize,
    pub effects: usize,
    pub tasks: usize,
    pub contexts: usize,
    pub context_consumers: usize,
    pub virtual_rows: usize,
    pub queued_messages: usize,
    pub queue_closed: bool,
    pub graph_objects: usize,
    pub retirements: usize,
}

impl<A: Adapter> ComponentHost<A> {
    pub fn test_adapter_mut(&mut self) -> &mut A {
        self.runtime.adapter_mut()
    }

    pub fn test_state(&self) -> ComponentHostState {
        let queue = self.queue.lock().unwrap();
        ComponentHostState {
            live_scopes: self
                .scopes
                .iter()
                .filter(|slot| slot.scope.is_some())
                .count(),
            scope_slots: self.scopes.len(),
            free_scopes: self.free_scopes.len(),
            effects: self
                .scopes
                .iter()
                .filter_map(|slot| slot.scope.as_ref())
                .map(|scope| scope.effects.len())
                .sum(),
            tasks: self
                .scopes
                .iter()
                .filter_map(|slot| slot.scope.as_ref())
                .map(|scope| scope.component.tracked_tasks())
                .sum(),
            contexts: self.contexts.len(),
            context_consumers: self.context_consumers.values().map(HashSet::len).sum(),
            virtual_rows: self.virtual_rows.len(),
            queued_messages: queue.messages.len() + self.local_queue.borrow().len(),
            queue_closed: queue.closed,
            graph_objects: self.runtime.graph().object_count(),
            retirements: self.runtime.graph().retired_count(),
        }
    }
}

#[derive(Default)]
struct DefaultComponentUiServices;

impl ComponentUiServices for DefaultComponentUiServices {
    fn open_window(&self, _root: ComponentNode, _policy: WindowPolicy) -> bool {
        false
    }
}

#[derive(Default)]
pub struct DefaultComponentServices;

impl ComponentServices for DefaultComponentServices {
    fn spawn_background(&self, work: Box<dyn FnOnce() + Send>) {
        windows_threading::submit(work);
    }

    fn set_timeout(
        &self,
        delay: Duration,
        callback: Box<dyn FnOnce() + Send>,
    ) -> Arc<dyn ComponentTimerRegistration> {
        let timer = Arc::new(ThreadPoolTimer::default());
        let thread_timer = Arc::clone(&timer);
        windows_threading::submit(move || {
            let wait = thread_timer.wait.lock().unwrap();
            let _ = thread_timer
                .changed
                .wait_timeout_while(wait, delay, |_| {
                    !thread_timer.cancelled.load(Ordering::Acquire)
                })
                .unwrap();
            if !thread_timer.cancelled.load(Ordering::Acquire) {
                callback();
            }
        });
        timer
    }
}

#[derive(Default)]
struct ThreadPoolTimer {
    cancelled: AtomicBool,
    changed: Condvar,
    wait: Mutex<()>,
}

impl ComponentTimerRegistration for ThreadPoolTimer {
    fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
        self.changed.notify_all();
    }
}

impl<A: Adapter> ComponentHost<A> {
    fn scope_depth(&self, mut id: ComponentId) -> usize {
        let mut depth = 0;
        while let Some(parent) = self.scope(id).and_then(|scope| scope.parent) {
            depth += 1;
            id = parent;
        }
        depth
    }

    pub fn mount(
        adapter: A,
        components: impl IntoIterator<Item = ComponentNode>,
    ) -> Result<Self, ComponentError<A::Error>> {
        Self::mount_with_all_services(
            adapter,
            Arc::new(DefaultComponentServices),
            Rc::new(DefaultComponentUiServices),
            components,
        )
    }

    pub fn mount_with_services(
        adapter: A,
        services: Arc<dyn ComponentServices>,
        components: impl IntoIterator<Item = ComponentNode>,
    ) -> Result<Self, ComponentError<A::Error>> {
        Self::mount_with_all_services(
            adapter,
            services,
            Rc::new(DefaultComponentUiServices),
            components,
        )
    }

    pub fn adapter(&self) -> &A {
        self.runtime.adapter()
    }

    pub fn focus(&mut self, reference: &ElementRef) -> Result<bool, ComponentError<A::Error>> {
        self.ensure_active()?;
        self.runtime
            .focus(reference)
            .map_err(|error| self.runtime_error(error))
    }

    pub fn sender<C: Component>(&self, key: &Key) -> Option<ComponentSender<C::Message>>
    where
        C::Message: Send,
    {
        self.sender_at::<C>(std::slice::from_ref(key))
    }

    pub fn sender_at<C: Component>(&self, path: &[Key]) -> Option<ComponentSender<C::Message>>
    where
        C::Message: Send,
    {
        if self.poisoned {
            return None;
        }
        let scope = self.scope(self.find_path(path)?)?;
        (scope.component.component_type() == TypeId::of::<C>()).then(|| {
            scope
                .component
                .sender()
                .downcast_ref::<ComponentSender<C::Message>>()
                .unwrap()
                .clone()
        })
    }

    pub fn reference(&self, key: &Key) -> Option<ElementRef> {
        self.reference_at(std::slice::from_ref(key))
    }

    pub fn reference_at(&self, path: &[Key]) -> Option<ElementRef> {
        if self.poisoned {
            return None;
        }
        self.scope(self.find_path(path)?)
            .map(|scope| scope.reference.clone())
    }

    pub fn update_input<C: Component>(
        &mut self,
        key: &Key,
        input: C::Input,
    ) -> Result<UpdateStats, ComponentError<A::Error>> {
        self.update_input_at::<C>(std::slice::from_ref(key), input)
    }

    pub fn update_input_at<C: Component>(
        &mut self,
        path: &[Key],
        input: C::Input,
    ) -> Result<UpdateStats, ComponentError<A::Error>> {
        self.ensure_active()?;
        self.prepare_operation()?;
        let key = path.last().cloned().unwrap_or_else(|| Key::from(""));
        let id = self
            .find_path(path)
            .ok_or_else(|| ComponentError::MissingComponent(key.clone()))?;
        if self.scope(id).unwrap().component.component_type() != TypeId::of::<C>() {
            return Err(ComponentError::ComponentType(key));
        }
        self.run_component_operation(move |host| {
            let contexts = host.scope(id).unwrap().contexts.clone();
            let scope = host.scope_mut(id).unwrap();
            let reference = scope.reference.clone();
            let Some(render) = scope
                .component
                .apply_input(&input, reference, &contexts)
                .map_err(ComponentError::from)?
            else {
                return Ok(UpdateStats::default());
            };
            host.apply_render(id, render)
        })
    }

    pub fn set_context<T: Clone + PartialEq + 'static>(
        &mut self,
        context: &Context<T>,
        value: T,
    ) -> Result<ComponentDrain, ComponentError<A::Error>> {
        self.ensure_active()?;
        self.prepare_operation()?;
        if self
            .contexts
            .get(&context.id)
            .is_some_and(|current| (current.equals)(current.value.as_ref(), &value))
        {
            return Ok(ComponentDrain::default());
        }
        self.run_component_operation(move |host| {
            let mut contexts = host.contexts.clone();
            let value = ContextValue {
                equals: |left, right| {
                    left.downcast_ref::<T>()
                        .zip(right.downcast_ref::<T>())
                        .is_some_and(|(left, right)| left == right)
                },
                value: Rc::new(value),
            };
            contexts.insert(context.id, value.clone());
            host.contexts = contexts;
            let mut affected = host
                .context_consumers
                .get(&context.id)
                .map(|consumers| {
                    consumers
                        .iter()
                        .copied()
                        .filter(|id| {
                            host.scope(*id)
                                .is_some_and(|scope| !scope.provided_contexts.contains(&context.id))
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            affected.sort_unstable_by_key(|id| host.scope_depth(*id));
            let mut report = ComponentDrain::default();
            for id in affected {
                let Some(scope) = host.scope(id) else {
                    continue;
                };
                if scope.provided_contexts.contains(&context.id) {
                    continue;
                }
                let reference = scope.reference.clone();
                let mut contexts = scope.contexts.clone();
                contexts.insert(context.id, value.clone());
                if !scope.dependencies_changed(&contexts) {
                    continue;
                }
                let provided_contexts = scope.provided_contexts.clone();
                let scope = host.scope_mut(id).unwrap();
                let render = scope
                    .component
                    .render_view(reference, &contexts)
                    .map_err(ComponentError::from)?;
                let mutations =
                    host.apply_render_with_contexts(id, render, contexts, provided_contexts)?;
                report.dispatched += 1;
                report.mutations += mutations.len();
            }
            for slot in &mut host.scopes {
                if let Some(scope) = slot.scope.as_mut()
                    && !scope.provided_contexts.contains(&context.id)
                {
                    scope.contexts.insert(context.id, value.clone());
                }
            }
            Ok(report)
        })
    }

    pub fn remove(&mut self, key: &Key) -> Result<(), ComponentError<A::Error>> {
        self.ensure_active()?;
        self.prepare_operation()?;
        let id = self
            .find(key)
            .ok_or_else(|| ComponentError::MissingComponent(key.clone()))?;
        let parent = self.runtime.graph().root().unwrap();
        let child = self.scope(id).unwrap().root.unwrap();
        if let Err(error) = self
            .runtime
            .remove_child(parent, RelationId::Children, child)
        {
            return Err(self.runtime_error(error));
        }
        self.run_component_operation(|host| {
            host.order.retain(|current| *current != id);
            host.keys.remove(key);
            host.retire_scope(id);
            host.context_consumers
                .retain(|_, consumers| !consumers.is_empty());
            host.publish_window();
            Ok(())
        })
    }

    fn find(&self, key: &Key) -> Option<ComponentId> {
        self.keys.get(key).copied()
    }

    fn find_path(&self, path: &[Key]) -> Option<ComponentId> {
        let (first, rest) = path.split_first()?;
        let mut id = self.find(first)?;
        for key in rest {
            let parent = id;
            id = *self.scope(parent)?.children.get(key)?;
            debug_assert_eq!(self.scope(id)?.parent, Some(parent));
        }
        Some(id)
    }

    fn prepare_operation(&mut self) -> Result<(), ComponentError<A::Error>> {
        match self.runtime.prepare_update() {
            Ok(()) => Ok(()),
            Err(UpdateError::PendingNativeEvent) => {
                self.drain(usize::MAX)?;
                self.runtime
                    .prepare_update()
                    .map_err(|error| self.runtime_error(error))
            }
            Err(error) => Err(self.runtime_error(error)),
        }
    }
}
