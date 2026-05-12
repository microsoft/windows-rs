//! Byte-identical layout proof: `Outer<Foo, (IValue,)>` (foundation) vs
//! today's `#[implement(IValue)] Foo`'s `Foo_Impl` (proc macro).
//!
//! Both share the same `Foo { x: u32 }` payload, the same `IValue` trait,
//! the same `repr(C)` field set, and the same field types in the same
//! declaration order — so matching `size_of` / `align_of` is sufficient
//! evidence of byte-identical layout. Per-field `offset_of!` checks would
//! also work but require reaching into both sides' private fields.

use static_assertions::{assert_eq_align, assert_eq_size};
use test_implement_foundation::sample::foundation_path::Foo as FoundationFoo;
use test_implement_foundation::sample::macro_path::Foo_Impl as MacroFooImpl;
use test_implement_foundation::sample::IValue;
use windows_core::imp::Outer;

assert_eq_size!(Outer<FoundationFoo, (IValue,)>, MacroFooImpl);
assert_eq_align!(Outer<FoundationFoo, (IValue,)>, MacroFooImpl);

// ---- HList tower folds to a flat sequence of pointers --------------------
//
// Central layout claim of `docs/option-d.md`. A regression in rustc's
// `repr(C)` layout for nested ZST tails would be caught here.

#[test]
fn vcons_tower_folds_to_flat_pointer_sequence() {
    use core::mem::size_of;
    use windows_core::imp::{VCons, VNil};
    use windows_core::IUnknown;

    let p = size_of::<usize>();

    assert_eq!(size_of::<VNil>(), 0);
    assert_eq!(size_of::<VCons<IUnknown, VNil>>(), p);
    assert_eq!(size_of::<VCons<IUnknown, VCons<IUnknown, VNil>>>(), 2 * p);
    assert_eq!(
        size_of::<VCons<IUnknown, VCons<IUnknown, VCons<IUnknown, VNil>>>>(),
        3 * p
    );
    assert_eq!(
        size_of::<VCons<IUnknown, VCons<IUnknown, VCons<IUnknown, VCons<IUnknown, VNil>>>>>(),
        4 * p
    );
}

#[test]
fn weak_ref_count_is_pointer_sized() {
    use windows_core::imp::WeakRefCount;
    assert_eq!(
        core::mem::size_of::<WeakRefCount>(),
        core::mem::size_of::<usize>()
    );
}
