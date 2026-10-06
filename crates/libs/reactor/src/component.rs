use super::*;

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

impl fmt::Debug for ContextProvision {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    marker: PhantomData<fn(M)>,
}

impl<M> Clone for LocalSender<M> {
    fn clone(&self) -> Self {
        Self {
            component: self.component,
            queue: Rc::clone(&self.queue),
            wake: Arc::clone(&self.wake),
            marker: PhantomData,
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
    marker: PhantomData<fn(M)>,
}

impl<M> Clone for ComponentSender<M> {
    fn clone(&self) -> Self {
        Self {
            component: self.component,
            queue: Arc::clone(&self.queue),
            marker: PhantomData,
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
    tasks: Arc<Mutex<Vec<SyncWeak<TaskControl>>>>,
    ui_services: Rc<dyn ComponentUiServices>,
}

pub struct WindowHandle<'a> {
    raw: *mut core::ffi::c_void,
    marker: PhantomData<(&'a mut (), Rc<()>)>,
}

impl WindowHandle<'_> {
    fn new(raw: *mut core::ffi::c_void) -> Self {
        Self {
            raw,
            marker: PhantomData,
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
    pub(crate) on_placement: Option<Callback<WindowPlacement>>,
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
    WindowPlacement,
    WindowTitle,
    WindowVisuals,
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

    /// Returns whether the result message was rejected because the component queue was full
    /// or closed. Rejection is terminal; the result is not retried.
    ///
    /// This is a snapshot, not a notification. `false` does not imply successful delivery:
    /// the task may still be running, queued, delivered, or cancelled.
    pub fn is_rejected(&self) -> bool {
        self.control.status() == ComponentTaskStatus::Rejected
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

    /// Returns whether the timer message was rejected because the component queue was full
    /// or closed. Rejection is terminal; the message is not retried.
    ///
    /// This is a snapshot, not a notification. `false` does not imply successful delivery:
    /// the timer may still be waiting, queued, delivered, or cancelled.
    pub fn is_rejected(&self) -> bool {
        self.task.is_rejected()
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
    window_placement_duplicate: bool,
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

    /// Observes restored outer bounds and maximized state, including the initial placement.
    ///
    /// Notifications are coalesced, exclude minimized state, and stop when the window or
    /// subscribing component closes. Persist values in this callback's message handler rather
    /// than relying on a shutdown notification. Repeated unchanged declarations do not replay
    /// the current placement.
    pub fn on_window_placement(&mut self, callback: impl IntoPayloadCallback<WindowPlacement>) {
        self.window_placement_duplicate |= self
            .window
            .on_placement
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
        &mut self,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<ComponentRender, ComponentDeclarationError>;
    fn cached_view(&self) -> View;
    #[cfg(any(test, feature = "test"))]
    fn sender(&self) -> &dyn Any;
    #[cfg(any(test, feature = "test"))]
    fn tracked_tasks(&self) -> usize;
}

struct TypedScope<C: Component> {
    component: C,
    input: C::Input,
    view: Option<View>,
    local_sender: LocalSender<C::Message>,
    sender: ComponentSender<C::Message>,
    services: Arc<dyn ComponentServices>,
    tasks: Arc<Mutex<Vec<SyncWeak<TaskControl>>>>,
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
        &mut self,
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
            window_placement_duplicate: false,
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
        if context.window_placement_duplicate {
            return Err(ComponentDeclarationError::WindowPlacement);
        }
        if context.window_title_duplicate {
            return Err(ComponentDeclarationError::WindowTitle);
        }
        if context.window_visuals_duplicate {
            return Err(ComponentDeclarationError::WindowVisuals);
        }
        self.view = Some(view.clone());
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
        &mut self,
        reference: ElementRef,
        contexts: &HashMap<ContextId, ContextValue>,
    ) -> Result<ComponentRender, ComponentDeclarationError> {
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

impl Scope {
    fn dependencies_changed(&self, contexts: &HashMap<ContextId, ContextValue>) -> bool {
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

impl ExpansionState {
    fn push_render(
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
    DuplicateWindowPlacement,
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
            ComponentDeclarationError::WindowPlacement => Self::DuplicateWindowPlacement,
            ComponentDeclarationError::WindowTitle => Self::DuplicateWindowTitle,
            ComponentDeclarationError::WindowVisuals => Self::DuplicateWindowVisuals,
        }
    }
}

impl<E: fmt::Debug> From<ComponentError<E>> for windows_core::Error {
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
            if let Some(on_placement) = &window.on_placement
                && publication
                    .on_placement
                    .replace(on_placement.clone())
                    .is_some()
            {
                return Err(ComponentError::DuplicateWindowPlacement);
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
                let mut expansion = ExpansionState::default();
                let (id, view, created_row) = if let Some(id) = existing {
                    let reference = self.scope(id).unwrap().reference.clone();
                    let view = {
                        let scope = self.scope_mut(id).unwrap();
                        let render = scope
                            .component
                            .apply_input(&view, reference.clone(), &contexts)
                            .map_err(ComponentError::from)?;
                        let render = match render {
                            Some(render) => Some(render),
                            None if scope.dependencies_changed(&contexts) => Some(
                                scope
                                    .component
                                    .render_view(reference, &contexts)
                                    .map_err(ComponentError::from)?,
                            ),
                            None => None,
                        };
                        if let Some(render) = render {
                            expansion.push_render(
                                id,
                                render,
                                OwnedContextEnvironment {
                                    contexts: contexts.clone(),
                                    provided: provided_contexts.clone(),
                                },
                            )
                        } else {
                            scope.component.cached_view()
                        }
                    };
                    (id, view, false)
                } else {
                    let node = component::<VirtualRow>(lease.key.clone(), view);
                    let (id, render) = self.create_scope(
                        owner,
                        node,
                        contexts.clone(),
                        provided_contexts.clone(),
                    )?;
                    let view = expansion.push_render(
                        id,
                        render,
                        OwnedContextEnvironment {
                            contexts: contexts.clone(),
                            provided: provided_contexts.clone(),
                        },
                    );
                    (id, view, true)
                };
                self.scope_mut(id).unwrap().contexts = contexts;
                self.scope_mut(id).unwrap().provided_contexts = provided_contexts;
                let declaration = match self.expand_virtual_view(id, view, &mut expansion) {
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
                    let environment = *pending.environment;
                    self.scope_mut(pending.id).unwrap().window = pending.window;
                    self.scope_mut(pending.id).unwrap().contexts = environment.contexts;
                    self.scope_mut(pending.id).unwrap().provided_contexts = environment.provided;
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
        self.expand_declaration_in_place(
            owner,
            &mut declaration,
            environment,
            scope_root,
            location,
            expansion,
        )?;
        Ok(declaration)
    }

    fn expand_declaration_in_place(
        &mut self,
        owner: ComponentId,
        declaration: &mut Declaration,
        environment: &ContextEnvironment<'_>,
        scope_root: bool,
        location: ExpansionLocation<'_>,
        expansion: &mut ExpansionState,
    ) -> Result<(), ComponentError<A::Error>> {
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
                        )
                        .map(|()| None)
                    } else {
                        let child = Rc::try_unwrap(retained.take().unwrap())
                            .unwrap_or_else(|child| child.as_ref().clone());
                        self.expand_node(
                            owner,
                            child,
                            environment,
                            location.depth + 1,
                            expansion,
                            location.path,
                        )
                        .map(Some)
                    };
                    location.path.pop();
                    match result? {
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
                            DeclaredNode::Object(declaration) => self
                                .expand_declaration_in_place(
                                    owner,
                                    declaration,
                                    environment,
                                    false,
                                    ExpansionLocation {
                                        depth: location.depth + 1,
                                        path: location.path,
                                    },
                                    expansion,
                                )
                                .map(|()| None),
                            _ => self
                                .expand_node(
                                    owner,
                                    child.clone(),
                                    environment,
                                    location.depth + 1,
                                    expansion,
                                    location.path,
                                )
                                .map(Some),
                        };
                        location.path.pop();
                        if let Some(expanded) = result? {
                            *child = DeclaredNode::Object(expanded);
                        }
                    }
                    RelationValue::Many(children)
                }
                RelationValue::One(None) => RelationValue::One(None),
            };
        }
        declaration.relations = relations;
        Ok(())
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
        let (id, view) = if let Some(id) = existing {
            if self.scope(id).unwrap().component.component_type() != node.factory.component_type() {
                return Err(ComponentError::ComponentType(component_key));
            }
            let scope = self.scope_mut(id).unwrap();
            let reference = scope.reference.clone();
            let render = scope
                .component
                .apply_input(
                    node.factory.input(),
                    reference.clone(),
                    environment.contexts,
                )
                .map_err(ComponentError::from)?;
            let render = match render {
                Some(render) => Some(render),
                None if scope.dependencies_changed(environment.contexts) => Some(
                    scope
                        .component
                        .render_view(reference, environment.contexts)
                        .map_err(ComponentError::from)?,
                ),
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
            )?;
            expansion.created.push(created.0);
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
        Ok(Box::new(PreparedComponentExpansion {
            attachments,
            declaration_key,
            id,
            view: Some(view),
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
        if let Some(attachments) = &declaration.attachments {
            if let Some(tooltip) = &attachments.tooltip {
                let tooltip_object = self.runtime.graph().tooltip(object).unwrap();
                let content_object = self
                    .runtime
                    .graph()
                    .child(tooltip_object, RelationId::Content)
                    .unwrap();
                self.refresh_roots_from(tooltip.content.as_object().unwrap(), content_object);
            }
            if let Some(flyout) = &attachments.flyout {
                let content_object = self.runtime.graph().flyout(object).unwrap().0;
                self.refresh_roots_from(flyout.content.as_object().unwrap(), content_object);
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
                    self.refresh_roots_from(child.as_object().unwrap(), object);
                }
                RelationValue::Many(children) => {
                    for (index, child) in children.iter().enumerate() {
                        let object =
                            self.runtime.graph().children(object, relation.id).unwrap()[index];
                        self.refresh_roots_from(child.as_object().unwrap(), object);
                    }
                }
                RelationValue::One(None) => {}
            }
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
        let view = expansion.push_render(
            id,
            render,
            OwnedContextEnvironment {
                contexts: contexts.clone(),
                provided: provided_contexts.clone(),
            },
        );
        let view = match self.expand_view(
            id,
            view,
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
#[path = "tests/component/mod.rs"]
mod tests;

#[cfg(any(test, feature = "test"))]
#[path = "test_support/component.rs"]
mod test_support;
#[cfg(any(test, feature = "test"))]
pub use test_support::*;
