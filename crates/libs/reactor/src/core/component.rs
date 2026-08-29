use super::arena::NodeId;
use super::runtime::WindowToken;
use super::scope::{ScopeArena, ScopeError, ScopeId, ScopeState};
use crate::element::{
    Callback, CallbackSource, ColorScheme, IntoPayloadCallback, View, WindowSize, WindowVisuals,
};
use crate::reference::{HostRequest, WindowEndpoint, WindowRef};
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::marker::PhantomData;
use std::mem::size_of;
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
    DuplicateColorSchemeObservation,
    DuplicateWindowSizeObservation,
    DuplicateWindowTitle,
    DuplicateWindowVisuals,
    MessageTypeMismatch {
        expected: TypeId,
        actual: TypeId,
    },
    InputTypeMismatch {
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
    delivery: BackgroundDelivery,
    payload: Box<dyn Any + Send>,
    rejection: Option<Box<dyn Any + Send>>,
    token: ComponentToken,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum BackgroundDelivery {
    Completion,
    Rejection,
}

impl BackgroundEnvelope {
    fn reject(mut self) -> Option<Self> {
        if self.delivery == BackgroundDelivery::Rejection {
            return Some(self);
        }
        if !self.control.reject() {
            return None;
        }
        self.payload = self.rejection.take()?;
        self.delivery = BackgroundDelivery::Rejection;
        Some(self)
    }
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

    fn reject(&self) -> bool {
        self.finish(ComponentTaskStatus::Rejected)
    }

    fn finish(&self, status: ComponentTaskStatus) -> bool {
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
                Ok(_) => return true,
                Err(actual) => current = actual,
            }
        }
        false
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
        self.spawn_inner(work, None)
    }

    fn spawn_with_rejection<M, F>(&self, work: F, rejection: M) -> ComponentTask
    where
        M: Send + 'static,
        F: FnOnce(CancellationToken) -> M + Send + 'static,
    {
        self.spawn_inner(work, Some(Box::new(rejection)))
    }

    fn spawn_inner<M, F>(&self, work: F, rejection: Option<Box<dyn Any + Send>>) -> ComponentTask
    where
        M: Send + 'static,
        F: FnOnce(CancellationToken) -> M + Send + 'static,
    {
        let control = Arc::new(TaskControl::new());
        let rejection = Arc::new(Mutex::new(rejection));
        let task = ComponentTask {
            control: Arc::clone(&control),
            queue: Arc::clone(&self.queue),
            token: self.token,
        };
        let Some(slot) = self.limiter.acquire() else {
            self.queue_rejection(&control, rejection.lock().unwrap().take());
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
        let thread_rejection = Arc::clone(&rejection);
        let spawn = std::thread::Builder::new()
            .name("windows-reactor".to_string())
            .spawn(move || {
                let _slot = slot;
                let message = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    work(CancellationToken {
                        control: Arc::clone(&thread_control),
                    })
                }));
                let wake = {
                    let mut background = queue.lock().unwrap();
                    let registered = background.tasks.get_mut(&token.scope).is_some_and(|tasks| {
                        let before = tasks.len();
                        tasks.retain(|task| {
                            task.upgrade()
                                .is_some_and(|task| !Arc::ptr_eq(&task, &thread_control))
                        });
                        tasks.len() != before
                    });
                    if background
                        .tasks
                        .get(&token.scope)
                        .is_some_and(Vec::is_empty)
                    {
                        background.tasks.remove(&token.scope);
                    }
                    if !registered
                        || thread_control.status() == ComponentTaskStatus::Cancelled
                        || !background.open
                    {
                        thread_control.cancel();
                        return;
                    }
                    let Ok(message) = message else {
                        drop(background);
                        Self::queue_rejection_shared(
                            &queue,
                            &thread_control,
                            token,
                            thread_rejection.lock().unwrap().take(),
                        );
                        return;
                    };
                    if background.envelopes.len() >= BACKGROUND_MESSAGE_QUEUE_CAPACITY {
                        drop(background);
                        Self::queue_rejection_shared(
                            &queue,
                            &thread_control,
                            token,
                            thread_rejection.lock().unwrap().take(),
                        );
                        return;
                    }
                    if !thread_control.queue() {
                        return;
                    }
                    background.envelopes.push_back(BackgroundEnvelope {
                        control: Arc::clone(&thread_control),
                        delivery: BackgroundDelivery::Completion,
                        payload: Box::new(message),
                        rejection: thread_rejection.lock().unwrap().take(),
                        token,
                    });
                    Self::background_wake(&mut background)
                };
                Self::wake_or_reject(&queue, wake);
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
            drop(queue);
            self.queue_rejection(&control, rejection.lock().unwrap().take());
        }
        task
    }

    fn background_wake(queue: &mut BackgroundQueue) -> Option<Arc<dyn Fn() -> bool + Send + Sync>> {
        let wake = (!queue.wake_pending).then(|| queue.wake.clone()).flatten();
        queue.wake_pending |= wake.is_some();
        wake
    }

    fn queue_rejection(&self, control: &Arc<TaskControl>, rejection: Option<Box<dyn Any + Send>>) {
        Self::queue_rejection_shared(&self.queue, control, self.token, rejection);
    }

    fn queue_rejection_shared(
        queue: &Arc<Mutex<BackgroundQueue>>,
        control: &Arc<TaskControl>,
        token: ComponentToken,
        rejection: Option<Box<dyn Any + Send>>,
    ) {
        let Some(payload) = rejection else {
            control.reject();
            return;
        };
        let mut background = queue.lock().unwrap();
        if !background.open {
            control.reject();
            return;
        }
        if !control.reject() {
            return;
        }
        background.envelopes.push_back(BackgroundEnvelope {
            control: Arc::clone(control),
            delivery: BackgroundDelivery::Rejection,
            payload,
            rejection: None,
            token,
        });
        let wake = Self::background_wake(&mut background);
        drop(background);
        Self::wake_or_reject(queue, wake);
    }

    fn wake_or_reject(
        queue: &Arc<Mutex<BackgroundQueue>>,
        wake: Option<Arc<dyn Fn() -> bool + Send + Sync>>,
    ) {
        if wake.is_some_and(|wake| !wake()) {
            let mut queue = queue.lock().unwrap();
            queue.wake_pending = false;
            queue.envelopes = queue
                .envelopes
                .drain(..)
                .filter_map(BackgroundEnvelope::reject)
                .collect();
        }
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

    pub fn callback<T, F>(&self, map: F) -> Callback<T>
    where
        F: Fn(T) -> M + 'static,
    {
        let sender = self.clone();
        if size_of::<F>() == 0 {
            let source = CallbackSource::new(Rc::as_ptr(&self.queue) as usize, self.token);
            Callback::new_identified(source, TypeId::of::<F>(), move |value| {
                sender.send(map(value))
            })
        } else {
            Callback::new_with_acceptance(move |value| sender.send(map(value)))
        }
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
    window: WindowRef,
}

impl<C: Component> ComponentContext<C> {
    /// Requests a new application-owned window with an independent Pump.
    #[must_use = "false means there is no active component publication"]
    pub fn open_window(&self, root: View) -> bool {
        self.window.request_open(root)
    }

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

    /// Starts scope-owned work and dispatches `rejected` if task infrastructure rejects it.
    ///
    /// Cancellation and scope retirement do not dispatch `rejected`.
    pub fn spawn_background_with_rejection<F>(&self, work: F, rejected: C::Message) -> ComponentTask
    where
        C::Message: Send,
        F: FnOnce(CancellationToken) -> C::Message + Send + 'static,
    {
        self.tasks.spawn_with_rejection(work, rejected)
    }

    /// Returns a token-bound capability for the component's owning window.
    pub fn window(&self) -> WindowRef {
        self.window.clone()
    }
}

#[derive(Default)]
enum SingleDeclaration<T> {
    #[default]
    Empty,
    Value(T),
    Duplicate,
}

impl<T> SingleDeclaration<T> {
    fn declare(&mut self, value: T) {
        *self = if matches!(self, Self::Empty) {
            Self::Value(value)
        } else {
            Self::Duplicate
        };
    }

    fn resolve<E>(self, duplicate: E) -> Result<Option<T>, E> {
        match self {
            Self::Empty => Ok(None),
            Self::Value(value) => Ok(Some(value)),
            Self::Duplicate => Err(duplicate),
        }
    }
}

pub struct ViewContext<C: Component> {
    contexts: ContextSnapshot,
    effects: Rc<RefCell<ComponentEffects>>,
    reads: HashSet<ContextDependency>,
    sender: LocalSender<C::Message>,
    color_scheme_observation: SingleDeclaration<Callback<ColorScheme>>,
    window_size_observation: SingleDeclaration<Callback<WindowSize>>,
    window_title: SingleDeclaration<String>,
    window_visuals: SingleDeclaration<WindowVisuals>,
}

impl<C: Component> ViewContext<C> {
    /// Declares a queued color-scheme observation for the owning window.
    pub fn on_color_scheme(&mut self, callback: impl IntoPayloadCallback<ColorScheme>) {
        self.color_scheme_observation
            .declare(callback.into_payload_callback());
    }

    /// Declares a queued client-size observation for the owning window.
    pub fn on_window_size(&mut self, callback: impl IntoPayloadCallback<WindowSize>) {
        self.window_size_observation
            .declare(callback.into_payload_callback());
    }

    /// Declares the owning window's title for this component publication.
    pub fn window_title(&mut self, title: impl Into<String>) {
        self.window_title.declare(title.into());
    }

    /// Declares the owning window's visual environment for this publication.
    pub fn window_visuals(&mut self, visuals: WindowVisuals) {
        self.window_visuals.declare(visuals);
    }

    pub fn sender(&self) -> LocalSender<C::Message> {
        self.sender.clone()
    }

    pub fn callback<T>(&self, map: impl Fn(T) -> C::Message + 'static) -> Callback<T> {
        self.sender.callback(map)
    }

    pub fn forward(&self) -> Callback<C::Message> {
        self.sender.callback(std::convert::identity)
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
    type Input: Clone + PartialEq + 'static;
    type Message: 'static;

    fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self;
    fn input_changed(&mut self, _input: &Self::Input, _context: &ComponentContext<Self>) {}
    fn update(&mut self, _message: Self::Message, _context: &ComponentContext<Self>) {}
    fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View;
}

trait ErasedComponentFactory {
    fn apply_input(
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
    input: C::Input,
}

impl<C: Component> ErasedComponentFactory for TypedComponentFactory<C> {
    fn apply_input(
        &self,
        store: &mut ComponentStore,
        token: ComponentToken,
    ) -> Result<bool, ComponentStoreError> {
        store.apply_input(token, self.input.clone())
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
                .is_some_and(|other| self.input == other.input)
    }

    fn reserve(&self, store: &mut ComponentStore) -> Result<ComponentToken, ComponentStoreError> {
        store.reserve_component::<C>(self.input.clone())
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
    pub(crate) fn new<C: Component>(input: C::Input) -> Self {
        Self {
            factory: Rc::new(TypedComponentFactory::<C> { input }),
        }
    }

    pub(crate) fn component_type(&self) -> TypeId {
        self.factory.component_type()
    }

    pub(crate) fn apply_input(
        &self,
        store: &mut ComponentStore,
        token: ComponentToken,
    ) -> Result<bool, ComponentStoreError> {
        self.factory.apply_input(store, token)
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
    fn apply_input(
        &mut self,
        input: Box<dyn Any>,
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
    fn input_type(&self) -> TypeId;
    fn type_name(&self) -> &'static str;
    fn context_dependencies(&self) -> Option<&HashSet<ContextDependency>>;
    fn set_context_dependencies(&mut self, dependencies: HashSet<ContextDependency>);
    fn view(&self, contexts: ContextSnapshot) -> Result<ComponentRender, ComponentStoreError>;
    fn cleanup_effects(&self);
    fn commit_effects(&self);
    fn prepare_effects(&self);
}

pub(crate) struct ComponentRender {
    pub(crate) color_scheme_observation: Option<Callback<ColorScheme>>,
    pub(crate) dependencies: HashSet<ContextDependency>,
    pub(crate) view: View,
    pub(crate) window_size_observation: Option<Callback<WindowSize>>,
    pub(crate) window_title: Option<String>,
    pub(crate) window_visuals: Option<WindowVisuals>,
}

type EffectCleanup = Box<dyn FnOnce()>;
type EffectSetup = Box<dyn FnOnce() -> Option<EffectCleanup>>;

struct EffectSlot {
    cleanup: Option<EffectCleanup>,
    dependency: Box<dyn Any>,
    key: EffectKey,
}

enum EffectRegistration {
    Retain {
        key: EffectKey,
    },
    Replace {
        dependency: Box<dyn Any>,
        key: EffectKey,
        setup: EffectSetup,
    },
}

impl EffectRegistration {
    fn key(&self) -> &EffectKey {
        match self {
            Self::Retain { key } | Self::Replace { key, .. } => key,
        }
    }
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
            .any(|registration| registration.key() == &key)
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
        self.registrations.push(if changed {
            EffectRegistration::Replace {
                dependency: Box::new(dependency),
                key,
                setup: Box::new(setup),
            }
        } else {
            EffectRegistration::Retain { key }
        });
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
                .find(|registration| registration.key() == &slot.key)
                .is_none_or(|registration| {
                    matches!(registration, EffectRegistration::Replace { .. })
                });
            if cleanup_required && let Some(cleanup) = slot.cleanup.take() {
                cleanup();
            }
        }
    }

    fn commit(&mut self) {
        let mut published = std::mem::take(&mut self.slots);
        for registration in self.registrations.drain(..) {
            let slot = match registration {
                EffectRegistration::Replace {
                    dependency,
                    key,
                    setup,
                } => EffectSlot {
                    cleanup: setup(),
                    dependency,
                    key,
                },
                EffectRegistration::Retain { key } => {
                    let index = published.iter().position(|slot| slot.key == key).unwrap();
                    published.remove(index)
                }
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

struct TypedScope<C, I, M> {
    component: C,
    context_dependencies: Option<Rc<HashSet<ContextDependency>>>,
    effects: Rc<RefCell<ComponentEffects>>,
    input: I,
    input_changed: fn(&mut C, &I, LocalSender<M>, TaskSpawner, WindowRef),
    sender: LocalSender<M>,
    update: fn(&mut C, M, LocalSender<M>, TaskSpawner, WindowRef),
    view: fn(
        &C,
        &I,
        LocalSender<M>,
        Rc<RefCell<ComponentEffects>>,
        ContextSnapshot,
    ) -> Result<ComponentRender, ComponentStoreError>,
    window: WindowEndpoint,
}

impl<C, I, M> Drop for TypedScope<C, I, M> {
    fn drop(&mut self) {
        self.effects.borrow_mut().cleanup();
    }
}

impl<C, I, M> ErasedScope for TypedScope<C, I, M>
where
    C: 'static,
    I: Clone + PartialEq + 'static,
    M: 'static,
{
    fn apply_input(
        &mut self,
        input: Box<dyn Any>,
        tasks: TaskSpawner,
    ) -> Result<bool, ComponentStoreError> {
        let actual = input.as_ref().type_id();
        let input = input
            .downcast::<I>()
            .map_err(|_| ComponentStoreError::InputTypeMismatch {
                expected: TypeId::of::<I>(),
                actual,
            })?;
        if self.input == *input {
            return Ok(false);
        }
        self.input = *input;
        self.window.begin();
        (self.input_changed)(
            &mut self.component,
            &self.input,
            self.sender.clone(),
            tasks,
            self.window.reference(),
        );
        self.window.finish();
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
        self.window.begin();
        (self.update)(
            &mut self.component,
            *message,
            self.sender.clone(),
            tasks,
            self.window.reference(),
        );
        self.window.finish();
        Ok(())
    }

    #[cfg(test)]
    fn message_type(&self) -> TypeId {
        TypeId::of::<M>()
    }

    fn input_type(&self) -> TypeId {
        TypeId::of::<I>()
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<C>()
    }

    fn view(&self, contexts: ContextSnapshot) -> Result<ComponentRender, ComponentStoreError> {
        self.effects.borrow_mut().begin_view();
        let render = (self.view)(
            &self.component,
            &self.input,
            self.sender.clone(),
            Rc::clone(&self.effects),
            contexts,
        )?;
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
    window_endpoint: WindowEndpoint,
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
            window_endpoint: WindowEndpoint::new(window),
        }
    }

    pub fn reserve_component<C: Component>(
        &mut self,
        input: C::Input,
    ) -> Result<ComponentToken, ComponentStoreError> {
        fn input_changed<C: Component>(
            component: &mut C,
            input: &C::Input,
            sender: LocalSender<C::Message>,
            tasks: TaskSpawner,
            window: WindowRef,
        ) {
            component.input_changed(
                input,
                &ComponentContext {
                    sender,
                    tasks,
                    window,
                },
            );
        }

        fn update<C: Component>(
            component: &mut C,
            message: C::Message,
            sender: LocalSender<C::Message>,
            tasks: TaskSpawner,
            window: WindowRef,
        ) {
            component.update(
                message,
                &ComponentContext {
                    sender,
                    tasks,
                    window,
                },
            );
        }

        fn view<C: Component>(
            component: &C,
            input: &C::Input,
            sender: LocalSender<C::Message>,
            effects: Rc<RefCell<ComponentEffects>>,
            contexts: ContextSnapshot,
        ) -> Result<ComponentRender, ComponentStoreError> {
            let mut context = ViewContext {
                contexts,
                effects,
                reads: HashSet::new(),
                sender,
                color_scheme_observation: SingleDeclaration::default(),
                window_size_observation: SingleDeclaration::default(),
                window_title: SingleDeclaration::default(),
                window_visuals: SingleDeclaration::default(),
            };
            let view = component.view(input, &mut context);
            let color_scheme_observation = context
                .color_scheme_observation
                .resolve(ComponentStoreError::DuplicateColorSchemeObservation)?;
            let window_size_observation = context
                .window_size_observation
                .resolve(ComponentStoreError::DuplicateWindowSizeObservation)?;
            let window_title = context
                .window_title
                .resolve(ComponentStoreError::DuplicateWindowTitle)?;
            let window_visuals = context
                .window_visuals
                .resolve(ComponentStoreError::DuplicateWindowVisuals)?;
            Ok(ComponentRender {
                color_scheme_observation,
                dependencies: context.reads,
                view,
                window_size_observation,
                window_title,
                window_visuals,
            })
        }

        let background = Arc::clone(&self.background);
        let queue = Rc::clone(&self.queue);
        let task_limiter = Arc::clone(&self.task_limiter);
        let window = self.window;
        let window_endpoint = self.window_endpoint.clone();
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
            window_endpoint.begin();
            let component = C::create(
                &input,
                &ComponentContext {
                    sender: sender.clone(),
                    tasks,
                    window: window_endpoint.reference(),
                },
            );
            window_endpoint.finish();
            Box::new(TypedScope {
                component,
                context_dependencies: None,
                effects: Rc::new(RefCell::new(ComponentEffects::default())),
                input,
                input_changed: input_changed::<C>,
                sender,
                update: update::<C>,
                view: view::<C>,
                window: window_endpoint,
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

    pub(crate) fn take_host_requests(&self) -> Vec<HostRequest> {
        self.window_endpoint.take_requests()
    }

    pub(crate) fn commit_window_close(&self) {
        self.window_endpoint.commit_close();
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

    pub fn apply_input<I: 'static>(
        &mut self,
        token: ComponentToken,
        input: I,
    ) -> Result<bool, ComponentStoreError> {
        self.validate_window(token)?;
        let tasks = self.task_spawner(token);
        let scope = self.scopes.get_mut(token.scope)?;
        let actual = TypeId::of::<I>();
        let expected = scope.input_type();
        if actual != expected {
            return Err(ComponentStoreError::InputTypeMismatch { expected, actual });
        }
        scope.apply_input(Box::new(input), tasks)
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
                    if let PendingEnvelope::Background(background) = &envelope {
                        match background.delivery {
                            BackgroundDelivery::Completion if !background.control.deliver() => {
                                report.dropped += 1;
                                continue;
                            }
                            BackgroundDelivery::Rejection
                                if background.control.status() != ComponentTaskStatus::Rejected =>
                            {
                                report.dropped += 1;
                                continue;
                            }
                            BackgroundDelivery::Completion | BackgroundDelivery::Rejection => {}
                        }
                    } else if let Some(control) = envelope.control()
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

    #[cfg(test)]
    pub(crate) fn exhaust_task_capacity(&self) {
        self.task_limiter
            .active
            .store(BACKGROUND_TASK_CAPACITY, Ordering::Release);
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

    pub(crate) fn type_name(
        &self,
        token: ComponentToken,
    ) -> Result<&'static str, ComponentStoreError> {
        self.validate_window(token)?;
        Ok(self.scopes.get(token.scope)?.type_name())
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
        self.window_endpoint.close();
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
#[path = "component_tests.rs"]
mod tests;
