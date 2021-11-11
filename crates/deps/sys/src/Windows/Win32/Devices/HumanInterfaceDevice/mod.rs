#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DirectInput8Create(hinst: super::super::Foundation::HINSTANCE, dwversion: u32, riidltf: *const ::windows::runtime::GUID, ppvout: *mut *mut ::core::ffi::c_void, punkouter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_FlushQueue(hiddeviceobject: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_FreePreparsedData(preparseddata: isize) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetAttributes(hiddeviceobject: super::super::Foundation::HANDLE, attributes: *mut HIDD_ATTRIBUTES) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetConfiguration(hiddeviceobject: super::super::Foundation::HANDLE, configuration: *mut HIDD_CONFIGURATION, configurationlength: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetFeature(hiddeviceobject: super::super::Foundation::HANDLE, reportbuffer: *mut ::core::ffi::c_void, reportbufferlength: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`*"]
    pub fn HidD_GetHidGuid(hidguid: *mut ::windows::runtime::GUID);
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetIndexedString(hiddeviceobject: super::super::Foundation::HANDLE, stringindex: u32, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetInputReport(hiddeviceobject: super::super::Foundation::HANDLE, reportbuffer: *mut ::core::ffi::c_void, reportbufferlength: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetManufacturerString(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetMsGenreDescriptor(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetNumInputBuffers(hiddeviceobject: super::super::Foundation::HANDLE, numberbuffers: *mut u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetPhysicalDescriptor(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetPreparsedData(hiddeviceobject: super::super::Foundation::HANDLE, preparseddata: *mut isize) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetProductString(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_GetSerialNumberString(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut ::core::ffi::c_void, bufferlength: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_SetConfiguration(hiddeviceobject: super::super::Foundation::HANDLE, configuration: *const HIDD_CONFIGURATION, configurationlength: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_SetFeature(hiddeviceobject: super::super::Foundation::HANDLE, reportbuffer: *const ::core::ffi::c_void, reportbufferlength: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_SetNumInputBuffers(hiddeviceobject: super::super::Foundation::HANDLE, numberbuffers: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidD_SetOutputReport(hiddeviceobject: super::super::Foundation::HANDLE, reportbuffer: *const ::core::ffi::c_void, reportbufferlength: u32) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetButtonArray(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, buttondata: *mut HIDP_BUTTON_ARRAY_DATA, buttondatalength: *mut u16, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetButtonCaps(reporttype: HIDP_REPORT_TYPE, buttoncaps: *mut HIDP_BUTTON_CAPS, buttoncapslength: *mut u16, preparseddata: isize) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetCaps(preparseddata: isize, capabilities: *mut HIDP_CAPS) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetData(reporttype: HIDP_REPORT_TYPE, datalist: *mut HIDP_DATA, datalength: *mut u32, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetExtendedAttributes(reporttype: HIDP_REPORT_TYPE, dataindex: u16, preparseddata: isize, attributes: *mut HIDP_EXTENDED_ATTRIBUTES, lengthattributes: *mut u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetLinkCollectionNodes(linkcollectionnodes: *mut HIDP_LINK_COLLECTION_NODE, linkcollectionnodeslength: *mut u32, preparseddata: isize) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetScaledUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, usagevalue: *mut i32, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetSpecificButtonCaps(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, buttoncaps: *mut HIDP_BUTTON_CAPS, buttoncapslength: *mut u16, preparseddata: isize) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetSpecificValueCaps(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, valuecaps: *mut HIDP_VALUE_CAPS, valuecapslength: *mut u16, preparseddata: isize) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, usagevalue: *mut u32, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetUsageValueArray(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, usagevalue: super::super::Foundation::PSTR, usagevaluebytelength: u16, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetUsages(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usagelist: *mut u16, usagelength: *mut u32, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetUsagesEx(reporttype: HIDP_REPORT_TYPE, linkcollection: u16, buttonlist: *mut USAGE_AND_PAGE, usagelength: *mut u32, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_GetValueCaps(reporttype: HIDP_REPORT_TYPE, valuecaps: *mut HIDP_VALUE_CAPS, valuecapslength: *mut u16, preparseddata: isize) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_InitializeReportForID(reporttype: HIDP_REPORT_TYPE, reportid: u8, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`*"]
    pub fn HidP_MaxDataListLength(reporttype: HIDP_REPORT_TYPE, preparseddata: isize) -> u32;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`*"]
    pub fn HidP_MaxUsageListLength(reporttype: HIDP_REPORT_TYPE, usagepage: u16, preparseddata: isize) -> u32;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_SetButtonArray(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, buttondata: *const HIDP_BUTTON_ARRAY_DATA, buttondatalength: u16, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_SetData(reporttype: HIDP_REPORT_TYPE, datalist: *mut HIDP_DATA, datalength: *mut u32, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_SetScaledUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, usagevalue: i32, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_SetUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, usagevalue: u32, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_SetUsageValueArray(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usage: u16, usagevalue: super::super::Foundation::PSTR, usagevaluebytelength: u16, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_SetUsages(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usagelist: *mut u16, usagelength: *mut u32, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_TranslateUsagesToI8042ScanCodes(changedusagelist: *const u16, usagelistlength: u32, keyaction: HIDP_KEYBOARD_DIRECTION, modifierstate: *mut HIDP_KEYBOARD_MODIFIER_STATE, insertcodesprocedure: ::windows::runtime::RawPtr, insertcodescontext: *const ::core::ffi::c_void) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_UnsetUsages(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: u16, usagelist: *mut u16, usagelength: *mut u32, preparseddata: isize, report: super::super::Foundation::PSTR, reportlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HidP_UsageListDifference(previoususagelist: *const u16, currentusagelist: *const u16, breakusagelist: *mut u16, makeusagelist: *mut u16, usagelistlength: u32) -> super::super::Foundation::NTSTATUS;
    #[doc = "*Required features: `Win32_Devices_HumanInterfaceDevice`*"]
    pub fn joyConfigChanged(dwflags: u32) -> u32;
}
