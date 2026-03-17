#[inline]
pub unsafe fn DirectInput8Create<P4>(hinst: super::super::Foundation::HINSTANCE, dwversion: u32, riidltf: *const windows_core::GUID, ppvout: *mut *mut core::ffi::c_void, punkouter: P4) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("dinput8.dll" "system" fn DirectInput8Create(hinst : super::super::Foundation:: HINSTANCE, dwversion : u32, riidltf : *const windows_core::GUID, ppvout : *mut *mut core::ffi::c_void, punkouter : * mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectInput8Create(hinst, dwversion, riidltf, ppvout as _, punkouter.param().abi()).ok() }
}
#[inline]
pub unsafe fn HidD_FlushQueue(hiddeviceobject: super::super::Foundation::HANDLE) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_FlushQueue(hiddeviceobject : super::super::Foundation:: HANDLE) -> bool);
    unsafe { HidD_FlushQueue(hiddeviceobject) }
}
#[inline]
pub unsafe fn HidD_FreePreparsedData(preparseddata: PHIDP_PREPARSED_DATA) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_FreePreparsedData(preparseddata : PHIDP_PREPARSED_DATA) -> bool);
    unsafe { HidD_FreePreparsedData(preparseddata) }
}
#[inline]
pub unsafe fn HidD_GetAttributes(hiddeviceobject: super::super::Foundation::HANDLE, attributes: *mut HIDD_ATTRIBUTES) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetAttributes(hiddeviceobject : super::super::Foundation:: HANDLE, attributes : *mut HIDD_ATTRIBUTES) -> bool);
    unsafe { HidD_GetAttributes(hiddeviceobject, attributes as _) }
}
#[inline]
pub unsafe fn HidD_GetConfiguration(hiddeviceobject: super::super::Foundation::HANDLE, configuration: *mut HIDD_CONFIGURATION, configurationlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetConfiguration(hiddeviceobject : super::super::Foundation:: HANDLE, configuration : *mut HIDD_CONFIGURATION, configurationlength : u32) -> bool);
    unsafe { HidD_GetConfiguration(hiddeviceobject, configuration as _, configurationlength) }
}
#[inline]
pub unsafe fn HidD_GetFeature(hiddeviceobject: super::super::Foundation::HANDLE, reportbuffer: *mut core::ffi::c_void, reportbufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetFeature(hiddeviceobject : super::super::Foundation:: HANDLE, reportbuffer : *mut core::ffi::c_void, reportbufferlength : u32) -> bool);
    unsafe { HidD_GetFeature(hiddeviceobject, reportbuffer as _, reportbufferlength) }
}
#[inline]
pub unsafe fn HidD_GetHidGuid() -> windows_core::GUID {
    windows_core::link!("hid.dll" "system" fn HidD_GetHidGuid(hidguid : *mut windows_core::GUID));
    unsafe {
        let mut result__ = core::mem::zeroed();
        HidD_GetHidGuid(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn HidD_GetIndexedString(hiddeviceobject: super::super::Foundation::HANDLE, stringindex: u32, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetIndexedString(hiddeviceobject : super::super::Foundation:: HANDLE, stringindex : u32, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetIndexedString(hiddeviceobject, stringindex, buffer as _, bufferlength) }
}
#[inline]
pub unsafe fn HidD_GetInputReport(hiddeviceobject: super::super::Foundation::HANDLE, reportbuffer: *mut core::ffi::c_void, reportbufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetInputReport(hiddeviceobject : super::super::Foundation:: HANDLE, reportbuffer : *mut core::ffi::c_void, reportbufferlength : u32) -> bool);
    unsafe { HidD_GetInputReport(hiddeviceobject, reportbuffer as _, reportbufferlength) }
}
#[inline]
pub unsafe fn HidD_GetManufacturerString(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetManufacturerString(hiddeviceobject : super::super::Foundation:: HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetManufacturerString(hiddeviceobject, buffer as _, bufferlength) }
}
#[inline]
pub unsafe fn HidD_GetMsGenreDescriptor(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetMsGenreDescriptor(hiddeviceobject : super::super::Foundation:: HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetMsGenreDescriptor(hiddeviceobject, buffer as _, bufferlength) }
}
#[inline]
pub unsafe fn HidD_GetNumInputBuffers(hiddeviceobject: super::super::Foundation::HANDLE, numberbuffers: *mut u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetNumInputBuffers(hiddeviceobject : super::super::Foundation:: HANDLE, numberbuffers : *mut u32) -> bool);
    unsafe { HidD_GetNumInputBuffers(hiddeviceobject, numberbuffers as _) }
}
#[inline]
pub unsafe fn HidD_GetPhysicalDescriptor(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetPhysicalDescriptor(hiddeviceobject : super::super::Foundation:: HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetPhysicalDescriptor(hiddeviceobject, buffer as _, bufferlength) }
}
#[inline]
pub unsafe fn HidD_GetPreparsedData(hiddeviceobject: super::super::Foundation::HANDLE, preparseddata: *mut PHIDP_PREPARSED_DATA) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetPreparsedData(hiddeviceobject : super::super::Foundation:: HANDLE, preparseddata : *mut PHIDP_PREPARSED_DATA) -> bool);
    unsafe { HidD_GetPreparsedData(hiddeviceobject, preparseddata as _) }
}
#[inline]
pub unsafe fn HidD_GetProductString(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetProductString(hiddeviceobject : super::super::Foundation:: HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetProductString(hiddeviceobject, buffer as _, bufferlength) }
}
#[inline]
pub unsafe fn HidD_GetSerialNumberString(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetSerialNumberString(hiddeviceobject : super::super::Foundation:: HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetSerialNumberString(hiddeviceobject, buffer as _, bufferlength) }
}
#[inline]
pub unsafe fn HidD_SetConfiguration(hiddeviceobject: super::super::Foundation::HANDLE, configuration: *const HIDD_CONFIGURATION, configurationlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_SetConfiguration(hiddeviceobject : super::super::Foundation:: HANDLE, configuration : *const HIDD_CONFIGURATION, configurationlength : u32) -> bool);
    unsafe { HidD_SetConfiguration(hiddeviceobject, configuration, configurationlength) }
}
#[inline]
pub unsafe fn HidD_SetFeature(hiddeviceobject: super::super::Foundation::HANDLE, reportbuffer: *const core::ffi::c_void, reportbufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_SetFeature(hiddeviceobject : super::super::Foundation:: HANDLE, reportbuffer : *const core::ffi::c_void, reportbufferlength : u32) -> bool);
    unsafe { HidD_SetFeature(hiddeviceobject, reportbuffer, reportbufferlength) }
}
#[inline]
pub unsafe fn HidD_SetNumInputBuffers(hiddeviceobject: super::super::Foundation::HANDLE, numberbuffers: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_SetNumInputBuffers(hiddeviceobject : super::super::Foundation:: HANDLE, numberbuffers : u32) -> bool);
    unsafe { HidD_SetNumInputBuffers(hiddeviceobject, numberbuffers) }
}
#[inline]
pub unsafe fn HidD_SetOutputReport(hiddeviceobject: super::super::Foundation::HANDLE, reportbuffer: *const core::ffi::c_void, reportbufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_SetOutputReport(hiddeviceobject : super::super::Foundation:: HANDLE, reportbuffer : *const core::ffi::c_void, reportbufferlength : u32) -> bool);
    unsafe { HidD_SetOutputReport(hiddeviceobject, reportbuffer, reportbufferlength) }
}
#[inline]
pub unsafe fn HidP_GetButtonArray(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, buttondata: *mut HIDP_BUTTON_ARRAY_DATA, buttondatalength: *mut u16, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, buttondata : *mut HIDP_BUTTON_ARRAY_DATA, buttondatalength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetButtonArray(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, buttondata as _, buttondatalength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_GetButtonCaps(reporttype: HIDP_REPORT_TYPE, buttoncaps: *mut HIDP_BUTTON_CAPS, buttoncapslength: *mut u16, preparseddata: PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetButtonCaps(reporttype : HIDP_REPORT_TYPE, buttoncaps : *mut HIDP_BUTTON_CAPS, buttoncapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetButtonCaps(reporttype, buttoncaps as _, buttoncapslength as _, preparseddata) }
}
#[inline]
pub unsafe fn HidP_GetCaps(preparseddata: PHIDP_PREPARSED_DATA, capabilities: *mut HIDP_CAPS) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetCaps(preparseddata : PHIDP_PREPARSED_DATA, capabilities : *mut HIDP_CAPS) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetCaps(preparseddata, capabilities as _) }
}
#[inline]
pub unsafe fn HidP_GetData(reporttype: HIDP_REPORT_TYPE, datalist: *mut HIDP_DATA, datalength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetData(reporttype : HIDP_REPORT_TYPE, datalist : *mut HIDP_DATA, datalength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetData(reporttype, datalist as _, datalength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_GetExtendedAttributes(reporttype: HIDP_REPORT_TYPE, dataindex: u16, preparseddata: PHIDP_PREPARSED_DATA, attributes: *mut HIDP_EXTENDED_ATTRIBUTES, lengthattributes: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetExtendedAttributes(reporttype : HIDP_REPORT_TYPE, dataindex : u16, preparseddata : PHIDP_PREPARSED_DATA, attributes : *mut HIDP_EXTENDED_ATTRIBUTES, lengthattributes : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetExtendedAttributes(reporttype, dataindex, preparseddata, attributes as _, lengthattributes as _) }
}
#[inline]
pub unsafe fn HidP_GetLinkCollectionNodes(linkcollectionnodes: *mut HIDP_LINK_COLLECTION_NODE, linkcollectionnodeslength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetLinkCollectionNodes(linkcollectionnodes : *mut HIDP_LINK_COLLECTION_NODE, linkcollectionnodeslength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetLinkCollectionNodes(linkcollectionnodes as _, linkcollectionnodeslength as _, preparseddata) }
}
#[inline]
pub unsafe fn HidP_GetScaledUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, usagevalue: *mut i32, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : *mut i32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetScaledUsageValue(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, usagevalue as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_GetSpecificButtonCaps(reporttype: HIDP_REPORT_TYPE, usagepage: Option<u16>, linkcollection: Option<u16>, usage: Option<u16>, buttoncaps: *mut HIDP_BUTTON_CAPS, buttoncapslength: *mut u16, preparseddata: PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetSpecificButtonCaps(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, buttoncaps : *mut HIDP_BUTTON_CAPS, buttoncapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetSpecificButtonCaps(reporttype, usagepage.unwrap_or(core::mem::zeroed()) as _, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage.unwrap_or(core::mem::zeroed()) as _, buttoncaps as _, buttoncapslength as _, preparseddata) }
}
#[inline]
pub unsafe fn HidP_GetSpecificValueCaps(reporttype: HIDP_REPORT_TYPE, usagepage: Option<u16>, linkcollection: Option<u16>, usage: Option<u16>, valuecaps: *mut HIDP_VALUE_CAPS, valuecapslength: *mut u16, preparseddata: PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetSpecificValueCaps(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, valuecaps : *mut HIDP_VALUE_CAPS, valuecapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetSpecificValueCaps(reporttype, usagepage.unwrap_or(core::mem::zeroed()) as _, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage.unwrap_or(core::mem::zeroed()) as _, valuecaps as _, valuecapslength as _, preparseddata) }
}
#[inline]
pub unsafe fn HidP_GetUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, usagevalue: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetUsageValue(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, usagevalue as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_GetUsageValueArray(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, usagevalue: &mut [u8], preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : windows_core::PSTR, usagevaluebytelength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetUsageValueArray(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, core::mem::transmute(usagevalue.as_ptr()), usagevalue.len().try_into().unwrap(), preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_GetUsages(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usagelist: *mut u16, usagelength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetUsages(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usagelist as _, usagelength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_GetUsagesEx(reporttype: HIDP_REPORT_TYPE, linkcollection: Option<u16>, buttonlist: *mut USAGE_AND_PAGE, usagelength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetUsagesEx(reporttype : HIDP_REPORT_TYPE, linkcollection : u16, buttonlist : *mut USAGE_AND_PAGE, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetUsagesEx(reporttype, linkcollection.unwrap_or(core::mem::zeroed()) as _, buttonlist as _, usagelength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_GetValueCaps(reporttype: HIDP_REPORT_TYPE, valuecaps: *mut HIDP_VALUE_CAPS, valuecapslength: *mut u16, preparseddata: PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetValueCaps(reporttype : HIDP_REPORT_TYPE, valuecaps : *mut HIDP_VALUE_CAPS, valuecapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> windows_core:: NTSTATUS);
    unsafe { HidP_GetValueCaps(reporttype, valuecaps as _, valuecapslength as _, preparseddata) }
}
#[inline]
pub unsafe fn HidP_InitializeReportForID(reporttype: HIDP_REPORT_TYPE, reportid: u8, preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_InitializeReportForID(reporttype : HIDP_REPORT_TYPE, reportid : u8, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_InitializeReportForID(reporttype, reportid, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_MaxDataListLength(reporttype: HIDP_REPORT_TYPE, preparseddata: PHIDP_PREPARSED_DATA) -> u32 {
    windows_core::link!("hid.dll" "system" fn HidP_MaxDataListLength(reporttype : HIDP_REPORT_TYPE, preparseddata : PHIDP_PREPARSED_DATA) -> u32);
    unsafe { HidP_MaxDataListLength(reporttype, preparseddata) }
}
#[inline]
pub unsafe fn HidP_MaxUsageListLength(reporttype: HIDP_REPORT_TYPE, usagepage: Option<u16>, preparseddata: PHIDP_PREPARSED_DATA) -> u32 {
    windows_core::link!("hid.dll" "system" fn HidP_MaxUsageListLength(reporttype : HIDP_REPORT_TYPE, usagepage : u16, preparseddata : PHIDP_PREPARSED_DATA) -> u32);
    unsafe { HidP_MaxUsageListLength(reporttype, usagepage.unwrap_or(core::mem::zeroed()) as _, preparseddata) }
}
#[inline]
pub unsafe fn HidP_SetButtonArray(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, buttondata: &[HIDP_BUTTON_ARRAY_DATA], preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, buttondata : *const HIDP_BUTTON_ARRAY_DATA, buttondatalength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_SetButtonArray(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, core::mem::transmute(buttondata.as_ptr()), buttondata.len().try_into().unwrap(), preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_SetData(reporttype: HIDP_REPORT_TYPE, datalist: *mut HIDP_DATA, datalength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetData(reporttype : HIDP_REPORT_TYPE, datalist : *mut HIDP_DATA, datalength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_SetData(reporttype, datalist as _, datalength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_SetScaledUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, usagevalue: i32, preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : i32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_SetScaledUsageValue(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, usagevalue, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_SetUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, usagevalue: u32, preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_SetUsageValue(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, usagevalue, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_SetUsageValueArray(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, usagevalue: &[u8], preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : windows_core::PCSTR, usagevaluebytelength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_SetUsageValueArray(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, core::mem::transmute(usagevalue.as_ptr()), usagevalue.len().try_into().unwrap(), preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_SetUsages(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usagelist: *mut u16, usagelength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_SetUsages(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usagelist as _, usagelength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_TranslateUsagesToI8042ScanCodes(changedusagelist: &[u16], keyaction: HIDP_KEYBOARD_DIRECTION, modifierstate: *mut HIDP_KEYBOARD_MODIFIER_STATE, insertcodesprocedure: PHIDP_INSERT_SCANCODES, insertcodescontext: Option<*const core::ffi::c_void>) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_TranslateUsagesToI8042ScanCodes(changedusagelist : *const u16, usagelistlength : u32, keyaction : HIDP_KEYBOARD_DIRECTION, modifierstate : *mut HIDP_KEYBOARD_MODIFIER_STATE, insertcodesprocedure : PHIDP_INSERT_SCANCODES, insertcodescontext : *const core::ffi::c_void) -> windows_core:: NTSTATUS);
    unsafe { HidP_TranslateUsagesToI8042ScanCodes(core::mem::transmute(changedusagelist.as_ptr()), changedusagelist.len().try_into().unwrap(), keyaction, modifierstate as _, insertcodesprocedure, insertcodescontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn HidP_UnsetUsages(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usagelist: *mut u16, usagelength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_UnsetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_UnsetUsages(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usagelist as _, usagelength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn HidP_UsageListDifference(previoususagelist: *const u16, currentusagelist: *const u16, breakusagelist: *mut u16, makeusagelist: *mut u16, usagelistlength: u32) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_UsageListDifference(previoususagelist : *const u16, currentusagelist : *const u16, breakusagelist : *mut u16, makeusagelist : *mut u16, usagelistlength : u32) -> windows_core:: NTSTATUS);
    unsafe { HidP_UsageListDifference(previoususagelist, currentusagelist, breakusagelist as _, makeusagelist as _, usagelistlength) }
}
#[inline]
pub unsafe fn joyConfigChanged(dwflags: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn joyConfigChanged(dwflags : u32) -> u32);
    unsafe { joyConfigChanged(dwflags) }
}
pub const BALLPOINT_I8042_HARDWARE: u32 = 8u32;
pub const BALLPOINT_SERIAL_HARDWARE: u32 = 16u32;
pub const BUTTON_BIT_ALLBUTTONSMASK: u32 = 16383u32;
pub const BUTTON_BIT_BACK: u32 = 32u32;
pub const BUTTON_BIT_CAMERAFOCUS: u32 = 128u32;
pub const BUTTON_BIT_CAMERALENS: u32 = 4096u32;
pub const BUTTON_BIT_CAMERASHUTTER: u32 = 256u32;
pub const BUTTON_BIT_HEADSET: u32 = 1024u32;
pub const BUTTON_BIT_HWKBDEPLOY: u32 = 2048u32;
pub const BUTTON_BIT_OEMCUSTOM: u32 = 8192u32;
pub const BUTTON_BIT_OEMCUSTOM2: u32 = 16384u32;
pub const BUTTON_BIT_OEMCUSTOM3: u32 = 32768u32;
pub const BUTTON_BIT_POWER: u32 = 1u32;
pub const BUTTON_BIT_RINGERTOGGLE: u32 = 512u32;
pub const BUTTON_BIT_ROTATION_LOCK: u32 = 16u32;
pub const BUTTON_BIT_SEARCH: u32 = 64u32;
pub const BUTTON_BIT_VOLUMEDOWN: u32 = 8u32;
pub const BUTTON_BIT_VOLUMEUP: u32 = 4u32;
pub const BUTTON_BIT_WINDOWS: u32 = 2u32;
pub const CLSID_DirectInput: windows_core::GUID = windows_core::GUID::from_u128(0x25e609e0_b259_11cf_bfc7_444553540000);
pub const CLSID_DirectInput8: windows_core::GUID = windows_core::GUID::from_u128(0x25e609e4_b259_11cf_bfc7_444553540000);
pub const CLSID_DirectInputDevice: windows_core::GUID = windows_core::GUID::from_u128(0x25e609e1_b259_11cf_bfc7_444553540000);
pub const CLSID_DirectInputDevice8: windows_core::GUID = windows_core::GUID::from_u128(0x25e609e5_b259_11cf_bfc7_444553540000);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CPOINT {
    pub lP: i32,
    pub dwLog: u32,
}
pub const DD_KEYBOARD_DEVICE_NAME: windows_core::PCSTR = windows_core::s!("\\Device\\KeyboardClass");
pub const DD_KEYBOARD_DEVICE_NAME_U: windows_core::PCWSTR = windows_core::w!("\\Device\\KeyboardClass");
pub const DD_MOUSE_DEVICE_NAME: windows_core::PCSTR = windows_core::s!("\\Device\\PointerClass");
pub const DD_MOUSE_DEVICE_NAME_U: windows_core::PCWSTR = windows_core::w!("\\Device\\PointerClass");
pub const DEVPKEY_DeviceInterface_HID_BackgroundAccess: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 8 };
pub const DEVPKEY_DeviceInterface_HID_IsReadOnly: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 4 };
pub const DEVPKEY_DeviceInterface_HID_ProductId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 6 };
pub const DEVPKEY_DeviceInterface_HID_UsageId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 3 };
pub const DEVPKEY_DeviceInterface_HID_UsagePage: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 2 };
pub const DEVPKEY_DeviceInterface_HID_VendorId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 5 };
pub const DEVPKEY_DeviceInterface_HID_VersionNumber: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 7 };
pub const DEVPKEY_DeviceInterface_HID_WakeScreenOnInputCapable: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 9 };
pub const DI8DEVCLASS_ALL: u32 = 0u32;
pub const DI8DEVCLASS_DEVICE: u32 = 1u32;
pub const DI8DEVCLASS_GAMECTRL: u32 = 4u32;
pub const DI8DEVCLASS_KEYBOARD: u32 = 3u32;
pub const DI8DEVCLASS_POINTER: u32 = 2u32;
pub const DI8DEVTYPE1STPERSON_LIMITED: u32 = 1u32;
pub const DI8DEVTYPE1STPERSON_SHOOTER: u32 = 4u32;
pub const DI8DEVTYPE1STPERSON_SIXDOF: u32 = 3u32;
pub const DI8DEVTYPE1STPERSON_UNKNOWN: u32 = 2u32;
pub const DI8DEVTYPEDEVICECTRL_COMMSSELECTION: u32 = 3u32;
pub const DI8DEVTYPEDEVICECTRL_COMMSSELECTION_HARDWIRED: u32 = 4u32;
pub const DI8DEVTYPEDEVICECTRL_UNKNOWN: u32 = 2u32;
pub const DI8DEVTYPEDRIVING_COMBINEDPEDALS: u32 = 2u32;
pub const DI8DEVTYPEDRIVING_DUALPEDALS: u32 = 3u32;
pub const DI8DEVTYPEDRIVING_HANDHELD: u32 = 5u32;
pub const DI8DEVTYPEDRIVING_LIMITED: u32 = 1u32;
pub const DI8DEVTYPEDRIVING_THREEPEDALS: u32 = 4u32;
pub const DI8DEVTYPEFLIGHT_LIMITED: u32 = 1u32;
pub const DI8DEVTYPEFLIGHT_RC: u32 = 4u32;
pub const DI8DEVTYPEFLIGHT_STICK: u32 = 2u32;
pub const DI8DEVTYPEFLIGHT_YOKE: u32 = 3u32;
pub const DI8DEVTYPEGAMEPAD_LIMITED: u32 = 1u32;
pub const DI8DEVTYPEGAMEPAD_STANDARD: u32 = 2u32;
pub const DI8DEVTYPEGAMEPAD_TILT: u32 = 3u32;
pub const DI8DEVTYPEJOYSTICK_LIMITED: u32 = 1u32;
pub const DI8DEVTYPEJOYSTICK_STANDARD: u32 = 2u32;
pub const DI8DEVTYPEKEYBOARD_J3100: u32 = 12u32;
pub const DI8DEVTYPEKEYBOARD_JAPAN106: u32 = 10u32;
pub const DI8DEVTYPEKEYBOARD_JAPANAX: u32 = 11u32;
pub const DI8DEVTYPEKEYBOARD_NEC98: u32 = 7u32;
pub const DI8DEVTYPEKEYBOARD_NEC98106: u32 = 9u32;
pub const DI8DEVTYPEKEYBOARD_NEC98LAPTOP: u32 = 8u32;
pub const DI8DEVTYPEKEYBOARD_NOKIA1050: u32 = 5u32;
pub const DI8DEVTYPEKEYBOARD_NOKIA9140: u32 = 6u32;
pub const DI8DEVTYPEKEYBOARD_OLIVETTI: u32 = 2u32;
pub const DI8DEVTYPEKEYBOARD_PCAT: u32 = 3u32;
pub const DI8DEVTYPEKEYBOARD_PCENH: u32 = 4u32;
pub const DI8DEVTYPEKEYBOARD_PCXT: u32 = 1u32;
pub const DI8DEVTYPEKEYBOARD_UNKNOWN: u32 = 0u32;
pub const DI8DEVTYPEMOUSE_ABSOLUTE: u32 = 6u32;
pub const DI8DEVTYPEMOUSE_FINGERSTICK: u32 = 3u32;
pub const DI8DEVTYPEMOUSE_TOUCHPAD: u32 = 4u32;
pub const DI8DEVTYPEMOUSE_TRACKBALL: u32 = 5u32;
pub const DI8DEVTYPEMOUSE_TRADITIONAL: u32 = 2u32;
pub const DI8DEVTYPEMOUSE_UNKNOWN: u32 = 1u32;
pub const DI8DEVTYPEREMOTE_UNKNOWN: u32 = 2u32;
pub const DI8DEVTYPESCREENPTR_LIGHTGUN: u32 = 3u32;
pub const DI8DEVTYPESCREENPTR_LIGHTPEN: u32 = 4u32;
pub const DI8DEVTYPESCREENPTR_TOUCH: u32 = 5u32;
pub const DI8DEVTYPESCREENPTR_UNKNOWN: u32 = 2u32;
pub const DI8DEVTYPESUPPLEMENTAL_2NDHANDCONTROLLER: u32 = 3u32;
pub const DI8DEVTYPESUPPLEMENTAL_COMBINEDPEDALS: u32 = 10u32;
pub const DI8DEVTYPESUPPLEMENTAL_DUALPEDALS: u32 = 11u32;
pub const DI8DEVTYPESUPPLEMENTAL_HANDTRACKER: u32 = 5u32;
pub const DI8DEVTYPESUPPLEMENTAL_HEADTRACKER: u32 = 4u32;
pub const DI8DEVTYPESUPPLEMENTAL_RUDDERPEDALS: u32 = 13u32;
pub const DI8DEVTYPESUPPLEMENTAL_SHIFTER: u32 = 7u32;
pub const DI8DEVTYPESUPPLEMENTAL_SHIFTSTICKGATE: u32 = 6u32;
pub const DI8DEVTYPESUPPLEMENTAL_SPLITTHROTTLE: u32 = 9u32;
pub const DI8DEVTYPESUPPLEMENTAL_THREEPEDALS: u32 = 12u32;
pub const DI8DEVTYPESUPPLEMENTAL_THROTTLE: u32 = 8u32;
pub const DI8DEVTYPESUPPLEMENTAL_UNKNOWN: u32 = 2u32;
pub const DI8DEVTYPE_1STPERSON: u32 = 24u32;
pub const DI8DEVTYPE_DEVICE: u32 = 17u32;
pub const DI8DEVTYPE_DEVICECTRL: u32 = 25u32;
pub const DI8DEVTYPE_DRIVING: u32 = 22u32;
pub const DI8DEVTYPE_FLIGHT: u32 = 23u32;
pub const DI8DEVTYPE_GAMEPAD: u32 = 21u32;
pub const DI8DEVTYPE_JOYSTICK: u32 = 20u32;
pub const DI8DEVTYPE_KEYBOARD: u32 = 19u32;
pub const DI8DEVTYPE_LIMITEDGAMESUBTYPE: u32 = 1u32;
pub const DI8DEVTYPE_MOUSE: u32 = 18u32;
pub const DI8DEVTYPE_REMOTE: u32 = 27u32;
pub const DI8DEVTYPE_SCREENPOINTER: u32 = 26u32;
pub const DI8DEVTYPE_SUPPLEMENTAL: u32 = 28u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DIACTIONA {
    pub uAppData: usize,
    pub dwSemantic: u32,
    pub dwFlags: u32,
    pub Anonymous: DIACTIONA_0,
    pub guidInstance: windows_core::GUID,
    pub dwObjID: u32,
    pub dwHow: u32,
}
impl Default for DIACTIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DIACTIONA_0 {
    pub lptszActionName: windows_core::PCSTR,
    pub uResIdString: u32,
}
impl Default for DIACTIONA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIACTIONFORMATA {
    pub dwSize: u32,
    pub dwActionSize: u32,
    pub dwDataSize: u32,
    pub dwNumActions: u32,
    pub rgoAction: *mut DIACTIONA,
    pub guidActionMap: windows_core::GUID,
    pub dwGenre: u32,
    pub dwBufferSize: u32,
    pub lAxisMin: i32,
    pub lAxisMax: i32,
    pub hInstString: super::super::Foundation::HINSTANCE,
    pub ftTimeStamp: super::super::Foundation::FILETIME,
    pub dwCRC: u32,
    pub tszActionMap: [i8; 260],
}
impl Default for DIACTIONFORMATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIACTIONFORMATW {
    pub dwSize: u32,
    pub dwActionSize: u32,
    pub dwDataSize: u32,
    pub dwNumActions: u32,
    pub rgoAction: *mut DIACTIONW,
    pub guidActionMap: windows_core::GUID,
    pub dwGenre: u32,
    pub dwBufferSize: u32,
    pub lAxisMin: i32,
    pub lAxisMax: i32,
    pub hInstString: super::super::Foundation::HINSTANCE,
    pub ftTimeStamp: super::super::Foundation::FILETIME,
    pub dwCRC: u32,
    pub tszActionMap: [u16; 260],
}
impl Default for DIACTIONFORMATW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DIACTIONW {
    pub uAppData: usize,
    pub dwSemantic: u32,
    pub dwFlags: u32,
    pub Anonymous: DIACTIONW_0,
    pub guidInstance: windows_core::GUID,
    pub dwObjID: u32,
    pub dwHow: u32,
}
impl Default for DIACTIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DIACTIONW_0 {
    pub lptszActionName: windows_core::PCWSTR,
    pub uResIdString: u32,
}
impl Default for DIACTIONW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIAFTS_NEWDEVICEHIGH: u32 = 4294967295u32;
pub const DIAFTS_NEWDEVICELOW: u32 = 4294967295u32;
pub const DIAFTS_UNUSEDDEVICEHIGH: u32 = 0u32;
pub const DIAFTS_UNUSEDDEVICELOW: u32 = 0u32;
pub const DIAH_APPREQUESTED: u32 = 2u32;
pub const DIAH_DEFAULT: u32 = 32u32;
pub const DIAH_ERROR: u32 = 2147483648u32;
pub const DIAH_HWAPP: u32 = 4u32;
pub const DIAH_HWDEFAULT: u32 = 8u32;
pub const DIAH_UNMAPPED: u32 = 0u32;
pub const DIAH_USERCONFIG: u32 = 1u32;
pub const DIAPPIDFLAG_NOSIZE: u32 = 2u32;
pub const DIAPPIDFLAG_NOTIME: u32 = 1u32;
pub const DIAXIS_2DCONTROL_INOUT: u32 = 587301379u32;
pub const DIAXIS_2DCONTROL_LATERAL: u32 = 587235841u32;
pub const DIAXIS_2DCONTROL_MOVE: u32 = 587268610u32;
pub const DIAXIS_2DCONTROL_ROTATEZ: u32 = 587350532u32;
pub const DIAXIS_3DCONTROL_INOUT: u32 = 604078595u32;
pub const DIAXIS_3DCONTROL_LATERAL: u32 = 604013057u32;
pub const DIAXIS_3DCONTROL_MOVE: u32 = 604045826u32;
pub const DIAXIS_3DCONTROL_ROTATEX: u32 = 604193284u32;
pub const DIAXIS_3DCONTROL_ROTATEY: u32 = 604160517u32;
pub const DIAXIS_3DCONTROL_ROTATEZ: u32 = 604127750u32;
pub const DIAXIS_ANY_1: u32 = 4278206977u32;
pub const DIAXIS_ANY_2: u32 = 4278206978u32;
pub const DIAXIS_ANY_3: u32 = 4278206979u32;
pub const DIAXIS_ANY_4: u32 = 4278206980u32;
pub const DIAXIS_ANY_A_1: u32 = 4278436353u32;
pub const DIAXIS_ANY_A_2: u32 = 4278436354u32;
pub const DIAXIS_ANY_B_1: u32 = 4278469121u32;
pub const DIAXIS_ANY_B_2: u32 = 4278469122u32;
pub const DIAXIS_ANY_C_1: u32 = 4278501889u32;
pub const DIAXIS_ANY_C_2: u32 = 4278501890u32;
pub const DIAXIS_ANY_R_1: u32 = 4278338049u32;
pub const DIAXIS_ANY_R_2: u32 = 4278338050u32;
pub const DIAXIS_ANY_S_1: u32 = 4278534657u32;
pub const DIAXIS_ANY_S_2: u32 = 4278534658u32;
pub const DIAXIS_ANY_U_1: u32 = 4278370817u32;
pub const DIAXIS_ANY_U_2: u32 = 4278370818u32;
pub const DIAXIS_ANY_V_1: u32 = 4278403585u32;
pub const DIAXIS_ANY_V_2: u32 = 4278403586u32;
pub const DIAXIS_ANY_X_1: u32 = 4278239745u32;
pub const DIAXIS_ANY_X_2: u32 = 4278239746u32;
pub const DIAXIS_ANY_Y_1: u32 = 4278272513u32;
pub const DIAXIS_ANY_Y_2: u32 = 4278272514u32;
pub const DIAXIS_ANY_Z_1: u32 = 4278305281u32;
pub const DIAXIS_ANY_Z_2: u32 = 4278305282u32;
pub const DIAXIS_ARCADEP_LATERAL: u32 = 570458625u32;
pub const DIAXIS_ARCADEP_MOVE: u32 = 570491394u32;
pub const DIAXIS_ARCADES_LATERAL: u32 = 553681409u32;
pub const DIAXIS_ARCADES_MOVE: u32 = 553714178u32;
pub const DIAXIS_BASEBALLB_LATERAL: u32 = 251691521u32;
pub const DIAXIS_BASEBALLB_MOVE: u32 = 251724290u32;
pub const DIAXIS_BASEBALLF_LATERAL: u32 = 285245953u32;
pub const DIAXIS_BASEBALLF_MOVE: u32 = 285278722u32;
pub const DIAXIS_BASEBALLP_LATERAL: u32 = 268468737u32;
pub const DIAXIS_BASEBALLP_MOVE: u32 = 268501506u32;
pub const DIAXIS_BBALLD_LATERAL: u32 = 318800385u32;
pub const DIAXIS_BBALLD_MOVE: u32 = 318833154u32;
pub const DIAXIS_BBALLO_LATERAL: u32 = 302023169u32;
pub const DIAXIS_BBALLO_MOVE: u32 = 302055938u32;
pub const DIAXIS_BIKINGM_BRAKE: u32 = 470041091u32;
pub const DIAXIS_BIKINGM_PEDAL: u32 = 469828098u32;
pub const DIAXIS_BIKINGM_TURN: u32 = 469795329u32;
pub const DIAXIS_BROWSER_LATERAL: u32 = 671121921u32;
pub const DIAXIS_BROWSER_MOVE: u32 = 671154690u32;
pub const DIAXIS_BROWSER_VIEW: u32 = 671187459u32;
pub const DIAXIS_CADF_INOUT: u32 = 620855811u32;
pub const DIAXIS_CADF_LATERAL: u32 = 620790273u32;
pub const DIAXIS_CADF_MOVE: u32 = 620823042u32;
pub const DIAXIS_CADF_ROTATEX: u32 = 620970500u32;
pub const DIAXIS_CADF_ROTATEY: u32 = 620937733u32;
pub const DIAXIS_CADF_ROTATEZ: u32 = 620904966u32;
pub const DIAXIS_CADM_INOUT: u32 = 637633027u32;
pub const DIAXIS_CADM_LATERAL: u32 = 637567489u32;
pub const DIAXIS_CADM_MOVE: u32 = 637600258u32;
pub const DIAXIS_CADM_ROTATEX: u32 = 637747716u32;
pub const DIAXIS_CADM_ROTATEY: u32 = 637714949u32;
pub const DIAXIS_CADM_ROTATEZ: u32 = 637682182u32;
pub const DIAXIS_DRIVINGC_ACCELERATE: u32 = 33788418u32;
pub const DIAXIS_DRIVINGC_ACCEL_AND_BRAKE: u32 = 33638916u32;
pub const DIAXIS_DRIVINGC_BRAKE: u32 = 33821187u32;
pub const DIAXIS_DRIVINGC_STEER: u32 = 33589761u32;
pub const DIAXIS_DRIVINGR_ACCELERATE: u32 = 17011202u32;
pub const DIAXIS_DRIVINGR_ACCEL_AND_BRAKE: u32 = 16861700u32;
pub const DIAXIS_DRIVINGR_BRAKE: u32 = 17043971u32;
pub const DIAXIS_DRIVINGR_STEER: u32 = 16812545u32;
pub const DIAXIS_DRIVINGT_ACCELERATE: u32 = 50565635u32;
pub const DIAXIS_DRIVINGT_ACCEL_AND_BRAKE: u32 = 50416134u32;
pub const DIAXIS_DRIVINGT_BARREL: u32 = 50397698u32;
pub const DIAXIS_DRIVINGT_BRAKE: u32 = 50614789u32;
pub const DIAXIS_DRIVINGT_ROTATE: u32 = 50463236u32;
pub const DIAXIS_DRIVINGT_STEER: u32 = 50366977u32;
pub const DIAXIS_FIGHTINGH_LATERAL: u32 = 134251009u32;
pub const DIAXIS_FIGHTINGH_MOVE: u32 = 134283778u32;
pub const DIAXIS_FIGHTINGH_ROTATE: u32 = 134365699u32;
pub const DIAXIS_FISHING_LATERAL: u32 = 234914305u32;
pub const DIAXIS_FISHING_MOVE: u32 = 234947074u32;
pub const DIAXIS_FISHING_ROTATE: u32 = 235028995u32;
pub const DIAXIS_FLYINGC_BANK: u32 = 67144193u32;
pub const DIAXIS_FLYINGC_BRAKE: u32 = 67398148u32;
pub const DIAXIS_FLYINGC_FLAPS: u32 = 67459590u32;
pub const DIAXIS_FLYINGC_PITCH: u32 = 67176962u32;
pub const DIAXIS_FLYINGC_RUDDER: u32 = 67260933u32;
pub const DIAXIS_FLYINGC_THROTTLE: u32 = 67342851u32;
pub const DIAXIS_FLYINGH_BANK: u32 = 100698625u32;
pub const DIAXIS_FLYINGH_COLLECTIVE: u32 = 100764163u32;
pub const DIAXIS_FLYINGH_PITCH: u32 = 100731394u32;
pub const DIAXIS_FLYINGH_THROTTLE: u32 = 100915717u32;
pub const DIAXIS_FLYINGH_TORQUE: u32 = 100817412u32;
pub const DIAXIS_FLYINGM_BANK: u32 = 83921409u32;
pub const DIAXIS_FLYINGM_BRAKE: u32 = 84173317u32;
pub const DIAXIS_FLYINGM_FLAPS: u32 = 84234758u32;
pub const DIAXIS_FLYINGM_PITCH: u32 = 83954178u32;
pub const DIAXIS_FLYINGM_RUDDER: u32 = 84036100u32;
pub const DIAXIS_FLYINGM_THROTTLE: u32 = 84120067u32;
pub const DIAXIS_FOOTBALLD_LATERAL: u32 = 385909249u32;
pub const DIAXIS_FOOTBALLD_MOVE: u32 = 385942018u32;
pub const DIAXIS_FOOTBALLO_LATERAL: u32 = 369132033u32;
pub const DIAXIS_FOOTBALLO_MOVE: u32 = 369164802u32;
pub const DIAXIS_FOOTBALLQ_LATERAL: u32 = 352354817u32;
pub const DIAXIS_FOOTBALLQ_MOVE: u32 = 352387586u32;
pub const DIAXIS_FPS_LOOKUPDOWN: u32 = 151093763u32;
pub const DIAXIS_FPS_MOVE: u32 = 151060994u32;
pub const DIAXIS_FPS_ROTATE: u32 = 151028225u32;
pub const DIAXIS_FPS_SIDESTEP: u32 = 151142916u32;
pub const DIAXIS_GOLF_LATERAL: u32 = 402686465u32;
pub const DIAXIS_GOLF_MOVE: u32 = 402719234u32;
pub const DIAXIS_HOCKEYD_LATERAL: u32 = 436240897u32;
pub const DIAXIS_HOCKEYD_MOVE: u32 = 436273666u32;
pub const DIAXIS_HOCKEYG_LATERAL: u32 = 453018113u32;
pub const DIAXIS_HOCKEYG_MOVE: u32 = 453050882u32;
pub const DIAXIS_HOCKEYO_LATERAL: u32 = 419463681u32;
pub const DIAXIS_HOCKEYO_MOVE: u32 = 419496450u32;
pub const DIAXIS_HUNTING_LATERAL: u32 = 218137089u32;
pub const DIAXIS_HUNTING_MOVE: u32 = 218169858u32;
pub const DIAXIS_HUNTING_ROTATE: u32 = 218251779u32;
pub const DIAXIS_MECHA_ROTATE: u32 = 687997443u32;
pub const DIAXIS_MECHA_STEER: u32 = 687899137u32;
pub const DIAXIS_MECHA_THROTTLE: u32 = 688095748u32;
pub const DIAXIS_MECHA_TORSO: u32 = 687931906u32;
pub const DIAXIS_RACQUET_LATERAL: u32 = 536904193u32;
pub const DIAXIS_RACQUET_MOVE: u32 = 536936962u32;
pub const DIAXIS_REMOTE_SLIDER: u32 = 654639617u32;
pub const DIAXIS_REMOTE_SLIDER2: u32 = 654656002u32;
pub const DIAXIS_SKIING_SPEED: u32 = 486605314u32;
pub const DIAXIS_SKIING_TURN: u32 = 486572545u32;
pub const DIAXIS_SOCCERD_LATERAL: u32 = 520126977u32;
pub const DIAXIS_SOCCERD_MOVE: u32 = 520159746u32;
pub const DIAXIS_SOCCERO_BEND: u32 = 503415299u32;
pub const DIAXIS_SOCCERO_LATERAL: u32 = 503349761u32;
pub const DIAXIS_SOCCERO_MOVE: u32 = 503382530u32;
pub const DIAXIS_SPACESIM_CLIMB: u32 = 117555716u32;
pub const DIAXIS_SPACESIM_LATERAL: u32 = 117473793u32;
pub const DIAXIS_SPACESIM_MOVE: u32 = 117506562u32;
pub const DIAXIS_SPACESIM_ROTATE: u32 = 117588485u32;
pub const DIAXIS_SPACESIM_THROTTLE: u32 = 117670403u32;
pub const DIAXIS_STRATEGYR_LATERAL: u32 = 184582657u32;
pub const DIAXIS_STRATEGYR_MOVE: u32 = 184615426u32;
pub const DIAXIS_STRATEGYR_ROTATE: u32 = 184697347u32;
pub const DIAXIS_STRATEGYT_LATERAL: u32 = 201359873u32;
pub const DIAXIS_STRATEGYT_MOVE: u32 = 201392642u32;
pub const DIAXIS_TPS_MOVE: u32 = 167838210u32;
pub const DIAXIS_TPS_STEP: u32 = 167821827u32;
pub const DIAXIS_TPS_TURN: u32 = 167903745u32;
pub const DIA_APPFIXED: u32 = 16u32;
pub const DIA_APPMAPPED: u32 = 2u32;
pub const DIA_APPNOMAP: u32 = 4u32;
pub const DIA_FORCEFEEDBACK: u32 = 1u32;
pub const DIA_NORANGE: u32 = 8u32;
pub const DIBUTTON_2DCONTROL_DEVICE: u32 = 587220222u32;
pub const DIBUTTON_2DCONTROL_DISPLAY: u32 = 587219973u32;
pub const DIBUTTON_2DCONTROL_MENU: u32 = 587203837u32;
pub const DIBUTTON_2DCONTROL_PAUSE: u32 = 587220220u32;
pub const DIBUTTON_2DCONTROL_SELECT: u32 = 587203585u32;
pub const DIBUTTON_2DCONTROL_SPECIAL: u32 = 587203587u32;
pub const DIBUTTON_2DCONTROL_SPECIAL1: u32 = 587203586u32;
pub const DIBUTTON_2DCONTROL_SPECIAL2: u32 = 587203588u32;
pub const DIBUTTON_3DCONTROL_DEVICE: u32 = 603997438u32;
pub const DIBUTTON_3DCONTROL_DISPLAY: u32 = 603997189u32;
pub const DIBUTTON_3DCONTROL_MENU: u32 = 603981053u32;
pub const DIBUTTON_3DCONTROL_PAUSE: u32 = 603997436u32;
pub const DIBUTTON_3DCONTROL_SELECT: u32 = 603980801u32;
pub const DIBUTTON_3DCONTROL_SPECIAL: u32 = 603980803u32;
pub const DIBUTTON_3DCONTROL_SPECIAL1: u32 = 603980802u32;
pub const DIBUTTON_3DCONTROL_SPECIAL2: u32 = 603980804u32;
pub const DIBUTTON_ARCADEP_BACK_LINK: u32 = 570508520u32;
pub const DIBUTTON_ARCADEP_CROUCH: u32 = 570426371u32;
pub const DIBUTTON_ARCADEP_DEVICE: u32 = 570443006u32;
pub const DIBUTTON_ARCADEP_FIRE: u32 = 570426370u32;
pub const DIBUTTON_ARCADEP_FIRESECONDARY: u32 = 570442758u32;
pub const DIBUTTON_ARCADEP_FORWARD_LINK: u32 = 570508512u32;
pub const DIBUTTON_ARCADEP_JUMP: u32 = 570426369u32;
pub const DIBUTTON_ARCADEP_LEFT_LINK: u32 = 570475748u32;
pub const DIBUTTON_ARCADEP_MENU: u32 = 570426621u32;
pub const DIBUTTON_ARCADEP_PAUSE: u32 = 570443004u32;
pub const DIBUTTON_ARCADEP_RIGHT_LINK: u32 = 570475756u32;
pub const DIBUTTON_ARCADEP_SELECT: u32 = 570426373u32;
pub const DIBUTTON_ARCADEP_SPECIAL: u32 = 570426372u32;
pub const DIBUTTON_ARCADEP_VIEW_DOWN_LINK: u32 = 570934504u32;
pub const DIBUTTON_ARCADEP_VIEW_LEFT_LINK: u32 = 570934500u32;
pub const DIBUTTON_ARCADEP_VIEW_RIGHT_LINK: u32 = 570934508u32;
pub const DIBUTTON_ARCADEP_VIEW_UP_LINK: u32 = 570934496u32;
pub const DIBUTTON_ARCADES_ATTACK: u32 = 553649155u32;
pub const DIBUTTON_ARCADES_BACK_LINK: u32 = 553731304u32;
pub const DIBUTTON_ARCADES_CARRY: u32 = 553649154u32;
pub const DIBUTTON_ARCADES_DEVICE: u32 = 553665790u32;
pub const DIBUTTON_ARCADES_FORWARD_LINK: u32 = 553731296u32;
pub const DIBUTTON_ARCADES_LEFT_LINK: u32 = 553698532u32;
pub const DIBUTTON_ARCADES_MENU: u32 = 553649405u32;
pub const DIBUTTON_ARCADES_PAUSE: u32 = 553665788u32;
pub const DIBUTTON_ARCADES_RIGHT_LINK: u32 = 553698540u32;
pub const DIBUTTON_ARCADES_SELECT: u32 = 553649157u32;
pub const DIBUTTON_ARCADES_SPECIAL: u32 = 553649156u32;
pub const DIBUTTON_ARCADES_THROW: u32 = 553649153u32;
pub const DIBUTTON_ARCADES_VIEW_DOWN_LINK: u32 = 554157288u32;
pub const DIBUTTON_ARCADES_VIEW_LEFT_LINK: u32 = 554157284u32;
pub const DIBUTTON_ARCADES_VIEW_RIGHT_LINK: u32 = 554157292u32;
pub const DIBUTTON_ARCADES_VIEW_UP_LINK: u32 = 554157280u32;
pub const DIBUTTON_BASEBALLB_BACK_LINK: u32 = 251741416u32;
pub const DIBUTTON_BASEBALLB_BOX: u32 = 251675658u32;
pub const DIBUTTON_BASEBALLB_BUNT: u32 = 251659268u32;
pub const DIBUTTON_BASEBALLB_BURST: u32 = 251659270u32;
pub const DIBUTTON_BASEBALLB_CONTACT: u32 = 251659272u32;
pub const DIBUTTON_BASEBALLB_DEVICE: u32 = 251675902u32;
pub const DIBUTTON_BASEBALLB_FORWARD_LINK: u32 = 251741408u32;
pub const DIBUTTON_BASEBALLB_LEFT_LINK: u32 = 251708644u32;
pub const DIBUTTON_BASEBALLB_MENU: u32 = 251659517u32;
pub const DIBUTTON_BASEBALLB_NORMAL: u32 = 251659266u32;
pub const DIBUTTON_BASEBALLB_NOSTEAL: u32 = 251675657u32;
pub const DIBUTTON_BASEBALLB_PAUSE: u32 = 251675900u32;
pub const DIBUTTON_BASEBALLB_POWER: u32 = 251659267u32;
pub const DIBUTTON_BASEBALLB_RIGHT_LINK: u32 = 251708652u32;
pub const DIBUTTON_BASEBALLB_SELECT: u32 = 251659265u32;
pub const DIBUTTON_BASEBALLB_SLIDE: u32 = 251659271u32;
pub const DIBUTTON_BASEBALLB_STEAL: u32 = 251659269u32;
pub const DIBUTTON_BASEBALLF_AIM_LEFT_LINK: u32 = 285263076u32;
pub const DIBUTTON_BASEBALLF_AIM_RIGHT_LINK: u32 = 285263084u32;
pub const DIBUTTON_BASEBALLF_BACK_LINK: u32 = 285295848u32;
pub const DIBUTTON_BASEBALLF_BURST: u32 = 285213700u32;
pub const DIBUTTON_BASEBALLF_DEVICE: u32 = 285230334u32;
pub const DIBUTTON_BASEBALLF_DIVE: u32 = 285213702u32;
pub const DIBUTTON_BASEBALLF_FORWARD_LINK: u32 = 285295840u32;
pub const DIBUTTON_BASEBALLF_JUMP: u32 = 285213701u32;
pub const DIBUTTON_BASEBALLF_MENU: u32 = 285213949u32;
pub const DIBUTTON_BASEBALLF_NEAREST: u32 = 285213697u32;
pub const DIBUTTON_BASEBALLF_PAUSE: u32 = 285230332u32;
pub const DIBUTTON_BASEBALLF_SHIFTIN: u32 = 285230087u32;
pub const DIBUTTON_BASEBALLF_SHIFTOUT: u32 = 285230088u32;
pub const DIBUTTON_BASEBALLF_THROW1: u32 = 285213698u32;
pub const DIBUTTON_BASEBALLF_THROW2: u32 = 285213699u32;
pub const DIBUTTON_BASEBALLP_BACK_LINK: u32 = 268518632u32;
pub const DIBUTTON_BASEBALLP_BASE: u32 = 268436483u32;
pub const DIBUTTON_BASEBALLP_DEVICE: u32 = 268453118u32;
pub const DIBUTTON_BASEBALLP_FAKE: u32 = 268436485u32;
pub const DIBUTTON_BASEBALLP_FORWARD_LINK: u32 = 268518624u32;
pub const DIBUTTON_BASEBALLP_LEFT_LINK: u32 = 268485860u32;
pub const DIBUTTON_BASEBALLP_LOOK: u32 = 268452871u32;
pub const DIBUTTON_BASEBALLP_MENU: u32 = 268436733u32;
pub const DIBUTTON_BASEBALLP_PAUSE: u32 = 268453116u32;
pub const DIBUTTON_BASEBALLP_PITCH: u32 = 268436482u32;
pub const DIBUTTON_BASEBALLP_RIGHT_LINK: u32 = 268485868u32;
pub const DIBUTTON_BASEBALLP_SELECT: u32 = 268436481u32;
pub const DIBUTTON_BASEBALLP_THROW: u32 = 268436484u32;
pub const DIBUTTON_BASEBALLP_WALK: u32 = 268452870u32;
pub const DIBUTTON_BBALLD_BACK_LINK: u32 = 318850280u32;
pub const DIBUTTON_BBALLD_BURST: u32 = 318768134u32;
pub const DIBUTTON_BBALLD_DEVICE: u32 = 318784766u32;
pub const DIBUTTON_BBALLD_FAKE: u32 = 318768131u32;
pub const DIBUTTON_BBALLD_FORWARD_LINK: u32 = 318850272u32;
pub const DIBUTTON_BBALLD_JUMP: u32 = 318768129u32;
pub const DIBUTTON_BBALLD_LEFT_LINK: u32 = 318817508u32;
pub const DIBUTTON_BBALLD_MENU: u32 = 318768381u32;
pub const DIBUTTON_BBALLD_PAUSE: u32 = 318784764u32;
pub const DIBUTTON_BBALLD_PLAY: u32 = 318768135u32;
pub const DIBUTTON_BBALLD_PLAYER: u32 = 318768133u32;
pub const DIBUTTON_BBALLD_RIGHT_LINK: u32 = 318817516u32;
pub const DIBUTTON_BBALLD_SPECIAL: u32 = 318768132u32;
pub const DIBUTTON_BBALLD_STEAL: u32 = 318768130u32;
pub const DIBUTTON_BBALLD_SUBSTITUTE: u32 = 318784521u32;
pub const DIBUTTON_BBALLD_TIMEOUT: u32 = 318784520u32;
pub const DIBUTTON_BBALLO_BACK_LINK: u32 = 302073064u32;
pub const DIBUTTON_BBALLO_BURST: u32 = 301990919u32;
pub const DIBUTTON_BBALLO_CALL: u32 = 301990920u32;
pub const DIBUTTON_BBALLO_DEVICE: u32 = 302007550u32;
pub const DIBUTTON_BBALLO_DUNK: u32 = 301990914u32;
pub const DIBUTTON_BBALLO_FAKE: u32 = 301990916u32;
pub const DIBUTTON_BBALLO_FORWARD_LINK: u32 = 302073056u32;
pub const DIBUTTON_BBALLO_JAB: u32 = 302007307u32;
pub const DIBUTTON_BBALLO_LEFT_LINK: u32 = 302040292u32;
pub const DIBUTTON_BBALLO_MENU: u32 = 301991165u32;
pub const DIBUTTON_BBALLO_PASS: u32 = 301990915u32;
pub const DIBUTTON_BBALLO_PAUSE: u32 = 302007548u32;
pub const DIBUTTON_BBALLO_PLAY: u32 = 302007306u32;
pub const DIBUTTON_BBALLO_PLAYER: u32 = 301990918u32;
pub const DIBUTTON_BBALLO_POST: u32 = 302007308u32;
pub const DIBUTTON_BBALLO_RIGHT_LINK: u32 = 302040300u32;
pub const DIBUTTON_BBALLO_SCREEN: u32 = 302007305u32;
pub const DIBUTTON_BBALLO_SHOOT: u32 = 301990913u32;
pub const DIBUTTON_BBALLO_SPECIAL: u32 = 301990917u32;
pub const DIBUTTON_BBALLO_SUBSTITUTE: u32 = 302007310u32;
pub const DIBUTTON_BBALLO_TIMEOUT: u32 = 302007309u32;
pub const DIBUTTON_BIKINGM_BRAKE_BUTTON_LINK: u32 = 470041832u32;
pub const DIBUTTON_BIKINGM_CAMERA: u32 = 469763074u32;
pub const DIBUTTON_BIKINGM_DEVICE: u32 = 469779710u32;
pub const DIBUTTON_BIKINGM_FASTER_LINK: u32 = 469845216u32;
pub const DIBUTTON_BIKINGM_JUMP: u32 = 469763073u32;
pub const DIBUTTON_BIKINGM_LEFT_LINK: u32 = 469812452u32;
pub const DIBUTTON_BIKINGM_MENU: u32 = 469763325u32;
pub const DIBUTTON_BIKINGM_PAUSE: u32 = 469779708u32;
pub const DIBUTTON_BIKINGM_RIGHT_LINK: u32 = 469812460u32;
pub const DIBUTTON_BIKINGM_SELECT: u32 = 469763076u32;
pub const DIBUTTON_BIKINGM_SLOWER_LINK: u32 = 469845224u32;
pub const DIBUTTON_BIKINGM_SPECIAL1: u32 = 469763075u32;
pub const DIBUTTON_BIKINGM_SPECIAL2: u32 = 469763077u32;
pub const DIBUTTON_BIKINGM_ZOOM: u32 = 469779462u32;
pub const DIBUTTON_BROWSER_DEVICE: u32 = 671106302u32;
pub const DIBUTTON_BROWSER_FAVORITES: u32 = 671106054u32;
pub const DIBUTTON_BROWSER_HISTORY: u32 = 671106057u32;
pub const DIBUTTON_BROWSER_HOME: u32 = 671106053u32;
pub const DIBUTTON_BROWSER_MENU: u32 = 671089917u32;
pub const DIBUTTON_BROWSER_NEXT: u32 = 671106055u32;
pub const DIBUTTON_BROWSER_PAUSE: u32 = 671106300u32;
pub const DIBUTTON_BROWSER_PREVIOUS: u32 = 671106056u32;
pub const DIBUTTON_BROWSER_PRINT: u32 = 671106058u32;
pub const DIBUTTON_BROWSER_REFRESH: u32 = 671089666u32;
pub const DIBUTTON_BROWSER_SEARCH: u32 = 671106051u32;
pub const DIBUTTON_BROWSER_SELECT: u32 = 671089665u32;
pub const DIBUTTON_BROWSER_STOP: u32 = 671106052u32;
pub const DIBUTTON_CADF_DEVICE: u32 = 620774654u32;
pub const DIBUTTON_CADF_DISPLAY: u32 = 620774405u32;
pub const DIBUTTON_CADF_MENU: u32 = 620758269u32;
pub const DIBUTTON_CADF_PAUSE: u32 = 620774652u32;
pub const DIBUTTON_CADF_SELECT: u32 = 620758017u32;
pub const DIBUTTON_CADF_SPECIAL: u32 = 620758019u32;
pub const DIBUTTON_CADF_SPECIAL1: u32 = 620758018u32;
pub const DIBUTTON_CADF_SPECIAL2: u32 = 620758020u32;
pub const DIBUTTON_CADM_DEVICE: u32 = 637551870u32;
pub const DIBUTTON_CADM_DISPLAY: u32 = 637551621u32;
pub const DIBUTTON_CADM_MENU: u32 = 637535485u32;
pub const DIBUTTON_CADM_PAUSE: u32 = 637551868u32;
pub const DIBUTTON_CADM_SELECT: u32 = 637535233u32;
pub const DIBUTTON_CADM_SPECIAL: u32 = 637535235u32;
pub const DIBUTTON_CADM_SPECIAL1: u32 = 637535234u32;
pub const DIBUTTON_CADM_SPECIAL2: u32 = 637535236u32;
pub const DIBUTTON_DRIVINGC_ACCELERATE_LINK: u32 = 33805536u32;
pub const DIBUTTON_DRIVINGC_AIDS: u32 = 33571847u32;
pub const DIBUTTON_DRIVINGC_BRAKE: u32 = 33573896u32;
pub const DIBUTTON_DRIVINGC_DASHBOARD: u32 = 33571846u32;
pub const DIBUTTON_DRIVINGC_DEVICE: u32 = 33572094u32;
pub const DIBUTTON_DRIVINGC_FIRE: u32 = 33557505u32;
pub const DIBUTTON_DRIVINGC_FIRESECONDARY: u32 = 33573897u32;
pub const DIBUTTON_DRIVINGC_GLANCE_LEFT_LINK: u32 = 34063588u32;
pub const DIBUTTON_DRIVINGC_GLANCE_RIGHT_LINK: u32 = 34063596u32;
pub const DIBUTTON_DRIVINGC_MENU: u32 = 33555709u32;
pub const DIBUTTON_DRIVINGC_PAUSE: u32 = 33572092u32;
pub const DIBUTTON_DRIVINGC_SHIFTDOWN: u32 = 33573893u32;
pub const DIBUTTON_DRIVINGC_SHIFTUP: u32 = 33573892u32;
pub const DIBUTTON_DRIVINGC_STEER_LEFT_LINK: u32 = 33606884u32;
pub const DIBUTTON_DRIVINGC_STEER_RIGHT_LINK: u32 = 33606892u32;
pub const DIBUTTON_DRIVINGC_TARGET: u32 = 33557507u32;
pub const DIBUTTON_DRIVINGC_WEAPONS: u32 = 33557506u32;
pub const DIBUTTON_DRIVINGR_ACCELERATE_LINK: u32 = 17028320u32;
pub const DIBUTTON_DRIVINGR_AIDS: u32 = 16794630u32;
pub const DIBUTTON_DRIVINGR_BOOST: u32 = 16794632u32;
pub const DIBUTTON_DRIVINGR_BRAKE: u32 = 16796676u32;
pub const DIBUTTON_DRIVINGR_DASHBOARD: u32 = 16794629u32;
pub const DIBUTTON_DRIVINGR_DEVICE: u32 = 16794878u32;
pub const DIBUTTON_DRIVINGR_GLANCE_LEFT_LINK: u32 = 17286372u32;
pub const DIBUTTON_DRIVINGR_GLANCE_RIGHT_LINK: u32 = 17286380u32;
pub const DIBUTTON_DRIVINGR_MAP: u32 = 16794631u32;
pub const DIBUTTON_DRIVINGR_MENU: u32 = 16778493u32;
pub const DIBUTTON_DRIVINGR_PAUSE: u32 = 16794876u32;
pub const DIBUTTON_DRIVINGR_PIT: u32 = 16794633u32;
pub const DIBUTTON_DRIVINGR_SHIFTDOWN: u32 = 16780290u32;
pub const DIBUTTON_DRIVINGR_SHIFTUP: u32 = 16780289u32;
pub const DIBUTTON_DRIVINGR_STEER_LEFT_LINK: u32 = 16829668u32;
pub const DIBUTTON_DRIVINGR_STEER_RIGHT_LINK: u32 = 16829676u32;
pub const DIBUTTON_DRIVINGR_VIEW: u32 = 16784387u32;
pub const DIBUTTON_DRIVINGT_ACCELERATE_LINK: u32 = 50582752u32;
pub const DIBUTTON_DRIVINGT_BARREL_DOWN_LINK: u32 = 50414824u32;
pub const DIBUTTON_DRIVINGT_BARREL_UP_LINK: u32 = 50414816u32;
pub const DIBUTTON_DRIVINGT_BRAKE: u32 = 50351110u32;
pub const DIBUTTON_DRIVINGT_DASHBOARD: u32 = 50355205u32;
pub const DIBUTTON_DRIVINGT_DEVICE: u32 = 50349310u32;
pub const DIBUTTON_DRIVINGT_FIRE: u32 = 50334721u32;
pub const DIBUTTON_DRIVINGT_FIRESECONDARY: u32 = 50351111u32;
pub const DIBUTTON_DRIVINGT_GLANCE_LEFT_LINK: u32 = 50840804u32;
pub const DIBUTTON_DRIVINGT_GLANCE_RIGHT_LINK: u32 = 50840812u32;
pub const DIBUTTON_DRIVINGT_MENU: u32 = 50332925u32;
pub const DIBUTTON_DRIVINGT_PAUSE: u32 = 50349308u32;
pub const DIBUTTON_DRIVINGT_ROTATE_LEFT_LINK: u32 = 50480356u32;
pub const DIBUTTON_DRIVINGT_ROTATE_RIGHT_LINK: u32 = 50480364u32;
pub const DIBUTTON_DRIVINGT_STEER_LEFT_LINK: u32 = 50384100u32;
pub const DIBUTTON_DRIVINGT_STEER_RIGHT_LINK: u32 = 50384108u32;
pub const DIBUTTON_DRIVINGT_TARGET: u32 = 50334723u32;
pub const DIBUTTON_DRIVINGT_VIEW: u32 = 50355204u32;
pub const DIBUTTON_DRIVINGT_WEAPONS: u32 = 50334722u32;
pub const DIBUTTON_FIGHTINGH_BACKWARD_LINK: u32 = 134300904u32;
pub const DIBUTTON_FIGHTINGH_BLOCK: u32 = 134218755u32;
pub const DIBUTTON_FIGHTINGH_CROUCH: u32 = 134218756u32;
pub const DIBUTTON_FIGHTINGH_DEVICE: u32 = 134235390u32;
pub const DIBUTTON_FIGHTINGH_DISPLAY: u32 = 134235145u32;
pub const DIBUTTON_FIGHTINGH_DODGE: u32 = 134235146u32;
pub const DIBUTTON_FIGHTINGH_FORWARD_LINK: u32 = 134300896u32;
pub const DIBUTTON_FIGHTINGH_JUMP: u32 = 134218757u32;
pub const DIBUTTON_FIGHTINGH_KICK: u32 = 134218754u32;
pub const DIBUTTON_FIGHTINGH_LEFT_LINK: u32 = 134268132u32;
pub const DIBUTTON_FIGHTINGH_MENU: u32 = 134219005u32;
pub const DIBUTTON_FIGHTINGH_PAUSE: u32 = 134235388u32;
pub const DIBUTTON_FIGHTINGH_PUNCH: u32 = 134218753u32;
pub const DIBUTTON_FIGHTINGH_RIGHT_LINK: u32 = 134268140u32;
pub const DIBUTTON_FIGHTINGH_SELECT: u32 = 134235144u32;
pub const DIBUTTON_FIGHTINGH_SPECIAL1: u32 = 134218758u32;
pub const DIBUTTON_FIGHTINGH_SPECIAL2: u32 = 134218759u32;
pub const DIBUTTON_FISHING_BACK_LINK: u32 = 234964200u32;
pub const DIBUTTON_FISHING_BAIT: u32 = 234882052u32;
pub const DIBUTTON_FISHING_BINOCULAR: u32 = 234882051u32;
pub const DIBUTTON_FISHING_CAST: u32 = 234882049u32;
pub const DIBUTTON_FISHING_CROUCH: u32 = 234898439u32;
pub const DIBUTTON_FISHING_DEVICE: u32 = 234898686u32;
pub const DIBUTTON_FISHING_DISPLAY: u32 = 234898438u32;
pub const DIBUTTON_FISHING_FORWARD_LINK: u32 = 234964192u32;
pub const DIBUTTON_FISHING_JUMP: u32 = 234898440u32;
pub const DIBUTTON_FISHING_LEFT_LINK: u32 = 234931428u32;
pub const DIBUTTON_FISHING_MAP: u32 = 234882053u32;
pub const DIBUTTON_FISHING_MENU: u32 = 234882301u32;
pub const DIBUTTON_FISHING_PAUSE: u32 = 234898684u32;
pub const DIBUTTON_FISHING_RIGHT_LINK: u32 = 234931436u32;
pub const DIBUTTON_FISHING_ROTATE_LEFT_LINK: u32 = 235029732u32;
pub const DIBUTTON_FISHING_ROTATE_RIGHT_LINK: u32 = 235029740u32;
pub const DIBUTTON_FISHING_TYPE: u32 = 234882050u32;
pub const DIBUTTON_FLYINGC_BRAKE_LINK: u32 = 67398880u32;
pub const DIBUTTON_FLYINGC_DEVICE: u32 = 67126526u32;
pub const DIBUTTON_FLYINGC_DISPLAY: u32 = 67118082u32;
pub const DIBUTTON_FLYINGC_FASTER_LINK: u32 = 67359968u32;
pub const DIBUTTON_FLYINGC_FLAPSDOWN: u32 = 67134469u32;
pub const DIBUTTON_FLYINGC_FLAPSUP: u32 = 67134468u32;
pub const DIBUTTON_FLYINGC_GEAR: u32 = 67120131u32;
pub const DIBUTTON_FLYINGC_GLANCE_DOWN_LINK: u32 = 67618024u32;
pub const DIBUTTON_FLYINGC_GLANCE_LEFT_LINK: u32 = 67618020u32;
pub const DIBUTTON_FLYINGC_GLANCE_RIGHT_LINK: u32 = 67618028u32;
pub const DIBUTTON_FLYINGC_GLANCE_UP_LINK: u32 = 67618016u32;
pub const DIBUTTON_FLYINGC_MENU: u32 = 67110141u32;
pub const DIBUTTON_FLYINGC_PAUSE: u32 = 67126524u32;
pub const DIBUTTON_FLYINGC_SLOWER_LINK: u32 = 67359976u32;
pub const DIBUTTON_FLYINGC_VIEW: u32 = 67118081u32;
pub const DIBUTTON_FLYINGH_COUNTER: u32 = 100684804u32;
pub const DIBUTTON_FLYINGH_DEVICE: u32 = 100680958u32;
pub const DIBUTTON_FLYINGH_FASTER_LINK: u32 = 100916448u32;
pub const DIBUTTON_FLYINGH_FIRE: u32 = 100668417u32;
pub const DIBUTTON_FLYINGH_FIRESECONDARY: u32 = 100682759u32;
pub const DIBUTTON_FLYINGH_GEAR: u32 = 100688902u32;
pub const DIBUTTON_FLYINGH_GLANCE_DOWN_LINK: u32 = 101172456u32;
pub const DIBUTTON_FLYINGH_GLANCE_LEFT_LINK: u32 = 101172452u32;
pub const DIBUTTON_FLYINGH_GLANCE_RIGHT_LINK: u32 = 101172460u32;
pub const DIBUTTON_FLYINGH_GLANCE_UP_LINK: u32 = 101172448u32;
pub const DIBUTTON_FLYINGH_MENU: u32 = 100664573u32;
pub const DIBUTTON_FLYINGH_PAUSE: u32 = 100680956u32;
pub const DIBUTTON_FLYINGH_SLOWER_LINK: u32 = 100916456u32;
pub const DIBUTTON_FLYINGH_TARGET: u32 = 100668419u32;
pub const DIBUTTON_FLYINGH_VIEW: u32 = 100688901u32;
pub const DIBUTTON_FLYINGH_WEAPONS: u32 = 100668418u32;
pub const DIBUTTON_FLYINGM_BRAKE_LINK: u32 = 84174048u32;
pub const DIBUTTON_FLYINGM_COUNTER: u32 = 83909636u32;
pub const DIBUTTON_FLYINGM_DEVICE: u32 = 83903742u32;
pub const DIBUTTON_FLYINGM_DISPLAY: u32 = 83911686u32;
pub const DIBUTTON_FLYINGM_FASTER_LINK: u32 = 84137184u32;
pub const DIBUTTON_FLYINGM_FIRE: u32 = 83889153u32;
pub const DIBUTTON_FLYINGM_FIRESECONDARY: u32 = 83905545u32;
pub const DIBUTTON_FLYINGM_FLAPSDOWN: u32 = 83907592u32;
pub const DIBUTTON_FLYINGM_FLAPSUP: u32 = 83907591u32;
pub const DIBUTTON_FLYINGM_GEAR: u32 = 83911690u32;
pub const DIBUTTON_FLYINGM_GLANCE_DOWN_LINK: u32 = 84395240u32;
pub const DIBUTTON_FLYINGM_GLANCE_LEFT_LINK: u32 = 84395236u32;
pub const DIBUTTON_FLYINGM_GLANCE_RIGHT_LINK: u32 = 84395244u32;
pub const DIBUTTON_FLYINGM_GLANCE_UP_LINK: u32 = 84395232u32;
pub const DIBUTTON_FLYINGM_MENU: u32 = 83887357u32;
pub const DIBUTTON_FLYINGM_PAUSE: u32 = 83903740u32;
pub const DIBUTTON_FLYINGM_SLOWER_LINK: u32 = 84137192u32;
pub const DIBUTTON_FLYINGM_TARGET: u32 = 83889155u32;
pub const DIBUTTON_FLYINGM_VIEW: u32 = 83911685u32;
pub const DIBUTTON_FLYINGM_WEAPONS: u32 = 83889154u32;
pub const DIBUTTON_FOOTBALLD_AUDIBLE: u32 = 385893387u32;
pub const DIBUTTON_FOOTBALLD_BACK_LINK: u32 = 385959144u32;
pub const DIBUTTON_FOOTBALLD_BULLRUSH: u32 = 385893385u32;
pub const DIBUTTON_FOOTBALLD_DEVICE: u32 = 385893630u32;
pub const DIBUTTON_FOOTBALLD_FAKE: u32 = 385876997u32;
pub const DIBUTTON_FOOTBALLD_FORWARD_LINK: u32 = 385959136u32;
pub const DIBUTTON_FOOTBALLD_JUMP: u32 = 385876995u32;
pub const DIBUTTON_FOOTBALLD_LEFT_LINK: u32 = 385926372u32;
pub const DIBUTTON_FOOTBALLD_MENU: u32 = 385877245u32;
pub const DIBUTTON_FOOTBALLD_PAUSE: u32 = 385893628u32;
pub const DIBUTTON_FOOTBALLD_PLAY: u32 = 385876993u32;
pub const DIBUTTON_FOOTBALLD_RIGHT_LINK: u32 = 385926380u32;
pub const DIBUTTON_FOOTBALLD_RIP: u32 = 385893386u32;
pub const DIBUTTON_FOOTBALLD_SELECT: u32 = 385876994u32;
pub const DIBUTTON_FOOTBALLD_SPIN: u32 = 385893383u32;
pub const DIBUTTON_FOOTBALLD_SUBSTITUTE: u32 = 385893389u32;
pub const DIBUTTON_FOOTBALLD_SUPERTACKLE: u32 = 385876998u32;
pub const DIBUTTON_FOOTBALLD_SWIM: u32 = 385893384u32;
pub const DIBUTTON_FOOTBALLD_TACKLE: u32 = 385876996u32;
pub const DIBUTTON_FOOTBALLD_ZOOM: u32 = 385893388u32;
pub const DIBUTTON_FOOTBALLO_BACK_LINK: u32 = 369181928u32;
pub const DIBUTTON_FOOTBALLO_DEVICE: u32 = 369116414u32;
pub const DIBUTTON_FOOTBALLO_DIVE: u32 = 369116169u32;
pub const DIBUTTON_FOOTBALLO_FORWARD_LINK: u32 = 369181920u32;
pub const DIBUTTON_FOOTBALLO_JUKE: u32 = 369116166u32;
pub const DIBUTTON_FOOTBALLO_JUMP: u32 = 369099777u32;
pub const DIBUTTON_FOOTBALLO_LEFTARM: u32 = 369099778u32;
pub const DIBUTTON_FOOTBALLO_LEFT_LINK: u32 = 369149156u32;
pub const DIBUTTON_FOOTBALLO_MENU: u32 = 369100029u32;
pub const DIBUTTON_FOOTBALLO_PAUSE: u32 = 369116412u32;
pub const DIBUTTON_FOOTBALLO_RIGHTARM: u32 = 369099779u32;
pub const DIBUTTON_FOOTBALLO_RIGHT_LINK: u32 = 369149164u32;
pub const DIBUTTON_FOOTBALLO_SHOULDER: u32 = 369116167u32;
pub const DIBUTTON_FOOTBALLO_SPIN: u32 = 369099781u32;
pub const DIBUTTON_FOOTBALLO_SUBSTITUTE: u32 = 369116171u32;
pub const DIBUTTON_FOOTBALLO_THROW: u32 = 369099780u32;
pub const DIBUTTON_FOOTBALLO_TURBO: u32 = 369116168u32;
pub const DIBUTTON_FOOTBALLO_ZOOM: u32 = 369116170u32;
pub const DIBUTTON_FOOTBALLP_DEVICE: u32 = 335561982u32;
pub const DIBUTTON_FOOTBALLP_HELP: u32 = 335545347u32;
pub const DIBUTTON_FOOTBALLP_MENU: u32 = 335545597u32;
pub const DIBUTTON_FOOTBALLP_PAUSE: u32 = 335561980u32;
pub const DIBUTTON_FOOTBALLP_PLAY: u32 = 335545345u32;
pub const DIBUTTON_FOOTBALLP_SELECT: u32 = 335545346u32;
pub const DIBUTTON_FOOTBALLQ_AUDIBLE: u32 = 352338953u32;
pub const DIBUTTON_FOOTBALLQ_BACK_LINK: u32 = 352404712u32;
pub const DIBUTTON_FOOTBALLQ_DEVICE: u32 = 352339198u32;
pub const DIBUTTON_FOOTBALLQ_FAKE: u32 = 352322566u32;
pub const DIBUTTON_FOOTBALLQ_FAKESNAP: u32 = 352338951u32;
pub const DIBUTTON_FOOTBALLQ_FORWARD_LINK: u32 = 352404704u32;
pub const DIBUTTON_FOOTBALLQ_JUMP: u32 = 352322563u32;
pub const DIBUTTON_FOOTBALLQ_LEFT_LINK: u32 = 352371940u32;
pub const DIBUTTON_FOOTBALLQ_MENU: u32 = 352322813u32;
pub const DIBUTTON_FOOTBALLQ_MOTION: u32 = 352338952u32;
pub const DIBUTTON_FOOTBALLQ_PASS: u32 = 352322565u32;
pub const DIBUTTON_FOOTBALLQ_PAUSE: u32 = 352339196u32;
pub const DIBUTTON_FOOTBALLQ_RIGHT_LINK: u32 = 352371948u32;
pub const DIBUTTON_FOOTBALLQ_SELECT: u32 = 352322561u32;
pub const DIBUTTON_FOOTBALLQ_SLIDE: u32 = 352322564u32;
pub const DIBUTTON_FOOTBALLQ_SNAP: u32 = 352322562u32;
pub const DIBUTTON_FPS_APPLY: u32 = 150995971u32;
pub const DIBUTTON_FPS_BACKWARD_LINK: u32 = 151078120u32;
pub const DIBUTTON_FPS_CROUCH: u32 = 150995973u32;
pub const DIBUTTON_FPS_DEVICE: u32 = 151012606u32;
pub const DIBUTTON_FPS_DISPLAY: u32 = 151012360u32;
pub const DIBUTTON_FPS_DODGE: u32 = 151012361u32;
pub const DIBUTTON_FPS_FIRE: u32 = 150995969u32;
pub const DIBUTTON_FPS_FIRESECONDARY: u32 = 151012364u32;
pub const DIBUTTON_FPS_FORWARD_LINK: u32 = 151078112u32;
pub const DIBUTTON_FPS_GLANCEL: u32 = 151012362u32;
pub const DIBUTTON_FPS_GLANCER: u32 = 151012363u32;
pub const DIBUTTON_FPS_GLANCE_DOWN_LINK: u32 = 151110888u32;
pub const DIBUTTON_FPS_GLANCE_UP_LINK: u32 = 151110880u32;
pub const DIBUTTON_FPS_JUMP: u32 = 150995974u32;
pub const DIBUTTON_FPS_MENU: u32 = 150996221u32;
pub const DIBUTTON_FPS_PAUSE: u32 = 151012604u32;
pub const DIBUTTON_FPS_ROTATE_LEFT_LINK: u32 = 151045348u32;
pub const DIBUTTON_FPS_ROTATE_RIGHT_LINK: u32 = 151045356u32;
pub const DIBUTTON_FPS_SELECT: u32 = 150995972u32;
pub const DIBUTTON_FPS_STEP_LEFT_LINK: u32 = 151143652u32;
pub const DIBUTTON_FPS_STEP_RIGHT_LINK: u32 = 151143660u32;
pub const DIBUTTON_FPS_STRAFE: u32 = 150995975u32;
pub const DIBUTTON_FPS_WEAPONS: u32 = 150995970u32;
pub const DIBUTTON_GOLF_BACK_LINK: u32 = 402736360u32;
pub const DIBUTTON_GOLF_DEVICE: u32 = 402670846u32;
pub const DIBUTTON_GOLF_DOWN: u32 = 402654212u32;
pub const DIBUTTON_GOLF_FLYBY: u32 = 402654214u32;
pub const DIBUTTON_GOLF_FORWARD_LINK: u32 = 402736352u32;
pub const DIBUTTON_GOLF_LEFT_LINK: u32 = 402703588u32;
pub const DIBUTTON_GOLF_MENU: u32 = 402654461u32;
pub const DIBUTTON_GOLF_PAUSE: u32 = 402670844u32;
pub const DIBUTTON_GOLF_RIGHT_LINK: u32 = 402703596u32;
pub const DIBUTTON_GOLF_SELECT: u32 = 402654210u32;
pub const DIBUTTON_GOLF_SUBSTITUTE: u32 = 402670601u32;
pub const DIBUTTON_GOLF_SWING: u32 = 402654209u32;
pub const DIBUTTON_GOLF_TERRAIN: u32 = 402654213u32;
pub const DIBUTTON_GOLF_TIMEOUT: u32 = 402670600u32;
pub const DIBUTTON_GOLF_UP: u32 = 402654211u32;
pub const DIBUTTON_GOLF_ZOOM: u32 = 402670599u32;
pub const DIBUTTON_HOCKEYD_BACK_LINK: u32 = 436290792u32;
pub const DIBUTTON_HOCKEYD_BLOCK: u32 = 436208644u32;
pub const DIBUTTON_HOCKEYD_BURST: u32 = 436208643u32;
pub const DIBUTTON_HOCKEYD_DEVICE: u32 = 436225278u32;
pub const DIBUTTON_HOCKEYD_FAKE: u32 = 436208645u32;
pub const DIBUTTON_HOCKEYD_FORWARD_LINK: u32 = 436290784u32;
pub const DIBUTTON_HOCKEYD_LEFT_LINK: u32 = 436258020u32;
pub const DIBUTTON_HOCKEYD_MENU: u32 = 436208893u32;
pub const DIBUTTON_HOCKEYD_PAUSE: u32 = 436225276u32;
pub const DIBUTTON_HOCKEYD_PLAYER: u32 = 436208641u32;
pub const DIBUTTON_HOCKEYD_RIGHT_LINK: u32 = 436258028u32;
pub const DIBUTTON_HOCKEYD_STEAL: u32 = 436208642u32;
pub const DIBUTTON_HOCKEYD_STRATEGY: u32 = 436225031u32;
pub const DIBUTTON_HOCKEYD_SUBSTITUTE: u32 = 436225033u32;
pub const DIBUTTON_HOCKEYD_TIMEOUT: u32 = 436225032u32;
pub const DIBUTTON_HOCKEYD_ZOOM: u32 = 436225030u32;
pub const DIBUTTON_HOCKEYG_BACK_LINK: u32 = 453068008u32;
pub const DIBUTTON_HOCKEYG_BLOCK: u32 = 452985860u32;
pub const DIBUTTON_HOCKEYG_DEVICE: u32 = 453002494u32;
pub const DIBUTTON_HOCKEYG_FORWARD_LINK: u32 = 453068000u32;
pub const DIBUTTON_HOCKEYG_LEFT_LINK: u32 = 453035236u32;
pub const DIBUTTON_HOCKEYG_MENU: u32 = 452986109u32;
pub const DIBUTTON_HOCKEYG_PASS: u32 = 452985857u32;
pub const DIBUTTON_HOCKEYG_PAUSE: u32 = 453002492u32;
pub const DIBUTTON_HOCKEYG_POKE: u32 = 452985858u32;
pub const DIBUTTON_HOCKEYG_RIGHT_LINK: u32 = 453035244u32;
pub const DIBUTTON_HOCKEYG_STEAL: u32 = 452985859u32;
pub const DIBUTTON_HOCKEYG_STRATEGY: u32 = 453002246u32;
pub const DIBUTTON_HOCKEYG_SUBSTITUTE: u32 = 453002248u32;
pub const DIBUTTON_HOCKEYG_TIMEOUT: u32 = 453002247u32;
pub const DIBUTTON_HOCKEYG_ZOOM: u32 = 453002245u32;
pub const DIBUTTON_HOCKEYO_BACK_LINK: u32 = 419513576u32;
pub const DIBUTTON_HOCKEYO_BURST: u32 = 419431427u32;
pub const DIBUTTON_HOCKEYO_DEVICE: u32 = 419448062u32;
pub const DIBUTTON_HOCKEYO_FAKE: u32 = 419431429u32;
pub const DIBUTTON_HOCKEYO_FORWARD_LINK: u32 = 419513568u32;
pub const DIBUTTON_HOCKEYO_LEFT_LINK: u32 = 419480804u32;
pub const DIBUTTON_HOCKEYO_MENU: u32 = 419431677u32;
pub const DIBUTTON_HOCKEYO_PASS: u32 = 419431426u32;
pub const DIBUTTON_HOCKEYO_PAUSE: u32 = 419448060u32;
pub const DIBUTTON_HOCKEYO_RIGHT_LINK: u32 = 419480812u32;
pub const DIBUTTON_HOCKEYO_SHOOT: u32 = 419431425u32;
pub const DIBUTTON_HOCKEYO_SPECIAL: u32 = 419431428u32;
pub const DIBUTTON_HOCKEYO_STRATEGY: u32 = 419447815u32;
pub const DIBUTTON_HOCKEYO_SUBSTITUTE: u32 = 419447817u32;
pub const DIBUTTON_HOCKEYO_TIMEOUT: u32 = 419447816u32;
pub const DIBUTTON_HOCKEYO_ZOOM: u32 = 419447814u32;
pub const DIBUTTON_HUNTING_AIM: u32 = 218104834u32;
pub const DIBUTTON_HUNTING_BACK_LINK: u32 = 218186984u32;
pub const DIBUTTON_HUNTING_BINOCULAR: u32 = 218104836u32;
pub const DIBUTTON_HUNTING_CALL: u32 = 218104837u32;
pub const DIBUTTON_HUNTING_CROUCH: u32 = 218121225u32;
pub const DIBUTTON_HUNTING_DEVICE: u32 = 218121470u32;
pub const DIBUTTON_HUNTING_DISPLAY: u32 = 218121224u32;
pub const DIBUTTON_HUNTING_FIRE: u32 = 218104833u32;
pub const DIBUTTON_HUNTING_FIRESECONDARY: u32 = 218121227u32;
pub const DIBUTTON_HUNTING_FORWARD_LINK: u32 = 218186976u32;
pub const DIBUTTON_HUNTING_JUMP: u32 = 218121226u32;
pub const DIBUTTON_HUNTING_LEFT_LINK: u32 = 218154212u32;
pub const DIBUTTON_HUNTING_MAP: u32 = 218104838u32;
pub const DIBUTTON_HUNTING_MENU: u32 = 218105085u32;
pub const DIBUTTON_HUNTING_PAUSE: u32 = 218121468u32;
pub const DIBUTTON_HUNTING_RIGHT_LINK: u32 = 218154220u32;
pub const DIBUTTON_HUNTING_ROTATE_LEFT_LINK: u32 = 218252516u32;
pub const DIBUTTON_HUNTING_ROTATE_RIGHT_LINK: u32 = 218252524u32;
pub const DIBUTTON_HUNTING_SPECIAL: u32 = 218104839u32;
pub const DIBUTTON_HUNTING_WEAPON: u32 = 218104835u32;
pub const DIBUTTON_MECHA_BACK_LINK: u32 = 687949032u32;
pub const DIBUTTON_MECHA_CENTER: u32 = 687883271u32;
pub const DIBUTTON_MECHA_DEVICE: u32 = 687883518u32;
pub const DIBUTTON_MECHA_FASTER_LINK: u32 = 688112864u32;
pub const DIBUTTON_MECHA_FIRE: u32 = 687866881u32;
pub const DIBUTTON_MECHA_FIRESECONDARY: u32 = 687883273u32;
pub const DIBUTTON_MECHA_FORWARD_LINK: u32 = 687949024u32;
pub const DIBUTTON_MECHA_JUMP: u32 = 687866886u32;
pub const DIBUTTON_MECHA_LEFT_LINK: u32 = 687916260u32;
pub const DIBUTTON_MECHA_MENU: u32 = 687867133u32;
pub const DIBUTTON_MECHA_PAUSE: u32 = 687883516u32;
pub const DIBUTTON_MECHA_REVERSE: u32 = 687866884u32;
pub const DIBUTTON_MECHA_RIGHT_LINK: u32 = 687916268u32;
pub const DIBUTTON_MECHA_ROTATE_LEFT_LINK: u32 = 688014564u32;
pub const DIBUTTON_MECHA_ROTATE_RIGHT_LINK: u32 = 688014572u32;
pub const DIBUTTON_MECHA_SLOWER_LINK: u32 = 688112872u32;
pub const DIBUTTON_MECHA_TARGET: u32 = 687866883u32;
pub const DIBUTTON_MECHA_VIEW: u32 = 687883272u32;
pub const DIBUTTON_MECHA_WEAPONS: u32 = 687866882u32;
pub const DIBUTTON_MECHA_ZOOM: u32 = 687866885u32;
pub const DIBUTTON_RACQUET_BACKSWING: u32 = 536871938u32;
pub const DIBUTTON_RACQUET_BACK_LINK: u32 = 536954088u32;
pub const DIBUTTON_RACQUET_DEVICE: u32 = 536888574u32;
pub const DIBUTTON_RACQUET_FORWARD_LINK: u32 = 536954080u32;
pub const DIBUTTON_RACQUET_LEFT_LINK: u32 = 536921316u32;
pub const DIBUTTON_RACQUET_MENU: u32 = 536872189u32;
pub const DIBUTTON_RACQUET_PAUSE: u32 = 536888572u32;
pub const DIBUTTON_RACQUET_RIGHT_LINK: u32 = 536921324u32;
pub const DIBUTTON_RACQUET_SELECT: u32 = 536871941u32;
pub const DIBUTTON_RACQUET_SMASH: u32 = 536871939u32;
pub const DIBUTTON_RACQUET_SPECIAL: u32 = 536871940u32;
pub const DIBUTTON_RACQUET_SUBSTITUTE: u32 = 536888327u32;
pub const DIBUTTON_RACQUET_SWING: u32 = 536871937u32;
pub const DIBUTTON_RACQUET_TIMEOUT: u32 = 536888326u32;
pub const DIBUTTON_REMOTE_ADJUST: u32 = 654334990u32;
pub const DIBUTTON_REMOTE_CABLE: u32 = 654334985u32;
pub const DIBUTTON_REMOTE_CD: u32 = 654334986u32;
pub const DIBUTTON_REMOTE_CHANGE: u32 = 654320646u32;
pub const DIBUTTON_REMOTE_CUE: u32 = 654320644u32;
pub const DIBUTTON_REMOTE_DEVICE: u32 = 654329086u32;
pub const DIBUTTON_REMOTE_DIGIT0: u32 = 654332943u32;
pub const DIBUTTON_REMOTE_DIGIT1: u32 = 654332944u32;
pub const DIBUTTON_REMOTE_DIGIT2: u32 = 654332945u32;
pub const DIBUTTON_REMOTE_DIGIT3: u32 = 654332946u32;
pub const DIBUTTON_REMOTE_DIGIT4: u32 = 654332947u32;
pub const DIBUTTON_REMOTE_DIGIT5: u32 = 654332948u32;
pub const DIBUTTON_REMOTE_DIGIT6: u32 = 654332949u32;
pub const DIBUTTON_REMOTE_DIGIT7: u32 = 654332950u32;
pub const DIBUTTON_REMOTE_DIGIT8: u32 = 654332951u32;
pub const DIBUTTON_REMOTE_DIGIT9: u32 = 654332952u32;
pub const DIBUTTON_REMOTE_DVD: u32 = 654334989u32;
pub const DIBUTTON_REMOTE_MENU: u32 = 654312701u32;
pub const DIBUTTON_REMOTE_MUTE: u32 = 654312449u32;
pub const DIBUTTON_REMOTE_PAUSE: u32 = 654329084u32;
pub const DIBUTTON_REMOTE_PLAY: u32 = 654320643u32;
pub const DIBUTTON_REMOTE_RECORD: u32 = 654320647u32;
pub const DIBUTTON_REMOTE_REVIEW: u32 = 654320645u32;
pub const DIBUTTON_REMOTE_SELECT: u32 = 654312450u32;
pub const DIBUTTON_REMOTE_TUNER: u32 = 654334988u32;
pub const DIBUTTON_REMOTE_TV: u32 = 654334984u32;
pub const DIBUTTON_REMOTE_VCR: u32 = 654334987u32;
pub const DIBUTTON_SKIING_CAMERA: u32 = 486540291u32;
pub const DIBUTTON_SKIING_CROUCH: u32 = 486540290u32;
pub const DIBUTTON_SKIING_DEVICE: u32 = 486556926u32;
pub const DIBUTTON_SKIING_FASTER_LINK: u32 = 486622432u32;
pub const DIBUTTON_SKIING_JUMP: u32 = 486540289u32;
pub const DIBUTTON_SKIING_LEFT_LINK: u32 = 486589668u32;
pub const DIBUTTON_SKIING_MENU: u32 = 486540541u32;
pub const DIBUTTON_SKIING_PAUSE: u32 = 486556924u32;
pub const DIBUTTON_SKIING_RIGHT_LINK: u32 = 486589676u32;
pub const DIBUTTON_SKIING_SELECT: u32 = 486540293u32;
pub const DIBUTTON_SKIING_SLOWER_LINK: u32 = 486622440u32;
pub const DIBUTTON_SKIING_SPECIAL1: u32 = 486540292u32;
pub const DIBUTTON_SKIING_SPECIAL2: u32 = 486540294u32;
pub const DIBUTTON_SKIING_ZOOM: u32 = 486556679u32;
pub const DIBUTTON_SOCCERD_BACK_LINK: u32 = 520176872u32;
pub const DIBUTTON_SOCCERD_BLOCK: u32 = 520094721u32;
pub const DIBUTTON_SOCCERD_CLEAR: u32 = 520111114u32;
pub const DIBUTTON_SOCCERD_DEVICE: u32 = 520111358u32;
pub const DIBUTTON_SOCCERD_FAKE: u32 = 520094723u32;
pub const DIBUTTON_SOCCERD_FORWARD_LINK: u32 = 520176864u32;
pub const DIBUTTON_SOCCERD_FOUL: u32 = 520111112u32;
pub const DIBUTTON_SOCCERD_GOALIECHARGE: u32 = 520111115u32;
pub const DIBUTTON_SOCCERD_HEAD: u32 = 520111113u32;
pub const DIBUTTON_SOCCERD_LEFT_LINK: u32 = 520144100u32;
pub const DIBUTTON_SOCCERD_MENU: u32 = 520094973u32;
pub const DIBUTTON_SOCCERD_PAUSE: u32 = 520111356u32;
pub const DIBUTTON_SOCCERD_PLAYER: u32 = 520094724u32;
pub const DIBUTTON_SOCCERD_RIGHT_LINK: u32 = 520144108u32;
pub const DIBUTTON_SOCCERD_SELECT: u32 = 520094726u32;
pub const DIBUTTON_SOCCERD_SLIDE: u32 = 520094727u32;
pub const DIBUTTON_SOCCERD_SPECIAL: u32 = 520094725u32;
pub const DIBUTTON_SOCCERD_STEAL: u32 = 520094722u32;
pub const DIBUTTON_SOCCERD_SUBSTITUTE: u32 = 520111116u32;
pub const DIBUTTON_SOCCERO_BACK_LINK: u32 = 503399656u32;
pub const DIBUTTON_SOCCERO_CONTROL: u32 = 503333900u32;
pub const DIBUTTON_SOCCERO_DEVICE: u32 = 503334142u32;
pub const DIBUTTON_SOCCERO_FAKE: u32 = 503317507u32;
pub const DIBUTTON_SOCCERO_FORWARD_LINK: u32 = 503399648u32;
pub const DIBUTTON_SOCCERO_HEAD: u32 = 503333901u32;
pub const DIBUTTON_SOCCERO_LEFT_LINK: u32 = 503366884u32;
pub const DIBUTTON_SOCCERO_MENU: u32 = 503317757u32;
pub const DIBUTTON_SOCCERO_PASS: u32 = 503317506u32;
pub const DIBUTTON_SOCCERO_PASSTHRU: u32 = 503333898u32;
pub const DIBUTTON_SOCCERO_PAUSE: u32 = 503334140u32;
pub const DIBUTTON_SOCCERO_PLAYER: u32 = 503317508u32;
pub const DIBUTTON_SOCCERO_RIGHT_LINK: u32 = 503366892u32;
pub const DIBUTTON_SOCCERO_SELECT: u32 = 503317510u32;
pub const DIBUTTON_SOCCERO_SHOOT: u32 = 503317505u32;
pub const DIBUTTON_SOCCERO_SHOOTHIGH: u32 = 503333897u32;
pub const DIBUTTON_SOCCERO_SHOOTLOW: u32 = 503333896u32;
pub const DIBUTTON_SOCCERO_SPECIAL1: u32 = 503317509u32;
pub const DIBUTTON_SOCCERO_SPRINT: u32 = 503333899u32;
pub const DIBUTTON_SOCCERO_SUBSTITUTE: u32 = 503333895u32;
pub const DIBUTTON_SPACESIM_BACKWARD_LINK: u32 = 117523688u32;
pub const DIBUTTON_SPACESIM_DEVICE: u32 = 117458174u32;
pub const DIBUTTON_SPACESIM_DISPLAY: u32 = 117457925u32;
pub const DIBUTTON_SPACESIM_FASTER_LINK: u32 = 117687520u32;
pub const DIBUTTON_SPACESIM_FIRE: u32 = 117441537u32;
pub const DIBUTTON_SPACESIM_FIRESECONDARY: u32 = 117457929u32;
pub const DIBUTTON_SPACESIM_FORWARD_LINK: u32 = 117523680u32;
pub const DIBUTTON_SPACESIM_GEAR: u32 = 117457928u32;
pub const DIBUTTON_SPACESIM_GLANCE_DOWN_LINK: u32 = 117949672u32;
pub const DIBUTTON_SPACESIM_GLANCE_LEFT_LINK: u32 = 117949668u32;
pub const DIBUTTON_SPACESIM_GLANCE_RIGHT_LINK: u32 = 117949676u32;
pub const DIBUTTON_SPACESIM_GLANCE_UP_LINK: u32 = 117949664u32;
pub const DIBUTTON_SPACESIM_LEFT_LINK: u32 = 117490916u32;
pub const DIBUTTON_SPACESIM_LOWER: u32 = 117457927u32;
pub const DIBUTTON_SPACESIM_MENU: u32 = 117441789u32;
pub const DIBUTTON_SPACESIM_PAUSE: u32 = 117458172u32;
pub const DIBUTTON_SPACESIM_RAISE: u32 = 117457926u32;
pub const DIBUTTON_SPACESIM_RIGHT_LINK: u32 = 117490924u32;
pub const DIBUTTON_SPACESIM_SLOWER_LINK: u32 = 117687528u32;
pub const DIBUTTON_SPACESIM_TARGET: u32 = 117441539u32;
pub const DIBUTTON_SPACESIM_TURN_LEFT_LINK: u32 = 117589220u32;
pub const DIBUTTON_SPACESIM_TURN_RIGHT_LINK: u32 = 117589228u32;
pub const DIBUTTON_SPACESIM_VIEW: u32 = 117457924u32;
pub const DIBUTTON_SPACESIM_WEAPONS: u32 = 117441538u32;
pub const DIBUTTON_STRATEGYR_APPLY: u32 = 184550402u32;
pub const DIBUTTON_STRATEGYR_ATTACK: u32 = 184550404u32;
pub const DIBUTTON_STRATEGYR_BACK_LINK: u32 = 184632552u32;
pub const DIBUTTON_STRATEGYR_CAST: u32 = 184550405u32;
pub const DIBUTTON_STRATEGYR_CROUCH: u32 = 184550406u32;
pub const DIBUTTON_STRATEGYR_DEVICE: u32 = 184567038u32;
pub const DIBUTTON_STRATEGYR_DISPLAY: u32 = 184566793u32;
pub const DIBUTTON_STRATEGYR_FORWARD_LINK: u32 = 184632544u32;
pub const DIBUTTON_STRATEGYR_GET: u32 = 184550401u32;
pub const DIBUTTON_STRATEGYR_JUMP: u32 = 184550407u32;
pub const DIBUTTON_STRATEGYR_LEFT_LINK: u32 = 184599780u32;
pub const DIBUTTON_STRATEGYR_MAP: u32 = 184566792u32;
pub const DIBUTTON_STRATEGYR_MENU: u32 = 184550653u32;
pub const DIBUTTON_STRATEGYR_PAUSE: u32 = 184567036u32;
pub const DIBUTTON_STRATEGYR_RIGHT_LINK: u32 = 184599788u32;
pub const DIBUTTON_STRATEGYR_ROTATE_LEFT_LINK: u32 = 184698084u32;
pub const DIBUTTON_STRATEGYR_ROTATE_RIGHT_LINK: u32 = 184698092u32;
pub const DIBUTTON_STRATEGYR_SELECT: u32 = 184550403u32;
pub const DIBUTTON_STRATEGYT_APPLY: u32 = 201327619u32;
pub const DIBUTTON_STRATEGYT_BACK_LINK: u32 = 201409768u32;
pub const DIBUTTON_STRATEGYT_DEVICE: u32 = 201344254u32;
pub const DIBUTTON_STRATEGYT_DISPLAY: u32 = 201344008u32;
pub const DIBUTTON_STRATEGYT_FORWARD_LINK: u32 = 201409760u32;
pub const DIBUTTON_STRATEGYT_INSTRUCT: u32 = 201327618u32;
pub const DIBUTTON_STRATEGYT_LEFT_LINK: u32 = 201376996u32;
pub const DIBUTTON_STRATEGYT_MAP: u32 = 201344007u32;
pub const DIBUTTON_STRATEGYT_MENU: u32 = 201327869u32;
pub const DIBUTTON_STRATEGYT_PAUSE: u32 = 201344252u32;
pub const DIBUTTON_STRATEGYT_RIGHT_LINK: u32 = 201377004u32;
pub const DIBUTTON_STRATEGYT_SELECT: u32 = 201327617u32;
pub const DIBUTTON_STRATEGYT_TEAM: u32 = 201327620u32;
pub const DIBUTTON_STRATEGYT_TURN: u32 = 201327621u32;
pub const DIBUTTON_STRATEGYT_ZOOM: u32 = 201344006u32;
pub const DIBUTTON_TPS_ACTION: u32 = 167773186u32;
pub const DIBUTTON_TPS_BACKWARD_LINK: u32 = 167855336u32;
pub const DIBUTTON_TPS_DEVICE: u32 = 167789822u32;
pub const DIBUTTON_TPS_DODGE: u32 = 167789577u32;
pub const DIBUTTON_TPS_FORWARD_LINK: u32 = 167855328u32;
pub const DIBUTTON_TPS_GLANCE_DOWN_LINK: u32 = 168281320u32;
pub const DIBUTTON_TPS_GLANCE_LEFT_LINK: u32 = 168281316u32;
pub const DIBUTTON_TPS_GLANCE_RIGHT_LINK: u32 = 168281324u32;
pub const DIBUTTON_TPS_GLANCE_UP_LINK: u32 = 168281312u32;
pub const DIBUTTON_TPS_INVENTORY: u32 = 167789578u32;
pub const DIBUTTON_TPS_JUMP: u32 = 167773189u32;
pub const DIBUTTON_TPS_MENU: u32 = 167773437u32;
pub const DIBUTTON_TPS_PAUSE: u32 = 167789820u32;
pub const DIBUTTON_TPS_RUN: u32 = 167773185u32;
pub const DIBUTTON_TPS_SELECT: u32 = 167773187u32;
pub const DIBUTTON_TPS_STEPLEFT: u32 = 167789575u32;
pub const DIBUTTON_TPS_STEPRIGHT: u32 = 167789576u32;
pub const DIBUTTON_TPS_TURN_LEFT_LINK: u32 = 167920868u32;
pub const DIBUTTON_TPS_TURN_RIGHT_LINK: u32 = 167920876u32;
pub const DIBUTTON_TPS_USE: u32 = 167773188u32;
pub const DIBUTTON_TPS_VIEW: u32 = 167789574u32;
pub const DICD_DEFAULT: u32 = 0u32;
pub const DICD_EDIT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DICOLORSET {
    pub dwSize: u32,
    pub cTextFore: u32,
    pub cTextHighlight: u32,
    pub cCalloutLine: u32,
    pub cCalloutHighlight: u32,
    pub cBorder: u32,
    pub cControlFill: u32,
    pub cHighlightFill: u32,
    pub cAreaFill: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DICONDITION {
    pub lOffset: i32,
    pub lPositiveCoefficient: i32,
    pub lNegativeCoefficient: i32,
    pub dwPositiveSaturation: u32,
    pub dwNegativeSaturation: u32,
    pub lDeadBand: i32,
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct DICONFIGUREDEVICESPARAMSA {
    pub dwSize: u32,
    pub dwcUsers: u32,
    pub lptszUserNames: windows_core::PSTR,
    pub dwcFormats: u32,
    pub lprgFormats: *mut DIACTIONFORMATA,
    pub hwnd: super::super::Foundation::HWND,
    pub dics: DICOLORSET,
    pub lpUnkDDSTarget: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
impl Default for DICONFIGUREDEVICESPARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct DICONFIGUREDEVICESPARAMSW {
    pub dwSize: u32,
    pub dwcUsers: u32,
    pub lptszUserNames: windows_core::PWSTR,
    pub dwcFormats: u32,
    pub lprgFormats: *mut DIACTIONFORMATW,
    pub hwnd: super::super::Foundation::HWND,
    pub dics: DICOLORSET,
    pub lpUnkDDSTarget: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
impl Default for DICONFIGUREDEVICESPARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DICONSTANTFORCE {
    pub lMagnitude: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DICUSTOMFORCE {
    pub cChannels: u32,
    pub dwSamplePeriod: u32,
    pub cSamples: u32,
    pub rglForceData: *mut i32,
}
impl Default for DICUSTOMFORCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIDAL_BOTTOMALIGNED: u32 = 8u32;
pub const DIDAL_CENTERED: u32 = 0u32;
pub const DIDAL_LEFTALIGNED: u32 = 1u32;
pub const DIDAL_MIDDLE: u32 = 0u32;
pub const DIDAL_RIGHTALIGNED: u32 = 2u32;
pub const DIDAL_TOPALIGNED: u32 = 4u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDATAFORMAT {
    pub dwSize: u32,
    pub dwObjSize: u32,
    pub dwFlags: u32,
    pub dwDataSize: u32,
    pub dwNumObjs: u32,
    pub rgodf: *mut DIOBJECTDATAFORMAT,
}
impl Default for DIDATAFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIDBAM_DEFAULT: u32 = 0u32;
pub const DIDBAM_HWDEFAULTS: u32 = 4u32;
pub const DIDBAM_INITIALIZE: u32 = 2u32;
pub const DIDBAM_PRESERVE: u32 = 1u32;
pub const DIDC_ALIAS: u32 = 65536u32;
pub const DIDC_ATTACHED: u32 = 1u32;
pub const DIDC_DEADBAND: u32 = 16384u32;
pub const DIDC_EMULATED: u32 = 4u32;
pub const DIDC_FFATTACK: u32 = 512u32;
pub const DIDC_FFFADE: u32 = 1024u32;
pub const DIDC_FORCEFEEDBACK: u32 = 256u32;
pub const DIDC_HIDDEN: u32 = 262144u32;
pub const DIDC_PHANTOM: u32 = 131072u32;
pub const DIDC_POLLEDDATAFORMAT: u32 = 8u32;
pub const DIDC_POLLEDDEVICE: u32 = 2u32;
pub const DIDC_POSNEGCOEFFICIENTS: u32 = 4096u32;
pub const DIDC_POSNEGSATURATION: u32 = 8192u32;
pub const DIDC_SATURATION: u32 = 2048u32;
pub const DIDC_STARTDELAY: u32 = 32768u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIDEVCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDevType: u32,
    pub dwAxes: u32,
    pub dwButtons: u32,
    pub dwPOVs: u32,
    pub dwFFSamplePeriod: u32,
    pub dwFFMinTimeResolution: u32,
    pub dwFirmwareRevision: u32,
    pub dwHardwareRevision: u32,
    pub dwFFDriverVersion: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIDEVCAPS_DX3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDevType: u32,
    pub dwAxes: u32,
    pub dwButtons: u32,
    pub dwPOVs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEIMAGEINFOA {
    pub tszImagePath: [i8; 260],
    pub dwFlags: u32,
    pub dwViewID: u32,
    pub rcOverlay: super::super::Foundation::RECT,
    pub dwObjID: u32,
    pub dwcValidPts: u32,
    pub rgptCalloutLine: [super::super::Foundation::POINT; 5],
    pub rcCalloutRect: super::super::Foundation::RECT,
    pub dwTextAlign: u32,
}
impl Default for DIDEVICEIMAGEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEIMAGEINFOHEADERA {
    pub dwSize: u32,
    pub dwSizeImageInfo: u32,
    pub dwcViews: u32,
    pub dwcButtons: u32,
    pub dwcAxes: u32,
    pub dwcPOVs: u32,
    pub dwBufferSize: u32,
    pub dwBufferUsed: u32,
    pub lprgImageInfoArray: *mut DIDEVICEIMAGEINFOA,
}
impl Default for DIDEVICEIMAGEINFOHEADERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEIMAGEINFOHEADERW {
    pub dwSize: u32,
    pub dwSizeImageInfo: u32,
    pub dwcViews: u32,
    pub dwcButtons: u32,
    pub dwcAxes: u32,
    pub dwcPOVs: u32,
    pub dwBufferSize: u32,
    pub dwBufferUsed: u32,
    pub lprgImageInfoArray: *mut DIDEVICEIMAGEINFOW,
}
impl Default for DIDEVICEIMAGEINFOHEADERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEIMAGEINFOW {
    pub tszImagePath: [u16; 260],
    pub dwFlags: u32,
    pub dwViewID: u32,
    pub rcOverlay: super::super::Foundation::RECT,
    pub dwObjID: u32,
    pub dwcValidPts: u32,
    pub rgptCalloutLine: [super::super::Foundation::POINT; 5],
    pub rcCalloutRect: super::super::Foundation::RECT,
    pub dwTextAlign: u32,
}
impl Default for DIDEVICEIMAGEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEINSTANCEA {
    pub dwSize: u32,
    pub guidInstance: windows_core::GUID,
    pub guidProduct: windows_core::GUID,
    pub dwDevType: u32,
    pub tszInstanceName: [i8; 260],
    pub tszProductName: [i8; 260],
    pub guidFFDriver: windows_core::GUID,
    pub wUsagePage: u16,
    pub wUsage: u16,
}
impl Default for DIDEVICEINSTANCEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEINSTANCEW {
    pub dwSize: u32,
    pub guidInstance: windows_core::GUID,
    pub guidProduct: windows_core::GUID,
    pub dwDevType: u32,
    pub tszInstanceName: [u16; 260],
    pub tszProductName: [u16; 260],
    pub guidFFDriver: windows_core::GUID,
    pub wUsagePage: u16,
    pub wUsage: u16,
}
impl Default for DIDEVICEINSTANCEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEINSTANCE_DX3A {
    pub dwSize: u32,
    pub guidInstance: windows_core::GUID,
    pub guidProduct: windows_core::GUID,
    pub dwDevType: u32,
    pub tszInstanceName: [i8; 260],
    pub tszProductName: [i8; 260],
}
impl Default for DIDEVICEINSTANCE_DX3A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEINSTANCE_DX3W {
    pub dwSize: u32,
    pub guidInstance: windows_core::GUID,
    pub guidProduct: windows_core::GUID,
    pub dwDevType: u32,
    pub tszInstanceName: [u16; 260],
    pub tszProductName: [u16; 260],
}
impl Default for DIDEVICEINSTANCE_DX3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIDEVICEOBJECTDATA {
    pub dwOfs: u32,
    pub dwData: u32,
    pub dwTimeStamp: u32,
    pub dwSequence: u32,
    pub uAppData: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIDEVICEOBJECTDATA_DX3 {
    pub dwOfs: u32,
    pub dwData: u32,
    pub dwTimeStamp: u32,
    pub dwSequence: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEOBJECTINSTANCEA {
    pub dwSize: u32,
    pub guidType: windows_core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub tszName: [i8; 260],
    pub dwFFMaxForce: u32,
    pub dwFFForceResolution: u32,
    pub wCollectionNumber: u16,
    pub wDesignatorIndex: u16,
    pub wUsagePage: u16,
    pub wUsage: u16,
    pub dwDimension: u32,
    pub wExponent: u16,
    pub wReportId: u16,
}
impl Default for DIDEVICEOBJECTINSTANCEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEOBJECTINSTANCEW {
    pub dwSize: u32,
    pub guidType: windows_core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub tszName: [u16; 260],
    pub dwFFMaxForce: u32,
    pub dwFFForceResolution: u32,
    pub wCollectionNumber: u16,
    pub wDesignatorIndex: u16,
    pub wUsagePage: u16,
    pub wUsage: u16,
    pub dwDimension: u32,
    pub wExponent: u16,
    pub wReportId: u16,
}
impl Default for DIDEVICEOBJECTINSTANCEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEOBJECTINSTANCE_DX3A {
    pub dwSize: u32,
    pub guidType: windows_core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub tszName: [i8; 260],
}
impl Default for DIDEVICEOBJECTINSTANCE_DX3A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIDEVICEOBJECTINSTANCE_DX3W {
    pub dwSize: u32,
    pub guidType: windows_core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
    pub tszName: [u16; 260],
}
impl Default for DIDEVICEOBJECTINSTANCE_DX3W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIDEVICESTATE {
    pub dwSize: u32,
    pub dwState: u32,
    pub dwLoad: u32,
}
pub const DIDEVTYPEJOYSTICK_FLIGHTSTICK: u32 = 3u32;
pub const DIDEVTYPEJOYSTICK_GAMEPAD: u32 = 4u32;
pub const DIDEVTYPEJOYSTICK_HEADTRACKER: u32 = 7u32;
pub const DIDEVTYPEJOYSTICK_RUDDER: u32 = 5u32;
pub const DIDEVTYPEJOYSTICK_TRADITIONAL: u32 = 2u32;
pub const DIDEVTYPEJOYSTICK_UNKNOWN: u32 = 1u32;
pub const DIDEVTYPEJOYSTICK_WHEEL: u32 = 6u32;
pub const DIDEVTYPEKEYBOARD_J3100: u32 = 12u32;
pub const DIDEVTYPEKEYBOARD_JAPAN106: u32 = 10u32;
pub const DIDEVTYPEKEYBOARD_JAPANAX: u32 = 11u32;
pub const DIDEVTYPEKEYBOARD_NEC98: u32 = 7u32;
pub const DIDEVTYPEKEYBOARD_NEC98106: u32 = 9u32;
pub const DIDEVTYPEKEYBOARD_NEC98LAPTOP: u32 = 8u32;
pub const DIDEVTYPEKEYBOARD_NOKIA1050: u32 = 5u32;
pub const DIDEVTYPEKEYBOARD_NOKIA9140: u32 = 6u32;
pub const DIDEVTYPEKEYBOARD_OLIVETTI: u32 = 2u32;
pub const DIDEVTYPEKEYBOARD_PCAT: u32 = 3u32;
pub const DIDEVTYPEKEYBOARD_PCENH: u32 = 4u32;
pub const DIDEVTYPEKEYBOARD_PCXT: u32 = 1u32;
pub const DIDEVTYPEKEYBOARD_UNKNOWN: u32 = 0u32;
pub const DIDEVTYPEMOUSE_FINGERSTICK: u32 = 3u32;
pub const DIDEVTYPEMOUSE_TOUCHPAD: u32 = 4u32;
pub const DIDEVTYPEMOUSE_TRACKBALL: u32 = 5u32;
pub const DIDEVTYPEMOUSE_TRADITIONAL: u32 = 2u32;
pub const DIDEVTYPEMOUSE_UNKNOWN: u32 = 1u32;
pub const DIDEVTYPE_DEVICE: u32 = 1u32;
pub const DIDEVTYPE_HID: u32 = 65536u32;
pub const DIDEVTYPE_JOYSTICK: u32 = 4u32;
pub const DIDEVTYPE_KEYBOARD: u32 = 3u32;
pub const DIDEVTYPE_MOUSE: u32 = 2u32;
pub const DIDFT_ABSAXIS: u32 = 2u32;
pub const DIDFT_ALIAS: u32 = 134217728u32;
pub const DIDFT_ALL: u32 = 0u32;
pub const DIDFT_ANYINSTANCE: u32 = 16776960u32;
pub const DIDFT_AXIS: u32 = 3u32;
pub const DIDFT_BUTTON: u32 = 12u32;
pub const DIDFT_COLLECTION: u32 = 64u32;
pub const DIDFT_FFACTUATOR: u32 = 16777216u32;
pub const DIDFT_FFEFFECTTRIGGER: u32 = 33554432u32;
pub const DIDFT_INSTANCEMASK: u32 = 16776960u32;
pub const DIDFT_NOCOLLECTION: u32 = 16776960u32;
pub const DIDFT_NODATA: u32 = 128u32;
pub const DIDFT_OUTPUT: u32 = 268435456u32;
pub const DIDFT_POV: u32 = 16u32;
pub const DIDFT_PSHBUTTON: u32 = 4u32;
pub const DIDFT_RELAXIS: u32 = 1u32;
pub const DIDFT_TGLBUTTON: u32 = 8u32;
pub const DIDFT_VENDORDEFINED: u32 = 67108864u32;
pub const DIDF_ABSAXIS: u32 = 1u32;
pub const DIDF_RELAXIS: u32 = 2u32;
pub const DIDIFT_CONFIGURATION: u32 = 1u32;
pub const DIDIFT_DELETE: u32 = 16777216u32;
pub const DIDIFT_OVERLAY: u32 = 2u32;
pub const DIDOI_ASPECTACCEL: u32 = 768u32;
pub const DIDOI_ASPECTFORCE: u32 = 1024u32;
pub const DIDOI_ASPECTMASK: u32 = 3840u32;
pub const DIDOI_ASPECTPOSITION: u32 = 256u32;
pub const DIDOI_ASPECTVELOCITY: u32 = 512u32;
pub const DIDOI_FFACTUATOR: u32 = 1u32;
pub const DIDOI_FFEFFECTTRIGGER: u32 = 2u32;
pub const DIDOI_GUIDISUSAGE: u32 = 65536u32;
pub const DIDOI_POLLED: u32 = 32768u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIDRIVERVERSIONS {
    pub dwSize: u32,
    pub dwFirmwareRevision: u32,
    pub dwHardwareRevision: u32,
    pub dwFFDriverVersion: u32,
}
pub const DIDSAM_DEFAULT: u32 = 0u32;
pub const DIDSAM_FORCESAVE: u32 = 2u32;
pub const DIDSAM_NOUSER: u32 = 1u32;
pub const DIEB_NOTRIGGER: u32 = 4294967295u32;
pub const DIEDBSFL_ATTACHEDONLY: u32 = 0u32;
pub const DIEDBSFL_AVAILABLEDEVICES: u32 = 4096u32;
pub const DIEDBSFL_FORCEFEEDBACK: u32 = 256u32;
pub const DIEDBSFL_MULTIMICEKEYBOARDS: u32 = 8192u32;
pub const DIEDBSFL_NONGAMINGDEVICES: u32 = 16384u32;
pub const DIEDBSFL_THISUSER: u32 = 16u32;
pub const DIEDBSFL_VALID: u32 = 28944u32;
pub const DIEDBS_MAPPEDPRI1: u32 = 1u32;
pub const DIEDBS_MAPPEDPRI2: u32 = 2u32;
pub const DIEDBS_NEWDEVICE: u32 = 32u32;
pub const DIEDBS_RECENTDEVICE: u32 = 16u32;
pub const DIEDFL_ALLDEVICES: u32 = 0u32;
pub const DIEDFL_ATTACHEDONLY: u32 = 1u32;
pub const DIEDFL_FORCEFEEDBACK: u32 = 256u32;
pub const DIEDFL_INCLUDEALIASES: u32 = 65536u32;
pub const DIEDFL_INCLUDEHIDDEN: u32 = 262144u32;
pub const DIEDFL_INCLUDEPHANTOMS: u32 = 131072u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIEFFECT {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDuration: u32,
    pub dwSamplePeriod: u32,
    pub dwGain: u32,
    pub dwTriggerButton: u32,
    pub dwTriggerRepeatInterval: u32,
    pub cAxes: u32,
    pub rgdwAxes: *mut u32,
    pub rglDirection: *mut i32,
    pub lpEnvelope: *mut DIENVELOPE,
    pub cbTypeSpecificParams: u32,
    pub lpvTypeSpecificParams: *mut core::ffi::c_void,
    pub dwStartDelay: u32,
}
impl Default for DIEFFECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIEFFECTATTRIBUTES {
    pub dwEffectId: u32,
    pub dwEffType: u32,
    pub dwStaticParams: u32,
    pub dwDynamicParams: u32,
    pub dwCoords: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIEFFECTINFOA {
    pub dwSize: u32,
    pub guid: windows_core::GUID,
    pub dwEffType: u32,
    pub dwStaticParams: u32,
    pub dwDynamicParams: u32,
    pub tszName: [i8; 260],
}
impl Default for DIEFFECTINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIEFFECTINFOW {
    pub dwSize: u32,
    pub guid: windows_core::GUID,
    pub dwEffType: u32,
    pub dwStaticParams: u32,
    pub dwDynamicParams: u32,
    pub tszName: [u16; 260],
}
impl Default for DIEFFECTINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIEFFECT_DX5 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDuration: u32,
    pub dwSamplePeriod: u32,
    pub dwGain: u32,
    pub dwTriggerButton: u32,
    pub dwTriggerRepeatInterval: u32,
    pub cAxes: u32,
    pub rgdwAxes: *mut u32,
    pub rglDirection: *mut i32,
    pub lpEnvelope: *mut DIENVELOPE,
    pub cbTypeSpecificParams: u32,
    pub lpvTypeSpecificParams: *mut core::ffi::c_void,
}
impl Default for DIEFFECT_DX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIEFFESCAPE {
    pub dwSize: u32,
    pub dwCommand: u32,
    pub lpvInBuffer: *mut core::ffi::c_void,
    pub cbInBuffer: u32,
    pub lpvOutBuffer: *mut core::ffi::c_void,
    pub cbOutBuffer: u32,
}
impl Default for DIEFFESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIEFF_CARTESIAN: u32 = 16u32;
pub const DIEFF_OBJECTIDS: u32 = 1u32;
pub const DIEFF_OBJECTOFFSETS: u32 = 2u32;
pub const DIEFF_POLAR: u32 = 32u32;
pub const DIEFF_SPHERICAL: u32 = 64u32;
pub const DIEFT_ALL: u32 = 0u32;
pub const DIEFT_CONDITION: u32 = 4u32;
pub const DIEFT_CONSTANTFORCE: u32 = 1u32;
pub const DIEFT_CUSTOMFORCE: u32 = 5u32;
pub const DIEFT_DEADBAND: u32 = 16384u32;
pub const DIEFT_FFATTACK: u32 = 512u32;
pub const DIEFT_FFFADE: u32 = 1024u32;
pub const DIEFT_HARDWARE: u32 = 255u32;
pub const DIEFT_PERIODIC: u32 = 3u32;
pub const DIEFT_POSNEGCOEFFICIENTS: u32 = 4096u32;
pub const DIEFT_POSNEGSATURATION: u32 = 8192u32;
pub const DIEFT_RAMPFORCE: u32 = 2u32;
pub const DIEFT_SATURATION: u32 = 2048u32;
pub const DIEFT_STARTDELAY: u32 = 32768u32;
pub const DIEGES_EMULATED: u32 = 2u32;
pub const DIEGES_PLAYING: u32 = 1u32;
pub const DIENUM_CONTINUE: u32 = 1u32;
pub const DIENUM_STOP: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIENVELOPE {
    pub dwSize: u32,
    pub dwAttackLevel: u32,
    pub dwAttackTime: u32,
    pub dwFadeLevel: u32,
    pub dwFadeTime: u32,
}
pub const DIEP_ALLPARAMS: u32 = 1023u32;
pub const DIEP_ALLPARAMS_DX5: u32 = 511u32;
pub const DIEP_AXES: u32 = 32u32;
pub const DIEP_DIRECTION: u32 = 64u32;
pub const DIEP_DURATION: u32 = 1u32;
pub const DIEP_ENVELOPE: u32 = 128u32;
pub const DIEP_GAIN: u32 = 4u32;
pub const DIEP_NODOWNLOAD: u32 = 2147483648u32;
pub const DIEP_NORESTART: u32 = 1073741824u32;
pub const DIEP_SAMPLEPERIOD: u32 = 2u32;
pub const DIEP_START: u32 = 536870912u32;
pub const DIEP_STARTDELAY: u32 = 512u32;
pub const DIEP_TRIGGERBUTTON: u32 = 8u32;
pub const DIEP_TRIGGERREPEATINTERVAL: u32 = 16u32;
pub const DIEP_TYPESPECIFICPARAMS: u32 = 256u32;
pub const DIERR_ACQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x800700AA_u32 as _);
pub const DIERR_ALREADYINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x800704DF_u32 as _);
pub const DIERR_BADDRIVERVER: windows_core::HRESULT = windows_core::HRESULT(0x80070077_u32 as _);
pub const DIERR_BADINF: i32 = -2147220478i32;
pub const DIERR_BETADIRECTINPUTVERSION: windows_core::HRESULT = windows_core::HRESULT(0x80070481_u32 as _);
pub const DIERR_CANCELLED: i32 = -2147220479i32;
pub const DIERR_DEVICEFULL: i32 = -2147220991i32;
pub const DIERR_DEVICENOTREG: i32 = -2147221164i32;
pub const DIERR_DRIVERFIRST: i32 = -2147220736i32;
pub const DIERR_DRIVERLAST: i32 = -2147220481i32;
pub const DIERR_EFFECTPLAYING: i32 = -2147220984i32;
pub const DIERR_GENERIC: i32 = -2147467259i32;
pub const DIERR_HANDLEEXISTS: i32 = -2147024891i32;
pub const DIERR_HASEFFECTS: i32 = -2147220988i32;
pub const DIERR_INCOMPLETEEFFECT: i32 = -2147220986i32;
pub const DIERR_INPUTLOST: windows_core::HRESULT = windows_core::HRESULT(0x8007001E_u32 as _);
pub const DIERR_INSUFFICIENTPRIVS: i32 = -2147220992i32;
pub const DIERR_INVALIDCLASSINSTALLER: i32 = -2147220480i32;
pub const DIERR_INVALIDPARAM: i32 = -2147024809i32;
pub const DIERR_MAPFILEFAIL: i32 = -2147220981i32;
pub const DIERR_MOREDATA: i32 = -2147220990i32;
pub const DIERR_NOAGGREGATION: i32 = -2147221232i32;
pub const DIERR_NOINTERFACE: i32 = -2147467262i32;
pub const DIERR_NOMOREITEMS: windows_core::HRESULT = windows_core::HRESULT(0x80070103_u32 as _);
pub const DIERR_NOTACQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8007000C_u32 as _);
pub const DIERR_NOTBUFFERED: i32 = -2147220985i32;
pub const DIERR_NOTDOWNLOADED: i32 = -2147220989i32;
pub const DIERR_NOTEXCLUSIVEACQUIRED: i32 = -2147220987i32;
pub const DIERR_NOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80070002_u32 as _);
pub const DIERR_NOTINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80070015_u32 as _);
pub const DIERR_OBJECTNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80070002_u32 as _);
pub const DIERR_OLDDIRECTINPUTVERSION: windows_core::HRESULT = windows_core::HRESULT(0x8007047E_u32 as _);
pub const DIERR_OTHERAPPHASPRIO: i32 = -2147024891i32;
pub const DIERR_OUTOFMEMORY: i32 = -2147024882i32;
pub const DIERR_READONLY: i32 = -2147024891i32;
pub const DIERR_REPORTFULL: i32 = -2147220982i32;
pub const DIERR_UNPLUGGED: i32 = -2147220983i32;
pub const DIERR_UNSUPPORTED: i32 = -2147467263i32;
pub const DIES_NODOWNLOAD: u32 = 2147483648u32;
pub const DIES_SOLO: u32 = 1u32;
pub const DIFEF_DEFAULT: u32 = 0u32;
pub const DIFEF_INCLUDENONSTANDARD: u32 = 1u32;
pub const DIFEF_MODIFYIFNEEDED: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIFFDEVICEATTRIBUTES {
    pub dwFlags: u32,
    pub dwFFSamplePeriod: u32,
    pub dwFFMinTimeResolution: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIFFOBJECTATTRIBUTES {
    pub dwFFMaxForce: u32,
    pub dwFFForceResolution: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIFILEEFFECT {
    pub dwSize: u32,
    pub GuidEffect: windows_core::GUID,
    pub lpDiEffect: *mut DIEFFECT,
    pub szFriendlyName: [i8; 260],
}
impl Default for DIFILEEFFECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIGDD_PEEK: u32 = 1u32;
pub const DIGFFS_ACTUATORSOFF: u32 = 32u32;
pub const DIGFFS_ACTUATORSON: u32 = 16u32;
pub const DIGFFS_DEVICELOST: u32 = 2147483648u32;
pub const DIGFFS_EMPTY: u32 = 1u32;
pub const DIGFFS_PAUSED: u32 = 4u32;
pub const DIGFFS_POWEROFF: u32 = 128u32;
pub const DIGFFS_POWERON: u32 = 64u32;
pub const DIGFFS_SAFETYSWITCHOFF: u32 = 512u32;
pub const DIGFFS_SAFETYSWITCHON: u32 = 256u32;
pub const DIGFFS_STOPPED: u32 = 2u32;
pub const DIGFFS_USERFFSWITCHOFF: u32 = 2048u32;
pub const DIGFFS_USERFFSWITCHON: u32 = 1024u32;
pub const DIHATSWITCH_2DCONTROL_HATSWITCH: u32 = 587220481u32;
pub const DIHATSWITCH_3DCONTROL_HATSWITCH: u32 = 603997697u32;
pub const DIHATSWITCH_ARCADEP_VIEW: u32 = 570443265u32;
pub const DIHATSWITCH_ARCADES_VIEW: u32 = 553666049u32;
pub const DIHATSWITCH_BBALLD_GLANCE: u32 = 318785025u32;
pub const DIHATSWITCH_BBALLO_GLANCE: u32 = 302007809u32;
pub const DIHATSWITCH_BIKINGM_SCROLL: u32 = 469779969u32;
pub const DIHATSWITCH_CADF_HATSWITCH: u32 = 620774913u32;
pub const DIHATSWITCH_CADM_HATSWITCH: u32 = 637552129u32;
pub const DIHATSWITCH_DRIVINGC_GLANCE: u32 = 33572353u32;
pub const DIHATSWITCH_DRIVINGR_GLANCE: u32 = 16795137u32;
pub const DIHATSWITCH_DRIVINGT_GLANCE: u32 = 50349569u32;
pub const DIHATSWITCH_FIGHTINGH_SLIDE: u32 = 134235649u32;
pub const DIHATSWITCH_FISHING_GLANCE: u32 = 234898945u32;
pub const DIHATSWITCH_FLYINGC_GLANCE: u32 = 67126785u32;
pub const DIHATSWITCH_FLYINGH_GLANCE: u32 = 100681217u32;
pub const DIHATSWITCH_FLYINGM_GLANCE: u32 = 83904001u32;
pub const DIHATSWITCH_FPS_GLANCE: u32 = 151012865u32;
pub const DIHATSWITCH_GOLF_SCROLL: u32 = 402671105u32;
pub const DIHATSWITCH_HOCKEYD_SCROLL: u32 = 436225537u32;
pub const DIHATSWITCH_HOCKEYG_SCROLL: u32 = 453002753u32;
pub const DIHATSWITCH_HOCKEYO_SCROLL: u32 = 419448321u32;
pub const DIHATSWITCH_HUNTING_GLANCE: u32 = 218121729u32;
pub const DIHATSWITCH_MECHA_GLANCE: u32 = 687883777u32;
pub const DIHATSWITCH_RACQUET_GLANCE: u32 = 536888833u32;
pub const DIHATSWITCH_SKIING_GLANCE: u32 = 486557185u32;
pub const DIHATSWITCH_SOCCERD_GLANCE: u32 = 520111617u32;
pub const DIHATSWITCH_SOCCERO_GLANCE: u32 = 503334401u32;
pub const DIHATSWITCH_SPACESIM_GLANCE: u32 = 117458433u32;
pub const DIHATSWITCH_STRATEGYR_GLANCE: u32 = 184567297u32;
pub const DIHATSWITCH_TPS_GLANCE: u32 = 167790081u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIHIDFFINITINFO {
    pub dwSize: u32,
    pub pwszDeviceInterface: windows_core::PWSTR,
    pub GuidInstance: windows_core::GUID,
}
pub const DIJC_CALLOUT: u32 = 8u32;
pub const DIJC_GAIN: u32 = 4u32;
pub const DIJC_GUIDINSTANCE: u32 = 1u32;
pub const DIJC_REGHWCONFIGTYPE: u32 = 2u32;
pub const DIJC_WDMGAMEPORT: u32 = 16u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYCONFIG {
    pub dwSize: u32,
    pub guidInstance: windows_core::GUID,
    pub hwc: JOYREGHWCONFIG,
    pub dwGain: u32,
    pub wszType: [u16; 256],
    pub wszCallout: [u16; 256],
    pub guidGameport: windows_core::GUID,
}
impl Default for DIJOYCONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYCONFIG_DX5 {
    pub dwSize: u32,
    pub guidInstance: windows_core::GUID,
    pub hwc: JOYREGHWCONFIG,
    pub dwGain: u32,
    pub wszType: [u16; 256],
    pub wszCallout: [u16; 256],
}
impl Default for DIJOYCONFIG_DX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYSTATE {
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub lRx: i32,
    pub lRy: i32,
    pub lRz: i32,
    pub rglSlider: [i32; 2],
    pub rgdwPOV: [u32; 4],
    pub rgbButtons: [u8; 32],
}
impl Default for DIJOYSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYSTATE2 {
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub lRx: i32,
    pub lRy: i32,
    pub lRz: i32,
    pub rglSlider: [i32; 2],
    pub rgdwPOV: [u32; 4],
    pub rgbButtons: [u8; 128],
    pub lVX: i32,
    pub lVY: i32,
    pub lVZ: i32,
    pub lVRx: i32,
    pub lVRy: i32,
    pub lVRz: i32,
    pub rglVSlider: [i32; 2],
    pub lAX: i32,
    pub lAY: i32,
    pub lAZ: i32,
    pub lARx: i32,
    pub lARy: i32,
    pub lARz: i32,
    pub rglASlider: [i32; 2],
    pub lFX: i32,
    pub lFY: i32,
    pub lFZ: i32,
    pub lFRx: i32,
    pub lFRy: i32,
    pub lFRz: i32,
    pub rglFSlider: [i32; 2],
}
impl Default for DIJOYSTATE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYTYPEINFO {
    pub dwSize: u32,
    pub hws: JOYREGHWSETTINGS,
    pub clsidConfig: windows_core::GUID,
    pub wszDisplayName: [u16; 256],
    pub wszCallout: [u16; 260],
    pub wszHardwareId: [u16; 256],
    pub dwFlags1: u32,
    pub dwFlags2: u32,
    pub wszMapFile: [u16; 256],
}
impl Default for DIJOYTYPEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYTYPEINFO_DX5 {
    pub dwSize: u32,
    pub hws: JOYREGHWSETTINGS,
    pub clsidConfig: windows_core::GUID,
    pub wszDisplayName: [u16; 256],
    pub wszCallout: [u16; 260],
}
impl Default for DIJOYTYPEINFO_DX5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYTYPEINFO_DX6 {
    pub dwSize: u32,
    pub hws: JOYREGHWSETTINGS,
    pub clsidConfig: windows_core::GUID,
    pub wszDisplayName: [u16; 256],
    pub wszCallout: [u16; 260],
    pub wszHardwareId: [u16; 256],
    pub dwFlags1: u32,
}
impl Default for DIJOYTYPEINFO_DX6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIJOYUSERVALUES {
    pub dwSize: u32,
    pub ruv: JOYREGUSERVALUES,
    pub wszGlobalDriver: [u16; 256],
    pub wszGameportEmulator: [u16; 256],
}
impl Default for DIJOYUSERVALUES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIJU_GAMEPORTEMULATOR: u32 = 4u32;
pub const DIJU_GLOBALDRIVER: u32 = 2u32;
pub const DIJU_USERVALUES: u32 = 1u32;
pub const DIKEYBOARD_0: u32 = 2164261899u32;
pub const DIKEYBOARD_1: u32 = 2164261890u32;
pub const DIKEYBOARD_2: u32 = 2164261891u32;
pub const DIKEYBOARD_3: u32 = 2164261892u32;
pub const DIKEYBOARD_4: u32 = 2164261893u32;
pub const DIKEYBOARD_5: u32 = 2164261894u32;
pub const DIKEYBOARD_6: u32 = 2164261895u32;
pub const DIKEYBOARD_7: u32 = 2164261896u32;
pub const DIKEYBOARD_8: u32 = 2164261897u32;
pub const DIKEYBOARD_9: u32 = 2164261898u32;
pub const DIKEYBOARD_A: u32 = 2164261918u32;
pub const DIKEYBOARD_ABNT_C1: u32 = 2164262003u32;
pub const DIKEYBOARD_ABNT_C2: u32 = 2164262014u32;
pub const DIKEYBOARD_ADD: u32 = 2164261966u32;
pub const DIKEYBOARD_APOSTROPHE: u32 = 2164261928u32;
pub const DIKEYBOARD_APPS: u32 = 2164262109u32;
pub const DIKEYBOARD_AT: u32 = 2164262033u32;
pub const DIKEYBOARD_AX: u32 = 2164262038u32;
pub const DIKEYBOARD_B: u32 = 2164261936u32;
pub const DIKEYBOARD_BACK: u32 = 2164261902u32;
pub const DIKEYBOARD_BACKSLASH: u32 = 2164261931u32;
pub const DIKEYBOARD_C: u32 = 2164261934u32;
pub const DIKEYBOARD_CALCULATOR: u32 = 2164262049u32;
pub const DIKEYBOARD_CAPITAL: u32 = 2164261946u32;
pub const DIKEYBOARD_COLON: u32 = 2164262034u32;
pub const DIKEYBOARD_COMMA: u32 = 2164261939u32;
pub const DIKEYBOARD_CONVERT: u32 = 2164262009u32;
pub const DIKEYBOARD_D: u32 = 2164261920u32;
pub const DIKEYBOARD_DECIMAL: u32 = 2164261971u32;
pub const DIKEYBOARD_DELETE: u32 = 2164262099u32;
pub const DIKEYBOARD_DIVIDE: u32 = 2164262069u32;
pub const DIKEYBOARD_DOWN: u32 = 2164262096u32;
pub const DIKEYBOARD_E: u32 = 2164261906u32;
pub const DIKEYBOARD_END: u32 = 2164262095u32;
pub const DIKEYBOARD_EQUALS: u32 = 2164261901u32;
pub const DIKEYBOARD_ESCAPE: u32 = 2164261889u32;
pub const DIKEYBOARD_F: u32 = 2164261921u32;
pub const DIKEYBOARD_F1: u32 = 2164261947u32;
pub const DIKEYBOARD_F10: u32 = 2164261956u32;
pub const DIKEYBOARD_F11: u32 = 2164261975u32;
pub const DIKEYBOARD_F12: u32 = 2164261976u32;
pub const DIKEYBOARD_F13: u32 = 2164261988u32;
pub const DIKEYBOARD_F14: u32 = 2164261989u32;
pub const DIKEYBOARD_F15: u32 = 2164261990u32;
pub const DIKEYBOARD_F2: u32 = 2164261948u32;
pub const DIKEYBOARD_F3: u32 = 2164261949u32;
pub const DIKEYBOARD_F4: u32 = 2164261950u32;
pub const DIKEYBOARD_F5: u32 = 2164261951u32;
pub const DIKEYBOARD_F6: u32 = 2164261952u32;
pub const DIKEYBOARD_F7: u32 = 2164261953u32;
pub const DIKEYBOARD_F8: u32 = 2164261954u32;
pub const DIKEYBOARD_F9: u32 = 2164261955u32;
pub const DIKEYBOARD_G: u32 = 2164261922u32;
pub const DIKEYBOARD_GRAVE: u32 = 2164261929u32;
pub const DIKEYBOARD_H: u32 = 2164261923u32;
pub const DIKEYBOARD_HOME: u32 = 2164262087u32;
pub const DIKEYBOARD_I: u32 = 2164261911u32;
pub const DIKEYBOARD_INSERT: u32 = 2164262098u32;
pub const DIKEYBOARD_J: u32 = 2164261924u32;
pub const DIKEYBOARD_K: u32 = 2164261925u32;
pub const DIKEYBOARD_KANA: u32 = 2164262000u32;
pub const DIKEYBOARD_KANJI: u32 = 2164262036u32;
pub const DIKEYBOARD_L: u32 = 2164261926u32;
pub const DIKEYBOARD_LBRACKET: u32 = 2164261914u32;
pub const DIKEYBOARD_LCONTROL: u32 = 2164261917u32;
pub const DIKEYBOARD_LEFT: u32 = 2164262091u32;
pub const DIKEYBOARD_LMENU: u32 = 2164261944u32;
pub const DIKEYBOARD_LSHIFT: u32 = 2164261930u32;
pub const DIKEYBOARD_LWIN: u32 = 2164262107u32;
pub const DIKEYBOARD_M: u32 = 2164261938u32;
pub const DIKEYBOARD_MAIL: u32 = 2164262124u32;
pub const DIKEYBOARD_MEDIASELECT: u32 = 2164262125u32;
pub const DIKEYBOARD_MEDIASTOP: u32 = 2164262052u32;
pub const DIKEYBOARD_MINUS: u32 = 2164261900u32;
pub const DIKEYBOARD_MULTIPLY: u32 = 2164261943u32;
pub const DIKEYBOARD_MUTE: u32 = 2164262048u32;
pub const DIKEYBOARD_MYCOMPUTER: u32 = 2164262123u32;
pub const DIKEYBOARD_N: u32 = 2164261937u32;
pub const DIKEYBOARD_NEXT: u32 = 2164262097u32;
pub const DIKEYBOARD_NEXTTRACK: u32 = 2164262041u32;
pub const DIKEYBOARD_NOCONVERT: u32 = 2164262011u32;
pub const DIKEYBOARD_NUMLOCK: u32 = 2164261957u32;
pub const DIKEYBOARD_NUMPAD0: u32 = 2164261970u32;
pub const DIKEYBOARD_NUMPAD1: u32 = 2164261967u32;
pub const DIKEYBOARD_NUMPAD2: u32 = 2164261968u32;
pub const DIKEYBOARD_NUMPAD3: u32 = 2164261969u32;
pub const DIKEYBOARD_NUMPAD4: u32 = 2164261963u32;
pub const DIKEYBOARD_NUMPAD5: u32 = 2164261964u32;
pub const DIKEYBOARD_NUMPAD6: u32 = 2164261965u32;
pub const DIKEYBOARD_NUMPAD7: u32 = 2164261959u32;
pub const DIKEYBOARD_NUMPAD8: u32 = 2164261960u32;
pub const DIKEYBOARD_NUMPAD9: u32 = 2164261961u32;
pub const DIKEYBOARD_NUMPADCOMMA: u32 = 2164262067u32;
pub const DIKEYBOARD_NUMPADENTER: u32 = 2164262044u32;
pub const DIKEYBOARD_NUMPADEQUALS: u32 = 2164262029u32;
pub const DIKEYBOARD_O: u32 = 2164261912u32;
pub const DIKEYBOARD_OEM_102: u32 = 2164261974u32;
pub const DIKEYBOARD_P: u32 = 2164261913u32;
pub const DIKEYBOARD_PAUSE: u32 = 2164262085u32;
pub const DIKEYBOARD_PERIOD: u32 = 2164261940u32;
pub const DIKEYBOARD_PLAYPAUSE: u32 = 2164262050u32;
pub const DIKEYBOARD_POWER: u32 = 2164262110u32;
pub const DIKEYBOARD_PREVTRACK: u32 = 2164262032u32;
pub const DIKEYBOARD_PRIOR: u32 = 2164262089u32;
pub const DIKEYBOARD_Q: u32 = 2164261904u32;
pub const DIKEYBOARD_R: u32 = 2164261907u32;
pub const DIKEYBOARD_RBRACKET: u32 = 2164261915u32;
pub const DIKEYBOARD_RCONTROL: u32 = 2164262045u32;
pub const DIKEYBOARD_RETURN: u32 = 2164261916u32;
pub const DIKEYBOARD_RIGHT: u32 = 2164262093u32;
pub const DIKEYBOARD_RMENU: u32 = 2164262072u32;
pub const DIKEYBOARD_RSHIFT: u32 = 2164261942u32;
pub const DIKEYBOARD_RWIN: u32 = 2164262108u32;
pub const DIKEYBOARD_S: u32 = 2164261919u32;
pub const DIKEYBOARD_SCROLL: u32 = 2164261958u32;
pub const DIKEYBOARD_SEMICOLON: u32 = 2164261927u32;
pub const DIKEYBOARD_SLASH: u32 = 2164261941u32;
pub const DIKEYBOARD_SLEEP: u32 = 2164262111u32;
pub const DIKEYBOARD_SPACE: u32 = 2164261945u32;
pub const DIKEYBOARD_STOP: u32 = 2164262037u32;
pub const DIKEYBOARD_SUBTRACT: u32 = 2164261962u32;
pub const DIKEYBOARD_SYSRQ: u32 = 2164262071u32;
pub const DIKEYBOARD_T: u32 = 2164261908u32;
pub const DIKEYBOARD_TAB: u32 = 2164261903u32;
pub const DIKEYBOARD_U: u32 = 2164261910u32;
pub const DIKEYBOARD_UNDERLINE: u32 = 2164262035u32;
pub const DIKEYBOARD_UNLABELED: u32 = 2164262039u32;
pub const DIKEYBOARD_UP: u32 = 2164262088u32;
pub const DIKEYBOARD_V: u32 = 2164261935u32;
pub const DIKEYBOARD_VOLUMEDOWN: u32 = 2164262062u32;
pub const DIKEYBOARD_VOLUMEUP: u32 = 2164262064u32;
pub const DIKEYBOARD_W: u32 = 2164261905u32;
pub const DIKEYBOARD_WAKE: u32 = 2164262115u32;
pub const DIKEYBOARD_WEBBACK: u32 = 2164262122u32;
pub const DIKEYBOARD_WEBFAVORITES: u32 = 2164262118u32;
pub const DIKEYBOARD_WEBFORWARD: u32 = 2164262121u32;
pub const DIKEYBOARD_WEBHOME: u32 = 2164262066u32;
pub const DIKEYBOARD_WEBREFRESH: u32 = 2164262119u32;
pub const DIKEYBOARD_WEBSEARCH: u32 = 2164262117u32;
pub const DIKEYBOARD_WEBSTOP: u32 = 2164262120u32;
pub const DIKEYBOARD_X: u32 = 2164261933u32;
pub const DIKEYBOARD_Y: u32 = 2164261909u32;
pub const DIKEYBOARD_YEN: u32 = 2164262013u32;
pub const DIKEYBOARD_Z: u32 = 2164261932u32;
pub const DIK_0: u32 = 11u32;
pub const DIK_1: u32 = 2u32;
pub const DIK_2: u32 = 3u32;
pub const DIK_3: u32 = 4u32;
pub const DIK_4: u32 = 5u32;
pub const DIK_5: u32 = 6u32;
pub const DIK_6: u32 = 7u32;
pub const DIK_7: u32 = 8u32;
pub const DIK_8: u32 = 9u32;
pub const DIK_9: u32 = 10u32;
pub const DIK_A: u32 = 30u32;
pub const DIK_ABNT_C1: u32 = 115u32;
pub const DIK_ABNT_C2: u32 = 126u32;
pub const DIK_ADD: u32 = 78u32;
pub const DIK_APOSTROPHE: u32 = 40u32;
pub const DIK_APPS: u32 = 221u32;
pub const DIK_AT: u32 = 145u32;
pub const DIK_AX: u32 = 150u32;
pub const DIK_B: u32 = 48u32;
pub const DIK_BACK: u32 = 14u32;
pub const DIK_BACKSLASH: u32 = 43u32;
pub const DIK_BACKSPACE: u32 = 14u32;
pub const DIK_C: u32 = 46u32;
pub const DIK_CALCULATOR: u32 = 161u32;
pub const DIK_CAPITAL: u32 = 58u32;
pub const DIK_CAPSLOCK: u32 = 58u32;
pub const DIK_CIRCUMFLEX: u32 = 144u32;
pub const DIK_COLON: u32 = 146u32;
pub const DIK_COMMA: u32 = 51u32;
pub const DIK_CONVERT: u32 = 121u32;
pub const DIK_D: u32 = 32u32;
pub const DIK_DECIMAL: u32 = 83u32;
pub const DIK_DELETE: u32 = 211u32;
pub const DIK_DIVIDE: u32 = 181u32;
pub const DIK_DOWN: u32 = 208u32;
pub const DIK_DOWNARROW: u32 = 208u32;
pub const DIK_E: u32 = 18u32;
pub const DIK_END: u32 = 207u32;
pub const DIK_EQUALS: u32 = 13u32;
pub const DIK_ESCAPE: u32 = 1u32;
pub const DIK_F: u32 = 33u32;
pub const DIK_F1: u32 = 59u32;
pub const DIK_F10: u32 = 68u32;
pub const DIK_F11: u32 = 87u32;
pub const DIK_F12: u32 = 88u32;
pub const DIK_F13: u32 = 100u32;
pub const DIK_F14: u32 = 101u32;
pub const DIK_F15: u32 = 102u32;
pub const DIK_F2: u32 = 60u32;
pub const DIK_F3: u32 = 61u32;
pub const DIK_F4: u32 = 62u32;
pub const DIK_F5: u32 = 63u32;
pub const DIK_F6: u32 = 64u32;
pub const DIK_F7: u32 = 65u32;
pub const DIK_F8: u32 = 66u32;
pub const DIK_F9: u32 = 67u32;
pub const DIK_G: u32 = 34u32;
pub const DIK_GRAVE: u32 = 41u32;
pub const DIK_H: u32 = 35u32;
pub const DIK_HOME: u32 = 199u32;
pub const DIK_I: u32 = 23u32;
pub const DIK_INSERT: u32 = 210u32;
pub const DIK_J: u32 = 36u32;
pub const DIK_K: u32 = 37u32;
pub const DIK_KANA: u32 = 112u32;
pub const DIK_KANJI: u32 = 148u32;
pub const DIK_L: u32 = 38u32;
pub const DIK_LALT: u32 = 56u32;
pub const DIK_LBRACKET: u32 = 26u32;
pub const DIK_LCONTROL: u32 = 29u32;
pub const DIK_LEFT: u32 = 203u32;
pub const DIK_LEFTARROW: u32 = 203u32;
pub const DIK_LMENU: u32 = 56u32;
pub const DIK_LSHIFT: u32 = 42u32;
pub const DIK_LWIN: u32 = 219u32;
pub const DIK_M: u32 = 50u32;
pub const DIK_MAIL: u32 = 236u32;
pub const DIK_MEDIASELECT: u32 = 237u32;
pub const DIK_MEDIASTOP: u32 = 164u32;
pub const DIK_MINUS: u32 = 12u32;
pub const DIK_MULTIPLY: u32 = 55u32;
pub const DIK_MUTE: u32 = 160u32;
pub const DIK_MYCOMPUTER: u32 = 235u32;
pub const DIK_N: u32 = 49u32;
pub const DIK_NEXT: u32 = 209u32;
pub const DIK_NEXTTRACK: u32 = 153u32;
pub const DIK_NOCONVERT: u32 = 123u32;
pub const DIK_NUMLOCK: u32 = 69u32;
pub const DIK_NUMPAD0: u32 = 82u32;
pub const DIK_NUMPAD1: u32 = 79u32;
pub const DIK_NUMPAD2: u32 = 80u32;
pub const DIK_NUMPAD3: u32 = 81u32;
pub const DIK_NUMPAD4: u32 = 75u32;
pub const DIK_NUMPAD5: u32 = 76u32;
pub const DIK_NUMPAD6: u32 = 77u32;
pub const DIK_NUMPAD7: u32 = 71u32;
pub const DIK_NUMPAD8: u32 = 72u32;
pub const DIK_NUMPAD9: u32 = 73u32;
pub const DIK_NUMPADCOMMA: u32 = 179u32;
pub const DIK_NUMPADENTER: u32 = 156u32;
pub const DIK_NUMPADEQUALS: u32 = 141u32;
pub const DIK_NUMPADMINUS: u32 = 74u32;
pub const DIK_NUMPADPERIOD: u32 = 83u32;
pub const DIK_NUMPADPLUS: u32 = 78u32;
pub const DIK_NUMPADSLASH: u32 = 181u32;
pub const DIK_NUMPADSTAR: u32 = 55u32;
pub const DIK_O: u32 = 24u32;
pub const DIK_OEM_102: u32 = 86u32;
pub const DIK_P: u32 = 25u32;
pub const DIK_PAUSE: u32 = 197u32;
pub const DIK_PERIOD: u32 = 52u32;
pub const DIK_PGDN: u32 = 209u32;
pub const DIK_PGUP: u32 = 201u32;
pub const DIK_PLAYPAUSE: u32 = 162u32;
pub const DIK_POWER: u32 = 222u32;
pub const DIK_PREVTRACK: u32 = 144u32;
pub const DIK_PRIOR: u32 = 201u32;
pub const DIK_Q: u32 = 16u32;
pub const DIK_R: u32 = 19u32;
pub const DIK_RALT: u32 = 184u32;
pub const DIK_RBRACKET: u32 = 27u32;
pub const DIK_RCONTROL: u32 = 157u32;
pub const DIK_RETURN: u32 = 28u32;
pub const DIK_RIGHT: u32 = 205u32;
pub const DIK_RIGHTARROW: u32 = 205u32;
pub const DIK_RMENU: u32 = 184u32;
pub const DIK_RSHIFT: u32 = 54u32;
pub const DIK_RWIN: u32 = 220u32;
pub const DIK_S: u32 = 31u32;
pub const DIK_SCROLL: u32 = 70u32;
pub const DIK_SEMICOLON: u32 = 39u32;
pub const DIK_SLASH: u32 = 53u32;
pub const DIK_SLEEP: u32 = 223u32;
pub const DIK_SPACE: u32 = 57u32;
pub const DIK_STOP: u32 = 149u32;
pub const DIK_SUBTRACT: u32 = 74u32;
pub const DIK_SYSRQ: u32 = 183u32;
pub const DIK_T: u32 = 20u32;
pub const DIK_TAB: u32 = 15u32;
pub const DIK_U: u32 = 22u32;
pub const DIK_UNDERLINE: u32 = 147u32;
pub const DIK_UNLABELED: u32 = 151u32;
pub const DIK_UP: u32 = 200u32;
pub const DIK_UPARROW: u32 = 200u32;
pub const DIK_V: u32 = 47u32;
pub const DIK_VOLUMEDOWN: u32 = 174u32;
pub const DIK_VOLUMEUP: u32 = 176u32;
pub const DIK_W: u32 = 17u32;
pub const DIK_WAKE: u32 = 227u32;
pub const DIK_WEBBACK: u32 = 234u32;
pub const DIK_WEBFAVORITES: u32 = 230u32;
pub const DIK_WEBFORWARD: u32 = 233u32;
pub const DIK_WEBHOME: u32 = 178u32;
pub const DIK_WEBREFRESH: u32 = 231u32;
pub const DIK_WEBSEARCH: u32 = 229u32;
pub const DIK_WEBSTOP: u32 = 232u32;
pub const DIK_X: u32 = 45u32;
pub const DIK_Y: u32 = 21u32;
pub const DIK_YEN: u32 = 125u32;
pub const DIK_Z: u32 = 44u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIMOUSESTATE {
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub rgbButtons: [u8; 4],
}
impl Default for DIMOUSESTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIMOUSESTATE2 {
    pub lX: i32,
    pub lY: i32,
    pub lZ: i32,
    pub rgbButtons: [u8; 8],
}
impl Default for DIMOUSESTATE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIMSGWP_DX8APPSTART: u32 = 2u32;
pub const DIMSGWP_DX8MAPPERAPPSTART: u32 = 3u32;
pub const DIMSGWP_NEWAPPSTART: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIOBJECTATTRIBUTES {
    pub dwFlags: u32,
    pub wUsagePage: u16,
    pub wUsage: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIOBJECTCALIBRATION {
    pub lMin: i32,
    pub lCenter: i32,
    pub lMax: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIOBJECTDATAFORMAT {
    pub pguid: *const windows_core::GUID,
    pub dwOfs: u32,
    pub dwType: u32,
    pub dwFlags: u32,
}
impl Default for DIOBJECTDATAFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIPERIODIC {
    pub dwMagnitude: u32,
    pub lOffset: i32,
    pub dwPhase: u32,
    pub dwPeriod: u32,
}
pub const DIPH_BYID: u32 = 2u32;
pub const DIPH_BYOFFSET: u32 = 1u32;
pub const DIPH_BYUSAGE: u32 = 3u32;
pub const DIPH_DEVICE: u32 = 0u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIPOVCALIBRATION {
    pub lMin: [i32; 5],
    pub lMax: [i32; 5],
}
impl Default for DIPOVCALIBRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIPOV_ANY_1: u32 = 4278208001u32;
pub const DIPOV_ANY_2: u32 = 4278208002u32;
pub const DIPOV_ANY_3: u32 = 4278208003u32;
pub const DIPOV_ANY_4: u32 = 4278208004u32;
pub const DIPROPAUTOCENTER_OFF: u32 = 0u32;
pub const DIPROPAUTOCENTER_ON: u32 = 1u32;
pub const DIPROPAXISMODE_ABS: u32 = 0u32;
pub const DIPROPAXISMODE_REL: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIPROPCAL {
    pub diph: DIPROPHEADER,
    pub lMin: i32,
    pub lCenter: i32,
    pub lMax: i32,
}
pub const DIPROPCALIBRATIONMODE_COOKED: u32 = 0u32;
pub const DIPROPCALIBRATIONMODE_RAW: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIPROPCALPOV {
    pub diph: DIPROPHEADER,
    pub lMin: [i32; 5],
    pub lMax: [i32; 5],
}
impl Default for DIPROPCALPOV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIPROPCPOINTS {
    pub diph: DIPROPHEADER,
    pub dwCPointsNum: u32,
    pub cp: [CPOINT; 8],
}
impl Default for DIPROPCPOINTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIPROPDWORD {
    pub diph: DIPROPHEADER,
    pub dwData: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIPROPGUIDANDPATH {
    pub diph: DIPROPHEADER,
    pub guidClass: windows_core::GUID,
    pub wszPath: [u16; 260],
}
impl Default for DIPROPGUIDANDPATH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIPROPHEADER {
    pub dwSize: u32,
    pub dwHeaderSize: u32,
    pub dwObj: u32,
    pub dwHow: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIPROPPOINTER {
    pub diph: DIPROPHEADER,
    pub uData: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIPROPRANGE {
    pub diph: DIPROPHEADER,
    pub lMin: i32,
    pub lMax: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DIPROPSTRING {
    pub diph: DIPROPHEADER,
    pub wsz: [u16; 260],
}
impl Default for DIPROPSTRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIPROP_APPDATA: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000016);
pub const DIPROP_AUTOCENTER: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000009);
pub const DIPROP_AXISMODE: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000002);
pub const DIPROP_BUFFERSIZE: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000001);
pub const DIPROP_CALIBRATION: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000b);
pub const DIPROP_CALIBRATIONMODE: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000a);
pub const DIPROP_CPOINTS: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000015);
pub const DIPROP_DEADZONE: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000005);
pub const DIPROP_FFGAIN: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000007);
pub const DIPROP_FFLOAD: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000008);
pub const DIPROP_GETPORTDISPLAYNAME: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000010);
pub const DIPROP_GRANULARITY: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000003);
pub const DIPROP_GUIDANDPATH: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000c);
pub const DIPROP_INSTANCENAME: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000d);
pub const DIPROP_JOYSTICKID: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000f);
pub const DIPROP_KEYNAME: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000014);
pub const DIPROP_LOGICALRANGE: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000013);
pub const DIPROP_PHYSICALRANGE: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000012);
pub const DIPROP_PRODUCTNAME: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000000e);
pub const DIPROP_RANGE: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000004);
pub const DIPROP_SATURATION: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000006);
pub const DIPROP_SCANCODE: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000017);
pub const DIPROP_TYPENAME: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_00000000001a);
pub const DIPROP_USERNAME: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000019);
pub const DIPROP_VIDPID: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000018);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIRAMPFORCE {
    pub lStart: i32,
    pub lEnd: i32,
}
pub const DIRECTINPUT_HEADER_VERSION: u32 = 2048u32;
pub const DIRECTINPUT_NOTIFICATION_MSGSTRING: windows_core::PCWSTR = windows_core::w!("DIRECTINPUT_NOTIFICATION_MSGSTRING");
pub const DIRECTINPUT_NOTIFICATION_MSGSTRINGA: windows_core::PCSTR = windows_core::s!("DIRECTINPUT_NOTIFICATION_MSGSTRING");
pub const DIRECTINPUT_NOTIFICATION_MSGSTRINGW: windows_core::PCWSTR = windows_core::w!("DIRECTINPUT_NOTIFICATION_MSGSTRING");
pub const DIRECTINPUT_REGSTR_KEY_LASTAPP: windows_core::PCWSTR = windows_core::w!("MostRecentApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTAPPA: windows_core::PCSTR = windows_core::s!("MostRecentApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTAPPW: windows_core::PCWSTR = windows_core::w!("MostRecentApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTMAPAPP: windows_core::PCWSTR = windows_core::w!("MostRecentMapperApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTMAPAPPA: windows_core::PCSTR = windows_core::s!("MostRecentMapperApplication");
pub const DIRECTINPUT_REGSTR_KEY_LASTMAPAPPW: windows_core::PCWSTR = windows_core::w!("MostRecentMapperApplication");
pub const DIRECTINPUT_REGSTR_VAL_APPIDFLAG: windows_core::PCWSTR = windows_core::w!("AppIdFlag");
pub const DIRECTINPUT_REGSTR_VAL_APPIDFLAGA: windows_core::PCSTR = windows_core::s!("AppIdFlag");
pub const DIRECTINPUT_REGSTR_VAL_APPIDFLAGW: windows_core::PCWSTR = windows_core::w!("AppIdFlag");
pub const DIRECTINPUT_REGSTR_VAL_ID: windows_core::PCWSTR = windows_core::w!("Id");
pub const DIRECTINPUT_REGSTR_VAL_IDA: windows_core::PCSTR = windows_core::s!("Id");
pub const DIRECTINPUT_REGSTR_VAL_IDW: windows_core::PCWSTR = windows_core::w!("Id");
pub const DIRECTINPUT_REGSTR_VAL_LASTSTART: windows_core::PCWSTR = windows_core::w!("MostRecentStart");
pub const DIRECTINPUT_REGSTR_VAL_LASTSTARTA: windows_core::PCSTR = windows_core::s!("MostRecentStart");
pub const DIRECTINPUT_REGSTR_VAL_LASTSTARTW: windows_core::PCWSTR = windows_core::w!("MostRecentStart");
pub const DIRECTINPUT_REGSTR_VAL_MAPPER: windows_core::PCWSTR = windows_core::w!("UsesMapper");
pub const DIRECTINPUT_REGSTR_VAL_MAPPERA: windows_core::PCSTR = windows_core::s!("UsesMapper");
pub const DIRECTINPUT_REGSTR_VAL_MAPPERW: windows_core::PCWSTR = windows_core::w!("UsesMapper");
pub const DIRECTINPUT_REGSTR_VAL_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const DIRECTINPUT_REGSTR_VAL_NAMEA: windows_core::PCSTR = windows_core::s!("Name");
pub const DIRECTINPUT_REGSTR_VAL_NAMEW: windows_core::PCWSTR = windows_core::w!("Name");
pub const DIRECTINPUT_REGSTR_VAL_VERSION: windows_core::PCWSTR = windows_core::w!("Version");
pub const DIRECTINPUT_REGSTR_VAL_VERSIONA: windows_core::PCSTR = windows_core::s!("Version");
pub const DIRECTINPUT_REGSTR_VAL_VERSIONW: windows_core::PCWSTR = windows_core::w!("Version");
pub const DIRECTINPUT_VERSION: u32 = 2048u32;
pub const DISCL_BACKGROUND: u32 = 8u32;
pub const DISCL_EXCLUSIVE: u32 = 1u32;
pub const DISCL_FOREGROUND: u32 = 4u32;
pub const DISCL_NONEXCLUSIVE: u32 = 2u32;
pub const DISCL_NOWINKEY: u32 = 16u32;
pub const DISDD_CONTINUE: u32 = 1u32;
pub const DISFFC_CONTINUE: u32 = 8u32;
pub const DISFFC_PAUSE: u32 = 4u32;
pub const DISFFC_RESET: u32 = 1u32;
pub const DISFFC_SETACTUATORSOFF: u32 = 32u32;
pub const DISFFC_SETACTUATORSON: u32 = 16u32;
pub const DISFFC_STOPALL: u32 = 2u32;
pub const DITC_CALLOUT: u32 = 8u32;
pub const DITC_CLSIDCONFIG: u32 = 2u32;
pub const DITC_DISPLAYNAME: u32 = 4u32;
pub const DITC_FLAGS1: u32 = 32u32;
pub const DITC_FLAGS2: u32 = 64u32;
pub const DITC_HARDWAREID: u32 = 16u32;
pub const DITC_MAPFILE: u32 = 128u32;
pub const DITC_REGHWSETTINGS: u32 = 1u32;
pub const DIVIRTUAL_ARCADE_PLATFORM: u32 = 570425344u32;
pub const DIVIRTUAL_ARCADE_SIDE2SIDE: u32 = 553648128u32;
pub const DIVIRTUAL_BROWSER_CONTROL: u32 = 671088640u32;
pub const DIVIRTUAL_CAD_2DCONTROL: u32 = 587202560u32;
pub const DIVIRTUAL_CAD_3DCONTROL: u32 = 603979776u32;
pub const DIVIRTUAL_CAD_FLYBY: u32 = 620756992u32;
pub const DIVIRTUAL_CAD_MODEL: u32 = 637534208u32;
pub const DIVIRTUAL_DRIVING_COMBAT: u32 = 33554432u32;
pub const DIVIRTUAL_DRIVING_MECHA: u32 = 687865856u32;
pub const DIVIRTUAL_DRIVING_RACE: u32 = 16777216u32;
pub const DIVIRTUAL_DRIVING_TANK: u32 = 50331648u32;
pub const DIVIRTUAL_FIGHTING_FPS: u32 = 150994944u32;
pub const DIVIRTUAL_FIGHTING_HAND2HAND: u32 = 134217728u32;
pub const DIVIRTUAL_FIGHTING_THIRDPERSON: u32 = 167772160u32;
pub const DIVIRTUAL_FLYING_CIVILIAN: u32 = 67108864u32;
pub const DIVIRTUAL_FLYING_HELICOPTER: u32 = 100663296u32;
pub const DIVIRTUAL_FLYING_MILITARY: u32 = 83886080u32;
pub const DIVIRTUAL_REMOTE_CONTROL: u32 = 654311424u32;
pub const DIVIRTUAL_SPACESIM: u32 = 117440512u32;
pub const DIVIRTUAL_SPORTS_BASEBALL_BAT: u32 = 251658240u32;
pub const DIVIRTUAL_SPORTS_BASEBALL_FIELD: u32 = 285212672u32;
pub const DIVIRTUAL_SPORTS_BASEBALL_PITCH: u32 = 268435456u32;
pub const DIVIRTUAL_SPORTS_BASKETBALL_DEFENSE: u32 = 318767104u32;
pub const DIVIRTUAL_SPORTS_BASKETBALL_OFFENSE: u32 = 301989888u32;
pub const DIVIRTUAL_SPORTS_BIKING_MOUNTAIN: u32 = 469762048u32;
pub const DIVIRTUAL_SPORTS_FISHING: u32 = 234881024u32;
pub const DIVIRTUAL_SPORTS_FOOTBALL_DEFENSE: u32 = 385875968u32;
pub const DIVIRTUAL_SPORTS_FOOTBALL_FIELD: u32 = 335544320u32;
pub const DIVIRTUAL_SPORTS_FOOTBALL_OFFENSE: u32 = 369098752u32;
pub const DIVIRTUAL_SPORTS_FOOTBALL_QBCK: u32 = 352321536u32;
pub const DIVIRTUAL_SPORTS_GOLF: u32 = 402653184u32;
pub const DIVIRTUAL_SPORTS_HOCKEY_DEFENSE: u32 = 436207616u32;
pub const DIVIRTUAL_SPORTS_HOCKEY_GOALIE: u32 = 452984832u32;
pub const DIVIRTUAL_SPORTS_HOCKEY_OFFENSE: u32 = 419430400u32;
pub const DIVIRTUAL_SPORTS_HUNTING: u32 = 218103808u32;
pub const DIVIRTUAL_SPORTS_RACQUET: u32 = 536870912u32;
pub const DIVIRTUAL_SPORTS_SKIING: u32 = 486539264u32;
pub const DIVIRTUAL_SPORTS_SOCCER_DEFENSE: u32 = 520093696u32;
pub const DIVIRTUAL_SPORTS_SOCCER_OFFENSE: u32 = 503316480u32;
pub const DIVIRTUAL_STRATEGY_ROLEPLAYING: u32 = 184549376u32;
pub const DIVIRTUAL_STRATEGY_TURN: u32 = 201326592u32;
pub const DIVOICE_ALL: u32 = 2197816330u32;
pub const DIVOICE_CHANNEL1: u32 = 2197816321u32;
pub const DIVOICE_CHANNEL2: u32 = 2197816322u32;
pub const DIVOICE_CHANNEL3: u32 = 2197816323u32;
pub const DIVOICE_CHANNEL4: u32 = 2197816324u32;
pub const DIVOICE_CHANNEL5: u32 = 2197816325u32;
pub const DIVOICE_CHANNEL6: u32 = 2197816326u32;
pub const DIVOICE_CHANNEL7: u32 = 2197816327u32;
pub const DIVOICE_CHANNEL8: u32 = 2197816328u32;
pub const DIVOICE_PLAYBACKMUTE: u32 = 2197816332u32;
pub const DIVOICE_RECORDMUTE: u32 = 2197816331u32;
pub const DIVOICE_TEAM: u32 = 2197816329u32;
pub const DIVOICE_TRANSMIT: u32 = 2197816333u32;
pub const DIVOICE_VOICECOMMAND: u32 = 2197816336u32;
pub const DI_BUFFEROVERFLOW: i32 = 1i32;
pub const DI_DEGREES: u32 = 100u32;
pub const DI_DOWNLOADSKIPPED: windows_core::HRESULT = windows_core::HRESULT(0x3_u32 as _);
pub const DI_EFFECTRESTARTED: windows_core::HRESULT = windows_core::HRESULT(0x4_u32 as _);
pub const DI_FFNOMINALMAX: u32 = 10000u32;
pub const DI_NOEFFECT: i32 = 1i32;
pub const DI_NOTATTACHED: i32 = 1i32;
pub const DI_OK: i32 = 0i32;
pub const DI_POLLEDDEVICE: windows_core::HRESULT = windows_core::HRESULT(0x2_u32 as _);
pub const DI_PROPNOEFFECT: i32 = 1i32;
pub const DI_SECONDS: u32 = 1000000u32;
pub const DI_SETTINGSNOTSAVED: windows_core::HRESULT = windows_core::HRESULT(0xB_u32 as _);
pub const DI_TRUNCATED: windows_core::HRESULT = windows_core::HRESULT(0x8_u32 as _);
pub const DI_TRUNCATEDANDRESTARTED: windows_core::HRESULT = windows_core::HRESULT(0xC_u32 as _);
pub const DI_WRITEPROTECT: windows_core::HRESULT = windows_core::HRESULT(0x13_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPIOBUTTONS_BUTTON_TYPE(pub i32);
pub const GPIO_BUTTON_BACK: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(5i32);
pub const GPIO_BUTTON_CAMERA_FOCUS: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(7i32);
pub const GPIO_BUTTON_CAMERA_LENS: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(12i32);
pub const GPIO_BUTTON_CAMERA_SHUTTER: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(8i32);
pub const GPIO_BUTTON_COUNT: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(16i32);
pub const GPIO_BUTTON_COUNT_MIN: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(5i32);
pub const GPIO_BUTTON_HEADSET: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(10i32);
pub const GPIO_BUTTON_HWKB_DEPLOY: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(11i32);
pub const GPIO_BUTTON_OEM_CUSTOM: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(13i32);
pub const GPIO_BUTTON_OEM_CUSTOM2: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(14i32);
pub const GPIO_BUTTON_OEM_CUSTOM3: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(15i32);
pub const GPIO_BUTTON_POWER: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(0i32);
pub const GPIO_BUTTON_RINGER_TOGGLE: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(9i32);
pub const GPIO_BUTTON_ROTATION_LOCK: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(4i32);
pub const GPIO_BUTTON_SEARCH: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(6i32);
pub const GPIO_BUTTON_VOLUME_DOWN: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(3i32);
pub const GPIO_BUTTON_VOLUME_UP: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(2i32);
pub const GPIO_BUTTON_WINDOWS: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(1i32);
pub const GUID_Button: windows_core::GUID = windows_core::GUID::from_u128(0xa36d02f0_c9f3_11cf_bfc7_444553540000);
pub const GUID_ConstantForce: windows_core::GUID = windows_core::GUID::from_u128(0x13541c20_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_CustomForce: windows_core::GUID = windows_core::GUID::from_u128(0x13541c2b_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_DEVINTERFACE_HID: windows_core::GUID = windows_core::GUID::from_u128(0x4d1e55b2_f16f_11cf_88cb_001111000030);
pub const GUID_DEVINTERFACE_KEYBOARD: windows_core::GUID = windows_core::GUID::from_u128(0x884b96c3_56ef_11d1_bc8c_00a0c91405dd);
pub const GUID_DEVINTERFACE_MOUSE: windows_core::GUID = windows_core::GUID::from_u128(0x378de44c_56ef_11d1_bc8c_00a0c91405dd);
pub const GUID_Damper: windows_core::GUID = windows_core::GUID::from_u128(0x13541c28_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Friction: windows_core::GUID = windows_core::GUID::from_u128(0x13541c2a_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_HIDClass: windows_core::GUID = windows_core::GUID::from_u128(0x745a17a0_74d3_11d0_b6fe_00a0c90f57da);
pub const GUID_HID_INTERFACE_HIDPARSE: windows_core::GUID = windows_core::GUID::from_u128(0xf5c315a5_69ac_4bc2_9279_d0b64576f44b);
pub const GUID_HID_INTERFACE_NOTIFY: windows_core::GUID = windows_core::GUID::from_u128(0x2c4e2e88_25e6_4c33_882f_3d82e6073681);
pub const GUID_Inertia: windows_core::GUID = windows_core::GUID::from_u128(0x13541c29_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Joystick: windows_core::GUID = windows_core::GUID::from_u128(0x6f1d2b70_d5a0_11cf_bfc7_444553540000);
pub const GUID_Key: windows_core::GUID = windows_core::GUID::from_u128(0x55728220_d33c_11cf_bfc7_444553540000);
pub const GUID_KeyboardClass: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e96b_e325_11ce_bfc1_08002be10318);
pub const GUID_MediaClass: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e96c_e325_11ce_bfc1_08002be10318);
pub const GUID_MouseClass: windows_core::GUID = windows_core::GUID::from_u128(0x4d36e96f_e325_11ce_bfc1_08002be10318);
pub const GUID_POV: windows_core::GUID = windows_core::GUID::from_u128(0xa36d02f2_c9f3_11cf_bfc7_444553540000);
pub const GUID_RampForce: windows_core::GUID = windows_core::GUID::from_u128(0x13541c21_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_RxAxis: windows_core::GUID = windows_core::GUID::from_u128(0xa36d02f4_c9f3_11cf_bfc7_444553540000);
pub const GUID_RyAxis: windows_core::GUID = windows_core::GUID::from_u128(0xa36d02f5_c9f3_11cf_bfc7_444553540000);
pub const GUID_RzAxis: windows_core::GUID = windows_core::GUID::from_u128(0xa36d02e3_c9f3_11cf_bfc7_444553540000);
pub const GUID_SawtoothDown: windows_core::GUID = windows_core::GUID::from_u128(0x13541c26_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_SawtoothUp: windows_core::GUID = windows_core::GUID::from_u128(0x13541c25_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Sine: windows_core::GUID = windows_core::GUID::from_u128(0x13541c23_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Slider: windows_core::GUID = windows_core::GUID::from_u128(0xa36d02e4_c9f3_11cf_bfc7_444553540000);
pub const GUID_Spring: windows_core::GUID = windows_core::GUID::from_u128(0x13541c27_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Square: windows_core::GUID = windows_core::GUID::from_u128(0x13541c22_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_SysKeyboard: windows_core::GUID = windows_core::GUID::from_u128(0x6f1d2b61_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysKeyboardEm: windows_core::GUID = windows_core::GUID::from_u128(0x6f1d2b82_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysKeyboardEm2: windows_core::GUID = windows_core::GUID::from_u128(0x6f1d2b83_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysMouse: windows_core::GUID = windows_core::GUID::from_u128(0x6f1d2b60_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysMouseEm: windows_core::GUID = windows_core::GUID::from_u128(0x6f1d2b80_d5a0_11cf_bfc7_444553540000);
pub const GUID_SysMouseEm2: windows_core::GUID = windows_core::GUID::from_u128(0x6f1d2b81_d5a0_11cf_bfc7_444553540000);
pub const GUID_Triangle: windows_core::GUID = windows_core::GUID::from_u128(0x13541c24_8e33_11d0_9ad0_00a0c9a06e35);
pub const GUID_Unknown: windows_core::GUID = windows_core::GUID::from_u128(0xa36d02f3_c9f3_11cf_bfc7_444553540000);
pub const GUID_XAxis: windows_core::GUID = windows_core::GUID::from_u128(0xa36d02e0_c9f3_11cf_bfc7_444553540000);
pub const GUID_YAxis: windows_core::GUID = windows_core::GUID::from_u128(0xa36d02e1_c9f3_11cf_bfc7_444553540000);
pub const GUID_ZAxis: windows_core::GUID = windows_core::GUID::from_u128(0xa36d02e2_c9f3_11cf_bfc7_444553540000);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDD_ATTRIBUTES {
    pub Size: u32,
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct HIDD_CONFIGURATION {
    pub cookie: *mut core::ffi::c_void,
    pub size: u32,
    pub RingBufferSize: u32,
}
impl Default for HIDD_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDP_BUTTON_ARRAY_DATA {
    pub ArrayIndex: u16,
    pub On: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HIDP_BUTTON_CAPS {
    pub UsagePage: u16,
    pub ReportID: u8,
    pub IsAlias: bool,
    pub BitField: u16,
    pub LinkCollection: u16,
    pub LinkUsage: u16,
    pub LinkUsagePage: u16,
    pub IsRange: bool,
    pub IsStringRange: bool,
    pub IsDesignatorRange: bool,
    pub IsAbsolute: bool,
    pub ReportCount: u16,
    pub Reserved2: u16,
    pub Reserved: [u32; 9],
    pub Anonymous: HIDP_BUTTON_CAPS_0,
}
impl Default for HIDP_BUTTON_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HIDP_BUTTON_CAPS_0 {
    pub Range: HIDP_BUTTON_CAPS_0_0,
    pub NotRange: HIDP_BUTTON_CAPS_0_1,
}
impl Default for HIDP_BUTTON_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDP_BUTTON_CAPS_0_1 {
    pub Usage: u16,
    pub Reserved1: u16,
    pub StringIndex: u16,
    pub Reserved2: u16,
    pub DesignatorIndex: u16,
    pub Reserved3: u16,
    pub DataIndex: u16,
    pub Reserved4: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDP_BUTTON_CAPS_0_0 {
    pub UsageMin: u16,
    pub UsageMax: u16,
    pub StringMin: u16,
    pub StringMax: u16,
    pub DesignatorMin: u16,
    pub DesignatorMax: u16,
    pub DataIndexMin: u16,
    pub DataIndexMax: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HIDP_CAPS {
    pub Usage: u16,
    pub UsagePage: u16,
    pub InputReportByteLength: u16,
    pub OutputReportByteLength: u16,
    pub FeatureReportByteLength: u16,
    pub Reserved: [u16; 17],
    pub NumberLinkCollectionNodes: u16,
    pub NumberInputButtonCaps: u16,
    pub NumberInputValueCaps: u16,
    pub NumberInputDataIndices: u16,
    pub NumberOutputButtonCaps: u16,
    pub NumberOutputValueCaps: u16,
    pub NumberOutputDataIndices: u16,
    pub NumberFeatureButtonCaps: u16,
    pub NumberFeatureValueCaps: u16,
    pub NumberFeatureDataIndices: u16,
}
impl Default for HIDP_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HIDP_DATA {
    pub DataIndex: u16,
    pub Reserved: u16,
    pub Anonymous: HIDP_DATA_0,
}
impl Default for HIDP_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HIDP_DATA_0 {
    pub RawValue: u32,
    pub On: bool,
}
impl Default for HIDP_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct HIDP_EXTENDED_ATTRIBUTES {
    pub NumGlobalUnknowns: u8,
    pub Reserved: [u8; 3],
    pub GlobalUnknowns: *mut HIDP_UNKNOWN_TOKEN,
    pub Data: [u32; 1],
}
impl Default for HIDP_EXTENDED_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HIDP_KEYBOARD_DIRECTION(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HIDP_KEYBOARD_MODIFIER_STATE {
    pub Anonymous: HIDP_KEYBOARD_MODIFIER_STATE_0,
}
impl Default for HIDP_KEYBOARD_MODIFIER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HIDP_KEYBOARD_MODIFIER_STATE_0 {
    pub Anonymous: HIDP_KEYBOARD_MODIFIER_STATE_0_0,
    pub ul: u32,
}
impl Default for HIDP_KEYBOARD_MODIFIER_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDP_KEYBOARD_MODIFIER_STATE_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct HIDP_LINK_COLLECTION_NODE {
    pub LinkUsage: u16,
    pub LinkUsagePage: u16,
    pub Parent: u16,
    pub NumberOfChildren: u16,
    pub NextSibling: u16,
    pub FirstChild: u16,
    pub _bitfield: u32,
    pub UserContext: *mut core::ffi::c_void,
}
impl Default for HIDP_LINK_COLLECTION_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HIDP_REPORT_TYPE(pub i32);
pub const HIDP_STATUS_BAD_LOG_PHY_VALUES: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0110006_u32 as _);
pub const HIDP_STATUS_BUFFER_TOO_SMALL: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0110007_u32 as _);
pub const HIDP_STATUS_BUTTON_NOT_PRESSED: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC011000F_u32 as _);
pub const HIDP_STATUS_DATA_INDEX_NOT_FOUND: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC011000D_u32 as _);
pub const HIDP_STATUS_DATA_INDEX_OUT_OF_RANGE: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC011000E_u32 as _);
pub const HIDP_STATUS_I8042_TRANS_UNKNOWN: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0110009_u32 as _);
pub const HIDP_STATUS_I8242_TRANS_UNKNOWN: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0110009_u32 as _);
pub const HIDP_STATUS_INCOMPATIBLE_REPORT_ID: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC011000A_u32 as _);
pub const HIDP_STATUS_INTERNAL_ERROR: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0110008_u32 as _);
pub const HIDP_STATUS_INVALID_PREPARSED_DATA: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0110001_u32 as _);
pub const HIDP_STATUS_INVALID_REPORT_LENGTH: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0110003_u32 as _);
pub const HIDP_STATUS_INVALID_REPORT_TYPE: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0110002_u32 as _);
pub const HIDP_STATUS_IS_VALUE_ARRAY: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC011000C_u32 as _);
pub const HIDP_STATUS_NOT_BUTTON_ARRAY: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0110021_u32 as _);
pub const HIDP_STATUS_NOT_IMPLEMENTED: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0110020_u32 as _);
pub const HIDP_STATUS_NOT_VALUE_ARRAY: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC011000B_u32 as _);
pub const HIDP_STATUS_NULL: windows_core::NTSTATUS = windows_core::NTSTATUS(0x80110001_u32 as _);
pub const HIDP_STATUS_REPORT_DOES_NOT_EXIST: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0110010_u32 as _);
pub const HIDP_STATUS_SUCCESS: windows_core::NTSTATUS = windows_core::NTSTATUS(0x110000_u32 as _);
pub const HIDP_STATUS_USAGE_NOT_FOUND: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0110004_u32 as _);
pub const HIDP_STATUS_VALUE_OUT_OF_RANGE: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0110005_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HIDP_UNKNOWN_TOKEN {
    pub Token: u8,
    pub Reserved: [u8; 3],
    pub BitField: u32,
}
impl Default for HIDP_UNKNOWN_TOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HIDP_VALUE_CAPS {
    pub UsagePage: u16,
    pub ReportID: u8,
    pub IsAlias: bool,
    pub BitField: u16,
    pub LinkCollection: u16,
    pub LinkUsage: u16,
    pub LinkUsagePage: u16,
    pub IsRange: bool,
    pub IsStringRange: bool,
    pub IsDesignatorRange: bool,
    pub IsAbsolute: bool,
    pub HasNull: bool,
    pub Reserved: u8,
    pub BitSize: u16,
    pub ReportCount: u16,
    pub Reserved2: [u16; 5],
    pub UnitsExp: u32,
    pub Units: u32,
    pub LogicalMin: i32,
    pub LogicalMax: i32,
    pub PhysicalMin: i32,
    pub PhysicalMax: i32,
    pub Anonymous: HIDP_VALUE_CAPS_0,
}
impl Default for HIDP_VALUE_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union HIDP_VALUE_CAPS_0 {
    pub Range: HIDP_VALUE_CAPS_0_0,
    pub NotRange: HIDP_VALUE_CAPS_0_1,
}
impl Default for HIDP_VALUE_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDP_VALUE_CAPS_0_1 {
    pub Usage: u16,
    pub Reserved1: u16,
    pub StringIndex: u16,
    pub Reserved2: u16,
    pub DesignatorIndex: u16,
    pub Reserved3: u16,
    pub DataIndex: u16,
    pub Reserved4: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HIDP_VALUE_CAPS_0_0 {
    pub UsageMin: u16,
    pub UsageMax: u16,
    pub StringMin: u16,
    pub StringMax: u16,
    pub DesignatorMin: u16,
    pub DesignatorMax: u16,
    pub DataIndexMin: u16,
    pub DataIndexMax: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HID_COLLECTION_INFORMATION {
    pub DescriptorSize: u32,
    pub Polled: bool,
    pub Reserved1: [u8; 1],
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
}
impl Default for HID_COLLECTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HID_DRIVER_CONFIG {
    pub Size: u32,
    pub RingBufferSize: u32,
}
pub const HID_REVISION: u32 = 1u32;
pub const HID_USAGE_ALPHANUMERIC_14_SEGMENT_DIRECT_MAP: u16 = 69u16;
pub const HID_USAGE_ALPHANUMERIC_7_SEGMENT_DIRECT_MAP: u16 = 67u16;
pub const HID_USAGE_ALPHANUMERIC_ALPHANUMERIC_DISPLAY: u16 = 1u16;
pub const HID_USAGE_ALPHANUMERIC_ASCII_CHARACTER_SET: u16 = 33u16;
pub const HID_USAGE_ALPHANUMERIC_ATTRIBUTE_DATA: u16 = 74u16;
pub const HID_USAGE_ALPHANUMERIC_ATTRIBUTE_READBACK: u16 = 73u16;
pub const HID_USAGE_ALPHANUMERIC_BITMAPPED_DISPLAY: u16 = 2u16;
pub const HID_USAGE_ALPHANUMERIC_BITMAP_SIZE_X: u16 = 128u16;
pub const HID_USAGE_ALPHANUMERIC_BITMAP_SIZE_Y: u16 = 129u16;
pub const HID_USAGE_ALPHANUMERIC_BIT_DEPTH_FORMAT: u16 = 131u16;
pub const HID_USAGE_ALPHANUMERIC_BLIT_DATA: u16 = 143u16;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_X1: u16 = 139u16;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_X2: u16 = 141u16;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_Y1: u16 = 140u16;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_Y2: u16 = 142u16;
pub const HID_USAGE_ALPHANUMERIC_BLIT_REPORT: u16 = 138u16;
pub const HID_USAGE_ALPHANUMERIC_CHARACTER_ATTRIBUTE: u16 = 72u16;
pub const HID_USAGE_ALPHANUMERIC_CHARACTER_MAPPING: u16 = 207u16;
pub const HID_USAGE_ALPHANUMERIC_CHARACTER_PAGE_MAPPING: u16 = 223u16;
pub const HID_USAGE_ALPHANUMERIC_CHARACTER_REPORT: u16 = 43u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_ATTR_BLINK: u16 = 77u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_ATTR_ENHANCE: u16 = 75u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_ATTR_UNDERLINE: u16 = 76u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_HEIGHT: u16 = 62u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_SPACING_HORIZONTAL: u16 = 63u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_SPACING_VERTICAL: u16 = 64u16;
pub const HID_USAGE_ALPHANUMERIC_CHAR_WIDTH: u16 = 61u16;
pub const HID_USAGE_ALPHANUMERIC_CLEAR_DISPLAY: u16 = 37u16;
pub const HID_USAGE_ALPHANUMERIC_COLUMN: u16 = 52u16;
pub const HID_USAGE_ALPHANUMERIC_COLUMNS: u16 = 54u16;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_BLINK: u16 = 58u16;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_ENABLE: u16 = 57u16;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_MODE: u16 = 56u16;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_PIXEL_POSITIONING: u16 = 55u16;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_POSITION_REPORT: u16 = 50u16;
pub const HID_USAGE_ALPHANUMERIC_DATA_READ_BACK: u16 = 34u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_ATTRIBUTES_REPORT: u16 = 32u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_BRIGHTNESS: u16 = 70u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_CONTRAST: u16 = 71u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_CONTROL_REPORT: u16 = 36u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_DATA: u16 = 44u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_DATA_EXTENSIONS: u16 = 204u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_ENABLE: u16 = 38u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_ORIENTATION: u16 = 132u16;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_STATUS: u16 = 45u16;
pub const HID_USAGE_ALPHANUMERIC_ERR_FONT_DATA_CANNOT_BE_READ: u16 = 49u16;
pub const HID_USAGE_ALPHANUMERIC_ERR_NOT_A_LOADABLE_CHARACTER: u16 = 48u16;
pub const HID_USAGE_ALPHANUMERIC_FONT_14_SEGMENT: u16 = 68u16;
pub const HID_USAGE_ALPHANUMERIC_FONT_7_SEGMENT: u16 = 66u16;
pub const HID_USAGE_ALPHANUMERIC_FONT_DATA: u16 = 60u16;
pub const HID_USAGE_ALPHANUMERIC_FONT_READ_BACK: u16 = 35u16;
pub const HID_USAGE_ALPHANUMERIC_FONT_REPORT: u16 = 59u16;
pub const HID_USAGE_ALPHANUMERIC_HORIZONTAL_SCROLL: u16 = 42u16;
pub const HID_USAGE_ALPHANUMERIC_MAX_BLIT_SIZE: u16 = 130u16;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_DATA: u16 = 136u16;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_DATA_OFFSET: u16 = 135u16;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_DATA_SIZE: u16 = 134u16;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_REPORT: u16 = 133u16;
pub const HID_USAGE_ALPHANUMERIC_REQUEST_REPORT: u16 = 255u16;
pub const HID_USAGE_ALPHANUMERIC_ROW: u16 = 51u16;
pub const HID_USAGE_ALPHANUMERIC_ROWS: u16 = 53u16;
pub const HID_USAGE_ALPHANUMERIC_SCREEN_SAVER_DELAY: u16 = 39u16;
pub const HID_USAGE_ALPHANUMERIC_SCREEN_SAVER_ENABLE: u16 = 40u16;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON: u16 = 144u16;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_ID: u16 = 145u16;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_OFFSET1: u16 = 147u16;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_OFFSET2: u16 = 148u16;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_REPORT: u16 = 149u16;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_SIDE: u16 = 146u16;
pub const HID_USAGE_ALPHANUMERIC_SOFT_KEYS: u16 = 194u16;
pub const HID_USAGE_ALPHANUMERIC_STATUS_NOT_READY: u16 = 46u16;
pub const HID_USAGE_ALPHANUMERIC_STATUS_READY: u16 = 47u16;
pub const HID_USAGE_ALPHANUMERIC_UNICODE_CHAR_SET: u16 = 65u16;
pub const HID_USAGE_ALPHANUMERIC_UNICODE_EQUIVALENT: u16 = 221u16;
pub const HID_USAGE_ALPHANUMERIC_VERTICAL_SCROLL: u16 = 41u16;
pub const HID_USAGE_ARCADE_ALARM_INPUT: u16 = 69u16;
pub const HID_USAGE_ARCADE_COIN_DOOR: u16 = 2u16;
pub const HID_USAGE_ARCADE_COIN_DOOR_COUNTER: u16 = 70u16;
pub const HID_USAGE_ARCADE_COIN_DOOR_LOCKOUT: u16 = 64u16;
pub const HID_USAGE_ARCADE_COIN_DOOR_TEST: u16 = 57u16;
pub const HID_USAGE_ARCADE_COIN_DRAWER_DROP_COUNT: u16 = 53u16;
pub const HID_USAGE_ARCADE_COIN_DRAWER_SERVICE: u16 = 55u16;
pub const HID_USAGE_ARCADE_COIN_DRAWER_START: u16 = 54u16;
pub const HID_USAGE_ARCADE_COIN_DRAWER_TILT: u16 = 56u16;
pub const HID_USAGE_ARCADE_EXTENDED_OPTICAL_INPUT_STATE: u16 = 73u16;
pub const HID_USAGE_ARCADE_GENERAL_PURPOSE_ANALOG_INPUT_STATE: u16 = 48u16;
pub const HID_USAGE_ARCADE_GENERAL_PURPOSE_DIGITAL_INPUT_STATE: u16 = 49u16;
pub const HID_USAGE_ARCADE_GENERAL_PURPOSE_DIGITAL_OUTPUT_STATE: u16 = 51u16;
pub const HID_USAGE_ARCADE_GENERAL_PURPOSE_IO_CARD: u16 = 1u16;
pub const HID_USAGE_ARCADE_GENERAL_PURPOSE_OPTICAL_INPUT_STATE: u16 = 50u16;
pub const HID_USAGE_ARCADE_IO_DIRECTION_MAPPING: u16 = 71u16;
pub const HID_USAGE_ARCADE_NUMBER_OF_COIN_DOORS: u16 = 52u16;
pub const HID_USAGE_ARCADE_PIN_PAD_COMMAND: u16 = 77u16;
pub const HID_USAGE_ARCADE_PIN_PAD_INPUT_STATE: u16 = 74u16;
pub const HID_USAGE_ARCADE_PIN_PAD_OUTPUT: u16 = 76u16;
pub const HID_USAGE_ARCADE_PIN_PAD_STATUS: u16 = 75u16;
pub const HID_USAGE_ARCADE_SET_IO_DIRECTION_MAPPING: u16 = 72u16;
pub const HID_USAGE_ARCADE_WATCHDOG_ACTION: u16 = 66u16;
pub const HID_USAGE_ARCADE_WATCHDOG_REBOOT: u16 = 67u16;
pub const HID_USAGE_ARCADE_WATCHDOG_RESTART: u16 = 68u16;
pub const HID_USAGE_ARCADE_WATCHDOG_TIMEOUT: u16 = 65u16;
pub const HID_USAGE_ARCADE_WATCHDOG_TIMER: u16 = 3u16;
pub const HID_USAGE_BARCODE_SCANNER_2D_CONTROL_REPORT: u16 = 31u16;
pub const HID_USAGE_BARCODE_SCANNER_ACTIVE_TIME: u16 = 85u16;
pub const HID_USAGE_BARCODE_SCANNER_ADD_EAN_23_LABEL_DEFINITION: u16 = 191u16;
pub const HID_USAGE_BARCODE_SCANNER_AIMINGPOINTER_MODE: u16 = 48u16;
pub const HID_USAGE_BARCODE_SCANNER_AIMING_LASER_PATTERN: u16 = 86u16;
pub const HID_USAGE_BARCODE_SCANNER_AIM_DURATION: u16 = 122u16;
pub const HID_USAGE_BARCODE_SCANNER_ATTRIBUTE_REPORT: u16 = 16u16;
pub const HID_USAGE_BARCODE_SCANNER_AZTEC_CODE: u16 = 272u16;
pub const HID_USAGE_BARCODE_SCANNER_BARCODE_BADGE_READER: u16 = 1u16;
pub const HID_USAGE_BARCODE_SCANNER_BARCODE_SCANNER: u16 = 2u16;
pub const HID_USAGE_BARCODE_SCANNER_BAR_CODE_PRESENT: u16 = 87u16;
pub const HID_USAGE_BARCODE_SCANNER_BAR_CODE_PRESENT_SENSOR: u16 = 49u16;
pub const HID_USAGE_BARCODE_SCANNER_BAR_CODE_SCANNER_CRADLE: u16 = 5u16;
pub const HID_USAGE_BARCODE_SCANNER_BAR_SPACE_DATA: u16 = 256u16;
pub const HID_USAGE_BARCODE_SCANNER_BC412: u16 = 273u16;
pub const HID_USAGE_BARCODE_SCANNER_BEEPER_STATE: u16 = 88u16;
pub const HID_USAGE_BARCODE_SCANNER_BOOKLAND_EAN: u16 = 145u16;
pub const HID_USAGE_BARCODE_SCANNER_CHANNEL_CODE: u16 = 274u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK: u16 = 176u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT: u16 = 214u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_CODABAR_ENABLE: u16 = 222u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_CODE_39_ENABLE: u16 = 223u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_DISABLE: u16 = 215u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_ENABLE_INTERLEAVED_2_OF_5_OPCC: u16 = 216u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_ENABLE_INTERLEAVED_2_OF_5_USS: u16 = 217u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_ENABLE_ONE_MSI_PLESSEY: u16 = 220u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_ENABLE_STANDARD_2_OF_5_OPCC: u16 = 218u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_ENABLE_STANDARD_2_OF_5_USS: u16 = 219u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DIGIT_ENABLE_TWO_MSI_PLESSEY: u16 = 221u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_DISABLE_PRICE: u16 = 177u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_ENABLE_4_DIGIT_PRICE: u16 = 178u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_ENABLE_5_DIGIT_PRICE: u16 = 179u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_ENABLE_EUROPEAN_4_DIGIT_PRICE: u16 = 180u16;
pub const HID_USAGE_BARCODE_SCANNER_CHECK_ENABLE_EUROPEAN_5_DIGIT_PRICE: u16 = 181u16;
pub const HID_USAGE_BARCODE_SCANNER_CLASS_1A_LASER: u16 = 50u16;
pub const HID_USAGE_BARCODE_SCANNER_CLASS_2_LASER: u16 = 51u16;
pub const HID_USAGE_BARCODE_SCANNER_CLEAR_ALL_EAN_23_LABEL_DEFINITIONS: u16 = 192u16;
pub const HID_USAGE_BARCODE_SCANNER_CODABAR: u16 = 195u16;
pub const HID_USAGE_BARCODE_SCANNER_CODABAR_CONTROL_REPORT: u16 = 28u16;
pub const HID_USAGE_BARCODE_SCANNER_CODE_128: u16 = 196u16;
pub const HID_USAGE_BARCODE_SCANNER_CODE_128_CONTROL_REPORT: u16 = 29u16;
pub const HID_USAGE_BARCODE_SCANNER_CODE_16: u16 = 275u16;
pub const HID_USAGE_BARCODE_SCANNER_CODE_32: u16 = 276u16;
pub const HID_USAGE_BARCODE_SCANNER_CODE_39: u16 = 199u16;
pub const HID_USAGE_BARCODE_SCANNER_CODE_39_CONTROL_REPORT: u16 = 24u16;
pub const HID_USAGE_BARCODE_SCANNER_CODE_49: u16 = 277u16;
pub const HID_USAGE_BARCODE_SCANNER_CODE_93: u16 = 200u16;
pub const HID_USAGE_BARCODE_SCANNER_CODE_ONE: u16 = 278u16;
pub const HID_USAGE_BARCODE_SCANNER_COLORCODE: u16 = 279u16;
pub const HID_USAGE_BARCODE_SCANNER_COMMIT_PARAMETERS_TO_NVM: u16 = 109u16;
pub const HID_USAGE_BARCODE_SCANNER_CONSTANT_ELECTRONIC_ARTICLE_SURVEILLANCE: u16 = 55u16;
pub const HID_USAGE_BARCODE_SCANNER_CONTACT_SCANNER: u16 = 53u16;
pub const HID_USAGE_BARCODE_SCANNER_CONVERT_EAN_8_TO_13_TYPE: u16 = 146u16;
pub const HID_USAGE_BARCODE_SCANNER_CONVERT_UPCE_TO_A: u16 = 148u16;
pub const HID_USAGE_BARCODE_SCANNER_CONVERT_UPC_A_TO_EAN13: u16 = 147u16;
pub const HID_USAGE_BARCODE_SCANNER_CORDLESS_SCANNER_BASE: u16 = 4u16;
pub const HID_USAGE_BARCODE_SCANNER_DATA_LENGTH_METHOD: u16 = 266u16;
pub const HID_USAGE_BARCODE_SCANNER_DATA_MATRIX: u16 = 280u16;
pub const HID_USAGE_BARCODE_SCANNER_DATA_PREFIX: u16 = 79u16;
pub const HID_USAGE_BARCODE_SCANNER_DECODED_DATA: u16 = 254u16;
pub const HID_USAGE_BARCODE_SCANNER_DECODE_DATA_CONTINUED: u16 = 255u16;
pub const HID_USAGE_BARCODE_SCANNER_DISABLE_CHECK_DIGIT_TRANSMIT: u16 = 241u16;
pub const HID_USAGE_BARCODE_SCANNER_DISCRETE_LENGTH_TO_DECODE_1: u16 = 264u16;
pub const HID_USAGE_BARCODE_SCANNER_DISCRETE_LENGTH_TO_DECODE_2: u16 = 265u16;
pub const HID_USAGE_BARCODE_SCANNER_DL_METHOD_CHECK_FOR_DISCRETE: u16 = 269u16;
pub const HID_USAGE_BARCODE_SCANNER_DL_METHOD_CHECK_IN_RANGE: u16 = 268u16;
pub const HID_USAGE_BARCODE_SCANNER_DL_METHOD_READ_ANY: u16 = 267u16;
pub const HID_USAGE_BARCODE_SCANNER_DUMB_BAR_CODE_SCANNER: u16 = 3u16;
pub const HID_USAGE_BARCODE_SCANNER_EAN13: u16 = 149u16;
pub const HID_USAGE_BARCODE_SCANNER_EAN8: u16 = 150u16;
pub const HID_USAGE_BARCODE_SCANNER_EAN99_128_MANDATORY: u16 = 151u16;
pub const HID_USAGE_BARCODE_SCANNER_EAN99_P5128_OPTIONAL: u16 = 152u16;
pub const HID_USAGE_BARCODE_SCANNER_EAN_13_FLAG_DIGIT_1: u16 = 188u16;
pub const HID_USAGE_BARCODE_SCANNER_EAN_13_FLAG_DIGIT_2: u16 = 189u16;
pub const HID_USAGE_BARCODE_SCANNER_EAN_13_FLAG_DIGIT_3: u16 = 190u16;
pub const HID_USAGE_BARCODE_SCANNER_EAN_23_LABEL_CONTROL_REPORT: u16 = 23u16;
pub const HID_USAGE_BARCODE_SCANNER_EAN_8_FLAG_DIGIT_1: u16 = 185u16;
pub const HID_USAGE_BARCODE_SCANNER_EAN_8_FLAG_DIGIT_2: u16 = 186u16;
pub const HID_USAGE_BARCODE_SCANNER_EAN_8_FLAG_DIGIT_3: u16 = 187u16;
pub const HID_USAGE_BARCODE_SCANNER_EAN_THREE_LABEL: u16 = 184u16;
pub const HID_USAGE_BARCODE_SCANNER_EAN_TWO_LABEL: u16 = 183u16;
pub const HID_USAGE_BARCODE_SCANNER_ELECTRONIC_ARTICLE_SURVEILLANCE_NOTIFICATION: u16 = 54u16;
pub const HID_USAGE_BARCODE_SCANNER_ENABLE_CHECK_DIGIT_TRANSMIT: u16 = 242u16;
pub const HID_USAGE_BARCODE_SCANNER_ENABLE_EAN_TWO_LABEL: u16 = 153u16;
pub const HID_USAGE_BARCODE_SCANNER_ERROR_INDICATION: u16 = 56u16;
pub const HID_USAGE_BARCODE_SCANNER_FIXED_BEEPER: u16 = 57u16;
pub const HID_USAGE_BARCODE_SCANNER_FRAGMENT_DECODING: u16 = 77u16;
pub const HID_USAGE_BARCODE_SCANNER_FULL_ASCII_CONVERSION: u16 = 201u16;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_DECODE_INDICATION: u16 = 58u16;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_LAMP_DURATION: u16 = 123u16;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_LAMP_INTENSITY: u16 = 124u16;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_LED: u16 = 125u16;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_TONE_FREQUENCY: u16 = 126u16;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_TONE_LENGTH: u16 = 127u16;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_TONE_VOLUME: u16 = 128u16;
pub const HID_USAGE_BARCODE_SCANNER_GOOD_READ_WHEN_TO_WRITE: u16 = 136u16;
pub const HID_USAGE_BARCODE_SCANNER_GRWTI_AFTER_DECODE: u16 = 137u16;
pub const HID_USAGE_BARCODE_SCANNER_GRWTI_BEEPLAMP_AFTER_TRANSMIT: u16 = 138u16;
pub const HID_USAGE_BARCODE_SCANNER_GRWTI_NO_BEEPLAMP_USE_AT_ALL: u16 = 139u16;
pub const HID_USAGE_BARCODE_SCANNER_HANDS_FREE_SCANNING: u16 = 59u16;
pub const HID_USAGE_BARCODE_SCANNER_HEATER_PRESENT: u16 = 52u16;
pub const HID_USAGE_BARCODE_SCANNER_INITIATE_BARCODE_READ: u16 = 96u16;
pub const HID_USAGE_BARCODE_SCANNER_INTERLEAVED_2_OF_5: u16 = 202u16;
pub const HID_USAGE_BARCODE_SCANNER_INTERLEAVED_2_OF_5_CONTROL_REPORT: u16 = 25u16;
pub const HID_USAGE_BARCODE_SCANNER_INTRINSICALLY_SAFE: u16 = 60u16;
pub const HID_USAGE_BARCODE_SCANNER_ITALIAN_PHARMACY_CODE: u16 = 203u16;
pub const HID_USAGE_BARCODE_SCANNER_KLASSE_EINS_LASER: u16 = 61u16;
pub const HID_USAGE_BARCODE_SCANNER_LASER_ON_TIME: u16 = 89u16;
pub const HID_USAGE_BARCODE_SCANNER_LASER_STATE: u16 = 90u16;
pub const HID_USAGE_BARCODE_SCANNER_LOCKOUT_TIME: u16 = 91u16;
pub const HID_USAGE_BARCODE_SCANNER_LONG_RANGE_SCANNER: u16 = 62u16;
pub const HID_USAGE_BARCODE_SCANNER_MAXICODE: u16 = 281u16;
pub const HID_USAGE_BARCODE_SCANNER_MAXIMUM_LENGTH_TO_DECODE: u16 = 263u16;
pub const HID_USAGE_BARCODE_SCANNER_MICROPDF: u16 = 282u16;
pub const HID_USAGE_BARCODE_SCANNER_MINIMUM_LENGTH_TO_DECODE: u16 = 262u16;
pub const HID_USAGE_BARCODE_SCANNER_MIRROR_SPEED_CONTROL: u16 = 63u16;
pub const HID_USAGE_BARCODE_SCANNER_MISC_1D_CONTROL_REPORT: u16 = 30u16;
pub const HID_USAGE_BARCODE_SCANNER_MOTOR_STATE: u16 = 92u16;
pub const HID_USAGE_BARCODE_SCANNER_MOTOR_TIMEOUT: u16 = 93u16;
pub const HID_USAGE_BARCODE_SCANNER_MSIPLESSEY: u16 = 204u16;
pub const HID_USAGE_BARCODE_SCANNER_MSI_PLESSEY_CONTROL_REPORT: u16 = 27u16;
pub const HID_USAGE_BARCODE_SCANNER_MULTIRANGE_SCANNER: u16 = 69u16;
pub const HID_USAGE_BARCODE_SCANNER_NOT_ON_FILE_INDICATION: u16 = 64u16;
pub const HID_USAGE_BARCODE_SCANNER_NOT_ON_FILE_VOLUME: u16 = 131u16;
pub const HID_USAGE_BARCODE_SCANNER_NO_READ_MESSAGE: u16 = 130u16;
pub const HID_USAGE_BARCODE_SCANNER_PARAMETERS_CHANGED: u16 = 111u16;
pub const HID_USAGE_BARCODE_SCANNER_PARAMETER_SCANNING: u16 = 110u16;
pub const HID_USAGE_BARCODE_SCANNER_PDF417: u16 = 283u16;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL: u16 = 169u16;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL_AUTODISCRIMINATE_2: u16 = 170u16;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL_AUTODISCRIMINATE_5: u16 = 173u16;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL_IGNORE_2: u16 = 172u16;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL_IGNORE_5: u16 = 175u16;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL_ONLY_DECODE_WITH_2: u16 = 171u16;
pub const HID_USAGE_BARCODE_SCANNER_PERIODICAL_ONLY_DECODE_WITH_5: u16 = 174u16;
pub const HID_USAGE_BARCODE_SCANNER_POLARITY_INVERTED_BAR_CODE: u16 = 259u16;
pub const HID_USAGE_BARCODE_SCANNER_POLARITY_NORMAL_BAR_CODE: u16 = 260u16;
pub const HID_USAGE_BARCODE_SCANNER_POSICODE: u16 = 284u16;
pub const HID_USAGE_BARCODE_SCANNER_POWERUP_BEEP: u16 = 132u16;
pub const HID_USAGE_BARCODE_SCANNER_POWER_ON_RESET_SCANNER: u16 = 94u16;
pub const HID_USAGE_BARCODE_SCANNER_PREFIX_AIMI: u16 = 80u16;
pub const HID_USAGE_BARCODE_SCANNER_PREFIX_NONE: u16 = 81u16;
pub const HID_USAGE_BARCODE_SCANNER_PREFIX_PROPRIETARY: u16 = 82u16;
pub const HID_USAGE_BARCODE_SCANNER_PREVENT_READ_OF_BARCODES: u16 = 95u16;
pub const HID_USAGE_BARCODE_SCANNER_PROGRAMMABLE_BEEPER: u16 = 65u16;
pub const HID_USAGE_BARCODE_SCANNER_PROXIMITY_SENSOR: u16 = 70u16;
pub const HID_USAGE_BARCODE_SCANNER_QR_CODE: u16 = 285u16;
pub const HID_USAGE_BARCODE_SCANNER_RAW_DATA_POLARITY: u16 = 258u16;
pub const HID_USAGE_BARCODE_SCANNER_RAW_SCANNED_DATA_REPORT: u16 = 19u16;
pub const HID_USAGE_BARCODE_SCANNER_SCANNED_DATA_REPORT: u16 = 18u16;
pub const HID_USAGE_BARCODE_SCANNER_SCANNER_DATA_ACCURACY: u16 = 257u16;
pub const HID_USAGE_BARCODE_SCANNER_SCANNER_IN_CRADLE: u16 = 117u16;
pub const HID_USAGE_BARCODE_SCANNER_SCANNER_IN_RANGE: u16 = 118u16;
pub const HID_USAGE_BARCODE_SCANNER_SCANNER_READ_CONFIDENCE: u16 = 78u16;
pub const HID_USAGE_BARCODE_SCANNER_SETTINGS_REPORT: u16 = 17u16;
pub const HID_USAGE_BARCODE_SCANNER_SET_PARAMETER_DEFAULT_VALUES: u16 = 112u16;
pub const HID_USAGE_BARCODE_SCANNER_SOUND_ERROR_BEEP: u16 = 133u16;
pub const HID_USAGE_BARCODE_SCANNER_SOUND_GOOD_READ_BEEP: u16 = 134u16;
pub const HID_USAGE_BARCODE_SCANNER_SOUND_NOT_ON_FILE_BEEP: u16 = 135u16;
pub const HID_USAGE_BARCODE_SCANNER_STANDARD_2_OF_5: u16 = 206u16;
pub const HID_USAGE_BARCODE_SCANNER_STANDARD_2_OF_5_CONTROL_REPORT: u16 = 26u16;
pub const HID_USAGE_BARCODE_SCANNER_STANDARD_2_OF_5_IATA: u16 = 205u16;
pub const HID_USAGE_BARCODE_SCANNER_STATUS_REPORT: u16 = 21u16;
pub const HID_USAGE_BARCODE_SCANNER_SUPERCODE: u16 = 286u16;
pub const HID_USAGE_BARCODE_SCANNER_SYMBOLOGY_IDENTIFIER_1: u16 = 251u16;
pub const HID_USAGE_BARCODE_SCANNER_SYMBOLOGY_IDENTIFIER_2: u16 = 252u16;
pub const HID_USAGE_BARCODE_SCANNER_SYMBOLOGY_IDENTIFIER_3: u16 = 253u16;
pub const HID_USAGE_BARCODE_SCANNER_TRANSMIT_CHECK_DIGIT: u16 = 240u16;
pub const HID_USAGE_BARCODE_SCANNER_TRANSMIT_STARTSTOP: u16 = 211u16;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGERLESS: u16 = 66u16;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_MODE: u16 = 98u16;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_MODE_BLINKING_LASER_ON: u16 = 99u16;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_MODE_CONTINUOUS_LASER_ON: u16 = 100u16;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_MODE_LASER_ON_WHILE_PULLED: u16 = 101u16;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_MODE_LASER_STAYS_ON_AFTER_RELEASE: u16 = 102u16;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_REPORT: u16 = 20u16;
pub const HID_USAGE_BARCODE_SCANNER_TRIGGER_STATE: u16 = 97u16;
pub const HID_USAGE_BARCODE_SCANNER_TRIOPTIC: u16 = 212u16;
pub const HID_USAGE_BARCODE_SCANNER_UCCEAN128: u16 = 213u16;
pub const HID_USAGE_BARCODE_SCANNER_ULTRACODE: u16 = 287u16;
pub const HID_USAGE_BARCODE_SCANNER_UPCA: u16 = 157u16;
pub const HID_USAGE_BARCODE_SCANNER_UPCA_WITH_128_MANDATORY: u16 = 158u16;
pub const HID_USAGE_BARCODE_SCANNER_UPCA_WITH_128_OPTIONAL: u16 = 159u16;
pub const HID_USAGE_BARCODE_SCANNER_UPCA_WITH_P5_OPTIONAL: u16 = 160u16;
pub const HID_USAGE_BARCODE_SCANNER_UPCE: u16 = 161u16;
pub const HID_USAGE_BARCODE_SCANNER_UPCE1: u16 = 162u16;
pub const HID_USAGE_BARCODE_SCANNER_UPCEAN: u16 = 154u16;
pub const HID_USAGE_BARCODE_SCANNER_UPCEAN_CONTROL_REPORT: u16 = 22u16;
pub const HID_USAGE_BARCODE_SCANNER_UPCEAN_COUPON_CODE: u16 = 155u16;
pub const HID_USAGE_BARCODE_SCANNER_UPCEAN_PERIODICALS: u16 = 156u16;
pub const HID_USAGE_BARCODE_SCANNER_USD5_SLUG_CODE: u16 = 288u16;
pub const HID_USAGE_BARCODE_SCANNER_VERICODE: u16 = 289u16;
pub const HID_USAGE_BARCODE_SCANNER_WAND: u16 = 67u16;
pub const HID_USAGE_BARCODE_SCANNER_WATER_RESISTANT: u16 = 68u16;
pub const HID_USAGE_BATTERY_SYSTEM_ABSOLUTE_STATE_OF_CHARGE: u16 = 101u16;
pub const HID_USAGE_BATTERY_SYSTEM_AC_PRESENT: u16 = 208u16;
pub const HID_USAGE_BATTERY_SYSTEM_ALARM_INHIBITED: u16 = 211u16;
pub const HID_USAGE_BATTERY_SYSTEM_AT_RATE: u16 = 43u16;
pub const HID_USAGE_BATTERY_SYSTEM_AT_RATE_OK: u16 = 73u16;
pub const HID_USAGE_BATTERY_SYSTEM_AT_RATE_TIME_TO_EMPTY: u16 = 97u16;
pub const HID_USAGE_BATTERY_SYSTEM_AT_RATE_TIME_TO_FULL: u16 = 96u16;
pub const HID_USAGE_BATTERY_SYSTEM_AVERAGE_CURRENT: u16 = 98u16;
pub const HID_USAGE_BATTERY_SYSTEM_AVERAGE_TIME_TO_EMPTY: u16 = 105u16;
pub const HID_USAGE_BATTERY_SYSTEM_AVERAGE_TIME_TO_FULL: u16 = 106u16;
pub const HID_USAGE_BATTERY_SYSTEM_BATTERY_INSERTION: u16 = 24u16;
pub const HID_USAGE_BATTERY_SYSTEM_BATTERY_PACK_MODEL_LEVEL: u16 = 128u16;
pub const HID_USAGE_BATTERY_SYSTEM_BATTERY_PRESENT: u16 = 209u16;
pub const HID_USAGE_BATTERY_SYSTEM_BATTERY_SUPPORTED: u16 = 27u16;
pub const HID_USAGE_BATTERY_SYSTEM_BELOW_REMAINING_CAPACITY_LIMIT: u16 = 66u16;
pub const HID_USAGE_BATTERY_SYSTEM_BROADCAST_TO_CHARGER: u16 = 45u16;
pub const HID_USAGE_BATTERY_SYSTEM_CAPACITY_GRANULARITY_1: u16 = 141u16;
pub const HID_USAGE_BATTERY_SYSTEM_CAPACITY_GRANULARITY_2: u16 = 142u16;
pub const HID_USAGE_BATTERY_SYSTEM_CAPACITY_MODE: u16 = 44u16;
pub const HID_USAGE_BATTERY_SYSTEM_CHARGER_CONNECTION: u16 = 23u16;
pub const HID_USAGE_BATTERY_SYSTEM_CHARGER_SELECTOR_SUPPORT: u16 = 240u16;
pub const HID_USAGE_BATTERY_SYSTEM_CHARGER_SPEC: u16 = 241u16;
pub const HID_USAGE_BATTERY_SYSTEM_CHARGE_CONTROLLER: u16 = 47u16;
pub const HID_USAGE_BATTERY_SYSTEM_CHARGING: u16 = 68u16;
pub const HID_USAGE_BATTERY_SYSTEM_CHARGING_INDICATOR: u16 = 29u16;
pub const HID_USAGE_BATTERY_SYSTEM_CONDITIONING_FLAG: u16 = 72u16;
pub const HID_USAGE_BATTERY_SYSTEM_CONNECTION_TO_SM_BUS: u16 = 21u16;
pub const HID_USAGE_BATTERY_SYSTEM_CURRENT_NOT_REGULATED: u16 = 218u16;
pub const HID_USAGE_BATTERY_SYSTEM_CURRENT_OUT_OF_RANGE: u16 = 217u16;
pub const HID_USAGE_BATTERY_SYSTEM_CYCLE_COUNT: u16 = 107u16;
pub const HID_USAGE_BATTERY_SYSTEM_DESIGN_CAPACITY: u16 = 131u16;
pub const HID_USAGE_BATTERY_SYSTEM_DISCHARGING: u16 = 69u16;
pub const HID_USAGE_BATTERY_SYSTEM_ENABLE_POLLING: u16 = 193u16;
pub const HID_USAGE_BATTERY_SYSTEM_FULLY_CHARGED: u16 = 70u16;
pub const HID_USAGE_BATTERY_SYSTEM_FULLY_DISCHARGED: u16 = 71u16;
pub const HID_USAGE_BATTERY_SYSTEM_FULL_CHARGE_CAPACITY: u16 = 103u16;
pub const HID_USAGE_BATTERY_SYSTEM_IDEVICE_CHEMISTRY: u16 = 137u16;
pub const HID_USAGE_BATTERY_SYSTEM_IDEVICE_NAME: u16 = 136u16;
pub const HID_USAGE_BATTERY_SYSTEM_IMANUFACTURER_NAME: u16 = 135u16;
pub const HID_USAGE_BATTERY_SYSTEM_INHIBIT_CHARGE: u16 = 192u16;
pub const HID_USAGE_BATTERY_SYSTEM_INTERNAL_CHARGE_CONTROLLER: u16 = 129u16;
pub const HID_USAGE_BATTERY_SYSTEM_IOEM_INFORMATION: u16 = 143u16;
pub const HID_USAGE_BATTERY_SYSTEM_LEVEL_2: u16 = 242u16;
pub const HID_USAGE_BATTERY_SYSTEM_LEVEL_3: u16 = 243u16;
pub const HID_USAGE_BATTERY_SYSTEM_MANUFACTURER_ACCESS: u16 = 40u16;
pub const HID_USAGE_BATTERY_SYSTEM_MANUFACTURER_DATA: u16 = 138u16;
pub const HID_USAGE_BATTERY_SYSTEM_MANUFACTURE_DATE: u16 = 133u16;
pub const HID_USAGE_BATTERY_SYSTEM_MASTER_MODE: u16 = 220u16;
pub const HID_USAGE_BATTERY_SYSTEM_MAX_ERROR: u16 = 99u16;
pub const HID_USAGE_BATTERY_SYSTEM_NEED_REPLACEMENT: u16 = 75u16;
pub const HID_USAGE_BATTERY_SYSTEM_OK_TO_USE: u16 = 26u16;
pub const HID_USAGE_BATTERY_SYSTEM_OPTIONAL_MFG_FUNCTION_1: u16 = 16u16;
pub const HID_USAGE_BATTERY_SYSTEM_OPTIONAL_MFG_FUNCTION_2: u16 = 17u16;
pub const HID_USAGE_BATTERY_SYSTEM_OPTIONAL_MFG_FUNCTION_3: u16 = 18u16;
pub const HID_USAGE_BATTERY_SYSTEM_OPTIONAL_MFG_FUNCTION_4: u16 = 19u16;
pub const HID_USAGE_BATTERY_SYSTEM_OPTIONAL_MFG_FUNCTION_5: u16 = 20u16;
pub const HID_USAGE_BATTERY_SYSTEM_OUTPUT_CONNECTION: u16 = 22u16;
pub const HID_USAGE_BATTERY_SYSTEM_POWER_FAIL: u16 = 210u16;
pub const HID_USAGE_BATTERY_SYSTEM_PRIMARY_BATTERY: u16 = 46u16;
pub const HID_USAGE_BATTERY_SYSTEM_PRIMARY_BATTERY_SUPPORT: u16 = 130u16;
pub const HID_USAGE_BATTERY_SYSTEM_RECHARGABLE: u16 = 139u16;
pub const HID_USAGE_BATTERY_SYSTEM_RELATIVE_STATE_OF_CHARGE: u16 = 100u16;
pub const HID_USAGE_BATTERY_SYSTEM_REMAINING_CAPACITY: u16 = 102u16;
pub const HID_USAGE_BATTERY_SYSTEM_REMAINING_CAPACITY_LIMIT: u16 = 41u16;
pub const HID_USAGE_BATTERY_SYSTEM_REMAINING_TIME_LIMIT: u16 = 42u16;
pub const HID_USAGE_BATTERY_SYSTEM_REMAINING_TIME_LIMIT_EXPIRED: u16 = 67u16;
pub const HID_USAGE_BATTERY_SYSTEM_RESET_TO_ZERO: u16 = 194u16;
pub const HID_USAGE_BATTERY_SYSTEM_RUN_TIME_TO_EMPTY: u16 = 104u16;
pub const HID_USAGE_BATTERY_SYSTEM_SELECTOR_REVISION: u16 = 28u16;
pub const HID_USAGE_BATTERY_SYSTEM_SERIAL_NUMBER: u16 = 134u16;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_ALARM_WARNING: u16 = 3u16;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_BATTERY_MODE: u16 = 1u16;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_BATTERY_STATUS: u16 = 2u16;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_CHARGER_MODE: u16 = 4u16;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_CHARGER_SPEC_INFO: u16 = 6u16;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_CHARGER_STATUS: u16 = 5u16;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_ERROR_CODE: u16 = 74u16;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_SELECTOR_INFO: u16 = 9u16;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_SELECTOR_PRESETS: u16 = 8u16;
pub const HID_USAGE_BATTERY_SYSTEM_SMART_BATTERY_SELECTOR_STATE: u16 = 7u16;
pub const HID_USAGE_BATTERY_SYSTEM_SPECIFICATION_INFO: u16 = 132u16;
pub const HID_USAGE_BATTERY_SYSTEM_TERMINATE_CHARGE: u16 = 64u16;
pub const HID_USAGE_BATTERY_SYSTEM_TERMINATE_DISCHARGE: u16 = 65u16;
pub const HID_USAGE_BATTERY_SYSTEM_THERMISTOR_COLD: u16 = 214u16;
pub const HID_USAGE_BATTERY_SYSTEM_THERMISTOR_HOT: u16 = 213u16;
pub const HID_USAGE_BATTERY_SYSTEM_THERMISTOR_OVER_RANGE: u16 = 215u16;
pub const HID_USAGE_BATTERY_SYSTEM_THERMISTOR_UNDER_RANGE: u16 = 212u16;
pub const HID_USAGE_BATTERY_SYSTEM_USE_NEXT: u16 = 25u16;
pub const HID_USAGE_BATTERY_SYSTEM_VOLTAGE_NOT_REGULATED: u16 = 219u16;
pub const HID_USAGE_BATTERY_SYSTEM_VOLTAGE_OUT_OF_RANGE: u16 = 216u16;
pub const HID_USAGE_BATTERY_SYSTEM_WARNING_CAPACITY_LIMIT: u16 = 140u16;
pub const HID_USAGE_BRAILLE_DISPLAY_6_DOT_BRAILLE_CELL: u16 = 4u16;
pub const HID_USAGE_BRAILLE_DISPLAY_8_DOT_BRAILLE_CELL: u16 = 3u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_BUTTONS: u16 = 512u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_DISPLAY: u16 = 1u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_DPAD_CENTER: u16 = 533u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_DPAD_DOWN: u16 = 535u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_DPAD_LEFT: u16 = 536u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_DPAD_RIGHT: u16 = 537u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_DPAD_UP: u16 = 534u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_FACE_CONTROLS: u16 = 524u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_JOYSTICK_CENTER: u16 = 528u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_JOYSTICK_DOWN: u16 = 530u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_JOYSTICK_LEFT: u16 = 531u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_JOYSTICK_RIGHT: u16 = 532u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_JOYSTICK_UP: u16 = 529u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_1: u16 = 513u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_2: u16 = 514u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_3: u16 = 515u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_4: u16 = 516u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_5: u16 = 517u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_6: u16 = 518u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_7: u16 = 519u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_DOT_8: u16 = 520u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_LEFT_SPACE: u16 = 522u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_RIGHT_SPACE: u16 = 523u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_KEYBOARD_SPACE: u16 = 521u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_LEFT_CONTROLS: u16 = 525u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_PAN_LEFT: u16 = 538u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_PAN_RIGHT: u16 = 539u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_RIGHT_CONTROLS: u16 = 526u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_ROCKER_DOWN: u16 = 541u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_ROCKER_PRESS: u16 = 542u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_ROCKER_UP: u16 = 540u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_ROW: u16 = 2u16;
pub const HID_USAGE_BRAILLE_DISPLAY_BRAILLE_TOP_CONTROLS: u16 = 527u16;
pub const HID_USAGE_BRAILLE_DISPLAY_NUMBER_OF_BRAILLE_CELLS: u16 = 5u16;
pub const HID_USAGE_BRAILLE_DISPLAY_ROUTER_KEY: u16 = 256u16;
pub const HID_USAGE_BRAILLE_DISPLAY_ROUTER_SET_1: u16 = 250u16;
pub const HID_USAGE_BRAILLE_DISPLAY_ROUTER_SET_2: u16 = 251u16;
pub const HID_USAGE_BRAILLE_DISPLAY_ROUTER_SET_3: u16 = 252u16;
pub const HID_USAGE_BRAILLE_DISPLAY_ROW_ROUTER_KEY: u16 = 257u16;
pub const HID_USAGE_BRAILLE_DISPLAY_SCREEN_READER_CONTROL: u16 = 6u16;
pub const HID_USAGE_BRAILLE_DISPLAY_SCREEN_READER_IDENTIFIER: u16 = 7u16;
pub const HID_USAGE_CAMERA_AUTO_FOCUS: u16 = 32u16;
pub const HID_USAGE_CAMERA_SHUTTER: u16 = 33u16;
pub const HID_USAGE_CONSUMERCTRL: u16 = 1u16;
pub const HID_USAGE_CONSUMER_10: u16 = 32u16;
pub const HID_USAGE_CONSUMER_100: u16 = 33u16;
pub const HID_USAGE_CONSUMER_3D_MODE_SELECT: u16 = 110u16;
pub const HID_USAGE_CONSUMER_AC_ADD_TO_CART: u16 = 610u16;
pub const HID_USAGE_CONSUMER_AC_ALL_CAPS: u16 = 580u16;
pub const HID_USAGE_CONSUMER_AC_ATTACH_COMMENT: u16 = 623u16;
pub const HID_USAGE_CONSUMER_AC_ATTACH_FILE: u16 = 653u16;
pub const HID_USAGE_CONSUMER_AC_BACK: u16 = 548u16;
pub const HID_USAGE_CONSUMER_AC_BOLD: u16 = 574u16;
pub const HID_USAGE_CONSUMER_AC_BOOKMARKS: u16 = 554u16;
pub const HID_USAGE_CONSUMER_AC_BULLETED_LIST: u16 = 602u16;
pub const HID_USAGE_CONSUMER_AC_BUYCHECKOUT: u16 = 609u16;
pub const HID_USAGE_CONSUMER_AC_CANCEL: u16 = 607u16;
pub const HID_USAGE_CONSUMER_AC_CATALOG: u16 = 608u16;
pub const HID_USAGE_CONSUMER_AC_CLEAR_ALARM: u16 = 643u16;
pub const HID_USAGE_CONSUMER_AC_CLOSE: u16 = 515u16;
pub const HID_USAGE_CONSUMER_AC_COLLAPSE: u16 = 613u16;
pub const HID_USAGE_CONSUMER_AC_COLLAPSE_ALL: u16 = 614u16;
pub const HID_USAGE_CONSUMER_AC_COPY: u16 = 539u16;
pub const HID_USAGE_CONSUMER_AC_CUT: u16 = 540u16;
pub const HID_USAGE_CONSUMER_AC_DELETE: u16 = 618u16;
pub const HID_USAGE_CONSUMER_AC_DELETE_COMMENT: u16 = 624u16;
pub const HID_USAGE_CONSUMER_AC_DEMOTE: u16 = 604u16;
pub const HID_USAGE_CONSUMER_AC_DESKTOP_SHOW_ALL_APPLICATIONS: u16 = 674u16;
pub const HID_USAGE_CONSUMER_AC_DESKTOP_SHOW_ALL_WINDOWS: u16 = 671u16;
pub const HID_USAGE_CONSUMER_AC_DISRIBUTE_HORIZONTALLY: u16 = 667u16;
pub const HID_USAGE_CONSUMER_AC_DISTRIBUTE_VERTICALLY: u16 = 668u16;
pub const HID_USAGE_CONSUMER_AC_DOWNLOAD_SAVE_TARGET_AS: u16 = 655u16;
pub const HID_USAGE_CONSUMER_AC_EDIT: u16 = 573u16;
pub const HID_USAGE_CONSUMER_AC_EDIT_TIME_ZONES: u16 = 641u16;
pub const HID_USAGE_CONSUMER_AC_EXIT: u16 = 516u16;
pub const HID_USAGE_CONSUMER_AC_EXPAND: u16 = 611u16;
pub const HID_USAGE_CONSUMER_AC_EXPAND_ALL: u16 = 612u16;
pub const HID_USAGE_CONSUMER_AC_FILTER: u16 = 637u16;
pub const HID_USAGE_CONSUMER_AC_FIND: u16 = 543u16;
pub const HID_USAGE_CONSUMER_AC_FIND_AND_REPLACE: u16 = 544u16;
pub const HID_USAGE_CONSUMER_AC_FLIP_HORIZONTAL: u16 = 583u16;
pub const HID_USAGE_CONSUMER_AC_FLIP_VERTICAL: u16 = 584u16;
pub const HID_USAGE_CONSUMER_AC_FONT_COLOR: u16 = 588u16;
pub const HID_USAGE_CONSUMER_AC_FONT_SELECT: u16 = 587u16;
pub const HID_USAGE_CONSUMER_AC_FONT_SIZE: u16 = 589u16;
pub const HID_USAGE_CONSUMER_AC_FORMAT: u16 = 572u16;
pub const HID_USAGE_CONSUMER_AC_FORWARD: u16 = 549u16;
pub const HID_USAGE_CONSUMER_AC_FORWARD_MSG: u16 = 651u16;
pub const HID_USAGE_CONSUMER_AC_FULL_SCREEN_VIEW: u16 = 560u16;
pub const HID_USAGE_CONSUMER_AC_GOTO: u16 = 546u16;
pub const HID_USAGE_CONSUMER_AC_HISTORY: u16 = 555u16;
pub const HID_USAGE_CONSUMER_AC_HOME: u16 = 547u16;
pub const HID_USAGE_CONSUMER_AC_IDLE_KEEP_ALIVE: u16 = 688u16;
pub const HID_USAGE_CONSUMER_AC_INDENT_DECREASE: u16 = 598u16;
pub const HID_USAGE_CONSUMER_AC_INDENT_INCREASE: u16 = 599u16;
pub const HID_USAGE_CONSUMER_AC_INSERT_COLUMN: u16 = 658u16;
pub const HID_USAGE_CONSUMER_AC_INSERT_FILE: u16 = 659u16;
pub const HID_USAGE_CONSUMER_AC_INSERT_MODE: u16 = 617u16;
pub const HID_USAGE_CONSUMER_AC_INSERT_OBJECT: u16 = 661u16;
pub const HID_USAGE_CONSUMER_AC_INSERT_PICTURE: u16 = 660u16;
pub const HID_USAGE_CONSUMER_AC_INSERT_ROW: u16 = 657u16;
pub const HID_USAGE_CONSUMER_AC_INSERT_SYMBOL: u16 = 662u16;
pub const HID_USAGE_CONSUMER_AC_ITALICS: u16 = 575u16;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_BLOCK_H: u16 = 593u16;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_BLOCK_V: u16 = 597u16;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_BOTTOM: u16 = 596u16;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_CENTER_H: u16 = 591u16;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_CENTER_V: u16 = 595u16;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_LEFT: u16 = 590u16;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_RIGHT: u16 = 592u16;
pub const HID_USAGE_CONSUMER_AC_JUSTIFY_TOP: u16 = 594u16;
pub const HID_USAGE_CONSUMER_AC_LOCK: u16 = 619u16;
pub const HID_USAGE_CONSUMER_AC_MAXIMIZE: u16 = 517u16;
pub const HID_USAGE_CONSUMER_AC_MERGE: u16 = 665u16;
pub const HID_USAGE_CONSUMER_AC_MINIMIZE: u16 = 518u16;
pub const HID_USAGE_CONSUMER_AC_MIRROR_HORIZONTAL: u16 = 585u16;
pub const HID_USAGE_CONSUMER_AC_MIRROR_VERTICAL: u16 = 586u16;
pub const HID_USAGE_CONSUMER_AC_NAVIGATION_GUIDANCE: u16 = 670u16;
pub const HID_USAGE_CONSUMER_AC_NEW: u16 = 513u16;
pub const HID_USAGE_CONSUMER_AC_NEW_WINDOW: u16 = 569u16;
pub const HID_USAGE_CONSUMER_AC_NEXT: u16 = 553u16;
pub const HID_USAGE_CONSUMER_AC_NEXT_KEYBOARD_LAYOUT_SELECT: u16 = 669u16;
pub const HID_USAGE_CONSUMER_AC_NO: u16 = 606u16;
pub const HID_USAGE_CONSUMER_AC_NORMAL_VIEW: u16 = 561u16;
pub const HID_USAGE_CONSUMER_AC_NUMBERED_LIST: u16 = 600u16;
pub const HID_USAGE_CONSUMER_AC_OPEN: u16 = 514u16;
pub const HID_USAGE_CONSUMER_AC_PAN: u16 = 568u16;
pub const HID_USAGE_CONSUMER_AC_PAN_LEFT: u16 = 566u16;
pub const HID_USAGE_CONSUMER_AC_PAN_RIGHT: u16 = 567u16;
pub const HID_USAGE_CONSUMER_AC_PASTE: u16 = 541u16;
pub const HID_USAGE_CONSUMER_AC_PASTE_SPECIAL: u16 = 616u16;
pub const HID_USAGE_CONSUMER_AC_PREVIOUS: u16 = 552u16;
pub const HID_USAGE_CONSUMER_AC_PRINT: u16 = 520u16;
pub const HID_USAGE_CONSUMER_AC_PRINT_PREVIEW: u16 = 615u16;
pub const HID_USAGE_CONSUMER_AC_PROMOTE: u16 = 603u16;
pub const HID_USAGE_CONSUMER_AC_PROPERTIES: u16 = 521u16;
pub const HID_USAGE_CONSUMER_AC_PROTECT: u16 = 621u16;
pub const HID_USAGE_CONSUMER_AC_REDOREPEAT: u16 = 633u16;
pub const HID_USAGE_CONSUMER_AC_REFRESH: u16 = 551u16;
pub const HID_USAGE_CONSUMER_AC_RENAME: u16 = 664u16;
pub const HID_USAGE_CONSUMER_AC_REPLY: u16 = 649u16;
pub const HID_USAGE_CONSUMER_AC_REPLY_ALL: u16 = 650u16;
pub const HID_USAGE_CONSUMER_AC_RESET_ALARM: u16 = 645u16;
pub const HID_USAGE_CONSUMER_AC_RESIZE: u16 = 582u16;
pub const HID_USAGE_CONSUMER_AC_RESTART_NUMBERING: u16 = 601u16;
pub const HID_USAGE_CONSUMER_AC_ROTATE: u16 = 581u16;
pub const HID_USAGE_CONSUMER_AC_SAVE: u16 = 519u16;
pub const HID_USAGE_CONSUMER_AC_SAVE_AND_CLOSE: u16 = 663u16;
pub const HID_USAGE_CONSUMER_AC_SCROLL: u16 = 565u16;
pub const HID_USAGE_CONSUMER_AC_SCROLL_DOWN: u16 = 564u16;
pub const HID_USAGE_CONSUMER_AC_SCROLL_UP: u16 = 563u16;
pub const HID_USAGE_CONSUMER_AC_SEARCH: u16 = 545u16;
pub const HID_USAGE_CONSUMER_AC_SELECT_ALL: u16 = 542u16;
pub const HID_USAGE_CONSUMER_AC_SELECT_COLUMN: u16 = 629u16;
pub const HID_USAGE_CONSUMER_AC_SELECT_OBJECT: u16 = 632u16;
pub const HID_USAGE_CONSUMER_AC_SELECT_PARAGRAPH: u16 = 628u16;
pub const HID_USAGE_CONSUMER_AC_SELECT_ROW: u16 = 630u16;
pub const HID_USAGE_CONSUMER_AC_SELECT_SENTENCE: u16 = 627u16;
pub const HID_USAGE_CONSUMER_AC_SELECT_TABLE: u16 = 631u16;
pub const HID_USAGE_CONSUMER_AC_SELECT_TIME_ZONE: u16 = 640u16;
pub const HID_USAGE_CONSUMER_AC_SELECT_WORD: u16 = 626u16;
pub const HID_USAGE_CONSUMER_AC_SEND: u16 = 652u16;
pub const HID_USAGE_CONSUMER_AC_SENDRECEIVE: u16 = 647u16;
pub const HID_USAGE_CONSUMER_AC_SEND_TO: u16 = 648u16;
pub const HID_USAGE_CONSUMER_AC_SET_ALARM: u16 = 642u16;
pub const HID_USAGE_CONSUMER_AC_SET_BORDERS: u16 = 656u16;
pub const HID_USAGE_CONSUMER_AC_SET_CLOCK: u16 = 638u16;
pub const HID_USAGE_CONSUMER_AC_SNOOZE_ALARM: u16 = 644u16;
pub const HID_USAGE_CONSUMER_AC_SOFT_KEY_LEFT: u16 = 672u16;
pub const HID_USAGE_CONSUMER_AC_SOFT_KEY_RIGHT: u16 = 673u16;
pub const HID_USAGE_CONSUMER_AC_SORT: u16 = 634u16;
pub const HID_USAGE_CONSUMER_AC_SORT_ASCENDING: u16 = 635u16;
pub const HID_USAGE_CONSUMER_AC_SORT_DESCENDING: u16 = 636u16;
pub const HID_USAGE_CONSUMER_AC_SPLIT: u16 = 666u16;
pub const HID_USAGE_CONSUMER_AC_STOP: u16 = 550u16;
pub const HID_USAGE_CONSUMER_AC_STRIKETHROUGH: u16 = 577u16;
pub const HID_USAGE_CONSUMER_AC_SUBSCRIPT: u16 = 578u16;
pub const HID_USAGE_CONSUMER_AC_SUBSCRIPTIONS: u16 = 556u16;
pub const HID_USAGE_CONSUMER_AC_SUPERSCRIPT: u16 = 579u16;
pub const HID_USAGE_CONSUMER_AC_SYNCHRONIZE: u16 = 646u16;
pub const HID_USAGE_CONSUMER_AC_TILE_HORIZONTALLY: u16 = 570u16;
pub const HID_USAGE_CONSUMER_AC_TILE_VERTICALLY: u16 = 571u16;
pub const HID_USAGE_CONSUMER_AC_UNDERLINE: u16 = 576u16;
pub const HID_USAGE_CONSUMER_AC_UNDO: u16 = 538u16;
pub const HID_USAGE_CONSUMER_AC_UNLOCK: u16 = 620u16;
pub const HID_USAGE_CONSUMER_AC_UNPROTECT: u16 = 622u16;
pub const HID_USAGE_CONSUMER_AC_UPLOAD: u16 = 654u16;
pub const HID_USAGE_CONSUMER_AC_VIEW_CLOCK: u16 = 639u16;
pub const HID_USAGE_CONSUMER_AC_VIEW_COMMENT: u16 = 625u16;
pub const HID_USAGE_CONSUMER_AC_VIEW_TOGGLE: u16 = 562u16;
pub const HID_USAGE_CONSUMER_AC_YES: u16 = 605u16;
pub const HID_USAGE_CONSUMER_AC_ZOOM: u16 = 559u16;
pub const HID_USAGE_CONSUMER_AC_ZOOM_IN: u16 = 557u16;
pub const HID_USAGE_CONSUMER_AC_ZOOM_OUT: u16 = 558u16;
pub const HID_USAGE_CONSUMER_ALTERNATE_AUDIO_DECREMENT: u16 = 372u16;
pub const HID_USAGE_CONSUMER_ALTERNATE_AUDIO_INCREMENT: u16 = 371u16;
pub const HID_USAGE_CONSUMER_AL_ALARMS: u16 = 434u16;
pub const HID_USAGE_CONSUMER_AL_AUDIO_BROWSER: u16 = 439u16;
pub const HID_USAGE_CONSUMER_AL_AUDIO_PLAYER: u16 = 455u16;
pub const HID_USAGE_CONSUMER_AL_AV_CAPTUREPLAYBACK: u16 = 403u16;
pub const HID_USAGE_CONSUMER_AL_BROWSER: u16 = 404u16;
pub const HID_USAGE_CONSUMER_AL_CALCULATOR: u16 = 402u16;
pub const HID_USAGE_CONSUMER_AL_CALENDARSCHEDULE: u16 = 398u16;
pub const HID_USAGE_CONSUMER_AL_CHECKBOOKFINANCE: u16 = 401u16;
pub const HID_USAGE_CONSUMER_AL_CLOCK: u16 = 435u16;
pub const HID_USAGE_CONSUMER_AL_COMMAND_LINE_PROCESSORRUN: u16 = 416u16;
pub const HID_USAGE_CONSUMER_AL_CONFIGURATION: u16 = 387u16;
pub const HID_USAGE_CONSUMER_AL_CONTACTSADDRESS_BOOK: u16 = 397u16;
pub const HID_USAGE_CONSUMER_AL_CONTACT_SYNC: u16 = 457u16;
pub const HID_USAGE_CONSUMER_AL_CONTEXTAWARE_DESKTOP_ASSISTANT: u16 = 459u16;
pub const HID_USAGE_CONSUMER_AL_CONTROL_PANEL: u16 = 415u16;
pub const HID_USAGE_CONSUMER_AL_CUSTOMIZED_CORPORATE_NEWS_BROWSER: u16 = 452u16;
pub const HID_USAGE_CONSUMER_AL_DATABASE_APP: u16 = 393u16;
pub const HID_USAGE_CONSUMER_AL_DESKTOP: u16 = 426u16;
pub const HID_USAGE_CONSUMER_AL_DICTIONARY: u16 = 425u16;
pub const HID_USAGE_CONSUMER_AL_DIGITAL_RIGHTS_MANAGER: u16 = 441u16;
pub const HID_USAGE_CONSUMER_AL_DIGITAL_WALLET: u16 = 442u16;
pub const HID_USAGE_CONSUMER_AL_DOCUMENTS: u16 = 423u16;
pub const HID_USAGE_CONSUMER_AL_EMAIL: u16 = 394u16;
pub const HID_USAGE_CONSUMER_AL_ENCRYPTION: u16 = 432u16;
pub const HID_USAGE_CONSUMER_AL_ENTERTAINMENT_CONTENT_BROWSER: u16 = 448u16;
pub const HID_USAGE_CONSUMER_AL_FILE_BROWSER: u16 = 436u16;
pub const HID_USAGE_CONSUMER_AL_GRAMMAR_CHECK: u16 = 428u16;
pub const HID_USAGE_CONSUMER_AL_GRAPHICS_EDITOR: u16 = 391u16;
pub const HID_USAGE_CONSUMER_AL_IMAGE_BROWSER: u16 = 438u16;
pub const HID_USAGE_CONSUMER_AL_INSTANT_MESSAGING: u16 = 444u16;
pub const HID_USAGE_CONSUMER_AL_INTEGRATED_HELP_CENTER: u16 = 422u16;
pub const HID_USAGE_CONSUMER_AL_INTERNET_BROWSER: u16 = 406u16;
pub const HID_USAGE_CONSUMER_AL_KEYBOARD_LAYOUT: u16 = 430u16;
pub const HID_USAGE_CONSUMER_AL_LANWAN_BROWSER: u16 = 405u16;
pub const HID_USAGE_CONSUMER_AL_LAUNCH_BUTTON_CONFIGURATION_TOOL: u16 = 385u16;
pub const HID_USAGE_CONSUMER_AL_LOGJOURNALTIMECARD: u16 = 400u16;
pub const HID_USAGE_CONSUMER_AL_LOGOFF: u16 = 412u16;
pub const HID_USAGE_CONSUMER_AL_LOGON: u16 = 411u16;
pub const HID_USAGE_CONSUMER_AL_LOGONLOGOFF: u16 = 413u16;
pub const HID_USAGE_CONSUMER_AL_MARKET_MONITORFINANCE_BROWSER: u16 = 451u16;
pub const HID_USAGE_CONSUMER_AL_MESSAGE_STATUS: u16 = 456u16;
pub const HID_USAGE_CONSUMER_AL_MOVIE_BROWSER: u16 = 440u16;
pub const HID_USAGE_CONSUMER_AL_NAVIGATION: u16 = 458u16;
pub const HID_USAGE_CONSUMER_AL_NETWORK_CHAT: u16 = 409u16;
pub const HID_USAGE_CONSUMER_AL_NETWORK_CONFERENCE: u16 = 408u16;
pub const HID_USAGE_CONSUMER_AL_NEWSREADER: u16 = 395u16;
pub const HID_USAGE_CONSUMER_AL_NEXT_TASKAPPLICATION: u16 = 419u16;
pub const HID_USAGE_CONSUMER_AL_OEM_FEATURES_TIPSTUTORIAL_BROWSER: u16 = 445u16;
pub const HID_USAGE_CONSUMER_AL_OEM_HELP: u16 = 446u16;
pub const HID_USAGE_CONSUMER_AL_ONLINE_ACTIVITY_BROWSER: u16 = 453u16;
pub const HID_USAGE_CONSUMER_AL_ONLINE_COMMUNITY: u16 = 447u16;
pub const HID_USAGE_CONSUMER_AL_ONLINE_SHOPPING_BROWSER: u16 = 449u16;
pub const HID_USAGE_CONSUMER_AL_POWER_STATUS: u16 = 437u16;
pub const HID_USAGE_CONSUMER_AL_PREEMPTIVE_HALT_TASKAPPLICATION: u16 = 421u16;
pub const HID_USAGE_CONSUMER_AL_PRESENTATION_APP: u16 = 392u16;
pub const HID_USAGE_CONSUMER_AL_PREVIOUS_TASKAPPLICATION: u16 = 420u16;
pub const HID_USAGE_CONSUMER_AL_PROCESSTASK_MANAGER: u16 = 417u16;
pub const HID_USAGE_CONSUMER_AL_PROGRAMMABLE_BUTTON_CONFIGURATION: u16 = 386u16;
pub const HID_USAGE_CONSUMER_AL_REMOTE_NETWORKINGISP_CONNECT: u16 = 407u16;
pub const HID_USAGE_CONSUMER_AL_SCREEN_SAVER: u16 = 433u16;
pub const HID_USAGE_CONSUMER_AL_SEARCH: u16 = 454u16;
pub const HID_USAGE_CONSUMER_AL_SELECT_TASKAPPLICATION: u16 = 418u16;
pub const HID_USAGE_CONSUMER_AL_SMARTCARD_INFORMATIONHELP: u16 = 450u16;
pub const HID_USAGE_CONSUMER_AL_SPELL_CHECK: u16 = 427u16;
pub const HID_USAGE_CONSUMER_AL_SPREADSHEET: u16 = 390u16;
pub const HID_USAGE_CONSUMER_AL_TASKPROJECT_MANAGER: u16 = 399u16;
pub const HID_USAGE_CONSUMER_AL_TELEPHONYDIALER: u16 = 410u16;
pub const HID_USAGE_CONSUMER_AL_TERMINAL_LOCKSCREENSAVER: u16 = 414u16;
pub const HID_USAGE_CONSUMER_AL_TEXT_EDITOR: u16 = 389u16;
pub const HID_USAGE_CONSUMER_AL_THESAURUS: u16 = 424u16;
pub const HID_USAGE_CONSUMER_AL_VIRUS_PROTECTION: u16 = 431u16;
pub const HID_USAGE_CONSUMER_AL_VOICEMAIL: u16 = 396u16;
pub const HID_USAGE_CONSUMER_AL_WIRELESS_STATUS: u16 = 429u16;
pub const HID_USAGE_CONSUMER_AL_WORD_PROCESSOR: u16 = 388u16;
pub const HID_USAGE_CONSUMER_AMPM: u16 = 34u16;
pub const HID_USAGE_CONSUMER_APPLICATION_LAUNCH_BUTTONS: u16 = 384u16;
pub const HID_USAGE_CONSUMER_ASPECT: u16 = 109u16;
pub const HID_USAGE_CONSUMER_ASSIGN_SELECTION: u16 = 129u16;
pub const HID_USAGE_CONSUMER_BALANCE: u16 = 225u16;
pub const HID_USAGE_CONSUMER_BALANCE_LEFT: u16 = 337u16;
pub const HID_USAGE_CONSUMER_BALANCE_RIGHT: u16 = 336u16;
pub const HID_USAGE_CONSUMER_BASS: u16 = 227u16;
pub const HID_USAGE_CONSUMER_BASS_BOOST: u16 = 229u16;
pub const HID_USAGE_CONSUMER_BASS_DECREMENT: u16 = 339u16;
pub const HID_USAGE_CONSUMER_BASS_INCREMENT: u16 = 338u16;
pub const HID_USAGE_CONSUMER_BLUE_MENU_BUTTON: u16 = 107u16;
pub const HID_USAGE_CONSUMER_BROADCAST_MODE: u16 = 100u16;
pub const HID_USAGE_CONSUMER_CAMERA_ACCESS_DISABLED: u16 = 119u16;
pub const HID_USAGE_CONSUMER_CAMERA_ACCESS_ENABLED: u16 = 118u16;
pub const HID_USAGE_CONSUMER_CAMERA_ACCESS_TOGGLE: u16 = 120u16;
pub const HID_USAGE_CONSUMER_CHANNEL: u16 = 134u16;
pub const HID_USAGE_CONSUMER_CHANNEL_CENTER: u16 = 355u16;
pub const HID_USAGE_CONSUMER_CHANNEL_CENTER_FRONT: u16 = 357u16;
pub const HID_USAGE_CONSUMER_CHANNEL_DECREMENT: u16 = 157u16;
pub const HID_USAGE_CONSUMER_CHANNEL_FRONT: u16 = 356u16;
pub const HID_USAGE_CONSUMER_CHANNEL_INCREMENT: u16 = 156u16;
pub const HID_USAGE_CONSUMER_CHANNEL_LEFT: u16 = 353u16;
pub const HID_USAGE_CONSUMER_CHANNEL_LOW_FREQUENCY_ENHANCEMENT: u16 = 360u16;
pub const HID_USAGE_CONSUMER_CHANNEL_RIGHT: u16 = 354u16;
pub const HID_USAGE_CONSUMER_CHANNEL_SIDE: u16 = 358u16;
pub const HID_USAGE_CONSUMER_CHANNEL_SURROUND: u16 = 359u16;
pub const HID_USAGE_CONSUMER_CHANNEL_TOP: u16 = 361u16;
pub const HID_USAGE_CONSUMER_CHANNEL_UNKNOWN: u16 = 362u16;
pub const HID_USAGE_CONSUMER_CLEAR_MARK: u16 = 195u16;
pub const HID_USAGE_CONSUMER_CLIMATE_CONTROL_ENABLE: u16 = 260u16;
pub const HID_USAGE_CONSUMER_CLOSED_CAPTION: u16 = 97u16;
pub const HID_USAGE_CONSUMER_CLOSED_CAPTION_SELECT: u16 = 98u16;
pub const HID_USAGE_CONSUMER_CONTACT_ADDED: u16 = 1281u16;
pub const HID_USAGE_CONSUMER_CONTACT_EDITED: u16 = 1280u16;
pub const HID_USAGE_CONSUMER_CONTACT_EMAIL_BUSINESS: u16 = 1295u16;
pub const HID_USAGE_CONSUMER_CONTACT_EMAIL_MAIN: u16 = 1297u16;
pub const HID_USAGE_CONSUMER_CONTACT_EMAIL_OTHER: u16 = 1296u16;
pub const HID_USAGE_CONSUMER_CONTACT_EMAIL_PERSONAL: u16 = 1294u16;
pub const HID_USAGE_CONSUMER_CONTACT_FIRST_NAME: u16 = 1285u16;
pub const HID_USAGE_CONSUMER_CONTACT_FULL_NAME: u16 = 1287u16;
pub const HID_USAGE_CONSUMER_CONTACT_INDEX: u16 = 1283u16;
pub const HID_USAGE_CONSUMER_CONTACT_LAST_NAME: u16 = 1286u16;
pub const HID_USAGE_CONSUMER_CONTACT_MISC: u16 = 1300u16;
pub const HID_USAGE_CONSUMER_CONTACT_NICKNAME: u16 = 1284u16;
pub const HID_USAGE_CONSUMER_CONTACT_PHONE_NUMBER_BUSINESS: u16 = 1289u16;
pub const HID_USAGE_CONSUMER_CONTACT_PHONE_NUMBER_FAX: u16 = 1292u16;
pub const HID_USAGE_CONSUMER_CONTACT_PHONE_NUMBER_MOBILE: u16 = 1290u16;
pub const HID_USAGE_CONSUMER_CONTACT_PHONE_NUMBER_OTHER: u16 = 1293u16;
pub const HID_USAGE_CONSUMER_CONTACT_PHONE_NUMBER_PAGER: u16 = 1291u16;
pub const HID_USAGE_CONSUMER_CONTACT_PHONE_NUMBER_PERSONAL: u16 = 1288u16;
pub const HID_USAGE_CONSUMER_CONTACT_RECORD_ACTIVE: u16 = 1282u16;
pub const HID_USAGE_CONSUMER_CONTACT_SPEED_DIAL_NUMBER: u16 = 1298u16;
pub const HID_USAGE_CONSUMER_CONTACT_STATUS_FLAG: u16 = 1299u16;
pub const HID_USAGE_CONSUMER_COUNTER_RESET: u16 = 200u16;
pub const HID_USAGE_CONSUMER_DAILY: u16 = 162u16;
pub const HID_USAGE_CONSUMER_DATA_ON_SCREEN: u16 = 96u16;
pub const HID_USAGE_CONSUMER_DISPLAY_BACKLIGHT_TOGGLE: u16 = 114u16;
pub const HID_USAGE_CONSUMER_DISPLAY_BRIGHTNESS: u16 = 113u16;
pub const HID_USAGE_CONSUMER_DISPLAY_BRIGHTNESS_DECREMENT: u16 = 112u16;
pub const HID_USAGE_CONSUMER_DISPLAY_BRIGHTNESS_INCREMENT: u16 = 111u16;
pub const HID_USAGE_CONSUMER_DISPLAY_SET_AUTO_BRIGHTNESS: u16 = 117u16;
pub const HID_USAGE_CONSUMER_DISPLAY_SET_BRIGHTNESS_TO_MAXIMUM: u16 = 116u16;
pub const HID_USAGE_CONSUMER_DISPLAY_SET_BRIGHTNESS_TO_MINIMUM: u16 = 115u16;
pub const HID_USAGE_CONSUMER_DURESS_ALARM: u16 = 267u16;
pub const HID_USAGE_CONSUMER_EJECT: u16 = 184u16;
pub const HID_USAGE_CONSUMER_ENTER_CHANNEL: u16 = 132u16;
pub const HID_USAGE_CONSUMER_ENTER_DISC: u16 = 187u16;
pub const HID_USAGE_CONSUMER_EXTENDED_KEYBOARD_ATTRIBUTES_COLLECTION: u16 = 704u16;
pub const HID_USAGE_CONSUMER_EXTENDED_PLAY: u16 = 244u16;
pub const HID_USAGE_CONSUMER_FAN_ENABLE: u16 = 256u16;
pub const HID_USAGE_CONSUMER_FAN_SPEED: u16 = 257u16;
pub const HID_USAGE_CONSUMER_FAST_FORWARD: u16 = 179u16;
pub const HID_USAGE_CONSUMER_FIRE_ALARM: u16 = 263u16;
pub const HID_USAGE_CONSUMER_FRAME_BACK: u16 = 193u16;
pub const HID_USAGE_CONSUMER_FRAME_FORWARD: u16 = 192u16;
pub const HID_USAGE_CONSUMER_FUNCTION_BUTTONS: u16 = 54u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_OPEN_GAMEBAR: u16 = 208u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_RECORD_CLIP: u16 = 210u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_SCREENSHOT: u16 = 211u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_BROADCAST: u16 = 215u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_CAMERA: u16 = 214u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_INDICATOR: u16 = 212u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_MICROPHONE: u16 = 213u16;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_RECORD: u16 = 209u16;
pub const HID_USAGE_CONSUMER_GENERIC_GUI_APPLICATION_CONTROLS: u16 = 512u16;
pub const HID_USAGE_CONSUMER_GRAPHIC_EQUALIZER: u16 = 6u16;
pub const HID_USAGE_CONSUMER_GREEN_MENU_BUTTON: u16 = 106u16;
pub const HID_USAGE_CONSUMER_HEADPHONE: u16 = 5u16;
pub const HID_USAGE_CONSUMER_HELP: u16 = 149u16;
pub const HID_USAGE_CONSUMER_HOLDUP_ALARM: u16 = 268u16;
pub const HID_USAGE_CONSUMER_ILLUMINATION: u16 = 53u16;
pub const HID_USAGE_CONSUMER_IMPLEMENTED_KEYBOARD_INPUT_ASSIST_CONTROLS: u16 = 710u16;
pub const HID_USAGE_CONSUMER_INVOKEDISMISS_EMOJI_PICKER: u16 = 217u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT: u16 = 7u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT_AUTO: u16 = 127u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT_LEVEL_SUGGESTION: u16 = 1303u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT_OOC: u16 = 124u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT_SET_LEVEL: u16 = 123u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT_SET_MAXIMUM: u16 = 126u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_BACKLIGHT_SET_MINIMUM: u16 = 125u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_BRIGHTNESS_DECREMENT: u16 = 122u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_BRIGHTNESS_INCREMENT: u16 = 121u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_BRIGHTNESS_NEXT: u16 = 1301u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_BRIGHTNESS_PREVIOUS: u16 = 1302u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_FORM_FACTOR: u16 = 705u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_IETF_LANGUAGE_TAG_INDEX: u16 = 709u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_INPUT_ASSIST_ACCEPT: u16 = 715u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_INPUT_ASSIST_CANCEL: u16 = 716u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_INPUT_ASSIST_NEXT: u16 = 712u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_INPUT_ASSIST_NEXT_GROUP: u16 = 714u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_INPUT_ASSIST_PREVIOUS: u16 = 711u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_INPUT_ASSIST_PREVIOUS_GROUP: u16 = 713u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_KEY_TYPE: u16 = 706u16;
pub const HID_USAGE_CONSUMER_KEYBOARD_PHYSICAL_LAYOUT: u16 = 707u16;
pub const HID_USAGE_CONSUMER_LIGHT_ENABLE: u16 = 258u16;
pub const HID_USAGE_CONSUMER_LIGHT_ILLUMINATION_LEVEL: u16 = 259u16;
pub const HID_USAGE_CONSUMER_LONG_PLAY: u16 = 243u16;
pub const HID_USAGE_CONSUMER_LOUDNESS: u16 = 231u16;
pub const HID_USAGE_CONSUMER_MARK: u16 = 194u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECTION: u16 = 135u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_CABLE: u16 = 151u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_CALL: u16 = 155u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_CD: u16 = 145u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_COMPUTER: u16 = 136u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_DVD: u16 = 139u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_GAMES: u16 = 143u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_HOME: u16 = 154u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_MESSAGES: u16 = 144u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_PROGRAM_GUIDE: u16 = 141u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_SAP: u16 = 158u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_SATELLITE: u16 = 152u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_SECURITY: u16 = 153u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_TAPE: u16 = 150u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_TELEPHONE: u16 = 140u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_TUNER: u16 = 147u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_TV: u16 = 137u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_VCR: u16 = 146u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_VIDEO_PHONE: u16 = 142u16;
pub const HID_USAGE_CONSUMER_MEDIA_SELECT_WWW: u16 = 138u16;
pub const HID_USAGE_CONSUMER_MEDICAL_ALARM: u16 = 269u16;
pub const HID_USAGE_CONSUMER_MENU: u16 = 64u16;
pub const HID_USAGE_CONSUMER_MENU_DOWN: u16 = 67u16;
pub const HID_USAGE_CONSUMER_MENU_ESCAPE: u16 = 70u16;
pub const HID_USAGE_CONSUMER_MENU_LEFT: u16 = 68u16;
pub const HID_USAGE_CONSUMER_MENU_PICK: u16 = 65u16;
pub const HID_USAGE_CONSUMER_MENU_RIGHT: u16 = 69u16;
pub const HID_USAGE_CONSUMER_MENU_UP: u16 = 66u16;
pub const HID_USAGE_CONSUMER_MENU_VALUE_DECREASE: u16 = 72u16;
pub const HID_USAGE_CONSUMER_MENU_VALUE_INCREASE: u16 = 71u16;
pub const HID_USAGE_CONSUMER_MICROPHONE: u16 = 4u16;
pub const HID_USAGE_CONSUMER_MODE_STEP: u16 = 130u16;
pub const HID_USAGE_CONSUMER_MONTHLY: u16 = 164u16;
pub const HID_USAGE_CONSUMER_MOTION: u16 = 266u16;
pub const HID_USAGE_CONSUMER_MPX: u16 = 232u16;
pub const HID_USAGE_CONSUMER_MUTE: u16 = 226u16;
pub const HID_USAGE_CONSUMER_NUMERIC_KEY_PAD: u16 = 2u16;
pub const HID_USAGE_CONSUMER_ONCE: u16 = 161u16;
pub const HID_USAGE_CONSUMER_ORDER_MOVIE: u16 = 133u16;
pub const HID_USAGE_CONSUMER_PAUSE: u16 = 177u16;
pub const HID_USAGE_CONSUMER_PICTUREINPICTURE_SWAP: u16 = 104u16;
pub const HID_USAGE_CONSUMER_PICTUREINPICTURE_TOGGLE: u16 = 103u16;
pub const HID_USAGE_CONSUMER_PLAY: u16 = 176u16;
pub const HID_USAGE_CONSUMER_PLAYBACK_SPEED: u16 = 241u16;
pub const HID_USAGE_CONSUMER_PLAYSKIP: u16 = 206u16;
pub const HID_USAGE_CONSUMER_PLAY_PAUSE: u16 = 205u16;
pub const HID_USAGE_CONSUMER_POLICE_ALARM: u16 = 264u16;
pub const HID_USAGE_CONSUMER_POWER: u16 = 48u16;
pub const HID_USAGE_CONSUMER_PRIVACY_SCREEN_LEVEL_DECREMENT: u16 = 721u16;
pub const HID_USAGE_CONSUMER_PRIVACY_SCREEN_LEVEL_INCREMENT: u16 = 722u16;
pub const HID_USAGE_CONSUMER_PRIVACY_SCREEN_LEVEL_MAXIMUM: u16 = 724u16;
pub const HID_USAGE_CONSUMER_PRIVACY_SCREEN_LEVEL_MINIMUM: u16 = 723u16;
pub const HID_USAGE_CONSUMER_PRIVACY_SCREEN_TOGGLE: u16 = 720u16;
pub const HID_USAGE_CONSUMER_PROGRAMMABLE_BUTTONS: u16 = 3u16;
pub const HID_USAGE_CONSUMER_PROXIMITY: u16 = 265u16;
pub const HID_USAGE_CONSUMER_QUIT: u16 = 148u16;
pub const HID_USAGE_CONSUMER_RANDOM_PLAY: u16 = 185u16;
pub const HID_USAGE_CONSUMER_RECALL_LAST: u16 = 131u16;
pub const HID_USAGE_CONSUMER_RECORD: u16 = 178u16;
pub const HID_USAGE_CONSUMER_RED_MENU_BUTTON: u16 = 105u16;
pub const HID_USAGE_CONSUMER_REPEAT: u16 = 188u16;
pub const HID_USAGE_CONSUMER_REPEAT_FROM_MARK: u16 = 196u16;
pub const HID_USAGE_CONSUMER_RESET: u16 = 49u16;
pub const HID_USAGE_CONSUMER_RETURN_TO_MARK: u16 = 197u16;
pub const HID_USAGE_CONSUMER_REWIND: u16 = 180u16;
pub const HID_USAGE_CONSUMER_ROOM_TEMPERATURE: u16 = 261u16;
pub const HID_USAGE_CONSUMER_SCAN_NEXT_TRACK: u16 = 181u16;
pub const HID_USAGE_CONSUMER_SCAN_PREV_TRACK: u16 = 182u16;
pub const HID_USAGE_CONSUMER_SEARCH_MARK_BACKWARDS: u16 = 199u16;
pub const HID_USAGE_CONSUMER_SEARCH_MARK_FORWARD: u16 = 198u16;
pub const HID_USAGE_CONSUMER_SECURITY_ENABLE: u16 = 262u16;
pub const HID_USAGE_CONSUMER_SELECTION: u16 = 128u16;
pub const HID_USAGE_CONSUMER_SELECT_DISC: u16 = 186u16;
pub const HID_USAGE_CONSUMER_SHOW_COUNTER: u16 = 201u16;
pub const HID_USAGE_CONSUMER_SLEEP: u16 = 50u16;
pub const HID_USAGE_CONSUMER_SLEEP_AFTER: u16 = 51u16;
pub const HID_USAGE_CONSUMER_SLEEP_MODE: u16 = 52u16;
pub const HID_USAGE_CONSUMER_SLOW: u16 = 245u16;
pub const HID_USAGE_CONSUMER_SLOW_TRACKING: u16 = 191u16;
pub const HID_USAGE_CONSUMER_SNAPSHOT: u16 = 101u16;
pub const HID_USAGE_CONSUMER_SPEAKER_SYSTEM: u16 = 352u16;
pub const HID_USAGE_CONSUMER_SPEED_SELECT: u16 = 240u16;
pub const HID_USAGE_CONSUMER_STANDARD_PLAY: u16 = 242u16;
pub const HID_USAGE_CONSUMER_START_OR_STOP_VOICE_DICTATION_SESSION: u16 = 216u16;
pub const HID_USAGE_CONSUMER_STILL: u16 = 102u16;
pub const HID_USAGE_CONSUMER_STOP: u16 = 183u16;
pub const HID_USAGE_CONSUMER_STOPEJECT: u16 = 204u16;
pub const HID_USAGE_CONSUMER_SUBCHANNEL: u16 = 368u16;
pub const HID_USAGE_CONSUMER_SUBCHANNEL_DECREMENT: u16 = 370u16;
pub const HID_USAGE_CONSUMER_SUBCHANNEL_INCREMENT: u16 = 369u16;
pub const HID_USAGE_CONSUMER_SURROUND_MODE: u16 = 230u16;
pub const HID_USAGE_CONSUMER_TRACKING: u16 = 189u16;
pub const HID_USAGE_CONSUMER_TRACKING_DECREMENT: u16 = 203u16;
pub const HID_USAGE_CONSUMER_TRACKING_INCREMENT: u16 = 202u16;
pub const HID_USAGE_CONSUMER_TRACK_NORMAL: u16 = 190u16;
pub const HID_USAGE_CONSUMER_TREBLE: u16 = 228u16;
pub const HID_USAGE_CONSUMER_TREBLE_DECREMENT: u16 = 341u16;
pub const HID_USAGE_CONSUMER_TREBLE_INCREMENT: u16 = 340u16;
pub const HID_USAGE_CONSUMER_VCRTV: u16 = 99u16;
pub const HID_USAGE_CONSUMER_VCR_PLUS: u16 = 160u16;
pub const HID_USAGE_CONSUMER_VENDOR_SPECIFIC_KEYBOARD_PHYSICAL_LAYOUT: u16 = 708u16;
pub const HID_USAGE_CONSUMER_VOICE_COMMAND: u16 = 207u16;
pub const HID_USAGE_CONSUMER_VOLUME: u16 = 224u16;
pub const HID_USAGE_CONSUMER_VOLUME_DECREMENT: u16 = 234u16;
pub const HID_USAGE_CONSUMER_VOLUME_INCREMENT: u16 = 233u16;
pub const HID_USAGE_CONSUMER_WEEKLY: u16 = 163u16;
pub const HID_USAGE_CONSUMER_YELLOW_MENU_BUTTON: u16 = 108u16;
pub const HID_USAGE_DIGITIZER_3D_DIGITIZER: u16 = 8u16;
pub const HID_USAGE_DIGITIZER_ALTITUDE: u16 = 64u16;
pub const HID_USAGE_DIGITIZER_ARMATURE: u16 = 11u16;
pub const HID_USAGE_DIGITIZER_ARTICULATED_ARM: u16 = 10u16;
pub const HID_USAGE_DIGITIZER_AZIMUTH: u16 = 63u16;
pub const HID_USAGE_DIGITIZER_BARREL_PRESSURE: u16 = 49u16;
pub const HID_USAGE_DIGITIZER_BARREL_SWITCH: u16 = 68u16;
pub const HID_USAGE_DIGITIZER_BATTERY_STRENGTH: u16 = 59u16;
pub const HID_USAGE_DIGITIZER_BRUSH: u16 = 118u16;
pub const HID_USAGE_DIGITIZER_BUTTON_PRESS_THRESHOLD: u16 = 176u16;
pub const HID_USAGE_DIGITIZER_BUTTON_SWITCH: u16 = 88u16;
pub const HID_USAGE_DIGITIZER_CHARACTER_GESTURE: u16 = 36u16;
pub const HID_USAGE_DIGITIZER_CHARACTER_GESTURE_DATA: u16 = 99u16;
pub const HID_USAGE_DIGITIZER_CHARACTER_GESTURE_DATA_LENGTH: u16 = 98u16;
pub const HID_USAGE_DIGITIZER_CHISEL_MARKER: u16 = 117u16;
pub const HID_USAGE_DIGITIZER_CONTACT_COUNT: u16 = 84u16;
pub const HID_USAGE_DIGITIZER_CONTACT_COUNT_MAXIMUM: u16 = 85u16;
pub const HID_USAGE_DIGITIZER_CONTACT_IDENTIFIER: u16 = 81u16;
pub const HID_USAGE_DIGITIZER_COORD_MEASURING: u16 = 7u16;
pub const HID_USAGE_DIGITIZER_DATA_VALID: u16 = 55u16;
pub const HID_USAGE_DIGITIZER_DEVICE_CONFIGURATION: u16 = 14u16;
pub const HID_USAGE_DIGITIZER_DEVICE_IDENTIFIER: u16 = 83u16;
pub const HID_USAGE_DIGITIZER_DEVICE_MODE: u16 = 82u16;
pub const HID_USAGE_DIGITIZER_DEVICE_SETTINGS: u16 = 35u16;
pub const HID_USAGE_DIGITIZER_DEVICE_SUPPORTED_PROTOCOLS: u16 = 147u16;
pub const HID_USAGE_DIGITIZER_DIGITIZER: u16 = 1u16;
pub const HID_USAGE_DIGITIZER_DIGITIZER_DIAGNOSTIC: u16 = 128u16;
pub const HID_USAGE_DIGITIZER_DIGITIZER_ERROR: u16 = 129u16;
pub const HID_USAGE_DIGITIZER_ERASER: u16 = 69u16;
pub const HID_USAGE_DIGITIZER_ERR_CHARGE_LOW: u16 = 133u16;
pub const HID_USAGE_DIGITIZER_ERR_FULL_TRANS_FEATURES_UNAVAILABLE: u16 = 132u16;
pub const HID_USAGE_DIGITIZER_ERR_NORMAL_STATUS: u16 = 130u16;
pub const HID_USAGE_DIGITIZER_ERR_TRANSDUCERS_EXCEEDED: u16 = 131u16;
pub const HID_USAGE_DIGITIZER_FINGER: u16 = 34u16;
pub const HID_USAGE_DIGITIZER_FREE_SPACE_WAND: u16 = 13u16;
pub const HID_USAGE_DIGITIZER_GESTURE_CHARACTER_ENABLE: u16 = 109u16;
pub const HID_USAGE_DIGITIZER_GESTURE_CHARACTER_ENCODING: u16 = 100u16;
pub const HID_USAGE_DIGITIZER_GESTURE_CHARACTER_QUALITY: u16 = 97u16;
pub const HID_USAGE_DIGITIZER_HEAT_MAP: u16 = 15u16;
pub const HID_USAGE_DIGITIZER_HEAT_MAP_FRAME_DATA: u16 = 108u16;
pub const HID_USAGE_DIGITIZER_HEAT_MAP_PROTOCOL_VENDOR_ID: u16 = 106u16;
pub const HID_USAGE_DIGITIZER_HEAT_MAP_PROTOCOL_VERSION: u16 = 107u16;
pub const HID_USAGE_DIGITIZER_HEIGHT: u16 = 73u16;
pub const HID_USAGE_DIGITIZER_HIGHLIGHTER: u16 = 116u16;
pub const HID_USAGE_DIGITIZER_INK: u16 = 114u16;
pub const HID_USAGE_DIGITIZER_INVERT: u16 = 60u16;
pub const HID_USAGE_DIGITIZER_IN_RANGE: u16 = 50u16;
pub const HID_USAGE_DIGITIZER_LATENCY_MODE: u16 = 96u16;
pub const HID_USAGE_DIGITIZER_LIGHT_PEN: u16 = 3u16;
pub const HID_USAGE_DIGITIZER_MICROSOFT_PEN_PROTOCOL: u16 = 152u16;
pub const HID_USAGE_DIGITIZER_MULTI_POINT: u16 = 12u16;
pub const HID_USAGE_DIGITIZER_NO_PREFERENCE: u16 = 119u16;
pub const HID_USAGE_DIGITIZER_NO_PREFERRED_COLOR: u16 = 111u16;
pub const HID_USAGE_DIGITIZER_NO_PROTOCOL: u16 = 149u16;
pub const HID_USAGE_DIGITIZER_PAD_TYPE: u16 = 89u16;
pub const HID_USAGE_DIGITIZER_PEN: u16 = 2u16;
pub const HID_USAGE_DIGITIZER_PENCIL: u16 = 115u16;
pub const HID_USAGE_DIGITIZER_PREFERRED_COLOR: u16 = 92u16;
pub const HID_USAGE_DIGITIZER_PREFERRED_COLOR_IS_LOCKED: u16 = 93u16;
pub const HID_USAGE_DIGITIZER_PREFERRED_LINE_STYLE: u16 = 112u16;
pub const HID_USAGE_DIGITIZER_PREFERRED_LINE_STYLE_IS_LOCKED: u16 = 113u16;
pub const HID_USAGE_DIGITIZER_PREFERRED_LINE_WIDTH: u16 = 94u16;
pub const HID_USAGE_DIGITIZER_PREFERRED_LINE_WIDTH_IS_LOCKED: u16 = 95u16;
pub const HID_USAGE_DIGITIZER_PROG_CHANGE_KEYS: u16 = 58u16;
pub const HID_USAGE_DIGITIZER_PUCK: u16 = 33u16;
pub const HID_USAGE_DIGITIZER_QUALITY: u16 = 54u16;
pub const HID_USAGE_DIGITIZER_REPORT_RATE: u16 = 161u16;
pub const HID_USAGE_DIGITIZER_SCAN_TIME: u16 = 86u16;
pub const HID_USAGE_DIGITIZER_SECONDARY_BARREL_SWITCH: u16 = 90u16;
pub const HID_USAGE_DIGITIZER_SECONDARY_TIP_SWITCH: u16 = 67u16;
pub const HID_USAGE_DIGITIZER_STEREO_PLOTTER: u16 = 9u16;
pub const HID_USAGE_DIGITIZER_STYLUS: u16 = 32u16;
pub const HID_USAGE_DIGITIZER_SUPPORTED_REPORT_RATES: u16 = 160u16;
pub const HID_USAGE_DIGITIZER_SURFACE_SWITCH: u16 = 87u16;
pub const HID_USAGE_DIGITIZER_SWITCH_DISABLED: u16 = 163u16;
pub const HID_USAGE_DIGITIZER_SWITCH_UNIMPLEMENTED: u16 = 164u16;
pub const HID_USAGE_DIGITIZER_TABLET_FUNC_KEYS: u16 = 57u16;
pub const HID_USAGE_DIGITIZER_TABLET_PICK: u16 = 70u16;
pub const HID_USAGE_DIGITIZER_TAP: u16 = 53u16;
pub const HID_USAGE_DIGITIZER_TIP_PRESSURE: u16 = 48u16;
pub const HID_USAGE_DIGITIZER_TIP_SWITCH: u16 = 66u16;
pub const HID_USAGE_DIGITIZER_TOUCH: u16 = 51u16;
pub const HID_USAGE_DIGITIZER_TOUCH_PAD: u16 = 5u16;
pub const HID_USAGE_DIGITIZER_TOUCH_SCREEN: u16 = 4u16;
pub const HID_USAGE_DIGITIZER_TOUCH_VALID: u16 = 71u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_CONNECTED: u16 = 162u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_INDEX: u16 = 56u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_INDEX_SELECTOR: u16 = 166u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_PRODUCT: u16 = 146u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SERIAL: u16 = 91u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SERIAL_PART2: u16 = 110u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SOFTWARE_INFO: u16 = 144u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SUPPORTED_PROTOCOLS: u16 = 148u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SWITCHES: u16 = 165u16;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_VENDOR: u16 = 145u16;
pub const HID_USAGE_DIGITIZER_TWIST: u16 = 65u16;
pub const HID_USAGE_DIGITIZER_UNTOUCH: u16 = 52u16;
pub const HID_USAGE_DIGITIZER_USI_PROTOCOL: u16 = 151u16;
pub const HID_USAGE_DIGITIZER_UTF16_BIG_ENDIAN_CHARACTER_GESTURE_ENCODING: u16 = 103u16;
pub const HID_USAGE_DIGITIZER_UTF16_LITTLE_ENDIAN_CHARACTER_GESTURE_ENCODING: u16 = 102u16;
pub const HID_USAGE_DIGITIZER_UTF32_BIG_ENDIAN_CHARACTER_GESTURE_ENCODING: u16 = 105u16;
pub const HID_USAGE_DIGITIZER_UTF32_LITTLE_ENDIAN_CHARACTER_GESTURE_ENCODING: u16 = 104u16;
pub const HID_USAGE_DIGITIZER_UTF8_CHARACTER_GESTURE_ENCODING: u16 = 101u16;
pub const HID_USAGE_DIGITIZER_WACOM_AES_PROTOCOL: u16 = 150u16;
pub const HID_USAGE_DIGITIZER_WHITE_BOARD: u16 = 6u16;
pub const HID_USAGE_DIGITIZER_WIDTH: u16 = 72u16;
pub const HID_USAGE_DIGITIZER_X_TILT: u16 = 61u16;
pub const HID_USAGE_DIGITIZER_Y_TILT: u16 = 62u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_CALIBRATED_SCREEN_HEIGHT: u16 = 517u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_CALIBRATED_SCREEN_WIDTH: u16 = 516u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_CAPABILITIES: u16 = 17u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_CONFIGURATION: u16 = 18u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_CONFIGURATION_STATUS: u16 = 769u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_CONTROL: u16 = 20u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_DEVICE_MODE_REQUEST: u16 = 1024u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_DISPLAY_MANUFACTURER_DATE: u16 = 515u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_DISPLAY_MANUFACTURER_ID: u16 = 512u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_DISPLAY_PRODUCT_ID: u16 = 513u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_DISPLAY_SERIAL_NUMBER: u16 = 514u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_EYE_TRACKER: u16 = 1u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_GAZE_POINT: u16 = 36u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_HEAD_DIRECTION_POINT: u16 = 40u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_HEAD_POSITION: u16 = 39u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_HEAD_TRACKER: u16 = 2u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_LEFT_EYE_POSITION: u16 = 37u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_MAXIMUM_SCREEN_PLANE_HEIGHT: u16 = 261u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_MAXIMUM_SCREEN_PLANE_WIDTH: u16 = 260u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_MAXIMUM_TRACKING_DISTANCE: u16 = 259u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_MINIMUM_TRACKING_DISTANCE: u16 = 257u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_OPTIMUM_TRACKING_DISTANCE: u16 = 258u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_POSITION_X: u16 = 33u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_POSITION_Y: u16 = 34u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_POSITION_Z: u16 = 35u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_RIGHT_EYE_POSITION: u16 = 38u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_ROTATION_ABOUT_X_AXIS: u16 = 41u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_ROTATION_ABOUT_Y_AXIS: u16 = 42u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_ROTATION_ABOUT_Z_AXIS: u16 = 43u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_SAMPLING_FREQUENCY: u16 = 768u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_SENSOR_TIMESTAMP: u16 = 32u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_STATUS: u16 = 19u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_TRACKER_QUALITY: u16 = 256u16;
pub const HID_USAGE_EYE_AND_HEAD_TRACKERS_TRACKING_DATA: u16 = 16u16;
pub const HID_USAGE_FIDO_ALLIANCE_INPUT_REPORT_DATA: u16 = 32u16;
pub const HID_USAGE_FIDO_ALLIANCE_OUTPUT_REPORT_DATA: u16 = 33u16;
pub const HID_USAGE_FIDO_ALLIANCE_U2F_AUTHENTICATOR_DEVICE: u16 = 1u16;
pub const HID_USAGE_GAME_3D_GAME_CONTROLLER: u16 = 1u16;
pub const HID_USAGE_GAME_BUMP: u16 = 44u16;
pub const HID_USAGE_GAME_FLIPPER: u16 = 42u16;
pub const HID_USAGE_GAME_FORMFITTING_GAMEPAD: u16 = 58u16;
pub const HID_USAGE_GAME_GAMEPAD_FIRE_JUMP: u16 = 55u16;
pub const HID_USAGE_GAME_GAMEPAD_TRIGGER: u16 = 57u16;
pub const HID_USAGE_GAME_GUN_AUTOMATIC: u16 = 53u16;
pub const HID_USAGE_GAME_GUN_BOLT: u16 = 48u16;
pub const HID_USAGE_GAME_GUN_BURST: u16 = 52u16;
pub const HID_USAGE_GAME_GUN_CLIP: u16 = 49u16;
pub const HID_USAGE_GAME_GUN_DEVICE: u16 = 3u16;
pub const HID_USAGE_GAME_GUN_SAFETY: u16 = 54u16;
pub const HID_USAGE_GAME_GUN_SELECTOR: u16 = 50u16;
pub const HID_USAGE_GAME_GUN_SINGLE_SHOT: u16 = 51u16;
pub const HID_USAGE_GAME_LEAN_FORWARD_BACK: u16 = 40u16;
pub const HID_USAGE_GAME_LEAN_RIGHT_LEFT: u16 = 39u16;
pub const HID_USAGE_GAME_MOVE_FORWARD_BACK: u16 = 37u16;
pub const HID_USAGE_GAME_MOVE_RIGHT_LEFT: u16 = 36u16;
pub const HID_USAGE_GAME_MOVE_UP_DOWN: u16 = 38u16;
pub const HID_USAGE_GAME_NEW_GAME: u16 = 45u16;
pub const HID_USAGE_GAME_PINBALL_DEVICE: u16 = 2u16;
pub const HID_USAGE_GAME_PITCH_FORWARD_BACK: u16 = 34u16;
pub const HID_USAGE_GAME_PLAYER: u16 = 47u16;
pub const HID_USAGE_GAME_POV: u16 = 32u16;
pub const HID_USAGE_GAME_POV_HEIGHT: u16 = 41u16;
pub const HID_USAGE_GAME_ROLL_RIGHT_LEFT: u16 = 35u16;
pub const HID_USAGE_GAME_SECONDARY_FLIPPER: u16 = 43u16;
pub const HID_USAGE_GAME_SHOOT_BALL: u16 = 46u16;
pub const HID_USAGE_GAME_TURN_RIGHT_LEFT: u16 = 33u16;
pub const HID_USAGE_GENERIC_ASSISTIVE_CONTROL: u16 = 16u16;
pub const HID_USAGE_GENERIC_BYTE_COUNT: u16 = 59u16;
pub const HID_USAGE_GENERIC_CALL_ACTIVE_LED: u16 = 224u16;
pub const HID_USAGE_GENERIC_CALL_MUTE_LED: u16 = 226u16;
pub const HID_USAGE_GENERIC_CALL_MUTE_TOGGLE: u16 = 225u16;
pub const HID_USAGE_GENERIC_CALL_STATE_MANAGEMENT_CONTROL: u16 = 19u16;
pub const HID_USAGE_GENERIC_CHASSIS_ENCLOSURE: u16 = 197u16;
pub const HID_USAGE_GENERIC_COMPUTER_CHASSIS_DEVICE: u16 = 11u16;
pub const HID_USAGE_GENERIC_CONTROL_ENABLE: u16 = 203u16;
pub const HID_USAGE_GENERIC_COOLANT_CRITICAL_LEVEL: u16 = 195u16;
pub const HID_USAGE_GENERIC_COOLANT_LEVEL: u16 = 194u16;
pub const HID_USAGE_GENERIC_COOLANT_PUMP: u16 = 196u16;
pub const HID_USAGE_GENERIC_COUNTED_BUFFER: u16 = 58u16;
pub const HID_USAGE_GENERIC_DEVICE_BACKGROUNDNONUSER_CONTROLS: u16 = 1u16;
pub const HID_USAGE_GENERIC_DEVICE_BATTERY_STRENGTH: u16 = 32u16;
pub const HID_USAGE_GENERIC_DEVICE_BOTH_HANDS: u16 = 52u16;
pub const HID_USAGE_GENERIC_DEVICE_DISCOVER_WIRELESS_CONTROL: u16 = 35u16;
pub const HID_USAGE_GENERIC_DEVICE_DOCK: u16 = 17u16;
pub const HID_USAGE_GENERIC_DEVICE_EITHER_HAND: u16 = 49u16;
pub const HID_USAGE_GENERIC_DEVICE_GRIP_POSE_OFFSET: u16 = 64u16;
pub const HID_USAGE_GENERIC_DEVICE_HANDEDNESS: u16 = 48u16;
pub const HID_USAGE_GENERIC_DEVICE_HARDWARE_VERSION: u16 = 44u16;
pub const HID_USAGE_GENERIC_DEVICE_LEFT_HAND: u16 = 50u16;
pub const HID_USAGE_GENERIC_DEVICE_MAJOR: u16 = 45u16;
pub const HID_USAGE_GENERIC_DEVICE_MINOR: u16 = 46u16;
pub const HID_USAGE_GENERIC_DEVICE_POINTER_POSE_OFFSET: u16 = 65u16;
pub const HID_USAGE_GENERIC_DEVICE_PROTOCOL_VERSION: u16 = 43u16;
pub const HID_USAGE_GENERIC_DEVICE_REVISION: u16 = 47u16;
pub const HID_USAGE_GENERIC_DEVICE_RF_SIGNAL_STRENGTH: u16 = 41u16;
pub const HID_USAGE_GENERIC_DEVICE_RIGHT_HAND: u16 = 51u16;
pub const HID_USAGE_GENERIC_DEVICE_SECURITY_CODE_CHAR_ENTERED: u16 = 36u16;
pub const HID_USAGE_GENERIC_DEVICE_SECURITY_CODE_CHAR_ERASED: u16 = 37u16;
pub const HID_USAGE_GENERIC_DEVICE_SECURITY_CODE_CLEARED: u16 = 38u16;
pub const HID_USAGE_GENERIC_DEVICE_SEQUENCE_ID: u16 = 39u16;
pub const HID_USAGE_GENERIC_DEVICE_SEQUENCE_ID_RESET: u16 = 40u16;
pub const HID_USAGE_GENERIC_DEVICE_SOFTWARE_VERSION: u16 = 42u16;
pub const HID_USAGE_GENERIC_DEVICE_WIRELESS_CHANNEL: u16 = 33u16;
pub const HID_USAGE_GENERIC_DEVICE_WIRELESS_ID: u16 = 34u16;
pub const HID_USAGE_GENERIC_DIAL: u16 = 55u16;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE: u16 = 18u16;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_DISPLAY_OCCLUSION: u16 = 213u16;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_DOCKING_STATE: u16 = 212u16;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_OBJECT_TYPE: u16 = 214u16;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_PRIMARY_USAGE_ID: u16 = 211u16;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_PRIMARY_USAGE_PAGE: u16 = 210u16;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_UNIQUE_ID: u16 = 208u16;
pub const HID_USAGE_GENERIC_DOCKABLE_DEVICE_VENDOR_ID: u16 = 209u16;
pub const HID_USAGE_GENERIC_DPAD_DOWN: u16 = 145u16;
pub const HID_USAGE_GENERIC_DPAD_LEFT: u16 = 147u16;
pub const HID_USAGE_GENERIC_DPAD_RIGHT: u16 = 146u16;
pub const HID_USAGE_GENERIC_DPAD_UP: u16 = 144u16;
pub const HID_USAGE_GENERIC_FEATURE_NOTIFICATION: u16 = 71u16;
pub const HID_USAGE_GENERIC_GAMEPAD: u16 = 5u16;
pub const HID_USAGE_GENERIC_HATSWITCH: u16 = 57u16;
pub const HID_USAGE_GENERIC_INDEX_TRIGGER: u16 = 148u16;
pub const HID_USAGE_GENERIC_INTERACTIVE_CONTROL: u16 = 14u16;
pub const HID_USAGE_GENERIC_JOYSTICK: u16 = 4u16;
pub const HID_USAGE_GENERIC_KEYBOARD: u16 = 6u16;
pub const HID_USAGE_GENERIC_KEYPAD: u16 = 7u16;
pub const HID_USAGE_GENERIC_MOTION_WAKEUP: u16 = 60u16;
pub const HID_USAGE_GENERIC_MOUSE: u16 = 2u16;
pub const HID_USAGE_GENERIC_MULTI_AXIS_CONTROLLER: u16 = 8u16;
pub const HID_USAGE_GENERIC_PALM_TRIGGER: u16 = 149u16;
pub const HID_USAGE_GENERIC_POINTER: u16 = 1u16;
pub const HID_USAGE_GENERIC_PORTABLE_DEVICE_CONTROL: u16 = 13u16;
pub const HID_USAGE_GENERIC_QW: u16 = 76u16;
pub const HID_USAGE_GENERIC_QX: u16 = 73u16;
pub const HID_USAGE_GENERIC_QY: u16 = 74u16;
pub const HID_USAGE_GENERIC_QZ: u16 = 75u16;
pub const HID_USAGE_GENERIC_RESOLUTION_MULTIPLIER: u16 = 72u16;
pub const HID_USAGE_GENERIC_RPM: u16 = 193u16;
pub const HID_USAGE_GENERIC_RX: u16 = 51u16;
pub const HID_USAGE_GENERIC_RY: u16 = 52u16;
pub const HID_USAGE_GENERIC_RZ: u16 = 53u16;
pub const HID_USAGE_GENERIC_SELECT: u16 = 62u16;
pub const HID_USAGE_GENERIC_SENSOR_ZONE: u16 = 192u16;
pub const HID_USAGE_GENERIC_SLIDER: u16 = 54u16;
pub const HID_USAGE_GENERIC_SPATIAL_CONTROLLER: u16 = 15u16;
pub const HID_USAGE_GENERIC_START: u16 = 61u16;
pub const HID_USAGE_GENERIC_SYSCTL_ACCESSIBILITY_BINDING: u16 = 170u16;
pub const HID_USAGE_GENERIC_SYSCTL_APP_BREAK: u16 = 165u16;
pub const HID_USAGE_GENERIC_SYSCTL_APP_DBG_BREAK: u16 = 166u16;
pub const HID_USAGE_GENERIC_SYSCTL_APP_MENU: u16 = 134u16;
pub const HID_USAGE_GENERIC_SYSCTL_COLD_RESTART: u16 = 142u16;
pub const HID_USAGE_GENERIC_SYSCTL_CONTEXT_MENU: u16 = 132u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISMISS_NOTIFICATION: u16 = 154u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_AUTOSCALE: u16 = 183u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_BOTH: u16 = 179u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_DUAL: u16 = 180u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_EXTERNAL: u16 = 178u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_INTERNAL: u16 = 177u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_INVERT: u16 = 176u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_SWAP: u16 = 182u16;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_TOGGLE: u16 = 181u16;
pub const HID_USAGE_GENERIC_SYSCTL_DOCK: u16 = 160u16;
pub const HID_USAGE_GENERIC_SYSCTL_FN: u16 = 151u16;
pub const HID_USAGE_GENERIC_SYSCTL_FN_LOCK: u16 = 152u16;
pub const HID_USAGE_GENERIC_SYSCTL_FN_LOCK_INDICATOR: u16 = 153u16;
pub const HID_USAGE_GENERIC_SYSCTL_HELP_MENU: u16 = 135u16;
pub const HID_USAGE_GENERIC_SYSCTL_HIBERNATE: u16 = 168u16;
pub const HID_USAGE_GENERIC_SYSCTL_MAIN_MENU: u16 = 133u16;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_DOWN: u16 = 141u16;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_EXIT: u16 = 136u16;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_LEFT: u16 = 139u16;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_RIGHT: u16 = 138u16;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_SELECT: u16 = 137u16;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_UP: u16 = 140u16;
pub const HID_USAGE_GENERIC_SYSCTL_MICROPHONE_MUTE: u16 = 169u16;
pub const HID_USAGE_GENERIC_SYSCTL_MUTE: u16 = 167u16;
pub const HID_USAGE_GENERIC_SYSCTL_POWER: u16 = 129u16;
pub const HID_USAGE_GENERIC_SYSCTL_SETUP: u16 = 162u16;
pub const HID_USAGE_GENERIC_SYSCTL_SLEEP: u16 = 130u16;
pub const HID_USAGE_GENERIC_SYSCTL_SYS_BREAK: u16 = 163u16;
pub const HID_USAGE_GENERIC_SYSCTL_SYS_DBG_BREAK: u16 = 164u16;
pub const HID_USAGE_GENERIC_SYSCTL_UNDOCK: u16 = 161u16;
pub const HID_USAGE_GENERIC_SYSCTL_WAKE: u16 = 131u16;
pub const HID_USAGE_GENERIC_SYSCTL_WARM_RESTART: u16 = 143u16;
pub const HID_USAGE_GENERIC_SYSTEM_CTL: u16 = 128u16;
pub const HID_USAGE_GENERIC_SYSTEM_DISPLAY_ROTATION_LOCK_BUTTON: u16 = 201u16;
pub const HID_USAGE_GENERIC_SYSTEM_DISPLAY_ROTATION_LOCK_SLIDER_SWITCH: u16 = 202u16;
pub const HID_USAGE_GENERIC_SYSTEM_DO_NOT_DISTURB: u16 = 155u16;
pub const HID_USAGE_GENERIC_TABLET_PC_SYSTEM_CTL: u16 = 9u16;
pub const HID_USAGE_GENERIC_THUMBSTICK: u16 = 150u16;
pub const HID_USAGE_GENERIC_VBRX: u16 = 67u16;
pub const HID_USAGE_GENERIC_VBRY: u16 = 68u16;
pub const HID_USAGE_GENERIC_VBRZ: u16 = 69u16;
pub const HID_USAGE_GENERIC_VNO: u16 = 70u16;
pub const HID_USAGE_GENERIC_VX: u16 = 64u16;
pub const HID_USAGE_GENERIC_VY: u16 = 65u16;
pub const HID_USAGE_GENERIC_VZ: u16 = 66u16;
pub const HID_USAGE_GENERIC_WATER_COOLING_DEVICE: u16 = 10u16;
pub const HID_USAGE_GENERIC_WHEEL: u16 = 56u16;
pub const HID_USAGE_GENERIC_WIRELESS_RADIO_BUTTON: u16 = 198u16;
pub const HID_USAGE_GENERIC_WIRELESS_RADIO_CONTROLS: u16 = 12u16;
pub const HID_USAGE_GENERIC_WIRELESS_RADIO_LED: u16 = 199u16;
pub const HID_USAGE_GENERIC_WIRELESS_RADIO_SLIDER_SWITCH: u16 = 200u16;
pub const HID_USAGE_GENERIC_X: u16 = 48u16;
pub const HID_USAGE_GENERIC_Y: u16 = 49u16;
pub const HID_USAGE_GENERIC_Z: u16 = 50u16;
pub const HID_USAGE_HAPTICS_AUTO_ASSOCIATED_CONTROL: u16 = 34u16;
pub const HID_USAGE_HAPTICS_AUTO_TRIGGER: u16 = 32u16;
pub const HID_USAGE_HAPTICS_DURATION_LIST: u16 = 17u16;
pub const HID_USAGE_HAPTICS_INTENSITY: u16 = 35u16;
pub const HID_USAGE_HAPTICS_MANUAL_TRIGGER: u16 = 33u16;
pub const HID_USAGE_HAPTICS_REPEAT_COUNT: u16 = 36u16;
pub const HID_USAGE_HAPTICS_RETRIGGER_PERIOD: u16 = 37u16;
pub const HID_USAGE_HAPTICS_SIMPLE_CONTROLLER: u16 = 1u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_BRUSH_CONTINUOUS: u16 = 4111u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_BUZZ: u16 = 4100u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_CHISEL_MARKER_CONTINUOUS: u16 = 4110u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_CLICK: u16 = 4099u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_CUTOFF_TIME: u16 = 40u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_ERASER_CONTINUOUS: u16 = 4112u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_ERROR: u16 = 4106u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_HOVER: u16 = 4104u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_INK_CONTINUOUS: u16 = 4107u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_LIST: u16 = 16u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_MARKER_CONTINUOUS: u16 = 4109u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_NONE: u16 = 4097u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_PENCIL_CONTINUOUS: u16 = 4108u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_PRESS: u16 = 4102u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_RELEASE: u16 = 4103u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_RUMBLE: u16 = 4101u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_SPARKLE_CONTINUOUS: u16 = 4113u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_STOP: u16 = 4098u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_SUCCESS: u16 = 4105u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_VENDOR_ID: u16 = 39u16;
pub const HID_USAGE_HAPTICS_WAVEFORM_VENDOR_PAGE: u16 = 38u16;
pub const HID_USAGE_KEYBOARD_AGAIN: u16 = 121u16;
pub const HID_USAGE_KEYBOARD_ALTERNATE_ERASE: u16 = 153u16;
pub const HID_USAGE_KEYBOARD_APPLICATION: u16 = 101u16;
pub const HID_USAGE_KEYBOARD_BACKSLASH_AND_PIPE: u16 = 49u16;
pub const HID_USAGE_KEYBOARD_CANCEL: u16 = 155u16;
pub const HID_USAGE_KEYBOARD_CAPS_LOCK: u16 = 57u16;
pub const HID_USAGE_KEYBOARD_CLEAR: u16 = 156u16;
pub const HID_USAGE_KEYBOARD_CLEAR_AGAIN: u16 = 162u16;
pub const HID_USAGE_KEYBOARD_COMMA_AND_LESSTHAN: u16 = 54u16;
pub const HID_USAGE_KEYBOARD_COPY: u16 = 124u16;
pub const HID_USAGE_KEYBOARD_CRSEL_PROPS: u16 = 163u16;
pub const HID_USAGE_KEYBOARD_CURRENCY_SUBUNIT: u16 = 181u16;
pub const HID_USAGE_KEYBOARD_CURRENCY_UNIT: u16 = 180u16;
pub const HID_USAGE_KEYBOARD_CUT: u16 = 123u16;
pub const HID_USAGE_KEYBOARD_DASH: u16 = 86u16;
pub const HID_USAGE_KEYBOARD_DASH_AND_UNDERSCORE: u16 = 45u16;
pub const HID_USAGE_KEYBOARD_DECIMAL_SEPARATOR: u16 = 179u16;
pub const HID_USAGE_KEYBOARD_DELETE: u16 = 42u16;
pub const HID_USAGE_KEYBOARD_DELETE_FORWARD: u16 = 76u16;
pub const HID_USAGE_KEYBOARD_DOWNARROW: u16 = 81u16;
pub const HID_USAGE_KEYBOARD_EIGHT_AND_STAR: u16 = 37u16;
pub const HID_USAGE_KEYBOARD_END: u16 = 77u16;
pub const HID_USAGE_KEYBOARD_EQUALS_AND_PLUS: u16 = 46u16;
pub const HID_USAGE_KEYBOARD_ESCAPE: u16 = 41u16;
pub const HID_USAGE_KEYBOARD_EXECUTE: u16 = 116u16;
pub const HID_USAGE_KEYBOARD_EXSEL: u16 = 164u16;
pub const HID_USAGE_KEYBOARD_F1: u16 = 58u16;
pub const HID_USAGE_KEYBOARD_F10: u16 = 67u16;
pub const HID_USAGE_KEYBOARD_F11: u16 = 68u16;
pub const HID_USAGE_KEYBOARD_F12: u16 = 69u16;
pub const HID_USAGE_KEYBOARD_F13: u16 = 104u16;
pub const HID_USAGE_KEYBOARD_F14: u16 = 105u16;
pub const HID_USAGE_KEYBOARD_F15: u16 = 106u16;
pub const HID_USAGE_KEYBOARD_F16: u16 = 107u16;
pub const HID_USAGE_KEYBOARD_F17: u16 = 108u16;
pub const HID_USAGE_KEYBOARD_F18: u16 = 109u16;
pub const HID_USAGE_KEYBOARD_F19: u16 = 110u16;
pub const HID_USAGE_KEYBOARD_F2: u16 = 59u16;
pub const HID_USAGE_KEYBOARD_F20: u16 = 111u16;
pub const HID_USAGE_KEYBOARD_F21: u16 = 112u16;
pub const HID_USAGE_KEYBOARD_F22: u16 = 113u16;
pub const HID_USAGE_KEYBOARD_F23: u16 = 114u16;
pub const HID_USAGE_KEYBOARD_F24: u16 = 115u16;
pub const HID_USAGE_KEYBOARD_F3: u16 = 60u16;
pub const HID_USAGE_KEYBOARD_F4: u16 = 61u16;
pub const HID_USAGE_KEYBOARD_F5: u16 = 62u16;
pub const HID_USAGE_KEYBOARD_F6: u16 = 63u16;
pub const HID_USAGE_KEYBOARD_F7: u16 = 64u16;
pub const HID_USAGE_KEYBOARD_F8: u16 = 65u16;
pub const HID_USAGE_KEYBOARD_F9: u16 = 66u16;
pub const HID_USAGE_KEYBOARD_FIND: u16 = 126u16;
pub const HID_USAGE_KEYBOARD_FIVE_AND_PERCENT: u16 = 34u16;
pub const HID_USAGE_KEYBOARD_FORWARDSLASH_AND_QUESTIONMARK: u16 = 56u16;
pub const HID_USAGE_KEYBOARD_FOUR_AND_DOLLAR: u16 = 33u16;
pub const HID_USAGE_KEYBOARD_GRAVE_ACCENT_AND_TILDE: u16 = 53u16;
pub const HID_USAGE_KEYBOARD_HELP: u16 = 117u16;
pub const HID_USAGE_KEYBOARD_HOME: u16 = 74u16;
pub const HID_USAGE_KEYBOARD_INSERT: u16 = 73u16;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL1: u16 = 135u16;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL2: u16 = 136u16;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL3: u16 = 137u16;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL4: u16 = 138u16;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL5: u16 = 139u16;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL6: u16 = 140u16;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL7: u16 = 141u16;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL8: u16 = 142u16;
pub const HID_USAGE_KEYBOARD_INTERNATIONAL9: u16 = 143u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_A: u16 = 188u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_AMPERSAND: u16 = 199u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_AT: u16 = 206u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_B: u16 = 189u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_BACKSPACE: u16 = 187u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_BANG: u16 = 207u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_BAR: u16 = 201u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_BINARY: u16 = 218u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_C: u16 = 190u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_CARET: u16 = 195u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_CLEAR: u16 = 216u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_CLEAR_ENTRY: u16 = 217u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_COLON: u16 = 203u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_COMMA: u16 = 133u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_D: u16 = 191u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_DECIMAL: u16 = 220u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_DOUBLE_0: u16 = 176u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_DOUBLE_AMPERSAND: u16 = 200u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_DOUBLE_BAR: u16 = 202u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_E: u16 = 192u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_EIGHT_AND_UP_ARROW: u16 = 96u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_ENTER: u16 = 88u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_EQUALS: u16 = 103u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_EQUAL_SIGN: u16 = 134u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_F: u16 = 193u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_FIVE: u16 = 93u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_FORWARDSLASH: u16 = 84u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_FOUR_AND_LEFT_ARROW: u16 = 92u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_GREATER: u16 = 198u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_HASH: u16 = 204u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_HEXADECIMAL: u16 = 221u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_LEFT_BRACE: u16 = 184u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_LEFT_BRACKET: u16 = 182u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_LESS: u16 = 197u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_ADD: u16 = 211u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_CLEAR: u16 = 210u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_DIVIDE: u16 = 214u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_MULTIPLY: u16 = 213u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_RECALL: u16 = 209u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_STORE: u16 = 208u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_MEMORY_SUBTRACT: u16 = 212u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_NINE_AND_PAGEUP: u16 = 97u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_OCTAL: u16 = 219u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_ONE_AND_END: u16 = 89u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_PERCENTAGE: u16 = 196u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_PERIOD_AND_DELETE: u16 = 99u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_PLUS: u16 = 87u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_PLUS_MINUS: u16 = 215u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_RIGHT_BRACE: u16 = 185u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_RIGHT_BRACKET: u16 = 183u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_SEVEN_AND_HOME: u16 = 95u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_SIX_AND_RIGHT_ARROW: u16 = 94u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_SPACE: u16 = 205u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_STAR: u16 = 85u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_TAB: u16 = 186u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_THREE_AND_PAGEDN: u16 = 91u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_TRIPLE_0: u16 = 177u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_TWO_AND_DOWN_ARROW: u16 = 90u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_XOR: u16 = 194u16;
pub const HID_USAGE_KEYBOARD_KEYPAD_ZERO_AND_INSERT: u16 = 98u16;
pub const HID_USAGE_KEYBOARD_LALT: u16 = 226u16;
pub const HID_USAGE_KEYBOARD_LANG1: u16 = 144u16;
pub const HID_USAGE_KEYBOARD_LANG2: u16 = 145u16;
pub const HID_USAGE_KEYBOARD_LANG3: u16 = 146u16;
pub const HID_USAGE_KEYBOARD_LANG4: u16 = 147u16;
pub const HID_USAGE_KEYBOARD_LANG5: u16 = 148u16;
pub const HID_USAGE_KEYBOARD_LANG6: u16 = 149u16;
pub const HID_USAGE_KEYBOARD_LANG7: u16 = 150u16;
pub const HID_USAGE_KEYBOARD_LANG8: u16 = 151u16;
pub const HID_USAGE_KEYBOARD_LANG9: u16 = 152u16;
pub const HID_USAGE_KEYBOARD_LCTRL: u16 = 224u16;
pub const HID_USAGE_KEYBOARD_LEFTARROW: u16 = 80u16;
pub const HID_USAGE_KEYBOARD_LEFT_APOS_AND_DOUBLE: u16 = 52u16;
pub const HID_USAGE_KEYBOARD_LEFT_BRACE: u16 = 47u16;
pub const HID_USAGE_KEYBOARD_LGUI: u16 = 227u16;
pub const HID_USAGE_KEYBOARD_LOCKING_CAPS_LOCK: u16 = 130u16;
pub const HID_USAGE_KEYBOARD_LOCKING_NUM_LOCK: u16 = 131u16;
pub const HID_USAGE_KEYBOARD_LOCKING_SCROLL_LOCK: u16 = 132u16;
pub const HID_USAGE_KEYBOARD_LSHFT: u16 = 225u16;
pub const HID_USAGE_KEYBOARD_MENU: u16 = 118u16;
pub const HID_USAGE_KEYBOARD_MUTE: u16 = 127u16;
pub const HID_USAGE_KEYBOARD_NINE_AND_LEFT_BRACKET: u16 = 38u16;
pub const HID_USAGE_KEYBOARD_NOEVENT: u16 = 0u16;
pub const HID_USAGE_KEYBOARD_NONUS_BACKSLASH_AND_PIPE: u16 = 100u16;
pub const HID_USAGE_KEYBOARD_NONUS_HASH_AND_TILDE: u16 = 50u16;
pub const HID_USAGE_KEYBOARD_NUM_LOCK: u16 = 83u16;
pub const HID_USAGE_KEYBOARD_ONE: u16 = 30u16;
pub const HID_USAGE_KEYBOARD_OPER: u16 = 161u16;
pub const HID_USAGE_KEYBOARD_OUT: u16 = 160u16;
pub const HID_USAGE_KEYBOARD_PAGEDOWN: u16 = 78u16;
pub const HID_USAGE_KEYBOARD_PAGEUP: u16 = 75u16;
pub const HID_USAGE_KEYBOARD_PASTE: u16 = 125u16;
pub const HID_USAGE_KEYBOARD_PAUSE: u16 = 72u16;
pub const HID_USAGE_KEYBOARD_PERIOD_AND_GREATERTHAN: u16 = 55u16;
pub const HID_USAGE_KEYBOARD_POSTFAIL: u16 = 2u16;
pub const HID_USAGE_KEYBOARD_POWER: u16 = 102u16;
pub const HID_USAGE_KEYBOARD_PRINT_SCREEN: u16 = 70u16;
pub const HID_USAGE_KEYBOARD_PRIOR: u16 = 157u16;
pub const HID_USAGE_KEYBOARD_RALT: u16 = 230u16;
pub const HID_USAGE_KEYBOARD_RCTRL: u16 = 228u16;
pub const HID_USAGE_KEYBOARD_RETURN: u16 = 40u16;
pub const HID_USAGE_KEYBOARD_RETURN_NO_ENTER: u16 = 158u16;
pub const HID_USAGE_KEYBOARD_RGUI: u16 = 231u16;
pub const HID_USAGE_KEYBOARD_RIGHTARROW: u16 = 79u16;
pub const HID_USAGE_KEYBOARD_RIGHT_BRACE: u16 = 48u16;
pub const HID_USAGE_KEYBOARD_ROLLOVER: u16 = 1u16;
pub const HID_USAGE_KEYBOARD_RSHFT: u16 = 229u16;
pub const HID_USAGE_KEYBOARD_SCROLL_LOCK: u16 = 71u16;
pub const HID_USAGE_KEYBOARD_SELECT: u16 = 119u16;
pub const HID_USAGE_KEYBOARD_SEMICOLON_AND_COLON: u16 = 51u16;
pub const HID_USAGE_KEYBOARD_SEPARATOR: u16 = 159u16;
pub const HID_USAGE_KEYBOARD_SEVEN_AND_AMPERSAND: u16 = 36u16;
pub const HID_USAGE_KEYBOARD_SIX_AND_CARET: u16 = 35u16;
pub const HID_USAGE_KEYBOARD_SPACEBAR: u16 = 44u16;
pub const HID_USAGE_KEYBOARD_STOP: u16 = 120u16;
pub const HID_USAGE_KEYBOARD_SYSREQ_ATTENTION: u16 = 154u16;
pub const HID_USAGE_KEYBOARD_TAB: u16 = 43u16;
pub const HID_USAGE_KEYBOARD_THOUSANDS_SEPARATOR: u16 = 178u16;
pub const HID_USAGE_KEYBOARD_THREE_AND_HASH: u16 = 32u16;
pub const HID_USAGE_KEYBOARD_TWO_AND_AT: u16 = 31u16;
pub const HID_USAGE_KEYBOARD_UNDEFINED: u16 = 3u16;
pub const HID_USAGE_KEYBOARD_UNDO: u16 = 122u16;
pub const HID_USAGE_KEYBOARD_UPARROW: u16 = 82u16;
pub const HID_USAGE_KEYBOARD_VOLUME_DOWN: u16 = 129u16;
pub const HID_USAGE_KEYBOARD_VOLUME_UP: u16 = 128u16;
pub const HID_USAGE_KEYBOARD_ZERO: u16 = 39u16;
pub const HID_USAGE_KEYBOARD_aA: u16 = 4u16;
pub const HID_USAGE_KEYBOARD_bB: u16 = 5u16;
pub const HID_USAGE_KEYBOARD_cC: u16 = 6u16;
pub const HID_USAGE_KEYBOARD_dD: u16 = 7u16;
pub const HID_USAGE_KEYBOARD_eE: u16 = 8u16;
pub const HID_USAGE_KEYBOARD_fF: u16 = 9u16;
pub const HID_USAGE_KEYBOARD_gG: u16 = 10u16;
pub const HID_USAGE_KEYBOARD_hH: u16 = 11u16;
pub const HID_USAGE_KEYBOARD_iI: u16 = 12u16;
pub const HID_USAGE_KEYBOARD_jJ: u16 = 13u16;
pub const HID_USAGE_KEYBOARD_kK: u16 = 14u16;
pub const HID_USAGE_KEYBOARD_lL: u16 = 15u16;
pub const HID_USAGE_KEYBOARD_mM: u16 = 16u16;
pub const HID_USAGE_KEYBOARD_nN: u16 = 17u16;
pub const HID_USAGE_KEYBOARD_oO: u16 = 18u16;
pub const HID_USAGE_KEYBOARD_pP: u16 = 19u16;
pub const HID_USAGE_KEYBOARD_qQ: u16 = 20u16;
pub const HID_USAGE_KEYBOARD_rR: u16 = 21u16;
pub const HID_USAGE_KEYBOARD_sS: u16 = 22u16;
pub const HID_USAGE_KEYBOARD_tT: u16 = 23u16;
pub const HID_USAGE_KEYBOARD_uU: u16 = 24u16;
pub const HID_USAGE_KEYBOARD_vV: u16 = 25u16;
pub const HID_USAGE_KEYBOARD_wW: u16 = 26u16;
pub const HID_USAGE_KEYBOARD_xX: u16 = 27u16;
pub const HID_USAGE_KEYBOARD_yY: u16 = 28u16;
pub const HID_USAGE_KEYBOARD_zZ: u16 = 29u16;
pub const HID_USAGE_LAMPARRAY: u16 = 1u16;
pub const HID_USAGE_LAMPARRAY_ATTRBIUTES_REPORT: u16 = 2u16;
pub const HID_USAGE_LAMPARRAY_AUTONOMOUS_MODE: u16 = 113u16;
pub const HID_USAGE_LAMPARRAY_BLUE_LEVEL_COUNT: u16 = 42u16;
pub const HID_USAGE_LAMPARRAY_BOUNDING_BOX_DEPTH_IN_MICROMETERS: u16 = 6u16;
pub const HID_USAGE_LAMPARRAY_BOUNDING_BOX_HEIGHT_IN_MICROMETERS: u16 = 5u16;
pub const HID_USAGE_LAMPARRAY_BOUNDING_BOX_WIDTH_IN_MICROMETERS: u16 = 4u16;
pub const HID_USAGE_LAMPARRAY_CONTROL_REPORT: u16 = 112u16;
pub const HID_USAGE_LAMPARRAY_GREEN_LEVEL_COUNT: u16 = 41u16;
pub const HID_USAGE_LAMPARRAY_INPUT_BINDING: u16 = 45u16;
pub const HID_USAGE_LAMPARRAY_INTENSITY_LEVEL_COUNT: u16 = 43u16;
pub const HID_USAGE_LAMPARRAY_IS_PROGRAMMABLE: u16 = 44u16;
pub const HID_USAGE_LAMPARRAY_KIND: u16 = 7u16;
pub const HID_USAGE_LAMPARRAY_LAMP_ATTRIBUTES_REQUEST_REPORT: u16 = 32u16;
pub const HID_USAGE_LAMPARRAY_LAMP_ATTRIBUTES_RESPONSE_REPORT: u16 = 34u16;
pub const HID_USAGE_LAMPARRAY_LAMP_BLUE_UPDATE_CHANNEL: u16 = 83u16;
pub const HID_USAGE_LAMPARRAY_LAMP_COUNT: u16 = 3u16;
pub const HID_USAGE_LAMPARRAY_LAMP_GREEN_UPDATE_CHANNEL: u16 = 82u16;
pub const HID_USAGE_LAMPARRAY_LAMP_ID: u16 = 33u16;
pub const HID_USAGE_LAMPARRAY_LAMP_ID_END: u16 = 98u16;
pub const HID_USAGE_LAMPARRAY_LAMP_ID_START: u16 = 97u16;
pub const HID_USAGE_LAMPARRAY_LAMP_INTENSITY_UPDATE_CHANNEL: u16 = 84u16;
pub const HID_USAGE_LAMPARRAY_LAMP_MULTI_UPDATE_REPORT: u16 = 80u16;
pub const HID_USAGE_LAMPARRAY_LAMP_PURPOSES: u16 = 38u16;
pub const HID_USAGE_LAMPARRAY_LAMP_RANGE_UPDATE_REPORT: u16 = 96u16;
pub const HID_USAGE_LAMPARRAY_LAMP_RED_UPDATE_CHANNEL: u16 = 81u16;
pub const HID_USAGE_LAMPARRAY_LAMP_UPDATE_FLAGS: u16 = 85u16;
pub const HID_USAGE_LAMPARRAY_MIN_UPDATE_INTERVAL_IN_MICROSECONDS: u16 = 8u16;
pub const HID_USAGE_LAMPARRAY_POSITION_X_IN_MICROMETERS: u16 = 35u16;
pub const HID_USAGE_LAMPARRAY_POSITION_Y_IN_MICROMETERS: u16 = 36u16;
pub const HID_USAGE_LAMPARRAY_POSITION_Z_IN_MICROMETERS: u16 = 37u16;
pub const HID_USAGE_LAMPARRAY_RED_LEVEL_COUNT: u16 = 40u16;
pub const HID_USAGE_LAMPARRAY_UPDATE_LATENCY_IN_MICROSECONDS: u16 = 39u16;
pub const HID_USAGE_LED_AMBER: u16 = 74u16;
pub const HID_USAGE_LED_BATTERY_LOW: u16 = 29u16;
pub const HID_USAGE_LED_BATTERY_OK: u16 = 28u16;
pub const HID_USAGE_LED_BATTERY_OPERATION: u16 = 27u16;
pub const HID_USAGE_LED_BLUE_LED_CHANNEL: u16 = 84u16;
pub const HID_USAGE_LED_BUSY: u16 = 44u16;
pub const HID_USAGE_LED_CALL_PICKUP: u16 = 37u16;
pub const HID_USAGE_LED_CAMERA_OFF: u16 = 41u16;
pub const HID_USAGE_LED_CAMERA_ON: u16 = 40u16;
pub const HID_USAGE_LED_CAPS_LOCK: u16 = 2u16;
pub const HID_USAGE_LED_CAV: u16 = 20u16;
pub const HID_USAGE_LED_CLV: u16 = 21u16;
pub const HID_USAGE_LED_COMPOSE: u16 = 4u16;
pub const HID_USAGE_LED_CONFERENCE: u16 = 38u16;
pub const HID_USAGE_LED_COVERAGE: u16 = 34u16;
pub const HID_USAGE_LED_DATA_MODE: u16 = 26u16;
pub const HID_USAGE_LED_DO_NOT_DISTURB: u16 = 8u16;
pub const HID_USAGE_LED_EQUALIZER_ENABLE: u16 = 13u16;
pub const HID_USAGE_LED_ERROR: u16 = 57u16;
pub const HID_USAGE_LED_EXTERNAL_POWER: u16 = 77u16;
pub const HID_USAGE_LED_FAST_BLINK_OFF_TIME: u16 = 70u16;
pub const HID_USAGE_LED_FAST_BLINK_ON_TIME: u16 = 69u16;
pub const HID_USAGE_LED_FAST_FORWARD: u16 = 53u16;
pub const HID_USAGE_LED_FLASH_ON_TIME: u16 = 66u16;
pub const HID_USAGE_LED_FORWARD: u16 = 49u16;
pub const HID_USAGE_LED_GENERIC_INDICATOR: u16 = 75u16;
pub const HID_USAGE_LED_GOOD_STATUS: u16 = 80u16;
pub const HID_USAGE_LED_GREEN: u16 = 73u16;
pub const HID_USAGE_LED_GREEN_LED_CHANNEL: u16 = 85u16;
pub const HID_USAGE_LED_HEAD_SET: u16 = 31u16;
pub const HID_USAGE_LED_HIGH_CUT_FILTER: u16 = 11u16;
pub const HID_USAGE_LED_HOLD: u16 = 32u16;
pub const HID_USAGE_LED_INDICATOR_BLUE: u16 = 78u16;
pub const HID_USAGE_LED_INDICATOR_COLOR: u16 = 71u16;
pub const HID_USAGE_LED_INDICATOR_FAST_BLINK: u16 = 64u16;
pub const HID_USAGE_LED_INDICATOR_FLASH: u16 = 62u16;
pub const HID_USAGE_LED_INDICATOR_OFF: u16 = 65u16;
pub const HID_USAGE_LED_INDICATOR_ON: u16 = 61u16;
pub const HID_USAGE_LED_INDICATOR_ORANGE: u16 = 79u16;
pub const HID_USAGE_LED_INDICATOR_SLOW_BLINK: u16 = 63u16;
pub const HID_USAGE_LED_IN_USE_INDICATOR: u16 = 59u16;
pub const HID_USAGE_LED_KANA: u16 = 5u16;
pub const HID_USAGE_LED_LED_INTENSITY: u16 = 86u16;
pub const HID_USAGE_LED_LOW_CUT_FILTER: u16 = 12u16;
pub const HID_USAGE_LED_MESSAGE_WAITING: u16 = 25u16;
pub const HID_USAGE_LED_MICROPHONE: u16 = 33u16;
pub const HID_USAGE_LED_MULTI_MODE_INDICATOR: u16 = 60u16;
pub const HID_USAGE_LED_MUTE: u16 = 9u16;
pub const HID_USAGE_LED_NIGHT_MODE: u16 = 35u16;
pub const HID_USAGE_LED_NUM_LOCK: u16 = 1u16;
pub const HID_USAGE_LED_OFF_HOOK: u16 = 23u16;
pub const HID_USAGE_LED_OFF_LINE: u16 = 43u16;
pub const HID_USAGE_LED_ON_LINE: u16 = 42u16;
pub const HID_USAGE_LED_PAPER_JAM: u16 = 47u16;
pub const HID_USAGE_LED_PAPER_OUT: u16 = 46u16;
pub const HID_USAGE_LED_PAUSE: u16 = 55u16;
pub const HID_USAGE_LED_PLAY: u16 = 54u16;
pub const HID_USAGE_LED_PLAYER_1: u16 = 97u16;
pub const HID_USAGE_LED_PLAYER_2: u16 = 98u16;
pub const HID_USAGE_LED_PLAYER_3: u16 = 99u16;
pub const HID_USAGE_LED_PLAYER_4: u16 = 100u16;
pub const HID_USAGE_LED_PLAYER_5: u16 = 101u16;
pub const HID_USAGE_LED_PLAYER_6: u16 = 102u16;
pub const HID_USAGE_LED_PLAYER_7: u16 = 103u16;
pub const HID_USAGE_LED_PLAYER_8: u16 = 104u16;
pub const HID_USAGE_LED_PLAYER_INDICATOR: u16 = 96u16;
pub const HID_USAGE_LED_POWER: u16 = 6u16;
pub const HID_USAGE_LED_READY: u16 = 45u16;
pub const HID_USAGE_LED_RECORD: u16 = 56u16;
pub const HID_USAGE_LED_RECORDING_FORMAT_DET: u16 = 22u16;
pub const HID_USAGE_LED_RED: u16 = 72u16;
pub const HID_USAGE_LED_RED_LED_CHANNEL: u16 = 83u16;
pub const HID_USAGE_LED_REMOTE: u16 = 48u16;
pub const HID_USAGE_LED_REPEAT: u16 = 16u16;
pub const HID_USAGE_LED_REVERSE: u16 = 50u16;
pub const HID_USAGE_LED_REWIND: u16 = 52u16;
pub const HID_USAGE_LED_RGB_LED: u16 = 82u16;
pub const HID_USAGE_LED_RING: u16 = 24u16;
pub const HID_USAGE_LED_SAMPLING_RATE_DETECT: u16 = 18u16;
pub const HID_USAGE_LED_SCROLL_LOCK: u16 = 3u16;
pub const HID_USAGE_LED_SELECTED_INDICATOR: u16 = 58u16;
pub const HID_USAGE_LED_SEND_CALLS: u16 = 36u16;
pub const HID_USAGE_LED_SHIFT: u16 = 7u16;
pub const HID_USAGE_LED_SLOW_BLINK_OFF_TIME: u16 = 68u16;
pub const HID_USAGE_LED_SLOW_BLINK_ON_TIME: u16 = 67u16;
pub const HID_USAGE_LED_SOUND_FIELD_ON: u16 = 14u16;
pub const HID_USAGE_LED_SPEAKER: u16 = 30u16;
pub const HID_USAGE_LED_SPINNING: u16 = 19u16;
pub const HID_USAGE_LED_STAND_BY: u16 = 39u16;
pub const HID_USAGE_LED_STEREO: u16 = 17u16;
pub const HID_USAGE_LED_STOP: u16 = 51u16;
pub const HID_USAGE_LED_SURROUND_FIELD_ON: u16 = 15u16;
pub const HID_USAGE_LED_SYSTEM_MICROPHONE_MUTE: u16 = 87u16;
pub const HID_USAGE_LED_SYSTEM_SUSPEND: u16 = 76u16;
pub const HID_USAGE_LED_TONE_ENABLE: u16 = 10u16;
pub const HID_USAGE_LED_WARNING_STATUS: u16 = 81u16;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_MSR_DEVICE_READONLY: u16 = 1u16;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_1_DATA: u16 = 33u16;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_1_LENGTH: u16 = 17u16;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_2_DATA: u16 = 34u16;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_2_LENGTH: u16 = 18u16;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_3_DATA: u16 = 35u16;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_3_LENGTH: u16 = 19u16;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_DATA: u16 = 32u16;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_JIS_DATA: u16 = 36u16;
pub const HID_USAGE_MAGNETIC_STRIPE_READER_TRACK_JIS_LENGTH: u16 = 20u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_2D_MODE_ADJUST: u16 = 137u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_2D_MODE_SELECT: u16 = 136u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_CINE: u16 = 64u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_CLIP_STORE: u16 = 34u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_COLOR_DOPPLER_ADJUST: u16 = 133u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_COLOR_DOPPLER_MODE_SELECT: u16 = 132u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_DEPTH: u16 = 68u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_DEPTH_GAIN_COMPENSATION: u16 = 112u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_FOCUS: u16 = 67u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_FREEZETHAW: u16 = 33u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_MEDICAL_ULTRASOUND: u16 = 1u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_MICROPHONE_ENABLE: u16 = 39u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_MOTION_MODE_ADJUST: u16 = 135u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_MOTION_MODE_SELECT: u16 = 134u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_NEXT: u16 = 36u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_PRINT: u16 = 38u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SAVE: u16 = 37u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SOFT_CONTROL_ADJUST: u16 = 161u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SOFT_CONTROL_SELECT: u16 = 160u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SOFT_STEP__PRIMARY: u16 = 96u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SOFT_STEP__SECONDARY: u16 = 97u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SPECTRAL_DOPPLER_ADJUST: u16 = 131u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_SPECTRAL_DOPPLER_MODE_SELECT: u16 = 130u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_TRANSMIT_POWER: u16 = 65u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_UPDATE: u16 = 35u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_VCRACQUISITION: u16 = 32u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_VOLUME: u16 = 66u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_ZOOM_ADJUST: u16 = 129u16;
pub const HID_USAGE_MEDICAL_INSTRUMENT_ZOOM_SELECT: u16 = 128u16;
pub const HID_USAGE_MICROSOFT_BLUETOOTH_HANDSFREE_CALL_SETUP: u16 = 36u16;
pub const HID_USAGE_MICROSOFT_BLUETOOTH_HANDSFREE_DIAL_MEMORY: u16 = 34u16;
pub const HID_USAGE_MICROSOFT_BLUETOOTH_HANDSFREE_DIAL_NUMBER: u16 = 33u16;
pub const HID_USAGE_MICROSOFT_BLUETOOTH_HANDSFREE_GENERAL: u16 = 1u16;
pub const HID_USAGE_MONITOR_EDID_INFORMATION: u16 = 2u16;
pub const HID_USAGE_MONITOR_MONITOR_CONTROL: u16 = 1u16;
pub const HID_USAGE_MONITOR_VDIF_INFORMATION: u16 = 3u16;
pub const HID_USAGE_MONITOR_VESA_VERSION: u16 = 4u16;
pub const HID_USAGE_PAGE_ALPHANUMERIC: u16 = 20u16;
pub const HID_USAGE_PAGE_ARCADE: u16 = 145u16;
pub const HID_USAGE_PAGE_BARCODE_SCANNER: u16 = 140u16;
pub const HID_USAGE_PAGE_BATTERY_SYSTEM: u16 = 133u16;
pub const HID_USAGE_PAGE_BRAILLE_DISPLAY: u16 = 65u16;
pub const HID_USAGE_PAGE_BUTTON: u16 = 9u16;
pub const HID_USAGE_PAGE_CAMERA_CONTROL: u16 = 144u16;
pub const HID_USAGE_PAGE_CONSUMER: u16 = 12u16;
pub const HID_USAGE_PAGE_DIGITIZER: u16 = 13u16;
pub const HID_USAGE_PAGE_EYE_AND_HEAD_TRACKERS: u16 = 18u16;
pub const HID_USAGE_PAGE_FIDO_ALLIANCE: u16 = 61904u16;
pub const HID_USAGE_PAGE_GAME: u16 = 5u16;
pub const HID_USAGE_PAGE_GAMING_DEVICE: u16 = 146u16;
pub const HID_USAGE_PAGE_GENERIC: u16 = 1u16;
pub const HID_USAGE_PAGE_GENERIC_DEVICE: u16 = 6u16;
pub const HID_USAGE_PAGE_HAPTICS: u16 = 14u16;
pub const HID_USAGE_PAGE_KEYBOARD: u16 = 7u16;
pub const HID_USAGE_PAGE_LED: u16 = 8u16;
pub const HID_USAGE_PAGE_LIGHTING_ILLUMINATION: u16 = 89u16;
pub const HID_USAGE_PAGE_MAGNETIC_STRIPE_READER: u16 = 142u16;
pub const HID_USAGE_PAGE_MEDICAL_INSTRUMENT: u16 = 64u16;
pub const HID_USAGE_PAGE_MICROSOFT_BLUETOOTH_HANDSFREE: u16 = 65523u16;
pub const HID_USAGE_PAGE_MONITOR: u16 = 128u16;
pub const HID_USAGE_PAGE_MONITOR_ENUMERATED: u16 = 129u16;
pub const HID_USAGE_PAGE_ORDINAL: u16 = 10u16;
pub const HID_USAGE_PAGE_PID: u16 = 15u16;
pub const HID_USAGE_PAGE_POWER: u16 = 132u16;
pub const HID_USAGE_PAGE_SENSOR: u16 = 32u16;
pub const HID_USAGE_PAGE_SIMULATION: u16 = 2u16;
pub const HID_USAGE_PAGE_SOC: u16 = 17u16;
pub const HID_USAGE_PAGE_SPORT: u16 = 4u16;
pub const HID_USAGE_PAGE_TELEPHONY: u16 = 11u16;
pub const HID_USAGE_PAGE_UNDEFINED: u16 = 0u16;
pub const HID_USAGE_PAGE_UNICODE: u16 = 16u16;
pub const HID_USAGE_PAGE_VENDOR_DEFINED_BEGIN: u16 = 65280u16;
pub const HID_USAGE_PAGE_VENDOR_DEFINED_END: u16 = 65535u16;
pub const HID_USAGE_PAGE_VESA_VIRTUAL_CONTROLS: u16 = 130u16;
pub const HID_USAGE_PAGE_VR: u16 = 3u16;
pub const HID_USAGE_PAGE_WEIGHING_DEVICE: u16 = 141u16;
pub const HID_USAGE_PID_ACTUATORS_ENABLED: u16 = 160u16;
pub const HID_USAGE_PID_ACTUATOR_OVERRIDE_SWITCH: u16 = 165u16;
pub const HID_USAGE_PID_ACTUATOR_POWER: u16 = 166u16;
pub const HID_USAGE_PID_ATTACK_LEVEL: u16 = 91u16;
pub const HID_USAGE_PID_ATTACK_TIME: u16 = 92u16;
pub const HID_USAGE_PID_AXES_ENABLE: u16 = 85u16;
pub const HID_USAGE_PID_BLOCK_FREE_REPORT: u16 = 144u16;
pub const HID_USAGE_PID_BLOCK_HANDLE: u16 = 143u16;
pub const HID_USAGE_PID_BLOCK_LOAD_ERROR: u16 = 142u16;
pub const HID_USAGE_PID_BLOCK_LOAD_FULL: u16 = 141u16;
pub const HID_USAGE_PID_BLOCK_LOAD_REPORT: u16 = 137u16;
pub const HID_USAGE_PID_BLOCK_LOAD_STATUS: u16 = 139u16;
pub const HID_USAGE_PID_BLOCK_LOAD_SUCCESS: u16 = 140u16;
pub const HID_USAGE_PID_BLOCK_TYPE: u16 = 89u16;
pub const HID_USAGE_PID_CP_OFFSET: u16 = 96u16;
pub const HID_USAGE_PID_CREATE_NEW_EFFECT: u16 = 171u16;
pub const HID_USAGE_PID_CUSTOM_FORCE_DATA: u16 = 105u16;
pub const HID_USAGE_PID_CUSTOM_FORCE_DATA_OFFSET: u16 = 108u16;
pub const HID_USAGE_PID_CUSTOM_FORCE_DATA_REPORT: u16 = 104u16;
pub const HID_USAGE_PID_CUSTOM_FORCE_VENDOR_DEFINED_DATA: u16 = 106u16;
pub const HID_USAGE_PID_DC_DEVICE_CONTINUE: u16 = 156u16;
pub const HID_USAGE_PID_DC_DEVICE_PAUSE: u16 = 155u16;
pub const HID_USAGE_PID_DC_DEVICE_RESET: u16 = 154u16;
pub const HID_USAGE_PID_DC_DISABLE_ACTUATORS: u16 = 152u16;
pub const HID_USAGE_PID_DC_ENABLE_ACTUATORS: u16 = 151u16;
pub const HID_USAGE_PID_DC_STOP_ALL_EFFECTS: u16 = 153u16;
pub const HID_USAGE_PID_DEAD_BAND: u16 = 101u16;
pub const HID_USAGE_PID_DEVICE_CONTROL: u16 = 150u16;
pub const HID_USAGE_PID_DEVICE_GAIN: u16 = 126u16;
pub const HID_USAGE_PID_DEVICE_GAIN_REPORT: u16 = 125u16;
pub const HID_USAGE_PID_DEVICE_MANAGED_POOL: u16 = 169u16;
pub const HID_USAGE_PID_DEVICE_PAUSED: u16 = 159u16;
pub const HID_USAGE_PID_DIRECTION: u16 = 87u16;
pub const HID_USAGE_PID_DIRECTION_ENABLE: u16 = 86u16;
pub const HID_USAGE_PID_DOWNLOAD_FORCE_SAMPLE: u16 = 102u16;
pub const HID_USAGE_PID_DURATION: u16 = 80u16;
pub const HID_USAGE_PID_EFFECT_BLOCK_INDEX: u16 = 34u16;
pub const HID_USAGE_PID_EFFECT_OPERATION: u16 = 120u16;
pub const HID_USAGE_PID_EFFECT_OPERATION_REPORT: u16 = 119u16;
pub const HID_USAGE_PID_EFFECT_PLAYING: u16 = 148u16;
pub const HID_USAGE_PID_EFFECT_TYPE: u16 = 37u16;
pub const HID_USAGE_PID_ET_CONSTANT: u16 = 38u16;
pub const HID_USAGE_PID_ET_CUSTOM: u16 = 40u16;
pub const HID_USAGE_PID_ET_DAMPER: u16 = 65u16;
pub const HID_USAGE_PID_ET_FRICTION: u16 = 67u16;
pub const HID_USAGE_PID_ET_INERTIA: u16 = 66u16;
pub const HID_USAGE_PID_ET_RAMP: u16 = 39u16;
pub const HID_USAGE_PID_ET_SAWTOOTH_DOWN: u16 = 52u16;
pub const HID_USAGE_PID_ET_SAWTOOTH_UP: u16 = 51u16;
pub const HID_USAGE_PID_ET_SINE: u16 = 49u16;
pub const HID_USAGE_PID_ET_SPRING: u16 = 64u16;
pub const HID_USAGE_PID_ET_SQUARE: u16 = 48u16;
pub const HID_USAGE_PID_ET_TRIANGLE: u16 = 50u16;
pub const HID_USAGE_PID_FADE_LEVEL: u16 = 93u16;
pub const HID_USAGE_PID_FADE_TIME: u16 = 94u16;
pub const HID_USAGE_PID_GAIN: u16 = 82u16;
pub const HID_USAGE_PID_ISOCH_CUSTOMFORCE_ENABLE: u16 = 103u16;
pub const HID_USAGE_PID_LOOP_COUNT: u16 = 124u16;
pub const HID_USAGE_PID_MAGNITUDE: u16 = 112u16;
pub const HID_USAGE_PID_MOVE_DESTINATION: u16 = 135u16;
pub const HID_USAGE_PID_MOVE_LENGTH: u16 = 136u16;
pub const HID_USAGE_PID_MOVE_SOURCE: u16 = 134u16;
pub const HID_USAGE_PID_NEGATIVE_COEFFICIENT: u16 = 98u16;
pub const HID_USAGE_PID_NEGATIVE_SATURATION: u16 = 100u16;
pub const HID_USAGE_PID_NORMAL: u16 = 32u16;
pub const HID_USAGE_PID_OFFSET: u16 = 111u16;
pub const HID_USAGE_PID_OP_EFFECT_START: u16 = 121u16;
pub const HID_USAGE_PID_OP_EFFECT_START_SOLO: u16 = 122u16;
pub const HID_USAGE_PID_OP_EFFECT_STOP: u16 = 123u16;
pub const HID_USAGE_PID_PARAMETER_BLOCK_MOVE_REPORT: u16 = 133u16;
pub const HID_USAGE_PID_PARAMETER_BLOCK_OFFSET: u16 = 35u16;
pub const HID_USAGE_PID_PARAMETER_BLOCK_SIZE: u16 = 168u16;
pub const HID_USAGE_PID_PERIOD: u16 = 114u16;
pub const HID_USAGE_PID_PHASE: u16 = 113u16;
pub const HID_USAGE_PID_PHYSICAL_INPUT_DEVICE: u16 = 1u16;
pub const HID_USAGE_PID_PID_DEVICE_CONTROL_REPORT: u16 = 149u16;
pub const HID_USAGE_PID_POOL_ALIGNMENT: u16 = 132u16;
pub const HID_USAGE_PID_POOL_REPORT: u16 = 127u16;
pub const HID_USAGE_PID_POSITIVE_COEFFICIENT: u16 = 97u16;
pub const HID_USAGE_PID_POSITIVE_SATURATION: u16 = 99u16;
pub const HID_USAGE_PID_RAMPOOL_AVAILABLE: u16 = 172u16;
pub const HID_USAGE_PID_RAMP_END: u16 = 118u16;
pub const HID_USAGE_PID_RAMP_START: u16 = 117u16;
pub const HID_USAGE_PID_RAM_POOL_SIZE: u16 = 128u16;
pub const HID_USAGE_PID_ROM_EFFECT_BLOCK_COUNT: u16 = 130u16;
pub const HID_USAGE_PID_ROM_FLAG: u16 = 36u16;
pub const HID_USAGE_PID_ROM_POOL_SIZE: u16 = 129u16;
pub const HID_USAGE_PID_SAFETY_SWITCH: u16 = 164u16;
pub const HID_USAGE_PID_SAMPLE_COUNT: u16 = 109u16;
pub const HID_USAGE_PID_SAMPLE_PERIOD: u16 = 81u16;
pub const HID_USAGE_PID_SET_CONDITION_REPORT: u16 = 95u16;
pub const HID_USAGE_PID_SET_CONSTANT_FORCE_REPORT: u16 = 115u16;
pub const HID_USAGE_PID_SET_CUSTOM_FORCE_REPORT: u16 = 107u16;
pub const HID_USAGE_PID_SET_EFFECT_REPORT: u16 = 33u16;
pub const HID_USAGE_PID_SET_ENVELOPE_REPORT: u16 = 90u16;
pub const HID_USAGE_PID_SET_PERIODIC_REPORT: u16 = 110u16;
pub const HID_USAGE_PID_SET_RAMP_FORCE_REPORT: u16 = 116u16;
pub const HID_USAGE_PID_SHARED_PARAMETER_BLOCKS: u16 = 170u16;
pub const HID_USAGE_PID_SIMULTANEOUS_EFFECTS_MAX: u16 = 131u16;
pub const HID_USAGE_PID_START_DELAY: u16 = 167u16;
pub const HID_USAGE_PID_STATE_REPORT: u16 = 146u16;
pub const HID_USAGE_PID_TRIGGER_BUTTON: u16 = 83u16;
pub const HID_USAGE_PID_TRIGGER_REPEAT_INTERVAL: u16 = 84u16;
pub const HID_USAGE_PID_TYPE_SPECIFIC_BLOCK_HANDLE: u16 = 145u16;
pub const HID_USAGE_PID_TYPE_SPECIFIC_BLOCK_OFFSET: u16 = 88u16;
pub const HID_USAGE_POWER_ACTIVE_POWER: u16 = 52u16;
pub const HID_USAGE_POWER_APPARENT_POWER: u16 = 51u16;
pub const HID_USAGE_POWER_AUDIBLE_ALARM_CONTROL: u16 = 90u16;
pub const HID_USAGE_POWER_AWAITING_POWER: u16 = 114u16;
pub const HID_USAGE_POWER_BAD_COUNT: u16 = 56u16;
pub const HID_USAGE_POWER_BATTERY: u16 = 18u16;
pub const HID_USAGE_POWER_BATTERY_ID: u16 = 19u16;
pub const HID_USAGE_POWER_BATTERY_SYSTEM: u16 = 16u16;
pub const HID_USAGE_POWER_BATTERY_SYSTEM_ID: u16 = 17u16;
pub const HID_USAGE_POWER_BOOST: u16 = 110u16;
pub const HID_USAGE_POWER_BUCK: u16 = 111u16;
pub const HID_USAGE_POWER_CHANGED_STATUS: u16 = 3u16;
pub const HID_USAGE_POWER_CHARGER: u16 = 20u16;
pub const HID_USAGE_POWER_CHARGER_ID: u16 = 21u16;
pub const HID_USAGE_POWER_COMMUNICATION_LOST: u16 = 115u16;
pub const HID_USAGE_POWER_CONFIG_ACTIVE_POWER: u16 = 68u16;
pub const HID_USAGE_POWER_CONFIG_APPARENT_POWER: u16 = 67u16;
pub const HID_USAGE_POWER_CONFIG_CURRENT: u16 = 65u16;
pub const HID_USAGE_POWER_CONFIG_FREQUENCY: u16 = 66u16;
pub const HID_USAGE_POWER_CONFIG_HUMIDITY: u16 = 71u16;
pub const HID_USAGE_POWER_CONFIG_PERCENT_LOAD: u16 = 69u16;
pub const HID_USAGE_POWER_CONFIG_TEMPERATURE: u16 = 70u16;
pub const HID_USAGE_POWER_CONFIG_VOLTAGE: u16 = 64u16;
pub const HID_USAGE_POWER_CURRENT: u16 = 49u16;
pub const HID_USAGE_POWER_DELAY_BEFORE_REBOOT: u16 = 85u16;
pub const HID_USAGE_POWER_DELAY_BEFORE_SHUTDOWN: u16 = 87u16;
pub const HID_USAGE_POWER_DELAY_BEFORE_STARTUP: u16 = 86u16;
pub const HID_USAGE_POWER_FLOW: u16 = 30u16;
pub const HID_USAGE_POWER_FLOW_ID: u16 = 31u16;
pub const HID_USAGE_POWER_FREQUENCY: u16 = 50u16;
pub const HID_USAGE_POWER_FREQUENCY_OUT_OF_RANGE: u16 = 100u16;
pub const HID_USAGE_POWER_GANG: u16 = 34u16;
pub const HID_USAGE_POWER_GANG_ID: u16 = 35u16;
pub const HID_USAGE_POWER_GOOD: u16 = 97u16;
pub const HID_USAGE_POWER_HIGH_VOLTAGE_TRANSFER: u16 = 84u16;
pub const HID_USAGE_POWER_HUMIDITY: u16 = 55u16;
pub const HID_USAGE_POWER_IMANUFACTURER: u16 = 253u16;
pub const HID_USAGE_POWER_INAME: u16 = 1u16;
pub const HID_USAGE_POWER_INITIALIZED: u16 = 112u16;
pub const HID_USAGE_POWER_INPUT: u16 = 26u16;
pub const HID_USAGE_POWER_INPUT_ID: u16 = 27u16;
pub const HID_USAGE_POWER_INTERNAL_FAILURE: u16 = 98u16;
pub const HID_USAGE_POWER_IPRODUCT: u16 = 254u16;
pub const HID_USAGE_POWER_ISERIALNUMBER: u16 = 255u16;
pub const HID_USAGE_POWER_LOW_VOLTAGE_TRANSFER: u16 = 83u16;
pub const HID_USAGE_POWER_MODULE_RESET: u16 = 89u16;
pub const HID_USAGE_POWER_OUTLET: u16 = 32u16;
pub const HID_USAGE_POWER_OUTLET_ID: u16 = 33u16;
pub const HID_USAGE_POWER_OUTLET_SYSTEM: u16 = 24u16;
pub const HID_USAGE_POWER_OUTLET_SYSTEM_ID: u16 = 25u16;
pub const HID_USAGE_POWER_OUTPUT: u16 = 28u16;
pub const HID_USAGE_POWER_OUTPUT_ID: u16 = 29u16;
pub const HID_USAGE_POWER_OVERLOAD: u16 = 101u16;
pub const HID_USAGE_POWER_OVER_CHARGED: u16 = 102u16;
pub const HID_USAGE_POWER_OVER_TEMPERATURE: u16 = 103u16;
pub const HID_USAGE_POWER_PERCENT_LOAD: u16 = 53u16;
pub const HID_USAGE_POWER_POWER_CONVERTER: u16 = 22u16;
pub const HID_USAGE_POWER_POWER_CONVERTER_ID: u16 = 23u16;
pub const HID_USAGE_POWER_POWER_SUMMARY: u16 = 36u16;
pub const HID_USAGE_POWER_POWER_SUMMARY_ID: u16 = 37u16;
pub const HID_USAGE_POWER_POWER_SUPPLY: u16 = 5u16;
pub const HID_USAGE_POWER_PRESENT: u16 = 96u16;
pub const HID_USAGE_POWER_PRESENT_STATUS: u16 = 2u16;
pub const HID_USAGE_POWER_SHUTDOWN_IMMINENT: u16 = 105u16;
pub const HID_USAGE_POWER_SHUTDOWN_REQUESTED: u16 = 104u16;
pub const HID_USAGE_POWER_SWITCHABLE: u16 = 108u16;
pub const HID_USAGE_POWER_SWITCH_OFF_CONTROL: u16 = 81u16;
pub const HID_USAGE_POWER_SWITCH_ONOFF: u16 = 107u16;
pub const HID_USAGE_POWER_SWITCH_ON_CONTROL: u16 = 80u16;
pub const HID_USAGE_POWER_TEMPERATURE: u16 = 54u16;
pub const HID_USAGE_POWER_TEST: u16 = 88u16;
pub const HID_USAGE_POWER_TESTED: u16 = 113u16;
pub const HID_USAGE_POWER_TOGGLE_CONTROL: u16 = 82u16;
pub const HID_USAGE_POWER_UPS: u16 = 4u16;
pub const HID_USAGE_POWER_USED: u16 = 109u16;
pub const HID_USAGE_POWER_VOLTAGE: u16 = 48u16;
pub const HID_USAGE_POWER_VOLTAG_OUT_OF_RANGE: u16 = 99u16;
pub const HID_USAGE_SENSORS_ACCURACY_DEFAULT: u16 = 2144u16;
pub const HID_USAGE_SENSORS_ACCURACY_HIGH: u16 = 2145u16;
pub const HID_USAGE_SENSORS_ACCURACY_LOW: u16 = 2147u16;
pub const HID_USAGE_SENSORS_ACCURACY_MEDIUM: u16 = 2146u16;
pub const HID_USAGE_SENSORS_ACTIVITY_STATE_END_ACTIVITY: u16 = 2402u16;
pub const HID_USAGE_SENSORS_ACTIVITY_STATE_NO_STATE_CHANGE: u16 = 2400u16;
pub const HID_USAGE_SENSORS_ACTIVITY_STATE_START_ACTIVITY: u16 = 2401u16;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_BIKING: u16 = 2358u16;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_FIDGETING: u16 = 2354u16;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_IDLE: u16 = 2359u16;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_IN_VEHICLE: u16 = 2357u16;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_RUNNING: u16 = 2356u16;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_STATIONARY: u16 = 2353u16;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_UNKNOWN: u16 = 2352u16;
pub const HID_USAGE_SENSORS_ACTIVITY_TYPE_WALKING: u16 = 2355u16;
pub const HID_USAGE_SENSORS_BIOMETRIC: u16 = 16u16;
pub const HID_USAGE_SENSORS_BIOMETRIC_BLOOD_PRESSURE: u16 = 20u16;
pub const HID_USAGE_SENSORS_BIOMETRIC_BODY_TEMPERATURE: u16 = 21u16;
pub const HID_USAGE_SENSORS_BIOMETRIC_HEART_RATE: u16 = 22u16;
pub const HID_USAGE_SENSORS_BIOMETRIC_HEART_RATE_VARIABILITY: u16 = 23u16;
pub const HID_USAGE_SENSORS_BIOMETRIC_HUMAN_PRESENCE: u16 = 17u16;
pub const HID_USAGE_SENSORS_BIOMETRIC_HUMAN_PROXIMITY: u16 = 18u16;
pub const HID_USAGE_SENSORS_BIOMETRIC_HUMAN_TOUCH: u16 = 19u16;
pub const HID_USAGE_SENSORS_BIOMETRIC_PERIPHERAL_OXYGEN_SATURATION: u16 = 24u16;
pub const HID_USAGE_SENSORS_BIOMETRIC_RESPIRATORY_RATE: u16 = 25u16;
pub const HID_USAGE_SENSORS_CONNECTION_TYPE_PC_ATTACHED: u16 = 2097u16;
pub const HID_USAGE_SENSORS_CONNECTION_TYPE_PC_EXTERNAL: u16 = 2098u16;
pub const HID_USAGE_SENSORS_CONNECTION_TYPE_PC_INTEGRATED: u16 = 2096u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ABSOLUTE_PRESSURE: u16 = 1173u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ACCELERATION: u16 = 1106u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ACCELERATION_AXIS_X: u16 = 1107u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ACCELERATION_AXIS_Y: u16 = 1108u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ACCELERATION_AXIS_Z: u16 = 1109u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ACTIVITY_STATE: u16 = 1426u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ACTIVITY_TYPE: u16 = 1425u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ADDRESS_LINE_1: u16 = 1058u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ADDRESS_LINE_2: u16 = 1059u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_AIR_QUALITY_INDEX: u16 = 1079u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ALTITUDE_ANTENNA_SEA_LEVEL: u16 = 1026u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ALTITUDE_ELLIPSOID: u16 = 1029u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ALTITUDE_ELLIPSOID_ERROR: u16 = 1028u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ALTITUDE_SEA_LEVEL: u16 = 1031u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ALTITUDE_SEA_LEVEL_ERROR: u16 = 1030u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_POSITION: u16 = 1114u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_POSITION_ABOUT_X_AXIS: u16 = 1115u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_POSITION_ABOUT_Y_AXIS: u16 = 1116u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_POSITION_ABOUT_Z_AXIS: u16 = 1117u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_VELOCITY: u16 = 1110u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_VELOCITY_ABOUT_X_AXIS: u16 = 1111u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_VELOCITY_ABOUT_Y_AXIS: u16 = 1112u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ANGULAR_VELOCITY_ABOUT_Z_AXIS: u16 = 1113u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ATMOSPHERIC_PRESSURE: u16 = 1073u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_BIOMETRIC: u16 = 1200u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_BLOOD_PRESSURE: u16 = 1205u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_BLOOD_PRESSURE_DIASTOLIC: u16 = 1206u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_BLOOD_PRESSURE_SYSTOLIC: u16 = 1207u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_BLUE_LIGHT: u16 = 1242u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_BOOLEAN_SWITCH_ARRAY_STATES: u16 = 1170u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_BOOLEAN_SWITCH_STATE: u16 = 1169u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CAPACITANCE: u16 = 1281u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CHROMATICITY: u16 = 1235u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CHROMATICITY_X: u16 = 1236u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CHROMATICITY_Y: u16 = 1237u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CITY: u16 = 1060u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_COLOR_TEMPERATURE: u16 = 1234u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CONSUMER_IR_SENTENCE_RECEIVE: u16 = 1238u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_COUNTRY_OR_REGION: u16 = 1062u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CURRENT: u16 = 1282u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM: u16 = 1344u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_BOOLEAN_ARRAY: u16 = 1346u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_TYPE_ID: u16 = 1456u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_USAGE: u16 = 1345u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE: u16 = 1347u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_1: u16 = 1348u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_10: u16 = 1357u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_11: u16 = 1358u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_12: u16 = 1359u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_13: u16 = 1360u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_14: u16 = 1361u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_15: u16 = 1362u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_16: u16 = 1363u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_17: u16 = 1364u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_18: u16 = 1365u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_19: u16 = 1366u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_2: u16 = 1349u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_20: u16 = 1367u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_21: u16 = 1368u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_22: u16 = 1369u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_23: u16 = 1370u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_24: u16 = 1371u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_25: u16 = 1372u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_26: u16 = 1373u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_27: u16 = 1374u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_28: u16 = 1375u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_3: u16 = 1350u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_4: u16 = 1351u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_5: u16 = 1352u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_6: u16 = 1353u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_7: u16 = 1354u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_8: u16 = 1355u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_CUSTOM_VALUE_9: u16 = 1356u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_DAY: u16 = 1315u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_DAY_OF_WEEK: u16 = 1316u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_DEVICE_POSITION: u16 = 1427u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_DIFFERENTIAL_GPS_DATA_AGE: u16 = 1032u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_DIFFERENTIAL_REFERENCE_STATION_ID: u16 = 1027u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_DISTANCE: u16 = 1145u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_DISTANCE_OUTOFRANGE: u16 = 1149u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_DISTANCE_X_AXIS: u16 = 1146u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_DISTANCE_Y_AXIS: u16 = 1147u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_DISTANCE_Z_AXIS: u16 = 1148u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ELECTRICAL: u16 = 1280u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ELECTRICAL_POWER: u16 = 1283u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ENUMERATOR_TABLE_ROW_COUNT: u16 = 1387u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ENUMERATOR_TABLE_ROW_INDEX: u16 = 1386u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ENVIRONMENTAL: u16 = 1072u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_EQUIVALENT_CO2: u16 = 1080u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ERROR_RADIUS: u16 = 1033u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_FIX_QUALITY: u16 = 1034u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_FIX_TYPE: u16 = 1035u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_FORCE: u16 = 1172u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_FREQUENCY: u16 = 1287u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GAUGE_PRESSURE: u16 = 1174u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC: u16 = 1376u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_CATEGORY_GUID: u16 = 1378u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_DATA_FIELD: u16 = 1385u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_DATA_FIELD_PROPERTYKEY: u16 = 1382u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_EVENT: u16 = 1383u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_EVENT_PROPERTYKEY: u16 = 1380u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_FIRMWARE_VARTYPE: u16 = 1394u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_GUID: u16 = 1389u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_GUID_OR_PROPERTYKEY: u16 = 1377u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_GUID_OR_PROPERTYKEY_KIND: u16 = 1388u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_PROPERTY: u16 = 1384u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_PROPERTYKEY: u16 = 1390u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_PROPERTY_PROPERTYKEY: u16 = 1381u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_REPORT_COUNT: u16 = 1398u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_REPORT_ID: u16 = 1392u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_REPORT_ITEM_POSITION_INDEX: u16 = 1393u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_REPORT_SIZE: u16 = 1397u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_TOP_LEVEL_COLLECTION_ID: u16 = 1391u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_TYPE_GUID: u16 = 1379u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_UNIT_EXPONENT: u16 = 1396u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GENERIC_UNIT_OF_MEASURE: u16 = 1395u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GEOIDAL_SEPARATION: u16 = 1036u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GESTURE_SENSOR: u16 = 1520u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GESTURE_STATE: u16 = 1521u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GPS_OPERATION_MODE: u16 = 1037u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GPS_SELECTION_MODE: u16 = 1038u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GPS_STATUS: u16 = 1039u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_GREEN_LIGHT: u16 = 1241u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING: u16 = 1137u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_COMPENSATED_MAGNETIC_NORTH: u16 = 1141u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_COMPENSATED_TRUE_NORTH: u16 = 1142u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_MAGNETIC_NORTH: u16 = 1143u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_TRUE_NORTH: u16 = 1144u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_X_AXIS: u16 = 1138u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_Y_AXIS: u16 = 1139u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEADING_Z_AXIS: u16 = 1140u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEARTBEAT_INTERVAL: u16 = 1210u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HEART_RATE: u16 = 1208u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HINGE: u16 = 1504u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HINGE_ANGLE: u16 = 1505u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HINGE_FOLD_CONTRIBUTING_PANEL: u16 = 1524u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HINGE_FOLD_FINAL_ANGLE: u16 = 1523u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HINGE_FOLD_INITIAL_ANGLE: u16 = 1522u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HINGE_FOLD_TYPE: u16 = 1525u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HORIZONTAL_DILUTION_OF_PRECISION: u16 = 1041u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HOUR: u16 = 1317u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_ATTENTION_DETECTED: u16 = 1213u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_CORRELATION_ID: u16 = 1219u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_HEAD_ALTITUDE: u16 = 1215u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_HEAD_AZIMUTH: u16 = 1214u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_HEAD_PITCH: u16 = 1217u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_HEAD_ROLL: u16 = 1216u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_HEAD_YAW: u16 = 1218u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_PRESENCE: u16 = 1201u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_PROXIMITY_OUT_OF_RANGE: u16 = 1203u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_PROXIMITY_RANGE: u16 = 1202u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_HUMAN_TOUCH_STATE: u16 = 1204u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ILLUMINANCE: u16 = 1233u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_INDUCTANCE: u16 = 1284u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_INFRARED_LIGHT: u16 = 1239u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_JULIAN_DAY_OF_YEAR: u16 = 1322u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_LATITUDE: u16 = 1043u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_LIGHT: u16 = 1232u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_LOCATION: u16 = 1024u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_LONGITUDE: u16 = 1044u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETIC_FLUX: u16 = 1156u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETIC_FLUX_X_AXIS: u16 = 1157u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETIC_FLUX_Y_AXIS: u16 = 1158u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETIC_FLUX_Z_AXIS: u16 = 1159u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETIC_HEADING: u16 = 1046u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETIC_VARIATION: u16 = 1047u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MAGNETOMETER_ACCURACY: u16 = 1160u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MECHANICAL: u16 = 1168u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MILLISECOND: u16 = 1320u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MINUTE: u16 = 1318u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MONTH: u16 = 1314u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MOTION: u16 = 1104u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MOTION_INTENSITY: u16 = 1119u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MOTION_SPEED: u16 = 1118u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MOTION_STATE: u16 = 1105u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_MULTIVALUE_SWITCH_VALUE: u16 = 1171u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_NEAR_INFRARED_LIGHT: u16 = 1246u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_NFC_SENTENCE_RECEIVE: u16 = 1266u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_NMEA_SENTENCE: u16 = 1057u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_OBJECT_PRESENCE: u16 = 1082u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_OBJECT_PROXIMITY_OUT_OF_RANGE: u16 = 1084u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_OBJECT_PROXIMITY_RANGE: u16 = 1083u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ORIENTATION: u16 = 1136u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_PERCENT_OF_RANGE: u16 = 1289u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_PERIOD: u16 = 1288u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_PERSONAL_ACTIVITY: u16 = 1424u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_POSITION_DILUTION_OF_PRECISION: u16 = 1040u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_POSTAL_CODE: u16 = 1063u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_QUATERNION: u16 = 1155u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_RED_LIGHT: u16 = 1240u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_RELATIVE_HUMIDITY: u16 = 1075u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_RESISTANCE: u16 = 1285u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_RESPIRATORY_RATE: u16 = 1211u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_RESTING_HEART_RATE: u16 = 1209u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_RFID_TAG_40_BIT: u16 = 1265u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ROTATION_MATRIX: u16 = 1154u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_IN_VIEW: u16 = 1049u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_IN_VIEW_AZIMUTH: u16 = 1050u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_IN_VIEW_ELEVATION: u16 = 1051u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_IN_VIEW_IDS: u16 = 1052u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_IN_VIEW_PRNS: u16 = 1053u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_IN_VIEW_SN_RATIOS: u16 = 1054u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_USED_COUNT: u16 = 1055u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_SATELLITES_USED_PRNS: u16 = 1056u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_SCANNER: u16 = 1264u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_SECOND: u16 = 1319u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_SIMPLE_ORIENTATION_DIRECTION: u16 = 1161u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_SPEED: u16 = 1048u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_SPO2: u16 = 1212u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_STATE_OR_PROVINCE: u16 = 1061u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_STEP_COUNT: u16 = 1428u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_STEP_COUNT_RESET: u16 = 1429u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_STEP_DURATION: u16 = 1430u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_STEP_TYPE: u16 = 1431u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_STRAIN: u16 = 1175u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_TEMPERATURE: u16 = 1076u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_TILT: u16 = 1150u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_TILT_X_AXIS: u16 = 1151u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_TILT_Y_AXIS: u16 = 1152u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_TILT_Z_AXIS: u16 = 1153u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_TIME: u16 = 1312u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_TIMESTAMP: u16 = 1321u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_TIME_SINCE_SYSTEM_BOOT: u16 = 1323u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_TRUE_HEADING: u16 = 1045u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ULTRAVIOLET_A_LIGHT: u16 = 1243u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ULTRAVIOLET_B_LIGHT: u16 = 1244u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_ULTRAVIOLET_INDEX: u16 = 1245u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_VERTICAL_DILUTION_OF_PRECISION: u16 = 1042u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_VOLATILE_ORGANIC_COMPOUND_CONCENTRATION: u16 = 1081u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_VOLTAGE: u16 = 1286u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_WEIGHT: u16 = 1176u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_WIND_DIRECTION: u16 = 1077u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_WIND_SPEED: u16 = 1078u16;
pub const HID_USAGE_SENSORS_DATA_FIELD_YEAR: u16 = 1313u16;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_FRIDAY: u16 = 2245u16;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_MONDAY: u16 = 2241u16;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_SATURDAY: u16 = 2246u16;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_SUNDAY: u16 = 2240u16;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_THURSDAY: u16 = 2244u16;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_TUESDAY: u16 = 2242u16;
pub const HID_USAGE_SENSORS_DAY_OF_WEEK_WEDNESDAY: u16 = 2243u16;
pub const HID_USAGE_SENSORS_DEVICE_POSITION_IN_HAND: u16 = 2435u16;
pub const HID_USAGE_SENSORS_DEVICE_POSITION_MOVING_IN_BAG: u16 = 2436u16;
pub const HID_USAGE_SENSORS_DEVICE_POSITION_ON_DESK: u16 = 2434u16;
pub const HID_USAGE_SENSORS_DEVICE_POSITION_STATIONARY_IN_BAG: u16 = 2437u16;
pub const HID_USAGE_SENSORS_DEVICE_POSITION_UNCHANGED: u16 = 2433u16;
pub const HID_USAGE_SENSORS_DEVICE_POSITION_UNKNOWN: u16 = 2432u16;
pub const HID_USAGE_SENSORS_ELECTRICAL: u16 = 32u16;
pub const HID_USAGE_SENSORS_ELECTRICAL_CAPACITANCE: u16 = 33u16;
pub const HID_USAGE_SENSORS_ELECTRICAL_CURRENT: u16 = 34u16;
pub const HID_USAGE_SENSORS_ELECTRICAL_FREQUENCY: u16 = 40u16;
pub const HID_USAGE_SENSORS_ELECTRICAL_INDUCTANCE: u16 = 36u16;
pub const HID_USAGE_SENSORS_ELECTRICAL_PERIOD: u16 = 41u16;
pub const HID_USAGE_SENSORS_ELECTRICAL_POTENTIOMETER: u16 = 39u16;
pub const HID_USAGE_SENSORS_ELECTRICAL_POWER: u16 = 35u16;
pub const HID_USAGE_SENSORS_ELECTRICAL_RESISTANCE: u16 = 37u16;
pub const HID_USAGE_SENSORS_ELECTRICAL_VOLTAGE: u16 = 38u16;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL: u16 = 48u16;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_AIR_QUALITY: u16 = 54u16;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_ATMOSPHERIC_PRESSURE: u16 = 49u16;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_HEAT_INDEX: u16 = 55u16;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_HUMIDITY: u16 = 50u16;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_OBJECT_PRESENCE: u16 = 58u16;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_OBJECT_PROXIMITY: u16 = 59u16;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_SURFACE_TEMPERATURE: u16 = 56u16;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_TEMPERATURE: u16 = 51u16;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_VOLATILE_ORGANIC_COMPOUNDS: u16 = 57u16;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_WIND_DIRECTION: u16 = 52u16;
pub const HID_USAGE_SENSORS_ENVIRONMENTAL_WIND_SPEED: u16 = 53u16;
pub const HID_USAGE_SENSORS_EVENT: u16 = 512u16;
pub const HID_USAGE_SENSORS_EVENT_SENSOR_EVENT: u16 = 514u16;
pub const HID_USAGE_SENSORS_EVENT_SENSOR_STATE: u16 = 513u16;
pub const HID_USAGE_SENSORS_EXPONENT_0: u16 = 2416u16;
pub const HID_USAGE_SENSORS_EXPONENT_1: u16 = 2417u16;
pub const HID_USAGE_SENSORS_EXPONENT_2: u16 = 2418u16;
pub const HID_USAGE_SENSORS_EXPONENT_3: u16 = 2419u16;
pub const HID_USAGE_SENSORS_EXPONENT_4: u16 = 2420u16;
pub const HID_USAGE_SENSORS_EXPONENT_5: u16 = 2421u16;
pub const HID_USAGE_SENSORS_EXPONENT_6: u16 = 2422u16;
pub const HID_USAGE_SENSORS_EXPONENT_7: u16 = 2423u16;
pub const HID_USAGE_SENSORS_EXPONENT_8: u16 = 2424u16;
pub const HID_USAGE_SENSORS_EXPONENT_9: u16 = 2425u16;
pub const HID_USAGE_SENSORS_EXPONENT_A: u16 = 2426u16;
pub const HID_USAGE_SENSORS_EXPONENT_B: u16 = 2427u16;
pub const HID_USAGE_SENSORS_EXPONENT_C: u16 = 2428u16;
pub const HID_USAGE_SENSORS_EXPONENT_D: u16 = 2429u16;
pub const HID_USAGE_SENSORS_EXPONENT_E: u16 = 2430u16;
pub const HID_USAGE_SENSORS_EXPONENT_F: u16 = 2431u16;
pub const HID_USAGE_SENSORS_FIX_QUALITY_DGPS: u16 = 2162u16;
pub const HID_USAGE_SENSORS_FIX_QUALITY_GPS: u16 = 2161u16;
pub const HID_USAGE_SENSORS_FIX_QUALITY_NO_FIX: u16 = 2160u16;
pub const HID_USAGE_SENSORS_FIX_TYPE_DGPS_SPS_MODE_FIX_VALID: u16 = 2178u16;
pub const HID_USAGE_SENSORS_FIX_TYPE_ESTIMATED_DEAD_RECKONED: u16 = 2182u16;
pub const HID_USAGE_SENSORS_FIX_TYPE_FLOAT_RTK: u16 = 2181u16;
pub const HID_USAGE_SENSORS_FIX_TYPE_GPS_PPS_MODE_FIX_VALID: u16 = 2179u16;
pub const HID_USAGE_SENSORS_FIX_TYPE_GPS_SPS_MODE_FIX_VALID: u16 = 2177u16;
pub const HID_USAGE_SENSORS_FIX_TYPE_MANUAL_INPUT_MODE: u16 = 2183u16;
pub const HID_USAGE_SENSORS_FIX_TYPE_NO_FIX: u16 = 2176u16;
pub const HID_USAGE_SENSORS_FIX_TYPE_REAL_TIME_KINEMATIC: u16 = 2180u16;
pub const HID_USAGE_SENSORS_FIX_TYPE_SIMULATOR_MODE: u16 = 2184u16;
pub const HID_USAGE_SENSORS_GESTURE: u16 = 208u16;
pub const HID_USAGE_SENSORS_GESTURE_CHASSIS_FLIP_GESTURE: u16 = 209u16;
pub const HID_USAGE_SENSORS_GESTURE_HINGE_FOLD_GESTURE: u16 = 210u16;
pub const HID_USAGE_SENSORS_GESTURE_STATE_CANCELLED: u16 = 2467u16;
pub const HID_USAGE_SENSORS_GESTURE_STATE_COMPLETED: u16 = 2466u16;
pub const HID_USAGE_SENSORS_GESTURE_STATE_STARTED: u16 = 2465u16;
pub const HID_USAGE_SENSORS_GESTURE_STATE_UNKNOWN: u16 = 2464u16;
pub const HID_USAGE_SENSORS_GPS_OPERATION_MODE_AUTOMATIC: u16 = 2193u16;
pub const HID_USAGE_SENSORS_GPS_OPERATION_MODE_MANUAL: u16 = 2192u16;
pub const HID_USAGE_SENSORS_GPS_SELECTION_MODE_AUTONOMOUS: u16 = 2208u16;
pub const HID_USAGE_SENSORS_GPS_SELECTION_MODE_DATA_NOT_VALID: u16 = 2213u16;
pub const HID_USAGE_SENSORS_GPS_SELECTION_MODE_DGPS: u16 = 2209u16;
pub const HID_USAGE_SENSORS_GPS_SELECTION_MODE_ESTIMATED_DEAD_RECKONED: u16 = 2210u16;
pub const HID_USAGE_SENSORS_GPS_SELECTION_MODE_MANUAL_INPUT: u16 = 2211u16;
pub const HID_USAGE_SENSORS_GPS_SELECTION_MODE_SIMULATOR: u16 = 2212u16;
pub const HID_USAGE_SENSORS_GPS_STATUS_DATA_NOT_VALID: u16 = 2225u16;
pub const HID_USAGE_SENSORS_GPS_STATUS_DATA_VALID: u16 = 2224u16;
pub const HID_USAGE_SENSORS_HINGE_FOLD_CONTRIBUTING_PANEL_BOTH: u16 = 2483u16;
pub const HID_USAGE_SENSORS_HINGE_FOLD_CONTRIBUTING_PANEL_PANEL_1: u16 = 2481u16;
pub const HID_USAGE_SENSORS_HINGE_FOLD_CONTRIBUTING_PANEL_PANEL_2: u16 = 2482u16;
pub const HID_USAGE_SENSORS_HINGE_FOLD_CONTRIBUTING_PANEL_UNKNOWN: u16 = 2480u16;
pub const HID_USAGE_SENSORS_HINGE_FOLD_TYPE_DECREASING: u16 = 2486u16;
pub const HID_USAGE_SENSORS_HINGE_FOLD_TYPE_INCREASING: u16 = 2485u16;
pub const HID_USAGE_SENSORS_HINGE_FOLD_TYPE_UNKNOWN: u16 = 2484u16;
pub const HID_USAGE_SENSORS_HUMAN_PRESENCE_DETECTION_TYPE_AUDIO_BIOMETRIC: u16 = 2499u16;
pub const HID_USAGE_SENSORS_HUMAN_PRESENCE_DETECTION_TYPE_FACIAL_BIOMETRIC: u16 = 2498u16;
pub const HID_USAGE_SENSORS_HUMAN_PRESENCE_DETECTION_TYPE_VENDORDEFINED_BIOMETRIC: u16 = 2497u16;
pub const HID_USAGE_SENSORS_HUMAN_PRESENCE_DETECTION_TYPE_VENDORDEFINED_NONBIOMETRIC: u16 = 2496u16;
pub const HID_USAGE_SENSORS_KIND_CATEGORY: u16 = 2256u16;
pub const HID_USAGE_SENSORS_KIND_DATA_FIELD: u16 = 2260u16;
pub const HID_USAGE_SENSORS_KIND_EVENT: u16 = 2258u16;
pub const HID_USAGE_SENSORS_KIND_PROPERTY: u16 = 2259u16;
pub const HID_USAGE_SENSORS_KIND_TYPE: u16 = 2257u16;
pub const HID_USAGE_SENSORS_LIGHT: u16 = 64u16;
pub const HID_USAGE_SENSORS_LIGHT_AMBIENT_LIGHT: u16 = 65u16;
pub const HID_USAGE_SENSORS_LIGHT_CONSUMER_INFRARED: u16 = 66u16;
pub const HID_USAGE_SENSORS_LIGHT_INFRARED_LIGHT: u16 = 67u16;
pub const HID_USAGE_SENSORS_LIGHT_ULTRAVIOLET_LIGHT: u16 = 69u16;
pub const HID_USAGE_SENSORS_LIGHT_VISIBLE_LIGHT: u16 = 68u16;
pub const HID_USAGE_SENSORS_LOCATION: u16 = 80u16;
pub const HID_USAGE_SENSORS_LOCATION_BROADCAST: u16 = 81u16;
pub const HID_USAGE_SENSORS_LOCATION_DEAD_RECKONING: u16 = 82u16;
pub const HID_USAGE_SENSORS_LOCATION_GPS_GLOBAL_POSITIONING_SYSTEM: u16 = 83u16;
pub const HID_USAGE_SENSORS_LOCATION_LOOKUP: u16 = 84u16;
pub const HID_USAGE_SENSORS_LOCATION_OTHER: u16 = 85u16;
pub const HID_USAGE_SENSORS_LOCATION_STATIC: u16 = 86u16;
pub const HID_USAGE_SENSORS_LOCATION_TRIANGULATION: u16 = 87u16;
pub const HID_USAGE_SENSORS_MAGNETOMETER_ACCURACY_HIGH: u16 = 2274u16;
pub const HID_USAGE_SENSORS_MAGNETOMETER_ACCURACY_LOW: u16 = 2272u16;
pub const HID_USAGE_SENSORS_MAGNETOMETER_ACCURACY_MEDIUM: u16 = 2273u16;
pub const HID_USAGE_SENSORS_MECHANICAL: u16 = 96u16;
pub const HID_USAGE_SENSORS_MECHANICAL_BOOLEAN_SWITCH: u16 = 97u16;
pub const HID_USAGE_SENSORS_MECHANICAL_BOOLEAN_SWITCH_ARRAY: u16 = 98u16;
pub const HID_USAGE_SENSORS_MECHANICAL_FORCE: u16 = 100u16;
pub const HID_USAGE_SENSORS_MECHANICAL_HALL_EFFECT_SWITCH: u16 = 105u16;
pub const HID_USAGE_SENSORS_MECHANICAL_HAPTIC_VIBRATOR: u16 = 104u16;
pub const HID_USAGE_SENSORS_MECHANICAL_MULTIVALUE_SWITCH: u16 = 99u16;
pub const HID_USAGE_SENSORS_MECHANICAL_PRESSURE: u16 = 101u16;
pub const HID_USAGE_SENSORS_MECHANICAL_STRAIN: u16 = 102u16;
pub const HID_USAGE_SENSORS_MECHANICAL_WEIGHT: u16 = 103u16;
pub const HID_USAGE_SENSORS_MODIFIER_ACCURACY: u16 = 16384u16;
pub const HID_USAGE_SENSORS_MODIFIER_CALIBRATION_MULTIPLIER: u16 = 36864u16;
pub const HID_USAGE_SENSORS_MODIFIER_CALIBRATION_OFFSET: u16 = 32768u16;
pub const HID_USAGE_SENSORS_MODIFIER_CHANGE_SENSITIVITY_ABSOLUTE: u16 = 4096u16;
pub const HID_USAGE_SENSORS_MODIFIER_CHANGE_SENSITIVITY_PERCENT_OF_RANGE: u16 = 53248u16;
pub const HID_USAGE_SENSORS_MODIFIER_CHANGE_SENSITIVITY_PERCENT_RELATIVE: u16 = 57344u16;
pub const HID_USAGE_SENSORS_MODIFIER_FREQUENCY_MAX: u16 = 45056u16;
pub const HID_USAGE_SENSORS_MODIFIER_MAXIMUM: u16 = 8192u16;
pub const HID_USAGE_SENSORS_MODIFIER_MINIMUM: u16 = 12288u16;
pub const HID_USAGE_SENSORS_MODIFIER_PERIOD_MAX: u16 = 49152u16;
pub const HID_USAGE_SENSORS_MODIFIER_REPORT_INTERVAL: u16 = 40960u16;
pub const HID_USAGE_SENSORS_MODIFIER_RESOLUTION: u16 = 20480u16;
pub const HID_USAGE_SENSORS_MODIFIER_THRESHOLD_HIGH: u16 = 24576u16;
pub const HID_USAGE_SENSORS_MODIFIER_THRESHOLD_LOW: u16 = 28672u16;
pub const HID_USAGE_SENSORS_MODIFIER_VENDOR_RESERVED: u16 = 61440u16;
pub const HID_USAGE_SENSORS_MOTION: u16 = 112u16;
pub const HID_USAGE_SENSORS_MOTION_ACCELEROMETER: u16 = 121u16;
pub const HID_USAGE_SENSORS_MOTION_ACCELEROMETER_1D: u16 = 113u16;
pub const HID_USAGE_SENSORS_MOTION_ACCELEROMETER_2D: u16 = 114u16;
pub const HID_USAGE_SENSORS_MOTION_ACCELEROMETER_3D: u16 = 115u16;
pub const HID_USAGE_SENSORS_MOTION_GRAVITY_VECTOR: u16 = 123u16;
pub const HID_USAGE_SENSORS_MOTION_GYROMETER: u16 = 122u16;
pub const HID_USAGE_SENSORS_MOTION_GYROMETER_1D: u16 = 116u16;
pub const HID_USAGE_SENSORS_MOTION_GYROMETER_2D: u16 = 117u16;
pub const HID_USAGE_SENSORS_MOTION_GYROMETER_3D: u16 = 118u16;
pub const HID_USAGE_SENSORS_MOTION_LINEAR_ACCELEROMETER: u16 = 124u16;
pub const HID_USAGE_SENSORS_MOTION_MOTION_DETECTOR: u16 = 119u16;
pub const HID_USAGE_SENSORS_MOTION_SPEEDOMETER: u16 = 120u16;
pub const HID_USAGE_SENSORS_ORIENTATION: u16 = 128u16;
pub const HID_USAGE_SENSORS_ORIENTATION_COMPASS: u16 = 139u16;
pub const HID_USAGE_SENSORS_ORIENTATION_COMPASS_1D: u16 = 129u16;
pub const HID_USAGE_SENSORS_ORIENTATION_COMPASS_2D: u16 = 130u16;
pub const HID_USAGE_SENSORS_ORIENTATION_COMPASS_3D: u16 = 131u16;
pub const HID_USAGE_SENSORS_ORIENTATION_DEVICE_ORIENTATION: u16 = 138u16;
pub const HID_USAGE_SENSORS_ORIENTATION_DISTANCE: u16 = 141u16;
pub const HID_USAGE_SENSORS_ORIENTATION_DISTANCE_1D: u16 = 135u16;
pub const HID_USAGE_SENSORS_ORIENTATION_DISTANCE_2D: u16 = 136u16;
pub const HID_USAGE_SENSORS_ORIENTATION_DISTANCE_3D: u16 = 137u16;
pub const HID_USAGE_SENSORS_ORIENTATION_EXTENDED: u16 = 192u16;
pub const HID_USAGE_SENSORS_ORIENTATION_EXTENDED_GEOMAGNETIC_ORIENTATION: u16 = 193u16;
pub const HID_USAGE_SENSORS_ORIENTATION_EXTENDED_MAGNETOMETER: u16 = 194u16;
pub const HID_USAGE_SENSORS_ORIENTATION_INCLINOMETER: u16 = 140u16;
pub const HID_USAGE_SENSORS_ORIENTATION_INCLINOMETER_1D: u16 = 132u16;
pub const HID_USAGE_SENSORS_ORIENTATION_INCLINOMETER_2D: u16 = 133u16;
pub const HID_USAGE_SENSORS_ORIENTATION_INCLINOMETER_3D: u16 = 134u16;
pub const HID_USAGE_SENSORS_ORIENTATION_RELATIVE_ORIENTATION: u16 = 142u16;
pub const HID_USAGE_SENSORS_ORIENTATION_SIMPLE_ORIENTATION: u16 = 143u16;
pub const HID_USAGE_SENSORS_OTHER: u16 = 224u16;
pub const HID_USAGE_SENSORS_OTHER_CUSTOM: u16 = 225u16;
pub const HID_USAGE_SENSORS_OTHER_GENERIC: u16 = 226u16;
pub const HID_USAGE_SENSORS_OTHER_GENERIC_ENUMERATOR: u16 = 227u16;
pub const HID_USAGE_SENSORS_OTHER_HINGE_ANGLE: u16 = 228u16;
pub const HID_USAGE_SENSORS_PERSONAL_ACTIVITY: u16 = 176u16;
pub const HID_USAGE_SENSORS_PERSONAL_ACTIVITY_ACTIVITY_DETECTION: u16 = 177u16;
pub const HID_USAGE_SENSORS_PERSONAL_ACTIVITY_DEVICE_POSITION: u16 = 178u16;
pub const HID_USAGE_SENSORS_PERSONAL_ACTIVITY_FLOOR_TRACKER: u16 = 179u16;
pub const HID_USAGE_SENSORS_PERSONAL_ACTIVITY_PEDOMETER: u16 = 180u16;
pub const HID_USAGE_SENSORS_PERSONAL_ACTIVITY_STEP_DETECTION: u16 = 181u16;
pub const HID_USAGE_SENSORS_POWER_STATE_D0_FULL_POWER: u16 = 2129u16;
pub const HID_USAGE_SENSORS_POWER_STATE_D1_LOW_POWER: u16 = 2130u16;
pub const HID_USAGE_SENSORS_POWER_STATE_D2_STANDBY_POWER_WITH_WAKEUP: u16 = 2131u16;
pub const HID_USAGE_SENSORS_POWER_STATE_D3_SLEEP_WITH_WAKEUP: u16 = 2132u16;
pub const HID_USAGE_SENSORS_POWER_STATE_D4_POWER_OFF: u16 = 2133u16;
pub const HID_USAGE_SENSORS_POWER_STATE_UNDEFINED: u16 = 2128u16;
pub const HID_USAGE_SENSORS_PROPERTY: u16 = 768u16;
pub const HID_USAGE_SENSORS_PROPERTY_ACCURACY: u16 = 786u16;
pub const HID_USAGE_SENSORS_PROPERTY_ARM_ALARM: u16 = 1333u16;
pub const HID_USAGE_SENSORS_PROPERTY_AUTO_BRIGHTNESS_PREFERRED: u16 = 1250u16;
pub const HID_USAGE_SENSORS_PROPERTY_AUTO_COLOR_PREFERRED: u16 = 1251u16;
pub const HID_USAGE_SENSORS_PROPERTY_BACKWARD_VIBRATION_SPEED: u16 = 1187u16;
pub const HID_USAGE_SENSORS_PROPERTY_CHANGE_SENSITIVITY_ABSOLUTE: u16 = 783u16;
pub const HID_USAGE_SENSORS_PROPERTY_CHANGE_SENSITIVITY_PERCENT_OF_RANGE: u16 = 784u16;
pub const HID_USAGE_SENSORS_PROPERTY_CHANGE_SENSITIVITY_PERCENT_RELATIVE: u16 = 785u16;
pub const HID_USAGE_SENSORS_PROPERTY_CONSUMER_IR_SENTENCE_SEND: u16 = 1248u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM: u16 = 1472u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_1: u16 = 1473u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_10: u16 = 1482u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_11: u16 = 1483u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_12: u16 = 1484u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_13: u16 = 1485u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_14: u16 = 1486u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_15: u16 = 1487u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_16: u16 = 1488u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_2: u16 = 1474u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_3: u16 = 1475u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_4: u16 = 1476u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_5: u16 = 1477u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_6: u16 = 1478u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_7: u16 = 1479u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_8: u16 = 1480u16;
pub const HID_USAGE_SENSORS_PROPERTY_CUSTOM_VALUE_9: u16 = 1481u16;
pub const HID_USAGE_SENSORS_PROPERTY_DAYLIGHT_SAVINGS_TIME_OBSERVED: u16 = 1331u16;
pub const HID_USAGE_SENSORS_PROPERTY_ENUMERATOR_TABLE_ROW_COUNT: u16 = 1410u16;
pub const HID_USAGE_SENSORS_PROPERTY_ENUMERATOR_TABLE_ROW_INDEX: u16 = 1409u16;
pub const HID_USAGE_SENSORS_PROPERTY_ENVIRONMENTAL: u16 = 1088u16;
pub const HID_USAGE_SENSORS_PROPERTY_FIRMWARE_VERSION: u16 = 780u16;
pub const HID_USAGE_SENSORS_PROPERTY_FLOOR_HEIGHT: u16 = 1445u16;
pub const HID_USAGE_SENSORS_PROPERTY_FLUSH_FIFO_EVENTS: u16 = 796u16;
pub const HID_USAGE_SENSORS_PROPERTY_FORWARD_VIBRATION_SPEED: u16 = 1186u16;
pub const HID_USAGE_SENSORS_PROPERTY_FRIENDLY_NAME: u16 = 769u16;
pub const HID_USAGE_SENSORS_PROPERTY_GENERIC: u16 = 1408u16;
pub const HID_USAGE_SENSORS_PROPERTY_HARDWARE_REVISION: u16 = 779u16;
pub const HID_USAGE_SENSORS_PROPERTY_HUMAN_PRESENCE_DETECTION_TYPE: u16 = 799u16;
pub const HID_USAGE_SENSORS_PROPERTY_IS_PRIMARY: u16 = 798u16;
pub const HID_USAGE_SENSORS_PROPERTY_LIGHT: u16 = 1247u16;
pub const HID_USAGE_SENSORS_PROPERTY_LOCATION: u16 = 1066u16;
pub const HID_USAGE_SENSORS_PROPERTY_LOCATION_DESIRED_ACCURACY: u16 = 1067u16;
pub const HID_USAGE_SENSORS_PROPERTY_MAXIMUM: u16 = 788u16;
pub const HID_USAGE_SENSORS_PROPERTY_MAXIMUM_FIFO_EVENTS: u16 = 794u16;
pub const HID_USAGE_SENSORS_PROPERTY_MAXIMUM_POWER_CONSUMPTION: u16 = 797u16;
pub const HID_USAGE_SENSORS_PROPERTY_MECHANICAL: u16 = 1184u16;
pub const HID_USAGE_SENSORS_PROPERTY_MINIMUM: u16 = 789u16;
pub const HID_USAGE_SENSORS_PROPERTY_MINIMUM_ACTIVITY_DETECTION_INTERVAL: u16 = 1440u16;
pub const HID_USAGE_SENSORS_PROPERTY_MINIMUM_REPORT_INTERVAL: u16 = 772u16;
pub const HID_USAGE_SENSORS_PROPERTY_NFC_SENTENCE_SEND: u16 = 1273u16;
pub const HID_USAGE_SENSORS_PROPERTY_PERSISTENT_UNIQUE_ID: u16 = 770u16;
pub const HID_USAGE_SENSORS_PROPERTY_POWER_STATE: u16 = 793u16;
pub const HID_USAGE_SENSORS_PROPERTY_REFERENCE_PRESSURE: u16 = 1089u16;
pub const HID_USAGE_SENSORS_PROPERTY_RELEASE_DATE: u16 = 781u16;
pub const HID_USAGE_SENSORS_PROPERTY_REPORTING_STATE: u16 = 790u16;
pub const HID_USAGE_SENSORS_PROPERTY_REPORT_INTERVAL: u16 = 782u16;
pub const HID_USAGE_SENSORS_PROPERTY_REPORT_LATENCY: u16 = 795u16;
pub const HID_USAGE_SENSORS_PROPERTY_RESOLUTION: u16 = 787u16;
pub const HID_USAGE_SENSORS_PROPERTY_RESPONSE_CURVE: u16 = 792u16;
pub const HID_USAGE_SENSORS_PROPERTY_SAMPLING_RATE: u16 = 791u16;
pub const HID_USAGE_SENSORS_PROPERTY_SCANNER: u16 = 1272u16;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_CONNECTION_TYPE: u16 = 777u16;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_DESCRIPTION: u16 = 776u16;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_DEVICE_PATH: u16 = 778u16;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_MANUFACTURER: u16 = 773u16;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_MODEL: u16 = 774u16;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_SERIAL_NUMBER: u16 = 775u16;
pub const HID_USAGE_SENSORS_PROPERTY_SENSOR_STATUS: u16 = 771u16;
pub const HID_USAGE_SENSORS_PROPERTY_SUBSCRIBED_ACTIVITY_TYPES: u16 = 1442u16;
pub const HID_USAGE_SENSORS_PROPERTY_SUBSCRIBED_STEP_TYPES: u16 = 1444u16;
pub const HID_USAGE_SENSORS_PROPERTY_SUPPORTED_ACTIVITY_TYPES: u16 = 1441u16;
pub const HID_USAGE_SENSORS_PROPERTY_SUPPORTED_STEP_TYPES: u16 = 1443u16;
pub const HID_USAGE_SENSORS_PROPERTY_TIME: u16 = 1328u16;
pub const HID_USAGE_SENSORS_PROPERTY_TIME_TRIM_ADJUSTMENT: u16 = 1332u16;
pub const HID_USAGE_SENSORS_PROPERTY_TIME_ZONE_NAME: u16 = 1330u16;
pub const HID_USAGE_SENSORS_PROPERTY_TIME_ZONE_OFFSET_FROM_UTC: u16 = 1329u16;
pub const HID_USAGE_SENSORS_PROPERTY_VIBRATION_STATE: u16 = 1185u16;
pub const HID_USAGE_SENSORS_REPORTING_STATE_ANYTIME_SEL: u16 = 2118u16;
pub const HID_USAGE_SENSORS_REPORTING_STATE_REPORT_ALL_EVENTS: u16 = 2113u16;
pub const HID_USAGE_SENSORS_REPORTING_STATE_REPORT_NO_EVENTS: u16 = 2112u16;
pub const HID_USAGE_SENSORS_REPORTING_STATE_REPORT_THRESHOLD_EVENTS: u16 = 2114u16;
pub const HID_USAGE_SENSORS_REPORTING_STATE_WAKE_ON_ALL_EVENTS: u16 = 2116u16;
pub const HID_USAGE_SENSORS_REPORTING_STATE_WAKE_ON_NO_EVENTS: u16 = 2115u16;
pub const HID_USAGE_SENSORS_REPORTING_STATE_WAKE_ON_THRESHOLD_EVENTS: u16 = 2117u16;
pub const HID_USAGE_SENSORS_SCANNER: u16 = 144u16;
pub const HID_USAGE_SENSORS_SCANNER_BARCODE: u16 = 145u16;
pub const HID_USAGE_SENSORS_SCANNER_NFC: u16 = 147u16;
pub const HID_USAGE_SENSORS_SCANNER_RFID: u16 = 146u16;
pub const HID_USAGE_SENSORS_SENSOR: u16 = 1u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_CHANGE_SENSITIVITY: u16 = 2069u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_COMPLEX_TRIGGER: u16 = 2080u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_DATA_UPDATED: u16 = 2067u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_FREQUENCY_EXCEEDED: u16 = 2079u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_HIGH_THRESHOLD_CROSS_DOWNWARD: u16 = 2073u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_HIGH_THRESHOLD_CROSS_UPWARD: u16 = 2072u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_LOW_THRESHOLD_CROSS_DOWNWARD: u16 = 2075u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_LOW_THRESHOLD_CROSS_UPWARD: u16 = 2074u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_PERIOD_EXCEEDED: u16 = 2078u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_POLL_RESPONSE: u16 = 2068u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_PROPERTY_CHANGED: u16 = 2066u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_RANGE_MAXIMUM_REACHED: u16 = 2070u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_RANGE_MINIMUM_REACHED: u16 = 2071u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_STATE_CHANGED: u16 = 2065u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_UNKNOWN: u16 = 2064u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_ZERO_THRESHOLD_CROSS_DOWNWARD: u16 = 2077u16;
pub const HID_USAGE_SENSORS_SENSOR_EVENT_ZERO_THRESHOLD_CROSS_UPWARD: u16 = 2076u16;
pub const HID_USAGE_SENSORS_SENSOR_STATE_ACCESS_DENIED: u16 = 2053u16;
pub const HID_USAGE_SENSORS_SENSOR_STATE_ERROR: u16 = 2054u16;
pub const HID_USAGE_SENSORS_SENSOR_STATE_INITIALIZING: u16 = 2052u16;
pub const HID_USAGE_SENSORS_SENSOR_STATE_NOT_AVAILABLE: u16 = 2050u16;
pub const HID_USAGE_SENSORS_SENSOR_STATE_NO_DATA: u16 = 2051u16;
pub const HID_USAGE_SENSORS_SENSOR_STATE_READY: u16 = 2049u16;
pub const HID_USAGE_SENSORS_SENSOR_STATE_UNDEFINED: u16 = 2048u16;
pub const HID_USAGE_SENSORS_SIMPLE_ORIENTATION_DIRECTION_FACE_DOWN: u16 = 2293u16;
pub const HID_USAGE_SENSORS_SIMPLE_ORIENTATION_DIRECTION_FACE_UP: u16 = 2292u16;
pub const HID_USAGE_SENSORS_SIMPLE_ORIENTATION_DIRECTION_NOT_ROTATED: u16 = 2288u16;
pub const HID_USAGE_SENSORS_SIMPLE_ORIENTATION_DIRECTION_ROTATED_180_DEGREES_CCW: u16 = 2290u16;
pub const HID_USAGE_SENSORS_SIMPLE_ORIENTATION_DIRECTION_ROTATED_270_DEGREES_CCW: u16 = 2291u16;
pub const HID_USAGE_SENSORS_SIMPLE_ORIENTATION_DIRECTION_ROTATED_90_DEGREES_CCW: u16 = 2289u16;
pub const HID_USAGE_SENSORS_STEP_TYPE_RUNNING: u16 = 2450u16;
pub const HID_USAGE_SENSORS_STEP_TYPE_UNKNOWN: u16 = 2448u16;
pub const HID_USAGE_SENSORS_STEP_TYPE_WALKING: u16 = 2449u16;
pub const HID_USAGE_SENSORS_TIME: u16 = 160u16;
pub const HID_USAGE_SENSORS_TIME_ALARM_TIMER: u16 = 161u16;
pub const HID_USAGE_SENSORS_TIME_REAL_TIME_CLOCK: u16 = 162u16;
pub const HID_USAGE_SENSORS_UNIT_AMPERE: u16 = 2379u16;
pub const HID_USAGE_SENSORS_UNIT_BAR: u16 = 2385u16;
pub const HID_USAGE_SENSORS_UNIT_BITS: u16 = 2398u16;
pub const HID_USAGE_SENSORS_UNIT_BYTES: u16 = 2396u16;
pub const HID_USAGE_SENSORS_UNIT_DEGREES: u16 = 2388u16;
pub const HID_USAGE_SENSORS_UNIT_DEGREESSECOND: u16 = 2389u16;
pub const HID_USAGE_SENSORS_UNIT_DEGREESSECONDSECOND: u16 = 2390u16;
pub const HID_USAGE_SENSORS_UNIT_DEGREES_ANTICLOCKWISE: u16 = 2386u16;
pub const HID_USAGE_SENSORS_UNIT_DEGREES_CELSIUS: u16 = 2371u16;
pub const HID_USAGE_SENSORS_UNIT_DEGREES_CLOCKWISE: u16 = 2387u16;
pub const HID_USAGE_SENSORS_UNIT_DEGREES_KELVIN: u16 = 2370u16;
pub const HID_USAGE_SENSORS_UNIT_FARAD: u16 = 2378u16;
pub const HID_USAGE_SENSORS_UNIT_G: u16 = 2395u16;
pub const HID_USAGE_SENSORS_UNIT_HENRY: u16 = 2381u16;
pub const HID_USAGE_SENSORS_UNIT_HERTZ: u16 = 2384u16;
pub const HID_USAGE_SENSORS_UNIT_KILOGRAM: u16 = 2375u16;
pub const HID_USAGE_SENSORS_UNIT_KNOT: u16 = 2391u16;
pub const HID_USAGE_SENSORS_UNIT_LUX: u16 = 2369u16;
pub const HID_USAGE_SENSORS_UNIT_METER: u16 = 2376u16;
pub const HID_USAGE_SENSORS_UNIT_METERSSECOND: u16 = 2374u16;
pub const HID_USAGE_SENSORS_UNIT_METERSSECONDSECOND: u16 = 2377u16;
pub const HID_USAGE_SENSORS_UNIT_MILLIGAUSS: u16 = 2397u16;
pub const HID_USAGE_SENSORS_UNIT_MILLISECOND: u16 = 2394u16;
pub const HID_USAGE_SENSORS_UNIT_NEWTON: u16 = 2373u16;
pub const HID_USAGE_SENSORS_UNIT_NOT_SPECIFIED: u16 = 2368u16;
pub const HID_USAGE_SENSORS_UNIT_OHM: u16 = 2382u16;
pub const HID_USAGE_SENSORS_UNIT_PASCAL: u16 = 2372u16;
pub const HID_USAGE_SENSORS_UNIT_PERCENT: u16 = 2392u16;
pub const HID_USAGE_SENSORS_UNIT_SECOND: u16 = 2393u16;
pub const HID_USAGE_SENSORS_UNIT_VOLT: u16 = 2383u16;
pub const HID_USAGE_SENSORS_UNIT_WATT: u16 = 2380u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_1: u16 = 240u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_10: u16 = 249u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_11: u16 = 250u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_12: u16 = 251u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_13: u16 = 252u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_14: u16 = 253u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_15: u16 = 254u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_16: u16 = 255u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_2: u16 = 241u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_3: u16 = 242u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_4: u16 = 243u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_5: u16 = 244u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_6: u16 = 245u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_7: u16 = 246u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_8: u16 = 247u16;
pub const HID_USAGE_SENSORS_VENDOR_RESERVED_9: u16 = 248u16;
pub const HID_USAGE_SENSORS_VT_BOOL: u16 = 2305u16;
pub const HID_USAGE_SENSORS_VT_CLSID: u16 = 2318u16;
pub const HID_USAGE_SENSORS_VT_F16E0: u16 = 2320u16;
pub const HID_USAGE_SENSORS_VT_F16E1: u16 = 2321u16;
pub const HID_USAGE_SENSORS_VT_F16E2: u16 = 2322u16;
pub const HID_USAGE_SENSORS_VT_F16E3: u16 = 2323u16;
pub const HID_USAGE_SENSORS_VT_F16E4: u16 = 2324u16;
pub const HID_USAGE_SENSORS_VT_F16E5: u16 = 2325u16;
pub const HID_USAGE_SENSORS_VT_F16E6: u16 = 2326u16;
pub const HID_USAGE_SENSORS_VT_F16E7: u16 = 2327u16;
pub const HID_USAGE_SENSORS_VT_F16E8: u16 = 2328u16;
pub const HID_USAGE_SENSORS_VT_F16E9: u16 = 2329u16;
pub const HID_USAGE_SENSORS_VT_F16EA: u16 = 2330u16;
pub const HID_USAGE_SENSORS_VT_F16EB: u16 = 2331u16;
pub const HID_USAGE_SENSORS_VT_F16EC: u16 = 2332u16;
pub const HID_USAGE_SENSORS_VT_F16ED: u16 = 2333u16;
pub const HID_USAGE_SENSORS_VT_F16EE: u16 = 2334u16;
pub const HID_USAGE_SENSORS_VT_F16EF: u16 = 2335u16;
pub const HID_USAGE_SENSORS_VT_F32E0: u16 = 2336u16;
pub const HID_USAGE_SENSORS_VT_F32E1: u16 = 2337u16;
pub const HID_USAGE_SENSORS_VT_F32E2: u16 = 2338u16;
pub const HID_USAGE_SENSORS_VT_F32E3: u16 = 2339u16;
pub const HID_USAGE_SENSORS_VT_F32E4: u16 = 2340u16;
pub const HID_USAGE_SENSORS_VT_F32E5: u16 = 2341u16;
pub const HID_USAGE_SENSORS_VT_F32E6: u16 = 2342u16;
pub const HID_USAGE_SENSORS_VT_F32E7: u16 = 2343u16;
pub const HID_USAGE_SENSORS_VT_F32E8: u16 = 2344u16;
pub const HID_USAGE_SENSORS_VT_F32E9: u16 = 2345u16;
pub const HID_USAGE_SENSORS_VT_F32EA: u16 = 2346u16;
pub const HID_USAGE_SENSORS_VT_F32EB: u16 = 2347u16;
pub const HID_USAGE_SENSORS_VT_F32EC: u16 = 2348u16;
pub const HID_USAGE_SENSORS_VT_F32ED: u16 = 2349u16;
pub const HID_USAGE_SENSORS_VT_F32EE: u16 = 2350u16;
pub const HID_USAGE_SENSORS_VT_F32EF: u16 = 2351u16;
pub const HID_USAGE_SENSORS_VT_I1: u16 = 2307u16;
pub const HID_USAGE_SENSORS_VT_I2: u16 = 2309u16;
pub const HID_USAGE_SENSORS_VT_I4: u16 = 2311u16;
pub const HID_USAGE_SENSORS_VT_I8: u16 = 2313u16;
pub const HID_USAGE_SENSORS_VT_NULL: u16 = 2304u16;
pub const HID_USAGE_SENSORS_VT_R4: u16 = 2314u16;
pub const HID_USAGE_SENSORS_VT_R8: u16 = 2315u16;
pub const HID_USAGE_SENSORS_VT_STR: u16 = 2317u16;
pub const HID_USAGE_SENSORS_VT_UI1: u16 = 2306u16;
pub const HID_USAGE_SENSORS_VT_UI2: u16 = 2308u16;
pub const HID_USAGE_SENSORS_VT_UI4: u16 = 2310u16;
pub const HID_USAGE_SENSORS_VT_UI8: u16 = 2312u16;
pub const HID_USAGE_SENSORS_VT_VECTOR_VT_UI1: u16 = 2319u16;
pub const HID_USAGE_SENSORS_VT_WSTR: u16 = 2316u16;
pub const HID_USAGE_SIMULATION_ACCELERATOR: u16 = 196u16;
pub const HID_USAGE_SIMULATION_AILERON: u16 = 176u16;
pub const HID_USAGE_SIMULATION_AILERON_TRIM: u16 = 177u16;
pub const HID_USAGE_SIMULATION_AIRPLANE_SIMULATION_DEVICE: u16 = 9u16;
pub const HID_USAGE_SIMULATION_ANTI_TORQUE_CONTROL: u16 = 178u16;
pub const HID_USAGE_SIMULATION_AUTOMOBILE_SIMULATION_DEVICE: u16 = 2u16;
pub const HID_USAGE_SIMULATION_AUTOPIOLOT_ENABLE: u16 = 179u16;
pub const HID_USAGE_SIMULATION_BALLAST: u16 = 204u16;
pub const HID_USAGE_SIMULATION_BARREL_ELEVATION: u16 = 202u16;
pub const HID_USAGE_SIMULATION_BICYCLE_CRANK: u16 = 205u16;
pub const HID_USAGE_SIMULATION_BICYCLE_SIMULATION_DEVICE: u16 = 12u16;
pub const HID_USAGE_SIMULATION_BRAKE: u16 = 197u16;
pub const HID_USAGE_SIMULATION_CHAFF_RELEASE: u16 = 180u16;
pub const HID_USAGE_SIMULATION_CLUTCH: u16 = 198u16;
pub const HID_USAGE_SIMULATION_COLLECTIVE_CONTROL: u16 = 181u16;
pub const HID_USAGE_SIMULATION_CYCLIC_CONTROL: u16 = 34u16;
pub const HID_USAGE_SIMULATION_CYCLIC_TRIM: u16 = 35u16;
pub const HID_USAGE_SIMULATION_DIVE_BRAKE: u16 = 182u16;
pub const HID_USAGE_SIMULATION_DIVE_PLANE: u16 = 203u16;
pub const HID_USAGE_SIMULATION_ELECTRONIC_COUNTERMEASURES: u16 = 183u16;
pub const HID_USAGE_SIMULATION_ELEVATOR: u16 = 184u16;
pub const HID_USAGE_SIMULATION_ELEVATOR_TRIM: u16 = 185u16;
pub const HID_USAGE_SIMULATION_FLARE_RELEASE: u16 = 189u16;
pub const HID_USAGE_SIMULATION_FLIGHT_COMMUNICATIONS: u16 = 188u16;
pub const HID_USAGE_SIMULATION_FLIGHT_CONTROL_STICK: u16 = 32u16;
pub const HID_USAGE_SIMULATION_FLIGHT_SIMULATION_DEVICE: u16 = 1u16;
pub const HID_USAGE_SIMULATION_FLIGHT_STICK: u16 = 33u16;
pub const HID_USAGE_SIMULATION_FLIGHT_YOKE: u16 = 36u16;
pub const HID_USAGE_SIMULATION_FRONT_BRAKE: u16 = 207u16;
pub const HID_USAGE_SIMULATION_HANDLE_BARS: u16 = 206u16;
pub const HID_USAGE_SIMULATION_HELICOPTER_SIMULATION_DEVICE: u16 = 10u16;
pub const HID_USAGE_SIMULATION_LANDING_GEAR: u16 = 190u16;
pub const HID_USAGE_SIMULATION_MAGIC_CARPET_SIMULATION_DEVICE: u16 = 11u16;
pub const HID_USAGE_SIMULATION_MOTORCYCLE_SIMULATION_DEVICE: u16 = 7u16;
pub const HID_USAGE_SIMULATION_REAR_BRAKE: u16 = 208u16;
pub const HID_USAGE_SIMULATION_RUDDER: u16 = 186u16;
pub const HID_USAGE_SIMULATION_SAILING_SIMULATION_DEVICE: u16 = 6u16;
pub const HID_USAGE_SIMULATION_SHIFTER: u16 = 199u16;
pub const HID_USAGE_SIMULATION_SPACESHIP_SIMULATION_DEVICE: u16 = 4u16;
pub const HID_USAGE_SIMULATION_SPORTS_SIMULATION_DEVICE: u16 = 8u16;
pub const HID_USAGE_SIMULATION_STEERING: u16 = 200u16;
pub const HID_USAGE_SIMULATION_SUBMARINE_SIMULATION_DEVICE: u16 = 5u16;
pub const HID_USAGE_SIMULATION_TANK_SIMULATION_DEVICE: u16 = 3u16;
pub const HID_USAGE_SIMULATION_THROTTLE: u16 = 187u16;
pub const HID_USAGE_SIMULATION_TOE_BRAKE: u16 = 191u16;
pub const HID_USAGE_SIMULATION_TRACK_CONTROL: u16 = 37u16;
pub const HID_USAGE_SIMULATION_TRIGGER: u16 = 192u16;
pub const HID_USAGE_SIMULATION_TURRET_DIRECTION: u16 = 201u16;
pub const HID_USAGE_SIMULATION_WEAPONS_ARM: u16 = 193u16;
pub const HID_USAGE_SIMULATION_WEAPONS_SELECT: u16 = 194u16;
pub const HID_USAGE_SIMULATION_WING_FLAPS: u16 = 195u16;
pub const HID_USAGE_SOC_FILE_OFFSET_IN_BYTES: u16 = 4u16;
pub const HID_USAGE_SOC_FILE_PAYLOAD: u16 = 6u16;
pub const HID_USAGE_SOC_FILE_PAYLOAD_CONTAINS_LAST_BYTES: u16 = 8u16;
pub const HID_USAGE_SOC_FILE_PAYLOAD_SIZE_IN_BYTES: u16 = 7u16;
pub const HID_USAGE_SOC_FILE_TRANSFER_SIZE_MAX_IN_BYTES: u16 = 5u16;
pub const HID_USAGE_SOC_FILE_TRANSFER_STOP: u16 = 9u16;
pub const HID_USAGE_SOC_FILE_TRANSFER_TILL_END: u16 = 10u16;
pub const HID_USAGE_SOC_FIRMWARE_FILE_ID: u16 = 3u16;
pub const HID_USAGE_SOC_FIRMWARE_TRANSFER: u16 = 2u16;
pub const HID_USAGE_SOC_SOC_CONTROL: u16 = 1u16;
pub const HID_USAGE_SPORT_10_IRON: u16 = 90u16;
pub const HID_USAGE_SPORT_11_IRON: u16 = 91u16;
pub const HID_USAGE_SPORT_1_IRON: u16 = 81u16;
pub const HID_USAGE_SPORT_1_WOOD: u16 = 95u16;
pub const HID_USAGE_SPORT_2_IRON: u16 = 82u16;
pub const HID_USAGE_SPORT_3_IRON: u16 = 83u16;
pub const HID_USAGE_SPORT_3_WOOD: u16 = 96u16;
pub const HID_USAGE_SPORT_4_IRON: u16 = 84u16;
pub const HID_USAGE_SPORT_5_IRON: u16 = 85u16;
pub const HID_USAGE_SPORT_5_WOOD: u16 = 97u16;
pub const HID_USAGE_SPORT_6_IRON: u16 = 86u16;
pub const HID_USAGE_SPORT_7_IRON: u16 = 87u16;
pub const HID_USAGE_SPORT_7_WOOD: u16 = 98u16;
pub const HID_USAGE_SPORT_8_IRON: u16 = 88u16;
pub const HID_USAGE_SPORT_9_IRON: u16 = 89u16;
pub const HID_USAGE_SPORT_9_WOOD: u16 = 99u16;
pub const HID_USAGE_SPORT_BASEBALL_BAT: u16 = 1u16;
pub const HID_USAGE_SPORT_FOLLOW_THROUGH: u16 = 54u16;
pub const HID_USAGE_SPORT_GOLF_CLUB: u16 = 2u16;
pub const HID_USAGE_SPORT_HEEL_TOE: u16 = 53u16;
pub const HID_USAGE_SPORT_HEIGHT: u16 = 57u16;
pub const HID_USAGE_SPORT_LOFT_WEDGE: u16 = 93u16;
pub const HID_USAGE_SPORT_OAR: u16 = 48u16;
pub const HID_USAGE_SPORT_POWER_WEDGE: u16 = 94u16;
pub const HID_USAGE_SPORT_PUTTER: u16 = 80u16;
pub const HID_USAGE_SPORT_RATE: u16 = 50u16;
pub const HID_USAGE_SPORT_ROWING_MACHINE: u16 = 3u16;
pub const HID_USAGE_SPORT_SAND_WEDGE: u16 = 92u16;
pub const HID_USAGE_SPORT_SLOPE: u16 = 49u16;
pub const HID_USAGE_SPORT_STICK_FACE_ANGLE: u16 = 52u16;
pub const HID_USAGE_SPORT_STICK_SPEED: u16 = 51u16;
pub const HID_USAGE_SPORT_STICK_TYPE: u16 = 56u16;
pub const HID_USAGE_SPORT_TEMPO: u16 = 55u16;
pub const HID_USAGE_SPORT_TREADMILL: u16 = 4u16;
pub const HID_USAGE_TELEPHONY_ACTIVATE_HANDSET_AUDIO: u16 = 243u16;
pub const HID_USAGE_TELEPHONY_ADDRESS_BOOK_ID: u16 = 327u16;
pub const HID_USAGE_TELEPHONY_ALERT_SOUND_CONFIRM: u16 = 252u16;
pub const HID_USAGE_TELEPHONY_ALERT_SOUND_ERROR: u16 = 251u16;
pub const HID_USAGE_TELEPHONY_ALERT_SOUND_NOTIFICATION: u16 = 253u16;
pub const HID_USAGE_TELEPHONY_ALTERNATE_FUNCTION: u16 = 41u16;
pub const HID_USAGE_TELEPHONY_ANSWERING_MACHINE: u16 = 2u16;
pub const HID_USAGE_TELEPHONY_ANSWER_ONOFF: u16 = 116u16;
pub const HID_USAGE_TELEPHONY_CALLER_ID: u16 = 48u16;
pub const HID_USAGE_TELEPHONY_CALL_DURATION: u16 = 330u16;
pub const HID_USAGE_TELEPHONY_CALL_WAITING_TONE: u16 = 153u16;
pub const HID_USAGE_TELEPHONY_CONFERENCE: u16 = 44u16;
pub const HID_USAGE_TELEPHONY_CONFIRMATION_TONE_1: u16 = 154u16;
pub const HID_USAGE_TELEPHONY_CONFIRMATION_TONE_2: u16 = 155u16;
pub const HID_USAGE_TELEPHONY_DO_NOT_DISTURB: u16 = 114u16;
pub const HID_USAGE_TELEPHONY_DROP: u16 = 38u16;
pub const HID_USAGE_TELEPHONY_DUAL_MODE_PHONE: u16 = 331u16;
pub const HID_USAGE_TELEPHONY_EMAIL_MESSAGE_WAITING: u16 = 264u16;
pub const HID_USAGE_TELEPHONY_FEATURE: u16 = 34u16;
pub const HID_USAGE_TELEPHONY_FLASH: u16 = 33u16;
pub const HID_USAGE_TELEPHONY_FORWARD_CALLS: u16 = 40u16;
pub const HID_USAGE_TELEPHONY_HANDSET: u16 = 4u16;
pub const HID_USAGE_TELEPHONY_HANDSET_NICKNAME: u16 = 326u16;
pub const HID_USAGE_TELEPHONY_HEADSET: u16 = 5u16;
pub const HID_USAGE_TELEPHONY_HOLD: u16 = 35u16;
pub const HID_USAGE_TELEPHONY_HOOK_SWITCH: u16 = 32u16;
pub const HID_USAGE_TELEPHONY_HOST_AVAILABLE: u16 = 241u16;
pub const HID_USAGE_TELEPHONY_HOST_CALL_ACTIVE: u16 = 242u16;
pub const HID_USAGE_TELEPHONY_HOST_CONTROL: u16 = 240u16;
pub const HID_USAGE_TELEPHONY_HOST_HOLD: u16 = 266u16;
pub const HID_USAGE_TELEPHONY_HOST_RING_TONE: u16 = 250u16;
pub const HID_USAGE_TELEPHONY_INCOMING_CALL_HISTORY: u16 = 274u16;
pub const HID_USAGE_TELEPHONY_INCOMING_CALL_HISTORY_COUNT: u16 = 272u16;
pub const HID_USAGE_TELEPHONY_INSIDE_DIAL_TONE: u16 = 144u16;
pub const HID_USAGE_TELEPHONY_INSIDE_RINGBACK: u16 = 149u16;
pub const HID_USAGE_TELEPHONY_INSIDE_RING_TONE: u16 = 146u16;
pub const HID_USAGE_TELEPHONY_KEYPAD: u16 = 6u16;
pub const HID_USAGE_TELEPHONY_LINE: u16 = 42u16;
pub const HID_USAGE_TELEPHONY_LINE_BUSY_TONE: u16 = 151u16;
pub const HID_USAGE_TELEPHONY_MESSAGE: u16 = 115u16;
pub const HID_USAGE_TELEPHONY_MESSAGE_CONTROLS: u16 = 3u16;
pub const HID_USAGE_TELEPHONY_OUTGOING_CALL_HISTORY: u16 = 275u16;
pub const HID_USAGE_TELEPHONY_OUTGOING_CALL_HISTORY_COUNT: u16 = 273u16;
pub const HID_USAGE_TELEPHONY_OUTSIDE_DIAL_TONE: u16 = 145u16;
pub const HID_USAGE_TELEPHONY_OUTSIDE_RINGBACK: u16 = 157u16;
pub const HID_USAGE_TELEPHONY_OUTSIDE_RING_TONE: u16 = 147u16;
pub const HID_USAGE_TELEPHONY_PARK: u16 = 39u16;
pub const HID_USAGE_TELEPHONY_PHONE: u16 = 1u16;
pub const HID_USAGE_TELEPHONY_PHONE_CALLER_ID_KEY: u16 = 193u16;
pub const HID_USAGE_TELEPHONY_PHONE_CALL_HISTORY_KEY: u16 = 192u16;
pub const HID_USAGE_TELEPHONY_PHONE_DATE_DAY: u16 = 323u16;
pub const HID_USAGE_TELEPHONY_PHONE_DATE_MONTH: u16 = 324u16;
pub const HID_USAGE_TELEPHONY_PHONE_DATE_YEAR: u16 = 325u16;
pub const HID_USAGE_TELEPHONY_PHONE_DIRECTORY: u16 = 83u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_0: u16 = 176u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_1: u16 = 177u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_2: u16 = 178u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_3: u16 = 179u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_4: u16 = 180u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_5: u16 = 181u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_6: u16 = 182u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_7: u16 = 183u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_8: u16 = 184u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_9: u16 = 185u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_A: u16 = 188u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_B: u16 = 189u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_C: u16 = 190u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_D: u16 = 191u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_POUND: u16 = 187u16;
pub const HID_USAGE_TELEPHONY_PHONE_KEY_STAR: u16 = 186u16;
pub const HID_USAGE_TELEPHONY_PHONE_LOCALE: u16 = 276u16;
pub const HID_USAGE_TELEPHONY_PHONE_MUTE: u16 = 47u16;
pub const HID_USAGE_TELEPHONY_PHONE_SETTINGS_KEY: u16 = 194u16;
pub const HID_USAGE_TELEPHONY_PHONE_TIME_HOUR: u16 = 322u16;
pub const HID_USAGE_TELEPHONY_PHONE_TIME_MINUTE: u16 = 321u16;
pub const HID_USAGE_TELEPHONY_PHONE_TIME_SECOND: u16 = 320u16;
pub const HID_USAGE_TELEPHONY_PRIORITY_RINGBACK: u16 = 150u16;
pub const HID_USAGE_TELEPHONY_PRIORITY_RING_TONE: u16 = 148u16;
pub const HID_USAGE_TELEPHONY_PROGRAMMABLE_BUTTON: u16 = 7u16;
pub const HID_USAGE_TELEPHONY_PSTN_RING_TONE: u16 = 249u16;
pub const HID_USAGE_TELEPHONY_RECALL_NUMBER: u16 = 82u16;
pub const HID_USAGE_TELEPHONY_REDIAL: u16 = 36u16;
pub const HID_USAGE_TELEPHONY_REDIALABLE_PHONE_NUMBER: u16 = 245u16;
pub const HID_USAGE_TELEPHONY_REORDER_TONE: u16 = 152u16;
pub const HID_USAGE_TELEPHONY_RINGER: u16 = 158u16;
pub const HID_USAGE_TELEPHONY_RING_ENABLE: u16 = 45u16;
pub const HID_USAGE_TELEPHONY_RING_SELECT: u16 = 46u16;
pub const HID_USAGE_TELEPHONY_RING_TYPE: u16 = 244u16;
pub const HID_USAGE_TELEPHONY_SCREEN_CALLS: u16 = 113u16;
pub const HID_USAGE_TELEPHONY_SEND: u16 = 49u16;
pub const HID_USAGE_TELEPHONY_SILENT_RING: u16 = 254u16;
pub const HID_USAGE_TELEPHONY_SPEAKER_PHONE: u16 = 43u16;
pub const HID_USAGE_TELEPHONY_SPEED_DIAL: u16 = 80u16;
pub const HID_USAGE_TELEPHONY_STOP_RING_TONE: u16 = 248u16;
pub const HID_USAGE_TELEPHONY_STORE_NUMBER: u16 = 81u16;
pub const HID_USAGE_TELEPHONY_TONES_OFF: u16 = 156u16;
pub const HID_USAGE_TELEPHONY_TRANSFER: u16 = 37u16;
pub const HID_USAGE_TELEPHONY_VOICEMAIL_MESSAGE_WAITING: u16 = 265u16;
pub const HID_USAGE_TELEPHONY_VOICE_MAIL: u16 = 112u16;
pub const HID_USAGE_UNDEFINED: u16 = 0u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_AUTO_SIZE_CENTER: u16 = 162u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_BLUE_VIDEO_BLACK_LEVEL: u16 = 112u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_BLUE_VIDEO_GAIN: u16 = 26u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_BOTTOM_CORNER_DISTORTION_BALANCE: u16 = 76u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_BOTTOM_CORNER_DISTORTION_CONTROL: u16 = 74u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_BRIGHTNESS: u16 = 16u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_CONTRAST: u16 = 18u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_DEGAUSS: u16 = 1u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_FOCUS: u16 = 28u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_GREEN_VIDEO_BLACK_LEVEL: u16 = 110u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_GREEN_VIDEO_GAIN: u16 = 24u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_FREQUENCY: u16 = 172u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_LINEARITY: u16 = 42u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_LINEARITY_BALANCE: u16 = 44u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_MISCONVERGENCE: u16 = 40u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_MOIRE: u16 = 86u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_PINCUSHION: u16 = 36u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_PINCUSHION_BALANCE: u16 = 38u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_POSITION: u16 = 32u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_HORIZONTAL_SIZE: u16 = 34u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_INPUT_LEVEL_SELECT: u16 = 94u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_INPUT_SOURCE_SELECT: u16 = 96u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_ON_SCREEN_DISPLAY: u16 = 202u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_PARALLELOGRAM_DISTORTION_KEY_BALANCE: u16 = 64u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_POLARITY_HORIZONTAL_SYNCHRONIZATION: u16 = 164u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_POLARITY_VERTICAL_SYNCHRONIZATION: u16 = 166u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_RED_VIDEO_BLACK_LEVEL: u16 = 108u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_RED_VIDEO_GAIN: u16 = 22u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_SCREEN_ORIENTATION: u16 = 170u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_SETTINGS: u16 = 176u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_STEREO_MODE: u16 = 212u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_SYNCHRONIZATION_TYPE: u16 = 168u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_TILT_ROTATION: u16 = 68u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_TOP_CORNER_DISTORTION_BALANCE: u16 = 72u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_TOP_CORNER_DISTORTION_CONTROL: u16 = 70u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_TRAPEZOIDAL_DISTORTION_KEY: u16 = 66u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_FREQUENCY: u16 = 174u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_LINEARITY: u16 = 58u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_LINEARITY_BALANCE: u16 = 60u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_MISCONVERGENCE: u16 = 56u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_MOIRE: u16 = 88u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_PINCUSHION: u16 = 52u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_PINCUSHION_BALANCE: u16 = 54u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_POSITION: u16 = 48u16;
pub const HID_USAGE_VESA_VIRTUAL_CONTROLS_VERTICAL_SIZE: u16 = 50u16;
pub const HID_USAGE_VR_ANIMATRONIC_DEVICE: u16 = 10u16;
pub const HID_USAGE_VR_BELT: u16 = 1u16;
pub const HID_USAGE_VR_BODY_SUIT: u16 = 2u16;
pub const HID_USAGE_VR_DISPLAY_ENABLE: u16 = 33u16;
pub const HID_USAGE_VR_FLEXOR: u16 = 3u16;
pub const HID_USAGE_VR_GLOVE: u16 = 4u16;
pub const HID_USAGE_VR_HAND_TRACKER: u16 = 7u16;
pub const HID_USAGE_VR_HEAD_MOUNTED_DISPLAY: u16 = 6u16;
pub const HID_USAGE_VR_HEAD_TRACKER: u16 = 5u16;
pub const HID_USAGE_VR_OCULOMETER: u16 = 8u16;
pub const HID_USAGE_VR_STEREO_ENABLE: u16 = 32u16;
pub const HID_USAGE_VR_VEST: u16 = 9u16;
pub const HID_USAGE_WEIGHING_DEVICE_CALIBRATION_COUNT: u16 = 96u16;
pub const HID_USAGE_WEIGHING_DEVICE_DATA_SCALING: u16 = 65u16;
pub const HID_USAGE_WEIGHING_DEVICE_DATA_WEIGHT: u16 = 64u16;
pub const HID_USAGE_WEIGHING_DEVICE_ENFORCED_ZERO_RETURN: u16 = 129u16;
pub const HID_USAGE_WEIGHING_DEVICE_REZERO_COUNT: u16 = 97u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALES: u16 = 1u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_ATTRIBUTE_REPORT: u16 = 48u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS: u16 = 33u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_GENERIC: u16 = 42u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_IIIL_ENGLISH: u16 = 40u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_IIIL_METRIC: u16 = 37u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_III_ENGLISH: u16 = 39u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_III_METRIC: u16 = 36u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_II_METRIC: u16 = 35u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_IV_ENGLISH: u16 = 41u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_IV_METRIC: u16 = 38u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CLASS_I_METRIC: u16 = 34u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_CONTROL_REPORT: u16 = 49u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_DATA_REPORT: u16 = 50u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_DEVICE: u16 = 32u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATISTICS_REPORT: u16 = 53u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS: u16 = 112u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_FAULT: u16 = 113u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_IN_MOTION: u16 = 115u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_OVER_WEIGHT_LIMIT: u16 = 118u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_REPORT: u16 = 51u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_REQUIRES_CALIBRATION: u16 = 119u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_REQUIRES_REZEROING: u16 = 120u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_STABLE_AT_CENTER_OF_ZERO: u16 = 114u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_UNDER_ZERO: u16 = 117u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_STATUS_WEIGHT_STABLE: u16 = 116u16;
pub const HID_USAGE_WEIGHING_DEVICE_SCALE_WEIGHT_LIMIT_REPORT: u16 = 52u16;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT: u16 = 80u16;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_AVOIR_TON: u16 = 89u16;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_CARATS: u16 = 84u16;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_GRAINS: u16 = 86u16;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_GRAM: u16 = 82u16;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_KILOGRAM: u16 = 83u16;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_METRIC_TON: u16 = 88u16;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_MILLIGRAM: u16 = 81u16;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_OUNCE: u16 = 91u16;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_PENNYWEIGHTS: u16 = 87u16;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_POUND: u16 = 92u16;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_TAELS: u16 = 85u16;
pub const HID_USAGE_WEIGHING_DEVICE_WEIGHT_UNIT_TROY_OUNCE: u16 = 90u16;
pub const HID_USAGE_WEIGHING_DEVICE_ZERO_SCALE: u16 = 128u16;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HID_XFER_PACKET {
    pub reportBuffer: *mut u8,
    pub reportBufferLen: u32,
    pub reportId: u8,
}
impl Default for HID_XFER_PACKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HORIZONTAL_WHEEL_PRESENT: u32 = 32768u32;
pub const HidP_Feature: HIDP_REPORT_TYPE = HIDP_REPORT_TYPE(2i32);
pub const HidP_Input: HIDP_REPORT_TYPE = HIDP_REPORT_TYPE(0i32);
pub const HidP_Keyboard_Break: HIDP_KEYBOARD_DIRECTION = HIDP_KEYBOARD_DIRECTION(0i32);
pub const HidP_Keyboard_Make: HIDP_KEYBOARD_DIRECTION = HIDP_KEYBOARD_DIRECTION(1i32);
pub const HidP_Output: HIDP_REPORT_TYPE = HIDP_REPORT_TYPE(1i32);
windows_core::imp::define_interface!(IDirectInput2A, IDirectInput2A_Vtbl, 0x5944e662_aa8a_11cf_bfc7_444553540000);
impl core::ops::Deref for IDirectInput2A {
    type Target = IDirectInputA;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectInput2A, windows_core::IUnknown, IDirectInputA);
impl IDirectInput2A {
    pub unsafe fn FindDevice<P1>(&self, param0: *const windows_core::GUID, param1: P1, param2: *mut windows_core::GUID) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindDevice)(windows_core::Interface::as_raw(self), param0, param1.param().abi(), param2 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput2A_Vtbl {
    pub base__: IDirectInputA_Vtbl,
    pub FindDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCSTR, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IDirectInput2A_Impl: IDirectInputA_Impl {
    fn FindDevice(&self, param0: *const windows_core::GUID, param1: &windows_core::PCSTR, param2: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl IDirectInput2A_Vtbl {
    pub const fn new<Identity: IDirectInput2A_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindDevice<Identity: IDirectInput2A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: windows_core::PCSTR, param2: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput2A_Impl::FindDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        Self { base__: IDirectInputA_Vtbl::new::<Identity, OFFSET>(), FindDevice: FindDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput2A as windows_core::Interface>::IID || iid == &<IDirectInputA as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInput2A {}
windows_core::imp::define_interface!(IDirectInput2W, IDirectInput2W_Vtbl, 0x5944e663_aa8a_11cf_bfc7_444553540000);
impl core::ops::Deref for IDirectInput2W {
    type Target = IDirectInputW;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectInput2W, windows_core::IUnknown, IDirectInputW);
impl IDirectInput2W {
    pub unsafe fn FindDevice<P1>(&self, param0: *const windows_core::GUID, param1: P1, param2: *mut windows_core::GUID) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindDevice)(windows_core::Interface::as_raw(self), param0, param1.param().abi(), param2 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput2W_Vtbl {
    pub base__: IDirectInputW_Vtbl,
    pub FindDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IDirectInput2W_Impl: IDirectInputW_Impl {
    fn FindDevice(&self, param0: *const windows_core::GUID, param1: &windows_core::PCWSTR, param2: *mut windows_core::GUID) -> windows_core::Result<()>;
}
impl IDirectInput2W_Vtbl {
    pub const fn new<Identity: IDirectInput2W_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindDevice<Identity: IDirectInput2W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: windows_core::PCWSTR, param2: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput2W_Impl::FindDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        Self { base__: IDirectInputW_Vtbl::new::<Identity, OFFSET>(), FindDevice: FindDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput2W as windows_core::Interface>::IID || iid == &<IDirectInputW as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInput2W {}
windows_core::imp::define_interface!(IDirectInput7A, IDirectInput7A_Vtbl, 0x9a4cb684_236d_11d3_8e9d_00c04f6844ae);
impl core::ops::Deref for IDirectInput7A {
    type Target = IDirectInput2A;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectInput7A, windows_core::IUnknown, IDirectInputA, IDirectInput2A);
impl IDirectInput7A {
    pub unsafe fn CreateDeviceEx<P3>(&self, param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void, param3: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDeviceEx)(windows_core::Interface::as_raw(self), param0, param1, param2 as _, param3.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput7A_Vtbl {
    pub base__: IDirectInput2A_Vtbl,
    pub CreateDeviceEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectInput7A_Impl: IDirectInput2A_Impl {
    fn CreateDeviceEx(&self, param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void, param3: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IDirectInput7A_Vtbl {
    pub const fn new<Identity: IDirectInput7A_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDeviceEx<Identity: IDirectInput7A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput7A_Impl::CreateDeviceEx(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        Self { base__: IDirectInput2A_Vtbl::new::<Identity, OFFSET>(), CreateDeviceEx: CreateDeviceEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput7A as windows_core::Interface>::IID || iid == &<IDirectInputA as windows_core::Interface>::IID || iid == &<IDirectInput2A as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInput7A {}
windows_core::imp::define_interface!(IDirectInput7W, IDirectInput7W_Vtbl, 0x9a4cb685_236d_11d3_8e9d_00c04f6844ae);
impl core::ops::Deref for IDirectInput7W {
    type Target = IDirectInput2W;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectInput7W, windows_core::IUnknown, IDirectInputW, IDirectInput2W);
impl IDirectInput7W {
    pub unsafe fn CreateDeviceEx<P3>(&self, param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void, param3: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDeviceEx)(windows_core::Interface::as_raw(self), param0, param1, param2 as _, param3.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput7W_Vtbl {
    pub base__: IDirectInput2W_Vtbl,
    pub CreateDeviceEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectInput7W_Impl: IDirectInput2W_Impl {
    fn CreateDeviceEx(&self, param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void, param3: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IDirectInput7W_Vtbl {
    pub const fn new<Identity: IDirectInput7W_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDeviceEx<Identity: IDirectInput7W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput7W_Impl::CreateDeviceEx(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        Self { base__: IDirectInput2W_Vtbl::new::<Identity, OFFSET>(), CreateDeviceEx: CreateDeviceEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput7W as windows_core::Interface>::IID || iid == &<IDirectInputW as windows_core::Interface>::IID || iid == &<IDirectInput2W as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInput7W {}
windows_core::imp::define_interface!(IDirectInput8A, IDirectInput8A_Vtbl, 0xbf798030_483a_4da2_aa99_5d64ed369700);
windows_core::imp::interface_hierarchy!(IDirectInput8A, windows_core::IUnknown);
impl IDirectInput8A {
    pub unsafe fn CreateDevice<P2>(&self, param0: *const windows_core::GUID, param1: *mut Option<IDirectInputDevice8A>, param2: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), param0, core::mem::transmute(param1), param2.param().abi()).ok() }
    }
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumDevices)(windows_core::Interface::as_raw(self), param0, param1, param2 as _, param3).ok() }
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceStatus)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn FindDevice<P1>(&self, param0: *const windows_core::GUID, param1: P1, param2: *mut windows_core::GUID) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindDevice)(windows_core::Interface::as_raw(self), param0, param1.param().abi(), param2 as _).ok() }
    }
    pub unsafe fn EnumDevicesBySemantics<P0>(&self, param0: P0, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumDevicesBySemantics)(windows_core::Interface::as_raw(self), param0.param().abi(), param1 as _, param2, param3 as _, param4).ok() }
    }
    pub unsafe fn ConfigureDevices(&self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConfigureDevices)(windows_core::Interface::as_raw(self), param0, core::mem::transmute(param1), param2, param3 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput8A_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, LPDIENUMDEVICESCALLBACKA, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32) -> windows_core::HRESULT,
    pub FindDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCSTR, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub EnumDevicesBySemantics: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut DIACTIONFORMATA, LPDIENUMDEVICESBYSEMANTICSCBA, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ConfigureDevices: unsafe extern "system" fn(*mut core::ffi::c_void, LPDICONFIGUREDEVICESCALLBACK, *mut DICONFIGUREDEVICESPARAMSA, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectInput8A_Impl: windows_core::IUnknownImpl {
    fn CreateDevice(&self, param0: *const windows_core::GUID, param1: windows_core::OutRef<IDirectInputDevice8A>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()>;
    fn FindDevice(&self, param0: *const windows_core::GUID, param1: &windows_core::PCSTR, param2: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn EnumDevicesBySemantics(&self, param0: &windows_core::PCSTR, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::Result<()>;
    fn ConfigureDevices(&self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDirectInput8A_Vtbl {
    pub const fn new<Identity: IDirectInput8A_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDevice<Identity: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8A_Impl::CreateDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn EnumDevices<Identity: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8A_Impl::EnumDevices(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8A_Impl::GetDeviceStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn RunControlPanel<Identity: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8A_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8A_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn FindDevice<Identity: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: windows_core::PCSTR, param2: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8A_Impl::FindDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Identity: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8A_Impl::EnumDevicesBySemantics(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn ConfigureDevices<Identity: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8A_Impl::ConfigureDevices(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            EnumDevices: EnumDevices::<Identity, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            FindDevice: FindDevice::<Identity, OFFSET>,
            EnumDevicesBySemantics: EnumDevicesBySemantics::<Identity, OFFSET>,
            ConfigureDevices: ConfigureDevices::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput8A as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInput8A {}
windows_core::imp::define_interface!(IDirectInput8W, IDirectInput8W_Vtbl, 0xbf798031_483a_4da2_aa99_5d64ed369700);
windows_core::imp::interface_hierarchy!(IDirectInput8W, windows_core::IUnknown);
impl IDirectInput8W {
    pub unsafe fn CreateDevice<P2>(&self, param0: *const windows_core::GUID, param1: *mut Option<IDirectInputDevice8W>, param2: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), param0, core::mem::transmute(param1), param2.param().abi()).ok() }
    }
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumDevices)(windows_core::Interface::as_raw(self), param0, param1, param2 as _, param3).ok() }
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceStatus)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn FindDevice<P1>(&self, param0: *const windows_core::GUID, param1: P1, param2: *mut windows_core::GUID) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindDevice)(windows_core::Interface::as_raw(self), param0, param1.param().abi(), param2 as _).ok() }
    }
    pub unsafe fn EnumDevicesBySemantics<P0>(&self, param0: P0, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumDevicesBySemantics)(windows_core::Interface::as_raw(self), param0.param().abi(), param1 as _, param2, param3 as _, param4).ok() }
    }
    pub unsafe fn ConfigureDevices(&self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConfigureDevices)(windows_core::Interface::as_raw(self), param0, core::mem::transmute(param1), param2, param3 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput8W_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, LPDIENUMDEVICESCALLBACKW, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32) -> windows_core::HRESULT,
    pub FindDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub EnumDevicesBySemantics: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut DIACTIONFORMATW, LPDIENUMDEVICESBYSEMANTICSCBW, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ConfigureDevices: unsafe extern "system" fn(*mut core::ffi::c_void, LPDICONFIGUREDEVICESCALLBACK, *mut DICONFIGUREDEVICESPARAMSW, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectInput8W_Impl: windows_core::IUnknownImpl {
    fn CreateDevice(&self, param0: *const windows_core::GUID, param1: windows_core::OutRef<IDirectInputDevice8W>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()>;
    fn FindDevice(&self, param0: *const windows_core::GUID, param1: &windows_core::PCWSTR, param2: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn EnumDevicesBySemantics(&self, param0: &windows_core::PCWSTR, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::Result<()>;
    fn ConfigureDevices(&self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDirectInput8W_Vtbl {
    pub const fn new<Identity: IDirectInput8W_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDevice<Identity: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8W_Impl::CreateDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn EnumDevices<Identity: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8W_Impl::EnumDevices(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8W_Impl::GetDeviceStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn RunControlPanel<Identity: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8W_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8W_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn FindDevice<Identity: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: windows_core::PCWSTR, param2: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8W_Impl::FindDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Identity: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8W_Impl::EnumDevicesBySemantics(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn ConfigureDevices<Identity: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8W_Impl::ConfigureDevices(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            EnumDevices: EnumDevices::<Identity, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            FindDevice: FindDevice::<Identity, OFFSET>,
            EnumDevicesBySemantics: EnumDevicesBySemantics::<Identity, OFFSET>,
            ConfigureDevices: ConfigureDevices::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput8W as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInput8W {}
windows_core::imp::define_interface!(IDirectInputA, IDirectInputA_Vtbl, 0x89521360_aa8a_11cf_bfc7_444553540000);
windows_core::imp::interface_hierarchy!(IDirectInputA, windows_core::IUnknown);
impl IDirectInputA {
    pub unsafe fn CreateDevice<P2>(&self, param0: *const windows_core::GUID, param1: *mut Option<IDirectInputDeviceA>, param2: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), param0, core::mem::transmute(param1), param2.param().abi()).ok() }
    }
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumDevices)(windows_core::Interface::as_raw(self), param0, param1, param2 as _, param3).ok() }
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceStatus)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, LPDIENUMDEVICESCALLBACKA, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32) -> windows_core::HRESULT,
}
pub trait IDirectInputA_Impl: windows_core::IUnknownImpl {
    fn CreateDevice(&self, param0: *const windows_core::GUID, param1: windows_core::OutRef<IDirectInputDeviceA>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()>;
}
impl IDirectInputA_Vtbl {
    pub const fn new<Identity: IDirectInputA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDevice<Identity: IDirectInputA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputA_Impl::CreateDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn EnumDevices<Identity: IDirectInputA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputA_Impl::EnumDevices(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: IDirectInputA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputA_Impl::GetDeviceStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn RunControlPanel<Identity: IDirectInputA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputA_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectInputA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputA_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            EnumDevices: EnumDevices::<Identity, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputA as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInputA {}
windows_core::imp::define_interface!(IDirectInputDevice2A, IDirectInputDevice2A_Vtbl, 0x5944e682_c92e_11cf_bfc7_444553540000);
impl core::ops::Deref for IDirectInputDevice2A {
    type Target = IDirectInputDeviceA;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectInputDevice2A, windows_core::IUnknown, IDirectInputDeviceA);
impl IDirectInputDevice2A {
    pub unsafe fn CreateEffect<P3>(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut Option<IDirectInputEffect>, param3: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateEffect)(windows_core::Interface::as_raw(self), param0, param1 as _, core::mem::transmute(param2), param3.param().abi()).ok() }
    }
    pub unsafe fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumEffects)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOA, param1: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEffectInfo)(windows_core::Interface::as_raw(self), param0 as _, param1).ok() }
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetForceFeedbackState)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendForceFeedbackCommand)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumCreatedEffectObjects)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn Poll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Poll)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendDeviceData)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDevice2A_Vtbl {
    pub base__: IDirectInputDeviceA_Vtbl,
    pub CreateEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DIEFFECT, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumEffects: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIENUMEFFECTSCALLBACKA, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetEffectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIEFFECTINFOA, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetForceFeedbackState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SendForceFeedbackCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumCreatedEffectObjects: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIEFFESCAPE) -> windows_core::HRESULT,
    pub Poll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendDeviceData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DIDEVICEOBJECTDATA, *mut u32, u32) -> windows_core::HRESULT,
}
pub trait IDirectInputDevice2A_Impl: IDirectInputDeviceA_Impl {
    fn CreateEffect(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: windows_core::OutRef<IDirectInputEffect>, param3: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOA, param1: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetForceFeedbackState(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32) -> windows_core::Result<()>;
    fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()>;
    fn Poll(&self) -> windows_core::Result<()>;
    fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
}
impl IDirectInputDevice2A_Vtbl {
    pub const fn new<Identity: IDirectInputDevice2A_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateEffect<Identity: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2A_Impl::CreateEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn EnumEffects<Identity: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2A_Impl::EnumEffects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetEffectInfo<Identity: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2A_Impl::GetEffectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2A_Impl::GetForceFeedbackState(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2A_Impl::SendForceFeedbackCommand(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2A_Impl::EnumCreatedEffectObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn Escape<Identity: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFESCAPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2A_Impl::Escape(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Poll<Identity: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2A_Impl::Poll(this).into()
            }
        }
        unsafe extern "system" fn SendDeviceData<Identity: IDirectInputDevice2A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2A_Impl::SendDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        Self {
            base__: IDirectInputDeviceA_Vtbl::new::<Identity, OFFSET>(),
            CreateEffect: CreateEffect::<Identity, OFFSET>,
            EnumEffects: EnumEffects::<Identity, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            Poll: Poll::<Identity, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice2A as windows_core::Interface>::IID || iid == &<IDirectInputDeviceA as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInputDevice2A {}
windows_core::imp::define_interface!(IDirectInputDevice2W, IDirectInputDevice2W_Vtbl, 0x5944e683_c92e_11cf_bfc7_444553540000);
impl core::ops::Deref for IDirectInputDevice2W {
    type Target = IDirectInputDeviceW;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectInputDevice2W, windows_core::IUnknown, IDirectInputDeviceW);
impl IDirectInputDevice2W {
    pub unsafe fn CreateEffect<P3>(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut Option<IDirectInputEffect>, param3: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateEffect)(windows_core::Interface::as_raw(self), param0, param1 as _, core::mem::transmute(param2), param3.param().abi()).ok() }
    }
    pub unsafe fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumEffects)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOW, param1: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEffectInfo)(windows_core::Interface::as_raw(self), param0 as _, param1).ok() }
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetForceFeedbackState)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendForceFeedbackCommand)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumCreatedEffectObjects)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn Poll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Poll)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendDeviceData)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDevice2W_Vtbl {
    pub base__: IDirectInputDeviceW_Vtbl,
    pub CreateEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DIEFFECT, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumEffects: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIENUMEFFECTSCALLBACKW, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetEffectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIEFFECTINFOW, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetForceFeedbackState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SendForceFeedbackCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumCreatedEffectObjects: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIEFFESCAPE) -> windows_core::HRESULT,
    pub Poll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendDeviceData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DIDEVICEOBJECTDATA, *mut u32, u32) -> windows_core::HRESULT,
}
pub trait IDirectInputDevice2W_Impl: IDirectInputDeviceW_Impl {
    fn CreateEffect(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: windows_core::OutRef<IDirectInputEffect>, param3: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOW, param1: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetForceFeedbackState(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32) -> windows_core::Result<()>;
    fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()>;
    fn Poll(&self) -> windows_core::Result<()>;
    fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
}
impl IDirectInputDevice2W_Vtbl {
    pub const fn new<Identity: IDirectInputDevice2W_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateEffect<Identity: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2W_Impl::CreateEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn EnumEffects<Identity: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2W_Impl::EnumEffects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetEffectInfo<Identity: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2W_Impl::GetEffectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2W_Impl::GetForceFeedbackState(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2W_Impl::SendForceFeedbackCommand(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2W_Impl::EnumCreatedEffectObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn Escape<Identity: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFESCAPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2W_Impl::Escape(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Poll<Identity: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2W_Impl::Poll(this).into()
            }
        }
        unsafe extern "system" fn SendDeviceData<Identity: IDirectInputDevice2W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice2W_Impl::SendDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        Self {
            base__: IDirectInputDeviceW_Vtbl::new::<Identity, OFFSET>(),
            CreateEffect: CreateEffect::<Identity, OFFSET>,
            EnumEffects: EnumEffects::<Identity, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            Poll: Poll::<Identity, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice2W as windows_core::Interface>::IID || iid == &<IDirectInputDeviceW as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInputDevice2W {}
windows_core::imp::define_interface!(IDirectInputDevice7A, IDirectInputDevice7A_Vtbl, 0x57d7c6bc_2356_11d3_8e9d_00c04f6844ae);
impl core::ops::Deref for IDirectInputDevice7A {
    type Target = IDirectInputDevice2A;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectInputDevice7A, windows_core::IUnknown, IDirectInputDeviceA, IDirectInputDevice2A);
impl IDirectInputDevice7A {
    pub unsafe fn EnumEffectsInFile<P0>(&self, param0: P0, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumEffectsInFile)(windows_core::Interface::as_raw(self), param0.param().abi(), param1, param2 as _, param3).ok() }
    }
    pub unsafe fn WriteEffectToFile<P0>(&self, param0: P0, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteEffectToFile)(windows_core::Interface::as_raw(self), param0.param().abi(), param1, param2 as _, param3).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDevice7A_Vtbl {
    pub base__: IDirectInputDevice2A_Vtbl,
    pub EnumEffectsInFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, LPDIENUMEFFECTSINFILECALLBACK, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub WriteEffectToFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, *mut DIFILEEFFECT, u32) -> windows_core::HRESULT,
}
pub trait IDirectInputDevice7A_Impl: IDirectInputDevice2A_Impl {
    fn EnumEffectsInFile(&self, param0: &windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn WriteEffectToFile(&self, param0: &windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>;
}
impl IDirectInputDevice7A_Vtbl {
    pub const fn new<Identity: IDirectInputDevice7A_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumEffectsInFile<Identity: IDirectInputDevice7A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice7A_Impl::EnumEffectsInFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: IDirectInputDevice7A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice7A_Impl::WriteEffectToFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        Self {
            base__: IDirectInputDevice2A_Vtbl::new::<Identity, OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Identity, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice7A as windows_core::Interface>::IID || iid == &<IDirectInputDeviceA as windows_core::Interface>::IID || iid == &<IDirectInputDevice2A as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInputDevice7A {}
windows_core::imp::define_interface!(IDirectInputDevice7W, IDirectInputDevice7W_Vtbl, 0x57d7c6bd_2356_11d3_8e9d_00c04f6844ae);
impl core::ops::Deref for IDirectInputDevice7W {
    type Target = IDirectInputDevice2W;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectInputDevice7W, windows_core::IUnknown, IDirectInputDeviceW, IDirectInputDevice2W);
impl IDirectInputDevice7W {
    pub unsafe fn EnumEffectsInFile<P0>(&self, param0: P0, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumEffectsInFile)(windows_core::Interface::as_raw(self), param0.param().abi(), param1, param2 as _, param3).ok() }
    }
    pub unsafe fn WriteEffectToFile<P0>(&self, param0: P0, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteEffectToFile)(windows_core::Interface::as_raw(self), param0.param().abi(), param1, param2 as _, param3).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDevice7W_Vtbl {
    pub base__: IDirectInputDevice2W_Vtbl,
    pub EnumEffectsInFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, LPDIENUMEFFECTSINFILECALLBACK, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub WriteEffectToFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut DIFILEEFFECT, u32) -> windows_core::HRESULT,
}
pub trait IDirectInputDevice7W_Impl: IDirectInputDevice2W_Impl {
    fn EnumEffectsInFile(&self, param0: &windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn WriteEffectToFile(&self, param0: &windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>;
}
impl IDirectInputDevice7W_Vtbl {
    pub const fn new<Identity: IDirectInputDevice7W_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumEffectsInFile<Identity: IDirectInputDevice7W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice7W_Impl::EnumEffectsInFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: IDirectInputDevice7W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice7W_Impl::WriteEffectToFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        Self {
            base__: IDirectInputDevice2W_Vtbl::new::<Identity, OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Identity, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice7W as windows_core::Interface>::IID || iid == &<IDirectInputDeviceW as windows_core::Interface>::IID || iid == &<IDirectInputDevice2W as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInputDevice7W {}
windows_core::imp::define_interface!(IDirectInputDevice8A, IDirectInputDevice8A_Vtbl, 0x54d41080_dc15_4833_a41b_748f73a38179);
windows_core::imp::interface_hierarchy!(IDirectInputDevice8A, windows_core::IUnknown);
impl IDirectInputDevice8A {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumObjects)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn GetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn SetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn Acquire(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Acquire)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Unacquire(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unacquire)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceState)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceData)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3).ok() }
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDataFormat)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventNotification)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectInfo)(windows_core::Interface::as_raw(self), param0 as _, param1, param2).ok() }
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceInfo)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0, param1, param2).ok() }
    }
    pub unsafe fn CreateEffect<P3>(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut Option<IDirectInputEffect>, param3: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateEffect)(windows_core::Interface::as_raw(self), param0, param1 as _, core::mem::transmute(param2), param3.param().abi()).ok() }
    }
    pub unsafe fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumEffects)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOA, param1: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEffectInfo)(windows_core::Interface::as_raw(self), param0 as _, param1).ok() }
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetForceFeedbackState)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendForceFeedbackCommand)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumCreatedEffectObjects)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn Poll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Poll)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendDeviceData)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3).ok() }
    }
    pub unsafe fn EnumEffectsInFile<P0>(&self, param0: P0, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumEffectsInFile)(windows_core::Interface::as_raw(self), param0.param().abi(), param1, param2 as _, param3).ok() }
    }
    pub unsafe fn WriteEffectToFile<P0>(&self, param0: P0, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteEffectToFile)(windows_core::Interface::as_raw(self), param0.param().abi(), param1, param2 as _, param3).ok() }
    }
    pub unsafe fn BuildActionMap<P1>(&self, param0: *mut DIACTIONFORMATA, param1: P1, param2: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BuildActionMap)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2).ok() }
    }
    pub unsafe fn SetActionMap<P1>(&self, param0: *mut DIACTIONFORMATA, param1: P1, param2: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActionMap)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2).ok() }
    }
    pub unsafe fn GetImageInfo(&self, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetImageInfo)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDevice8A_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVCAPS) -> windows_core::HRESULT,
    pub EnumObjects: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIENUMDEVICEOBJECTSCALLBACKA, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DIPROPHEADER) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DIPROPHEADER) -> windows_core::HRESULT,
    pub Acquire: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unacquire: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DIDEVICEOBJECTDATA, *mut u32, u32) -> windows_core::HRESULT,
    pub SetDataFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDATAFORMAT) -> windows_core::HRESULT,
    pub SetEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub GetObjectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEOBJECTINSTANCEA, u32, u32) -> windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEINSTANCEA) -> windows_core::HRESULT,
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub CreateEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DIEFFECT, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumEffects: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIENUMEFFECTSCALLBACKA, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetEffectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIEFFECTINFOA, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetForceFeedbackState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SendForceFeedbackCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumCreatedEffectObjects: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIEFFESCAPE) -> windows_core::HRESULT,
    pub Poll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendDeviceData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DIDEVICEOBJECTDATA, *mut u32, u32) -> windows_core::HRESULT,
    pub EnumEffectsInFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, LPDIENUMEFFECTSINFILECALLBACK, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub WriteEffectToFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, u32, *mut DIFILEEFFECT, u32) -> windows_core::HRESULT,
    pub BuildActionMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIACTIONFORMATA, windows_core::PCSTR, u32) -> windows_core::HRESULT,
    pub SetActionMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIACTIONFORMATA, windows_core::PCSTR, u32) -> windows_core::HRESULT,
    pub GetImageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEIMAGEINFOHEADERA) -> windows_core::HRESULT,
}
pub trait IDirectInputDevice8A_Impl: windows_core::IUnknownImpl {
    fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> windows_core::Result<()>;
    fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn SetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn Acquire(&self) -> windows_core::Result<()>;
    fn Unacquire(&self) -> windows_core::Result<()>;
    fn GetDeviceState(&self, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
    fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> windows_core::Result<()>;
    fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()>;
    fn CreateEffect(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: windows_core::OutRef<IDirectInputEffect>, param3: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOA, param1: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetForceFeedbackState(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32) -> windows_core::Result<()>;
    fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()>;
    fn Poll(&self) -> windows_core::Result<()>;
    fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
    fn EnumEffectsInFile(&self, param0: &windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn WriteEffectToFile(&self, param0: &windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>;
    fn BuildActionMap(&self, param0: *mut DIACTIONFORMATA, param1: &windows_core::PCSTR, param2: u32) -> windows_core::Result<()>;
    fn SetActionMap(&self, param0: *mut DIACTIONFORMATA, param1: &windows_core::PCSTR, param2: u32) -> windows_core::Result<()>;
    fn GetImageInfo(&self, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> windows_core::Result<()>;
}
impl IDirectInputDevice8A_Vtbl {
    pub const fn new<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCapabilities<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::GetCapabilities(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn EnumObjects<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::EnumObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::GetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::SetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Acquire<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::Acquire(this).into()
            }
        }
        unsafe extern "system" fn Unacquire<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::Unacquire(this).into()
            }
        }
        unsafe extern "system" fn GetDeviceState<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::GetDeviceState(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetDeviceData<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::GetDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn SetDataFormat<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDATAFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::SetDataFormat(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetEventNotification<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::SetEventNotification(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetObjectInfo<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::GetObjectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::GetDeviceInfo(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn RunControlPanel<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn CreateEffect<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::CreateEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn EnumEffects<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::EnumEffects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetEffectInfo<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFECTINFOA, param1: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::GetEffectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::GetForceFeedbackState(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::SendForceFeedbackCommand(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::EnumCreatedEffectObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn Escape<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFESCAPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::Escape(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Poll<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::Poll(this).into()
            }
        }
        unsafe extern "system" fn SendDeviceData<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::SendDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn EnumEffectsInFile<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::EnumEffectsInFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::WriteEffectToFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn BuildActionMap<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: windows_core::PCSTR, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::BuildActionMap(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn SetActionMap<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: windows_core::PCSTR, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::SetActionMap(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetImageInfo<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::GetImageInfo(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            EnumObjects: EnumObjects::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Acquire: Acquire::<Identity, OFFSET>,
            Unacquire: Unacquire::<Identity, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            CreateEffect: CreateEffect::<Identity, OFFSET>,
            EnumEffects: EnumEffects::<Identity, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            Poll: Poll::<Identity, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, OFFSET>,
            EnumEffectsInFile: EnumEffectsInFile::<Identity, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, OFFSET>,
            BuildActionMap: BuildActionMap::<Identity, OFFSET>,
            SetActionMap: SetActionMap::<Identity, OFFSET>,
            GetImageInfo: GetImageInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice8A as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInputDevice8A {}
windows_core::imp::define_interface!(IDirectInputDevice8W, IDirectInputDevice8W_Vtbl, 0x54d41081_dc15_4833_a41b_748f73a38179);
windows_core::imp::interface_hierarchy!(IDirectInputDevice8W, windows_core::IUnknown);
impl IDirectInputDevice8W {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumObjects)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn GetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn SetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn Acquire(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Acquire)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Unacquire(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unacquire)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceState)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceData)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3).ok() }
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDataFormat)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventNotification)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectInfo)(windows_core::Interface::as_raw(self), param0 as _, param1, param2).ok() }
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceInfo)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0, param1, param2).ok() }
    }
    pub unsafe fn CreateEffect<P3>(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut Option<IDirectInputEffect>, param3: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateEffect)(windows_core::Interface::as_raw(self), param0, param1 as _, core::mem::transmute(param2), param3.param().abi()).ok() }
    }
    pub unsafe fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumEffects)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOW, param1: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEffectInfo)(windows_core::Interface::as_raw(self), param0 as _, param1).ok() }
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetForceFeedbackState)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendForceFeedbackCommand)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumCreatedEffectObjects)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn Poll(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Poll)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendDeviceData)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3).ok() }
    }
    pub unsafe fn EnumEffectsInFile<P0>(&self, param0: P0, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumEffectsInFile)(windows_core::Interface::as_raw(self), param0.param().abi(), param1, param2 as _, param3).ok() }
    }
    pub unsafe fn WriteEffectToFile<P0>(&self, param0: P0, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteEffectToFile)(windows_core::Interface::as_raw(self), param0.param().abi(), param1, param2 as _, param3).ok() }
    }
    pub unsafe fn BuildActionMap<P1>(&self, param0: *mut DIACTIONFORMATW, param1: P1, param2: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BuildActionMap)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2).ok() }
    }
    pub unsafe fn SetActionMap<P1>(&self, param0: *mut DIACTIONFORMATW, param1: P1, param2: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActionMap)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2).ok() }
    }
    pub unsafe fn GetImageInfo(&self, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetImageInfo)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDevice8W_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVCAPS) -> windows_core::HRESULT,
    pub EnumObjects: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIENUMDEVICEOBJECTSCALLBACKW, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DIPROPHEADER) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DIPROPHEADER) -> windows_core::HRESULT,
    pub Acquire: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unacquire: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DIDEVICEOBJECTDATA, *mut u32, u32) -> windows_core::HRESULT,
    pub SetDataFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDATAFORMAT) -> windows_core::HRESULT,
    pub SetEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub GetObjectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEOBJECTINSTANCEW, u32, u32) -> windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEINSTANCEW) -> windows_core::HRESULT,
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub CreateEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DIEFFECT, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumEffects: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIENUMEFFECTSCALLBACKW, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetEffectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIEFFECTINFOW, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetForceFeedbackState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SendForceFeedbackCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumCreatedEffectObjects: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIEFFESCAPE) -> windows_core::HRESULT,
    pub Poll: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendDeviceData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DIDEVICEOBJECTDATA, *mut u32, u32) -> windows_core::HRESULT,
    pub EnumEffectsInFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, LPDIENUMEFFECTSINFILECALLBACK, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub WriteEffectToFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut DIFILEEFFECT, u32) -> windows_core::HRESULT,
    pub BuildActionMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIACTIONFORMATW, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetActionMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIACTIONFORMATW, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetImageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEIMAGEINFOHEADERW) -> windows_core::HRESULT,
}
pub trait IDirectInputDevice8W_Impl: windows_core::IUnknownImpl {
    fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> windows_core::Result<()>;
    fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn SetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn Acquire(&self) -> windows_core::Result<()>;
    fn Unacquire(&self) -> windows_core::Result<()>;
    fn GetDeviceState(&self, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
    fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> windows_core::Result<()>;
    fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()>;
    fn CreateEffect(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: windows_core::OutRef<IDirectInputEffect>, param3: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumEffects(&self, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetEffectInfo(&self, param0: *mut DIEFFECTINFOW, param1: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetForceFeedbackState(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32) -> windows_core::Result<()>;
    fn EnumCreatedEffectObjects(&self, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()>;
    fn Poll(&self) -> windows_core::Result<()>;
    fn SendDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
    fn EnumEffectsInFile(&self, param0: &windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn WriteEffectToFile(&self, param0: &windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>;
    fn BuildActionMap(&self, param0: *mut DIACTIONFORMATW, param1: &windows_core::PCWSTR, param2: u32) -> windows_core::Result<()>;
    fn SetActionMap(&self, param0: *mut DIACTIONFORMATW, param1: &windows_core::PCWSTR, param2: u32) -> windows_core::Result<()>;
    fn GetImageInfo(&self, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> windows_core::Result<()>;
}
impl IDirectInputDevice8W_Vtbl {
    pub const fn new<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCapabilities<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::GetCapabilities(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn EnumObjects<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::EnumObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::GetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::SetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Acquire<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::Acquire(this).into()
            }
        }
        unsafe extern "system" fn Unacquire<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::Unacquire(this).into()
            }
        }
        unsafe extern "system" fn GetDeviceState<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::GetDeviceState(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetDeviceData<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::GetDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn SetDataFormat<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDATAFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::SetDataFormat(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetEventNotification<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::SetEventNotification(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetObjectInfo<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::GetObjectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::GetDeviceInfo(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn RunControlPanel<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn CreateEffect<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::CreateEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn EnumEffects<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMEFFECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::EnumEffects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetEffectInfo<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFECTINFOW, param1: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::GetEffectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::GetForceFeedbackState(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::SendForceFeedbackCommand(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn EnumCreatedEffectObjects<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMCREATEDEFFECTOBJECTSCALLBACK, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::EnumCreatedEffectObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn Escape<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFESCAPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::Escape(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Poll<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::Poll(this).into()
            }
        }
        unsafe extern "system" fn SendDeviceData<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::SendDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn EnumEffectsInFile<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::EnumEffectsInFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::WriteEffectToFile(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn BuildActionMap<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: windows_core::PCWSTR, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::BuildActionMap(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn SetActionMap<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: windows_core::PCWSTR, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::SetActionMap(this, core::mem::transmute_copy(&param0), core::mem::transmute(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetImageInfo<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEIMAGEINFOHEADERW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::GetImageInfo(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            EnumObjects: EnumObjects::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Acquire: Acquire::<Identity, OFFSET>,
            Unacquire: Unacquire::<Identity, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            CreateEffect: CreateEffect::<Identity, OFFSET>,
            EnumEffects: EnumEffects::<Identity, OFFSET>,
            GetEffectInfo: GetEffectInfo::<Identity, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, OFFSET>,
            EnumCreatedEffectObjects: EnumCreatedEffectObjects::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            Poll: Poll::<Identity, OFFSET>,
            SendDeviceData: SendDeviceData::<Identity, OFFSET>,
            EnumEffectsInFile: EnumEffectsInFile::<Identity, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, OFFSET>,
            BuildActionMap: BuildActionMap::<Identity, OFFSET>,
            SetActionMap: SetActionMap::<Identity, OFFSET>,
            GetImageInfo: GetImageInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice8W as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInputDevice8W {}
windows_core::imp::define_interface!(IDirectInputDeviceA, IDirectInputDeviceA_Vtbl, 0x5944e680_c92e_11cf_bfc7_444553540000);
windows_core::imp::interface_hierarchy!(IDirectInputDeviceA, windows_core::IUnknown);
impl IDirectInputDeviceA {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumObjects)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn GetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn SetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn Acquire(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Acquire)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Unacquire(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unacquire)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceState)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceData)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3).ok() }
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDataFormat)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventNotification)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectInfo)(windows_core::Interface::as_raw(self), param0 as _, param1, param2).ok() }
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceInfo)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0, param1, param2).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDeviceA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVCAPS) -> windows_core::HRESULT,
    pub EnumObjects: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIENUMDEVICEOBJECTSCALLBACKA, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DIPROPHEADER) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DIPROPHEADER) -> windows_core::HRESULT,
    pub Acquire: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unacquire: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DIDEVICEOBJECTDATA, *mut u32, u32) -> windows_core::HRESULT,
    pub SetDataFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDATAFORMAT) -> windows_core::HRESULT,
    pub SetEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub GetObjectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEOBJECTINSTANCEA, u32, u32) -> windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEINSTANCEA) -> windows_core::HRESULT,
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IDirectInputDeviceA_Impl: windows_core::IUnknownImpl {
    fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> windows_core::Result<()>;
    fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn SetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn Acquire(&self) -> windows_core::Result<()>;
    fn Unacquire(&self) -> windows_core::Result<()>;
    fn GetDeviceState(&self, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
    fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> windows_core::Result<()>;
    fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IDirectInputDeviceA_Vtbl {
    pub const fn new<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCapabilities<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::GetCapabilities(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn EnumObjects<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKA, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::EnumObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::GetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::SetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Acquire<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::Acquire(this).into()
            }
        }
        unsafe extern "system" fn Unacquire<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::Unacquire(this).into()
            }
        }
        unsafe extern "system" fn GetDeviceState<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::GetDeviceState(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetDeviceData<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::GetDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn SetDataFormat<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDATAFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::SetDataFormat(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetEventNotification<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::SetEventNotification(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetObjectInfo<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::GetObjectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEINSTANCEA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::GetDeviceInfo(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn RunControlPanel<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectInputDeviceA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceA_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            EnumObjects: EnumObjects::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Acquire: Acquire::<Identity, OFFSET>,
            Unacquire: Unacquire::<Identity, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDeviceA as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInputDeviceA {}
windows_core::imp::define_interface!(IDirectInputDeviceW, IDirectInputDeviceW_Vtbl, 0x5944e681_c92e_11cf_bfc7_444553540000);
windows_core::imp::interface_hierarchy!(IDirectInputDeviceW, windows_core::IUnknown);
impl IDirectInputDeviceW {
    pub unsafe fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumObjects)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn GetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn SetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn Acquire(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Acquire)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Unacquire(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unacquire)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDeviceState(&self, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceState)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceData)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3).ok() }
    }
    pub unsafe fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDataFormat)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventNotification)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectInfo)(windows_core::Interface::as_raw(self), param0 as _, param1, param2).ok() }
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceInfo)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0, param1, param2).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputDeviceW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVCAPS) -> windows_core::HRESULT,
    pub EnumObjects: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIENUMDEVICEOBJECTSCALLBACKW, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DIPROPHEADER) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut DIPROPHEADER) -> windows_core::HRESULT,
    pub Acquire: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unacquire: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDeviceData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DIDEVICEOBJECTDATA, *mut u32, u32) -> windows_core::HRESULT,
    pub SetDataFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDATAFORMAT) -> windows_core::HRESULT,
    pub SetEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub GetObjectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEOBJECTINSTANCEW, u32, u32) -> windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEINSTANCEW) -> windows_core::HRESULT,
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32, *const windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IDirectInputDeviceW_Impl: windows_core::IUnknownImpl {
    fn GetCapabilities(&self, param0: *mut DIDEVCAPS) -> windows_core::Result<()>;
    fn EnumObjects(&self, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::Result<()>;
    fn GetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn SetProperty(&self, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::Result<()>;
    fn Acquire(&self) -> windows_core::Result<()>;
    fn Unacquire(&self) -> windows_core::Result<()>;
    fn GetDeviceState(&self, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetDeviceData(&self, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::Result<()>;
    fn SetDataFormat(&self, param0: *mut DIDATAFORMAT) -> windows_core::Result<()>;
    fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> windows_core::Result<()>;
    fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()>;
}
impl IDirectInputDeviceW_Vtbl {
    pub const fn new<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCapabilities<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::GetCapabilities(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn EnumObjects<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIENUMDEVICEOBJECTSCALLBACKW, param1: *mut core::ffi::c_void, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::EnumObjects(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::GetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut DIPROPHEADER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::SetProperty(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Acquire<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::Acquire(this).into()
            }
        }
        unsafe extern "system" fn Unacquire<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::Unacquire(this).into()
            }
        }
        unsafe extern "system" fn GetDeviceState<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::GetDeviceState(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetDeviceData<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICEOBJECTDATA, param2: *mut u32, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::GetDeviceData(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn SetDataFormat<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDATAFORMAT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::SetDataFormat(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetEventNotification<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::SetEventNotification(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetObjectInfo<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::GetObjectInfo(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetDeviceInfo<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDEVICEINSTANCEW) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::GetDeviceInfo(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn RunControlPanel<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectInputDeviceW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDeviceW_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            EnumObjects: EnumObjects::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Acquire: Acquire::<Identity, OFFSET>,
            Unacquire: Unacquire::<Identity, OFFSET>,
            GetDeviceState: GetDeviceState::<Identity, OFFSET>,
            GetDeviceData: GetDeviceData::<Identity, OFFSET>,
            SetDataFormat: SetDataFormat::<Identity, OFFSET>,
            SetEventNotification: SetEventNotification::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            GetObjectInfo: GetObjectInfo::<Identity, OFFSET>,
            GetDeviceInfo: GetDeviceInfo::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDeviceW as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInputDeviceW {}
windows_core::imp::define_interface!(IDirectInputEffect, IDirectInputEffect_Vtbl, 0xe7e1f7c0_88d2_11d0_9ad0_00a0c9a06e35);
windows_core::imp::interface_hierarchy!(IDirectInputEffect, windows_core::IUnknown);
impl IDirectInputEffect {
    pub unsafe fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0, param1, param2).ok() }
    }
    pub unsafe fn GetEffectGuid(&self, param0: *mut windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEffectGuid)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn GetParameters(&self, param0: *mut DIEFFECT, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetParameters)(windows_core::Interface::as_raw(self), param0 as _, param1).ok() }
    }
    pub unsafe fn SetParameters(&self, param0: *mut DIEFFECT, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetParameters)(windows_core::Interface::as_raw(self), param0 as _, param1).ok() }
    }
    pub unsafe fn Start(&self, param0: u32, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn Stop(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetEffectStatus(&self, param0: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEffectStatus)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn Download(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Download)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Unload(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unload)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputEffect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetEffectGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIEFFECT, u32) -> windows_core::HRESULT,
    pub SetParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIEFFECT, u32) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEffectStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Download: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unload: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIEFFESCAPE) -> windows_core::HRESULT,
}
pub trait IDirectInputEffect_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetEffectGuid(&self, param0: *mut windows_core::GUID) -> windows_core::Result<()>;
    fn GetParameters(&self, param0: *mut DIEFFECT, param1: u32) -> windows_core::Result<()>;
    fn SetParameters(&self, param0: *mut DIEFFECT, param1: u32) -> windows_core::Result<()>;
    fn Start(&self, param0: u32, param1: u32) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn GetEffectStatus(&self, param0: *mut u32) -> windows_core::Result<()>;
    fn Download(&self) -> windows_core::Result<()>;
    fn Unload(&self) -> windows_core::Result<()>;
    fn Escape(&self, param0: *mut DIEFFESCAPE) -> windows_core::Result<()>;
}
impl IDirectInputEffect_Vtbl {
    pub const fn new<Identity: IDirectInputEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffect_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn GetEffectGuid<Identity: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffect_Impl::GetEffectGuid(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetParameters<Identity: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffect_Impl::GetParameters(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetParameters<Identity: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFECT, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffect_Impl::SetParameters(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Start<Identity: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffect_Impl::Start(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffect_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn GetEffectStatus<Identity: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffect_Impl::GetEffectStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Download<Identity: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffect_Impl::Download(this).into()
            }
        }
        unsafe extern "system" fn Unload<Identity: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffect_Impl::Unload(this).into()
            }
        }
        unsafe extern "system" fn Escape<Identity: IDirectInputEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIEFFESCAPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffect_Impl::Escape(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetEffectGuid: GetEffectGuid::<Identity, OFFSET>,
            GetParameters: GetParameters::<Identity, OFFSET>,
            SetParameters: SetParameters::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            GetEffectStatus: GetEffectStatus::<Identity, OFFSET>,
            Download: Download::<Identity, OFFSET>,
            Unload: Unload::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInputEffect {}
windows_core::imp::define_interface!(IDirectInputEffectDriver, IDirectInputEffectDriver_Vtbl, 0x02538130_898f_11d0_9ad0_00a0c9a06e35);
windows_core::imp::interface_hierarchy!(IDirectInputEffectDriver, windows_core::IUnknown);
impl IDirectInputEffectDriver {
    pub unsafe fn DeviceID(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeviceID)(windows_core::Interface::as_raw(self), param0, param1, param2, param3, param4 as _).ok() }
    }
    pub unsafe fn GetVersions(&self, param0: *mut DIDRIVERVERSIONS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVersions)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    pub unsafe fn Escape(&self, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Escape)(windows_core::Interface::as_raw(self), param0, param1, param2 as _).ok() }
    }
    pub unsafe fn SetGain(&self, param0: u32, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGain)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn SendForceFeedbackCommand(&self, param0: u32, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendForceFeedbackCommand)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn GetForceFeedbackState(&self, param0: u32, param1: *mut DIDEVICESTATE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetForceFeedbackState)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn DownloadEffect(&self, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DownloadEffect)(windows_core::Interface::as_raw(self), param0, param1, param2 as _, param3 as _, param4).ok() }
    }
    pub unsafe fn DestroyEffect(&self, param0: u32, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DestroyEffect)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn StartEffect(&self, param0: u32, param1: u32, param2: u32, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartEffect)(windows_core::Interface::as_raw(self), param0, param1, param2, param3).ok() }
    }
    pub unsafe fn StopEffect(&self, param0: u32, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopEffect)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn GetEffectStatus(&self, param0: u32, param1: u32, param2: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEffectStatus)(windows_core::Interface::as_raw(self), param0, param1, param2 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputEffectDriver_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DeviceID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVersions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDRIVERVERSIONS) -> windows_core::HRESULT,
    pub Escape: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut DIEFFESCAPE) -> windows_core::HRESULT,
    pub SetGain: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub SendForceFeedbackCommand: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetForceFeedbackState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DIDEVICESTATE) -> windows_core::HRESULT,
    pub DownloadEffect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32, *mut DIEFFECT, u32) -> windows_core::HRESULT,
    pub DestroyEffect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub StartEffect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32) -> windows_core::HRESULT,
    pub StopEffect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetEffectStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IDirectInputEffectDriver_Impl: windows_core::IUnknownImpl {
    fn DeviceID(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetVersions(&self, param0: *mut DIDRIVERVERSIONS) -> windows_core::Result<()>;
    fn Escape(&self, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> windows_core::Result<()>;
    fn SetGain(&self, param0: u32, param1: u32) -> windows_core::Result<()>;
    fn SendForceFeedbackCommand(&self, param0: u32, param1: u32) -> windows_core::Result<()>;
    fn GetForceFeedbackState(&self, param0: u32, param1: *mut DIDEVICESTATE) -> windows_core::Result<()>;
    fn DownloadEffect(&self, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> windows_core::Result<()>;
    fn DestroyEffect(&self, param0: u32, param1: u32) -> windows_core::Result<()>;
    fn StartEffect(&self, param0: u32, param1: u32, param2: u32, param3: u32) -> windows_core::Result<()>;
    fn StopEffect(&self, param0: u32, param1: u32) -> windows_core::Result<()>;
    fn GetEffectStatus(&self, param0: u32, param1: u32, param2: *mut u32) -> windows_core::Result<()>;
}
impl IDirectInputEffectDriver_Vtbl {
    pub const fn new<Identity: IDirectInputEffectDriver_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeviceID<Identity: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffectDriver_Impl::DeviceID(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn GetVersions<Identity: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIDRIVERVERSIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffectDriver_Impl::GetVersions(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn Escape<Identity: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut DIEFFESCAPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffectDriver_Impl::Escape(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn SetGain<Identity: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffectDriver_Impl::SetGain(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SendForceFeedbackCommand<Identity: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffectDriver_Impl::SendForceFeedbackCommand(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetForceFeedbackState<Identity: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIDEVICESTATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffectDriver_Impl::GetForceFeedbackState(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn DownloadEffect<Identity: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32, param3: *mut DIEFFECT, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffectDriver_Impl::DownloadEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
            }
        }
        unsafe extern "system" fn DestroyEffect<Identity: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffectDriver_Impl::DestroyEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn StartEffect<Identity: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffectDriver_Impl::StartEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn StopEffect<Identity: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffectDriver_Impl::StopEffect(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetEffectStatus<Identity: IDirectInputEffectDriver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputEffectDriver_Impl::GetEffectStatus(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeviceID: DeviceID::<Identity, OFFSET>,
            GetVersions: GetVersions::<Identity, OFFSET>,
            Escape: Escape::<Identity, OFFSET>,
            SetGain: SetGain::<Identity, OFFSET>,
            SendForceFeedbackCommand: SendForceFeedbackCommand::<Identity, OFFSET>,
            GetForceFeedbackState: GetForceFeedbackState::<Identity, OFFSET>,
            DownloadEffect: DownloadEffect::<Identity, OFFSET>,
            DestroyEffect: DestroyEffect::<Identity, OFFSET>,
            StartEffect: StartEffect::<Identity, OFFSET>,
            StopEffect: StopEffect::<Identity, OFFSET>,
            GetEffectStatus: GetEffectStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputEffectDriver as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInputEffectDriver {}
windows_core::imp::define_interface!(IDirectInputJoyConfig, IDirectInputJoyConfig_Vtbl, 0x1de12ab1_c9f5_11cf_bfc7_444553540000);
windows_core::imp::interface_hierarchy!(IDirectInputJoyConfig, windows_core::IUnknown);
impl IDirectInputJoyConfig {
    pub unsafe fn Acquire(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Acquire)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Unacquire(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unacquire)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn SendNotify(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendNotify)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnumTypes(&self, param0: LPDIJOYTYPECALLBACK, param1: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumTypes)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn GetTypeInfo<P0>(&self, param0: P0, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTypeInfo)(windows_core::Interface::as_raw(self), param0.param().abi(), param1 as _, param2).ok() }
    }
    pub unsafe fn SetTypeInfo<P0>(&self, param0: P0, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTypeInfo)(windows_core::Interface::as_raw(self), param0.param().abi(), param1 as _, param2).ok() }
    }
    pub unsafe fn DeleteType<P0>(&self, param0: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteType)(windows_core::Interface::as_raw(self), param0.param().abi()).ok() }
    }
    pub unsafe fn GetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetConfig)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn SetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConfig)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn DeleteConfig(&self, param0: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteConfig)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn GetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetUserValues)(windows_core::Interface::as_raw(self), param0 as _, param1).ok() }
    }
    pub unsafe fn SetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUserValues)(windows_core::Interface::as_raw(self), param0 as _, param1).ok() }
    }
    pub unsafe fn AddNewHardware(&self, param0: super::super::Foundation::HWND, param1: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddNewHardware)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn OpenTypeKey<P0>(&self, param0: P0, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenTypeKey)(windows_core::Interface::as_raw(self), param0.param().abi(), param1, param2 as _).ok() }
    }
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn OpenConfigKey(&self, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OpenConfigKey)(windows_core::Interface::as_raw(self), param0, param1, param2 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputJoyConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Acquire: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unacquire: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub SendNotify: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumTypes: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIJOYTYPECALLBACK, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut DIJOYTYPEINFO, u32) -> windows_core::HRESULT,
    pub SetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut DIJOYTYPEINFO, u32) -> windows_core::HRESULT,
    pub DeleteType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetConfig: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DIJOYCONFIG, u32) -> windows_core::HRESULT,
    pub SetConfig: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DIJOYCONFIG, u32) -> windows_core::HRESULT,
    pub DeleteConfig: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetUserValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIJOYUSERVALUES, u32) -> windows_core::HRESULT,
    pub SetUserValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIJOYUSERVALUES, u32) -> windows_core::HRESULT,
    pub AddNewHardware: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenTypeKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenTypeKey: usize,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenConfigKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenConfigKey: usize,
}
#[cfg(feature = "Win32_System_Registry")]
pub trait IDirectInputJoyConfig_Impl: windows_core::IUnknownImpl {
    fn Acquire(&self) -> windows_core::Result<()>;
    fn Unacquire(&self) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn SendNotify(&self) -> windows_core::Result<()>;
    fn EnumTypes(&self, param0: LPDIJOYTYPECALLBACK, param1: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetTypeInfo(&self, param0: &windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::Result<()>;
    fn SetTypeInfo(&self, param0: &windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::Result<()>;
    fn DeleteType(&self, param0: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::Result<()>;
    fn SetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::Result<()>;
    fn DeleteConfig(&self, param0: u32) -> windows_core::Result<()>;
    fn GetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::Result<()>;
    fn SetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::Result<()>;
    fn AddNewHardware(&self, param0: super::super::Foundation::HWND, param1: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OpenTypeKey(&self, param0: &windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::Result<()>;
    fn OpenConfigKey(&self, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Registry")]
impl IDirectInputJoyConfig_Vtbl {
    pub const fn new<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Acquire<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::Acquire(this).into()
            }
        }
        unsafe extern "system" fn Unacquire<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::Unacquire(this).into()
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SendNotify<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::SendNotify(this).into()
            }
        }
        unsafe extern "system" fn EnumTypes<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIJOYTYPECALLBACK, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::EnumTypes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetTypeInfo<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::GetTypeInfo(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn SetTypeInfo<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::SetTypeInfo(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn DeleteType<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::DeleteType(this, core::mem::transmute(&param0)).into()
            }
        }
        unsafe extern "system" fn GetConfig<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::GetConfig(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn SetConfig<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::SetConfig(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn DeleteConfig<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::DeleteConfig(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetUserValues<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::GetUserValues(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetUserValues<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::SetUserValues(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn AddNewHardware<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::AddNewHardware(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn OpenTypeKey<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::OpenTypeKey(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn OpenConfigKey<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::OpenConfigKey(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Acquire: Acquire::<Identity, OFFSET>,
            Unacquire: Unacquire::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            SendNotify: SendNotify::<Identity, OFFSET>,
            EnumTypes: EnumTypes::<Identity, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, OFFSET>,
            SetTypeInfo: SetTypeInfo::<Identity, OFFSET>,
            DeleteType: DeleteType::<Identity, OFFSET>,
            GetConfig: GetConfig::<Identity, OFFSET>,
            SetConfig: SetConfig::<Identity, OFFSET>,
            DeleteConfig: DeleteConfig::<Identity, OFFSET>,
            GetUserValues: GetUserValues::<Identity, OFFSET>,
            SetUserValues: SetUserValues::<Identity, OFFSET>,
            AddNewHardware: AddNewHardware::<Identity, OFFSET>,
            OpenTypeKey: OpenTypeKey::<Identity, OFFSET>,
            OpenConfigKey: OpenConfigKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputJoyConfig as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::RuntimeName for IDirectInputJoyConfig {}
windows_core::imp::define_interface!(IDirectInputJoyConfig8, IDirectInputJoyConfig8_Vtbl, 0xeb0d7dfa_1990_4f27_b4d6_edf2eec4a44c);
windows_core::imp::interface_hierarchy!(IDirectInputJoyConfig8, windows_core::IUnknown);
impl IDirectInputJoyConfig8 {
    pub unsafe fn Acquire(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Acquire)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Unacquire(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unacquire)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn SendNotify(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SendNotify)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn EnumTypes(&self, param0: LPDIJOYTYPECALLBACK, param1: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumTypes)(windows_core::Interface::as_raw(self), param0, param1 as _).ok() }
    }
    pub unsafe fn GetTypeInfo<P0>(&self, param0: P0, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTypeInfo)(windows_core::Interface::as_raw(self), param0.param().abi(), param1 as _, param2).ok() }
    }
    pub unsafe fn SetTypeInfo<P0, P3>(&self, param0: P0, param1: *mut DIJOYTYPEINFO, param2: u32, param3: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTypeInfo)(windows_core::Interface::as_raw(self), param0.param().abi(), param1 as _, param2, param3.param().abi()).ok() }
    }
    pub unsafe fn DeleteType<P0>(&self, param0: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteType)(windows_core::Interface::as_raw(self), param0.param().abi()).ok() }
    }
    pub unsafe fn GetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetConfig)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn SetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetConfig)(windows_core::Interface::as_raw(self), param0, param1 as _, param2).ok() }
    }
    pub unsafe fn DeleteConfig(&self, param0: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteConfig)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn GetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetUserValues)(windows_core::Interface::as_raw(self), param0 as _, param1).ok() }
    }
    pub unsafe fn SetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUserValues)(windows_core::Interface::as_raw(self), param0 as _, param1).ok() }
    }
    pub unsafe fn AddNewHardware(&self, param0: super::super::Foundation::HWND, param1: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddNewHardware)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn OpenTypeKey<P0>(&self, param0: P0, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenTypeKey)(windows_core::Interface::as_raw(self), param0.param().abi(), param1, param2 as _).ok() }
    }
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn OpenAppStatusKey(&self, param0: *mut super::super::System::Registry::HKEY) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OpenAppStatusKey)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputJoyConfig8_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Acquire: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Unacquire: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub SendNotify: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumTypes: unsafe extern "system" fn(*mut core::ffi::c_void, LPDIJOYTYPECALLBACK, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut DIJOYTYPEINFO, u32) -> windows_core::HRESULT,
    pub SetTypeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut DIJOYTYPEINFO, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub DeleteType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetConfig: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DIJOYCONFIG, u32) -> windows_core::HRESULT,
    pub SetConfig: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut DIJOYCONFIG, u32) -> windows_core::HRESULT,
    pub DeleteConfig: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetUserValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIJOYUSERVALUES, u32) -> windows_core::HRESULT,
    pub SetUserValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIJOYUSERVALUES, u32) -> windows_core::HRESULT,
    pub AddNewHardware: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenTypeKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenTypeKey: usize,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenAppStatusKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenAppStatusKey: usize,
}
#[cfg(feature = "Win32_System_Registry")]
pub trait IDirectInputJoyConfig8_Impl: windows_core::IUnknownImpl {
    fn Acquire(&self) -> windows_core::Result<()>;
    fn Unacquire(&self) -> windows_core::Result<()>;
    fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn SendNotify(&self) -> windows_core::Result<()>;
    fn EnumTypes(&self, param0: LPDIJOYTYPECALLBACK, param1: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetTypeInfo(&self, param0: &windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::Result<()>;
    fn SetTypeInfo(&self, param0: &windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeleteType(&self, param0: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::Result<()>;
    fn SetConfig(&self, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::Result<()>;
    fn DeleteConfig(&self, param0: u32) -> windows_core::Result<()>;
    fn GetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::Result<()>;
    fn SetUserValues(&self, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::Result<()>;
    fn AddNewHardware(&self, param0: super::super::Foundation::HWND, param1: *const windows_core::GUID) -> windows_core::Result<()>;
    fn OpenTypeKey(&self, param0: &windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::Result<()>;
    fn OpenAppStatusKey(&self, param0: *mut super::super::System::Registry::HKEY) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Registry")]
impl IDirectInputJoyConfig8_Vtbl {
    pub const fn new<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Acquire<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::Acquire(this).into()
            }
        }
        unsafe extern "system" fn Unacquire<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::Unacquire(this).into()
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::SetCooperativeLevel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SendNotify<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::SendNotify(this).into()
            }
        }
        unsafe extern "system" fn EnumTypes<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: LPDIJOYTYPECALLBACK, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::EnumTypes(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn GetTypeInfo<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::GetTypeInfo(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn SetTypeInfo<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::SetTypeInfo(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute(&param3)).into()
            }
        }
        unsafe extern "system" fn DeleteType<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::DeleteType(this, core::mem::transmute(&param0)).into()
            }
        }
        unsafe extern "system" fn GetConfig<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::GetConfig(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn SetConfig<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: *mut DIJOYCONFIG, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::SetConfig(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn DeleteConfig<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::DeleteConfig(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn GetUserValues<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::GetUserValues(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn SetUserValues<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIJOYUSERVALUES, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::SetUserValues(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn AddNewHardware<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::AddNewHardware(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn OpenTypeKey<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: u32, param2: *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::OpenTypeKey(this, core::mem::transmute(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn OpenAppStatusKey<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::OpenAppStatusKey(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Acquire: Acquire::<Identity, OFFSET>,
            Unacquire: Unacquire::<Identity, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, OFFSET>,
            SendNotify: SendNotify::<Identity, OFFSET>,
            EnumTypes: EnumTypes::<Identity, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, OFFSET>,
            SetTypeInfo: SetTypeInfo::<Identity, OFFSET>,
            DeleteType: DeleteType::<Identity, OFFSET>,
            GetConfig: GetConfig::<Identity, OFFSET>,
            SetConfig: SetConfig::<Identity, OFFSET>,
            DeleteConfig: DeleteConfig::<Identity, OFFSET>,
            GetUserValues: GetUserValues::<Identity, OFFSET>,
            SetUserValues: SetUserValues::<Identity, OFFSET>,
            AddNewHardware: AddNewHardware::<Identity, OFFSET>,
            OpenTypeKey: OpenTypeKey::<Identity, OFFSET>,
            OpenAppStatusKey: OpenAppStatusKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputJoyConfig8 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Registry")]
impl windows_core::RuntimeName for IDirectInputJoyConfig8 {}
windows_core::imp::define_interface!(IDirectInputW, IDirectInputW_Vtbl, 0x89521361_aa8a_11cf_bfc7_444553540000);
windows_core::imp::interface_hierarchy!(IDirectInputW, windows_core::IUnknown);
impl IDirectInputW {
    pub unsafe fn CreateDevice<P2>(&self, param0: *const windows_core::GUID, param1: *mut Option<IDirectInputDeviceW>, param2: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), param0, core::mem::transmute(param1), param2.param().abi()).ok() }
    }
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumDevices)(windows_core::Interface::as_raw(self), param0, param1, param2 as _, param3).ok() }
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceStatus)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInputW_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, LPDIENUMDEVICESCALLBACKW, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32) -> windows_core::HRESULT,
}
pub trait IDirectInputW_Impl: windows_core::IUnknownImpl {
    fn CreateDevice(&self, param0: *const windows_core::GUID, param1: windows_core::OutRef<IDirectInputDeviceW>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()>;
}
impl IDirectInputW_Vtbl {
    pub const fn new<Identity: IDirectInputW_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDevice<Identity: IDirectInputW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: *mut *mut core::ffi::c_void, param2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputW_Impl::CreateDevice(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn EnumDevices<Identity: IDirectInputW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputW_Impl::EnumDevices(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: IDirectInputW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputW_Impl::GetDeviceStatus(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        unsafe extern "system" fn RunControlPanel<Identity: IDirectInputW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputW_Impl::RunControlPanel(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        unsafe extern "system" fn Initialize<Identity: IDirectInputW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputW_Impl::Initialize(this, core::mem::transmute_copy(&param0), core::mem::transmute_copy(&param1)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateDevice: CreateDevice::<Identity, OFFSET>,
            EnumDevices: EnumDevices::<Identity, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, OFFSET>,
            RunControlPanel: RunControlPanel::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputW as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectInputW {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INDICATOR_LIST {
    pub MakeCode: u16,
    pub IndicatorFlags: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INPUT_BUTTON_ENABLE_INFO {
    pub ButtonType: GPIOBUTTONS_BUTTON_TYPE,
    pub Enabled: bool,
}
pub const IOCTL_BUTTON_GET_ENABLED_ON_IDLE: u32 = 721580u32;
pub const IOCTL_BUTTON_SET_ENABLED_ON_IDLE: u32 = 721576u32;
pub const IOCTL_KEYBOARD_INSERT_DATA: u32 = 721152u32;
pub const IOCTL_KEYBOARD_QUERY_ATTRIBUTES: u32 = 720896u32;
pub const IOCTL_KEYBOARD_QUERY_EXTENDED_ATTRIBUTES: u32 = 721408u32;
pub const IOCTL_KEYBOARD_QUERY_IME_STATUS: u32 = 724992u32;
pub const IOCTL_KEYBOARD_QUERY_INDICATORS: u32 = 720960u32;
pub const IOCTL_KEYBOARD_QUERY_INDICATOR_TRANSLATION: u32 = 721024u32;
pub const IOCTL_KEYBOARD_QUERY_TYPEMATIC: u32 = 720928u32;
pub const IOCTL_KEYBOARD_SET_IME_STATUS: u32 = 724996u32;
pub const IOCTL_KEYBOARD_SET_INDICATORS: u32 = 720904u32;
pub const IOCTL_KEYBOARD_SET_TYPEMATIC: u32 = 720900u32;
pub const IOCTL_MOUSE_INSERT_DATA: u32 = 983044u32;
pub const IOCTL_MOUSE_QUERY_ATTRIBUTES: u32 = 983040u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOYCALIBRATE {
    pub wXbase: u32,
    pub wXdelta: u32,
    pub wYbase: u32,
    pub wYdelta: u32,
    pub wZbase: u32,
    pub wZdelta: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOYPOS {
    pub dwX: u32,
    pub dwY: u32,
    pub dwZ: u32,
    pub dwR: u32,
    pub dwU: u32,
    pub dwV: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOYRANGE {
    pub jpMin: JOYPOS,
    pub jpMax: JOYPOS,
    pub jpCenter: JOYPOS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOYREGHWCONFIG {
    pub hws: JOYREGHWSETTINGS,
    pub dwUsageSettings: u32,
    pub hwv: JOYREGHWVALUES,
    pub dwType: u32,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOYREGHWSETTINGS {
    pub dwFlags: u32,
    pub dwNumButtons: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JOYREGHWVALUES {
    pub jrvHardware: JOYRANGE,
    pub dwPOVValues: [u32; 4],
    pub dwCalFlags: u32,
}
impl Default for JOYREGHWVALUES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOYREGUSERVALUES {
    pub dwTimeOut: u32,
    pub jrvRanges: JOYRANGE,
    pub jpDeadZone: JOYPOS,
}
pub const JOYTYPE_ANALOGCOMPAT: i32 = 8i32;
pub const JOYTYPE_DEFAULTPROPSHEET: i32 = -2147483648i32;
pub const JOYTYPE_DEVICEHIDE: i32 = 65536i32;
pub const JOYTYPE_ENABLEINPUTREPORT: i32 = 16777216i32;
pub const JOYTYPE_GAMEHIDE: i32 = 524288i32;
pub const JOYTYPE_HIDEACTIVE: i32 = 1048576i32;
pub const JOYTYPE_INFODEFAULT: i32 = 0i32;
pub const JOYTYPE_INFOMASK: i32 = 14680064i32;
pub const JOYTYPE_INFOYRPEDALS: i32 = 6291456i32;
pub const JOYTYPE_INFOYYPEDALS: i32 = 2097152i32;
pub const JOYTYPE_INFOZISSLIDER: i32 = 2097152i32;
pub const JOYTYPE_INFOZISZ: i32 = 4194304i32;
pub const JOYTYPE_INFOZRPEDALS: i32 = 8388608i32;
pub const JOYTYPE_INFOZYPEDALS: i32 = 4194304i32;
pub const JOYTYPE_KEYBHIDE: i32 = 262144i32;
pub const JOYTYPE_MOUSEHIDE: i32 = 131072i32;
pub const JOYTYPE_NOAUTODETECTGAMEPORT: i32 = 2i32;
pub const JOYTYPE_NOHIDDIRECT: i32 = 4i32;
pub const JOYTYPE_ZEROGAMEENUMOEMDATA: i32 = 1i32;
pub const JOY_HWS_AUTOLOAD: i32 = 268435456i32;
pub const JOY_HWS_GAMEPORTBUSBUSY: i32 = 1i32;
pub const JOY_HWS_HASPOV: i32 = 2i32;
pub const JOY_HWS_HASR: i32 = 524288i32;
pub const JOY_HWS_HASU: i32 = 8388608i32;
pub const JOY_HWS_HASV: i32 = 16777216i32;
pub const JOY_HWS_HASZ: i32 = 1i32;
pub const JOY_HWS_ISANALOGPORTDRIVER: i32 = 134217728i32;
pub const JOY_HWS_ISCARCTRL: i32 = 64i32;
pub const JOY_HWS_ISGAMEPAD: i32 = 32i32;
pub const JOY_HWS_ISGAMEPORTBUS: i32 = -2147483648i32;
pub const JOY_HWS_ISGAMEPORTDRIVER: i32 = 67108864i32;
pub const JOY_HWS_ISHEADTRACKER: i32 = 33554432i32;
pub const JOY_HWS_ISYOKE: i32 = 16i32;
pub const JOY_HWS_NODEVNODE: i32 = 536870912i32;
pub const JOY_HWS_POVISBUTTONCOMBOS: i32 = 4i32;
pub const JOY_HWS_POVISJ1X: i32 = 65536i32;
pub const JOY_HWS_POVISJ1Y: i32 = 131072i32;
pub const JOY_HWS_POVISJ2X: i32 = 262144i32;
pub const JOY_HWS_POVISPOLL: i32 = 8i32;
pub const JOY_HWS_RISJ1X: i32 = 1048576i32;
pub const JOY_HWS_RISJ1Y: i32 = 2097152i32;
pub const JOY_HWS_RISJ2Y: i32 = 4194304i32;
pub const JOY_HWS_XISJ1Y: i32 = 128i32;
pub const JOY_HWS_XISJ2X: i32 = 256i32;
pub const JOY_HWS_XISJ2Y: i32 = 512i32;
pub const JOY_HWS_YISJ1X: i32 = 1024i32;
pub const JOY_HWS_YISJ2X: i32 = 2048i32;
pub const JOY_HWS_YISJ2Y: i32 = 4096i32;
pub const JOY_HWS_ZISJ1X: i32 = 8192i32;
pub const JOY_HWS_ZISJ1Y: i32 = 16384i32;
pub const JOY_HWS_ZISJ2X: i32 = 32768i32;
pub const JOY_HW_2A_2B_GENERIC: u32 = 2u32;
pub const JOY_HW_2A_4B_GENERIC: u32 = 3u32;
pub const JOY_HW_2B_FLIGHTYOKE: u32 = 5u32;
pub const JOY_HW_2B_FLIGHTYOKETHROTTLE: u32 = 6u32;
pub const JOY_HW_2B_GAMEPAD: u32 = 4u32;
pub const JOY_HW_3A_2B_GENERIC: u32 = 7u32;
pub const JOY_HW_3A_4B_GENERIC: u32 = 8u32;
pub const JOY_HW_4B_FLIGHTYOKE: u32 = 10u32;
pub const JOY_HW_4B_FLIGHTYOKETHROTTLE: u32 = 11u32;
pub const JOY_HW_4B_GAMEPAD: u32 = 9u32;
pub const JOY_HW_CUSTOM: u32 = 1u32;
pub const JOY_HW_LASTENTRY: u32 = 13u32;
pub const JOY_HW_NONE: u32 = 0u32;
pub const JOY_HW_TWO_2A_2B_WITH_Y: u32 = 12u32;
pub const JOY_ISCAL_POV: i32 = 32i32;
pub const JOY_ISCAL_R: i32 = 4i32;
pub const JOY_ISCAL_U: i32 = 8i32;
pub const JOY_ISCAL_V: i32 = 16i32;
pub const JOY_ISCAL_XY: i32 = 1i32;
pub const JOY_ISCAL_Z: i32 = 2i32;
pub const JOY_OEMPOLL_PASSDRIVERDATA: u32 = 7u32;
pub const JOY_PASSDRIVERDATA: i32 = 268435456i32;
pub const JOY_POVVAL_BACKWARD: u32 = 1u32;
pub const JOY_POVVAL_FORWARD: u32 = 0u32;
pub const JOY_POVVAL_LEFT: u32 = 2u32;
pub const JOY_POVVAL_RIGHT: u32 = 3u32;
pub const JOY_POV_NUMDIRS: u32 = 4u32;
pub const JOY_US_HASRUDDER: i32 = 1i32;
pub const JOY_US_ISOEM: i32 = 4i32;
pub const JOY_US_PRESENT: i32 = 2i32;
pub const JOY_US_RESERVED: i32 = -2147483648i32;
pub const JOY_US_VOLATILE: i32 = 8i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_ATTRIBUTES {
    pub KeyboardIdentifier: KEYBOARD_ID,
    pub KeyboardMode: u16,
    pub NumberOfFunctionKeys: u16,
    pub NumberOfIndicators: u16,
    pub NumberOfKeysTotal: u16,
    pub InputDataQueueLength: u32,
    pub KeyRepeatMinimum: KEYBOARD_TYPEMATIC_PARAMETERS,
    pub KeyRepeatMaximum: KEYBOARD_TYPEMATIC_PARAMETERS,
}
pub const KEYBOARD_CAPS_LOCK_ON: u32 = 4u32;
pub const KEYBOARD_ERROR_VALUE_BASE: u32 = 10000u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_EXTENDED_ATTRIBUTES {
    pub Version: u8,
    pub FormFactor: u8,
    pub KeyType: u8,
    pub PhysicalLayout: u8,
    pub VendorSpecificPhysicalLayout: u8,
    pub IETFLanguageTagIndex: u8,
    pub ImplementedInputAssistControls: u8,
}
pub const KEYBOARD_EXTENDED_ATTRIBUTES_STRUCT_VERSION_1: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_ID {
    pub Type: u8,
    pub Subtype: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_IME_STATUS {
    pub UnitId: u16,
    pub ImeOpen: u32,
    pub ImeConvMode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_INDICATOR_PARAMETERS {
    pub UnitId: u16,
    pub LedFlags: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KEYBOARD_INDICATOR_TRANSLATION {
    pub NumberOfIndicatorKeys: u16,
    pub IndicatorList: [INDICATOR_LIST; 1],
}
impl Default for KEYBOARD_INDICATOR_TRANSLATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_INPUT_DATA {
    pub UnitId: u16,
    pub MakeCode: u16,
    pub Flags: u16,
    pub Reserved: u16,
    pub ExtraInformation: u32,
}
pub const KEYBOARD_KANA_LOCK_ON: u32 = 8u32;
pub const KEYBOARD_LED_INJECTED: u32 = 32768u32;
pub const KEYBOARD_NUM_LOCK_ON: u32 = 2u32;
pub const KEYBOARD_OVERRUN_MAKE_CODE: u32 = 255u32;
pub const KEYBOARD_SCROLL_LOCK_ON: u32 = 1u32;
pub const KEYBOARD_SHADOW: u32 = 16384u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_TYPEMATIC_PARAMETERS {
    pub UnitId: u16,
    pub Rate: u16,
    pub Delay: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEYBOARD_UNIT_ID_PARAMETER {
    pub UnitId: u16,
}
pub const KEY_BREAK: u32 = 1u32;
pub const KEY_E0: u32 = 2u32;
pub const KEY_E1: u32 = 4u32;
pub const KEY_FROM_KEYBOARD_OVERRIDER: u32 = 128u32;
pub const KEY_MAKE: u32 = 0u32;
pub const KEY_RIM_VKEY: u32 = 64u32;
pub const KEY_TERMSRV_SET_LED: u32 = 8u32;
pub const KEY_TERMSRV_SHADOW: u32 = 16u32;
pub const KEY_TERMSRV_VKPACKET: u32 = 32u32;
pub const KEY_UNICODE_SEQUENCE_END: u32 = 512u32;
pub const KEY_UNICODE_SEQUENCE_ITEM: u32 = 256u32;
pub type LPDICONFIGUREDEVICESCALLBACK = Option<unsafe extern "system" fn(param0: windows_core::Ref<windows_core::IUnknown>, param1: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPDIENUMCREATEDEFFECTOBJECTSCALLBACK = Option<unsafe extern "system" fn(param0: windows_core::Ref<IDirectInputEffect>, param1: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPDIENUMDEVICEOBJECTSCALLBACKA = Option<unsafe extern "system" fn(param0: *mut DIDEVICEOBJECTINSTANCEA, param1: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPDIENUMDEVICEOBJECTSCALLBACKW = Option<unsafe extern "system" fn(param0: *mut DIDEVICEOBJECTINSTANCEW, param1: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPDIENUMDEVICESBYSEMANTICSCBA = Option<unsafe extern "system" fn(param0: *mut DIDEVICEINSTANCEA, param1: windows_core::Ref<IDirectInputDevice8A>, param2: u32, param3: u32, param4: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPDIENUMDEVICESBYSEMANTICSCBW = Option<unsafe extern "system" fn(param0: *mut DIDEVICEINSTANCEW, param1: windows_core::Ref<IDirectInputDevice8W>, param2: u32, param3: u32, param4: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPDIENUMDEVICESCALLBACKA = Option<unsafe extern "system" fn(param0: *mut DIDEVICEINSTANCEA, param1: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPDIENUMDEVICESCALLBACKW = Option<unsafe extern "system" fn(param0: *mut DIDEVICEINSTANCEW, param1: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPDIENUMEFFECTSCALLBACKA = Option<unsafe extern "system" fn(param0: *mut DIEFFECTINFOA, param1: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPDIENUMEFFECTSCALLBACKW = Option<unsafe extern "system" fn(param0: *mut DIEFFECTINFOW, param1: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPDIENUMEFFECTSINFILECALLBACK = Option<unsafe extern "system" fn(param0: *mut DIFILEEFFECT, param1: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPDIJOYTYPECALLBACK = Option<unsafe extern "system" fn(param0: windows_core::PCWSTR, param1: *mut core::ffi::c_void) -> windows_core::BOOL>;
pub type LPFNSHOWJOYCPL = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND)>;
pub const MAXCPOINTSNUM: u32 = 8u32;
pub const MAX_JOYSTICKOEMVXDNAME: u32 = 260u32;
pub const MAX_JOYSTRING: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MOUSE_ATTRIBUTES {
    pub MouseIdentifier: u16,
    pub NumberOfButtons: u16,
    pub SampleRate: u16,
    pub InputDataQueueLength: u32,
}
pub const MOUSE_BUTTON_1_DOWN: u32 = 1u32;
pub const MOUSE_BUTTON_1_UP: u32 = 2u32;
pub const MOUSE_BUTTON_2_DOWN: u32 = 4u32;
pub const MOUSE_BUTTON_2_UP: u32 = 8u32;
pub const MOUSE_BUTTON_3_DOWN: u32 = 16u32;
pub const MOUSE_BUTTON_3_UP: u32 = 32u32;
pub const MOUSE_BUTTON_4_DOWN: u32 = 64u32;
pub const MOUSE_BUTTON_4_UP: u32 = 128u32;
pub const MOUSE_BUTTON_5_DOWN: u32 = 256u32;
pub const MOUSE_BUTTON_5_UP: u32 = 512u32;
pub const MOUSE_ERROR_VALUE_BASE: u32 = 20000u32;
pub const MOUSE_HID_HARDWARE: u32 = 128u32;
pub const MOUSE_HWHEEL: u32 = 2048u32;
pub const MOUSE_I8042_HARDWARE: u32 = 2u32;
pub const MOUSE_INPORT_HARDWARE: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MOUSE_INPUT_DATA {
    pub UnitId: u16,
    pub Flags: u16,
    pub Anonymous: MOUSE_INPUT_DATA_0,
    pub RawButtons: u32,
    pub LastX: i32,
    pub LastY: i32,
    pub ExtraInformation: u32,
}
impl Default for MOUSE_INPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MOUSE_INPUT_DATA_0 {
    pub Buttons: u32,
    pub Anonymous: MOUSE_INPUT_DATA_0_0,
}
impl Default for MOUSE_INPUT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MOUSE_INPUT_DATA_0_0 {
    pub ButtonFlags: u16,
    pub ButtonData: u16,
}
pub const MOUSE_LEFT_BUTTON_DOWN: u32 = 1u32;
pub const MOUSE_LEFT_BUTTON_UP: u32 = 2u32;
pub const MOUSE_MIDDLE_BUTTON_DOWN: u32 = 16u32;
pub const MOUSE_MIDDLE_BUTTON_UP: u32 = 32u32;
pub const MOUSE_RIGHT_BUTTON_DOWN: u32 = 4u32;
pub const MOUSE_RIGHT_BUTTON_UP: u32 = 8u32;
pub const MOUSE_SERIAL_HARDWARE: u32 = 4u32;
pub const MOUSE_TERMSRV_SRC_SHADOW: u32 = 256u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MOUSE_UNIT_ID_PARAMETER {
    pub UnitId: u16,
}
pub const MOUSE_WHEEL: u32 = 1024u32;
pub type PFN_HidP_GetVersionInternal = Option<unsafe extern "system" fn(version: *mut u32) -> windows_core::NTSTATUS>;
pub type PHIDP_INSERT_SCANCODES = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, newscancodes: windows_core::PCSTR, length: u32) -> bool>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PHIDP_PREPARSED_DATA(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USAGE_AND_PAGE {
    pub Usage: u16,
    pub UsagePage: u16,
}
pub const WHEELMOUSE_HID_HARDWARE: u32 = 256u32;
pub const WHEELMOUSE_I8042_HARDWARE: u32 = 32u32;
pub const WHEELMOUSE_SERIAL_HARDWARE: u32 = 64u32;
