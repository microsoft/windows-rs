//! A `macro_rules!` declarative alternative to the `#[implement]` proc-macro.
//!
//! `#[implement]` is the canonical way to wire up a Rust type as the implementer of one or
//! more COM interfaces, but it pulls in `syn`, `quote`, and `proc-macro2`. Consumers who
//! disable the `proc-macros` default feature on `windows-core` still need a way to do this;
//! [`implement_decl!`] fills that role.
//!
//! ## Scope
//!
//! `implement_decl!` targets the dominant hand-written case: an **always-agile** Rust type
//! (generic or non-generic) that implements **one or more** COM interfaces (each declared
//! either by [`crate::interface_decl!`] or by `#[interface]`). It does not support:
//!
//! - per-instance `trust_level` configuration (always `0` / Base),
//! - per-instance opt-out of agility,
//! - per-instance opt-out of dynamic casting.
//!
//! It **does** support COM aggregation (a `Compose` impl is emitted unconditionally), the
//! IMarshal tear-off, the dynamic-cast pseudo-IID, and the agile-object IID.
//!
//! ## Syntax
//!
//! ```rust,ignore
//! use windows_core::*;
//!
//! interface_decl! {
//!     pub unsafe trait IFoo(IFoo_Vtbl, IFoo_Impl) : IUnknown
//!         = 0x094d70d6_5202_44b8_abb8_43860da5aca2
//!     {
//!         unsafe fn Method(&self) -> HRESULT;
//!     }
//! }
//!
//! pub struct Foo;
//!
//! implement_decl! {
//!     impl Foo as pub Foo_Impl: [IFoo]
//! }
//!
//! impl IFoo_Impl for Foo_Impl {
//!     unsafe fn Method(&self) -> HRESULT { HRESULT(0) }
//! }
//! ```
//!
//! - `Foo` is the user-defined implementer type — declared **separately**, before invoking
//!   the macro.
//! - `Foo_Impl` is the wrapper that the macro defines. Visibility (`pub`, `pub(crate)`, …)
//!   may be supplied before the ident; it defaults to private.
//! - Each list entry is just the interface ident. The associated `_Vtbl` type is reached
//!   through `<IFoo as Interface>::Vtable`, so it does not need to be spelled out, and
//!   the `_Impl` trait is referenced only by user code outside the macro.
//! - At least one interface must be supplied.
//!
//! ## Generic implementer types
//!
//! For generic implementers like `StockIterable<T>` that implement generic interfaces such
//! as `IIterable<T>`, the macro accepts a leading `<G…>` generic-parameter list and a
//! mandatory trailing `where` clause. Each interface entry is then spelled out as a full
//! type rather than a bare ident:
//!
//! ```rust,ignore
//! implement_decl! {
//!     impl<T> StockIterable as pub(crate) StockIterable_Impl: [
//!         IIterable<T>,
//!     ]
//!     where T: RuntimeType + 'static, T::Default: Clone
//! }
//! ```
//!
//! The `where` clause is forwarded verbatim to every emitted impl and to the `Foo_Impl`
//! struct definition.
//!
//! Generic invocations differ from non-generic ones in two emission details:
//!
//! - per-interface vtables are stored as **associated constants** on `impl<G…> Foo_Impl<G…>`
//!   (an in-`fn` `const C: T = …;` cannot reference outer generic parameters; an associated
//!   constant can),
//! - `into_outer` is *not* `const fn` (a generic `fn` cannot be `const fn` while reading
//!   associated constants whose values depend on `Self`'s type arguments), and there is no
//!   `into_static` (a generic type has no fixed layout for static storage).
//!
//! ## Generated items
//!
//! - `struct Foo_Impl` — `#[repr(C)]` with leading `base: ComposeBase`, `identity:
//!   &'static IInspectable_Vtbl`, one `&'static IFace_Vtbl` field per interface (named
//!   after the interface ident), then `this: Foo`, `count: WeakRefCount`.
//! - `impl Foo { fn into_outer, fn into_static }` — the same shape as the proc-macro.
//!   Vtables are stored as const-promoted `&'static` references to `const fn` results;
//!   this avoids needing to synthesize per-interface const identifiers.
//! - `impl Deref<Target=Foo> for Foo_Impl`.
//! - `impl IUnknownImpl for Foo_Impl` with a `QueryInterface` that handles, in order:
//!   - `IUnknown` / `IInspectable` / `IAgileObject` → identity vtable,
//!   - each declared interface IID → that interface's vtable,
//!   - `IMarshal` (Windows only) → standard marshaler,
//!   - `DYNAMIC_CAST_IID` → `&dyn Any` write,
//!   - weak-reference tear-off,
//!   - aggregation fall-through to the inner non-delegating `IInspectable`.
//! - `impl ComObjectInner for Foo`.
//! - `impl Compose for Foo` so the type can participate in WinRT aggregation.
//! - `impl From<Foo>` for `IUnknown`, `IInspectable`, and each declared interface.
//! - `impl ComObjectInterface<I> for Foo_Impl` for `IUnknown`, `IInspectable`, and each
//!   declared interface.
//! - `impl AsImpl<Foo> for I` for each declared interface.

