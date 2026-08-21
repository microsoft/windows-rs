use super::arena::NodeId;
use super::runtime::WindowToken;
use super::scope::{ScopeArena, ScopeError, ScopeId, ScopeState};
use crate::element::{Callback, View};
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Weak};

pub(crate) const BACKGROUND_MESSAGE_QUEUE_CAPACITY: usize = 4_096;
pub(crate) const BACKGROUND_TASK_CAPACITY: usize = 64;
pub(crate) const LOCAL_MESSAGE_QUEUE_CAPACITY: usize = 4_096;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EffectKey(EffectKeyKind);

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum EffectKeyKind {
    Integer(u64),
    String(Rc<str>),
}

impl From<u64> for EffectKey {
    fn from(value: u64) -> Self {
        Self(EffectKeyKind::Integer(value))
    }
}

impl From<u32> for EffectKey {
    fn from(value: u32) -> Self {
        Self(EffectKeyKind::Integer(value.into()))
    }
}

impl From<usize> for EffectKey {
    fn from(value: usize) -> Self {
        Self(EffectKeyKind::Integer(u64::try_from(value).unwrap()))
    }
}

impl From<String> for EffectKey {
    fn from(value: String) -> Self {
        Self(EffectKeyKind::String(value.into()))
    }
}

impl From<&str> for EffectKey {
    fn from(value: &str) -> Self {
        Self(EffectKeyKind::String(value.into()))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ContextId(u64);

static NEXT_CONTEXT_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Clone, Debug)]
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

    #[cfg(test)]
    pub(crate) fn id(&self) -> ContextId {
        self.id
    }
}

impl<T> PartialEq for Context<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Context<T> {}

#[derive(Clone)]
pub(crate) struct ContextProvision {
    pub(crate) id: ContextId,
    pub(crate) value: Rc<dyn Any>,
    value_type: TypeId,
    equals: fn(&dyn Any, &dyn Any) -> bool,
}

impl ContextProvision {
    pub(crate) fn new<T: Clone + PartialEq + 'static>(context: &Context<T>, value: T) -> Self {
        Self {
            id: context.id,
            value: Rc::new(value),
            value_type: TypeId::of::<T>(),
            equals: |left, right| {
                left.downcast_ref::<T>()
                    .zip(right.downcast_ref::<T>())
                    .is_some_and(|(left, right)| left == right)
            },
        }
    }
}

impl fmt::Debug for ContextProvision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ContextProvision")
            .field("id", &self.id)
            .field("value_type", &self.value_type)
            .finish()
    }
}

impl PartialEq for ContextProvision {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.value_type == other.value_type
            && (Rc::ptr_eq(&self.value, &other.value)
                || (self.equals)(self.value.as_ref(), other.value.as_ref()))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ContextDependency {
    pub(crate) id: ContextId,
    pub(crate) provider: Option<NodeId>,
}

#[derive(Clone, Default)]
pub(crate) struct ContextSnapshot {
    values: HashMap<ContextId, (NodeId, TypeId, Rc<dyn Any>)>,
}

impl ContextSnapshot {
    pub(crate) fn insert(&mut self, provider: NodeId, provision: &ContextProvision) {
        self.values
            .entry(provision.id)
            .or_insert_with(|| (provider, provision.value_type, Rc::clone(&provision.value)));
    }

