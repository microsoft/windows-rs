#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CanSendToFaxRecipient();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxAbort();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxAccessCheck();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxClose();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxCompleteJobParamsA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxCompleteJobParamsW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxConnectFaxServerA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxConnectFaxServerW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnableRoutingMethodA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnableRoutingMethodW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumGlobalRoutingInfoA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumGlobalRoutingInfoW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumJobsA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumJobsW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumPortsA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumPortsW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumRoutingMethodsA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumRoutingMethodsW();
    #[doc = "*Required features: `Win32_Devices_Fax`*"]
    pub fn FaxFreeBuffer();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetConfigurationA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetConfigurationW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetDeviceStatusA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetDeviceStatusW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetJobA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetJobW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetLoggingCategoriesA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetLoggingCategoriesW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPageData();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPortA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPortW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetRoutingInfoA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetRoutingInfoW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxInitializeEventQueue();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxOpenPort();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxPrintCoverPageA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxPrintCoverPageW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxRegisterRoutingExtensionW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxRegisterServiceProviderW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentForBroadcastA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentForBroadcastW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetConfigurationA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetConfigurationW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetGlobalRoutingInfoA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetGlobalRoutingInfoW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetJobA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetJobW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetLoggingCategoriesA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetLoggingCategoriesW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetPortA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetPortW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetRoutingInfoA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetRoutingInfoW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxStartPrintJobA();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxStartPrintJobW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxUnregisterServiceProviderW();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendToFaxRecipient();
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StiCreateInstanceW();
}
