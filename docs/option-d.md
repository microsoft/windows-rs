# Option D — A library-based foundation for `#[implement]`

> **Status:** design draft for review. No code under `crates/` has been changed
> yet. The goal of this document is to settle the shape of the foundation
> before any of the existing `#[implement]` machinery is touched.

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

The rest of this section describes each piece. Open questions (with a
proposed default and a fallback) are called out as **OQ-N**.

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

Each tuple impl maps to the corresponding `VCons` tower. Beyond the supported
arity, users (and the macros) can drop down to HList syntax. **OQ-1:** pick a
maximum tuple arity. Proposed default: 16. Anyone needing more uses the
explicit HList form.

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

**OQ-2:** the cleanest way to do this is to teach `windows-interface` to emit
that one extra impl alongside `IFoo_Vtbl`. The alternative is to leave
`IFoo_Vtbl::new` callable only at concrete-type sites and emit the per-slot
const inside the macro (which puts the duplicate knowledge back in two
places). I recommend the bindgen change; it is mechanical and bounded.

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
Phantom types let us derive other things from them (e.g. a future
"thread-affine" agility could add a `'static` bound). The plan suggests const
generics; this design uses sealed marker types because they compose better
with `where`-clauses. **OQ-3.**

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
checks on the GUID. We do not expect any measurable regression. If a
benchmark shows one, the per-arity tuple impl can specialise to an unrolled
match. **OQ-4** flags this as something to benchmark before locking the
foundation.

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
asks rustc to prove non-overlap with hypothetical future blanket impls of
`From<X> for Y` written by users. In practice, the existing macro emits
non-blanket impls (one per interface) and those impls would collide with this
blanket. So:

* In Step 1 (foundation only, macro untouched) the blanket `From` is **gated
  by the `Implements<I>` bound**, which means it does not fire unless the
  user (or the macro) has declared `Implemented` with a matching list. The
  per-interface `From` impls emitted by today's macro continue to exist; they
  win where they apply, and the blanket fills in for hand-written
  `Implemented` declarations.
* In Step 2 (macro reskinned) the macro stops emitting the per-interface
  `From`s and relies on the blanket instead.

Step 1 alone is therefore safe to land without touching any existing user
code. **OQ-5** captures the one edge case: if a user has both a
`#[implement]` and a hand-written `impl Implemented`, the macro-generated
per-interface impls and the blanket would coexist; rustc would reject that
program. Acceptable, because nobody should be doing both for the same type.

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

**OQ-6:** the `const` distinction. The proc macro currently inspects
`generics.params.is_empty()` and decides whether to emit `const fn`. The
foundation can do the same with a sealed `NonGeneric` marker auto-implemented
where appropriate, or by simply offering two constructors and letting the
macro pick. Two constructors is the simpler path.

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
and `into_static`. The first becomes `Outer::new` (inherent) and the second
stays as an inherent fn on `Foo` provided by a blanket impl on
`T: Implemented`. **OQ-7:** check that `Foo::into_static()` stays callable
through normal method resolution — it should, because blanket impls on `T`
provide inherent-looking methods at the call site even when they live in a
separate trait. If method resolution turns out to be unhappy, fall back to
the macro emitting a trivial `impl Foo { pub const fn into_static(self) -> ...
{ Outer::new(self).into_static() } }`.

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

If the spike compiles and the layout matches, the design is locked. If not,
the report goes back to this document and the OQs are revisited.

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
single-instruction tolerance for `QueryInterface`. **OQ-4** (above) calls
out the one place where we should measure before locking the design.

## Open questions, consolidated

| #     | Question                                                              | Default                            | Fallback                         |
|-------|-----------------------------------------------------------------------|------------------------------------|----------------------------------|
| OQ-1  | Maximum supported tuple arity                                         | 16                                 | Higher, or HList literals        |
| OQ-2  | Where does `VtableCtor` get emitted                                   | `windows-interface` adds one line  | Macro re-emits per-slot consts   |
| OQ-3  | Knob representation                                                   | Sealed marker types                | `const` generics                 |
| OQ-4  | `QueryInterface` loop vs. unrolled match                              | Loop, benchmarked                  | Per-arity unrolled match         |
| OQ-5  | `From<T> for I` blanket vs. coexisting macro emission                 | Blanket gated by `Implements<I>`   | Keep macro emission, no blanket  |
| OQ-6  | `const fn` constructor vs. two ctors                                  | Two ctors (`new`, `new_generic`)   | Sealed `NonGeneric` marker       |
| OQ-7  | Method resolution for `into_static` via blanket                       | Blanket on `T: Implemented`        | Macro emits inherent forwarder   |

None of these block the spike. All are revisited after Step 0 lands.

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
contained to a new test crate, and answers every load-bearing question in
this document. After it lands, this doc is updated with the spike's results
and the OQs are resolved before Step 1 begins.