/// Declares a Rust type as the COM implementer of one or more interfaces, without using
/// the `#[implement]` proc-macro. See the module-level documentation for the supported
/// syntax and scope.
#[macro_export]
macro_rules! implement_decl {
    // Generic form: `impl<G, …> Name as Vis Name_Impl : [Iface<…>, …] where …`.
    //
    // Listed before the non-generic arm so that a leading `<` reliably steers here.
    // The `where` clause is required and is forwarded verbatim to every emitted impl.
    //
    // Reskinned in Step 2c of `docs/option-d.md`: like the non-generic arm (Step 2b),
    // `Foo_Impl<G…>` is now a **type alias** for `windows_core::imp::Outer<Foo<G…>,
    // (IFace1<…>, …)>`, and the foundation provides `IUnknownImpl`, `ComObjectInner`,
    // `Compose`, identity `From` / `ComObjectInterface` impls, `Deref`, and the QI
    // dispatch as blanket impls keyed on `T: Implemented`. The generic-arm-specific
    // helpers (`__implement_decl_g_first_iface`, `__implement_decl_g_struct`,
    // `__implement_decl_offset_negate`, `__implement_decl_index_plus_two`) have been
    // retired; only `__implement_decl_g_zip` (interface-list normalisation) and
    // `__implement_decl_g_per_iface_impls` (orphan-rule-bound per-`IFace` emission)
    // remain.
    //
    // **Behavioural change vs the pre-reskin emission:** same as the non-generic arm —
    // `GetRuntimeClassName` now returns `IInspectable` rather than the first declared
    // interface's name. None of the in-tree generic `implement_decl!` consumers
    // (`windows-collections`, `windows-future`) are activatable runtime classes.
    (
        impl < $($gp:ident),+ $(,)? >
            $name:ident as $impl_vis:vis $impl_name:ident
        : [
            $( $ifty:ty ),+ $(,)?
        ]
        where $($wc:tt)+
    ) => {
        // `Foo_Impl<G…>` is a type alias resolving through `Outer<T, L>`, so user
        // code like `impl<G…> IFoo_Impl<G…> for Foo_Impl<G…> { … }` keeps compiling.
        //
        // No `where` clause on the alias itself: stable Rust's type-alias where-clause
        // handling is "parsed but not enforced" (warning since 1.79+); the bound is
        // already enforced at every use site by `Outer<T, L>`'s own
        // `T: Implemented<Interfaces = L>, L: InterfaceList` requirement.
        #[allow(non_camel_case_types)]
        $impl_vis type $impl_name< $($gp),+ > = $crate::imp::Outer<
            $name< $($gp),+ >,
            ( $($ifty,)+ ),
        >;

        impl< $($gp),+ > $crate::imp::Implemented for $name< $($gp),+ >
        where $($wc)+
        {
            type Interfaces = ( $($ifty,)+ );
            type Agility = $crate::imp::Agile;
        }

        impl< $($gp),+ > $name< $($gp),+ >
        where $($wc)+
        {
            /// Constructs the outer (boxed) representation of this implementer.
            ///
            /// This is an implementation detail; user code should normally go
            /// through [`ComObject::new`](::windows_core::ComObject::new) instead.
            ///
            /// `pub const fn` is possible because `Outer::new_generic` is itself
            /// `const fn` (its body only reads associated constants and calls
            /// `const fn` ctors). No `into_static` companion: a generic type
            /// has no fixed layout for static storage.
            #[doc(hidden)]
            #[inline(always)]
            #[allow(non_snake_case)]
            pub const fn into_outer(self) -> $impl_name< $($gp),+ > {
                $crate::imp::Outer::new_generic(self)
            }
        }

        // Per-declared-interface impls. Walked one at a time by the helper so that
        // `index`-derived SLOT (= 1 + index) and BACK (= 2 + index) offsets stay in
        // sync with the foundation layout. See the non-generic helper docstring in
        // `__implement_decl_per_iface_impls` for the offset rationale.
        $crate::__implement_decl_g_per_iface_impls! {
            @walk
            generics:    [ $($gp),+ ],
            wc:          { $($wc)+ },
            name:        $name,
            impl_name:   $impl_name,
            index:       [ ],
            remaining:   [ $( ($ifty) )+ ]
        }
    };

    // Non-generic form: `impl Name as Vis Name_Impl : [Iface, …]`.
    //
    // Reskinned in Step 2b of `docs/option-d.md`: instead of emitting a
    // bespoke `Foo_Impl` struct + `IUnknownImpl` + identity blanket impls,
    // this arm now emits a `Foo_Impl` **type alias** for
    // `windows_core::imp::Outer<Foo, (IFace1, IFace2, …)>` plus a one-line
    // `Implemented for Foo`. The foundation in
    // `crates/libs/core/src/imp/implement/` provides `IUnknownImpl`,
    // `ComObjectInner`, `Compose`, `From<Foo> for IUnknown`/`IInspectable`,
    // identity `ComObjectInterface<IUnknown>`/`<IInspectable>`, `Deref`,
    // and the QI dispatch as blanket impls keyed on `T: Implemented`.
    //
    // The macro still emits per-declared-interface `From<Foo> for IFace`,
    // `ComObjectInterface<IFace> for Foo_Impl`, and `AsImpl<Foo> for IFace`
    // because the orphan rules forbid blanketing those (see the rationale
    // in `crates/libs/core/src/imp/implement/runtime.rs`). It also emits
    // an inherent `pub const fn into_outer` and `pub const fn into_static`
    // on `Foo` so the public API contract advertised by this module's
    // docstring (and used by hand-written `implement_decl!` callers) is
    // preserved.
    //
    // **Behavioural change vs the pre-reskin emission:** the IInspectable
    // identity vtable now uses `IInspectable` itself as the
    // `GetRuntimeClassName` source (the foundation default — see
    // `crates/libs/core/src/imp/implement/vtbl.rs`), rather than the first
    // declared interface. The pre-reskin emission picked the first
    // interface to "[mirror] the proc-macro so that GetRuntimeClassName
    // works for runtime-class implementers". Hand-written `implement_decl!`
    // sites in this repo (`windows-future`, `windows-collections`, the
    // `macro_rules_decl` test) are stock implementation-detail wrappers
    // that are never queried for `GetRuntimeClassName`, so the change is a
    // no-op in practice. Code that wants a custom runtime-class name
    // should switch to `#[implement]` (which Step 3 of `docs/option-d.md`
    // will reskin onto the same foundation).
    (
        impl $name:ident as $impl_vis:vis $impl_name:ident : [
            $( $iface:ident ),+ $(,)?
        ] $(,)?
    ) => {
        // `Foo_Impl` is now a type alias resolving through `Outer<T, L>`,
        // so `impl IFoo_Impl for Foo_Impl { … }` user blocks keep
        // compiling unchanged.
        #[allow(non_camel_case_types)]
        $impl_vis type $impl_name = $crate::imp::Outer<$name, ( $($iface,)+ )>;

        impl $crate::imp::Implemented for $name {
            type Interfaces = ( $($iface,)+ );
            type Agility = $crate::imp::Agile;
        }

        impl $name {
            /// Constructs the outer (boxed) representation of this implementer.
            ///
            /// This is an implementation detail; user code should normally go through
            /// [`ComObject::new`](::windows_core::ComObject::new) instead.
            #[doc(hidden)]
            #[inline(always)]
            #[allow(non_snake_case)]
            pub const fn into_outer(self) -> $impl_name {
                $crate::imp::Outer::new_generic(self)
            }

            /// Converts a value into a [`StaticComObject`](::windows_core::StaticComObject)
            /// suitable for storage in a static (global) variable.
            pub const fn into_static(self) -> $crate::StaticComObject<Self> {
                $crate::StaticComObject::from_outer(self.into_outer())
            }
        }

        // Per-interface impls. Each of these is a sequence of `impl` items, which the
        // helper macro emits at item position by recursion.
        $crate::__implement_decl_per_iface_impls!(
            $name, $impl_name,
            $( ($iface), )+
        );
    };
}

