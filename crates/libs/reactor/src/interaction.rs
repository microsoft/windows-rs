// Interaction: callbacks, keyboard shortcuts, context propagation, and async resources.

use std::any::{Any, TypeId};
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use rustc_hash::FxHashMap;

use super::*;

// ─── impl_rc_fn_wrapper! macro ──────────────────────────────────────────

/// Generate an `Rc`-pointer-equal newtype around `dyn Fn(...)`. Provides
/// `new`, `Clone`, `Debug` (pointer-formatted), and `PartialEq`/`Eq`
/// based on `Rc::ptr_eq`.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_rc_fn_wrapper {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident $(< $param:ident >)? (
            dyn Fn ( $($args:tt)* ) $(-> $ret:ty)?
        );
    ) => {
        $(#[$attr])*
        $vis struct $name $(< $param >)? {
            inner: ::std::rc::Rc<dyn Fn($($args)*) $(-> $ret)?>,
        }

        impl $(< $param >)? $name $(< $param >)? {
            pub fn new<__F>(f: __F) -> Self
            where
                __F: Fn($($args)*) $(-> $ret)? + 'static,
            {
                Self { inner: ::std::rc::Rc::new(f) }
            }
        }

        impl $(< $param >)? ::std::clone::Clone for $name $(< $param >)? {
            fn clone(&self) -> Self {
                Self { inner: ::std::rc::Rc::clone(&self.inner) }
            }
        }

        impl $(< $param >)? ::std::fmt::Debug for $name $(< $param >)? {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&::std::format_args!(
                        "{:p}",
                        ::std::rc::Rc::as_ptr(&self.inner)
                    ))
                    .finish()
            }
        }

        impl $(< $param >)? ::std::cmp::PartialEq for $name $(< $param >)? {
            fn eq(&self, other: &Self) -> bool {
                ::std::rc::Rc::ptr_eq(&self.inner, &other.inner)
            }
        }

        impl $(< $param >)? ::std::cmp::Eq for $name $(< $param >)? {}
    };
}

/// Like [`impl_rc_fn_wrapper!`] but Arc-based; the closure must be
/// `Send + Sync`, allowing the wrapper itself to be sent across threads.
#[macro_export]
#[doc(hidden)]
macro_rules! impl_arc_fn_wrapper {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident $(< $param:ident >)? (
            dyn Fn ( $($args:tt)* ) $(-> $ret:ty)?
        );
    ) => {
        $(#[$attr])*
        $vis struct $name $(< $param >)? {
            inner: ::std::sync::Arc<dyn Fn($($args)*) $(-> $ret)? + Send + Sync>,
        }

        impl $(< $param >)? $name $(< $param >)? {
            pub fn new<__F>(f: __F) -> Self
            where
                __F: Fn($($args)*) $(-> $ret)? + Send + Sync + 'static,
            {
                Self { inner: ::std::sync::Arc::new(f) }
            }
        }

        impl $(< $param >)? ::std::clone::Clone for $name $(< $param >)? {
            fn clone(&self) -> Self {
                Self { inner: ::std::sync::Arc::clone(&self.inner) }
            }
        }

        impl $(< $param >)? ::std::fmt::Debug for $name $(< $param >)? {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&::std::format_args!(
                        "{:p}",
                        ::std::sync::Arc::as_ptr(&self.inner)
                    ))
                    .finish()
            }
        }

        impl $(< $param >)? ::std::cmp::PartialEq for $name $(< $param >)? {
            fn eq(&self, other: &Self) -> bool {
                ::std::sync::Arc::ptr_eq(&self.inner, &other.inner)
            }
        }

        impl $(< $param >)? ::std::cmp::Eq for $name $(< $param >)? {}

        unsafe impl $(< $param >)? Send for $name $(< $param >)? {}
        unsafe impl $(< $param >)? Sync for $name $(< $param >)? {}
    };
}

// ─── Callback ───────────────────────────────────────────────────────────

