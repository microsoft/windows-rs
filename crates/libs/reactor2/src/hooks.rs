use std::any::Any;
use std::cell::{Cell, RefCell};
use std::marker::PhantomData;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Weak as SyncWeak};
use std::time::Duration;

use crate::element::{Callback, Element};
use crate::id::NodeId;
use crate::references::ElementRef;
use crate::resources::{Context, ContextDefaults, ContextEntry, ContextKey};

pub(crate) type RenderFn = Rc<dyn for<'a> Fn(&mut RenderCx<'a>) -> Element>;

#[derive(Clone)]
pub(crate) struct ComponentMemo {
    value: Rc<dyn Any>,
    equal: fn(&dyn Any, &dyn Any) -> bool,
}

impl ComponentMemo {
    pub(crate) fn new<T: PartialEq + 'static>(value: T) -> Self {
        Self::from_rc(Rc::new(value))
    }

    pub(crate) fn from_rc<T: PartialEq + 'static>(value: Rc<T>) -> Self {
        fn equal<T: PartialEq + 'static>(left: &dyn Any, right: &dyn Any) -> bool {
            left.downcast_ref::<T>() == right.downcast_ref::<T>()
        }

        Self {
            value,
            equal: equal::<T>,
        }
    }

    pub(crate) fn equivalent(&self, other: &Self) -> bool {
        self.value.as_ref().type_id() == other.value.as_ref().type_id()
            && (self.equal)(&*self.value, &*other.value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum TimerKind {
    Timeout,
    Interval,
}

pub(crate) struct TimerSlot {
    pub kind: TimerKind,
    pub revision: Rc<Cell<u64>>,
    callback: TimerCallback,
}

enum TimerCallback {
    Once(Option<Box<dyn FnOnce()>>),
    Repeating(Rc<dyn Fn()>),
}

pub(crate) enum TimerInvocation {
    Once(Box<dyn FnOnce()>),
    Repeating(Rc<dyn Fn()>),
}

#[derive(Clone)]
pub struct CancellationToken(Arc<AtomicBool>);

impl CancellationToken {
    pub(crate) fn new() -> Self {
        Self(Arc::new(AtomicBool::new(false)))
    }

    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Acquire)
    }

    fn cancel(&self) {
        self.0.store(true, Ordering::Release);
    }
}

#[derive(Debug)]
pub enum Resource<T> {
    Loading,
    Ready(Rc<T>),
    Failed(windows_core::Error),
}

impl<T> Clone for Resource<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Loading => Self::Loading,
            Self::Ready(value) => Self::Ready(Rc::clone(value)),
            Self::Failed(error) => Self::Failed(error.clone()),
        }
    }
}

pub(crate) struct ResourceTask {
    pub owner: NodeId,
    pub slot: u32,
    pub revision: u64,
    pub work: Box<dyn FnOnce() -> Box<dyn Any + Send> + Send>,
}

pub(crate) type AsyncSetFn = Arc<dyn Fn(Box<dyn Any + Send>) -> bool + Send + Sync>;

pub(crate) type Cleanup = Box<dyn FnOnce()>;

pub(crate) type Effect = Box<dyn FnOnce() -> Option<Cleanup>>;

pub(crate) trait RenderScheduler {
    fn invalidate(&self, node: NodeId);
    fn activate_window(&self, node: NodeId);
    fn focus_element(&self, node: NodeId);
    fn run_composition_action(&self, node: NodeId, action: crate::composition::CompositionAction);
    #[cfg(feature = "canvas")]
    fn invalidate_canvas(&self, node: NodeId, revision: u64);
    #[cfg(feature = "canvas")]
    fn run_swap_chain_host_action(&self, node: NodeId, action: crate::canvas::SwapChainHostAction);
    #[cfg(feature = "webview")]
    fn run_webview_action(&self, node: NodeId, action: crate::webview::WebViewAction);
    fn queue_effect(&self, effect: Weak<HookCell>);
    fn start_timer(
        &self,
        owner: NodeId,
        slot: u32,
        revision: u64,
        interval: Duration,
        repeating: bool,
    );
    fn stop_timer(&self, owner: NodeId, slot: u32, revision: u64);
    fn queue_resource(&self, resource: Weak<HookCell>);
    fn launch_resource(&self, task: ResourceTask);
    fn async_setter(&self, owner: NodeId, slot: u32, live: SyncWeak<()>) -> AsyncSetFn;
}

