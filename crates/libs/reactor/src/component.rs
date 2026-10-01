use super::*;
use std::any::{Any, TypeId};
#[cfg(test)]
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
use std::mem::size_of;
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

#[derive(Clone)]
pub(crate) struct ContextProvision {
    id: ContextId,
    value: ContextValue,
}

impl ContextProvision {
    fn new<T: Clone + PartialEq + 'static>(context: &Context<T>, value: T) -> Self {
        Self {
            id: context.id,
            value: ContextValue {
                equals: |left, right| {
                    left.downcast_ref::<T>()
                        .zip(right.downcast_ref::<T>())
                        .is_some_and(|(left, right)| left == right)
                },
                value: Rc::new(value),
            },
        }
    }
}

impl std::fmt::Debug for ContextProvision {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_tuple("ContextProvision")
            .field(&self.id)
            .finish()
    }
}

impl PartialEq for ContextProvision {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && (self.value.equals)(self.value.value.as_ref(), other.value.value.as_ref())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ComponentId {
    index: u32,
    generation: u32,
}

struct Message {
    component: ComponentId,
    control: Option<Arc<TaskControl>>,
    sequence: u64,
    value: MessageValue,
}

enum MessageValue {
    Send(Box<dyn Any + Send>),
    Local(Box<dyn FnOnce() -> Box<dyn Any> + Send>),
}

struct LocalMessage {
    component: ComponentId,
    sequence: u64,
    value: Box<dyn Any>,
}

#[derive(Default)]
struct MessageQueue {
    closed: bool,
    messages: VecDeque<Message>,
    next_sequence: u64,
    wake_pending: bool,
    waker: Option<Arc<dyn Fn() + Send + Sync>>,
}

type SharedQueue = Arc<Mutex<MessageQueue>>;
type LocalQueue = Rc<RefCell<VecDeque<LocalMessage>>>;

#[derive(Clone)]
struct ComponentQueues {
    shared: SharedQueue,
    local: LocalQueue,
}

pub struct LocalSender<M> {
    component: ComponentId,
    queue: LocalQueue,
    wake: SharedQueue,
    marker: std::marker::PhantomData<fn(M)>,
}

impl<M> Clone for LocalSender<M> {
    fn clone(&self) -> Self {
        Self {
            component: self.component,
            queue: Rc::clone(&self.queue),
            wake: Arc::clone(&self.wake),
            marker: std::marker::PhantomData,
        }
    }
}

impl<M: 'static> LocalSender<M> {
    #[must_use]
    pub fn send(&self, value: M) -> bool {
        let mut wake_queue = self.wake.lock().unwrap();
        if wake_queue.closed {
            return false;
        }
        let mut queue = self.queue.borrow_mut();
        if queue.len() >= MESSAGE_CAPACITY {
            return false;
        }
        let sequence = wake_queue.next_sequence;
        wake_queue.next_sequence = wake_queue.next_sequence.wrapping_add(1);
        queue.push_back(LocalMessage {
            component: self.component,
            sequence,
            value: Box::new(value),
        });
        drop(queue);
        let wake = (!wake_queue.wake_pending)
            .then(|| wake_queue.waker.clone())
            .flatten();
        wake_queue.wake_pending |= wake.is_some();
        drop(wake_queue);
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
        let callback = move |value| {
            _ = sender.send(map(value));
        };
        if size_of::<F>() == 0 {
            Callback::new_identified(
                CallbackIdentity {
                    queue: Rc::as_ptr(&self.queue) as usize,
                    component: self.component,
                    mapping: TypeId::of::<F>(),
                },
                callback,
            )
        } else {
            Callback::new(callback)
        }
    }

    pub fn forward(&self) -> Callback<M> {
        self.callback(std::convert::identity)
    }