impl_rc_fn_wrapper! {
    /// Cheap-to-clone reference-counted callback. Two clones of the same
    /// `Callback` compare equal (`Rc` pointer equality), letting the
    /// reconciler skip rebinding when the same handler is re-rendered.
    pub struct Callback<T>(dyn Fn(T));
}

impl<T> Callback<T> {
    pub fn invoke(&self, arg: T) {
        fault::catch("event handler", || (self.inner)(arg));
    }

    /// Construct a `Callback` from a raw `Rc<dyn Fn(T)>`. Used internally
    /// to bridge `SetState` / `Dispatch` into `Callback` without cloning.
    pub fn from_rc(inner: Rc<dyn Fn(T)>) -> Self {
        Self { inner }
    }

    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.inner)
    }
}

/// Trait for types that can be converted into a [`Callback<T>`].
///
/// Implemented for closures (`Fn(T) + 'static`) and state setters
/// ([`SetState<T>`], [`Dispatch<A>`]), allowing them to be passed
/// directly to event handler methods without wrapping in a manual closure.
///
/// # Examples
/// ```ignore
/// // Before: manual adapter closure
/// text_box(name).on_text_changed(move |v| set_name.call(v))
///
/// // After: pass the setter directly
/// text_box(name).on_text_changed(set_name)
/// ```
pub trait IntoCallback<T> {
    fn into_callback(self) -> Callback<T>;
}

impl<T, F> IntoCallback<T> for F
where
    F: Fn(T) + 'static,
{
    fn into_callback(self) -> Callback<T> {
        Callback::new(self)
    }
}

impl<T: 'static> IntoCallback<T> for Callback<T> {
    fn into_callback(self) -> Self {
        self
    }
}

impl<T: 'static> IntoCallback<T> for SetState<T> {
    fn into_callback(self) -> Callback<T> {
        self.into()
    }
}

impl<T: 'static> IntoCallback<T> for Dispatch<T> {
    fn into_callback(self) -> Callback<T> {
        self.into()
    }
}

/// Trait for types that can be converted into a parameterless
/// [`Callback<()>`]. The sibling of [`IntoCallback`] for `on_*` handlers
/// that take no argument (e.g. `Button::on_click`): a bare `Fn()` closure
/// is wrapped, while an existing `Callback<()>` (e.g. from
/// [`RenderCx::use_callback`]) is passed through unchanged so its identity
/// is preserved and `can_skip_update` can skip the control.
pub trait IntoUnitCallback {
    fn into_unit_callback(self) -> Callback<()>;
}

impl<F> IntoUnitCallback for F
where
    F: Fn() + 'static,
{
    fn into_unit_callback(self) -> Callback<()> {
        Callback::new(move |()| self())
    }
}

impl IntoUnitCallback for Callback<()> {
    fn into_unit_callback(self) -> Callback<()> {
        self
    }
}

// ─── Keyboard ───────────────────────────────────────────────────────────

/// A single keyboard shortcut bound to an element via
/// [`Modifiers`]`.keyboard_accelerators`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyboardAccelerator {
    pub key: VirtualKey,
    pub modifiers: VirtualKeyModifiers,
    pub on_invoked: Callback<()>,
}

impl KeyboardAccelerator {
    pub fn new<F: Fn() + 'static>(
        key: VirtualKey,
        modifiers: VirtualKeyModifiers,
        on_invoked: F,
    ) -> Self {
        Self {
            key,
            modifiers,
            on_invoked: Callback::new(move |()| on_invoked()),
        }
    }
}

// ─── Context ────────────────────────────────────────────────────────────

/// Process-wide unique identifier for a [`Context`] of any value type.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ContextId(u32);

static NEXT_CONTEXT_ID: AtomicU32 = AtomicU32::new(1);

