// This is a test specifically for the breaking change discussed in https://github.com/microsoft/windows-rs/pull/3669
// and https://github.com/microsoft/windows-rs/issues/3626 where a function pointer is stored. All of this can be
// removed if the hardcoding of "C" is removed in a future breaking change.

windows_link::link!("kernel32.dll" "system" fn GetTickCount() -> u32);

#[cfg(target_arch = "x86")]
type GetTickCountType = unsafe extern "system" fn() -> u32;

#[cfg(target_arch = "x86_64")]
type GetTickCountType = unsafe extern "C" fn() -> u32;

static GET_TICK_COUNT: GetTickCountType = GetTickCount;

#[test]
fn store_ptr() {
    unsafe {
        GET_TICK_COUNT();
    }
}
