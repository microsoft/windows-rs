#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IWsbApplicationAsync();
    fn IWsbApplicationBackupSupport();
    fn IWsbApplicationRestoreSupport();
    fn WSBAPP_ASYNC_IN_PROGRESS();
    fn WSB_MAX_OB_STATUS_ENTRY();
    fn WSB_MAX_OB_STATUS_VALUE_TYPE_PAIR();
    fn WSB_OB_REGISTRATION_INFO();
    fn WSB_OB_STATUS_ENTRY();
    fn WSB_OB_STATUS_ENTRY_PAIR_TYPE();
    fn WSB_OB_STATUS_ENTRY_VALUE_TYPE_PAIR();
    fn WSB_OB_STATUS_INFO();
}
