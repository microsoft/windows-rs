//! Phase 2 runtime wiring on top of phase 1's types.
//!
//! See `docs/option-d.md` ("Generic `QueryInterface`" + "Blanket conversions"
//! + "Construction"). This file proves the production blanket impls compile
//! on stable Rust and behave correctly at runtime for a single
//! `Outer<Foo, (IValue,)>` case. The shape is what `windows-core` will host
//! in Step 1; the spike emits it here without modifying `windows-core` so
//! the design is reviewable in isolation.
//!
//! What lives here:
//!
//! * `ComObjectInner` blanket for `T: Implemented` — wires `ComObject::new(t)`.
//! * `Outer::new_generic` — the `non-const` ctor (OQ-6).
//! * `IUnknownImpl` blanket on `Outer<T, L>` with the generic `QueryInterface`
//!   body (identity short-circuit; `IID_SLOTS` loop; tear-off + aggregation
//!   fall-through; `IMarshal` gated on `cfg(windows) && agile`; `DYNAMIC_CAST`).
//! * `core::ops::Deref` blanket — `Outer<T, L>` deref's to `T`.
//! * `ComObjectInterface<I>` blanket gated on `Implements<I>`, plus explicit
//!   identity impls for `IUnknown` / `IInspectable` (the identity slot is
//!   not in `IID_SLOTS`).
//! * `From<T> for I` blanket gated on `Implements<I>`, plus identity
//!   `From<T>` for `IUnknown` / `IInspectable`.

use crate::foundation::agility::Agility;
use crate::foundation::list::{InterfaceList, ListVtables};
use crate::foundation::outer::{Implemented, Outer};
use crate::foundation::vtbl::VtableCtor;
use core::ffi::c_void;
use core::ops::Deref;
use core::ptr::NonNull;
use windows_core::imp::WeakRefCount;
use windows_core::{
    imp, ComObject, ComObjectInner, ComObjectInterface, ComposeBase, IInspectable,
    IInspectable_Vtbl, IUnknown, IUnknownImpl, Interface, InterfaceRef, HRESULT,
};

// =============================================================================
// `ComObjectInner` blanket — orphan rules
// =============================================================================
//
// The production design has a fully-blanket `impl<T: Implemented>
// ComObjectInner for T` inside `windows-core`. The spike *cannot* host that
// blanket here: `ComObjectInner` is foreign and `T` is generic, which
// violates orphan rules (E0210).
//
// For the spike's runtime test, `sample.rs` writes a per-type impl for `Foo`
// using exactly the shape that the production blanket would emit. The
// per-type form is also what `implement_decl!` would expand to for a
// zero-proc-macro user.

// =============================================================================
// `Outer::new_generic` — the non-const ctor (OQ-6)
// =============================================================================
//
// The construction bound `IInspectable_Vtbl: VtableCtor<Self, -1>` is
// scoped to this inherent impl, NOT to the `IUnknownImpl` blanket below.
// Putting it on `IUnknownImpl` creates an inference cycle: rustc would try
// `Outer<T,L>: IUnknownImpl` ⇒ `IInspectable_Vtbl: VtableCtor<Outer<T,L>, -1>`
// (impl bound) ⇒ `Outer<T,L>: IUnknownImpl` (T: IUnknownImpl bound on the
// generic `VtableCtor` impl) ⇒ overflow.
//
// Today's macro avoids the cycle by emitting `Foo_Impl: IUnknownImpl` and
// `Foo_Impl::VTABLE_IDENTITY` as two unrelated items in two unrelated
// `impl` blocks. We mirror that split here.

impl<T, L> Outer<T, L>
where
    T: Implemented<Interfaces = L>,
    L: InterfaceList + ListVtables<Outer<T, L>>,
    IInspectable_Vtbl: VtableCtor<Outer<T, L>, -1>,
{
    /// Non-const constructor (OQ-6).
    pub fn new_generic(value: T) -> Self {
        Self {
            base: ComposeBase::new(),
            identity: <IInspectable_Vtbl as VtableCtor<Outer<T, L>, -1>>::NEW_REF,
            vtables: <L as ListVtables<Outer<T, L>>>::STORAGE,
            this: value,
            count: WeakRefCount::new(),
        }
    }
}

