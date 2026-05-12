//! `InterfaceList` (sealed) + `Implements<I>` membership marker.
//!
//! See `docs/option-d.md` ("`InterfaceList` and `Implements<I>`").
//!
//! Tuple impls are provided for arity 0..=4 in the spike (the doc commits to
//! arity 16 in production via OQ-1; 0..=4 is enough to hand a wide range of
//! real-world cases — the largest in-tree `#[implement]` use today is 3).
//!
//! ## Slot indexing
//!
//! Slot indices are pointer-units offsets from the **identity** field
//! (`&'static IInspectable_Vtbl`), not from the start of the struct. This
//! mirrors today's macro: identity sits at slot 0, the first declared
//! interface at slot 1, the next at slot 2, etc. The macro's `OFFSET = -1`
//! for identity and `-2 - k` for chain `k` is the negation of "slot index +
//! one" (the +1 covers the leading `base: ComposeBase` field).
//!
//! ## Storage tower construction
//!
//! Each tuple impl exposes a `STORAGE` associated const that builds the
//! `VCons` tower with each cell pre-populated by `<I::Vtable as
//! VtableCtor<T, OFFSET>>::NEW`. The tower is built in declaration order so
//! cell 0's `vtable` field sits at the same byte offset as today's
//! `Foo_Impl::interface1` field.

use crate::foundation::sealed::Sealed;
use crate::foundation::storage::{VCons, VNil};
use crate::foundation::vtbl::VtableCtor;
use windows_core::{Interface, GUID};

/// Sealed marker — see `docs/option-d.md` ("`InterfaceList` is `Sealed` so we
/// can extend it without breaking users").
pub trait InterfaceList: Sealed + 'static {
    /// Number of interface chains, excluding the identity (IInspectable) slot.
    const LEN: usize;

    /// Storage tower (a `VCons` / `VNil` chain).
    type Storage: 'static;

    /// `(IID, slot index)` pairs used by the generic `QueryInterface`. Slot 0
    /// is the identity (IInspectable); declared interfaces start at slot 1.
    const IID_SLOTS: &'static [(GUID, usize)];
}

/// Per-list, per-implementer storage construction.
///
/// Split from `InterfaceList` because building `Self::Storage` needs to know
/// `T` (it threads `T` into each cell's `VtableCtor<T, OFFSET>::NEW`), and we
/// don't want `T` to appear in `InterfaceList`'s signature.
///
/// In production (Step 1) this trait is what `Outer<T, L>::new` calls to
/// initialise `vtables`. In the spike, the layout-equality test only needs
/// the type-level `Storage` shape, so the runtime constructor lives behind a
/// generic associated const and is invoked from `Outer::vtables_for_layout`
/// in `outer.rs`.
pub trait ListVtables<T: 'static>: InterfaceList {
    const STORAGE: Self::Storage;
}

/// Membership marker — `L: Implements<I>` means "interface `I` appears
/// somewhere in list `L`". Returns the `SLOT` (pointer-units offset from
/// identity) where the matching vtable lives.
///
/// In Step 2 the blanket `From<T> for I where T::Interfaces: Implements<I>`
/// uses this trait. The spike does not rely on `From` blankets for the
/// layout proof but provides the trait so a downstream sanity test can be
/// written.
pub trait Implements<I: Interface>: InterfaceList {
    /// Pointer-units offset from `&self.identity` to the matching vtable
    /// field. 0 = identity (IInspectable / IUnknown), 1 = first declared,
    /// etc.
    const SLOT: usize;
}

// ============================================================================
// Tuple impls — arity 0..=4
// ============================================================================
//
// The doc commits to arity 16 in production (OQ-1). The spike covers 0..=4
// because that is enough to demonstrate (a) the empty case, (b) the
// single-interface case used by the layout proof, and (c) arity ≥3 so the
// `STORAGE` recursion is exercised on a non-trivial tower. Bumping the
// ceiling to 16 in production is a search-and-replace inside the macro that
// generates these impls in `windows-core`.

// ---- arity 0 -------------------------------------------------------------

impl Sealed for () {}
impl InterfaceList for () {
    const LEN: usize = 0;
    type Storage = VNil;
    const IID_SLOTS: &'static [(GUID, usize)] = &[];
}
impl<T: 'static> ListVtables<T> for () {
    const STORAGE: Self::Storage = VNil;
}

// ---- arity 1 -------------------------------------------------------------

impl<I0: Interface + 'static> Sealed for (I0,) {}
impl<I0: Interface + 'static> InterfaceList for (I0,) {
    const LEN: usize = 1;
    type Storage = VCons<I0, VNil>;
    const IID_SLOTS: &'static [(GUID, usize)] = &[(I0::IID, 1)];
}
impl<T: 'static, I0> ListVtables<T> for (I0,)
where
    I0: Interface + 'static,
    I0::Vtable: VtableCtor<T, -2>,
{
    const STORAGE: Self::Storage = VCons {
        vtable: <I0::Vtable as VtableCtor<T, -2>>::NEW_REF,
        rest: VNil,
    };
}
impl<I0: Interface + 'static> Implements<I0> for (I0,) {
    const SLOT: usize = 1;
}

