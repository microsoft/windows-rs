//! Declares COM interfaces without the `windows-interface` proc-macro dependency.
//!
//! The direct parent must be `IUnknown`. Methods may return `Result<()>`, nothing, or a raw
//! ABI type. Use an out parameter for other result values. `Ref<T>`, `OutRef<T>`, scoped
//! interfaces, and WinRT interfaces require `#[interface]`.
//!
//! ```
//! use windows_core::*;
//!
//! interface_decl! {
//!     unsafe trait IFoo(IFoo_Vtbl, IFoo_Impl) : IUnknown
//!         = 0x094d70d6_5202_44b8_abb8_43860da5aca2
//!     {
//!         unsafe fn Void(&self);
//!         unsafe fn TryGetValue(&self, value: *mut i32) -> Result<()>;
//!     }
//! }
//! ```

/// Declares a COM interface inheriting from `IUnknown`, without using the `#[interface]`
/// proc-macro.
#[macro_export]
macro_rules! interface_decl {
    (
        unsafe trait $name:ident ( $vtbl:ident, $impl_trait:ident ) : $parent:ty = $iid:literal {
            $($methods:tt)*
        }
    ) => {
        $crate::imp::define_interface!($name, $vtbl, $iid);
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

        impl $name {
            $crate::__interface_decl_safe_wrappers!($($methods)*);
        }

        #[allow(non_camel_case_types)]
        pub trait $impl_trait: Sized + $crate::IUnknownImpl {
            $crate::__interface_decl_trait_methods!($($methods)*);
        }

        // Struct field lists and initializers cannot call helper macros, so one accumulator
        // emits the vtable struct and impl together.
        $crate::__interface_decl_vtbl! {
            @start
            name: $name,
            vtbl: $vtbl,
            impl_trait: $impl_trait,
            parent: $parent,
            methods: { $($methods)* }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __interface_decl_safe_wrappers {
    () => {};
    (
        unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) -> Result < () > ;
        $($rest:tt)*
    ) => {
        #[inline]
        pub unsafe fn $mname(&self $(, $aname: $aty)*) -> $crate::Result<()> {
            unsafe {
                ($crate::Interface::vtable(self).$mname)($crate::Interface::as_raw(self) $(, $aname)*).ok()
            }
        }
        $crate::__interface_decl_safe_wrappers!($($rest)*);
    };
    (
        unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) ;
        $($rest:tt)*
    ) => {
        #[inline]
        pub unsafe fn $mname(&self $(, $aname: $aty)*) {
            unsafe {
                ($crate::Interface::vtable(self).$mname)($crate::Interface::as_raw(self) $(, $aname)*)
            }
        }
        $crate::__interface_decl_safe_wrappers!($($rest)*);
    };
    // Method with an arbitrary (non-`Result<()>`) return type - passed through verbatim.
    // This arm must follow the `Result<()>` arm above because `$rty:ty` would also match
    // `Result<()>`; macro_rules tries arms top-down.
    (
        unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) -> $rty:ty ;
        $($rest:tt)*
    ) => {
        #[inline]
        pub unsafe fn $mname(&self $(, $aname: $aty)*) -> $rty {
            unsafe {
                ($crate::Interface::vtable(self).$mname)($crate::Interface::as_raw(self) $(, $aname)*)
            }
        }
        $crate::__interface_decl_safe_wrappers!($($rest)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __interface_decl_trait_methods {
    () => {};
    (
        unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) -> Result < () > ;
        $($rest:tt)*
    ) => {
        unsafe fn $mname(&self $(, $aname: $aty)*) -> $crate::Result<()>;
        $crate::__interface_decl_trait_methods!($($rest)*);
    };
    (
        unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) ;
        $($rest:tt)*
    ) => {
        unsafe fn $mname(&self $(, $aname: $aty)*);
        $crate::__interface_decl_trait_methods!($($rest)*);
    };
    // Arbitrary non-`Result<()>` return type. Must follow the `Result<()>` arm.
    (
        unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) -> $rty:ty ;
        $($rest:tt)*
    ) => {
        unsafe fn $mname(&self $(, $aname: $aty)*) -> $rty;
        $crate::__interface_decl_trait_methods!($($rest)*);
    };
}

// Helper macros cannot expand into struct fields, so this accumulates and emits the entire vtable.

#[doc(hidden)]
#[macro_export]
macro_rules! __interface_decl_vtbl {
    (@start
        name: $name:ident,
        vtbl: $vtbl:ident,
        impl_trait: $impl_trait:ident,
        parent: $parent:ty,
        methods: { $($methods:tt)* }
    ) => {
        $crate::__interface_decl_vtbl! {
            @walk
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

    (@walk
        name: $name:ident,
        vtbl: $vtbl:ident,
        impl_trait: $impl_trait:ident,
        parent: $parent:ty,
        fields: { $($fields:tt)* },
        inits: { $($inits:tt)* },
        thunks: { $($thunks:tt)* },
        rest: {
            unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) -> Result < () > ;
            $($more:tt)*
        }
    ) => {
        $crate::__interface_decl_vtbl! {
            @walk
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
                unsafe extern "system" fn $mname<Identity, const OFFSET: isize>(
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

    (@walk
        name: $name:ident,
        vtbl: $vtbl:ident,
        impl_trait: $impl_trait:ident,
        parent: $parent:ty,
        fields: { $($fields:tt)* },
        inits: { $($inits:tt)* },
        thunks: { $($thunks:tt)* },
        rest: {
            unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) ;
            $($more:tt)*
        }
    ) => {
        $crate::__interface_decl_vtbl! {
            @walk
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
                unsafe extern "system" fn $mname<Identity, const OFFSET: isize>(
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

    // Method with an arbitrary (non-`Result<()>`) return type - passed through verbatim.
    // Must come after the `Result<()>` arm because `$rty:ty` would also match `Result<()>`.
    (@walk
        name: $name:ident,
        vtbl: $vtbl:ident,
        impl_trait: $impl_trait:ident,
        parent: $parent:ty,
        fields: { $($fields:tt)* },
        inits: { $($inits:tt)* },
        thunks: { $($thunks:tt)* },
        rest: {
            unsafe fn $mname:ident (&self $(, $aname:ident : $aty:ty)* $(,)? ) -> $rty:ty ;
            $($more:tt)*
        }
    ) => {
        $crate::__interface_decl_vtbl! {
            @walk
            name: $name,
            vtbl: $vtbl,
            impl_trait: $impl_trait,
            parent: $parent,
            fields: {
                $($fields)*
                pub $mname: unsafe extern "system" fn(
                    this: *mut ::core::ffi::c_void
                    $(, $aname: $aty)*
                ) -> $rty,
            },
            inits: {
                $($inits)*
                $mname: $mname::<Identity, OFFSET>,
            },
            thunks: {
                $($thunks)*
                unsafe extern "system" fn $mname<Identity, const OFFSET: isize>(
                    this: *mut ::core::ffi::c_void
                    $(, $aname: $aty)*
                ) -> $rty
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

    (@walk
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
        pub struct $vtbl {
            pub base__: <$parent as $crate::Interface>::Vtable,
            $($fields)*
        }

        impl $vtbl {
            pub const fn new<Identity, const OFFSET: isize>() -> Self
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
