#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AbortSystemShutdownA();
    fn AbortSystemShutdownW();
    fn CheckForHiberboot();
    fn EXIT_WINDOWS_FLAGS();
    fn ExitWindowsEx();
    fn InitiateShutdownA();
    fn InitiateShutdownW();
    fn InitiateSystemShutdownA();
    fn InitiateSystemShutdownExA();
    fn InitiateSystemShutdownExW();
    fn InitiateSystemShutdownW();
    fn LockWorkStation();
    fn MAX_NUM_REASONS();
    fn MAX_REASON_BUGID_LEN();
    fn MAX_REASON_COMMENT_LEN();
    fn MAX_REASON_DESC_LEN();
    fn MAX_REASON_NAME_LEN();
    fn POLICY_SHOWREASONUI_ALWAYS();
    fn POLICY_SHOWREASONUI_NEVER();
    fn POLICY_SHOWREASONUI_SERVERONLY();
    fn POLICY_SHOWREASONUI_WORKSTATIONONLY();
    fn SHUTDOWN_FLAGS();
    fn SHUTDOWN_REASON();
    fn SHUTDOWN_TYPE_LEN();
    fn SNAPSHOT_POLICY_ALWAYS();
    fn SNAPSHOT_POLICY_NEVER();
    fn SNAPSHOT_POLICY_UNPLANNED();
    fn ShutdownBlockReasonCreate();
    fn ShutdownBlockReasonDestroy();
    fn ShutdownBlockReasonQuery();
}
