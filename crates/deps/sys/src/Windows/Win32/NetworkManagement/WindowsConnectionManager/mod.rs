#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn FreeInterfaceContextTable();
    fn GetInterfaceContextTableForHostName();
    fn NET_INTERFACE_CONTEXT();
    fn NET_INTERFACE_CONTEXT_TABLE();
    fn NET_INTERFACE_FLAG_CONNECT_IF_NEEDED();
    fn NET_INTERFACE_FLAG_NONE();
    fn ONDEMAND_NOTIFICATION_CALLBACK();
    fn OnDemandGetRoutingHint();
    fn OnDemandRegisterNotification();
    fn OnDemandUnRegisterNotification();
    fn WCM_API_VERSION();
    fn WCM_API_VERSION_1_0();
    fn WCM_BILLING_CYCLE_INFO();
    fn WCM_CONNECTION_COST();
    fn WCM_CONNECTION_COST_DATA();
    fn WCM_CONNECTION_COST_SOURCE();
    fn WCM_DATAPLAN_STATUS();
    fn WCM_MAX_PROFILE_NAME();
    fn WCM_MEDIA_TYPE();
    fn WCM_POLICY_VALUE();
    fn WCM_PROFILE_INFO();
    fn WCM_PROFILE_INFO_LIST();
    fn WCM_PROPERTY();
    fn WCM_TIME_INTERVAL();
    fn WCM_UNKNOWN_DATAPLAN_STATUS();
    fn WCM_USAGE_DATA();
    fn WcmFreeMemory();
    fn WcmGetProfileList();
    fn WcmQueryProperty();
    fn WcmSetProfileList();
    fn WcmSetProperty();
}
