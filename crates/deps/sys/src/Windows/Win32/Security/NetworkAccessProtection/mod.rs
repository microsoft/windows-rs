#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const ComponentTypeEnforcementClientRp: u32 = 2u32;
pub const ComponentTypeEnforcementClientSoH: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CorrelationId {
    pub connId: ::windows_sys::core::GUID,
    pub timeStamp: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CorrelationId {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CorrelationId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CountedString {
    pub length: u16,
    pub string: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CountedString {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CountedString {
    fn clone(&self) -> Self {
        *self
    }
}
pub const extendedIsolationStateNoData: i32 = 0i32;
pub const extendedIsolationStateTransition: i32 = 1i32;
pub const extendedIsolationStateInfected: i32 = 2i32;
pub const extendedIsolationStateUnknown: i32 = 3i32;
pub const failureCategoryNone: i32 = 0i32;
pub const failureCategoryOther: i32 = 1i32;
pub const failureCategoryClientComponent: i32 = 2i32;
pub const failureCategoryClientCommunication: i32 = 3i32;
pub const failureCategoryServerComponent: i32 = 4i32;
pub const failureCategoryServerCommunication: i32 = 5i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct FailureCategoryMapping {
    pub mappingCompliance: [super::super::Foundation::BOOL; 5],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FailureCategoryMapping {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FailureCategoryMapping {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct FixupInfo {
    pub state: FixupState,
    pub percentage: u8,
    pub resultCodes: ResultCodes,
    pub fixupMsgId: u32,
}
impl ::core::marker::Copy for FixupInfo {}
impl ::core::clone::Clone for FixupInfo {
    fn clone(&self) -> Self {
        *self
    }
}
pub const fixupStateSuccess: i32 = 0i32;
pub const fixupStateInProgress: i32 = 1i32;
pub const fixupStateCouldNotUpdate: i32 = 2i32;
#[repr(C)]
pub struct Ipv4Address {
    pub addr: [u8; 4],
}
impl ::core::marker::Copy for Ipv4Address {}
impl ::core::clone::Clone for Ipv4Address {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct Ipv6Address {
    pub addr: [u8; 16],
}
impl ::core::marker::Copy for Ipv6Address {}
impl ::core::clone::Clone for Ipv6Address {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolationInfo {
    pub isolationState: IsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IsolationInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IsolationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolationInfoEx {
    pub isolationState: IsolationState,
    pub extendedIsolationState: ExtendedIsolationState,
    pub probEndTime: super::super::Foundation::FILETIME,
    pub failureUrl: CountedString,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IsolationInfoEx {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IsolationInfoEx {
    fn clone(&self) -> Self {
        *self
    }
}
pub const isolationStateNotRestricted: i32 = 1i32;
pub const isolationStateInProbation: i32 = 2i32;
pub const isolationStateRestrictedAccess: i32 = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NapComponentRegistrationInfo {
    pub id: u32,
    pub friendlyName: CountedString,
    pub description: CountedString,
    pub version: CountedString,
    pub vendorName: CountedString,
    pub infoClsid: ::windows_sys::core::GUID,
    pub configClsid: ::windows_sys::core::GUID,
    pub registrationDate: super::super::Foundation::FILETIME,
    pub componentType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NapComponentRegistrationInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NapComponentRegistrationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
pub const napNotifyTypeUnknown: i32 = 0i32;
pub const napNotifyTypeServiceState: i32 = 1i32;
pub const napNotifyTypeQuarState: i32 = 2i32;
pub const tracingLevelUndefined: i32 = 0i32;
pub const tracingLevelBasic: i32 = 1i32;
pub const tracingLevelAdvanced: i32 = 2i32;
pub const tracingLevelDebug: i32 = 3i32;
#[repr(C)]
pub struct NetworkSoH {
    pub size: u16,
    pub data: *mut u8,
}
impl ::core::marker::Copy for NetworkSoH {}
impl ::core::clone::Clone for NetworkSoH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PrivateData {
    pub size: u16,
    pub data: *mut u8,
}
impl ::core::marker::Copy for PrivateData {}
impl ::core::clone::Clone for PrivateData {
    fn clone(&self) -> Self {
        *self
    }
}
pub const remoteConfigTypeMachine: i32 = 1i32;
pub const remoteConfigTypeConfigBlob: i32 = 2i32;
#[repr(C)]
pub struct ResultCodes {
    pub count: u16,
    pub results: *mut ::windows_sys::core::HRESULT,
}
impl ::core::marker::Copy for ResultCodes {}
impl ::core::clone::Clone for ResultCodes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SoH {
    pub count: u16,
    pub attributes: *mut SoHAttribute,
}
impl ::core::marker::Copy for SoH {}
impl ::core::clone::Clone for SoH {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SoHAttribute {
    pub r#type: u16,
    pub size: u16,
    pub value: *mut u8,
}
impl ::core::marker::Copy for SoHAttribute {}
impl ::core::clone::Clone for SoHAttribute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SystemHealthAgentState {
    pub id: u32,
    pub shaResultCodes: ResultCodes,
    pub failureCategory: FailureCategory,
    pub fixupInfo: FixupInfo,
}
impl ::core::marker::Copy for SystemHealthAgentState {}
impl ::core::clone::Clone for SystemHealthAgentState {
    fn clone(&self) -> Self {
        *self
    }
}
