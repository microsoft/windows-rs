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
    services: Arc<dyn ComponentServices>,
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
        self.services.spawn_background(Box::new(move || {
            let message = work(CancellationToken {
                control: Arc::clone(&thread_control),
            });
            if thread_control.queue() {
                sender.send_controlled(message, Arc::clone(&thread_control));
            }
        }));
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
        let timer_control = Arc::clone(&control);
        let registration = self.services.set_timeout(
            delay,
            Box::new(move || {
                timer_control.timer_fired();
                if timer_control.queue() {
                    sender.send_controlled(message, Arc::clone(&timer_control));
                }
            }),
        );
        control.set_timer(registration);
        ComponentTimer {
            task: ComponentTask { control },
        }
    }
}

pub trait ComponentTimerRegistration: Send + Sync {
    fn cancel(&self);
}

pub trait ComponentServices: Send + Sync {
    fn spawn_background(&self, work: Box<dyn FnOnce() + Send>);

    fn set_timeout(
        &self,
        delay: Duration,
        callback: Box<dyn FnOnce() + Send>,
    ) -> Arc<dyn ComponentTimerRegistration>;
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
    cancelled: std::sync::atomic::AtomicBool,
    changed: Condvar,
    wait: Mutex<()>,
}

impl ComponentTimerRegistration for ThreadPoolTimer {
    fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
        self.changed.notify_all();
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
    status: AtomicU8,
    timer: Mutex<Option<Arc<dyn ComponentTimerRegistration>>>,
}

impl Default for TaskControl {
    fn default() -> Self {
        Self {
            status: AtomicU8::new(0),
            timer: Mutex::new(None),
        }
    }
}

impl TaskControl {
    fn set_timer(&self, timer: Arc<dyn ComponentTimerRegistration>) {
        let mut current = self.timer.lock().unwrap();
        if self.status() == ComponentTaskStatus::Running {
            *current = Some(timer);
        } else {
            drop(current);
            timer.cancel();
        }
    }

    fn timer_fired(&self) {
        self.timer.lock().unwrap().take();
    }

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
                    if let Some(timer) = self.timer.lock().unwrap().take() {
                        timer.cancel();
                    }
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

struct PreparedEffects(Vec<PreparedEffect>);

enum PreparedEffect {
    Retained(EffectSlot),
    Setup(EffectRegistration),
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

    fn prepare(self, slots: &mut Vec<EffectSlot>) -> PreparedEffects {
        let mut previous = std::mem::take(slots);
        let mut prepared = Vec::with_capacity(self.registrations.len());
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
                prepared.push(PreparedEffect::Retained(slot));
            } else {
                if let Some(index) = previous
                    .iter()
                    .position(|slot| slot.key == registration.key)
                    && let Some(cleanup) = previous.remove(index).cleanup
                {
                    cleanup();
                }
                prepared.push(PreparedEffect::Setup(registration));
            }
        }
        for slot in previous.iter_mut().rev() {
            if let Some(cleanup) = slot.cleanup.take() {
                cleanup();
            }
        }
        PreparedEffects(prepared)
    }

    fn commit(self, slots: &mut Vec<EffectSlot>) {
        self.prepare(slots).commit(slots);
    }
}

impl PreparedEffects {
    fn commit(self, slots: &mut Vec<EffectSlot>) {
        for effect in self.0 {
            match effect {
                PreparedEffect::Retained(slot) => slots.push(slot),
                PreparedEffect::Setup(registration) => slots.push(EffectSlot {
                    cleanup: (registration.setup)(),
                    dependency: registration.dependency,
                    equals: registration.equals,
                    key: registration.key,
                }),
            }
        }
    }