impl ContextId {
    pub fn new() -> Self {
        Self(NEXT_CONTEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl Default for ContextId {
    fn default() -> Self {
        Self::new()
    }
}

/// Typed lookup key + default for a value made available to descendants
/// via [`ProviderElement`] and [`RenderCx::use_context()`].
pub struct Context<T> {
    pub default: T,
    pub id: ContextId,
}

impl<T> Context<T> {
    pub fn new(default: T) -> Self {
        Self {
            default,
            id: ContextId::new(),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Context<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("id", &self.id)
            .field("default", &self.default)
            .finish()
    }
}

struct ContextEntry {
    value_type_id: TypeId,
    value: Rc<dyn Any>,
}

#[derive(Default)]
pub struct ContextStack {
    entries: std::cell::RefCell<FxHashMap<ContextId, Vec<ContextEntry>>>,
    push_order: std::cell::RefCell<Vec<ContextId>>,
}

impl ContextStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push<T: 'static>(&self, context_id: ContextId, value: T) -> ContextScope<'_> {
        self.push_raw_retain(context_id, TypeId::of::<T>(), Rc::new(value));
        ContextScope { stack: self }
    }

    pub fn push_raw_retain(
        &self,
        context_id: ContextId,
        value_type_id: TypeId,
        value: Rc<dyn Any>,
    ) {
        self.entries
            .borrow_mut()
            .entry(context_id)
            .or_default()
            .push(ContextEntry {
                value_type_id,
                value,
            });
        self.push_order.borrow_mut().push(context_id);
    }

    pub fn pop_raw(&self) {
        if let Some(id) = self.push_order.borrow_mut().pop() {
            let mut entries = self.entries.borrow_mut();
            if let Some(stack) = entries.get_mut(&id) {
                stack.pop();
                if stack.is_empty() {
                    entries.remove(&id);
                }
            }
        }
    }

    pub fn get<T: 'static + Clone>(&self, context_id: ContextId) -> Option<T> {
        let entries = self.entries.borrow();
        let entry = entries.get(&context_id)?.last()?;
        assert!(
            entry.value_type_id == TypeId::of::<T>(),
            "context type mismatch for context {:?}: provided as {:?}, consumed as {:?}",
            context_id,
            entry.value_type_id,
            TypeId::of::<T>()
        );
        entry.value.downcast_ref::<T>().cloned()
    }

    fn pop(&self) {
        self.pop_raw();
    }
}

pub struct ContextScope<'a> {
    stack: &'a ContextStack,
}

impl Drop for ContextScope<'_> {
    fn drop(&mut self) {
        self.stack.pop();
    }
}

#[derive(Clone)]
pub struct ContextProvision {
    pub context_id: ContextId,
    pub value_type_id: TypeId,
    pub value: Rc<dyn Any>,
    eq_fn: fn(&dyn Any, &dyn Any) -> bool,
}

impl ContextProvision {
    pub fn new<T: Clone + PartialEq + 'static>(id: ContextId, value: T) -> Self {
        Self {
            context_id: id,
            value_type_id: TypeId::of::<T>(),
            value: Rc::new(value),
            eq_fn: |a, b| match (a.downcast_ref::<T>(), b.downcast_ref::<T>()) {
                (Some(a), Some(b)) => a == b,
                _ => false,
            },
        }
    }
}

impl std::fmt::Debug for ContextProvision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ContextProvision")
            .field("context_id", &self.context_id)
            .field("value_type_id", &self.value_type_id)
            .finish()
    }
}

impl PartialEq for ContextProvision {
    fn eq(&self, other: &Self) -> bool {
        if self.context_id != other.context_id || self.value_type_id != other.value_type_id {
            return false;
        }
        if Rc::ptr_eq(&self.value, &other.value) {
            return true;
        }
        (self.eq_fn)(self.value.as_ref(), other.value.as_ref())
    }
}

#[derive(Clone, Debug)]
pub struct ProviderElement {
    pub key: Option<String>,
    pub provisions: Vec<ContextProvision>,
    pub child: Box<Element>,
}

impl PartialEq for ProviderElement {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.provisions == other.provisions && self.child == other.child
    }
}

// ─── Resource ───────────────────────────────────────────────────────────

/// Represents the lifecycle of an async data fetch.
#[derive(Clone, Debug, PartialEq)]
pub enum Resource<T> {
    /// No fetch has been initiated (initial state before first render cycle).
    Idle,
    /// A fetch is in progress (no prior data available).
    Loading,
    /// Data was loaded successfully.
    Ready(T),
    /// A refetch is in progress but previous data is still available.
    Reloading(T),
    /// The fetch failed.
    Error(String),
}