pub(crate) type SchedulerRef = Rc<dyn RenderScheduler>;

#[derive(Clone, Copy)]
struct EffectVtable {
    commit: fn(&HookCell),
    take_cleanup: fn(&HookCell) -> Option<Cleanup>,
}

#[derive(Clone, Copy)]
struct ResourceVtable {
    commit: fn(&HookCell),
    take_cleanup: fn(&HookCell) -> Option<Cleanup>,
    accept: fn(&HookCell, u64, Box<dyn Any + Send>) -> bool,
}

#[derive(Clone, Copy)]
struct AsyncStateVtable {
    accept: fn(&HookCell, Box<dyn Any + Send>) -> bool,
}

pub(crate) struct HookCell {
    pub(crate) value: RefCell<Box<dyn Any>>,
    effect: Option<EffectVtable>,
    resource: Option<ResourceVtable>,
    async_state: Option<AsyncStateVtable>,
}

impl HookCell {
    pub(crate) fn value(value: impl Any) -> Self {
        Self {
            value: RefCell::new(Box::new(value)),
            effect: None,
            resource: None,
            async_state: None,
        }
    }

    pub(crate) fn effect<D: 'static>(slot: EffectSlot<D>) -> Self {
        Self {
            value: RefCell::new(Box::new(slot)),
            effect: Some(EffectVtable {
                commit: commit_effect::<D>,
                take_cleanup: take_effect_cleanup::<D>,
            }),
            resource: None,
            async_state: None,
        }
    }

    pub(crate) fn resource<D: 'static, T: Send + 'static>(slot: ResourceSlot<D, T>) -> Self {
        Self {
            value: RefCell::new(Box::new(slot)),
            effect: None,
            resource: Some(ResourceVtable {
                commit: commit_resource::<D, T>,
                take_cleanup: take_resource_cleanup::<D, T>,
                accept: accept_resource::<D, T>,
            }),
            async_state: None,
        }
    }

    pub(crate) fn async_state<T: Send + 'static>(slot: AsyncStateSlot<T>) -> Self {
        Self {
            value: RefCell::new(Box::new(slot)),
            effect: None,
            resource: None,
            async_state: Some(AsyncStateVtable {
                accept: accept_async_state::<T>,
            }),
        }
    }

    pub(crate) fn with_mut<T: Any, U>(&self, f: impl FnOnce(&mut T) -> U) -> U {
        let mut value = self.value.borrow_mut();
        f(value.downcast_mut::<T>().unwrap())
    }

    pub(crate) fn commit_effect(&self) {
        if let Some(effect) = self.effect {
            (effect.commit)(self);
        }
    }

    pub(crate) fn take_effect_cleanup(&self) -> Option<Cleanup> {
        self.effect.and_then(|effect| (effect.take_cleanup)(self))
    }

    pub(crate) fn has_effect(&self) -> bool {
        self.effect.is_some()
    }

    pub(crate) fn take_timer_callback(&self, revision: u64) -> Option<TimerInvocation> {
        let mut value = self.value.borrow_mut();
        let slot = value.downcast_mut::<TimerSlot>()?;
        if slot.revision.get() != revision {
            return None;
        }
        match &mut slot.callback {
            TimerCallback::Once(callback) => callback.take().map(TimerInvocation::Once),
            TimerCallback::Repeating(callback) => {
                Some(TimerInvocation::Repeating(Rc::clone(callback)))
            }
        }
    }

    pub(crate) fn commit_resource(&self) {
        if let Some(resource) = self.resource {
            (resource.commit)(self);
        }
    }

    pub(crate) fn take_resource_cleanup(&self) -> Option<Cleanup> {
        self.resource
            .and_then(|resource| (resource.take_cleanup)(self))
    }

    pub(crate) fn has_resource(&self) -> bool {
        self.resource.is_some()
    }

    pub(crate) fn accept_resource(&self, revision: u64, result: Box<dyn Any + Send>) -> bool {
        self.resource
            .is_some_and(|resource| (resource.accept)(self, revision, result))
    }

    pub(crate) fn accept_async_state(&self, result: Box<dyn Any + Send>) -> bool {
        self.async_state
            .is_some_and(|async_state| (async_state.accept)(self, result))
    }

    pub(crate) fn async_liveness<T: 'static>(&self) -> SyncWeak<()> {
        let value = self.value.borrow();
        Arc::downgrade(&value.downcast_ref::<AsyncStateSlot<T>>().unwrap().liveness)
    }
}

