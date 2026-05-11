//! A `macro_rules!` declarative alternative to the `#[interface]` proc-macro.
//!
//! The proc-macro `#[interface]` in the `windows-interface` crate is the canonical way to
//! define a COM interface in this codebase, but it transitively pulls in `syn`, `quote`,
//! and `proc-macro2`. Consumers who disable the default `proc-macros` feature on
//! `windows-core` still need a way to declare COM interfaces; that is the role of the
//! [`interface_decl!`] macro defined here.
//!
//! ## Scope
//!
//! `interface_decl!` targets the dominant case: a COM interface whose direct parent is
//! `IUnknown`, whose methods either return `windows_core::Result<T>`, return a value type,
//! or return nothing. It does **not** support `Ref<T>` / `OutRef<T>` parameters with the
//! implicit `Param` / `OutParam` bound generation that `#[interface]` provides — pass the
//! underlying ABI types instead. It does not support scoped (non-`IUnknown`) interfaces
//! or `IInspectable`-derived (WinRT) interfaces. The matching chain for derived custom
//! interfaces is not traversed; use the proc-macro for those cases.
//!
//! ## Syntax
//!
//! ```rust,ignore
//! use windows_core::*;
//!
//! interface_decl! {
//!     /// Docs go here.
//!     pub unsafe trait IFoo(IFoo_Vtbl, IFoo_Impl) : IUnknown
//!         = 0x094d70d6_5202_44b8_abb8_43860da5aca2
//!     {
//!         unsafe fn Void(&self);
//!         unsafe fn GetValue(&self, value: *mut i32) -> HRESULT;
//!         unsafe fn TryGetValue(&self, value: *mut i32) -> Result<()>;
//!     }
//! }
//! ```
//!
//! The struct ident (`IFoo`), the vtable struct ident (`IFoo_Vtbl`), and the implementation
//! trait ident (`IFoo_Impl`) are all spelled out by the caller; `macro_rules!` cannot
//! synthesize identifiers from another ident the way a proc-macro can. The IID is supplied
//! as a `u128` integer literal.
//!
//! ## Semantics
//!
//! - `-> Result<T>` methods produce a safe caller-side wrapper that appends `.ok()` and a
//!   vtable entry typed `-> HRESULT`. The thunk converts the implementer's `Result<T>`
//!   back into an `HRESULT` via `Into`.
//! - Any other return type is passed through unchanged.
//! - A method with no return type produces a void-returning thunk.

