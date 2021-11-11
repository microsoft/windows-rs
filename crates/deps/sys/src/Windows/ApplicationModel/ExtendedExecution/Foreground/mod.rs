#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ExtendedExecutionForegroundReason();
    fn ExtendedExecutionForegroundResult();
    fn ExtendedExecutionForegroundRevokedEventArgs();
    fn ExtendedExecutionForegroundRevokedReason();
    fn ExtendedExecutionForegroundSession();
    fn IExtendedExecutionForegroundRevokedEventArgs();
    fn IExtendedExecutionForegroundSession();
}
