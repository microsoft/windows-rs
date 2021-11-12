#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CanSendToFaxRecipient() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxAbort(faxhandle: super::super::Foundation::HANDLE, jobid: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxAccessCheck(faxhandle: super::super::Foundation::HANDLE, accessmask: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxClose(faxhandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxCompleteJobParamsA(jobparams: *mut *mut FAX_JOB_PARAMA, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxCompleteJobParamsW(jobparams: *mut *mut FAX_JOB_PARAMW, coverpageinfo: *mut *mut FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxConnectFaxServerA(machinename: super::super::Foundation::PSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxConnectFaxServerW(machinename: super::super::Foundation::PWSTR, faxhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnableRoutingMethodA(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnableRoutingMethodW(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, enabled: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *mut *mut FAX_GLOBAL_ROUTING_INFOW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumJobsA(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYA, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumJobsW(faxhandle: super::super::Foundation::HANDLE, jobentry: *mut *mut FAX_JOB_ENTRYW, jobsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumPortsA(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumPortsW(faxhandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW, portsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumRoutingMethodsA(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODA, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxEnumRoutingMethodsW(faxporthandle: super::super::Foundation::HANDLE, routingmethod: *mut *mut FAX_ROUTING_METHODW, methodsreturned: *mut u32) -> super::super::Foundation::BOOL;
    pub fn FaxFreeBuffer(buffer: *mut ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *mut *mut FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetDeviceStatusA(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetDeviceStatusW(faxporthandle: super::super::Foundation::HANDLE, devicestatus: *mut *mut FAX_DEVICE_STATUSW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, jobentry: *mut *mut FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYA, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *mut *mut FAX_LOG_CATEGORYW, numbercategories: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPageData(faxhandle: super::super::Foundation::HANDLE, jobid: u32, buffer: *mut *mut u8, buffersize: *mut u32, imagewidth: *mut u32, imageheight: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *mut *mut FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetRoutingInfoA(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxGetRoutingInfoW(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, routinginfobuffer: *mut *mut u8, routinginfobuffersize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxInitializeEventQueue(faxhandle: super::super::Foundation::HANDLE, completionport: super::super::Foundation::HANDLE, completionkey: usize, hwnd: super::super::Foundation::HWND, messagestart: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxOpenPort(faxhandle: super::super::Foundation::HANDLE, deviceid: u32, flags: u32, faxporthandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxPrintCoverPageA(faxcontextinfo: *const FAX_CONTEXT_INFOA, coverpageinfo: *const FAX_COVERPAGE_INFOA) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxPrintCoverPageW(faxcontextinfo: *const FAX_CONTEXT_INFOW, coverpageinfo: *const FAX_COVERPAGE_INFOW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxRegisterRoutingExtensionW(faxhandle: super::super::Foundation::HANDLE, extensionname: super::super::Foundation::PWSTR, friendlyname: super::super::Foundation::PWSTR, imagename: super::super::Foundation::PWSTR, callback: PFAX_ROUTING_INSTALLATION_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxRegisterServiceProviderW(deviceprovider: super::super::Foundation::PWSTR, friendlyname: super::super::Foundation::PWSTR, imagename: super::super::Foundation::PWSTR, tspname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentA(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PSTR, jobparams: *mut FAX_JOB_PARAMA, coverpageinfo: *const FAX_COVERPAGE_INFOA, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentForBroadcastA(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKA, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentForBroadcastW(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PWSTR, faxjobid: *mut u32, faxrecipientcallback: PFAX_RECIPIENT_CALLBACKW, context: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSendDocumentW(faxhandle: super::super::Foundation::HANDLE, filename: super::super::Foundation::PWSTR, jobparams: *mut FAX_JOB_PARAMW, coverpageinfo: *const FAX_COVERPAGE_INFOW, faxjobid: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetConfigurationA(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetConfigurationW(faxhandle: super::super::Foundation::HANDLE, faxconfig: *const FAX_CONFIGURATIONW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetGlobalRoutingInfoA(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetGlobalRoutingInfoW(faxhandle: super::super::Foundation::HANDLE, routinginfo: *const FAX_GLOBAL_ROUTING_INFOW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetJobA(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetJobW(faxhandle: super::super::Foundation::HANDLE, jobid: u32, command: u32, jobentry: *const FAX_JOB_ENTRYW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetLoggingCategoriesA(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYA, numbercategories: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetLoggingCategoriesW(faxhandle: super::super::Foundation::HANDLE, categories: *const FAX_LOG_CATEGORYW, numbercategories: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetPortA(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetPortW(faxporthandle: super::super::Foundation::HANDLE, portinfo: *const FAX_PORT_INFOW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetRoutingInfoA(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxSetRoutingInfoW(faxporthandle: super::super::Foundation::HANDLE, routingguid: super::super::Foundation::PWSTR, routinginfobuffer: *const u8, routinginfobuffersize: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxStartPrintJobA(printername: super::super::Foundation::PSTR, printinfo: *const FAX_PRINT_INFOA, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOA) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn FaxStartPrintJobW(printername: super::super::Foundation::PWSTR, printinfo: *const FAX_PRINT_INFOW, faxjobid: *mut u32, faxcontextinfo: *mut FAX_CONTEXT_INFOW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FaxUnregisterServiceProviderW(deviceprovider: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendToFaxRecipient(sndmode: SendToMode, lpfilename: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StiCreateInstanceW(hinst: super::super::Foundation::HINSTANCE, dwver: u32, ppsti: *mut IStillImageW, punkouter: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
}
pub const CLSID_Sti: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3005479136, data2: 11880, data3: 4560, data4: [144, 234, 0, 170, 0, 96, 248, 108] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WIA_DeviceType: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1809653702, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WIA_USDClassId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 1809653702, data2: 33039, data3: 4560, data4: [190, 199, 8, 0, 43, 226, 9, 47] },
    pid: 3u32,
};
pub const FAXDEVRECEIVE_SIZE: u32 = 4096u32;
pub const FAXDEVREPORTSTATUS_SIZE: u32 = 4096u32;
#[repr(transparent)]
pub struct FAXROUTE_ENABLE(pub i32);
pub const QUERY_STATUS: FAXROUTE_ENABLE = FAXROUTE_ENABLE(-1i32);
pub const STATUS_DISABLE: FAXROUTE_ENABLE = FAXROUTE_ENABLE(0i32);
pub const STATUS_ENABLE: FAXROUTE_ENABLE = FAXROUTE_ENABLE(1i32);
#[repr(transparent)]
pub struct FAX_ACCESS_RIGHTS_ENUM(pub i32);
pub const farSUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(1i32);
pub const farSUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(2i32);
pub const farSUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(4i32);
pub const farQUERY_JOBS: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(8i32);
pub const farMANAGE_JOBS: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(16i32);
pub const farQUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(32i32);
pub const farMANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(64i32);
pub const farQUERY_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(128i32);
pub const farMANAGE_IN_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(256i32);
pub const farQUERY_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(512i32);
pub const farMANAGE_OUT_ARCHIVE: FAX_ACCESS_RIGHTS_ENUM = FAX_ACCESS_RIGHTS_ENUM(1024i32);
#[repr(transparent)]
pub struct FAX_ACCESS_RIGHTS_ENUM_2(pub i32);
pub const far2SUBMIT_LOW: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(1i32);
pub const far2SUBMIT_NORMAL: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(2i32);
pub const far2SUBMIT_HIGH: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(4i32);
pub const far2QUERY_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(8i32);
pub const far2MANAGE_OUT_JOBS: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(16i32);
pub const far2QUERY_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(32i32);
pub const far2MANAGE_CONFIG: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(64i32);
pub const far2QUERY_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(128i32);
pub const far2MANAGE_ARCHIVES: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(256i32);
pub const far2MANAGE_RECEIVE_FOLDER: FAX_ACCESS_RIGHTS_ENUM_2 = FAX_ACCESS_RIGHTS_ENUM_2(512i32);
#[repr(transparent)]
pub struct FAX_ACCOUNT_EVENTS_TYPE_ENUM(pub i32);
pub const faetNONE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(0i32);
pub const faetIN_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(1i32);
pub const faetOUT_QUEUE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(2i32);
pub const faetIN_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(4i32);
pub const faetOUT_ARCHIVE: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(8i32);
pub const faetFXSSVC_ENDED: FAX_ACCOUNT_EVENTS_TYPE_ENUM = FAX_ACCOUNT_EVENTS_TYPE_ENUM(16i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_CONFIGURATIONA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_CONFIGURATIONW(i32);
pub const FAX_CONFIG_QUERY: u32 = 4u32;
pub const FAX_CONFIG_SET: u32 = 8u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct FAX_CONTEXT_INFOA(i32);
#[cfg(feature = "Win32_Graphics_Gdi")]
#[repr(C)]
pub struct FAX_CONTEXT_INFOW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_COVERPAGE_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_COVERPAGE_INFOW(i32);
#[repr(transparent)]
pub struct FAX_COVERPAGE_TYPE_ENUM(pub i32);
pub const fcptNONE: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(0i32);
pub const fcptLOCAL: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(1i32);
pub const fcptSERVER: FAX_COVERPAGE_TYPE_ENUM = FAX_COVERPAGE_TYPE_ENUM(2i32);
#[repr(transparent)]
pub struct FAX_DEVICE_RECEIVE_MODE_ENUM(pub i32);
pub const fdrmNO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(0i32);
pub const fdrmAUTO_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(1i32);
pub const fdrmMANUAL_ANSWER: FAX_DEVICE_RECEIVE_MODE_ENUM = FAX_DEVICE_RECEIVE_MODE_ENUM(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_DEVICE_STATUSA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_DEVICE_STATUSW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_DEV_STATUS(i32);
#[repr(transparent)]
pub struct FAX_ENUM_DELIVERY_REPORT_TYPES(pub i32);
pub const DRT_NONE: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(0i32);
pub const DRT_EMAIL: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(1i32);
pub const DRT_INBOX: FAX_ENUM_DELIVERY_REPORT_TYPES = FAX_ENUM_DELIVERY_REPORT_TYPES(2i32);
#[repr(transparent)]
pub struct FAX_ENUM_DEVICE_ID_SOURCE(pub i32);
pub const DEV_ID_SRC_FAX: FAX_ENUM_DEVICE_ID_SOURCE = FAX_ENUM_DEVICE_ID_SOURCE(0i32);
pub const DEV_ID_SRC_TAPI: FAX_ENUM_DEVICE_ID_SOURCE = FAX_ENUM_DEVICE_ID_SOURCE(1i32);
#[repr(transparent)]
pub struct FAX_ENUM_JOB_COMMANDS(pub i32);
pub const JC_UNKNOWN: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(0i32);
pub const JC_DELETE: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(1i32);
pub const JC_PAUSE: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(2i32);
pub const JC_RESUME: FAX_ENUM_JOB_COMMANDS = FAX_ENUM_JOB_COMMANDS(3i32);
#[repr(transparent)]
pub struct FAX_ENUM_JOB_SEND_ATTRIBUTES(pub i32);
pub const JSA_NOW: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(0i32);
pub const JSA_SPECIFIC_TIME: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(1i32);
pub const JSA_DISCOUNT_PERIOD: FAX_ENUM_JOB_SEND_ATTRIBUTES = FAX_ENUM_JOB_SEND_ATTRIBUTES(2i32);
#[repr(transparent)]
pub struct FAX_ENUM_LOG_CATEGORIES(pub i32);
pub const FAXLOG_CATEGORY_INIT: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(1i32);
pub const FAXLOG_CATEGORY_OUTBOUND: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(2i32);
pub const FAXLOG_CATEGORY_INBOUND: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(3i32);
pub const FAXLOG_CATEGORY_UNKNOWN: FAX_ENUM_LOG_CATEGORIES = FAX_ENUM_LOG_CATEGORIES(4i32);
#[repr(transparent)]
pub struct FAX_ENUM_LOG_LEVELS(pub i32);
pub const FAXLOG_LEVEL_NONE: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(0i32);
pub const FAXLOG_LEVEL_MIN: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(1i32);
pub const FAXLOG_LEVEL_MED: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(2i32);
pub const FAXLOG_LEVEL_MAX: FAX_ENUM_LOG_LEVELS = FAX_ENUM_LOG_LEVELS(3i32);
#[repr(transparent)]
pub struct FAX_ENUM_PORT_OPEN_TYPE(pub i32);
pub const PORT_OPEN_QUERY: FAX_ENUM_PORT_OPEN_TYPE = FAX_ENUM_PORT_OPEN_TYPE(1i32);
pub const PORT_OPEN_MODIFY: FAX_ENUM_PORT_OPEN_TYPE = FAX_ENUM_PORT_OPEN_TYPE(2i32);
pub const FAX_ERR_BAD_GROUP_CONFIGURATION: i32 = 7003i32;
pub const FAX_ERR_DEVICE_NUM_LIMIT_EXCEEDED: i32 = 7010i32;
pub const FAX_ERR_DIRECTORY_IN_USE: i32 = 7007i32;
pub const FAX_ERR_END: i32 = 7013i32;
pub const FAX_ERR_FILE_ACCESS_DENIED: i32 = 7008i32;
pub const FAX_ERR_GROUP_IN_USE: i32 = 7004i32;
pub const FAX_ERR_GROUP_NOT_FOUND: i32 = 7002i32;
pub const FAX_ERR_MESSAGE_NOT_FOUND: i32 = 7009i32;
pub const FAX_ERR_NOT_NTFS: i32 = 7006i32;
pub const FAX_ERR_NOT_SUPPORTED_ON_THIS_SKU: i32 = 7011i32;
pub const FAX_ERR_RECIPIENTS_LIMIT: i32 = 7013i32;
pub const FAX_ERR_RULE_NOT_FOUND: i32 = 7005i32;
pub const FAX_ERR_SRV_OUTOFMEMORY: i32 = 7001i32;
pub const FAX_ERR_START: i32 = 7001i32;
pub const FAX_ERR_VERSION_MISMATCH: i32 = 7012i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_EVENTA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_EVENTW(i32);
pub const FAX_E_BAD_GROUP_CONFIGURATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214501i32 as _);
pub const FAX_E_DEVICE_NUM_LIMIT_EXCEEDED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214494i32 as _);
pub const FAX_E_DIRECTORY_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214497i32 as _);
pub const FAX_E_FILE_ACCESS_DENIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214496i32 as _);
pub const FAX_E_GROUP_IN_USE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214500i32 as _);
pub const FAX_E_GROUP_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214502i32 as _);
pub const FAX_E_MESSAGE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214495i32 as _);
pub const FAX_E_NOT_NTFS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214498i32 as _);
pub const FAX_E_NOT_SUPPORTED_ON_THIS_SKU: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214493i32 as _);
pub const FAX_E_RECIPIENTS_LIMIT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214491i32 as _);
pub const FAX_E_RULE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214499i32 as _);
pub const FAX_E_SRV_OUTOFMEMORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214503i32 as _);
pub const FAX_E_VERSION_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147214492i32 as _);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_GLOBAL_ROUTING_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_GLOBAL_ROUTING_INFOW(i32);
#[repr(transparent)]
pub struct FAX_GROUP_STATUS_ENUM(pub i32);
pub const fgsALL_DEV_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(0i32);
pub const fgsEMPTY: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(1i32);
pub const fgsALL_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(2i32);
pub const fgsSOME_DEV_NOT_VALID: FAX_GROUP_STATUS_ENUM = FAX_GROUP_STATUS_ENUM(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_JOB_ENTRYA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_JOB_ENTRYW(i32);
#[repr(transparent)]
pub struct FAX_JOB_EXTENDED_STATUS_ENUM(pub i32);
pub const fjesNONE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(0i32);
pub const fjesDISCONNECTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(1i32);
pub const fjesINITIALIZING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(2i32);
pub const fjesDIALING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(3i32);
pub const fjesTRANSMITTING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(4i32);
pub const fjesANSWERED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(5i32);
pub const fjesRECEIVING: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(6i32);
pub const fjesLINE_UNAVAILABLE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(7i32);
pub const fjesBUSY: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(8i32);
pub const fjesNO_ANSWER: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(9i32);
pub const fjesBAD_ADDRESS: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(10i32);
pub const fjesNO_DIAL_TONE: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(11i32);
pub const fjesFATAL_ERROR: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(12i32);
pub const fjesCALL_DELAYED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(13i32);
pub const fjesCALL_BLACKLISTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(14i32);
pub const fjesNOT_FAX_CALL: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(15i32);
pub const fjesPARTIALLY_RECEIVED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(16i32);
pub const fjesHANDLED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(17i32);
pub const fjesCALL_COMPLETED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(18i32);
pub const fjesCALL_ABORTED: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(19i32);
pub const fjesPROPRIETARY: FAX_JOB_EXTENDED_STATUS_ENUM = FAX_JOB_EXTENDED_STATUS_ENUM(16777216i32);
pub const FAX_JOB_MANAGE: u32 = 64u32;
#[repr(transparent)]
pub struct FAX_JOB_OPERATIONS_ENUM(pub i32);
pub const fjoVIEW: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(1i32);
pub const fjoPAUSE: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(2i32);
pub const fjoRESUME: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(4i32);
pub const fjoRESTART: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(8i32);
pub const fjoDELETE: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(16i32);
pub const fjoRECIPIENT_INFO: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(32i32);
pub const fjoSENDER_INFO: FAX_JOB_OPERATIONS_ENUM = FAX_JOB_OPERATIONS_ENUM(64i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_JOB_PARAMA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_JOB_PARAMW(i32);
pub const FAX_JOB_QUERY: u32 = 2u32;
#[repr(transparent)]
pub struct FAX_JOB_STATUS_ENUM(pub i32);
pub const fjsPENDING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(1i32);
pub const fjsINPROGRESS: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(2i32);
pub const fjsFAILED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(8i32);
pub const fjsPAUSED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(16i32);
pub const fjsNOLINE: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(32i32);
pub const fjsRETRYING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(64i32);
pub const fjsRETRIES_EXCEEDED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(128i32);
pub const fjsCOMPLETED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(256i32);
pub const fjsCANCELED: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(512i32);
pub const fjsCANCELING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(1024i32);
pub const fjsROUTING: FAX_JOB_STATUS_ENUM = FAX_JOB_STATUS_ENUM(2048i32);
pub const FAX_JOB_SUBMIT: u32 = 1u32;
#[repr(transparent)]
pub struct FAX_JOB_TYPE_ENUM(pub i32);
pub const fjtSEND: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(0i32);
pub const fjtRECEIVE: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(1i32);
pub const fjtROUTING: FAX_JOB_TYPE_ENUM = FAX_JOB_TYPE_ENUM(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_LOG_CATEGORYA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_LOG_CATEGORYW(i32);
#[repr(transparent)]
pub struct FAX_LOG_LEVEL_ENUM(pub i32);
pub const fllNONE: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(0i32);
pub const fllMIN: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(1i32);
pub const fllMED: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(2i32);
pub const fllMAX: FAX_LOG_LEVEL_ENUM = FAX_LOG_LEVEL_ENUM(3i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_PORT_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_PORT_INFOW(i32);
pub const FAX_PORT_QUERY: u32 = 16u32;
pub const FAX_PORT_SET: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_PRINT_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_PRINT_INFOW(i32);
#[repr(transparent)]
pub struct FAX_PRIORITY_TYPE_ENUM(pub i32);
pub const fptLOW: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(0i32);
pub const fptNORMAL: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(1i32);
pub const fptHIGH: FAX_PRIORITY_TYPE_ENUM = FAX_PRIORITY_TYPE_ENUM(2i32);
#[repr(transparent)]
pub struct FAX_PROVIDER_STATUS_ENUM(pub i32);
pub const fpsSUCCESS: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(0i32);
pub const fpsSERVER_ERROR: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(1i32);
pub const fpsBAD_GUID: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(2i32);
pub const fpsBAD_VERSION: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(3i32);
pub const fpsCANT_LOAD: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(4i32);
pub const fpsCANT_LINK: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(5i32);
pub const fpsCANT_INIT: FAX_PROVIDER_STATUS_ENUM = FAX_PROVIDER_STATUS_ENUM(6i32);
#[repr(transparent)]
pub struct FAX_RECEIPT_TYPE_ENUM(pub i32);
pub const frtNONE: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(0i32);
pub const frtMAIL: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(1i32);
pub const frtMSGBOX: FAX_RECEIPT_TYPE_ENUM = FAX_RECEIPT_TYPE_ENUM(4i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_RECEIVE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_ROUTE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_ROUTE_CALLBACKROUTINES(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_ROUTING_METHODA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_ROUTING_METHODW(i32);
#[repr(transparent)]
pub struct FAX_ROUTING_RULE_CODE_ENUM(pub i32);
pub const frrcANY_CODE: FAX_ROUTING_RULE_CODE_ENUM = FAX_ROUTING_RULE_CODE_ENUM(0i32);
#[repr(transparent)]
pub struct FAX_RULE_STATUS_ENUM(pub i32);
pub const frsVALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(0i32);
pub const frsEMPTY_GROUP: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(1i32);
pub const frsALL_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(2i32);
pub const frsSOME_GROUP_DEV_NOT_VALID: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(3i32);
pub const frsBAD_DEVICE: FAX_RULE_STATUS_ENUM = FAX_RULE_STATUS_ENUM(4i32);
#[repr(transparent)]
pub struct FAX_SCHEDULE_TYPE_ENUM(pub i32);
pub const fstNOW: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(0i32);
pub const fstSPECIFIC_TIME: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(1i32);
pub const fstDISCOUNT_PERIOD: FAX_SCHEDULE_TYPE_ENUM = FAX_SCHEDULE_TYPE_ENUM(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FAX_SEND(i32);
#[repr(transparent)]
pub struct FAX_SERVER_APIVERSION_ENUM(pub i32);
pub const fsAPI_VERSION_0: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(0i32);
pub const fsAPI_VERSION_1: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(65536i32);
pub const fsAPI_VERSION_2: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(131072i32);
pub const fsAPI_VERSION_3: FAX_SERVER_APIVERSION_ENUM = FAX_SERVER_APIVERSION_ENUM(196608i32);
#[repr(transparent)]
pub struct FAX_SERVER_EVENTS_TYPE_ENUM(pub i32);
pub const fsetNONE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(0i32);
pub const fsetIN_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(1i32);
pub const fsetOUT_QUEUE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(2i32);
pub const fsetCONFIG: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(4i32);
pub const fsetACTIVITY: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(8i32);
pub const fsetQUEUE_STATE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(16i32);
pub const fsetIN_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(32i32);
pub const fsetOUT_ARCHIVE: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(64i32);
pub const fsetFXSSVC_ENDED: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(128i32);
pub const fsetDEVICE_STATUS: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(256i32);
pub const fsetINCOMING_CALL: FAX_SERVER_EVENTS_TYPE_ENUM = FAX_SERVER_EVENTS_TYPE_ENUM(512i32);
#[repr(transparent)]
pub struct FAX_SMTP_AUTHENTICATION_TYPE_ENUM(pub i32);
pub const fsatANONYMOUS: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(0i32);
pub const fsatBASIC: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(1i32);
pub const fsatNTLM: FAX_SMTP_AUTHENTICATION_TYPE_ENUM = FAX_SMTP_AUTHENTICATION_TYPE_ENUM(2i32);
#[repr(C)]
pub struct FAX_TIME(i32);
pub const FEI_ABORTING: u32 = 15u32;
pub const FEI_ANSWERED: u32 = 21u32;
pub const FEI_BAD_ADDRESS: u32 = 7u32;
pub const FEI_BUSY: u32 = 5u32;
pub const FEI_CALL_BLACKLISTED: u32 = 13u32;
pub const FEI_CALL_DELAYED: u32 = 12u32;
pub const FEI_COMPLETED: u32 = 4u32;
pub const FEI_DELETED: u32 = 23u32;
pub const FEI_DIALING: u32 = 1u32;
pub const FEI_DISCONNECTED: u32 = 9u32;
pub const FEI_FATAL_ERROR: u32 = 10u32;
pub const FEI_FAXSVC_ENDED: u32 = 20u32;
pub const FEI_FAXSVC_STARTED: u32 = 27u32;
pub const FEI_HANDLED: u32 = 26u32;
pub const FEI_IDLE: u32 = 19u32;
pub const FEI_INITIALIZING: u32 = 24u32;
pub const FEI_JOB_QUEUED: u32 = 22u32;
pub const FEI_LINE_UNAVAILABLE: u32 = 25u32;
pub const FEI_MODEM_POWERED_OFF: u32 = 18u32;
pub const FEI_MODEM_POWERED_ON: u32 = 17u32;
pub const FEI_NEVENTS: u32 = 27u32;
pub const FEI_NOT_FAX_CALL: u32 = 11u32;
pub const FEI_NO_ANSWER: u32 = 6u32;
pub const FEI_NO_DIAL_TONE: u32 = 8u32;
pub const FEI_RECEIVING: u32 = 3u32;
pub const FEI_RINGING: u32 = 14u32;
pub const FEI_ROUTING: u32 = 16u32;
pub const FEI_SENDING: u32 = 2u32;
pub const FPF_RECEIVE: u32 = 1u32;
pub const FPF_SEND: u32 = 2u32;
pub const FPF_VIRTUAL: u32 = 4u32;
pub const FPS_ABORTING: u32 = 538968064u32;
pub const FPS_ANSWERED: u32 = 545259520u32;
pub const FPS_AVAILABLE: u32 = 537919488u32;
pub const FPS_BAD_ADDRESS: u32 = 536871168u32;
pub const FPS_BUSY: u32 = 536870976u32;
pub const FPS_CALL_BLACKLISTED: u32 = 536887296u32;
pub const FPS_CALL_DELAYED: u32 = 536879104u32;
pub const FPS_COMPLETED: u32 = 536870920u32;
pub const FPS_DIALING: u32 = 536870913u32;
pub const FPS_DISCONNECTED: u32 = 536871936u32;
pub const FPS_FATAL_ERROR: u32 = 536872960u32;
pub const FPS_HANDLED: u32 = 536870928u32;
pub const FPS_INITIALIZING: u32 = 536903680u32;
pub const FPS_NOT_FAX_CALL: u32 = 536875008u32;
pub const FPS_NO_ANSWER: u32 = 536871040u32;
pub const FPS_NO_DIAL_TONE: u32 = 536871424u32;
pub const FPS_OFFLINE: u32 = 536936448u32;
pub const FPS_RECEIVING: u32 = 536870916u32;
pub const FPS_RINGING: u32 = 537001984u32;
pub const FPS_ROUTING: u32 = 541065216u32;
pub const FPS_SENDING: u32 = 536870914u32;
pub const FPS_UNAVAILABLE: u32 = 536870944u32;
pub const FS_ANSWERED: u32 = 545259520u32;
pub const FS_BAD_ADDRESS: u32 = 536871168u32;
pub const FS_BUSY: u32 = 536870976u32;
pub const FS_CALL_BLACKLISTED: u32 = 536887296u32;
pub const FS_CALL_DELAYED: u32 = 536879104u32;
pub const FS_COMPLETED: u32 = 536870920u32;
pub const FS_DIALING: u32 = 536870913u32;
pub const FS_DISCONNECTED: u32 = 536871936u32;
pub const FS_FATAL_ERROR: u32 = 536872960u32;
pub const FS_HANDLED: u32 = 536870928u32;
pub const FS_INITIALIZING: u32 = 536870912u32;
pub const FS_LINE_UNAVAILABLE: u32 = 536870944u32;
pub const FS_NOT_FAX_CALL: u32 = 536875008u32;
pub const FS_NO_ANSWER: u32 = 536871040u32;
pub const FS_NO_DIAL_TONE: u32 = 536871424u32;
pub const FS_RECEIVING: u32 = 536870916u32;
pub const FS_TRANSMITTING: u32 = 536870914u32;
pub const FS_USER_ABORT: u32 = 538968064u32;
#[repr(C)]
pub struct FaxAccount(i32);
#[repr(C)]
pub struct FaxAccountFolders(i32);
#[repr(C)]
pub struct FaxAccountIncomingArchive(i32);
#[repr(C)]
pub struct FaxAccountIncomingQueue(i32);
#[repr(C)]
pub struct FaxAccountOutgoingArchive(i32);
#[repr(C)]
pub struct FaxAccountOutgoingQueue(i32);
#[repr(C)]
pub struct FaxAccountSet(i32);
#[repr(C)]
pub struct FaxAccounts(i32);
#[repr(C)]
pub struct FaxActivity(i32);
#[repr(C)]
pub struct FaxActivityLogging(i32);
#[repr(C)]
pub struct FaxConfiguration(i32);
#[repr(C)]
pub struct FaxDevice(i32);
#[repr(C)]
pub struct FaxDeviceIds(i32);
#[repr(C)]
pub struct FaxDeviceProvider(i32);
#[repr(C)]
pub struct FaxDeviceProviders(i32);
#[repr(C)]
pub struct FaxDevices(i32);
#[repr(C)]
pub struct FaxDocument(i32);
#[repr(C)]
pub struct FaxEventLogging(i32);
#[repr(C)]
pub struct FaxFolders(i32);
#[repr(C)]
pub struct FaxInboundRouting(i32);
#[repr(C)]
pub struct FaxInboundRoutingExtension(i32);
#[repr(C)]
pub struct FaxInboundRoutingExtensions(i32);
#[repr(C)]
pub struct FaxInboundRoutingMethod(i32);
#[repr(C)]
pub struct FaxInboundRoutingMethods(i32);
#[repr(C)]
pub struct FaxIncomingArchive(i32);
#[repr(C)]
pub struct FaxIncomingJob(i32);
#[repr(C)]
pub struct FaxIncomingJobs(i32);
#[repr(C)]
pub struct FaxIncomingMessage(i32);
#[repr(C)]
pub struct FaxIncomingMessageIterator(i32);
#[repr(C)]
pub struct FaxIncomingQueue(i32);
#[repr(C)]
pub struct FaxJobStatus(i32);
#[repr(C)]
pub struct FaxLoggingOptions(i32);
#[repr(C)]
pub struct FaxOutboundRouting(i32);
#[repr(C)]
pub struct FaxOutboundRoutingGroup(i32);
#[repr(C)]
pub struct FaxOutboundRoutingGroups(i32);
#[repr(C)]
pub struct FaxOutboundRoutingRule(i32);
#[repr(C)]
pub struct FaxOutboundRoutingRules(i32);
#[repr(C)]
pub struct FaxOutgoingArchive(i32);
#[repr(C)]
pub struct FaxOutgoingJob(i32);
#[repr(C)]
pub struct FaxOutgoingJobs(i32);
#[repr(C)]
pub struct FaxOutgoingMessage(i32);
#[repr(C)]
pub struct FaxOutgoingMessageIterator(i32);
#[repr(C)]
pub struct FaxOutgoingQueue(i32);
#[repr(C)]
pub struct FaxReceiptOptions(i32);
#[repr(C)]
pub struct FaxRecipient(i32);
#[repr(C)]
pub struct FaxRecipients(i32);
#[repr(C)]
pub struct FaxSecurity(i32);
#[repr(C)]
pub struct FaxSecurity2(i32);
#[repr(C)]
pub struct FaxSender(i32);
#[repr(C)]
pub struct FaxServer(i32);
pub const GUID_DeviceArrivedLaunch: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1947049702, data2: 28913, data3: 4561, data4: [173, 16, 0, 160, 36, 56, 173, 72] };
pub const GUID_STIUserDefined1: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3222189973, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_STIUserDefined2: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3346721221, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_STIUserDefined3: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3346721222, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_ScanFaxImage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3222189971, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_ScanImage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2797971221, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
pub const GUID_ScanPrintImage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3024221221, data2: 35950, data3: 4562, data4: [151, 122, 0, 0, 248, 122, 146, 111] };
#[repr(transparent)]
pub struct IFaxAccount(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxAccountFolders(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxAccountIncomingArchive(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxAccountIncomingQueue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxAccountNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxAccountOutgoingArchive(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxAccountOutgoingQueue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxAccountSet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxAccounts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxActivity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxActivityLogging(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxDeviceIds(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxDeviceProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxDeviceProviders(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxDevices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxDocument2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxEventLogging(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxFolders(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxInboundRouting(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxInboundRoutingExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxInboundRoutingExtensions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxInboundRoutingMethod(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxInboundRoutingMethods(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxIncomingArchive(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxIncomingJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxIncomingJobs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxIncomingMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxIncomingMessage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxIncomingMessageIterator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxIncomingQueue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxJobStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxLoggingOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxOutboundRouting(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxOutboundRoutingGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxOutboundRoutingGroups(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxOutboundRoutingRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxOutboundRoutingRules(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxOutgoingArchive(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxOutgoingJob(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxOutgoingJob2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxOutgoingJobs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxOutgoingMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxOutgoingMessage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxOutgoingMessageIterator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxOutgoingQueue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxReceiptOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxRecipient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxRecipients(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxSecurity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxSecurity2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxSender(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxServer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxServer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxServerNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFaxServerNotify2(pub *mut ::core::ffi::c_void);
pub const IS_DIGITAL_CAMERA_VAL: u32 = 1u32;
#[repr(transparent)]
pub struct IStiDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStiDeviceControl(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct IStiDeviceW(i32);
#[repr(transparent)]
pub struct IStiUSD(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStillImageW(pub *mut ::core::ffi::c_void);
pub const JS_DELETING: u32 = 2u32;
pub const JS_FAILED: u32 = 4u32;
pub const JS_INPROGRESS: u32 = 1u32;
pub const JS_NOLINE: u32 = 16u32;
pub const JS_PAUSED: u32 = 8u32;
pub const JS_PENDING: u32 = 0u32;
pub const JS_RETRIES_EXCEEDED: u32 = 64u32;
pub const JS_RETRYING: u32 = 32u32;
pub const JT_FAIL_RECEIVE: u32 = 4u32;
pub const JT_RECEIVE: u32 = 2u32;
pub const JT_ROUTING: u32 = 3u32;
pub const JT_SEND: u32 = 1u32;
pub const JT_UNKNOWN: u32 = 0u32;
pub const MAX_NOTIFICATION_DATA: u32 = 64u32;
#[repr(C)]
pub struct PFAXABORT(i32);
#[repr(C)]
pub struct PFAXACCESSCHECK(i32);
#[repr(C)]
pub struct PFAXCLOSE(i32);
#[repr(C)]
pub struct PFAXCOMPLETEJOBPARAMSA(i32);
#[repr(C)]
pub struct PFAXCOMPLETEJOBPARAMSW(i32);
#[repr(C)]
pub struct PFAXCONNECTFAXSERVERA(i32);
#[repr(C)]
pub struct PFAXCONNECTFAXSERVERW(i32);
#[repr(C)]
pub struct PFAXDEVABORTOPERATION(i32);
#[repr(C)]
pub struct PFAXDEVCONFIGURE(i32);
#[repr(C)]
pub struct PFAXDEVENDJOB(i32);
#[repr(C)]
pub struct PFAXDEVINITIALIZE(i32);
#[repr(C)]
pub struct PFAXDEVRECEIVE(i32);
#[repr(C)]
pub struct PFAXDEVREPORTSTATUS(i32);
#[repr(C)]
pub struct PFAXDEVSEND(i32);
#[repr(C)]
pub struct PFAXDEVSHUTDOWN(i32);
#[repr(C)]
pub struct PFAXDEVSTARTJOB(i32);
#[repr(C)]
pub struct PFAXDEVVIRTUALDEVICECREATION(i32);
#[repr(C)]
pub struct PFAXENABLEROUTINGMETHODA(i32);
#[repr(C)]
pub struct PFAXENABLEROUTINGMETHODW(i32);
#[repr(C)]
pub struct PFAXENUMGLOBALROUTINGINFOA(i32);
#[repr(C)]
pub struct PFAXENUMGLOBALROUTINGINFOW(i32);
#[repr(C)]
pub struct PFAXENUMJOBSA(i32);
#[repr(C)]
pub struct PFAXENUMJOBSW(i32);
#[repr(C)]
pub struct PFAXENUMPORTSA(i32);
#[repr(C)]
pub struct PFAXENUMPORTSW(i32);
#[repr(C)]
pub struct PFAXENUMROUTINGMETHODSA(i32);
#[repr(C)]
pub struct PFAXENUMROUTINGMETHODSW(i32);
#[repr(C)]
pub struct PFAXFREEBUFFER(i32);
#[repr(C)]
pub struct PFAXGETCONFIGURATIONA(i32);
#[repr(C)]
pub struct PFAXGETCONFIGURATIONW(i32);
#[repr(C)]
pub struct PFAXGETDEVICESTATUSA(i32);
#[repr(C)]
pub struct PFAXGETDEVICESTATUSW(i32);
#[repr(C)]
pub struct PFAXGETJOBA(i32);
#[repr(C)]
pub struct PFAXGETJOBW(i32);
#[repr(C)]
pub struct PFAXGETLOGGINGCATEGORIESA(i32);
#[repr(C)]
pub struct PFAXGETLOGGINGCATEGORIESW(i32);
#[repr(C)]
pub struct PFAXGETPAGEDATA(i32);
#[repr(C)]
pub struct PFAXGETPORTA(i32);
#[repr(C)]
pub struct PFAXGETPORTW(i32);
#[repr(C)]
pub struct PFAXGETROUTINGINFOA(i32);
#[repr(C)]
pub struct PFAXGETROUTINGINFOW(i32);
#[repr(C)]
pub struct PFAXINITIALIZEEVENTQUEUE(i32);
#[repr(C)]
pub struct PFAXOPENPORT(i32);
#[repr(C)]
pub struct PFAXPRINTCOVERPAGEA(i32);
#[repr(C)]
pub struct PFAXPRINTCOVERPAGEW(i32);
#[repr(C)]
pub struct PFAXREGISTERROUTINGEXTENSIONW(i32);
#[repr(C)]
pub struct PFAXREGISTERSERVICEPROVIDERW(i32);
#[repr(C)]
pub struct PFAXROUTEADDFILE(i32);
#[repr(C)]
pub struct PFAXROUTEDELETEFILE(i32);
#[repr(C)]
pub struct PFAXROUTEDEVICECHANGENOTIFICATION(i32);
#[repr(C)]
pub struct PFAXROUTEDEVICEENABLE(i32);
#[repr(C)]
pub struct PFAXROUTEENUMFILE(i32);
#[repr(C)]
pub struct PFAXROUTEENUMFILES(i32);
#[repr(C)]
pub struct PFAXROUTEGETFILE(i32);
#[repr(C)]
pub struct PFAXROUTEGETROUTINGINFO(i32);
#[repr(C)]
pub struct PFAXROUTEINITIALIZE(i32);
#[repr(C)]
pub struct PFAXROUTEMETHOD(i32);
#[repr(C)]
pub struct PFAXROUTEMODIFYROUTINGDATA(i32);
#[repr(C)]
pub struct PFAXROUTESETROUTINGINFO(i32);
#[repr(C)]
pub struct PFAXSENDDOCUMENTA(i32);
#[repr(C)]
pub struct PFAXSENDDOCUMENTFORBROADCASTA(i32);
#[repr(C)]
pub struct PFAXSENDDOCUMENTFORBROADCASTW(i32);
#[repr(C)]
pub struct PFAXSENDDOCUMENTW(i32);
#[repr(C)]
pub struct PFAXSETCONFIGURATIONA(i32);
#[repr(C)]
pub struct PFAXSETCONFIGURATIONW(i32);
#[repr(C)]
pub struct PFAXSETGLOBALROUTINGINFOA(i32);
#[repr(C)]
pub struct PFAXSETGLOBALROUTINGINFOW(i32);
#[repr(C)]
pub struct PFAXSETJOBA(i32);
#[repr(C)]
pub struct PFAXSETJOBW(i32);
#[repr(C)]
pub struct PFAXSETLOGGINGCATEGORIESA(i32);
#[repr(C)]
pub struct PFAXSETLOGGINGCATEGORIESW(i32);
#[repr(C)]
pub struct PFAXSETPORTA(i32);
#[repr(C)]
pub struct PFAXSETPORTW(i32);
#[repr(C)]
pub struct PFAXSETROUTINGINFOA(i32);
#[repr(C)]
pub struct PFAXSETROUTINGINFOW(i32);
#[repr(C)]
pub struct PFAXSTARTPRINTJOBA(i32);
#[repr(C)]
pub struct PFAXSTARTPRINTJOBW(i32);
#[repr(C)]
pub struct PFAXUNREGISTERSERVICEPROVIDERW(i32);
#[repr(C)]
pub struct PFAX_EXT_CONFIG_CHANGE(i32);
#[repr(C)]
pub struct PFAX_EXT_FREE_BUFFER(i32);
#[repr(C)]
pub struct PFAX_EXT_GET_DATA(i32);
#[repr(C)]
pub struct PFAX_EXT_INITIALIZE_CONFIG(i32);
#[repr(C)]
pub struct PFAX_EXT_REGISTER_FOR_EVENTS(i32);
#[repr(C)]
pub struct PFAX_EXT_SET_DATA(i32);
#[repr(C)]
pub struct PFAX_EXT_UNREGISTER_FOR_EVENTS(i32);
#[repr(C)]
pub struct PFAX_LINECALLBACK(i32);
#[repr(C)]
pub struct PFAX_RECIPIENT_CALLBACKA(i32);
#[repr(C)]
pub struct PFAX_RECIPIENT_CALLBACKW(i32);
#[repr(C)]
pub struct PFAX_ROUTING_INSTALLATION_CALLBACKW(i32);
#[repr(C)]
pub struct PFAX_SEND_CALLBACK(i32);
#[repr(C)]
pub struct PFAX_SERVICE_CALLBACK(i32);
pub const STIEDFL_ALLDEVICES: u32 = 0u32;
pub const STIEDFL_ATTACHEDONLY: u32 = 1u32;
pub const STIERR_ALREADY_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147023649i32 as _);
pub const STIERR_BADDRIVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024777i32 as _);
pub const STIERR_BETA_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147023743i32 as _);
pub const STIERR_DEVICENOTREG: i32 = -2147221164i32;
pub const STIERR_DEVICE_LOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024863i32 as _);
pub const STIERR_DEVICE_NOTREADY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024875i32 as _);
pub const STIERR_GENERIC: i32 = -2147467259i32;
pub const STIERR_HANDLEEXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024713i32 as _);
pub const STIERR_INVALID_DEVICE_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024773i32 as _);
pub const STIERR_INVALID_HW_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024883i32 as _);
pub const STIERR_INVALID_PARAM: i32 = -2147024809i32;
pub const STIERR_NEEDS_LOCK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024738i32 as _);
pub const STIERR_NOEVENTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024637i32 as _);
pub const STIERR_NOINTERFACE: i32 = -2147467262i32;
pub const STIERR_NOTINITIALIZED: i32 = -2147024891i32;
pub const STIERR_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024875i32 as _);
pub const STIERR_OBJECTNOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024894i32 as _);
pub const STIERR_OLD_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147023746i32 as _);
pub const STIERR_OUTOFMEMORY: i32 = -2147024882i32;
pub const STIERR_READONLY: i32 = -2147024891i32;
pub const STIERR_SHARING_VIOLATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147024864i32 as _);
pub const STIERR_UNSUPPORTED: i32 = -2147467263i32;
#[repr(C)]
pub struct STINOTIFY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STISUBSCRIBE(i32);
pub const STI_CHANGENOEFFECT: i32 = 1i32;
pub const STI_DEVICE_CREATE_BOTH: u32 = 3u32;
pub const STI_DEVICE_CREATE_DATA: u32 = 2u32;
pub const STI_DEVICE_CREATE_FOR_MONITOR: u32 = 16777216u32;
pub const STI_DEVICE_CREATE_MASK: u32 = 65535u32;
pub const STI_DEVICE_CREATE_STATUS: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STI_DEVICE_INFORMATIONW(i32);
#[repr(transparent)]
pub struct STI_DEVICE_MJ_TYPE(pub i32);
pub const StiDeviceTypeDefault: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(0i32);
pub const StiDeviceTypeScanner: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(1i32);
pub const StiDeviceTypeDigitalCamera: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(2i32);
pub const StiDeviceTypeStreamingVideo: STI_DEVICE_MJ_TYPE = STI_DEVICE_MJ_TYPE(3i32);
#[repr(C)]
pub struct STI_DEVICE_STATUS(i32);
pub const STI_DEVSTATUS_EVENTS_STATE: u32 = 2u32;
pub const STI_DEVSTATUS_ONLINE_STATE: u32 = 1u32;
#[repr(C)]
pub struct STI_DEV_CAPS(i32);
#[repr(C)]
pub struct STI_DIAG(i32);
pub const STI_DIAGCODE_HWPRESENCE: u32 = 1u32;
pub const STI_ERROR_NO_ERROR: i32 = 0i32;
pub const STI_EVENTHANDLING_ENABLED: u32 = 1u32;
pub const STI_EVENTHANDLING_PENDING: u32 = 4u32;
pub const STI_EVENTHANDLING_POLLING: u32 = 2u32;
pub const STI_GENCAP_AUTO_PORTSELECT: u32 = 8u32;
pub const STI_GENCAP_GENERATE_ARRIVALEVENT: u32 = 4u32;
pub const STI_GENCAP_NOTIFICATIONS: u32 = 1u32;
pub const STI_GENCAP_POLLING_NEEDED: u32 = 2u32;
pub const STI_GENCAP_SUBSET: u32 = 32u32;
pub const STI_GENCAP_WIA: u32 = 16u32;
pub const STI_HW_CONFIG_PARALLEL: u32 = 16u32;
pub const STI_HW_CONFIG_SCSI: u32 = 2u32;
pub const STI_HW_CONFIG_SERIAL: u32 = 8u32;
pub const STI_HW_CONFIG_UNKNOWN: u32 = 1u32;
pub const STI_HW_CONFIG_USB: u32 = 4u32;
pub const STI_MAX_INTERNAL_NAME_LENGTH: u32 = 128u32;
pub const STI_NOTCONNECTED: i32 = 1i32;
pub const STI_OK: i32 = 0i32;
pub const STI_ONLINESTATE_BUSY: u32 = 256u32;
pub const STI_ONLINESTATE_ERROR: u32 = 4u32;
pub const STI_ONLINESTATE_INITIALIZING: u32 = 1024u32;
pub const STI_ONLINESTATE_IO_ACTIVE: u32 = 128u32;
pub const STI_ONLINESTATE_OFFLINE: u32 = 64u32;
pub const STI_ONLINESTATE_OPERATIONAL: u32 = 1u32;
pub const STI_ONLINESTATE_PAPER_JAM: u32 = 16u32;
pub const STI_ONLINESTATE_PAPER_PROBLEM: u32 = 32u32;
pub const STI_ONLINESTATE_PAUSED: u32 = 8u32;
pub const STI_ONLINESTATE_PENDING: u32 = 2u32;
pub const STI_ONLINESTATE_POWER_SAVE: u32 = 8192u32;
pub const STI_ONLINESTATE_TRANSFERRING: u32 = 512u32;
pub const STI_ONLINESTATE_USER_INTERVENTION: u32 = 4096u32;
pub const STI_ONLINESTATE_WARMING_UP: u32 = 2048u32;
pub const STI_RAW_RESERVED: u32 = 4096u32;
pub const STI_SUBSCRIBE_FLAG_EVENT: u32 = 2u32;
pub const STI_SUBSCRIBE_FLAG_WINDOW: u32 = 1u32;
pub const STI_TRACE_ERROR: u32 = 4u32;
pub const STI_TRACE_INFORMATION: u32 = 1u32;
pub const STI_TRACE_WARNING: u32 = 2u32;
pub const STI_UNICODE: u32 = 1u32;
#[repr(C)]
pub struct STI_USD_CAPS(i32);
pub const STI_USD_GENCAP_NATIVE_PUSHSUPPORT: u32 = 1u32;
pub const STI_VERSION: u32 = 2u32;
pub const STI_VERSION_FLAG_MASK: u32 = 4278190080u32;
pub const STI_VERSION_FLAG_UNICODE: u32 = 16777216u32;
pub const STI_VERSION_MIN_ALLOWED: u32 = 2u32;
pub const STI_VERSION_REAL: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct STI_WIA_DEVICE_INFORMATIONW(i32);
pub const SUPPORTS_MSCPLUS_VAL: u32 = 1u32;
#[repr(transparent)]
pub struct SendToMode(pub i32);
pub const SEND_TO_FAX_RECIPIENT_ATTACHMENT: SendToMode = SendToMode(0i32);
pub const WIA_INCOMPAT_XP: u32 = 1u32;
#[repr(C)]
pub struct _ERROR_INFOW(i32);
#[repr(transparent)]
pub struct _IFaxAccountNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _IFaxServerNotify2(pub *mut ::core::ffi::c_void);
pub const lDEFAULT_PREFETCH_SIZE: i32 = 100i32;
pub const wcharREASSIGN_RECIPIENTS_DELIMITER: u16 = 59u16;
