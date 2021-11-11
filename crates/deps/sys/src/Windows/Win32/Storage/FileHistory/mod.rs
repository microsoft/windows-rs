#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn FhServiceBlockBackup();
    fn FhServiceClosePipe();
    fn FhServiceOpenPipe();
    fn FhServiceReloadConfiguration();
    fn FhServiceStartBackup();
    fn FhServiceStopBackup();
    fn FhServiceUnblockBackup();
}
