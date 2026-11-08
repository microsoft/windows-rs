//! Implements COM interfaces without the `windows-implement` proc-macro dependency.
//!
//! Implementers are always agile and support dynamic casting. Add `#[compose]` before `impl` to
//! aggregate a composable WinRT class; the first interface must then derive from `IInspectable`.
//! Per-instance trust levels or opt-outs require `#[implement]`.
//!
//! ```
//! use windows_core::*;
//!
//! interface_decl! {
//!     unsafe trait IFoo(IFoo_Vtbl, IFoo_Impl) : IUnknown
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
//! Generic implementers use `impl<T> Type as Type_Impl: [Interface<T>] where ...` and do not
//! support static storage. At most 16 interfaces are supported.
//!
//! Construct objects through `ComObject::new`, interface conversions, or `into_static`, not by
//! taking ownership of the generated outer type.
//!
//! ```compile_fail,E0133
//! use windows_core::*;
//! interface_decl! {
//!     unsafe trait IFoo(IFoo_Vtbl, IFoo_Impl): IUnknown
//!         = 0x094d70d6_5202_44b8_abb8_43860da5aca2 {}
//! }
//! struct Foo;
//! implement_decl! { impl Foo as Foo_Impl: [IFoo] }
//! impl IFoo_Impl for Foo_Impl {}
//! let outer = Foo.into_outer();
//! let interface: IFoo = outer.to_interface();
//! ```

