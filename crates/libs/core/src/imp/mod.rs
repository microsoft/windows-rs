mod bindings;
mod com_bindings;
mod delay_load;
mod factory_cache;
mod generic_factory;
mod heap;
mod ref_count;
mod sha1;
mod waiter;
mod weak_ref_count;

pub use bindings::{CloseHandle, CoCreateGuid, CoTaskMemAlloc, CoTaskMemFree, CreateEventW, EncodePointer, FormatMessageW, FreeLibrary, GetErrorInfo, GetLastError, GetProcAddress, GetProcessHeap, HeapAlloc, HeapFree, LoadLibraryExA, RoGetAgileReference, SetErrorInfo, SetEvent, SysAllocStringLen, SysFreeString, SysStringLen, WaitForSingleObject, AGILEREFERENCE_DEFAULT, CLASS_E_CLASSNOTAVAILABLE, CO_E_NOTINITIALIZED, ERROR_NO_UNICODE_TRANSLATION, E_BOUNDS, E_NOINTERFACE, E_OUTOFMEMORY, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, GUID, HRESULT, JSCRIPT_E_CANTEXECUTE, LOAD_LIBRARY_SEARCH_DEFAULT_DIRS, RPC_E_DISCONNECTED, S_OK};
pub use com_bindings::*;
pub use delay_load::*;
pub use factory_cache::*;
pub use generic_factory::*;
pub use heap::*;
pub use ref_count::*;
pub use sha1::*;
pub use waiter::*;
pub use weak_ref_count::*;

// This is a workaround since 1.48 does not include `bool::then_some`.
pub fn then_some<T>(value: bool, t: T) -> Option<T> {
    if value {
        Some(t)
    } else {
        None
    }
}

// This is a workaround since 1.48 does not include `bool::then`.
pub fn then<T, F: FnOnce() -> T>(value: bool, f: F) -> Option<T> {
    if value {
        Some(f())
    } else {
        None
    }
}

pub fn wide_trim_end(mut wide: &[u16]) -> &[u16] {
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
        impl ::windows_core::CanInto<$parent> for $child {}
    };
    ($child:ty, $first:ty, $($rest:ty),+) => {
        $crate::imp::interface_hierarchy!($child, $first);
        $crate::imp::interface_hierarchy!($child, $($rest),+);
    };
}

#[doc(hidden)]
pub use interface_hierarchy;