    fn get<T: Clone + 'static>(&self, context: &Context<T>) -> Option<(NodeId, T)> {
        let (provider, value_type, value) = self.values.get(&context.id)?;
        assert_eq!(*value_type, TypeId::of::<T>(), "context type mismatch");
        Some((*provider, value.downcast_ref::<T>().unwrap().clone()))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ComponentToken {
    window: WindowToken,
    scope: ScopeId,
}

impl ComponentToken {
    pub(crate) fn scope(self) -> ScopeId {
        self.scope
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComponentStoreError {
    #[cfg(test)]
    ComponentTypeMismatch {
        expected: TypeId,
        actual: TypeId,
    },
    DuplicateEffectKey(EffectKey),
    MessageTypeMismatch {
        expected: TypeId,
        actual: TypeId,
    },
    PropsTypeMismatch {
        expected: TypeId,
        actual: TypeId,
    },
    Scope(ScopeError),
    WindowMismatch,
}

impl From<ScopeError> for ComponentStoreError {
    fn from(value: ScopeError) -> Self {
        Self::Scope(value)
    }
}

struct MessageEnvelope {
    control: Option<Arc<TaskControl>>,
    token: ComponentToken,
    payload: Box<dyn Any>,
}

struct BackgroundEnvelope {
    control: Arc<TaskControl>,
    payload: Box<dyn Any + Send>,
    token: ComponentToken,
}

enum PendingEnvelope {
    Background(BackgroundEnvelope),
    Local(MessageEnvelope),
}

impl PendingEnvelope {
    fn control(&self) -> Option<&Arc<TaskControl>> {
        match self {
            Self::Background(envelope) => Some(&envelope.control),
            Self::Local(envelope) => envelope.control.as_ref(),
        }
    }

    fn token(&self) -> ComponentToken {
        match self {
            Self::Background(envelope) => envelope.token,
            Self::Local(envelope) => envelope.token,
        }
    }
}

struct BackgroundQueue {
    envelopes: VecDeque<BackgroundEnvelope>,
    open: bool,
    tasks: HashMap<ScopeId, Vec<Weak<TaskControl>>>,
    wake: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
    wake_pending: bool,
}

#[derive(Clone)]
struct TaskSpawner {
    limiter: Arc<TaskLimiter>,
    queue: Arc<Mutex<BackgroundQueue>>,
    token: ComponentToken,
}

#[derive(Default)]
struct TaskLimiter {
    active: std::sync::atomic::AtomicUsize,
}

impl TaskLimiter {
    fn acquire(self: &Arc<Self>) -> Option<TaskSlot> {
        self.active
            .try_update(Ordering::AcqRel, Ordering::Acquire, |active| {
                (active < BACKGROUND_TASK_CAPACITY).then_some(active + 1)
            })
            .ok()
            .map(|_| TaskSlot(Arc::clone(self)))
    }
}

struct TaskSlot(Arc<TaskLimiter>);

impl Drop for TaskSlot {
    fn drop(&mut self) {
        self.0.active.fetch_sub(1, Ordering::AcqRel);
    }
}

#[derive(Clone, Debug)]
pub struct CancellationToken {
    control: Arc<TaskControl>,
}

impl CancellationToken {
    pub fn is_cancelled(&self) -> bool {
        self.control.status() == ComponentTaskStatus::Cancelled
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ComponentTaskStatus {
    Running,
    Queued,
    Delivered,
    Cancelled,
    Rejected,
}

#[derive(Debug)]
struct TaskControl {
    status: std::sync::atomic::AtomicU8,
}

impl TaskControl {
    fn new() -> Self {
        Self {
            status: std::sync::atomic::AtomicU8::new(ComponentTaskStatus::Running as u8),
        }
    }

    fn queue(&self) -> bool {
        self.status
            .compare_exchange(
                ComponentTaskStatus::Running as u8,
                ComponentTaskStatus::Queued as u8,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    fn deliver(&self) -> bool {
        self.status
            .compare_exchange(
                ComponentTaskStatus::Queued as u8,
                ComponentTaskStatus::Delivered as u8,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .is_ok()
    }

    fn reject(&self) {
        self.finish(ComponentTaskStatus::Rejected);
    }

    fn finish(&self, status: ComponentTaskStatus) {
        let mut current = self.status.load(Ordering::Acquire);
        while current == ComponentTaskStatus::Running as u8
            || current == ComponentTaskStatus::Queued as u8
        {
            match self.status.compare_exchange_weak(
                current,
                status as u8,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => return,
                Err(actual) => current = actual,
            }
        }
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

    fn cancel(&self) {
        self.finish(ComponentTaskStatus::Cancelled);
    }
}

#[derive(Clone)]
/// A handle for observing or cancelling background work.
///
/// Dropping the handle does not cancel the task. Scope retirement, Pump shutdown, or an explicit
/// [`cancel`](Self::cancel) call cancels it.
pub struct ComponentTask {
    control: Arc<TaskControl>,
    queue: Arc<Mutex<BackgroundQueue>>,
    token: ComponentToken,
}

impl ComponentTask {
    pub fn cancel(&self) {
        self.control.cancel();
        let mut queue = self.queue.lock().unwrap();
        queue
            .envelopes
            .retain(|envelope| !Arc::ptr_eq(&envelope.control, &self.control));
    }

    pub fn is_cancelled(&self) -> bool {
        self.status() == ComponentTaskStatus::Cancelled
    }

    pub fn status(&self) -> ComponentTaskStatus {
        self.control.status()
    }
}

impl fmt::Debug for ComponentTask {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ComponentTask")
            .field("status", &self.status())
            .field("token", &self.token)
            .finish()
    }
}

impl TaskSpawner {
    fn spawn<M, F>(&self, work: F) -> ComponentTask
    where
        M: Send + 'static,
        F: FnOnce(CancellationToken) -> M + Send + 'static,
    {
        let control = Arc::new(TaskControl::new());
        let task = ComponentTask {
            control: Arc::clone(&control),
            queue: Arc::clone(&self.queue),
            token: self.token,
        };
        let Some(slot) = self.limiter.acquire() else {
            control.reject();
            return task;
        };
        {
            let mut queue = self.queue.lock().unwrap();
            if !queue.open {
                control.reject();
                return task;
            }
            queue
                .tasks
                .entry(self.token.scope)
                .or_default()
                .push(Arc::downgrade(&control));
        }
        let queue = Arc::clone(&self.queue);
        let token = self.token;
        let thread_control = Arc::clone(&control);
        let spawn = std::thread::Builder::new()
            .name("windows-reactor-next".to_string())
            .spawn(move || {
                let _slot = slot;
                let message = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    work(CancellationToken {
                        control: Arc::clone(&thread_control),
                    })
                }));
                let wake = {
                    let mut queue = queue.lock().unwrap();
                    let registered = queue.tasks.get_mut(&token.scope).is_some_and(|tasks| {
                        let before = tasks.len();
                        tasks.retain(|task| {
                            task.upgrade()
                                .is_some_and(|task| !Arc::ptr_eq(&task, &thread_control))
                        });
                        tasks.len() != before
                    });
                    if queue.tasks.get(&token.scope).is_some_and(Vec::is_empty) {
                        queue.tasks.remove(&token.scope);
                    }
                    if !registered
                        || thread_control.status() == ComponentTaskStatus::Cancelled
                        || !queue.open
                    {
                        thread_control.cancel();
                        return;
                    }
                    let Ok(message) = message else {
                        thread_control.reject();
                        return;
                    };
                    if queue.envelopes.len() >= BACKGROUND_MESSAGE_QUEUE_CAPACITY {
                        thread_control.reject();
                        return;
                    }
                    if !thread_control.queue() {
                        return;
                    }
                    queue.envelopes.push_back(BackgroundEnvelope {
                        control: Arc::clone(&thread_control),
                        payload: Box::new(message),
                        token,
                    });
                    let wake = (!queue.wake_pending).then(|| queue.wake.clone()).flatten();
                    queue.wake_pending |= wake.is_some();
                    wake
                };
                if let Some(wake) = wake
                    && !wake()
                {
                    let mut queue = queue.lock().unwrap();
                    queue.wake_pending = false;
                    for envelope in queue.envelopes.drain(..) {
                        envelope.control.reject();
                    }
                }
            });
        if spawn.is_err() {
            let mut queue = self.queue.lock().unwrap();
            if let Some(tasks) = queue.tasks.get_mut(&self.token.scope) {
                tasks.retain(|task| {
                    task.upgrade()
                        .is_some_and(|task| !Arc::ptr_eq(&task, &control))
                });
                if tasks.is_empty() {
                    queue.tasks.remove(&self.token.scope);
                }
            }
            control.reject();
        }
        task
    }
}

struct ComponentQueue {
    active: HashSet<ScopeId>,
    envelopes: VecDeque<MessageEnvelope>,
    open: bool,
    wake: Option<Rc<dyn Fn()>>,
}

pub struct LocalSender<M> {
    queue: Rc<RefCell<ComponentQueue>>,
    token: ComponentToken,
    marker: PhantomData<fn(M)>,
}

impl<M> Clone for LocalSender<M> {
    fn clone(&self) -> Self {
        Self {
            queue: Rc::clone(&self.queue),
            token: self.token,
            marker: PhantomData,
        }
    }
}

impl<M: 'static> LocalSender<M> {
    pub fn send(&self, message: M) -> bool {
        let wake = {
            let mut queue = self.queue.borrow_mut();
            if !queue.open
                || !queue.active.contains(&self.token.scope)
                || queue.envelopes.len() >= LOCAL_MESSAGE_QUEUE_CAPACITY
            {
                return false;
            }
            let wake = queue
                .envelopes
                .is_empty()
                .then(|| queue.wake.clone())
                .flatten();
            queue.envelopes.push_back(MessageEnvelope {
                control: None,
                token: self.token,
                payload: Box::new(message),
            });
            wake
        };
        if let Some(wake) = wake {
            wake();
        }
        true
    }

    pub fn callback<T>(&self, map: impl Fn(T) -> M + 'static) -> Callback<T> {
        let sender = self.clone();
        Callback::new_with_acceptance(move |value| sender.send(map(value)))
    }

    pub fn message(&self, message: M) -> Callback<()>
    where
        M: Clone,
    {
        self.callback(move |()| message.clone())
    }
}

pub struct ComponentContext<C: Component> {
    sender: LocalSender<C::Message>,
    tasks: TaskSpawner,
}

impl<C: Component> ComponentContext<C> {
    pub fn sender(&self) -> LocalSender<C::Message> {
        self.sender.clone()
    }

    pub fn spawn_background<F>(&self, work: F) -> ComponentTask
    where
        C::Message: Send,
        F: FnOnce(CancellationToken) -> C::Message + Send + 'static,
    {
        self.tasks.spawn(work)
    }
}

pub struct ViewContext<C: Component> {
    contexts: ContextSnapshot,
    effects: Rc<RefCell<ComponentEffects>>,
    reads: HashSet<ContextDependency>,
    sender: LocalSender<C::Message>,
}

impl<C: Component> ViewContext<C> {
    pub fn sender(&self) -> LocalSender<C::Message> {
        self.sender.clone()
    }

    pub fn callback<T>(&self, map: impl Fn(T) -> C::Message + 'static) -> Callback<T> {
        self.sender.callback(map)
    }

    pub fn message(&self, message: C::Message) -> Callback<()>
    where
        C::Message: Clone,
    {
        self.sender.message(message)
    }

    pub fn use_context<T: Clone + 'static>(&mut self, context: &Context<T>) -> T {
        let resolved = self.contexts.get(context);
        self.reads.insert(ContextDependency {
            id: context.id,
            provider: resolved.as_ref().map(|(provider, _)| *provider),
        });
        resolved.map_or_else(|| context.default.clone(), |(_, value)| value)
    }

    pub fn use_effect<D>(
        &mut self,
        key: impl Into<EffectKey>,
        dependency: D,
        setup: impl FnOnce() -> Option<Box<dyn FnOnce()>> + 'static,
    ) where
        D: PartialEq + 'static,
    {
        self.effects
            .borrow_mut()
            .use_effect(key.into(), dependency, setup);
    }
}

pub trait Component: Sized + 'static {
    type Props: Clone + PartialEq + 'static;
    type Message: 'static;

    fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self;
    fn changed(&mut self, _props: &Self::Props, _context: &mut ComponentContext<Self>) {}
    fn update(&mut self, message: Self::Message, context: &mut ComponentContext<Self>);
    fn view(&self, props: &Self::Props, context: &mut ViewContext<Self>) -> View;
}

trait ErasedComponentFactory {
    fn apply_props(
        &self,
        store: &mut ComponentStore,
        token: ComponentToken,
    ) -> Result<bool, ComponentStoreError>;
    fn as_any(&self) -> &dyn Any;
    fn component_type(&self) -> TypeId;
    fn equals(&self, other: &dyn ErasedComponentFactory) -> bool;
    fn reserve(&self, store: &mut ComponentStore) -> Result<ComponentToken, ComponentStoreError>;
    fn type_name(&self) -> &'static str;
}

struct TypedComponentFactory<C: Component> {
    props: C::Props,
}

impl<C: Component> ErasedComponentFactory for TypedComponentFactory<C> {
    fn apply_props(
        &self,
        store: &mut ComponentStore,
        token: ComponentToken,
    ) -> Result<bool, ComponentStoreError> {
        store.apply_props(token, self.props.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn component_type(&self) -> TypeId {
        TypeId::of::<C>()
    }

    fn equals(&self, other: &dyn ErasedComponentFactory) -> bool {
        other.component_type() == TypeId::of::<C>()
            && other
                .as_any()
                .downcast_ref::<Self>()
                .is_some_and(|other| self.props == other.props)
    }

    fn reserve(&self, store: &mut ComponentStore) -> Result<ComponentToken, ComponentStoreError> {
        store.reserve_component::<C>(self.props.clone())
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<C>()
    }
}

#[derive(Clone)]
pub(crate) struct ComponentView {
    factory: Rc<dyn ErasedComponentFactory>,
}

impl ComponentView {
    pub(crate) fn new<C: Component>(props: C::Props) -> Self {
        Self {
            factory: Rc::new(TypedComponentFactory::<C> { props }),
        }
    }

    pub(crate) fn component_type(&self) -> TypeId {
        self.factory.component_type()
    }

    pub(crate) fn apply_props(
        &self,
        store: &mut ComponentStore,
        token: ComponentToken,
    ) -> Result<bool, ComponentStoreError> {
        self.factory.apply_props(store, token)
    }

    pub(crate) fn reserve(
        &self,
        store: &mut ComponentStore,
    ) -> Result<ComponentToken, ComponentStoreError> {
        self.factory.reserve(store)
    }
}

impl fmt::Debug for ComponentView {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_tuple("Component")
            .field(&self.factory.type_name())
            .finish()
    }
}

impl PartialEq for ComponentView {
    fn eq(&self, other: &Self) -> bool {
        self.factory.equals(&*other.factory)
    }
}

trait ErasedScope {
    fn apply_props(
        &mut self,
        props: Box<dyn Any>,
        tasks: TaskSpawner,
    ) -> Result<bool, ComponentStoreError>;
    #[cfg(test)]
    fn component(&self) -> &dyn Any;
    fn dispatch(
        &mut self,
        message: Box<dyn Any>,
        tasks: TaskSpawner,
    ) -> Result<(), ComponentStoreError>;
    #[cfg(test)]
    fn message_type(&self) -> TypeId;
    fn props_type(&self) -> TypeId;
    fn context_dependencies(&self) -> Option<&HashSet<ContextDependency>>;
    fn set_context_dependencies(&mut self, dependencies: HashSet<ContextDependency>);
    fn view(&self, contexts: ContextSnapshot) -> Result<ComponentRender, ComponentStoreError>;
    fn cleanup_effects(&self);
    fn commit_effects(&self);
    fn prepare_effects(&self);
}

pub(crate) struct ComponentRender {
    pub(crate) dependencies: HashSet<ContextDependency>,
    pub(crate) view: View,
}

type EffectCleanup = Box<dyn FnOnce()>;
type EffectSetup = Box<dyn FnOnce() -> Option<EffectCleanup>>;

struct EffectSlot {
    cleanup: Option<EffectCleanup>,
    dependency: Box<dyn Any>,
    key: EffectKey,
}

struct PendingEffect {
    dependency: Box<dyn Any>,
    setup: EffectSetup,
}

struct EffectRegistration {
    key: EffectKey,
    pending: Option<PendingEffect>,
}

#[derive(Default)]
pub(crate) struct ComponentEffects {
    duplicate: Option<EffectKey>,
    registrations: Vec<EffectRegistration>,
    slots: Vec<EffectSlot>,
}

impl ComponentEffects {
    fn begin_view(&mut self) {
        self.duplicate = None;
        self.registrations.clear();
    }

