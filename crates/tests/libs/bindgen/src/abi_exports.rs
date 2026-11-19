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

// Raw ABI fields for Test.OwnedValue in ownership.rdl.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OwnedValue {
    text: *mut c_void,
    enabled: bool,
    number: f64,
    object: *mut c_void,
}

fn owned_value() -> OwnedValue {
    OwnedValue {
        text: core::ptr::null_mut(),
        enabled: true,
        number: 42.25,
        object: core::ptr::null_mut(),
    }
}

unsafe fn write_result<T>(status: i32, output: *mut T, value: T) -> i32 {
    unsafe {
        if status < 0 {
            output.cast::<u8>().write_bytes(0xff, size_of::<T>());
        } else {
            output.write(value);
        }
    }
    status
}

#[unsafe(export_name = "GetOwned")]
pub unsafe extern "system" fn get_owned(status: i32, output: *mut OwnedValue) -> i32 {
    unsafe { write_result(status, output, owned_value()) }
}

#[unsafe(export_name = "GetOwnedArray")]
pub unsafe extern "system" fn get_owned_array(status: i32, output: *mut [OwnedValue; 2]) -> i32 {
    unsafe { write_result(status, output, [owned_value(); 2]) }
}

#[unsafe(export_name = "GetOwnedNestedArray")]
pub unsafe extern "system" fn get_owned_nested_array(
    status: i32,
    output: *mut [[OwnedValue; 2]; 2],
) -> i32 {
    unsafe { write_result(status, output, [[owned_value(); 2]; 2]) }
}

#[unsafe(export_name = "BorrowOwned")]
pub extern "system" fn borrow_owned(value: *const OwnedValue) -> *const OwnedValue {
    value
}