/// Declares a COM interface inheriting from `IUnknown`, without using the `#[interface]`
/// proc-macro. See the module-level documentation for the supported syntax and scope.
#[macro_export]
macro_rules! interface_decl {
    (
        $(#[doc = $doc:expr])*
        $vis:vis unsafe trait $name:ident ( $vtbl:ident, $impl_trait:ident ) : $parent:ty = $iid:literal {
            $($methods:tt)*
        }
    ) => {
        // Struct + Interface + Debug.
        $crate::imp::define_interface!($name, $vtbl, $iid);
        // CanInto + From conversions to ancestors.
        $crate::imp::interface_hierarchy!($name, $parent);

        impl ::core::ops::Deref for $name {
            type Target = $parent;
            #[inline]
            fn deref(&self) -> &Self::Target {
                // SAFETY: every interface declared via `define_interface!` is
                // `#[repr(transparent)]` over `IUnknown`, and so is any custom parent
                // declared the same way. The transmute is therefore a no-op layout-wise.
                unsafe { ::core::mem::transmute(self) }
            }
        }

        impl $crate::RuntimeName for $name {}

        // Safe caller-side wrappers (inside `impl $name { ... }`, item-position — helper
        // macros are permitted here and may emit a sequence of `fn` items).
        impl $name {
            $crate::__interface_decl_safe_wrappers!($vis, $($methods)*);
        }

        // Implementation trait (inside `trait { ... }`, item-position).
        #[allow(non_camel_case_types)]
        $vis trait $impl_trait: Sized + $crate::IUnknownImpl {
            $crate::__interface_decl_trait_methods!($($methods)*);
        }

        // Vtable struct + its `impl` block. These cannot use helper macros inside their
        // field lists or struct-expression initializers, so we hand everything off to a
        // TT-muncher accumulator that emits both items together when the method list is
        // exhausted.
        $crate::__interface_decl_vtbl! {
            @start
            vis: $vis,
            name: $name,
            vtbl: $vtbl,
            impl_trait: $impl_trait,
            parent: $parent,
            methods: { $($methods)* }
        }
    };
}

// --- safe caller-side wrappers ---
//
// One arm per supported return-type shape (Result, value, void); each arm peels off the
// head method and recurses on the tail.

#[doc(hidden)]
#[macro_export]
macro_rules! __interface_decl_safe_wrappers {
    ($vis:vis,) => {};
    ($vis:vis,
        $(#[doc = $mdoc:expr])*
        unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) -> Result < $res:ty > ;
        $($rest:tt)*
    ) => {
        $(#[doc = $mdoc])*
        #[inline]
        $vis unsafe fn $mname(&self $(, $aname: $aty)*) -> $crate::Result<$res> {
            unsafe {
                ($crate::Interface::vtable(self).$mname)($crate::Interface::as_raw(self) $(, $aname)*).ok()
            }
        }
        $crate::__interface_decl_safe_wrappers!($vis, $($rest)*);
    };
    ($vis:vis,
        $(#[doc = $mdoc:expr])*
        unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) -> $ret:ty ;
        $($rest:tt)*
    ) => {
        $(#[doc = $mdoc])*
        #[inline]
        $vis unsafe fn $mname(&self $(, $aname: $aty)*) -> $ret {
            unsafe {
                ($crate::Interface::vtable(self).$mname)($crate::Interface::as_raw(self) $(, $aname)*)
            }
        }
        $crate::__interface_decl_safe_wrappers!($vis, $($rest)*);
    };
    ($vis:vis,
        $(#[doc = $mdoc:expr])*
        unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) ;
        $($rest:tt)*
    ) => {
        $(#[doc = $mdoc])*
        #[inline]
        $vis unsafe fn $mname(&self $(, $aname: $aty)*) {
            unsafe {
                ($crate::Interface::vtable(self).$mname)($crate::Interface::as_raw(self) $(, $aname)*)
            }
        }
        $crate::__interface_decl_safe_wrappers!($vis, $($rest)*);
    };
}

// --- _Impl trait method declarations ---

#[doc(hidden)]
#[macro_export]
macro_rules! __interface_decl_trait_methods {
    () => {};
    (
        $(#[doc = $mdoc:expr])*
        unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) -> Result < $res:ty > ;
        $($rest:tt)*
    ) => {
        $(#[doc = $mdoc])*
        unsafe fn $mname(&self $(, $aname: $aty)*) -> $crate::Result<$res>;
        $crate::__interface_decl_trait_methods!($($rest)*);
    };
    (
        $(#[doc = $mdoc:expr])*
        unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) -> $ret:ty ;
        $($rest:tt)*
    ) => {
        $(#[doc = $mdoc])*
        unsafe fn $mname(&self $(, $aname: $aty)*) -> $ret;
        $crate::__interface_decl_trait_methods!($($rest)*);
    };
    (
        $(#[doc = $mdoc:expr])*
        unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) ;
        $($rest:tt)*
    ) => {
        $(#[doc = $mdoc])*
        unsafe fn $mname(&self $(, $aname: $aty)*);
        $crate::__interface_decl_trait_methods!($($rest)*);
    };
}

// --- vtable struct + impl emission via TT-muncher accumulator ---
//
// We can't use helper macros in struct field lists or struct-expression initializers, so
// this macro accumulates four token lists (vtbl fields, vtbl initializers, thunk fn defs,
// and matches-arms for parent chain traversal — currently unused but reserved for a
// future extension) and emits the whole `struct $vtbl { ... } impl $vtbl { ... }` block
// at the end.

#[doc(hidden)]
#[macro_export]
macro_rules! __interface_decl_vtbl {
    // Entry point: initialize empty accumulators.
    (@start
        vis: $vis:vis,
        name: $name:ident,
        vtbl: $vtbl:ident,
        impl_trait: $impl_trait:ident,
        parent: $parent:ty,
        methods: { $($methods:tt)* }
    ) => {
        $crate::__interface_decl_vtbl! {
            @walk
            vis: $vis,
            name: $name,
            vtbl: $vtbl,
            impl_trait: $impl_trait,
            parent: $parent,
            fields: { },
            inits: { },
            thunks: { },
            rest: { $($methods)* }
        }
    };

    // Result-returning method.
    (@walk
        vis: $vis:vis,
        name: $name:ident,
        vtbl: $vtbl:ident,
        impl_trait: $impl_trait:ident,
        parent: $parent:ty,
        fields: { $($fields:tt)* },
        inits: { $($inits:tt)* },
        thunks: { $($thunks:tt)* },
        rest: {
            $(#[doc = $mdoc:expr])*
            unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) -> Result < $res:ty > ;
            $($more:tt)*
        }
    ) => {
        $crate::__interface_decl_vtbl! {
            @walk
            vis: $vis,
            name: $name,
            vtbl: $vtbl,
            impl_trait: $impl_trait,
            parent: $parent,
            fields: {
                $($fields)*
                pub $mname: unsafe extern "system" fn(
                    this: *mut ::core::ffi::c_void
                    $(, $aname: $aty)*
                ) -> $crate::HRESULT,
            },
            inits: {
                $($inits)*
                $mname: $mname::<Identity, OFFSET>,
            },
            thunks: {
                $($thunks)*
                unsafe extern "system" fn $mname<Identity: $crate::IUnknownImpl, const OFFSET: isize>(
                    this: *mut ::core::ffi::c_void
                    $(, $aname: $aty)*
                ) -> $crate::HRESULT
                where
                    Identity: $impl_trait,
                {
                    let this_outer: &Identity = unsafe {
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity)
                    };
                    unsafe { <Identity as $impl_trait>::$mname(this_outer $(, $aname)*) }.into()
                }
            },
            rest: { $($more)* }
        }
    };

    // Value-returning method.
    (@walk
        vis: $vis:vis,
        name: $name:ident,
        vtbl: $vtbl:ident,
        impl_trait: $impl_trait:ident,
        parent: $parent:ty,
        fields: { $($fields:tt)* },
        inits: { $($inits:tt)* },
        thunks: { $($thunks:tt)* },
        rest: {
            $(#[doc = $mdoc:expr])*
            unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) -> $ret:ty ;
            $($more:tt)*
        }
    ) => {
        $crate::__interface_decl_vtbl! {
            @walk
            vis: $vis,
            name: $name,
            vtbl: $vtbl,
            impl_trait: $impl_trait,
            parent: $parent,
            fields: {
                $($fields)*
                pub $mname: unsafe extern "system" fn(
                    this: *mut ::core::ffi::c_void
                    $(, $aname: $aty)*
                ) -> $ret,
            },
            inits: {
                $($inits)*
                $mname: $mname::<Identity, OFFSET>,
            },
            thunks: {
                $($thunks)*
                unsafe extern "system" fn $mname<Identity: $crate::IUnknownImpl, const OFFSET: isize>(
                    this: *mut ::core::ffi::c_void
                    $(, $aname: $aty)*
                ) -> $ret
                where
                    Identity: $impl_trait,
                {
                    let this_outer: &Identity = unsafe {
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity)
                    };
                    unsafe { <Identity as $impl_trait>::$mname(this_outer $(, $aname)*) }
                }
            },
            rest: { $($more)* }
        }
    };

    // Void-returning method.
    (@walk
        vis: $vis:vis,
        name: $name:ident,
        vtbl: $vtbl:ident,
        impl_trait: $impl_trait:ident,
        parent: $parent:ty,
        fields: { $($fields:tt)* },
        inits: { $($inits:tt)* },
        thunks: { $($thunks:tt)* },
        rest: {
            $(#[doc = $mdoc:expr])*
            unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) ;
            $($more:tt)*
        }
    ) => {
        $crate::__interface_decl_vtbl! {
            @walk
            vis: $vis,
            name: $name,
            vtbl: $vtbl,
            impl_trait: $impl_trait,
            parent: $parent,
            fields: {
                $($fields)*
                pub $mname: unsafe extern "system" fn(
                    this: *mut ::core::ffi::c_void
                    $(, $aname: $aty)*
                ),
            },
            inits: {
                $($inits)*
                $mname: $mname::<Identity, OFFSET>,
            },
            thunks: {
                $($thunks)*
                unsafe extern "system" fn $mname<Identity: $crate::IUnknownImpl, const OFFSET: isize>(
                    this: *mut ::core::ffi::c_void
                    $(, $aname: $aty)*
                )
                where
                    Identity: $impl_trait,
                {
                    let this_outer: &Identity = unsafe {
                        &*((this as *const *const ()).offset(OFFSET) as *const Identity)
                    };
                    unsafe { <Identity as $impl_trait>::$mname(this_outer $(, $aname)*) }
                }
            },
            rest: { $($more)* }
        }
    };

    // Base case: rest is empty, emit the struct and its impl block.
    (@walk
        vis: $vis:vis,
        name: $name:ident,
        vtbl: $vtbl:ident,
        impl_trait: $impl_trait:ident,
        parent: $parent:ty,
        fields: { $($fields:tt)* },
        inits: { $($inits:tt)* },
        thunks: { $($thunks:tt)* },
        rest: { }
    ) => {
        #[repr(C)]
        #[doc(hidden)]
        $vis struct $vtbl {
            pub base__: <$parent as $crate::Interface>::Vtable,
            $($fields)*
        }

        impl $vtbl {
            pub const fn new<Identity: $crate::IUnknownImpl, const OFFSET: isize>() -> Self
            where
                Identity: $impl_trait,
            {
                $($thunks)*
                Self {
                    base__: <<$parent as $crate::Interface>::Vtable>::new::<Identity, OFFSET>(),
                    $($inits)*
                }
            }

            #[inline]
            pub fn matches(iid: &$crate::GUID) -> bool {
                *iid == <$name as $crate::Interface>::IID
            }
        }
    };
}
