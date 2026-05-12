//! Runtime wiring on top of `Outer<T, L>` — the blanket impls that replace
//! the proc macro's per-use-site emissions for everything except the small
//! `Implemented` declaration itself and the per-declared-interface
//! `From<T> for IFoo` / `ComObjectInterface<IFoo>` / `AsImpl<T> for IFoo`
//! shims (which orphan rules force to stay per-interface).
//!
//! What lives here:
//!
//! * [`ComObjectInner`] blanket for `T: Implemented` — wires
//!   `ComObject::new(t)`.
//! * [`Outer::new_generic`] — the non-const ctor (OQ-6).
//! * [`IUnknownImpl`] blanket on `Outer<T, L>` with the generic
//!   `QueryInterface` body (identity short-circuit; `IID_SLOTS` loop;
//!   tear-off + aggregation fall-through; `IMarshal` gated on `cfg(windows)`
//!   and agile; `DYNAMIC_CAST`).
//! * `core::ops::Deref<Target = T>` blanket.
//! * Identity `ComObjectInterface<IUnknown>` / `<IInspectable>` blankets.
//! * Identity `From<T> for IUnknown` / `for IInspectable` blankets gated on
//!   `T: Implemented`.
//!
//! No coherence collision with today's `#[implement]`-generated code:
//! macro-generated `Foo` types do not implement `Implemented`, so the
//! blankets here only fire for hand-written-foundation users (and, in
//! Step 2, for the macro's reskinned emission).

use super::agility::Agility;
use super::list::{Implements, InterfaceList, ListVtables};
use super::outer::{Implemented, Outer};
use super::vtbl::VtableCtor;
use crate::imp::WeakRefCount;
use crate::{
    imp, ComObject, ComObjectInner, ComObjectInterface, ComposeBase, IInspectable,
    IInspectable_Vtbl, IUnknown, IUnknownImpl, Interface, InterfaceRef, GUID, HRESULT,
};
use core::ffi::c_void;
use core::ops::Deref;
use core::ptr::NonNull;

// =============================================================================
// `Outer::new_generic` — the non-const ctor (OQ-6)
// =============================================================================
//
// Construction requires `IInspectable_Vtbl: VtableCtor<Self, -1>` plus
// `L: ListVtables<Self>`. Both bounds live on this inherent impl rather
// than on `IUnknownImpl` below to avoid an inference cycle: putting
// `IInspectable_Vtbl: VtableCtor<Outer<T,L>, -1>` on `IUnknownImpl` would
// trigger `T: IUnknownImpl` (the bound on the generic VtableCtor impl) ⇒
// `Outer<T,L>: IUnknownImpl` ⇒ overflow. Today's macro avoids the same
// cycle by emitting `Foo_Impl: IUnknownImpl` and `Foo_Impl::VTABLE_IDENTITY`
// in two unrelated `impl` blocks.