// --- Per-interface impls ----------------------------------------------------------------
//
// Recursive emission of:
//   - `From<Foo> for IFace`
//   - `ComObjectInterface<IFace> for Foo_Impl`
//   - `AsImpl<Foo> for IFace`
//
// Each of these is emitted **per declared interface** because the orphan rules forbid
// a generic `impl<I> ... for Outer<T, L> where L: Implements<I>` blanket inside
// windows-core (it would collide with the identity `<IUnknown>` / `<IInspectable>` impls
// emitted by the foundation). See the rationale in
// `crates/libs/core/src/imp/implement/runtime.rs`.
//
// `index` is a unary counter tracking the position of the current interface in the
// declared list (0-based). Two derived offsets:
//
//   * `SLOT = 1 + index` — pointer-units offset from `&Outer.identity` to the
//     vtable cell for this interface. Used by `ComObjectInterface::as_interface_ref`
//     via the const-generic `Outer::as_slot_interface<I, SLOT>` accessor.
//   * `BACK = 2 + index` — pointer-units offset from a vtable pointer back to the
//     start of `Outer<T, L>`. Used by `AsImpl::as_impl_ptr` to recover the user
//     value from a raw COM interface pointer. The `+2` accounts for the `base` and
//     `identity` fields preceding the vtable storage.
//
// `Outer.this` is `pub(super)` (private to `windows_core::imp::implement::*`), so
// `as_impl_ptr` reaches the user value via the public `IUnknownImpl::get_impl` blanket
// instead of poking the field directly.

