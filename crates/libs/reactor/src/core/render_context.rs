use std::any::Any;
use std::cell::{Cell, Ref, RefCell};
use std::fmt;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread::{self, ThreadId};

use rustc_hash::FxHashSet;

use super::*;

/// Type-erased dependency tuple used to gate `use_memo` / `use_effect`
/// re-runs; auto-implemented for any `T: PartialEq + Clone + 'static`.
pub trait Deps: 'static {
    fn as_any(&self) -> &dyn Any;

    fn deps_eq(&self, other: &dyn Deps) -> bool;

    fn clone_boxed(&self) -> Box<dyn Deps>;
}

impl<T: PartialEq + Clone + 'static> Deps for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn deps_eq(&self, other: &dyn Deps) -> bool {
        match other.as_any().downcast_ref::<T>() {
            Some(o) => self == o,
            None => false,
        }
    }

    fn clone_boxed(&self) -> Box<dyn Deps> {
        Box::new(self.clone())
    }
}

type Cleanup = Box<dyn FnOnce()>;

type PendingEffect = Box<dyn FnOnce() -> Option<Cleanup>>;

pub(crate) enum HookSlot {
    State {
        cell: Rc<RefCell<Box<dyn Any>>>,
        type_name: &'static str,
    },
    Memo {
        value: Rc<RefCell<Box<dyn Any>>>,
        deps: Option<Box<dyn Deps>>,
    },
    Effect {
        deps: Option<Box<dyn Deps>>,
        pending: Option<PendingEffect>,
        cleanup: Option<Cleanup>,
        pending_cleanup: Option<Cleanup>,
    },
    /// State slot whose cell is `Arc<Mutex<…>>` so writes can come from
    /// any thread. Backs [`RenderCx::use_async_state`].
    AsyncState {
        cell: Arc<Mutex<Box<dyn Any + Send>>>,
        type_name: &'static str,
    },
}

impl fmt::Debug for HookSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HookSlot::State { type_name, .. } => write!(f, "State({type_name})"),
            HookSlot::Memo { .. } => f.write_str("Memo { .. }"),
            HookSlot::Effect {
                pending, cleanup, ..
            } => f
                .debug_struct("Effect")
                .field("pending", &pending.is_some())
                .field("has_cleanup", &cleanup.is_some())
                .finish(),
            HookSlot::AsyncState { type_name, .. } => write!(f, "AsyncState({type_name})"),
        }
    }
}

crate::impl_rc_fn_wrapper! {
    /// Setter returned by `use_state`; replaces the slot value and
    /// requests a rerender.
    pub struct SetState<T>(dyn Fn(T));
}

impl<T> SetState<T> {
    pub fn call(&self, value: T) {
        (self.inner)(value);
    }
}

impl<T: 'static> From<SetState<T>> for Callback<T> {
    fn from(s: SetState<T>) -> Self {
        Self::from_rc(s.inner)
    }
}

type ReducerClosure<T> = Box<dyn FnOnce(T) -> T>;

crate::impl_rc_fn_wrapper! {
    /// Functional updater returned by `use_state`; mutates the slot value
    /// via `f(prev) -> next` and requests a rerender.
    pub struct Updater<T>(dyn Fn(ReducerClosure<T>));
}

impl<T: 'static> Updater<T> {
    pub fn call<F>(&self, reducer: F)
    where
        F: FnOnce(T) -> T + 'static,
    {
        (self.inner)(Box::new(reducer));
    }
}

crate::impl_rc_fn_wrapper! {
    /// Action dispatcher returned by `use_reducer`.
    pub struct Dispatch<A>(dyn Fn(A));
}

impl<A> Dispatch<A> {
    pub fn call(&self, action: A) {
        (self.inner)(action);
    }
}

impl<A: 'static> From<Dispatch<A>> for Callback<A> {
    fn from(d: Dispatch<A>) -> Self {
        Self::from_rc(d.inner)
    }
}

