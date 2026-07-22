use std::any::{Any, TypeId};
use std::cell::{Cell, Ref, RefCell};
use std::fmt;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread::{self, ThreadId};
use std::time::Instant;

use rustc_hash::FxHashSet;

use super::*;

/// Stateless render unit parameterised by props `P`. Implementors return
/// an [`Element`] tree from `render`; the host re-invokes `render` when
/// `should_update` returns `true` for the latest props.
pub trait Component<P = ()>: 'static
where
    P: Clone + PartialEq + 'static,
{
    fn render(&self, props: &P, cx: &mut RenderCx) -> Element;

    fn should_update(&self, old_props: &P, new_props: &P) -> bool {
        old_props != new_props
    }

    fn has_on_appeared(&self) -> bool {
        false
    }

    fn has_on_disappeared(&self) -> bool {
        false
    }

    fn on_appeared(&self, _props: &P, _cx: &mut RenderCx) {}

    fn on_disappeared(&self, _props: &P, _cx: &mut RenderCx) {}
}

/// Blanket impl: any `Fn(&P, &mut RenderCx) -> Element` is a [`Component<P>`].
///
/// ```ignore
/// fn greeting(props: &GreetingProps, _cx: &mut RenderCx) -> Element {
///     text_block(format!("Hello, {}!", props.name)).into()
/// }
/// ```
///
/// For unit-props, use `fn(_: &(), cx: &mut RenderCx) -> Element`.
impl<F, P> Component<P> for F
where
    F: Fn(&P, &mut RenderCx) -> Element + 'static,
    P: Clone + PartialEq + 'static,
{
    fn render(&self, props: &P, cx: &mut RenderCx) -> Element {
        (self)(props, cx)
    }
}

pub trait ComponentObject: 'static {
    fn render(&self, cx: &mut RenderCx) -> Element;

    fn props_as_any(&self) -> &dyn Any;

    fn component_type_id(&self) -> TypeId;

    fn is_equivalent(&self, other: &dyn ComponentObject) -> bool;

    fn should_update(&self, other: &dyn ComponentObject) -> bool;

    fn invoke_appeared(&self, cx: &mut RenderCx);

    fn invoke_disappeared(&self, cx: &mut RenderCx);

    fn has_on_appeared(&self) -> bool;

    fn has_on_disappeared(&self) -> bool;
}

struct ComponentCell<C, P>
where
    C: Component<P>,
    P: Clone + PartialEq + 'static,
{
    component: C,
    props: P,
}

impl<C, P> ComponentObject for ComponentCell<C, P>
where
    C: Component<P>,
    P: Clone + PartialEq + 'static,
{
    fn render(&self, cx: &mut RenderCx) -> Element {
        self.component.render(&self.props, cx)
    }

    fn props_as_any(&self) -> &dyn Any {
        &self.props
    }

    fn component_type_id(&self) -> TypeId {
        TypeId::of::<C>()
    }

    fn is_equivalent(&self, other: &dyn ComponentObject) -> bool {
        if self.component_type_id() != other.component_type_id() {
            return false;
        }
        let Some(other_props) = other.props_as_any().downcast_ref::<P>() else {
            return false;
        };
        &self.props == other_props
    }

    fn should_update(&self, other: &dyn ComponentObject) -> bool {
        if self.component_type_id() != other.component_type_id() {
            return true;
        }
        let Some(other_props) = other.props_as_any().downcast_ref::<P>() else {
            return true;
        };

        self.component.should_update(&self.props, other_props)
    }

    fn invoke_appeared(&self, cx: &mut RenderCx) {
        self.component.on_appeared(&self.props, cx);
    }

    fn invoke_disappeared(&self, cx: &mut RenderCx) {
        self.component.on_disappeared(&self.props, cx);
    }

    fn has_on_appeared(&self) -> bool {
        self.component.has_on_appeared()
    }

    fn has_on_disappeared(&self) -> bool {
        self.component.has_on_disappeared()
    }
}

