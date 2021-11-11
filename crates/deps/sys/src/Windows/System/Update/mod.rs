#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ISystemUpdateItem();
    fn ISystemUpdateLastErrorInfo();
    fn ISystemUpdateManagerStatics();
    fn SystemUpdateAttentionRequiredReason();
    fn SystemUpdateItem();
    fn SystemUpdateItemState();
    fn SystemUpdateLastErrorInfo();
    fn SystemUpdateManager();
    fn SystemUpdateManagerState();
    fn SystemUpdateStartInstallAction();
}
