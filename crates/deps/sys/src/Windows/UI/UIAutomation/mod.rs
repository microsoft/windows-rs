#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "UI_UIAutomation_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {
    fn AutomationConnection();
    fn AutomationConnectionBoundObject();
    fn AutomationElement();
    fn AutomationTextRange();
    fn IAutomationConnection();
    fn IAutomationConnectionBoundObject();
    fn IAutomationElement();
    fn IAutomationTextRange();
    fn UIAutomationContract();
}