    fn use_effect<D>(
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
            if self.duplicate.is_none() {
                self.duplicate = Some(key);
            }
            return;
        }
        let changed = self
            .slots
            .iter()
            .find(|slot| slot.key == key)
            .and_then(|slot| slot.dependency.downcast_ref::<D>())
            != Some(&dependency);
        let pending = if changed {
            Some(PendingEffect {
                dependency: Box::new(dependency),
                setup: Box::new(setup),
            })
        } else {
            None
        };
        self.registrations.push(EffectRegistration { key, pending });
    }

    fn finish_view(&self) -> Result<(), ComponentStoreError> {
        match &self.duplicate {
            Some(key) => Err(ComponentStoreError::DuplicateEffectKey(key.clone())),
            None => Ok(()),
        }
    }

    fn prepare(&mut self) {
        for slot in &mut self.slots {
            let cleanup_required = self
                .registrations
                .iter()
                .find(|registration| registration.key == slot.key)
                .is_none_or(|registration| registration.pending.is_some());
            if cleanup_required && let Some(cleanup) = slot.cleanup.take() {
                cleanup();
            }
        }
    }

    fn commit(&mut self) {
        let mut published = std::mem::take(&mut self.slots);
        for registration in self.registrations.drain(..) {
            let slot = if let Some(pending) = registration.pending {
                EffectSlot {
                    cleanup: (pending.setup)(),
                    dependency: pending.dependency,
                    key: registration.key,
                }
            } else {
                let index = published
                    .iter()
                    .position(|slot| slot.key == registration.key)
                    .unwrap();
                published.remove(index)
            };
            self.slots.push(slot);
        }
        self.duplicate = None;
    }