    fn cancel(self) {
        for effect in self.0 {
            if let PreparedEffect::Retained(mut slot) = effect
                && let Some(cleanup) = slot.cleanup.take()
            {
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
    pub(crate) key: Key,
    factory: Rc<dyn ErasedFactory>,
}

pub fn component<C: Component>(key: impl Into<Key>, input: C::Input) -> ComponentNode {
    ComponentNode {
        key: key.into(),
        factory: Rc::new(TypedFactory::<C> { input }),
    }
}

impl ComponentNode {
    pub fn keyed(self) -> KeyedVisual {
        let key = self.key.clone();
        keyed(key, self)
    }
}

trait ErasedFactory {
    fn component_type(&self) -> TypeId;
    fn create(
        &self,
        id: ComponentId,
        queue: SharedQueue,
        reference: ElementRef,
        services: Arc<dyn ComponentServices>,
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
        queue: SharedQueue,
        reference: ElementRef,
        services: Arc<dyn ComponentServices>,
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
            services: Arc::clone(&services),
            tasks: Arc::clone(&tasks),
        };
        let component = C::create(&self.input, &context);
        let scope = TypedScope {
            component,
            input: self.input.clone(),
            sender,
            services,
            tasks,
        };
        let (view, effects, dependencies) = scope.render_view(reference, contexts)?;
        Ok((Box::new(scope), view, effects, dependencies))
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

impl From<ComponentNode> for Visual {
    fn from(node: ComponentNode) -> Self {
        Self(DeclaredNode::Component {
            node,
            relation_key: None,
        })
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
    services: Arc<dyn ComponentServices>,
    tasks: Arc<Mutex<Vec<Weak<TaskControl>>>>,
}

impl<C: Component> TypedScope<C> {
    fn context(&self, reference: ElementRef) -> ComponentContext<C::Message> {
        ComponentContext {
            reference,
            sender: self.sender.clone(),
            services: Arc::clone(&self.services),
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
    children: HashMap<Key, ComponentId>,
    component: Box<dyn ErasedComponent>,
    dependencies: HashSet<ContextId>,
    effects: Vec<EffectSlot>,
    key: Key,
    parent: Option<ComponentId>,
    reference: ElementRef,
    root: Option<ObjectId>,
}

struct ScopeSlot {
    generation: u32,
    scope: Option<Scope>,
}

struct PendingScopeRender {
    dependencies: HashSet<ContextId>,
    effects: EffectDraft,
    id: ComponentId,
}

#[derive(Default)]
struct ExpansionState {
    created: Vec<ComponentId>,
    objects: usize,
    pending: Vec<PendingScopeRender>,
    seen: HashMap<ComponentId, HashSet<Key>>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ComponentDrain {
    pub dispatched: usize,
    pub dropped: usize,
    pub mutations: usize,
}

#[derive(Debug)]
pub enum ComponentError<E> {
    ComponentKey { component: Key, relation: Key },
    ComponentRoot,
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

impl<E: std::fmt::Debug> From<ComponentError<E>> for windows_core::Error {
    fn from(value: ComponentError<E>) -> Self {
        Self::new(
            windows_core::HRESULT(0x80004005_u32 as i32),
            format!("{value:?}"),
        )
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
    services: Arc<dyn ComponentServices>,
    free_scopes: Vec<u32>,
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
        Self::mount_with_services(adapter, Arc::new(DefaultComponentServices), components)
    }

    pub fn mount_with_services(
        adapter: A,
        services: Arc<dyn ComponentServices>,
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
            services,
            free_scopes: Vec::new(),
            scopes: Vec::new(),
        };
        let mut declarations = Vec::new();
        let mut keys = HashSet::new();
        let mut expansion = ExpansionState::default();
        for node in components {
            if !keys.insert(node.key.clone()) {
                return Err(ComponentError::DuplicateKey(node.key));
            }
            let key = node.key.clone();
            let (id, view, effects, dependencies) = host.create_scope(None, node)?;
            host.order.push(id);
            host.keys.insert(key, id);
            expansion.pending.push(PendingScopeRender {
                dependencies,
                effects,
                id,
            });
            let view = host.expand_view(id, view, 0, &mut expansion)?;
            declarations.push(keyed(host.scope(id).unwrap().key.clone(), view));
        }
        let root: Visual = Grid::new().children(declarations).into();
        host.runtime.update(root.clone())?;
        host.refresh_roots(&root);
        for pending in expansion.pending {
            pending
                .effects
                .commit(&mut host.scope_mut(pending.id).unwrap().effects);
            host.replace_dependencies(pending.id, pending.dependencies);
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
        self.sender_at::<C>(std::slice::from_ref(key))
    }

    pub fn sender_at<C: Component>(&self, path: &[Key]) -> Option<ComponentSender<C::Message>> {
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
    ) -> Result<Vec<Mutation>, ComponentError<A::Error>> {
        self.update_input_at::<C>(std::slice::from_ref(key), input)
    }

    pub fn update_input_at<C: Component>(
        &mut self,
        path: &[Key],
        input: C::Input,
    ) -> Result<Vec<Mutation>, ComponentError<A::Error>> {
        self.ensure_active()?;
        let key = path.last().cloned().unwrap_or_else(|| Key::from(""));
        let id = self
            .find_path(path)
            .ok_or_else(|| ComponentError::MissingComponent(key.clone()))?;
        if self.scope(id).unwrap().component.component_type() != TypeId::of::<C>() {
            return Err(ComponentError::ComponentType(key));
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
        self.apply_render(id, view, effects, dependencies)
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
        let affected_set = affected.iter().copied().collect::<HashSet<_>>();
        let affected = affected
            .into_iter()
            .filter(|id| {
                let mut parent = self.scope(*id).and_then(|scope| scope.parent);
                while let Some(current) = parent {
                    if affected_set.contains(&current) {
                        return false;
                    }
                    parent = self.scope(current).and_then(|scope| scope.parent);
                }
                true
            })
            .collect::<Vec<_>>();
        let mut pending = Vec::with_capacity(affected.len());
        for id in affected {
            let scope = self.scope(id).unwrap();
            let (view, effects, dependencies) = scope
                .component
                .render_view(scope.reference.clone(), &contexts)
                .map_err(ComponentError::DuplicateEffect)?;
            pending.push((id, view, effects, dependencies));
        }
        self.contexts = contexts;
        let mut report = ComponentDrain::default();
        for (id, view, effects, dependencies) in pending {
            let mutations = self.apply_render(id, view, effects, dependencies)?;
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
            self.rearm_wake();
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
            let (view, effects, dependencies) =
                match scope
                    .component
                    .dispatch(message.value, reference, &contexts)
                {
                    Ok(rendered) => rendered,
                    Err(error) => {
                        self.rearm_wake();
                        return Err(ComponentError::DuplicateEffect(error));
                    }
                };
            let mutations = match self.apply_render(message.component, view, effects, dependencies)
            {
                Ok(mutations) => mutations,
                Err(error) => {
                    self.rearm_wake();
                    return Err(error);
                }
            };
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
        self.retire_scope(id);
        self.context_consumers
            .retain(|_, consumers| !consumers.is_empty());
        Ok(())
    }

    fn create_scope(
        &mut self,
        parent: Option<ComponentId>,
        node: ComponentNode,
    ) -> Result<(ComponentId, Visual, EffectDraft, HashSet<ContextId>), ComponentError<A::Error>>
    {
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
        let key = node.key.clone();
        let (component, view, effects, dependencies) = match node.factory.create(
            id,
            Arc::clone(&self.queue),
            reference.clone(),
            Arc::clone(&self.services),
            &self.contexts,
        ) {
            Ok(created) => created,
            Err(error) => {
                if let Some(index) = reused {
                    self.free_scopes.push(index);
                }
                return Err(ComponentError::DuplicateEffect(error));
            }
        };
        let scope = Scope {
            children: HashMap::new(),
            component,
            dependencies: HashSet::new(),
            effects: Vec::new(),
            key,
            parent,
            reference,
            root: None,
        };
        if id.index as usize == self.scopes.len() {
            self.scopes.push(ScopeSlot {
                generation: id.generation,
                scope: Some(scope),
            });
        } else {
            self.scopes[id.index as usize].scope = Some(scope);
        }
        Ok((id, view, effects, dependencies))
    }

    fn expand_view(
        &mut self,
        owner: ComponentId,
        view: Visual,
        depth: usize,
        expansion: &mut ExpansionState,
    ) -> Result<Visual, ComponentError<A::Error>> {
        expansion.seen.entry(owner).or_default();
        let declaration = match view.0 {
            DeclaredNode::Object(declaration) => declaration,
            DeclaredNode::Component { .. } => return Err(ComponentError::ComponentRoot),
        };
        Ok(Visual(DeclaredNode::Object(self.expand_declaration(
            owner,
            declaration,
            true,
            depth,
            expansion,
        )?)))
    }

    fn expand_declaration(
        &mut self,
        owner: ComponentId,
        mut declaration: Declaration,
        scope_root: bool,
        depth: usize,
        expansion: &mut ExpansionState,
    ) -> Result<Declaration, ComponentError<A::Error>> {
        if depth > MAX_DEPTH {
            return Err(ComponentError::Runtime(UpdateError::Graph(
                GraphError::DepthExceeded,
            )));
        }
        if expansion.objects >= MAX_OBJECTS {
            return Err(ComponentError::Runtime(UpdateError::Graph(
                GraphError::SizeExceeded,
            )));
        }
        expansion.objects += 1;
        if scope_root {
            declaration.component = Some(owner);
        }
        for relation in declaration.relations.as_slice().to_vec() {
            let mut value = relation.value;
            match &mut value {
                RelationValue::One(Some(child)) => {
                    let expanded =
                        self.expand_node(owner, child.as_ref().clone(), depth + 1, expansion)?;
                    *child = Rc::new(DeclaredNode::Object(expanded));
                }
                RelationValue::Many(children) => {
                    for child in Rc::make_mut(children) {
                        let expanded =
                            self.expand_node(owner, child.clone(), depth + 1, expansion)?;
                        *child = DeclaredNode::Object(expanded);
                    }
                }
                RelationValue::One(None) => {}
            }
            declaration = declaration.relation(relation.id, value);
        }
        Ok(declaration)
    }

    fn expand_node(
        &mut self,
        owner: ComponentId,
        node: DeclaredNode,
        depth: usize,
        expansion: &mut ExpansionState,
    ) -> Result<Declaration, ComponentError<A::Error>> {
        match node {
            DeclaredNode::Object(declaration) => {
                self.expand_declaration(owner, declaration, false, depth, expansion)
            }
            DeclaredNode::Component { node, relation_key } => {
                if let Some(relation_key) = &relation_key
                    && relation_key != &node.key
                {
                    return Err(ComponentError::ComponentKey {
                        component: node.key,
                        relation: relation_key.clone(),
                    });
                }
                let component_key = node.key.clone();
                let seen = expansion.seen.entry(owner).or_default();
                if !seen.insert(node.key.clone()) {
                    return Err(ComponentError::DuplicateKey(node.key));
                }
                let existing = self
                    .scope(owner)
                    .and_then(|scope| scope.children.get(&node.key).copied());
                let (id, view, effects, dependencies) = if let Some(id) = existing {
                    if self.scope(id).unwrap().component.component_type()
                        != node.factory.component_type()
                    {
                        return Err(ComponentError::ComponentType(node.key));
                    }
                    let contexts = self.contexts.clone();
                    let scope = self.scope_mut(id).unwrap();
                    let reference = scope.reference.clone();
                    let rendered = scope
                        .component
                        .apply_input(node.factory.input(), reference.clone(), &contexts)
                        .map_err(ComponentError::DuplicateEffect)?;
                    let (view, effects, dependencies) = match rendered {
                        Some(rendered) => rendered,
                        None => scope
                            .component
                            .render_view(reference, &contexts)
                            .map_err(ComponentError::DuplicateEffect)?,
                    };
                    (id, view, effects, dependencies)
                } else {
                    let key = node.key.clone();
                    let created = self.create_scope(Some(owner), node)?;
                    expansion.created.push(created.0);
                    self.scope_mut(owner)
                        .unwrap()
                        .children
                        .insert(key, created.0);
                    created
                };
                expansion.pending.push(PendingScopeRender {
                    dependencies,
                    effects,
                    id,
                });
                let expanded = self.expand_view(id, view, depth, expansion)?;
                let mut declaration = expanded.0.object().unwrap();
                declaration.key = Some(component_key);
                Ok(declaration)
            }
        }
    }

    fn refresh_roots(&mut self, root: &Visual) {
        let object = self.runtime.graph().root().unwrap();
        let declaration = root.0.as_object().unwrap();
        self.refresh_roots_from(declaration, object);
    }

    fn refresh_roots_from(&mut self, declaration: &Declaration, object: ObjectId) {
        if let Some(id) = declaration.component {
            let scope = self.scope_mut(id).unwrap();
            scope.root = Some(object);
            scope.reference.set(Some(object));
        }
        let mut pending = Vec::new();
        for relation in declaration.relations.iter() {
            match &relation.value {
                RelationValue::One(Some(child)) => {
                    let child = child.as_object().unwrap().clone();
                    let object = self.runtime.graph().child(object, relation.id).unwrap();
                    pending.push((child, object));
                }
                RelationValue::Many(children) => {
                    let objects = self.runtime.graph().children(object, relation.id).unwrap();
                    pending.extend(
                        children
                            .iter()
                            .zip(objects)
                            .map(|(child, object)| (child.as_object().unwrap().clone(), *object)),
                    );
                }
                RelationValue::One(None) => {}
            }
        }
        for (declaration, object) in pending {
            self.refresh_roots_from(&declaration, object);
        }
    }

    fn apply_render(
        &mut self,
        id: ComponentId,
        view: Visual,
        effects: EffectDraft,
        dependencies: HashSet<ContextId>,
    ) -> Result<Vec<Mutation>, ComponentError<A::Error>> {
        let root = self.scope(id).unwrap().root.unwrap();
        let mut expansion = ExpansionState::default();
        expansion.pending.push(PendingScopeRender {
            dependencies,
            effects,
            id,
        });
        let view = match self.expand_view(id, view, 0, &mut expansion) {
            Ok(view) => view,
            Err(error) => {
                self.discard_created(&expansion.created);
                return Err(error);
            }
        };
        let retired = self.unseen_scopes(&expansion.seen);
        let mut pending = Some(expansion.pending);
        let mut prepared = Vec::new();
        let (runtime, scopes) = (&mut self.runtime, &mut self.scopes);
        let mutations = match runtime.update_subtree_before_apply(root, view.clone(), || {
            for pending in pending.take().unwrap() {
                let slots = &mut scopes[pending.id.index as usize]
                    .scope
                    .as_mut()
                    .unwrap()
                    .effects;
                prepared.push((
                    pending.id,
                    pending.effects.prepare(slots),
                    pending.dependencies,
                ));
            }
            prepare_scope_retirement(scopes, &retired);
        }) {
            Ok(mutations) => mutations,
            Err(error) => {
                for (_, prepared, _) in prepared {
                    prepared.cancel();
                }
                self.discard_created(&expansion.created);
                return Err(self.runtime_error(error));
            }
        };
        self.refresh_roots_from(view.0.as_object().unwrap(), root);
        self.retire_unseen(&expansion.seen);
        for (id, prepared, dependencies) in prepared {
            prepared.commit(&mut self.scope_mut(id).unwrap().effects);
            self.replace_dependencies(id, dependencies);
        }
        Ok(mutations)
    }

    fn retire_unseen(&mut self, seen: &HashMap<ComponentId, HashSet<Key>>) {
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

    fn retire_scope(&mut self, id: ComponentId) {
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

    fn discard_created(&mut self, created: &[ComponentId]) {
        for id in created.iter().rev().copied() {
            let parent = self.scope(id).and_then(|scope| scope.parent);
            if let Some(parent) = parent
                && let Some(scope) = self.scope_mut(parent)
            {
                scope.children.retain(|_, child| *child != id);
            }

            self.retire_scope(id);
        }
    }

    fn unseen_scopes(&self, seen: &HashMap<ComponentId, HashSet<Key>>) -> Vec<ComponentId> {
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

fn prepare_scope_retirement(scopes: &mut [ScopeSlot], roots: &[ComponentId]) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    type PendingTimer = (Arc<TestTimerRegistration>, Box<dyn FnOnce() + Send>);

    #[derive(Default)]
    struct TestServices {
        background: Mutex<VecDeque<Box<dyn FnOnce() + Send>>>,
        timers: Mutex<VecDeque<PendingTimer>>,
    }

    impl TestServices {
        fn run_background(&self) {
            self.background.lock().unwrap().pop_front().unwrap()();
        }

        fn fire_timer(&self) {
            let (timer, callback) = self.timers.lock().unwrap().pop_front().unwrap();
            if !timer.cancelled.load(Ordering::Acquire) {
                callback();
            }
        }
    }

    impl ComponentServices for TestServices {
        fn spawn_background(&self, work: Box<dyn FnOnce() + Send>) {
            self.background.lock().unwrap().push_back(work);
        }

        fn set_timeout(
            &self,
            _delay: Duration,
            callback: Box<dyn FnOnce() + Send>,
        ) -> Arc<dyn ComponentTimerRegistration> {
            let timer = Arc::new(TestTimerRegistration::default());
            self.timers
                .lock()
                .unwrap()
                .push_back((Arc::clone(&timer), callback));
            timer
        }
    }

    #[derive(Default)]
    struct TestTimerRegistration {
        cancelled: AtomicBool,
    }

    impl ComponentTimerRegistration for TestTimerRegistration {
        fn cancel(&self) {
            self.cancelled.store(true, Ordering::Release);
        }
    }

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

    struct ContextParent;

    impl Component for ContextParent {
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
            let _ = context.use_context(&input.context);
            Border::new()
                .content(component::<ContextReader>("reader", input.clone()))
                .into()
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

    #[derive(Clone, Default)]
    struct ServiceProbeInput {
        task: Arc<Mutex<Option<ComponentTask>>>,
        timer: Arc<Mutex<Option<ComponentTimer>>>,
    }

    impl PartialEq for ServiceProbeInput {
        fn eq(&self, other: &Self) -> bool {
            Arc::ptr_eq(&self.task, &other.task) && Arc::ptr_eq(&self.timer, &other.timer)
        }
    }

    enum ServiceProbeMessage {
        Background,
        BackgroundComplete,
        Timer,
        TimerComplete,
    }

    struct ServiceProbe {
        input: ServiceProbeInput,
        value: usize,
    }

    impl Component for ServiceProbe {
        type Input = ServiceProbeInput;
        type Message = ServiceProbeMessage;

        fn create(input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self {
                input: input.clone(),
                value: 0,
            }
        }

        fn update(&mut self, message: Self::Message, context: &ComponentContext<Self::Message>) {
            match message {
                ServiceProbeMessage::Background => {
                    let task =
                        context.spawn_background(|_| ServiceProbeMessage::BackgroundComplete);
                    *self.input.task.lock().unwrap() = Some(task);
                    self.value += 1;
                }
                ServiceProbeMessage::BackgroundComplete => self.value += 1,
                ServiceProbeMessage::Timer => {
                    let timer =
                        context.set_timeout(Duration::ZERO, ServiceProbeMessage::TimerComplete);
                    *self.input.timer.lock().unwrap() = Some(timer);
                    self.value += 1;
                }
                ServiceProbeMessage::TimerComplete => self.value += 1,
            }
        }

        fn view(
            &self,
            _input: &Self::Input,
            _context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            TextBlock::new(self.value.to_string()).into()
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

    struct NestedRoot;

    impl Component for NestedRoot {
        type Input = (f64, bool);
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self
        }

        fn view(
            &self,
            input: &Self::Input,
            _context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            let root = Border::new().canvas_left(input.0);
            if input.1 {
                root.content(component::<RootSwitch>("child", ())).into()
            } else {
                root.into()
            }
        }
    }

    struct CounterParent;

    impl Component for CounterParent {
        type Input = CounterInput;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self
        }

        fn view(
            &self,
            input: &Self::Input,
            _context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            Border::new()
                .content(component::<Counter>("counter", input.clone()))
                .into()
        }
    }

    struct ReorderParent(bool);

    impl Component for ReorderParent {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self(false)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self::Message>) {
            self.0 = !self.0;
        }

        fn view(
            &self,
            _input: &Self::Input,
            _context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            let first = component::<RootSwitch>("first", ()).keyed();
            let second = component::<RootSwitch>("second", ()).keyed();
            if self.0 {
                Grid::new().children([second, first]).into()
            } else {
                Grid::new().children([first, second]).into()
            }
        }
    }

    struct DuplicateNested;

    impl Component for DuplicateNested {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self
        }

        fn view(
            &self,
            _input: &Self::Input,
            _context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            StackPanel::new()
                .children([
                    component::<RootSwitch>("child", ()).into(),
                    component::<RootSwitch>("child", ()).into(),
                ])
                .into()
        }
    }

    struct TreeContent(usize);

    impl Component for TreeContent {
        type Input = Rc<str>;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self(0)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self::Message>) {
            self.0 += 1;
        }

        fn view(
            &self,
            input: &Self::Input,
            _context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            StackPanel::new()
                .children([
                    TextBlock::new(input.clone()).into(),
                    TextBlock::new(self.0.to_string()).into(),
                ])
                .into()
        }
    }

    struct TreeComponents(bool);

    impl Component for TreeComponents {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self(false)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self::Message>) {
            self.0 = !self.0;
        }

        fn view(
            &self,
            _input: &Self::Input,
            _context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            let first = TreeNode::new("first", "First")
                .expanded(true)
                .content(component::<TreeContent>("first-content", Rc::from("First")));
            let second = TreeNode::new("second", "Second")
                .expanded(true)
                .content(component::<TreeContent>(
                    "second-content",
                    Rc::from("Second"),
                ));
            if self.0 {
                TreeView::new().nodes([second, first]).into()
            } else {
                TreeView::new().nodes([first, second]).into()
            }
        }
    }

    struct RecursiveComponent;

    impl Component for RecursiveComponent {
        type Input = usize;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self
        }

        fn view(
            &self,
            input: &Self::Input,
            _context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            if *input == 0 {
                TextBlock::new("leaf").into()
            } else {
                Border::new()
                    .content(component::<Self>("child", *input - 1))
                    .into()
            }
        }
    }

    struct MismatchedKey;

    impl Component for MismatchedKey {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self
        }

        fn view(
            &self,
            _input: &Self::Input,
            _context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            Grid::new()
                .children([keyed("relation", component::<RootSwitch>("component", ()))])
                .into()
        }
    }

    #[derive(Clone)]
    struct RootSwitchEffects(Rc<RefCell<Vec<(&'static str, Option<ObjectId>)>>>);

    impl PartialEq for RootSwitchEffects {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct NestedEffects;

    impl Component for NestedEffects {
        type Input = (RootSwitchEffects, bool);
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self
        }

        fn view(
            &self,
            input: &Self::Input,
            _context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            let root = Border::new();
            if input.1 {
                root.content(component::<EffectRootSwitch>("effect", input.0.clone()))
                    .into()
            } else {
                root.into()
            }
        }
    }

    struct EffectRootSwitch(bool);

    impl Component for EffectRootSwitch {
        type Input = RootSwitchEffects;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self(false)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self::Message>) {
            self.0 = true;
        }

        fn view(
            &self,
            input: &Self::Input,
            context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            let events = Rc::clone(&input.0);
            let reference = context.root();
            context.use_effect("root", self.0, move || {
                events.borrow_mut().push(("setup", reference.get()));
                Some(Box::new(move || {
                    events.borrow_mut().push(("cleanup", reference.get()));
                }))
            });
            if self.0 {
                Border::new().into()
            } else {
                TextBlock::new("Stable").into()
            }
        }
    }

    struct InvalidEffectUpdate(bool);

    impl Component for InvalidEffectUpdate {
        type Input = RootSwitchEffects;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self::Message>) -> Self {
            Self(false)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self::Message>) {
            self.0 = true;
        }

        fn view(
            &self,
            input: &Self::Input,
            context: &mut ComponentViewContext<Self::Message>,
        ) -> Visual {
            let events = Rc::clone(&input.0);
            context.use_effect("root", self.0, move || {
                events.borrow_mut().push(("setup", None));
                Some(Box::new(move || {
                    events.borrow_mut().push(("cleanup", None));
                }))
            });
            if self.0 {
                Grid::new()
                    .children([
                        keyed("duplicate", TextBlock::new("First")),
                        keyed("duplicate", TextBlock::new("Second")),
                    ])
                    .into()
            } else {
                TextBlock::new("Valid").into()
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
    fn context_change_renders_nested_subscribers_once() {
        let context = Rc::new(Context::new(0usize));
        let renders = Arc::new(AtomicUsize::new(0));
        let input = ContextInput {
            context: Rc::clone(&context),
            renders: Arc::clone(&renders),
            subscribe: true,
        };
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<ContextParent>("parent", input)],
        )
        .unwrap();
        assert_eq!(renders.load(Ordering::Relaxed), 2);

        let report = host.set_context(&context, 1).unwrap();
        assert_eq!(report.dispatched, 1);
        assert_eq!(renders.load(Ordering::Relaxed), 4);
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
    fn injected_services_preserve_task_and_timer_statuses() {
        let services = Arc::new(TestServices::default());
        let input = ServiceProbeInput::default();
        let mut host = ComponentHost::mount_with_services(
            RecordingAdapter::default(),
            services.clone(),
            [component::<ServiceProbe>("probe", input.clone())],
        )
        .unwrap();
        let sender = host.sender::<ServiceProbe>(&Key::from("probe")).unwrap();

        assert!(sender.send(ServiceProbeMessage::Background));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(
            input.task.lock().unwrap().as_ref().unwrap().status(),
            ComponentTaskStatus::Running
        );
        services.run_background();
        assert_eq!(
            input.task.lock().unwrap().as_ref().unwrap().status(),
            ComponentTaskStatus::Queued
        );
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(
            input.task.lock().unwrap().as_ref().unwrap().status(),
            ComponentTaskStatus::Delivered
        );

        assert!(sender.send(ServiceProbeMessage::Timer));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(
            input.timer.lock().unwrap().as_ref().unwrap().status(),
            ComponentTaskStatus::Running
        );
        services.fire_timer();
        assert_eq!(
            input.timer.lock().unwrap().as_ref().unwrap().status(),
            ComponentTaskStatus::Queued
        );
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(
            input.timer.lock().unwrap().as_ref().unwrap().status(),
            ComponentTaskStatus::Delivered
        );
    }

    #[test]
    fn retirement_cancels_injected_timer() {
        let services = Arc::new(TestServices::default());
        let input = ServiceProbeInput::default();
        let mut host = ComponentHost::mount_with_services(
            RecordingAdapter::default(),
            services.clone(),
            [component::<ServiceProbe>("probe", input.clone())],
        )
        .unwrap();
        assert!(
            host.sender::<ServiceProbe>(&Key::from("probe"))
                .unwrap()
                .send(ServiceProbeMessage::Timer)
        );
        assert_eq!(host.drain(1).unwrap().dispatched, 1);

        host.remove(&Key::from("probe")).unwrap();
        assert_eq!(
            input.timer.lock().unwrap().as_ref().unwrap().status(),
            ComponentTaskStatus::Cancelled
        );
        services.fire_timer();
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
    fn component_boundary_preserves_scope_across_root_type_changes() {
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
        let switch_reference = host.reference(&Key::from("switch")).unwrap();
        let label_reference = host.reference(&Key::from("label")).unwrap();
        let switch_root = switch_reference.get();
        let label_root = label_reference.get();
        host.runtime_mut().adapter_mut().record_batches(true);

        assert!(switch.send(()));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(switch_reference.get(), switch_root);
        assert_eq!(label_reference.get(), label_root);
        assert!(
            host.runtime()
                .adapter()
                .batches()
                .last()
                .unwrap()
                .iter()
                .any(|mutation| matches!(
                    mutation,
                    Mutation::Replace {
                        object,
                        kind: ObjectType::Border
                    } if Some(*object) == switch_root
                ))
        );

        assert!(label.send(()));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(label_reference.get(), label_root);
    }

    #[test]
    fn nested_component_updates_through_its_retained_owner() {
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<NestedRoot>("parent", (0.0, true))],
        )
        .unwrap();
        let path = [Key::from("parent"), Key::from("child")];
        let sender = host.sender_at::<RootSwitch>(&path).unwrap();
        let reference = host.reference_at(&path).unwrap();
        let root = reference.get().unwrap();
        let parent = host.runtime().graph().owner(root).unwrap();
        assert_eq!(parent.1, RelationId::Content);
        assert_eq!(
            host.runtime().graph().kind(root),
            Some(ObjectType::TextBlock)
        );

        assert!(sender.send(()));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(reference.get(), Some(root));
        assert_eq!(host.runtime().graph().kind(root), Some(ObjectType::Border));
        assert_eq!(host.runtime().graph().owner(root), Some(parent));
    }

    #[test]
    fn parent_rerender_preserves_and_retires_nested_scope() {
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<NestedRoot>("parent", (0.0, true))],
        )
        .unwrap();
        let path = [Key::from("parent"), Key::from("child")];
        let sender = host.sender_at::<RootSwitch>(&path).unwrap();
        let reference = host.reference_at(&path).unwrap();
        let child = reference.get();

        host.update_input::<NestedRoot>(&Key::from("parent"), (12.0, true))
            .unwrap();
        assert_eq!(reference.get(), child);
        assert!(host.sender_at::<RootSwitch>(&path).is_some());

        assert!(sender.send(()));
        host.update_input::<NestedRoot>(&Key::from("parent"), (12.0, false))
            .unwrap();
        assert_eq!(reference.get(), None);
        assert!(host.sender_at::<RootSwitch>(&path).is_none());
        assert_eq!(host.drain(1).unwrap().dropped, 1);

        host.update_input::<NestedRoot>(&Key::from("parent"), (24.0, true))
            .unwrap();
        assert_eq!(host.scopes.len(), 2);
        let replacement = host.sender_at::<RootSwitch>(&path).unwrap();
        assert!(sender.send(()));
        assert_eq!(host.drain(1).unwrap().dropped, 1);
        assert!(replacement.send(()));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
    }

    #[test]
    fn nested_input_update_isolated_to_child_subtree() {
        let cleanup = Arc::new(AtomicUsize::new(0));
        let input = CounterInput {
            cleanup: Arc::clone(&cleanup),
            value: 1,
        };
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<CounterParent>("parent", input)],
        )
        .unwrap();
        let path = [Key::from("parent"), Key::from("counter")];
        let parent = host.reference(&Key::from("parent")).unwrap().get();
        let child = host.reference_at(&path).unwrap().get();
        host.runtime_mut().adapter_mut().record_batches(true);

        host.update_input_at::<Counter>(
            &path,
            CounterInput {
                cleanup: Arc::clone(&cleanup),
                value: 2,
            },
        )
        .unwrap();
        assert_eq!(host.reference(&Key::from("parent")).unwrap().get(), parent);
        assert_eq!(host.reference_at(&path).unwrap().get(), child);
        assert_eq!(cleanup.load(Ordering::Relaxed), 1);
        assert!(
            host.runtime()
                .adapter()
                .batches()
                .last()
                .is_some_and(|batch| batch.iter().all(|mutation| match mutation {
                    Mutation::SetProperties { object, .. } => Some(*object) == child,
                    _ => false,
                }))
        );
    }

    #[test]
    fn nested_keys_are_parent_local_and_stable_through_reorder() {
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [
                component::<ReorderParent>("left", ()),
                component::<NestedRoot>("right", (0.0, true)),
            ],
        )
        .unwrap();
        let first_path = [Key::from("left"), Key::from("first")];
        let second_path = [Key::from("left"), Key::from("second")];
        let right_path = [Key::from("right"), Key::from("child")];
        let first = host.reference_at(&first_path).unwrap().get().unwrap();
        let second = host.reference_at(&second_path).unwrap().get().unwrap();
        assert!(host.reference_at(&right_path).unwrap().get().is_some());
        let parent = host.reference(&Key::from("left")).unwrap().get().unwrap();
        assert_eq!(
            host.runtime()
                .graph()
                .children(parent, RelationId::Children)
                .unwrap(),
            [first, second]
        );

        assert!(
            host.sender::<ReorderParent>(&Key::from("left"))
                .unwrap()
                .send(())
        );
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(
            host.runtime()
                .graph()
                .children(parent, RelationId::Children)
                .unwrap(),
            [second, first]
        );
        assert_eq!(host.reference_at(&first_path).unwrap().get(), Some(first));
        assert_eq!(host.reference_at(&second_path).unwrap().get(), Some(second));
    }

    #[test]
    fn duplicate_nested_keys_are_rejected_per_parent() {
        assert!(matches!(
            ComponentHost::mount(
                RecordingAdapter::default(),
                [component::<DuplicateNested>("parent", ())]
            ),
            Err(ComponentError::DuplicateKey(key)) if key == Key::from("child")
        ));
    }

    #[test]
    fn component_and_relation_keys_cannot_diverge() {
        assert!(matches!(
            ComponentHost::mount(
                RecordingAdapter::default(),
                [component::<MismatchedKey>("parent", ())]
            ),
            Err(ComponentError::ComponentKey {
                component,
                relation
            }) if component == Key::from("component") && relation == Key::from("relation")
        ));
    }

    #[test]
    fn recursive_component_expansion_enforces_depth_limit() {
        assert!(matches!(
            ComponentHost::mount(
                RecordingAdapter::default(),
                [component::<RecursiveComponent>("root", MAX_DEPTH + 2)]
            ),
            Err(ComponentError::Runtime(UpdateError::Graph(
                GraphError::DepthExceeded
            )))
        ));
    }

    #[test]
    fn tree_node_component_content_survives_structural_reorder() {
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<TreeComponents>("tree", ())],
        )
        .unwrap();
        let first_path = [Key::from("tree"), Key::from("first-content")];
        let second_path = [Key::from("tree"), Key::from("second-content")];
        let first = host.reference_at(&first_path).unwrap().get().unwrap();
        let second = host.reference_at(&second_path).unwrap().get().unwrap();
        assert_eq!(
            host.runtime().graph().kind(first),
            Some(ObjectType::StackPanel)
        );
        assert_eq!(
            host.runtime().graph().owner(first).unwrap().1,
            RelationId::Content
        );

        assert!(host.sender_at::<TreeContent>(&first_path).unwrap().send(()));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(host.reference_at(&first_path).unwrap().get(), Some(first));

        assert!(
            host.sender::<TreeComponents>(&Key::from("tree"))
                .unwrap()
                .send(())
        );
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(host.reference_at(&first_path).unwrap().get(), Some(first));
        assert_eq!(host.reference_at(&second_path).unwrap().get(), Some(second));

        assert!(host.sender_at::<TreeContent>(&first_path).unwrap().send(()));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(host.reference_at(&first_path).unwrap().get(), Some(first));
    }

    #[test]
    fn root_effect_cleanup_precedes_replacement_and_setup_follows_it() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<EffectRootSwitch>(
                "switch",
                RootSwitchEffects(Rc::clone(&events)),
            )],
        )
        .unwrap();
        let sender = host
            .sender::<EffectRootSwitch>(&Key::from("switch"))
            .unwrap();
        let reference = host.reference(&Key::from("switch")).unwrap();
        let previous = reference.get();

        assert!(sender.send(()));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        let next = reference.get();

        assert_eq!(previous, next);
        assert_eq!(
            events.borrow().as_slice(),
            [("setup", previous), ("cleanup", previous), ("setup", next)]
        );
    }

