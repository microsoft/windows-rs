//! Async data primitives: [`Resource`], [`use_resource`], and [`use_mutation`].
//!
//! These compose over the lower-level [`use_async_state`] and [`use_effect`]
//! hooks to provide a batteries-included pattern for loading remote data
//! and performing async writes.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use super::render_context::{Deps, RenderCx};

// ─── Resource enum ──────────────────────────────────────────────────────

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
    pub fn view<F, R>(&self, ready: F) -> ResourceView<'_, T, impl FnOnce(&T) -> super::element::Element>
    where
        F: FnOnce(&T) -> R,
        R: Into<super::element::Element>,
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
/// to override defaults before converting to [`Element`](crate::Element) via `.into()`.
pub struct ResourceView<'a, T, F>
where
    F: FnOnce(&T) -> super::element::Element,
{
    resource: &'a Resource<T>,
    ready: F,
    loading: Option<super::element::Element>,
    error: Option<Box<dyn FnOnce(&str) -> super::element::Element>>,
}

impl<'a, T, F> ResourceView<'a, T, F>
where
    F: FnOnce(&T) -> super::element::Element,
{
    /// Override the loading state element (default: indeterminate ProgressRing).
    pub fn loading(mut self, el: impl Into<super::element::Element>) -> Self {
        self.loading = Some(el.into());
        self
    }

    /// Override the error state renderer (default: text block showing the error).
    pub fn error<E>(mut self, f: E) -> Self
    where
        E: FnOnce(&str) -> super::element::Element + 'static,
    {
        self.error = Some(Box::new(f));
        self
    }
}

impl<T, F> From<ResourceView<'_, T, F>> for super::element::Element
where
    F: FnOnce(&T) -> super::element::Element,
{
    fn from(rv: ResourceView<'_, T, F>) -> Self {
        use super::element::ProgressRing;
        match rv.resource {
            Resource::Loading | Resource::Idle => rv
                .loading
                .unwrap_or_else(|| ProgressRing::indeterminate().into()),
            Resource::Ready(data) | Resource::Reloading(data) => (rv.ready)(data),
            Resource::Error(e) => rv.error.map_or_else(
                || super::element::text_block(format!("Error: {e}")).into(),
                |f| f(e.as_str()),
            ),
        }
    }
}

// ─── Mutation state ─────────────────────────────────────────────────────

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

// ─── Generation token for cancellation ──────────────────────────────────

/// Monotonic counter used to discard results from stale fetches.
/// When deps change, the generation increments; results stamped with
/// an older generation are silently dropped.
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

// ─── use_resource ───────────────────────────────────────────────────────

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
        F: Fn(D) -> Result<T, String> + Send + 'static,
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

// ─── MutationTrigger ────────────────────────────────────────────────────

/// Handle returned by [`RenderCx::use_mutation`] to fire async write
/// operations from event handlers.
#[derive(Clone)]
pub struct MutationTrigger<T: Send + Clone + PartialEq + 'static> {
    set_state: super::render_context::AsyncSetState<MutationState<T>>,
}

impl<T: Send + Clone + PartialEq + 'static> MutationTrigger<T> {
    /// Fire a mutation on a background thread.
    pub fn fire<F>(&self, f: F)
    where
        F: FnOnce() -> Result<T, String> + Send + 'static,
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
