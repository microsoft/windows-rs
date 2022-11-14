mod abi;
mod agile_reference;
mod array;
mod as_impl;
pub(crate) mod bindings;
mod borrowed;
mod delay_load;
mod error;
mod event;
mod factory_cache;
mod generic_factory;
mod guid;
mod heap;
mod hresult;
mod inspectable;
mod interface;
mod param;
mod ref_count;
mod runtime_name;
mod runtime_type;
mod scoped_interface;
mod sha1;
mod strings;
mod unknown;
mod vtable;
mod waiter;
mod weak;
mod weak_ref_count;

#[doc(hidden)]
pub use abi::*;
pub use agile_reference::*;
pub use array::*;
#[doc(hidden)]
pub use as_impl::*;
pub use borrowed::*;
pub use delay_load::*;
pub use error::*;
pub use event::*;
pub use factory_cache::*;
#[doc(hidden)]
pub use generic_factory::*;
pub use guid::*;
pub(crate) use heap::*;
pub use hresult::*;
pub use inspectable::*;
pub use interface::*;
pub use param::*;
#[doc(hidden)]
pub use ref_count::*;
#[doc(hidden)]
pub use runtime_name::*;
#[doc(hidden)]
pub use runtime_type::*;
pub use scoped_interface::*;
#[doc(hidden)]
pub use sha1::*;
pub use strings::*;
pub use unknown::*;
pub use vtable::*;
#[doc(hidden)]
pub use waiter::*;
pub use weak::*;
#[doc(hidden)]
pub use weak_ref_count::*;

/// A specialized [`Result`] type that provides Windows error information.
pub type Result<T> = std::result::Result<T, Error>;

#[doc(hidden)]
pub use bindings::IAgileObject;

#[doc(hidden)]
#[cfg(feature = "implement")]
pub use windows_implement::implement;

#[doc(hidden)]
#[cfg(feature = "interface")]
pub use windows_interface::interface;

extern "C" {
    #[doc(hidden)]
    pub fn memcmp(left: *const std::ffi::c_void, right: *const std::ffi::c_void, len: usize) -> i32;
}

#[doc(hidden)]
pub extern crate alloc;

fn wide_trim_end(mut wide: &[u16]) -> &[u16] {
    while let Some(last) = wide.last() {
        match last {
            32 | 9..=13 => wide = &wide[..wide.len() - 1],
            _ => break,
        }
    }
    wide
}

#[doc(hidden)]
#[macro_export]
macro_rules! interface_hierarchy {
    ($child:ty, $parent:ty) => {
        impl ::std::convert::From<$child> for $parent {
            fn from(value: $child) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&$child> for &$parent {
            fn from(value: &$child) -> Self {
                unsafe { ::std::mem::transmute(value) }
            }
        }
        impl ::std::convert::From<&$child> for $parent {
            fn from(value: &$child) -> Self {
                unsafe { ::std::mem::transmute(::std::clone::Clone::clone(value)) }
            }
        }
    };
    ($child:ty, $first:ty, $($rest:ty),+) => {
        $crate::core::interface_hierarchy!($child, $first);
        $crate::core::interface_hierarchy!($child, $($rest),+);
    };
}

#[doc(hidden)]
pub use interface_hierarchy;

#[cfg(not(windows_delay_load))]
#[macro_export]
macro_rules! link {
    ($library:literal $abi:literal fn $name:ident($($arg:ident: $argty:ty),*)->$ret:ty) => (
        #[link(name = "windows")]
        extern $abi {
            pub fn $name($($arg: $argty),*) -> $ret;
        }
    )
}

#[cfg(windows_delay_load)]
#[macro_export]
macro_rules! link {
    ($library:literal $abi:literal fn $name:ident($($arg:ident: $argty:ty),*)->$ret:ty) => (
        #[cfg(target_arch = "x86")]
        type $name = ::core::option::Option<unsafe extern $abi fn($($argty),*) -> $ret>;
        #[cfg(not(target_arch = "x86"))]
        type $name = ::core::option::Option<unsafe extern "system" fn($($argty),*) -> $ret>;
        pub unsafe fn $name($($arg: $argty),*) -> $ret {
            static mut CACHE: $name = None;
            unsafe {
                if CACHE.is_none() {
                    let name = ::core::concat!(::core::stringify!($name), "\0");
                    CACHE = $crate::core::delay_load($crate::core::s!($library), $crate::core::PCSTR(name.as_ptr()));
                }
                CACHE.unwrap()($($arg),*)
            }
        }
    )
}

#[doc(hidden)]
pub use crate::link;