    #[test]
    fn nested_effect_cleanup_sees_root_before_removal() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let input = RootSwitchEffects(Rc::clone(&events));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<NestedEffects>("parent", (input.clone(), true))],
        )
        .unwrap();
        let path = [Key::from("parent"), Key::from("effect")];
        let reference = host.reference_at(&path).unwrap();
        let root = reference.get();
        assert_eq!(events.borrow().as_slice(), [("setup", root)]);

        host.update_input::<NestedEffects>(&Key::from("parent"), (input, false))
            .unwrap();
        assert_eq!(
            events.borrow().as_slice(),
            [("setup", root), ("cleanup", root)]
        );
        assert_eq!(reference.get(), None);
    }

    #[test]
    fn invalid_update_preserves_active_effects() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<InvalidEffectUpdate>(
                "invalid",
                RootSwitchEffects(Rc::clone(&events)),
            )],
        )
        .unwrap();
        let sender = host
            .sender::<InvalidEffectUpdate>(&Key::from("invalid"))
            .unwrap();
        let wakes = Arc::new(AtomicUsize::new(0));
        let callback_wakes = Arc::clone(&wakes);
        host.set_waker(move || {
            callback_wakes.fetch_add(1, Ordering::Relaxed);
        });

        assert!(sender.send(()));
        assert!(sender.send(()));
        assert_eq!(wakes.load(Ordering::Relaxed), 1);
        assert!(matches!(
            host.drain(1),
            Err(ComponentError::Runtime(UpdateError::Graph(
                GraphError::DuplicateKey(_)
            )))
        ));
        assert_eq!(wakes.load(Ordering::Relaxed), 2);
        assert_eq!(events.borrow().as_slice(), [("setup", None)]);

        drop(host);
        assert_eq!(
            events.borrow().as_slice(),
            [("setup", None), ("cleanup", None)]
        );
    }
}
