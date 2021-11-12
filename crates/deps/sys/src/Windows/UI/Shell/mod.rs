#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AdaptiveCardBuilder(i32);
pub struct IAdaptiveCard(pub *mut ::core::ffi::c_void);
pub struct IAdaptiveCardBuilderStatics(pub *mut ::core::ffi::c_void);
pub struct ISecurityAppManager(pub *mut ::core::ffi::c_void);
pub struct IShareWindowCommandEventArgs(pub *mut ::core::ffi::c_void);
pub struct IShareWindowCommandSource(pub *mut ::core::ffi::c_void);
pub struct IShareWindowCommandSourceStatics(pub *mut ::core::ffi::c_void);
pub struct ITaskbarManager(pub *mut ::core::ffi::c_void);
pub struct ITaskbarManager2(pub *mut ::core::ffi::c_void);
pub struct ITaskbarManagerStatics(pub *mut ::core::ffi::c_void);
pub struct SecurityAppKind(i32);
pub struct SecurityAppManager(i32);
pub struct SecurityAppManagerContract(i32);
pub struct SecurityAppState(i32);
pub struct SecurityAppSubstatus(i32);
pub struct ShareWindowCommand(i32);
pub struct ShareWindowCommandEventArgs(i32);
pub struct ShareWindowCommandSource(i32);
pub struct TaskbarManager(i32);
