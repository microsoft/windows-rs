#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub const ComponentTypeEnforcementClientRp: u32 = 2u32;
pub const ComponentTypeEnforcementClientSoH: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct CorrelationId(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CountedString(i32);
pub struct ExtendedIsolationState(i32);
pub struct FailureCategory(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct FailureCategoryMapping(i32);
pub struct FixupInfo(i32);
pub struct FixupState(i32);
pub struct Ipv4Address(i32);
pub struct Ipv6Address(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct IsolationInfo(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct IsolationInfoEx(i32);
pub struct IsolationState(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct NapComponentRegistrationInfo(i32);
pub struct NapNotifyType(i32);
pub struct NapTracingLevel(i32);
pub struct NetworkSoH(i32);
pub struct PrivateData(i32);
pub struct RemoteConfigurationType(i32);
pub struct ResultCodes(i32);
pub struct SoH(i32);
pub struct SoHAttribute(i32);
pub struct SystemHealthAgentState(i32);
