//! The two parallel implementations the layout proof compares.
//!
//! * [`foundation_path::Foo`] — wired up using the proposed `Outer<T, L>`
//!   foundation directly, with a hand-written `impl Implemented for Foo`.
//! * [`macro_path`] — the same `Foo` type wired up via today's
//!   `#[implement(IValue)]` proc macro.
//!
//! `tests/layout.rs` then asserts:
//!
//! * `size_of::<Outer<Foo, (IValue,)>>() == size_of::<MacroFoo_Impl>()`
//! * `align_of`            equal
//! * Per-field offsets via `offset_of!` (where reachable) — `base`,
//!   `identity`, `vtables`'s leading vtable, `this`, `count`.
//!
//! The two paths use a *single* `IValue` interface declaration so the
//! comparison is apples-to-apples. See [`foundation_path::Foo`] and
//! [`macro_path`] below.

use windows_core::{interface, IUnknown};

// ============================================================================
// Hand-rolled fake IValue interface, shared by both paths.
// ============================================================================
//
// The interface is declared with the standard `#[interface(...)]` proc macro
// from `windows-interface`. Today's `#[interface]` already generates the
// `IValue_Vtbl` struct with an inherent `pub const fn new<Identity, OFFSET>()`
// (see `crates/libs/interface/src/gen.rs:241-250`). The spike tests that the
// `VtableCtor<T, OFFSET>` opt-in (defined in `super::foundation::vtbl`) can
// be added for `IValue_Vtbl` by hand without changing `windows-interface`.

#[interface("01010101-0101-0101-0101-010101010101")]
pub unsafe trait IValue: IUnknown {
    fn get(&self) -> u32;
}

// One-line opt-in to `VtableCtor` for `IValue_Vtbl`. This is the line
// `windows-bindgen` will emit alongside `IValue_Vtbl` in Step 2 (one such
// line per generated interface). Per the spike finding documented in
// `super::foundation::vtbl`, the trait body is an associated `const NEW`,
// not a `const fn` — `const fn` in trait position is gated behind nightly
// `const_trait_impl`.
//
// `IValue_Vtbl::new::<T, OFFSET>()` requires `T: IUnknownImpl + IValue_Impl`
// (see `crates/libs/interface/src/gen.rs:246`). Production carries both
// bounds; the spike reflects them here.
impl<T, const OFFSET: isize> crate::foundation::VtableCtor<T, OFFSET> for IValue_Vtbl
where
    T: windows_core::IUnknownImpl + IValue_Impl + 'static,
{
    const NEW: Self = <IValue_Vtbl>::new::<T, OFFSET>();
    const NEW_REF: &'static Self =
        &<Self as crate::foundation::VtableCtor<T, OFFSET>>::NEW;
}

// ============================================================================
// Path A — `Foo` via the proposed foundation.
// ============================================================================

pub mod foundation_path {
    use super::IValue;
    use crate::foundation::{Agile, Implemented};

    /// User-supplied state. The exact same struct is reused by `macro_path`
    /// below (see [`super::macro_path::Foo`]) so that any size/align
    /// mismatch can only come from the surrounding wiring, not from `T`'s
    /// own layout.
    pub struct Foo {
        pub x: u32,
    }

    impl Implemented for Foo {
        type Interfaces = (IValue,);
        type Agility = Agile;
    }

    // No `IValue_Impl for Outer<Foo, (IValue,)>` here yet — that's wired up
    // in phase 2, when `Outer` learns `IUnknownImpl`. Phase 1 only proves
    // the layout claim and the `Implemented` declaration itself compiles.
}

// ============================================================================
// Path B — `Foo` via today's `#[implement]` proc macro.
// ============================================================================

pub mod macro_path {
    use super::{IValue, IValue_Impl};
    use windows_core::implement;

    /// Same fields as [`super::foundation_path::Foo`] so the comparison
    /// isolates the wiring overhead from the user payload.
    #[implement(IValue)]
    pub struct Foo {
        pub x: u32,
    }

    // Trait-method body is irrelevant to layout; we provide a stub so the
    // macro emission compiles. The layout proof never calls into it.
    impl IValue_Impl for Foo_Impl {
        unsafe fn get(&self) -> u32 {
            self.x
        }
    }
}
