use super::*;
use std::any::{Any, TypeId};
use std::cell::Cell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::rc::Rc;
use std::sync::atomic::{AtomicU8, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, Weak};
use std::time::Duration;

const MESSAGE_CAPACITY: usize = 4_096;
static NEXT_CONTEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct ContextId(u64);

#[derive(Clone)]
pub struct Context<T> {
    default: T,
    id: ContextId,
}

impl<T> Context<T> {
    pub fn new(default: T) -> Self {
        Self {
            default,
            id: ContextId(NEXT_CONTEXT_ID.fetch_add(1, Ordering::Relaxed)),
        }
    }
}

#[derive(Clone)]
struct ContextValue {
    equals: fn(&dyn Any, &dyn Any) -> bool,
    value: Rc<dyn Any>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ComponentId {
    index: u32,
    generation: u32,
}

#[derive(Clone, Default)]
pub struct ElementRef(Rc<Cell<Option<ObjectId>>>);

impl ElementRef {
    pub fn get(&self) -> Option<ObjectId> {
        self.0.get()
    }

    fn set(&self, object: Option<ObjectId>) {
        self.0.set(object);
    }
}

struct Message {
    component: ComponentId,
    control: Option<Arc<TaskControl>>,
    value: Box<dyn Any + Send>,
}

#[derive(Default)]
struct MessageQueue {
    messages: VecDeque<Message>,
    wake_pending: bool,
    waker: Option<Arc<dyn Fn() + Send + Sync>>,
}

type SharedQueue = Arc<Mutex<MessageQueue>>;

pub struct ComponentSender<M> {
    component: ComponentId,
    queue: SharedQueue,
    marker: std::marker::PhantomData<fn(M)>,
}

impl<M> Clone for ComponentSender<M> {
    fn clone(&self) -> Self {
        Self {
            component: self.component,
            queue: Arc::clone(&self.queue),
            marker: std::marker::PhantomData,
        }
    }
}

impl<M: Send + 'static> ComponentSender<M> {
    #[must_use]
    pub fn send(&self, value: M) -> bool {
        self.enqueue(value, None)
    }

    pub fn completion(&self) -> ComponentCompletion<M> {
        ComponentCompletion(self.clone())
    }

    fn send_controlled(&self, value: M, control: Arc<TaskControl>) -> bool {
        self.enqueue(value, Some(control))
    }

    fn enqueue(&self, value: M, control: Option<Arc<TaskControl>>) -> bool {
        let mut queue = self.queue.lock().unwrap();
        if queue.messages.len() >= MESSAGE_CAPACITY {
            if let Some(control) = control {
                control.reject();
            }
            return false;
        }
        queue.messages.push_back(Message {
            component: self.component,
            control,
            value: Box::new(value),
        });
        let wake = (!queue.wake_pending).then(|| queue.waker.clone()).flatten();
        queue.wake_pending |= wake.is_some();
        drop(queue);
        if let Some(wake) = wake {
            wake();
        }
        true
    }
}

pub struct ComponentCompletion<M>(ComponentSender<M>);

impl<M> Clone for ComponentCompletion<M> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<M: Send + 'static> ComponentCompletion<M> {
    #[must_use]
    pub fn complete(self, value: M) -> bool {
        self.0.send(value)
    }
}

pub struct ComponentContext<M> {
    reference: ElementRef,
    sender: ComponentSender<M>,
    tasks: Arc<Mutex<Vec<Weak<TaskControl>>>>,
}