#[doc(hidden)]
#[macro_export]
macro_rules! __implement_decl_per_iface_impls {
    ($name:ident, $impl_name:ident, $(($iface:ident),)+ ) => {
        $crate::__implement_decl_per_iface_impls!(
            @walk
            name: $name,
            impl_name: $impl_name,
            index: [ ],
            remaining: [ $( ($iface) )+ ]
        );
    };
    (@walk
        name: $name:ident,
        impl_name: $impl_name:ident,
        index: [ $($index:tt)* ],
        remaining: [ ($iface:ident) $($rest:tt)* ]
    ) => {
        impl ::core::convert::From<$name> for $iface {
            #[inline(always)]
            fn from(this: $name) -> Self {
                let com_object = $crate::ComObject::new(this);
                com_object.into_interface()
            }
        }

        impl $crate::ComObjectInterface<$iface> for $impl_name {
            #[inline(always)]
            fn as_interface_ref(&self) -> $crate::InterfaceRef<'_, $iface> {
                unsafe {
                    self.as_slot_interface::<
                        $iface,
                        { $crate::__implement_decl_index_plus_one!($($index)*) },
                    >()
                }
            }
        }

        impl $crate::AsImpl<$name> for $iface {
            #[inline(always)]
            unsafe fn as_impl_ptr(&self) -> ::core::ptr::NonNull<$name> {
                unsafe {
                    let this = $crate::Interface::as_raw(self);
                    // 2 + index pointer-slots back from the vtable pointer = start of
                    // Foo_Impl (identity at -1, this chain at -(2 + index)).
                    let this = (this as *mut *mut ::core::ffi::c_void)
                        .sub({ $crate::__implement_decl_index_plus_one!($($index)*) } + 1)
                        as *const $impl_name;
                    let inner: &$name = $crate::IUnknownImpl::get_impl(&*this);
                    ::core::ptr::NonNull::new_unchecked(inner as *const $name as *mut $name)
                }
            }
        }

        $crate::__implement_decl_per_iface_impls!(
            @walk
            name: $name,
            impl_name: $impl_name,
            index: [ $($index)* () ],
            remaining: [ $($rest)* ]
        );
    };
    (@walk
        name: $name:ident,
        impl_name: $impl_name:ident,
        index: [ $($index:tt)* ],
        remaining: [ ]
    ) => {};
}

