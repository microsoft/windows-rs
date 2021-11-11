#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CCH_RM_MAX_APP_NAME();
    fn CCH_RM_MAX_SVC_NAME();
    fn CCH_RM_SESSION_KEY();
    fn RM_APP_STATUS();
    fn RM_APP_TYPE();
    fn RM_FILTER_ACTION();
    fn RM_FILTER_INFO();
    fn RM_FILTER_TRIGGER();
    fn RM_INVALID_PROCESS();
    fn RM_INVALID_TS_SESSION();
    fn RM_PROCESS_INFO();
    fn RM_REBOOT_REASON();
    fn RM_SHUTDOWN_TYPE();
    fn RM_UNIQUE_PROCESS();
    fn RM_WRITE_STATUS_CALLBACK();
    fn RmAddFilter();
    fn RmCancelCurrentTask();
    fn RmEndSession();
    fn RmGetFilterList();
    fn RmGetList();
    fn RmJoinSession();
    fn RmRegisterResources();
    fn RmRemoveFilter();
    fn RmRestart();
    fn RmShutdown();
    fn RmStartSession();
}
