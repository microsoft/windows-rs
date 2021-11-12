#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_UIAutomation_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
pub struct AutomationConnection(i32);
pub struct AutomationConnectionBoundObject(i32);
pub struct AutomationElement(i32);
pub struct AutomationTextRange(i32);
pub struct IAutomationConnection(i32);
pub struct IAutomationConnectionBoundObject(i32);
pub struct IAutomationElement(i32);
pub struct IAutomationTextRange(i32);
pub struct UIAutomationContract(i32);
