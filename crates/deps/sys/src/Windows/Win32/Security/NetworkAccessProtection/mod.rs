#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn ComponentTypeEnforcementClientRp();
    fn ComponentTypeEnforcementClientSoH();
    fn CorrelationId();
    fn CountedString();
    fn ExtendedIsolationState();
    fn FailureCategory();
    fn FailureCategoryMapping();
    fn FixupInfo();
    fn FixupState();
    fn Ipv4Address();
    fn Ipv6Address();
    fn IsolationInfo();
    fn IsolationInfoEx();
    fn IsolationState();
    fn NapComponentRegistrationInfo();
    fn NapNotifyType();
    fn NapTracingLevel();
    fn NetworkSoH();
    fn PrivateData();
    fn RemoteConfigurationType();
    fn ResultCodes();
    fn SoH();
    fn SoHAttribute();
    fn SystemHealthAgentState();
}
