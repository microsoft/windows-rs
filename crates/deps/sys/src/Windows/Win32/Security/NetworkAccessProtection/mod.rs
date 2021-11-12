#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub const ComponentTypeEnforcementClientRp: u32 = 2u32;
pub const ComponentTypeEnforcementClientSoH: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CorrelationId(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CountedString(i32);
#[repr(C)]
pub struct ExtendedIsolationState(i32);
#[repr(C)]
pub struct FailureCategory(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FailureCategoryMapping(i32);
#[repr(C)]
pub struct FixupInfo(i32);
#[repr(C)]
pub struct FixupState(i32);
#[repr(C)]
pub struct Ipv4Address(i32);
#[repr(C)]
pub struct Ipv6Address(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IsolationInfo(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct IsolationInfoEx(i32);
#[repr(C)]
pub struct IsolationState(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NapComponentRegistrationInfo(i32);
#[repr(C)]
pub struct NapNotifyType(i32);
#[repr(C)]
pub struct NapTracingLevel(i32);
#[repr(C)]
pub struct NetworkSoH(i32);
#[repr(C)]
pub struct PrivateData(i32);
#[repr(C)]
pub struct RemoteConfigurationType(i32);
#[repr(C)]
pub struct ResultCodes(i32);
#[repr(C)]
pub struct SoH(i32);
#[repr(C)]
pub struct SoHAttribute(i32);
#[repr(C)]
pub struct SystemHealthAgentState(i32);
