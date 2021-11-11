#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn CoCreateActivity();
    fn CoEnterServiceDomain();
    fn CoGetDefaultContext();
    fn CoLeaveServiceDomain();
    fn GetDispenserManager();
    fn GetManagedExtensions();
    fn MTSCreateActivity();
    fn RecycleSurrogate();
    fn SafeRef();
}
