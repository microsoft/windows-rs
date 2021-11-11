#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ApplicationRecoveryFinished();
    fn ApplicationRecoveryInProgress();
    fn GetApplicationRecoveryCallback();
    fn GetApplicationRestartSettings();
    fn RegisterApplicationRecoveryCallback();
    fn RegisterApplicationRestart();
    fn UnregisterApplicationRecoveryCallback();
    fn UnregisterApplicationRestart();
}
