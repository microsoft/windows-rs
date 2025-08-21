use windows_core::*;

pub const S_OK: HRESULT = HRESULT(0);
pub const E_POINTER: HRESULT = HRESULT(0x80004003_u32 as _);

#[no_mangle]
unsafe extern "system" fn Add(left: u32, right: u32, result: *mut u32) -> HRESULT {
    if result.is_null() {
        return E_POINTER;
    }

    unsafe {
        *result = left + right;
    }

    S_OK
}

#[no_mangle]
unsafe extern "system" fn Concat(left: PCWSTR, right: PCWSTR, result: *mut BSTR) -> HRESULT {
    if left.is_null() || right.is_null() || result.is_null() {
        return E_POINTER;
    }

    unsafe {
        let left = left.as_wide();
        let right = right.as_wide();
        let combined: Vec<u16> = [left, right].concat();

        *result = BSTR::from_wide(&combined);
    }

    S_OK
}
