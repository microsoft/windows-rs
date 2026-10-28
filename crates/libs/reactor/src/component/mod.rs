use super::*;

mod context;
mod effects;
mod expand;
mod messaging;
mod provider;
mod scope;
mod tasks;
mod virtualization;
mod window;

pub use context::*;
pub use effects::*;
pub use messaging::*;
pub use provider::*;
pub use tasks::*;
pub use window::*;

use expand::*;
use scope::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ComponentId {
    index: u32,
    generation: u32,
}

pub trait Component: Sized + 'static {
    type Input: Clone + PartialEq + 'static;
    type Message: 'static;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self;
    fn input_changed(&mut self, _input: &Self::Input, _context: &ComponentContext<Self>) {}
    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}
    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View;
}

pub struct ComponentNode {
    pub(crate) key: Option<Key>,
    factory: Rc<dyn ErasedFactory>,
}

pub fn component<C: Component>(key: impl Into<Key>, input: C::Input) -> ComponentNode {
    ComponentNode {
        key: Some(key.into()),
        factory: Rc::new(TypedFactory::<C> { input }),
    }
}

impl View {
    pub fn component<C: Component>(input: C::Input) -> Self {
        ComponentNode {
            key: None,
            factory: Rc::new(TypedFactory::<C> { input }),
        }
        .into()
    }
}

impl ComponentNode {
    pub fn keyed(self) -> KeyedView {
        let key = self
            .key
            .clone()
            .expect("only explicitly keyed components can be converted directly to KeyedView");
        keyed(key, self)
    }
}

trait ErasedFactory {
    fn component_type(&self) -> TypeId;
    fn create(
        &self,
        id: ComponentId,
        queues: ComponentQueues,
        reference: ElementRef,
        services: Arc<dyn ComponentServices>,
        ui_services: Rc<dyn ComponentUiServices>,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> (Box<dyn ErasedComponent>, ComponentRender);
    fn input(&self) -> &dyn Any;
}

struct TypedFactory<C: Component> {
    input: C::Input,
}

impl<C: Component> ErasedFactory for TypedFactory<C> {
    fn component_type(&self) -> TypeId {
        TypeId::of::<C>()
    }

    fn create(
        &self,
        id: ComponentId,
        queues: ComponentQueues,
        reference: ElementRef,
        services: Arc<dyn ComponentServices>,
        ui_services: Rc<dyn ComponentUiServices>,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> (Box<dyn ErasedComponent>, ComponentRender) {
        let sender = ComponentSender {
            component: id,
            queue: queues.shared,
            marker: PhantomData,
        };
        let local_sender = LocalSender {
            component: id,
            queue: queues.local,
            wake: Arc::clone(&sender.queue),
            marker: PhantomData,
        };
        let tasks = Arc::new(Mutex::new(Vec::new()));
        let context: ComponentContext<C> = ComponentContext {
            reference: reference.clone(),
            local_sender: local_sender.clone(),
            sender: sender.clone(),
            services: Arc::clone(&services),
            tasks: Arc::clone(&tasks),
            ui_services: Rc::clone(&ui_services),
        };
        let component = C::create(&self.input, &context);
        let mut scope = TypedScope {
            component,
            input: self.input.clone(),
            view: None,
            local_sender,
            sender,
            services,
            tasks,
            ui_services,
        };
        let render = scope.render_view(reference, contexts);
        (Box::new(scope), render)
    }

    fn input(&self) -> &dyn Any {
        &self.input
    }
}

impl Clone for ComponentNode {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            factory: Rc::clone(&self.factory),
        }
    }
}

impl PartialEq for ComponentNode {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && Rc::ptr_eq(&self.factory, &other.factory)
    }
}