impl<T> Resource<T> {
    /// Returns the data if in `Ready` or `Reloading` state.
    pub fn data(&self) -> Option<&T> {
        match self {
            Self::Ready(v) | Self::Reloading(v) => Some(v),
            _ => None,
        }
    }

    /// Returns `true` if currently loading or reloading.
    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading | Self::Reloading(_))
    }

    /// Returns the error string if in `Error` state.
    pub fn error(&self) -> Option<&str> {
        match self {
            Self::Error(e) => Some(e.as_str()),
            _ => None,
        }
    }

    /// Returns `true` if data is available (Ready or Reloading).
    pub fn has_data(&self) -> bool {
        matches!(self, Self::Ready(_) | Self::Reloading(_))
    }

    /// Render the resource with a `ready` closure; loading and error use
    /// sensible defaults (overridable via [`ResourceView::loading`] /
    /// [`ResourceView::error`]).
    ///
    /// # Example
    /// ```ignore
    /// let items = cx.use_resource(fetch_items, page);
    /// items.view(|data| {
    ///     vstack(data.iter().map(|s| -> Element { text_block(s).into() }).collect::<Vec<Element>>())
    ///         .into()
    /// })
    /// ```
    pub fn view<F, R>(&self, ready: F) -> ResourceView<'_, T, impl FnOnce(&T) -> Element>
    where
        F: FnOnce(&T) -> R,
        R: Into<Element>,
    {
        ResourceView {
            resource: self,
            ready: move |data: &T| ready(data).into(),
            loading: None,
            error: None,
        }
    }
}

/// Builder returned by [`Resource::view()`] — call `.loading()` or `.error()`
/// to override defaults before converting to [`Element`] via `.into()`.
pub struct ResourceView<'a, T, F>
where
    F: FnOnce(&T) -> Element,
{
    resource: &'a Resource<T>,
    ready: F,
    loading: Option<Element>,
    error: Option<Box<dyn FnOnce(&str) -> Element>>,
}

impl<'a, T, F> ResourceView<'a, T, F>
where
    F: FnOnce(&T) -> Element,
{
    /// Override the loading state element (default: indeterminate ProgressRing).
    pub fn loading(mut self, el: impl Into<Element>) -> Self {
        self.loading = Some(el.into());
        self
    }

    /// Override the error state renderer (default: text block showing the error).
    pub fn error<E>(mut self, f: E) -> Self
    where
        E: FnOnce(&str) -> Element + 'static,
    {
        self.error = Some(Box::new(f));
        self
    }
}

impl<T, F> From<ResourceView<'_, T, F>> for Element
where
    F: FnOnce(&T) -> Self,
{
    fn from(rv: ResourceView<'_, T, F>) -> Self {
        match rv.resource {
            Resource::Loading | Resource::Idle => rv
                .loading
                .unwrap_or_else(|| ProgressRing::indeterminate().into()),
            Resource::Ready(data) | Resource::Reloading(data) => (rv.ready)(data),
            Resource::Error(e) => rv.error.map_or_else(
                || text_block(format!("Error: {e}")).into(),
                |f| f(e.as_str()),
            ),
        }
    }
}

/// Represents the lifecycle of an async write/mutation.
#[derive(Clone, Debug, PartialEq)]
pub enum MutationState<T> {
    /// No mutation in progress.
    Idle,
    /// A mutation is in progress.
    Loading,
    /// The mutation completed successfully.
    Success(T),
    /// The mutation failed.
    Error(String),
}

impl<T> MutationState<T> {
    pub fn is_loading(&self) -> bool {
        matches!(self, Self::Loading)
    }

    pub fn is_idle(&self) -> bool {
        matches!(self, Self::Idle)
    }

    pub fn data(&self) -> Option<&T> {
        match self {
            Self::Success(v) => Some(v),
            _ => None,
        }
    }

