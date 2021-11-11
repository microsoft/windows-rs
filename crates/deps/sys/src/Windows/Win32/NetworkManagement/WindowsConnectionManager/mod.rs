#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn FreeInterfaceContextTable();
    fn GetInterfaceContextTableForHostName();
    fn OnDemandGetRoutingHint();
    fn OnDemandRegisterNotification();
    fn OnDemandUnRegisterNotification();
    fn WcmFreeMemory();
    fn WcmGetProfileList();
    fn WcmQueryProperty();
    fn WcmSetProfileList();
    fn WcmSetProperty();
}
