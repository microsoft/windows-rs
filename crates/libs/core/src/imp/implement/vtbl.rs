//! `VtableCtor<T, OFFSET>` — the opt-in trait each interface `_Vtbl`
//! implements so generic code can construct a per-slot vtable without naming
//! the concrete `_Vtbl` type. See `docs/option-d.md` ("Per-slot vtable
//! initialisers" + OQ-2).
//!
//! ## Stable-Rust constraints (validated by the Step 0 spike)
//!
//! 1. **`const fn` in trait position is unstable** (gated behind nightly's
//!    `const_trait_impl`). The trait carries an associated `const NEW: Self`
//!    instead.
//! 2. **`&<V as VtableCtor<T, O>>::NEW` does not promote when `V` is generic**
//!    (`E0492` — interior mutability not provable). The trait must therefore
//!    expose both `NEW: Self` and `NEW_REF: &'static Self`. Each per-`_Vtbl`
//!    impl writes `NEW_REF` as `&<Self as VtableCtor<T, O>>::NEW` where
//!    `Self` is concrete, so the borrow does promote. Generic call sites
//!    read `NEW_REF` directly.
//!
//! `windows-core` itself emits the `IUnknown_Vtbl` and `IInspectable_Vtbl`
//! opt-ins below. Per-`_Vtbl` opt-ins for downstream interfaces are emitted
//! by `windows-interface` / `windows-bindgen` (Step 1b in the migration
//! plan); until then, hand-written users add the three-line impl themselves.

use crate::{IInspectable, IInspectable_Vtbl, IUnknownImpl, IUnknown_Vtbl};

/// The opt-in trait that each interface `_Vtbl` implements.
///
/// `T` is the implementer (`Outer<T, L>` in production), `OFFSET` is the
/// pointer-units offset from the slot's vtable field back to the start of
/// the implementer struct (`-1` for identity, `-2 - k` for chain `k`).
pub trait VtableCtor<T, const OFFSET: isize>: Sized + 'static {
    /// The constructed vtable.
    const NEW: Self;

    /// `&'static Self` reference to `NEW`. Required because in generic
    /// contexts `&<V as VtableCtor<T, O>>::NEW` does not promote (Finding 2
    /// in the module docstring).
    const NEW_REF: &'static Self;
}

// ---- IUnknown_Vtbl opt-in -----------------------------------------------

impl<T: IUnknownImpl + 'static, const OFFSET: isize> VtableCtor<T, OFFSET> for IUnknown_Vtbl {
    const NEW: Self = <IUnknown_Vtbl>::new::<T, OFFSET>();
    const NEW_REF: &'static Self = &<Self as VtableCtor<T, OFFSET>>::NEW;
}

// ---- IInspectable_Vtbl opt-in -------------------------------------------
//
// `IInspectable_Vtbl::new` carries an extra `Name: RuntimeName` parameter
// driving `IInspectable::GetRuntimeClassName`. This impl pins `Name` to
// `IInspectable` itself, mirroring how today's macro behaves when the
// implementer declares no WinRT runtime class (see
// `crates/libs/implement/src/gen.rs`'s identity vtable emission).

impl<T: IUnknownImpl + 'static, const OFFSET: isize> VtableCtor<T, OFFSET> for IInspectable_Vtbl {
    const NEW: Self = <IInspectable_Vtbl>::new::<T, IInspectable, OFFSET>();
    const NEW_REF: &'static Self = &<Self as VtableCtor<T, OFFSET>>::NEW;
}
