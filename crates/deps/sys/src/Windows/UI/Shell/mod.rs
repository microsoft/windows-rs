#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdaptiveCardBuilder(pub *mut ::core::ffi::c_void);
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
pub struct SecurityAppKind(i32);
#[repr(transparent)]
pub struct SecurityAppManager(pub *mut ::core::ffi::c_void);
pub struct SecurityAppManagerContract(i32);
pub struct SecurityAppState(i32);
pub struct SecurityAppSubstatus(i32);
pub struct ShareWindowCommand(i32);
#[repr(transparent)]
pub struct ShareWindowCommandEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ShareWindowCommandSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TaskbarManager(pub *mut ::core::ffi::c_void);