/// Erased [`Component`]+props pair carried inside [`Element::Component`].
pub struct ComponentElement {
    pub key: Option<String>,
    pub memoised: bool,
    pub obj: Rc<dyn ComponentObject>,
}

impl Clone for ComponentElement {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            memoised: self.memoised,
            obj: Rc::clone(&self.obj),
        }
    }
}

impl fmt::Debug for ComponentElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ComponentElement")
            .field("key", &self.key)
            .field("memoised", &self.memoised)
            .field("component_type_id", &self.obj.component_type_id())
            .finish()
    }
}

impl PartialEq for ComponentElement {
    fn eq(&self, other: &Self) -> bool {
        if self.key != other.key || self.memoised != other.memoised {
            return false;
        }
        if self.memoised {
            self.obj.is_equivalent(&*other.obj)
        } else {
            !self.obj.should_update(&*other.obj)
        }
    }
}

/// Wrap a [`Component`] + props into an [`Element`] for embedding in a tree.
pub fn component<C, P>(component: C, props: P) -> Element
where
    C: Component<P>,
    P: Clone + PartialEq + 'static,
{
    let cell = ComponentCell { component, props };
    Element::Component(ComponentElement {
        key: None,
        memoised: false,
        obj: Rc::new(cell),
    })
}

/// Like [`component()`], but treat the component as memoised: the host
/// reuses the previous render output when props compare equal.
pub fn memo<C, P>(component: C, props: P) -> Element
where
    C: Component<P>,
    P: Clone + PartialEq + 'static,
{
    let cell = ComponentCell { component, props };
    Element::Component(ComponentElement {
        key: None,
        memoised: true,
        obj: Rc::new(cell),
    })
}

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

enum HookSlot {
    State {
        cell: Rc<RefCell<Box<dyn Any>>>,
        type_name: &'static str,
        handle: Option<Box<dyn Any>>,
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
            Self::State { type_name, .. } => write!(f, "State({type_name})"),
            Self::Memo { .. } => f.write_str("Memo { .. }"),
            Self::Effect {
                pending, cleanup, ..
            } => f
                .debug_struct("Effect")
                .field("pending", &pending.is_some())
                .field("has_cleanup", &cleanup.is_some())
                .finish(),
            Self::AsyncState { type_name, .. } => write!(f, "AsyncState({type_name})"),
        }
    }
}

impl_rc_fn_wrapper! {
    /// Setter returned by `use_state`; replaces the slot value and
    /// requests a rerender.
    pub struct SetState<T>(dyn Fn(T));
}

impl<T> SetState<T> {
    pub fn call(&self, value: T) {
        (self.inner)(value);
    }
}

impl<T: Clone + 'static> SetState<T> {
    /// Returns a parameterless handler that sets the state to `value`.
    ///
    /// Sugar for `move || set.call(value.clone())`, handy for `on_click`-style
    /// events that store a fixed or pre-computed value, e.g.
    /// `button("Reset").on_click(set_count.setter(0))`. When the handler simply
    /// forwards the event's own argument, pass the setter directly instead
    /// (`on_text_changed(set_text)`) to keep a stable identity.
    pub fn setter(&self, value: T) -> impl Fn() + Clone + 'static {
        let set = self.clone();
        move || set.call(value.clone())
    }
}

impl<T: 'static> From<SetState<T>> for Callback<T> {
    fn from(s: SetState<T>) -> Self {
        Self::from_rc(s.inner)
    }
}

type ReducerClosure<T> = Box<dyn FnOnce(T) -> T>;

