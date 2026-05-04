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

use crate::{HRESULT, Interface, Result, Type};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{IUnknown, implement};

    // A trivial implementer to exercise the helpers without depending on the
    // OS. The helpers themselves are agnostic to the underlying COM impl.
    #[implement()]
    struct Object;

    #[test]
    fn call_in_returns_ok_on_success() {
        let object: IUnknown = Object.into();
        // Closure that returns S_OK and ignores the raw pointer.
        let result =
            unsafe { call_in(&object, |_this| HRESULT(0)) };
        assert!(result.is_ok());
    }

    #[test]
    fn call_in_propagates_failure() {
        let object: IUnknown = Object.into();
        let result =
            unsafe { call_in(&object, |_this| HRESULT(0x80004005u32 as i32)) };
        assert!(result.is_err());
    }

    #[test]
    fn call_in_out_copy_scalar() {
        // u32 is CopyType; Abi = u32; Default = 0.
        let object: IUnknown = Object.into();
        let result: Result<u32> = unsafe {
            call_in_out(&object, |_this, out| {
                *out = 42;
                HRESULT(0)
            })
        };
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn call_in_out_propagates_failure() {
        let object: IUnknown = Object.into();
        let result: Result<u32> = unsafe {
            call_in_out(&object, |_this, _out| HRESULT(0x80004005u32 as i32))
        };
        assert!(result.is_err());
    }

    #[test]
    fn call_in_out_interface_handles_null() {
        // For interface returns, Abi = *mut c_void; from_abi returns Err on null.
        let object: IUnknown = Object.into();
        let result: Result<IUnknown> = unsafe {
            call_in_out(&object, |_this, _out| {
                // Leave out as null (Default for *mut c_void).
                HRESULT(0)
            })
        };
        assert!(result.is_err());
    }
}
