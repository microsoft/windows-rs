#[inline]
pub unsafe fn NtQuerySystemInformation(systeminformationclass: SYSTEM_INFORMATION_CLASS, systeminformation: *mut core::ffi::c_void, systeminformationlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtQuerySystemInformation(systeminformationclass : SYSTEM_INFORMATION_CLASS, systeminformation : *mut core::ffi::c_void, systeminformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtQuerySystemInformation(systeminformationclass, core::mem::transmute(systeminformation), systeminformationlength, core::mem::transmute(returnlength)) }
}
#[inline]
pub unsafe fn NtQuerySystemTime(systemtime: *mut i64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtQuerySystemTime(systemtime : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtQuerySystemTime(core::mem::transmute(systemtime)) }
}
#[inline]
pub unsafe fn NtQueryTimerResolution(maximumtime: *mut u32, minimumtime: *mut u32, currenttime: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn NtQueryTimerResolution(maximumtime : *mut u32, minimumtime : *mut u32, currenttime : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { NtQueryTimerResolution(core::mem::transmute(maximumtime), core::mem::transmute(minimumtime), core::mem::transmute(currenttime)) }
}
#[inline]
pub unsafe fn ZwQuerySystemInformation(systeminformationclass: SYSTEM_INFORMATION_CLASS, systeminformation: *mut core::ffi::c_void, systeminformationlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwQuerySystemInformation(systeminformationclass : SYSTEM_INFORMATION_CLASS, systeminformation : *mut core::ffi::c_void, systeminformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwQuerySystemInformation(systeminformationclass, core::mem::transmute(systeminformation), systeminformationlength, core::mem::transmute(returnlength)) }
}
#[inline]
pub unsafe fn ZwQuerySystemTime(systemtime: *mut i64) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwQuerySystemTime(systemtime : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwQuerySystemTime(core::mem::transmute(systemtime)) }
}
#[inline]
pub unsafe fn ZwQueryTimerResolution(maximumtime: *mut u32, minimumtime: *mut u32, currenttime: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS {
    windows_targets::link!("ntdll.dll" "system" fn ZwQueryTimerResolution(maximumtime : *mut u32, minimumtime : *mut u32, currenttime : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
    unsafe { ZwQueryTimerResolution(core::mem::transmute(maximumtime), core::mem::transmute(minimumtime), core::mem::transmute(currenttime)) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEM_INFORMATION_CLASS(pub i32);
pub const SystemBasicInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(0i32);
pub const SystemCodeIntegrityInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(103i32);
pub const SystemExceptionInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(33i32);
pub const SystemInterruptInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(23i32);
pub const SystemLookasideInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(45i32);
pub const SystemPerformanceInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(2i32);
pub const SystemPolicyInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(134i32);
pub const SystemProcessInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(5i32);
pub const SystemProcessorPerformanceInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(8i32);
pub const SystemRegistryQuotaInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(37i32);
pub const SystemTimeOfDayInformation: SYSTEM_INFORMATION_CLASS = SYSTEM_INFORMATION_CLASS(3i32);
