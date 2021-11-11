#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn NdfCancelIncident();
    fn NdfCloseIncident();
    fn NdfCreateConnectivityIncident();
    fn NdfCreateDNSIncident();
    fn NdfCreateGroupingIncident();
    fn NdfCreateIncident();
    fn NdfCreateNetConnectionIncident();
    fn NdfCreatePnrpIncident();
    fn NdfCreateSharingIncident();
    fn NdfCreateWebIncident();
    fn NdfCreateWebIncidentEx();
    fn NdfCreateWinSockIncident();
    fn NdfDiagnoseIncident();
    fn NdfExecuteDiagnosis();
    fn NdfGetTraceFile();
    fn NdfRepairIncident();
}
