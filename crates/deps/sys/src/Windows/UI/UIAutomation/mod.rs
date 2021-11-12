#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_UIAutomation_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AutomationConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationConnectionBoundObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AutomationTextRange(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationConnectionBoundObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutomationTextRange(pub *mut ::core::ffi::c_void);
pub struct UIAutomationContract(i32);
