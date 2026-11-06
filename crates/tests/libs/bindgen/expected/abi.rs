#[inline]
pub unsafe fn AbiFill(values: Option<*mut u32>, count: u32, written: *mut u32) -> u32 {
    windows_core::link!("bindgen_abi.dll" "system" fn AbiFill(values : *mut u32, count : u32, written : *mut u32) -> u32);
    unsafe {
        AbiFill(
            values.unwrap_or(core::mem::zeroed()) as _,
            count,
            written as _,
        )
    }
}
#[inline]
pub unsafe fn AbiInvoke(
    callback: AbiCallback,
    context: *mut core::ffi::c_void,
    values: *mut u32,
    count: u32,
) -> u64 {
    windows_core::link!("bindgen_abi.dll" "system" fn AbiInvoke(callback : AbiCallback, context : *mut core::ffi::c_void, values : *mut u32, count : u32) -> u64);
    unsafe { AbiInvoke(callback, context as _, values as _, count) }
}
#[inline]
pub unsafe fn AbiInvokeCdecl(
    callback: AbiCdeclCallback,
    context: *mut core::ffi::c_void,
    values: *mut u32,
    count: u32,
) -> u64 {
    windows_core::link!("bindgen_abi.dll" "C" fn AbiInvokeCdecl(callback : AbiCdeclCallback, context : *mut core::ffi::c_void, values : *mut u32, count : u32) -> u64);
    unsafe { AbiInvokeCdecl(callback, context as _, values as _, count) }
}
#[inline]
pub unsafe fn AbiSelect(values: *mut u32, count: u32, selected: *mut *mut u32) -> u32 {
    windows_core::link!("bindgen_abi.dll" "system" fn AbiSelect(values : *mut u32, count : u32, selected : *mut *mut u32) -> u32);
    unsafe { AbiSelect(values as _, count, selected as _) }
}
#[inline]
pub unsafe fn AbiSum(count: u32, values: Option<*const u32>) -> u32 {
    windows_core::link!("bindgen_abi.dll" "system" fn AbiSum(count : u32, values : *const u32) -> u32);
    unsafe { AbiSum(count, values.unwrap_or(core::mem::zeroed()) as _) }
}
pub type AbiCallback = Option<
    unsafe extern "system" fn(context: *mut core::ffi::c_void, values: *mut u32, count: u32) -> u64,
>;
pub type AbiCdeclCallback = Option<
    unsafe extern "C" fn(context: *mut core::ffi::c_void, values: *mut u32, count: u32) -> u64,
>;