/// Thread-safe setter returned by [`RenderCx::use_async_state`]. Calling
/// `.call(value)` from any thread auto-marshals the state write + rerender
/// request back onto the UI thread via the host's [`UiMarshaller`].
pub struct AsyncSetState<T: Send + 'static> {
    cell: Arc<Mutex<Box<dyn Any + Send>>>,
    marshaller: UiMarshaller,
    /// Owning component's `state_dirty` flag (shared with `RenderCx`), set on
    /// a real change so the reconciler re-renders the component even when
    /// nested under a parent whose element tree is unchanged.
    dirty: Arc<AtomicBool>,
    type_name: &'static str,
    _marker: std::marker::PhantomData<fn(T)>,
}

impl<T: Send + 'static> Clone for AsyncSetState<T> {
    fn clone(&self) -> Self {
        Self {
            cell: Arc::clone(&self.cell),
            marshaller: self.marshaller.clone(),
            dirty: Arc::clone(&self.dirty),
            type_name: self.type_name,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T: Send + 'static> fmt::Debug for AsyncSetState<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AsyncSetState")
            .field("type", &self.type_name)
            .finish_non_exhaustive()
    }
}

impl<T: Send + Clone + PartialEq + 'static> AsyncSetState<T> {
    /// Apply `value` to the state slot from any thread. The write is
    /// marshalled to the UI thread; no-op if value is unchanged.
    pub fn call(&self, value: T) {
        let cell = Arc::clone(&self.cell);
        let dirty = Arc::clone(&self.dirty);
        let type_name = self.type_name;
        self.marshaller.dispatch(move || {
            let mut slot = cell.lock().unwrap();
            let prev = slot.downcast_ref::<T>().unwrap_or_else(|| {
                panic!(
                    "AsyncSetState slot type mismatch (expected `{type_name}`): \
                     hook order invariant violated"
                )
            });
            if *prev == value {
                return;
            }
            *slot = Box::new(value);
            drop(slot);
            // Mark the owning component dirty so the reconciler does not skip
            // it; a bare rerender request only re-renders the root.
            dirty.store(true, Ordering::Relaxed);
            request_ui_rerender_on_ui_thread();
        });
    }
}

/// `Rc<RefCell<T>>` wrapper returned by `use_ref`; identity-stable across
/// renders so callers can pin mutable state outside the hooks vector.
pub struct HookRef<T> {
    inner: Rc<RefCell<T>>,
}

impl<T> HookRef<T> {
    pub fn borrow(&self) -> Ref<'_, T> {
        self.inner.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<'_, T> {
        self.inner.borrow_mut()
    }

    pub fn replace(&self, v: T) -> T {
        self.inner.replace(v)
    }

    pub fn set(&self, v: T) {
        *self.inner.borrow_mut() = v;
    }
}

impl<T: Clone> HookRef<T> {
    pub fn get_cloned(&self) -> T {
        self.inner.borrow().clone()
    }
}

impl<T> Clone for HookRef<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for HookRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner.try_borrow() {
            Ok(b) => f.debug_tuple("HookRef").field(&*b).finish(),
            Err(_) => f.debug_tuple("HookRef").field(&"<borrowed>").finish(),
        }
    }
}

/// Stable `ContextId` used to track which components subscribed to
/// [`RenderCx::use_inner_size`].
pub(crate) fn inner_size_context_id() -> ContextId {
    static ID: OnceLock<ContextId> = OnceLock::new();
    *ID.get_or_init(ContextId::new)
}

/// Stable `ContextId` used to track which components subscribed to
/// [`RenderCx::use_dpi`].
pub(crate) fn dpi_context_id() -> ContextId {
    static ID: OnceLock<ContextId> = OnceLock::new();
    *ID.get_or_init(ContextId::new)
}