// `index` is a unary count starting at 0 (empty); emit `1 + index` as a usize literal.
// Used to compute the SLOT const-generic for `Outer::as_slot_interface`.
#[doc(hidden)]
#[macro_export]
macro_rules! __implement_decl_index_plus_one {
    () => {
        1usize
    };
    (()) => {
        2usize
    };
    (() ()) => {
        3usize
    };
    (() () ()) => {
        4usize
    };
    (() () () ()) => {
        5usize
    };
    (() () () () ()) => {
        6usize
    };
    (() () () () () ()) => {
        7usize
    };
    (() () () () () () ()) => {
        8usize
    };
    (() () () () () () () ()) => {
        9usize
    };
    (() () () () () () () () ()) => {
        10usize
    };
    (() () () () () () () () () ()) => {
        11usize
    };
    (() () () () () () () () () () ()) => {
        12usize
    };
    (() () () () () () () () () () () ()) => {
        13usize
    };
    (() () () () () () () () () () () () ()) => {
        14usize
    };
    (() () () () () () () () () () () () () ()) => {
        15usize
    };
    (() () () () () () () () () () () () () () ()) => {
        16usize
    };
}

// --- Per-interface impls (generic arm) -------------------------------------------------
//
// Same shape as `__implement_decl_per_iface_impls` (non-generic) but with a forwarded
// `where` clause and `< $($gp),+ >` plumbing on every emitted impl. See the non-generic
// helper's docstring for the SLOT / BACK offset rationale and orphan-rule justification.
//
// The generic arm dispatches directly into `@walk` (no entry/normalisation arm) — the
// caller already passes the `index: [ ]` accumulator and `remaining: [ (ifty)+ ]` list.

#[doc(hidden)]
#[macro_export]
macro_rules! __implement_decl_g_per_iface_impls {
    (@walk
        generics:    [ $($gp:ident),+ ],
        wc:          { $($wc:tt)* },
        name:        $name:ident,
        impl_name:   $impl_name:ident,
        index:       [ $($index:tt)* ],
        remaining:   [ ($ifty:ty) $($rest:tt)* ]
    ) => {
        impl< $($gp),+ > ::core::convert::From<$name< $($gp),+ >> for $ifty
        where $($wc)*
        {
            #[inline(always)]
            fn from(this: $name< $($gp),+ >) -> Self {
                let com_object = $crate::ComObject::new(this);
                com_object.into_interface()
            }
        }

        impl< $($gp),+ > $crate::ComObjectInterface<$ifty> for $impl_name< $($gp),+ >
        where $($wc)*
        {
            #[inline(always)]
            fn as_interface_ref(&self) -> $crate::InterfaceRef<'_, $ifty> {
                unsafe {
                    self.as_slot_interface::<
                        $ifty,
                        { $crate::__implement_decl_index_plus_one!($($index)*) },
                    >()
                }
            }
        }

        impl< $($gp),+ > $crate::AsImpl<$name< $($gp),+ >> for $ifty
        where $($wc)*
        {
            #[inline(always)]
            unsafe fn as_impl_ptr(&self) -> ::core::ptr::NonNull<$name< $($gp),+ >> {
                unsafe {
                    let this = $crate::Interface::as_raw(self);
                    // SLOT + 1 pointer-units back from the vtable pointer = start of
                    // `Outer<Foo<G…>, …>` (identity at -1, this chain at -(1 + SLOT) =
                    // -(2 + index)). Reach the user value through the public
                    // `IUnknownImpl::get_impl` blanket since `Outer.this` is `pub(super)`.
                    let this = (this as *mut *mut ::core::ffi::c_void)
                        .sub({ $crate::__implement_decl_index_plus_one!($($index)*) } + 1)
                        as *const $impl_name< $($gp),+ >;
                    let inner: &$name< $($gp),+ > = $crate::IUnknownImpl::get_impl(&*this);
                    ::core::ptr::NonNull::new_unchecked(
                        inner as *const $name< $($gp),+ > as *mut $name< $($gp),+ >,
                    )
                }
            }
        }

        $crate::__implement_decl_g_per_iface_impls! {
            @walk
            generics:    [ $($gp),+ ],
            wc:          { $($wc)* },
            name:        $name,
            impl_name:   $impl_name,
            index:       [ $($index)* () ],
            remaining:   [ $($rest)* ]
        }
    };

    (@walk
        generics:    [ $($gp:ident),+ ],
        wc:          { $($wc:tt)* },
        name:        $name:ident,
        impl_name:   $impl_name:ident,
        index:       [ $($index:tt)* ],
        remaining:   [ ]
    ) => {};
}
