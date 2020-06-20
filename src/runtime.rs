use crate::{hstring, ErrorCode, Guid, RawPtr};

pub const LOAD_LIBRARY_SEARCH_SYSTEM32: u32 = 0x0000_0800;

#[link(name = "kernel32")]
extern "system" {
    pub fn GetProcessHeap() -> RawPtr;
    pub fn HeapAlloc(heap: RawPtr, flags: u32, bytes: usize) -> RawPtr;
    pub fn HeapFree(heap: RawPtr, flags: u32, ptr: RawPtr) -> i32;

    pub fn CreateEventW(security: RawPtr, manual: i32, state: i32, name: RawPtr) -> RawPtr;
    pub fn SetEvent(handle: RawPtr) -> i32;
    pub fn WaitForSingleObject(handle: RawPtr, milliseconds: u32) -> u32;
    pub fn CloseHandle(handle: RawPtr) -> i32;

    pub fn LoadLibraryExW(name: *const u16, file: RawPtr, flags: u32) -> RawPtr;
    pub fn FreeLibrary(library: RawPtr) -> i32;
    pub fn GetProcAddress(library: RawPtr, name: *const u8) -> RawPtr;

    pub fn GetLastError() -> u32;

    pub fn FormatMessageW(
        flags: u32,
        source: RawPtr,
        code: ErrorCode,
        language: u32,
        buffer: *mut *mut u16,
        size: u32,
        args: RawPtr,
    ) -> u32;
}

fn load_proc(library_name: &str, sym: &str) -> Result<RawPtr, ErrorCode> {
    let library_name = library_name
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    let library = unsafe {
        LoadLibraryExW(
            library_name.as_ptr(),
            std::ptr::null_mut(),
            LOAD_LIBRARY_SEARCH_SYSTEM32,
        )
    };
    if library.is_null() {
        return Err(ErrorCode::last_win32_error());
    }
    let addr = unsafe { GetProcAddress(library, sym.as_ptr()) };
    if addr.is_null() {
        return Err(ErrorCode::last_win32_error());
    }
    Ok(addr)
}

macro_rules! demand_load {
    ( $( $library:literal {
        $(pub fn $sym:ident ( $( $param: ident : $pty: ty ),* $(,)? ) -> $rt: ty;)*
    } )* ) => {
        $($(
            #[allow(non_snake_case)]
            pub unsafe fn $sym( $( $param: $pty ),* ) -> Result<$rt, ErrorCode> {
                // Thread-safe initialize VALUE to load_proc() result once, including any error.
                // Could replace with static_init crate?
                use std::{sync::Once, mem::MaybeUninit};
                static ONCE: Once = Once::new();
                static mut VALUE: MaybeUninit<Result<RawPtr, ErrorCode>> = MaybeUninit::uninit();
                ONCE.call_once(|| {
                    VALUE = MaybeUninit::new(
                        load_proc($library, concat!(stringify!($sym), "\0"))
                    )
                });

                // transmute() doesn't work on generic types, as you can't constrain to a
                // function pointer, so it must be done here outside load_proc().
                type FnPtr = extern "system" fn ( $( $param: $pty ),* ) -> $rt;
                let f = std::mem::transmute::<RawPtr, FnPtr>(VALUE.assume_init()?);
                Ok( (f)( $( $param ),* ) )
            }
        )*)*
    };
}

demand_load! {
    "ole32.dll" {
        pub fn CoIncrementMTAUsage(cookie: *mut RawPtr) -> ErrorCode;
    }
    "combase.dll" {
        pub fn RoGetActivationFactory(hstring: *mut hstring::Header, interface: &Guid, result: *mut RawPtr) -> ErrorCode;
        pub fn SetRestrictedErrorInfo(info: RawPtr) -> ErrorCode;
        pub fn RoOriginateError(code: ErrorCode, message: RawPtr) -> i32;
    }
}

#[link(name = "oleaut32")]
extern "system" {
    pub fn SysStringLen(bstr: *mut u16) -> u32;
    pub fn SysFreeString(bstr: *mut u16);
    pub fn GetErrorInfo(reserved: u32, info: *mut *mut u16) -> ErrorCode;
}

#[link(name = "ole32")]
extern "system" {
    pub fn CoTaskMemFree(ptr: RawPtr);
}
