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
    // The `where` clause is required and is forwarded verbatim to every emitted impl
    // and to the `Foo_Impl` struct definition.
    (
        impl < $($gp:ident),+ $(,)? >
            $name:ident as $impl_vis:vis $impl_name:ident
        : [
            $( $ifty:ty ),+ $(,)?
        ]
        where $($wc:tt)+
    ) => {
        $crate::__implement_decl_g_zip! {
            @zip
            ctx: {
                generics:  [ $($gp),+ ],
                wc:        { $($wc)+ },
                vis:       $impl_vis,
                name:      $name,
                impl_name: $impl_name,
            },
            names: [
                __iface0  __iface1  __iface2  __iface3
                __iface4  __iface5  __iface6  __iface7
                __iface8  __iface9  __iface10 __iface11
                __iface12 __iface13 __iface14 __iface15
            ],
            tys: [ $($ifty),+ ],
            acc: [ ]
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

// --- Offset computation (used by the generic arm only) ---------------------------------
//
// The non-generic arm no longer needs this helper because its `Foo_Impl` is now a
// type alias for `Outer<Foo, (…)>` and the foundation computes vtable offsets
// internally. The generic arm (`__implement_decl_g_*`) still emits a hand-rolled
// `Foo_Impl<G…>` struct (Step 2c is the future reskin) and writes each interface
// vtable as `IFoo_Vtbl::new::<Foo_Impl<G…>, OFFSET>()`. The offsets follow the
// proc-macro convention: identity is at -1, the first interface chain at -2, the
// second at -3, and so on.
//
// `__implement_decl_offset_negate!()` takes a unary-counted token list and emits
// the corresponding negative `isize` literal.  Each `()` in the input represents
// one pointer-sized slot of offset.  The accumulator starts with `[() ()]`
// (= -2) before any interface is consumed; each interface push adds one more `()`.

#[doc(hidden)]
#[macro_export]
macro_rules! __implement_decl_offset_negate {
    (()) => {
        -1isize
    };
    (() ()) => {
        -2isize
    };
    (() () ()) => {
        -3isize
    };
    (() () () ()) => {
        -4isize
    };
    (() () () () ()) => {
        -5isize
    };
    (() () () () () ()) => {
        -6isize
    };
    (() () () () () () ()) => {
        -7isize
    };
    (() () () () () () () ()) => {
        -8isize
    };
    (() () () () () () () () ()) => {
        -9isize
    };
    (() () () () () () () () () ()) => {
        -10isize
    };
    (() () () () () () () () () () ()) => {
        -11isize
    };
    (() () () () () () () () () () () ()) => {
        -12isize
    };
    (() () () () () () () () () () () () ()) => {
        -13isize
    };
    (() () () () () () () () () () () () () ()) => {
        -14isize
    };
    (() () () () () () () () () () () () () () ()) => {
        -15isize
    };
    (() () () () () () () () () () () () () () () ()) => {
        -16isize
    }; // Hand-written implementers rarely declare more than a handful of interfaces;
       // the hard cap is more than the practical maximum. If you hit this, split your
       // implementation across multiple objects or use the proc-macro.
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
                        .sub($crate::__implement_decl_index_plus_two!($($index)*))
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

// `index` is a unary count starting at 0 (empty); emit `2 + index` as a usize literal.
#[doc(hidden)]
#[macro_export]
macro_rules! __implement_decl_index_plus_two {
    () => {
        2usize
    };
    (()) => {
        3usize
    };
    (() ()) => {
        4usize
    };
    (() () ()) => {
        5usize
    };
    (() () () ()) => {
        6usize
    };
    (() () () () ()) => {
        7usize
    };
    (() () () () () ()) => {
        8usize
    };
    (() () () () () () ()) => {
        9usize
    };
    (() () () () () () () ()) => {
        10usize
    };
    (() () () () () () () () ()) => {
        11usize
    };
    (() () () () () () () () () ()) => {
        12usize
    };
    (() () () () () () () () () () ()) => {
        13usize
    };
    (() () () () () () () () () () () ()) => {
        14usize
    };
    (() () () () () () () () () () () () ()) => {
        15usize
    };
    (() () () () () () () () () () () () () ()) => {
        16usize
    };
    (() () () () () () () () () () () () () () ()) => {
        17usize
    };
}

// --- Zip helper: pair each interface type with a fresh internal field name ---------------
//
// The user-facing macro accepts a bare comma-separated list of interface types like
// `[ IAsyncOperation<T>, IAsyncInfo, ]`. macro_rules! can't tokenize that with a `tt`
// repetition because of `<`/`>` ambiguity, but `$ty` matches each entry as a single
// fragment. The zip then pairs each `ty` with an internal ident drawn from a fixed pool
// so the rest of the pipeline (which uses `(ident : ty)` pairs) is unchanged.

#[doc(hidden)]
#[macro_export]
macro_rules! __implement_decl_g_zip {
    // Step: pop one name and one type.
    (@zip
        ctx: $ctx:tt,
        names: [ $name_head:ident $($name_rest:ident)* ],
        tys:   [ $ty_head:ty $(, $ty_rest:ty)* $(,)? ],
        acc:   [ $($acc:tt)* ]
    ) => {
        $crate::__implement_decl_g_zip! {
            @zip
            ctx:   $ctx,
            names: [ $($name_rest)* ],
            tys:   [ $($ty_rest),* ],
            acc:   [ $($acc)* ($name_head : $ty_head) ]
        }
    };

    // Done: dispatch to the existing pipeline.
    (@zip
        ctx: {
            generics:  [ $($gp:ident),+ ],
            wc:        { $($wc:tt)* },
            vis:       $impl_vis:vis,
            name:      $name:ident,
            impl_name: $impl_name:ident,
        },
        names: [ $($unused:ident)* ],
        tys:   [ ],
        acc:   [ $( ($iface:ident : $ifty:ty) )+ ]
    ) => {
        $crate::__implement_decl_g_first_iface! {
            @find
            generics:  [ $($gp),+ ],
            wc:        { $($wc)* },
            vis:       $impl_vis,
            name:      $name,
            impl_name: $impl_name,
            remaining: [ $( ($iface : $ifty) )+ ]
        }

        $crate::__implement_decl_g_per_iface_impls! {
            generics:   [ $($gp),+ ],
            wc:         { $($wc)* },
            name:       $name,
            impl_name:  $impl_name,
            interfaces: [ $( ($iface : $ifty) )+ ]
        }
    };
}

// --- Entry helper: capture the first interface, kick off the main walk -----------------

#[doc(hidden)]
#[macro_export]
macro_rules! __implement_decl_g_first_iface {
    (@find
        generics: [ $($gp:ident),+ ],
        wc:       { $($wc:tt)* },
        vis:      $impl_vis:vis,
        name:     $name:ident,
        impl_name: $impl_name:ident,
        remaining: [ ($first_iface:ident : $first_ifty:ty) $($rest:tt)* ]
    ) => {
        $crate::__implement_decl_g_struct! {
            @walk
            generics:    [ $($gp),+ ],
            wc:          { $($wc)* },
            vis:         $impl_vis,
            name:        $name,
            impl_name:   $impl_name,
            first_ifty:  $first_ifty,
            fields:      { },
            consts:      { },
            inits:       { },
            qi_pairs:    [ ],
            offset:      [ () () ],
            remaining:   [ ($first_iface : $first_ifty) $($rest)* ]
        }
    };
}

// --- Main accumulator -------------------------------------------------------------------
//
// Walks the interface list and accumulates:
//   * struct field declarations (`fields`),
//   * associated-constant declarations on `Foo_Impl` (`consts`),
//   * struct field initializers for `into_outer` (`inits`),
//   * `(field_ident, ifty)` pairs for `QueryInterface` matching (`qi_pairs`),
//   * a unary-counted offset (`offset`) so the per-chain vtable knows its slot.
//
// Hygiene: like `implement_decl!`, the accumulator carries *data* only. References to
// `self`, `iid`, and the `'found` label are emitted from the single base arm at the end.

#[doc(hidden)]
#[macro_export]
macro_rules! __implement_decl_g_struct {
    // One more interface to consume.
    (@walk
        generics:    [ $($gp:ident),+ ],
        wc:          { $($wc:tt)* },
        vis:         $impl_vis:vis,
        name:        $name:ident,
        impl_name:   $impl_name:ident,
        first_ifty:  $first_ifty:ty,
        fields:      { $($fields:tt)* },
        consts:      { $($consts:tt)* },
        inits:       { $($inits:tt)* },
        qi_pairs:    [ $($qi_pairs:tt)* ],
        offset:      [ $($offset:tt)* ],
        remaining:   [ ($iface:ident : $ifty:ty) $($rest:tt)* ]
    ) => {
        $crate::__implement_decl_g_struct! {
            @walk
            generics:    [ $($gp),+ ],
            wc:          { $($wc)* },
            vis:         $impl_vis,
            name:        $name,
            impl_name:   $impl_name,
            first_ifty:  $first_ifty,
            fields: {
                $($fields)*
                #[allow(non_snake_case)]
                pub $iface: &'static <$ifty as $crate::Interface>::Vtable,
            },
            consts: {
                $($consts)*
                // Per-chain vtable lives as an associated constant on the generic
                // `impl Foo_Impl<G…>` block. Associated constants are allowed to
                // reference the outer impl's generic parameters; an in-function
                // `const C: ... = ...;` would not be (E0401).
                #[allow(non_upper_case_globals)]
                const $iface: <$ifty as $crate::Interface>::Vtable =
                    <<$ifty as $crate::Interface>::Vtable>::new::<
                        Self,
                        { $crate::__implement_decl_offset_negate!($($offset)*) },
                    >();
            },
            inits: {
                $($inits)*
                $iface: &<$impl_name < $($gp),+ >>::$iface,
            },
            qi_pairs: [ $($qi_pairs)* ($iface : $ifty) ],
            offset: [ $($offset)* () ],
            remaining: [ $($rest)* ]
        }
    };

    // No more interfaces: emit struct + all impls.
    (@walk
        generics:    [ $($gp:ident),+ ],
        wc:          { $($wc:tt)* },
        vis:         $impl_vis:vis,
        name:        $name:ident,
        impl_name:   $impl_name:ident,
        first_ifty:  $first_ifty:ty,
        fields:      { $($fields:tt)* },
        consts:      { $($consts:tt)* },
        inits:       { $($inits:tt)* },
        qi_pairs:    [ $(($qi_iface:ident : $qi_ifty:ty))* ],
        offset:      [ $($offset:tt)* ],
        remaining:   [ ]
    ) => {
        #[repr(C)]
        #[allow(non_camel_case_types, non_snake_case)]
        $impl_vis struct $impl_name < $($gp),+ >
        where $($wc)*
        {
            pub base: $crate::ComposeBase,
            pub identity: &'static $crate::IInspectable_Vtbl,
            $($fields)*
            pub this: $name < $($gp),+ >,
            pub count: $crate::imp::WeakRefCount,
        }

        impl< $($gp),+ > $impl_name < $($gp),+ >
        where $($wc)*
        {
            // The identity vtable, like the per-interface vtables below, has to live
            // on an associated constant rather than as an in-function `const C: T`
            // because the implementer's generic parameters flow into the vtable's
            // type parameters (via `Self`).
            #[allow(non_upper_case_globals)]
            const __VTABLE_IDENTITY: $crate::IInspectable_Vtbl =
                <$crate::IInspectable_Vtbl>::new::<Self, $first_ifty, -1>();

            $($consts)*
        }

        impl< $($gp),+ > $name < $($gp),+ >
        where $($wc)*
        {
            /// Constructs the outer (boxed) representation of this implementer.
            #[doc(hidden)]
            #[inline(always)]
            #[allow(non_snake_case)]
            // Not `const`: a generic `fn` cannot be `const fn` while reading associated
            // constants whose values depend on `Self`'s type arguments.
            pub fn into_outer(self) -> $impl_name < $($gp),+ > {
                $impl_name {
                    base: $crate::ComposeBase::new(),
                    identity: &<$impl_name < $($gp),+ >>::__VTABLE_IDENTITY,
                    $($inits)*
                    this: self,
                    count: $crate::imp::WeakRefCount::new(),
                }
            }
        }

        impl< $($gp),+ > ::core::ops::Deref for $impl_name < $($gp),+ >
        where $($wc)*
        {
            type Target = $name < $($gp),+ >;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.this
            }
        }

        impl< $($gp),+ > $crate::IUnknownImpl for $impl_name < $($gp),+ >
        where $($wc)*
        {
            type Impl = $name < $($gp),+ >;

            #[inline(always)]
            fn get_impl(&self) -> &Self::Impl {
                &self.this
            }

            #[inline(always)]
            fn get_impl_mut(&mut self) -> &mut Self::Impl {
                &mut self.this
            }

            #[inline(always)]
            fn into_inner(self) -> Self::Impl {
                self.this
            }

            #[inline(always)]
            fn AddRef(&self) -> u32 {
                self.count.add_ref()
            }

            #[inline(always)]
            unsafe fn Release(self_: *mut Self) -> u32 {
                unsafe {
                    let remaining = (*self_).count.release();
                    if remaining == 0 {
                        _ = $crate::imp::Box::from_raw(self_);
                    }
                    remaining
                }
            }

            #[inline(always)]
            fn is_reference_count_one(&self) -> bool {
                self.count.is_one()
            }

            unsafe fn GetTrustLevel(&self, value: *mut i32) -> $crate::HRESULT {
                if value.is_null() {
                    return $crate::imp::E_POINTER;
                }
                unsafe { *value = 0; }
                $crate::HRESULT(0)
            }

            fn to_object(&self) -> $crate::ComObject<Self::Impl> {
                self.count.add_ref();
                unsafe {
                    $crate::ComObject::from_raw(
                        ::core::ptr::NonNull::new_unchecked(self as *const Self as *mut Self),
                    )
                }
            }

            unsafe fn QueryInterface(
                &self,
                iid: *const $crate::GUID,
                interface: *mut *mut ::core::ffi::c_void,
            ) -> $crate::HRESULT {
                unsafe {
                    if iid.is_null() || interface.is_null() {
                        return $crate::imp::E_POINTER;
                    }
                    let iid = *iid;
                    let interface_ptr: *const ::core::ffi::c_void = 'found: {
                        if iid == <$crate::IUnknown as $crate::Interface>::IID
                            || iid == <$crate::IInspectable as $crate::Interface>::IID
                            || iid == <$crate::imp::IAgileObject as $crate::Interface>::IID
                        {
                            break 'found &self.identity as *const _ as *const ::core::ffi::c_void;
                        }
                        $(
                            if <<$qi_ifty as $crate::Interface>::Vtable>::matches(&iid) {
                                break 'found &self.$qi_iface as *const _ as *const ::core::ffi::c_void;
                            }
                        )*
                        #[cfg(windows)]
                        if iid == <$crate::imp::IMarshal as $crate::Interface>::IID {
                            return $crate::imp::marshaler(
                                <Self as $crate::IUnknownImpl>::to_interface::<$crate::IUnknown>(self),
                                interface,
                            );
                        }
                        if iid == $crate::DYNAMIC_CAST_IID {
                            (interface as *mut *const dyn ::core::any::Any)
                                .write(self as &dyn ::core::any::Any as *const dyn ::core::any::Any);
                            return $crate::HRESULT(0);
                        }
                        let tear_off_ptr = self.count.query(
                            &iid,
                            &self.identity as *const _ as *mut _,
                        );
                        if !tear_off_ptr.is_null() {
                            *interface = tear_off_ptr;
                            return $crate::HRESULT(0);
                        }
                        if let ::core::option::Option::Some(base) = self.base.as_option() {
                            return $crate::Interface::query(
                                base,
                                &iid as *const $crate::GUID,
                                interface,
                            );
                        }
                        *interface = ::core::ptr::null_mut();
                        return $crate::imp::E_NOINTERFACE;
                    };
                    debug_assert!(!interface_ptr.is_null());
                    *interface = interface_ptr as *mut ::core::ffi::c_void;
                    self.count.add_ref();
                    $crate::HRESULT(0)
                }
            }
        }

        impl< $($gp),+ > $crate::ComObjectInner for $name < $($gp),+ >
        where $($wc)*
        {
            type Outer = $impl_name < $($gp),+ >;

            fn into_object(self) -> $crate::ComObject<Self> {
                let boxed = $crate::imp::Box::<$impl_name < $($gp),+ >>::new(self.into_outer());
                unsafe {
                    let ptr = $crate::imp::Box::into_raw(boxed);
                    $crate::ComObject::from_raw(::core::ptr::NonNull::new_unchecked(ptr))
                }
            }
        }

        impl< $($gp),+ > $crate::Compose for $name < $($gp),+ >
        where $($wc)*
        {
            unsafe fn compose<'a>(
                implementation: Self,
            ) -> ($crate::IInspectable, &'a mut ::core::option::Option<$crate::IInspectable>) {
                unsafe {
                    let inspectable: $crate::IInspectable = implementation.into();
                    let identity_ptr: *mut ::core::ffi::c_void = $crate::Interface::as_raw(&inspectable);
                    let base_ptr = (identity_ptr as *mut *mut ::core::ffi::c_void).sub(1)
                        as *mut ::core::option::Option<$crate::IInspectable>;
                    (inspectable, &mut *base_ptr)
                }
            }
        }

        impl< $($gp),+ > ::core::convert::From<$name < $($gp),+ >> for $crate::IUnknown
        where $($wc)*
        {
            #[inline(always)]
            fn from(this: $name < $($gp),+ >) -> Self {
                let com_object = $crate::ComObject::new(this);
                com_object.into_interface()
            }
        }

        impl< $($gp),+ > ::core::convert::From<$name < $($gp),+ >> for $crate::IInspectable
        where $($wc)*
        {
            #[inline(always)]
            fn from(this: $name < $($gp),+ >) -> Self {
                let com_object = $crate::ComObject::new(this);
                com_object.into_interface()
            }
        }

        impl< $($gp),+ > $crate::ComObjectInterface<$crate::IUnknown> for $impl_name < $($gp),+ >
        where $($wc)*
        {
            #[inline(always)]
            fn as_interface_ref(&self) -> $crate::InterfaceRef<'_, $crate::IUnknown> {
                unsafe { ::core::mem::transmute(&self.identity) }
            }
        }

        impl< $($gp),+ > $crate::ComObjectInterface<$crate::IInspectable> for $impl_name < $($gp),+ >
        where $($wc)*
        {
            #[inline(always)]
            fn as_interface_ref(&self) -> $crate::InterfaceRef<'_, $crate::IInspectable> {
                unsafe { ::core::mem::transmute(&self.identity) }
            }
        }
    };
}