    pub fn error(&self) -> Option<&str> {
        match self {
            Self::Error(e) => Some(e.as_str()),
            _ => None,
        }
    }
}

/// Monotonic counter used to discard results from stale fetches.
#[derive(Clone)]
struct Generation(Arc<AtomicU64>);

impl Generation {
    fn new() -> Self {
        Self(Arc::new(AtomicU64::new(0)))
    }

    fn advance(&self) -> u64 {
        self.0.fetch_add(1, Ordering::AcqRel) + 1
    }

    fn is_current(&self, value: u64) -> bool {
        self.0.load(Ordering::Acquire) == value
    }
}

impl RenderCx {
    /// Fetch data on a background thread, refetching when `deps` change.
    ///
    /// ```ignore
    /// let users = cx.use_resource(
    ///     |filter| api::search_users_blocking(&filter),
    ///     search_term.clone(),
    /// );
    /// ```
    ///
    /// If you need an async runtime, create one inside the fetcher:
    /// `|deps| { tokio::runtime::Runtime::new()?.block_on(fetch(deps)) }`
    pub fn use_resource<T, D, F>(&mut self, fetcher: F, deps: D) -> Resource<T>
    where
        T: Send + Clone + PartialEq + 'static,
        D: Deps + Clone + Send + 'static,
        F: Fn(D) -> std::result::Result<T, String> + Send + 'static,
    {
        // Hook 1: the resource state
        let (state, set_state) = self.use_async_state(Resource::<T>::Loading);

        let generation = self.use_ref(Generation::new());

        let deps_clone = deps.clone();
        let gen_handle = generation;
        self.use_effect(deps, move || {
            let my_gen = gen_handle.borrow().advance();
            let set_state2 = set_state.clone();
            set_state.call(Resource::Loading);

            let fetcher_deps = deps_clone.clone();
            let gen_for_thread = gen_handle.get_cloned();

            windows_threading::submit(move || {
                let result = fetcher(fetcher_deps);
                if gen_for_thread.is_current(my_gen) {
                    match result {
                        Ok(data) => set_state2.call(Resource::Ready(data)),
                        Err(e) => set_state2.call(Resource::Error(e)),
                    }
                }
            });
        });

        state
    }

    /// Mutation hook for async write operations. Returns the mutation state
    /// and a trigger function.
    ///
    /// ```ignore
    /// let (state, mutate) = cx.use_mutation();
    /// // In an event handler:
    /// mutate.fire(|| api::save_user_blocking(&user));
    /// ```
    pub fn use_mutation<T>(&mut self) -> (MutationState<T>, MutationTrigger<T>)
    where
        T: Send + Clone + PartialEq + 'static,
    {
        let (state, set_state) = self.use_async_state(MutationState::<T>::Idle);
        let trigger = MutationTrigger { set_state };
        (state, trigger)
    }
}

/// Handle returned by [`RenderCx::use_mutation`] to fire async write
/// operations from event handlers.
#[derive(Clone)]
pub struct MutationTrigger<T: Send + Clone + PartialEq + 'static> {
    set_state: AsyncSetState<MutationState<T>>,
}

impl<T: Send + Clone + PartialEq + 'static> MutationTrigger<T> {
    /// Fire a mutation on a background thread.
    pub fn fire<F>(&self, f: F)
    where
        F: FnOnce() -> std::result::Result<T, String> + Send + 'static,
    {
        let set = self.set_state.clone();
        set.call(MutationState::Loading);
        let set2 = self.set_state.clone();
        windows_threading::submit(move || {
            let result = f();
            match result {
                Ok(data) => set2.call(MutationState::Success(data)),
                Err(e) => set2.call(MutationState::Error(e)),
            }
        });
    }

    /// Reset the mutation state back to Idle (e.g. after showing success).
    pub fn reset(&self) {
        self.set_state.call(MutationState::Idle);
    }
}

impl<T: Send + Clone + PartialEq + 'static> std::fmt::Debug for MutationTrigger<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MutationTrigger").finish_non_exhaustive()
    }
}
