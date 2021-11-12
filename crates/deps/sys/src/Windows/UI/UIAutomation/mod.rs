#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_UIAutomation_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AutomationConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutomationConnection {}
impl ::core::clone::Clone for AutomationConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationConnectionBoundObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutomationConnectionBoundObject {}
impl ::core::clone::Clone for AutomationConnectionBoundObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutomationElement {}
impl ::core::clone::Clone for AutomationElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AutomationTextRange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AutomationTextRange {}
impl ::core::clone::Clone for AutomationTextRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationConnection {}
impl ::core::clone::Clone for IAutomationConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationConnectionBoundObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationConnectionBoundObject {}
impl ::core::clone::Clone for IAutomationConnectionBoundObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationElement {}
impl ::core::clone::Clone for IAutomationElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAutomationTextRange(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAutomationTextRange {}
impl ::core::clone::Clone for IAutomationTextRange {
    fn clone(&self) -> Self {
        *self
    }
}
