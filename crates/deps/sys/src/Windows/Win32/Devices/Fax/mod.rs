#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CanSendToFaxRecipient() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxAbort(faxhandle: super::super::Foundation::HANDLE, jobid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxAccessCheck(faxhandle: super::super::Foundation::HANDLE, accessmask: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxClose(faxhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxCompleteJobParamsA(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxCompleteJobParamsW(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxConnectFaxServerA(machinename: super::super::Foundation::PSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxConnectFaxServerW(machinename: super::super::Foundation::PWSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnableRoutingMethodA(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnableRoutingMethodW(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumJobsA(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumJobsW(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumPortsA(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumPortsW(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumRoutingMethodsA(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumRoutingMethodsW(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`*"]
    pub fn FaxFreeBuffer(buffer: *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetDeviceStatusA(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetDeviceStatusW(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPageData(faxhandle: super::super::Foundation::HANDLE, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetRoutingInfoA(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetRoutingInfoW(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxInitializeEventQueue(faxhandle: super::super::Foundation::HANDLE, completionport: super::super::Foundation::HANDLE, completionkey: usize, hwnd: super::super::Foundation::HWND, messagestart: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxOpenPort(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxPrintCoverPageA(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxPrintCoverPageW(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxRegisterRoutingExtensionW(faxhandle: super::super::Foundation::HANDLE, extensionname: super::super::Foundation::PWSTR, friendlyname: super::super::Foundation::PWSTR, imagename: super::super::Foundation::PWSTR, callback: ::windows::runtime::RawPtr, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxRegisterServiceProviderW(deviceprovider: super::super::Foundation::PWSTR, friendlyname: super::super::Foundation::PWSTR, imagename: super::super::Foundation::PWSTR, tspname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentA(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PSTR, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentForBroadcastA(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PSTR, faxjobid: *mut u32, faxrecipientcallback: ::windows::runtime::RawPtr, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentForBroadcastW(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PWSTR, faxjobid: *mut u32, faxrecipientcallback: ::windows::runtime::RawPtr, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentW(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PWSTR, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetRoutingInfoA(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetRoutingInfoW(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxStartPrintJobA(printername: super::super::Foundation::PSTR, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxStartPrintJobW(printername: super::super::Foundation::PWSTR, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxUnregisterServiceProviderW(deviceprovider: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendToFaxRecipient(sndmode: SendToMode, lpfilename: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Devices_Fax`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StiCreateInstanceW(hinst: super::super::Foundation::HINSTANCE, dwver: u32, ppsti: *mut ::windows::runtime::RawPtr, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
}
