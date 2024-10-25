pub const SystemBasicInformation: SYSTEM_INFORMATION_CLASS = 0i32;
pub const SystemCodeIntegrityInformation: SYSTEM_INFORMATION_CLASS = 103i32;
pub const SystemExceptionInformation: SYSTEM_INFORMATION_CLASS = 33i32;
pub const SystemInterruptInformation: SYSTEM_INFORMATION_CLASS = 23i32;
pub const SystemLookasideInformation: SYSTEM_INFORMATION_CLASS = 45i32;
pub const SystemPerformanceInformation: SYSTEM_INFORMATION_CLASS = 2i32;
pub const SystemPolicyInformation: SYSTEM_INFORMATION_CLASS = 134i32;
pub const SystemProcessInformation: SYSTEM_INFORMATION_CLASS = 5i32;
pub const SystemProcessorPerformanceInformation: SYSTEM_INFORMATION_CLASS = 8i32;
pub const SystemRegistryQuotaInformation: SYSTEM_INFORMATION_CLASS = 37i32;
pub const SystemTimeOfDayInformation: SYSTEM_INFORMATION_CLASS = 3i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SYSTEM_INFORMATION_CLASS(pub i32);
impl windows_core::TypeKind for SYSTEM_INFORMATION_CLASS {
    type TypeKind = windows_core::CopyType;
}