/// Per-component render context: hooks vector cursor, the rerender
/// request closure, and ambient context-provider stack. Passed to every
/// [`Component::render`] call.
pub struct RenderCx {
    hooks: Rc<RefCell<Vec<HookSlot>>>,
    cursor: usize,
    request_rerender: Rc<dyn Fn()>,
    /// Shared flag set whenever a hook value changes, by both the synchronous
    /// setters (`SetState`/`Updater`/`Dispatch`) and the off-thread
    /// [`AsyncSetState`]. The reconciler checks this to force a re-render of
    /// the owning component even when the parent's element tree is
    /// structurally identical, so nested components are not skipped after a
    /// state update. It is an `Arc<AtomicBool>` rather than a `Cell` because
    /// [`AsyncSetState`] captures it into a `Send` closure that is marshalled
    /// onto the UI thread; the synchronous setters only ever touch it from the
    /// UI thread.
    state_dirty: Arc<AtomicBool>,
    ui_thread: Option<ThreadId>,
    context_stack: Option<Rc<ContextStack>>,
    read_contexts: RefCell<FxHashSet<ContextId>>,
    /// `Send + Sync` handle to the UI thread's render-aware dispatcher,
    /// populated by the host for [`Self::use_ui_marshaller`] /
    /// [`Self::use_async_state`].
    marshaller: Option<UiMarshaller>,
    inner_size: Rc<Cell<WindowSize>>,
    /// Per-monitor DPI for the host window. Shared with the
    /// reconciler via [`Self::set_dpi_cell`]; updated by the host when the
    /// window moves across monitors.
    dpi: Rc<Cell<u32>>,
}

impl fmt::Debug for RenderCx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RenderCx")
            .field("cursor", &self.cursor)
            .field("hook_count", &self.hooks.borrow().len())
            .field("ui_thread", &self.ui_thread)
            .field("has_marshaller", &self.marshaller.is_some())
            .finish()
    }
}

impl RenderCx {
    pub fn new(request_rerender: Rc<dyn Fn()>) -> Self {
        Self {
            hooks: Rc::new(RefCell::new(Vec::new())),
            cursor: 0,
            request_rerender,
            state_dirty: Arc::new(AtomicBool::new(false)),
            ui_thread: None,
            context_stack: None,
            read_contexts: RefCell::new(FxHashSet::default()),
            marshaller: None,
            inner_size: Rc::new(Cell::new(WindowSize::default())),
            dpi: Rc::new(Cell::new(96)),
        }
    }

    pub fn take_read_contexts(&mut self) -> FxHashSet<ContextId> {
        std::mem::take(&mut *self.read_contexts.borrow_mut())
    }

    pub fn set_context_stack(&mut self, stack: Rc<ContextStack>) {
        self.context_stack = Some(stack);
    }

    pub fn use_context<T>(&self, context: &Context<T>) -> T
    where
        T: 'static + Clone,
    {
        self.read_contexts.borrow_mut().insert(context.id);

        if let Some(stack) = &self.context_stack
            && let Some(v) = stack.get::<T>(context.id) {
                return v;
            }
        context.default.clone()
    }

    pub fn for_test() -> Self {
        Self::new(Rc::new(|| {}))
    }

    pub fn begin_render(&mut self) {
        self.cursor = 0;
        self.ui_thread = Some(thread::current().id());
        self.read_contexts.borrow_mut().clear();
        self.state_dirty.store(false, Ordering::Relaxed);
    }

    pub fn hook_count(&self) -> usize {
        self.hooks.borrow().len()
    }

    pub fn set_request_rerender(&mut self, request_rerender: Rc<dyn Fn()>) {
        self.request_rerender = request_rerender;
    }

    /// Returns `true` if any state write (synchronous or off-thread async)
    /// has changed a hook value since the last render, and clears the flag.
    pub fn take_state_dirty(&self) -> bool {
        self.state_dirty.swap(false, Ordering::Relaxed)
    }

    /// Returns the current dirty state without clearing it.
    pub fn peek_state_dirty(&self) -> bool {
        self.state_dirty.load(Ordering::Relaxed)
    }

