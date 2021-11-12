#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IAdaptiveCard(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdaptiveCardBuilderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecurityAppManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareWindowCommandEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareWindowCommandSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShareWindowCommandSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskbarManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskbarManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskbarManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SecurityAppKind(pub i32);
impl SecurityAppKind {
    pub const WebProtection: Self = Self(0i32);
}
#[repr(transparent)]
pub struct SecurityAppManager(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SecurityAppManagerContract(i32);
#[repr(transparent)]
pub struct SecurityAppState(pub i32);
impl SecurityAppState {
    pub const Disabled: Self = Self(0i32);
    pub const Enabled: Self = Self(1i32);
}
#[repr(transparent)]
pub struct SecurityAppSubstatus(pub i32);
impl SecurityAppSubstatus {
    pub const Undetermined: Self = Self(0i32);
    pub const NoActionNeeded: Self = Self(1i32);
    pub const ActionRecommended: Self = Self(2i32);
    pub const ActionNeeded: Self = Self(3i32);
}
#[repr(transparent)]
pub struct ShareWindowCommand(pub i32);
impl ShareWindowCommand {
    pub const None: Self = Self(0i32);
    pub const StartSharing: Self = Self(1i32);
    pub const StopSharing: Self = Self(2i32);
}
#[repr(transparent)]
pub struct ShareWindowCommandEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShareWindowCommandSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TaskbarManager(pub *mut ::core::ffi::c_void);
