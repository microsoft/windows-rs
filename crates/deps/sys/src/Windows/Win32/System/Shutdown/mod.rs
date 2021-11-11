#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn AbortSystemShutdownA();
    fn AbortSystemShutdownW();
    fn CheckForHiberboot();
    fn ExitWindowsEx();
    fn InitiateShutdownA();
    fn InitiateShutdownW();
    fn InitiateSystemShutdownA();
    fn InitiateSystemShutdownExA();
    fn InitiateSystemShutdownExW();
    fn InitiateSystemShutdownW();
    fn LockWorkStation();
    fn ShutdownBlockReasonCreate();
    fn ShutdownBlockReasonDestroy();
    fn ShutdownBlockReasonQuery();
}
