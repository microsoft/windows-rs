#![allow(non_snake_case, non_camel_case_types, dead_code)]
#![cfg_attr(named_import_probe, no_std)]
#![cfg_attr(named_import_probe, no_main)]

include!("bindings.rs");

const SOURCE: &[u8] = b"#include \"color.h\"\nfloat4 main() : SV_TARGET { return get_color(); }\n";
const HEADER: &[u8] = b"float4 get_color() { return float4(1, 0, 0, 1); }\n";

#[repr(C)]
struct Include {
    vtable: *const ID3DInclude_Vtbl,
    opens: u32,
    closes: u32,
}

unsafe extern "system" fn open(
    this: *mut core::ffi::c_void,
    kind: D3D_INCLUDE_TYPE,
    name: LPCSTR,
    parent: LPCVOID,
    data: *mut LPCVOID,
    size: *mut UINT,
) -> HRESULT {
    unsafe {
        assert_eq!(kind, D3D_INCLUDE_LOCAL);
        assert_eq!(core::ffi::CStr::from_ptr(name).to_bytes(), b"color.h");
        assert!(parent.is_null());
        (*this.cast::<Include>()).opens += 1;
        *data = HEADER.as_ptr().cast();
        *size = HEADER.len() as u32;
    }
    0
}

unsafe extern "system" fn close(this: *mut core::ffi::c_void, data: LPCVOID) -> HRESULT {
    unsafe {
        assert_eq!(data, HEADER.as_ptr().cast());
        (*this.cast::<Include>()).closes += 1;
    }
    0
}

fn check_calls() {
    let pointer = core::mem::size_of::<usize>();
    assert_eq!(core::mem::size_of::<IAVIPersistFile_Vtbl>(), 10 * pointer);
    assert_eq!(
        core::mem::offset_of!(IAVIPersistFile_Vtbl, Reserved1),
        9 * pointer
    );
    let vtable = ID3DInclude_Vtbl {
        Open: open,
        Close: close,
    };
    let mut include = Include {
        vtable: &vtable,
        opens: 0,
        closes: 0,
    };
    let mut code = core::ptr::null_mut();
    let mut errors = core::ptr::null_mut();
    unsafe {
        let result = D3DCompile(
            SOURCE.as_ptr().cast(),
            SOURCE.len() as SIZE_T,
            core::ptr::null(),
            core::ptr::null(),
            (&raw mut include).cast(),
            c"main".as_ptr(),
            c"ps_5_0".as_ptr(),
            0,
            0,
            &raw mut code,
            &raw mut errors,
        );
        if !errors.is_null() {
            let vtable = *errors.cast::<*const ID3D10Blob_Vtbl>();
            ((*vtable).base__.Release)(errors);
        }
        assert_eq!(result, 0);
        assert!(!code.is_null());
        let vtable = *code.cast::<*const ID3D10Blob_Vtbl>();
        assert!(((*vtable).GetBufferSize)(code) > 0);
        assert!(!((*vtable).GetBufferPointer)(code).is_null());
        ((*vtable).base__.Release)(code);
    }
    assert_eq!(include.opens, 1);
    assert_eq!(include.closes, 1);
}

#[cfg(not(named_import_probe))]
fn main() {
    check_calls();
}

#[cfg(named_import_probe)]
windows_link::link!("kernel32.dll" "system" fn ExitProcess(code: u32) -> !);

#[cfg(named_import_probe)]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    unsafe { ExitProcess(1) }
}

#[cfg(named_import_probe)]
#[unsafe(no_mangle)]
extern "C" fn mainCRTStartup() -> ! {
    check_calls();
    unsafe { ExitProcess(0) }
}
