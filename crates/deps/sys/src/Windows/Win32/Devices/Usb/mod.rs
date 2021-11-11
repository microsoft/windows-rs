#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn WinUsb_AbortPipe();
    fn WinUsb_ControlTransfer();
    fn WinUsb_FlushPipe();
    fn WinUsb_Free();
    fn WinUsb_GetAdjustedFrameNumber();
    fn WinUsb_GetAssociatedInterface();
    fn WinUsb_GetCurrentAlternateSetting();
    fn WinUsb_GetCurrentFrameNumber();
    fn WinUsb_GetCurrentFrameNumberAndQpc();
    fn WinUsb_GetDescriptor();
    fn WinUsb_GetOverlappedResult();
    fn WinUsb_GetPipePolicy();
    fn WinUsb_GetPowerPolicy();
    fn WinUsb_Initialize();
    fn WinUsb_ParseConfigurationDescriptor();
    fn WinUsb_ParseDescriptors();
    fn WinUsb_QueryDeviceInformation();
    fn WinUsb_QueryInterfaceSettings();
    fn WinUsb_QueryPipe();
    fn WinUsb_QueryPipeEx();
    fn WinUsb_ReadIsochPipe();
    fn WinUsb_ReadIsochPipeAsap();
    fn WinUsb_ReadPipe();
    fn WinUsb_RegisterIsochBuffer();
    fn WinUsb_ResetPipe();
    fn WinUsb_SetCurrentAlternateSetting();
    fn WinUsb_SetPipePolicy();
    fn WinUsb_SetPowerPolicy();
    fn WinUsb_StartTrackingForTimeSync();
    fn WinUsb_StopTrackingForTimeSync();
    fn WinUsb_UnregisterIsochBuffer();
    fn WinUsb_WriteIsochPipe();
    fn WinUsb_WriteIsochPipeAsap();
    fn WinUsb_WritePipe();
}
