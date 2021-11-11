#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn GetDeviceID();
    fn GetDeviceIDString();
    fn Tbsi_Context_Create();
    fn Tbsi_Create_Windows_Key();
    fn Tbsi_GetDeviceInfo();
    fn Tbsi_Get_OwnerAuth();
    fn Tbsi_Get_TCG_Log();
    fn Tbsi_Get_TCG_Log_Ex();
    fn Tbsi_Physical_Presence_Command();
    fn Tbsi_Revoke_Attestation();
    fn Tbsip_Cancel_Commands();
    fn Tbsip_Context_Close();
    fn Tbsip_Submit_Command();
}
