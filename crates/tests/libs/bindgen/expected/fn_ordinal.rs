#[inline]
pub unsafe fn Named(value: i32) -> i32 {
    windows_core::link!("ordinal_fixture.dll" "system" fn Named(value : i32) -> i32);
    unsafe { Named(value) }
}
#[inline]
pub unsafe fn OrdinalC(value: i32) -> i32 {
    windows_core::link!("ordinal_fixture.dll" "C" ordinal(23) fn OrdinalC(value : i32) -> i32);
    unsafe { OrdinalC(value) }
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn OrdinalFast(value: i32) -> i32 {
    windows_core::link!("ordinal_fixture.dll" "fastcall" ordinal(47) fn OrdinalFast(value : i32) -> i32);
    unsafe { OrdinalFast(value) }
}
#[inline]
pub unsafe fn OrdinalSystem(value: i32) -> i32 {
    windows_core::link!("ordinal_fixture.dll" "system" ordinal(17) fn OrdinalSystem(value : i32) -> i32);
    unsafe { OrdinalSystem(value) }
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn PerArch() -> i32 {
    windows_core::link!("ordinal_fixture.dll" "system" ordinal(31) fn PerArch() -> i32);
    unsafe { PerArch() }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn PerArch() -> i32 {
    windows_core::link!("ordinal_fixture.dll" "system" ordinal(32) fn PerArch() -> i32);
    unsafe { PerArch() }
}
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn PerArch() -> i32 {
    windows_core::link!("ordinal_fixture.dll" "system" ordinal(33) fn PerArch() -> i32);
    unsafe { PerArch() }
}