    pub fn message(&self, message: M) -> Callback<()>
    where
        M: Clone,
    {
        self.callback(move |()| message.clone())
    }
}

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

    pub fn callback<T, F>(&self, map: F) -> Callback<T>
    where
        F: Fn(T) -> M + 'static,
    {
        let sender = self.clone();
        let callback = move |value| {
            _ = sender.send(map(value));
        };
        if size_of::<F>() == 0 {
            Callback::new_identified(
                CallbackIdentity {
                    queue: Arc::as_ptr(&self.queue) as usize,
                    component: self.component,
                    mapping: TypeId::of::<F>(),
                },
                callback,
            )
        } else {
            Callback::new(callback)
        }
    }

    pub fn forward(&self) -> Callback<M> {
        self.callback(std::convert::identity)
    }

    pub fn message(&self, message: M) -> Callback<()>
    where
        M: Clone,
    {
        self.callback(move |()| message.clone())
    }

    fn send_controlled(&self, value: M, control: Arc<TaskControl>) -> bool {
        self.enqueue(value, Some(control))
    }

    fn enqueue(&self, value: M, control: Option<Arc<TaskControl>>) -> bool {
        let mut queue = self.queue.lock().unwrap();
        if queue.closed || queue.messages.len() >= MESSAGE_CAPACITY {
            if let Some(control) = control {
                control.reject();
            }
            return false;
        }
        let sequence = queue.next_sequence;
        queue.next_sequence = queue.next_sequence.wrapping_add(1);
        queue.messages.push_back(Message {
            component: self.component,
            control,
            sequence,
            value: MessageValue::Send(Box::new(value)),
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

pub struct ComponentContext<C: Component> {
    reference: ElementRef,
    local_sender: LocalSender<C::Message>,
    sender: ComponentSender<C::Message>,
    services: Arc<dyn ComponentServices>,
    tasks: Arc<Mutex<Vec<Weak<TaskControl>>>>,
    ui_services: Rc<dyn ComponentUiServices>,
}

pub struct WindowHandle<'a> {
    raw: *mut core::ffi::c_void,
    marker: std::marker::PhantomData<(&'a mut (), Rc<()>)>,
}

impl WindowHandle<'_> {
    fn new(raw: *mut core::ffi::c_void) -> Self {
        Self {
            raw,
            marker: std::marker::PhantomData,
        }
    }

    pub fn as_raw(&self) -> *mut core::ffi::c_void {
        self.raw
    }
}

impl<C: Component> ComponentContext<C> {
    pub fn sender(&self) -> LocalSender<C::Message> {
        self.local_sender.clone()
    }

    pub fn callback<T>(&self, map: impl Fn(T) -> C::Message + 'static) -> Callback<T> {
        self.local_sender.callback(map)
    }

    pub fn forward(&self) -> Callback<C::Message> {
        self.local_sender.forward()
    }

    pub fn message(&self, message: C::Message) -> Callback<()>
    where
        C::Message: Clone,
    {
        self.local_sender.message(message)
    }

    pub fn root(&self) -> ElementRef {
        self.reference.clone()
    }

    #[must_use]
    pub fn open_window<W: Component>(&self, input: W::Input) -> bool {
        self.open_window_with_policy::<W>(input, WindowPolicy::new())
    }

    #[must_use]
    pub fn open_window_with_policy<W: Component>(
        &self,
        input: W::Input,
        policy: WindowPolicy,
    ) -> bool {
        self.ui_services
            .open_window(component::<W>("root", input), policy)
    }

    #[must_use]
    pub fn activate_window(&self) -> bool {
        self.ui_services.activate_window()
    }

    #[must_use]
    pub fn close_window(&self) -> bool {
        self.ui_services.close_window()
    }

    #[must_use]
    pub fn run_window(
        &self,
        work: impl for<'a> FnOnce(WindowHandle<'a>) -> C::Message + 'static,
    ) -> bool {
        let sender = self.local_sender.clone();
        self.ui_services.run_window(Box::new(move |raw| {
            _ = sender.send(work(WindowHandle::new(raw)));
        }))
    }

    #[must_use = "dropping the timer cancels message delivery"]
    pub fn set_local_timeout(
        &self,
        delay: Duration,
        message: impl FnOnce() -> C::Message + Send + 'static,
    ) -> ComponentTimer {
        let control = Arc::new(TaskControl::default());
        let mut tasks = self.tasks.lock().unwrap();
        tasks.retain(|task| task.strong_count() != 0);
        tasks.push(Arc::downgrade(&control));
        drop(tasks);
        let component = self.sender.component;
        let queue = Arc::clone(&self.sender.queue);
        let timer_control = Arc::clone(&control);
        let registration = self.services.set_timeout(
            delay,
            Box::new(move || {
                timer_control.timer_fired();
                if !timer_control.queue() {
                    return;
                }
                let mut queue = queue.lock().unwrap();
                if queue.closed || queue.messages.len() >= MESSAGE_CAPACITY {
                    timer_control.reject();
                    return;
                }
                let sequence = queue.next_sequence;
                queue.next_sequence = queue.next_sequence.wrapping_add(1);
                queue.messages.push_back(Message {
                    component,
                    control: Some(Arc::clone(&timer_control)),
                    sequence,
                    value: MessageValue::Local(Box::new(move || Box::new(message()))),
                });
                let wake = (!queue.wake_pending).then(|| queue.waker.clone()).flatten();
                queue.wake_pending |= wake.is_some();
                drop(queue);
                if let Some(wake) = wake {
                    wake();
                }
            }),
        );
        control.set_timer(registration);
        ComponentTimer {
            task: ComponentTask { control },
        }
    }
}

impl<C> ComponentContext<C>
where
    C: Component,
    C::Message: Send,
{
    pub fn completion(&self) -> ComponentCompletion<C::Message> {
        self.sender.completion()
    }

    pub fn spawn_background(
        &self,
        work: impl FnOnce(CancellationToken) -> C::Message + Send + 'static,
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
    pub fn set_timeout(&self, delay: Duration, message: C::Message) -> ComponentTimer {
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

pub(crate) trait ComponentUiServices {
    fn open_window(&self, root: ComponentNode, policy: WindowPolicy) -> bool;

    fn publish_window(&self, _publication: WindowPublication) {}

    fn activate_window(&self) -> bool {
        false
    }

    fn close_window(&self) -> bool {
        false
    }

    fn run_window(&self, _operation: Box<dyn FnOnce(*mut core::ffi::c_void)>) -> bool {
        false
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct WindowPublication {
    pub(crate) on_color_scheme: Option<Callback<ColorScheme>>,
    pub(crate) on_size: Option<Callback<WindowSize>>,
    pub(crate) title: Option<String>,
    pub(crate) visuals: Option<WindowVisuals>,
}

struct ComponentRender {
    dependencies: HashSet<ContextId>,
    effects: EffectDraft,
    view: View,
    window: Option<Box<WindowPublication>>,
}

enum ComponentDeclarationError {
    Effect(EffectKey),
    WindowColorScheme,
    WindowSize,
    WindowTitle,
    WindowVisuals,
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

pub struct ViewContext<'a, C: Component> {
    context: ComponentContext<C>,
    contexts: &'a HashMap<ContextId, ContextValue>,
    dependencies: HashSet<ContextId>,
    effects: EffectDraft,
    window: WindowPublication,
    window_color_scheme_duplicate: bool,
    window_size_duplicate: bool,
    window_title_duplicate: bool,
    window_visuals_duplicate: bool,
}

impl<C: Component> ViewContext<'_, C> {
    pub fn sender(&self) -> LocalSender<C::Message> {
        self.context.sender()
    }

    pub fn callback<T>(&self, map: impl Fn(T) -> C::Message + 'static) -> Callback<T> {
        self.context.callback(map)
    }

    pub fn forward(&self) -> Callback<C::Message> {
        self.context.forward()
    }

    pub fn message(&self, message: C::Message) -> Callback<()>
    where
        C::Message: Clone,
    {
        self.context.message(message)
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

    pub fn on_window_size(&mut self, callback: impl IntoPayloadCallback<WindowSize>) {
        self.window_size_duplicate |= self
            .window
            .on_size
            .replace(callback.into_payload_callback())
            .is_some();
    }

    pub fn on_color_scheme(&mut self, callback: impl IntoPayloadCallback<ColorScheme>) {
        self.window_color_scheme_duplicate |= self
            .window
            .on_color_scheme
            .replace(callback.into_payload_callback())
            .is_some();
    }

    pub fn window_title(&mut self, title: impl Into<String>) {
        self.window_title_duplicate |= self.window.title.replace(title.into()).is_some();
    }

    #[must_use]
    pub fn window_frame(&mut self, title: impl Into<String>, content: impl Into<View>) -> View {
        let title = title.into();
        self.window_title(title.clone());
        Grid::new()
            .rows([GridLength::Auto, GridLength::STAR])
            .children((
                TitleBar::new().title(title),
                Border::new().grid_row(1).content(content),
            ))
            .into()
    }

    pub fn window_visuals(&mut self, visuals: WindowVisuals) {
        self.window_visuals_duplicate |= self.window.visuals.replace(visuals).is_some();
    }
}

impl<C> ViewContext<'_, C>
where
    C: Component,
    C::Message: Send,
{
    pub fn completion(&self) -> ComponentCompletion<C::Message> {
        self.context.completion()
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

struct VirtualRow;

impl Component for VirtualRow {
    type Input = View;
    type Message = ();

    fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
        Self
    }

    fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
        input.clone()
    }
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

pub fn provide<T>(context: &Context<T>, value: T, child: impl Into<View>) -> View
where
    T: Clone + PartialEq + 'static,
{
    View(DeclaredNode::Provider(Box::new(DeclaredProvider {
        provision: ContextProvision::new(context, value),
        child: child.into().0,
    })))
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
    ) -> Result<(Box<dyn ErasedComponent>, ComponentRender), ComponentDeclarationError>;
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
    ) -> Result<(Box<dyn ErasedComponent>, ComponentRender), ComponentDeclarationError> {
        let sender = ComponentSender {
            component: id,
            queue: queues.shared,
            marker: std::marker::PhantomData,
        };
        let local_sender = LocalSender {
            component: id,
            queue: queues.local,
            wake: Arc::clone(&sender.queue),
            marker: std::marker::PhantomData,
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
        let scope = TypedScope {
            component,
            input: self.input.clone(),
            local_sender,
            sender,
            services,
            tasks,
            ui_services,
        };
        let render = scope.render_view(reference, contexts)?;
        Ok((Box::new(scope), render))
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

trait ErasedComponent {
    fn apply_input(
        &mut self,
        input: &dyn Any,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<Option<ComponentRender>, ComponentDeclarationError>;
    fn component_type(&self) -> TypeId;
    fn cancel_tasks(&self);
    fn dispatch(
        &mut self,
        message: Box<dyn Any>,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<ComponentRender, ComponentDeclarationError>;
    fn render_view(
        &self,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<ComponentRender, ComponentDeclarationError>;
    fn sender(&self) -> &dyn Any;
    #[cfg(any(test, feature = "test"))]
    fn tracked_tasks(&self) -> usize;
}

struct TypedScope<C: Component> {
    component: C,
    input: C::Input,
    local_sender: LocalSender<C::Message>,
    sender: ComponentSender<C::Message>,
    services: Arc<dyn ComponentServices>,
    tasks: Arc<Mutex<Vec<Weak<TaskControl>>>>,
    ui_services: Rc<dyn ComponentUiServices>,
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
        &self,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<ComponentRender, ComponentDeclarationError> {
        let mut context: ViewContext<'_, C> = ViewContext {
            context: self.context(reference),
            contexts,
            dependencies: HashSet::new(),
            effects: EffectDraft::default(),
            window: WindowPublication::default(),
            window_color_scheme_duplicate: false,
            window_size_duplicate: false,
            window_title_duplicate: false,
            window_visuals_duplicate: false,
        };
        let view = self.component.view(&self.input, &mut context);
        if let Some(key) = context.effects.duplicate {
            return Err(ComponentDeclarationError::Effect(key));
        }
        if context.window_color_scheme_duplicate {
            return Err(ComponentDeclarationError::WindowColorScheme);
        }
        if context.window_size_duplicate {
            return Err(ComponentDeclarationError::WindowSize);
        }
        if context.window_title_duplicate {
            return Err(ComponentDeclarationError::WindowTitle);
        }
        if context.window_visuals_duplicate {
            return Err(ComponentDeclarationError::WindowVisuals);
        }
        Ok(ComponentRender {
            dependencies: context.dependencies,
            effects: context.effects,
            view,
            window: (context.window != WindowPublication::default())
                .then(|| Box::new(context.window)),
        })
    }
}

impl<C: Component> ErasedComponent for TypedScope<C> {
    fn apply_input(
        &mut self,
        input: &dyn Any,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<Option<ComponentRender>, ComponentDeclarationError> {
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
        message: Box<dyn Any>,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<ComponentRender, ComponentDeclarationError> {
        let context = self.context(reference.clone());
        self.component
            .update(*message.downcast::<C::Message>().unwrap(), &context);
        self.render_view(reference, contexts)
    }

    fn render_view(
        &self,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<ComponentRender, ComponentDeclarationError> {
        self.render_view(reference, contexts)
    }

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

struct Scope {
    children: HashMap<Key, ComponentId>,
    component: Box<dyn ErasedComponent>,
    contexts: HashMap<ContextId, ContextValue>,
    dependencies: HashSet<ContextId>,
    effects: Vec<EffectSlot>,
    key: Key,
    parent: Option<ComponentId>,
    provided_contexts: HashSet<ContextId>,
    reference: ElementRef,
    root: Option<ObjectId>,
    window: Option<Box<WindowPublication>>,
}

struct ScopeSlot {
    generation: u32,
    scope: Option<Scope>,
}

struct PendingScopeRender {
    dependencies: HashSet<ContextId>,
    environment: Box<OwnedContextEnvironment>,
    effects: EffectDraft,
    id: ComponentId,
    window: Option<Box<WindowPublication>>,
}

struct ContextEnvironment<'a> {
    contexts: &'a HashMap<ContextId, ContextValue>,
    provided: &'a HashSet<ContextId>,
}

struct OwnedContextEnvironment {
    contexts: HashMap<ContextId, ContextValue>,
    provided: HashSet<ContextId>,
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

struct ExpansionLocation<'a> {
    depth: usize,
    path: &'a mut Vec<ComponentPathSegment>,
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
    DuplicateWindowColorScheme,
    DuplicateWindowSize,
    DuplicateWindowTitle,
    DuplicateWindowVisuals,
    MissingComponent(Key),
    Runtime(UpdateError<E>),
}

impl<E> From<UpdateError<E>> for ComponentError<E> {
    fn from(value: UpdateError<E>) -> Self {
        Self::Runtime(value)
    }
}

impl<E> From<ComponentDeclarationError> for ComponentError<E> {
    fn from(value: ComponentDeclarationError) -> Self {
        match value {
            ComponentDeclarationError::Effect(key) => Self::DuplicateEffect(key),
            ComponentDeclarationError::WindowColorScheme => Self::DuplicateWindowColorScheme,
            ComponentDeclarationError::WindowSize => Self::DuplicateWindowSize,
            ComponentDeclarationError::WindowTitle => Self::DuplicateWindowTitle,
            ComponentDeclarationError::WindowVisuals => Self::DuplicateWindowVisuals,
        }
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
    poisoned: bool,
    queue: SharedQueue,
    local_queue: LocalQueue,
    continuation_waker: Option<Rc<dyn Fn()>>,
    runtime: Runtime<A>,
    services: Arc<dyn ComponentServices>,
    ui_services: Rc<dyn ComponentUiServices>,
    free_scopes: Vec<u32>,
    scopes: Vec<ScopeSlot>,
    virtual_rows: HashMap<(ObjectId, Key), ComponentId>,
}

#[cfg(any(test, feature = "test"))]
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

impl<A: Adapter> Drop for ComponentHost<A> {
    fn drop(&mut self) {
        self.invalidate_host();
    }
}

impl<A: Adapter> ComponentHost<A> {
    fn resolve_window_publication(
        &self,
        pending: &[PendingScopeRender],
        retired: &[ComponentId],
    ) -> Result<WindowPublication, ComponentError<A::Error>> {
        let mut publication = WindowPublication::default();
        for (index, slot) in self.scopes.iter().enumerate() {
            let Some(scope) = &slot.scope else {
                continue;
            };
            let id = ComponentId {
                index: u32::try_from(index).unwrap(),
                generation: slot.generation,
            };
            if retired.iter().any(|retired| self.owns_scope(*retired, id)) {
                continue;
            }
            let window = match pending.iter().find(|pending| pending.id == id) {
                Some(pending) => pending.window.as_deref(),
                None => scope.window.as_deref(),
            };
            let Some(window) = window else {
                continue;
            };
            if let Some(on_color_scheme) = &window.on_color_scheme
                && publication
                    .on_color_scheme
                    .replace(on_color_scheme.clone())
                    .is_some()
            {
                return Err(ComponentError::DuplicateWindowColorScheme);
            }
            if let Some(on_size) = &window.on_size
                && publication.on_size.replace(on_size.clone()).is_some()
            {
                return Err(ComponentError::DuplicateWindowSize);
            }
            if let Some(title) = &window.title
                && publication.title.replace(title.clone()).is_some()
            {
                return Err(ComponentError::DuplicateWindowTitle);
            }
            if let Some(visuals) = &window.visuals
                && publication.visuals.replace(visuals.clone()).is_some()
            {
                return Err(ComponentError::DuplicateWindowVisuals);
            }
        }
        Ok(publication)
    }

    fn publish_window(&self) {
        let Ok(publication) = self.resolve_window_publication(&[], &[]) else {
            unreachable!()
        };
        self.ui_services.publish_window(publication);
    }

    fn owns_scope(&self, owner: ComponentId, mut child: ComponentId) -> bool {
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

    pub(crate) fn mount_with_all_services(
        adapter: A,
        services: Arc<dyn ComponentServices>,
        ui_services: Rc<dyn ComponentUiServices>,
        components: impl IntoIterator<Item = ComponentNode>,
    ) -> Result<Self, ComponentError<A::Error>> {
        let queue = Arc::new(Mutex::new(MessageQueue::default()));
        let local_queue = Rc::new(RefCell::new(VecDeque::new()));
        let mut host = Self {
            context_consumers: HashMap::new(),
            contexts: HashMap::new(),
            keys: HashMap::new(),
            order: Vec::new(),
            poisoned: false,
            queue,
            local_queue,
            continuation_waker: None,
            runtime: Runtime::new(adapter),
            services,
            ui_services,
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
            if !keys.insert(key.clone()) {
                return Err(ComponentError::DuplicateKey(key));
            }
            let contexts = host.contexts.clone();
            let (id, render) = host.create_scope(None, node, contexts.clone(), HashSet::new())?;
            host.order.push(id);
            host.keys.insert(key, id);
            expansion.pending.push(PendingScopeRender {
                dependencies: render.dependencies,
                environment: Box::new(OwnedContextEnvironment {
                    contexts: contexts.clone(),
                    provided: HashSet::new(),
                }),
                effects: render.effects,
                id,
                window: render.window,
            });
            let provided = HashSet::new();
            let view = host.expand_view(
                id,
                render.view,
                &ContextEnvironment {
                    contexts: &contexts,
                    provided: &provided,
                },
                0,
                &mut expansion,
            )?;
            declarations.push(keyed(host.scope(id).unwrap().key.clone(), view));
        }
        let root: View = Grid::new().keyed_children(declarations).into();
        let publication = host.resolve_window_publication(&expansion.pending, &[])?;
        host.runtime.update(root.clone())?;
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
        host.ui_services.publish_window(publication);
        Ok(host)
    }

    pub fn runtime(&self) -> &Runtime<A> {
        &self.runtime
    }

    pub(crate) fn runtime_mut_internal(&mut self) -> &mut Runtime<A> {
        &mut self.runtime
    }

    pub fn adapter(&self) -> &A {
        self.runtime.adapter()
    }

    #[cfg(any(test, feature = "test"))]
    pub fn test_adapter_mut(&mut self) -> &mut A {
        self.runtime.adapter_mut()
    }

    #[cfg(any(test, feature = "test"))]
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

    pub fn focus(&mut self, reference: &ElementRef) -> Result<bool, ComponentError<A::Error>> {
        self.ensure_active()?;
        self.runtime
            .focus(reference)
            .map_err(|error| self.runtime_error(error))
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
            for slot in &mut host.scopes {
                if let Some(scope) = slot.scope.as_mut()
                    && !scope.provided_contexts.contains(&context.id)
                {
                    scope.contexts.insert(context.id, value.clone());
                }
            }
            host.contexts = contexts;
            let affected = host
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
            let affected_set = affected.iter().copied().collect::<HashSet<_>>();
            let affected = affected
                .into_iter()
                .filter(|id| {
                    let mut parent = host.scope(*id).and_then(|scope| scope.parent);
                    while let Some(current) = parent {
                        if affected_set.contains(&current) {
                            return false;
                        }
                        parent = host.scope(current).and_then(|scope| scope.parent);
                    }
                    true
                })
                .collect::<Vec<_>>();
            let mut pending = Vec::with_capacity(affected.len());
            for id in affected {
                let scope = host.scope(id).unwrap();
                let render = scope
                    .component
                    .render_view(scope.reference.clone(), &scope.contexts)
                    .map_err(ComponentError::from)?;
                pending.push((id, render));
            }
            let mut report = ComponentDrain::default();
            for (id, render) in pending {
                let mutations = host.apply_render(id, render)?;
                report.dispatched += 1;
                report.mutations += mutations.len();
            }
            Ok(report)
        })
    }

    pub fn drain(&mut self, limit: usize) -> Result<ComponentDrain, ComponentError<A::Error>> {
        self.ensure_active()?;
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
            let native_pending = if active_event.is_none() {
                match self.runtime.prepare_update() {
                    Ok(()) => false,
                    Err(UpdateError::PendingNativeEvent) => true,
                    Err(error) => {
                        self.rearm_wake();
                        return Err(self.runtime_error(error));
                    }
                }
            } else {
                false
            };
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
                            (message.component, message.control, value)
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
                let mutations = match self.run_component_operation(|host| {
                    let contexts = host.scope(component).unwrap().contexts.clone();
                    let scope = host.scope_mut(component).unwrap();
                    let reference = scope.reference.clone();
                    let render = scope
                        .component
                        .dispatch(value, reference, &contexts)
                        .map_err(ComponentError::from)?;
                    host.apply_render(component, render)
                }) {
                    Ok(mutations) => mutations,
                    Err(error) => {
                        self.rearm_wake();
                        return Err(error);
                    }
                };
                report.dispatched += 1;
                report.mutations += mutations.len();
                continue;
            }
            if active_event.take().is_some() {
                continue;
            }
            match self.runtime.next_native_work() {
                Ok(Some(NativeWork::Event(mut event))) => {
                    processed += 1;
                    self.run_component_operation(|_| {
                        event.invoke();
                        Ok(())
                    })?;
                    active_event = Some(event);
                }
                Ok(Some(NativeWork::Virtual(work))) => {
                    processed += 1;
                    let mutations =
                        match self.run_component_operation(|host| host.apply_virtual_work(work)) {
                            Ok(mutations) => mutations,
                            Err(error) => {
                                self.rearm_wake();
                                return Err(error);
                            }
                        };
                    report.mutations += mutations.len();
                }
                Ok(Some(NativeWork::Maintenance)) => {
                    processed += 1;
                }
                Ok(None) => break,
                Err(error) => {
                    self.rearm_wake();
                    return Err(self.runtime_error(error));
                }
            }
        }
        self.rearm_wake();
        Ok(report)
    }

    fn apply_virtual_work(
        &mut self,
        work: VirtualWork,
    ) -> Result<UpdateStats, ComponentError<A::Error>> {
        match work {
            VirtualWork::Realize {
                lease,
                index,
                view,
                owner,
            } => {
                let view = *view;
                if let Some(owner) = owner
                    && self.scope(owner).is_none()
                {
                    return Ok(UpdateStats::default());
                }
                let row_key = (lease.collection, lease.key.clone());
                let existing = self.virtual_rows.get(&row_key).copied();
                let contexts = owner.map_or_else(
                    || self.contexts.clone(),
                    |owner| self.scope(owner).unwrap().contexts.clone(),
                );
                let provided_contexts = owner.map_or_else(HashSet::new, |owner| {
                    self.scope(owner).unwrap().provided_contexts.clone()
                });
                let (id, render, created_row) = if let Some(id) = existing {
                    let reference = self.scope(id).unwrap().reference.clone();
                    let rendered = {
                        let scope = self.scope_mut(id).unwrap();
                        scope
                            .component
                            .apply_input(&view, reference.clone(), &contexts)
                            .map_err(ComponentError::from)?
                            .map_or_else(|| scope.component.render_view(reference, &contexts), Ok)
                            .map_err(ComponentError::from)?
                    };
                    (id, rendered, false)
                } else {
                    let node = component::<VirtualRow>(lease.key.clone(), view);
                    let (id, render) = self.create_scope(
                        owner,
                        node,
                        contexts.clone(),
                        provided_contexts.clone(),
                    )?;
                    (id, render, true)
                };
                let mut expansion = ExpansionState::default();
                expansion.pending.push(PendingScopeRender {
                    dependencies: render.dependencies,
                    environment: Box::new(OwnedContextEnvironment {
                        contexts,
                        provided: provided_contexts,
                    }),
                    effects: render.effects,
                    id,
                    window: render.window,
                });
                let declaration = match self.expand_virtual_view(id, render.view, &mut expansion) {
                    Ok(declaration) => declaration,
                    Err(error) => {
                        self.discard_created(&expansion.created);
                        if created_row {
                            self.retire_scope(id);
                        }
                        return Err(error);
                    }
                };
                let retired = self.unseen_scopes(&expansion.seen);
                let publication = self.resolve_window_publication(&expansion.pending, &retired)?;
                let mutations =
                    match self
                        .runtime
                        .realize_virtual(&lease, index, declaration.clone())
                    {
                        Ok(mutations) => mutations,
                        Err(error) => {
                            self.discard_created(&expansion.created);
                            if created_row {
                                self.retire_scope(id);
                            }
                            return Err(self.runtime_error(error));
                        }
                    };
                let root = self
                    .runtime
                    .graph()
                    .virtual_realization(lease.collection, lease.container)
                    .unwrap();
                self.virtual_rows.insert(row_key, id);
                self.refresh_roots_from(declaration.0.as_object().unwrap(), root);
                for pending in expansion.pending {
                    self.scope_mut(pending.id).unwrap().window = pending.window;
                    pending
                        .effects
                        .commit(&mut self.scope_mut(pending.id).unwrap().effects);
                    self.replace_dependencies(pending.id, pending.dependencies);
                }
                self.retire_unseen(&expansion.seen);
                self.sweep_virtual_rows();
                self.ui_services.publish_window(publication);
                Ok(mutations)
            }
            VirtualWork::Recycle { lease } => {
                let row = self
                    .virtual_rows
                    .get(&(lease.collection, lease.key.clone()))
                    .copied();
                let (runtime, scopes) = (&mut self.runtime, &mut self.scopes);
                let mutations = runtime
                    .recycle_virtual_before_apply(&lease, || {
                        if let Some(row) = row {
                            prepare_scope_retirement(scopes, &[row]);
                        }
                    })
                    .map_err(|error| self.runtime_error(error))?;
                self.sweep_virtual_rows();
                self.publish_window();
                Ok(mutations)
            }
            VirtualWork::Cancel {
                collection,
                relation,
                container,
            } => self
                .runtime
                .cancel_virtual(collection, relation, container)
                .map_err(|error| self.runtime_error(error)),
        }
    }

    fn sweep_virtual_rows(&mut self) {
        let retired = self
            .virtual_rows
            .iter()
            .filter_map(|((collection, key), id)| {
                (!self.runtime.virtual_lease_active(*collection, key))
                    .then_some(((*collection, key.clone()), *id))
            })
            .collect::<Vec<_>>();
        for (key, id) in retired {
            self.virtual_rows.remove(&key);
            self.retire_scope(id);
        }
    }

    fn expand_virtual_view(
        &mut self,
        owner: ComponentId,
        view: View,
        expansion: &mut ExpansionState,
    ) -> Result<View, ComponentError<A::Error>> {
        expansion.seen.entry(owner).or_default();
        let contexts = self.scope(owner).unwrap().contexts.clone();
        let provided_contexts = self.scope(owner).unwrap().provided_contexts.clone();
        let environment = ContextEnvironment {
            contexts: &contexts,
            provided: &provided_contexts,
        };
        let mut path = Vec::new();
        let declaration = match view.0 {
            DeclaredNode::Object(declaration) => self.expand_declaration(
                owner,
                declaration,
                &environment,
                true,
                ExpansionLocation {
                    depth: 0,
                    path: &mut path,
                },
                expansion,
            )?,
            node => {
                path.push(ComponentPathSegment::VirtualRoot);
                self.expand_node(owner, node, &environment, 0, expansion, &mut path)?
            }
        };
        Ok(View(DeclaredNode::Object(declaration)))
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

    fn create_scope(
        &mut self,
        parent: Option<ComponentId>,
        node: ComponentNode,
        contexts: HashMap<ContextId, ContextValue>,
        provided_contexts: HashSet<ContextId>,
    ) -> Result<(ComponentId, ComponentRender), ComponentError<A::Error>> {
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
        let (component, render) = match node.factory.create(
            id,
            ComponentQueues {
                shared: Arc::clone(&self.queue),
                local: Rc::clone(&self.local_queue),
            },
            reference.clone(),
            Arc::clone(&self.services),
            Rc::clone(&self.ui_services),
            &contexts,
        ) {
            Ok(created) => created,
            Err(error) => {
                if let Some(index) = reused {
                    self.free_scopes.push(index);
                }
                return Err(error.into());
            }
        };
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
        Ok((id, render))
    }

    fn expand_view(
        &mut self,
        owner: ComponentId,
        view: View,
        environment: &ContextEnvironment<'_>,
        depth: usize,
        expansion: &mut ExpansionState,
    ) -> Result<View, ComponentError<A::Error>> {
        expansion.seen.entry(owner).or_default();
        let mut path = Vec::new();
        let declaration = match view.0 {
            DeclaredNode::Object(declaration) => declaration,
            DeclaredNode::Component { .. } => return Err(ComponentError::ComponentRoot),
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
        Ok(View(DeclaredNode::Object(self.expand_declaration(
            owner,
            declaration,
            environment,
            true,
            ExpansionLocation {
                depth,
                path: &mut path,
            },
            expansion,
        )?)))
    }

    fn expand_provider_view(
        &mut self,
        owner: ComponentId,
        provision: ContextProvision,
        child: DeclaredNode,
        environment: &ContextEnvironment<'_>,
        location: ExpansionLocation<'_>,
        expansion: &mut ExpansionState,
    ) -> Result<View, ComponentError<A::Error>> {
        let mut contexts = environment.contexts.clone();
        contexts.insert(provision.id, provision.value);
        let mut provided = environment.provided.clone();
        provided.insert(provision.id);
        match child {
            DeclaredNode::Object(declaration) => {
                Ok(View(DeclaredNode::Object(self.expand_declaration(
                    owner,
                    declaration,
                    &ContextEnvironment {
                        contexts: &contexts,
                        provided: &provided,
                    },
                    true,
                    location,
                    expansion,
                )?)))
            }
            _ => Err(ComponentError::ComponentRoot),
        }
    }

    fn expand_declaration(
        &mut self,
        owner: ComponentId,
        mut declaration: Declaration,
        environment: &ContextEnvironment<'_>,
        scope_root: bool,
        location: ExpansionLocation<'_>,
        expansion: &mut ExpansionState,
    ) -> Result<Declaration, ComponentError<A::Error>> {
        if location.depth > MAX_DEPTH {
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
        )?;
        for relation in declaration.relations.as_slice().to_vec() {
            let mut value = relation.value;
            match &mut value {
                RelationValue::One(Some(child)) => {
                    location.path.push(ComponentPathSegment::Relation(
                        relation.id,
                        child.as_ref().relation_identity(0),
                    ));
                    let result = self.expand_node(
                        owner,
                        child.as_ref().clone(),
                        environment,
                        location.depth + 1,
                        expansion,
                        location.path,
                    );
                    location.path.pop();
                    let expanded = result?;
                    *child = Rc::new(DeclaredNode::Object(expanded));
                }
                RelationValue::Many(children) => {
                    for (index, child) in Rc::make_mut(children).iter_mut().enumerate() {
                        location.path.push(ComponentPathSegment::Relation(
                            relation.id,
                            child.relation_identity(index),
                        ));
                        let result = self.expand_node(
                            owner,
                            child.clone(),
                            environment,
                            location.depth + 1,
                            expansion,
                            location.path,
                        );
                        location.path.pop();
                        let expanded = result?;
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
        environment: &ContextEnvironment<'_>,
        depth: usize,
        expansion: &mut ExpansionState,
        path: &mut Vec<ComponentPathSegment>,
    ) -> Result<Declaration, ComponentError<A::Error>> {
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
    ) -> Result<Declaration, ComponentError<A::Error>> {
        let mut prepared =
            self.prepare_component(owner, component, environment, expansion, path)?;
        let expanded = self.expand_view(
            prepared.id,
            prepared.view.take().unwrap(),
            environment,
            depth,
            expansion,
        )?;
        let mut declaration = expanded.0.object().unwrap();
        declaration.key = Some(prepared.declaration_key);
        let attachments = self.expand_attachments(
            owner,
            prepared.attachments.take(),
            environment,
            depth,
            expansion,
            path,
        )?;
        merge_attachments(&mut declaration.attachments, attachments);
        Ok(declaration)
    }

    fn prepare_component(
        &mut self,
        owner: ComponentId,
        component: ComponentExpansion,
        environment: &ContextEnvironment<'_>,
        expansion: &mut ExpansionState,
        path: &[ComponentPathSegment],
    ) -> Result<Box<PreparedComponentExpansion>, ComponentError<A::Error>> {
        let ComponentExpansion {
            attachments,
            mut node,
            relation_key,
        } = component;
        if let Some(component_key) = node.key.as_ref()
            && let Some(RelationKey::Explicit(relation_key)) = &relation_key
            && relation_key != component_key
        {
            return Err(ComponentError::ComponentKey {
                component: component_key.clone(),
                relation: relation_key.clone(),
            });
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
        if !seen.insert(component_key.clone()) {
            return Err(ComponentError::DuplicateKey(component_key));
        }
        let existing = self
            .scope(owner)
            .and_then(|scope| scope.children.get(&component_key).copied());
        let (id, render) = if let Some(id) = existing {
            if self.scope(id).unwrap().component.component_type() != node.factory.component_type() {
                return Err(ComponentError::ComponentType(component_key));
            }
            let scope = self.scope_mut(id).unwrap();
            let reference = scope.reference.clone();
            let rendered = scope
                .component
                .apply_input(
                    node.factory.input(),
                    reference.clone(),
                    environment.contexts,
                )
                .map_err(ComponentError::from)?;
            let render = match rendered {
                Some(rendered) => rendered,
                None => scope
                    .component
                    .render_view(reference, environment.contexts)
                    .map_err(ComponentError::from)?,
            };
            (id, render)
        } else {
            let created = self.create_scope(
                Some(owner),
                node,
                environment.contexts.clone(),
                environment.provided.clone(),
            )?;
            expansion.created.push(created.0);
            self.scope_mut(owner)
                .unwrap()
                .children
                .insert(component_key, created.0);
            created
        };
        expansion.pending.push(PendingScopeRender {
            dependencies: render.dependencies,
            environment: Box::new(OwnedContextEnvironment {
                contexts: environment.contexts.clone(),
                provided: environment.provided.clone(),
            }),
            effects: render.effects,
            id,
            window: render.window,
        });
        Ok(Box::new(PreparedComponentExpansion {
            attachments,
            declaration_key,
            id,
            view: Some(render.view),
        }))
    }

    fn expand_provider_node(
        &mut self,
        owner: ComponentId,
        provision: ContextProvision,
        child: DeclaredNode,
        environment: &ContextEnvironment<'_>,
        location: ExpansionLocation<'_>,
        expansion: &mut ExpansionState,
    ) -> Result<Declaration, ComponentError<A::Error>> {
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
    ) -> Result<Option<Box<DeclaredAttachments>>, ComponentError<A::Error>> {
        let Some(mut attachments) = attachments else {
            return Ok(None);
        };
        if let Some(mut tooltip) = attachments.tooltip.take() {
            if expansion.objects >= MAX_OBJECTS {
                return Err(ComponentError::Runtime(UpdateError::Graph(
                    GraphError::SizeExceeded,
                )));
            }
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
            let expanded = result?;
            tooltip.content = Box::new(DeclaredNode::Object(expanded));
            attachments.tooltip = Some(tooltip);
        }
        if let Some(mut flyout) = attachments.flyout.take() {
            if expansion.objects >= MAX_OBJECTS {
                return Err(ComponentError::Runtime(UpdateError::Graph(
                    GraphError::SizeExceeded,
                )));
            }
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
            let expanded = result?;
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
            dialog.declaration = result?;
            attachments.content_dialog = Some(dialog);
        }
        Ok(Some(attachments))
    }

    fn refresh_roots(&mut self, root: &View) {
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
        if let Some(attachments) = &declaration.attachments {
            if let Some(tooltip) = &attachments.tooltip {
                let tooltip_object = self.runtime.graph().tooltip(object).unwrap();
                let content_object = self
                    .runtime
                    .graph()
                    .child(tooltip_object, RelationId::Content)
                    .unwrap();
                pending.push((tooltip.content.as_object().unwrap().clone(), content_object));
            }
            if let Some(flyout) = &attachments.flyout {
                let content_object = self.runtime.graph().flyout(object).unwrap().0;
                pending.push((flyout.content.as_object().unwrap().clone(), content_object));
            }
            if let Some(dialog) = &attachments.content_dialog {
                let dialog_object = self.runtime.graph().content_dialog(object).unwrap().0;
                pending.push((dialog.declaration.clone(), dialog_object));
            }
        }
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
        render: ComponentRender,
    ) -> Result<UpdateStats, ComponentError<A::Error>> {
        let contexts = self.scope(id).unwrap().contexts.clone();
        let provided_contexts = self.scope(id).unwrap().provided_contexts.clone();
        self.apply_render_with_contexts(id, render, contexts, provided_contexts)
    }

    fn apply_render_with_contexts(
        &mut self,
        id: ComponentId,
        render: ComponentRender,
        contexts: HashMap<ContextId, ContextValue>,
        provided_contexts: HashSet<ContextId>,
    ) -> Result<UpdateStats, ComponentError<A::Error>> {
        let root = self.scope(id).unwrap().root.unwrap();
        let mut expansion = ExpansionState::default();
        expansion.pending.push(PendingScopeRender {
            dependencies: render.dependencies,
            environment: Box::new(OwnedContextEnvironment {
                contexts: contexts.clone(),
                provided: provided_contexts.clone(),
            }),
            effects: render.effects,
            id,
            window: render.window,
        });
        let view = match self.expand_view(
            id,
            render.view,
            &ContextEnvironment {
                contexts: &contexts,
                provided: &provided_contexts,
            },
            0,
            &mut expansion,
        ) {
            Ok(view) => view,
            Err(error) => {
                self.discard_created(&expansion.created);
                return Err(error);
            }
        };
        let retired = self.unseen_scopes(&expansion.seen);
        let publication = self.resolve_window_publication(&expansion.pending, &retired)?;
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
                    pending.environment,
                    pending.dependencies,
                    pending.window,
                ));
            }
            prepare_scope_retirement(scopes, &retired);
        }) {
            Ok(mutations) => mutations,
            Err(error) => {
                for (_, prepared, _, _, _) in prepared {
                    prepared.cancel();
                }
                self.discard_created(&expansion.created);
                return Err(self.runtime_error(error));
            }
        };
        self.refresh_roots_from(view.0.as_object().unwrap(), root);
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
        self.ui_services.publish_window(publication);
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

    fn run_component_operation<T>(
        &mut self,
        operation: impl FnOnce(&mut Self) -> Result<T, ComponentError<A::Error>>,
    ) -> Result<T, ComponentError<A::Error>> {
        match operation(self) {
            Ok(value) => Ok(value),
            Err(error) => {
                self.invalidate_host();
                Err(error)
            }
        }
    }

    fn invalidate_host(&mut self) {
        self.poisoned = true;
        self.runtime.poison_and_discard_all();
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

    fn ensure_active(&self) -> Result<(), ComponentError<A::Error>> {
        if self.poisoned {
            Err(ComponentError::Runtime(UpdateError::Poisoned))
        } else {
            Ok(())
        }
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

    fn runtime_error(&mut self, error: UpdateError<A::Error>) -> ComponentError<A::Error> {
        if matches!(
            &error,
            UpdateError::Adapter(_) | UpdateError::InvalidNativeEvent(_) | UpdateError::Poisoned
        ) {
            self.invalidate_host();
        }
        ComponentError::Runtime(error)
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
    struct LocalMessageInput {
        seen: Rc<Cell<usize>>,
        sender: Rc<RefCell<Option<LocalSender<Rc<Cell<usize>>>>>>,
    }

    impl PartialEq for LocalMessageInput {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.seen, &other.seen) && Rc::ptr_eq(&self.sender, &other.sender)
        }
    }

    struct LocalMessageProbe {
        _timer: ComponentTimer,
        seen: Rc<Cell<usize>>,
    }

    impl Component for LocalMessageProbe {
        type Input = LocalMessageInput;
        type Message = Rc<Cell<usize>>;

        fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
            input.sender.replace(Some(context.sender()));
            Self {
                _timer: context.set_local_timeout(Duration::ZERO, || Rc::new(Cell::new(2))),
                seen: Rc::clone(&input.seen),
            }
        }

        fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
            self.seen.set(message.get());
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            Grid::new().into()
        }
    }

    #[derive(Clone)]
    struct MessageOrderInput {
        local: Rc<RefCell<Option<LocalSender<u8>>>>,
        seen: Rc<RefCell<Vec<u8>>>,
    }

    impl PartialEq for MessageOrderInput {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.local, &other.local) && Rc::ptr_eq(&self.seen, &other.seen)
        }
    }

    struct MessageOrderProbe {
        seen: Rc<RefCell<Vec<u8>>>,
    }

    impl Component for MessageOrderProbe {
        type Input = MessageOrderInput;
        type Message = u8;

        fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
            input.local.replace(Some(context.sender()));
            Self {
                seen: Rc::clone(&input.seen),
            }
        }

        fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
            self.seen.borrow_mut().push(message);
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            Grid::new().into()
        }
    }

    #[derive(Default)]
    struct TestUiServices {
        activated: Cell<usize>,
        closed: Cell<usize>,
        operations: Cell<usize>,
        publications: RefCell<Vec<WindowPublication>>,
        requests: RefCell<Vec<(ComponentNode, WindowPolicy)>>,
    }

    impl ComponentUiServices for TestUiServices {
        fn open_window(&self, root: ComponentNode, policy: WindowPolicy) -> bool {
            self.requests.borrow_mut().push((root, policy));
            true
        }

        fn publish_window(&self, publication: WindowPublication) {
            self.publications.borrow_mut().push(publication);
        }

        fn activate_window(&self) -> bool {
            self.activated.set(self.activated.get() + 1);
            true
        }

        fn close_window(&self) -> bool {
            self.closed.set(self.closed.get() + 1);
            true
        }

        fn run_window(&self, operation: Box<dyn FnOnce(*mut core::ffi::c_void)>) -> bool {
            self.operations.set(self.operations.get() + 1);
            operation(std::ptr::dangling_mut::<core::ffi::c_void>());
            true
        }
    }

    struct WindowContent;

    impl Component for WindowContent {
        type Input = String;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            TextBlock::new().text(input.as_str()).into()
        }
    }

    #[derive(Clone)]
    struct WindowRequesterInput {
        accepted: Rc<Cell<Option<bool>>>,
    }

    impl PartialEq for WindowRequesterInput {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.accepted, &other.accepted)
        }
    }

    struct WindowRequester {
        accepted: Rc<Cell<Option<bool>>>,
    }

    struct WindowPublisher;

    impl Component for WindowPublisher {
        type Input = bool;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            if *input {
                context.window_title("Published");
                context.window_visuals(
                    WindowVisuals::new()
                        .theme(WindowTheme::Dark)
                        .backdrop(WindowBackdrop::Mica)
                        .client_size(640.0, 480.0)
                        .constraints(WindowConstraints {
                            min_width: Some(320.0),
                            min_height: Some(240.0),
                            max_width: Some(1280.0),
                            max_height: Some(960.0),
                        }),
                );
                context.on_color_scheme(|_| {});
                context.on_window_size(|_| {});
            }
            TextBlock::new().text("Window publisher").into()
        }
    }

    struct WindowTitlePublisher;

    impl Component for WindowTitlePublisher {
        type Input = &'static str;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            context.window_title(*input);
            TextBlock::new().text(*input).into()
        }
    }

    enum WindowRequestMessage {
        Activate,
        Close,
        Open,
        Run,
        Window(usize),
    }

    impl Component for WindowRequester {
        type Input = WindowRequesterInput;
        type Message = WindowRequestMessage;

        fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self {
                accepted: Rc::clone(&input.accepted),
            }
        }

        fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
            let accepted = match message {
                WindowRequestMessage::Activate => context.activate_window(),
                WindowRequestMessage::Close => context.close_window(),
                WindowRequestMessage::Open => context.open_window_with_policy::<WindowContent>(
                    "secondary".to_string(),
                    WindowPolicy::new()
                        .title("Secondary")
                        .client_size(320.0, 200.0),
                ),
                WindowRequestMessage::Run => context
                    .run_window(|window| WindowRequestMessage::Window(window.as_raw() as usize)),
                WindowRequestMessage::Window(raw) => raw == 1,
            };
            self.accepted.set(Some(accepted));
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            TextBlock::new().text("primary").into()
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

        fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self { value: input.value }
        }

        fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
            self.value = input.value;
        }

        fn update(&mut self, message: usize, _context: &ComponentContext<Self>) {
            self.value += message;
        }

        fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            let cleanup = Arc::clone(&input.cleanup);
            context.use_effect_guard("value", self.value, move || Cleanup(cleanup));
            TextBlock::new().text(self.value.to_string()).into()
        }
    }

    #[derive(Clone)]
    struct OrderedInput {
        rendered: Rc<RefCell<String>>,
        seen: Rc<RefCell<Vec<(String, String)>>>,
    }

    impl PartialEq for OrderedInput {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.rendered, &other.rendered) && Rc::ptr_eq(&self.seen, &other.seen)
        }
    }

    struct OrderedText {
        callback: Callback<Rc<str>>,
        text: String,
    }

    impl Component for OrderedText {
        type Input = OrderedInput;
        type Message = String;

        fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
            let sender = context.sender();
            let rendered = Rc::clone(&input.rendered);
            let seen = Rc::clone(&input.seen);
            Self {
                callback: Callback::new(move |value: Rc<str>| {
                    seen.borrow_mut()
                        .push((value.to_string(), rendered.borrow().clone()));
                    let _ = sender.send(value.to_string());
                }),
                text: String::from("Before"),
            }
        }

        fn update(&mut self, message: Self::Message, _context: &ComponentContext<Self>) {
            self.text = message;
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            input.rendered.replace(self.text.clone());
            TextBox::new(self.text.clone())
                .on_text_changed(self.callback.clone())
                .into()
        }
    }

    struct Label;

    impl Component for Label {
        type Input = Rc<str>;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            TextBlock::new().text(input.clone()).into()
        }
    }

    struct TooltipParent;

    impl Component for TooltipParent {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            Border::new()
                .content(component::<Label>("label", Rc::from("Label")).tooltip("Help"))
                .into()
        }
    }

    struct ContentDialogParent;

    impl Component for ContentDialogParent {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            Border::new()
                .content(
                    component::<Label>("owner", Rc::from("Owner")).content_dialog(
                        ContentDialog::new()
                            .content(component::<Label>("dialog", Rc::from("Dialog"))),
                    ),
                )
                .into()
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

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            input.renders.fetch_add(1, Ordering::Relaxed);
            let value = if input.subscribe {
                context.use_context(&input.context)
            } else {
                0
            };
            TextBlock::new().text(value.to_string()).into()
        }
    }

    struct ContextParent;

    impl Component for ContextParent {
        type Input = ContextInput;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            input.renders.fetch_add(1, Ordering::Relaxed);
            let _ = context.use_context(&input.context);
            Border::new()
                .content(component::<ContextReader>("reader", input.clone()))
                .into()
        }
    }

    #[derive(Clone)]
    struct ProviderInput {
        context: Rc<Context<usize>>,
        seen: Rc<RefCell<Vec<usize>>>,
        value: usize,
    }

    impl PartialEq for ProviderInput {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.context, &other.context)
                && Rc::ptr_eq(&self.seen, &other.seen)
                && self.value == other.value
        }
    }

    struct ProviderChild;

    impl Component for ProviderChild {
        type Input = ProviderInput;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            let value = context.use_context(&input.context);
            input.seen.borrow_mut().push(value);
            TextBlock::new().text(value.to_string()).into()
        }
    }

    struct ProviderParent;

    impl Component for ProviderParent {
        type Input = ProviderInput;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            provide(
                &input.context,
                input.value,
                Border::new().content(component::<ProviderChild>("child", input.clone())),
            )
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

        fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(input.clone())
        }

        fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
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

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            TextBlock::new().text("Worker").into()
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

        fn create(input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self {
                input: input.clone(),
                value: 0,
            }
        }

        fn update(&mut self, message: Self::Message, context: &ComponentContext<Self>) {
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

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            TextBlock::new().text(self.value.to_string()).into()
        }
    }

    struct RootSwitch(bool);

    impl Component for RootSwitch {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(false)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
            self.0 = true;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            if self.0 {
                Border::new().into()
            } else {
                TextBlock::new().text("Stable").into()
            }
        }
    }

    struct NestedRoot;

    impl Component for NestedRoot {
        type Input = (f64, bool);
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
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

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            Border::new()
                .content(component::<Counter>("counter", input.clone()))
                .into()
        }
    }

    struct ReorderParent(bool);

    impl Component for ReorderParent {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(false)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
            self.0 = !self.0;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            let first = component::<RootSwitch>("first", ()).keyed();
            let second = component::<RootSwitch>("second", ()).keyed();
            if self.0 {
                Grid::new().keyed_children([second, first]).into()
            } else {
                Grid::new().keyed_children([first, second]).into()
            }
        }
    }

    struct RelationOwnedReorder(bool);

    impl Component for RelationOwnedReorder {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(false)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
            self.0 = !self.0;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            let first = keyed("first", View::component::<RootSwitch>(()));
            let second = keyed("second", View::component::<RootSwitch>(()));
            if self.0 {
                Grid::new().keyed_children([second, first]).into()
            } else {
                Grid::new().keyed_children([first, second]).into()
            }
        }
    }

    struct RelationOwnedSiblings;

    impl Component for RelationOwnedSiblings {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            StackPanel::new()
                .children((
                    Border::new().content(View::component::<RootSwitch>(())),
                    Border::new().content(View::component::<RootSwitch>(())),
                ))
                .into()
        }
    }

    struct RelationOwnedTypeSwitch(bool);

    impl Component for RelationOwnedTypeSwitch {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(false)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
            self.0 = !self.0;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            let child = if self.0 {
                View::component::<AlternateRoot>(())
            } else {
                View::component::<RootSwitch>(())
            };
            Border::new().content(child).into()
        }
    }

    struct AlternateRoot;

    impl Component for AlternateRoot {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            Button::new().content("alternate").into()
        }
    }

    struct DuplicateNested;

    impl Component for DuplicateNested {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
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

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(0)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
            self.0 += 1;
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            StackPanel::new()
                .children([
                    TextBlock::new().text(input.clone()).into(),
                    TextBlock::new().text(self.0.to_string()).into(),
                ])
                .into()
        }
    }

    struct TreeComponents(bool);

    impl Component for TreeComponents {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(false)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
            self.0 = !self.0;
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
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

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            if *input == 0 {
                TextBlock::new().text("leaf").into()
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

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, _input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            Grid::new()
                .keyed_children([keyed("relation", component::<RootSwitch>("component", ()))])
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

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
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

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(false)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
            self.0 = true;
        }

        fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
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
                TextBlock::new().text("Stable").into()
            }
        }
    }

    struct InvalidEffectUpdate(bool);

    impl Component for InvalidEffectUpdate {
        type Input = RootSwitchEffects;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(false)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
            self.0 = true;
        }

        fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            let events = Rc::clone(&input.0);
            context.use_effect("root", self.0, move || {
                events.borrow_mut().push(("setup", None));
                Some(Box::new(move || {
                    events.borrow_mut().push(("cleanup", None));
                }))
            });
            if self.0 {
                Grid::new()
                    .keyed_children([
                        keyed("duplicate", TextBlock::new().text("First")),
                        keyed("duplicate", TextBlock::new().text("Second")),
                    ])
                    .into()
            } else {
                TextBlock::new().text("Valid").into()
            }
        }
    }

    #[derive(Clone)]
    struct StatefulInput {
        changes: Rc<Cell<usize>>,
        value: usize,
    }

    impl PartialEq for StatefulInput {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value && Rc::ptr_eq(&self.changes, &other.changes)
        }
    }

    struct StatefulInputComponent;

    impl Component for StatefulInputComponent {
        type Input = StatefulInput;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn input_changed(&mut self, input: &Self::Input, _context: &ComponentContext<Self>) {
            input.changes.set(input.changes.get() + 1);
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            TextBlock::new().text(input.value.to_string()).into()
        }
    }

    struct DropPayload(Arc<AtomicUsize>);

    impl Drop for DropPayload {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::Relaxed);
        }
    }

    struct PayloadComponent;

    impl Component for PayloadComponent {
        type Input = usize;
        type Message = DropPayload;

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            TextBlock::new().text(input.to_string()).into()
        }
    }

    struct CallbackRenderFailure(bool);

    impl Component for CallbackRenderFailure {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self(false)
        }

        fn update(&mut self, (): (), _context: &ComponentContext<Self>) {
            self.0 = true;
        }

        fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            if self.0 {
                Grid::new()
                    .keyed_children([
                        keyed("duplicate", TextBlock::new().text("first")),
                        keyed("duplicate", TextBlock::new().text("second")),
                    ])
                    .into()
            } else {
                Button::new().on_click(context.message(())).into()
            }
        }
    }

    struct StableForward;

    impl Component for StableForward {
        type Input = ();
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, _input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            Button::new().on_click(context.forward()).into()
        }
    }

    struct ControlledApplyAdapter {
        fail_after: Rc<Cell<Option<usize>>>,
        inner: RecordingAdapter,
        successful: Rc<Cell<usize>>,
    }

    impl Adapter for ControlledApplyAdapter {
        type Error = ();

        fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
            self.inner.preview_native_events(events);
        }

        fn pop_native_event(&mut self) -> Option<NativeEvent> {
            self.inner.pop_native_event()
        }

        fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
            self.inner.validate(mutations).map_err(|_| ())
        }

        fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
            if let Some(remaining) = self.fail_after.get() {
                if remaining == 0 {
                    return Err(());
                }
                self.fail_after.set(Some(remaining - 1));
            }
            self.inner.apply(mutations).map_err(|_| ())?;
            self.successful.set(self.successful.get() + 1);
            Ok(())
        }

        fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
            self.inner.focus(object).map_err(|_| ())
        }
    }

    #[derive(Clone)]
    struct VirtualEffectLog(Rc<RefCell<Vec<&'static str>>>);

    impl PartialEq for VirtualEffectLog {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.0, &other.0)
        }
    }

    struct VirtualEffect;

    impl Component for VirtualEffect {
        type Input = VirtualEffectLog;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            let setup = Rc::clone(&input.0);
            let cleanup = Rc::clone(&input.0);
            context.use_effect("virtual", (), move || {
                setup.borrow_mut().push("setup");
                Some(Box::new(move || cleanup.borrow_mut().push("cleanup")))
            });
            TextBlock::new().text("virtual").into()
        }
    }

    struct VirtualParent;

    impl Component for VirtualParent {
        type Input = VirtualEffectLog;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            let log = input.clone();
            ItemsRepeater::new()
                .virtual_source(VirtualSource::new(
                    1,
                    10_000,
                    Key::from,
                    move |index| -> View { component::<VirtualEffect>(index, log.clone()).into() },
                ))
                .into()
        }
    }

    #[derive(Clone)]
    struct VirtualCleanupInput {
        events: Rc<RefCell<Vec<(Option<ObjectId>, bool, ComponentTaskStatus)>>>,
        fail_apply: Rc<Cell<bool>>,
        fail_validate: Rc<Cell<bool>>,
        published: Rc<Cell<bool>>,
        reference: ElementRef,
        task: Arc<Mutex<Option<ComponentTask>>>,
    }

    impl PartialEq for VirtualCleanupInput {
        fn eq(&self, other: &Self) -> bool {
            Rc::ptr_eq(&self.events, &other.events)
                && Rc::ptr_eq(&self.published, &other.published)
                && self.reference == other.reference
                && Arc::ptr_eq(&self.task, &other.task)
        }
    }

    struct VirtualCleanup;

    impl Component for VirtualCleanup {
        type Input = VirtualCleanupInput;
        type Message = ();

        fn create(input: &Self::Input, context: &ComponentContext<Self>) -> Self {
            *input.task.lock().unwrap() = Some(context.spawn_background(|_| ()));
            Self
        }

        fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
            let events = Rc::clone(&input.events);
            let published = Rc::clone(&input.published);
            let reference = input.reference.clone();
            let task = Arc::clone(&input.task);
            context.use_effect("cleanup", (), move || {
                Some(Box::new(move || {
                    events.borrow_mut().push((
                        reference.get(),
                        published.get(),
                        task.lock().unwrap().as_ref().unwrap().status(),
                    ));
                }))
            });
            Button::new().element_ref(&input.reference).into()
        }
    }

    struct VirtualCleanupParent;

    impl Component for VirtualCleanupParent {
        type Input = VirtualCleanupInput;
        type Message = ();

        fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
            Self
        }

        fn view(&self, input: &Self::Input, _context: &mut ViewContext<Self>) -> View {
            let input = input.clone();
            ItemsRepeater::new()
                .virtual_source(VirtualSource::new(1, 1, Key::from, move |_| {
                    component::<VirtualCleanup>("row", input.clone())
                }))
                .into()
        }
    }

    struct VirtualLifecycleAdapter {
        inner: RecordingAdapter,
        fail_apply: Rc<Cell<bool>>,
        fail_validate: Rc<Cell<bool>>,
        published: Rc<Cell<bool>>,
    }

    impl Adapter for VirtualLifecycleAdapter {
        type Error = ();

        fn preview_native_events(&self, events: &mut Vec<NativeEvent>) {
            self.inner.preview_native_events(events);
        }

        fn pop_native_event(&mut self) -> Option<NativeEvent> {
            self.inner.pop_native_event()
        }

        fn validate(&self, mutations: &[Mutation]) -> Result<(), Self::Error> {
            if self.fail_validate.get() {
                Err(())
            } else {
                self.inner.validate(mutations).map_err(|_| ())
            }
        }

        fn apply(&mut self, mutations: &[Mutation]) -> Result<(), Self::Error> {
            self.published.set(true);
            if self.fail_apply.get() {
                Err(())
            } else {
                self.inner.apply(mutations).map_err(|_| ())
            }
        }

        fn focus(&mut self, object: ObjectId) -> Result<bool, Self::Error> {
            self.inner.focus(object).map_err(|_| ())
        }
    }

    fn virtual_cleanup_host(
        fail_validate: bool,
        fail_apply: bool,
    ) -> (
        ComponentHost<VirtualLifecycleAdapter>,
        VirtualCleanupInput,
        ObjectId,
    ) {
        let input = VirtualCleanupInput {
            events: Rc::new(RefCell::new(Vec::new())),
            fail_apply: Rc::new(Cell::new(false)),
            fail_validate: Rc::new(Cell::new(false)),
            published: Rc::new(Cell::new(false)),
            reference: ElementRef::default(),
            task: Arc::new(Mutex::new(None)),
        };
        let adapter = VirtualLifecycleAdapter {
            inner: RecordingAdapter::default(),
            fail_apply: Rc::clone(&input.fail_apply),
            fail_validate: Rc::clone(&input.fail_validate),
            published: Rc::clone(&input.published),
        };
        let mut host = ComponentHost::mount_with_services(
            adapter,
            Arc::new(TestServices::default()),
            [component::<VirtualCleanupParent>("virtual", input.clone())],
        )
        .unwrap();
        let root = host.runtime().graph().root().unwrap();
        let collection = host
            .runtime()
            .graph()
            .children(root, RelationId::Children)
            .unwrap()[0];
        host.runtime
            .adapter_mut()
            .inner
            .queue_realization(RealizationRequest::Realize {
                collection,
                container: RealizedContainer(1),
                index: 0,
                source_revision: 0,
            });
        host.drain(10).unwrap();
        input.published.set(false);
        input.fail_validate.set(fail_validate);
        input.fail_apply.set(fail_apply);
        (host, input, collection)
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
    fn tooltip_attachment_expands_across_component_boundary() {
        let host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<TooltipParent>("parent", ())],
        )
        .unwrap();
        let target = host
            .reference_at(&[Key::from("parent"), Key::from("label")])
            .unwrap()
            .get()
            .unwrap();
        let tooltip = host.runtime().graph().tooltip(target).unwrap();

        assert_eq!(
            host.runtime().adapter().tooltip(target),
            Some((tooltip, TooltipPlacement::Top))
        );
    }

    #[test]
    fn content_dialog_attachment_expands_and_refreshes_component_roots() {
        let host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<ContentDialogParent>("parent", ())],
        )
        .unwrap();
        let owner = host
            .reference_at(&[Key::from("parent"), Key::from("owner")])
            .unwrap()
            .get()
            .unwrap();
        let content = host
            .reference_at(&[Key::from("parent"), Key::from("dialog")])
            .unwrap()
            .get()
            .unwrap();
        let dialog = host.runtime().graph().content_dialog(owner).unwrap().0;

        assert_eq!(
            host.runtime().graph().child(dialog, RelationId::Content),
            Some(content)
        );
        assert_eq!(
            host.runtime().adapter().content_dialog(owner),
            Some((dialog, false))
        );
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
    fn exit_retirement_does_not_retain_component_or_effect_ownership() {
        #[derive(Clone)]
        struct Input(Arc<AtomicUsize>);

        impl PartialEq for Input {
            fn eq(&self, other: &Self) -> bool {
                Arc::ptr_eq(&self.0, &other.0)
            }
        }

        struct Exiting;

        impl Component for Exiting {
            type Input = Input;
            type Message = ();

            fn create(_input: &Self::Input, _context: &ComponentContext<Self>) -> Self {
                Self
            }

            fn view(&self, input: &Self::Input, context: &mut ViewContext<Self>) -> View {
                let cleanup = Arc::clone(&input.0);
                context.use_effect("cleanup", (), move || {
                    Some(Box::new(move || {
                        cleanup.fetch_add(1, Ordering::Relaxed);
                    }))
                });
                Button::new().exit_fade(Duration::from_millis(200)).into()
            }
        }

        let cleanup = Arc::new(AtomicUsize::new(0));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<Exiting>("exiting", Input(Arc::clone(&cleanup)))],
        )
        .unwrap();
        let reference = host.reference(&Key::from("exiting")).unwrap();
        let root = reference.get().unwrap();

        host.remove(&Key::from("exiting")).unwrap();
        assert_eq!(reference.get(), None);
        assert_eq!(cleanup.load(Ordering::Relaxed), 1);
        assert!(host.sender::<Exiting>(&Key::from("exiting")).is_none());
        assert_eq!(host.runtime().graph().retired_count(), 1);
        assert_eq!(host.runtime().adapter().retirement_count(), 1);

        assert!(host.runtime.adapter_mut().complete_retirement(root));
        host.drain(1).unwrap();
        assert_eq!(host.runtime().graph().retired_count(), 0);
        assert_eq!(host.runtime().adapter().retirement_count(), 0);
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
    fn provided_context_shadows_host_context_across_updates() {
        let context = Rc::new(Context::new(0usize));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let input = ProviderInput {
            context: Rc::clone(&context),
            seen: Rc::clone(&seen),
            value: 1,
        };
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<ProviderParent>("parent", input.clone())],
        )
        .unwrap();
        assert_eq!(&*seen.borrow(), &[1]);

        assert_eq!(
            host.set_context(&context, 9).unwrap(),
            ComponentDrain::default()
        );
        assert_eq!(&*seen.borrow(), &[1]);

        host.update_input::<ProviderParent>(
            &Key::from("parent"),
            ProviderInput { value: 2, ..input },
        )
        .unwrap();
        assert_eq!(&*seen.borrow(), &[1, 2]);

        let sender = host
            .sender_at::<ProviderChild>(&[Key::from("parent"), Key::from("child")])
            .unwrap();
        assert!(sender.send(()));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(&*seen.borrow(), &[1, 2, 2]);
    }

    #[test]
    fn broad_context_update_does_not_scan_all_graph_references_per_consumer() {
        let count = 16_384;
        let context = Rc::new(Context::new(0usize));
        let renders = Arc::new(AtomicUsize::new(0));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            (0..count).map(|index| {
                component::<ContextReader>(
                    index,
                    ContextInput {
                        context: Rc::clone(&context),
                        renders: Arc::clone(&renders),
                        subscribe: true,
                    },
                )
            }),
        )
        .unwrap();
        let scans = host.runtime().graph().full_reference_scan_count();

        let report = host.set_context(&context, 1).unwrap();

        assert_eq!(report.dispatched, count);
        assert_eq!(host.runtime().graph().full_reference_scan_count(), scans);
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
    fn ui_local_services_accept_typed_window_requests() {
        let accepted = Rc::new(Cell::new(None));
        let input = WindowRequesterInput {
            accepted: Rc::clone(&accepted),
        };
        let ui_services = Rc::new(TestUiServices::default());
        let mut host = ComponentHost::mount_with_all_services(
            RecordingAdapter::default(),
            Arc::new(DefaultComponentServices),
            ui_services.clone(),
            [component::<WindowRequester>("requester", input.clone())],
        )
        .unwrap();
        let sender = host
            .sender::<WindowRequester>(&Key::from("requester"))
            .unwrap();

        assert!(sender.send(WindowRequestMessage::Open));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(accepted.get(), Some(true));
        let (root, policy) = ui_services.requests.borrow_mut().pop().unwrap();
        assert_eq!(
            policy,
            WindowPolicy::new()
                .title("Secondary")
                .client_size(320.0, 200.0)
        );
        let secondary = ComponentHost::mount(RecordingAdapter::default(), [root]).unwrap();
        let root = secondary
            .reference_at(&[Key::from("root")])
            .unwrap()
            .get()
            .unwrap();
        assert_eq!(
            secondary.runtime().graph().kind(root),
            Some(ObjectType::TextBlock)
        );
        assert!(sender.send(WindowRequestMessage::Activate));
        assert!(sender.send(WindowRequestMessage::Close));
        assert_eq!(host.drain(2).unwrap().dispatched, 2);
        assert_eq!(ui_services.activated.get(), 1);
        assert_eq!(ui_services.closed.get(), 1);
        assert!(sender.send(WindowRequestMessage::Run));
        assert_eq!(host.drain(2).unwrap().dispatched, 2);
        assert_eq!(accepted.get(), Some(true));
        assert_eq!(ui_services.operations.get(), 1);

        let mut headless = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<WindowRequester>("requester", input)],
        )
        .unwrap();
        let sender = headless
            .sender::<WindowRequester>(&Key::from("requester"))
            .unwrap();
        assert!(sender.send(WindowRequestMessage::Open));
        assert_eq!(headless.drain(1).unwrap().dispatched, 1);
        assert_eq!(accepted.get(), Some(false));
    }

    #[test]
    fn component_window_declarations_follow_scope_updates() {
        let ui_services = Rc::new(TestUiServices::default());
        let mut host = ComponentHost::mount_with_all_services(
            RecordingAdapter::default(),
            Arc::new(DefaultComponentServices),
            ui_services.clone(),
            [component::<WindowPublisher>("publisher", true)],
        )
        .unwrap();

        let published = ui_services.publications.borrow().last().unwrap().clone();
        assert_eq!(published.title.as_deref(), Some("Published"));
        assert_eq!(
            published.visuals,
            Some(
                WindowVisuals::new()
                    .theme(WindowTheme::Dark)
                    .backdrop(WindowBackdrop::Mica)
                    .client_size(640.0, 480.0)
                    .constraints(WindowConstraints {
                        min_width: Some(320.0),
                        min_height: Some(240.0),
                        max_width: Some(1280.0),
                        max_height: Some(960.0),
                    })
            )
        );
        assert!(published.on_color_scheme.is_some());
        assert!(published.on_size.is_some());

        host.update_input::<WindowPublisher>(&Key::from("publisher"), false)
            .unwrap();
        assert_eq!(
            ui_services.publications.borrow().last().unwrap(),
            &WindowPublication::default()
        );
    }

    #[test]
    fn duplicate_component_window_declarations_are_rejected() {
        let result = ComponentHost::mount(
            RecordingAdapter::default(),
            [
                component::<WindowTitlePublisher>("first", "First"),
                component::<WindowTitlePublisher>("second", "Second"),
            ],
        );
        assert!(matches!(result, Err(ComponentError::DuplicateWindowTitle)));

        let result = ComponentHost::mount(
            RecordingAdapter::default(),
            [
                component::<WindowPublisher>("first", true),
                component::<WindowPublisher>("second", true),
            ],
        );
        assert!(matches!(
            result,
            Err(ComponentError::DuplicateWindowColorScheme)
        ));
    }

    #[test]
    fn local_messages_and_timers_support_ui_thread_values() {
        let services = Arc::new(TestServices::default());
        let input = LocalMessageInput {
            seen: Rc::new(Cell::new(0)),
            sender: Rc::new(RefCell::new(None)),
        };
        let mut host = ComponentHost::mount_with_services(
            RecordingAdapter::default(),
            services.clone(),
            [component::<LocalMessageProbe>("probe", input.clone())],
        )
        .unwrap();

        let message = Rc::new(Cell::new(1));
        assert!(input.sender.borrow().as_ref().unwrap().send(message));
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(input.seen.get(), 1);

        services.fire_timer();
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(input.seen.get(), 2);
    }

    #[test]
    fn local_and_cross_thread_messages_preserve_enqueue_order() {
        let input = MessageOrderInput {
            local: Rc::new(RefCell::new(None)),
            seen: Rc::new(RefCell::new(Vec::new())),
        };
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<MessageOrderProbe>("probe", input.clone())],
        )
        .unwrap();
        let remote = host
            .sender::<MessageOrderProbe>(&Key::from("probe"))
            .unwrap();
        let local = input.local.borrow().as_ref().unwrap().clone();

        assert!(remote.send(1));
        assert!(local.send(2));
        assert!(local.send(3));
        assert!(remote.send(4));
        assert_eq!(host.drain(4).unwrap().dispatched, 4);
        assert_eq!(&*input.seen.borrow(), &[1, 2, 3, 4]);
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
    fn installing_waker_rearms_queued_messages() {
        let wakes = Arc::new(AtomicUsize::new(0));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<Label>("label", Rc::from("Label"))],
        )
        .unwrap();
        let sender = host.sender::<Label>(&Key::from("label")).unwrap();
        assert!(sender.send(()));
        assert_eq!(wakes.load(Ordering::Relaxed), 0);

        let callback_wakes = Arc::clone(&wakes);
        host.set_waker(move || {
            callback_wakes.fetch_add(1, Ordering::Relaxed);
        });

        assert_eq!(wakes.load(Ordering::Relaxed), 1);
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
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
        host.runtime.adapter_mut().record_batches(true);

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
    fn native_callbacks_reconcile_before_the_next_native_occurrence() {
        let rendered = Rc::new(RefCell::new(String::new()));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<OrderedText>(
                "ordered",
                OrderedInput {
                    rendered: Rc::clone(&rendered),
                    seen: Rc::clone(&seen),
                },
            )],
        )
        .unwrap();
        let object = host
            .reference_at(&[Key::from("ordered")])
            .unwrap()
            .get()
            .unwrap();
        let callback = match &host.runtime().graph().events(object).unwrap()[0].value {
            EventValue::String(callback) => callback.clone(),
            _ => unreachable!(),
        };
        for value in ["A", "B"] {
            host.runtime.adapter_mut().queue_native_event(
                Some(Observation::SetProperty {
                    object,
                    property: Property {
                        id: PropertyId::Text,
                        value: PropertyValue::String(Rc::from(value)),
                    },
                }),
                Some(EventDispatch::new(
                    object,
                    EventId::TextChanged,
                    EventValue::String(callback.clone()),
                    EventPayload::String(Rc::from(value)),
                )),
            );
        }

        let report = host.drain(usize::MAX).unwrap();

        assert_eq!(report.dispatched, 2);
        assert_eq!(
            &*seen.borrow(),
            &[
                (String::from("A"), String::from("Before")),
                (String::from("B"), String::from("A")),
            ]
        );
        assert_eq!(rendered.borrow().as_str(), "B");
    }

    #[test]
    fn drain_budget_counts_native_work_and_rearms() {
        let rendered = Rc::new(RefCell::new(String::new()));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<OrderedText>(
                "ordered",
                OrderedInput {
                    rendered,
                    seen: Rc::clone(&seen),
                },
            )],
        )
        .unwrap();
        let object = host
            .reference_at(&[Key::from("ordered")])
            .unwrap()
            .get()
            .unwrap();
        let callback = match &host.runtime().graph().events(object).unwrap()[0].value {
            EventValue::String(callback) => callback.clone(),
            _ => unreachable!(),
        };
        for value in ["A", "B"] {
            host.runtime.adapter_mut().queue_native_event(
                None,
                Some(EventDispatch::new(
                    object,
                    EventId::TextChanged,
                    EventValue::String(callback.clone()),
                    EventPayload::String(Rc::from(value)),
                )),
            );
        }
        let continuations = Rc::new(Cell::new(0));
        let woken = Rc::clone(&continuations);
        host.set_continuation_waker(move || woken.set(woken.get() + 1));

        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(seen.borrow().len(), 1);
        assert_eq!(continuations.get(), 1);
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(seen.borrow().len(), 2);
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
        host.runtime.adapter_mut().record_batches(true);

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
    fn relation_owned_component_keys_preserve_objects_through_reorder() {
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<RelationOwnedReorder>("parent", ())],
        )
        .unwrap();
        let parent = host.reference(&Key::from("parent")).unwrap().get().unwrap();
        let before = host
            .runtime()
            .graph()
            .children(parent, RelationId::Children)
            .unwrap()
            .to_vec();

        assert!(
            host.sender::<RelationOwnedReorder>(&Key::from("parent"))
                .unwrap()
                .send(())
        );
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        assert_eq!(
            host.runtime()
                .graph()
                .children(parent, RelationId::Children)
                .unwrap(),
            [before[1], before[0]]
        );
    }

    #[test]
    fn relation_owned_component_keys_include_the_complete_native_path() {
        let host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<RelationOwnedSiblings>("parent", ())],
        )
        .unwrap();
        let parent = host.reference(&Key::from("parent")).unwrap().get().unwrap();
        let borders = host
            .runtime()
            .graph()
            .children(parent, RelationId::Children)
            .unwrap();
        let first = host
            .runtime()
            .graph()
            .child(borders[0], RelationId::Content)
            .unwrap();
        let second = host
            .runtime()
            .graph()
            .child(borders[1], RelationId::Content)
            .unwrap();
        assert_ne!(first, second);
    }

    #[test]
    fn relation_owned_component_keys_include_component_type() {
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<RelationOwnedTypeSwitch>("parent", ())],
        )
        .unwrap();
        let parent = host.reference(&Key::from("parent")).unwrap().get().unwrap();
        let before = host
            .runtime()
            .graph()
            .child(parent, RelationId::Content)
            .unwrap();

        assert!(
            host.sender::<RelationOwnedTypeSwitch>(&Key::from("parent"))
                .unwrap()
                .send(())
        );
        assert_eq!(host.drain(1).unwrap().dispatched, 1);
        let after = host
            .runtime()
            .graph()
            .child(parent, RelationId::Content)
            .unwrap();
        assert_ne!(before, after);
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
    fn invalid_update_poisons_host_and_cleans_active_effects() {
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
        assert_eq!(wakes.load(Ordering::Relaxed), 1);
        assert_eq!(
            events.borrow().as_slice(),
            [("setup", None), ("cleanup", None)]
        );
        assert!(
            host.sender::<InvalidEffectUpdate>(&Key::from("invalid"))
                .is_none()
        );
        assert!(matches!(
            host.drain(1),
            Err(ComponentError::Runtime(UpdateError::Poisoned))
        ));

        drop(host);
        assert_eq!(
            events.borrow().as_slice(),
            [("setup", None), ("cleanup", None)]
        );
    }

    #[test]
    fn input_state_change_followed_by_adapter_error_poisons_same_input_retry() {
        let fail_after = Rc::new(Cell::new(None));
        let successful = Rc::new(Cell::new(0));
        let changes = Rc::new(Cell::new(0));
        let mut host = ComponentHost::mount(
            ControlledApplyAdapter {
                fail_after: Rc::clone(&fail_after),
                inner: RecordingAdapter::default(),
                successful,
            },
            [component::<StatefulInputComponent>(
                "input",
                StatefulInput {
                    changes: Rc::clone(&changes),
                    value: 0,
                },
            )],
        )
        .unwrap();
        let reference = host.reference(&Key::from("input")).unwrap();
        fail_after.set(Some(0));
        let next = StatefulInput {
            changes: Rc::clone(&changes),
            value: 1,
        };

        assert!(matches!(
            host.update_input::<StatefulInputComponent>(&Key::from("input"), next.clone()),
            Err(ComponentError::Runtime(UpdateError::Adapter(())))
        ));
        assert_eq!(changes.get(), 1);
        assert_eq!(reference.get(), None);
        assert!(matches!(
            host.update_input::<StatefulInputComponent>(&Key::from("input"), next),
            Err(ComponentError::Runtime(UpdateError::Poisoned))
        ));
    }

    #[test]
    fn closed_queue_rejects_stale_sender_completion_and_callback() {
        let fail_after = Rc::new(Cell::new(None));
        let rendered = Rc::new(RefCell::new(String::new()));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let input = OrderedInput {
            rendered: Rc::clone(&rendered),
            seen: Rc::clone(&seen),
        };
        let changes = Rc::new(Cell::new(0));
        let mut host = ComponentHost::mount(
            ControlledApplyAdapter {
                fail_after: Rc::clone(&fail_after),
                inner: RecordingAdapter::default(),
                successful: Rc::new(Cell::new(0)),
            },
            [
                component::<OrderedText>("ordered", input),
                component::<StatefulInputComponent>(
                    "poison",
                    StatefulInput {
                        changes: Rc::clone(&changes),
                        value: 0,
                    },
                ),
            ],
        )
        .unwrap();
        let sender = host.sender::<OrderedText>(&Key::from("ordered")).unwrap();
        let completion = sender.completion();
        let object = host
            .reference(&Key::from("ordered"))
            .unwrap()
            .get()
            .unwrap();
        let EventValue::String(callback) = host.runtime().graph().events(object).unwrap()[0]
            .value
            .clone()
        else {
            panic!("expected string callback");
        };
        fail_after.set(Some(0));

        assert!(matches!(
            host.update_input::<StatefulInputComponent>(
                &Key::from("poison"),
                StatefulInput { changes, value: 1 }
            ),
            Err(ComponentError::Runtime(UpdateError::Adapter(())))
        ));
        assert!(host.queue.lock().unwrap().closed);
        assert!(!sender.send(String::from("sender")));
        assert!(!completion.complete(String::from("completion")));
        callback.call(Rc::from("callback"));
        assert!(host.queue.lock().unwrap().messages.is_empty());
        assert_eq!(
            seen.borrow().as_slice(),
            [(String::from("callback"), String::from("Before"))]
        );
    }

    #[test]
    fn queue_capacity_and_closure_release_all_payloads() {
        let fail_after = Rc::new(Cell::new(None));
        let drops = Arc::new(AtomicUsize::new(0));
        let mut host = ComponentHost::mount(
            ControlledApplyAdapter {
                fail_after: Rc::clone(&fail_after),
                inner: RecordingAdapter::default(),
                successful: Rc::new(Cell::new(0)),
            },
            [component::<PayloadComponent>("payload", 0)],
        )
        .unwrap();
        let sender = host
            .sender::<PayloadComponent>(&Key::from("payload"))
            .unwrap();
        for _ in 0..MESSAGE_CAPACITY {
            assert!(sender.send(DropPayload(Arc::clone(&drops))));
        }
        assert!(!sender.send(DropPayload(Arc::clone(&drops))));
        assert_eq!(drops.load(Ordering::Relaxed), 1);
        fail_after.set(Some(0));

        assert!(matches!(
            host.update_input::<PayloadComponent>(&Key::from("payload"), 1),
            Err(ComponentError::Runtime(UpdateError::Adapter(())))
        ));
        assert_eq!(drops.load(Ordering::Relaxed), MESSAGE_CAPACITY + 1);
        assert!(!sender.send(DropPayload(Arc::clone(&drops))));
        assert_eq!(drops.load(Ordering::Relaxed), MESSAGE_CAPACITY + 2);
        assert!(host.queue.lock().unwrap().messages.is_empty());
    }

    #[test]
    fn controlled_task_send_after_closure_is_cancelled_or_rejected() {
        let services = Arc::new(TestServices::default());
        let task_input = ServiceProbeInput::default();
        let fail_after = Rc::new(Cell::new(None));
        let changes = Rc::new(Cell::new(0));
        let mut host = ComponentHost::mount_with_services(
            ControlledApplyAdapter {
                fail_after: Rc::clone(&fail_after),
                inner: RecordingAdapter::default(),
                successful: Rc::new(Cell::new(0)),
            },
            services,
            [
                component::<ServiceProbe>("task", task_input.clone()),
                component::<StatefulInputComponent>(
                    "poison",
                    StatefulInput {
                        changes: Rc::clone(&changes),
                        value: 0,
                    },
                ),
            ],
        )
        .unwrap();
        let sender = host.sender::<ServiceProbe>(&Key::from("task")).unwrap();
        assert!(sender.send(ServiceProbeMessage::Background));
        host.drain(1).unwrap();
        let task = task_input.task.lock().unwrap().as_ref().unwrap().clone();
        let control = Arc::clone(&task.control);
        assert!(control.queue());
        fail_after.set(Some(0));

        assert!(matches!(
            host.update_input::<StatefulInputComponent>(
                &Key::from("poison"),
                StatefulInput { changes, value: 1 }
            ),
            Err(ComponentError::Runtime(UpdateError::Adapter(())))
        ));
        assert_eq!(task.status(), ComponentTaskStatus::Cancelled);
        assert!(!sender.send_controlled(
            ServiceProbeMessage::BackgroundComplete,
            Arc::clone(&control)
        ));
        assert_eq!(control.status(), ComponentTaskStatus::Cancelled);

        let untracked = Arc::new(TaskControl::default());
        assert!(untracked.queue());
        assert!(!sender.send_controlled(
            ServiceProbeMessage::BackgroundComplete,
            Arc::clone(&untracked)
        ));
        assert_eq!(untracked.status(), ComponentTaskStatus::Rejected);
    }

    #[test]
    fn context_failure_after_one_consumer_commit_poisons_every_consumer() {
        let context = Rc::new(Context::new(0usize));
        let fail_after = Rc::new(Cell::new(None));
        let successful = Rc::new(Cell::new(0));
        let first_renders = Arc::new(AtomicUsize::new(0));
        let second_renders = Arc::new(AtomicUsize::new(0));
        let mut host = ComponentHost::mount(
            ControlledApplyAdapter {
                fail_after: Rc::clone(&fail_after),
                inner: RecordingAdapter::default(),
                successful: Rc::clone(&successful),
            },
            [
                component::<ContextReader>(
                    "first",
                    ContextInput {
                        context: Rc::clone(&context),
                        renders: Arc::clone(&first_renders),
                        subscribe: true,
                    },
                ),
                component::<ContextReader>(
                    "second",
                    ContextInput {
                        context: Rc::clone(&context),
                        renders: Arc::clone(&second_renders),
                        subscribe: true,
                    },
                ),
            ],
        )
        .unwrap();
        let first = host.reference(&Key::from("first")).unwrap();
        let second = host.reference(&Key::from("second")).unwrap();
        let baseline = successful.get();
        fail_after.set(Some(1));

        assert!(matches!(
            host.set_context(&context, 1),
            Err(ComponentError::Runtime(UpdateError::Adapter(())))
        ));
        assert_eq!(successful.get(), baseline + 1);
        assert_eq!(first_renders.load(Ordering::Relaxed), 2);
        assert_eq!(second_renders.load(Ordering::Relaxed), 2);
        assert_eq!(first.get(), None);
        assert_eq!(second.get(), None);
        assert!(matches!(
            host.set_context(&context, 1),
            Err(ComponentError::Runtime(UpdateError::Poisoned))
        ));
    }

    #[test]
    fn callback_triggered_render_failure_poisons_host() {
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<CallbackRenderFailure>("callback", ())],
        )
        .unwrap();
        let reference = host.reference(&Key::from("callback")).unwrap();
        let object = reference.get().unwrap();
        let event = host.runtime().graph().events(object).unwrap()[0].clone();
        host.queue_event(EventDispatch::new(
            object,
            event.id,
            event.value,
            EventPayload::Unit,
        ));

        assert!(matches!(
            host.drain(usize::MAX),
            Err(ComponentError::Runtime(UpdateError::Graph(
                GraphError::DuplicateKey(_)
            )))
        ));
        assert_eq!(reference.get(), None);
        assert!(matches!(
            host.drain(1),
            Err(ComponentError::Runtime(UpdateError::Poisoned))
        ));
    }

    #[test]
    fn captureless_component_callbacks_keep_event_identity() {
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<StableForward>("callback", ())],
        )
        .unwrap();
        let object = host
            .reference(&Key::from("callback"))
            .unwrap()
            .get()
            .unwrap();
        let before = host.runtime().graph().events(object).unwrap()[0].clone();
        host.queue_event(EventDispatch::new(
            object,
            before.id,
            before.value.clone(),
            EventPayload::Unit,
        ));

        assert_eq!(host.drain(usize::MAX).unwrap().dispatched, 1);
        let after = host.runtime().graph().events(object).unwrap()[0].clone();
        assert_eq!(before.value, after.value);
    }

    #[test]
    fn virtual_rows_own_components_only_while_realized() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<VirtualParent>(
                "virtual",
                VirtualEffectLog(Rc::clone(&events)),
            )],
        )
        .unwrap();
        let root = host.runtime().graph().root().unwrap();
        let collection = host
            .runtime()
            .graph()
            .children(root, RelationId::Children)
            .unwrap()[0];
        assert_eq!(host.runtime().adapter().object_count(), 2);

        host.runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container: RealizedContainer(1),
                index: 9_999,
                source_revision: 0,
            });
        host.drain(10).unwrap();
        assert_eq!(events.borrow().as_slice(), ["setup"]);
        assert_eq!(host.runtime().adapter().object_count(), 3);

        host.runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container: RealizedContainer(1),
                source_revision: 0,
            });
        host.drain(10).unwrap();
        assert_eq!(events.borrow().as_slice(), ["setup", "cleanup"]);
        assert_eq!(host.runtime().adapter().object_count(), 2);
    }

    #[test]
    fn pending_virtual_work_precedes_queued_component_messages() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<VirtualParent>(
                "virtual",
                VirtualEffectLog(Rc::clone(&events)),
            )],
        )
        .unwrap();
        let root = host.runtime().graph().root().unwrap();
        let collection = host
            .runtime()
            .graph()
            .children(root, RelationId::Children)
            .unwrap()[0];

        host.runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container: RealizedContainer(1),
                index: 9_999,
                source_revision: 0,
            });
        assert!(
            host.sender::<VirtualParent>(&Key::from("virtual"))
                .unwrap()
                .send(())
        );

        assert_eq!(host.drain(10).unwrap().dispatched, 1);
        assert_eq!(events.borrow().as_slice(), ["setup"]);
    }

    #[test]
    fn component_update_drains_pending_virtual_work_exactly_once() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let input = VirtualEffectLog(Rc::clone(&events));
        let mut host = ComponentHost::mount(
            RecordingAdapter::default(),
            [component::<VirtualParent>("virtual", input.clone())],
        )
        .unwrap();
        let root = host.runtime().graph().root().unwrap();
        let collection = host
            .runtime()
            .graph()
            .children(root, RelationId::Children)
            .unwrap()[0];

        host.runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Realize {
                collection,
                container: RealizedContainer(1),
                index: 9_999,
                source_revision: 0,
            });
        host.update_input::<VirtualParent>(&Key::from("virtual"), input.clone())
            .unwrap();
        assert_eq!(events.borrow().as_slice(), ["setup"]);
        assert_eq!(host.runtime().adapter().object_count(), 3);
        host.drain(10).unwrap();
        assert_eq!(events.borrow().as_slice(), ["setup"]);

        host.runtime
            .adapter_mut()
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container: RealizedContainer(1),
                source_revision: 0,
            });
        host.update_input::<VirtualParent>(&Key::from("virtual"), input)
            .unwrap();
        assert_eq!(events.borrow().as_slice(), ["setup", "cleanup"]);
        assert_eq!(host.runtime().adapter().object_count(), 2);
        host.drain(10).unwrap();
        assert_eq!(events.borrow().as_slice(), ["setup", "cleanup"]);
    }

    #[test]
    fn virtual_cleanup_precedes_native_recycle_publication() {
        let (mut host, input, collection) = virtual_cleanup_host(false, false);
        let object = input.reference.get();
        assert!(object.is_some());
        assert_eq!(
            input.task.lock().unwrap().as_ref().unwrap().status(),
            ComponentTaskStatus::Running
        );

        host.runtime
            .adapter_mut()
            .inner
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container: RealizedContainer(1),
                source_revision: 0,
            });
        host.drain(10).unwrap();

        assert_eq!(
            input.events.borrow().as_slice(),
            [(object, false, ComponentTaskStatus::Cancelled)]
        );
        assert!(input.published.get());
        assert_eq!(input.reference.get(), None);
    }

    #[test]
    fn virtual_cleanup_is_committed_once_when_native_recycle_apply_fails() {
        let (mut host, input, collection) = virtual_cleanup_host(false, true);
        let object = input.reference.get();

        host.runtime
            .adapter_mut()
            .inner
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container: RealizedContainer(1),
                source_revision: 0,
            });
        assert!(matches!(
            host.drain(10),
            Err(ComponentError::Runtime(UpdateError::Adapter(())))
        ));
        assert_eq!(
            input.events.borrow().as_slice(),
            [(object, false, ComponentTaskStatus::Cancelled)]
        );
        assert!(input.published.get());
        assert_eq!(input.reference.get(), None);

        drop(host);
        assert_eq!(input.events.borrow().len(), 1);
    }

    #[test]
    fn virtual_cleanup_waits_for_successful_recycle_validation() {
        let (mut host, input, collection) = virtual_cleanup_host(true, false);

        host.runtime
            .adapter_mut()
            .inner
            .queue_realization(RealizationRequest::Recycle {
                collection,
                container: RealizedContainer(1),
                source_revision: 0,
            });
        assert!(matches!(
            host.drain(10),
            Err(ComponentError::Runtime(UpdateError::Adapter(())))
        ));
        assert_eq!(input.events.borrow().len(), 1);
        assert!(!input.published.get());
        assert_eq!(
            input.task.lock().unwrap().as_ref().unwrap().status(),
            ComponentTaskStatus::Cancelled
        );

        drop(host);
        assert_eq!(input.events.borrow().len(), 1);
    }
}