/// Declares a Rust type as the COM implementer of one or more interfaces, without using
/// the `#[implement]` proc-macro.
#[macro_export]
macro_rules! implement_decl {
    (
        $(#[$compose:ident])?
        impl < $($gp:ident),+ $(,)? > $name:ident as $vis:vis $impl_name:ident
        : [ $($ty:ty),+ $(,)? ]
        $(where $($wc:tt)+)?
    ) => {
        $crate::__implement_decl! {
            @zip { [$($gp),+] { $($($wc)+)? } [] [$($compose)?] $vis $name $impl_name }
            [ (__I0 0) (__I1 1) (__I2 2) (__I3 3) (__I4 4) (__I5 5) (__I6 6) (__I7 7)
              (__I8 8) (__I9 9) (__I10 10) (__I11 11) (__I12 12) (__I13 13) (__I14 14) (__I15 15) ]
            [ $($ty),+ ]
            []
        }
    };
    (
        $(#[$compose:ident])?
        impl $name:ident as $vis:vis $impl_name:ident : [ $($ty:ty),+ $(,)? ] $(,)?
    ) => {
        $crate::__implement_decl! {
            @zip { [] {} [const] [$($compose)?] $vis $name $impl_name }
            [ (__I0 0) (__I1 1) (__I2 2) (__I3 3) (__I4 4) (__I5 5) (__I6 6) (__I7 7)
              (__I8 8) (__I9 9) (__I10 10) (__I11 11) (__I12 12) (__I13 13) (__I14 14) (__I15 15) ]
            [ $($ty),+ ]
            []
        }
    };
}

// The context `{ [generics] { where } [const] [compose] vis name impl_name }` is passed around
// as a single token tree so per-interface arms can use it inside a repetition. Interface `i`
// gets the vtable constant `__Ii` and is the `i`th pointer of the object.
#[doc(hidden)]
#[macro_export]
macro_rules! __implement_decl {
    // Pair each interface with a name and index from the pool.
    (@zip $ctx:tt [ $($pool:tt)* ] [ ] [ ($id:ident $idx:literal $first:ty) $($acc:tt)* ]) => {
        $crate::__implement_decl! { @object $ctx $ctx $first [ ($id $idx $first) $($acc)* ] }
    };
    (@zip $ctx:tt [ ($id:ident $idx:literal) $($pool:tt)* ] [ $ty:ty $(, $rest:ty)* ] [ $($acc:tt)* ]) => {
        $crate::__implement_decl! { @zip $ctx [ $($pool)* ] [ $($rest),* ] [ $($acc)* ($id $idx $ty) ] }
    };

    (@object $ctx:tt
        { [$($gp:ident),*] { $($wc:tt)* } [$($const:tt)?] [$($compose:ident)?] $vis:vis $name:ident $impl_name:ident }
        $first:ty
        [ $( ($id:ident $idx:literal $ty:ty) )+ ]
    ) => {
        #[repr(transparent)]
        #[allow(non_camel_case_types)]
        $vis struct $impl_name<$($gp),*> where $($wc)* {
            __com: $crate::imp::ComCell<
                { [$($idx),+].len() },
                $crate::__implement_decl!(@base $($compose)?),
                $name<$($gp),*>,
            >,
        }

        impl<$($gp),*> $impl_name<$($gp),*> where $($wc)* {
            $(
                #[allow(non_upper_case_globals)]
                const $id: <$ty as $crate::Interface>::Vtable =
                    <<$ty as $crate::Interface>::Vtable>::new::<Self, { -$idx }>();
            )+

            #[inline(always)]
            $($const)? unsafe fn __new(this: $name<$($gp),*>) -> Self {
                Self {
                    __com: unsafe { $crate::imp::ComCell::new(
                        [$( $crate::imp::Vtable::new(&Self::$id) ),+],
                        $crate::__implement_decl!(@base_new $($compose)?),
                        this,
                    ) },
                }
            }
        }

        impl<$($gp),*> $name<$($gp),*> where $($wc)* {
            #[doc(hidden)]
            #[inline(always)]
            $($const)? unsafe fn into_outer(self) -> $impl_name<$($gp),*> {
                unsafe { <$impl_name<$($gp),*>>::__new(self) }
            }

            $(
                /// Converts a value into a [`StaticComObject`](::windows_core::StaticComObject)
                /// suitable for storage in a static (global) variable.
                pub $const fn into_static(self) -> $crate::StaticComObject<Self> {
                    $crate::StaticComObject::from_outer(unsafe { self.into_outer() })
                }
            )?
        }

        impl<$($gp),*> ::core::ops::Deref for $impl_name<$($gp),*> where $($wc)* {
            type Target = $name<$($gp),*>;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                self.__com.get()
            }
        }

        impl<$($gp),*> $crate::IUnknownImpl for $impl_name<$($gp),*> where $($wc)* {
            type Impl = $name<$($gp),*>;
            type Identity = $first;

            #[inline(always)]
            fn get_impl(&self) -> &Self::Impl {
                self.__com.get()
            }

            #[inline(always)]
            fn get_impl_mut(&mut self) -> &mut Self::Impl {
                self.__com.get_mut()
            }

            #[inline(always)]
            fn into_inner(self) -> Self::Impl {
                self.__com.into_inner()
            }

            unsafe fn QueryInterface(
                &self,
                iid: *const $crate::GUID,
                interface: *mut *mut ::core::ffi::c_void,
            ) -> $crate::HRESULT {
                unsafe {
                    self.__com.query_interface::<$first>(
                        iid,
                        interface,
                        true,
                        ::core::option::Option::Some(self as &dyn ::core::any::Any),
                        |iid| {
                            $(
                                if <<$ty as $crate::Interface>::Vtable>::matches(iid)
                                    || (<$ty as $crate::Interface>::INSPECTABLE
                                        && *iid == <$crate::IInspectable as $crate::Interface>::IID)
                                {
                                    return ::core::option::Option::Some($idx);
                                }
                            )+
                            ::core::option::Option::None
                        },
                    )
                }
            }

            #[inline(always)]
            fn AddRef(&self) -> u32 {
                self.__com.count().add_ref()
            }

            #[inline(always)]
            unsafe fn Release(self_: *mut Self) -> u32 {
                unsafe {
                    let remaining = (*self_).__com.count().release();
                    if remaining == 0 {
                        _ = $crate::imp::object_from_raw(self_);
                    }
                    remaining
                }
            }

            #[inline(always)]
            fn is_reference_count_one(&self) -> bool {
                self.__com.count().is_one()
            }

            unsafe fn GetTrustLevel(&self, value: *mut i32) -> $crate::HRESULT {
                if value.is_null() {
                    return $crate::imp::E_POINTER;
                }
                unsafe { *value = 0 };
                $crate::HRESULT(0)
            }

            fn to_object(&self) -> $crate::ComObject<Self::Impl> {
                self.__com.count().add_ref();
                unsafe { $crate::ComObject::from_raw(::core::ptr::NonNull::from(self)) }
            }
        }

        impl<$($gp),*> $crate::ComObjectInner for $name<$($gp),*> where $($wc)* {
            type Outer = $impl_name<$($gp),*>;

            fn into_object(self) -> $crate::ComObject<Self> {
                let ptr = $crate::imp::object_into_raw(unsafe { self.into_outer() });
                unsafe { $crate::ComObject::from_raw(ptr) }
            }
        }

        impl<$($gp),*> $crate::ComObjectInterface<$crate::IUnknown> for $impl_name<$($gp),*> where $($wc)* {
            #[inline(always)]
            fn as_interface_ref(&self) -> $crate::InterfaceRef<'_, $crate::IUnknown> {
                unsafe { self.__com.interface_ref(0) }
            }
        }

        $( $crate::__implement_decl! { @compose $compose $ctx $first } )?
        $( $crate::__implement_decl! { @interface $ctx $idx $ty } )+
    };

    (@interface
        { [$($gp:ident),*] { $($wc:tt)* } [$($const:tt)?] [$($compose:ident)?] $vis:vis $name:ident $impl_name:ident }
        $idx:literal $ty:ty
    ) => {
        impl<$($gp),*> ::core::convert::From<$name<$($gp),*>> for $ty where $($wc)* {
            #[inline(always)]
            fn from(this: $name<$($gp),*>) -> Self {
                $crate::ComObject::new(this).into_interface()
            }
        }

        impl<$($gp),*> $crate::ComObjectInterface<$ty> for $impl_name<$($gp),*> where $($wc)* {
            #[inline(always)]
            fn as_interface_ref(&self) -> $crate::InterfaceRef<'_, $ty> {
                unsafe { self.__com.interface_ref($idx) }
            }
        }

        impl<$($gp),*> $crate::AsImpl<$name<$($gp),*>> for $ty where $($wc)* {
            #[inline(always)]
            unsafe fn as_impl_ptr(&self) -> ::core::ptr::NonNull<$name<$($gp),*>> {
                unsafe {
                    let this = ($crate::Interface::as_raw(self) as *mut *mut ::core::ffi::c_void)
                        .sub($idx) as *mut $impl_name<$($gp),*>;
                    ::core::ptr::NonNull::new_unchecked($crate::imp::ComCell::this_ptr(&raw mut (*this).__com))
                }
            }
        }
    };

    (@compose compose
        { [$($gp:ident),*] { $($wc:tt)* } [$($const:tt)?] [$($compose:ident)?] $vis:vis $name:ident $impl_name:ident }
        $first:ty
    ) => {
        impl<$($gp),*> $crate::Compose for $name<$($gp),*> where $($wc)* {
            unsafe fn compose<'a>(
                implementation: Self,
            ) -> ($crate::IInspectable, &'a mut ::core::option::Option<$crate::IInspectable>) {
                unsafe {
                    let identity: $crate::IUnknown = $crate::ComObject::new(implementation).into_interface();
                    let this = $crate::Interface::as_raw(&identity) as *mut $impl_name<$($gp),*>;
                    $crate::imp::ComCell::compose::<$first>(&raw mut (*this).__com, identity)
                }
            }
        }
    };

    (@base) => { () };
    (@base compose) => { $crate::ComposeBase };
    (@base_new) => { () };
    (@base_new compose) => { $crate::ComposeBase::new() };
}
