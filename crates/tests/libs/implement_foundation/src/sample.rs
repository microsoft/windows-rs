//! Two parallel implementations of the same `Foo { x: u32 }` user type:
//!
//! * [`foundation_path`] — wired through the real
//!   `windows_core::imp::implement::*` foundation by hand. Mirrors the
//!   shape that `implement_decl!` (Step 3) and the reskinned
//!   `#[implement]` macro (Step 2) will emit.
//! * [`macro_path`] — wired through today's `#[implement(IValue)]` proc
//!   macro. The benchmark and layout tests compare the two side-by-side.
//!
//! Both share the same `IValue` trait and `Foo` field shape so any
//! observable difference comes from the wiring, not the user payload.

use windows_core::IUnknown;

// ============================================================================
// Hand-rolled fake IValue interface, shared by both paths.
// ============================================================================

#[windows_core::interface("01010101-0101-0101-0101-010101010101")]
pub unsafe trait IValue: IUnknown {
    pub fn get(&self) -> u32;
}

// One-line opt-in to `VtableCtor` for `IValue_Vtbl`. This is the line
// `windows-interface` / `windows-bindgen` will emit alongside each
// generated `_Vtbl` in Step 1b — for now, the user (or this test crate)
// emits it by hand.
//
// `IValue_Vtbl::new::<T, OFFSET>()` requires `T: IUnknownImpl + IValue_Impl`
// (see `crates/libs/interface/src/gen.rs`). We mirror that here.
impl<T, const OFFSET: isize> windows_core::imp::VtableCtor<T, OFFSET> for IValue_Vtbl
where
    T: windows_core::IUnknownImpl + IValue_Impl + 'static,
{
    const NEW: Self = <IValue_Vtbl>::new::<T, OFFSET>();
    const NEW_REF: &'static Self = &<Self as windows_core::imp::VtableCtor<T, OFFSET>>::NEW;
}

// ============================================================================
// Path A — `Foo` via the real `windows_core::imp::implement` foundation.
// ============================================================================
//
// Three small things the user (or future macro emission) writes per
// `#[implement(IValue)] Foo`:
//
//   1. `pub type Foo_Impl = Outer<Foo, (IValue,)>;`
//   2. `impl Implemented for Foo { type Interfaces = (IValue,); ... }`
//   3. The user-supplied `impl IValue_Impl for Foo_Impl { ... }`
//
// Plus per-declared-interface `From<Foo> for IValue` and
// `ComObjectInterface<IValue> for Foo_Impl` (see comments in
// `runtime.rs` for why these stay per-interface).

pub mod foundation_path {
    use super::{IValue, IValue_Impl};
    use windows_core::imp::{Agile, Implemented, Outer};
    use windows_core::{ComObject, ComObjectInterface, InterfaceRef};

    /// User-supplied state. Same shape as [`super::macro_path::Foo`] so
    /// layout comparisons isolate the wiring overhead.
    pub struct Foo {
        pub x: u32,
    }

    impl Implemented for Foo {
        type Interfaces = (IValue,);
        type Agility = Agile;
    }

    pub type Foo_Impl = Outer<Foo, (IValue,)>;

    // The user's interface-method body. `Foo_Impl` is a type alias for
    // `Outer<Foo, (IValue,)>`, and `Outer` derefs to `Foo`, so `self.x`
    // reaches the user field exactly the way today's macro arranges.
    impl IValue_Impl for Foo_Impl {
        unsafe fn get(&self) -> u32 {
            self.x
        }
    }

    // Per-declared-interface emissions. These cannot be expressed as a
    // generic blanket inside windows-core because of orphan rules — see
    // the rationale in `crates/libs/core/src/imp/implement/runtime.rs`.
    // The reskinned macro (Step 2) and `implement_decl!` (Step 3) emit
    // these per declared interface; here they're hand-written.

    impl ComObjectInterface<IValue> for Foo_Impl {
        #[inline(always)]
        fn as_interface_ref(&self) -> InterfaceRef<'_, IValue> {
            self.as_declared_interface()
        }
    }

    impl From<Foo> for IValue {
        #[inline(always)]
        fn from(this: Foo) -> Self {
            ComObject::new(this).into_interface()
        }
    }
}

// ============================================================================
// Path B — `Foo` via today's `#[implement(IValue)]` proc macro.
// ============================================================================

pub mod macro_path {
    use super::{IValue, IValue_Impl};
    use windows_core::implement;

    /// Same fields as [`super::foundation_path::Foo`].
    #[implement(IValue)]
    pub struct Foo {
        pub x: u32,
    }

    impl IValue_Impl for Foo_Impl {
        unsafe fn get(&self) -> u32 {
            self.x
        }
    }
}
