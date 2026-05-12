# Option D — A library-based foundation for `#[implement]`

> **Status:** design draft. All seven open questions are resolved (see
> [Resolved decisions](#resolved-decisions)). The Step 0 spike lives at
> [`crates/tests/libs/implement_foundation_spike`](../crates/tests/libs/implement_foundation_spike);
> both phases have landed. Phase 2's OQ-4 microbenchmark shows the foundation
> dispatching `QueryInterface` **faster** than today's macro on all three
> paths (identity, declared, unknown) — well inside the ±1 ns/call bar.

## Summary

Today the `#[implement]` proc macro emits ~650 lines of tokens per use site:
a `#[repr(C)]` outer struct, a vtable const per interface, an `IUnknownImpl`
impl with a hand-rolled `QueryInterface`, plus `ComObjectInner`, `Compose`,
`Deref`, and per-interface `From` / `ComObjectInterface<I>` / `AsImpl` impls.
Every fact about the COM ABI — field order, `OFFSET = -2 - k` thunk trick,
`DYNAMIC_CAST_IID` carve-out, `IMarshal` `#[cfg(windows)]` gate, aggregation
fall-through — lives inside that token surgery.

Option D moves that knowledge into `windows-core` as ordinary generic code.
The macro shrinks to an attribute parser plus three emissions: a type alias,
an `impl Implemented`, and (for non-generic `T`) a two-line `into_static`
forwarder.

The user-facing surface (`#[implement(IFoo, IBar)] struct Foo`, the
`Foo_Impl` type name, `ComObject<Foo>`, `AsImpl<Foo>`, `From<Foo> for IFoo`)
is preserved. The COM ABI on the wire is byte-for-byte identical.

## Goals and non-goals

**Goals:**

1. Single source of truth for the COM-object ABI inside `windows-core`.
2. Proc macro reduced from ~650 lines of emitted tokens to ~30.
3. Foundation usable without any macro, for zero-procedural-macro users.
4. Optional `macro_rules!` shim (`implement_decl!`) that emits the same items.
5. No regression in compile time, run-time, or binary size.
6. No new IDL, bindgen attributes, dependencies, or nightly features.

**Non-goals:** removing `windows-implement`; changing `windows-interface`;
reworking `ComObject`/`IUnknownImpl`/`ComObjectInner`/`AsImpl`/
`ComObjectInterface`/`Compose` signatures; supporting any layout the current
macro doesn't support.

## Current layout (today)

```text
#[repr(C)] struct Foo_Impl {
    base: ComposeBase,                      // ptr 0
    identity: &'static IInspectable_Vtbl,   // ptr 1
    interface1_ifoo: &'static IFoo_Vtbl,    // ptr 2
    interface2_ibar: &'static IBar_Vtbl,    // ptr 3
    this: Foo,
    count: WeakRefCount,
}
```

A caller holding `IFoo*` points at `interface1_ifoo`. Thunks recover the
`Foo_Impl*` via `OFFSET = -2 - k` (the `-2` accounts for `base` + `identity`).
Each vtable is materialised by an associated const on `impl Foo_Impl` so
`Self`-substitution works in the thunk generics (avoiding E0401).

Option D keeps every one of these mechanics. It only changes who emits them.

## The foundation

A new private module `windows_core::imp::implement` contains the ABI-aware
generic machinery, with a small `pub` re-export surface
(`windows_core::{Outer, Implemented, Implements, InterfaceList, Agile,
NonAgile}`). Hidden behind `imp::` so the internals can evolve without a
semver event.

### Representation: tuple-driven HList

`Outer<T, L>` carries the list at the type level. `L::Storage` is a `#[repr(C)]`
tower of `VCons` cells, byte-identical to today's flat sequence of
`&'static _` fields:

```rust,ignore
pub struct VNil;

#[repr(C)]
pub struct VCons<I: Interface, R> {
    vtable: &'static I::Vtable,
    rest: R,
}

#[repr(C)]
pub struct Outer<T: Implemented, L: InterfaceList = <T as Implemented>::Interfaces> {
    base: ComposeBase,
    identity: &'static IInspectable_Vtbl,
    vtables: L::Storage,
    this: T,
    count: imp::WeakRefCount,
}
```

A `const N: usize` array shape would need `generic_const_exprs` (nightly).
The HList shape works on stable today; if `[T; N]` sized by an associated
const ever stabilises, the field can be swapped internally without a surface
change.

### `InterfaceList`, `Implements<I>`, `ListVtables<T>`

```rust,ignore
mod sealed { pub trait Sealed {} }

pub trait InterfaceList: sealed::Sealed {
    const LEN: usize;
    type Storage: 'static;
    const IID_SLOTS: &'static [(GUID, usize)];   // (IID, slot index from identity)
}

pub trait Implements<I: Interface>: InterfaceList {
    const SLOT: usize;                            // 0 = identity, 1.. = declared
}

pub trait ListVtables<T: 'static>: InterfaceList {
    const STORAGE: Self::Storage;
}
```

Tuple impls for `()`, `(I0,)`, `(I0, I1)`, … cover the full user-facing
syntax. Maximum supported arity is **16** (OQ-1); explicit `VCons` form is
documented for users who need more. `InterfaceList` is sealed so it can be
extended without breaking users. `Implements<I>` gates the blanket
`From<T> for I` (OQ-5).

### `VtableCtor` — the one bindgen edit

The macro today calls `IFoo_Vtbl::new::<Foo_Impl, -2 - k>()`. The
foundation needs to do the same call generically over `I::Vtable`, so
each `_Vtbl` opts into a trait:

```rust,ignore
pub trait VtableCtor<T, const OFFSET: isize>: Sized + 'static {
    const NEW: Self;
    const NEW_REF: &'static Self;
}

impl<T: ::windows_core::IFoo_Impl + 'static, const OFFSET: isize>
    ::windows_core::imp::VtableCtor<T, OFFSET> for IFoo_Vtbl
{
    const NEW: Self = <Self>::new::<T, OFFSET>();
    const NEW_REF: &'static Self = &<Self as ::windows_core::imp::VtableCtor<T, OFFSET>>::NEW;
}
```

This three-line block is what `windows-interface` emits alongside each
`_Vtbl` in Step 2 (OQ-2). Two stable-Rust constraints, validated by the
spike, dictate the shape:

* **`const fn` in trait position is unstable**, so the trait exposes an
  associated `const NEW: Self` rather than `const fn new() -> Self`.
* **`&<V as VtableCtor<T, O>>::NEW` does not promote when `V` is generic**
  (rustc rejects with `E0492`: interior mutability not provable). So the
  trait also exposes `NEW_REF: &'static Self`, materialised inside each
  per-`_Vtbl` impl where `Self` is concrete and the borrow does promote.
  Generic call sites in `ListVtables::STORAGE` read `NEW_REF` directly.

`IInspectable_Vtbl::new` takes an extra `FirstInterface: RuntimeName`
parameter; its `VtableCtor` impl pins that to `L::FirstInterface` (an
associated type on `InterfaceList`, defaulting to `IInspectable`).

### `Implemented`, agility, knobs

```rust,ignore
pub trait Implemented: Sized + 'static {
    type Interfaces: InterfaceList;
    type Agility: Agility;
    const TRUST: u8 = 0;
    const DYNAMIC_CAST: bool = true;   // forced false for non-'static lifetimes
}

pub trait Agility: sealed::Sealed {
    const IS_AGILE: bool;
    const HAS_MARSHAL: bool;
}
pub struct Agile;     // IS_AGILE = true,  HAS_MARSHAL = cfg(windows)
pub struct NonAgile;  // IS_AGILE = false, HAS_MARSHAL = false
```

Sealed marker types over const-generic bools (OQ-3): they compose in
`where`-clauses, and a future third level (e.g. STA-only) is purely additive.

### Blanket impls — the substance of `gen.rs`

The bulk of `gen.rs` becomes blanket impls on `Outer<T, L>`:

| Today (`gen.rs`)                  | Tomorrow (blanket impl on `Outer<T, L>`)               |
|-----------------------------------|--------------------------------------------------------|
| `gen_impl_struct`                 | `Outer` definition (single struct)                     |
| `gen_impl_deref`                  | `impl Deref for Outer<T, L>`                           |
| `gen_impl_impl` (vtable consts)   | `ListVtables` per-tuple-arity blanket impls            |
| `gen_iunknown_impl`               | `impl IUnknownImpl for Outer<T, L>`                    |
| `gen_query_interface`             | a single generic `QueryInterface` method               |
| `gen_impl_com_object_inner`       | `impl ComObjectInner for T where T: Implemented`       |
| `gen_impl_compose`                | `impl Compose for T where T: Implemented`              |
| `gen_impl_from` (per interface)   | `impl<I> From<T> for I where L: Implements<I>`         |
| `gen_impl_com_object_interfaces`  | `impl<I> ComObjectInterface<I> for Outer<...>` ditto   |
| `gen_impl_as_impl`                | `impl<I> AsImpl<T> for I` ditto                        |

#### Generic `QueryInterface`

```rust,ignore
impl<T: Implemented> IUnknownImpl for Outer<T, T::Interfaces>
where
    T::Interfaces: ListVtables<Outer<T, T::Interfaces>>,
{
    type Impl = T;

    unsafe fn QueryInterface(&self, iid: *const GUID, interface: *mut *mut c_void) -> HRESULT {
        if iid.is_null() || interface.is_null() { return imp::E_POINTER; }
        let iid = unsafe { *iid };

        // Identity: IUnknown, IInspectable, (and IAgileObject if agile) → &self.identity.
        if iid == IUnknown::IID
            || iid == IInspectable::IID
            || (<T::Agility as Agility>::IS_AGILE && iid == imp::IAgileObject::IID)
        {
            return finish(self, &self.identity as *const _ as *const c_void, interface);
        }

        // Declared interfaces: scan IID_SLOTS, address arithmetic on `identity`.
        for &(slot_iid, slot) in <T::Interfaces as InterfaceList>::IID_SLOTS {
            if iid == slot_iid {
                let base = &self.identity as *const _ as *const *const c_void;
                let p = unsafe { base.add(slot) };
                return finish(self, p as *const c_void, interface);
            }
        }

        // IMarshal (agile, windows-only); DYNAMIC_CAST; tear-off; aggregation fall-through.
        #[cfg(windows)]
        if <T::Agility as Agility>::HAS_MARSHAL && iid == imp::IMarshal::IID {
            return imp::marshaler(self.to_interface::<IUnknown>(), interface);
        }
        if T::DYNAMIC_CAST && iid == DYNAMIC_CAST_IID { /* write &dyn Any */ return HRESULT(0); }
        let tear = self.count.query(&iid, &self.identity as *const _ as *mut _);
        if !tear.is_null() { unsafe { *interface = tear; } return HRESULT(0); }
        if let Some(base) = self.base.as_option() {
            return unsafe { Interface::query(base, &iid, interface) };
        }
        unsafe { *interface = core::ptr::null_mut(); }
        imp::E_NOINTERFACE
    }
}
```

Start with the loop, benchmark in Step 0, specialise per-arity only if a
regression appears (OQ-4). `IID_SLOTS` is a `'static` table of small fixed
length; LLVM's unroller is expected to flatten it after inlining.

#### Blanket conversions

```rust,ignore
impl<T: Implemented, I: Interface> From<T> for I
where T::Interfaces: Implements<I>
{
    fn from(this: T) -> Self { ComObject::new(this).into_interface() }
}

impl<T: Implemented, I: Interface> ComObjectInterface<I> for Outer<T, T::Interfaces>
where T::Interfaces: Implements<I>
{
    fn as_interface_ref(&self) -> InterfaceRef<'_, I> {
        let base = &self.identity as *const _ as *const *const c_void;
        let p = unsafe { base.add(<T::Interfaces as Implements<I>>::SLOT) };
        unsafe { core::mem::transmute(p) }
    }
}

impl<T: Implemented, I: Interface> AsImpl<T> for I
where T::Interfaces: Implements<I>
{
    unsafe fn as_impl_ptr(&self) -> NonNull<T> {
        let this = Interface::as_raw(self);
        let outer = (this as *mut *mut c_void)
            .sub(1 + <T::Interfaces as Implements<I>>::SLOT)
            as *mut Outer<T, T::Interfaces>;
        NonNull::new_unchecked(core::ptr::addr_of!((*outer).this) as *mut T)
    }
}
```

The `1 + SLOT` accounts for the leading `base` field; today's macro carries
this as `2 +` because it counts from the first vtable slot. Identity
(`IUnknown`/`IInspectable`) gets explicit non-blanket impls because
`Implements<IUnknown>` is not (and should not be) automatic.

### Construction

```rust,ignore
impl<T: Implemented> Outer<T, T::Interfaces> {
    pub const fn new(value: T) -> Self where T: NonGeneric { /* const */ }
    pub fn new_generic(value: T) -> Self { /* non-const */ }
}
```

Two named ctors (OQ-6): `const fn` cannot be conditionally `const` on a
trait bound, so callers in generic contexts would get confusing const-eval
errors. The two-ctor split maps 1:1 to today's macro `is_generic` flag.
`ComObjectInner::into_object` calls `new_generic` so `ComObject::new(value)`
keeps working for both.

### What the proc macro emits (Step 2)

```rust,ignore
quote! {
    #original_struct

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

    #into_static_forwarder   // only for non-generic T (OQ-7)
}
```

`into_static` stays as a two-line inherent `fn on Foo` because Rust doesn't
let blanket impls add inherent methods to a foreign type (OQ-7). The
`Foo_Impl` alias resolves through `Outer<...>`, so `impl IFoo_Impl for
Foo_Impl` blocks the user wrote keep compiling.

### What `implement_decl!` emits

```rust,ignore
macro_rules! implement_decl {
    (impl $T:ty as $vis:vis $Alias:ident : [ $($I:ty),+ $(,)? ]) => {
        $vis type $Alias = $crate::Outer<$T, ( $($I,)+ )>;
        impl $crate::Implemented for $T {
            type Interfaces = ( $($I,)+ );
            type Agility = $crate::Agile;
        }
    };
}
```

No accumulator, no `'found` label, no `self`/`iid` references, no
const-promotion gymnastics. Earlier attempts hit TT-muncher hygiene
problems because they were emitting `QueryInterface` arms; with the
foundation none of that lives in the macro any more. A
`implement_decl_generic!` sibling covers generic `T`.

### Hand-written use (zero macros)

```rust,ignore
use windows_core::*;

struct Foo(i32);
pub type Foo_Impl = Outer<Foo, (IValue,)>;

impl Implemented for Foo {
    type Interfaces = (IValue,);
    type Agility = Agile;
}

impl IValue_Impl for Foo_Impl {
    unsafe fn GetValue(&self, out: *mut i32) -> HRESULT { *out = self.0; HRESULT(0) }
}

let v: IValue = Foo(42).into();
```

## Migration plan

### Step 0 — Spike

Both phases have landed at
[`crates/tests/libs/implement_foundation_spike`](../crates/tests/libs/implement_foundation_spike).

* **Phase 1** — defines `Outer`/`InterfaceList`/`Implemented`/`VtableCtor`/
  HList cells with tuple impls 0..=4, adds opt-ins for `IUnknown_Vtbl`/
  `IInspectable_Vtbl`/a fake `IValue_Vtbl`, and asserts byte-identical
  layout (`size_of` + `align_of` + per-field `offset_of!`) between
  `Outer<Foo, (IValue,)>` and the parallel `#[implement(IValue)] Foo`'s
  `Foo_Impl`. Builds on stable Rust 1.82.

* **Phase 2** — wires the `IUnknownImpl` blanket on `Outer<T, L>` with the
  generic `QueryInterface` body, the `Outer::new_generic` ctor, identity
  `ComObjectInterface<IUnknown/IInspectable>` impls, and `Deref` to `T`.
  Per-type `ComObjectInner` / `From<Foo> for I` / `ComObjectInterface<IValue>`
  impls live in `sample.rs` (orphan rules prevent the production blankets in
  this crate; their shape is identical to what `windows-core` would emit).
  Runtime tests cover construct, AddRef/Release, QI(IUnknown), QI(IInspectable),
  QI(IValue) including a method-dispatch round-trip, QI(unknown) →
  `E_NOINTERFACE`, and `Foo.into() → IUnknown → .cast::<IValue>()`.

* **OQ-4 microbenchmark** (`tests/bench_qi.rs`, 5M iters each) on a release
  build, debug-instrumented release (no LTO):

  | path                         | foundation | macro    | delta    |
  |------------------------------|-----------:|---------:|---------:|
  | QI(IUnknown)  — identity     |  7.11 ns   |  8.53 ns | −1.42 ns |
  | QI(IValue)    — declared     | 10.47 ns   | 14.24 ns | −3.77 ns |
  | QI(unknown)   — fall-through |  6.64 ns   | 12.01 ns | −5.36 ns |

  The foundation matches or beats the macro on all three paths. OQ-4 bet
  (don't pre-emptively unroll, benchmark first) is vindicated — the
  `IID_SLOTS` loop inlines well, and the macro's open-coded if-chain is
  actually *worse* on the unknown-IID path because it falls through more
  branches. A second pass with `lto = "thin"` is left for Step 1, when the
  foundation lives in `windows-core` proper.

### Step 1 — Foundation only, macro untouched

* New `windows_core::imp::implement` module with the foundation.
* `Outer`/`Implemented`/`Implements`/`InterfaceList`/`Agile`/`NonAgile`
  re-exported `#[doc(hidden)]` initially.
* `VtableCtor` emitted by `windows-interface` (one impl per `_Vtbl`).
* New `crates/tests/libs/implement_foundation` tests for hand-written use.
* `crates/libs/implement` untouched; existing implement test suite still
  passes unchanged.

### Step 2 — Reskin `#[implement]` as a shim

* Delete `gen.rs` (or shrink to ~30 lines of emission).
* Existing implement test suite is the regression gate.
* Mitigations: snapshot before/after with `cargo expand`; diff the assembly
  of one representative `Foo_Impl::QueryInterface` for the identity fast
  path.

### Step 3 — Land `implement_decl!`

Net-new (the macro does not exist in this branch). Two `macro_rules!`
definitions under `crates/libs/core/src/implement_macro.rs`, tests under
`crates/tests/libs/implement_decl`.

### Step 4 — Document the foundation as public API

Drop `#[doc(hidden)]` from the re-exports; add an "Implementing COM by
hand" doc; mention in the `windows-core` readme.

### Step 5 — Optional follow-ups

Specialise the per-arity tuple impl if Step 0 shows a regression. Evaluate
whether `windows-implement` can collapse to a re-export plus the attribute
parser.

## Compatibility

* `Foo_Impl` is still a nameable type (now an alias instead of a fresh
  struct); `impl IFoo_Impl for Foo_Impl` still resolves.
* `ComObject<Foo>`, `AsImpl<Foo>`, `From<Foo> for IFoo`, `Deref` from
  `Foo_Impl` to `Foo`: all preserved.
* Field order, in-memory layout, vtable offsets: byte-for-byte identical.
* `IUnknownImpl`, `ComObjectInner`, `Compose`, `ComObjectInterface`,
  `AsImpl`, `Interface`, `ComObject`: signatures unchanged.

The only observable break is the orphan-rules edge case noted in OQ-5
(simultaneous `#[implement]` and hand-written `impl Implemented`), which
no sensible user program contains.

## Resolved decisions

The "Why not the alternative" column captures the load-bearing reason; not
an exhaustive trade-off list.

| #    | Question                                                | **Decision**                                                              | Why not the alternative                                                                            |
|------|---------------------------------------------------------|---------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------|
| OQ-1 | Maximum supported tuple arity                           | **16**, with explicit `VCons` form for anything larger                    | Lower forces most use cases through HList syntax; higher is a search-and-replace away              |
| OQ-2 | Where does `VtableCtor` get emitted                     | **`windows-interface` emits one extra impl per `_Vtbl`**                  | Per-slot const in macros duplicates ABI knowledge across proc macro + `macro_rules!` shim          |
| OQ-3 | Knob representation                                     | **Sealed marker types** (`Agile` / `NonAgile`)                            | Const generics can't carry future knobs (e.g. STA-only) without widening every const               |
| OQ-4 | `QueryInterface` loop vs. unrolled match                | **Loop**; Step 0 microbenchmark decides whether to specialise per-arity   | Unconditional unrolled match adds monomorphisation cost we don't yet need                          |
| OQ-5 | `From<T> for I` blanket vs. coexisting macro emission   | **Blanket gated by `Implements<I>`**, macro stops emitting per-iface From | Keeping macro emission would force `implement_decl!` to re-emit too                                |
| OQ-6 | `const fn` constructor vs. two ctors                    | **Two named ctors** (`Outer::new`, `Outer::new_generic`)                  | One conditionally-`const` ctor produces confusing errors in generic contexts                       |
| OQ-7 | How does `Foo::into_static()` stay callable             | **Macro emits a 2-line forwarder on `Foo`**                               | Blanket on `T: Implemented` cannot add inherent methods to a foreign type                          |
