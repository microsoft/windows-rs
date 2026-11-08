#[cfg(windows)]
include!("windows.rs");

use alloc::boxed::Box;

mod agile;
mod array_slice;
#[cfg_attr(not(windows), allow(dead_code))]
#[cfg_attr(windows, expect(dead_code))]
mod bindings;
mod can_into;
#[cfg(windows_cast_diagnostics)]
mod cast_diagnostics;
mod com_cell;
mod delegate_box;
#[cfg(windows_cast_diagnostics)]
mod live_objects;
mod ref_count;
mod sha1;
mod weak_ref_count;

pub use crate::r#type::{
    AbiType, CloneType, CopyType, InterfaceType, Type, TypeKind, ref_as_default,
};
pub use agile::*;
pub use array_slice::*;
pub use bindings::*;
pub use can_into::*;
#[cfg(windows_cast_diagnostics)]
pub use cast_diagnostics::*;
pub use com_cell::*;
pub use delegate_box::*;
#[cfg(windows_cast_diagnostics)]
pub use live_objects::{LiveObjects, live_objects};
pub use ref_count::*;
pub use sha1::*;
pub use weak_ref_count::*;

/// Returns an opaque, process-specific encoding of a pointer suitable for use
/// as a delegate token. On Windows this calls `EncodePointer` so tokens cannot
/// be forged from raw pointer values; on non-Windows targets the raw pointer
/// value is returned as-is because there is no equivalent OS primitive and the
/// token is already opaque to callers and only meaningful within this process.
///
/// The pointer is treated as an opaque value and is never dereferenced.
#[inline]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn encode_pointer(ptr: *const core::ffi::c_void) -> i64 {
    #[cfg(windows)]
    unsafe {
        EncodePointer(ptr.cast_mut()) as i64
    }
    #[cfg(not(windows))]
    {
        ptr as i64
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! interface_hierarchy {
    ($child:ident, $parent:ty) => {
        impl ::windows_core::imp::CanInto<$parent> for $child {}
        impl ::core::convert::From<&$child> for &$parent {
            fn from(value: &$child) -> Self {
                unsafe { ::core::mem::transmute(value) }
            }
        }
        impl ::core::convert::From<$child> for $parent {
            fn from(value: $child) -> Self {
                unsafe { ::core::mem::transmute(value) }
            }
        }
    };
    ($child:ident, $first:ty, $($rest:ty),+) => {
        $crate::imp::interface_hierarchy!($child, $first);
        $crate::imp::interface_hierarchy!($child, $($rest),+);
    };
}

#[doc(hidden)]
pub use interface_hierarchy;

#[doc(hidden)]
#[macro_export]
macro_rules! required_hierarchy {
    ($child:ident, $parent:ty) => {
        impl ::windows_core::imp::CanInto<$parent> for $child { const QUERY: bool = true; }
    };
    ($child:ident, $first:ty, $($rest:ty),+) => {
        $crate::imp::required_hierarchy!($child, $first);
        $crate::imp::required_hierarchy!($child, $($rest),+);
    };
}

#[doc(hidden)]
pub use required_hierarchy;

#[doc(hidden)]
#[macro_export]
macro_rules! define_interface {
    ($name:ident, $vtbl:ident, $iid:literal) => {
        $crate::imp::define_interface!(@unknown $name, $vtbl, $iid, false);
    };
    ($name:ident, $vtbl:ident, $iid:literal, inspectable) => {
        $crate::imp::define_interface!(@unknown $name, $vtbl, $iid, true);
    };
    (@unknown $name:ident, $vtbl:ident, $iid:literal, $inspectable:literal) => {
        #[repr(transparent)]
        #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
        pub struct $name(::windows_core::IUnknown);
        unsafe impl ::windows_core::Interface for $name {
            type Vtable = $vtbl;
            const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128($iid);
            const INSPECTABLE: bool = $inspectable;
        }
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> core::fmt::Result {
                f.debug_tuple(stringify!($name))
                    .field(&::windows_core::Interface::as_raw(self))
                    .finish()
            }
        }
    };
    ($name:ident, $vtbl:ident) => {
        #[repr(transparent)]
        #[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::clone::Clone)]
        pub struct $name(::core::ptr::NonNull<::core::ffi::c_void>);
        unsafe impl ::windows_core::Interface for $name {
            type Vtable = $vtbl;
            const IID: ::windows_core::GUID = ::windows_core::GUID::zeroed();
            const UNKNOWN: bool = false;
        }
        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> core::fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }
    };
}

