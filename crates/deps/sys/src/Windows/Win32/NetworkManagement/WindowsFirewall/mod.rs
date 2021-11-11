#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn NetworkIsolationDiagnoseConnectFailureAndGetInfo();
    fn NetworkIsolationEnumAppContainers();
    fn NetworkIsolationFreeAppContainers();
    fn NetworkIsolationGetAppContainerConfig();
    fn NetworkIsolationRegisterForAppContainerChanges();
    fn NetworkIsolationSetAppContainerConfig();
    fn NetworkIsolationSetupAppContainerBinaries();
    fn NetworkIsolationUnregisterForAppContainerChanges();
}
