use core::ffi::c_void;

// Event tokens need a stable, one-to-one encoding, not a dereferenceable pointer.
#[cfg(miri)]
#[unsafe(export_name = "EncodePointer")]
pub extern "system" fn encode_pointer(pointer: *mut c_void) -> *mut c_void {
    core::ptr::without_provenance_mut(pointer.addr().rotate_left(13))
}

#[unsafe(export_name = "AbiFill")]
pub unsafe extern "system" fn fill(values: *mut u32, count: u32, written: *mut u32) -> u32 {
    unsafe {
        written.write(count);
        if values.is_null() {
            return 0;
        }
        for index in 0..count {
            values.add(index as usize).write(index + 10);
        }
    }
    1
}

#[unsafe(export_name = "AbiSum")]
pub unsafe extern "system" fn sum(count: u32, values: *const u32) -> u32 {
    if values.is_null() {
        return 0;
    }
    let mut result = 0u32;
    for index in 0..count {
        result = result.wrapping_add(unsafe { values.add(index as usize).read() });
    }
    result
}

#[unsafe(export_name = "AbiSelect")]
pub unsafe extern "system" fn select(values: *mut u32, count: u32, selected: *mut *mut u32) -> u32 {
    unsafe {
        if count == 0 {
            selected.write(core::ptr::null_mut());
            0
        } else {
            selected.write(values.add(count as usize - 1));
            1
        }
    }
}

#[unsafe(export_name = "AbiInvoke")]
pub unsafe extern "system" fn invoke(
    callback: Option<unsafe extern "system" fn(*mut c_void, *mut u32, u32) -> u64>,
    context: *mut c_void,
    values: *mut u32,
    count: u32,
) -> u64 {
    callback.map_or(0, |callback| unsafe { callback(context, values, count) })
}

#[unsafe(export_name = "AbiInvokeCdecl")]
pub unsafe extern "C" fn invoke_cdecl(
    callback: Option<unsafe extern "C" fn(*mut c_void, *mut u32, u32) -> u64>,
    context: *mut c_void,
    values: *mut u32,
    count: u32,
) -> u64 {
    callback.map_or(0, |callback| unsafe { callback(context, values, count) })
}