    /// Install (or replace) the [`UiMarshaller`] used by
    /// [`Self::use_ui_marshaller`] / [`Self::use_async_state`].
    pub fn set_marshaller(&mut self, marshaller: Option<UiMarshaller>) {
        self.marshaller = marshaller;
    }

    /// Returns the UI-thread marshaller installed by the host.
    pub fn use_ui_marshaller(&mut self) -> UiMarshaller {
        self.marshaller.clone().expect(
            "use_ui_marshaller requires a WinUI host",
        )
    }

    /// Like [`Self::use_state`], but returns an [`AsyncSetState<T>`]
    /// whose `.call(value)` is safe to invoke from any thread; writes
    /// auto-marshal onto the UI thread via the host's [`UiMarshaller`].
    pub fn use_async_state<T>(&mut self, initial: T) -> (T, AsyncSetState<T>)
    where
        T: 'static + Send + Clone + PartialEq,
    {
        let slot_index = self.cursor;
        self.cursor += 1;

        self.debug_assert_on_ui_thread(slot_index);

        let (cell, type_name) = {
            let mut hooks = self.hooks.borrow_mut();
            if slot_index == hooks.len() {
                let boxed: Box<dyn Any + Send> = Box::new(initial);
                let cell = Arc::new(Mutex::new(boxed));
                let type_name = std::any::type_name::<T>();
                hooks.push(HookSlot::AsyncState {
                    cell: Arc::clone(&cell),
                    type_name,
                });
                (cell, type_name)
            } else {
                match &hooks[slot_index] {
                    HookSlot::AsyncState { cell, type_name } => (Arc::clone(cell), *type_name),
                    _ => panic!(
                        "hook called in different order: slot {slot_index} held a \
                         non-AsyncState slot, now called as `use_async_state`"
                    ),
                }
            }
        };

        let current = {
            let slot = cell.lock().unwrap();
            match slot.downcast_ref::<T>() {
                Some(v) => v.clone(),
                None => panic!(
                    "hook called in different order: slot {slot_index} was \
                     `{type_name}`, now called as `use_async_state::<{requested}>`",
                    requested = std::any::type_name::<T>()
                ),
            }
        };

        let marshaller = self.marshaller.clone().expect(
            "use_async_state requires a WinUI host",
        );

        let setter = AsyncSetState::<T> {
            cell,
            marshaller,
            dirty: Arc::clone(&self.state_dirty),
            type_name,
            _marker: std::marker::PhantomData,
        };

        (current, setter)
    }

    pub fn use_state<T>(&mut self, initial: T) -> (T, SetState<T>)
    where
        T: 'static + Clone + PartialEq,
    {
        let (current, cell, slot_index) = self.state_slot(initial);
        let setter = self.make_state_setter::<T>(cell, slot_index);
        (current, setter)
    }

    pub fn use_reducer<T>(&mut self, initial: T) -> (T, Updater<T>)
    where
        T: 'static + Clone + PartialEq,
    {
        let (current, cell, slot_index) = self.state_slot(initial);
        let updater = self.make_updater::<T>(cell, slot_index);
        (current, updater)
    }

    pub fn use_reducer_fn<S, A, R>(&mut self, reducer: R, initial: S) -> (S, Dispatch<A>)
    where
        S: 'static + Clone + PartialEq,
        A: 'static,
        R: Fn(S, A) -> S + 'static,
    {
        let (current, cell, slot_index) = self.state_slot(initial);
        let dispatch = self.make_dispatch::<S, A, R>(cell, slot_index, reducer);
        (current, dispatch)
    }

    pub fn use_ref<T: 'static>(&mut self, initial: T) -> HookRef<T> {
        let slot_index = self.cursor;
        self.cursor += 1;

        self.debug_assert_on_ui_thread(slot_index);