// --- Per-interface impls ----------------------------------------------------------------
//
// Emits `From<Foo<G…>> for IFace`, `ComObjectInterface<IFace> for Foo_Impl<G…>`, and
// `AsImpl<Foo<G…>> for IFace`, mirroring the per-interface emission in `implement_decl!`.

#[doc(hidden)]
#[macro_export]
macro_rules! __implement_decl_g_per_iface_impls {
    (
        generics:    [ $($gp:ident),+ ],
        wc:          { $($wc:tt)* },
        name:        $name:ident,
        impl_name:   $impl_name:ident,
        interfaces:  [ $( ($iface:ident : $ifty:ty) )+ ]
    ) => {
        $crate::__implement_decl_g_per_iface_impls! {
            @walk
            generics:    [ $($gp),+ ],
            wc:          { $($wc)* },
            name:        $name,
            impl_name:   $impl_name,
            index:       [ ],
            remaining:   [ $( ($iface : $ifty) )+ ]
        }
    };

    (@walk
        generics:    [ $($gp:ident),+ ],
        wc:          { $($wc:tt)* },
        name:        $name:ident,
        impl_name:   $impl_name:ident,
        index:       [ $($index:tt)* ],
        remaining:   [ ($iface:ident : $ifty:ty) $($rest:tt)* ]
    ) => {
        impl< $($gp),+ > ::core::convert::From<$name < $($gp),+ >> for $ifty
        where $($wc)*
        {
            #[inline(always)]
            fn from(this: $name < $($gp),+ >) -> Self {
                let com_object = $crate::ComObject::new(this);
                com_object.into_interface()
            }
        }

        impl< $($gp),+ > $crate::ComObjectInterface<$ifty> for $impl_name < $($gp),+ >
        where $($wc)*
        {
            #[inline(always)]
            fn as_interface_ref(&self) -> $crate::InterfaceRef<'_, $ifty> {
                unsafe { ::core::mem::transmute(&self.$iface) }
            }
        }

        impl< $($gp),+ > $crate::AsImpl<$name < $($gp),+ >> for $ifty
        where $($wc)*
        {
            #[inline(always)]
            unsafe fn as_impl_ptr(&self) -> ::core::ptr::NonNull<$name < $($gp),+ >> {
                unsafe {
                    let this = $crate::Interface::as_raw(self);
                    let this = (this as *mut *mut ::core::ffi::c_void)
                        .sub($crate::__implement_decl_index_plus_two!($($index)*))
                        as *mut $impl_name < $($gp),+ >;
                    ::core::ptr::NonNull::new_unchecked(
                        ::core::ptr::addr_of!((*this).this)
                            as *const $name < $($gp),+ >
                            as *mut $name < $($gp),+ >,
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
