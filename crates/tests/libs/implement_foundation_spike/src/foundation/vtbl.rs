//! `VtableCtor<T, OFFSET>` — the one trait that an interface `_Vtbl` opts into
//! so that callers in generic contexts can construct a per-slot vtable
//! without naming the concrete `_Vtbl` type.
//!
//! See `docs/option-d.md` ("Per-slot vtable initialisers") and the resolution
//! of OQ-2.
//!
//! # Spike findings (these inform the doc)
//!
//! ## Finding 1 — `const fn` in trait position is unstable
//!
//! The doc draft used a `const fn new() -> Self` method on the trait. That is
//! gated behind nightly's `const_trait_impl`. The stable-Rust replacement is
//! an associated `const NEW: Self`:
//!
//! ```ignore
//! pub trait VtableCtor<T, const OFFSET: isize>: Sized {
//!     const NEW: Self;
//! }
//! ```
//!
//! ## Finding 2 — `&<V as VtableCtor<T, O>>::NEW` does not promote when `V`
//! is generic
//!
//! Stable rustc rejects `&<V as VtableCtor<T, O>>::NEW` at the use site
//! ("interior mutable shared borrows of temporaries that have their lifetime
//! extended until the end of the program are not allowed", `E0492`) when
//! `V` is a generic parameter. The reason: the compiler cannot prove `V` is
//! free of interior mutability for an arbitrary `V`, and constant promotion
//! is conservative.
//!
//! It works in two cases the spike actually relies on:
//!
//! * **Concrete `Self` inside the per-impl.** `&<Self as VtableCtor<T, O>>::NEW`
//!   inside `impl ... for IFoo_Vtbl { ... }` promotes, because rustc sees
//!   `Self == IFoo_Vtbl` and inspects its layout.
//! * **No generic `V` at the use site.** Reading an already-`&'static`-typed
//!   associated const works without re-borrow.
//!
//! Therefore the trait must carry both:
//!
//! ```ignore
//! pub trait VtableCtor<T, const OFFSET: isize>: Sized + 'static {
//!     const NEW: Self;
//!     const NEW_REF: &'static Self;
//! }
//! ```
//!
//! Each per-`_Vtbl` opt-in fills in *both*, with `NEW_REF` written as
//! `&<Self as VtableCtor<T, OFFSET>>::NEW`. The generic call site
//! (`storage` construction in `list.rs`) reads `NEW_REF` directly, never
//! `&NEW`. This is the **shape `windows-bindgen` emits in Step 2** — three
//! lines per `_Vtbl` instead of one.
//!
//! Both findings have been folded back into `docs/option-d.md`.

use windows_core::{IInspectable_Vtbl, IUnknown_Vtbl, IUnknownImpl};

/// The opt-in trait that each interface `_Vtbl` implements (one impl per
/// `_Vtbl`, emitted by `windows-interface` in Step 2).
///
/// `T` is the implementer (`Outer<T, L>` in production), `OFFSET` is the
/// pointer-units offset from the slot's vtable field back to the start of
/// the implementer struct (today `-1` for identity, `-2 - k` for chain `k`).
///
/// `'static` bound on `Self` is necessary so `&'static Self` is meaningful;
/// every concrete `_Vtbl` is `'static` anyway (just a struct of `fn` pointers).
pub trait VtableCtor<T, const OFFSET: isize>: Sized + 'static {
    /// The constructed vtable. Always `<Self>::new::<T, OFFSET>()` in
    /// practice — the indirection is what makes the call generic over `Self`.
    const NEW: Self;

    /// `&'static Self` reference to `NEW`. Required because in generic
    /// contexts `&<V as VtableCtor<T, O>>::NEW` does not promote (see the
    /// module-level "Finding 2"). Each per-`_Vtbl` impl writes this as
    /// `&<Self as VtableCtor<T, OFFSET>>::NEW` — `Self` is concrete inside
    /// the impl, so the borrow does promote there.
    const NEW_REF: &'static Self;
}

// ---- IUnknown_Vtbl opt-in ------------------------------------------------
//
// The existing inherent ctor is `IUnknown_Vtbl::new::<T: IUnknownImpl, OFFSET>()`.

impl<T: IUnknownImpl + 'static, const OFFSET: isize> VtableCtor<T, OFFSET> for IUnknown_Vtbl {
    const NEW: Self = <IUnknown_Vtbl>::new::<T, OFFSET>();
    const NEW_REF: &'static Self = &<Self as VtableCtor<T, OFFSET>>::NEW;
}

// ---- IInspectable_Vtbl opt-in -------------------------------------------
//
// IInspectable carries an extra `Name: RuntimeName` parameter that drives
// `IInspectable::GetRuntimeClassName`. See
// `docs/option-d.md` ("Note: `IInspectable_Vtbl::new` already takes an extra
// `FirstInterface` type parameter").
//
// In the spike the identity vtable always uses `IInspectable` itself as the
// runtime name, which mirrors how today's macro behaves when the implementer
// declares no WinRT interface (see `crates/libs/implement/src/gen.rs:142-146`).
//
// In the production design (Step 1+), `InterfaceList` exposes a
// `type FirstInterface: RuntimeName = IInspectable;` associated type that
// `Outer<T, L>` plumbs through to this impl. The spike pins it to
// `IInspectable` to keep the trait shape simple — this is sufficient to
// prove the layout claim, which is the spike's single load-bearing question.

impl<T, const OFFSET: isize> VtableCtor<T, OFFSET> for IInspectable_Vtbl
where
    T: IUnknownImpl + 'static,
{
    const NEW: Self = <IInspectable_Vtbl>::new::<T, windows_core::IInspectable, OFFSET>();
    const NEW_REF: &'static Self = &<Self as VtableCtor<T, OFFSET>>::NEW;
}

/// Type alias for backward compatibility with the doc's draft `HasVtblCtor`
/// name. The spike collapsed `HasVtblCtor` into `VtableCtor` (see Finding 1
/// in the module docstring).
pub use VtableCtor as HasVtblCtor;
