#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn DXCORE_ADAPTER_ATTRIBUTE_D3D11_GRAPHICS();
    fn DXCORE_ADAPTER_ATTRIBUTE_D3D12_CORE_COMPUTE();
    fn DXCORE_ADAPTER_ATTRIBUTE_D3D12_GRAPHICS();
    fn DXCoreAdapterMemoryBudget();
    fn DXCoreAdapterMemoryBudgetNodeSegmentGroup();
    fn DXCoreAdapterPreference();
    fn DXCoreAdapterProperty();
    fn DXCoreAdapterState();
    fn DXCoreCreateAdapterFactory();
    fn DXCoreHardwareID();
    fn DXCoreHardwareIDParts();
    fn DXCoreNotificationType();
    fn DXCoreSegmentGroup();
    fn IDXCoreAdapter();
    fn IDXCoreAdapterFactory();
    fn IDXCoreAdapterList();
    fn PFN_DXCORE_NOTIFICATION_CALLBACK();
    fn _FACDXCORE();
}