pub(crate) struct StateSlot<T>(pub T);

pub(crate) struct AsyncStateSlot<T> {
    pub value: T,
    liveness: Arc<()>,
}

pub(crate) struct RefSlot<T>(pub T);

pub(crate) struct MemoSlot<D, T> {
    pub deps: D,
    pub value: T,
}

pub(crate) struct EffectSlot<D> {
    pub deps: D,
    pub pending: Option<Effect>,
    pub cleanup: Option<Cleanup>,
}

pub(crate) struct ResourceSlot<D, T> {
    pub deps: D,
    pub revision: u64,
    pub value: Resource<T>,
    pub pending: Option<(CancellationToken, Box<dyn FnOnce()>)>,
    pub cancellation: Option<CancellationToken>,
}

fn commit_effect<D: 'static>(cell: &HookCell) {
    cell.with_mut::<EffectSlot<D>, _>(|slot| {
        let effect = slot.pending.take().unwrap();
        if let Some(cleanup) = slot.cleanup.take() {
            cleanup();
        }
        slot.cleanup = effect();
    });
}

fn take_effect_cleanup<D: 'static>(cell: &HookCell) -> Option<Cleanup> {
    cell.with_mut::<EffectSlot<D>, _>(|slot| {
        slot.pending = None;
        slot.cleanup.take()
    })
}

fn commit_resource<D: 'static, T: Send + 'static>(cell: &HookCell) {
    let launch = cell.with_mut::<ResourceSlot<D, T>, _>(|slot| {
        if let Some(cancellation) = slot.cancellation.take() {
            cancellation.cancel();
        }
        let (cancellation, launch) = slot.pending.take().unwrap();
        slot.cancellation = Some(cancellation);
        launch
    });
    launch();
}

fn take_resource_cleanup<D: 'static, T: Send + 'static>(cell: &HookCell) -> Option<Cleanup> {
    cell.with_mut::<ResourceSlot<D, T>, _>(|slot| {
        let pending = slot.pending.take().map(|(cancellation, _)| cancellation);
        let active = slot.cancellation.take();
        if pending.is_none() && active.is_none() {
            return None;
        }
        Some(Box::new(move || {
            if let Some(cancellation) = pending {
                cancellation.cancel();
            }
            if let Some(cancellation) = active {
                cancellation.cancel();
            }
        }) as Cleanup)
    })
}

fn accept_resource<D: 'static, T: Send + 'static>(
    cell: &HookCell,
    revision: u64,
    result: Box<dyn Any + Send>,
) -> bool {
    cell.with_mut::<ResourceSlot<D, T>, _>(|slot| {
        if slot.revision != revision {
            return false;
        }
        let result = *result.downcast::<windows_core::Result<T>>().unwrap();
        slot.value = match result {
            Ok(value) => Resource::Ready(Rc::new(value)),
            Err(error) => Resource::Failed(error),
        };
        true
    })
}

