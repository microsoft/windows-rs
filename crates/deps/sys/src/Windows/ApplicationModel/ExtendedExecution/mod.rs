#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "ApplicationModel_ExtendedExecution_Foreground")]
pub mod Foreground;
#[link(name = "windows")]
extern "system" {
    fn ExtendedExecutionReason();
    fn ExtendedExecutionResult();
    fn ExtendedExecutionRevokedEventArgs();
    fn ExtendedExecutionRevokedReason();
    fn ExtendedExecutionSession();
    fn IExtendedExecutionRevokedEventArgs();
    fn IExtendedExecutionSession();
}
