//! Generic call-site helpers used by `--middleware`-mode bindings.
//!
//! These helpers collapse the repeated `unsafe { let mut result__ = zeroed(); (vtable(self).M)(...).and_then(|| Type::from_abi(result__)) }`
//! shape that bindgen otherwise emits inline for every WinRT method. Each
//! generated `pub fn` becomes a single tail call into one of these helpers,
//! which shrinks both LLVM IR size and the borrow-checker workload (number of
//! `unsafe` blocks) without changing the public signature.
//!
//! The helpers are deliberately minimal: they cover only the shapes whose
//! generic bounds work cleanly. Shapes that don't fit (e.g. `MaybeUninit<HSTRING>`
//! `CloneType` returns, WinRT arrays, factory closures) fall back to the
//! pre-existing inline expansion.
//!
//! These are `#[doc(hidden)]` and considered an implementation detail of the
//! generated bindings. They live in `windows_core::imp` because bindgen-emitted
//! code can already address that path.
//!
//! # Safety
//!
//! Each helper is `unsafe` because the closure it accepts performs a raw
//! vtable call. Callers (i.e. bindgen-emitted bodies) must ensure:
//!
//! * `this` is a valid interface pointer of type `I`.
//! * The closure invokes the correct vtable slot with arguments matching its
//!   ABI signature, and stores into the supplied out-pointer on success.
//! * For [`call_in_out`], the out-pointer is left in a state from which
//!   `T::from_abi` can correctly reconstruct an owned `T` on success.

use crate::{Compose, Interface, Result, Type, HRESULT};
use core::ffi::c_void;

/// Helper for the canonical "one out-param, return `Result<T>`" call shape:
///
/// ```ignore
/// let mut result__ = core::mem::zeroed();
/// (Interface::vtable(self).M)(Interface::as_raw(self), …, &mut result__)
///     .and_then(|| Type::from_abi(result__))
/// ```
///
/// The closure receives the raw `this` pointer and the out-pointer slot;
/// callers are responsible for invoking the correct vtable slot with any
/// additional in-parameters.
///
/// Works for any `T: Type<T>` whose `Abi` type is `Default` — i.e. interface
/// types (`Abi = *mut c_void`) and copyable scalar / enum / `BOOL`-class types
/// (`Abi = T`). For `CloneType` returns whose `Abi = MaybeUninit<T>` callers
/// should fall back to the inline expansion (the helper would be unsound or
/// would need a per-shape variant).
///
/// # Safety
///
/// See module-level safety notes.
#[doc(hidden)]
#[inline]
pub unsafe fn call_in_out<I, T, F>(this: &I, f: F) -> Result<T>
where
    I: Interface,
    T: Type<T>,
    <T as Type<T>>::Abi: Default,
    F: FnOnce(*mut c_void, *mut <T as Type<T>>::Abi) -> HRESULT,
{
    let mut result__ = <<T as Type<T>>::Abi as Default>::default();
    f(Interface::as_raw(this), &mut result__).and_then(|| unsafe { T::from_abi(result__) })
}

/// Helper for the "no out-param, return `Result<()>`" call shape:
///
/// ```ignore
/// (Interface::vtable(self).M)(Interface::as_raw(self), …).ok()
/// ```
///
/// The closure receives the raw `this` pointer; callers are responsible for
/// invoking the correct vtable slot with any in-parameters.
///
/// # Safety
///
/// See module-level safety notes.
#[doc(hidden)]
#[inline]
pub unsafe fn call_in<I, F>(this: &I, f: F) -> Result<()>
where
    I: Interface,
    F: FnOnce(*mut c_void) -> HRESULT,
{
    f(Interface::as_raw(this)).ok()
}