    fn cleanup(&mut self) {
        for slot in self.slots.iter_mut().rev() {
            if let Some(cleanup) = slot.cleanup.take() {
                cleanup();
            }
        }
        self.slots.clear();
        self.registrations.clear();
        self.duplicate = None;
    }
}

struct TypedScope<C, P, M> {
    component: C,
    context_dependencies: Option<Rc<HashSet<ContextDependency>>>,
    effects: Rc<RefCell<ComponentEffects>>,
    props: P,
    changed: fn(&mut C, &P, LocalSender<M>, TaskSpawner),
    sender: LocalSender<M>,
    update: fn(&mut C, M, LocalSender<M>, TaskSpawner),
    view: fn(
        &C,
        &P,
        LocalSender<M>,
        Rc<RefCell<ComponentEffects>>,
        ContextSnapshot,
    ) -> ComponentRender,
}

impl<C, P, M> Drop for TypedScope<C, P, M> {
    fn drop(&mut self) {
        self.effects.borrow_mut().cleanup();
    }
}

impl<C, P, M> ErasedScope for TypedScope<C, P, M>
where
    C: 'static,
    P: Clone + PartialEq + 'static,
    M: 'static,
{
    fn apply_props(
        &mut self,
        props: Box<dyn Any>,
        tasks: TaskSpawner,
    ) -> Result<bool, ComponentStoreError> {
        let actual = props.as_ref().type_id();
        let props = props
            .downcast::<P>()
            .map_err(|_| ComponentStoreError::PropsTypeMismatch {
                expected: TypeId::of::<P>(),
                actual,
            })?;
        if self.props == *props {
            return Ok(false);
        }
        self.props = *props;
        (self.changed)(&mut self.component, &self.props, self.sender.clone(), tasks);
        Ok(true)
    }

    #[cfg(test)]
    fn component(&self) -> &dyn Any {
        &self.component
    }

    fn context_dependencies(&self) -> Option<&HashSet<ContextDependency>> {
        self.context_dependencies.as_deref()
    }

    fn set_context_dependencies(&mut self, dependencies: HashSet<ContextDependency>) {
        self.context_dependencies = (!dependencies.is_empty()).then(|| Rc::new(dependencies));
    }

    fn dispatch(
        &mut self,
        message: Box<dyn Any>,
        tasks: TaskSpawner,
    ) -> Result<(), ComponentStoreError> {
        let actual = message.as_ref().type_id();
        let message =
            message
                .downcast::<M>()
                .map_err(|_| ComponentStoreError::MessageTypeMismatch {
                    expected: TypeId::of::<M>(),
                    actual,
                })?;
        (self.update)(&mut self.component, *message, self.sender.clone(), tasks);
        Ok(())
    }

    #[cfg(test)]
    fn message_type(&self) -> TypeId {
        TypeId::of::<M>()
    }

    fn props_type(&self) -> TypeId {
        TypeId::of::<P>()
    }

    fn view(&self, contexts: ContextSnapshot) -> Result<ComponentRender, ComponentStoreError> {
        self.effects.borrow_mut().begin_view();
        let render = (self.view)(
            &self.component,
            &self.props,
            self.sender.clone(),
            Rc::clone(&self.effects),
            contexts,
        );
        self.effects.borrow().finish_view()?;
        Ok(render)
    }

    fn cleanup_effects(&self) {
        self.effects.borrow_mut().cleanup();
    }

    fn commit_effects(&self) {
        self.effects.borrow_mut().commit();
    }

    fn prepare_effects(&self) {
        self.effects.borrow_mut().prepare();
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DrainReport {
    pub blocked: bool,
    pub dispatched: usize,
    pub dropped: usize,
    pub(crate) dirty: Vec<ComponentToken>,
}

pub struct ComponentStore {
    background: Arc<Mutex<BackgroundQueue>>,
    context_consumers: HashMap<ContextDependency, HashSet<ScopeId>>,
    context_consumers_by_id: HashMap<ContextId, HashSet<ScopeId>>,
    drain_background_next: bool,
    window: WindowToken,
    scopes: ScopeArena<Box<dyn ErasedScope>>,
    task_limiter: Arc<TaskLimiter>,
    queue: Rc<RefCell<ComponentQueue>>,
}

impl ComponentStore {
    pub fn new(window: WindowToken) -> Self {
        Self::with_task_limiter(window, Arc::new(TaskLimiter::default()))
    }

    fn with_task_limiter(window: WindowToken, task_limiter: Arc<TaskLimiter>) -> Self {
        Self {
            background: Arc::new(Mutex::new(BackgroundQueue {
                envelopes: VecDeque::new(),
                open: true,
                tasks: HashMap::new(),
                wake: None,
                wake_pending: false,
            })),
            context_consumers: HashMap::new(),
            context_consumers_by_id: HashMap::new(),
            drain_background_next: false,
            window,
            scopes: ScopeArena::new(),
            task_limiter,
            queue: Rc::new(RefCell::new(ComponentQueue {
                active: HashSet::new(),
                envelopes: VecDeque::new(),
                open: true,
                wake: None,
            })),
        }
    }

    pub fn reserve_component<C: Component>(
        &mut self,
        props: C::Props,
    ) -> Result<ComponentToken, ComponentStoreError> {
        fn changed<C: Component>(
            component: &mut C,
            props: &C::Props,
            sender: LocalSender<C::Message>,
            tasks: TaskSpawner,
        ) {
            component.changed(props, &mut ComponentContext { sender, tasks });
        }

        fn update<C: Component>(
            component: &mut C,
            message: C::Message,
            sender: LocalSender<C::Message>,
            tasks: TaskSpawner,
        ) {
            component.update(message, &mut ComponentContext { sender, tasks });
        }

        fn view<C: Component>(
            component: &C,
            props: &C::Props,
            sender: LocalSender<C::Message>,
            effects: Rc<RefCell<ComponentEffects>>,
            contexts: ContextSnapshot,
        ) -> ComponentRender {
            let mut context = ViewContext {
                contexts,
                effects,
                reads: HashSet::new(),
                sender,
            };
            let view = component.view(props, &mut context);
            ComponentRender {
                dependencies: context.reads,
                view,
            }
        }

        let background = Arc::clone(&self.background);
        let queue = Rc::clone(&self.queue);
        let task_limiter = Arc::clone(&self.task_limiter);
        let window = self.window;
        let scope = self.scopes.reserve_with(move |scope| {
            queue.borrow_mut().active.insert(scope);
            let sender = LocalSender {
                queue: Rc::clone(&queue),
                token: ComponentToken { window, scope },
                marker: PhantomData,
            };
            let tasks = TaskSpawner {
                limiter: Arc::clone(&task_limiter),
                queue: Arc::clone(&background),
                token: ComponentToken { window, scope },
            };
            let component = C::create(
                &props,
                &mut ComponentContext {
                    sender: sender.clone(),
                    tasks,
                },
            );
            Box::new(TypedScope {
                component,
                context_dependencies: None,
                effects: Rc::new(RefCell::new(ComponentEffects::default())),
                props,
                changed: changed::<C>,
                sender,
                update: update::<C>,
                view: view::<C>,
            }) as Box<dyn ErasedScope>
        })?;
        Ok(ComponentToken {
            window: self.window,
            scope,
        })
    }

    pub fn publish(&mut self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        self.validate_window(token)?;
        self.scopes.publish(token.scope)?;
        Ok(())
    }

    pub(crate) fn restarted(&self, window: WindowToken) -> Self {
        Self::with_task_limiter(window, Arc::clone(&self.task_limiter))
    }

    pub fn remove(&mut self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        self.validate_window(token)?;
        self.clear_context_dependencies(token.scope)?;
        self.scopes.remove(token.scope)?;
        self.cancel_scope_tasks(token.scope);
        self.remove_scope_messages(token.scope);
        Ok(())
    }

    #[cfg(test)]
    pub fn sender<M: 'static>(
        &self,
        token: ComponentToken,
    ) -> Result<LocalSender<M>, ComponentStoreError> {
        self.validate_window(token)?;
        let scope = self.scopes.get(token.scope)?;
        let actual = TypeId::of::<M>();
        let expected = scope.message_type();
        if actual != expected {
            return Err(ComponentStoreError::MessageTypeMismatch { expected, actual });
        }
        Ok(LocalSender {
            queue: Rc::clone(&self.queue),
            token,
            marker: PhantomData,
        })
    }

    pub fn apply_props<P: 'static>(
        &mut self,
        token: ComponentToken,
        props: P,
    ) -> Result<bool, ComponentStoreError> {
        self.validate_window(token)?;
        let tasks = self.task_spawner(token);
        let scope = self.scopes.get_mut(token.scope)?;
        let actual = TypeId::of::<P>();
        let expected = scope.props_type();
        if actual != expected {
            return Err(ComponentStoreError::PropsTypeMismatch { expected, actual });
        }
        scope.apply_props(Box::new(props), tasks)
    }

    #[cfg(test)]
    pub fn component<C: 'static>(&self, token: ComponentToken) -> Result<&C, ComponentStoreError> {
        self.validate_window(token)?;
        let component = self.scopes.get(token.scope)?.component();
        let actual = component.type_id();
        component
            .downcast_ref()
            .ok_or(ComponentStoreError::ComponentTypeMismatch {
                expected: TypeId::of::<C>(),
                actual,
            })
    }