// =============================================================================
// `Deref` — so `Outer<T, L>` behaves like `&T` for user-trait impls
// =============================================================================
//
// Today's macro emits `impl Deref for Foo_Impl { type Target = Foo; ... }`.
// The user's `impl IFoo_Impl for Foo_Impl` body therefore reaches `Foo`'s
// fields directly via `self.0`. We preserve that behaviour.

impl<T, L> Deref for Outer<T, L>
where
    T: Implemented<Interfaces = L>,
    L: InterfaceList,
{
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        &self.this
    }
}

// =============================================================================
// `IUnknownImpl` — the generic `QueryInterface`
// =============================================================================
//
// Mirrors `crates/libs/implement/src/gen.rs:gen_iunknown_impl` /
// `gen_query_interface`. Differences vs the macro emission:
//
//   * Declared interfaces are scanned via `IID_SLOTS` (a `'static` table)
//     rather than emitted as a sequence of `if` arms. LLVM unrolls the
//     loop; OQ-4 benchmarks the difference.
//   * Identity (`IUnknown`/`IInspectable`/`IAgileObject` if agile) is the
//     same short-circuit-against-`&self.identity`.
//   * `IMarshal` is `#[cfg(windows)]`-gated and only when `HAS_MARSHAL`.
//   * Aggregation fall-through and tear-off behaviour are identical.

impl<T, L> IUnknownImpl for Outer<T, L>
where
    T: Implemented<Interfaces = L>,
    L: InterfaceList,
{
    type Impl = T;

    #[inline(always)]
    fn get_impl(&self) -> &T {
        &self.this
    }

    #[inline(always)]
    fn get_impl_mut(&mut self) -> &mut T {
        &mut self.this
    }

    fn into_inner(self) -> T {
        self.this
    }

    #[inline(always)]
    fn AddRef(&self) -> u32 {
        self.count.add_ref()
    }

    /// # Safety
    /// The pointer must originate from a heap-allocated `Outer<T, L>` whose
    /// reference count is > 0. After this call returns, `self_` may be
    /// dangling.
    unsafe fn Release(self_: *mut Self) -> u32 {
        let remaining = unsafe { (*self_).count.release() };
        if remaining == 0 {
            unsafe {
                // SAFETY: ref count just dropped to zero, which means this
                // is the unique pointer; the box is ours to drop.
                drop(imp::Box::from_raw(self_));
            }
        }
        remaining
    }

    #[inline(always)]
    fn is_reference_count_one(&self) -> bool {
        self.count.is_one()
    }

    unsafe fn GetTrustLevel(&self, value: *mut i32) -> HRESULT {
        if value.is_null() {
            return imp::E_POINTER;
        }
        unsafe { *value = <T as Implemented>::TRUST as i32 };
        HRESULT(0)
    }

    fn to_object(&self) -> ComObject<T>
    where
        T: ComObjectInner<Outer = Self>,
    {
        // AddRef the box; reconstruct a ComObject pointing at the same
        // outer. Same shape as the default `to_object` implementation today.
        self.count.add_ref();
        unsafe { ComObject::from_raw(NonNull::new_unchecked(self as *const _ as *mut Self)) }
    }

    unsafe fn QueryInterface(
        &self,
        iid: *const windows_core::GUID,
        interface: *mut *mut c_void,
    ) -> HRESULT {
        unsafe {
            if iid.is_null() || interface.is_null() {
                return imp::E_POINTER;
            }
            let iid = *iid;

            // Computed once; reused by both the identity branch and the
            // IID_SLOTS scan. This is the address of the `identity` field,
            // which is also the address that COM callers expect to receive
            // when they QueryInterface for IUnknown/IInspectable.
            let identity_ptr = &self.identity as *const _ as *const c_void;

            // Identity short-circuit. Three IIDs (or two when non-agile).
            if iid == IUnknown::IID
                || iid == IInspectable::IID
                || (<T::Agility as Agility>::IS_AGILE && iid == imp::IAgileObject::IID)
            {
                *interface = identity_ptr as *mut c_void;
                self.count.add_ref();
                return HRESULT(0);
            }

            // Declared interfaces: scan IID_SLOTS. Each entry is
            // `(IID, slot)` where slot 1 = first declared, etc. The
            // matching pointer lives at `&self.identity + slot` pointer-units.
            //
            // OQ-4: production may specialise per-arity if this loop is
            // measurably slower than today's open-coded match. Spike
            // benchmark (see tests/bench_qi.rs) is the gate.
            let base = &self.identity as *const _ as *const *const c_void;
            for &(slot_iid, slot) in <L as InterfaceList>::IID_SLOTS {
                if iid == slot_iid {
                    // `base.add(slot)` is the address of the vtable-pointer
                    // *slot*, which is what COM expects (callers dereference
                    // it once to reach the vtable). `*base.add(slot)` would
                    // be the vtable pointer itself — wrong.
                    *interface = base.add(slot) as *mut c_void;
                    self.count.add_ref();
                    return HRESULT(0);
                }
            }

            // IMarshal (windows + agile).
            #[cfg(windows)]
            if <T::Agility as Agility>::HAS_MARSHAL && iid == imp::IMarshal::IID {
                // The default agile marshaler. The macro's emission uses
                // `self.to_interface()` which goes through `ComObjectInterface`.
                // We do the same — `Outer<T, L>: ComObjectInterface<IUnknown>`
                // is provided below.
                return imp::marshaler(
                    <Self as ComObjectInterface<IUnknown>>::as_interface_ref(self).to_owned(),
                    interface,
                );
            }

            // DYNAMIC_CAST: write `&dyn Any` directly. Caller is
            // `Interface::cast_to_any`, which knows the size convention.
            if <T as Implemented>::DYNAMIC_CAST && iid == windows_core::DYNAMIC_CAST_IID {
                (interface as *mut *const dyn core::any::Any)
                    .write(self as &dyn core::any::Any as *const dyn core::any::Any);
                return HRESULT(0);
            }

            // Tear-off (IWeakReferenceSource via WeakRefCount::query).
            let tear_off = self
                .count
                .query(&iid, &self.identity as *const _ as *mut c_void);
            if !tear_off.is_null() {
                *interface = tear_off;
                return HRESULT(0);
            }

            // Aggregation fall-through.
            if let Some(base_iface) = self.base.as_option() {
                return Interface::query(base_iface, &iid, interface);
            }

            *interface = core::ptr::null_mut();
            imp::E_NOINTERFACE
        }
    }
}

