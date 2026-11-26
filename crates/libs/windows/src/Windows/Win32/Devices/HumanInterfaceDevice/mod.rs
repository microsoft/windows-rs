#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectInput8Create<P4>(hinst: super::super::Foundation::HINSTANCE, dwversion: u32, riidltf: *const windows_core::GUID, ppvout: *mut *mut core::ffi::c_void, punkouter: P4) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("dinput8.dll" "system" fn DirectInput8Create(hinst : super::super::Foundation::HINSTANCE, dwversion : u32, riidltf : *const windows_core::GUID, ppvout : *mut *mut core::ffi::c_void, punkouter : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DirectInput8Create(hinst, dwversion, riidltf, ppvout as _, punkouter.param().abi()).ok() }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_FlushQueue(hiddeviceobject: super::super::Foundation::HANDLE) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_FlushQueue(hiddeviceobject : super::super::Foundation::HANDLE) -> bool);
    unsafe { HidD_FlushQueue(hiddeviceobject) }
}
#[inline]
pub unsafe fn HidD_FreePreparsedData(preparseddata: PHIDP_PREPARSED_DATA) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_FreePreparsedData(preparseddata : PHIDP_PREPARSED_DATA) -> bool);
    unsafe { HidD_FreePreparsedData(preparseddata) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetAttributes(hiddeviceobject: super::super::Foundation::HANDLE, attributes: *mut HIDD_ATTRIBUTES) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetAttributes(hiddeviceobject : super::super::Foundation::HANDLE, attributes : *mut HIDD_ATTRIBUTES) -> bool);
    unsafe { HidD_GetAttributes(hiddeviceobject, attributes as _) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetConfiguration(hiddeviceobject: super::super::Foundation::HANDLE, configuration: *mut HIDD_CONFIGURATION, configurationlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetConfiguration(hiddeviceobject : super::super::Foundation::HANDLE, configuration : *mut HIDD_CONFIGURATION, configurationlength : u32) -> bool);
    unsafe { HidD_GetConfiguration(hiddeviceobject, configuration as _, configurationlength) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetFeature(hiddeviceobject: super::super::Foundation::HANDLE, reportbuffer: *mut core::ffi::c_void, reportbufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetFeature(hiddeviceobject : super::super::Foundation::HANDLE, reportbuffer : *mut core::ffi::c_void, reportbufferlength : u32) -> bool);
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
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetIndexedString(hiddeviceobject: super::super::Foundation::HANDLE, stringindex: u32, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetIndexedString(hiddeviceobject : super::super::Foundation::HANDLE, stringindex : u32, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetIndexedString(hiddeviceobject, stringindex, buffer as _, bufferlength) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetInputReport(hiddeviceobject: super::super::Foundation::HANDLE, reportbuffer: *mut core::ffi::c_void, reportbufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetInputReport(hiddeviceobject : super::super::Foundation::HANDLE, reportbuffer : *mut core::ffi::c_void, reportbufferlength : u32) -> bool);
    unsafe { HidD_GetInputReport(hiddeviceobject, reportbuffer as _, reportbufferlength) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetManufacturerString(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetManufacturerString(hiddeviceobject : super::super::Foundation::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetManufacturerString(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetMsGenreDescriptor(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetMsGenreDescriptor(hiddeviceobject : super::super::Foundation::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetMsGenreDescriptor(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetNumInputBuffers(hiddeviceobject: super::super::Foundation::HANDLE, numberbuffers: *mut u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetNumInputBuffers(hiddeviceobject : super::super::Foundation::HANDLE, numberbuffers : *mut u32) -> bool);
    unsafe { HidD_GetNumInputBuffers(hiddeviceobject, numberbuffers as _) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetPhysicalDescriptor(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetPhysicalDescriptor(hiddeviceobject : super::super::Foundation::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetPhysicalDescriptor(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetPreparsedData(hiddeviceobject: super::super::Foundation::HANDLE, preparseddata: *mut PHIDP_PREPARSED_DATA) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetPreparsedData(hiddeviceobject : super::super::Foundation::HANDLE, preparseddata : *mut PHIDP_PREPARSED_DATA) -> bool);
    unsafe { HidD_GetPreparsedData(hiddeviceobject, preparseddata as _) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetProductString(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetProductString(hiddeviceobject : super::super::Foundation::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetProductString(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_GetSerialNumberString(hiddeviceobject: super::super::Foundation::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetSerialNumberString(hiddeviceobject : super::super::Foundation::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetSerialNumberString(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_SetConfiguration(hiddeviceobject: super::super::Foundation::HANDLE, configuration: *const HIDD_CONFIGURATION, configurationlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_SetConfiguration(hiddeviceobject : super::super::Foundation::HANDLE, configuration : *const HIDD_CONFIGURATION, configurationlength : u32) -> bool);
    unsafe { HidD_SetConfiguration(hiddeviceobject, configuration, configurationlength) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_SetFeature(hiddeviceobject: super::super::Foundation::HANDLE, reportbuffer: *const core::ffi::c_void, reportbufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_SetFeature(hiddeviceobject : super::super::Foundation::HANDLE, reportbuffer : *const core::ffi::c_void, reportbufferlength : u32) -> bool);
    unsafe { HidD_SetFeature(hiddeviceobject, reportbuffer, reportbufferlength) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_SetNumInputBuffers(hiddeviceobject: super::super::Foundation::HANDLE, numberbuffers: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_SetNumInputBuffers(hiddeviceobject : super::super::Foundation::HANDLE, numberbuffers : u32) -> bool);
    unsafe { HidD_SetNumInputBuffers(hiddeviceobject, numberbuffers) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidD_SetOutputReport(hiddeviceobject: super::super::Foundation::HANDLE, reportbuffer: *const core::ffi::c_void, reportbufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_SetOutputReport(hiddeviceobject : super::super::Foundation::HANDLE, reportbuffer : *const core::ffi::c_void, reportbufferlength : u32) -> bool);
    unsafe { HidD_SetOutputReport(hiddeviceobject, reportbuffer, reportbufferlength) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetButtonArray(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, buttondata: *mut HIDP_BUTTON_ARRAY_DATA, buttondatalength: *mut u16, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, buttondata : *mut HIDP_BUTTON_ARRAY_DATA, buttondatalength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetButtonArray(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, buttondata as _, buttondatalength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetButtonCaps(reporttype: HIDP_REPORT_TYPE, buttoncaps: *mut HIDP_BUTTON_CAPS, buttoncapslength: *mut u16, preparseddata: PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetButtonCaps(reporttype : HIDP_REPORT_TYPE, buttoncaps : *mut HIDP_BUTTON_CAPS, buttoncapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetButtonCaps(reporttype, buttoncaps as _, buttoncapslength as _, preparseddata) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetCaps(preparseddata: PHIDP_PREPARSED_DATA, capabilities: *mut HIDP_CAPS) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetCaps(preparseddata : PHIDP_PREPARSED_DATA, capabilities : *mut HIDP_CAPS) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetCaps(preparseddata, capabilities as _) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetData(reporttype: HIDP_REPORT_TYPE, datalist: *mut HIDP_DATA, datalength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetData(reporttype : HIDP_REPORT_TYPE, datalist : *mut HIDP_DATA, datalength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetData(reporttype, datalist as _, datalength as _, preparseddata, core::mem::transmute(report.as_mut_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetExtendedAttributes(reporttype: HIDP_REPORT_TYPE, dataindex: u16, preparseddata: PHIDP_PREPARSED_DATA, attributes: *mut HIDP_EXTENDED_ATTRIBUTES, lengthattributes: *mut u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetExtendedAttributes(reporttype : HIDP_REPORT_TYPE, dataindex : u16, preparseddata : PHIDP_PREPARSED_DATA, attributes : *mut HIDP_EXTENDED_ATTRIBUTES, lengthattributes : *mut u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetExtendedAttributes(reporttype, dataindex, preparseddata, attributes as _, lengthattributes as _) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetLinkCollectionNodes(linkcollectionnodes: *mut HIDP_LINK_COLLECTION_NODE, linkcollectionnodeslength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetLinkCollectionNodes(linkcollectionnodes : *mut HIDP_LINK_COLLECTION_NODE, linkcollectionnodeslength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetLinkCollectionNodes(linkcollectionnodes as _, linkcollectionnodeslength as _, preparseddata) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetScaledUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, usagevalue: *mut i32, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : *mut i32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetScaledUsageValue(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, usagevalue as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetSpecificButtonCaps(reporttype: HIDP_REPORT_TYPE, usagepage: Option<u16>, linkcollection: Option<u16>, usage: Option<u16>, buttoncaps: *mut HIDP_BUTTON_CAPS, buttoncapslength: *mut u16, preparseddata: PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetSpecificButtonCaps(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, buttoncaps : *mut HIDP_BUTTON_CAPS, buttoncapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetSpecificButtonCaps(reporttype, usagepage.unwrap_or(core::mem::zeroed()) as _, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage.unwrap_or(core::mem::zeroed()) as _, buttoncaps as _, buttoncapslength as _, preparseddata) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetSpecificValueCaps(reporttype: HIDP_REPORT_TYPE, usagepage: Option<u16>, linkcollection: Option<u16>, usage: Option<u16>, valuecaps: *mut HIDP_VALUE_CAPS, valuecapslength: *mut u16, preparseddata: PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetSpecificValueCaps(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, valuecaps : *mut HIDP_VALUE_CAPS, valuecapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetSpecificValueCaps(reporttype, usagepage.unwrap_or(core::mem::zeroed()) as _, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage.unwrap_or(core::mem::zeroed()) as _, valuecaps as _, valuecapslength as _, preparseddata) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, usagevalue: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetUsageValue(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, usagevalue as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetUsageValueArray(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, usagevalue: &mut [u8], preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : windows_core::PSTR, usagevaluebytelength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetUsageValueArray(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, core::mem::transmute(usagevalue.as_mut_ptr()), usagevalue.len().try_into().unwrap(), preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetUsages(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usagelist: *mut u16, usagelength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetUsages(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usagelist as _, usagelength as _, preparseddata, core::mem::transmute(report.as_mut_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetUsagesEx(reporttype: HIDP_REPORT_TYPE, linkcollection: Option<u16>, buttonlist: *mut USAGE_AND_PAGE, usagelength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetUsagesEx(reporttype : HIDP_REPORT_TYPE, linkcollection : u16, buttonlist : *mut USAGE_AND_PAGE, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetUsagesEx(reporttype, linkcollection.unwrap_or(core::mem::zeroed()) as _, buttonlist as _, usagelength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_GetValueCaps(reporttype: HIDP_REPORT_TYPE, valuecaps: *mut HIDP_VALUE_CAPS, valuecapslength: *mut u16, preparseddata: PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetValueCaps(reporttype : HIDP_REPORT_TYPE, valuecaps : *mut HIDP_VALUE_CAPS, valuecapslength : *mut u16, preparseddata : PHIDP_PREPARSED_DATA) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_GetValueCaps(reporttype, valuecaps as _, valuecapslength as _, preparseddata) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_InitializeReportForID(reporttype: HIDP_REPORT_TYPE, reportid: u8, preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_InitializeReportForID(reporttype : HIDP_REPORT_TYPE, reportid : u8, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_InitializeReportForID(reporttype, reportid, preparseddata, core::mem::transmute(report.as_mut_ptr()), report.len().try_into().unwrap()) }
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
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_SetButtonArray(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, buttondata: &[HIDP_BUTTON_ARRAY_DATA], preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, buttondata : *const HIDP_BUTTON_ARRAY_DATA, buttondatalength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_SetButtonArray(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, core::mem::transmute(buttondata.as_ptr()), buttondata.len().try_into().unwrap(), preparseddata, core::mem::transmute(report.as_mut_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_SetData(reporttype: HIDP_REPORT_TYPE, datalist: *mut HIDP_DATA, datalength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetData(reporttype : HIDP_REPORT_TYPE, datalist : *mut HIDP_DATA, datalength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_SetData(reporttype, datalist as _, datalength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_SetScaledUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, usagevalue: i32, preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : i32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_SetScaledUsageValue(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, usagevalue, preparseddata, core::mem::transmute(report.as_mut_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_SetUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, usagevalue: u32, preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_SetUsageValue(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, usagevalue, preparseddata, core::mem::transmute(report.as_mut_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_SetUsageValueArray(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usage: u16, usagevalue: &[u8], preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usage : u16, usagevalue : windows_core::PCSTR, usagevaluebytelength : u16, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_SetUsageValueArray(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, core::mem::transmute(usagevalue.as_ptr()), usagevalue.len().try_into().unwrap(), preparseddata, core::mem::transmute(report.as_mut_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_SetUsages(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usagelist: *mut u16, usagelength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_SetUsages(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usagelist as _, usagelength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_TranslateUsagesToI8042ScanCodes(changedusagelist: &[u16], keyaction: HIDP_KEYBOARD_DIRECTION, modifierstate: *mut HIDP_KEYBOARD_MODIFIER_STATE, insertcodesprocedure: PHIDP_INSERT_SCANCODES, insertcodescontext: Option<*const core::ffi::c_void>) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_TranslateUsagesToI8042ScanCodes(changedusagelist : *const u16, usagelistlength : u32, keyaction : HIDP_KEYBOARD_DIRECTION, modifierstate : *mut HIDP_KEYBOARD_MODIFIER_STATE, insertcodesprocedure : PHIDP_INSERT_SCANCODES, insertcodescontext : *const core::ffi::c_void) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_TranslateUsagesToI8042ScanCodes(core::mem::transmute(changedusagelist.as_ptr()), changedusagelist.len().try_into().unwrap(), keyaction, modifierstate as _, insertcodesprocedure, insertcodescontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_UnsetUsages(reporttype: HIDP_REPORT_TYPE, usagepage: u16, linkcollection: Option<u16>, usagelist: *mut u16, usagelength: *mut u32, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_UnsetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : u16, linkcollection : u16, usagelist : *mut u16, usagelength : *mut u32, preparseddata : PHIDP_PREPARSED_DATA, report : windows_core::PCSTR, reportlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_UnsetUsages(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usagelist as _, usagelength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HidP_UsageListDifference(previoususagelist: *const u16, currentusagelist: *const u16, breakusagelist: *mut u16, makeusagelist: *mut u16, usagelistlength: u32) -> super::super::Foundation::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_UsageListDifference(previoususagelist : *const u16, currentusagelist : *const u16, breakusagelist : *mut u16, makeusagelist : *mut u16, usagelistlength : u32) -> super::super::Foundation::NTSTATUS);
    unsafe { HidP_UsageListDifference(previoususagelist, currentusagelist, breakusagelist as _, makeusagelist as _, usagelistlength) }
}
#[inline]
pub unsafe fn joyConfigChanged(dwflags: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn joyConfigChanged(dwflags : u32) -> u32);
    unsafe { joyConfigChanged(dwflags) }
}
pub const BALLPOINT_I8042_HARDWARE: u32 = 8;
pub const BALLPOINT_SERIAL_HARDWARE: u32 = 16;
pub const BUTTON_BIT_ALLBUTTONSMASK: u32 = 16383;
pub const BUTTON_BIT_BACK: u32 = 32;
pub const BUTTON_BIT_CAMERAFOCUS: u32 = 128;
pub const BUTTON_BIT_CAMERALENS: u32 = 4096;
pub const BUTTON_BIT_CAMERASHUTTER: u32 = 256;
pub const BUTTON_BIT_HEADSET: u32 = 1024;
pub const BUTTON_BIT_HWKBDEPLOY: u32 = 2048;
pub const BUTTON_BIT_OEMCUSTOM: u32 = 8192;
pub const BUTTON_BIT_OEMCUSTOM2: u32 = 16384;
pub const BUTTON_BIT_OEMCUSTOM3: u32 = 32768;
pub const BUTTON_BIT_POWER: u32 = 1;
pub const BUTTON_BIT_RINGERTOGGLE: u32 = 512;
pub const BUTTON_BIT_ROTATION_LOCK: u32 = 16;
pub const BUTTON_BIT_SEARCH: u32 = 64;
pub const BUTTON_BIT_VOLUMEDOWN: u32 = 8;
pub const BUTTON_BIT_VOLUMEUP: u32 = 4;
pub const BUTTON_BIT_WINDOWS: u32 = 2;
pub const CLSID_DirectInput: windows_core::GUID = windows_core::GUID::from_u128(0x25e609e0_b259_11cf_bfc7_444553540000);
pub const CLSID_DirectInput8: windows_core::GUID = windows_core::GUID::from_u128(0x25e609e4_b259_11cf_bfc7_444553540000);
pub const CLSID_DirectInputDevice: windows_core::GUID = windows_core::GUID::from_u128(0x25e609e1_b259_11cf_bfc7_444553540000);
pub const CLSID_DirectInputDevice8: windows_core::GUID = windows_core::GUID::from_u128(0x25e609e5_b259_11cf_bfc7_444553540000);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CPOINT {
    pub lP: i32,
    pub dwLog: u32,
}
pub const DD_KEYBOARD_DEVICE_NAME: windows_core::PCSTR = windows_core::s!("\\Device\\KeyboardClass");
pub const DD_KEYBOARD_DEVICE_NAME_U: windows_core::PCWSTR = windows_core::w!("\\Device\\KeyboardClass");
pub const DD_MOUSE_DEVICE_NAME: windows_core::PCSTR = windows_core::s!("\\Device\\PointerClass");
pub const DD_MOUSE_DEVICE_NAME_U: windows_core::PCWSTR = windows_core::w!("\\Device\\PointerClass");
#[cfg(feature = "Win32_Foundation")]
pub const DEVPKEY_DeviceInterface_HID_BackgroundAccess: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 8 };
#[cfg(feature = "Win32_Foundation")]
pub const DEVPKEY_DeviceInterface_HID_IsReadOnly: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 4 };
#[cfg(feature = "Win32_Foundation")]
pub const DEVPKEY_DeviceInterface_HID_ProductId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 6 };
#[cfg(feature = "Win32_Foundation")]
pub const DEVPKEY_DeviceInterface_HID_UsageId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 3 };
#[cfg(feature = "Win32_Foundation")]
pub const DEVPKEY_DeviceInterface_HID_UsagePage: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 2 };
#[cfg(feature = "Win32_Foundation")]
pub const DEVPKEY_DeviceInterface_HID_VendorId: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 5 };
#[cfg(feature = "Win32_Foundation")]
pub const DEVPKEY_DeviceInterface_HID_VersionNumber: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 7 };
#[cfg(feature = "Win32_Foundation")]
pub const DEVPKEY_DeviceInterface_HID_WakeScreenOnInputCapable: super::super::Foundation::DEVPROPKEY = super::super::Foundation::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0xcbf38310_4a17_4310_a1eb_247f0b67593b), pid: 9 };
pub const DI8DEVCLASS_ALL: u32 = 0;
pub const DI8DEVCLASS_DEVICE: u32 = 1;
pub const DI8DEVCLASS_GAMECTRL: u32 = 4;
pub const DI8DEVCLASS_KEYBOARD: u32 = 3;
pub const DI8DEVCLASS_POINTER: u32 = 2;
pub const DI8DEVTYPE1STPERSON_LIMITED: u32 = 1;
pub const DI8DEVTYPE1STPERSON_SHOOTER: u32 = 4;
pub const DI8DEVTYPE1STPERSON_SIXDOF: u32 = 3;
pub const DI8DEVTYPE1STPERSON_UNKNOWN: u32 = 2;
pub const DI8DEVTYPEDEVICECTRL_COMMSSELECTION: u32 = 3;
pub const DI8DEVTYPEDEVICECTRL_COMMSSELECTION_HARDWIRED: u32 = 4;
pub const DI8DEVTYPEDEVICECTRL_UNKNOWN: u32 = 2;
pub const DI8DEVTYPEDRIVING_COMBINEDPEDALS: u32 = 2;
pub const DI8DEVTYPEDRIVING_DUALPEDALS: u32 = 3;
pub const DI8DEVTYPEDRIVING_HANDHELD: u32 = 5;
pub const DI8DEVTYPEDRIVING_LIMITED: u32 = 1;
pub const DI8DEVTYPEDRIVING_THREEPEDALS: u32 = 4;
pub const DI8DEVTYPEFLIGHT_LIMITED: u32 = 1;
pub const DI8DEVTYPEFLIGHT_RC: u32 = 4;
pub const DI8DEVTYPEFLIGHT_STICK: u32 = 2;
pub const DI8DEVTYPEFLIGHT_YOKE: u32 = 3;
pub const DI8DEVTYPEGAMEPAD_LIMITED: u32 = 1;
pub const DI8DEVTYPEGAMEPAD_STANDARD: u32 = 2;
pub const DI8DEVTYPEGAMEPAD_TILT: u32 = 3;
pub const DI8DEVTYPEJOYSTICK_LIMITED: u32 = 1;
pub const DI8DEVTYPEJOYSTICK_STANDARD: u32 = 2;
pub const DI8DEVTYPEKEYBOARD_J3100: u32 = 12;
pub const DI8DEVTYPEKEYBOARD_JAPAN106: u32 = 10;
pub const DI8DEVTYPEKEYBOARD_JAPANAX: u32 = 11;
pub const DI8DEVTYPEKEYBOARD_NEC98: u32 = 7;
pub const DI8DEVTYPEKEYBOARD_NEC98106: u32 = 9;
pub const DI8DEVTYPEKEYBOARD_NEC98LAPTOP: u32 = 8;
pub const DI8DEVTYPEKEYBOARD_NOKIA1050: u32 = 5;
pub const DI8DEVTYPEKEYBOARD_NOKIA9140: u32 = 6;
pub const DI8DEVTYPEKEYBOARD_OLIVETTI: u32 = 2;
pub const DI8DEVTYPEKEYBOARD_PCAT: u32 = 3;
pub const DI8DEVTYPEKEYBOARD_PCENH: u32 = 4;
pub const DI8DEVTYPEKEYBOARD_PCXT: u32 = 1;
pub const DI8DEVTYPEKEYBOARD_UNKNOWN: u32 = 0;
pub const DI8DEVTYPEMOUSE_ABSOLUTE: u32 = 6;
pub const DI8DEVTYPEMOUSE_FINGERSTICK: u32 = 3;
pub const DI8DEVTYPEMOUSE_TOUCHPAD: u32 = 4;
pub const DI8DEVTYPEMOUSE_TRACKBALL: u32 = 5;
pub const DI8DEVTYPEMOUSE_TRADITIONAL: u32 = 2;
pub const DI8DEVTYPEMOUSE_UNKNOWN: u32 = 1;
pub const DI8DEVTYPEREMOTE_UNKNOWN: u32 = 2;
pub const DI8DEVTYPESCREENPTR_LIGHTGUN: u32 = 3;
pub const DI8DEVTYPESCREENPTR_LIGHTPEN: u32 = 4;
pub const DI8DEVTYPESCREENPTR_TOUCH: u32 = 5;
pub const DI8DEVTYPESCREENPTR_UNKNOWN: u32 = 2;
pub const DI8DEVTYPESUPPLEMENTAL_2NDHANDCONTROLLER: u32 = 3;
pub const DI8DEVTYPESUPPLEMENTAL_COMBINEDPEDALS: u32 = 10;
pub const DI8DEVTYPESUPPLEMENTAL_DUALPEDALS: u32 = 11;
pub const DI8DEVTYPESUPPLEMENTAL_HANDTRACKER: u32 = 5;
pub const DI8DEVTYPESUPPLEMENTAL_HEADTRACKER: u32 = 4;
pub const DI8DEVTYPESUPPLEMENTAL_RUDDERPEDALS: u32 = 13;
pub const DI8DEVTYPESUPPLEMENTAL_SHIFTER: u32 = 7;
pub const DI8DEVTYPESUPPLEMENTAL_SHIFTSTICKGATE: u32 = 6;
pub const DI8DEVTYPESUPPLEMENTAL_SPLITTHROTTLE: u32 = 9;
pub const DI8DEVTYPESUPPLEMENTAL_THREEPEDALS: u32 = 12;
pub const DI8DEVTYPESUPPLEMENTAL_THROTTLE: u32 = 8;
pub const DI8DEVTYPESUPPLEMENTAL_UNKNOWN: u32 = 2;
pub const DI8DEVTYPE_1STPERSON: u32 = 24;
pub const DI8DEVTYPE_DEVICE: u32 = 17;
pub const DI8DEVTYPE_DEVICECTRL: u32 = 25;
pub const DI8DEVTYPE_DRIVING: u32 = 22;
pub const DI8DEVTYPE_FLIGHT: u32 = 23;
pub const DI8DEVTYPE_GAMEPAD: u32 = 21;
pub const DI8DEVTYPE_JOYSTICK: u32 = 20;
pub const DI8DEVTYPE_KEYBOARD: u32 = 19;
pub const DI8DEVTYPE_LIMITEDGAMESUBTYPE: u32 = 1;
pub const DI8DEVTYPE_MOUSE: u32 = 18;
pub const DI8DEVTYPE_REMOTE: u32 = 27;
pub const DI8DEVTYPE_SCREENPOINTER: u32 = 26;
pub const DI8DEVTYPE_SUPPLEMENTAL: u32 = 28;
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
#[cfg(feature = "Win32_Foundation")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Foundation")]
impl Default for DIACTIONFORMATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Foundation")]
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
pub const DIAFTS_NEWDEVICEHIGH: u32 = 4294967295;
pub const DIAFTS_NEWDEVICELOW: u32 = 4294967295;
pub const DIAFTS_UNUSEDDEVICEHIGH: u32 = 0;
pub const DIAFTS_UNUSEDDEVICELOW: u32 = 0;
pub const DIAH_APPREQUESTED: u32 = 2;
pub const DIAH_DEFAULT: u32 = 32;
pub const DIAH_ERROR: u32 = 2147483648;
pub const DIAH_HWAPP: u32 = 4;
pub const DIAH_HWDEFAULT: u32 = 8;
pub const DIAH_UNMAPPED: u32 = 0;
pub const DIAH_USERCONFIG: u32 = 1;
pub const DIAPPIDFLAG_NOSIZE: u32 = 2;
pub const DIAPPIDFLAG_NOTIME: u32 = 1;
pub const DIAXIS_2DCONTROL_INOUT: u32 = 587301379;
pub const DIAXIS_2DCONTROL_LATERAL: u32 = 587235841;
pub const DIAXIS_2DCONTROL_MOVE: u32 = 587268610;
pub const DIAXIS_2DCONTROL_ROTATEZ: u32 = 587350532;
pub const DIAXIS_3DCONTROL_INOUT: u32 = 604078595;
pub const DIAXIS_3DCONTROL_LATERAL: u32 = 604013057;
pub const DIAXIS_3DCONTROL_MOVE: u32 = 604045826;
pub const DIAXIS_3DCONTROL_ROTATEX: u32 = 604193284;
pub const DIAXIS_3DCONTROL_ROTATEY: u32 = 604160517;
pub const DIAXIS_3DCONTROL_ROTATEZ: u32 = 604127750;
pub const DIAXIS_ANY_1: u32 = 4278206977;
pub const DIAXIS_ANY_2: u32 = 4278206978;
pub const DIAXIS_ANY_3: u32 = 4278206979;
pub const DIAXIS_ANY_4: u32 = 4278206980;
pub const DIAXIS_ANY_A_1: u32 = 4278436353;
pub const DIAXIS_ANY_A_2: u32 = 4278436354;
pub const DIAXIS_ANY_B_1: u32 = 4278469121;
pub const DIAXIS_ANY_B_2: u32 = 4278469122;
pub const DIAXIS_ANY_C_1: u32 = 4278501889;
pub const DIAXIS_ANY_C_2: u32 = 4278501890;
pub const DIAXIS_ANY_R_1: u32 = 4278338049;
pub const DIAXIS_ANY_R_2: u32 = 4278338050;
pub const DIAXIS_ANY_S_1: u32 = 4278534657;
pub const DIAXIS_ANY_S_2: u32 = 4278534658;
pub const DIAXIS_ANY_U_1: u32 = 4278370817;
pub const DIAXIS_ANY_U_2: u32 = 4278370818;
pub const DIAXIS_ANY_V_1: u32 = 4278403585;
pub const DIAXIS_ANY_V_2: u32 = 4278403586;
pub const DIAXIS_ANY_X_1: u32 = 4278239745;
pub const DIAXIS_ANY_X_2: u32 = 4278239746;
pub const DIAXIS_ANY_Y_1: u32 = 4278272513;
pub const DIAXIS_ANY_Y_2: u32 = 4278272514;
pub const DIAXIS_ANY_Z_1: u32 = 4278305281;
pub const DIAXIS_ANY_Z_2: u32 = 4278305282;
pub const DIAXIS_ARCADEP_LATERAL: u32 = 570458625;
pub const DIAXIS_ARCADEP_MOVE: u32 = 570491394;
pub const DIAXIS_ARCADES_LATERAL: u32 = 553681409;
pub const DIAXIS_ARCADES_MOVE: u32 = 553714178;
pub const DIAXIS_BASEBALLB_LATERAL: u32 = 251691521;
pub const DIAXIS_BASEBALLB_MOVE: u32 = 251724290;
pub const DIAXIS_BASEBALLF_LATERAL: u32 = 285245953;
pub const DIAXIS_BASEBALLF_MOVE: u32 = 285278722;
pub const DIAXIS_BASEBALLP_LATERAL: u32 = 268468737;
pub const DIAXIS_BASEBALLP_MOVE: u32 = 268501506;
pub const DIAXIS_BBALLD_LATERAL: u32 = 318800385;
pub const DIAXIS_BBALLD_MOVE: u32 = 318833154;
pub const DIAXIS_BBALLO_LATERAL: u32 = 302023169;
pub const DIAXIS_BBALLO_MOVE: u32 = 302055938;
pub const DIAXIS_BIKINGM_BRAKE: u32 = 470041091;
pub const DIAXIS_BIKINGM_PEDAL: u32 = 469828098;
pub const DIAXIS_BIKINGM_TURN: u32 = 469795329;
pub const DIAXIS_BROWSER_LATERAL: u32 = 671121921;
pub const DIAXIS_BROWSER_MOVE: u32 = 671154690;
pub const DIAXIS_BROWSER_VIEW: u32 = 671187459;
pub const DIAXIS_CADF_INOUT: u32 = 620855811;
pub const DIAXIS_CADF_LATERAL: u32 = 620790273;
pub const DIAXIS_CADF_MOVE: u32 = 620823042;
pub const DIAXIS_CADF_ROTATEX: u32 = 620970500;
pub const DIAXIS_CADF_ROTATEY: u32 = 620937733;
pub const DIAXIS_CADF_ROTATEZ: u32 = 620904966;
pub const DIAXIS_CADM_INOUT: u32 = 637633027;
pub const DIAXIS_CADM_LATERAL: u32 = 637567489;
pub const DIAXIS_CADM_MOVE: u32 = 637600258;
pub const DIAXIS_CADM_ROTATEX: u32 = 637747716;
pub const DIAXIS_CADM_ROTATEY: u32 = 637714949;
pub const DIAXIS_CADM_ROTATEZ: u32 = 637682182;
pub const DIAXIS_DRIVINGC_ACCELERATE: u32 = 33788418;
pub const DIAXIS_DRIVINGC_ACCEL_AND_BRAKE: u32 = 33638916;
pub const DIAXIS_DRIVINGC_BRAKE: u32 = 33821187;
pub const DIAXIS_DRIVINGC_STEER: u32 = 33589761;
pub const DIAXIS_DRIVINGR_ACCELERATE: u32 = 17011202;
pub const DIAXIS_DRIVINGR_ACCEL_AND_BRAKE: u32 = 16861700;
pub const DIAXIS_DRIVINGR_BRAKE: u32 = 17043971;
pub const DIAXIS_DRIVINGR_STEER: u32 = 16812545;
pub const DIAXIS_DRIVINGT_ACCELERATE: u32 = 50565635;
pub const DIAXIS_DRIVINGT_ACCEL_AND_BRAKE: u32 = 50416134;
pub const DIAXIS_DRIVINGT_BARREL: u32 = 50397698;
pub const DIAXIS_DRIVINGT_BRAKE: u32 = 50614789;
pub const DIAXIS_DRIVINGT_ROTATE: u32 = 50463236;
pub const DIAXIS_DRIVINGT_STEER: u32 = 50366977;
pub const DIAXIS_FIGHTINGH_LATERAL: u32 = 134251009;
pub const DIAXIS_FIGHTINGH_MOVE: u32 = 134283778;
pub const DIAXIS_FIGHTINGH_ROTATE: u32 = 134365699;
pub const DIAXIS_FISHING_LATERAL: u32 = 234914305;
pub const DIAXIS_FISHING_MOVE: u32 = 234947074;
pub const DIAXIS_FISHING_ROTATE: u32 = 235028995;
pub const DIAXIS_FLYINGC_BANK: u32 = 67144193;
pub const DIAXIS_FLYINGC_BRAKE: u32 = 67398148;
pub const DIAXIS_FLYINGC_FLAPS: u32 = 67459590;
pub const DIAXIS_FLYINGC_PITCH: u32 = 67176962;
pub const DIAXIS_FLYINGC_RUDDER: u32 = 67260933;
pub const DIAXIS_FLYINGC_THROTTLE: u32 = 67342851;
pub const DIAXIS_FLYINGH_BANK: u32 = 100698625;
pub const DIAXIS_FLYINGH_COLLECTIVE: u32 = 100764163;
pub const DIAXIS_FLYINGH_PITCH: u32 = 100731394;
pub const DIAXIS_FLYINGH_THROTTLE: u32 = 100915717;
pub const DIAXIS_FLYINGH_TORQUE: u32 = 100817412;
pub const DIAXIS_FLYINGM_BANK: u32 = 83921409;
pub const DIAXIS_FLYINGM_BRAKE: u32 = 84173317;
pub const DIAXIS_FLYINGM_FLAPS: u32 = 84234758;
pub const DIAXIS_FLYINGM_PITCH: u32 = 83954178;
pub const DIAXIS_FLYINGM_RUDDER: u32 = 84036100;
pub const DIAXIS_FLYINGM_THROTTLE: u32 = 84120067;
pub const DIAXIS_FOOTBALLD_LATERAL: u32 = 385909249;
pub const DIAXIS_FOOTBALLD_MOVE: u32 = 385942018;
pub const DIAXIS_FOOTBALLO_LATERAL: u32 = 369132033;
pub const DIAXIS_FOOTBALLO_MOVE: u32 = 369164802;
pub const DIAXIS_FOOTBALLQ_LATERAL: u32 = 352354817;
pub const DIAXIS_FOOTBALLQ_MOVE: u32 = 352387586;
pub const DIAXIS_FPS_LOOKUPDOWN: u32 = 151093763;
pub const DIAXIS_FPS_MOVE: u32 = 151060994;
pub const DIAXIS_FPS_ROTATE: u32 = 151028225;
pub const DIAXIS_FPS_SIDESTEP: u32 = 151142916;
pub const DIAXIS_GOLF_LATERAL: u32 = 402686465;
pub const DIAXIS_GOLF_MOVE: u32 = 402719234;
pub const DIAXIS_HOCKEYD_LATERAL: u32 = 436240897;
pub const DIAXIS_HOCKEYD_MOVE: u32 = 436273666;
pub const DIAXIS_HOCKEYG_LATERAL: u32 = 453018113;
pub const DIAXIS_HOCKEYG_MOVE: u32 = 453050882;
pub const DIAXIS_HOCKEYO_LATERAL: u32 = 419463681;
pub const DIAXIS_HOCKEYO_MOVE: u32 = 419496450;
pub const DIAXIS_HUNTING_LATERAL: u32 = 218137089;
pub const DIAXIS_HUNTING_MOVE: u32 = 218169858;
pub const DIAXIS_HUNTING_ROTATE: u32 = 218251779;
pub const DIAXIS_MECHA_ROTATE: u32 = 687997443;
pub const DIAXIS_MECHA_STEER: u32 = 687899137;
pub const DIAXIS_MECHA_THROTTLE: u32 = 688095748;
pub const DIAXIS_MECHA_TORSO: u32 = 687931906;
pub const DIAXIS_RACQUET_LATERAL: u32 = 536904193;
pub const DIAXIS_RACQUET_MOVE: u32 = 536936962;
pub const DIAXIS_REMOTE_SLIDER: u32 = 654639617;
pub const DIAXIS_REMOTE_SLIDER2: u32 = 654656002;
pub const DIAXIS_SKIING_SPEED: u32 = 486605314;
pub const DIAXIS_SKIING_TURN: u32 = 486572545;
pub const DIAXIS_SOCCERD_LATERAL: u32 = 520126977;
pub const DIAXIS_SOCCERD_MOVE: u32 = 520159746;
pub const DIAXIS_SOCCERO_BEND: u32 = 503415299;
pub const DIAXIS_SOCCERO_LATERAL: u32 = 503349761;
pub const DIAXIS_SOCCERO_MOVE: u32 = 503382530;
pub const DIAXIS_SPACESIM_CLIMB: u32 = 117555716;
pub const DIAXIS_SPACESIM_LATERAL: u32 = 117473793;
pub const DIAXIS_SPACESIM_MOVE: u32 = 117506562;
pub const DIAXIS_SPACESIM_ROTATE: u32 = 117588485;
pub const DIAXIS_SPACESIM_THROTTLE: u32 = 117670403;
pub const DIAXIS_STRATEGYR_LATERAL: u32 = 184582657;
pub const DIAXIS_STRATEGYR_MOVE: u32 = 184615426;
pub const DIAXIS_STRATEGYR_ROTATE: u32 = 184697347;
pub const DIAXIS_STRATEGYT_LATERAL: u32 = 201359873;
pub const DIAXIS_STRATEGYT_MOVE: u32 = 201392642;
pub const DIAXIS_TPS_MOVE: u32 = 167838210;
pub const DIAXIS_TPS_STEP: u32 = 167821827;
pub const DIAXIS_TPS_TURN: u32 = 167903745;
pub const DIA_APPFIXED: u32 = 16;
pub const DIA_APPMAPPED: u32 = 2;
pub const DIA_APPNOMAP: u32 = 4;
pub const DIA_FORCEFEEDBACK: u32 = 1;
pub const DIA_NORANGE: u32 = 8;
pub const DIBUTTON_2DCONTROL_DEVICE: u32 = 587220222;
pub const DIBUTTON_2DCONTROL_DISPLAY: u32 = 587219973;
pub const DIBUTTON_2DCONTROL_MENU: u32 = 587203837;
pub const DIBUTTON_2DCONTROL_PAUSE: u32 = 587220220;
pub const DIBUTTON_2DCONTROL_SELECT: u32 = 587203585;
pub const DIBUTTON_2DCONTROL_SPECIAL: u32 = 587203587;
pub const DIBUTTON_2DCONTROL_SPECIAL1: u32 = 587203586;
pub const DIBUTTON_2DCONTROL_SPECIAL2: u32 = 587203588;
pub const DIBUTTON_3DCONTROL_DEVICE: u32 = 603997438;
pub const DIBUTTON_3DCONTROL_DISPLAY: u32 = 603997189;
pub const DIBUTTON_3DCONTROL_MENU: u32 = 603981053;
pub const DIBUTTON_3DCONTROL_PAUSE: u32 = 603997436;
pub const DIBUTTON_3DCONTROL_SELECT: u32 = 603980801;
pub const DIBUTTON_3DCONTROL_SPECIAL: u32 = 603980803;
pub const DIBUTTON_3DCONTROL_SPECIAL1: u32 = 603980802;
pub const DIBUTTON_3DCONTROL_SPECIAL2: u32 = 603980804;
pub const DIBUTTON_ARCADEP_BACK_LINK: u32 = 570508520;
pub const DIBUTTON_ARCADEP_CROUCH: u32 = 570426371;
pub const DIBUTTON_ARCADEP_DEVICE: u32 = 570443006;
pub const DIBUTTON_ARCADEP_FIRE: u32 = 570426370;
pub const DIBUTTON_ARCADEP_FIRESECONDARY: u32 = 570442758;
pub const DIBUTTON_ARCADEP_FORWARD_LINK: u32 = 570508512;
pub const DIBUTTON_ARCADEP_JUMP: u32 = 570426369;
pub const DIBUTTON_ARCADEP_LEFT_LINK: u32 = 570475748;
pub const DIBUTTON_ARCADEP_MENU: u32 = 570426621;
pub const DIBUTTON_ARCADEP_PAUSE: u32 = 570443004;
pub const DIBUTTON_ARCADEP_RIGHT_LINK: u32 = 570475756;
pub const DIBUTTON_ARCADEP_SELECT: u32 = 570426373;
pub const DIBUTTON_ARCADEP_SPECIAL: u32 = 570426372;
pub const DIBUTTON_ARCADEP_VIEW_DOWN_LINK: u32 = 570934504;
pub const DIBUTTON_ARCADEP_VIEW_LEFT_LINK: u32 = 570934500;
pub const DIBUTTON_ARCADEP_VIEW_RIGHT_LINK: u32 = 570934508;
pub const DIBUTTON_ARCADEP_VIEW_UP_LINK: u32 = 570934496;
pub const DIBUTTON_ARCADES_ATTACK: u32 = 553649155;
pub const DIBUTTON_ARCADES_BACK_LINK: u32 = 553731304;
pub const DIBUTTON_ARCADES_CARRY: u32 = 553649154;
pub const DIBUTTON_ARCADES_DEVICE: u32 = 553665790;
pub const DIBUTTON_ARCADES_FORWARD_LINK: u32 = 553731296;
pub const DIBUTTON_ARCADES_LEFT_LINK: u32 = 553698532;
pub const DIBUTTON_ARCADES_MENU: u32 = 553649405;
pub const DIBUTTON_ARCADES_PAUSE: u32 = 553665788;
pub const DIBUTTON_ARCADES_RIGHT_LINK: u32 = 553698540;
pub const DIBUTTON_ARCADES_SELECT: u32 = 553649157;
pub const DIBUTTON_ARCADES_SPECIAL: u32 = 553649156;
pub const DIBUTTON_ARCADES_THROW: u32 = 553649153;
pub const DIBUTTON_ARCADES_VIEW_DOWN_LINK: u32 = 554157288;
pub const DIBUTTON_ARCADES_VIEW_LEFT_LINK: u32 = 554157284;
pub const DIBUTTON_ARCADES_VIEW_RIGHT_LINK: u32 = 554157292;
pub const DIBUTTON_ARCADES_VIEW_UP_LINK: u32 = 554157280;
pub const DIBUTTON_BASEBALLB_BACK_LINK: u32 = 251741416;
pub const DIBUTTON_BASEBALLB_BOX: u32 = 251675658;
pub const DIBUTTON_BASEBALLB_BUNT: u32 = 251659268;
pub const DIBUTTON_BASEBALLB_BURST: u32 = 251659270;
pub const DIBUTTON_BASEBALLB_CONTACT: u32 = 251659272;
pub const DIBUTTON_BASEBALLB_DEVICE: u32 = 251675902;
pub const DIBUTTON_BASEBALLB_FORWARD_LINK: u32 = 251741408;
pub const DIBUTTON_BASEBALLB_LEFT_LINK: u32 = 251708644;
pub const DIBUTTON_BASEBALLB_MENU: u32 = 251659517;
pub const DIBUTTON_BASEBALLB_NORMAL: u32 = 251659266;
pub const DIBUTTON_BASEBALLB_NOSTEAL: u32 = 251675657;
pub const DIBUTTON_BASEBALLB_PAUSE: u32 = 251675900;
pub const DIBUTTON_BASEBALLB_POWER: u32 = 251659267;
pub const DIBUTTON_BASEBALLB_RIGHT_LINK: u32 = 251708652;
pub const DIBUTTON_BASEBALLB_SELECT: u32 = 251659265;
pub const DIBUTTON_BASEBALLB_SLIDE: u32 = 251659271;
pub const DIBUTTON_BASEBALLB_STEAL: u32 = 251659269;
pub const DIBUTTON_BASEBALLF_AIM_LEFT_LINK: u32 = 285263076;
pub const DIBUTTON_BASEBALLF_AIM_RIGHT_LINK: u32 = 285263084;
pub const DIBUTTON_BASEBALLF_BACK_LINK: u32 = 285295848;
pub const DIBUTTON_BASEBALLF_BURST: u32 = 285213700;
pub const DIBUTTON_BASEBALLF_DEVICE: u32 = 285230334;
pub const DIBUTTON_BASEBALLF_DIVE: u32 = 285213702;
pub const DIBUTTON_BASEBALLF_FORWARD_LINK: u32 = 285295840;
pub const DIBUTTON_BASEBALLF_JUMP: u32 = 285213701;
pub const DIBUTTON_BASEBALLF_MENU: u32 = 285213949;
pub const DIBUTTON_BASEBALLF_NEAREST: u32 = 285213697;
pub const DIBUTTON_BASEBALLF_PAUSE: u32 = 285230332;
pub const DIBUTTON_BASEBALLF_SHIFTIN: u32 = 285230087;
pub const DIBUTTON_BASEBALLF_SHIFTOUT: u32 = 285230088;
pub const DIBUTTON_BASEBALLF_THROW1: u32 = 285213698;
pub const DIBUTTON_BASEBALLF_THROW2: u32 = 285213699;
pub const DIBUTTON_BASEBALLP_BACK_LINK: u32 = 268518632;
pub const DIBUTTON_BASEBALLP_BASE: u32 = 268436483;
pub const DIBUTTON_BASEBALLP_DEVICE: u32 = 268453118;
pub const DIBUTTON_BASEBALLP_FAKE: u32 = 268436485;
pub const DIBUTTON_BASEBALLP_FORWARD_LINK: u32 = 268518624;
pub const DIBUTTON_BASEBALLP_LEFT_LINK: u32 = 268485860;
pub const DIBUTTON_BASEBALLP_LOOK: u32 = 268452871;
pub const DIBUTTON_BASEBALLP_MENU: u32 = 268436733;
pub const DIBUTTON_BASEBALLP_PAUSE: u32 = 268453116;
pub const DIBUTTON_BASEBALLP_PITCH: u32 = 268436482;
pub const DIBUTTON_BASEBALLP_RIGHT_LINK: u32 = 268485868;
pub const DIBUTTON_BASEBALLP_SELECT: u32 = 268436481;
pub const DIBUTTON_BASEBALLP_THROW: u32 = 268436484;
pub const DIBUTTON_BASEBALLP_WALK: u32 = 268452870;
pub const DIBUTTON_BBALLD_BACK_LINK: u32 = 318850280;
pub const DIBUTTON_BBALLD_BURST: u32 = 318768134;
pub const DIBUTTON_BBALLD_DEVICE: u32 = 318784766;
pub const DIBUTTON_BBALLD_FAKE: u32 = 318768131;
pub const DIBUTTON_BBALLD_FORWARD_LINK: u32 = 318850272;
pub const DIBUTTON_BBALLD_JUMP: u32 = 318768129;
pub const DIBUTTON_BBALLD_LEFT_LINK: u32 = 318817508;
pub const DIBUTTON_BBALLD_MENU: u32 = 318768381;
pub const DIBUTTON_BBALLD_PAUSE: u32 = 318784764;
pub const DIBUTTON_BBALLD_PLAY: u32 = 318768135;
pub const DIBUTTON_BBALLD_PLAYER: u32 = 318768133;
pub const DIBUTTON_BBALLD_RIGHT_LINK: u32 = 318817516;
pub const DIBUTTON_BBALLD_SPECIAL: u32 = 318768132;
pub const DIBUTTON_BBALLD_STEAL: u32 = 318768130;
pub const DIBUTTON_BBALLD_SUBSTITUTE: u32 = 318784521;
pub const DIBUTTON_BBALLD_TIMEOUT: u32 = 318784520;
pub const DIBUTTON_BBALLO_BACK_LINK: u32 = 302073064;
pub const DIBUTTON_BBALLO_BURST: u32 = 301990919;
pub const DIBUTTON_BBALLO_CALL: u32 = 301990920;
pub const DIBUTTON_BBALLO_DEVICE: u32 = 302007550;
pub const DIBUTTON_BBALLO_DUNK: u32 = 301990914;
pub const DIBUTTON_BBALLO_FAKE: u32 = 301990916;
pub const DIBUTTON_BBALLO_FORWARD_LINK: u32 = 302073056;
pub const DIBUTTON_BBALLO_JAB: u32 = 302007307;
pub const DIBUTTON_BBALLO_LEFT_LINK: u32 = 302040292;
pub const DIBUTTON_BBALLO_MENU: u32 = 301991165;
pub const DIBUTTON_BBALLO_PASS: u32 = 301990915;
pub const DIBUTTON_BBALLO_PAUSE: u32 = 302007548;
pub const DIBUTTON_BBALLO_PLAY: u32 = 302007306;
pub const DIBUTTON_BBALLO_PLAYER: u32 = 301990918;
pub const DIBUTTON_BBALLO_POST: u32 = 302007308;
pub const DIBUTTON_BBALLO_RIGHT_LINK: u32 = 302040300;
pub const DIBUTTON_BBALLO_SCREEN: u32 = 302007305;
pub const DIBUTTON_BBALLO_SHOOT: u32 = 301990913;
pub const DIBUTTON_BBALLO_SPECIAL: u32 = 301990917;
pub const DIBUTTON_BBALLO_SUBSTITUTE: u32 = 302007310;
pub const DIBUTTON_BBALLO_TIMEOUT: u32 = 302007309;
pub const DIBUTTON_BIKINGM_BRAKE_BUTTON_LINK: u32 = 470041832;
pub const DIBUTTON_BIKINGM_CAMERA: u32 = 469763074;
pub const DIBUTTON_BIKINGM_DEVICE: u32 = 469779710;
pub const DIBUTTON_BIKINGM_FASTER_LINK: u32 = 469845216;
pub const DIBUTTON_BIKINGM_JUMP: u32 = 469763073;
pub const DIBUTTON_BIKINGM_LEFT_LINK: u32 = 469812452;
pub const DIBUTTON_BIKINGM_MENU: u32 = 469763325;
pub const DIBUTTON_BIKINGM_PAUSE: u32 = 469779708;
pub const DIBUTTON_BIKINGM_RIGHT_LINK: u32 = 469812460;
pub const DIBUTTON_BIKINGM_SELECT: u32 = 469763076;
pub const DIBUTTON_BIKINGM_SLOWER_LINK: u32 = 469845224;
pub const DIBUTTON_BIKINGM_SPECIAL1: u32 = 469763075;
pub const DIBUTTON_BIKINGM_SPECIAL2: u32 = 469763077;
pub const DIBUTTON_BIKINGM_ZOOM: u32 = 469779462;
pub const DIBUTTON_BROWSER_DEVICE: u32 = 671106302;
pub const DIBUTTON_BROWSER_FAVORITES: u32 = 671106054;
pub const DIBUTTON_BROWSER_HISTORY: u32 = 671106057;
pub const DIBUTTON_BROWSER_HOME: u32 = 671106053;
pub const DIBUTTON_BROWSER_MENU: u32 = 671089917;
pub const DIBUTTON_BROWSER_NEXT: u32 = 671106055;
pub const DIBUTTON_BROWSER_PAUSE: u32 = 671106300;
pub const DIBUTTON_BROWSER_PREVIOUS: u32 = 671106056;
pub const DIBUTTON_BROWSER_PRINT: u32 = 671106058;
pub const DIBUTTON_BROWSER_REFRESH: u32 = 671089666;
pub const DIBUTTON_BROWSER_SEARCH: u32 = 671106051;
pub const DIBUTTON_BROWSER_SELECT: u32 = 671089665;
pub const DIBUTTON_BROWSER_STOP: u32 = 671106052;
pub const DIBUTTON_CADF_DEVICE: u32 = 620774654;
pub const DIBUTTON_CADF_DISPLAY: u32 = 620774405;
pub const DIBUTTON_CADF_MENU: u32 = 620758269;
pub const DIBUTTON_CADF_PAUSE: u32 = 620774652;
pub const DIBUTTON_CADF_SELECT: u32 = 620758017;
pub const DIBUTTON_CADF_SPECIAL: u32 = 620758019;
pub const DIBUTTON_CADF_SPECIAL1: u32 = 620758018;
pub const DIBUTTON_CADF_SPECIAL2: u32 = 620758020;
pub const DIBUTTON_CADM_DEVICE: u32 = 637551870;
pub const DIBUTTON_CADM_DISPLAY: u32 = 637551621;
pub const DIBUTTON_CADM_MENU: u32 = 637535485;
pub const DIBUTTON_CADM_PAUSE: u32 = 637551868;
pub const DIBUTTON_CADM_SELECT: u32 = 637535233;
pub const DIBUTTON_CADM_SPECIAL: u32 = 637535235;
pub const DIBUTTON_CADM_SPECIAL1: u32 = 637535234;
pub const DIBUTTON_CADM_SPECIAL2: u32 = 637535236;
pub const DIBUTTON_DRIVINGC_ACCELERATE_LINK: u32 = 33805536;
pub const DIBUTTON_DRIVINGC_AIDS: u32 = 33571847;
pub const DIBUTTON_DRIVINGC_BRAKE: u32 = 33573896;
pub const DIBUTTON_DRIVINGC_DASHBOARD: u32 = 33571846;
pub const DIBUTTON_DRIVINGC_DEVICE: u32 = 33572094;
pub const DIBUTTON_DRIVINGC_FIRE: u32 = 33557505;
pub const DIBUTTON_DRIVINGC_FIRESECONDARY: u32 = 33573897;
pub const DIBUTTON_DRIVINGC_GLANCE_LEFT_LINK: u32 = 34063588;
pub const DIBUTTON_DRIVINGC_GLANCE_RIGHT_LINK: u32 = 34063596;
pub const DIBUTTON_DRIVINGC_MENU: u32 = 33555709;
pub const DIBUTTON_DRIVINGC_PAUSE: u32 = 33572092;
pub const DIBUTTON_DRIVINGC_SHIFTDOWN: u32 = 33573893;
pub const DIBUTTON_DRIVINGC_SHIFTUP: u32 = 33573892;
pub const DIBUTTON_DRIVINGC_STEER_LEFT_LINK: u32 = 33606884;
pub const DIBUTTON_DRIVINGC_STEER_RIGHT_LINK: u32 = 33606892;
pub const DIBUTTON_DRIVINGC_TARGET: u32 = 33557507;
pub const DIBUTTON_DRIVINGC_WEAPONS: u32 = 33557506;
pub const DIBUTTON_DRIVINGR_ACCELERATE_LINK: u32 = 17028320;
pub const DIBUTTON_DRIVINGR_AIDS: u32 = 16794630;
pub const DIBUTTON_DRIVINGR_BOOST: u32 = 16794632;
pub const DIBUTTON_DRIVINGR_BRAKE: u32 = 16796676;
pub const DIBUTTON_DRIVINGR_DASHBOARD: u32 = 16794629;
pub const DIBUTTON_DRIVINGR_DEVICE: u32 = 16794878;
pub const DIBUTTON_DRIVINGR_GLANCE_LEFT_LINK: u32 = 17286372;
pub const DIBUTTON_DRIVINGR_GLANCE_RIGHT_LINK: u32 = 17286380;
pub const DIBUTTON_DRIVINGR_MAP: u32 = 16794631;
pub const DIBUTTON_DRIVINGR_MENU: u32 = 16778493;
pub const DIBUTTON_DRIVINGR_PAUSE: u32 = 16794876;
pub const DIBUTTON_DRIVINGR_PIT: u32 = 16794633;
pub const DIBUTTON_DRIVINGR_SHIFTDOWN: u32 = 16780290;
pub const DIBUTTON_DRIVINGR_SHIFTUP: u32 = 16780289;
pub const DIBUTTON_DRIVINGR_STEER_LEFT_LINK: u32 = 16829668;
pub const DIBUTTON_DRIVINGR_STEER_RIGHT_LINK: u32 = 16829676;
pub const DIBUTTON_DRIVINGR_VIEW: u32 = 16784387;
pub const DIBUTTON_DRIVINGT_ACCELERATE_LINK: u32 = 50582752;
pub const DIBUTTON_DRIVINGT_BARREL_DOWN_LINK: u32 = 50414824;
pub const DIBUTTON_DRIVINGT_BARREL_UP_LINK: u32 = 50414816;
pub const DIBUTTON_DRIVINGT_BRAKE: u32 = 50351110;
pub const DIBUTTON_DRIVINGT_DASHBOARD: u32 = 50355205;
pub const DIBUTTON_DRIVINGT_DEVICE: u32 = 50349310;
pub const DIBUTTON_DRIVINGT_FIRE: u32 = 50334721;
pub const DIBUTTON_DRIVINGT_FIRESECONDARY: u32 = 50351111;
pub const DIBUTTON_DRIVINGT_GLANCE_LEFT_LINK: u32 = 50840804;
pub const DIBUTTON_DRIVINGT_GLANCE_RIGHT_LINK: u32 = 50840812;
pub const DIBUTTON_DRIVINGT_MENU: u32 = 50332925;
pub const DIBUTTON_DRIVINGT_PAUSE: u32 = 50349308;
pub const DIBUTTON_DRIVINGT_ROTATE_LEFT_LINK: u32 = 50480356;
pub const DIBUTTON_DRIVINGT_ROTATE_RIGHT_LINK: u32 = 50480364;
pub const DIBUTTON_DRIVINGT_STEER_LEFT_LINK: u32 = 50384100;
pub const DIBUTTON_DRIVINGT_STEER_RIGHT_LINK: u32 = 50384108;
pub const DIBUTTON_DRIVINGT_TARGET: u32 = 50334723;
pub const DIBUTTON_DRIVINGT_VIEW: u32 = 50355204;
pub const DIBUTTON_DRIVINGT_WEAPONS: u32 = 50334722;
pub const DIBUTTON_FIGHTINGH_BACKWARD_LINK: u32 = 134300904;
pub const DIBUTTON_FIGHTINGH_BLOCK: u32 = 134218755;
pub const DIBUTTON_FIGHTINGH_CROUCH: u32 = 134218756;
pub const DIBUTTON_FIGHTINGH_DEVICE: u32 = 134235390;
pub const DIBUTTON_FIGHTINGH_DISPLAY: u32 = 134235145;
pub const DIBUTTON_FIGHTINGH_DODGE: u32 = 134235146;
pub const DIBUTTON_FIGHTINGH_FORWARD_LINK: u32 = 134300896;
pub const DIBUTTON_FIGHTINGH_JUMP: u32 = 134218757;
pub const DIBUTTON_FIGHTINGH_KICK: u32 = 134218754;
pub const DIBUTTON_FIGHTINGH_LEFT_LINK: u32 = 134268132;
pub const DIBUTTON_FIGHTINGH_MENU: u32 = 134219005;
pub const DIBUTTON_FIGHTINGH_PAUSE: u32 = 134235388;
pub const DIBUTTON_FIGHTINGH_PUNCH: u32 = 134218753;
pub const DIBUTTON_FIGHTINGH_RIGHT_LINK: u32 = 134268140;
pub const DIBUTTON_FIGHTINGH_SELECT: u32 = 134235144;
pub const DIBUTTON_FIGHTINGH_SPECIAL1: u32 = 134218758;
pub const DIBUTTON_FIGHTINGH_SPECIAL2: u32 = 134218759;
pub const DIBUTTON_FISHING_BACK_LINK: u32 = 234964200;
pub const DIBUTTON_FISHING_BAIT: u32 = 234882052;
pub const DIBUTTON_FISHING_BINOCULAR: u32 = 234882051;
pub const DIBUTTON_FISHING_CAST: u32 = 234882049;
pub const DIBUTTON_FISHING_CROUCH: u32 = 234898439;
pub const DIBUTTON_FISHING_DEVICE: u32 = 234898686;
pub const DIBUTTON_FISHING_DISPLAY: u32 = 234898438;
pub const DIBUTTON_FISHING_FORWARD_LINK: u32 = 234964192;
pub const DIBUTTON_FISHING_JUMP: u32 = 234898440;
pub const DIBUTTON_FISHING_LEFT_LINK: u32 = 234931428;
pub const DIBUTTON_FISHING_MAP: u32 = 234882053;
pub const DIBUTTON_FISHING_MENU: u32 = 234882301;
pub const DIBUTTON_FISHING_PAUSE: u32 = 234898684;
pub const DIBUTTON_FISHING_RIGHT_LINK: u32 = 234931436;
pub const DIBUTTON_FISHING_ROTATE_LEFT_LINK: u32 = 235029732;
pub const DIBUTTON_FISHING_ROTATE_RIGHT_LINK: u32 = 235029740;
pub const DIBUTTON_FISHING_TYPE: u32 = 234882050;
pub const DIBUTTON_FLYINGC_BRAKE_LINK: u32 = 67398880;
pub const DIBUTTON_FLYINGC_DEVICE: u32 = 67126526;
pub const DIBUTTON_FLYINGC_DISPLAY: u32 = 67118082;
pub const DIBUTTON_FLYINGC_FASTER_LINK: u32 = 67359968;
pub const DIBUTTON_FLYINGC_FLAPSDOWN: u32 = 67134469;
pub const DIBUTTON_FLYINGC_FLAPSUP: u32 = 67134468;
pub const DIBUTTON_FLYINGC_GEAR: u32 = 67120131;
pub const DIBUTTON_FLYINGC_GLANCE_DOWN_LINK: u32 = 67618024;
pub const DIBUTTON_FLYINGC_GLANCE_LEFT_LINK: u32 = 67618020;
pub const DIBUTTON_FLYINGC_GLANCE_RIGHT_LINK: u32 = 67618028;
pub const DIBUTTON_FLYINGC_GLANCE_UP_LINK: u32 = 67618016;
pub const DIBUTTON_FLYINGC_MENU: u32 = 67110141;
pub const DIBUTTON_FLYINGC_PAUSE: u32 = 67126524;
pub const DIBUTTON_FLYINGC_SLOWER_LINK: u32 = 67359976;
pub const DIBUTTON_FLYINGC_VIEW: u32 = 67118081;
pub const DIBUTTON_FLYINGH_COUNTER: u32 = 100684804;
pub const DIBUTTON_FLYINGH_DEVICE: u32 = 100680958;
pub const DIBUTTON_FLYINGH_FASTER_LINK: u32 = 100916448;
pub const DIBUTTON_FLYINGH_FIRE: u32 = 100668417;
pub const DIBUTTON_FLYINGH_FIRESECONDARY: u32 = 100682759;
pub const DIBUTTON_FLYINGH_GEAR: u32 = 100688902;
pub const DIBUTTON_FLYINGH_GLANCE_DOWN_LINK: u32 = 101172456;
pub const DIBUTTON_FLYINGH_GLANCE_LEFT_LINK: u32 = 101172452;
pub const DIBUTTON_FLYINGH_GLANCE_RIGHT_LINK: u32 = 101172460;
pub const DIBUTTON_FLYINGH_GLANCE_UP_LINK: u32 = 101172448;
pub const DIBUTTON_FLYINGH_MENU: u32 = 100664573;
pub const DIBUTTON_FLYINGH_PAUSE: u32 = 100680956;
pub const DIBUTTON_FLYINGH_SLOWER_LINK: u32 = 100916456;
pub const DIBUTTON_FLYINGH_TARGET: u32 = 100668419;
pub const DIBUTTON_FLYINGH_VIEW: u32 = 100688901;
pub const DIBUTTON_FLYINGH_WEAPONS: u32 = 100668418;
pub const DIBUTTON_FLYINGM_BRAKE_LINK: u32 = 84174048;
pub const DIBUTTON_FLYINGM_COUNTER: u32 = 83909636;
pub const DIBUTTON_FLYINGM_DEVICE: u32 = 83903742;
pub const DIBUTTON_FLYINGM_DISPLAY: u32 = 83911686;
pub const DIBUTTON_FLYINGM_FASTER_LINK: u32 = 84137184;
pub const DIBUTTON_FLYINGM_FIRE: u32 = 83889153;
pub const DIBUTTON_FLYINGM_FIRESECONDARY: u32 = 83905545;
pub const DIBUTTON_FLYINGM_FLAPSDOWN: u32 = 83907592;
pub const DIBUTTON_FLYINGM_FLAPSUP: u32 = 83907591;
pub const DIBUTTON_FLYINGM_GEAR: u32 = 83911690;
pub const DIBUTTON_FLYINGM_GLANCE_DOWN_LINK: u32 = 84395240;
pub const DIBUTTON_FLYINGM_GLANCE_LEFT_LINK: u32 = 84395236;
pub const DIBUTTON_FLYINGM_GLANCE_RIGHT_LINK: u32 = 84395244;
pub const DIBUTTON_FLYINGM_GLANCE_UP_LINK: u32 = 84395232;
pub const DIBUTTON_FLYINGM_MENU: u32 = 83887357;
pub const DIBUTTON_FLYINGM_PAUSE: u32 = 83903740;
pub const DIBUTTON_FLYINGM_SLOWER_LINK: u32 = 84137192;
pub const DIBUTTON_FLYINGM_TARGET: u32 = 83889155;
pub const DIBUTTON_FLYINGM_VIEW: u32 = 83911685;
pub const DIBUTTON_FLYINGM_WEAPONS: u32 = 83889154;
pub const DIBUTTON_FOOTBALLD_AUDIBLE: u32 = 385893387;
pub const DIBUTTON_FOOTBALLD_BACK_LINK: u32 = 385959144;
pub const DIBUTTON_FOOTBALLD_BULLRUSH: u32 = 385893385;
pub const DIBUTTON_FOOTBALLD_DEVICE: u32 = 385893630;
pub const DIBUTTON_FOOTBALLD_FAKE: u32 = 385876997;
pub const DIBUTTON_FOOTBALLD_FORWARD_LINK: u32 = 385959136;
pub const DIBUTTON_FOOTBALLD_JUMP: u32 = 385876995;
pub const DIBUTTON_FOOTBALLD_LEFT_LINK: u32 = 385926372;
pub const DIBUTTON_FOOTBALLD_MENU: u32 = 385877245;
pub const DIBUTTON_FOOTBALLD_PAUSE: u32 = 385893628;
pub const DIBUTTON_FOOTBALLD_PLAY: u32 = 385876993;
pub const DIBUTTON_FOOTBALLD_RIGHT_LINK: u32 = 385926380;
pub const DIBUTTON_FOOTBALLD_RIP: u32 = 385893386;
pub const DIBUTTON_FOOTBALLD_SELECT: u32 = 385876994;
pub const DIBUTTON_FOOTBALLD_SPIN: u32 = 385893383;
pub const DIBUTTON_FOOTBALLD_SUBSTITUTE: u32 = 385893389;
pub const DIBUTTON_FOOTBALLD_SUPERTACKLE: u32 = 385876998;
pub const DIBUTTON_FOOTBALLD_SWIM: u32 = 385893384;
pub const DIBUTTON_FOOTBALLD_TACKLE: u32 = 385876996;
pub const DIBUTTON_FOOTBALLD_ZOOM: u32 = 385893388;
pub const DIBUTTON_FOOTBALLO_BACK_LINK: u32 = 369181928;
pub const DIBUTTON_FOOTBALLO_DEVICE: u32 = 369116414;
pub const DIBUTTON_FOOTBALLO_DIVE: u32 = 369116169;
pub const DIBUTTON_FOOTBALLO_FORWARD_LINK: u32 = 369181920;
pub const DIBUTTON_FOOTBALLO_JUKE: u32 = 369116166;
pub const DIBUTTON_FOOTBALLO_JUMP: u32 = 369099777;
pub const DIBUTTON_FOOTBALLO_LEFTARM: u32 = 369099778;
pub const DIBUTTON_FOOTBALLO_LEFT_LINK: u32 = 369149156;
pub const DIBUTTON_FOOTBALLO_MENU: u32 = 369100029;
pub const DIBUTTON_FOOTBALLO_PAUSE: u32 = 369116412;
pub const DIBUTTON_FOOTBALLO_RIGHTARM: u32 = 369099779;
pub const DIBUTTON_FOOTBALLO_RIGHT_LINK: u32 = 369149164;
pub const DIBUTTON_FOOTBALLO_SHOULDER: u32 = 369116167;
pub const DIBUTTON_FOOTBALLO_SPIN: u32 = 369099781;
pub const DIBUTTON_FOOTBALLO_SUBSTITUTE: u32 = 369116171;
pub const DIBUTTON_FOOTBALLO_THROW: u32 = 369099780;
pub const DIBUTTON_FOOTBALLO_TURBO: u32 = 369116168;
pub const DIBUTTON_FOOTBALLO_ZOOM: u32 = 369116170;
pub const DIBUTTON_FOOTBALLP_DEVICE: u32 = 335561982;
pub const DIBUTTON_FOOTBALLP_HELP: u32 = 335545347;
pub const DIBUTTON_FOOTBALLP_MENU: u32 = 335545597;
pub const DIBUTTON_FOOTBALLP_PAUSE: u32 = 335561980;
pub const DIBUTTON_FOOTBALLP_PLAY: u32 = 335545345;
pub const DIBUTTON_FOOTBALLP_SELECT: u32 = 335545346;
pub const DIBUTTON_FOOTBALLQ_AUDIBLE: u32 = 352338953;
pub const DIBUTTON_FOOTBALLQ_BACK_LINK: u32 = 352404712;
pub const DIBUTTON_FOOTBALLQ_DEVICE: u32 = 352339198;
pub const DIBUTTON_FOOTBALLQ_FAKE: u32 = 352322566;
pub const DIBUTTON_FOOTBALLQ_FAKESNAP: u32 = 352338951;
pub const DIBUTTON_FOOTBALLQ_FORWARD_LINK: u32 = 352404704;
pub const DIBUTTON_FOOTBALLQ_JUMP: u32 = 352322563;
pub const DIBUTTON_FOOTBALLQ_LEFT_LINK: u32 = 352371940;
pub const DIBUTTON_FOOTBALLQ_MENU: u32 = 352322813;
pub const DIBUTTON_FOOTBALLQ_MOTION: u32 = 352338952;
pub const DIBUTTON_FOOTBALLQ_PASS: u32 = 352322565;
pub const DIBUTTON_FOOTBALLQ_PAUSE: u32 = 352339196;
pub const DIBUTTON_FOOTBALLQ_RIGHT_LINK: u32 = 352371948;
pub const DIBUTTON_FOOTBALLQ_SELECT: u32 = 352322561;
pub const DIBUTTON_FOOTBALLQ_SLIDE: u32 = 352322564;
pub const DIBUTTON_FOOTBALLQ_SNAP: u32 = 352322562;
pub const DIBUTTON_FPS_APPLY: u32 = 150995971;
pub const DIBUTTON_FPS_BACKWARD_LINK: u32 = 151078120;
pub const DIBUTTON_FPS_CROUCH: u32 = 150995973;
pub const DIBUTTON_FPS_DEVICE: u32 = 151012606;
pub const DIBUTTON_FPS_DISPLAY: u32 = 151012360;
pub const DIBUTTON_FPS_DODGE: u32 = 151012361;
pub const DIBUTTON_FPS_FIRE: u32 = 150995969;
pub const DIBUTTON_FPS_FIRESECONDARY: u32 = 151012364;
pub const DIBUTTON_FPS_FORWARD_LINK: u32 = 151078112;
pub const DIBUTTON_FPS_GLANCEL: u32 = 151012362;
pub const DIBUTTON_FPS_GLANCER: u32 = 151012363;
pub const DIBUTTON_FPS_GLANCE_DOWN_LINK: u32 = 151110888;
pub const DIBUTTON_FPS_GLANCE_UP_LINK: u32 = 151110880;
pub const DIBUTTON_FPS_JUMP: u32 = 150995974;
pub const DIBUTTON_FPS_MENU: u32 = 150996221;
pub const DIBUTTON_FPS_PAUSE: u32 = 151012604;
pub const DIBUTTON_FPS_ROTATE_LEFT_LINK: u32 = 151045348;
pub const DIBUTTON_FPS_ROTATE_RIGHT_LINK: u32 = 151045356;
pub const DIBUTTON_FPS_SELECT: u32 = 150995972;
pub const DIBUTTON_FPS_STEP_LEFT_LINK: u32 = 151143652;
pub const DIBUTTON_FPS_STEP_RIGHT_LINK: u32 = 151143660;
pub const DIBUTTON_FPS_STRAFE: u32 = 150995975;
pub const DIBUTTON_FPS_WEAPONS: u32 = 150995970;
pub const DIBUTTON_GOLF_BACK_LINK: u32 = 402736360;
pub const DIBUTTON_GOLF_DEVICE: u32 = 402670846;
pub const DIBUTTON_GOLF_DOWN: u32 = 402654212;
pub const DIBUTTON_GOLF_FLYBY: u32 = 402654214;
pub const DIBUTTON_GOLF_FORWARD_LINK: u32 = 402736352;
pub const DIBUTTON_GOLF_LEFT_LINK: u32 = 402703588;
pub const DIBUTTON_GOLF_MENU: u32 = 402654461;
pub const DIBUTTON_GOLF_PAUSE: u32 = 402670844;
pub const DIBUTTON_GOLF_RIGHT_LINK: u32 = 402703596;
pub const DIBUTTON_GOLF_SELECT: u32 = 402654210;
pub const DIBUTTON_GOLF_SUBSTITUTE: u32 = 402670601;
pub const DIBUTTON_GOLF_SWING: u32 = 402654209;
pub const DIBUTTON_GOLF_TERRAIN: u32 = 402654213;
pub const DIBUTTON_GOLF_TIMEOUT: u32 = 402670600;
pub const DIBUTTON_GOLF_UP: u32 = 402654211;
pub const DIBUTTON_GOLF_ZOOM: u32 = 402670599;
pub const DIBUTTON_HOCKEYD_BACK_LINK: u32 = 436290792;
pub const DIBUTTON_HOCKEYD_BLOCK: u32 = 436208644;
pub const DIBUTTON_HOCKEYD_BURST: u32 = 436208643;
pub const DIBUTTON_HOCKEYD_DEVICE: u32 = 436225278;
pub const DIBUTTON_HOCKEYD_FAKE: u32 = 436208645;
pub const DIBUTTON_HOCKEYD_FORWARD_LINK: u32 = 436290784;
pub const DIBUTTON_HOCKEYD_LEFT_LINK: u32 = 436258020;
pub const DIBUTTON_HOCKEYD_MENU: u32 = 436208893;
pub const DIBUTTON_HOCKEYD_PAUSE: u32 = 436225276;
pub const DIBUTTON_HOCKEYD_PLAYER: u32 = 436208641;
pub const DIBUTTON_HOCKEYD_RIGHT_LINK: u32 = 436258028;
pub const DIBUTTON_HOCKEYD_STEAL: u32 = 436208642;
pub const DIBUTTON_HOCKEYD_STRATEGY: u32 = 436225031;
pub const DIBUTTON_HOCKEYD_SUBSTITUTE: u32 = 436225033;
pub const DIBUTTON_HOCKEYD_TIMEOUT: u32 = 436225032;
pub const DIBUTTON_HOCKEYD_ZOOM: u32 = 436225030;
pub const DIBUTTON_HOCKEYG_BACK_LINK: u32 = 453068008;
pub const DIBUTTON_HOCKEYG_BLOCK: u32 = 452985860;
pub const DIBUTTON_HOCKEYG_DEVICE: u32 = 453002494;
pub const DIBUTTON_HOCKEYG_FORWARD_LINK: u32 = 453068000;
pub const DIBUTTON_HOCKEYG_LEFT_LINK: u32 = 453035236;
pub const DIBUTTON_HOCKEYG_MENU: u32 = 452986109;
pub const DIBUTTON_HOCKEYG_PASS: u32 = 452985857;
pub const DIBUTTON_HOCKEYG_PAUSE: u32 = 453002492;
pub const DIBUTTON_HOCKEYG_POKE: u32 = 452985858;
pub const DIBUTTON_HOCKEYG_RIGHT_LINK: u32 = 453035244;
pub const DIBUTTON_HOCKEYG_STEAL: u32 = 452985859;
pub const DIBUTTON_HOCKEYG_STRATEGY: u32 = 453002246;
pub const DIBUTTON_HOCKEYG_SUBSTITUTE: u32 = 453002248;
pub const DIBUTTON_HOCKEYG_TIMEOUT: u32 = 453002247;
pub const DIBUTTON_HOCKEYG_ZOOM: u32 = 453002245;
pub const DIBUTTON_HOCKEYO_BACK_LINK: u32 = 419513576;
pub const DIBUTTON_HOCKEYO_BURST: u32 = 419431427;
pub const DIBUTTON_HOCKEYO_DEVICE: u32 = 419448062;
pub const DIBUTTON_HOCKEYO_FAKE: u32 = 419431429;
pub const DIBUTTON_HOCKEYO_FORWARD_LINK: u32 = 419513568;
pub const DIBUTTON_HOCKEYO_LEFT_LINK: u32 = 419480804;
pub const DIBUTTON_HOCKEYO_MENU: u32 = 419431677;
pub const DIBUTTON_HOCKEYO_PASS: u32 = 419431426;
pub const DIBUTTON_HOCKEYO_PAUSE: u32 = 419448060;
pub const DIBUTTON_HOCKEYO_RIGHT_LINK: u32 = 419480812;
pub const DIBUTTON_HOCKEYO_SHOOT: u32 = 419431425;
pub const DIBUTTON_HOCKEYO_SPECIAL: u32 = 419431428;
pub const DIBUTTON_HOCKEYO_STRATEGY: u32 = 419447815;
pub const DIBUTTON_HOCKEYO_SUBSTITUTE: u32 = 419447817;
pub const DIBUTTON_HOCKEYO_TIMEOUT: u32 = 419447816;
pub const DIBUTTON_HOCKEYO_ZOOM: u32 = 419447814;
pub const DIBUTTON_HUNTING_AIM: u32 = 218104834;
pub const DIBUTTON_HUNTING_BACK_LINK: u32 = 218186984;
pub const DIBUTTON_HUNTING_BINOCULAR: u32 = 218104836;
pub const DIBUTTON_HUNTING_CALL: u32 = 218104837;
pub const DIBUTTON_HUNTING_CROUCH: u32 = 218121225;
pub const DIBUTTON_HUNTING_DEVICE: u32 = 218121470;
pub const DIBUTTON_HUNTING_DISPLAY: u32 = 218121224;
pub const DIBUTTON_HUNTING_FIRE: u32 = 218104833;
pub const DIBUTTON_HUNTING_FIRESECONDARY: u32 = 218121227;
pub const DIBUTTON_HUNTING_FORWARD_LINK: u32 = 218186976;
pub const DIBUTTON_HUNTING_JUMP: u32 = 218121226;
pub const DIBUTTON_HUNTING_LEFT_LINK: u32 = 218154212;
pub const DIBUTTON_HUNTING_MAP: u32 = 218104838;
pub const DIBUTTON_HUNTING_MENU: u32 = 218105085;
pub const DIBUTTON_HUNTING_PAUSE: u32 = 218121468;
pub const DIBUTTON_HUNTING_RIGHT_LINK: u32 = 218154220;
pub const DIBUTTON_HUNTING_ROTATE_LEFT_LINK: u32 = 218252516;
pub const DIBUTTON_HUNTING_ROTATE_RIGHT_LINK: u32 = 218252524;
pub const DIBUTTON_HUNTING_SPECIAL: u32 = 218104839;
pub const DIBUTTON_HUNTING_WEAPON: u32 = 218104835;
pub const DIBUTTON_MECHA_BACK_LINK: u32 = 687949032;
pub const DIBUTTON_MECHA_CENTER: u32 = 687883271;
pub const DIBUTTON_MECHA_DEVICE: u32 = 687883518;
pub const DIBUTTON_MECHA_FASTER_LINK: u32 = 688112864;
pub const DIBUTTON_MECHA_FIRE: u32 = 687866881;
pub const DIBUTTON_MECHA_FIRESECONDARY: u32 = 687883273;
pub const DIBUTTON_MECHA_FORWARD_LINK: u32 = 687949024;
pub const DIBUTTON_MECHA_JUMP: u32 = 687866886;
pub const DIBUTTON_MECHA_LEFT_LINK: u32 = 687916260;
pub const DIBUTTON_MECHA_MENU: u32 = 687867133;
pub const DIBUTTON_MECHA_PAUSE: u32 = 687883516;
pub const DIBUTTON_MECHA_REVERSE: u32 = 687866884;
pub const DIBUTTON_MECHA_RIGHT_LINK: u32 = 687916268;
pub const DIBUTTON_MECHA_ROTATE_LEFT_LINK: u32 = 688014564;
pub const DIBUTTON_MECHA_ROTATE_RIGHT_LINK: u32 = 688014572;
pub const DIBUTTON_MECHA_SLOWER_LINK: u32 = 688112872;
pub const DIBUTTON_MECHA_TARGET: u32 = 687866883;
pub const DIBUTTON_MECHA_VIEW: u32 = 687883272;
pub const DIBUTTON_MECHA_WEAPONS: u32 = 687866882;
pub const DIBUTTON_MECHA_ZOOM: u32 = 687866885;
pub const DIBUTTON_RACQUET_BACKSWING: u32 = 536871938;
pub const DIBUTTON_RACQUET_BACK_LINK: u32 = 536954088;
pub const DIBUTTON_RACQUET_DEVICE: u32 = 536888574;
pub const DIBUTTON_RACQUET_FORWARD_LINK: u32 = 536954080;
pub const DIBUTTON_RACQUET_LEFT_LINK: u32 = 536921316;
pub const DIBUTTON_RACQUET_MENU: u32 = 536872189;
pub const DIBUTTON_RACQUET_PAUSE: u32 = 536888572;
pub const DIBUTTON_RACQUET_RIGHT_LINK: u32 = 536921324;
pub const DIBUTTON_RACQUET_SELECT: u32 = 536871941;
pub const DIBUTTON_RACQUET_SMASH: u32 = 536871939;
pub const DIBUTTON_RACQUET_SPECIAL: u32 = 536871940;
pub const DIBUTTON_RACQUET_SUBSTITUTE: u32 = 536888327;
pub const DIBUTTON_RACQUET_SWING: u32 = 536871937;
pub const DIBUTTON_RACQUET_TIMEOUT: u32 = 536888326;
pub const DIBUTTON_REMOTE_ADJUST: u32 = 654334990;
pub const DIBUTTON_REMOTE_CABLE: u32 = 654334985;
pub const DIBUTTON_REMOTE_CD: u32 = 654334986;
pub const DIBUTTON_REMOTE_CHANGE: u32 = 654320646;
pub const DIBUTTON_REMOTE_CUE: u32 = 654320644;
pub const DIBUTTON_REMOTE_DEVICE: u32 = 654329086;
pub const DIBUTTON_REMOTE_DIGIT0: u32 = 654332943;
pub const DIBUTTON_REMOTE_DIGIT1: u32 = 654332944;
pub const DIBUTTON_REMOTE_DIGIT2: u32 = 654332945;
pub const DIBUTTON_REMOTE_DIGIT3: u32 = 654332946;
pub const DIBUTTON_REMOTE_DIGIT4: u32 = 654332947;
pub const DIBUTTON_REMOTE_DIGIT5: u32 = 654332948;
pub const DIBUTTON_REMOTE_DIGIT6: u32 = 654332949;
pub const DIBUTTON_REMOTE_DIGIT7: u32 = 654332950;
pub const DIBUTTON_REMOTE_DIGIT8: u32 = 654332951;
pub const DIBUTTON_REMOTE_DIGIT9: u32 = 654332952;
pub const DIBUTTON_REMOTE_DVD: u32 = 654334989;
pub const DIBUTTON_REMOTE_MENU: u32 = 654312701;
pub const DIBUTTON_REMOTE_MUTE: u32 = 654312449;
pub const DIBUTTON_REMOTE_PAUSE: u32 = 654329084;
pub const DIBUTTON_REMOTE_PLAY: u32 = 654320643;
pub const DIBUTTON_REMOTE_RECORD: u32 = 654320647;
pub const DIBUTTON_REMOTE_REVIEW: u32 = 654320645;
pub const DIBUTTON_REMOTE_SELECT: u32 = 654312450;
pub const DIBUTTON_REMOTE_TUNER: u32 = 654334988;
pub const DIBUTTON_REMOTE_TV: u32 = 654334984;
pub const DIBUTTON_REMOTE_VCR: u32 = 654334987;
pub const DIBUTTON_SKIING_CAMERA: u32 = 486540291;
pub const DIBUTTON_SKIING_CROUCH: u32 = 486540290;
pub const DIBUTTON_SKIING_DEVICE: u32 = 486556926;
pub const DIBUTTON_SKIING_FASTER_LINK: u32 = 486622432;
pub const DIBUTTON_SKIING_JUMP: u32 = 486540289;
pub const DIBUTTON_SKIING_LEFT_LINK: u32 = 486589668;
pub const DIBUTTON_SKIING_MENU: u32 = 486540541;
pub const DIBUTTON_SKIING_PAUSE: u32 = 486556924;
pub const DIBUTTON_SKIING_RIGHT_LINK: u32 = 486589676;
pub const DIBUTTON_SKIING_SELECT: u32 = 486540293;
pub const DIBUTTON_SKIING_SLOWER_LINK: u32 = 486622440;
pub const DIBUTTON_SKIING_SPECIAL1: u32 = 486540292;
pub const DIBUTTON_SKIING_SPECIAL2: u32 = 486540294;
pub const DIBUTTON_SKIING_ZOOM: u32 = 486556679;
pub const DIBUTTON_SOCCERD_BACK_LINK: u32 = 520176872;
pub const DIBUTTON_SOCCERD_BLOCK: u32 = 520094721;
pub const DIBUTTON_SOCCERD_CLEAR: u32 = 520111114;
pub const DIBUTTON_SOCCERD_DEVICE: u32 = 520111358;
pub const DIBUTTON_SOCCERD_FAKE: u32 = 520094723;
pub const DIBUTTON_SOCCERD_FORWARD_LINK: u32 = 520176864;
pub const DIBUTTON_SOCCERD_FOUL: u32 = 520111112;
pub const DIBUTTON_SOCCERD_GOALIECHARGE: u32 = 520111115;
pub const DIBUTTON_SOCCERD_HEAD: u32 = 520111113;
pub const DIBUTTON_SOCCERD_LEFT_LINK: u32 = 520144100;
pub const DIBUTTON_SOCCERD_MENU: u32 = 520094973;
pub const DIBUTTON_SOCCERD_PAUSE: u32 = 520111356;
pub const DIBUTTON_SOCCERD_PLAYER: u32 = 520094724;
pub const DIBUTTON_SOCCERD_RIGHT_LINK: u32 = 520144108;
pub const DIBUTTON_SOCCERD_SELECT: u32 = 520094726;
pub const DIBUTTON_SOCCERD_SLIDE: u32 = 520094727;
pub const DIBUTTON_SOCCERD_SPECIAL: u32 = 520094725;
pub const DIBUTTON_SOCCERD_STEAL: u32 = 520094722;
pub const DIBUTTON_SOCCERD_SUBSTITUTE: u32 = 520111116;
pub const DIBUTTON_SOCCERO_BACK_LINK: u32 = 503399656;
pub const DIBUTTON_SOCCERO_CONTROL: u32 = 503333900;
pub const DIBUTTON_SOCCERO_DEVICE: u32 = 503334142;
pub const DIBUTTON_SOCCERO_FAKE: u32 = 503317507;
pub const DIBUTTON_SOCCERO_FORWARD_LINK: u32 = 503399648;
pub const DIBUTTON_SOCCERO_HEAD: u32 = 503333901;
pub const DIBUTTON_SOCCERO_LEFT_LINK: u32 = 503366884;
pub const DIBUTTON_SOCCERO_MENU: u32 = 503317757;
pub const DIBUTTON_SOCCERO_PASS: u32 = 503317506;
pub const DIBUTTON_SOCCERO_PASSTHRU: u32 = 503333898;
pub const DIBUTTON_SOCCERO_PAUSE: u32 = 503334140;
pub const DIBUTTON_SOCCERO_PLAYER: u32 = 503317508;
pub const DIBUTTON_SOCCERO_RIGHT_LINK: u32 = 503366892;
pub const DIBUTTON_SOCCERO_SELECT: u32 = 503317510;
pub const DIBUTTON_SOCCERO_SHOOT: u32 = 503317505;
pub const DIBUTTON_SOCCERO_SHOOTHIGH: u32 = 503333897;
pub const DIBUTTON_SOCCERO_SHOOTLOW: u32 = 503333896;
pub const DIBUTTON_SOCCERO_SPECIAL1: u32 = 503317509;
pub const DIBUTTON_SOCCERO_SPRINT: u32 = 503333899;
pub const DIBUTTON_SOCCERO_SUBSTITUTE: u32 = 503333895;
pub const DIBUTTON_SPACESIM_BACKWARD_LINK: u32 = 117523688;
pub const DIBUTTON_SPACESIM_DEVICE: u32 = 117458174;
pub const DIBUTTON_SPACESIM_DISPLAY: u32 = 117457925;
pub const DIBUTTON_SPACESIM_FASTER_LINK: u32 = 117687520;
pub const DIBUTTON_SPACESIM_FIRE: u32 = 117441537;
pub const DIBUTTON_SPACESIM_FIRESECONDARY: u32 = 117457929;
pub const DIBUTTON_SPACESIM_FORWARD_LINK: u32 = 117523680;
pub const DIBUTTON_SPACESIM_GEAR: u32 = 117457928;
pub const DIBUTTON_SPACESIM_GLANCE_DOWN_LINK: u32 = 117949672;
pub const DIBUTTON_SPACESIM_GLANCE_LEFT_LINK: u32 = 117949668;
pub const DIBUTTON_SPACESIM_GLANCE_RIGHT_LINK: u32 = 117949676;
pub const DIBUTTON_SPACESIM_GLANCE_UP_LINK: u32 = 117949664;
pub const DIBUTTON_SPACESIM_LEFT_LINK: u32 = 117490916;
pub const DIBUTTON_SPACESIM_LOWER: u32 = 117457927;
pub const DIBUTTON_SPACESIM_MENU: u32 = 117441789;
pub const DIBUTTON_SPACESIM_PAUSE: u32 = 117458172;
pub const DIBUTTON_SPACESIM_RAISE: u32 = 117457926;
pub const DIBUTTON_SPACESIM_RIGHT_LINK: u32 = 117490924;
pub const DIBUTTON_SPACESIM_SLOWER_LINK: u32 = 117687528;
pub const DIBUTTON_SPACESIM_TARGET: u32 = 117441539;
pub const DIBUTTON_SPACESIM_TURN_LEFT_LINK: u32 = 117589220;
pub const DIBUTTON_SPACESIM_TURN_RIGHT_LINK: u32 = 117589228;
pub const DIBUTTON_SPACESIM_VIEW: u32 = 117457924;
pub const DIBUTTON_SPACESIM_WEAPONS: u32 = 117441538;
pub const DIBUTTON_STRATEGYR_APPLY: u32 = 184550402;
pub const DIBUTTON_STRATEGYR_ATTACK: u32 = 184550404;
pub const DIBUTTON_STRATEGYR_BACK_LINK: u32 = 184632552;
pub const DIBUTTON_STRATEGYR_CAST: u32 = 184550405;
pub const DIBUTTON_STRATEGYR_CROUCH: u32 = 184550406;
pub const DIBUTTON_STRATEGYR_DEVICE: u32 = 184567038;
pub const DIBUTTON_STRATEGYR_DISPLAY: u32 = 184566793;
pub const DIBUTTON_STRATEGYR_FORWARD_LINK: u32 = 184632544;
pub const DIBUTTON_STRATEGYR_GET: u32 = 184550401;
pub const DIBUTTON_STRATEGYR_JUMP: u32 = 184550407;
pub const DIBUTTON_STRATEGYR_LEFT_LINK: u32 = 184599780;
pub const DIBUTTON_STRATEGYR_MAP: u32 = 184566792;
pub const DIBUTTON_STRATEGYR_MENU: u32 = 184550653;
pub const DIBUTTON_STRATEGYR_PAUSE: u32 = 184567036;
pub const DIBUTTON_STRATEGYR_RIGHT_LINK: u32 = 184599788;
pub const DIBUTTON_STRATEGYR_ROTATE_LEFT_LINK: u32 = 184698084;
pub const DIBUTTON_STRATEGYR_ROTATE_RIGHT_LINK: u32 = 184698092;
pub const DIBUTTON_STRATEGYR_SELECT: u32 = 184550403;
pub const DIBUTTON_STRATEGYT_APPLY: u32 = 201327619;
pub const DIBUTTON_STRATEGYT_BACK_LINK: u32 = 201409768;
pub const DIBUTTON_STRATEGYT_DEVICE: u32 = 201344254;
pub const DIBUTTON_STRATEGYT_DISPLAY: u32 = 201344008;
pub const DIBUTTON_STRATEGYT_FORWARD_LINK: u32 = 201409760;
pub const DIBUTTON_STRATEGYT_INSTRUCT: u32 = 201327618;
pub const DIBUTTON_STRATEGYT_LEFT_LINK: u32 = 201376996;
pub const DIBUTTON_STRATEGYT_MAP: u32 = 201344007;
pub const DIBUTTON_STRATEGYT_MENU: u32 = 201327869;
pub const DIBUTTON_STRATEGYT_PAUSE: u32 = 201344252;
pub const DIBUTTON_STRATEGYT_RIGHT_LINK: u32 = 201377004;
pub const DIBUTTON_STRATEGYT_SELECT: u32 = 201327617;
pub const DIBUTTON_STRATEGYT_TEAM: u32 = 201327620;
pub const DIBUTTON_STRATEGYT_TURN: u32 = 201327621;
pub const DIBUTTON_STRATEGYT_ZOOM: u32 = 201344006;
pub const DIBUTTON_TPS_ACTION: u32 = 167773186;
pub const DIBUTTON_TPS_BACKWARD_LINK: u32 = 167855336;
pub const DIBUTTON_TPS_DEVICE: u32 = 167789822;
pub const DIBUTTON_TPS_DODGE: u32 = 167789577;
pub const DIBUTTON_TPS_FORWARD_LINK: u32 = 167855328;
pub const DIBUTTON_TPS_GLANCE_DOWN_LINK: u32 = 168281320;
pub const DIBUTTON_TPS_GLANCE_LEFT_LINK: u32 = 168281316;
pub const DIBUTTON_TPS_GLANCE_RIGHT_LINK: u32 = 168281324;
pub const DIBUTTON_TPS_GLANCE_UP_LINK: u32 = 168281312;
pub const DIBUTTON_TPS_INVENTORY: u32 = 167789578;
pub const DIBUTTON_TPS_JUMP: u32 = 167773189;
pub const DIBUTTON_TPS_MENU: u32 = 167773437;
pub const DIBUTTON_TPS_PAUSE: u32 = 167789820;
pub const DIBUTTON_TPS_RUN: u32 = 167773185;
pub const DIBUTTON_TPS_SELECT: u32 = 167773187;
pub const DIBUTTON_TPS_STEPLEFT: u32 = 167789575;
pub const DIBUTTON_TPS_STEPRIGHT: u32 = 167789576;
pub const DIBUTTON_TPS_TURN_LEFT_LINK: u32 = 167920868;
pub const DIBUTTON_TPS_TURN_RIGHT_LINK: u32 = 167920876;
pub const DIBUTTON_TPS_USE: u32 = 167773188;
pub const DIBUTTON_TPS_VIEW: u32 = 167789574;
pub const DICD_DEFAULT: u32 = 0;
pub const DICD_EDIT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DICONDITION {
    pub lOffset: i32,
    pub lPositiveCoefficient: i32,
    pub lNegativeCoefficient: i32,
    pub dwPositiveSaturation: u32,
    pub dwNegativeSaturation: u32,
    pub lDeadBand: i32,
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Foundation")]
impl Default for DICONFIGUREDEVICESPARAMSA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Foundation")]
impl Default for DICONFIGUREDEVICESPARAMSW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DICONSTANTFORCE {
    pub lMagnitude: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const DIDAL_BOTTOMALIGNED: u32 = 8;
pub const DIDAL_CENTERED: u32 = 0;
pub const DIDAL_LEFTALIGNED: u32 = 1;
pub const DIDAL_MIDDLE: u32 = 0;
pub const DIDAL_RIGHTALIGNED: u32 = 2;
pub const DIDAL_TOPALIGNED: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const DIDBAM_DEFAULT: u32 = 0;
pub const DIDBAM_HWDEFAULTS: u32 = 4;
pub const DIDBAM_INITIALIZE: u32 = 2;
pub const DIDBAM_PRESERVE: u32 = 1;
pub const DIDC_ALIAS: u32 = 65536;
pub const DIDC_ATTACHED: u32 = 1;
pub const DIDC_DEADBAND: u32 = 16384;
pub const DIDC_EMULATED: u32 = 4;
pub const DIDC_FFATTACK: u32 = 512;
pub const DIDC_FFFADE: u32 = 1024;
pub const DIDC_FORCEFEEDBACK: u32 = 256;
pub const DIDC_HIDDEN: u32 = 262144;
pub const DIDC_PHANTOM: u32 = 131072;
pub const DIDC_POLLEDDATAFORMAT: u32 = 8;
pub const DIDC_POLLEDDEVICE: u32 = 2;
pub const DIDC_POSNEGCOEFFICIENTS: u32 = 4096;
pub const DIDC_POSNEGSATURATION: u32 = 8192;
pub const DIDC_SATURATION: u32 = 2048;
pub const DIDC_STARTDELAY: u32 = 32768;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIDEVCAPS_DX3 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDevType: u32,
    pub dwAxes: u32,
    pub dwButtons: u32,
    pub dwPOVs: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Foundation")]
impl Default for DIDEVICEIMAGEINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Foundation")]
impl Default for DIDEVICEIMAGEINFOHEADERA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Foundation")]
impl Default for DIDEVICEIMAGEINFOHEADERW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Foundation")]
impl Default for DIDEVICEIMAGEINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIDEVICEOBJECTDATA {
    pub dwOfs: u32,
    pub dwData: u32,
    pub dwTimeStamp: u32,
    pub dwSequence: u32,
    pub uAppData: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIDEVICEOBJECTDATA_DX3 {
    pub dwOfs: u32,
    pub dwData: u32,
    pub dwTimeStamp: u32,
    pub dwSequence: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIDEVICESTATE {
    pub dwSize: u32,
    pub dwState: u32,
    pub dwLoad: u32,
}
pub const DIDEVTYPEJOYSTICK_FLIGHTSTICK: u32 = 3;
pub const DIDEVTYPEJOYSTICK_GAMEPAD: u32 = 4;
pub const DIDEVTYPEJOYSTICK_HEADTRACKER: u32 = 7;
pub const DIDEVTYPEJOYSTICK_RUDDER: u32 = 5;
pub const DIDEVTYPEJOYSTICK_TRADITIONAL: u32 = 2;
pub const DIDEVTYPEJOYSTICK_UNKNOWN: u32 = 1;
pub const DIDEVTYPEJOYSTICK_WHEEL: u32 = 6;
pub const DIDEVTYPEKEYBOARD_J3100: u32 = 12;
pub const DIDEVTYPEKEYBOARD_JAPAN106: u32 = 10;
pub const DIDEVTYPEKEYBOARD_JAPANAX: u32 = 11;
pub const DIDEVTYPEKEYBOARD_NEC98: u32 = 7;
pub const DIDEVTYPEKEYBOARD_NEC98106: u32 = 9;
pub const DIDEVTYPEKEYBOARD_NEC98LAPTOP: u32 = 8;
pub const DIDEVTYPEKEYBOARD_NOKIA1050: u32 = 5;
pub const DIDEVTYPEKEYBOARD_NOKIA9140: u32 = 6;
pub const DIDEVTYPEKEYBOARD_OLIVETTI: u32 = 2;
pub const DIDEVTYPEKEYBOARD_PCAT: u32 = 3;
pub const DIDEVTYPEKEYBOARD_PCENH: u32 = 4;
pub const DIDEVTYPEKEYBOARD_PCXT: u32 = 1;
pub const DIDEVTYPEKEYBOARD_UNKNOWN: u32 = 0;
pub const DIDEVTYPEMOUSE_FINGERSTICK: u32 = 3;
pub const DIDEVTYPEMOUSE_TOUCHPAD: u32 = 4;
pub const DIDEVTYPEMOUSE_TRACKBALL: u32 = 5;
pub const DIDEVTYPEMOUSE_TRADITIONAL: u32 = 2;
pub const DIDEVTYPEMOUSE_UNKNOWN: u32 = 1;
pub const DIDEVTYPE_DEVICE: u32 = 1;
pub const DIDEVTYPE_HID: u32 = 65536;
pub const DIDEVTYPE_JOYSTICK: u32 = 4;
pub const DIDEVTYPE_KEYBOARD: u32 = 3;
pub const DIDEVTYPE_MOUSE: u32 = 2;
pub const DIDFT_ABSAXIS: u32 = 2;
pub const DIDFT_ALIAS: u32 = 134217728;
pub const DIDFT_ALL: u32 = 0;
pub const DIDFT_ANYINSTANCE: u32 = 16776960;
pub const DIDFT_AXIS: u32 = 3;
pub const DIDFT_BUTTON: u32 = 12;
pub const DIDFT_COLLECTION: u32 = 64;
pub const DIDFT_FFACTUATOR: u32 = 16777216;
pub const DIDFT_FFEFFECTTRIGGER: u32 = 33554432;
pub const DIDFT_INSTANCEMASK: u32 = 16776960;
pub const DIDFT_NOCOLLECTION: u32 = 16776960;
pub const DIDFT_NODATA: u32 = 128;
pub const DIDFT_OUTPUT: u32 = 268435456;
pub const DIDFT_POV: u32 = 16;
pub const DIDFT_PSHBUTTON: u32 = 4;
pub const DIDFT_RELAXIS: u32 = 1;
pub const DIDFT_TGLBUTTON: u32 = 8;
pub const DIDFT_VENDORDEFINED: u32 = 67108864;
pub const DIDF_ABSAXIS: u32 = 1;
pub const DIDF_RELAXIS: u32 = 2;
pub const DIDIFT_CONFIGURATION: u32 = 1;
pub const DIDIFT_DELETE: u32 = 16777216;
pub const DIDIFT_OVERLAY: u32 = 2;
pub const DIDOI_ASPECTACCEL: u32 = 768;
pub const DIDOI_ASPECTFORCE: u32 = 1024;
pub const DIDOI_ASPECTMASK: u32 = 3840;
pub const DIDOI_ASPECTPOSITION: u32 = 256;
pub const DIDOI_ASPECTVELOCITY: u32 = 512;
pub const DIDOI_FFACTUATOR: u32 = 1;
pub const DIDOI_FFEFFECTTRIGGER: u32 = 2;
pub const DIDOI_GUIDISUSAGE: u32 = 65536;
pub const DIDOI_POLLED: u32 = 32768;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIDRIVERVERSIONS {
    pub dwSize: u32,
    pub dwFirmwareRevision: u32,
    pub dwHardwareRevision: u32,
    pub dwFFDriverVersion: u32,
}
pub const DIDSAM_DEFAULT: u32 = 0;
pub const DIDSAM_FORCESAVE: u32 = 2;
pub const DIDSAM_NOUSER: u32 = 1;
pub const DIEB_NOTRIGGER: u32 = 4294967295;
pub const DIEDBSFL_ATTACHEDONLY: u32 = 0;
pub const DIEDBSFL_AVAILABLEDEVICES: u32 = 4096;
pub const DIEDBSFL_FORCEFEEDBACK: u32 = 256;
pub const DIEDBSFL_MULTIMICEKEYBOARDS: u32 = 8192;
pub const DIEDBSFL_NONGAMINGDEVICES: u32 = 16384;
pub const DIEDBSFL_THISUSER: u32 = 16;
pub const DIEDBSFL_VALID: u32 = 28944;
pub const DIEDBS_MAPPEDPRI1: u32 = 1;
pub const DIEDBS_MAPPEDPRI2: u32 = 2;
pub const DIEDBS_NEWDEVICE: u32 = 32;
pub const DIEDBS_RECENTDEVICE: u32 = 16;
pub const DIEDFL_ALLDEVICES: u32 = 0;
pub const DIEDFL_ATTACHEDONLY: u32 = 1;
pub const DIEDFL_FORCEFEEDBACK: u32 = 256;
pub const DIEDFL_INCLUDEALIASES: u32 = 65536;
pub const DIEDFL_INCLUDEHIDDEN: u32 = 262144;
pub const DIEDFL_INCLUDEPHANTOMS: u32 = 131072;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIEFFECTATTRIBUTES {
    pub dwEffectId: u32,
    pub dwEffType: u32,
    pub dwStaticParams: u32,
    pub dwDynamicParams: u32,
    pub dwCoords: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const DIEFF_CARTESIAN: u32 = 16;
pub const DIEFF_OBJECTIDS: u32 = 1;
pub const DIEFF_OBJECTOFFSETS: u32 = 2;
pub const DIEFF_POLAR: u32 = 32;
pub const DIEFF_SPHERICAL: u32 = 64;
pub const DIEFT_ALL: u32 = 0;
pub const DIEFT_CONDITION: u32 = 4;
pub const DIEFT_CONSTANTFORCE: u32 = 1;
pub const DIEFT_CUSTOMFORCE: u32 = 5;
pub const DIEFT_DEADBAND: u32 = 16384;
pub const DIEFT_FFATTACK: u32 = 512;
pub const DIEFT_FFFADE: u32 = 1024;
pub const DIEFT_HARDWARE: u32 = 255;
pub const DIEFT_PERIODIC: u32 = 3;
pub const DIEFT_POSNEGCOEFFICIENTS: u32 = 4096;
pub const DIEFT_POSNEGSATURATION: u32 = 8192;
pub const DIEFT_RAMPFORCE: u32 = 2;
pub const DIEFT_SATURATION: u32 = 2048;
pub const DIEFT_STARTDELAY: u32 = 32768;
pub const DIEGES_EMULATED: u32 = 2;
pub const DIEGES_PLAYING: u32 = 1;
pub const DIENUM_CONTINUE: u32 = 1;
pub const DIENUM_STOP: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIENVELOPE {
    pub dwSize: u32,
    pub dwAttackLevel: u32,
    pub dwAttackTime: u32,
    pub dwFadeLevel: u32,
    pub dwFadeTime: u32,
}
pub const DIEP_ALLPARAMS: u32 = 1023;
pub const DIEP_ALLPARAMS_DX5: u32 = 511;
pub const DIEP_AXES: u32 = 32;
pub const DIEP_DIRECTION: u32 = 64;
pub const DIEP_DURATION: u32 = 1;
pub const DIEP_ENVELOPE: u32 = 128;
pub const DIEP_GAIN: u32 = 4;
pub const DIEP_NODOWNLOAD: u32 = 2147483648;
pub const DIEP_NORESTART: u32 = 1073741824;
pub const DIEP_SAMPLEPERIOD: u32 = 2;
pub const DIEP_START: u32 = 536870912;
pub const DIEP_STARTDELAY: u32 = 512;
pub const DIEP_TRIGGERBUTTON: u32 = 8;
pub const DIEP_TRIGGERREPEATINTERVAL: u32 = 16;
pub const DIEP_TYPESPECIFICPARAMS: u32 = 256;
pub const DIERR_ACQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x800700AA_u32 as _);
pub const DIERR_ALREADYINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x800704DF_u32 as _);
pub const DIERR_BADDRIVERVER: windows_core::HRESULT = windows_core::HRESULT(0x80070077_u32 as _);
pub const DIERR_BADINF: i32 = -2147220478;
pub const DIERR_BETADIRECTINPUTVERSION: windows_core::HRESULT = windows_core::HRESULT(0x80070481_u32 as _);
pub const DIERR_CANCELLED: i32 = -2147220479;
pub const DIERR_DEVICEFULL: i32 = -2147220991;
pub const DIERR_DEVICENOTREG: i32 = -2147221164;
pub const DIERR_DRIVERFIRST: i32 = -2147220736;
pub const DIERR_DRIVERLAST: i32 = -2147220481;
pub const DIERR_EFFECTPLAYING: i32 = -2147220984;
pub const DIERR_GENERIC: i32 = -2147467259;
pub const DIERR_HANDLEEXISTS: i32 = -2147024891;
pub const DIERR_HASEFFECTS: i32 = -2147220988;
pub const DIERR_INCOMPLETEEFFECT: i32 = -2147220986;
pub const DIERR_INPUTLOST: windows_core::HRESULT = windows_core::HRESULT(0x8007001E_u32 as _);
pub const DIERR_INSUFFICIENTPRIVS: i32 = -2147220992;
pub const DIERR_INVALIDCLASSINSTALLER: i32 = -2147220480;
pub const DIERR_INVALIDPARAM: i32 = -2147024809;
pub const DIERR_MAPFILEFAIL: i32 = -2147220981;
pub const DIERR_MOREDATA: i32 = -2147220990;
pub const DIERR_NOAGGREGATION: i32 = -2147221232;
pub const DIERR_NOINTERFACE: i32 = -2147467262;
pub const DIERR_NOMOREITEMS: windows_core::HRESULT = windows_core::HRESULT(0x80070103_u32 as _);
pub const DIERR_NOTACQUIRED: windows_core::HRESULT = windows_core::HRESULT(0x8007000C_u32 as _);
pub const DIERR_NOTBUFFERED: i32 = -2147220985;
pub const DIERR_NOTDOWNLOADED: i32 = -2147220989;
pub const DIERR_NOTEXCLUSIVEACQUIRED: i32 = -2147220987;
pub const DIERR_NOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80070002_u32 as _);
pub const DIERR_NOTINITIALIZED: windows_core::HRESULT = windows_core::HRESULT(0x80070015_u32 as _);
pub const DIERR_OBJECTNOTFOUND: windows_core::HRESULT = windows_core::HRESULT(0x80070002_u32 as _);
pub const DIERR_OLDDIRECTINPUTVERSION: windows_core::HRESULT = windows_core::HRESULT(0x8007047E_u32 as _);
pub const DIERR_OTHERAPPHASPRIO: i32 = -2147024891;
pub const DIERR_OUTOFMEMORY: i32 = -2147024882;
pub const DIERR_READONLY: i32 = -2147024891;
pub const DIERR_REPORTFULL: i32 = -2147220982;
pub const DIERR_UNPLUGGED: i32 = -2147220983;
pub const DIERR_UNSUPPORTED: i32 = -2147467263;
pub const DIES_NODOWNLOAD: u32 = 2147483648;
pub const DIES_SOLO: u32 = 1;
pub const DIFEF_DEFAULT: u32 = 0;
pub const DIFEF_INCLUDENONSTANDARD: u32 = 1;
pub const DIFEF_MODIFYIFNEEDED: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIFFDEVICEATTRIBUTES {
    pub dwFlags: u32,
    pub dwFFSamplePeriod: u32,
    pub dwFFMinTimeResolution: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIFFOBJECTATTRIBUTES {
    pub dwFFMaxForce: u32,
    pub dwFFForceResolution: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const DIGDD_PEEK: u32 = 1;
pub const DIGFFS_ACTUATORSOFF: u32 = 32;
pub const DIGFFS_ACTUATORSON: u32 = 16;
pub const DIGFFS_DEVICELOST: u32 = 2147483648;
pub const DIGFFS_EMPTY: u32 = 1;
pub const DIGFFS_PAUSED: u32 = 4;
pub const DIGFFS_POWEROFF: u32 = 128;
pub const DIGFFS_POWERON: u32 = 64;
pub const DIGFFS_SAFETYSWITCHOFF: u32 = 512;
pub const DIGFFS_SAFETYSWITCHON: u32 = 256;
pub const DIGFFS_STOPPED: u32 = 2;
pub const DIGFFS_USERFFSWITCHOFF: u32 = 2048;
pub const DIGFFS_USERFFSWITCHON: u32 = 1024;
pub const DIHATSWITCH_2DCONTROL_HATSWITCH: u32 = 587220481;
pub const DIHATSWITCH_3DCONTROL_HATSWITCH: u32 = 603997697;
pub const DIHATSWITCH_ARCADEP_VIEW: u32 = 570443265;
pub const DIHATSWITCH_ARCADES_VIEW: u32 = 553666049;
pub const DIHATSWITCH_BBALLD_GLANCE: u32 = 318785025;
pub const DIHATSWITCH_BBALLO_GLANCE: u32 = 302007809;
pub const DIHATSWITCH_BIKINGM_SCROLL: u32 = 469779969;
pub const DIHATSWITCH_CADF_HATSWITCH: u32 = 620774913;
pub const DIHATSWITCH_CADM_HATSWITCH: u32 = 637552129;
pub const DIHATSWITCH_DRIVINGC_GLANCE: u32 = 33572353;
pub const DIHATSWITCH_DRIVINGR_GLANCE: u32 = 16795137;
pub const DIHATSWITCH_DRIVINGT_GLANCE: u32 = 50349569;
pub const DIHATSWITCH_FIGHTINGH_SLIDE: u32 = 134235649;
pub const DIHATSWITCH_FISHING_GLANCE: u32 = 234898945;
pub const DIHATSWITCH_FLYINGC_GLANCE: u32 = 67126785;
pub const DIHATSWITCH_FLYINGH_GLANCE: u32 = 100681217;
pub const DIHATSWITCH_FLYINGM_GLANCE: u32 = 83904001;
pub const DIHATSWITCH_FPS_GLANCE: u32 = 151012865;
pub const DIHATSWITCH_GOLF_SCROLL: u32 = 402671105;
pub const DIHATSWITCH_HOCKEYD_SCROLL: u32 = 436225537;
pub const DIHATSWITCH_HOCKEYG_SCROLL: u32 = 453002753;
pub const DIHATSWITCH_HOCKEYO_SCROLL: u32 = 419448321;
pub const DIHATSWITCH_HUNTING_GLANCE: u32 = 218121729;
pub const DIHATSWITCH_MECHA_GLANCE: u32 = 687883777;
pub const DIHATSWITCH_RACQUET_GLANCE: u32 = 536888833;
pub const DIHATSWITCH_SKIING_GLANCE: u32 = 486557185;
pub const DIHATSWITCH_SOCCERD_GLANCE: u32 = 520111617;
pub const DIHATSWITCH_SOCCERO_GLANCE: u32 = 503334401;
pub const DIHATSWITCH_SPACESIM_GLANCE: u32 = 117458433;
pub const DIHATSWITCH_STRATEGYR_GLANCE: u32 = 184567297;
pub const DIHATSWITCH_TPS_GLANCE: u32 = 167790081;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIHIDFFINITINFO {
    pub dwSize: u32,
    pub pwszDeviceInterface: windows_core::PWSTR,
    pub GuidInstance: windows_core::GUID,
}
pub const DIJC_CALLOUT: u32 = 8;
pub const DIJC_GAIN: u32 = 4;
pub const DIJC_GUIDINSTANCE: u32 = 1;
pub const DIJC_REGHWCONFIGTYPE: u32 = 2;
pub const DIJC_WDMGAMEPORT: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const DIJU_GAMEPORTEMULATOR: u32 = 4;
pub const DIJU_GLOBALDRIVER: u32 = 2;
pub const DIJU_USERVALUES: u32 = 1;
pub const DIKEYBOARD_0: u32 = 2164261899;
pub const DIKEYBOARD_1: u32 = 2164261890;
pub const DIKEYBOARD_2: u32 = 2164261891;
pub const DIKEYBOARD_3: u32 = 2164261892;
pub const DIKEYBOARD_4: u32 = 2164261893;
pub const DIKEYBOARD_5: u32 = 2164261894;
pub const DIKEYBOARD_6: u32 = 2164261895;
pub const DIKEYBOARD_7: u32 = 2164261896;
pub const DIKEYBOARD_8: u32 = 2164261897;
pub const DIKEYBOARD_9: u32 = 2164261898;
pub const DIKEYBOARD_A: u32 = 2164261918;
pub const DIKEYBOARD_ABNT_C1: u32 = 2164262003;
pub const DIKEYBOARD_ABNT_C2: u32 = 2164262014;
pub const DIKEYBOARD_ADD: u32 = 2164261966;
pub const DIKEYBOARD_APOSTROPHE: u32 = 2164261928;
pub const DIKEYBOARD_APPS: u32 = 2164262109;
pub const DIKEYBOARD_AT: u32 = 2164262033;
pub const DIKEYBOARD_AX: u32 = 2164262038;
pub const DIKEYBOARD_B: u32 = 2164261936;
pub const DIKEYBOARD_BACK: u32 = 2164261902;
pub const DIKEYBOARD_BACKSLASH: u32 = 2164261931;
pub const DIKEYBOARD_C: u32 = 2164261934;
pub const DIKEYBOARD_CALCULATOR: u32 = 2164262049;
pub const DIKEYBOARD_CAPITAL: u32 = 2164261946;
pub const DIKEYBOARD_COLON: u32 = 2164262034;
pub const DIKEYBOARD_COMMA: u32 = 2164261939;
pub const DIKEYBOARD_CONVERT: u32 = 2164262009;
pub const DIKEYBOARD_D: u32 = 2164261920;
pub const DIKEYBOARD_DECIMAL: u32 = 2164261971;
pub const DIKEYBOARD_DELETE: u32 = 2164262099;
pub const DIKEYBOARD_DIVIDE: u32 = 2164262069;
pub const DIKEYBOARD_DOWN: u32 = 2164262096;
pub const DIKEYBOARD_E: u32 = 2164261906;
pub const DIKEYBOARD_END: u32 = 2164262095;
pub const DIKEYBOARD_EQUALS: u32 = 2164261901;
pub const DIKEYBOARD_ESCAPE: u32 = 2164261889;
pub const DIKEYBOARD_F: u32 = 2164261921;
pub const DIKEYBOARD_F1: u32 = 2164261947;
pub const DIKEYBOARD_F10: u32 = 2164261956;
pub const DIKEYBOARD_F11: u32 = 2164261975;
pub const DIKEYBOARD_F12: u32 = 2164261976;
pub const DIKEYBOARD_F13: u32 = 2164261988;
pub const DIKEYBOARD_F14: u32 = 2164261989;
pub const DIKEYBOARD_F15: u32 = 2164261990;
pub const DIKEYBOARD_F2: u32 = 2164261948;
pub const DIKEYBOARD_F3: u32 = 2164261949;
pub const DIKEYBOARD_F4: u32 = 2164261950;
pub const DIKEYBOARD_F5: u32 = 2164261951;
pub const DIKEYBOARD_F6: u32 = 2164261952;
pub const DIKEYBOARD_F7: u32 = 2164261953;
pub const DIKEYBOARD_F8: u32 = 2164261954;
pub const DIKEYBOARD_F9: u32 = 2164261955;
pub const DIKEYBOARD_G: u32 = 2164261922;
pub const DIKEYBOARD_GRAVE: u32 = 2164261929;
pub const DIKEYBOARD_H: u32 = 2164261923;
pub const DIKEYBOARD_HOME: u32 = 2164262087;
pub const DIKEYBOARD_I: u32 = 2164261911;
pub const DIKEYBOARD_INSERT: u32 = 2164262098;
pub const DIKEYBOARD_J: u32 = 2164261924;
pub const DIKEYBOARD_K: u32 = 2164261925;
pub const DIKEYBOARD_KANA: u32 = 2164262000;
pub const DIKEYBOARD_KANJI: u32 = 2164262036;
pub const DIKEYBOARD_L: u32 = 2164261926;
pub const DIKEYBOARD_LBRACKET: u32 = 2164261914;
pub const DIKEYBOARD_LCONTROL: u32 = 2164261917;
pub const DIKEYBOARD_LEFT: u32 = 2164262091;
pub const DIKEYBOARD_LMENU: u32 = 2164261944;
pub const DIKEYBOARD_LSHIFT: u32 = 2164261930;
pub const DIKEYBOARD_LWIN: u32 = 2164262107;
pub const DIKEYBOARD_M: u32 = 2164261938;
pub const DIKEYBOARD_MAIL: u32 = 2164262124;
pub const DIKEYBOARD_MEDIASELECT: u32 = 2164262125;
pub const DIKEYBOARD_MEDIASTOP: u32 = 2164262052;
pub const DIKEYBOARD_MINUS: u32 = 2164261900;
pub const DIKEYBOARD_MULTIPLY: u32 = 2164261943;
pub const DIKEYBOARD_MUTE: u32 = 2164262048;
pub const DIKEYBOARD_MYCOMPUTER: u32 = 2164262123;
pub const DIKEYBOARD_N: u32 = 2164261937;
pub const DIKEYBOARD_NEXT: u32 = 2164262097;
pub const DIKEYBOARD_NEXTTRACK: u32 = 2164262041;
pub const DIKEYBOARD_NOCONVERT: u32 = 2164262011;
pub const DIKEYBOARD_NUMLOCK: u32 = 2164261957;
pub const DIKEYBOARD_NUMPAD0: u32 = 2164261970;
pub const DIKEYBOARD_NUMPAD1: u32 = 2164261967;
pub const DIKEYBOARD_NUMPAD2: u32 = 2164261968;
pub const DIKEYBOARD_NUMPAD3: u32 = 2164261969;
pub const DIKEYBOARD_NUMPAD4: u32 = 2164261963;
pub const DIKEYBOARD_NUMPAD5: u32 = 2164261964;
pub const DIKEYBOARD_NUMPAD6: u32 = 2164261965;
pub const DIKEYBOARD_NUMPAD7: u32 = 2164261959;
pub const DIKEYBOARD_NUMPAD8: u32 = 2164261960;
pub const DIKEYBOARD_NUMPAD9: u32 = 2164261961;
pub const DIKEYBOARD_NUMPADCOMMA: u32 = 2164262067;
pub const DIKEYBOARD_NUMPADENTER: u32 = 2164262044;
pub const DIKEYBOARD_NUMPADEQUALS: u32 = 2164262029;
pub const DIKEYBOARD_O: u32 = 2164261912;
pub const DIKEYBOARD_OEM_102: u32 = 2164261974;
pub const DIKEYBOARD_P: u32 = 2164261913;
pub const DIKEYBOARD_PAUSE: u32 = 2164262085;
pub const DIKEYBOARD_PERIOD: u32 = 2164261940;
pub const DIKEYBOARD_PLAYPAUSE: u32 = 2164262050;
pub const DIKEYBOARD_POWER: u32 = 2164262110;
pub const DIKEYBOARD_PREVTRACK: u32 = 2164262032;
pub const DIKEYBOARD_PRIOR: u32 = 2164262089;
pub const DIKEYBOARD_Q: u32 = 2164261904;
pub const DIKEYBOARD_R: u32 = 2164261907;
pub const DIKEYBOARD_RBRACKET: u32 = 2164261915;
pub const DIKEYBOARD_RCONTROL: u32 = 2164262045;
pub const DIKEYBOARD_RETURN: u32 = 2164261916;
pub const DIKEYBOARD_RIGHT: u32 = 2164262093;
pub const DIKEYBOARD_RMENU: u32 = 2164262072;
pub const DIKEYBOARD_RSHIFT: u32 = 2164261942;
pub const DIKEYBOARD_RWIN: u32 = 2164262108;
pub const DIKEYBOARD_S: u32 = 2164261919;
pub const DIKEYBOARD_SCROLL: u32 = 2164261958;
pub const DIKEYBOARD_SEMICOLON: u32 = 2164261927;
pub const DIKEYBOARD_SLASH: u32 = 2164261941;
pub const DIKEYBOARD_SLEEP: u32 = 2164262111;
pub const DIKEYBOARD_SPACE: u32 = 2164261945;
pub const DIKEYBOARD_STOP: u32 = 2164262037;
pub const DIKEYBOARD_SUBTRACT: u32 = 2164261962;
pub const DIKEYBOARD_SYSRQ: u32 = 2164262071;
pub const DIKEYBOARD_T: u32 = 2164261908;
pub const DIKEYBOARD_TAB: u32 = 2164261903;
pub const DIKEYBOARD_U: u32 = 2164261910;
pub const DIKEYBOARD_UNDERLINE: u32 = 2164262035;
pub const DIKEYBOARD_UNLABELED: u32 = 2164262039;
pub const DIKEYBOARD_UP: u32 = 2164262088;
pub const DIKEYBOARD_V: u32 = 2164261935;
pub const DIKEYBOARD_VOLUMEDOWN: u32 = 2164262062;
pub const DIKEYBOARD_VOLUMEUP: u32 = 2164262064;
pub const DIKEYBOARD_W: u32 = 2164261905;
pub const DIKEYBOARD_WAKE: u32 = 2164262115;
pub const DIKEYBOARD_WEBBACK: u32 = 2164262122;
pub const DIKEYBOARD_WEBFAVORITES: u32 = 2164262118;
pub const DIKEYBOARD_WEBFORWARD: u32 = 2164262121;
pub const DIKEYBOARD_WEBHOME: u32 = 2164262066;
pub const DIKEYBOARD_WEBREFRESH: u32 = 2164262119;
pub const DIKEYBOARD_WEBSEARCH: u32 = 2164262117;
pub const DIKEYBOARD_WEBSTOP: u32 = 2164262120;
pub const DIKEYBOARD_X: u32 = 2164261933;
pub const DIKEYBOARD_Y: u32 = 2164261909;
pub const DIKEYBOARD_YEN: u32 = 2164262013;
pub const DIKEYBOARD_Z: u32 = 2164261932;
pub const DIK_0: u32 = 11;
pub const DIK_1: u32 = 2;
pub const DIK_2: u32 = 3;
pub const DIK_3: u32 = 4;
pub const DIK_4: u32 = 5;
pub const DIK_5: u32 = 6;
pub const DIK_6: u32 = 7;
pub const DIK_7: u32 = 8;
pub const DIK_8: u32 = 9;
pub const DIK_9: u32 = 10;
pub const DIK_A: u32 = 30;
pub const DIK_ABNT_C1: u32 = 115;
pub const DIK_ABNT_C2: u32 = 126;
pub const DIK_ADD: u32 = 78;
pub const DIK_APOSTROPHE: u32 = 40;
pub const DIK_APPS: u32 = 221;
pub const DIK_AT: u32 = 145;
pub const DIK_AX: u32 = 150;
pub const DIK_B: u32 = 48;
pub const DIK_BACK: u32 = 14;
pub const DIK_BACKSLASH: u32 = 43;
pub const DIK_BACKSPACE: u32 = 14;
pub const DIK_C: u32 = 46;
pub const DIK_CALCULATOR: u32 = 161;
pub const DIK_CAPITAL: u32 = 58;
pub const DIK_CAPSLOCK: u32 = 58;
pub const DIK_CIRCUMFLEX: u32 = 144;
pub const DIK_COLON: u32 = 146;
pub const DIK_COMMA: u32 = 51;
pub const DIK_CONVERT: u32 = 121;
pub const DIK_D: u32 = 32;
pub const DIK_DECIMAL: u32 = 83;
pub const DIK_DELETE: u32 = 211;
pub const DIK_DIVIDE: u32 = 181;
pub const DIK_DOWN: u32 = 208;
pub const DIK_DOWNARROW: u32 = 208;
pub const DIK_E: u32 = 18;
pub const DIK_END: u32 = 207;
pub const DIK_EQUALS: u32 = 13;
pub const DIK_ESCAPE: u32 = 1;
pub const DIK_F: u32 = 33;
pub const DIK_F1: u32 = 59;
pub const DIK_F10: u32 = 68;
pub const DIK_F11: u32 = 87;
pub const DIK_F12: u32 = 88;
pub const DIK_F13: u32 = 100;
pub const DIK_F14: u32 = 101;
pub const DIK_F15: u32 = 102;
pub const DIK_F2: u32 = 60;
pub const DIK_F3: u32 = 61;
pub const DIK_F4: u32 = 62;
pub const DIK_F5: u32 = 63;
pub const DIK_F6: u32 = 64;
pub const DIK_F7: u32 = 65;
pub const DIK_F8: u32 = 66;
pub const DIK_F9: u32 = 67;
pub const DIK_G: u32 = 34;
pub const DIK_GRAVE: u32 = 41;
pub const DIK_H: u32 = 35;
pub const DIK_HOME: u32 = 199;
pub const DIK_I: u32 = 23;
pub const DIK_INSERT: u32 = 210;
pub const DIK_J: u32 = 36;
pub const DIK_K: u32 = 37;
pub const DIK_KANA: u32 = 112;
pub const DIK_KANJI: u32 = 148;
pub const DIK_L: u32 = 38;
pub const DIK_LALT: u32 = 56;
pub const DIK_LBRACKET: u32 = 26;
pub const DIK_LCONTROL: u32 = 29;
pub const DIK_LEFT: u32 = 203;
pub const DIK_LEFTARROW: u32 = 203;
pub const DIK_LMENU: u32 = 56;
pub const DIK_LSHIFT: u32 = 42;
pub const DIK_LWIN: u32 = 219;
pub const DIK_M: u32 = 50;
pub const DIK_MAIL: u32 = 236;
pub const DIK_MEDIASELECT: u32 = 237;
pub const DIK_MEDIASTOP: u32 = 164;
pub const DIK_MINUS: u32 = 12;
pub const DIK_MULTIPLY: u32 = 55;
pub const DIK_MUTE: u32 = 160;
pub const DIK_MYCOMPUTER: u32 = 235;
pub const DIK_N: u32 = 49;
pub const DIK_NEXT: u32 = 209;
pub const DIK_NEXTTRACK: u32 = 153;
pub const DIK_NOCONVERT: u32 = 123;
pub const DIK_NUMLOCK: u32 = 69;
pub const DIK_NUMPAD0: u32 = 82;
pub const DIK_NUMPAD1: u32 = 79;
pub const DIK_NUMPAD2: u32 = 80;
pub const DIK_NUMPAD3: u32 = 81;
pub const DIK_NUMPAD4: u32 = 75;
pub const DIK_NUMPAD5: u32 = 76;
pub const DIK_NUMPAD6: u32 = 77;
pub const DIK_NUMPAD7: u32 = 71;
pub const DIK_NUMPAD8: u32 = 72;
pub const DIK_NUMPAD9: u32 = 73;
pub const DIK_NUMPADCOMMA: u32 = 179;
pub const DIK_NUMPADENTER: u32 = 156;
pub const DIK_NUMPADEQUALS: u32 = 141;
pub const DIK_NUMPADMINUS: u32 = 74;
pub const DIK_NUMPADPERIOD: u32 = 83;
pub const DIK_NUMPADPLUS: u32 = 78;
pub const DIK_NUMPADSLASH: u32 = 181;
pub const DIK_NUMPADSTAR: u32 = 55;
pub const DIK_O: u32 = 24;
pub const DIK_OEM_102: u32 = 86;
pub const DIK_P: u32 = 25;
pub const DIK_PAUSE: u32 = 197;
pub const DIK_PERIOD: u32 = 52;
pub const DIK_PGDN: u32 = 209;
pub const DIK_PGUP: u32 = 201;
pub const DIK_PLAYPAUSE: u32 = 162;
pub const DIK_POWER: u32 = 222;
pub const DIK_PREVTRACK: u32 = 144;
pub const DIK_PRIOR: u32 = 201;
pub const DIK_Q: u32 = 16;
pub const DIK_R: u32 = 19;
pub const DIK_RALT: u32 = 184;
pub const DIK_RBRACKET: u32 = 27;
pub const DIK_RCONTROL: u32 = 157;
pub const DIK_RETURN: u32 = 28;
pub const DIK_RIGHT: u32 = 205;
pub const DIK_RIGHTARROW: u32 = 205;
pub const DIK_RMENU: u32 = 184;
pub const DIK_RSHIFT: u32 = 54;
pub const DIK_RWIN: u32 = 220;
pub const DIK_S: u32 = 31;
pub const DIK_SCROLL: u32 = 70;
pub const DIK_SEMICOLON: u32 = 39;
pub const DIK_SLASH: u32 = 53;
pub const DIK_SLEEP: u32 = 223;
pub const DIK_SPACE: u32 = 57;
pub const DIK_STOP: u32 = 149;
pub const DIK_SUBTRACT: u32 = 74;
pub const DIK_SYSRQ: u32 = 183;
pub const DIK_T: u32 = 20;
pub const DIK_TAB: u32 = 15;
pub const DIK_U: u32 = 22;
pub const DIK_UNDERLINE: u32 = 147;
pub const DIK_UNLABELED: u32 = 151;
pub const DIK_UP: u32 = 200;
pub const DIK_UPARROW: u32 = 200;
pub const DIK_V: u32 = 47;
pub const DIK_VOLUMEDOWN: u32 = 174;
pub const DIK_VOLUMEUP: u32 = 176;
pub const DIK_W: u32 = 17;
pub const DIK_WAKE: u32 = 227;
pub const DIK_WEBBACK: u32 = 234;
pub const DIK_WEBFAVORITES: u32 = 230;
pub const DIK_WEBFORWARD: u32 = 233;
pub const DIK_WEBHOME: u32 = 178;
pub const DIK_WEBREFRESH: u32 = 231;
pub const DIK_WEBSEARCH: u32 = 229;
pub const DIK_WEBSTOP: u32 = 232;
pub const DIK_X: u32 = 45;
pub const DIK_Y: u32 = 21;
pub const DIK_YEN: u32 = 125;
pub const DIK_Z: u32 = 44;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const DIMSGWP_DX8APPSTART: u32 = 2;
pub const DIMSGWP_DX8MAPPERAPPSTART: u32 = 3;
pub const DIMSGWP_NEWAPPSTART: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIOBJECTATTRIBUTES {
    pub dwFlags: u32,
    pub wUsagePage: u16,
    pub wUsage: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIOBJECTCALIBRATION {
    pub lMin: i32,
    pub lCenter: i32,
    pub lMax: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIPERIODIC {
    pub dwMagnitude: u32,
    pub lOffset: i32,
    pub dwPhase: u32,
    pub dwPeriod: u32,
}
pub const DIPH_BYID: u32 = 2;
pub const DIPH_BYOFFSET: u32 = 1;
pub const DIPH_BYUSAGE: u32 = 3;
pub const DIPH_DEVICE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DIPOVCALIBRATION {
    pub lMin: [i32; 5],
    pub lMax: [i32; 5],
}
impl Default for DIPOVCALIBRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIPOV_ANY_1: u32 = 4278208001;
pub const DIPOV_ANY_2: u32 = 4278208002;
pub const DIPOV_ANY_3: u32 = 4278208003;
pub const DIPOV_ANY_4: u32 = 4278208004;
pub const DIPROPAUTOCENTER_OFF: u32 = 0;
pub const DIPROPAUTOCENTER_ON: u32 = 1;
pub const DIPROPAXISMODE_ABS: u32 = 0;
pub const DIPROPAXISMODE_REL: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIPROPCAL {
    pub diph: DIPROPHEADER,
    pub lMin: i32,
    pub lCenter: i32,
    pub lMax: i32,
}
pub const DIPROPCALIBRATIONMODE_COOKED: u32 = 0;
pub const DIPROPCALIBRATIONMODE_RAW: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIPROPDWORD {
    pub diph: DIPROPHEADER,
    pub dwData: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIPROPHEADER {
    pub dwSize: u32,
    pub dwHeaderSize: u32,
    pub dwObj: u32,
    pub dwHow: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIPROPPOINTER {
    pub diph: DIPROPHEADER,
    pub uData: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIPROPRANGE {
    pub diph: DIPROPHEADER,
    pub lMin: i32,
    pub lMax: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DIRAMPFORCE {
    pub lStart: i32,
    pub lEnd: i32,
}
pub const DIRECTINPUT_HEADER_VERSION: u32 = 2048;
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
pub const DIRECTINPUT_VERSION: u32 = 2048;
pub const DISCL_BACKGROUND: u32 = 8;
pub const DISCL_EXCLUSIVE: u32 = 1;
pub const DISCL_FOREGROUND: u32 = 4;
pub const DISCL_NONEXCLUSIVE: u32 = 2;
pub const DISCL_NOWINKEY: u32 = 16;
pub const DISDD_CONTINUE: u32 = 1;
pub const DISFFC_CONTINUE: u32 = 8;
pub const DISFFC_PAUSE: u32 = 4;
pub const DISFFC_RESET: u32 = 1;
pub const DISFFC_SETACTUATORSOFF: u32 = 32;
pub const DISFFC_SETACTUATORSON: u32 = 16;
pub const DISFFC_STOPALL: u32 = 2;
pub const DITC_CALLOUT: u32 = 8;
pub const DITC_CLSIDCONFIG: u32 = 2;
pub const DITC_DISPLAYNAME: u32 = 4;
pub const DITC_FLAGS1: u32 = 32;
pub const DITC_FLAGS2: u32 = 64;
pub const DITC_HARDWAREID: u32 = 16;
pub const DITC_MAPFILE: u32 = 128;
pub const DITC_REGHWSETTINGS: u32 = 1;
pub const DIVIRTUAL_ARCADE_PLATFORM: u32 = 570425344;
pub const DIVIRTUAL_ARCADE_SIDE2SIDE: u32 = 553648128;
pub const DIVIRTUAL_BROWSER_CONTROL: u32 = 671088640;
pub const DIVIRTUAL_CAD_2DCONTROL: u32 = 587202560;
pub const DIVIRTUAL_CAD_3DCONTROL: u32 = 603979776;
pub const DIVIRTUAL_CAD_FLYBY: u32 = 620756992;
pub const DIVIRTUAL_CAD_MODEL: u32 = 637534208;
pub const DIVIRTUAL_DRIVING_COMBAT: u32 = 33554432;
pub const DIVIRTUAL_DRIVING_MECHA: u32 = 687865856;
pub const DIVIRTUAL_DRIVING_RACE: u32 = 16777216;
pub const DIVIRTUAL_DRIVING_TANK: u32 = 50331648;
pub const DIVIRTUAL_FIGHTING_FPS: u32 = 150994944;
pub const DIVIRTUAL_FIGHTING_HAND2HAND: u32 = 134217728;
pub const DIVIRTUAL_FIGHTING_THIRDPERSON: u32 = 167772160;
pub const DIVIRTUAL_FLYING_CIVILIAN: u32 = 67108864;
pub const DIVIRTUAL_FLYING_HELICOPTER: u32 = 100663296;
pub const DIVIRTUAL_FLYING_MILITARY: u32 = 83886080;
pub const DIVIRTUAL_REMOTE_CONTROL: u32 = 654311424;
pub const DIVIRTUAL_SPACESIM: u32 = 117440512;
pub const DIVIRTUAL_SPORTS_BASEBALL_BAT: u32 = 251658240;
pub const DIVIRTUAL_SPORTS_BASEBALL_FIELD: u32 = 285212672;
pub const DIVIRTUAL_SPORTS_BASEBALL_PITCH: u32 = 268435456;
pub const DIVIRTUAL_SPORTS_BASKETBALL_DEFENSE: u32 = 318767104;
pub const DIVIRTUAL_SPORTS_BASKETBALL_OFFENSE: u32 = 301989888;
pub const DIVIRTUAL_SPORTS_BIKING_MOUNTAIN: u32 = 469762048;
pub const DIVIRTUAL_SPORTS_FISHING: u32 = 234881024;
pub const DIVIRTUAL_SPORTS_FOOTBALL_DEFENSE: u32 = 385875968;
pub const DIVIRTUAL_SPORTS_FOOTBALL_FIELD: u32 = 335544320;
pub const DIVIRTUAL_SPORTS_FOOTBALL_OFFENSE: u32 = 369098752;
pub const DIVIRTUAL_SPORTS_FOOTBALL_QBCK: u32 = 352321536;
pub const DIVIRTUAL_SPORTS_GOLF: u32 = 402653184;
pub const DIVIRTUAL_SPORTS_HOCKEY_DEFENSE: u32 = 436207616;
pub const DIVIRTUAL_SPORTS_HOCKEY_GOALIE: u32 = 452984832;
pub const DIVIRTUAL_SPORTS_HOCKEY_OFFENSE: u32 = 419430400;
pub const DIVIRTUAL_SPORTS_HUNTING: u32 = 218103808;
pub const DIVIRTUAL_SPORTS_RACQUET: u32 = 536870912;
pub const DIVIRTUAL_SPORTS_SKIING: u32 = 486539264;
pub const DIVIRTUAL_SPORTS_SOCCER_DEFENSE: u32 = 520093696;
pub const DIVIRTUAL_SPORTS_SOCCER_OFFENSE: u32 = 503316480;
pub const DIVIRTUAL_STRATEGY_ROLEPLAYING: u32 = 184549376;
pub const DIVIRTUAL_STRATEGY_TURN: u32 = 201326592;
pub const DIVOICE_ALL: u32 = 2197816330;
pub const DIVOICE_CHANNEL1: u32 = 2197816321;
pub const DIVOICE_CHANNEL2: u32 = 2197816322;
pub const DIVOICE_CHANNEL3: u32 = 2197816323;
pub const DIVOICE_CHANNEL4: u32 = 2197816324;
pub const DIVOICE_CHANNEL5: u32 = 2197816325;
pub const DIVOICE_CHANNEL6: u32 = 2197816326;
pub const DIVOICE_CHANNEL7: u32 = 2197816327;
pub const DIVOICE_CHANNEL8: u32 = 2197816328;
pub const DIVOICE_PLAYBACKMUTE: u32 = 2197816332;
pub const DIVOICE_RECORDMUTE: u32 = 2197816331;
pub const DIVOICE_TEAM: u32 = 2197816329;
pub const DIVOICE_TRANSMIT: u32 = 2197816333;
pub const DIVOICE_VOICECOMMAND: u32 = 2197816336;
pub const DI_BUFFEROVERFLOW: i32 = 1;
pub const DI_DEGREES: u32 = 100;
pub const DI_DOWNLOADSKIPPED: windows_core::HRESULT = windows_core::HRESULT(0x3_u32 as _);
pub const DI_EFFECTRESTARTED: windows_core::HRESULT = windows_core::HRESULT(0x4_u32 as _);
pub const DI_FFNOMINALMAX: u32 = 10000;
pub const DI_NOEFFECT: i32 = 1;
pub const DI_NOTATTACHED: i32 = 1;
pub const DI_OK: i32 = 0;
pub const DI_POLLEDDEVICE: windows_core::HRESULT = windows_core::HRESULT(0x2_u32 as _);
pub const DI_PROPNOEFFECT: i32 = 1;
pub const DI_SECONDS: u32 = 1000000;
pub const DI_SETTINGSNOTSAVED: windows_core::HRESULT = windows_core::HRESULT(0xB_u32 as _);
pub const DI_TRUNCATED: windows_core::HRESULT = windows_core::HRESULT(0x8_u32 as _);
pub const DI_TRUNCATEDANDRESTARTED: windows_core::HRESULT = windows_core::HRESULT(0xC_u32 as _);
pub const DI_WRITEPROTECT: windows_core::HRESULT = windows_core::HRESULT(0x13_u32 as _);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GPIOBUTTONS_BUTTON_TYPE(pub i32);
pub const GPIO_BUTTON_BACK: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(5);
pub const GPIO_BUTTON_CAMERA_FOCUS: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(7);
pub const GPIO_BUTTON_CAMERA_LENS: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(12);
pub const GPIO_BUTTON_CAMERA_SHUTTER: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(8);
pub const GPIO_BUTTON_COUNT: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(16);
pub const GPIO_BUTTON_COUNT_MIN: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(5);
pub const GPIO_BUTTON_HEADSET: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(10);
pub const GPIO_BUTTON_HWKB_DEPLOY: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(11);
pub const GPIO_BUTTON_OEM_CUSTOM: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(13);
pub const GPIO_BUTTON_OEM_CUSTOM2: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(14);
pub const GPIO_BUTTON_OEM_CUSTOM3: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(15);
pub const GPIO_BUTTON_POWER: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(0);
pub const GPIO_BUTTON_RINGER_TOGGLE: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(9);
pub const GPIO_BUTTON_ROTATION_LOCK: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(4);
pub const GPIO_BUTTON_SEARCH: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(6);
pub const GPIO_BUTTON_VOLUME_DOWN: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(3);
pub const GPIO_BUTTON_VOLUME_UP: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(2);
pub const GPIO_BUTTON_WINDOWS: GPIOBUTTONS_BUTTON_TYPE = GPIOBUTTONS_BUTTON_TYPE(1);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_BAD_LOG_PHY_VALUES: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0110006_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_BUFFER_TOO_SMALL: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0110007_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_BUTTON_NOT_PRESSED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC011000F_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_DATA_INDEX_NOT_FOUND: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC011000D_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_DATA_INDEX_OUT_OF_RANGE: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC011000E_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_I8042_TRANS_UNKNOWN: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0110009_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_I8242_TRANS_UNKNOWN: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0110009_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_INCOMPATIBLE_REPORT_ID: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC011000A_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_INTERNAL_ERROR: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0110008_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_INVALID_PREPARSED_DATA: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0110001_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_INVALID_REPORT_LENGTH: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0110003_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_INVALID_REPORT_TYPE: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0110002_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_IS_VALUE_ARRAY: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC011000C_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_NOT_BUTTON_ARRAY: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0110021_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_NOT_IMPLEMENTED: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0110020_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_NOT_VALUE_ARRAY: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC011000B_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_NULL: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0x80110001_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_REPORT_DOES_NOT_EXIST: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0110010_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_SUCCESS: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0x110000_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_USAGE_NOT_FOUND: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0110004_u32 as _);
#[cfg(feature = "Win32_Foundation")]
pub const HIDP_STATUS_VALUE_OUT_OF_RANGE: super::super::Foundation::NTSTATUS = super::super::Foundation::NTSTATUS(0xC0110005_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HID_DRIVER_CONFIG {
    pub Size: u32,
    pub RingBufferSize: u32,
}
pub const HID_REVISION: u32 = 1;
pub const HID_USAGE_ALPHANUMERIC_14_SEGMENT_DIRECT_MAP: u16 = 69;
pub const HID_USAGE_ALPHANUMERIC_7_SEGMENT_DIRECT_MAP: u16 = 67;
pub const HID_USAGE_ALPHANUMERIC_ALPHANUMERIC_DISPLAY: u16 = 1;
pub const HID_USAGE_ALPHANUMERIC_ASCII_CHARACTER_SET: u16 = 33;
pub const HID_USAGE_ALPHANUMERIC_ATTRIBUTE_DATA: u16 = 74;
pub const HID_USAGE_ALPHANUMERIC_ATTRIBUTE_READBACK: u16 = 73;
pub const HID_USAGE_ALPHANUMERIC_BITMAPPED_DISPLAY: u16 = 2;
pub const HID_USAGE_ALPHANUMERIC_BITMAP_SIZE_X: u16 = 128;
pub const HID_USAGE_ALPHANUMERIC_BITMAP_SIZE_Y: u16 = 129;
pub const HID_USAGE_ALPHANUMERIC_BIT_DEPTH_FORMAT: u16 = 131;
pub const HID_USAGE_ALPHANUMERIC_BLIT_DATA: u16 = 143;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_X1: u16 = 139;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_X2: u16 = 141;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_Y1: u16 = 140;
pub const HID_USAGE_ALPHANUMERIC_BLIT_RECTANGLE_Y2: u16 = 142;
pub const HID_USAGE_ALPHANUMERIC_BLIT_REPORT: u16 = 138;
pub const HID_USAGE_ALPHANUMERIC_CHARACTER_ATTRIBUTE: u16 = 72;
pub const HID_USAGE_ALPHANUMERIC_CHARACTER_REPORT: u16 = 43;
pub const HID_USAGE_ALPHANUMERIC_CHAR_ATTR_BLINK: u16 = 77;
pub const HID_USAGE_ALPHANUMERIC_CHAR_ATTR_ENHANCE: u16 = 75;
pub const HID_USAGE_ALPHANUMERIC_CHAR_ATTR_UNDERLINE: u16 = 76;
pub const HID_USAGE_ALPHANUMERIC_CHAR_HEIGHT: u16 = 62;
pub const HID_USAGE_ALPHANUMERIC_CHAR_SPACING_HORIZONTAL: u16 = 63;
pub const HID_USAGE_ALPHANUMERIC_CHAR_SPACING_VERTICAL: u16 = 64;
pub const HID_USAGE_ALPHANUMERIC_CHAR_WIDTH: u16 = 61;
pub const HID_USAGE_ALPHANUMERIC_CLEAR_DISPLAY: u16 = 37;
pub const HID_USAGE_ALPHANUMERIC_COLUMN: u16 = 52;
pub const HID_USAGE_ALPHANUMERIC_COLUMNS: u16 = 54;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_BLINK: u16 = 58;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_ENABLE: u16 = 57;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_MODE: u16 = 56;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_PIXEL_POSITIONING: u16 = 55;
pub const HID_USAGE_ALPHANUMERIC_CURSOR_POSITION_REPORT: u16 = 50;
pub const HID_USAGE_ALPHANUMERIC_DATA_READ_BACK: u16 = 34;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_ATTRIBUTES_REPORT: u16 = 32;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_BRIGHTNESS: u16 = 70;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_CONTRAST: u16 = 71;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_CONTROL_REPORT: u16 = 36;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_DATA: u16 = 44;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_ENABLE: u16 = 38;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_ORIENTATION: u16 = 132;
pub const HID_USAGE_ALPHANUMERIC_DISPLAY_STATUS: u16 = 45;
pub const HID_USAGE_ALPHANUMERIC_ERR_FONT_DATA_CANNOT_BE_READ: u16 = 49;
pub const HID_USAGE_ALPHANUMERIC_ERR_NOT_A_LOADABLE_CHARACTER: u16 = 48;
pub const HID_USAGE_ALPHANUMERIC_FONT_14_SEGMENT: u16 = 68;
pub const HID_USAGE_ALPHANUMERIC_FONT_7_SEGMENT: u16 = 66;
pub const HID_USAGE_ALPHANUMERIC_FONT_DATA: u16 = 60;
pub const HID_USAGE_ALPHANUMERIC_FONT_READ_BACK: u16 = 35;
pub const HID_USAGE_ALPHANUMERIC_FONT_REPORT: u16 = 59;
pub const HID_USAGE_ALPHANUMERIC_HORIZONTAL_SCROLL: u16 = 42;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_DATA: u16 = 136;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_DATA_OFFSET: u16 = 135;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_DATA_SIZE: u16 = 134;
pub const HID_USAGE_ALPHANUMERIC_PALETTE_REPORT: u16 = 133;
pub const HID_USAGE_ALPHANUMERIC_ROW: u16 = 51;
pub const HID_USAGE_ALPHANUMERIC_ROWS: u16 = 53;
pub const HID_USAGE_ALPHANUMERIC_SCREEN_SAVER_DELAY: u16 = 39;
pub const HID_USAGE_ALPHANUMERIC_SCREEN_SAVER_ENABLE: u16 = 40;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON: u16 = 144;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_ID: u16 = 145;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_OFFSET1: u16 = 147;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_OFFSET2: u16 = 148;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_REPORT: u16 = 149;
pub const HID_USAGE_ALPHANUMERIC_SOFT_BUTTON_SIDE: u16 = 146;
pub const HID_USAGE_ALPHANUMERIC_STATUS_NOT_READY: u16 = 46;
pub const HID_USAGE_ALPHANUMERIC_STATUS_READY: u16 = 47;
pub const HID_USAGE_ALPHANUMERIC_UNICODE_CHAR_SET: u16 = 65;
pub const HID_USAGE_ALPHANUMERIC_VERTICAL_SCROLL: u16 = 41;
pub const HID_USAGE_CAMERA_AUTO_FOCUS: u16 = 32;
pub const HID_USAGE_CAMERA_SHUTTER: u16 = 33;
pub const HID_USAGE_CONSUMERCTRL: u16 = 1;
pub const HID_USAGE_CONSUMER_AC_BACK: u16 = 548;
pub const HID_USAGE_CONSUMER_AC_BOOKMARKS: u16 = 554;
pub const HID_USAGE_CONSUMER_AC_FORWARD: u16 = 549;
pub const HID_USAGE_CONSUMER_AC_GOTO: u16 = 546;
pub const HID_USAGE_CONSUMER_AC_HOME: u16 = 547;
pub const HID_USAGE_CONSUMER_AC_NEXT: u16 = 553;
pub const HID_USAGE_CONSUMER_AC_PAN: u16 = 568;
pub const HID_USAGE_CONSUMER_AC_PREVIOUS: u16 = 552;
pub const HID_USAGE_CONSUMER_AC_REFRESH: u16 = 551;
pub const HID_USAGE_CONSUMER_AC_SEARCH: u16 = 545;
pub const HID_USAGE_CONSUMER_AC_STOP: u16 = 550;
pub const HID_USAGE_CONSUMER_AL_BROWSER: u16 = 404;
pub const HID_USAGE_CONSUMER_AL_CALCULATOR: u16 = 402;
pub const HID_USAGE_CONSUMER_AL_CONFIGURATION: u16 = 387;
pub const HID_USAGE_CONSUMER_AL_EMAIL: u16 = 394;
pub const HID_USAGE_CONSUMER_AL_SEARCH: u16 = 454;
pub const HID_USAGE_CONSUMER_BALANCE: u16 = 225;
pub const HID_USAGE_CONSUMER_BASS: u16 = 227;
pub const HID_USAGE_CONSUMER_BASS_BOOST: u16 = 229;
pub const HID_USAGE_CONSUMER_BASS_DECREMENT: u16 = 339;
pub const HID_USAGE_CONSUMER_BASS_INCREMENT: u16 = 338;
pub const HID_USAGE_CONSUMER_CHANNEL_DECREMENT: u16 = 157;
pub const HID_USAGE_CONSUMER_CHANNEL_INCREMENT: u16 = 156;
pub const HID_USAGE_CONSUMER_EXTENDED_KEYBOARD_ATTRIBUTES_COLLECTION: u16 = 704;
pub const HID_USAGE_CONSUMER_FAST_FORWARD: u16 = 179;
pub const HID_USAGE_CONSUMER_GAMEDVR_OPEN_GAMEBAR: u16 = 208;
pub const HID_USAGE_CONSUMER_GAMEDVR_RECORD_CLIP: u16 = 210;
pub const HID_USAGE_CONSUMER_GAMEDVR_SCREENSHOT: u16 = 211;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_BROADCAST: u16 = 215;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_CAMERA: u16 = 214;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_INDICATOR: u16 = 212;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_MICROPHONE: u16 = 213;
pub const HID_USAGE_CONSUMER_GAMEDVR_TOGGLE_RECORD: u16 = 209;
pub const HID_USAGE_CONSUMER_IMPLEMENTED_KEYBOARD_INPUT_ASSIST_CONTROLS: u16 = 710;
pub const HID_USAGE_CONSUMER_KEYBOARD_FORM_FACTOR: u16 = 705;
pub const HID_USAGE_CONSUMER_KEYBOARD_IETF_LANGUAGE_TAG_INDEX: u16 = 709;
pub const HID_USAGE_CONSUMER_KEYBOARD_KEY_TYPE: u16 = 706;
pub const HID_USAGE_CONSUMER_KEYBOARD_PHYSICAL_LAYOUT: u16 = 707;
pub const HID_USAGE_CONSUMER_LOUDNESS: u16 = 231;
pub const HID_USAGE_CONSUMER_MPX: u16 = 232;
pub const HID_USAGE_CONSUMER_MUTE: u16 = 226;
pub const HID_USAGE_CONSUMER_PAUSE: u16 = 177;
pub const HID_USAGE_CONSUMER_PLAY: u16 = 176;
pub const HID_USAGE_CONSUMER_PLAY_PAUSE: u16 = 205;
pub const HID_USAGE_CONSUMER_RECORD: u16 = 178;
pub const HID_USAGE_CONSUMER_REWIND: u16 = 180;
pub const HID_USAGE_CONSUMER_SCAN_NEXT_TRACK: u16 = 181;
pub const HID_USAGE_CONSUMER_SCAN_PREV_TRACK: u16 = 182;
pub const HID_USAGE_CONSUMER_STOP: u16 = 183;
pub const HID_USAGE_CONSUMER_SURROUND_MODE: u16 = 230;
pub const HID_USAGE_CONSUMER_TREBLE: u16 = 228;
pub const HID_USAGE_CONSUMER_TREBLE_DECREMENT: u16 = 341;
pub const HID_USAGE_CONSUMER_TREBLE_INCREMENT: u16 = 340;
pub const HID_USAGE_CONSUMER_VENDOR_SPECIFIC_KEYBOARD_PHYSICAL_LAYOUT: u16 = 708;
pub const HID_USAGE_CONSUMER_VOLUME: u16 = 224;
pub const HID_USAGE_CONSUMER_VOLUME_DECREMENT: u16 = 234;
pub const HID_USAGE_CONSUMER_VOLUME_INCREMENT: u16 = 233;
pub const HID_USAGE_DIGITIZER_3D_DIGITIZER: u16 = 8;
pub const HID_USAGE_DIGITIZER_ALTITUDE: u16 = 64;
pub const HID_USAGE_DIGITIZER_ARMATURE: u16 = 11;
pub const HID_USAGE_DIGITIZER_ARTICULATED_ARM: u16 = 10;
pub const HID_USAGE_DIGITIZER_AZIMUTH: u16 = 63;
pub const HID_USAGE_DIGITIZER_BARREL_PRESSURE: u16 = 49;
pub const HID_USAGE_DIGITIZER_BARREL_SWITCH: u16 = 68;
pub const HID_USAGE_DIGITIZER_BATTERY_STRENGTH: u16 = 59;
pub const HID_USAGE_DIGITIZER_COORD_MEASURING: u16 = 7;
pub const HID_USAGE_DIGITIZER_DATA_VALID: u16 = 55;
pub const HID_USAGE_DIGITIZER_DIGITIZER: u16 = 1;
pub const HID_USAGE_DIGITIZER_ERASER: u16 = 69;
pub const HID_USAGE_DIGITIZER_FINGER: u16 = 34;
pub const HID_USAGE_DIGITIZER_FREE_SPACE_WAND: u16 = 13;
pub const HID_USAGE_DIGITIZER_HEAT_MAP: u16 = 15;
pub const HID_USAGE_DIGITIZER_HEAT_MAP_FRAME_DATA: u16 = 108;
pub const HID_USAGE_DIGITIZER_HEAT_MAP_PROTOCOL_VENDOR_ID: u16 = 106;
pub const HID_USAGE_DIGITIZER_HEAT_MAP_PROTOCOL_VERSION: u16 = 107;
pub const HID_USAGE_DIGITIZER_INVERT: u16 = 60;
pub const HID_USAGE_DIGITIZER_IN_RANGE: u16 = 50;
pub const HID_USAGE_DIGITIZER_LIGHT_PEN: u16 = 3;
pub const HID_USAGE_DIGITIZER_MULTI_POINT: u16 = 12;
pub const HID_USAGE_DIGITIZER_PEN: u16 = 2;
pub const HID_USAGE_DIGITIZER_PROG_CHANGE_KEYS: u16 = 58;
pub const HID_USAGE_DIGITIZER_PUCK: u16 = 33;
pub const HID_USAGE_DIGITIZER_QUALITY: u16 = 54;
pub const HID_USAGE_DIGITIZER_SECONDARY_TIP_SWITCH: u16 = 67;
pub const HID_USAGE_DIGITIZER_STEREO_PLOTTER: u16 = 9;
pub const HID_USAGE_DIGITIZER_STYLUS: u16 = 32;
pub const HID_USAGE_DIGITIZER_TABLET_FUNC_KEYS: u16 = 57;
pub const HID_USAGE_DIGITIZER_TABLET_PICK: u16 = 70;
pub const HID_USAGE_DIGITIZER_TAP: u16 = 53;
pub const HID_USAGE_DIGITIZER_TIP_PRESSURE: u16 = 48;
pub const HID_USAGE_DIGITIZER_TIP_SWITCH: u16 = 66;
pub const HID_USAGE_DIGITIZER_TOUCH: u16 = 51;
pub const HID_USAGE_DIGITIZER_TOUCH_PAD: u16 = 5;
pub const HID_USAGE_DIGITIZER_TOUCH_SCREEN: u16 = 4;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_CONNECTED: u16 = 162;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_INDEX: u16 = 56;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_PRODUCT: u16 = 146;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SERIAL: u16 = 91;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_SERIAL_PART2: u16 = 110;
pub const HID_USAGE_DIGITIZER_TRANSDUCER_VENDOR: u16 = 145;
pub const HID_USAGE_DIGITIZER_TWIST: u16 = 65;
pub const HID_USAGE_DIGITIZER_UNTOUCH: u16 = 52;
pub const HID_USAGE_DIGITIZER_WHITE_BOARD: u16 = 6;
pub const HID_USAGE_DIGITIZER_X_TILT: u16 = 61;
pub const HID_USAGE_DIGITIZER_Y_TILT: u16 = 62;
pub const HID_USAGE_GAME_3D_GAME_CONTROLLER: u16 = 1;
pub const HID_USAGE_GAME_BUMP: u16 = 44;
pub const HID_USAGE_GAME_FLIPPER: u16 = 42;
pub const HID_USAGE_GAME_GAMEPAD_FIRE_JUMP: u16 = 55;
pub const HID_USAGE_GAME_GAMEPAD_TRIGGER: u16 = 57;
pub const HID_USAGE_GAME_GUN_AUTOMATIC: u16 = 53;
pub const HID_USAGE_GAME_GUN_BOLT: u16 = 48;
pub const HID_USAGE_GAME_GUN_BURST: u16 = 52;
pub const HID_USAGE_GAME_GUN_CLIP: u16 = 49;
pub const HID_USAGE_GAME_GUN_DEVICE: u16 = 3;
pub const HID_USAGE_GAME_GUN_SAFETY: u16 = 54;
pub const HID_USAGE_GAME_GUN_SELECTOR: u16 = 50;
pub const HID_USAGE_GAME_GUN_SINGLE_SHOT: u16 = 51;
pub const HID_USAGE_GAME_LEAN_FORWARD_BACK: u16 = 40;
pub const HID_USAGE_GAME_LEAN_RIGHT_LEFT: u16 = 39;
pub const HID_USAGE_GAME_MOVE_FORWARD_BACK: u16 = 37;
pub const HID_USAGE_GAME_MOVE_RIGHT_LEFT: u16 = 36;
pub const HID_USAGE_GAME_MOVE_UP_DOWN: u16 = 38;
pub const HID_USAGE_GAME_NEW_GAME: u16 = 45;
pub const HID_USAGE_GAME_PINBALL_DEVICE: u16 = 2;
pub const HID_USAGE_GAME_PITCH_FORWARD_BACK: u16 = 34;
pub const HID_USAGE_GAME_PLAYER: u16 = 47;
pub const HID_USAGE_GAME_POINT_OF_VIEW: u16 = 32;
pub const HID_USAGE_GAME_POV_HEIGHT: u16 = 41;
pub const HID_USAGE_GAME_ROLL_RIGHT_LEFT: u16 = 35;
pub const HID_USAGE_GAME_SECONDARY_FLIPPER: u16 = 43;
pub const HID_USAGE_GAME_SHOOT_BALL: u16 = 46;
pub const HID_USAGE_GAME_TURN_RIGHT_LEFT: u16 = 33;
pub const HID_USAGE_GENERIC_BYTE_COUNT: u16 = 59;
pub const HID_USAGE_GENERIC_CONTROL_ENABLE: u16 = 203;
pub const HID_USAGE_GENERIC_COUNTED_BUFFER: u16 = 58;
pub const HID_USAGE_GENERIC_DEVICE_BATTERY_STRENGTH: u16 = 32;
pub const HID_USAGE_GENERIC_DEVICE_DISCOVER_WIRELESS_CONTROL: u16 = 35;
pub const HID_USAGE_GENERIC_DEVICE_SECURITY_CODE_CHAR_ENTERED: u16 = 36;
pub const HID_USAGE_GENERIC_DEVICE_SECURITY_CODE_CHAR_ERASED: u16 = 37;
pub const HID_USAGE_GENERIC_DEVICE_SECURITY_CODE_CLEARED: u16 = 38;
pub const HID_USAGE_GENERIC_DEVICE_WIRELESS_CHANNEL: u16 = 33;
pub const HID_USAGE_GENERIC_DEVICE_WIRELESS_ID: u16 = 34;
pub const HID_USAGE_GENERIC_DIAL: u16 = 55;
pub const HID_USAGE_GENERIC_DPAD_DOWN: u16 = 145;
pub const HID_USAGE_GENERIC_DPAD_LEFT: u16 = 147;
pub const HID_USAGE_GENERIC_DPAD_RIGHT: u16 = 146;
pub const HID_USAGE_GENERIC_DPAD_UP: u16 = 144;
pub const HID_USAGE_GENERIC_FEATURE_NOTIFICATION: u16 = 71;
pub const HID_USAGE_GENERIC_GAMEPAD: u16 = 5;
pub const HID_USAGE_GENERIC_HATSWITCH: u16 = 57;
pub const HID_USAGE_GENERIC_INTERACTIVE_CONTROL: u16 = 14;
pub const HID_USAGE_GENERIC_JOYSTICK: u16 = 4;
pub const HID_USAGE_GENERIC_KEYBOARD: u16 = 6;
pub const HID_USAGE_GENERIC_KEYPAD: u16 = 7;
pub const HID_USAGE_GENERIC_MOTION_WAKEUP: u16 = 60;
pub const HID_USAGE_GENERIC_MOUSE: u16 = 2;
pub const HID_USAGE_GENERIC_MULTI_AXIS_CONTROLLER: u16 = 8;
pub const HID_USAGE_GENERIC_POINTER: u16 = 1;
pub const HID_USAGE_GENERIC_PORTABLE_DEVICE_CONTROL: u16 = 13;
pub const HID_USAGE_GENERIC_RESOLUTION_MULTIPLIER: u16 = 72;
pub const HID_USAGE_GENERIC_RX: u16 = 51;
pub const HID_USAGE_GENERIC_RY: u16 = 52;
pub const HID_USAGE_GENERIC_RZ: u16 = 53;
pub const HID_USAGE_GENERIC_SELECT: u16 = 62;
pub const HID_USAGE_GENERIC_SLIDER: u16 = 54;
pub const HID_USAGE_GENERIC_START: u16 = 61;
pub const HID_USAGE_GENERIC_SYSCTL_APP_BREAK: u16 = 165;
pub const HID_USAGE_GENERIC_SYSCTL_APP_DBG_BREAK: u16 = 166;
pub const HID_USAGE_GENERIC_SYSCTL_APP_MENU: u16 = 134;
pub const HID_USAGE_GENERIC_SYSCTL_COLD_RESTART: u16 = 142;
pub const HID_USAGE_GENERIC_SYSCTL_CONTEXT_MENU: u16 = 132;
pub const HID_USAGE_GENERIC_SYSCTL_DISMISS_NOTIFICATION: u16 = 154;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_AUTOSCALE: u16 = 183;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_BOTH: u16 = 179;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_DUAL: u16 = 180;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_EXTERNAL: u16 = 178;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_INTERNAL: u16 = 177;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_INVERT: u16 = 176;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_SWAP: u16 = 182;
pub const HID_USAGE_GENERIC_SYSCTL_DISP_TOGGLE: u16 = 181;
pub const HID_USAGE_GENERIC_SYSCTL_DOCK: u16 = 160;
pub const HID_USAGE_GENERIC_SYSCTL_FN: u16 = 151;
pub const HID_USAGE_GENERIC_SYSCTL_FN_LOCK: u16 = 152;
pub const HID_USAGE_GENERIC_SYSCTL_FN_LOCK_INDICATOR: u16 = 153;
pub const HID_USAGE_GENERIC_SYSCTL_HELP_MENU: u16 = 135;
pub const HID_USAGE_GENERIC_SYSCTL_HIBERNATE: u16 = 168;
pub const HID_USAGE_GENERIC_SYSCTL_MAIN_MENU: u16 = 133;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_DOWN: u16 = 141;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_EXIT: u16 = 136;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_LEFT: u16 = 139;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_RIGHT: u16 = 138;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_SELECT: u16 = 137;
pub const HID_USAGE_GENERIC_SYSCTL_MENU_UP: u16 = 140;
pub const HID_USAGE_GENERIC_SYSCTL_MUTE: u16 = 167;
pub const HID_USAGE_GENERIC_SYSCTL_POWER: u16 = 129;
pub const HID_USAGE_GENERIC_SYSCTL_SETUP: u16 = 162;
pub const HID_USAGE_GENERIC_SYSCTL_SLEEP: u16 = 130;
pub const HID_USAGE_GENERIC_SYSCTL_SYS_BREAK: u16 = 163;
pub const HID_USAGE_GENERIC_SYSCTL_SYS_DBG_BREAK: u16 = 164;
pub const HID_USAGE_GENERIC_SYSCTL_UNDOCK: u16 = 161;
pub const HID_USAGE_GENERIC_SYSCTL_WAKE: u16 = 131;
pub const HID_USAGE_GENERIC_SYSCTL_WARM_RESTART: u16 = 143;
pub const HID_USAGE_GENERIC_SYSTEM_CTL: u16 = 128;
pub const HID_USAGE_GENERIC_SYSTEM_DISPLAY_ROTATION_LOCK_BUTTON: u16 = 201;
pub const HID_USAGE_GENERIC_SYSTEM_DISPLAY_ROTATION_LOCK_SLIDER_SWITCH: u16 = 202;
pub const HID_USAGE_GENERIC_TABLET_PC_SYSTEM_CTL: u16 = 9;
pub const HID_USAGE_GENERIC_VBRX: u16 = 67;
pub const HID_USAGE_GENERIC_VBRY: u16 = 68;
pub const HID_USAGE_GENERIC_VBRZ: u16 = 69;
pub const HID_USAGE_GENERIC_VNO: u16 = 70;
pub const HID_USAGE_GENERIC_VX: u16 = 64;
pub const HID_USAGE_GENERIC_VY: u16 = 65;
pub const HID_USAGE_GENERIC_VZ: u16 = 66;
pub const HID_USAGE_GENERIC_WHEEL: u16 = 56;
pub const HID_USAGE_GENERIC_X: u16 = 48;
pub const HID_USAGE_GENERIC_Y: u16 = 49;
pub const HID_USAGE_GENERIC_Z: u16 = 50;
pub const HID_USAGE_HAPTICS_AUTO_ASSOCIATED_CONTROL: u16 = 34;
pub const HID_USAGE_HAPTICS_AUTO_TRIGGER: u16 = 32;
pub const HID_USAGE_HAPTICS_DURATION_LIST: u16 = 17;
pub const HID_USAGE_HAPTICS_INTENSITY: u16 = 35;
pub const HID_USAGE_HAPTICS_MANUAL_TRIGGER: u16 = 33;
pub const HID_USAGE_HAPTICS_REPEAT_COUNT: u16 = 36;
pub const HID_USAGE_HAPTICS_RETRIGGER_PERIOD: u16 = 37;
pub const HID_USAGE_HAPTICS_SIMPLE_CONTROLLER: u16 = 1;
pub const HID_USAGE_HAPTICS_WAVEFORM_BEGIN: u16 = 4096;
pub const HID_USAGE_HAPTICS_WAVEFORM_BUZZ: u16 = 4100;
pub const HID_USAGE_HAPTICS_WAVEFORM_CLICK: u16 = 4099;
pub const HID_USAGE_HAPTICS_WAVEFORM_CUTOFF_TIME: u16 = 40;
pub const HID_USAGE_HAPTICS_WAVEFORM_END: u16 = 8191;
pub const HID_USAGE_HAPTICS_WAVEFORM_LIST: u16 = 16;
pub const HID_USAGE_HAPTICS_WAVEFORM_NULL: u16 = 4098;
pub const HID_USAGE_HAPTICS_WAVEFORM_PRESS: u16 = 4102;
pub const HID_USAGE_HAPTICS_WAVEFORM_RELEASE: u16 = 4103;
pub const HID_USAGE_HAPTICS_WAVEFORM_RUMBLE: u16 = 4101;
pub const HID_USAGE_HAPTICS_WAVEFORM_STOP: u16 = 4097;
pub const HID_USAGE_HAPTICS_WAVEFORM_VENDOR_BEGIN: u16 = 8192;
pub const HID_USAGE_HAPTICS_WAVEFORM_VENDOR_END: u16 = 12287;
pub const HID_USAGE_HAPTICS_WAVEFORM_VENDOR_ID: u16 = 39;
pub const HID_USAGE_HAPTICS_WAVEFORM_VENDOR_PAGE: u16 = 38;
pub const HID_USAGE_KEYBOARD_CAPS_LOCK: u16 = 57;
pub const HID_USAGE_KEYBOARD_DELETE: u16 = 42;
pub const HID_USAGE_KEYBOARD_DELETE_FORWARD: u16 = 76;
pub const HID_USAGE_KEYBOARD_ESCAPE: u16 = 41;
pub const HID_USAGE_KEYBOARD_F1: u16 = 58;
pub const HID_USAGE_KEYBOARD_F10: u16 = 67;
pub const HID_USAGE_KEYBOARD_F11: u16 = 68;
pub const HID_USAGE_KEYBOARD_F12: u16 = 69;
pub const HID_USAGE_KEYBOARD_F13: u16 = 104;
pub const HID_USAGE_KEYBOARD_F14: u16 = 105;
pub const HID_USAGE_KEYBOARD_F15: u16 = 106;
pub const HID_USAGE_KEYBOARD_F16: u16 = 107;
pub const HID_USAGE_KEYBOARD_F17: u16 = 108;
pub const HID_USAGE_KEYBOARD_F18: u16 = 109;
pub const HID_USAGE_KEYBOARD_F19: u16 = 110;
pub const HID_USAGE_KEYBOARD_F2: u16 = 59;
pub const HID_USAGE_KEYBOARD_F20: u16 = 111;
pub const HID_USAGE_KEYBOARD_F21: u16 = 112;
pub const HID_USAGE_KEYBOARD_F22: u16 = 113;
pub const HID_USAGE_KEYBOARD_F23: u16 = 114;
pub const HID_USAGE_KEYBOARD_F24: u16 = 115;
pub const HID_USAGE_KEYBOARD_F3: u16 = 60;
pub const HID_USAGE_KEYBOARD_F4: u16 = 61;
pub const HID_USAGE_KEYBOARD_F5: u16 = 62;
pub const HID_USAGE_KEYBOARD_F6: u16 = 63;
pub const HID_USAGE_KEYBOARD_F7: u16 = 64;
pub const HID_USAGE_KEYBOARD_F8: u16 = 65;
pub const HID_USAGE_KEYBOARD_F9: u16 = 66;
pub const HID_USAGE_KEYBOARD_KEYPAD_0_AND_INSERT: u16 = 98;
pub const HID_USAGE_KEYBOARD_KEYPAD_1_AND_END: u16 = 89;
pub const HID_USAGE_KEYBOARD_LALT: u16 = 226;
pub const HID_USAGE_KEYBOARD_LCTRL: u16 = 224;
pub const HID_USAGE_KEYBOARD_LGUI: u16 = 227;
pub const HID_USAGE_KEYBOARD_LSHFT: u16 = 225;
pub const HID_USAGE_KEYBOARD_NOEVENT: u16 = 0;
pub const HID_USAGE_KEYBOARD_NUM_LOCK: u16 = 83;
pub const HID_USAGE_KEYBOARD_ONE: u16 = 30;
pub const HID_USAGE_KEYBOARD_POSTFAIL: u16 = 2;
pub const HID_USAGE_KEYBOARD_PRINT_SCREEN: u16 = 70;
pub const HID_USAGE_KEYBOARD_RALT: u16 = 230;
pub const HID_USAGE_KEYBOARD_RCTRL: u16 = 228;
pub const HID_USAGE_KEYBOARD_RETURN: u16 = 40;
pub const HID_USAGE_KEYBOARD_RGUI: u16 = 231;
pub const HID_USAGE_KEYBOARD_ROLLOVER: u16 = 1;
pub const HID_USAGE_KEYBOARD_RSHFT: u16 = 229;
pub const HID_USAGE_KEYBOARD_SCROLL_LOCK: u16 = 71;
pub const HID_USAGE_KEYBOARD_UNDEFINED: u16 = 3;
pub const HID_USAGE_KEYBOARD_ZERO: u16 = 39;
pub const HID_USAGE_KEYBOARD_aA: u16 = 4;
pub const HID_USAGE_KEYBOARD_zZ: u16 = 29;
pub const HID_USAGE_LAMPARRAY: u16 = 1;
pub const HID_USAGE_LAMPARRAY_ATTRBIUTES_REPORT: u16 = 2;
pub const HID_USAGE_LAMPARRAY_AUTONOMOUS_MODE: u16 = 113;
pub const HID_USAGE_LAMPARRAY_BLUE_LEVEL_COUNT: u16 = 42;
pub const HID_USAGE_LAMPARRAY_BOUNDING_BOX_DEPTH_IN_MICROMETERS: u16 = 6;
pub const HID_USAGE_LAMPARRAY_BOUNDING_BOX_HEIGHT_IN_MICROMETERS: u16 = 5;
pub const HID_USAGE_LAMPARRAY_BOUNDING_BOX_WIDTH_IN_MICROMETERS: u16 = 4;
pub const HID_USAGE_LAMPARRAY_CONTROL_REPORT: u16 = 112;
pub const HID_USAGE_LAMPARRAY_GREEN_LEVEL_COUNT: u16 = 41;
pub const HID_USAGE_LAMPARRAY_INPUT_BINDING: u16 = 45;
pub const HID_USAGE_LAMPARRAY_INTENSITY_LEVEL_COUNT: u16 = 43;
pub const HID_USAGE_LAMPARRAY_IS_PROGRAMMABLE: u16 = 44;
pub const HID_USAGE_LAMPARRAY_KIND: u16 = 7;
pub const HID_USAGE_LAMPARRAY_LAMP_ATTRIBUTES_REQUEST_REPORT: u16 = 32;
pub const HID_USAGE_LAMPARRAY_LAMP_ATTRIBUTES_RESPONSE_REPORT: u16 = 34;
pub const HID_USAGE_LAMPARRAY_LAMP_BLUE_UPDATE_CHANNEL: u16 = 83;
pub const HID_USAGE_LAMPARRAY_LAMP_COUNT: u16 = 3;
pub const HID_USAGE_LAMPARRAY_LAMP_GREEN_UPDATE_CHANNEL: u16 = 82;
pub const HID_USAGE_LAMPARRAY_LAMP_ID: u16 = 33;
pub const HID_USAGE_LAMPARRAY_LAMP_ID_END: u16 = 98;
pub const HID_USAGE_LAMPARRAY_LAMP_ID_START: u16 = 97;
pub const HID_USAGE_LAMPARRAY_LAMP_INTENSITY_UPDATE_CHANNEL: u16 = 84;
pub const HID_USAGE_LAMPARRAY_LAMP_MULTI_UPDATE_REPORT: u16 = 80;
pub const HID_USAGE_LAMPARRAY_LAMP_PURPOSES: u16 = 38;
pub const HID_USAGE_LAMPARRAY_LAMP_RANGE_UPDATE_REPORT: u16 = 96;
pub const HID_USAGE_LAMPARRAY_LAMP_RED_UPDATE_CHANNEL: u16 = 81;
pub const HID_USAGE_LAMPARRAY_LAMP_UPDATE_FLAGS: u16 = 85;
pub const HID_USAGE_LAMPARRAY_MIN_UPDATE_INTERVAL_IN_MICROSECONDS: u16 = 8;
pub const HID_USAGE_LAMPARRAY_POSITION_X_IN_MICROMETERS: u16 = 35;
pub const HID_USAGE_LAMPARRAY_POSITION_Y_IN_MICROMETERS: u16 = 36;
pub const HID_USAGE_LAMPARRAY_POSITION_Z_IN_MICROMETERS: u16 = 37;
pub const HID_USAGE_LAMPARRAY_RED_LEVEL_COUNT: u16 = 40;
pub const HID_USAGE_LAMPARRAY_UPDATE_LATENCY_IN_MICROSECONDS: u16 = 39;
pub const HID_USAGE_LED_AMBER: u16 = 74;
pub const HID_USAGE_LED_BATTERY_LOW: u16 = 29;
pub const HID_USAGE_LED_BATTERY_OK: u16 = 28;
pub const HID_USAGE_LED_BATTERY_OPERATION: u16 = 27;
pub const HID_USAGE_LED_BUSY: u16 = 44;
pub const HID_USAGE_LED_CALL_PICKUP: u16 = 37;
pub const HID_USAGE_LED_CAMERA_OFF: u16 = 41;
pub const HID_USAGE_LED_CAMERA_ON: u16 = 40;
pub const HID_USAGE_LED_CAPS_LOCK: u16 = 2;
pub const HID_USAGE_LED_CAV: u16 = 20;
pub const HID_USAGE_LED_CLV: u16 = 21;
pub const HID_USAGE_LED_COMPOSE: u16 = 4;
pub const HID_USAGE_LED_CONFERENCE: u16 = 38;
pub const HID_USAGE_LED_COVERAGE: u16 = 34;
pub const HID_USAGE_LED_DATA_MODE: u16 = 26;
pub const HID_USAGE_LED_DO_NOT_DISTURB: u16 = 8;
pub const HID_USAGE_LED_EQUALIZER_ENABLE: u16 = 13;
pub const HID_USAGE_LED_ERROR: u16 = 57;
pub const HID_USAGE_LED_EXTERNAL_POWER: u16 = 77;
pub const HID_USAGE_LED_FAST_BLINK_OFF_TIME: u16 = 70;
pub const HID_USAGE_LED_FAST_BLINK_ON_TIME: u16 = 69;
pub const HID_USAGE_LED_FAST_FORWARD: u16 = 53;
pub const HID_USAGE_LED_FLASH_ON_TIME: u16 = 66;
pub const HID_USAGE_LED_FORWARD: u16 = 49;
pub const HID_USAGE_LED_GENERIC_INDICATOR: u16 = 75;
pub const HID_USAGE_LED_GREEN: u16 = 73;
pub const HID_USAGE_LED_HEAD_SET: u16 = 31;
pub const HID_USAGE_LED_HIGH_CUT_FILTER: u16 = 11;
pub const HID_USAGE_LED_HOLD: u16 = 32;
pub const HID_USAGE_LED_INDICATOR_COLOR: u16 = 71;
pub const HID_USAGE_LED_INDICATOR_FAST_BLINK: u16 = 64;
pub const HID_USAGE_LED_INDICATOR_FLASH: u16 = 62;
pub const HID_USAGE_LED_INDICATOR_OFF: u16 = 65;
pub const HID_USAGE_LED_INDICATOR_ON: u16 = 61;
pub const HID_USAGE_LED_INDICATOR_SLOW_BLINK: u16 = 63;
pub const HID_USAGE_LED_IN_USE_INDICATOR: u16 = 59;
pub const HID_USAGE_LED_KANA: u16 = 5;
pub const HID_USAGE_LED_LOW_CUT_FILTER: u16 = 12;
pub const HID_USAGE_LED_MESSAGE_WAITING: u16 = 25;
pub const HID_USAGE_LED_MICROPHONE: u16 = 33;
pub const HID_USAGE_LED_MULTI_MODE_INDICATOR: u16 = 60;
pub const HID_USAGE_LED_MUTE: u16 = 9;
pub const HID_USAGE_LED_NIGHT_MODE: u16 = 35;
pub const HID_USAGE_LED_NUM_LOCK: u16 = 1;
pub const HID_USAGE_LED_OFF_HOOK: u16 = 23;
pub const HID_USAGE_LED_OFF_LINE: u16 = 43;
pub const HID_USAGE_LED_ON_LINE: u16 = 42;
pub const HID_USAGE_LED_PAPER_JAM: u16 = 47;
pub const HID_USAGE_LED_PAPER_OUT: u16 = 46;
pub const HID_USAGE_LED_PAUSE: u16 = 55;
pub const HID_USAGE_LED_PLAY: u16 = 54;
pub const HID_USAGE_LED_POWER: u16 = 6;
pub const HID_USAGE_LED_READY: u16 = 45;
pub const HID_USAGE_LED_RECORD: u16 = 56;
pub const HID_USAGE_LED_RECORDING_FORMAT_DET: u16 = 22;
pub const HID_USAGE_LED_RED: u16 = 72;
pub const HID_USAGE_LED_REMOTE: u16 = 48;
pub const HID_USAGE_LED_REPEAT: u16 = 16;
pub const HID_USAGE_LED_REVERSE: u16 = 50;
pub const HID_USAGE_LED_REWIND: u16 = 52;
pub const HID_USAGE_LED_RING: u16 = 24;
pub const HID_USAGE_LED_SAMPLING_RATE_DETECT: u16 = 18;
pub const HID_USAGE_LED_SCROLL_LOCK: u16 = 3;
pub const HID_USAGE_LED_SELECTED_INDICATOR: u16 = 58;
pub const HID_USAGE_LED_SEND_CALLS: u16 = 36;
pub const HID_USAGE_LED_SHIFT: u16 = 7;
pub const HID_USAGE_LED_SLOW_BLINK_OFF_TIME: u16 = 68;
pub const HID_USAGE_LED_SLOW_BLINK_ON_TIME: u16 = 67;
pub const HID_USAGE_LED_SOUND_FIELD_ON: u16 = 14;
pub const HID_USAGE_LED_SPEAKER: u16 = 30;
pub const HID_USAGE_LED_SPINNING: u16 = 19;
pub const HID_USAGE_LED_STAND_BY: u16 = 39;
pub const HID_USAGE_LED_STEREO: u16 = 17;
pub const HID_USAGE_LED_STOP: u16 = 51;
pub const HID_USAGE_LED_SURROUND_FIELD_ON: u16 = 15;
pub const HID_USAGE_LED_SYSTEM_SUSPEND: u16 = 76;
pub const HID_USAGE_LED_TONE_ENABLE: u16 = 10;
pub const HID_USAGE_MS_BTH_HF_DIALMEMORY: u16 = 34;
pub const HID_USAGE_MS_BTH_HF_DIALNUMBER: u16 = 33;
pub const HID_USAGE_PAGE_ALPHANUMERIC: u16 = 20;
pub const HID_USAGE_PAGE_ARCADE: u16 = 145;
pub const HID_USAGE_PAGE_BARCODE_SCANNER: u16 = 140;
pub const HID_USAGE_PAGE_BUTTON: u16 = 9;
pub const HID_USAGE_PAGE_CAMERA_CONTROL: u16 = 144;
pub const HID_USAGE_PAGE_CONSUMER: u16 = 12;
pub const HID_USAGE_PAGE_DIGITIZER: u16 = 13;
pub const HID_USAGE_PAGE_GAME: u16 = 5;
pub const HID_USAGE_PAGE_GENERIC: u16 = 1;
pub const HID_USAGE_PAGE_GENERIC_DEVICE: u16 = 6;
pub const HID_USAGE_PAGE_HAPTICS: u16 = 14;
pub const HID_USAGE_PAGE_KEYBOARD: u16 = 7;
pub const HID_USAGE_PAGE_LED: u16 = 8;
pub const HID_USAGE_PAGE_LIGHTING_ILLUMINATION: u16 = 89;
pub const HID_USAGE_PAGE_MAGNETIC_STRIPE_READER: u16 = 142;
pub const HID_USAGE_PAGE_MICROSOFT_BLUETOOTH_HANDSFREE: u16 = 65523;
pub const HID_USAGE_PAGE_ORDINAL: u16 = 10;
pub const HID_USAGE_PAGE_PID: u16 = 15;
pub const HID_USAGE_PAGE_SENSOR: u16 = 32;
pub const HID_USAGE_PAGE_SIMULATION: u16 = 2;
pub const HID_USAGE_PAGE_SPORT: u16 = 4;
pub const HID_USAGE_PAGE_TELEPHONY: u16 = 11;
pub const HID_USAGE_PAGE_UNDEFINED: u16 = 0;
pub const HID_USAGE_PAGE_UNICODE: u16 = 16;
pub const HID_USAGE_PAGE_VENDOR_DEFINED_BEGIN: u16 = 65280;
pub const HID_USAGE_PAGE_VENDOR_DEFINED_END: u16 = 65535;
pub const HID_USAGE_PAGE_VR: u16 = 3;
pub const HID_USAGE_PAGE_WEIGHING_DEVICE: u16 = 141;
pub const HID_USAGE_SIMULATION_ACCELLERATOR: u16 = 196;
pub const HID_USAGE_SIMULATION_AILERON: u16 = 176;
pub const HID_USAGE_SIMULATION_AILERON_TRIM: u16 = 177;
pub const HID_USAGE_SIMULATION_AIRPLANE_SIMULATION_DEVICE: u16 = 9;
pub const HID_USAGE_SIMULATION_ANTI_TORQUE_CONTROL: u16 = 178;
pub const HID_USAGE_SIMULATION_AUTOMOBILE_SIMULATION_DEVICE: u16 = 2;
pub const HID_USAGE_SIMULATION_AUTOPIOLOT_ENABLE: u16 = 179;
pub const HID_USAGE_SIMULATION_BALLAST: u16 = 204;
pub const HID_USAGE_SIMULATION_BARREL_ELEVATION: u16 = 202;
pub const HID_USAGE_SIMULATION_BICYCLE_CRANK: u16 = 205;
pub const HID_USAGE_SIMULATION_BICYCLE_SIMULATION_DEVICE: u16 = 12;
pub const HID_USAGE_SIMULATION_BRAKE: u16 = 197;
pub const HID_USAGE_SIMULATION_CHAFF_RELEASE: u16 = 180;
pub const HID_USAGE_SIMULATION_CLUTCH: u16 = 198;
pub const HID_USAGE_SIMULATION_COLLECTIVE_CONTROL: u16 = 181;
pub const HID_USAGE_SIMULATION_CYCLIC_CONTROL: u16 = 34;
pub const HID_USAGE_SIMULATION_CYCLIC_TRIM: u16 = 35;
pub const HID_USAGE_SIMULATION_DIVE_BRAKE: u16 = 182;
pub const HID_USAGE_SIMULATION_DIVE_PLANE: u16 = 203;
pub const HID_USAGE_SIMULATION_ELECTRONIC_COUNTERMEASURES: u16 = 183;
pub const HID_USAGE_SIMULATION_ELEVATOR: u16 = 184;
pub const HID_USAGE_SIMULATION_ELEVATOR_TRIM: u16 = 185;
pub const HID_USAGE_SIMULATION_FLARE_RELEASE: u16 = 189;
pub const HID_USAGE_SIMULATION_FLIGHT_COMMUNICATIONS: u16 = 188;
pub const HID_USAGE_SIMULATION_FLIGHT_CONTROL_STICK: u16 = 32;
pub const HID_USAGE_SIMULATION_FLIGHT_SIMULATION_DEVICE: u16 = 1;
pub const HID_USAGE_SIMULATION_FLIGHT_STICK: u16 = 33;
pub const HID_USAGE_SIMULATION_FLIGHT_YOKE: u16 = 36;
pub const HID_USAGE_SIMULATION_FRONT_BRAKE: u16 = 207;
pub const HID_USAGE_SIMULATION_HANDLE_BARS: u16 = 206;
pub const HID_USAGE_SIMULATION_HELICOPTER_SIMULATION_DEVICE: u16 = 10;
pub const HID_USAGE_SIMULATION_LANDING_GEAR: u16 = 190;
pub const HID_USAGE_SIMULATION_MAGIC_CARPET_SIMULATION_DEVICE: u16 = 11;
pub const HID_USAGE_SIMULATION_MOTORCYCLE_SIMULATION_DEVICE: u16 = 7;
pub const HID_USAGE_SIMULATION_REAR_BRAKE: u16 = 208;
pub const HID_USAGE_SIMULATION_RUDDER: u16 = 186;
pub const HID_USAGE_SIMULATION_SAILING_SIMULATION_DEVICE: u16 = 6;
pub const HID_USAGE_SIMULATION_SHIFTER: u16 = 199;
pub const HID_USAGE_SIMULATION_SPACESHIP_SIMULATION_DEVICE: u16 = 4;
pub const HID_USAGE_SIMULATION_SPORTS_SIMULATION_DEVICE: u16 = 8;
pub const HID_USAGE_SIMULATION_STEERING: u16 = 200;
pub const HID_USAGE_SIMULATION_SUBMARINE_SIMULATION_DEVICE: u16 = 5;
pub const HID_USAGE_SIMULATION_TANK_SIMULATION_DEVICE: u16 = 3;
pub const HID_USAGE_SIMULATION_THROTTLE: u16 = 187;
pub const HID_USAGE_SIMULATION_TOE_BRAKE: u16 = 191;
pub const HID_USAGE_SIMULATION_TRACK_CONTROL: u16 = 37;
pub const HID_USAGE_SIMULATION_TRIGGER: u16 = 192;
pub const HID_USAGE_SIMULATION_TURRET_DIRECTION: u16 = 201;
pub const HID_USAGE_SIMULATION_WEAPONS_ARM: u16 = 193;
pub const HID_USAGE_SIMULATION_WEAPONS_SELECT: u16 = 194;
pub const HID_USAGE_SIMULATION_WING_FLAPS: u16 = 195;
pub const HID_USAGE_SPORT_10_IRON: u16 = 90;
pub const HID_USAGE_SPORT_11_IRON: u16 = 91;
pub const HID_USAGE_SPORT_1_IRON: u16 = 81;
pub const HID_USAGE_SPORT_1_WOOD: u16 = 95;
pub const HID_USAGE_SPORT_2_IRON: u16 = 82;
pub const HID_USAGE_SPORT_3_IRON: u16 = 83;
pub const HID_USAGE_SPORT_3_WOOD: u16 = 96;
pub const HID_USAGE_SPORT_4_IRON: u16 = 84;
pub const HID_USAGE_SPORT_5_IRON: u16 = 85;
pub const HID_USAGE_SPORT_5_WOOD: u16 = 97;
pub const HID_USAGE_SPORT_6_IRON: u16 = 86;
pub const HID_USAGE_SPORT_7_IRON: u16 = 87;
pub const HID_USAGE_SPORT_7_WOOD: u16 = 98;
pub const HID_USAGE_SPORT_8_IRON: u16 = 88;
pub const HID_USAGE_SPORT_9_IRON: u16 = 89;
pub const HID_USAGE_SPORT_9_WOOD: u16 = 99;
pub const HID_USAGE_SPORT_BASEBALL_BAT: u16 = 1;
pub const HID_USAGE_SPORT_FOLLOW_THROUGH: u16 = 54;
pub const HID_USAGE_SPORT_GOLF_CLUB: u16 = 2;
pub const HID_USAGE_SPORT_HEEL_TOE: u16 = 53;
pub const HID_USAGE_SPORT_HEIGHT: u16 = 57;
pub const HID_USAGE_SPORT_LOFT_WEDGE: u16 = 93;
pub const HID_USAGE_SPORT_OAR: u16 = 48;
pub const HID_USAGE_SPORT_POWER_WEDGE: u16 = 94;
pub const HID_USAGE_SPORT_PUTTER: u16 = 80;
pub const HID_USAGE_SPORT_RATE: u16 = 50;
pub const HID_USAGE_SPORT_ROWING_MACHINE: u16 = 3;
pub const HID_USAGE_SPORT_SAND_WEDGE: u16 = 92;
pub const HID_USAGE_SPORT_SLOPE: u16 = 49;
pub const HID_USAGE_SPORT_STICK_FACE_ANGLE: u16 = 52;
pub const HID_USAGE_SPORT_STICK_SPEED: u16 = 51;
pub const HID_USAGE_SPORT_STICK_TYPE: u16 = 56;
pub const HID_USAGE_SPORT_TEMPO: u16 = 55;
pub const HID_USAGE_SPORT_TREADMILL: u16 = 4;
pub const HID_USAGE_TELEPHONY_ANSWERING_MACHINE: u16 = 2;
pub const HID_USAGE_TELEPHONY_DROP: u16 = 38;
pub const HID_USAGE_TELEPHONY_HANDSET: u16 = 4;
pub const HID_USAGE_TELEPHONY_HEADSET: u16 = 5;
pub const HID_USAGE_TELEPHONY_HOST_AVAILABLE: u16 = 241;
pub const HID_USAGE_TELEPHONY_KEYPAD: u16 = 6;
pub const HID_USAGE_TELEPHONY_KEYPAD_0: u16 = 176;
pub const HID_USAGE_TELEPHONY_KEYPAD_D: u16 = 191;
pub const HID_USAGE_TELEPHONY_LINE: u16 = 42;
pub const HID_USAGE_TELEPHONY_MESSAGE_CONTROLS: u16 = 3;
pub const HID_USAGE_TELEPHONY_PHONE: u16 = 1;
pub const HID_USAGE_TELEPHONY_PROGRAMMABLE_BUTTON: u16 = 7;
pub const HID_USAGE_TELEPHONY_REDIAL: u16 = 36;
pub const HID_USAGE_TELEPHONY_RING_ENABLE: u16 = 45;
pub const HID_USAGE_TELEPHONY_SEND: u16 = 49;
pub const HID_USAGE_TELEPHONY_TRANSFER: u16 = 37;
pub const HID_USAGE_VR_ANIMATRONIC_DEVICE: u16 = 10;
pub const HID_USAGE_VR_BELT: u16 = 1;
pub const HID_USAGE_VR_BODY_SUIT: u16 = 2;
pub const HID_USAGE_VR_DISPLAY_ENABLE: u16 = 33;
pub const HID_USAGE_VR_FLEXOR: u16 = 3;
pub const HID_USAGE_VR_GLOVE: u16 = 4;
pub const HID_USAGE_VR_HAND_TRACKER: u16 = 7;
pub const HID_USAGE_VR_HEAD_MOUNTED_DISPLAY: u16 = 6;
pub const HID_USAGE_VR_HEAD_TRACKER: u16 = 5;
pub const HID_USAGE_VR_OCULOMETER: u16 = 8;
pub const HID_USAGE_VR_STEREO_ENABLE: u16 = 32;
pub const HID_USAGE_VR_VEST: u16 = 9;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const HORIZONTAL_WHEEL_PRESENT: u32 = 32768;
pub const HidP_Feature: HIDP_REPORT_TYPE = HIDP_REPORT_TYPE(2);
pub const HidP_Input: HIDP_REPORT_TYPE = HIDP_REPORT_TYPE(0);
pub const HidP_Keyboard_Break: HIDP_KEYBOARD_DIRECTION = HIDP_KEYBOARD_DIRECTION(0);
pub const HidP_Keyboard_Make: HIDP_KEYBOARD_DIRECTION = HIDP_KEYBOARD_DIRECTION(1);
pub const HidP_Output: HIDP_REPORT_TYPE = HIDP_REPORT_TYPE(1);
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
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2A_Impl: IDirectInputA_Impl {
    fn FindDevice(&self, param0: *const windows_core::GUID, param1: &windows_core::PCSTR, param2: *mut windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput2A_Vtbl {
    pub const fn new<Identity: IDirectInput2A_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindDevice<Identity: IDirectInput2A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: windows_core::PCSTR, param2: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput2A_Impl::FindDevice(this, core::mem::transmute_copy(&param0), &*(&raw const param1).cast(), core::mem::transmute_copy(&param2)).into()
            }
        }
        Self { base__: IDirectInputA_Vtbl::new::<Identity, OFFSET>(), FindDevice: FindDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput2A as windows_core::Interface>::IID || IDirectInputA_Vtbl::matches(iid)
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput2W_Impl: IDirectInputW_Impl {
    fn FindDevice(&self, param0: *const windows_core::GUID, param1: &windows_core::PCWSTR, param2: *mut windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInput2W_Vtbl {
    pub const fn new<Identity: IDirectInput2W_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindDevice<Identity: IDirectInput2W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *const windows_core::GUID, param1: windows_core::PCWSTR, param2: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput2W_Impl::FindDevice(this, core::mem::transmute_copy(&param0), &*(&raw const param1).cast(), core::mem::transmute_copy(&param2)).into()
            }
        }
        Self { base__: IDirectInputW_Vtbl::new::<Identity, OFFSET>(), FindDevice: FindDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInput2W as windows_core::Interface>::IID || IDirectInputW_Vtbl::matches(iid)
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7A_Impl: IDirectInput2A_Impl {
    fn CreateDeviceEx(&self, param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void, param3: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
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
        iid == &<IDirectInput7A as windows_core::Interface>::IID || IDirectInput2A_Vtbl::matches(iid)
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInput7W_Impl: IDirectInput2W_Impl {
    fn CreateDeviceEx(&self, param0: *const windows_core::GUID, param1: *const windows_core::GUID, param2: *mut *mut core::ffi::c_void, param3: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
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
        iid == &<IDirectInput7W as windows_core::Interface>::IID || IDirectInput2W_Vtbl::matches(iid)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl windows_core::RuntimeName for IDirectInput7W {}
windows_core::imp::define_interface!(IDirectInput8A, IDirectInput8A_Vtbl, 0xbf798030_483a_4da2_aa99_5d64ed369700);
windows_core::imp::interface_hierarchy!(IDirectInput8A, windows_core::IUnknown);
impl IDirectInput8A {
    pub unsafe fn CreateDevice<P2>(&self, param0: *const windows_core::GUID, param1: *mut Option<IDirectInputDevice8A>, param2: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), param0, param1 as _, param2.param().abi()).ok() }
    }
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumDevices)(windows_core::Interface::as_raw(self), param0, param1, param2 as _, param3).ok() }
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceStatus)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn FindDevice<P1>(&self, param0: *const windows_core::GUID, param1: P1, param2: *mut windows_core::GUID) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindDevice)(windows_core::Interface::as_raw(self), param0, param1.param().abi(), param2 as _).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevicesBySemantics<P0>(&self, param0: P0, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumDevicesBySemantics)(windows_core::Interface::as_raw(self), param0.param().abi(), param1 as _, param2, param3 as _, param4).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConfigureDevices(&self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSA, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConfigureDevices)(windows_core::Interface::as_raw(self), param0, param1, param2, param3 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput8A_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, LPDIENUMDEVICESCALLBACKA, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub FindDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCSTR, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumDevicesBySemantics: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut DIACTIONFORMATA, LPDIENUMDEVICESBYSEMANTICSCBA, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumDevicesBySemantics: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConfigureDevices: unsafe extern "system" fn(*mut core::ffi::c_void, LPDICONFIGUREDEVICESCALLBACK, *mut DICONFIGUREDEVICESPARAMSA, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConfigureDevices: usize,
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
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
                IDirectInput8A_Impl::FindDevice(this, core::mem::transmute_copy(&param0), &*(&raw const param1).cast(), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Identity: IDirectInput8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: *mut DIACTIONFORMATA, param2: LPDIENUMDEVICESBYSEMANTICSCBA, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8A_Impl::EnumDevicesBySemantics(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
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
#[cfg(feature = "Win32_Foundation")]
impl windows_core::RuntimeName for IDirectInput8A {}
windows_core::imp::define_interface!(IDirectInput8W, IDirectInput8W_Vtbl, 0xbf798031_483a_4da2_aa99_5d64ed369700);
windows_core::imp::interface_hierarchy!(IDirectInput8W, windows_core::IUnknown);
impl IDirectInput8W {
    pub unsafe fn CreateDevice<P2>(&self, param0: *const windows_core::GUID, param1: *mut Option<IDirectInputDevice8W>, param2: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), param0, param1 as _, param2.param().abi()).ok() }
    }
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumDevices)(windows_core::Interface::as_raw(self), param0, param1, param2 as _, param3).ok() }
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceStatus)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn FindDevice<P1>(&self, param0: *const windows_core::GUID, param1: P1, param2: *mut windows_core::GUID) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindDevice)(windows_core::Interface::as_raw(self), param0, param1.param().abi(), param2 as _).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumDevicesBySemantics<P0>(&self, param0: P0, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).EnumDevicesBySemantics)(windows_core::Interface::as_raw(self), param0.param().abi(), param1 as _, param2, param3 as _, param4).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ConfigureDevices(&self, param0: LPDICONFIGUREDEVICESCALLBACK, param1: *mut DICONFIGUREDEVICESPARAMSW, param2: u32, param3: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ConfigureDevices)(windows_core::Interface::as_raw(self), param0, param1, param2, param3 as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectInput8W_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumDevices: unsafe extern "system" fn(*mut core::ffi::c_void, u32, LPDIENUMDEVICESCALLBACKW, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDeviceStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub FindDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, *mut windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnumDevicesBySemantics: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut DIACTIONFORMATW, LPDIENUMDEVICESBYSEMANTICSCBW, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnumDevicesBySemantics: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ConfigureDevices: unsafe extern "system" fn(*mut core::ffi::c_void, LPDICONFIGUREDEVICESCALLBACK, *mut DICONFIGUREDEVICESPARAMSW, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ConfigureDevices: usize,
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
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
                IDirectInput8W_Impl::FindDevice(this, core::mem::transmute_copy(&param0), &*(&raw const param1).cast(), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn EnumDevicesBySemantics<Identity: IDirectInput8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: *mut DIACTIONFORMATW, param2: LPDIENUMDEVICESBYSEMANTICSCBW, param3: *mut core::ffi::c_void, param4: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInput8W_Impl::EnumDevicesBySemantics(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3), core::mem::transmute_copy(&param4)).into()
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
#[cfg(feature = "Win32_Foundation")]
impl windows_core::RuntimeName for IDirectInput8W {}
windows_core::imp::define_interface!(IDirectInputA, IDirectInputA_Vtbl, 0x89521360_aa8a_11cf_bfc7_444553540000);
windows_core::imp::interface_hierarchy!(IDirectInputA, windows_core::IUnknown);
impl IDirectInputA {
    pub unsafe fn CreateDevice<P2>(&self, param0: *const windows_core::GUID, param1: *mut Option<IDirectInputDeviceA>, param2: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), param0, param1 as _, param2.param().abi()).ok() }
    }
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumDevices)(windows_core::Interface::as_raw(self), param0, param1, param2 as _, param3).ok() }
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceStatus)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputA_Impl: windows_core::IUnknownImpl {
    fn CreateDevice(&self, param0: *const windows_core::GUID, param1: windows_core::OutRef<IDirectInputDeviceA>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKA, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
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
        unsafe { (windows_core::Interface::vtable(self).CreateEffect)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3.param().abi()).ok() }
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
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
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
        iid == &<IDirectInputDevice2A as windows_core::Interface>::IID || IDirectInputDeviceA_Vtbl::matches(iid)
    }
}
#[cfg(feature = "Win32_Foundation")]
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
        unsafe { (windows_core::Interface::vtable(self).CreateEffect)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3.param().abi()).ok() }
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
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
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
        iid == &<IDirectInputDevice2W as windows_core::Interface>::IID || IDirectInputDeviceW_Vtbl::matches(iid)
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice7A_Impl: IDirectInputDevice2A_Impl {
    fn EnumEffectsInFile(&self, param0: &windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn WriteEffectToFile(&self, param0: &windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice7A_Vtbl {
    pub const fn new<Identity: IDirectInputDevice7A_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumEffectsInFile<Identity: IDirectInputDevice7A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice7A_Impl::EnumEffectsInFile(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: IDirectInputDevice7A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice7A_Impl::WriteEffectToFile(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        Self {
            base__: IDirectInputDevice2A_Vtbl::new::<Identity, OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Identity, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice7A as windows_core::Interface>::IID || IDirectInputDevice2A_Vtbl::matches(iid)
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputDevice7W_Impl: IDirectInputDevice2W_Impl {
    fn EnumEffectsInFile(&self, param0: &windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn WriteEffectToFile(&self, param0: &windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectInputDevice7W_Vtbl {
    pub const fn new<Identity: IDirectInputDevice7W_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumEffectsInFile<Identity: IDirectInputDevice7W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: LPDIENUMEFFECTSINFILECALLBACK, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice7W_Impl::EnumEffectsInFile(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: IDirectInputDevice7W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice7W_Impl::WriteEffectToFile(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        Self {
            base__: IDirectInputDevice2W_Vtbl::new::<Identity, OFFSET>(),
            EnumEffectsInFile: EnumEffectsInFile::<Identity, OFFSET>,
            WriteEffectToFile: WriteEffectToFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectInputDevice7W as windows_core::Interface>::IID || IDirectInputDevice2W_Vtbl::matches(iid)
    }
}
#[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventNotification)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectInfo)(windows_core::Interface::as_raw(self), param0 as _, param1, param2).ok() }
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceInfo)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0, param1, param2).ok() }
    }
    pub unsafe fn CreateEffect<P3>(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut Option<IDirectInputEffect>, param3: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateEffect)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3.param().abi()).ok() }
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
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BuildActionMap<P1>(&self, param0: *mut DIACTIONFORMATA, param1: P1, param2: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BuildActionMap)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetActionMap<P1>(&self, param0: *mut DIACTIONFORMATA, param1: P1, param2: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActionMap)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventNotification: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub GetObjectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEOBJECTINSTANCEA, u32, u32) -> windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEINSTANCEA) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
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
    #[cfg(feature = "Win32_Foundation")]
    pub BuildActionMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIACTIONFORMATA, windows_core::PCSTR, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BuildActionMap: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetActionMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIACTIONFORMATA, windows_core::PCSTR, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetActionMap: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetImageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEIMAGEINFOHEADERA) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetImageInfo: usize,
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
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
                IDirectInputDevice8A_Impl::EnumEffectsInFile(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::WriteEffectToFile(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn BuildActionMap<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: windows_core::PCSTR, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::BuildActionMap(this, core::mem::transmute_copy(&param0), &*(&raw const param1).cast(), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn SetActionMap<Identity: IDirectInputDevice8A_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIACTIONFORMATA, param1: windows_core::PCSTR, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8A_Impl::SetActionMap(this, core::mem::transmute_copy(&param0), &*(&raw const param1).cast(), core::mem::transmute_copy(&param2)).into()
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
#[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventNotification)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectInfo)(windows_core::Interface::as_raw(self), param0 as _, param1, param2).ok() }
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceInfo)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32, param2: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), param0, param1, param2).ok() }
    }
    pub unsafe fn CreateEffect<P3>(&self, param0: *const windows_core::GUID, param1: *mut DIEFFECT, param2: *mut Option<IDirectInputEffect>, param3: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateEffect)(windows_core::Interface::as_raw(self), param0, param1 as _, param2 as _, param3.param().abi()).ok() }
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
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BuildActionMap<P1>(&self, param0: *mut DIACTIONFORMATW, param1: P1, param2: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).BuildActionMap)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetActionMap<P1>(&self, param0: *mut DIACTIONFORMATW, param1: P1, param2: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetActionMap)(windows_core::Interface::as_raw(self), param0 as _, param1.param().abi(), param2).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventNotification: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub GetObjectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEOBJECTINSTANCEW, u32, u32) -> windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEINSTANCEW) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
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
    #[cfg(feature = "Win32_Foundation")]
    pub BuildActionMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIACTIONFORMATW, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BuildActionMap: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetActionMap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIACTIONFORMATW, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetActionMap: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetImageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEIMAGEINFOHEADERW) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetImageInfo: usize,
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
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
                IDirectInputDevice8W_Impl::EnumEffectsInFile(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn WriteEffectToFile<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: u32, param2: *mut DIFILEEFFECT, param3: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::WriteEffectToFile(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), core::mem::transmute_copy(&param3)).into()
            }
        }
        unsafe extern "system" fn BuildActionMap<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: windows_core::PCWSTR, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::BuildActionMap(this, core::mem::transmute_copy(&param0), &*(&raw const param1).cast(), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn SetActionMap<Identity: IDirectInputDevice8W_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut DIACTIONFORMATW, param1: windows_core::PCWSTR, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputDevice8W_Impl::SetActionMap(this, core::mem::transmute_copy(&param0), &*(&raw const param1).cast(), core::mem::transmute_copy(&param2)).into()
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
#[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventNotification)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEA, param1: u32, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectInfo)(windows_core::Interface::as_raw(self), param0 as _, param1, param2).ok() }
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEA) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceInfo)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventNotification: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub GetObjectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEOBJECTINSTANCEA, u32, u32) -> windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEINSTANCEA) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventNotification(&self, param0: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetEventNotification)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCooperativeLevel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    pub unsafe fn GetObjectInfo(&self, param0: *mut DIDEVICEOBJECTINSTANCEW, param1: u32, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetObjectInfo)(windows_core::Interface::as_raw(self), param0 as _, param1, param2).ok() }
    }
    pub unsafe fn GetDeviceInfo(&self, param0: *mut DIDEVICEINSTANCEW) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceInfo)(windows_core::Interface::as_raw(self), param0 as _).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventNotification: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventNotification: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub GetObjectInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEOBJECTINSTANCEW, u32, u32) -> windows_core::HRESULT,
    pub GetDeviceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIDEVICEINSTANCEW) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl windows_core::RuntimeName for IDirectInputDeviceW {}
windows_core::imp::define_interface!(IDirectInputEffect, IDirectInputEffect_Vtbl, 0xe7e1f7c0_88d2_11d0_9ad0_00a0c9a06e35);
windows_core::imp::interface_hierarchy!(IDirectInputEffect, windows_core::IUnknown);
impl IDirectInputEffect {
    #[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
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
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
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
    #[cfg(feature = "Win32_Foundation")]
    pub AddNewHardware: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddNewHardware: usize,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenTypeKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenTypeKey: usize,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenConfigKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenConfigKey: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
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
                IDirectInputJoyConfig_Impl::GetTypeInfo(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn SetTypeInfo<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::SetTypeInfo(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn DeleteType<Identity: IDirectInputJoyConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig_Impl::DeleteType(this, &*(&raw const param0).cast()).into()
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
                IDirectInputJoyConfig_Impl::OpenTypeKey(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
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
    #[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
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
    #[cfg(feature = "Win32_Foundation")]
    pub AddNewHardware: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddNewHardware: usize,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenTypeKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenTypeKey: usize,
    #[cfg(feature = "Win32_System_Registry")]
    pub OpenAppStatusKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::System::Registry::HKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Registry"))]
    OpenAppStatusKey: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
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
                IDirectInputJoyConfig8_Impl::GetTypeInfo(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        unsafe extern "system" fn SetTypeInfo<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR, param1: *mut DIJOYTYPEINFO, param2: u32, param3: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::SetTypeInfo(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2), &*(&raw const param3).cast()).into()
            }
        }
        unsafe extern "system" fn DeleteType<Identity: IDirectInputJoyConfig8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectInputJoyConfig8_Impl::DeleteType(this, &*(&raw const param0).cast()).into()
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
                IDirectInputJoyConfig8_Impl::OpenTypeKey(this, &*(&raw const param0).cast(), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl windows_core::RuntimeName for IDirectInputJoyConfig8 {}
windows_core::imp::define_interface!(IDirectInputW, IDirectInputW_Vtbl, 0x89521361_aa8a_11cf_bfc7_444553540000);
windows_core::imp::interface_hierarchy!(IDirectInputW, windows_core::IUnknown);
impl IDirectInputW {
    pub unsafe fn CreateDevice<P2>(&self, param0: *const windows_core::GUID, param1: *mut Option<IDirectInputDeviceW>, param2: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), param0, param1 as _, param2.param().abi()).ok() }
    }
    pub unsafe fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnumDevices)(windows_core::Interface::as_raw(self), param0, param1, param2 as _, param3).ok() }
    }
    pub unsafe fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceStatus)(windows_core::Interface::as_raw(self), param0).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RunControlPanel)(windows_core::Interface::as_raw(self), param0, param1).ok() }
    }
    #[cfg(feature = "Win32_Foundation")]
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
    #[cfg(feature = "Win32_Foundation")]
    pub RunControlPanel: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RunControlPanel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HINSTANCE, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectInputW_Impl: windows_core::IUnknownImpl {
    fn CreateDevice(&self, param0: *const windows_core::GUID, param1: windows_core::OutRef<IDirectInputDeviceW>, param2: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EnumDevices(&self, param0: u32, param1: LPDIENUMDEVICESCALLBACKW, param2: *mut core::ffi::c_void, param3: u32) -> windows_core::Result<()>;
    fn GetDeviceStatus(&self, param0: *const windows_core::GUID) -> windows_core::Result<()>;
    fn RunControlPanel(&self, param0: super::super::Foundation::HWND, param1: u32) -> windows_core::Result<()>;
    fn Initialize(&self, param0: super::super::Foundation::HINSTANCE, param1: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl windows_core::RuntimeName for IDirectInputW {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INDICATOR_LIST {
    pub MakeCode: u16,
    pub IndicatorFlags: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct INPUT_BUTTON_ENABLE_INFO {
    pub ButtonType: GPIOBUTTONS_BUTTON_TYPE,
    pub Enabled: bool,
}
pub const IOCTL_BUTTON_GET_ENABLED_ON_IDLE: u32 = 721580;
pub const IOCTL_BUTTON_SET_ENABLED_ON_IDLE: u32 = 721576;
pub const IOCTL_KEYBOARD_INSERT_DATA: u32 = 721152;
pub const IOCTL_KEYBOARD_QUERY_ATTRIBUTES: u32 = 720896;
pub const IOCTL_KEYBOARD_QUERY_EXTENDED_ATTRIBUTES: u32 = 721408;
pub const IOCTL_KEYBOARD_QUERY_IME_STATUS: u32 = 724992;
pub const IOCTL_KEYBOARD_QUERY_INDICATORS: u32 = 720960;
pub const IOCTL_KEYBOARD_QUERY_INDICATOR_TRANSLATION: u32 = 721024;
pub const IOCTL_KEYBOARD_QUERY_TYPEMATIC: u32 = 720928;
pub const IOCTL_KEYBOARD_SET_IME_STATUS: u32 = 724996;
pub const IOCTL_KEYBOARD_SET_INDICATORS: u32 = 720904;
pub const IOCTL_KEYBOARD_SET_TYPEMATIC: u32 = 720900;
pub const IOCTL_MOUSE_INSERT_DATA: u32 = 983044;
pub const IOCTL_MOUSE_QUERY_ATTRIBUTES: u32 = 983040;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JOYCALIBRATE {
    pub wXbase: u32,
    pub wXdelta: u32,
    pub wYbase: u32,
    pub wYdelta: u32,
    pub wZbase: u32,
    pub wZdelta: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JOYPOS {
    pub dwX: u32,
    pub dwY: u32,
    pub dwZ: u32,
    pub dwR: u32,
    pub dwU: u32,
    pub dwV: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JOYRANGE {
    pub jpMin: JOYPOS,
    pub jpMax: JOYPOS,
    pub jpCenter: JOYPOS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JOYREGHWCONFIG {
    pub hws: JOYREGHWSETTINGS,
    pub dwUsageSettings: u32,
    pub hwv: JOYREGHWVALUES,
    pub dwType: u32,
    pub dwReserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JOYREGHWSETTINGS {
    pub dwFlags: u32,
    pub dwNumButtons: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JOYREGUSERVALUES {
    pub dwTimeOut: u32,
    pub jrvRanges: JOYRANGE,
    pub jpDeadZone: JOYPOS,
}
pub const JOYTYPE_ANALOGCOMPAT: i32 = 8;
pub const JOYTYPE_DEFAULTPROPSHEET: i32 = -2147483648;
pub const JOYTYPE_DEVICEHIDE: i32 = 65536;
pub const JOYTYPE_ENABLEINPUTREPORT: i32 = 16777216;
pub const JOYTYPE_GAMEHIDE: i32 = 524288;
pub const JOYTYPE_HIDEACTIVE: i32 = 1048576;
pub const JOYTYPE_INFODEFAULT: i32 = 0;
pub const JOYTYPE_INFOMASK: i32 = 14680064;
pub const JOYTYPE_INFOYRPEDALS: i32 = 6291456;
pub const JOYTYPE_INFOYYPEDALS: i32 = 2097152;
pub const JOYTYPE_INFOZISSLIDER: i32 = 2097152;
pub const JOYTYPE_INFOZISZ: i32 = 4194304;
pub const JOYTYPE_INFOZRPEDALS: i32 = 8388608;
pub const JOYTYPE_INFOZYPEDALS: i32 = 4194304;
pub const JOYTYPE_KEYBHIDE: i32 = 262144;
pub const JOYTYPE_MOUSEHIDE: i32 = 131072;
pub const JOYTYPE_NOAUTODETECTGAMEPORT: i32 = 2;
pub const JOYTYPE_NOHIDDIRECT: i32 = 4;
pub const JOYTYPE_ZEROGAMEENUMOEMDATA: i32 = 1;
pub const JOY_HWS_AUTOLOAD: i32 = 268435456;
pub const JOY_HWS_GAMEPORTBUSBUSY: i32 = 1;
pub const JOY_HWS_HASPOV: i32 = 2;
pub const JOY_HWS_HASR: i32 = 524288;
pub const JOY_HWS_HASU: i32 = 8388608;
pub const JOY_HWS_HASV: i32 = 16777216;
pub const JOY_HWS_HASZ: i32 = 1;
pub const JOY_HWS_ISANALOGPORTDRIVER: i32 = 134217728;
pub const JOY_HWS_ISCARCTRL: i32 = 64;
pub const JOY_HWS_ISGAMEPAD: i32 = 32;
pub const JOY_HWS_ISGAMEPORTBUS: i32 = -2147483648;
pub const JOY_HWS_ISGAMEPORTDRIVER: i32 = 67108864;
pub const JOY_HWS_ISHEADTRACKER: i32 = 33554432;
pub const JOY_HWS_ISYOKE: i32 = 16;
pub const JOY_HWS_NODEVNODE: i32 = 536870912;
pub const JOY_HWS_POVISBUTTONCOMBOS: i32 = 4;
pub const JOY_HWS_POVISJ1X: i32 = 65536;
pub const JOY_HWS_POVISJ1Y: i32 = 131072;
pub const JOY_HWS_POVISJ2X: i32 = 262144;
pub const JOY_HWS_POVISPOLL: i32 = 8;
pub const JOY_HWS_RISJ1X: i32 = 1048576;
pub const JOY_HWS_RISJ1Y: i32 = 2097152;
pub const JOY_HWS_RISJ2Y: i32 = 4194304;
pub const JOY_HWS_XISJ1Y: i32 = 128;
pub const JOY_HWS_XISJ2X: i32 = 256;
pub const JOY_HWS_XISJ2Y: i32 = 512;
pub const JOY_HWS_YISJ1X: i32 = 1024;
pub const JOY_HWS_YISJ2X: i32 = 2048;
pub const JOY_HWS_YISJ2Y: i32 = 4096;
pub const JOY_HWS_ZISJ1X: i32 = 8192;
pub const JOY_HWS_ZISJ1Y: i32 = 16384;
pub const JOY_HWS_ZISJ2X: i32 = 32768;
pub const JOY_HW_2A_2B_GENERIC: u32 = 2;
pub const JOY_HW_2A_4B_GENERIC: u32 = 3;
pub const JOY_HW_2B_FLIGHTYOKE: u32 = 5;
pub const JOY_HW_2B_FLIGHTYOKETHROTTLE: u32 = 6;
pub const JOY_HW_2B_GAMEPAD: u32 = 4;
pub const JOY_HW_3A_2B_GENERIC: u32 = 7;
pub const JOY_HW_3A_4B_GENERIC: u32 = 8;
pub const JOY_HW_4B_FLIGHTYOKE: u32 = 10;
pub const JOY_HW_4B_FLIGHTYOKETHROTTLE: u32 = 11;
pub const JOY_HW_4B_GAMEPAD: u32 = 9;
pub const JOY_HW_CUSTOM: u32 = 1;
pub const JOY_HW_LASTENTRY: u32 = 13;
pub const JOY_HW_NONE: u32 = 0;
pub const JOY_HW_TWO_2A_2B_WITH_Y: u32 = 12;
pub const JOY_ISCAL_POV: i32 = 32;
pub const JOY_ISCAL_R: i32 = 4;
pub const JOY_ISCAL_U: i32 = 8;
pub const JOY_ISCAL_V: i32 = 16;
pub const JOY_ISCAL_XY: i32 = 1;
pub const JOY_ISCAL_Z: i32 = 2;
pub const JOY_OEMPOLL_PASSDRIVERDATA: u32 = 7;
pub const JOY_PASSDRIVERDATA: i32 = 268435456;
pub const JOY_POVVAL_BACKWARD: u32 = 1;
pub const JOY_POVVAL_FORWARD: u32 = 0;
pub const JOY_POVVAL_LEFT: u32 = 2;
pub const JOY_POVVAL_RIGHT: u32 = 3;
pub const JOY_POV_NUMDIRS: u32 = 4;
pub const JOY_US_HASRUDDER: i32 = 1;
pub const JOY_US_ISOEM: i32 = 4;
pub const JOY_US_PRESENT: i32 = 2;
pub const JOY_US_RESERVED: i32 = -2147483648;
pub const JOY_US_VOLATILE: i32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const KEYBOARD_CAPS_LOCK_ON: u32 = 4;
pub const KEYBOARD_ERROR_VALUE_BASE: u32 = 10000;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KEYBOARD_EXTENDED_ATTRIBUTES {
    pub Version: u8,
    pub FormFactor: u8,
    pub KeyType: u8,
    pub PhysicalLayout: u8,
    pub VendorSpecificPhysicalLayout: u8,
    pub IETFLanguageTagIndex: u8,
    pub ImplementedInputAssistControls: u8,
}
pub const KEYBOARD_EXTENDED_ATTRIBUTES_STRUCT_VERSION_1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KEYBOARD_ID {
    pub Type: u8,
    pub Subtype: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KEYBOARD_IME_STATUS {
    pub UnitId: u16,
    pub ImeOpen: u32,
    pub ImeConvMode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KEYBOARD_INDICATOR_PARAMETERS {
    pub UnitId: u16,
    pub LedFlags: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KEYBOARD_INPUT_DATA {
    pub UnitId: u16,
    pub MakeCode: u16,
    pub Flags: u16,
    pub Reserved: u16,
    pub ExtraInformation: u32,
}
pub const KEYBOARD_KANA_LOCK_ON: u32 = 8;
pub const KEYBOARD_LED_INJECTED: u32 = 32768;
pub const KEYBOARD_NUM_LOCK_ON: u32 = 2;
pub const KEYBOARD_OVERRUN_MAKE_CODE: u32 = 255;
pub const KEYBOARD_SCROLL_LOCK_ON: u32 = 1;
pub const KEYBOARD_SHADOW: u32 = 16384;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KEYBOARD_TYPEMATIC_PARAMETERS {
    pub UnitId: u16,
    pub Rate: u16,
    pub Delay: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KEYBOARD_UNIT_ID_PARAMETER {
    pub UnitId: u16,
}
pub const KEY_BREAK: u32 = 1;
pub const KEY_E0: u32 = 2;
pub const KEY_E1: u32 = 4;
pub const KEY_FROM_KEYBOARD_OVERRIDER: u32 = 128;
pub const KEY_MAKE: u32 = 0;
pub const KEY_RIM_VKEY: u32 = 64;
pub const KEY_TERMSRV_SET_LED: u32 = 8;
pub const KEY_TERMSRV_SHADOW: u32 = 16;
pub const KEY_TERMSRV_VKPACKET: u32 = 32;
pub const KEY_UNICODE_SEQUENCE_END: u32 = 512;
pub const KEY_UNICODE_SEQUENCE_ITEM: u32 = 256;
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
#[cfg(feature = "Win32_Foundation")]
pub type LPFNSHOWJOYCPL = Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND)>;
pub const MAXCPOINTSNUM: u32 = 8;
pub const MAX_JOYSTICKOEMVXDNAME: u32 = 260;
pub const MAX_JOYSTRING: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOUSE_ATTRIBUTES {
    pub MouseIdentifier: u16,
    pub NumberOfButtons: u16,
    pub SampleRate: u16,
    pub InputDataQueueLength: u32,
}
pub const MOUSE_BUTTON_1_DOWN: u32 = 1;
pub const MOUSE_BUTTON_1_UP: u32 = 2;
pub const MOUSE_BUTTON_2_DOWN: u32 = 4;
pub const MOUSE_BUTTON_2_UP: u32 = 8;
pub const MOUSE_BUTTON_3_DOWN: u32 = 16;
pub const MOUSE_BUTTON_3_UP: u32 = 32;
pub const MOUSE_BUTTON_4_DOWN: u32 = 64;
pub const MOUSE_BUTTON_4_UP: u32 = 128;
pub const MOUSE_BUTTON_5_DOWN: u32 = 256;
pub const MOUSE_BUTTON_5_UP: u32 = 512;
pub const MOUSE_ERROR_VALUE_BASE: u32 = 20000;
pub const MOUSE_HID_HARDWARE: u32 = 128;
pub const MOUSE_HWHEEL: u32 = 2048;
pub const MOUSE_I8042_HARDWARE: u32 = 2;
pub const MOUSE_INPORT_HARDWARE: u32 = 1;
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOUSE_INPUT_DATA_0_0 {
    pub ButtonFlags: u16,
    pub ButtonData: u16,
}
pub const MOUSE_LEFT_BUTTON_DOWN: u32 = 1;
pub const MOUSE_LEFT_BUTTON_UP: u32 = 2;
pub const MOUSE_MIDDLE_BUTTON_DOWN: u32 = 16;
pub const MOUSE_MIDDLE_BUTTON_UP: u32 = 32;
pub const MOUSE_RIGHT_BUTTON_DOWN: u32 = 4;
pub const MOUSE_RIGHT_BUTTON_UP: u32 = 8;
pub const MOUSE_SERIAL_HARDWARE: u32 = 4;
pub const MOUSE_TERMSRV_SRC_SHADOW: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MOUSE_UNIT_ID_PARAMETER {
    pub UnitId: u16,
}
pub const MOUSE_WHEEL: u32 = 1024;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_HidP_GetVersionInternal = Option<unsafe extern "system" fn(version: *mut u32) -> super::super::Foundation::NTSTATUS>;
pub type PHIDP_INSERT_SCANCODES = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, newscancodes: windows_core::PCSTR, length: u32) -> bool>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PHIDP_PREPARSED_DATA(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USAGE_AND_PAGE {
    pub Usage: u16,
    pub UsagePage: u16,
}
pub const WHEELMOUSE_HID_HARDWARE: u32 = 256;
pub const WHEELMOUSE_I8042_HARDWARE: u32 = 32;
pub const WHEELMOUSE_SERIAL_HARDWARE: u32 = 64;