    pub fn drain(&mut self, budget: usize) -> Result<DrainReport, ComponentStoreError> {
        let mut report = DrainReport::default();
        self.background.lock().unwrap().wake_pending = false;
        for _ in 0..budget {
            let pop_background = || {
                self.background
                    .lock()
                    .unwrap()
                    .envelopes
                    .pop_front()
                    .map(PendingEnvelope::Background)
            };
            let pop_local = || {
                self.queue
                    .borrow_mut()
                    .envelopes
                    .pop_front()
                    .map(PendingEnvelope::Local)
            };
            let envelope = if self.drain_background_next {
                pop_background().or_else(pop_local)
            } else {
                pop_local().or_else(pop_background)
            };
            let Some(envelope) = envelope else {
                break;
            };
            let from_background = matches!(envelope, PendingEnvelope::Background(_));
            self.drain_background_next = !from_background;
            let token = envelope.token();
            if token.window != self.window {
                if let Some(control) = envelope.control() {
                    control.cancel();
                }
                report.dropped += 1;
                continue;
            }
            let state = match self.scopes.state(token.scope) {
                Ok(state) => state,
                Err(ScopeError::Stale(_)) => {
                    if let Some(control) = envelope.control() {
                        control.cancel();
                    }
                    report.dropped += 1;
                    continue;
                }
                Err(error) => return Err(error.into()),
            };
            match state {
                ScopeState::Reserved => {
                    match envelope {
                        PendingEnvelope::Background(envelope) => self
                            .background
                            .lock()
                            .unwrap()
                            .envelopes
                            .push_front(envelope),
                        PendingEnvelope::Local(envelope) => {
                            self.queue.borrow_mut().envelopes.push_front(envelope);
                        }
                    }
                    report.blocked = true;
                    break;
                }
                ScopeState::Published => {
                    if let Some(control) = envelope.control()
                        && !control.deliver()
                    {
                        report.dropped += 1;
                        continue;
                    }
                    let payload: Box<dyn Any> = match envelope {
                        PendingEnvelope::Background(envelope) => envelope.payload,
                        PendingEnvelope::Local(envelope) => envelope.payload,
                    };
                    let tasks = self.task_spawner(token);
                    self.scopes.get_mut(token.scope)?.dispatch(payload, tasks)?;
                    report.dispatched += 1;
                    report.dirty.push(token);
                }
            }
        }
        Ok(report)
    }

    pub fn pending(&self) -> usize {
        self.queue.borrow().envelopes.len() + self.background.lock().unwrap().envelopes.len()
    }

    pub(crate) fn pending_tokens(&self) -> Vec<ComponentToken> {
        let mut tokens = self
            .queue
            .borrow()
            .envelopes
            .iter()
            .map(|envelope| envelope.token)
            .collect::<Vec<_>>();
        tokens.extend(
            self.background
                .lock()
                .unwrap()
                .envelopes
                .iter()
                .map(|envelope| envelope.token),
        );
        tokens
    }

    pub(crate) fn next_pending_token(&self) -> Option<ComponentToken> {
        let local = self
            .queue
            .borrow()
            .envelopes
            .front()
            .map(|envelope| envelope.token);
        let background = self
            .background
            .lock()
            .unwrap()
            .envelopes
            .front()
            .map(|envelope| envelope.token);
        if self.drain_background_next {
            background.or(local)
        } else {
            local.or(background)
        }
    }

    pub(crate) fn context_dependencies(
        &self,
        token: ComponentToken,
    ) -> Result<Option<&HashSet<ContextDependency>>, ComponentStoreError> {
        self.validate_window(token)?;
        Ok(self.scopes.get(token.scope)?.context_dependencies())
    }

    pub(crate) fn set_context_dependencies(
        &mut self,
        token: ComponentToken,
        dependencies: HashSet<ContextDependency>,
    ) -> Result<(), ComponentStoreError> {
        self.validate_window(token)?;
        let previous = self
            .scopes
            .get(token.scope)?
            .context_dependencies()
            .cloned()
            .unwrap_or_default();
        for dependency in previous.difference(&dependencies) {
            self.remove_context_consumer(*dependency, token.scope);
        }
        for dependency in dependencies.difference(&previous).copied() {
            self.context_consumers
                .entry(dependency)
                .or_default()
                .insert(token.scope);
            self.context_consumers_by_id
                .entry(dependency.id)
                .or_default()
                .insert(token.scope);
        }
        self.scopes
            .get_mut(token.scope)?
            .set_context_dependencies(dependencies);
        Ok(())
    }

