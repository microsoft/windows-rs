use super::*;

/// A storage cell for the inner non-delegating `IInspectable` produced by COM aggregation
/// (composition).
///
/// Laid out as `#[repr(transparent)]` over `Option<IInspectable>` so callers can transmute
/// a `&mut ComposeBase` into a `&mut Option<IInspectable>`. The wrapper exists to satisfy
/// `Send`/`Sync` for objects that embed it as a field but never participate in aggregation.
///
/// # Safety contract
///
/// The slot is written exactly once, during the call to [`Compose::compose`], before the
/// outer object is shared. After that point it is only read (during `IUnknown::QueryInterface`
/// fall-through) through a shared reference. Any object using this slot must be agile or
/// otherwise guarantee that the inner `IInspectable` may be observed from multiple threads.
#[doc(hidden)]
#[repr(transparent)]
pub struct ComposeBase(Option<IInspectable>);

// SAFETY: see the type-level safety contract above.
unsafe impl Send for ComposeBase {}
// SAFETY: see the type-level safety contract above.
unsafe impl Sync for ComposeBase {}

impl ComposeBase {
    /// Constructs an empty (non-aggregated) base slot.
    #[doc(hidden)]
    pub const fn new() -> Self {
        Self(None)
    }

    /// Returns a shared reference to the underlying `Option<IInspectable>`.
    #[doc(hidden)]
    #[inline]
    pub fn as_option(&self) -> &Option<IInspectable> {
        &self.0
    }
}

/// A trait used to support aggregation (composition) of WinRT runtime classes.
///
/// Composable WinRT factory methods take an "outer" `IInspectable` (the controlling
/// unknown) and an out-pointer for the "inner" non-delegating `IInspectable`. When a Rust
/// implementation type derives from a composable WinRT class it must hand the runtime an
/// outer `IInspectable` and provide a slot for the runtime to write back the inner.
///
/// The `windows-implement` proc macro emits an implementation of this trait for every type
/// marked with `#[implement(...)]`.
#[doc(hidden)]
pub trait Compose: Sized {
    /// Returns the outer `IInspectable` for `implementation` together with a mutable
    /// reference to the slot where the runtime should write back the inner non-delegating
    /// `IInspectable`.
    ///
    /// # Safety
    ///
    /// The returned mutable reference points into the heap-allocated outer implementation
    /// object that backs the returned `IInspectable`. Callers must keep the `IInspectable`
    /// alive (so the slot remains valid) for the duration of the composable factory call
    /// that consumes it, and must ensure `'a` does not outlive the returned `IInspectable`.
    unsafe fn compose<'a>(implementation: Self) -> (IInspectable, &'a mut Option<IInspectable>);
}
