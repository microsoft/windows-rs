#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ICW_ALREADYRUN();
    fn ICW_CHECKSTATUS();
    fn ICW_FULLPRESENT();
    fn ICW_FULL_SMARTSTART();
    fn ICW_LAUNCHEDFULL();
    fn ICW_LAUNCHEDMANUAL();
    fn ICW_LAUNCHFULL();
    fn ICW_LAUNCHMANUAL();
    fn ICW_MANUALPRESENT();
    fn ICW_MAX_ACCTNAME();
    fn ICW_MAX_EMAILADDR();
    fn ICW_MAX_EMAILNAME();
    fn ICW_MAX_LOGONNAME();
    fn ICW_MAX_PASSWORD();
    fn ICW_MAX_RASNAME();
    fn ICW_MAX_SERVERNAME();
    fn ICW_USEDEFAULTS();
    fn ICW_USE_SHELLNEXT();
    fn PFNCHECKCONNECTIONWIZARD();
    fn PFNSETSHELLNEXT();
}
