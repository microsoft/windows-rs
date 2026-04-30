#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

// `windows-strings` keeps its `HSTRING` / `BSTR` types portable by routing all
// allocations through this module. On Windows the calls go to the same
// `kernel32` process heap and `oleaut32` BSTR allocator that Win32 uses, so
// strings allocated here can interoperate with native code. On non-Windows
// targets the calls are serviced by the Rust global allocator using a layout
// that mirrors the public Win32 contract (BSTR keeps its 4-byte length prefix
// followed by the wide chars and a null terminator).

#[cfg(windows)]
mod sys {
    use super::*;
    windows_link::link!("kernel32.dll" "system" fn GetProcessHeap() -> HANDLE);
    windows_link::link!("kernel32.dll" "system" fn HeapAlloc(hheap : HANDLE, dwflags : HEAP_FLAGS, dwbytes : usize) -> *mut core::ffi::c_void);
    windows_link::link!("kernel32.dll" "system" fn HeapFree(hheap : HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> BOOL);
    windows_link::link!("oleaut32.dll" "system" fn SysAllocStringLen(strin : PCWSTR, ui : u32) -> BSTR);
    windows_link::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : BSTR));
    windows_link::link!("oleaut32.dll" "system" fn SysStringLen(pbstr : BSTR) -> u32);
}

pub type BOOL = i32;
pub type BSTR = *const u16;
pub type HANDLE = *mut core::ffi::c_void;
pub type HEAP_FLAGS = u32;
pub type PCWSTR = *const u16;

/// Allocates `bytes` bytes of memory aligned to at least `align` for use as
/// the backing storage of an `HSTRING` header.
///
/// # Safety
/// `bytes` must be non-zero and `align` must be a power of two.
pub unsafe fn heap_alloc(bytes: usize, align: usize) -> *mut u8 {
    #[cfg(windows)]
    {
        // The Win32 process heap returns memory aligned to at least 8 bytes
        // (16 on 64-bit), which is sufficient for `HStringHeader`.
        let _ = align;
        unsafe { sys::HeapAlloc(sys::GetProcessHeap(), 0, bytes) as *mut u8 }
    }
    #[cfg(not(windows))]
    {
        let layout = match alloc::alloc::Layout::from_size_align(bytes, align) {
            Ok(layout) => layout,
            Err(_) => return core::ptr::null_mut(),
        };
        unsafe { alloc::alloc::alloc(layout) }
    }
}

/// Frees a block previously returned by `heap_alloc`.
///
/// # Safety
/// `ptr` must have been returned by `heap_alloc(bytes, align)` and not yet
/// freed.
pub unsafe fn heap_free(ptr: *mut u8, bytes: usize, align: usize) {
    #[cfg(windows)]
    {
        let _ = (bytes, align);
        unsafe {
            sys::HeapFree(sys::GetProcessHeap(), 0, ptr as *const _);
        }
    }
    #[cfg(not(windows))]
    {
        if let Ok(layout) = alloc::alloc::Layout::from_size_align(bytes, align) {
            unsafe { alloc::alloc::dealloc(ptr, layout) };
        }
    }
}

/// Allocates a new BSTR copying `len` u16 values from `data`.
///
/// # Safety
/// `data` must be valid for reads of `len` u16 values, or null (in which case
/// the destination is zero-initialised, matching the Win32 contract).
pub unsafe fn SysAllocStringLen(data: PCWSTR, len: u32) -> BSTR {
    #[cfg(windows)]
    {
        unsafe { sys::SysAllocStringLen(data, len) }
    }
    #[cfg(not(windows))]
    {
        unsafe { non_windows::sys_alloc_string_len(data, len) }
    }
}

/// # Safety
/// `bstr` must have been returned by `SysAllocStringLen` and not yet freed.
pub unsafe fn SysFreeString(bstr: BSTR) {
    #[cfg(windows)]
    {
        unsafe { sys::SysFreeString(bstr) }
    }
    #[cfg(not(windows))]
    {
        unsafe { non_windows::sys_free_string(bstr) }
    }
}

/// # Safety
/// `bstr` must be null or a valid BSTR returned by `SysAllocStringLen`.
pub unsafe fn SysStringLen(bstr: BSTR) -> u32 {
    #[cfg(windows)]
    {
        unsafe { sys::SysStringLen(bstr) }
    }
    #[cfg(not(windows))]
    {
        unsafe { non_windows::sys_string_len(bstr) }
    }
}

#[cfg(not(windows))]
mod non_windows {
    use super::*;

    // BSTR layout (matches Win32): a 4-byte length prefix in *bytes* (excluding
    // both the prefix and the trailing null), followed by `len` u16 wide
    // characters, followed by a trailing u16 null terminator. The pointer
    // exposed to callers points at the wide characters; the prefix lives at
    // offset -4. We use 4-byte alignment so the prefix is naturally aligned.
    const BSTR_PREFIX_BYTES: usize = core::mem::size_of::<u32>();
    const BSTR_NULL_BYTES: usize = core::mem::size_of::<u16>();
    const BSTR_ALIGN: usize = core::mem::align_of::<u32>();

    fn bstr_total_bytes(len: u32) -> usize {
        BSTR_PREFIX_BYTES + (len as usize) * core::mem::size_of::<u16>() + BSTR_NULL_BYTES
    }

    fn bstr_layout(len: u32) -> alloc::alloc::Layout {
        // Safe: BSTR_ALIGN is a non-zero power of two and the total never
        // overflows isize for any `u32` `len` on any supported target.
        alloc::alloc::Layout::from_size_align(bstr_total_bytes(len), BSTR_ALIGN).unwrap()
    }

    pub unsafe fn sys_alloc_string_len(data: PCWSTR, len: u32) -> BSTR {
        let layout = bstr_layout(len);
        let raw = unsafe { alloc::alloc::alloc(layout) };
        if raw.is_null() {
            return core::ptr::null();
        }
        unsafe {
            // Write the length prefix in bytes (excludes prefix and null).
            let prefix_bytes = (len as u32) * core::mem::size_of::<u16>() as u32;
            (raw as *mut u32).write(prefix_bytes);

            let chars = raw.add(BSTR_PREFIX_BYTES) as *mut u16;
            if data.is_null() {
                core::ptr::write_bytes(chars, 0, len as usize);
            } else {
                core::ptr::copy_nonoverlapping(data, chars, len as usize);
            }
            // Null terminator.
            chars.add(len as usize).write(0);
            chars as BSTR
        }
    }

    pub unsafe fn sys_string_len(bstr: BSTR) -> u32 {
        if bstr.is_null() {
            return 0;
        }
        unsafe {
            let prefix = (bstr as *const u8).sub(BSTR_PREFIX_BYTES) as *const u32;
            prefix.read() / core::mem::size_of::<u16>() as u32
        }
    }

    pub unsafe fn sys_free_string(bstr: BSTR) {
        if bstr.is_null() {
            return;
        }
        unsafe {
            let prefix = (bstr as *const u8).sub(BSTR_PREFIX_BYTES) as *mut u8;
            let bytes = (prefix as *const u32).read() as usize;
            let len = (bytes / core::mem::size_of::<u16>()) as u32;
            alloc::alloc::dealloc(prefix, bstr_layout(len));
        }
    }
}
