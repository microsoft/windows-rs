#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
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