// =============================================================================
// `ComObjectInterface<I>` — identity impls
// =============================================================================
//
// `IUnknown` and `IInspectable` resolve to `&self.identity` for every
// `Outer<T, L>` (the identity slot is shared by both). The
// declared-interface `ComObjectInterface<I> for Outer<T, L> where L:
// Implements<I>` blanket lives in `windows-core` in production; the spike
// emits a per-type version in `sample.rs` because the blanket would need
// `I: !{IUnknown, IInspectable}` discrimination, which Rust does not
// natively express. Production resolves this via a sealed `DeclaredInterface`
// marker; the spike skips that path because per-type emission is enough to
// exercise the runtime contract.

impl<T, L> ComObjectInterface<IUnknown> for Outer<T, L>
where
    T: Implemented<Interfaces = L>,
    L: InterfaceList,
{
    #[inline(always)]
    fn as_interface_ref(&self) -> InterfaceRef<'_, IUnknown> {
        unsafe { core::mem::transmute(&self.identity) }
    }
}

impl<T, L> ComObjectInterface<IInspectable> for Outer<T, L>
where
    T: Implemented<Interfaces = L>,
    L: InterfaceList,
{
    #[inline(always)]
    fn as_interface_ref(&self) -> InterfaceRef<'_, IInspectable> {
        unsafe { core::mem::transmute(&self.identity) }
    }
}

// =============================================================================
// `From<T> for I` — orphan rules
// =============================================================================
//
// Same situation as `ComObjectInner`: the production blanket lives in
// `windows-core` and is gated on `T::Interfaces: Implements<I>`. The spike
// emits per-type `From<Foo> for IUnknown` / `From<Foo> for IInspectable` /
// `From<Foo> for IValue` in `sample.rs`.
