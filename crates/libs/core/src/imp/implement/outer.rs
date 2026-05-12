//! `Outer<T, L>` and the `Implemented` user trait — see `docs/option-d.md`
//! ("`Outer<T, L>` and `Implemented`").
//!
//! ## Layout
//!
//! `Outer<T, L>` is `#[repr(C)]` with fields in this order, byte-identical
//! to today's `#[implement]`-emitted `Foo_Impl` (see
//! `crates/libs/implement/src/gen.rs:60-97`):
//!
//! 1. `base: ComposeBase`                         (1 pointer; niche-optimised)
//! 2. `identity: &'static IInspectable_Vtbl`      (1 pointer)
//! 3. `vtables: L::Storage`                       (`L::LEN` pointers)
//! 4. `this: T`                                   (T's layout)
//! 5. `count: imp::WeakRefCount`                  (1 `AtomicIsize`)
//!
//! Field order is load-bearing — do **not** reorder. `repr(C)` pins offsets;
//! reordering would break (a) the `OFFSET = -1` / `-2 - k` thunk arithmetic
//! baked into every `_Vtbl::new::<T, OFFSET>()` call, and (b) every existing
//! `#[implement]` user that observes the layout via `as_impl` / `as_interface`
//! casts.

use super::agility::Agility;
use super::list::InterfaceList;
use crate::imp::WeakRefCount;
use crate::{ComposeBase, IInspectable_Vtbl};

/// What the user (or the macro) declares about a type.
///
/// In Step 2 the proc macro will emit one of these per `#[implement(...)]`
/// use site; `implement_decl!` (Step 3) does the same; the
/// zero-procedural-macro path writes it by hand.
pub trait Implemented: Sized + 'static {
    /// The interfaces exposed via `QueryInterface`.
    type Interfaces: InterfaceList;

    /// Agile / non-agile selection. Today's macro defaults to `Agile`;
    /// hand-written users specify it explicitly because associated-type
    /// defaults are unstable.
    type Agility: Agility;

    /// Trust level returned by `IInspectable::GetTrustLevel`. 0 = Base
    /// trust (today's default).
    const TRUST: u8 = 0;

    /// Whether `DYNAMIC_CAST_IID` is honoured. The macro forces this to
    /// `false` for types containing non-`'static` lifetimes (preserving
    /// today's behaviour).
    const DYNAMIC_CAST: bool = true;
}

/// The single shape that backs every `#[implement]` type. See module
/// docstring for the layout claim.
#[repr(C)]
pub struct Outer<T, L = <T as Implemented>::Interfaces>
where
    T: Implemented<Interfaces = L>,
    L: InterfaceList,
{
    /// Holds the inner non-delegating `IInspectable` when this type
    /// aggregates a composable WinRT class. Stays `None` otherwise.
    /// `repr(transparent)` over `Option<IInspectable>`.
    pub(super) base: ComposeBase,
    /// Identity vtable (`IInspectable`). Stored as `&'static IInspectable_Vtbl`
    /// for byte-identical layout with today's macro.
    pub(super) identity: &'static IInspectable_Vtbl,
    /// HList tower of per-chain vtable pointers. `repr(C)` `VCons` cells
    /// fold to a flat sequence of pointers, mirroring today's per-chain
    /// `&'static IFoo_Vtbl` fields.
    pub(super) vtables: L::Storage,
    /// User-supplied data. `T`'s layout is preserved verbatim.
    pub(super) this: T,
    /// Reference count.
    pub(super) count: WeakRefCount,
}