#[doc(hidden)]
pub use define_interface;

/// Declares a WinRT interface. The optional runtime name includes the arity for generic interfaces.
#[doc(hidden)]
#[macro_export]
macro_rules! define_winrt_interface {
    ($name:ident, $vtbl:ident, $iid:literal $(, $runtime_name:literal)?) => {
        $crate::imp::define_interface!($name, $vtbl, $iid, inspectable);
        impl $crate::RuntimeType for $name {
            const SIGNATURE: $crate::imp::ConstBuffer =
                $crate::imp::ConstBuffer::for_interface::<Self>();
            $(
                const NAME: $crate::imp::ConstBuffer =
                    $crate::imp::ConstBuffer::from_slice($runtime_name);
            )?
        }
    };
    ($name:ident<$($generic:ident),+ $(,)?>, $vtbl:ty, $iid:literal $(, $runtime_name:literal)?) => {
        #[repr(transparent)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name<$($generic: $crate::RuntimeType + 'static),+>(
            $crate::IUnknown,
            $(::core::marker::PhantomData<$generic>),+
        );
        impl<$($generic: $crate::RuntimeType + 'static),+>
            $crate::imp::CanInto<$crate::IUnknown> for $name<$($generic),+> {}
        impl<$($generic: $crate::RuntimeType + 'static),+>
            $crate::imp::CanInto<$crate::IInspectable> for $name<$($generic),+> {}
        unsafe impl<$($generic: $crate::RuntimeType + 'static),+>
            $crate::Interface for $name<$($generic),+>
        {
            type Vtable = $vtbl;
            const IID: $crate::GUID =
                $crate::GUID::from_signature(<Self as $crate::RuntimeType>::SIGNATURE);
            const INSPECTABLE: bool = true;
        }
        impl<$($generic: $crate::RuntimeType + 'static),+>
            $crate::RuntimeType for $name<$($generic),+>
        {
            const SIGNATURE: $crate::imp::ConstBuffer =
                $crate::imp::ConstBuffer::new()
                    .push_slice(b"pinterface(").push_guid(&$crate::GUID::from_u128($iid))
                    $(.push_slice(b";").push_other(<$generic as $crate::RuntimeType>::SIGNATURE))+
                    .push_slice(b")");
            $crate::imp::define_winrt_interface!(@name [$($runtime_name)?] [$($generic),+]);
        }
    };
    (@name [] [$($generic:ident),+]) => {};
    (@name [$runtime_name:literal] [$first:ident $(, $rest:ident)*]) => {
        const NAME: $crate::imp::ConstBuffer =
            $crate::imp::ConstBuffer::from_slice($runtime_name)
                .push_slice(b"<").push_other(<$first as $crate::RuntimeType>::NAME)
                $(.push_slice(b", ").push_other(<$rest as $crate::RuntimeType>::NAME))*
                .push_slice(b">");
    };
}

#[doc(hidden)]
pub use define_winrt_interface;

// These helpers let generated code use `windows_core::imp::box_new` etc. without
// depending on `Box` being in scope at the call site.

#[inline(always)]
#[doc(hidden)]
pub fn box_new<T>(value: T) -> Box<T> {
    Box::new(value)
}

#[inline(always)]
#[doc(hidden)]
pub fn box_into_raw<T>(b: Box<T>) -> *mut T {
    Box::into_raw(b)
}

/// Moves an `#[implement]` outer object onto the heap. [`object_from_raw`] frees it.
#[inline(always)]
#[doc(hidden)]
pub fn object_into_raw<T>(outer: T) -> core::ptr::NonNull<T> {
    #[cfg(windows_cast_diagnostics)]
    live_objects::OBJECTS.add();
    unsafe { core::ptr::NonNull::new_unchecked(Box::into_raw(Box::new(outer))) }
}

/// Reclaims an `#[implement]` outer object allocated by [`object_into_raw`].
///
/// # Safety
///
/// `raw` must come from [`object_into_raw`] and must not be used afterwards.
#[inline(always)]
#[doc(hidden)]
pub unsafe fn object_from_raw<T>(raw: *mut T) -> Box<T> {
    #[cfg(windows_cast_diagnostics)]
    live_objects::OBJECTS.remove();
    unsafe { Box::from_raw(raw) }
}