impl<T, L> Outer<T, L>
where
    T: Implemented<Interfaces = L>,
    L: InterfaceList + ListVtables<Outer<T, L>>,
    IInspectable_Vtbl: VtableCtor<Outer<T, L>, -1>,
{
    /// Constructs an `Outer<T, L>` from `T`. Non-const (OQ-6).
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
// `Outer::as_declared_interface` — slot-pointer accessor for downstream
// `ComObjectInterface<I>` implementations.
// =============================================================================
//
// Per-declared-interface `ComObjectInterface<I> for Outer<T, L>` impls
// cannot be expressed as a generic blanket inside windows-core (would
// collide with the identity `<IUnknown>` / `<IInspectable>` impls below).
// The downstream emission therefore needs a way to get an `InterfaceRef<I>`
// for slot `Implements<I>::SLOT` *without* poking at the private
// `Outer.identity` field. This helper provides exactly that.

impl<T, L> Outer<T, L>
where
    T: Implemented<Interfaces = L>,
    L: InterfaceList,
{
    /// Returns an `InterfaceRef<I>` pointing at the `I`-vtable slot for
    /// this `Outer`. Bound by `L: Implements<I>` so only declared
    /// interfaces are reachable.
    ///
    /// `IUnknown` and `IInspectable` go through the dedicated identity
    /// `ComObjectInterface` impls below — `Implements<IUnknown>` /
    /// `Implements<IInspectable>` are deliberately not provided.
    #[inline(always)]
    pub fn as_declared_interface<I: Interface>(&self) -> InterfaceRef<'_, I>
    where
        L: Implements<I>,
    {
        let base = &self.identity as *const _ as *const *const c_void;
        let slot = <L as Implements<I>>::SLOT;
        unsafe { core::mem::transmute(base.add(slot)) }
    }
}

// =============================================================================
// `ComObjectInner` — wires `ComObject::new(t)`
// =============================================================================

impl<T> ComObjectInner for T
where
    T: Implemented,
    T::Interfaces: ListVtables<Outer<T, T::Interfaces>>,
    IInspectable_Vtbl: VtableCtor<Outer<T, T::Interfaces>, -1>,
{
    type Outer = Outer<T, T::Interfaces>;

    fn into_object(self) -> ComObject<Self> {
        let boxed = imp::Box::<Outer<T, T::Interfaces>>::new(Outer::new_generic(self));
        unsafe {
            let ptr = imp::Box::into_raw(boxed);
            ComObject::from_raw(NonNull::new_unchecked(ptr))
        }
    }
}

// =============================================================================
// `Deref` — `Outer<T, L>` deref's to `T` so user-trait method bodies reach
// user fields like `self.x` exactly the way today's macro arranges.
// =============================================================================

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
// `IUnknownImpl` — generic `QueryInterface`
// =============================================================================
//
// Mirrors `crates/libs/implement/src/gen.rs:gen_query_interface`. Differences
// vs the macro emission:
//
//   * Declared interfaces are scanned via `IID_SLOTS` (a `'static` table)
//     rather than emitted as a sequence of `if` arms. The Step 0 microbench
//     (OQ-4) shows the loop matches or beats the if-chain after inlining.
//   * `IMarshal` is `#[cfg(windows)]`-gated and only fires when the type is
//     agile; otherwise the branch is a compile-time no-op.

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
    /// `self_` must originate from a heap-allocated `Outer<T, L>` whose
    /// reference count is > 0. After this call returns, `self_` may be
    /// dangling.
    unsafe fn Release(self_: *mut Self) -> u32 {
        let remaining = unsafe { (*self_).count.release() };
        if remaining == 0 {
            unsafe {
                // Ref count just dropped to zero, so this is the unique
                // pointer; the box is ours to drop.
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
        self.count.add_ref();
        unsafe { ComObject::from_raw(NonNull::new_unchecked(self as *const _ as *mut Self)) }
    }

    unsafe fn QueryInterface(&self, iid: *const GUID, interface: *mut *mut c_void) -> HRESULT {
        unsafe {
            if iid.is_null() || interface.is_null() {
                return imp::E_POINTER;
            }
            let iid = *iid;

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

            // Declared interfaces — scan IID_SLOTS. Each entry is
            // `(IID, slot)` where slot 1 = first declared. The matching
            // pointer lives at `&self.identity + slot` pointer-units.
            let base = &self.identity as *const _ as *const *const c_void;
            for &(slot_iid, slot) in <L as InterfaceList>::IID_SLOTS {
                if iid == slot_iid {
                    // `base.add(slot)` is the address of the vtable-pointer
                    // *slot* — the COM caller dereferences it once to reach
                    // the vtable. `*base.add(slot)` would be the vtable
                    // pointer itself, which is wrong.
                    *interface = base.add(slot) as *mut c_void;
                    self.count.add_ref();
                    return HRESULT(0);
                }
            }

            // IMarshal — windows + agile.
            #[cfg(windows)]
            if <T::Agility as Agility>::HAS_MARSHAL && iid == imp::IMarshal::IID {
                return imp::marshaler(
                    <Self as ComObjectInterface<IUnknown>>::as_interface_ref(self).to_owned(),
                    interface,
                );
            }

            // DYNAMIC_CAST — write `&dyn Any` directly. Caller is
            // `Interface::cast_to_any`, which knows the size convention.
            if <T as Implemented>::DYNAMIC_CAST && iid == crate::DYNAMIC_CAST_IID {
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
// `Outer<T, L>`. Per-declared-interface impls stay in user code (or in the
// reskinned macro emission in Step 2), because a generic blanket
// `impl<I> ComObjectInterface<I> for Outer<T,L> where L: Implements<I>`
// would collide with these two identity impls.

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
// `From<T> for IUnknown` / `IInspectable` — identity blankets
// =============================================================================
//
// Per-declared-interface `From<T> for IFoo` cannot be a blanket here:
// `impl<T,I> From<T> for I` violates orphan rules even inside windows-core
// (both `T` and `I` are unconstrained type parameters; the trait `From` is
// foreign; no concrete local type appears in the impl head). The two
// identity conversions below are possible because `IUnknown` and
// `IInspectable` are concrete local types. Per-declared-interface impls
// stay user-emitted (or, in Step 2, reskinned-macro-emitted).

impl<T> From<T> for IUnknown
where
    T: Implemented,
    T::Interfaces: ListVtables<Outer<T, T::Interfaces>>,
    IInspectable_Vtbl: VtableCtor<Outer<T, T::Interfaces>, -1>,
{
    #[inline(always)]
    fn from(this: T) -> Self {
        ComObject::new(this).into_interface()
    }
}

impl<T> From<T> for IInspectable
where
    T: Implemented,
    T::Interfaces: ListVtables<Outer<T, T::Interfaces>>,
    IInspectable_Vtbl: VtableCtor<Outer<T, T::Interfaces>, -1>,
{
    #[inline(always)]
    fn from(this: T) -> Self {
        ComObject::new(this).into_interface()
    }
}
