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

use windows_core::IUnknown;

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

#[windows_core::interface("01010101-0101-0101-0101-010101010101")]
pub unsafe trait IValue: IUnknown {
    pub fn get(&self) -> u32;
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
    use super::{IValue, IValue_Impl, IValue_Vtbl};
    use crate::foundation::list::Implements;
    use crate::foundation::outer::Outer;
    use crate::foundation::vtbl::VtableCtor;
    use crate::foundation::{Agile, Implemented};
    use core::ffi::c_void;
    use core::ptr::NonNull;
    use windows_core::{
        imp, ComObject, ComObjectInner, ComObjectInterface, IInspectable, IInspectable_Vtbl,
        IUnknown, InterfaceRef,
    };

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

    pub type Foo_Impl = Outer<Foo, (IValue,)>;

    // ----------------------------------------------------------------------
    // The user's interface-method body. Production: the user writes this
    // verbatim — `Foo_Impl` is a type alias for `Outer<Foo, (IValue,)>`, so
    // `impl IValue_Impl for Foo_Impl` resolves to an impl on the `Outer`.
    // `Deref to Foo` (from runtime.rs's blanket) gives us `self.x`.
    // ----------------------------------------------------------------------
    impl IValue_Impl for Foo_Impl {
        unsafe fn get(&self) -> u32 {
            self.x
        }
    }

    // ----------------------------------------------------------------------
    // Per-type emissions of the blanket impls that live in `windows-core` in
    // production but cannot be expressed in the spike crate because of
    // orphan rules (E0210). See `runtime.rs` for the rationale.
    //
    // These are exactly what `windows-bindgen` / `windows-implement` /
    // `implement_decl!` would emit per `#[implement(...)]` use site after
    // Step 2 (other than the fact that in production the same code lives
    // once, generically, in `windows-core`).
    // ----------------------------------------------------------------------

    impl ComObjectInner for Foo {
        type Outer = Foo_Impl;
        fn into_object(self) -> ComObject<Self> {
            let boxed = imp::Box::<Foo_Impl>::new(Outer::new_generic(self));
            unsafe {
                let ptr = imp::Box::into_raw(boxed);
                ComObject::from_raw(NonNull::new_unchecked(ptr))
            }
        }
    }

    // `ComObjectInterface<IValue>` for the declared interface (slot 1).
    impl ComObjectInterface<IValue> for Foo_Impl {
        #[inline(always)]
        fn as_interface_ref(&self) -> InterfaceRef<'_, IValue> {
            let base = &self.identity as *const _ as *const *const c_void;
            let slot = <(IValue,) as Implements<IValue>>::SLOT;
            unsafe { core::mem::transmute(base.add(slot)) }
        }
    }

    // `From<Foo>` conversions: identity (IUnknown, IInspectable) plus the
    // single declared interface.
    impl From<Foo> for IUnknown {
        #[inline(always)]
        fn from(this: Foo) -> Self {
            ComObject::new(this).into_interface()
        }
    }
    impl From<Foo> for IInspectable {
        #[inline(always)]
        fn from(this: Foo) -> Self {
            ComObject::new(this).into_interface()
        }
    }
    impl From<Foo> for IValue {
        #[inline(always)]
        fn from(this: Foo) -> Self {
            ComObject::new(this).into_interface()
        }
    }

    // ----------------------------------------------------------------------
    // Phase-2 sanity: silences the `unused_*` warnings while the spike is
    // under review. Real coverage lives in `tests/runtime.rs` and
    // `tests/bench_qi.rs`.
    // ----------------------------------------------------------------------
    #[doc(hidden)]
    pub fn _force_monomorphisation() {
        let _ = <IInspectable_Vtbl as VtableCtor<Foo_Impl, -1>>::NEW_REF;
        let _ = <IValue_Vtbl as VtableCtor<Foo_Impl, -2>>::NEW_REF;
    }
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