impl<M: Send + 'static> ComponentContext<M> {
    pub fn sender(&self) -> ComponentSender<M> {
        self.sender.clone()
    }

    pub fn completion(&self) -> ComponentCompletion<M> {
        self.sender.completion()
    }

    pub fn root(&self) -> ElementRef {
        self.reference.clone()
    }

    pub fn spawn_background(
        &self,
        work: impl FnOnce(CancellationToken) -> M + Send + 'static,
    ) -> ComponentTask {
        let control = Arc::new(TaskControl::default());
        let mut tasks = self.tasks.lock().unwrap();
        tasks.retain(|task| task.strong_count() != 0);
        tasks.push(Arc::downgrade(&control));
        drop(tasks);
        let sender = self.sender.clone();
        let thread_control = Arc::clone(&control);
        std::thread::spawn(move || {
            let message = work(CancellationToken {
                control: Arc::clone(&thread_control),
            });
            if thread_control.queue() {
                sender.send_controlled(message, Arc::clone(&thread_control));
            }
        });
        ComponentTask { control }
    }

    #[must_use = "dropping the timer cancels message delivery"]
    pub fn set_timeout(&self, delay: Duration, message: M) -> ComponentTimer {
        let control = Arc::new(TaskControl::default());
        let mut tasks = self.tasks.lock().unwrap();
        tasks.retain(|task| task.strong_count() != 0);
        tasks.push(Arc::downgrade(&control));
        drop(tasks);
        let sender = self.sender.clone();
        let thread_control = Arc::clone(&control);
        std::thread::spawn(move || {
            if thread_control.wait(delay) && thread_control.queue() {
                sender.send_controlled(message, Arc::clone(&thread_control));
            }
        });
        ComponentTimer {
            task: ComponentTask { control },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentTaskStatus {
    Running,
    Queued,
    Delivered,
    Cancelled,
    Rejected,
}

struct TaskControl {
    changed: Condvar,
    status: AtomicU8,
    wait: Mutex<()>,
}

impl Default for TaskControl {
    fn default() -> Self {
        Self {
            changed: Condvar::new(),
            status: AtomicU8::new(0),
            wait: Mutex::new(()),
        }
    }
}

impl TaskControl {
    fn queue(&self) -> bool {
        self.status
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    fn deliver(&self) -> bool {
        self.status
            .compare_exchange(1, 2, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    fn cancel(&self) {
        let mut current = self.status.load(Ordering::Acquire);
        while current <= 1 {
            match self
                .status
                .compare_exchange(current, 3, Ordering::AcqRel, Ordering::Acquire)
            {
                Ok(_) => {
                    self.changed.notify_all();
                    return;
                }
                Err(actual) => current = actual,
            }
        }
    }

    fn reject(&self) {
        let _ = self
            .status
            .compare_exchange(1, 4, Ordering::AcqRel, Ordering::Acquire);
    }

    fn status(&self) -> ComponentTaskStatus {
        match self.status.load(Ordering::Acquire) {
            0 => ComponentTaskStatus::Running,
            1 => ComponentTaskStatus::Queued,
            2 => ComponentTaskStatus::Delivered,
            3 => ComponentTaskStatus::Cancelled,
            4 => ComponentTaskStatus::Rejected,
            _ => unreachable!(),
        }
    }

    fn wait(&self, delay: Duration) -> bool {
        let wait = self.wait.lock().unwrap();
        let _ = self
            .changed
            .wait_timeout_while(wait, delay, |_| {
                self.status() == ComponentTaskStatus::Running
            })
            .unwrap();
        self.status() == ComponentTaskStatus::Running
    }
}

#[derive(Clone)]
pub struct CancellationToken {
    control: Arc<TaskControl>,
}

impl CancellationToken {
    pub fn is_cancelled(&self) -> bool {
        self.control.status() == ComponentTaskStatus::Cancelled
    }
}

#[derive(Clone)]
pub struct ComponentTask {
    control: Arc<TaskControl>,
}

impl ComponentTask {
    pub fn cancel(&self) {
        self.control.cancel();
    }

    pub fn status(&self) -> ComponentTaskStatus {
        self.control.status()
    }
}

#[must_use = "dropping the timer cancels message delivery"]
pub struct ComponentTimer {
    task: ComponentTask,
}

impl ComponentTimer {
    pub fn cancel(&self) {
        self.task.cancel();
    }

    pub fn status(&self) -> ComponentTaskStatus {
        self.task.status()
    }
}

impl Drop for ComponentTimer {
    fn drop(&mut self) {
        self.cancel();
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EffectKey(Key);

impl<T: Into<Key>> From<T> for EffectKey {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

type EffectCleanup = Box<dyn FnOnce()>;
type EffectSetup = Box<dyn FnOnce() -> Option<EffectCleanup>>;

struct EffectRegistration {
    dependency: Box<dyn Any>,
    equals: fn(&dyn Any, &dyn Any) -> bool,
    key: EffectKey,
    setup: EffectSetup,
}

struct EffectSlot {
    cleanup: Option<EffectCleanup>,
    dependency: Box<dyn Any>,
    equals: fn(&dyn Any, &dyn Any) -> bool,
    key: EffectKey,
}

#[derive(Default)]
struct EffectDraft {
    duplicate: Option<EffectKey>,
    registrations: Vec<EffectRegistration>,
}

impl EffectDraft {
    fn register<D>(
        &mut self,
        key: EffectKey,
        dependency: D,
        setup: impl FnOnce() -> Option<EffectCleanup> + 'static,
    ) where
        D: PartialEq + 'static,
    {
        if self
            .registrations
            .iter()
            .any(|registration| registration.key == key)
        {
            self.duplicate.get_or_insert(key);
            return;
        }
        self.registrations.push(EffectRegistration {
            dependency: Box::new(dependency),
            equals: |left, right| {
                left.downcast_ref::<D>()
                    .zip(right.downcast_ref::<D>())
                    .is_some_and(|(left, right)| left == right)
            },
            key,
            setup: Box::new(setup),
        });
    }

    fn commit(self, slots: &mut Vec<EffectSlot>) {
        let mut previous = std::mem::take(slots);
        for registration in self.registrations {
            let retained = previous
                .iter()
                .position(|slot| slot.key == registration.key)
                .and_then(|index| {
                    (previous[index].equals)(
                        previous[index].dependency.as_ref(),
                        registration.dependency.as_ref(),
                    )
                    .then(|| previous.remove(index))
                });
            if let Some(slot) = retained {
                slots.push(slot);
            } else {
                if let Some(index) = previous
                    .iter()
                    .position(|slot| slot.key == registration.key)
                    && let Some(cleanup) = previous.remove(index).cleanup
                {
                    cleanup();
                }
                slots.push(EffectSlot {
                    cleanup: (registration.setup)(),
                    dependency: registration.dependency,
                    equals: registration.equals,
                    key: registration.key,
                });
            }
        }
        for slot in previous.iter_mut().rev() {
            if let Some(cleanup) = slot.cleanup.take() {
                cleanup();
            }
        }
    }
}

pub struct ComponentViewContext<'a, M> {
    context: ComponentContext<M>,
    contexts: &'a HashMap<ContextId, ContextValue>,
    dependencies: HashSet<ContextId>,
    effects: EffectDraft,
}

impl<M: Send + 'static> ComponentViewContext<'_, M> {
    pub fn sender(&self) -> ComponentSender<M> {
        self.context.sender()
    }

    pub fn completion(&self) -> ComponentCompletion<M> {
        self.context.completion()
    }

    pub fn root(&self) -> ElementRef {
        self.context.root()
    }

    pub fn use_context<T: Clone + 'static>(&mut self, context: &Context<T>) -> T {
        self.dependencies.insert(context.id);
        self.contexts.get(&context.id).map_or_else(
            || context.default.clone(),
            |value| value.value.downcast_ref::<T>().unwrap().clone(),
        )
    }

    pub fn use_effect<D>(
        &mut self,
        key: impl Into<EffectKey>,
        dependency: D,
        setup: impl FnOnce() -> Option<EffectCleanup> + 'static,
    ) where
        D: PartialEq + 'static,
    {
        self.effects.register(key.into(), dependency, setup);
    }

    pub fn use_effect_guard<D, G>(
        &mut self,
        key: impl Into<EffectKey>,
        dependency: D,
        setup: impl FnOnce() -> G + 'static,
    ) where
        D: PartialEq + 'static,
        G: 'static,
    {
        self.use_effect(key, dependency, move || {
            let guard = setup();
            Some(Box::new(move || drop(guard)))
        });
    }
}

pub trait Component: Sized + 'static {
    type Input: Clone + PartialEq + 'static;
    type Message: Send + 'static;

    fn create(input: &Self::Input, context: &ComponentContext<Self::Message>) -> Self;
    fn input_changed(&mut self, _input: &Self::Input, _context: &ComponentContext<Self::Message>) {}
    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self::Message>) {}
    fn view(
        &self,
        input: &Self::Input,
        context: &mut ComponentViewContext<'_, Self::Message>,
    ) -> Visual;
}

pub struct ComponentNode {
    key: Key,
    factory: Box<dyn ErasedFactory>,
}

pub fn component<C: Component>(key: impl Into<Key>, input: C::Input) -> ComponentNode {
    ComponentNode {
        key: key.into(),
        factory: Box::new(TypedFactory::<C> { input }),
    }
}

trait ErasedFactory {
    fn create(
        self: Box<Self>,
        id: ComponentId,
        queue: SharedQueue,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<
        (
            Box<dyn ErasedComponent>,
            Visual,
            EffectDraft,
            HashSet<ContextId>,
        ),
        EffectKey,
    >;
}

struct TypedFactory<C: Component> {
    input: C::Input,
}

impl<C: Component> ErasedFactory for TypedFactory<C> {
    fn create(
        self: Box<Self>,
        id: ComponentId,
        queue: SharedQueue,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<
        (
            Box<dyn ErasedComponent>,
            Visual,
            EffectDraft,
            HashSet<ContextId>,
        ),
        EffectKey,
    > {
        let sender = ComponentSender {
            component: id,
            queue,
            marker: std::marker::PhantomData,
        };
        let tasks = Arc::new(Mutex::new(Vec::new()));
        let context = ComponentContext {
            reference: reference.clone(),
            sender: sender.clone(),
            tasks: Arc::clone(&tasks),
        };
        let component = C::create(&self.input, &context);
        let scope = TypedScope {
            component,
            input: self.input,
            sender,
            tasks,
        };
        let (view, effects, dependencies) = scope.render_view(reference, contexts)?;
        Ok((Box::new(scope), view, effects, dependencies))
    }
}

trait ErasedComponent {
    fn apply_input(
        &mut self,
        input: &dyn Any,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<Option<(Visual, EffectDraft, HashSet<ContextId>)>, EffectKey>;
    fn component_type(&self) -> TypeId;
    fn cancel_tasks(&self);
    fn dispatch(
        &mut self,
        message: Box<dyn Any + Send>,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<(Visual, EffectDraft, HashSet<ContextId>), EffectKey>;
    fn render_view(
        &self,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<(Visual, EffectDraft, HashSet<ContextId>), EffectKey>;
    fn sender(&self) -> &dyn Any;
}

struct TypedScope<C: Component> {
    component: C,
    input: C::Input,
    sender: ComponentSender<C::Message>,
    tasks: Arc<Mutex<Vec<Weak<TaskControl>>>>,
}

impl<C: Component> TypedScope<C> {
    fn context(&self, reference: ElementRef) -> ComponentContext<C::Message> {
        ComponentContext {
            reference,
            sender: self.sender.clone(),
            tasks: Arc::clone(&self.tasks),
        }
    }

    fn render_view(
        &self,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<(Visual, EffectDraft, HashSet<ContextId>), EffectKey> {
        let mut context = ComponentViewContext {
            context: self.context(reference),
            contexts,
            dependencies: HashSet::new(),
            effects: EffectDraft::default(),
        };
        let view = self.component.view(&self.input, &mut context);
        if let Some(key) = context.effects.duplicate {
            return Err(key);
        }
        Ok((view, context.effects, context.dependencies))
    }
}

impl<C: Component> ErasedComponent for TypedScope<C> {
    fn apply_input(
        &mut self,
        input: &dyn Any,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<Option<(Visual, EffectDraft, HashSet<ContextId>)>, EffectKey> {
        let input = input.downcast_ref::<C::Input>().unwrap();
        if self.input == *input {
            return Ok(None);
        }
        self.input = input.clone();
        let context = self.context(reference.clone());
        self.component.input_changed(&self.input, &context);
        self.render_view(reference, contexts).map(Some)
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
        message: Box<dyn Any + Send>,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<(Visual, EffectDraft, HashSet<ContextId>), EffectKey> {
        let context = self.context(reference.clone());
        self.component
            .update(*message.downcast::<C::Message>().unwrap(), &context);
        self.render_view(reference, contexts)
    }

    fn render_view(
        &self,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<(Visual, EffectDraft, HashSet<ContextId>), EffectKey> {
        self.render_view(reference, contexts)
    }

    fn sender(&self) -> &dyn Any {
        &self.sender
    }
}

struct Scope {
    component: Box<dyn ErasedComponent>,
    dependencies: HashSet<ContextId>,
    effects: Vec<EffectSlot>,
    reference: ElementRef,
    root: Option<ObjectId>,
}

struct ScopeSlot {
    generation: u32,
    scope: Option<Scope>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ComponentDrain {
    pub dispatched: usize,
    pub dropped: usize,
    pub mutations: usize,
}

#[derive(Debug)]
pub enum ComponentError<E> {
    ComponentType(Key),
    DuplicateEffect(EffectKey),
    DuplicateKey(Key),
    MissingComponent(Key),
    Runtime(UpdateError<E>),
}

impl<E> From<UpdateError<E>> for ComponentError<E> {
    fn from(value: UpdateError<E>) -> Self {
        Self::Runtime(value)
    }
}

pub struct ComponentHost<A: Adapter> {
    context_consumers: HashMap<ContextId, HashSet<ComponentId>>,
    contexts: HashMap<ContextId, ContextValue>,
    keys: HashMap<Key, ComponentId>,
    order: Vec<ComponentId>,
    poisoned: bool,
    queue: SharedQueue,
    runtime: Runtime<A>,
    scopes: Vec<ScopeSlot>,
}

impl<A: Adapter> Drop for ComponentHost<A> {
    fn drop(&mut self) {
        for slot in &mut self.scopes {
            let Some(scope) = slot.scope.as_mut() else {
                continue;
            };
            scope.reference.set(None);
            scope.component.cancel_tasks();
            cleanup_effects(&mut scope.effects);
        }
    }
}

impl<A: Adapter> ComponentHost<A> {
    pub fn mount(
        adapter: A,
        components: impl IntoIterator<Item = ComponentNode>,
    ) -> Result<Self, ComponentError<A::Error>> {
        let queue = Arc::new(Mutex::new(MessageQueue::default()));
        let mut host = Self {
            context_consumers: HashMap::new(),
            contexts: HashMap::new(),
            keys: HashMap::new(),
            order: Vec::new(),
            poisoned: false,
            queue,
            runtime: Runtime::new(adapter),
            scopes: Vec::new(),
        };
        let mut declarations = Vec::new();
        let mut pending_effects = Vec::new();
        let mut keys = HashSet::new();
        for node in components {
            if !keys.insert(node.key.clone()) {
                return Err(ComponentError::DuplicateKey(node.key));
            }
            let id = ComponentId {
                index: u32::try_from(host.scopes.len()).unwrap(),
                generation: 0,
            };
            let reference = ElementRef::default();
            let key = node.key;
            let (component, view, effects, dependencies) = node
                .factory
                .create(
                    id,
                    Arc::clone(&host.queue),
                    reference.clone(),
                    &host.contexts,
                )
                .map_err(ComponentError::DuplicateEffect)?;
            declarations.push(keyed(key.clone(), view));
            host.scopes.push(ScopeSlot {
                generation: 0,
                scope: Some(Scope {
                    component,
                    dependencies,
                    effects: Vec::new(),
                    reference,
                    root: None,
                }),
            });
            host.order.push(id);
            host.keys.insert(key, id);
            pending_effects.push((id, effects));
        }
        host.runtime.update(Grid::new().children(declarations))?;
        host.refresh_roots();
        for (id, effects) in pending_effects {
            effects.commit(&mut host.scope_mut(id).unwrap().effects);
        }
        for id in host.order.clone() {
            for context in host.scope(id).unwrap().dependencies.clone() {
                host.context_consumers
                    .entry(context)
                    .or_default()
                    .insert(id);
            }
        }
        Ok(host)
    }

    pub fn runtime(&self) -> &Runtime<A> {
        &self.runtime
    }

    pub fn runtime_mut(&mut self) -> &mut Runtime<A> {
        &mut self.runtime
    }

    pub fn set_waker(&mut self, waker: impl Fn() + Send + Sync + 'static) {
        let mut queue = self.queue.lock().unwrap();
        queue.waker = Some(Arc::new(waker));
    }

    pub fn sender<C: Component>(&self, key: &Key) -> Option<ComponentSender<C::Message>> {
        if self.poisoned {
            return None;
        }
        let scope = self.scope(self.find(key)?)?;
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
        if self.poisoned {
            return None;
        }
        self.scope(self.find(key)?)
            .map(|scope| scope.reference.clone())
    }

    pub fn update_input<C: Component>(
        &mut self,
        key: &Key,
        input: C::Input,
    ) -> Result<Vec<Mutation>, ComponentError<A::Error>> {
        self.ensure_active()?;
        let id = self
            .find(key)
            .ok_or_else(|| ComponentError::MissingComponent(key.clone()))?;
        if self.scope(id).unwrap().component.component_type() != TypeId::of::<C>() {
            return Err(ComponentError::ComponentType(key.clone()));
        }
        let contexts = self.contexts.clone();
        let scope = self.scope_mut(id).unwrap();
        let reference = scope.reference.clone();
        let Some((view, effects, dependencies)) = scope
            .component
            .apply_input(&input, reference, &contexts)
            .map_err(ComponentError::DuplicateEffect)?
        else {
            return Ok(Vec::new());
        };
        let root = scope.root.unwrap();
        let mutations = match self.runtime.update_subtree(root, view) {
            Ok(mutations) => mutations,
            Err(error) => return Err(self.runtime_error(error)),
        };
        let scope = self.scope_mut(id).unwrap();
        effects.commit(&mut scope.effects);
        self.replace_dependencies(id, dependencies);
        Ok(mutations)
    }

    pub fn set_context<T: Clone + PartialEq + 'static>(
        &mut self,
        context: &Context<T>,
        value: T,
    ) -> Result<ComponentDrain, ComponentError<A::Error>> {
        self.ensure_active()?;
        if self
            .contexts
            .get(&context.id)
            .is_some_and(|current| (current.equals)(current.value.as_ref(), &value))
        {
            return Ok(ComponentDrain::default());
        }
        let mut contexts = self.contexts.clone();
        contexts.insert(
            context.id,
            ContextValue {
                equals: |left, right| {
                    left.downcast_ref::<T>()
                        .zip(right.downcast_ref::<T>())
                        .is_some_and(|(left, right)| left == right)
                },
                value: Rc::new(value),
            },
        );
        let affected = self
            .context_consumers
            .get(&context.id)
            .map(|consumers| consumers.iter().copied().collect::<Vec<_>>())
            .unwrap_or_default();
        let mut pending = Vec::with_capacity(affected.len());
        for id in affected {
            let scope = self.scope(id).unwrap();
            let (view, effects, dependencies) = scope
                .component
                .render_view(scope.reference.clone(), &contexts)
                .map_err(ComponentError::DuplicateEffect)?;
            validate_declaration(&view.0)
                .map_err(UpdateError::Graph)
                .map_err(ComponentError::Runtime)?;
            let root = scope.root.unwrap();
            let previous = self.runtime.graph().kind(root).unwrap();
            if previous != view.0.kind {
                return Err(ComponentError::Runtime(UpdateError::Graph(
                    GraphError::RootTypeChanged {
                        previous,
                        next: view.0.kind,
                    },
                )));
            }
            pending.push((id, root, view, effects, dependencies));
        }
        self.contexts = contexts;
        let mut report = ComponentDrain::default();
        for (id, root, view, effects, dependencies) in pending {
            let mutations = match self.runtime.update_subtree(root, view) {
                Ok(mutations) => mutations,
                Err(error) => return Err(self.runtime_error(error)),
            };
            let scope = self.scope_mut(id).unwrap();
            effects.commit(&mut scope.effects);
            self.replace_dependencies(id, dependencies);
            report.dispatched += 1;
            report.mutations += mutations.len();
        }
        Ok(report)
    }

    pub fn drain(&mut self, limit: usize) -> Result<ComponentDrain, ComponentError<A::Error>> {
        self.ensure_active()?;
        let mut report = ComponentDrain::default();
        self.queue.lock().unwrap().wake_pending = false;
        let contexts = self.contexts.clone();
        let mut events = Vec::new();
        if let Err(error) = self.runtime.drain_events(&mut events) {
            return Err(self.runtime_error(error));
        }
        for event in events {
            event.invoke();
        }
        let mut processed = 0;
        while processed < limit {
            let Some(message) = self.queue.lock().unwrap().messages.pop_front() else {
                break;
            };
            processed += 1;
            let Some(scope) = self.scope_mut(message.component) else {
                if let Some(control) = message.control {
                    control.cancel();
                }
                report.dropped += 1;
                continue;
            };
            if let Some(control) = &message.control
                && !control.deliver()
            {
                report.dropped += 1;
                continue;
            }
            let reference = scope.reference.clone();
            let (view, effects, dependencies) = scope
                .component
                .dispatch(message.value, reference, &contexts)
                .map_err(ComponentError::DuplicateEffect)?;
            let root = scope.root.unwrap();
            let mutations = match self.runtime.update_subtree(root, view) {
                Ok(mutations) => mutations,
                Err(error) => return Err(self.runtime_error(error)),
            };
            let scope = self.scope_mut(message.component).unwrap();
            effects.commit(&mut scope.effects);
            self.replace_dependencies(message.component, dependencies);
            report.dispatched += 1;
            report.mutations += mutations.len();
        }
        self.rearm_wake();
        Ok(report)
    }

    pub fn remove(&mut self, key: &Key) -> Result<(), ComponentError<A::Error>> {
        self.ensure_active()?;
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
        self.order.retain(|current| *current != id);
        self.keys.remove(key);
        for consumers in self.context_consumers.values_mut() {
            consumers.remove(&id);
        }
        self.context_consumers
            .retain(|_, consumers| !consumers.is_empty());
        let slot = &mut self.scopes[id.index as usize];
        let mut scope = slot.scope.take().unwrap();
        scope.reference.set(None);
        scope.component.cancel_tasks();
        cleanup_effects(&mut scope.effects);
        slot.generation = slot.generation.wrapping_add(1);
        Ok(())
    }

    fn refresh_roots(&mut self) {
        let root = self.runtime.graph().root().unwrap();
        let children = self
            .runtime
            .graph()
            .children(root, RelationId::Children)
            .unwrap()
            .to_vec();
        for (id, object) in self.order.clone().into_iter().zip(children) {
            let scope = self.scope_mut(id).unwrap();
            scope.root = Some(object);
            scope.reference.set(Some(object));
        }
    }

    fn find(&self, key: &Key) -> Option<ComponentId> {
        self.keys.get(key).copied()
    }

    fn scope(&self, id: ComponentId) -> Option<&Scope> {
        let slot = self.scopes.get(id.index as usize)?;
        (slot.generation == id.generation)
            .then_some(slot.scope.as_ref())
            .flatten()
    }

    fn scope_mut(&mut self, id: ComponentId) -> Option<&mut Scope> {
        let slot = self.scopes.get_mut(id.index as usize)?;
        (slot.generation == id.generation)
            .then_some(slot.scope.as_mut())
            .flatten()
    }

    fn invalidate(&mut self) {
        for slot in &mut self.scopes {
            if let Some(scope) = slot.scope.as_mut() {
                scope.root = None;
                scope.reference.set(None);
                scope.component.cancel_tasks();
            }
        }
    }

    fn ensure_active(&self) -> Result<(), ComponentError<A::Error>> {
        if self.poisoned {
            Err(ComponentError::Runtime(UpdateError::Poisoned))
        } else {
            Ok(())
        }
    }

    fn runtime_error(&mut self, error: UpdateError<A::Error>) -> ComponentError<A::Error> {
        if matches!(&error, UpdateError::Adapter(_) | UpdateError::Poisoned) {
            self.poisoned = true;
            self.invalidate();
        }
        ComponentError::Runtime(error)
    }

    fn rearm_wake(&self) {
        let mut queue = self.queue.lock().unwrap();
        let wake = (!queue.messages.is_empty() && !queue.wake_pending)
            .then(|| queue.waker.clone())
            .flatten();
        queue.wake_pending |= wake.is_some();
        drop(queue);
        if let Some(wake) = wake {
            wake();
        }
    }

    fn replace_dependencies(&mut self, id: ComponentId, dependencies: HashSet<ContextId>) {
        let previous = std::mem::replace(
            &mut self.scope_mut(id).unwrap().dependencies,
            dependencies.clone(),
        );
        for context in previous.difference(&dependencies) {
            let remove = if let Some(consumers) = self.context_consumers.get_mut(context) {
                consumers.remove(&id);
                consumers.is_empty()
            } else {
                false
            };
            if remove {
                self.context_consumers.remove(context);
            }
        }
        for context in dependencies.difference(&previous) {
            self.context_consumers
                .entry(*context)
                .or_default()
                .insert(id);
        }
    }
}

fn cleanup_effects(effects: &mut [EffectSlot]) {
    for effect in effects.iter_mut().rev() {
        if let Some(cleanup) = effect.cleanup.take() {
            cleanup();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Clone)]
    struct CounterInput {
        cleanup: Arc<AtomicUsize>,
        value: usize,
    }

    impl PartialEq for CounterInput {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value && Arc::ptr_eq(&self.cleanup, &other.cleanup)
        }
    }

    struct Counter {
        value: usize,
    }

    impl Component for Counter {
        type Input = CounterInput;
        type Message = usize;

        fn create(input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self { value: input.value }
        }

        fn input_changed(
            &mut self,
            input: &Self::Input,
            _context: &ComponentContext<Self::Message>,
        ) {
            self.value = input.value;
        }

        fn update(&mut self, message: usize, _context: &ComponentContext<Self::Message>) {
            self.value += message;
        }

        fn view(
            &self,
            input: &Self::Input,
            context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            let cleanup = Arc::clone(&input.cleanup);
            context.use_effect_guard("value", self.value, move || Cleanup(cleanup));
            TextBlock::new(self.value.to_string()).into()
        }
    }

    struct Label;

    impl Component for Label {
        type Input = Rc<str>;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self
        }

        fn view(
            &self,
            input: &Self::Input,
            _context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            TextBlock::new(input.clone()).into()
        }
    }

    struct Cleanup(Arc<AtomicUsize>);

    impl Drop for Cleanup {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[derive(Clone)]
    struct ContextInput {
        context: Rc<Context<usize>>,
        renders: Arc<AtomicUsize>,
        subscribe: bool,
    }

    impl PartialEq for ContextInput {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.context, &other.context)
                && Arc::ptr_eq(&self.renders, &other.renders)
                && self.subscribe == other.subscribe
        }
    }

    struct ContextReader;

    impl Component for ContextReader {
        type Input = ContextInput;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self
        }

        fn view(
            &self,
            input: &Self::Input,
            context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            input.renders.fetch_add(1, Ordering::Relaxed);
            let value = if input.subscribe {
                context.use_context(&input.context)
            } else {
                0
            };
            TextBlock::new(value.to_string()).into()
        }
    }

    #[derive(Clone)]
    struct WorkerInput {
        cancelled: Arc<AtomicUsize>,
        started: Arc<AtomicUsize>,
        task: Arc<Mutex<Option<ComponentTask>>>,
    }

    impl PartialEq for WorkerInput {
        fn eq(&self, other: &Self) -> bool {
            Arc::ptr_eq(&self.cancelled, &other.cancelled)
                && Arc::ptr_eq(&self.started, &other.started)
                && Arc::ptr_eq(&self.task, &other.task)
        }
    }

    enum WorkerMessage {
        Complete,
        Start,
    }

    struct Worker(WorkerInput);

    impl Component for Worker {
        type Input = WorkerInput;
        type Message = WorkerMessage;

        fn create(input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self(input.clone())
        }

        fn update(&mut self, message: Self::Message, context: &ComponentContext<Self::Message>) {
            match message {
                WorkerMessage::Complete => {}
                WorkerMessage::Start => {
                    let started = Arc::clone(&self.0.started);
                    let cancelled = Arc::clone(&self.0.cancelled);
                    let task = context.spawn_background(move |token| {
                        started.store(1, Ordering::Release);
                        while !token.is_cancelled() {
                            std::thread::yield_now();
                        }
                        cancelled.store(1, Ordering::Release);
                        WorkerMessage::Complete
                    });
                    *self.0.task.lock().unwrap() = Some(task);
                }
            }
        }

        fn view(
            &self,
            _input: &Self::Input,
            _context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            TextBlock::new("Worker").into()
        }
    }

    struct RootSwitch(bool);

    impl Component for RootSwitch {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self(false)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self::Message>) {
            self.0 = true;
        }

        fn view(
            &self,
            _input: &Self::Input,
            _context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            if self.0 {
                Border::new().into()
            } else {
                TextBlock::new("Stable").into()
            }
        }
    }

    #[test]
    fn heterogeneous_components_update_isolated_subtrees() {
        let cleanup = Arc::new(AtomicUsize::new(0));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [
                component::<Counter>(
                    "counter",
                    CounterInput {
                        cleanup: Arc::clone(&cleanup),
                        value: 0,
                    },
                ),
                component::<Label>("label", Rc::from("Label")),
            ],
        )
        .unwrap();
        let counter = host.reference(&Key::from("counter")).unwrap();
        let label = host.reference(&Key::from("label")).unwrap();
        let counter_root = counter.get().unwrap();
        let label_root = label.get().unwrap();

        let mutations = host
            .update_input::<Label>(&Key::from("label"), Rc::from("Changed"))
            .unwrap();
        assert_eq!(mutations.len(), 1);
        assert_eq!(counter.get(), Some(counter_root));
        assert_eq!(label.get(), Some(label_root));

        let sender = host.sender::<Counter>(&Key::from("counter")).unwrap();
        assert!(sender.send(1));
        assert_eq!(host.drain(1).unwrap().mutations, 1);
        assert_eq!(cleanup.load(Ordering::Relaxed), 1);

        assert!(sender.send(0));
        assert_eq!(host.drain(1).unwrap().mutations, 0);
        assert_eq!(cleanup.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn retired_component_drops_effects_and_async_completion() {
        let cleanup = Arc::new(AtomicUsize::new(0));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<Counter>(
                "counter",
                CounterInput {
                    cleanup: Arc::clone(&cleanup),
                    value: 0,
                },
            )],
        )
        .unwrap();
        let reference = host.reference(&Key::from("counter")).unwrap();
        let completion = host
            .sender::<Counter>(&Key::from("counter"))
            .unwrap()
            .completion();

        host.remove(&Key::from("counter")).unwrap();
        assert!(
            std::thread::spawn(move || completion.complete(1))
                .join()
                .unwrap()
        );
        let report = host.drain(1).unwrap();

        assert_eq!(reference.get(), None);
        assert_eq!(cleanup.load(Ordering::Relaxed), 1);
        assert_eq!(report.dropped, 1);
        assert_eq!(report.dispatched, 0);
    }

    #[test]
    fn context_change_renders_only_subscribers() {
        let context = Rc::new(Context::new(0usize));
        let subscriber_renders = Arc::new(AtomicUsize::new(0));
        let other_renders = Arc::new(AtomicUsize::new(0));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [
                component::<ContextReader>(
                    "subscriber",
                    ContextInput {
                        context: Rc::clone(&context),
                        renders: Arc::clone(&subscriber_renders),
                        subscribe: true,
                    },
                ),
                component::<ContextReader>(
                    "other",
                    ContextInput {
                        context: Rc::clone(&context),
                        renders: Arc::clone(&other_renders),
                        subscribe: false,
                    },
                ),
            ],
        )
        .unwrap();

        let report = host.set_context(&context, 1).unwrap();

        assert_eq!(report.dispatched, 1);
        assert_eq!(report.mutations, 1);
        assert_eq!(subscriber_renders.load(Ordering::Relaxed), 2);
        assert_eq!(other_renders.load(Ordering::Relaxed), 1);
        assert_eq!(
            host.set_context(&context, 1).unwrap(),
            ComponentDrain::default()
        );
    }

    #[test]
    fn retirement_cancels_background_delivery() {
        let input = WorkerInput {
            cancelled: Arc::new(AtomicUsize::new(0)),
            started: Arc::new(AtomicUsize::new(0)),
            task: Arc::new(Mutex::new(None)),
        };
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<Worker>("worker", input.clone())],
        )
        .unwrap();
        assert!(
            host.sender::<Worker>(&Key::from("worker"))
                .unwrap()
                .send(WorkerMessage::Start)
        );
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        while input.started.load(Ordering::Acquire) == 0 {
            std::thread::yield_now();
        }

        host.remove(&Key::from("worker")).unwrap();
        while input.cancelled.load(Ordering::Acquire) == 0 {
            std::thread::yield_now();
        }

        assert_eq!(
            input.task.lock().unwrap().as_ref().unwrap().status(),
            ComponentTaskStatus::Cancelled
        );
        assert_eq!(host.drain(1).unwrap(), ComponentDrain::default());
    }

    #[test]
    fn queued_messages_coalesce_host_wake() {
        let wakes = Arc::new(AtomicUsize::new(0));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<Label>("label", Rc::from("Label"))],
        )
        .unwrap();
        let callback_wakes = Arc::clone(&wakes);
        host.set_waker(move || {
            callback_wakes.fetch_add(1, Ordering::Relaxed);
        });
        let sender = host.sender::<Label>(&Key::from("label")).unwrap();

        assert!(sender.send(()));
        assert!(sender.send(()));
        assert!(sender.send(()));
        assert_eq!(wakes.load(Ordering::Relaxed), 1);
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(wakes.load(Ordering::Relaxed), 2);
        assert_eq!(host.drain(2).unwrap().dispatched, 2);

        assert!(sender.send(()));
        assert_eq!(wakes.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn recoverable_component_error_does_not_invalidate_other_scopes() {
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [
                component::<RootSwitch>("switch", ()),
                component::<Label>("label", Rc::from("Label")),
            ],
        )
        .unwrap();
        let switch = host.sender::<RootSwitch>(&Key::from("switch")).unwrap();
        let label = host.sender::<Label>(&Key::from("label")).unwrap();
        let label_reference = host.reference(&Key::from("label")).unwrap();
        let label_root = label_reference.get();

        assert!(switch.send(()));
        assert!(matches!(
            host.drain(1),
            Err(ComponentError::Runtime(UpdateError::Graph(
                GraphError::RootTypeChanged { .. }
            )))
        ));
        assert_eq!(label_reference.get(), label_root);

        assert!(label.send(()));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(label_reference.get(), label_root);
    }
}