impl From<ComponentNode> for View {
    fn from(node: ComponentNode) -> Self {
        Self(DeclaredNode::Component {
            node,
            relation_key: None,
            attachments: None,
        })
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ComponentDrain {
    pub dispatched: usize,
    pub dropped: usize,
    pub mutations: usize,
}

/// Coordinates component scopes with retained runtime updates.
///
/// The adapter is exposed read-only. Native protocol methods cannot be called through the host.
///
/// ```compile_fail
/// use windows_reactor::{Adapter, ComponentHost};
///
/// fn pop<A: Adapter>(host: &ComponentHost<A>) {
///     host.adapter().pop_native_event();
/// }
/// ```
///
/// ```compile_fail
/// use windows_reactor::{Adapter, ComponentHost};
///
/// fn apply<A: Adapter>(host: &mut ComponentHost<A>) {
///     let _ = host.adapter_mut().apply(&[]);
/// }
/// ```
pub struct ComponentHost<A: Adapter> {
    context_consumers: HashMap<ContextId, HashSet<ComponentId>>,
    contexts: HashMap<ContextId, ContextValue>,
    keys: HashMap<Key, ComponentId>,
    order: Vec<ComponentId>,
    queue: SharedQueue,
    local_queue: LocalQueue,
    continuation_waker: Option<Rc<dyn Fn()>>,
    runtime: Runtime<A>,
    services: Arc<dyn ComponentServices>,
    ui_services: Rc<dyn ComponentUiServices>,
    window_color_scheme: WindowObserverLifetime<ColorScheme>,
    window_size: WindowObserverLifetime<WindowSize>,
    window_placement: WindowObserverLifetime<WindowPlacement>,
    free_scopes: Vec<u32>,
    scopes: Vec<ScopeSlot>,
    virtual_rows: HashMap<(ObjectId, Key), ComponentId>,
}

impl<A: Adapter> Drop for ComponentHost<A> {
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl<A: Adapter> ComponentHost<A> {
    pub(crate) fn update_root(&mut self, node: ComponentNode) {
        let key = node.key.as_ref().unwrap();
        let id = *self.keys.get(key).unwrap();
        let scope = self.scope(id).unwrap();
        assert!(
            scope.component.component_type() == node.factory.component_type(),
            "component {key:?} changed type"
        );
        let retired = {
            let mut queue = self.queue.lock().unwrap();
            let mut local = self.local_queue.borrow_mut();
            let retired = local
                .iter()
                .position(|message| {
                    message.component == id && matches!(message.value, LocalMessageValue::Input(_))
                })
                .and_then(|index| local.remove(index));
            let sequence = queue.next_sequence;
            queue.next_sequence = sequence.wrapping_add(1);
            local.push_back(LocalMessage {
                component: id,
                sequence,
                value: LocalMessageValue::Input(node),
            });
            retired
        };
        drop(retired);
        self.rearm_wake();
    }

    pub(crate) fn has_pending_input(&self) -> bool {
        self.local_queue
            .borrow()
            .iter()
            .any(|message| matches!(message.value, LocalMessageValue::Input(_)))
    }

    pub(crate) fn mount_with_all_services(
        adapter: A,
        services: Arc<dyn ComponentServices>,
        ui_services: Rc<dyn ComponentUiServices>,
        components: impl IntoIterator<Item = ComponentNode>,
    ) -> Self {
        let queue = Arc::new(Mutex::new(MessageQueue::default()));
        let local_queue = Rc::new(RefCell::new(VecDeque::new()));
        let mut host = Self {
            context_consumers: HashMap::new(),
            contexts: HashMap::new(),
            keys: HashMap::new(),
            order: Vec::new(),
            queue,
            local_queue,
            continuation_waker: None,
            runtime: Runtime::new(adapter),
            services,
            ui_services,
            window_color_scheme: WindowObserverLifetime::default(),
            window_size: WindowObserverLifetime::default(),
            window_placement: WindowObserverLifetime::default(),
            free_scopes: Vec::new(),
            scopes: Vec::new(),
            virtual_rows: HashMap::new(),
        };
        let mut declarations = Vec::new();
        let mut keys = HashSet::new();
        let mut expansion = ExpansionState::default();
        for node in components {
            let key = node
                .key
                .clone()
                .expect("component roots require explicit keys");
            assert!(keys.insert(key.clone()), "duplicate key {key:?}");
            let contexts = host.contexts.clone();
            let (id, render) = host.create_scope(None, node, contexts.clone(), HashSet::new());
            host.order.push(id);
            host.keys.insert(key, id);
            let view = expansion.push_render(
                id,
                render,
                OwnedContextEnvironment {
                    contexts: contexts.clone(),
                    provided: HashSet::new(),
                },
            );
            let provided = HashSet::new();
            let view = host.expand_view(
                id,
                view,
                &ContextEnvironment {
                    contexts: &contexts,
                    provided: &provided,
                },
                0,
                &mut expansion,
            );
            declarations.push(keyed(host.scope(id).unwrap().key.clone(), view));
        }
        let root: View = Grid::new().keyed_children(declarations).into();
        let publication = host.resolve_window_publication(&expansion.pending, &[]);
        host.runtime.update(root.clone());
        host.refresh_roots(&root);
        for pending in expansion.pending {
            let environment = *pending.environment;
            host.scope_mut(pending.id).unwrap().window = pending.window;
            host.scope_mut(pending.id).unwrap().contexts = environment.contexts;
            host.scope_mut(pending.id).unwrap().provided_contexts = environment.provided;
            pending
                .effects
                .commit(&mut host.scope_mut(pending.id).unwrap().effects);
            host.replace_dependencies(pending.id, pending.dependencies);
        }
        host.publish_committed_window(publication);
        host
    }

    pub fn runtime(&self) -> &Runtime<A> {
        &self.runtime
    }

    pub(crate) fn runtime_mut_internal(&mut self) -> &mut Runtime<A> {
        &mut self.runtime
    }

    pub fn set_waker(&mut self, waker: impl Fn() + Send + Sync + 'static) {
        let mut queue = self.queue.lock().unwrap();
        if queue.closed {
            return;
        }
        let waker = Arc::new(waker);
        queue.waker = Some(waker.clone());
        let wake = (!queue.messages.is_empty() || !self.local_queue.borrow().is_empty())
            && !queue.wake_pending;
        queue.wake_pending |= wake;
        drop(queue);
        if wake {
            waker();
        }
    }

    pub(crate) fn set_continuation_waker(&mut self, waker: impl Fn() + 'static) {
        self.continuation_waker = Some(Rc::new(waker));
    }

    pub fn drain(&mut self, limit: usize) -> ComponentDrain {
        let mut report = ComponentDrain::default();
        self.queue.lock().unwrap().wake_pending = false;
        let mut processed = 0;
        let mut active_event = None;
        loop {
            if active_event.is_none() && processed >= limit {
                if let Some(waker) = self.continuation_waker.clone() {
                    waker();
                }
                break;
            }
            let native_pending = active_event.is_none() && !self.runtime.prepare_update();
            let message = (!native_pending)
                .then(|| {
                    let mut queue = self.queue.lock().unwrap();
                    let mut local_queue = self.local_queue.borrow_mut();
                    let take_local = match (local_queue.front(), queue.messages.front()) {
                        (Some(local), Some(shared)) => local.sequence < shared.sequence,
                        (Some(_), None) => true,
                        _ => false,
                    };
                    if take_local {
                        local_queue
                            .pop_front()
                            .map(|message| (message.component, None, message.value))
                    } else {
                        queue.messages.pop_front().map(|message| {
                            let value: Box<dyn Any> = match message.value {
                                MessageValue::Send(value) => value,
                                MessageValue::Local(factory) => factory(),
                            };
                            (
                                message.component,
                                message.control,
                                LocalMessageValue::Message(value),
                            )
                        })
                    }
                })
                .flatten();
            if let Some((component, control, value)) = message {
                if active_event.is_none() {
                    processed += 1;
                }
                if self.scope(component).is_none() {
                    if let Some(control) = control {
                        control.cancel();
                    }
                    report.dropped += 1;
                    continue;
                }
                if let Some(control) = &control
                    && !control.deliver()
                {
                    report.dropped += 1;
                    continue;
                }
                let contexts = self.scope(component).unwrap().contexts.clone();
                let scope = self.scope_mut(component).unwrap();
                let reference = scope.reference.clone();
                let render = match value {
                    LocalMessageValue::Message(value) => {
                        Some(scope.component.dispatch(value, reference, &contexts))
                    }
                    LocalMessageValue::Input(node) => {
                        scope
                            .component
                            .apply_input(node.factory.input(), reference, &contexts)
                    }
                };
                let mutations = render
                    .map(|render| self.apply_render(component, render))
                    .unwrap_or_default();
                report.dispatched += 1;
                report.mutations += mutations.len();
                continue;
            }
            if active_event.take().is_some() {
                continue;
            }
            match self.runtime.next_native_work() {
                Some(NativeWork::Event(mut event)) => {
                    processed += 1;
                    event.invoke();
                    active_event = Some(event);
                }
                Some(NativeWork::Virtual(work)) => {
                    processed += 1;
                    report.mutations += self.apply_virtual_work(work).len();
                }
                Some(NativeWork::Maintenance) => {
                    processed += 1;
                }
                None => break,
            }
        }
        self.rearm_wake();
        report
    }

    fn shutdown(&mut self) {
        self.window_color_scheme = WindowObserverLifetime::default();
        self.window_size = WindowObserverLifetime::default();
        self.window_placement = WindowObserverLifetime::default();
        self.context_consumers.clear();
        self.contexts.clear();
        self.keys.clear();
        self.order.clear();
        self.virtual_rows.clear();
        self.free_scopes.clear();
        self.local_queue.borrow_mut().clear();

        let controls = {
            let mut queue = self.queue.lock().unwrap_or_else(|error| error.into_inner());
            queue.closed = true;
            queue.wake_pending = false;
            queue.waker = None;
            queue
                .messages
                .drain(..)
                .filter_map(|message| message.control)
                .collect::<Vec<_>>()
        };
        let mut scopes = Vec::new();
        for (index, slot) in self.scopes.iter_mut().enumerate().rev() {
            let Some(mut scope) = slot.scope.take() else {
                continue;
            };
            scope.root = None;
            scope.reference.set(None);
            slot.generation = slot.generation.wrapping_add(1);
            if let Ok(index) = u32::try_from(index) {
                self.free_scopes.push(index);
            }
            scopes.push(scope);
        }

        for control in controls {
            control.cancel();
        }
        for mut scope in scopes {
            scope.component.cancel_tasks();
            for effect in scope.effects.iter_mut().rev() {
                if let Some(cleanup) = effect.cleanup.take() {
                    cleanup();
                }
            }
            drop(scope);
        }
    }

    fn rearm_wake(&self) {
        let mut queue = self.queue.lock().unwrap();
        let wake = (!queue.closed
            && (!queue.messages.is_empty() || !self.local_queue.borrow().is_empty())
            && !queue.wake_pending)
            .then(|| queue.waker.clone())
            .flatten();
        queue.wake_pending |= wake.is_some();
        drop(queue);
        if let Some(wake) = wake {
            wake();
        }
    }
}

#[cfg(test)]
#[path = "../tests/component/mod.rs"]
mod tests;

#[cfg(any(test, feature = "test"))]
#[path = "../test_support/component.rs"]
mod test_support;
#[cfg(test)]
pub use test_support::*;
