use crate::{hstring, ErrorCode, Guid, RawPtr};

#[link(name = "kernel32")]
extern "system" {
    pub fn GetProcessHeap() -> RawPtr;
    pub fn HeapAlloc(heap: RawPtr, flags: u32, bytes: usize) -> RawPtr;
    pub fn HeapFree(heap: RawPtr, flags: u32, ptr: RawPtr) -> i32;

    pub fn CreateEventW(security: RawPtr, manual: i32, state: i32, name: RawPtr) -> RawPtr;
    pub fn SetEvent(handle: RawPtr) -> i32;
    pub fn WaitForSingleObject(handle: RawPtr, milliseconds: u32) -> u32;
    pub fn CloseHandle(handle: RawPtr) -> i32;

    pub fn LoadLibraryW(name: *const u16) -> RawPtr;
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

unsafe fn load_proc(library: &[u16], sym: &str) -> Result<RawPtr, ErrorCode> {
    let library = LoadLibraryW(library.as_ptr());
    if library.is_null() {
        return Err(ErrorCode(0x8007_0000 | GetLastError()));
    }
    let addr = GetProcAddress(library, sym.as_ptr());
    if addr.is_null() {
        return Err(ErrorCode(0x8007_0000 | GetLastError()));
    }
    Ok(addr)
}

macro_rules! demand_load {
    ( $( $library:literal {
        $(pub extern $($abi: literal)? fn $sym:ident ( $( $param: ident : $pty: ty ),* $(,)? ) -> $rt: ty;)*
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
                        load_proc(wchar::wch_c!($library), concat!(stringify!($sym), "\0"))
                    )
                });

                // Transmute doesn't work on generic types, as you can't constrain to a
                // function pointer.
                type FnPtr = extern $($abi)? fn ( $( $param: $pty ),* ) -> $rt;
                let f = std::mem::transmute::<RawPtr, FnPtr>(VALUE.assume_init()?);
                Ok( (f)( $( $param ),* ) )
            }
        )*)*
    };
}

demand_load! {
    "ole32.dll" {
        pub extern "system" fn CoIncrementMTAUsage(cookie: *mut RawPtr) -> ErrorCode;
    }
    "combase.dll" {
        pub extern "system" fn RoGetActivationFactory(hstring: *mut hstring::Header, interface: &Guid, result: *mut RawPtr) -> ErrorCode;
        pub extern "system" fn SetRestrictedErrorInfo(info: RawPtr) -> ErrorCode;
        pub extern "system" fn RoOriginateError(code: ErrorCode, message: RawPtr) -> i32;
    }
}

#[link(name = "oleaut32")]
extern "system" {
    pub fn SysStringLen(bstr: *mut u16) -> u32;
    pub fn SysFreeString(bstr: *mut u16);
    pub fn GetErrorInfo(reserved: u32, info: *mut *mut u16) -> ErrorCode;
}
