# Option D — A library-based foundation for `#[implement]`

> **Status:** design draft for review. The seven open questions raised in the
> first revision have been resolved (see [Resolved decisions](#resolved-decisions)
> below). No code under `crates/` has been changed yet — the next concrete
> action is the Step 0 spike. The doc is written so that anyone reviewing it
> can see, for each load-bearing choice, both *what* was picked and *why* the
> alternative was rejected.

## Summary

Today the `#[implement]` proc macro in `windows-implement` emits roughly 650
lines of generated tokens per use site: a `#[repr(C)]` outer struct, a vtable
const per interface, an `IUnknownImpl` impl containing a hand-rolled
`QueryInterface`, a `ComObjectInner` impl, a `Compose` impl, a `Deref` impl,
plus per-interface `From`, `ComObjectInterface<I>`, and `AsImpl` impls. Every
fact about the COM ABI — the field order, the `OFFSET = -2 - k` thunk trick,
the `DYNAMIC_CAST_IID` carve-out, the `IMarshal` `#[cfg(windows)]` gate,
aggregation forwarding — lives inside that token surgery.

Option D moves that knowledge into `windows-core` as ordinary generic code and
leaves the macro responsible only for **picking names** (the `Foo_Impl` alias)
and **declaring user intent** (which interfaces, which knobs). The proc macro
shrinks to a small attribute parser plus three trait/alias emissions. A
`macro_rules!` sibling (`implement_decl!`) and hand-written use become trivial
because the heavy lifting now happens through trait resolution, not token
expansion.

The user-facing surface — `#[implement(IFoo, IBar)] struct Foo;`, the
`Foo_Impl` type name, `ComObject<Foo>`, `AsImpl<Foo>`, `From<Foo> for IFoo` —
is preserved. The COM ABI on the wire is byte-for-byte identical to the
current macro output.

## Goals and non-goals

### Goals

1. Single source of truth for the COM-object ABI inside `windows-core`,
   reviewable and testable in isolation.
2. Proc macro `#[implement]` reduced from ~650 lines of token output to under
   ~80 lines of glue.
3. Foundation usable directly, without any macro, for users who want zero
   procedural-macro dependencies.
4. Optional `macro_rules!` shim (`implement_decl!` and
   `implement_decl_generic!`) that emits the same three items the proc macro
   does, without the TT-muncher hygiene problems that earlier attempts hit.
5. No regression in compile time, run-time performance, or binary size for
   existing `#[implement]` users.
6. No new IDL, no new bindgen attributes, no new dependencies, no nightly
   features.

### Non-goals

* Removing or deprecating `windows-implement`. It stays. It just gets thinner.
* Changing `windows-interface`. `IFoo`, `IFoo_Vtbl`, and `IFoo_Impl` are
  consumed as-is.
* Reworking `ComObject`, `IUnknownImpl`, `ComObjectInner`, `AsImpl`,
  `ComObjectInterface`, `Compose`, or any other existing public trait in
  `windows-core`. Their *implementations* move from "emitted by the macro" to
  "blanket impls on the new generic outer type", but their signatures and
  semantics do not change.
* Supporting any layout the current macro does not support. Generic
  implementer types, lifetimes, trust levels, agile/non-agile, aggregation,
  and dynamic-cast all stay; nothing new is added in this PR series.

## Current state, briefly

For background, the layout emitted today by `crates/libs/implement/src/gen.rs`
is:

```text
#[repr(C)] struct Foo_Impl {
    base: ComposeBase,                      // offset  0 in pointer units
    identity: &'static IInspectable_Vtbl,   // offset  1
    interface1_ifoo: &'static IFoo_Vtbl,    // offset  2
    interface2_ibar: &'static IBar_Vtbl,    // offset  3
    ...
    this: Foo,                              // user data
    count: WeakRefCount,
}
```

A COM caller that holds an `IFoo*` points at the `interface1_ifoo` field. The
vtable's `QueryInterface`/`AddRef`/`Release` thunks recover the `Foo_Impl*` by
applying a *negative* offset `OFFSET = -2 - k`, where `k` is the position of
this interface in the declaration list (the `-2` accounts for `base` +
`identity`). Each vtable is materialised by an associated const on
`impl Foo_Impl`:

```rust,ignore
impl Foo_Impl {
    const VTABLE_IDENTITY: IInspectable_Vtbl =
        IInspectable_Vtbl::new::<Foo_Impl, FirstInterface, -1>();
    const VTABLE_INTERFACE1_IFOO: IFoo_Vtbl =
        IFoo_Vtbl::new::<Foo_Impl, -2>();
    const VTABLE_INTERFACE2_IBAR: IBar_Vtbl =
        IBar_Vtbl::new::<Foo_Impl, -3>();
}
```

The associated-const-in-an-`impl` trick is what makes `Self`-substitution work
in the vtable thunk generic arguments (avoiding E0401).

Option D keeps **every** one of these mechanics. It only changes who emits
them.

## The foundation: `windows_core::imp::implement`

We add a new private module `windows_core::imp::implement` containing the
ABI-aware generic machinery, plus a small `pub` re-export surface
(`windows_core::Outer`, `windows_core::Implemented`, marker traits). The
module sits behind `imp::` rather than its own top-level path because it is an
implementation detail consumed by macros and by advanced users; we want to be
free to evolve it without a semver event. The few names users may legitimately
mention (the type alias's right-hand side, the `Implemented` trait, the knob
types) are re-exported from the crate root with `#[doc(hidden)]` removed.

### Pieces

* **`trait InterfaceList`** — describes one interface list.
* **`struct Outer<T, L>`** — the generic outer struct.
* **`trait Implemented`** — what the user declares on their type.
* **`trait Implements<I>`** — marker bounding `From<T> for I` to listed
  interfaces.
* **Knob types** — `Agile`, `NonAgile`, and a `TrustLevel<const N: u8>`
  marker.
* **Blanket impls** of `Deref`, `IUnknownImpl`, `ComObjectInner`,
  `ComObjectInterface<I>`, `AsImpl<T>`, `From<T> for I`, and `Compose`.

The rest of this section describes each piece. The seven open questions
that drove the first revision of this doc are now resolved inline (with a
"**Resolution (OQ-*N*):**" tag at the relevant point) and summarised in the
[Resolved decisions](#resolved-decisions) table near the end.

### Representation choice: const-generic array vs. tuple-driven HList

There are two viable shapes for `Outer<T, L>`. The plan asks for a spike to
pick between them; this design commits to the **HList shape** as the primary
choice and treats the const-generic-array variant as an optimisation we can
add later without changing the public surface.

#### Why HList wins as the primary

The const-generic-array shape

```rust,ignore
#[repr(C)]
struct Outer<T, L: InterfaceList> {
    base: ComposeBase,
    identity: &'static IInspectable_Vtbl,
    vtables: [*const c_void; L::LEN],   // wants generic_const_exprs
    this: T,
    count: WeakRefCount,
}
```

needs `L::LEN` (an associated const on `InterfaceList`) to flow into the
array length. On stable Rust, generic associated consts can be *used* in
contexts that accept consts, but they **cannot** size an array in a struct
field without `#![feature(generic_const_exprs)]`. The workaround would be a
const-generic *parameter* `const N: usize` on `Outer<T, L, N>` and a `where
[(); N]:` style bound — also nightly. Either way it is a stable-Rust risk.

The HList shape instead expresses the list at the type level:

```rust,ignore
pub struct VNil;
pub struct VCons<I: Interface, R> {
    vtable: &'static I::Vtable,
    rest: R,
}

#[repr(C)]
pub struct Outer<T, L: InterfaceList> {
    base: ComposeBase,
    identity: &'static IInspectable_Vtbl,
    vtables: L::Storage, // a VCons<.., VCons<.., ..., VNil>> tower
    this: T,
    count: WeakRefCount,
}
```

`L::Storage` is an associated type that is itself a `#[repr(C)]` tower of
`VCons` cells. Because `VCons` is `#[repr(C)]` with `vtable` first and `rest`
second, the in-memory layout is *identical* to a flat array of pointers
(plus, on most targets, trailing padding only where natural alignment dictates,
which is the same as today's layout). This works on stable Rust without any
const-expr feature. It also generalises trivially: `L` may be a tuple
`(IFoo, IBar)`, an HList literal, or any other shape that maps to the same
`Storage` tower. We expose the tuple form for users; the HList form is the
internal representation.

#### Tuple sugar over HList

Users see tuples; the library converts internally. We provide:

```rust,ignore
pub trait InterfaceList: Sealed {
    const LEN: usize;
    type Storage;                       // VCons tower
    const IID_SLOTS: &'static [(GUID, usize)];
    // ...
}

impl InterfaceList for () { /* LEN = 0 */ }
impl<I: Interface> InterfaceList for (I,) { /* LEN = 1 */ }
impl<I0: Interface, I1: Interface> InterfaceList for (I0, I1) { /* LEN = 2 */ }
// ... up to a reasonable arity (16 matches today's effective ceiling)
```

Each tuple impl maps to the corresponding `VCons` tower. The maximum
supported arity is **16** (resolves OQ-1). The largest in-tree use today is 3
(`#[implement(IObservableMap<K,V>, IMap<K,V>, IIterable<...>)]`,
`#[implement(IPropertyStore, IInitializeWithStream, IPropertyStoreCapabilities)]`,
etc. — verified by grepping `#[implement(` across `crates/`); 16 leaves a
~5× headroom and matches the de-facto ceiling other Rust HList-via-tuple
libraries use (`frunk`, `axum::extract`, `bevy_ecs::SystemParam`). Anyone
needing more uses the explicit `VCons<I0, VCons<I1, ...VNil>>` form, which
is documented but not sugared. Bumping the limit later is one
search-and-replace inside the per-arity macro in `windows-core` and is
forward-compatible.

#### Optional later optimisation: const-generic array

Once stable Rust accepts `[T; N]` sized by an associated const, the
`vtables` field can be replaced with a flat array. The user-facing surface
does not change. The blanket impls might or might not need adjustment
depending on how rustc resolves the const. We design the foundation so that
this swap is internal.

### `InterfaceList` and `Implements<I>`

```rust,ignore
mod sealed { pub trait Sealed {} }

/// Describes the COM interface list a user's type exposes.
pub trait InterfaceList: sealed::Sealed {
    /// Number of interface chains, excluding the identity (IInspectable) slot.
    const LEN: usize;

    /// Storage tower (a VCons / VNil chain).
    type Storage: 'static;

    /// (IID, slot index) pairs used by the generic QueryInterface.
    const IID_SLOTS: &'static [(GUID, usize)];

    /// Build the storage tower from the per-slot vtable references provided
    /// by the outer impl. Called once per Outer construction.
    fn build_storage<T: 'static>() -> Self::Storage
    where
        Self: ListVtables<T>;
}

/// Marker that says "interface I appears somewhere in this list".
pub trait Implements<I: Interface>: InterfaceList {
    /// Pointer-units offset from the identity slot to this interface's slot.
    /// 0 = identity (IInspectable), 1 = first declared, etc.
    const SLOT: usize;
}
```

`Implements<I>` is what bounds `From<T> for I`: the generic blanket impl
fires only when `T::Interfaces: Implements<I>`. This satisfies the plan's
requirement that the blanket `From` be gated on list membership.

`InterfaceList` is `Sealed` so we can extend it without breaking users.

### Per-slot vtable initialisers

Today the macro emits one associated const per interface that calls
`IFoo_Vtbl::new::<Foo_Impl, -2 - k>()`. In the foundation that becomes a
blanket associated const on a helper trait, parameterised over `T` and the
list position:

```rust,ignore
pub trait ListVtables<T: 'static>: InterfaceList {
    const STORAGE: Self::Storage;
}
```

Implementations are generated for tuple arities 1..=N by a small internal
macro inside `windows-core`. They look like:

```rust,ignore
impl<T, I0> ListVtables<T> for (I0,)
where
    T: IUnknownImpl,
    I0: Interface,
    I0::Vtable: HasVtblCtor<T, -2>,   // see below
{
    const STORAGE: VCons<I0, VNil> = VCons {
        vtable: &<I0::Vtable as HasVtblCtor<T, -2>>::NEW,
        rest: VNil,
    };
}
```

The `HasVtblCtor<T, OFFSET>` helper trait wraps the existing
`Vtbl::new::<T, OFFSET>()` const fn. It exists because we cannot write
`I0::Vtable::new::<T, -2>` directly in a generic context — `new` is an
inherent `const fn` on each concrete `_Vtbl` type, not a trait method. The
helper has a blanket impl:

```rust,ignore
pub trait HasVtblCtor<T, const OFFSET: isize> {
    const NEW: Self;
}

impl<V, T, const OFFSET: isize> HasVtblCtor<T, OFFSET> for V
where
    V: VtableCtor<T, OFFSET>,    // a *sealed* trait, blanket-impl'd by ...
{
    const NEW: Self = <V as VtableCtor<T, OFFSET>>::new();
}
```

…and `VtableCtor<T, OFFSET>` is the one trait that `windows-interface`'s
generated `_Vtbl` types need to opt into. That opt-in is **the one bindgen
change** we have to make, and it is one line per interface:

```rust,ignore
impl<T: ::windows_core::IFoo_Impl, const OFFSET: isize>
    ::windows_core::imp::VtableCtor<T, OFFSET> for IFoo_Vtbl
{
    const fn new() -> Self { <Self>::new::<T, OFFSET>() }
}
```

**Resolution (OQ-2): the `VtableCtor` opt-in is emitted by `windows-interface`
alongside `IFoo_Vtbl`.** The alternative — leave `IFoo_Vtbl::new` callable
only at concrete-type sites and emit the per-slot const inside each macro —
puts the same knowledge in two places (proc macro and `macro_rules!` shim)
and undoes the central goal of Option D. The bindgen edit is mechanical
(one impl in `crates/libs/bindgen/src/types/interface.rs` next to the
existing `_Vtbl::new` emission), bounded (no signature change to `_Vtbl::new`
itself, so existing call sites keep working), and covered by the existing
golden tests under `crates/tests/fixtures/`. The `IFoo_Impl` bound mirrors
the bound `_Vtbl::new::<T, OFFSET>()` already requires today, so no new trait
plumbing is needed on the interface side.

Note: `IInspectable_Vtbl::new` already takes an extra `FirstInterface` type
parameter that drives the runtime-class-name lookup. The `VtableCtor` trait
for `IInspectable_Vtbl` therefore takes an extra type parameter pinned to
`L::FirstInterface` (an associated type on `InterfaceList` that defaults to
`IInspectable` when the list is empty).

### `Outer<T, L>` and `Implemented`

```rust,ignore
/// The single shape that backs every #[implement] type.
#[repr(C)]
pub struct Outer<T: Implemented + 'static, L: InterfaceList = <T as Implemented>::Interfaces> {
    base: ComposeBase,
    identity: &'static IInspectable_Vtbl,
    vtables: L::Storage,
    this: T,
    count: imp::WeakRefCount,
}

/// What the user (or the macro) declares about a type.
pub trait Implemented: Sized + 'static {
    /// The interfaces exposed via QueryInterface.
    type Interfaces: InterfaceList;

    /// Agile / non-agile selection. Defaults to `Agile`.
    type Agility: Agility = Agile;

    /// Trust level returned by IInspectable::GetTrustLevel.
    /// Defaults to 0 (Base trust).
    const TRUST: u8 = 0;

    /// Whether DYNAMIC_CAST_IID is honoured. Defaults to true and is
    /// automatically false for types containing non-'static lifetimes.
    /// The default impl is kept overridable for forward compatibility.
    const DYNAMIC_CAST: bool = true;
}
```

`Agility` is a sealed marker trait with two implementors:

```rust,ignore
pub trait Agility: sealed::Sealed {
    /// Whether QI for IAgileObject succeeds.
    const IS_AGILE: bool;
    /// Whether IMarshal is exposed (windows-only).
    const HAS_MARSHAL: bool;
}
pub struct Agile;     // IS_AGILE = true,  HAS_MARSHAL = cfg(windows)
pub struct NonAgile;  // IS_AGILE = false, HAS_MARSHAL = false
```

The choice between phantom types and const-generic bools is one of taste.
**Resolution (OQ-3): sealed marker types.** Two reasons:

1. **Composability.** `where T::Agility: Agility<HAS_MARSHAL = true>` is
   awkward in `where`-clauses; `where T::Agility = Agile` is not legal Rust
   at all. With marker types, callers write `where T::Agility: AgileMarshaler`
   and we get to pick the trait granularity later (e.g. a future
   `FreeThreadedMarshaler` selection without changing the `Agility`
   signature).
2. **Forward-extension cost.** Adding a third agility level (e.g. STA-only)
   is a new struct + `impl Agility for ...`. With const generics it would
   require widening every const, and any downstream code that pattern-matched
   on the bool would need updating.

Today's macro effectively does the same thing — it emits or omits whole
blocks based on `Agile = true/false` — so the user-visible knob (`Agile =
true`) does not change. The macro layer translates `Agile = true/false` to
`type Agility = Agile;` / `type Agility = NonAgile;` in the generated
`impl Implemented` block.

The defaults on `Implemented` correspond to today's defaults: agile, trust
level 0, dynamic cast on (subject to the lifetime constraint that the proc
macro currently enforces and that the macro will keep enforcing).

### Blanket impls

The substance of `gen.rs` becomes seven blanket impls on
`Outer<T, L>`. Each maps directly to one of today's `gen_*` functions:

| Today (`gen.rs`)                  | Tomorrow (blanket impl on `Outer<T, L>`)               |
|-----------------------------------|--------------------------------------------------------|
| `gen_original_impl::into_outer`   | `Outer::new` inherent fn (const where `T: !generic`)   |
| `gen_original_impl::into_static`  | inherent `into_static` on `T: Implemented`             |
| `gen_impl_struct`                 | `Outer` definition (single struct)                     |
| `gen_impl_deref`                  | `impl Deref for Outer<T, L>`                           |
| `gen_impl_impl` (vtable consts)   | `ListVtables` blanket impls (one per tuple arity)      |
| `gen_iunknown_impl`               | `impl IUnknownImpl for Outer<T, L>`                    |
| `gen_query_interface`             | a single generic `QueryInterface` method               |
| `gen_impl_com_object_inner`       | `impl ComObjectInner for T where T: Implemented`       |
| `gen_impl_compose`                | `impl Compose for T where T: Implemented`              |
| `gen_impl_from` (per iface)       | `impl<I> From<T> for I where L: Implements<I>`         |
| `gen_impl_com_object_interfaces`  | `impl<I> ComObjectInterface<I> for Outer<...>` ditto   |
| `gen_impl_as_impl`                | `impl<I> AsImpl<T> for I` ditto                        |

The two impls that need real care are `QueryInterface` and the blanket
conversions. The rest are mechanical.

#### Generic `QueryInterface`

```rust,ignore
impl<T: Implemented> IUnknownImpl for Outer<T, T::Interfaces>
where
    T::Interfaces: InterfaceList + ListVtables<Outer<T, T::Interfaces>>,
{
    type Impl = T;

    unsafe fn QueryInterface(
        &self,
        iid: *const GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        if iid.is_null() || interface.is_null() {
            return imp::E_POINTER;
        }
        let iid = unsafe { *iid };

        // Identity: IUnknown, IInspectable, and (if agile) IAgileObject all
        // resolve to &self.identity.
        if iid == IUnknown::IID
            || iid == IInspectable::IID
            || (<T::Agility as Agility>::IS_AGILE && iid == imp::IAgileObject::IID)
        {
            return finish(self, &self.identity as *const _ as *const c_void, interface);
        }

        // Declared interfaces: linear scan over IID_SLOTS. The slot index
        // is the pointer-units offset from `identity`, so the address of the
        // matching vtable pointer is `(&self.identity as *const _).add(slot)`.
        for &(slot_iid, slot) in <T::Interfaces as InterfaceList>::IID_SLOTS {
            if iid == slot_iid {
                let base = &self.identity as *const &'static IInspectable_Vtbl
                    as *const *const c_void;
                let p = unsafe { base.add(slot) };
                return finish(self, p as *const c_void, interface);
            }
        }

        // IMarshal (windows + agile).
        #[cfg(windows)]
        if <T::Agility as Agility>::HAS_MARSHAL && iid == imp::IMarshal::IID {
            return imp::marshaler(self.to_interface::<IUnknown>(), interface);
        }

        // Dynamic cast (Any).
        if T::DYNAMIC_CAST && iid == DYNAMIC_CAST_IID {
            unsafe {
                (interface as *mut *const dyn core::any::Any)
                    .write(self as &dyn core::any::Any as *const _);
            }
            return HRESULT(0);
        }

        // Tear-off + aggregation fallthrough.
        let tear = self.count.query(&iid, &self.identity as *const _ as *mut _);
        if !tear.is_null() {
            unsafe { *interface = tear; }
            return HRESULT(0);
        }
        if let Some(base) = self.base.as_option() {
            return unsafe { Interface::query(base, &iid, interface) };
        }

        unsafe { *interface = core::ptr::null_mut(); }
        imp::E_NOINTERFACE
    }
    // AddRef, Release, GetTrustLevel, etc. are direct translations of today's macro.
}
```

`IID_SLOTS` is materialised in the tuple `InterfaceList` impl:

```rust,ignore
impl<I0: Interface, I1: Interface> InterfaceList for (I0, I1) {
    const LEN: usize = 2;
    type Storage = VCons<I0, VCons<I1, VNil>>;
    // identity sits at slot 0; first declared at slot 1; second at slot 2.
    const IID_SLOTS: &'static [(GUID, usize)] = &[
        (I0::IID, 1),
        (I1::IID, 2),
    ];
    ...
}
```

The match-based approach the macro uses today (one `if` per interface,
short-circuiting on first hit) is equivalent in code-gen to this loop after
inlining: the slice is a `'static` table of small fixed length, the loop is
trivially unrolled by LLVM, and each comparison reduces to two 64-bit `eq`
checks on the GUID.

**Resolution (OQ-4): start with the loop; benchmark in Step 0; only
specialise if a regression appears.** Concretely the spike (Step 0) adds a
criterion microbenchmark that calls `QueryInterface` for (a) the identity
IID, (b) the last-declared IID, and (c) an unknown IID, with implementer
arities of 1, 3, 8, and 16. The pass criterion is "within 1 ns / call of
today's macro output on x86_64 release builds with `lto = "thin"`". If any
configuration regresses by more than that, the per-arity tuple `InterfaceList`
impl gains a hand-rolled match for the `IID_SLOTS` lookup (the rest of the
generic `QueryInterface` body still reuses the blanket). The decision point
is recorded in this doc when the spike lands; in the meantime the loop is
the working assumption because it minimises monomorphisation overhead and is
easier to audit.

#### Blanket conversions

```rust,ignore
impl<T, I> From<T> for I
where
    T: Implemented,
    I: Interface,
    T::Interfaces: Implements<I>,
{
    fn from(this: T) -> Self {
        ComObject::new(this).into_interface()
    }
}

impl<T, I> ComObjectInterface<I> for Outer<T, T::Interfaces>
where
    T: Implemented,
    I: Interface,
    T::Interfaces: Implements<I>,
{
    fn as_interface_ref(&self) -> InterfaceRef<'_, I> {
        // The slot offset is `Implements<I>::SLOT` pointer-units past `identity`.
        let base = &self.identity as *const _ as *const *const c_void;
        let p = unsafe { base.add(<T::Interfaces as Implements<I>>::SLOT) };
        unsafe { core::mem::transmute(p) }
    }
}

impl<T, I> AsImpl<T> for I
where
    T: Implemented,
    I: Interface,
    T::Interfaces: Implements<I>,
{
    unsafe fn as_impl_ptr(&self) -> NonNull<T> {
        let this = Interface::as_raw(self);
        let this = (this as *mut *mut c_void)
            .sub(1 + <T::Interfaces as Implements<I>>::SLOT)
            as *mut Outer<T, T::Interfaces>;
        NonNull::new_unchecked(core::ptr::addr_of!((*this).this) as *mut T)
    }
}
```

`Implements<()>::SLOT` rules: identity = 0, declared interfaces start at 1.
The `as_impl_ptr` arithmetic subtracts `1 + SLOT` because the `Outer` struct
adds *one* leading field (`base`) before `identity`, and `SLOT` is measured
from `identity`. (That `1 +` is the only place the magic number `1` appears
in the foundation; today's macro carries it as `2 +` because it counts from
the first vtable slot.)

There is **one coherence concern** with `From<T> for I`. Rust's orphan rules
allow this impl because either `T` or `I` is local at every use site (`T` is
the user's type, `I` is a `windows-core` interface), but the blanket form
asks rustc to prove non-overlap with the per-interface `From<T> for I` impls
the existing macro emits.

**Resolution (OQ-5): the blanket lives behind the `Implements<I>` bound, and
the macro stops emitting per-interface `From` impls in Step 2.** Stage-by-stage:

* **Step 1 (foundation only).** The blanket `From<T> for I where T: Implemented,
  T::Interfaces: Implements<I>` does not collide with the macro's per-interface
  `From` impls because the blanket only fires for types that have a
  hand-written `impl Implemented`, and `#[implement]` does *not* emit
  `impl Implemented` in Step 1. Existing user code is therefore untouched.
* **Step 2 (macro reskinned).** `#[implement]` emits `impl Implemented for T`
  and stops emitting per-interface `From`s. The blanket takes over. This is
  one `quote!` block deleted from `gen_impl_from`, and the existing macro
  test suite (every `From<Foo> for IFoo` call site in
  `crates/tests/libs/implement*`) is the regression gate.
* **Edge case.** If a user writes both `#[implement(IFoo)]` and a hand-written
  `impl Implemented for Foo`, rustc rejects the program with a duplicate-impl
  error. This is acceptable: the macro and the hand-written declaration are
  alternatives, not stackable. The error message is unambiguous because both
  impls cite the same `Foo` and `IFoo` types.

The fallback considered (keep the per-interface emission, no blanket at all)
was rejected because it would force `implement_decl!` and hand-written use to
re-emit the per-interface impls themselves, defeating the purpose of moving
the knowledge into `windows-core`.

### Construction

`Outer::<T, T::Interfaces>::new(value: T)` constructs the outer struct, the
identity vtable, the vtable storage tower, and a fresh `WeakRefCount`. For
non-generic `T`, this is a `const fn` (so `ComObject<T>::new` and
`StaticComObject` are unchanged). For generic `T`, it is a regular fn (today's
macro already has this distinction; `into_outer` is `const` only for
non-generic types).

```rust,ignore
impl<T: Implemented> Outer<T, T::Interfaces> {
    pub const fn new(value: T) -> Self
    where
        T: NonGeneric,         // sealed marker, auto-implemented (see below)
    {
        Self {
            base: ComposeBase::new(),
            identity: &<IInspectable_Vtbl as VtableCtor<...>>::NEW,
            vtables: <T::Interfaces as ListVtables<_>>::STORAGE,
            this: value,
            count: imp::WeakRefCount::new(),
        }
    }

    pub fn new_generic(value: T) -> Self { /* non-const */ }
}
```

**Resolution (OQ-6): two named constructors.** `Outer::new` is `const`, gated
by a sealed `NonGeneric` marker that is auto-implemented by a blanket impl
on every `T` whose `T::Interfaces` resolves to a tuple of *concrete* types
(no generic parameter mentioned). `Outer::new_generic` is the non-`const`
counterpart used when `T` carries free generics.

Why two named ctors instead of one polymorphic `new`:

* Rust does not allow a `const fn` to be conditionally `const` on a trait
  bound — `pub const fn new(value: T) -> Self where T: NonGeneric` is
  well-formed but the `const`-ness is a property of the function, not the
  bound, so callers in generic contexts get a confusing error.
* Two ctors map cleanly to today's macro split (`is_generic` flag, see
  `crates/libs/implement/src/gen.rs:443-448`) — the macro picks `new` or
  `new_generic` from the same `is_generic` check, with no other change to
  its emission shape.
* The sealed `NonGeneric` marker is still useful as a *bound* on `new`'s
  signature, because it gives a clear error ("`T` is generic; use
  `Outer::new_generic`") instead of a const-eval failure deep inside
  vtable initialisation.

`new_generic` is what `ComObjectInner::into_object` calls in its blanket impl,
so `ComObject::new(generic_value)` keeps working without the user picking a
ctor.

`ComObjectInner::into_object` becomes a blanket impl:

```rust,ignore
impl<T: Implemented> ComObjectInner for T {
    type Outer = Outer<T, T::Interfaces>;
    fn into_object(self) -> ComObject<Self> {
        let boxed = imp::Box::<Outer<T, T::Interfaces>>::new(Outer::new_generic(self));
        unsafe { ComObject::from_raw(NonNull::new_unchecked(imp::Box::into_raw(boxed))) }
    }
}
```

### What the proc macro emits

After the foundation is in place, the body of `gen_all` becomes:

```rust,ignore
quote! {
    #original_struct        // pass through

    #[allow(non_camel_case_types)]
    #vis type #impl_ident #generics = ::windows_core::Outer<
        #original_ident #generics_idents,
        ( #( #interface_ty , )* ),
    >;

    impl #generics ::windows_core::Implemented for #original_ident #generics_idents
    where #constraints
    {
        type Interfaces = ( #( #interface_ty , )* );
        type Agility    = #agility_ty;
        const TRUST: u8 = #trust_level;
        const DYNAMIC_CAST: bool = #dynamic_cast;
    }

    // Only emitted when `!is_generic` — see OQ-7 below.
    #into_static_forwarder
}
```

That is the entire macro body once today's `gen.rs` is deleted. The attribute
parser in `lib.rs` stays (it still has to understand `IFoo`, `IBar`,
`TrustLevel = Full`, `Agile = false`). Everything below the `quote!` shrinks
to nothing.

The struct definition is passed through unchanged. The `Foo_Impl` type alias
points at `Outer<Foo, (IFoo, IBar)>`, so any user code that mentions
`Foo_Impl` as a type still resolves. Any `impl IFoo_Impl for Foo_Impl` block
the user wrote also resolves, because `Foo_Impl` is just an alias and trait
impls on the underlying `Outer<...>` type satisfy the alias.

There is one wart: today the macro emits `impl Foo_Impl { fn into_outer(self) {...} }`
and `into_static` (the latter only for non-generic `T`). With the foundation,
`into_outer` becomes the inherent `Outer::new` (or `Outer::new_generic`) and
needs no forwarder — `Foo_Impl` is a type alias, so `Foo_Impl::new(value)`
calls the inherent ctor on `Outer`.

`into_static` is trickier because today it lives as an inherent method on
**`Foo`**, not on `Foo_Impl`. **Resolution (OQ-7): the macro keeps emitting
the two-line `into_static` forwarder on `Foo`.** Concretely:

```rust,ignore
impl Foo {
    pub const fn into_static(self) -> ::windows_core::StaticComObject<Foo_Impl> {
        ::windows_core::StaticComObject::from_outer(Foo_Impl::new(self))
    }
}
```

Reasoning:

* Rust does not allow inherent methods to be added to a foreign type via a
  blanket impl. The only ways to give `Foo::into_static()` to user code are
  (a) emit an inherent method from the macro, (b) put it on a trait the user
  must import, or (c) require the user to call
  `StaticComObject::from_outer(Foo_Impl::new(value))` directly.
* (b) breaks today's call sites unless we also add a `prelude` import, which
  is a wider conversation than Option D should drag in. (c) is a public-API
  break.
* (a) is two lines per macro invocation and zero lines elsewhere. The
  forwarder cannot drift out of sync with the foundation because it only
  references public API (`StaticComObject::from_outer` and `Outer::new`).
* `into_static` is gated on non-generic `T` exactly as today; the macro reuses
  its `is_generic` check.

`implement_decl!` users get the same forwarder by way of an extra macro arm
(also two lines). Hand-written zero-macro users call
`StaticComObject::from_outer(Foo_Impl::new(value))` directly, which is the
form documented in the "Hand-written use" section below.

### What `implement_decl!` emits

The plan envisions a `macro_rules!` sibling for users who want to avoid the
proc-macro dependency. Earlier attempts at this hit recursive
TT-muncher hygiene problems because they were trying to emit
`QueryInterface` arms, vtable const initialisers, and per-interface `From`
impls. With Option D those problems vanish — none of that lives in the
macro any more. The `macro_rules!` shim only has to emit the same three
items the proc macro emits:

```rust,ignore
#[macro_export]
macro_rules! implement_decl {
    (impl $T:ty as $vis:vis $Alias:ident : [ $($I:ty),+ $(,)? ] ) => {
        $vis type $Alias = $crate::Outer<$T, ( $($I,)+ )>;

        impl $crate::Implemented for $T {
            type Interfaces = ( $($I,)+ );
        }
    };
}

#[macro_export]
macro_rules! implement_decl_generic {
    (impl < $($G:tt),+ > $T:ty as $vis:vis $Alias:ident : [ $($I:ty),+ $(,)? ]
        where $($W:tt)* ) => {
        $vis type $Alias < $($G),+ > = $crate::Outer< $T, ( $($I,)+ ) >;

        impl< $($G),+ > $crate::Implemented for $T
        where $($W)*
        {
            type Interfaces = ( $($I,)+ );
        }
    };
}
```

No accumulator. No `'found` label. No `self`/`iid` references. No
const-promotion gymnastics. Each macro is under 20 lines including support
for the agile / trust knobs.

### Hand-written use

Because `Outer`, `Implemented`, the interface-list types, and the knob types
are all public, a user who wants zero macros can write:

```rust,ignore
use windows_core::*;

struct Foo(i32);

pub type Foo_Impl = Outer<Foo, (IValue,)>;

impl Implemented for Foo {
    type Interfaces = (IValue,);
}

impl IValue_Impl for Foo_Impl {
    unsafe fn GetValue(&self, out: *mut i32) -> HRESULT {
        *out = self.0;          // Deref to Foo
        HRESULT(0)
    }
}

fn main() {
    let v: IValue = Foo(42).into();
    // ...
}
```

That is the entire user-facing surface, and it falls out of the foundation
for free.

## Migration plan

Each step is independently shippable, builds on the previous one, and is
gated by the existing implement test suite plus the new foundation tests.

### Step 0 — Spike (this branch, no PR yet)

Build the HList shape end-to-end **in a test crate** under
`crates/tests/libs/implement_foundation_spike` (new). The spike:

* Defines `Outer`, `InterfaceList`, `Implemented`, `Implements`,
  `VtableCtor`, the blanket impls, and the tuple `InterfaceList` impls for
  arity 0..=4.
* Adds the one `VtableCtor` impl for `IUnknown_Vtbl`, `IInspectable_Vtbl`,
  and a hand-rolled fake `IValue_Vtbl` (so we are not yet asking bindgen to
  emit it).
* Hand-writes one `#[implement]`-equivalent test case using the foundation
  directly.
* Compares the layout of the resulting `Outer<Foo, (IValue,)>` to the layout
  of today's macro-emitted `Foo_Impl` with `core::mem::offset_of!` and a
  `static_assertions::assert_eq_size!` check.
* Compiles on stable Rust with `--no-default-features` and with default
  features.
* **Runs the OQ-4 microbenchmark** (criterion) comparing
  `QueryInterface(identity)`, `QueryInterface(last)`, and
  `QueryInterface(unknown)` between the macro-emitted `Foo_Impl` and the
  foundation's `Outer<Foo, ...>`, at arities 1, 3, 8, and 16. Pass criterion:
  within 1 ns/call on x86_64 release builds with `lto = "thin"`.

If the spike compiles, the layout matches, and the benchmark passes the OQ-4
criterion, the design is locked. If the layout check fails, this doc is
updated. If only the benchmark fails, the per-arity tuple `InterfaceList`
impl gains an unrolled-match specialisation (the rest of the design holds).

### Step 1 — Foundation only, macro untouched

* New module `windows_core::imp::implement`.
* Public re-exports: `Outer`, `Implemented`, `Implements`, `InterfaceList`,
  `Agile`, `NonAgile`. All marked `#[doc(hidden)]` initially so the surface
  can still evolve; visibility is widened in Step 4.
* New `VtableCtor` trait emitted by `windows-interface` (one line per
  interface, alongside `_Vtbl`).
* New tests under `crates/tests/libs/implement_foundation` exercising
  hand-written use of the foundation (covers QI, AddRef/Release,
  `From<T> for I`, agile/non-agile, trust level, dynamic cast,
  composition).
* No changes to `crates/libs/implement` and no changes to any user code.
* The pre-existing implement test suite still passes unchanged.

### Step 2 — Reskin `#[implement]` as a shim

* Delete `crates/libs/implement/src/gen.rs` (or shrink it to the ~30 lines of
  emission described above).
* `lib.rs` keeps the attribute parser.
* The existing implement test suite is the regression gate. It must pass
  unchanged.

This is the highest-risk PR in the series because it changes generated code
for every existing `#[implement]` use. Mitigations:

* Run the implement test suite with `cargo test --tests` plus the integration
  test crates in `crates/tests/libs/{collections,future,implement}`.
* Diff the assembly of one representative `Foo_Impl::QueryInterface` between
  branches and confirm no regression in instruction count for the
  fast-path identity match.
* Snapshot the macro output before/after with `cargo expand` on the
  `crates/tests/libs/implement` fixtures and review the diff.

### Step 3 — Land `implement_decl!`

This step is **net-new**, not a rewrite, because `implement_decl!` does not
currently exist in this branch (verified: no `implement_decl` references in
the tree on `copilot/explore-alternatives-for-com-objects`). The macros
described in the memories belong to PR #4403, which has not been merged.

* Add `crates/libs/core/src/implement_macro.rs` (or similar) with the two
  `macro_rules!` definitions described above.
* Add unit tests under `crates/tests/libs/implement_decl`.
* Document both as alternatives to `#[implement]`.

### Step 4 — Document the foundation as public API

* Drop `#[doc(hidden)]` from `Outer`, `Implemented`, `Implements`, the
  knob types, and the tuple `InterfaceList` impls.
* Add a "Implementing COM by hand" doc page in `docs/` walking through the
  zero-macro form.
* Add the foundation to `windows-core`'s readme.

### Step 5 — Optional follow-ups

* Benchmark the generic `QueryInterface` against the today's match-based form
  on a representative workload; specialise the per-arity tuple impl with an
  unrolled match if a regression is observed.
* Evaluate whether `windows-implement` can be reduced to a re-export plus the
  attribute parser, or eliminated entirely in favour of `implement_decl!`.
  This is a community conversation, not a step.

## Compatibility

* `Foo_Impl` is still a nameable type. It is now a type alias rather than a
  fresh struct, but `impl IFoo_Impl for Foo_Impl` still resolves.
* `ComObject<Foo>`, `AsImpl<Foo>`, `From<Foo> for IFoo`, `Deref` from
  `Foo_Impl` to `Foo`: all preserved.
* Field order, in-memory layout, vtable offsets: byte-for-byte identical.
* The COM ABI on the wire is unchanged.
* `IUnknownImpl`, `ComObjectInner`, `Compose`, `ComObjectInterface`, `AsImpl`,
  `Interface`, `ComObject`: signatures unchanged.

The only observable break is the orphan-rules edge case noted in OQ-5
(simultaneous `#[implement]` and hand-written `impl Implemented`), which no
sensible user program contains.

## Performance

The current macro emits, per interface:

* one `if iid == IFoo::IID` chain in `QueryInterface`,
* one direct field access in `ComObjectInterface<IFoo>::as_interface_ref`,
* one direct pointer arithmetic in `AsImpl<Foo> for IFoo::as_impl_ptr`,
* one direct vtable const,
* one direct `From<Foo> for IFoo`.

The foundation emits, per `Outer<T, L>` monomorphisation:

* one generic `QueryInterface` whose hot path is a 1..=N entry loop over
  `IID_SLOTS` plus the identity short-circuit;
* one `ComObjectInterface<I>` that resolves `Implements<I>::SLOT` at compile
  time and reduces to the same field access after inlining;
* same for `AsImpl<T>`;
* `STORAGE` is a single `const` tower equivalent to today's per-field consts;
* `From<T> for I` is one blanket that monomorphises into the same call.

After inlining and LLVM's loop-unroller, the two are expected to produce
identical code for `as_interface_ref` and `as_impl_ptr`, and within a
single-instruction tolerance for `QueryInterface`. The OQ-4 microbenchmark
in Step 0 is the empirical check on that expectation.

## Resolved decisions

The seven open questions raised in the first revision of this doc are now
resolved. Each row links to the section that contains the full reasoning;
the "Why not the alternative" column is intentionally short — it captures the
single load-bearing reason the alternative was rejected, not an exhaustive
trade-off list.

| #    | Question                                                | **Decision**                                                              | Why not the alternative                                                                            |
|------|---------------------------------------------------------|---------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|
| OQ-1 | Maximum supported tuple arity                           | **16**, with explicit `VCons` form for anything larger                    | Lower would force most use cases through HList syntax; higher is a search-and-replace away         |
| OQ-2 | Where does `VtableCtor` get emitted                     | **`windows-interface` emits one extra impl per `_Vtbl`**                  | Per-slot const in macros duplicates ABI knowledge across proc macro + `macro_rules!` shim          |
| OQ-3 | Knob representation                                     | **Sealed marker types** (`Agile` / `NonAgile`)                            | Const generics can't carry future knobs (e.g. STA-only) without widening every const               |
| OQ-4 | `QueryInterface` loop vs. unrolled match                | **Loop**, with Step 0 benchmark; specialise per-arity only if regression  | Unconditional unrolled match adds monomorphisation cost we don't yet need                          |
| OQ-5 | `From<T> for I` blanket vs. coexisting macro emission   | **Blanket gated by `Implements<I>`**, macro stops emitting per-iface From | Keeping macro emission would force `implement_decl!` to re-emit too, defeating the foundation's purpose |
| OQ-6 | `const fn` constructor vs. two ctors                    | **Two named ctors** (`Outer::new`, `Outer::new_generic`)                  | One conditionally-`const` ctor produces confusing errors in generic contexts                       |
| OQ-7 | How does `Foo::into_static()` stay callable             | **Macro emits a 2-line forwarder on `Foo`**                               | Blanket on `T: Implemented` cannot add inherent methods to a foreign type; trait import is API-breaking |

These decisions are reflected in the body of the doc above and in the
migration plan. None of them blocks the Step 0 spike; OQ-4 is the only one
whose final form can change as a result of the spike (the loop may gain a
per-arity match specialisation), and that change is internal to
`windows-core`.

## What this document does not commit to

* Whether `windows-implement` is eventually deprecated. That is a Step 5+
  conversation.
* Whether the foundation gains additional knobs beyond today's set
  (e.g. `IMarshal` strategies, custom `GetTrustLevel`, free-threaded
  marshaler choice). Those are independent extensions and do not change
  the shape of `Outer<T, L>`.
* Any change to `windows-interface`'s `_Impl` trait or `_Vtbl::new`
  signature beyond the one-line `VtableCtor` opt-in described in OQ-2.

## Next action

The next action is the spike (Step 0). It is small (under a day's work),
contained to a new test crate at
`crates/tests/libs/implement_foundation_spike`, and exercises every
load-bearing decision above:

* Layout-equality assertions answer the OQ-2 / OQ-6 plumbing.
* The criterion microbenchmark answers OQ-4 (loop vs. unrolled match).
* Hand-written `impl Implemented` plus a `#[implement]`-style call site
  exercise OQ-3 / OQ-5 / OQ-7 in the same crate.

After the spike lands this section is updated with the actual numbers and
the migration plan is unblocked.