fn accept_async_state<T: Send + 'static>(cell: &HookCell, result: Box<dyn Any + Send>) -> bool {
    cell.with_mut::<AsyncStateSlot<T>, _>(|slot| {
        slot.value = *result.downcast::<T>().unwrap();
    });
    true
}

pub struct RenderCx<'a> {
    pub(crate) node: NodeId,
    pub(crate) hooks: &'a mut Vec<Rc<HookCell>>,
    pub(crate) cursor: usize,
    pub(crate) mounting: bool,
    pub(crate) scheduler: SchedulerRef,
    pub(crate) contexts: &'a [ContextEntry],
    pub(crate) context_defaults: &'a ContextDefaults,
}

impl RenderCx<'_> {
    pub fn use_state<T>(&mut self, initial: impl FnOnce() -> T) -> State<T>
    where
        T: Clone + 'static,
    {
        let index = self.cursor;
        self.cursor += 1;
        if index == self.hooks.len() {
            assert!(
                self.mounting,
                "component added a hook at slot {index} after its initial render"
            );
            self.hooks
                .push(Rc::new(HookCell::value(StateSlot(initial()))));
        }
        let cell = Rc::downgrade(&self.hooks[index]);
        assert!(
            self.hooks[index].value.borrow().is::<StateSlot<T>>(),
            "component changed the hook kind or value type at slot {index}"
        );
        State::new(self.node, cell, Rc::clone(&self.scheduler))
    }

    pub fn use_async_state<T>(&mut self, initial: T) -> (T, AsyncSetState<T>)
    where
        T: Clone + Send + 'static,
    {
        let index = self.cursor;
        self.cursor += 1;
        if index == self.hooks.len() {
            assert!(
                self.mounting,
                "component added a hook at slot {index} after its initial render"
            );
            self.hooks
                .push(Rc::new(HookCell::async_state(AsyncStateSlot {
                    value: initial,
                    liveness: Arc::new(()),
                })));
        }
        let value = self.hooks[index]
            .value
            .borrow()
            .downcast_ref::<AsyncStateSlot<T>>()
            .unwrap_or_else(|| {
                panic!("component changed the hook kind or value type at slot {index}")
            })
            .value
            .clone();
        let slot = u32::try_from(index).unwrap();
        let setter = AsyncSetState {
            set: self.scheduler.async_setter(
                self.node,
                slot,
                self.hooks[index].async_liveness::<T>(),
            ),
            marker: PhantomData,
        };
        (value, setter)
    }

    pub fn use_mutation<T>(&mut self) -> (MutationState<T>, MutationTrigger<T>)
    where
        T: Clone + Send + 'static,
    {
        let (state, set_state) = self.use_async_state(MutationState::Idle);
        (state, MutationTrigger { set_state })
    }

    pub fn use_ref<T>(&mut self, initial: impl FnOnce() -> T) -> HookRef<T>
    where
        T: 'static,
    {
        let index = self.cursor;
        self.cursor += 1;
        if index == self.hooks.len() {
            assert!(
                self.mounting,
                "component added a hook at slot {index} after its initial render"
            );
            self.hooks
                .push(Rc::new(HookCell::value(RefSlot(initial()))));
        }
        assert!(
            self.hooks[index].value.borrow().is::<RefSlot<T>>(),
            "component changed the hook kind or value type at slot {index}"
        );
        HookRef::new(Rc::downgrade(&self.hooks[index]))
    }

    pub fn use_memo<D, T>(&mut self, deps: D, factory: impl FnOnce() -> T) -> T
    where
        D: PartialEq + 'static,
        T: Clone + 'static,
    {
        let index = self.cursor;
        self.cursor += 1;
        if index == self.hooks.len() {
            assert!(
                self.mounting,
                "component added a hook at slot {index} after its initial render"
            );
            let value = factory();
            self.hooks.push(Rc::new(HookCell::value(MemoSlot {
                deps,
                value: value.clone(),
            })));
            return value;
        }

        let mut slot = self.hooks[index].value.borrow_mut();
        let slot = slot.downcast_mut::<MemoSlot<D, T>>().unwrap_or_else(|| {
            panic!("component changed the hook kind or value type at slot {index}")
        });
        if slot.deps != deps {
            slot.deps = deps;
            slot.value = factory();
        }
        slot.value.clone()
    }

    pub fn use_callback<D, T>(&mut self, deps: D, callback: impl Fn(T) + 'static) -> Callback<T>
    where
        D: PartialEq + 'static,
        T: 'static,
    {
        self.use_memo(deps, || Callback::new(callback))
    }

    pub fn use_reducer<S, A>(
        &mut self,
        initial: impl FnOnce() -> S,
        reducer: fn(S, A) -> S,
    ) -> (S, Callback<A>)
    where
        S: Clone + 'static,
        A: 'static,
    {
        let state = self.use_state(initial);
        let value = state.get().unwrap();
        let dispatch = self.use_callback((), move |action| {
            if let Some(value) = state.get() {
                state.set(reducer(value, action));
            }
        });
        (value, dispatch)
    }

    pub fn use_context<T>(&self, context: &Context<T>) -> T
    where
        T: Clone + 'static,
    {
        self.contexts
            .iter()
            .rev()
            .find(|entry| entry.id == context.id)
            .map_or_else(
                || (*context.default).clone(),
                |entry| entry.value.downcast_ref::<T>().unwrap().clone(),
            )
    }

    pub fn use_context_key<T>(&self, context: &'static ContextKey<T>) -> T
    where
        T: Clone + 'static,
    {
        self.contexts
            .iter()
            .rev()
            .find(|entry| entry.id == context.id())
            .map_or_else(
                || self.context_defaults.get(context),
                |entry| entry.value.downcast_ref::<T>().unwrap().clone(),
            )
    }

    pub fn use_element_ref<T>(&mut self) -> ElementRef<T>
    where
        T: 'static,
    {
        let reference = self.use_memo((), ElementRef::new);
        reference.clear_lifecycle();
        reference
    }

    pub fn use_element_ref_with_lifecycle<T>(
        &mut self,
        mounted: impl Fn() + 'static,
        unmounted: impl Fn() + 'static,
    ) -> ElementRef<T>
    where
        T: 'static,
    {
        let reference = self.use_memo((), ElementRef::new);
        reference.set_lifecycle(Some(Rc::new(mounted)), Some(Rc::new(unmounted)));
        reference
    }
}

impl RenderCx<'_> {
    pub fn use_effect<D>(&mut self, deps: D, effect: impl FnOnce() + 'static)
    where
        D: PartialEq + 'static,
    {
        self.use_effect_inner(deps, move || {
            effect();
            None
        });
    }

    pub fn use_effect_with_cleanup<D, C>(&mut self, deps: D, effect: impl FnOnce() -> C + 'static)
    where
        D: PartialEq + 'static,
        C: FnOnce() + 'static,
    {
        self.use_effect_inner(deps, move || Some(Box::new(effect()) as Cleanup));
    }

    fn use_effect_inner<D, F>(&mut self, deps: D, effect: F)
    where
        D: PartialEq + 'static,
        F: FnOnce() -> Option<Cleanup> + 'static,
    {
        let index = self.cursor;
        self.cursor += 1;
        if index == self.hooks.len() {
            assert!(
                self.mounting,
                "component added a hook at slot {index} after its initial render"
            );
            let cell = Rc::new(HookCell::effect(EffectSlot {
                deps,
                pending: Some(Box::new(effect)),
                cleanup: None,
            }));
            self.scheduler.queue_effect(Rc::downgrade(&cell));
            self.hooks.push(cell);
            return;
        }

        let cell = &self.hooks[index];
        let mut value = cell.value.borrow_mut();
        let slot = value.downcast_mut::<EffectSlot<D>>().unwrap_or_else(|| {
            panic!("component changed the hook kind or value type at slot {index}")
        });
        if slot.deps != deps {
            slot.deps = deps;
            let queue = slot.pending.is_none();
            slot.pending = Some(Box::new(effect));
            drop(value);
            if queue {
                self.scheduler.queue_effect(Rc::downgrade(cell));
            }
        }
    }

    pub fn use_timeout<D>(&mut self, deps: D, delay: Duration, callback: impl FnOnce() + 'static)
    where
        D: PartialEq + 'static,
    {
        self.use_timer(
            TimerKind::Timeout,
            deps,
            delay,
            TimerCallback::Once(Some(Box::new(callback))),
        );
    }

    pub fn use_interval<D>(&mut self, deps: D, interval: Duration, callback: impl Fn() + 'static)
    where
        D: PartialEq + 'static,
    {
        self.use_timer(
            TimerKind::Interval,
            deps,
            interval,
            TimerCallback::Repeating(Rc::new(callback)),
        );
    }

    fn use_timer<D>(
        &mut self,
        kind: TimerKind,
        deps: D,
        interval: Duration,
        callback: TimerCallback,
    ) where
        D: PartialEq + 'static,
    {
        let index = self.cursor;
        self.cursor += 1;
        let revision = if index == self.hooks.len() {
            assert!(
                self.mounting,
                "component added a hook at slot {index} after its initial render"
            );
            let revision = Rc::new(Cell::new(0));
            self.hooks.push(Rc::new(HookCell::value(TimerSlot {
                kind,
                revision: Rc::clone(&revision),
                callback,
            })));
            revision
        } else {
            let mut value = self.hooks[index].value.borrow_mut();
            let slot = value.downcast_mut::<TimerSlot>().unwrap_or_else(|| {
                panic!("component changed the hook kind or value type at slot {index}")
            });
            assert!(
                slot.kind == kind,
                "component changed the hook kind or value type at slot {index}"
            );
            slot.callback = callback;
            Rc::clone(&slot.revision)
        };
        let owner = self.node;
        let slot = u32::try_from(index).unwrap();
        let scheduler = Rc::clone(&self.scheduler);
        self.use_effect_inner((deps, interval), move || {
            let next = revision
                .get()
                .checked_add(1)
                .unwrap_or_else(|| panic!("timer revision space exhausted"));
            revision.set(next);
            scheduler.start_timer(owner, slot, next, interval, kind == TimerKind::Interval);
            Some(Box::new(move || scheduler.stop_timer(owner, slot, next)) as Cleanup)
        });
    }
}

impl RenderCx<'_> {
    pub fn use_resource<D, T, F>(&mut self, deps: D, loader: F) -> Resource<T>
    where
        D: Clone + PartialEq + Send + 'static,
        T: Send + 'static,
        F: FnOnce(CancellationToken, D) -> windows_core::Result<T> + Send + 'static,
    {
        let index = self.cursor;
        self.cursor += 1;
        let owner = self.node;
        let slot = u32::try_from(index).unwrap();
        let scheduler = Rc::clone(&self.scheduler);
        let make_pending = |revision: u64, deps: D, loader: F| {
            let cancellation = CancellationToken::new();
            let worker_cancellation = cancellation.clone();
            let launch_scheduler = Rc::clone(&scheduler);
            let launch = Box::new(move || {
                launch_scheduler.launch_resource(ResourceTask {
                    owner,
                    slot,
                    revision,
                    work: Box::new(move || {
                        Box::new(loader(worker_cancellation, deps)) as Box<dyn Any + Send>
                    }),
                });
            }) as Box<dyn FnOnce()>;
            (cancellation, launch)
        };
        if index == self.hooks.len() {
            assert!(
                self.mounting,
                "component added a hook at slot {index} after its initial render"
            );
            let cell = Rc::new(HookCell::resource(ResourceSlot {
                deps: deps.clone(),
                revision: 1,
                value: Resource::<T>::Loading,
                pending: Some(make_pending(1, deps, loader)),
                cancellation: None,
            }));
            self.scheduler.queue_resource(Rc::downgrade(&cell));
            self.hooks.push(cell);
            return Resource::<T>::Loading;
        }

        let cell = &self.hooks[index];
        let mut value = cell.value.borrow_mut();
        let resource = value
            .downcast_mut::<ResourceSlot<D, T>>()
            .unwrap_or_else(|| {
                panic!("component changed the hook kind or value type at slot {index}")
            });
        if resource.deps != deps {
            resource.deps = deps.clone();
            resource.revision = resource
                .revision
                .checked_add(1)
                .unwrap_or_else(|| panic!("resource revision space exhausted"));
            resource.value = Resource::Loading;
            let queue = resource.pending.is_none();
            resource.pending = Some(make_pending(resource.revision, deps, loader));
            let result = resource.value.clone();
            drop(value);
            if queue {
                self.scheduler.queue_resource(Rc::downgrade(cell));
            }
            return result;
        }
        resource.value.clone()
    }
}

pub struct AsyncSetState<T> {
    set: AsyncSetFn,
    marker: PhantomData<fn() -> T>,
}

impl<T> AsyncSetState<T>
where
    T: Send + 'static,
{
    /// Queues a state replacement if the generation-bound owner is still live.
    ///
    /// Stale setters are an expected no-op after teardown or replacement. A queued replacement is
    /// checked again on the UI thread, so teardown may reject it before the next render.
    pub fn set(&self, value: T) {
        let _ = self.try_set(value);
    }

    /// Queues a state replacement, returning whether it was accepted for delivery.
    ///
    /// A queued replacement is checked again against the component generation and hook slot on
    /// the UI thread. `true` means queued, not guaranteed eventual application, because teardown
    /// may still reject it before the next render.
    #[must_use = "the async state update may be rejected after component teardown"]
    pub(crate) fn try_set(&self, value: T) -> bool {
        (self.set)(Box::new(value))
    }
}

impl<T> Clone for AsyncSetState<T> {
    fn clone(&self) -> Self {
        Self {
            set: Arc::clone(&self.set),
            marker: PhantomData,
        }
    }
}

impl<T> std::fmt::Debug for AsyncSetState<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncSetState").finish_non_exhaustive()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MutationState<T> {
    Idle,
    Loading,
    Success(T),
    Error(String),
}

impl<T> MutationState<T> {
    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }
}

pub struct MutationTrigger<T> {
    set_state: AsyncSetState<MutationState<T>>,
}

impl<T> MutationTrigger<T>
where
    T: Clone + Send + 'static,
{
    pub fn fire(&self, operation: impl FnOnce() -> Result<T, String> + Send + 'static) {
        self.set_state.set(MutationState::Loading);
        let set_state = self.set_state.clone();
        windows_threading::submit(move || {
            set_state.set(match operation() {
                Ok(value) => MutationState::Success(value),
                Err(error) => MutationState::Error(error),
            });
        });
    }

    pub fn reset(&self) {
        self.set_state.set(MutationState::Idle);
    }
}

impl<T> Clone for MutationTrigger<T> {
    fn clone(&self) -> Self {
        Self {
            set_state: self.set_state.clone(),
        }
    }
}

impl<T> std::fmt::Debug for MutationTrigger<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MutationTrigger").finish_non_exhaustive()
    }
}