    pub(crate) fn context_consumers(
        &self,
        dependency: ContextDependency,
    ) -> impl Iterator<Item = ScopeId> + '_ {
        self.context_consumers
            .get(&dependency)
            .into_iter()
            .flatten()
            .copied()
    }

    pub(crate) fn context_consumers_for_id(
        &self,
        id: ContextId,
    ) -> impl Iterator<Item = ScopeId> + '_ {
        self.context_consumers_by_id
            .get(&id)
            .into_iter()
            .flatten()
            .copied()
    }

    pub(crate) fn view(
        &self,
        token: ComponentToken,
        contexts: ContextSnapshot,
    ) -> Result<ComponentRender, ComponentStoreError> {
        self.validate_window(token)?;
        self.scopes.get(token.scope)?.view(contexts)
    }

    pub fn cleanup_effects(&self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        self.validate_window(token)?;
        self.scopes.get(token.scope)?.cleanup_effects();
        Ok(())
    }

    pub fn commit_effects(&self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        self.validate_window(token)?;
        self.scopes.get(token.scope)?.commit_effects();
        Ok(())
    }

    pub fn prepare_effects(&self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        self.validate_window(token)?;
        self.scopes.get(token.scope)?.prepare_effects();
        Ok(())
    }

    pub(crate) fn token(&self, scope: ScopeId) -> Result<ComponentToken, ComponentStoreError> {
        self.scopes.state(scope)?;
        Ok(ComponentToken {
            window: self.window,
            scope,
        })
    }

    pub(crate) fn set_waker(&mut self, wake: Rc<dyn Fn()>) {
        self.queue.borrow_mut().wake = Some(wake);
    }

    pub(crate) fn set_background_waker(&mut self, wake: Arc<dyn Fn() -> bool + Send + Sync>) {
        self.background.lock().unwrap().wake = Some(wake);
    }

    pub(crate) fn close(&mut self) {
        {
            let mut queue = self.queue.borrow_mut();
            queue.open = false;
            queue.active.clear();
            for envelope in queue.envelopes.drain(..) {
                if let Some(control) = envelope.control {
                    control.cancel();
                }
            }
            queue.wake = None;
        }
        let mut background = self.background.lock().unwrap();
        background.open = false;
        for envelope in background.envelopes.drain(..) {
            envelope.control.cancel();
        }
        background.wake = None;
        background.wake_pending = false;
        for tasks in background.tasks.values() {
            for control in tasks.iter().filter_map(Weak::upgrade) {
                control.cancel();
            }
        }
        background.tasks.clear();
        self.context_consumers.clear();
        self.context_consumers_by_id.clear();
    }

    fn cancel_scope_tasks(&mut self, scope: ScopeId) {
        let mut background = self.background.lock().unwrap();
        if let Some(tasks) = background.tasks.remove(&scope) {
            for control in tasks.iter().filter_map(Weak::upgrade) {
                control.cancel();
            }
        }
        for envelope in background
            .envelopes
            .iter()
            .filter(|envelope| envelope.token.scope == scope)
        {
            envelope.control.cancel();
        }
        background
            .envelopes
            .retain(|envelope| envelope.token.scope != scope);
    }

    fn task_spawner(&self, token: ComponentToken) -> TaskSpawner {
        TaskSpawner {
            limiter: Arc::clone(&self.task_limiter),
            queue: Arc::clone(&self.background),
            token,
        }
    }

    fn remove_scope_messages(&mut self, scope: ScopeId) {
        let mut queue = self.queue.borrow_mut();
        queue.active.remove(&scope);
        for envelope in queue
            .envelopes
            .iter()
            .filter(|envelope| envelope.token.scope == scope)
        {
            if let Some(control) = &envelope.control {
                control.cancel();
            }
        }
        queue
            .envelopes
            .retain(|envelope| envelope.token.scope != scope);
    }

    fn clear_context_dependencies(&mut self, scope: ScopeId) -> Result<(), ComponentStoreError> {
        self.set_context_dependencies(
            ComponentToken {
                window: self.window,
                scope,
            },
            HashSet::new(),
        )
    }

    fn remove_context_consumer(&mut self, dependency: ContextDependency, scope: ScopeId) {
        if let Some(consumers) = self.context_consumers.get_mut(&dependency) {
            consumers.remove(&scope);
            if consumers.is_empty() {
                self.context_consumers.remove(&dependency);
            }
        }
        if let Some(consumers) = self.context_consumers_by_id.get_mut(&dependency.id) {
            consumers.remove(&scope);
            if consumers.is_empty() {
                self.context_consumers_by_id.remove(&dependency.id);
            }
        }
    }

    fn validate_window(&self, token: ComponentToken) -> Result<(), ComponentStoreError> {
        if token.window == self.window {
            Ok(())
        } else {
            Err(ComponentStoreError::WindowMismatch)
        }
    }
}

impl Drop for ComponentStore {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TextBlock;
    use crate::core::{WindowId, WindowToken};
    use std::cell::Cell;
    use std::sync::Barrier;
    use std::sync::atomic::AtomicUsize;
    use std::time::{Duration, Instant};

    struct State {
        changed: usize,
        sender: Option<LocalSender<u32>>,
        value: u32,
    }

    impl Component for State {
        type Props = String;
        type Message = u32;

        fn create(_props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
            Self {
                changed: 0,
                sender: Some(context.sender()),
                value: 0,
            }
        }

        fn changed(&mut self, _props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.changed += 1;
        }

        fn update(&mut self, message: u32, _context: &mut ComponentContext<Self>) {
            self.value += message;
            if message == 1 {
                self.sender.as_ref().unwrap().send(2);
            }
        }

        fn view(&self, _props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
            View::empty()
        }
    }

    fn store() -> ComponentStore {
        ComponentStore::new(WindowToken::new(WindowId::allocate()))
    }

    fn reserve_state(store: &mut ComponentStore, props: &str) -> ComponentToken {
        store.reserve_component::<State>(props.to_string()).unwrap()
    }

    struct DirectProps;

    impl Component for DirectProps {
        type Props = String;
        type Message = ();

        fn create(_props: &Self::Props, _context: &mut ComponentContext<Self>) -> Self {
            Self
        }

        fn update(&mut self, _message: Self::Message, _context: &mut ComponentContext<Self>) {}

        fn view(&self, props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
            View::native(TextBlock::new().text(props.clone()))
        }
    }

    #[test]
    fn view_receives_current_store_owned_props() {
        let mut store = store();
        let token = store
            .reserve_component::<DirectProps>("first".to_string())
            .unwrap();
        store.publish(token).unwrap();

        assert_eq!(
            store.view(token, ContextSnapshot::default()).unwrap().view,
            View::native(TextBlock::new().text("first"))
        );
        assert!(store.apply_props(token, "second".to_string()).unwrap());
        assert_eq!(
            store.view(token, ContextSnapshot::default()).unwrap().view,
            View::native(TextBlock::new().text("second"))
        );
    }

    fn wait_for_status(task: &ComponentTask, status: ComponentTaskStatus) {
        let deadline = Instant::now() + Duration::from_secs(2);
        while task.status() != status {
            assert!(Instant::now() < deadline, "background task timed out");
            std::thread::sleep(Duration::from_millis(1));
        }
    }

