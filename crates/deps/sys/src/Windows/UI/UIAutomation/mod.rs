#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_UIAutomation_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
pub struct AutomationConnection(i32);
pub struct AutomationConnectionBoundObject(i32);
pub struct AutomationElement(i32);
pub struct AutomationTextRange(i32);
pub struct IAutomationConnection(pub *mut ::core::ffi::c_void);
pub struct IAutomationConnectionBoundObject(pub *mut ::core::ffi::c_void);
pub struct IAutomationElement(pub *mut ::core::ffi::c_void);
pub struct IAutomationTextRange(pub *mut ::core::ffi::c_void);
pub struct UIAutomationContract(i32);