/// Helper for the WinRT composable factory call shape.
///
/// Composable factory methods take an "outer" controlling [`IInspectable`]
/// (used to delegate `QueryInterface` back to the aggregating object) and an
/// out-pointer for the "inner" non-delegating `IInspectable`. After a
/// successful call, the inner is owned through the outer's `ComposeBase` slot
/// and the returned default-interface pointer's `IUnknown` methods delegate
/// back to the outer's controlling unknown.
///
/// The closure receives four raw pointers — `this__`, `derived__` (outer),
/// `base__` (writeback slot for the inner), and `result__` (out-pointer for
/// the aggregated default interface) — and is expected to invoke the correct
/// composable factory vtable slot with them, splicing in any additional
/// in-parameters before `derived__`.
///
/// The local `derived__: IInspectable` returned by [`Compose::compose`] is
/// kept alive across the vtable call (so the runtime can store a back-pointer
/// to the outer in the inner) and dropped only after `T::from_abi` has run on
/// success — its sole owning ref is replaced by the delegating ref baked into
/// `result__`. Dropping it earlier would be unsound.
///
/// # Safety
///
/// In addition to the module-level safety notes, the closure must invoke a
/// composable factory vtable slot whose ABI matches the
/// `(this, …, outer, &mut inner, &mut result)` shape and store the aggregated
/// default-interface pointer into `result__` on success.
#[doc(hidden)]
#[inline]
pub unsafe fn call_compose<I, T, P, F>(this: &I, compose: P, f: F) -> Result<T>
where
    I: Interface,
    T: Type<T>,
    <T as Type<T>>::Abi: Default,
    P: Compose,
    F: FnOnce(*mut c_void, *mut c_void, *mut *mut c_void, *mut <T as Type<T>>::Abi) -> HRESULT,
{
    let (derived__, base__) = unsafe { Compose::compose(compose) };
    let mut result__ = <<T as Type<T>>::Abi as Default>::default();
    f(
        Interface::as_raw(this),
        unsafe { core::mem::transmute_copy(&derived__) },
        base__ as *mut _ as _,
        &mut result__,
    )
    .ok()?;
    // Suppress unused-variable warning: `derived__` is held alive for the
    // duration of the factory call (so the factory can store a back-pointer
    // to the outer in the inner) and then dropped at scope end — its sole
    // owning ref is replaced by the delegating ref baked into `result__`.
    // Critical: this MUST happen *after* the success check and before
    // `from_abi` consumes `result__`, so the outer's lifetime spans the
    // entire vtable call. Dropping earlier is unsound.
    let _ = &derived__;
    unsafe { T::from_abi(result__) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{implement, IUnknown};

    // A trivial implementer to exercise the helpers without depending on the
    // OS. The helpers themselves are agnostic to the underlying COM impl.
    #[implement()]
    struct Object;

    #[test]
    fn call_in_returns_ok_on_success() {
        let object: IUnknown = Object.into();
        // Closure that returns S_OK and ignores the raw pointer.
        let result = unsafe { call_in(&object, |_| HRESULT(0)) };
        assert!(result.is_ok());
    }

    #[test]
    fn call_in_propagates_failure() {
        let object: IUnknown = Object.into();
        let result = unsafe { call_in(&object, |_| HRESULT(0x80004005u32 as i32)) };
        assert!(result.is_err());
    }

    #[test]
    fn call_in_out_copy_scalar() {
        // u32 is CopyType; Abi = u32; Default = 0.
        let object: IUnknown = Object.into();
        let result: Result<u32> = unsafe {
            call_in_out(&object, |_, out| {
                *out = 42;
                HRESULT(0)
            })
        };
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn call_in_out_propagates_failure() {
        let object: IUnknown = Object.into();
        let result: Result<u32> =
            unsafe { call_in_out(&object, |_, _| HRESULT(0x80004005u32 as i32)) };
        assert!(result.is_err());
    }

    #[test]
    fn call_in_out_interface_handles_null() {
        // For interface returns, Abi = *mut c_void; from_abi returns Err on null.
        let object: IUnknown = Object.into();
        let result: Result<IUnknown> = unsafe {
            call_in_out(&object, |_, _| {
                // Leave out as null (Default for *mut c_void).
                HRESULT(0)
            })
        };
        assert!(result.is_err());
    }

    // Refcount-tracking implementation type used by the `call_compose` tests
    // below. `Drop` decrements `ALIVE` so we can assert on live-object counts
    // across the helper's `compose` → vtable-call → `from_abi` sequence.
    static ALIVE: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

    #[implement()]
    struct CountedComposable;

    impl CountedComposable {
        fn new() -> Self {
            ALIVE.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
            CountedComposable
        }
    }

    impl Drop for CountedComposable {
        fn drop(&mut self) {
            ALIVE.fetch_sub(1, core::sync::atomic::Ordering::SeqCst);
        }
    }

    #[test]
    fn call_compose_observes_derived_alive_in_closure_and_drops_after_return() {
        // Reset; tests in `cargo test` may run in parallel but each test
        // creates its own `CountedComposable` and these atomics nest.
        let before = ALIVE.load(core::sync::atomic::Ordering::SeqCst);

        let counted = CountedComposable::new();
        assert_eq!(ALIVE.load(core::sync::atomic::Ordering::SeqCst), before + 1);

        let factory: IUnknown = Object.into();
        let result: Result<IUnknown> = unsafe {
            call_compose(
                &factory,
                counted,
                |_this, derived__, base__, _result_out| {
                    // While the closure runs, the heap-allocated outer is alive.
                    // `compose` moved `counted` into an `IInspectable`, which
                    // owns the only ref; `derived__` is a borrowed raw view of
                    // it via `transmute_copy`.
                    assert!(!derived__.is_null());
                    assert!(!base__.is_null());
                    assert_eq!(ALIVE.load(core::sync::atomic::Ordering::SeqCst), before + 1);
                    // Leave result__ null so the helper's `from_abi` fails on
                    // the success path; we still want the keep-alive guard to
                    // hold derived__ across the call. Use a successful HRESULT
                    // so we exercise the success path through `from_abi`.
                    HRESULT(0)
                },
            )
        };

        // `from_abi(null)` is the ultimate result.
        assert!(result.is_err());

        // After `call_compose` returns, the local derived__ has dropped, the
        // IInspectable's refcount has hit zero, and `CountedComposable::drop`
        // has run.
        assert_eq!(ALIVE.load(core::sync::atomic::Ordering::SeqCst), before);
    }

    #[test]
    fn call_compose_propagates_factory_failure() {
        let before = ALIVE.load(core::sync::atomic::Ordering::SeqCst);
        let counted = CountedComposable::new();
        let factory: IUnknown = Object.into();

        let result: Result<IUnknown> = unsafe {
            call_compose(&factory, counted, |_, _, _, _| {
                HRESULT(0x80004005u32 as i32)
            })
        };

        assert!(result.is_err());
        // Whether the helper bailed before or after `from_abi`, `derived__`
        // must have been dropped by the time we get here.
        assert_eq!(ALIVE.load(core::sync::atomic::Ordering::SeqCst), before);
    }
}