pub struct State<T> {
    pub(crate) node: NodeId,
    cell: Weak<HookCell>,
    scheduler: SchedulerRef,
    marker: PhantomData<fn() -> T>,
}

impl<T> State<T> {
    pub(crate) fn new(node: NodeId, cell: Weak<HookCell>, scheduler: SchedulerRef) -> Self {
        Self {
            node,
            cell,
            scheduler,
            marker: PhantomData,
        }
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            node: self.node,
            cell: self.cell.clone(),
            scheduler: Rc::clone(&self.scheduler),
            marker: PhantomData,
        }
    }
}

impl<T> State<T>
where
    T: Clone + 'static,
{
    pub fn value(&self) -> T {
        self.try_value()
            .unwrap_or_else(|| panic!("state handle is stale"))
    }

    pub fn try_value(&self) -> Option<T> {
        let cell = self.cell.upgrade()?;
        Some(
            cell.value
                .borrow()
                .downcast_ref::<StateSlot<T>>()
                .unwrap()
                .0
                .clone(),
        )
    }

    pub(crate) fn get(&self) -> Option<T> {
        self.try_value()
    }

    /// Replaces the value and schedules a render if the generation-bound owner is still live.
    ///
    /// A stale handle after teardown or replacement is an expected no-op.
    pub fn set(&self, value: T) {
        let _ = self.try_set(value);
    }

    /// Replaces the value and schedules a render, returning `false` if this handle is stale.
    #[must_use = "the state update may be rejected after component teardown"]
    pub(crate) fn try_set(&self, value: T) -> bool {
        let Some(cell) = self.cell.upgrade() else {
            return false;
        };
        cell.value
            .borrow_mut()
            .downcast_mut::<StateSlot<T>>()
            .unwrap()
            .0 = value;
        self.scheduler.invalidate(self.node);
        true
    }

    /// Updates the value and schedules a render if the generation-bound owner is still live.
    ///
    /// A stale handle after teardown or replacement is an expected no-op, and the closure is not
    /// called.
    pub fn update(&self, update: impl FnOnce(&mut T)) {
        let _ = self.try_update(update);
    }

    /// Updates the value and schedules a render, returning `false` if this handle is stale.
    ///
    /// The update closure is not called when the handle is stale.
    #[must_use = "the state update may be rejected after component teardown"]
    pub(crate) fn try_update(&self, update: impl FnOnce(&mut T)) -> bool {
        let Some(cell) = self.cell.upgrade() else {
            return false;
        };
        update(
            &mut cell
                .value
                .borrow_mut()
                .downcast_mut::<StateSlot<T>>()
                .unwrap()
                .0,
        );
        self.scheduler.invalidate(self.node);
        true
    }
}