// ---- arity 2 -------------------------------------------------------------

impl<I0: Interface + 'static, I1: Interface + 'static> Sealed for (I0, I1) {}
impl<I0: Interface + 'static, I1: Interface + 'static> InterfaceList for (I0, I1) {
    const LEN: usize = 2;
    type Storage = VCons<I0, VCons<I1, VNil>>;
    const IID_SLOTS: &'static [(GUID, usize)] = &[(I0::IID, 1), (I1::IID, 2)];
}
impl<T: 'static, I0, I1> ListVtables<T> for (I0, I1)
where
    I0: Interface + 'static,
    I1: Interface + 'static,
    I0::Vtable: VtableCtor<T, -2>,
    I1::Vtable: VtableCtor<T, -3>,
{
    const STORAGE: Self::Storage = VCons {
        vtable: <I0::Vtable as VtableCtor<T, -2>>::NEW_REF,
        rest: VCons {
            vtable: <I1::Vtable as VtableCtor<T, -3>>::NEW_REF,
            rest: VNil,
        },
    };
}
impl<I0: Interface + 'static, I1: Interface + 'static> Implements<I0> for (I0, I1) {
    const SLOT: usize = 1;
}
// NOTE: A second blanket `Implements<I1>` would overlap with the one above
// when `I0 == I1`. Production resolves this by emitting per-arity, per-position
// impls from a macro that knows `I0 != I1` is a constraint the user respects.
// The spike provides only the head-position blanket for arity ≥2; the layout
// proof does not need richer membership.

// ---- arity 3 -------------------------------------------------------------

impl<I0, I1, I2> Sealed for (I0, I1, I2)
where
    I0: Interface + 'static,
    I1: Interface + 'static,
    I2: Interface + 'static,
{
}
impl<I0, I1, I2> InterfaceList for (I0, I1, I2)
where
    I0: Interface + 'static,
    I1: Interface + 'static,
    I2: Interface + 'static,
{
    const LEN: usize = 3;
    type Storage = VCons<I0, VCons<I1, VCons<I2, VNil>>>;
    const IID_SLOTS: &'static [(GUID, usize)] =
        &[(I0::IID, 1), (I1::IID, 2), (I2::IID, 3)];
}
impl<T: 'static, I0, I1, I2> ListVtables<T> for (I0, I1, I2)
where
    I0: Interface + 'static,
    I1: Interface + 'static,
    I2: Interface + 'static,
    I0::Vtable: VtableCtor<T, -2>,
    I1::Vtable: VtableCtor<T, -3>,
    I2::Vtable: VtableCtor<T, -4>,
{
    const STORAGE: Self::Storage = VCons {
        vtable: <I0::Vtable as VtableCtor<T, -2>>::NEW_REF,
        rest: VCons {
            vtable: <I1::Vtable as VtableCtor<T, -3>>::NEW_REF,
            rest: VCons {
                vtable: <I2::Vtable as VtableCtor<T, -4>>::NEW_REF,
                rest: VNil,
            },
        },
    };
}

// ---- arity 4 -------------------------------------------------------------

impl<I0, I1, I2, I3> Sealed for (I0, I1, I2, I3)
where
    I0: Interface + 'static,
    I1: Interface + 'static,
    I2: Interface + 'static,
    I3: Interface + 'static,
{
}
impl<I0, I1, I2, I3> InterfaceList for (I0, I1, I2, I3)
where
    I0: Interface + 'static,
    I1: Interface + 'static,
    I2: Interface + 'static,
    I3: Interface + 'static,
{
    const LEN: usize = 4;
    type Storage = VCons<I0, VCons<I1, VCons<I2, VCons<I3, VNil>>>>;
    const IID_SLOTS: &'static [(GUID, usize)] = &[
        (I0::IID, 1),
        (I1::IID, 2),
        (I2::IID, 3),
        (I3::IID, 4),
    ];
}
impl<T: 'static, I0, I1, I2, I3> ListVtables<T> for (I0, I1, I2, I3)
where
    I0: Interface + 'static,
    I1: Interface + 'static,
    I2: Interface + 'static,
    I3: Interface + 'static,
    I0::Vtable: VtableCtor<T, -2>,
    I1::Vtable: VtableCtor<T, -3>,
    I2::Vtable: VtableCtor<T, -4>,
    I3::Vtable: VtableCtor<T, -5>,
{
    const STORAGE: Self::Storage = VCons {
        vtable: <I0::Vtable as VtableCtor<T, -2>>::NEW_REF,
        rest: VCons {
            vtable: <I1::Vtable as VtableCtor<T, -3>>::NEW_REF,
            rest: VCons {
                vtable: <I2::Vtable as VtableCtor<T, -4>>::NEW_REF,
                rest: VCons {
                    vtable: <I3::Vtable as VtableCtor<T, -5>>::NEW_REF,
                    rest: VNil,
                },
            },
        },
    };
}