impl_rc_fn_wrapper! {
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

impl_rc_fn_wrapper! {
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
    /// Owning host, so the marshalled write re-renders the correct host.
    host_id: HostId,
    _marker: std::marker::PhantomData<fn(T)>,
}

impl<T: Send + 'static> Clone for AsyncSetState<T> {
    fn clone(&self) -> Self {
        Self {
            cell: Arc::clone(&self.cell),
            marshaller: self.marshaller.clone(),
            dirty: Arc::clone(&self.dirty),
            type_name: self.type_name,
            host_id: self.host_id,
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
        let host_id = self.host_id;
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
            request_ui_rerender_on_ui_thread(host_id);
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
fn inner_size_context_id() -> ContextId {
    static ID: OnceLock<ContextId> = OnceLock::new();
    *ID.get_or_init(ContextId::new)
}

/// Stable `ContextId` used to track which components subscribed to
/// [`RenderCx::use_dpi`].
fn dpi_context_id() -> ContextId {
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
    /// Identifies the owning host so off-thread [`AsyncSetState`] writes route
    /// their rerender request to the correct host on the UI thread.
    host_id: HostId,
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
            host_id: HostId::next(),
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
            && let Some(v) = stack.get::<T>(context.id)
        {
            return v;
        }
        context.default.clone()
    }

    #[cfg(feature = "test")]
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

    /// The owning host's id. Used to route off-thread rerender requests.
    pub fn host_id(&self) -> HostId {
        self.host_id
    }

    fn set_request_rerender(&mut self, request_rerender: Rc<dyn Fn()>) {
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
        self.marshaller
            .clone()
            .expect("use_ui_marshaller requires a WinUI host")
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

        let marshaller = self
            .marshaller
            .clone()
            .expect("use_async_state requires a WinUI host");

        let setter = AsyncSetState::<T> {
            cell,
            marshaller,
            dirty: Arc::clone(&self.state_dirty),
            type_name,
            host_id: self.host_id,
            _marker: std::marker::PhantomData,
        };

        (current, setter)
    }

    pub fn use_state<T>(&mut self, initial: T) -> (T, SetState<T>)
    where
        T: 'static + Clone + PartialEq,
    {
        let (current, cell, slot_index) = self.state_slot(initial);
        let setter = self.memo_handle(slot_index, || self.make_state_setter::<T>(cell, slot_index));
        (current, setter)
    }

    /// Returns a per-slot handle (`SetState`, `Updater`, …) that is built once
    /// and reused for the life of the slot, so re-renders hand out the same
    /// `Rc` instead of allocating a fresh one each time. Stable identity also
    /// lets the reconciler skip subtrees whose handlers are unchanged. Only
    /// safe for handles that capture nothing render-specific (the synchronous
    /// state/reducer setters capture only the slot's stable context).
    fn memo_handle<H>(&self, slot_index: usize, build: impl FnOnce() -> H) -> H
    where
        H: Clone + 'static,
    {
        {
            let hooks = self.hooks.borrow();
            if let HookSlot::State {
                handle: Some(cached),
                ..
            } = &hooks[slot_index]
                && let Some(existing) = cached.downcast_ref::<H>()
            {
                return existing.clone();
            }
        }
        let built = build();
        if let HookSlot::State { handle, .. } = &mut self.hooks.borrow_mut()[slot_index] {
            *handle = Some(Box::new(built.clone()));
        }
        built
    }

    pub fn use_reducer<T>(&mut self, initial: T) -> (T, Updater<T>)
    where
        T: 'static + Clone + PartialEq,
    {
        let (current, cell, slot_index) = self.state_slot(initial);
        let updater = self.memo_handle(slot_index, || self.make_updater::<T>(cell, slot_index));
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
                handle: None,
            });
            return h;
        }

        match &hooks[slot_index] {
            HookSlot::State {
                cell, type_name, ..
            } => {
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
                && let Some(c) = pending_cleanup.take()
            {
                c();
            }
        }

        for slot in hooks.iter_mut() {
            if let HookSlot::Effect {
                pending, cleanup, ..
            } = slot
                && let Some(effect) = pending.take()
            {
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

    pub fn set_inner_size_cell(&mut self, cell: Rc<Cell<WindowSize>>) {
        self.inner_size = cell;
    }

    pub fn set_dpi_cell(&mut self, cell: Rc<Cell<u32>>) {
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
                    handle: None,
                });
                (cell, type_name)
            } else {
                match &hooks[slot_index] {
                    HookSlot::State {
                        cell, type_name, ..
                    } => (Rc::clone(cell), *type_name),
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
                let prev = slot.downcast_ref::<T>().unwrap();
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
                    slot.downcast_ref::<T>().unwrap().clone()
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
                    slot.downcast_ref::<S>().unwrap().clone()
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

/// Per-render telemetry exposed by [`RenderHost::stats`].
#[derive(Copy, Clone, Debug, Default)]
pub struct RenderStats {
    pub fps: f64,
    pub renders_in_window: u32,
    pub total_renders: u64,
    pub avg_tree_build_ms: f64,
    pub avg_reconcile_ms: f64,
    pub avg_effects_ms: f64,
    pub last_diffed: u64,
    pub last_skipped: u64,
    pub last_created: u64,
}

/// Information passed to the [`RenderHost::set_render_complete`] callback
/// after each reconcile pass completes.
#[derive(Copy, Clone, Debug)]
pub struct RenderCompleteInfo {
    pub tree_build_ms: f64,
    pub reconcile_ms: f64,
    pub effects_ms: f64,
    pub elements_diffed: u64,
    pub elements_skipped: u64,
    pub elements_created: u64,
}

struct StatsAccumulator {
    window_start: Cell<Instant>,
    tree_build_sum_ms: Cell<f64>,
    reconcile_sum_ms: Cell<f64>,
    effects_sum_ms: Cell<f64>,
    renders_in_window: Cell<u32>,
    total_renders: Cell<u64>,
}

impl StatsAccumulator {
    fn new() -> Self {
        Self {
            window_start: Cell::new(Instant::now()),
            tree_build_sum_ms: Cell::new(0.0),
            reconcile_sum_ms: Cell::new(0.0),
            effects_sum_ms: Cell::new(0.0),
            renders_in_window: Cell::new(0),
            total_renders: Cell::new(0),
        }
    }

    #[expect(clippy::too_many_arguments)]
    fn record(
        &self,
        prev: RenderStats,
        tree_build_ms: f64,
        reconcile_ms: f64,
        effects_ms: f64,
        last_diffed: u64,
        last_skipped: u64,
        last_created: u64,
    ) -> RenderStats {
        self.tree_build_sum_ms
            .set(self.tree_build_sum_ms.get() + tree_build_ms);
        self.reconcile_sum_ms
            .set(self.reconcile_sum_ms.get() + reconcile_ms);
        self.effects_sum_ms
            .set(self.effects_sum_ms.get() + effects_ms);
        self.renders_in_window.set(self.renders_in_window.get() + 1);
        self.total_renders.set(self.total_renders.get() + 1);

        let window = self.window_start.get().elapsed();
        if window.as_secs_f64() >= 1.0 {
            let renders = self.renders_in_window.get();
            let fps = renders as f64 / window.as_secs_f64().max(f64::EPSILON);
            let avg = |sum: f64| {
                if renders > 0 {
                    sum / renders as f64
                } else {
                    0.0
                }
            };

            let snapshot = RenderStats {
                fps,
                renders_in_window: renders,
                total_renders: self.total_renders.get(),
                avg_tree_build_ms: avg(self.tree_build_sum_ms.get()),
                avg_reconcile_ms: avg(self.reconcile_sum_ms.get()),
                avg_effects_ms: avg(self.effects_sum_ms.get()),
                last_diffed,
                last_skipped,
                last_created,
            };

            self.tree_build_sum_ms.set(0.0);
            self.reconcile_sum_ms.set(0.0);
            self.effects_sum_ms.set(0.0);
            self.renders_in_window.set(0);
            self.window_start.set(Instant::now());

            snapshot
        } else {
            RenderStats {
                last_diffed,
                last_skipped,
                last_created,
                total_renders: self.total_renders.get(),
                ..prev
            }
        }
    }
}

/// Owns a [`Reconciler`] and a root [`Component`], coalescing render
/// requests onto the supplied [`Dispatcher`] (typically the UI thread).
pub struct RenderHost<B: Backend, D: Dispatcher> {
    inner: Rc<RenderHostInner<B, D>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum RenderState {
    Idle,
    Pending,
    Rendering,
    RenderingDirty,
}

struct RenderHostInner<B: Backend, D: Dispatcher> {
    reconciler: RefCell<Reconciler<B>>,
    root: RefCell<Box<dyn Component>>,
    render_cx: RefCell<RenderCx>,
    root_id: Cell<Option<ControlId>>,
    last_tree: RefCell<Option<Element>>,
    render_state: Cell<RenderState>,
    dispatcher: D,
    /// Optional `Send + Sync` marshaller used to support
    /// [`RenderCx::use_async_state`]. When `None`, async setters
    /// will panic if any component asks for one.
    marshaller: RefCell<Option<UiMarshaller>>,
    render_count: Cell<u32>,
    stats: Cell<RenderStats>,
    post_render: RefCell<Option<Box<dyn Fn(Option<ControlId>)>>>,
    render_complete: RefCell<Option<Box<dyn Fn(&RenderCompleteInfo)>>>,
    stats_accum: StatsAccumulator,
    inner_size: Rc<Cell<WindowSize>>,
    dpi: Rc<Cell<u32>>,
    /// Owning host id, mirrored from the `RenderCx` so `set_marshaller` can
    /// install/clear this host's UI-thread rerender hook, and `Drop` can
    /// remove it when the host (e.g. a secondary window) is torn down.
    host_id: HostId,
}

impl<B: Backend, D: Dispatcher> Drop for RenderHostInner<B, D> {
    fn drop(&mut self) {
        set_ui_rerender(self.host_id, None);
    }
}

impl<B: Backend + 'static, D: Dispatcher + 'static> RenderHost<B, D> {
    pub fn new(backend: B, root: Box<dyn Component>, dispatcher: D) -> Self {
        let mut reconciler = Reconciler::new(backend);
        let mut render_cx = RenderCx::new(Rc::new(|| {}));
        let host_id = render_cx.host_id();

        let inner_size = Rc::new(Cell::new(WindowSize::default()));
        let dpi = Rc::new(Cell::new(96_u32));
        render_cx.set_inner_size_cell(Rc::clone(&inner_size));
        render_cx.set_dpi_cell(Rc::clone(&dpi));
        reconciler.set_inner_size_cell(Rc::clone(&inner_size));
        reconciler.set_dpi_cell(Rc::clone(&dpi));

        Self {
            inner: Rc::new(RenderHostInner {
                reconciler: RefCell::new(reconciler),
                root: RefCell::new(root),
                render_cx: RefCell::new(render_cx),
                root_id: Cell::new(None),
                last_tree: RefCell::new(None),
                render_state: Cell::new(RenderState::Idle),
                dispatcher,
                marshaller: RefCell::new(None),
                render_count: Cell::new(0),
                stats: Cell::new(RenderStats::default()),
                post_render: RefCell::new(None),
                render_complete: RefCell::new(None),
                stats_accum: StatsAccumulator::new(),
                inner_size,
                dpi,
                host_id,
            }),
        }
    }

    pub fn stats(&self) -> RenderStats {
        self.inner.stats.get()
    }

    /// Install (or replace) the [`UiMarshaller`] used to support
    /// off-UI-thread state writes via [`RenderCx::use_async_state`], and
    /// publish the host's rerender hook to the UI thread's `UI_RERENDER`
    /// slot. Passing `None` clears both.
    pub fn set_marshaller(&self, marshaller: Option<UiMarshaller>) {
        self.inner.marshaller.borrow_mut().clone_from(&marshaller);
        self.inner
            .render_cx
            .borrow_mut()
            .set_marshaller(marshaller.clone());
        self.inner
            .reconciler
            .borrow_mut()
            .set_marshaller(marshaller.clone());

        if marshaller.is_some() {
            let weak = Rc::downgrade(&self.inner);
            let rerender: Rc<dyn Fn()> = Rc::new(move || {
                if let Some(strong) = weak.upgrade() {
                    request_render(&strong);
                }
            });
            set_ui_rerender(self.inner.host_id, Some(rerender));
        } else {
            set_ui_rerender(self.inner.host_id, None);
        }
    }

    pub fn kick(&self) {
        let weak = Rc::downgrade(&self.inner);
        let request = Rc::new(move || {
            if let Some(strong) = weak.upgrade() {
                request_render(&strong);
            }
        });
        self.inner
            .render_cx
            .borrow_mut()
            .set_request_rerender(request);
        request_render(&self.inner);
    }

    pub fn request_render(&self) {
        request_render(&self.inner);
    }

    pub fn render_count(&self) -> u32 {
        self.inner.render_count.get()
    }

    #[cfg(feature = "test")]
    pub fn is_render_pending(&self) -> bool {
        self.inner.render_state.get() != RenderState::Idle
    }

    #[cfg(feature = "test")]
    pub fn is_rendering(&self) -> bool {
        matches!(
            self.inner.render_state.get(),
            RenderState::Rendering | RenderState::RenderingDirty
        )
    }

    #[cfg(feature = "test")]
    pub fn needs_rerender(&self) -> bool {
        matches!(
            self.inner.render_state.get(),
            RenderState::Pending | RenderState::RenderingDirty
        )
    }

    pub fn is_idle(&self) -> bool {
        self.inner.render_state.get() == RenderState::Idle
    }

    pub fn root_id(&self) -> Option<ControlId> {
        self.inner.root_id.get()
    }

    pub fn with_reconciler<R>(&self, f: impl FnOnce(&Reconciler<B>) -> R) -> R {
        f(&self.inner.reconciler.borrow())
    }

    pub fn with_reconciler_mut<R>(&self, f: impl FnOnce(&mut Reconciler<B>) -> R) -> R {
        f(&mut self.inner.reconciler.borrow_mut())
    }

    pub fn clone_inner(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }

    pub fn set_post_render<F>(&self, f: F)
    where
        F: Fn(Option<ControlId>) + 'static,
    {
        *self.inner.post_render.borrow_mut() = Some(Box::new(f));
    }

    pub fn set_render_complete<F>(&self, f: F)
    where
        F: Fn(&RenderCompleteInfo) + 'static,
    {
        *self.inner.render_complete.borrow_mut() = Some(Box::new(f));
    }

    pub fn inner_size(&self) -> WindowSize {
        self.inner.inner_size.get()
    }

    pub fn set_inner_size(&self, size: WindowSize) {
        if self.inner.inner_size.get() == size {
            return;
        }
        self.inner.inner_size.set(size);
        if let Some(root_id) = self.inner.root_id.get() {
            let mut changed = FxHashSet::default();
            changed.insert(inner_size_context_id());
            self.inner
                .reconciler
                .borrow_mut()
                .force_context_subscribers(root_id, &changed);
        }
        self.request_render();
    }

    pub fn dpi(&self) -> u32 {
        self.inner.dpi.get()
    }

    pub fn set_dpi(&self, dpi: u32) {
        if self.inner.dpi.get() == dpi {
            return;
        }
        self.inner.dpi.set(dpi);
        if let Some(root_id) = self.inner.root_id.get() {
            let mut changed = FxHashSet::default();
            changed.insert(dpi_context_id());
            self.inner
                .reconciler
                .borrow_mut()
                .force_context_subscribers(root_id, &changed);
        }
        self.request_render();
    }
}

fn request_render<B: Backend + 'static, D: Dispatcher + 'static>(
    inner: &Rc<RenderHostInner<B, D>>,
) {
    match inner.render_state.get() {
        RenderState::Idle => {
            inner.render_state.set(RenderState::Pending);
            let inner_c = Rc::clone(inner);
            if !inner.dispatcher.enqueue(
                DispatcherQueuePriority::Normal,
                Box::new(move || render_loop(&inner_c)),
            ) {
                inner.render_state.set(RenderState::Idle);
            }
        }
        RenderState::Pending | RenderState::RenderingDirty => {}
        RenderState::Rendering => {
            inner.render_state.set(RenderState::RenderingDirty);
        }
    }
}

fn render_loop<B: Backend + 'static, D: Dispatcher + 'static>(inner: &Rc<RenderHostInner<B, D>>) {
    inner.render_state.set(RenderState::Rendering);
    render_once(inner);

    if inner.render_state.get() == RenderState::RenderingDirty {
        inner.render_state.set(RenderState::Pending);
        let inner_c = Rc::clone(inner);
        if !inner.dispatcher.enqueue(
            DispatcherQueuePriority::Low,
            Box::new(move || render_loop(&inner_c)),
        ) {
            inner.render_state.set(RenderState::Idle);
        }
    } else {
        inner.render_state.set(RenderState::Idle);
    }
}

fn render_once<B: Backend + 'static, D: Dispatcher + 'static>(inner: &Rc<RenderHostInner<B, D>>) {
    fault::render_scope(|| render_once_inner(inner));
}

fn render_once_inner<B: Backend + 'static, D: Dispatcher + 'static>(
    inner: &Rc<RenderHostInner<B, D>>,
) {
    let tree_build_started = Instant::now();
    let new_tree = {
        let mut cx = inner.render_cx.borrow_mut();
        cx.begin_render();
        let root = inner.root.borrow();
        root.render(&(), &mut cx)
    };
    let tree_build_ms = tree_build_started.elapsed().as_secs_f64() * 1000.0;

    let reconcile_started = Instant::now();
    // Take (not clone) the old tree to avoid deep-cloning every frame.
    let old_tree_owned = inner.last_tree.borrow_mut().take();
    let (new_root_id, last_diffed, last_skipped, last_created) = {
        let mut reconciler = inner.reconciler.borrow_mut();
        reconciler.reset_stats();

        let existing = inner.root_id.get();
        let weak = Rc::downgrade(inner);
        let request_rerender: Rc<dyn Fn()> = Rc::new(move || {
            if let Some(strong) = weak.upgrade() {
                request_render(&strong);
            }
        });
        let id = reconciler.reconcile(
            old_tree_owned.as_ref(),
            &new_tree,
            existing,
            request_rerender,
        );
        reconciler.drain_realizations();
        reconciler.clear_forced_component_rerender();
        (
            id,
            reconciler.debug_elements_diffed,
            reconciler.debug_elements_skipped,
            reconciler.debug_ui_elements_created,
        )
    };
    let reconcile_ms = reconcile_started.elapsed().as_secs_f64() * 1000.0;
    inner.root_id.set(new_root_id);

    *inner.last_tree.borrow_mut() = Some(new_tree);

    let effects_started = Instant::now();
    inner.render_cx.borrow_mut().flush_effects();
    let effects_ms = effects_started.elapsed().as_secs_f64() * 1000.0;

    inner.render_count.set(inner.render_count.get() + 1);

    let cb_taken = inner.post_render.borrow_mut().take();
    if let Some(cb) = cb_taken {
        cb(inner.root_id.get());

        let mut slot = inner.post_render.borrow_mut();
        if slot.is_none() {
            *slot = Some(cb);
        }
    }

    let prev_stats = inner.stats.get();
    let new_stats = inner.stats_accum.record(
        prev_stats,
        tree_build_ms,
        reconcile_ms,
        effects_ms,
        last_diffed,
        last_skipped,
        last_created,
    );
    inner.stats.set(new_stats);

    let rc_taken = inner.render_complete.borrow_mut().take();
    if let Some(cb) = rc_taken {
        let info = RenderCompleteInfo {
            tree_build_ms,
            reconcile_ms,
            effects_ms,
            elements_diffed: last_diffed,
            elements_skipped: last_skipped,
            elements_created: last_created,
        };
        cb(&info);
        let mut slot = inner.render_complete.borrow_mut();
        if slot.is_none() {
            *slot = Some(cb);
        }
    }
}
