#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectInput8Create();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_FlushQueue();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_FreePreparsedData();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetAttributes();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetConfiguration();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetFeature();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`*"]
    pub fn HidD_GetHidGuid();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetIndexedString();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetInputReport();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetManufacturerString();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetMsGenreDescriptor();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetNumInputBuffers();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetPhysicalDescriptor();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetPreparsedData();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetProductString();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetSerialNumberString();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_SetConfiguration();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_SetFeature();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_SetNumInputBuffers();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_SetOutputReport();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetButtonArray();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetButtonCaps();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetCaps();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetData();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetExtendedAttributes();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetLinkCollectionNodes();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetScaledUsageValue();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetSpecificButtonCaps();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetSpecificValueCaps();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetUsageValue();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetUsageValueArray();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetUsages();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetUsagesEx();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetValueCaps();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_InitializeReportForID();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`*"]
    pub fn HidP_MaxDataListLength();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`*"]
    pub fn HidP_MaxUsageListLength();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_SetButtonArray();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_SetData();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_SetScaledUsageValue();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_SetUsageValue();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_SetUsageValueArray();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_SetUsages();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_TranslateUsagesToI8042ScanCodes();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_UnsetUsages();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_UsageListDifference();
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`*"]
    pub fn joyConfigChanged();
}
