//! `Outer<T, L>` and the `Implemented` user trait.
//!
//! See `docs/option-d.md` ("`Outer<T, L>` and `Implemented`").
//!
//! ## Layout claim
//!
//! `Outer<T, L>` is `#[repr(C)]` with fields in this order, byte-identical to
//! today's `#[implement]`-emitted `Foo_Impl` (see
//! `crates/libs/implement/src/gen.rs:60-97`):
//!
//! 1. `base: ComposeBase`                         (1 pointer; niche-optimised)
//! 2. `identity: &'static IInspectable_Vtbl`      (1 pointer)
//! 3. `vtables: L::Storage`                       (`L::LEN` pointers)
//! 4. `this: T`                                   (T's layout)
//! 5. `count: imp::WeakRefCount`                  (1 `AtomicIsize`)
//!
//! `tests/layout.rs` asserts this byte-for-byte against today's macro output.
//!
//! ## Phase 1 vs phase 2 of step 0
//!
//! This file (phase 1) establishes the type and the `Implemented` trait. It
//! deliberately does **not** implement `IUnknownImpl`, `Compose`,
//! `ComObjectInner`, `Deref`, the blanket `From<T> for I`, or
//! `ComObjectInterface<I>` — those are phase 2.
//!
//! Phase 1 establishes a load-bearing fact: the design **type-checks** on
//! stable Rust. The layout assertions in `tests/layout.rs` are type-level
//! (`core::mem::size_of::<Outer<...>>()` and `core::mem::offset_of!`), so
//! they don't require any value to be constructed. That keeps phase 1
//! focused on "does the design even compile?" — which is the question
//! `windows-core` maintainers cannot answer without code.
//!
//! Phase 2 wires `IUnknownImpl` and provides the real `Outer::new` ctor
//! (the OQ-6 `new` / `new_generic` split).

use crate::foundation::agility::Agility;
use crate::foundation::list::InterfaceList;
use windows_core::{imp::WeakRefCount, ComposeBase, IInspectable_Vtbl};

/// What the user (or the macro) declares about a type.
///
/// In Step 2 the proc macro emits one of these per `#[implement(...)]` use
/// site; in Step 3 a `macro_rules!` shim does the same; in the
/// zero-procedural-macro path the user writes it by hand (see
/// `crate::sample::Foo` for an example).
pub trait Implemented: Sized + 'static {
    /// The interfaces exposed via `QueryInterface`.
    type Interfaces: InterfaceList;

    /// Agile / non-agile selection. Today's macro defaults to `Agile`; users
    /// of the foundation specify it explicitly because associated-type
    /// defaults are unstable on stable Rust.
    type Agility: Agility;

    /// Trust level returned by `IInspectable::GetTrustLevel`.
    /// 0 = Base trust (today's default).
    const TRUST: u8 = 0;

    /// Whether `DYNAMIC_CAST_IID` is honoured. The Step 2 macro forces this
    /// to `false` for types containing non-`'static` lifetimes (a constraint
    /// preserved from today's behaviour). Defaults to `true`.
    const DYNAMIC_CAST: bool = true;
}

/// The single shape that backs every `#[implement]` type. See module docstring
/// for the byte-identical layout claim against today's `Foo_Impl`.
///
/// Field order is load-bearing — do **not** reorder. The `repr(C)` attribute
/// pins offsets; reordering would break (a) the `OFFSET = -1` / `-2 - k`
/// thunk arithmetic baked into every `_Vtbl::new::<T, OFFSET>()` call and
/// (b) every existing `#[implement]` user that observes the layout via
/// `as_impl` / `as_interface` casts.
#[repr(C)]
pub struct Outer<T, L>
where
    T: Implemented<Interfaces = L>,
    L: InterfaceList,
{
    /// Holds the inner non-delegating `IInspectable` when this type aggregates
    /// a composable WinRT class. Stays `None` otherwise. `repr(transparent)`
    /// over `Option<IInspectable>`.
    ///
    /// `pub` so `offset_of!` works from `tests/layout.rs`. In production the
    /// field would be `pub(crate)` of `windows-core` — same effective visibility
    /// because `windows-core` would own both the type and the macro that uses it.
    pub base: ComposeBase,
    /// Identity vtable (`IInspectable`). Today's macro stores this as
    /// `&'static IInspectable_Vtbl`; we do the same so layout matches.
    pub identity: &'static IInspectable_Vtbl,
    /// HList tower of per-chain vtable pointers. The `repr(C)` `VCons` cells
    /// fold down to a flat sequence of pointers, exactly mirroring today's
    /// per-chain `&'static IFoo_Vtbl` fields.
    pub vtables: L::Storage,
    /// User-supplied data. `T`'s layout is preserved verbatim.
    pub this: T,
    /// Reference count. `WeakRefCount` is one `AtomicIsize` (8 bytes on
    /// 64-bit, 4 on 32-bit), matching today's `Foo_Impl::count`.
    pub count: WeakRefCount,
}