        let mut hooks = self.hooks.borrow_mut();
        if slot_index == hooks.len() {
            let h = HookRef {
                inner: Rc::new(RefCell::new(initial)),
            };
            let boxed: Box<dyn Any> = Box::new(h.clone());
            hooks.push(HookSlot::State {
                cell: Rc::new(RefCell::new(boxed)),
                type_name: std::any::type_name::<HookRef<T>>(),
            });
            return h;
        }

        match &hooks[slot_index] {
            HookSlot::State { cell, type_name } => {
                let slot = cell.borrow();
                let h = slot.downcast_ref::<HookRef<T>>().unwrap_or_else(|| {
                    panic!(
                        "hook called in different order: slot {slot_index} was \
                         `{type_name}`, now called as `use_ref::<{requested}>`",
                        requested = std::any::type_name::<T>()
                    )
                });
                h.clone()
            }
            _ => panic!(
                "hook called in different order: slot {slot_index} held a \
                 non-State slot, now called as `use_ref`"
            ),
        }
    }

    /// Read the host's effective [`ColorScheme`] (Light / Dark) for the
    /// current render. Re-rendering on theme switches is driven by the
    /// host via `FrameworkElement::ActualThemeChanged`.
    pub fn use_color_scheme(&self) -> ColorScheme {
        current_color_scheme()
    }

    pub fn use_memo<T, D, F>(&mut self, deps: D, factory: F) -> T
    where
        T: 'static + Clone,
        D: Deps,
        F: FnOnce() -> T,
    {
        let slot_index = self.cursor;
        self.cursor += 1;

        self.debug_assert_on_ui_thread(slot_index);

        let mut hooks = self.hooks.borrow_mut();
        if slot_index == hooks.len() {
            let value = factory();
            let value_cell: Rc<RefCell<Box<dyn Any>>> =
                Rc::new(RefCell::new(Box::new(value.clone())));
            hooks.push(HookSlot::Memo {
                value: value_cell,
                deps: Some(deps.clone_boxed()),
            });
            return value;
        }

        match &mut hooks[slot_index] {
            HookSlot::Memo {
                value,
                deps: stored_deps,
            } => {
                let deps_equal = match stored_deps {
                    Some(stored) => stored.deps_eq(&deps),
                    None => false,
                };

                if deps_equal {
                    let b = value.borrow();
                    b.downcast_ref::<T>()
                        .unwrap_or_else(|| {
                            panic!(
                                "hook called in different order: slot \
                                 {slot_index} memo is not `{}`",
                                std::any::type_name::<T>()
                            )
                        })
                        .clone()
                } else {
                    let new_value = factory();
                    *value.borrow_mut() = Box::new(new_value.clone());
                    *stored_deps = Some(deps.clone_boxed());
                    new_value
                }
            }
            _ => panic!(
                "hook called in different order: slot {slot_index} held a \
                 non-Memo slot, now called as `use_memo`"
            ),
        }
    }

    pub fn use_callback<A, D, F>(&mut self, deps: D, f: F) -> Callback<A>
    where
        A: 'static,
        D: Deps,
        F: Fn(A) + 'static,
    {
        self.use_memo(deps, || Callback::new(f))
    }

    pub fn use_effect<D, F>(&mut self, deps: D, f: F)
    where
        D: Deps,
        F: FnOnce() + 'static,
    {
        self.use_effect_with_cleanup(deps, move || {
            f();
            None::<Cleanup>
        });
    }

    pub fn use_effect_with_cleanup<D, F, C>(&mut self, deps: D, f: F)
    where
        D: Deps,
        F: FnOnce() -> Option<C> + 'static,
        C: FnOnce() + 'static,
    {
        let slot_index = self.cursor;
        self.cursor += 1;

        self.debug_assert_on_ui_thread(slot_index);

        let mut hooks = self.hooks.borrow_mut();
        if slot_index == hooks.len() {
            let f_erased: PendingEffect = Box::new(move || f().map(|c| Box::new(c) as Cleanup));
            hooks.push(HookSlot::Effect {
                deps: Some(deps.clone_boxed()),
                pending: Some(f_erased),
                cleanup: None,
                pending_cleanup: None,
            });
            return;
        }

        match &mut hooks[slot_index] {
            HookSlot::Effect {
                deps: stored_deps,
                pending,
                cleanup,
                pending_cleanup,
            } => {
                let deps_equal = match stored_deps {
                    Some(stored) => stored.deps_eq(&deps),
                    None => false,
                };

                if !deps_equal {
                    if let Some(prev) = cleanup.take() {
                        *pending_cleanup = Some(prev);
                    }
                    *stored_deps = Some(deps.clone_boxed());

                    let f_erased: PendingEffect =
                        Box::new(move || f().map(|c| Box::new(c) as Cleanup));
                    *pending = Some(f_erased);
                }
            }
            _ => panic!(
                "hook called in different order: slot {slot_index} held a \
                 non-Effect slot, now called as `use_effect`"
            ),
        }
    }

    pub fn flush_effects(&mut self) {
        let hooks = Rc::clone(&self.hooks);
        let mut hooks = hooks.borrow_mut();

        for slot in hooks.iter_mut() {
            if let HookSlot::Effect {
                pending_cleanup, ..
            } = slot
                && let Some(c) = pending_cleanup.take() {
                    c();
                }
        }

        for slot in hooks.iter_mut() {
            if let HookSlot::Effect {
                pending, cleanup, ..
            } = slot
                && let Some(effect) = pending.take() {
                    let new_cleanup = effect();
                    *cleanup = new_cleanup;
                }
        }
    }

    pub fn run_cleanups(&mut self) {
        let hooks = Rc::clone(&self.hooks);
        let mut hooks = hooks.borrow_mut();
        for slot in hooks.iter_mut() {
            if let HookSlot::Effect {
                pending_cleanup,
                cleanup,
                ..
            } = slot
            {
                if let Some(c) = pending_cleanup.take() {
                    c();
                }
                if let Some(c) = cleanup.take() {
                    c();
                }
            }
        }
    }

    /// Returns the host window's client-area size in DIPs. Re-renders
    /// the component whenever the window is resized.
    pub fn use_inner_size(&self) -> WindowSize {
        self.read_contexts
            .borrow_mut()
            .insert(inner_size_context_id());
        self.inner_size.get()
    }

    /// Returns the host window's per-monitor DPI. Re-renders the component
    /// whenever the window moves to a monitor with a different scale factor.
    pub fn use_dpi(&self) -> u32 {
        self.read_contexts.borrow_mut().insert(dpi_context_id());
        self.dpi.get()
    }

    pub(crate) fn set_inner_size_cell(&mut self, cell: Rc<Cell<WindowSize>>) {
        self.inner_size = cell;
    }

    pub(crate) fn set_dpi_cell(&mut self, cell: Rc<Cell<u32>>) {
        self.dpi = cell;
    }

    fn state_slot<T>(&mut self, initial: T) -> (T, Rc<RefCell<Box<dyn Any>>>, usize)
    where
        T: 'static + Clone,
    {
        let slot_index = self.cursor;
        self.cursor += 1;

        self.debug_assert_on_ui_thread(slot_index);

        let (cell, stored_type_name) = {
            let mut hooks = self.hooks.borrow_mut();
            if slot_index == hooks.len() {
                let boxed: Box<dyn Any> = Box::new(initial);
                let cell = Rc::new(RefCell::new(boxed));
                let type_name = std::any::type_name::<T>();
                hooks.push(HookSlot::State {
                    cell: Rc::clone(&cell),
                    type_name,
                });
                (cell, type_name)
            } else {
                match &hooks[slot_index] {
                    HookSlot::State { cell, type_name } => (Rc::clone(cell), *type_name),
                    _ => panic!(
                        "hook called in different order: slot {slot_index} held a \
                         non-State slot, now called as `use_state`/`use_reducer`"
                    ),
                }
            }
        };

        let current = {
            let slot: Ref<Box<dyn Any>> = cell.borrow();
            match slot.downcast_ref::<T>() {
                Some(v) => v.clone(),
                None => panic!(
                    "hook called in different order: slot {slot_index} was \
                     `{stored_type_name}`, now called as `use_state::<{requested}>`",
                    requested = std::any::type_name::<T>()
                ),
            }
        };

        (current, cell, slot_index)
    }

    fn make_state_setter<T>(
        &self,
        cell: Rc<RefCell<Box<dyn Any>>>,
        slot_index: usize,
    ) -> SetState<T>
    where
        T: 'static + Clone + PartialEq,
    {
        let request = Rc::clone(&self.request_rerender);
        let dirty = Arc::clone(&self.state_dirty);
        let ui_thread = self.ui_thread;
        SetState {
            inner: Rc::new(move |value: T| {
                if let Some(tid) = ui_thread {
                    debug_assert_eq!(
                        tid,
                        thread::current().id(),
                        "SetState for slot {slot_index} called off the UI thread \
                         (expected {tid:?}, got {actual:?})",
                        actual = thread::current().id()
                    );
                }
                let mut slot = cell.borrow_mut();
                let prev = slot
                    .downcast_ref::<T>()
                    .unwrap();
                if *prev == value {
                    return;
                }
                *slot = Box::new(value);
                drop(slot);
                dirty.store(true, Ordering::Relaxed);
                request();
            }),
        }
    }

    fn make_updater<T>(&self, cell: Rc<RefCell<Box<dyn Any>>>, slot_index: usize) -> Updater<T>
    where
        T: 'static + Clone + PartialEq,
    {
        let request = Rc::clone(&self.request_rerender);
        let dirty = Arc::clone(&self.state_dirty);
        let ui_thread = self.ui_thread;
        Updater {
            inner: Rc::new(move |reducer: ReducerClosure<T>| {
                if let Some(tid) = ui_thread {
                    debug_assert_eq!(
                        tid,
                        thread::current().id(),
                        "Updater for slot {slot_index} called off the UI thread"
                    );
                }

                let prev = {
                    let slot = cell.borrow();
                    slot.downcast_ref::<T>()
                        .unwrap()
                        .clone()
                };
                let next = reducer(prev.clone());
                if prev == next {
                    return;
                }
                *cell.borrow_mut() = Box::new(next);
                dirty.store(true, Ordering::Relaxed);
                request();
            }),
        }
    }

    fn make_dispatch<S, A, R>(
        &self,
        cell: Rc<RefCell<Box<dyn Any>>>,
        slot_index: usize,
        reducer: R,
    ) -> Dispatch<A>
    where
        S: 'static + Clone + PartialEq,
        A: 'static,
        R: Fn(S, A) -> S + 'static,
    {
        let request = Rc::clone(&self.request_rerender);
        let dirty = Arc::clone(&self.state_dirty);
        let ui_thread = self.ui_thread;
        let reducer = Rc::new(reducer);
        Dispatch {
            inner: Rc::new(move |action: A| {
                if let Some(tid) = ui_thread {
                    debug_assert_eq!(
                        tid,
                        thread::current().id(),
                        "Dispatch for slot {slot_index} called off the UI thread"
                    );
                }
                let prev = {
                    let slot = cell.borrow();
                    slot.downcast_ref::<S>()
                        .unwrap()
                        .clone()
                };
                let next = reducer(prev.clone(), action);
                if prev == next {
                    return;
                }
                *cell.borrow_mut() = Box::new(next);
                dirty.store(true, Ordering::Relaxed);
                request();
            }),
        }
    }

    fn debug_assert_on_ui_thread(&self, slot_index: usize) {
        if let Some(tid) = self.ui_thread {
            debug_assert_eq!(
                tid,
                thread::current().id(),
                "hook at slot {slot_index} called off the UI thread"
            );
        }
    }
}