pub struct HookRef<T> {
    cell: Weak<HookCell>,
    marker: PhantomData<fn() -> T>,
}

impl<T> HookRef<T> {
    pub(crate) fn new(cell: Weak<HookCell>) -> Self {
        Self {
            cell,
            marker: PhantomData,
        }
    }
}

impl<T> Clone for HookRef<T> {
    fn clone(&self) -> Self {
        Self {
            cell: self.cell.clone(),
            marker: PhantomData,
        }
    }
}

impl<T: 'static> HookRef<T> {
    pub fn with<U>(&self, f: impl FnOnce(&T) -> U) -> Option<U> {
        let cell = self.cell.upgrade()?;
        let value = cell.value.borrow();
        Some(f(&value.downcast_ref::<RefSlot<T>>().unwrap().0))
    }

    pub fn with_mut<U>(&self, f: impl FnOnce(&mut T) -> U) -> Option<U> {
        let cell = self.cell.upgrade()?;
        let mut value = cell.value.borrow_mut();
        Some(f(&mut value.downcast_mut::<RefSlot<T>>().unwrap().0))
    }

    /// Replaces the value if the generation-bound hook slot is still live.
    ///
    /// A stale reference after teardown or replacement is an expected no-op.
    pub fn set(&self, value: T) {
        let _ = self.try_set(value);
    }

    /// Replaces the referenced value, returning `false` if this handle is stale.
    #[must_use = "the hook reference update may be rejected after component teardown"]
    pub(crate) fn try_set(&self, value: T) -> bool {
        let Some(cell) = self.cell.upgrade() else {
            return false;
        };
        cell.value
            .borrow_mut()
            .downcast_mut::<RefSlot<T>>()
            .unwrap()
            .0 = value;
        true
    }
}

impl<T> HookRef<T>
where
    T: Clone + 'static,
{
    pub fn get(&self) -> Option<T> {
        self.with(Clone::clone)
    }
}
