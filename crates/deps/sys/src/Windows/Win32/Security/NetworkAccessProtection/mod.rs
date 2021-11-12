#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[doc = "*Required features: `Win32_Security_NetworkAccessProtection`*"]
pub const ComponentTypeEnforcementClientRp: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_NetworkAccessProtection`*"]
pub const ComponentTypeEnforcementClientSoH: u32 = 1u32;
pub struct CorrelationId(i32);
pub struct CountedString(i32);
pub struct ExtendedIsolationState(i32);
pub struct FailureCategory(i32);
pub struct FailureCategoryMapping(i32);
pub struct FixupInfo(i32);
pub struct FixupState(i32);
pub struct Ipv4Address(i32);
pub struct Ipv6Address(i32);
pub struct IsolationInfo(i32);
pub struct IsolationInfoEx(i32);
pub struct IsolationState(i32);
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