    #[test]
    fn reserved_messages_wait_for_publication_and_reentrant_messages_queue() {
        let mut store = store();
        let token = reserve_state(&mut store, "first");
        let sender = store.sender::<u32>(token).unwrap();
        sender.send(1);

        assert_eq!(
            store.drain(10),
            Ok(DrainReport {
                blocked: true,
                dispatched: 0,
                dropped: 0,
                dirty: Vec::new(),
            })
        );
        store.publish(token).unwrap();
        assert_eq!(store.drain(10).unwrap().dispatched, 2);
        assert_eq!(store.component::<State>(token).unwrap().value, 3);
    }

    #[test]
    fn payload_callback_maps_into_the_component_queue() {
        let mut store = store();
        let token = reserve_state(&mut store, "");
        store.publish(token).unwrap();
        let callback = store
            .sender::<u32>(token)
            .unwrap()
            .callback(|value: String| value.len() as u32);

        assert!(callback.call("mapped".to_string()));
        assert_eq!(store.pending(), 1);
        assert_eq!(store.drain(1).unwrap().dispatched, 1);
        assert_eq!(store.component::<State>(token).unwrap().value, 6);
    }

    struct RepeatedMessage {
        clones: Rc<Cell<usize>>,
        value: u32,
    }

    impl Clone for RepeatedMessage {
        fn clone(&self) -> Self {
            self.clones.set(self.clones.get() + 1);
            Self {
                clones: Rc::clone(&self.clones),
                value: self.value,
            }
        }
    }

    struct RepeatedState {
        value: u32,
    }

    impl Component for RepeatedState {
        type Props = ();
        type Message = RepeatedMessage;

        fn create(_props: &(), _context: &mut ComponentContext<Self>) -> Self {
            Self { value: 0 }
        }

        fn update(&mut self, message: RepeatedMessage, _context: &mut ComponentContext<Self>) {
            self.value += message.value;
        }

        fn view(&self, _props: &(), _context: &mut ViewContext<Self>) -> View {
            View::empty()
        }
    }

    #[test]
    fn unit_message_callback_clones_for_every_delivery() {
        let mut store = store();
        let token = store.reserve_component::<RepeatedState>(()).unwrap();
        store.publish(token).unwrap();
        let clones = Rc::new(Cell::new(0));
        let callback = store
            .sender::<RepeatedMessage>(token)
            .unwrap()
            .message(RepeatedMessage {
                clones: Rc::clone(&clones),
                value: 7,
            });

        assert!(callback.call(()));
        assert!(callback.call(()));
        assert_eq!(clones.get(), 2);
        assert_eq!(store.drain(2).unwrap().dispatched, 2);
        assert_eq!(store.component::<RepeatedState>(token).unwrap().value, 14);
    }

    #[test]
    fn props_are_typed_coalesced_and_applied_before_messages() {
        let mut store = store();
        let token = reserve_state(&mut store, "first");
        store.publish(token).unwrap();

        assert_eq!(store.apply_props(token, "first".to_string()), Ok(false));
        assert_eq!(store.apply_props(token, "second".to_string()), Ok(true));
        assert_eq!(store.component::<State>(token).unwrap().changed, 1);
        assert!(matches!(
            store.apply_props(token, 1u32),
            Err(ComponentStoreError::PropsTypeMismatch { .. })
        ));
    }

    #[test]
    fn stale_tokens_cannot_reach_a_reused_slot() {
        let mut store = store();
        let first = reserve_state(&mut store, "");
        store.publish(first).unwrap();
        let old_sender = store.sender::<u32>(first).unwrap();
        store.remove(first).unwrap();
        old_sender.send(1);
        assert_eq!(store.drain(10).unwrap().dropped, 0);

        let second = reserve_state(&mut store, "");
        assert_ne!(first, second);
        store.publish(second).unwrap();
        old_sender.send(4);

        assert_eq!(store.drain(10).unwrap().dropped, 0);
        assert_eq!(store.component::<State>(second).unwrap().value, 0);
    }

    #[test]
    fn sender_creation_checks_the_message_type() {
        let mut store = store();
        let token = reserve_state(&mut store, "");

        assert!(matches!(
            store.sender::<String>(token),
            Err(ComponentStoreError::MessageTypeMismatch { .. })
        ));
    }

    #[test]
    fn background_queue_capacity_rejects_excess_completion() {
        let mut store = store();
        let token = reserve_state(&mut store, "");
        store.publish(token).unwrap();
        {
            let mut queue = store.background.lock().unwrap();
            for _ in 0..BACKGROUND_MESSAGE_QUEUE_CAPACITY {
                queue.envelopes.push_back(BackgroundEnvelope {
                    control: Arc::new(TaskControl::new()),
                    payload: Box::new(0u32),
                    token,
                });
            }
        }

        let task = TaskSpawner {
            limiter: Arc::clone(&store.task_limiter),
            queue: Arc::clone(&store.background),
            token,
        }
        .spawn::<u32, _>(|_| 1);
        wait_for_status(&task, ComponentTaskStatus::Rejected);
        assert_eq!(store.background.lock().unwrap().envelopes.len(), 4096);
    }

    #[test]
    fn panicking_background_work_is_rejected() {
        let mut store = store();
        let token = reserve_state(&mut store, "");
        store.publish(token).unwrap();

        let task = TaskSpawner {
            limiter: Arc::clone(&store.task_limiter),
            queue: Arc::clone(&store.background),
            token,
        }
        .spawn::<u32, _>(|_| panic!("injected task panic"));
        wait_for_status(&task, ComponentTaskStatus::Rejected);
        assert_eq!(store.pending(), 0);
    }

    #[test]
    fn dropping_task_handle_does_not_cancel_delivery() {
        let mut store = store();
        let token = reserve_state(&mut store, "");
        store.publish(token).unwrap();

        drop(
            TaskSpawner {
                limiter: Arc::clone(&store.task_limiter),
                queue: Arc::clone(&store.background),
                token,
            }
            .spawn::<u32, _>(|_| 7),
        );
        let deadline = Instant::now() + Duration::from_secs(2);
        while store.pending() == 0 {
            assert!(Instant::now() < deadline, "background task timed out");
            std::thread::sleep(Duration::from_millis(1));
        }
        assert_eq!(store.drain(1).unwrap().dispatched, 1);
        assert_eq!(store.component::<State>(token).unwrap().value, 7);
    }

