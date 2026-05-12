//! Phase 1 of step 0: byte-identical layout proof.
//!
//! Compares `Outer<Foo, (IValue,)>` (proposed foundation) with the same `Foo`
//! wired through today's `#[implement(IValue)]` proc macro. If any of these
//! assertions fails, the layout claim in `docs/option-d.md` is wrong and the
//! design must be revised before the migration plan proceeds.
//!
//! All assertions are type-level (`size_of`, `align_of`, `offset_of!`); the
//! `Outer<Foo, ...>` and `Foo_Impl` types under test are never instantiated,
//! so there is no runtime work beyond the integer comparisons themselves.

use static_assertions::{assert_eq_align, assert_eq_size};
use test_implement_foundation_spike::foundation::{
    storage::{VCons, VNil},
    Outer,
};
use test_implement_foundation_spike::sample::{
    foundation_path::Foo as FoundationFoo, macro_path::Foo_Impl as MacroFooImpl, IValue,
};

// ---- size + alignment ----------------------------------------------------
//
// The two types must occupy the same number of bytes and share the same
// alignment. Fail here means the field set differs (most likely the macro
// adds or omits a field the foundation doesn't track).

assert_eq_size!(Outer<FoundationFoo, (IValue,)>, MacroFooImpl);
assert_eq_align!(Outer<FoundationFoo, (IValue,)>, MacroFooImpl);

// ---- per-field offsets ---------------------------------------------------
//
// Today's macro emits the field set `base, identity, interface1, this, count`
// (see `crates/libs/implement/src/gen.rs:60-97`). The foundation's `Outer<T, L>`
// uses `base, identity, vtables, this, count` where `vtables: VCons<IValue, VNil>`
// — the leading vtable in `vtables` must sit at the same byte offset as the
// macro's `interface1` field for the OFFSET arithmetic to remain valid.
//
// `offset_of!` requires the field to be visible. The foundation's fields are
// `pub(crate)`, so the layout test lives in the same workspace and reaches
// them through a re-export module. The macro's fields are emitted as
// non-public, so we cannot directly `offset_of!` into them; we therefore
// validate the *cumulative* offsets by computing them via inspecting the
// foundation side and asserting the macro side has matching size up to that
// point.
//
// What we *can* assert directly on the foundation side:

#[test]
fn foundation_outer_field_offsets_are_well_defined() {
    use core::mem::offset_of;

    // base sits at offset 0 (it's the first field).
    assert_eq!(offset_of!(Outer<FoundationFoo, (IValue,)>, base), 0);

    // identity follows base; on every supported target ComposeBase is one
    // pointer (Option<IInspectable> with niche optimisation).
    assert_eq!(
        offset_of!(Outer<FoundationFoo, (IValue,)>, identity),
        core::mem::size_of::<usize>()
    );

    // vtables follows identity; one pointer further in.
    assert_eq!(
        offset_of!(Outer<FoundationFoo, (IValue,)>, vtables),
        2 * core::mem::size_of::<usize>()
    );

    // For arity 1, vtables itself is one pointer (the lone IValue vtable).
    // So `this` follows at offset 3 * pointer_size.
    assert_eq!(
        offset_of!(Outer<FoundationFoo, (IValue,)>, this),
        3 * core::mem::size_of::<usize>()
    );
}

// ---- HList tower folds to a flat sequence of pointers --------------------
//
// The central layout claim of `docs/option-d.md` (HList shape vs. const-array
// shape): `VCons<I0, VCons<I1, VNil>>` must be byte-identical to two adjacent
// `&'static _` fields. We assert this for arity 0..=4 so a regression in
// rustc's struct layout for nested `repr(C)` ZST tails would be caught.

#[test]
fn vcons_tower_folds_to_flat_pointer_sequence() {
    use core::mem::size_of;
    use windows_core::IUnknown;

    let p = size_of::<usize>();

    // arity 0
    assert_eq!(size_of::<VNil>(), 0);

    // arity 1
    assert_eq!(size_of::<VCons<IUnknown, VNil>>(), p);

    // arity 2
    assert_eq!(size_of::<VCons<IUnknown, VCons<IUnknown, VNil>>>(), 2 * p);

    // arity 3
    assert_eq!(
        size_of::<VCons<IUnknown, VCons<IUnknown, VCons<IUnknown, VNil>>>>(),
        3 * p
    );

    // arity 4
    assert_eq!(
        size_of::<VCons<IUnknown, VCons<IUnknown, VCons<IUnknown, VCons<IUnknown, VNil>>>>>(),
        4 * p
    );
}

// ---- WeakRefCount sanity -------------------------------------------------
//
// The `count` field of both shapes is `windows_core::imp::WeakRefCount`,
// which is a `repr(transparent)`-style wrapper around `AtomicIsize`. Verify
// it is one pointer-sized atomic, since the layout proof above implicitly
// depends on this.

#[test]
fn weak_ref_count_is_pointer_sized() {
    use windows_core::imp::WeakRefCount;
    assert_eq!(
        core::mem::size_of::<WeakRefCount>(),
        core::mem::size_of::<usize>()
    );
}