    #[test]
    fn failed_wake_rejects_every_completion_waiting_behind_it() {
        let mut store = store();
        let token = reserve_state(&mut store, "");
        store.publish(token).unwrap();
        let wake_barrier = Arc::new(Barrier::new(2));
        let wakes = Arc::new(AtomicUsize::new(0));
        {
            let wake_barrier = Arc::clone(&wake_barrier);
            let wakes = Arc::clone(&wakes);
            store.set_background_waker(Arc::new(move || {
                wakes.fetch_add(1, Ordering::AcqRel);
                wake_barrier.wait();
                false
            }));
        }
        let work_barrier = Arc::new(Barrier::new(3));
        let spawn = TaskSpawner {
            limiter: Arc::clone(&store.task_limiter),
            queue: Arc::clone(&store.background),
            token,
        };
        let first_barrier = Arc::clone(&work_barrier);
        let first = spawn.spawn::<u32, _>(move |_| {
            first_barrier.wait();
            1
        });
        let second_barrier = Arc::clone(&work_barrier);
        let second = spawn.spawn::<u32, _>(move |_| {
            second_barrier.wait();
            2
        });
        work_barrier.wait();
        let deadline = Instant::now() + Duration::from_secs(2);
        while store.background.lock().unwrap().envelopes.len() != 2 {
            assert!(Instant::now() < deadline, "background tasks timed out");
            std::thread::sleep(Duration::from_millis(1));
        }
        wake_barrier.wait();

        wait_for_status(&first, ComponentTaskStatus::Rejected);
        wait_for_status(&second, ComponentTaskStatus::Rejected);
        assert_eq!(wakes.load(Ordering::Acquire), 1);
        assert_eq!(store.pending(), 0);
    }

    #[test]
    fn cancelled_reserved_completion_cannot_deliver_after_publication() {
        let mut store = store();
        let token = reserve_state(&mut store, "");
        let task = TaskSpawner {
            limiter: Arc::clone(&store.task_limiter),
            queue: Arc::clone(&store.background),
            token,
        }
        .spawn::<u32, _>(|_| 4);
        wait_for_status(&task, ComponentTaskStatus::Queued);
        assert!(store.drain(1).unwrap().blocked);

        task.cancel();
        store.publish(token).unwrap();
        assert_eq!(store.drain(1).unwrap().dispatched, 0);
        assert_eq!(store.component::<State>(token).unwrap().value, 0);
    }

    #[test]
    fn local_and_background_messages_alternate() {
        let mut store = store();
        let token = reserve_state(&mut store, "");
        store.publish(token).unwrap();
        let sender = store.sender::<u32>(token).unwrap();
        assert!(sender.send(3));
        let control = Arc::new(TaskControl::new());
        control.queue();
        store
            .background
            .lock()
            .unwrap()
            .envelopes
            .push_back(BackgroundEnvelope {
                control,
                payload: Box::new(10u32),
                token,
            });

        assert_eq!(store.drain(1).unwrap().dispatched, 1);
        assert_eq!(store.component::<State>(token).unwrap().value, 3);
        assert!(sender.send(100));
        assert_eq!(store.drain(1).unwrap().dispatched, 1);
        assert_eq!(store.component::<State>(token).unwrap().value, 13);
        assert_eq!(store.pending(), 1);
    }

    #[test]
    fn stale_background_completion_cannot_reach_a_reused_scope() {
        let mut store = store();
        let first = reserve_state(&mut store, "");
        store.publish(first).unwrap();
        store.remove(first).unwrap();
        let second = reserve_state(&mut store, "");
        store.publish(second).unwrap();
        let control = Arc::new(TaskControl::new());
        control.queue();
        store
            .background
            .lock()
            .unwrap()
            .envelopes
            .push_back(BackgroundEnvelope {
                control: Arc::clone(&control),
                payload: Box::new(8u32),
                token: first,
            });

        assert_eq!(store.drain(1).unwrap().dropped, 1);
        assert_eq!(control.status(), ComponentTaskStatus::Cancelled);
        assert_eq!(store.component::<State>(second).unwrap().value, 0);
    }

    #[test]
    fn closed_store_and_live_task_limit_reject_without_spawning() {
        let mut store = store();
        let token = reserve_state(&mut store, "");
        let spawn = TaskSpawner {
            limiter: Arc::clone(&store.task_limiter),
            queue: Arc::clone(&store.background),
            token,
        };
        store
            .task_limiter
            .active
            .store(BACKGROUND_TASK_CAPACITY, Ordering::Release);
        let limited = spawn.spawn::<u32, _>(|_| 1);
        assert_eq!(limited.status(), ComponentTaskStatus::Rejected);
        let restarted = store.restarted(WindowToken::new(WindowId::allocate()));
        assert!(restarted.task_limiter.acquire().is_none());

        store.task_limiter.active.store(0, Ordering::Release);
        store.close();
        let closed = spawn.spawn::<u32, _>(|_| 1);
        assert_eq!(closed.status(), ComponentTaskStatus::Rejected);
    }

    #[test]
    fn valid_sender_wakes_once_and_retirement_blocks_new_traffic() {
        let wakes = Rc::new(Cell::new(0));
        let wake_capture = Rc::clone(&wakes);
        let mut store = store();
        store.set_waker(Rc::new(move || {
            wake_capture.set(wake_capture.get() + 1);
        }));
        let token = reserve_state(&mut store, "");
        store.publish(token).unwrap();
        let sender = store.sender::<u32>(token).unwrap();

        sender.send(1);
        sender.send(2);
        assert_eq!(wakes.get(), 1);
        assert_eq!(store.pending(), 2);
        store.remove(token).unwrap();
        assert_eq!(store.pending(), 0);
        sender.send(3);
        assert_eq!(store.pending(), 0);
        assert_eq!(wakes.get(), 1);
    }

    #[test]
    fn local_message_queue_reports_backpressure_at_its_fixed_capacity() {
        let mut store = store();
        let token = reserve_state(&mut store, "");
        store.publish(token).unwrap();
        let sender = store.sender::<u32>(token).unwrap();

        for value in 0..LOCAL_MESSAGE_QUEUE_CAPACITY {
            assert!(sender.send(value as u32));
        }
        assert!(!sender.send(u32::MAX));
        assert_eq!(store.pending(), LOCAL_MESSAGE_QUEUE_CAPACITY);
    }

    struct Counter {
        value: u32,
    }

    impl Component for Counter {
        type Props = u32;
        type Message = u32;

        fn create(props: &Self::Props, context: &mut ComponentContext<Self>) -> Self {
            context.sender().send(1);
            Self { value: *props }
        }

        fn changed(&mut self, props: &Self::Props, _context: &mut ComponentContext<Self>) {
            self.value = *props;
        }

        fn update(&mut self, message: Self::Message, _context: &mut ComponentContext<Self>) {
            self.value += message;
        }

        fn view(&self, _props: &Self::Props, _context: &mut ViewContext<Self>) -> View {
            View::native(TextBlock::new().text(self.value.to_string()))
        }
    }

    #[test]
    fn component_factory_creates_with_a_reserved_sender_and_composes() {
        let first = ComponentView::new::<Counter>(2);
        let same = ComponentView::new::<Counter>(2);
        let changed = ComponentView::new::<Counter>(3);
        assert_eq!(first, same);
        assert_ne!(first, changed);

        let mut store = store();
        let token = first.reserve(&mut store).unwrap();
        assert_eq!(store.pending(), 1);
        assert!(store.drain(10).unwrap().blocked);
        store.publish(token).unwrap();
        assert_eq!(store.drain(10).unwrap().dispatched, 1);
        assert_eq!(store.component::<Counter>(token).unwrap().value, 3);
        assert_eq!(
            store.view(token, ContextSnapshot::default()).unwrap().view,
            View::native(TextBlock::new().text("3"))
        );
    }
}
