#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_GetButtonArray(reporttype: HIDP_REPORT_TYPE, usagepage: super::USAGE, linkcollection: Option<super::USHORT>, usage: super::USAGE, buttondata: PHIDP_BUTTON_ARRAY_DATA, buttondatalength: super::PUSHORT, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : super::USHORT, usage : super::USAGE, buttondata : PHIDP_BUTTON_ARRAY_DATA, buttondatalength : super::PUSHORT, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_GetButtonArray(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, buttondata as _, buttondatalength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn HidP_GetButtonCaps(reporttype: HIDP_REPORT_TYPE, buttoncaps: PHIDP_BUTTON_CAPS, buttoncapslength: super::PUSHORT, preparseddata: PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetButtonCaps(reporttype : HIDP_REPORT_TYPE, buttoncaps : PHIDP_BUTTON_CAPS, buttoncapslength : super::PUSHORT, preparseddata : PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS);
    unsafe { HidP_GetButtonCaps(reporttype, buttoncaps as _, buttoncapslength as _, preparseddata) }
}
#[cfg(all(feature = "hidusage", feature = "shared"))]
#[inline]
pub unsafe fn HidP_GetCaps(preparseddata: PHIDP_PREPARSED_DATA, capabilities: PHIDP_CAPS) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetCaps(preparseddata : PHIDP_PREPARSED_DATA, capabilities : PHIDP_CAPS) -> windows_core::NTSTATUS);
    unsafe { HidP_GetCaps(preparseddata, capabilities as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_GetData(reporttype: HIDP_REPORT_TYPE, datalist: PHIDP_DATA, datalength: super::PULONG, preparseddata: PHIDP_PREPARSED_DATA, report: super::PCHAR, reportlength: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetData(reporttype : HIDP_REPORT_TYPE, datalist : PHIDP_DATA, datalength : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_GetData(reporttype, datalist as _, datalength as _, preparseddata, report as _, reportlength) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn HidP_GetExtendedAttributes(reporttype: HIDP_REPORT_TYPE, dataindex: super::USHORT, preparseddata: PHIDP_PREPARSED_DATA, attributes: PHIDP_EXTENDED_ATTRIBUTES, lengthattributes: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetExtendedAttributes(reporttype : HIDP_REPORT_TYPE, dataindex : super::USHORT, preparseddata : PHIDP_PREPARSED_DATA, attributes : PHIDP_EXTENDED_ATTRIBUTES, lengthattributes : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_GetExtendedAttributes(reporttype, dataindex, preparseddata, attributes as _, lengthattributes as _) }
}
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_GetLinkCollectionNodes(linkcollectionnodes: PHIDP_LINK_COLLECTION_NODE, linkcollectionnodeslength: super::PULONG, preparseddata: PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetLinkCollectionNodes(linkcollectionnodes : PHIDP_LINK_COLLECTION_NODE, linkcollectionnodeslength : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS);
    unsafe { HidP_GetLinkCollectionNodes(linkcollectionnodes as _, linkcollectionnodeslength as _, preparseddata) }
}
#[cfg(all(feature = "hidusage", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_GetScaledUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: super::USAGE, linkcollection: Option<super::USHORT>, usage: super::USAGE, usagevalue: super::PLONG, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : super::USHORT, usage : super::USAGE, usagevalue : super::PLONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_GetScaledUsageValue(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, usagevalue as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn HidP_GetSpecificButtonCaps(reporttype: HIDP_REPORT_TYPE, usagepage: Option<super::USAGE>, linkcollection: Option<super::USHORT>, usage: Option<super::USAGE>, buttoncaps: PHIDP_BUTTON_CAPS, buttoncapslength: super::PUSHORT, preparseddata: PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetSpecificButtonCaps(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : super::USHORT, usage : super::USAGE, buttoncaps : PHIDP_BUTTON_CAPS, buttoncapslength : super::PUSHORT, preparseddata : PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS);
    unsafe { HidP_GetSpecificButtonCaps(reporttype, usagepage.unwrap_or(core::mem::zeroed()) as _, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage.unwrap_or(core::mem::zeroed()) as _, buttoncaps as _, buttoncapslength as _, preparseddata) }
}
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn HidP_GetSpecificValueCaps(reporttype: HIDP_REPORT_TYPE, usagepage: Option<super::USAGE>, linkcollection: Option<super::USHORT>, usage: Option<super::USAGE>, valuecaps: PHIDP_VALUE_CAPS, valuecapslength: super::PUSHORT, preparseddata: PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetSpecificValueCaps(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : super::USHORT, usage : super::USAGE, valuecaps : PHIDP_VALUE_CAPS, valuecapslength : super::PUSHORT, preparseddata : PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS);
    unsafe { HidP_GetSpecificValueCaps(reporttype, usagepage.unwrap_or(core::mem::zeroed()) as _, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage.unwrap_or(core::mem::zeroed()) as _, valuecaps as _, valuecapslength as _, preparseddata) }
}
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_GetUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: super::USAGE, linkcollection: Option<super::USHORT>, usage: super::USAGE, usagevalue: super::PULONG, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : super::USHORT, usage : super::USAGE, usagevalue : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_GetUsageValue(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, usagevalue as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(all(feature = "hidusage", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_GetUsageValueArray(reporttype: HIDP_REPORT_TYPE, usagepage: super::USAGE, linkcollection: Option<super::USHORT>, usage: super::USAGE, usagevalue: &mut [u8], preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : super::USHORT, usage : super::USAGE, usagevalue : super::PCHAR, usagevaluebytelength : super::USHORT, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_GetUsageValueArray(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, core::mem::transmute(usagevalue.as_mut_ptr()), usagevalue.len().try_into().unwrap(), preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_GetUsages(reporttype: HIDP_REPORT_TYPE, usagepage: super::USAGE, linkcollection: Option<super::USHORT>, usagelist: super::PUSAGE, usagelength: super::PULONG, preparseddata: PHIDP_PREPARSED_DATA, report: super::PCHAR, reportlength: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : super::USHORT, usagelist : super::PUSAGE, usagelength : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_GetUsages(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usagelist as _, usagelength as _, preparseddata, report as _, reportlength) }
}
#[cfg(all(feature = "hidusage", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_GetUsagesEx(reporttype: HIDP_REPORT_TYPE, linkcollection: Option<super::USHORT>, buttonlist: PUSAGE_AND_PAGE, usagelength: *mut super::ULONG, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetUsagesEx(reporttype : HIDP_REPORT_TYPE, linkcollection : super::USHORT, buttonlist : PUSAGE_AND_PAGE, usagelength : *mut super::ULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_GetUsagesEx(reporttype, linkcollection.unwrap_or(core::mem::zeroed()) as _, buttonlist as _, usagelength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn HidP_GetValueCaps(reporttype: HIDP_REPORT_TYPE, valuecaps: PHIDP_VALUE_CAPS, valuecapslength: super::PUSHORT, preparseddata: PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_GetValueCaps(reporttype : HIDP_REPORT_TYPE, valuecaps : PHIDP_VALUE_CAPS, valuecapslength : super::PUSHORT, preparseddata : PHIDP_PREPARSED_DATA) -> windows_core::NTSTATUS);
    unsafe { HidP_GetValueCaps(reporttype, valuecaps as _, valuecapslength as _, preparseddata) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_InitializeReportForID(reporttype: HIDP_REPORT_TYPE, reportid: super::UCHAR, preparseddata: PHIDP_PREPARSED_DATA, report: super::PCHAR, reportlength: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_InitializeReportForID(reporttype : HIDP_REPORT_TYPE, reportid : super::UCHAR, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_InitializeReportForID(reporttype, reportid, preparseddata, report as _, reportlength) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn HidP_MaxDataListLength(reporttype: HIDP_REPORT_TYPE, preparseddata: PHIDP_PREPARSED_DATA) -> super::ULONG {
    windows_core::link!("hid.dll" "system" fn HidP_MaxDataListLength(reporttype : HIDP_REPORT_TYPE, preparseddata : PHIDP_PREPARSED_DATA) -> super::ULONG);
    unsafe { HidP_MaxDataListLength(reporttype, preparseddata) }
}
#[cfg(all(feature = "hidusage", feature = "shared"))]
#[inline]
pub unsafe fn HidP_MaxUsageListLength(reporttype: HIDP_REPORT_TYPE, usagepage: Option<super::USAGE>, preparseddata: PHIDP_PREPARSED_DATA) -> super::ULONG {
    windows_core::link!("hid.dll" "system" fn HidP_MaxUsageListLength(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, preparseddata : PHIDP_PREPARSED_DATA) -> super::ULONG);
    unsafe { HidP_MaxUsageListLength(reporttype, usagepage.unwrap_or(core::mem::zeroed()) as _, preparseddata) }
}
#[cfg(all(feature = "hidusage", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_SetButtonArray(reporttype: HIDP_REPORT_TYPE, usagepage: super::USAGE, linkcollection: Option<super::USHORT>, usage: super::USAGE, buttondata: &[HIDP_BUTTON_ARRAY_DATA], preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetButtonArray(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : super::USHORT, usage : super::USAGE, buttondata : PHIDP_BUTTON_ARRAY_DATA, buttondatalength : super::USHORT, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_SetButtonArray(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, core::mem::transmute(buttondata.as_ptr()), buttondata.len().try_into().unwrap(), preparseddata, core::mem::transmute(report.as_mut_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_SetData(reporttype: HIDP_REPORT_TYPE, datalist: PHIDP_DATA, datalength: super::PULONG, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetData(reporttype : HIDP_REPORT_TYPE, datalist : PHIDP_DATA, datalength : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_SetData(reporttype, datalist as _, datalength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(all(feature = "hidusage", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_SetScaledUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: super::USAGE, linkcollection: Option<super::USHORT>, usage: super::USAGE, usagevalue: super::LONG, preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetScaledUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : super::USHORT, usage : super::USAGE, usagevalue : super::LONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_SetScaledUsageValue(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, usagevalue, preparseddata, core::mem::transmute(report.as_mut_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(all(feature = "hidusage", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_SetUsageValue(reporttype: HIDP_REPORT_TYPE, usagepage: super::USAGE, linkcollection: Option<super::USHORT>, usage: super::USAGE, usagevalue: super::ULONG, preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetUsageValue(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : super::USHORT, usage : super::USAGE, usagevalue : super::ULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_SetUsageValue(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, usagevalue, preparseddata, core::mem::transmute(report.as_mut_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(all(feature = "hidusage", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_SetUsageValueArray(reporttype: HIDP_REPORT_TYPE, usagepage: super::USAGE, linkcollection: Option<super::USHORT>, usage: super::USAGE, usagevalue: &[u8], preparseddata: PHIDP_PREPARSED_DATA, report: &mut [u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetUsageValueArray(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : super::USHORT, usage : super::USAGE, usagevalue : super::PCHAR, usagevaluebytelength : super::USHORT, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_SetUsageValueArray(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usage, core::mem::transmute(usagevalue.as_ptr()), usagevalue.len().try_into().unwrap(), preparseddata, core::mem::transmute(report.as_mut_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_SetUsages(reporttype: HIDP_REPORT_TYPE, usagepage: super::USAGE, linkcollection: Option<super::USHORT>, usagelist: super::PUSAGE, usagelength: super::PULONG, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_SetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : super::USHORT, usagelist : super::PUSAGE, usagelength : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_SetUsages(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usagelist as _, usagelength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(all(feature = "hidusage", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_TranslateUsagesToI8042ScanCodes(changedusagelist: &[u16], keyaction: HIDP_KEYBOARD_DIRECTION, modifierstate: PHIDP_KEYBOARD_MODIFIER_STATE, insertcodesprocedure: PHIDP_INSERT_SCANCODES, insertcodescontext: Option<super::PVOID>) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_TranslateUsagesToI8042ScanCodes(changedusagelist : super::PUSAGE, usagelistlength : super::ULONG, keyaction : HIDP_KEYBOARD_DIRECTION, modifierstate : PHIDP_KEYBOARD_MODIFIER_STATE, insertcodesprocedure : PHIDP_INSERT_SCANCODES, insertcodescontext : super::PVOID) -> windows_core::NTSTATUS);
    unsafe { HidP_TranslateUsagesToI8042ScanCodes(core::mem::transmute(changedusagelist.as_ptr()), changedusagelist.len().try_into().unwrap(), keyaction, modifierstate as _, insertcodesprocedure, insertcodescontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "hidusage", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidP_UnsetUsages(reporttype: HIDP_REPORT_TYPE, usagepage: super::USAGE, linkcollection: Option<super::USHORT>, usagelist: super::PUSAGE, usagelength: super::PULONG, preparseddata: PHIDP_PREPARSED_DATA, report: &[u8]) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_UnsetUsages(reporttype : HIDP_REPORT_TYPE, usagepage : super::USAGE, linkcollection : super::USHORT, usagelist : super::PUSAGE, usagelength : super::PULONG, preparseddata : PHIDP_PREPARSED_DATA, report : super::PCHAR, reportlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_UnsetUsages(reporttype, usagepage, linkcollection.unwrap_or(core::mem::zeroed()) as _, usagelist as _, usagelength as _, preparseddata, core::mem::transmute(report.as_ptr()), report.len().try_into().unwrap()) }
}
#[cfg(all(feature = "hidusage", feature = "shared"))]
#[inline]
pub unsafe fn HidP_UsageListDifference(previoususagelist: super::PUSAGE, currentusagelist: super::PUSAGE, breakusagelist: super::PUSAGE, makeusagelist: super::PUSAGE, usagelistlength: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("hid.dll" "system" fn HidP_UsageListDifference(previoususagelist : super::PUSAGE, currentusagelist : super::PUSAGE, breakusagelist : super::PUSAGE, makeusagelist : super::PUSAGE, usagelistlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { HidP_UsageListDifference(previoususagelist, currentusagelist, breakusagelist as _, makeusagelist as _, usagelistlength) }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HIDP_BUTTON_ARRAY_DATA {
    pub ArrayIndex: super::USHORT,
    pub On: super::BOOLEAN,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "hidusage", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct HIDP_BUTTON_CAPS {
    pub UsagePage: super::USAGE,
    pub ReportID: super::UCHAR,
    pub IsAlias: super::BOOLEAN,
    pub BitField: super::USHORT,
    pub LinkCollection: super::USHORT,
    pub LinkUsage: super::USAGE,
    pub LinkUsagePage: super::USAGE,
    pub IsRange: super::BOOLEAN,
    pub IsStringRange: super::BOOLEAN,
    pub IsDesignatorRange: super::BOOLEAN,
    pub IsAbsolute: super::BOOLEAN,
    pub ReportCount: super::USHORT,
    pub Reserved2: super::USHORT,
    pub Reserved: [super::ULONG; 9],
    pub Anonymous: HIDP_BUTTON_CAPS_0,
}
#[cfg(all(feature = "hidusage", feature = "shared"))]
impl Default for HIDP_BUTTON_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "hidusage", feature = "shared"))]
#[derive(Clone, Copy)]
pub union HIDP_BUTTON_CAPS_0 {
    pub Range: HIDP_BUTTON_CAPS_0_0,
    pub NotRange: HIDP_BUTTON_CAPS_0_1,
}
#[cfg(all(feature = "hidusage", feature = "shared"))]
impl Default for HIDP_BUTTON_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "hidusage", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct HIDP_BUTTON_CAPS_0_0 {
    pub UsageMin: super::USAGE,
    pub UsageMax: super::USAGE,
    pub StringMin: super::USHORT,
    pub StringMax: super::USHORT,
    pub DesignatorMin: super::USHORT,
    pub DesignatorMax: super::USHORT,
    pub DataIndexMin: super::USHORT,
    pub DataIndexMax: super::USHORT,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "hidusage", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct HIDP_BUTTON_CAPS_0_1 {
    pub Usage: super::USAGE,
    pub Reserved1: super::USAGE,
    pub StringIndex: super::USHORT,
    pub Reserved2: super::USHORT,
    pub DesignatorIndex: super::USHORT,
    pub Reserved3: super::USHORT,
    pub DataIndex: super::USHORT,
    pub Reserved4: super::USHORT,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "hidusage", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct HIDP_CAPS {
    pub Usage: super::USAGE,
    pub UsagePage: super::USAGE,
    pub InputReportByteLength: super::USHORT,
    pub OutputReportByteLength: super::USHORT,
    pub FeatureReportByteLength: super::USHORT,
    pub Reserved: [super::USHORT; 17],
    pub NumberLinkCollectionNodes: super::USHORT,
    pub NumberInputButtonCaps: super::USHORT,
    pub NumberInputValueCaps: super::USHORT,
    pub NumberInputDataIndices: super::USHORT,
    pub NumberOutputButtonCaps: super::USHORT,
    pub NumberOutputValueCaps: super::USHORT,
    pub NumberOutputDataIndices: super::USHORT,
    pub NumberFeatureButtonCaps: super::USHORT,
    pub NumberFeatureValueCaps: super::USHORT,
    pub NumberFeatureDataIndices: super::USHORT,
}
#[cfg(all(feature = "hidusage", feature = "shared"))]
impl Default for HIDP_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct HIDP_DATA {
    pub DataIndex: super::USHORT,
    pub Reserved: super::USHORT,
    pub Anonymous: HIDP_DATA_0,
}
#[cfg(feature = "shared")]
impl Default for HIDP_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union HIDP_DATA_0 {
    pub RawValue: super::ULONG,
    pub On: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for HIDP_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct HIDP_EXTENDED_ATTRIBUTES {
    pub NumGlobalUnknowns: super::UCHAR,
    pub Reserved: [super::UCHAR; 3],
    pub GlobalUnknowns: PHIDP_UNKNOWN_TOKEN,
    pub Data: [super::ULONG; 1],
}
#[cfg(feature = "shared")]
impl Default for HIDP_EXTENDED_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HIDP_KEYBOARD_DIRECTION = i32;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct HIDP_KEYBOARD_MODIFIER_STATE {
    pub Anonymous: HIDP_KEYBOARD_MODIFIER_STATE_0,
}
#[cfg(feature = "shared")]
impl Default for HIDP_KEYBOARD_MODIFIER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union HIDP_KEYBOARD_MODIFIER_STATE_0 {
    pub Anonymous: HIDP_KEYBOARD_MODIFIER_STATE_0_0,
    pub ul: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for HIDP_KEYBOARD_MODIFIER_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HIDP_KEYBOARD_MODIFIER_STATE_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl HIDP_KEYBOARD_MODIFIER_STATE_0_0 {
    pub fn LeftControl(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_LeftControl(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn LeftShift(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_LeftShift(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn LeftAlt(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_LeftAlt(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn LeftGUI(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_LeftGUI(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn RightControl(&self) -> super::ULONG {
        (self._bitfield >> 4) & 1
    }
    pub fn set_RightControl(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn RightShift(&self) -> super::ULONG {
        (self._bitfield >> 5) & 1
    }
    pub fn set_RightShift(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn RightAlt(&self) -> super::ULONG {
        (self._bitfield >> 6) & 1
    }
    pub fn set_RightAlt(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn RigthGUI(&self) -> super::ULONG {
        (self._bitfield >> 7) & 1
    }
    pub fn set_RigthGUI(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn CapsLock(&self) -> super::ULONG {
        (self._bitfield >> 8) & 1
    }
    pub fn set_CapsLock(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn ScollLock(&self) -> super::ULONG {
        (self._bitfield >> 9) & 1
    }
    pub fn set_ScollLock(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn NumLock(&self) -> super::ULONG {
        (self._bitfield >> 10) & 1
    }
    pub fn set_NumLock(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value & 1) << 10);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 11
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(2097151 << 11)) | ((value & 2097151) << 11);
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "hidusage", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HIDP_LINK_COLLECTION_NODE {
    pub LinkUsage: super::USAGE,
    pub LinkUsagePage: super::USAGE,
    pub Parent: super::USHORT,
    pub NumberOfChildren: super::USHORT,
    pub NextSibling: super::USHORT,
    pub FirstChild: super::USHORT,
    pub _bitfield: u32,
    pub UserContext: super::PVOID,
}
#[cfg(all(feature = "hidusage", feature = "shared", feature = "winnt"))]
impl HIDP_LINK_COLLECTION_NODE {
    pub fn CollectionType(&self) -> super::ULONG {
        (self._bitfield) & 255
    }
    pub fn set_CollectionType(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !255) | (value & 255);
    }
    pub fn IsAlias(&self) -> super::ULONG {
        (self._bitfield >> 8) & 1
    }
    pub fn set_IsAlias(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 9
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(8388607 << 9)) | ((value & 8388607) << 9);
    }
}
#[cfg(feature = "shared")]
pub const HIDP_LINK_COLLECTION_ROOT: super::USHORT = 65535;
#[cfg(feature = "shared")]
pub const HIDP_LINK_COLLECTION_UNSPECIFIED: super::USHORT = 0;
pub type HIDP_REPORT_TYPE = i32;
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
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct HIDP_UNKNOWN_TOKEN {
    pub Token: super::UCHAR,
    pub Reserved: [super::UCHAR; 3],
    pub BitField: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for HIDP_UNKNOWN_TOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "hidusage", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct HIDP_VALUE_CAPS {
    pub UsagePage: super::USAGE,
    pub ReportID: super::UCHAR,
    pub IsAlias: super::BOOLEAN,
    pub BitField: super::USHORT,
    pub LinkCollection: super::USHORT,
    pub LinkUsage: super::USAGE,
    pub LinkUsagePage: super::USAGE,
    pub IsRange: super::BOOLEAN,
    pub IsStringRange: super::BOOLEAN,
    pub IsDesignatorRange: super::BOOLEAN,
    pub IsAbsolute: super::BOOLEAN,
    pub HasNull: super::BOOLEAN,
    pub Reserved: super::UCHAR,
    pub BitSize: super::USHORT,
    pub ReportCount: super::USHORT,
    pub Reserved2: [super::USHORT; 5],
    pub UnitsExp: super::ULONG,
    pub Units: super::ULONG,
    pub LogicalMin: super::LONG,
    pub LogicalMax: super::LONG,
    pub PhysicalMin: super::LONG,
    pub PhysicalMax: super::LONG,
    pub Anonymous: HIDP_VALUE_CAPS_0,
}
#[cfg(all(feature = "hidusage", feature = "shared"))]
impl Default for HIDP_VALUE_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "hidusage", feature = "shared"))]
#[derive(Clone, Copy)]
pub union HIDP_VALUE_CAPS_0 {
    pub Range: HIDP_VALUE_CAPS_0_0,
    pub NotRange: HIDP_VALUE_CAPS_0_1,
}
#[cfg(all(feature = "hidusage", feature = "shared"))]
impl Default for HIDP_VALUE_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(all(feature = "hidusage", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct HIDP_VALUE_CAPS_0_0 {
    pub UsageMin: super::USAGE,
    pub UsageMax: super::USAGE,
    pub StringMin: super::USHORT,
    pub StringMax: super::USHORT,
    pub DesignatorMin: super::USHORT,
    pub DesignatorMax: super::USHORT,
    pub DataIndexMin: super::USHORT,
    pub DataIndexMax: super::USHORT,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "hidusage", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct HIDP_VALUE_CAPS_0_1 {
    pub Usage: super::USAGE,
    pub Reserved1: super::USAGE,
    pub StringIndex: super::USHORT,
    pub Reserved2: super::USHORT,
    pub DesignatorIndex: super::USHORT,
    pub Reserved3: super::USHORT,
    pub DataIndex: super::USHORT,
    pub Reserved4: super::USHORT,
}
pub const HidP_Feature: HIDP_REPORT_TYPE = 2;
pub const HidP_Input: HIDP_REPORT_TYPE = 0;
pub const HidP_Keyboard_Break: HIDP_KEYBOARD_DIRECTION = 0;
pub const HidP_Keyboard_Make: HIDP_KEYBOARD_DIRECTION = 1;
pub const HidP_Output: HIDP_REPORT_TYPE = 1;
#[cfg(feature = "shared")]
pub type PFN_HidP_GetVersionInternal = Option<unsafe extern "C" fn(version: *mut super::ULONG) -> windows_core::NTSTATUS>;
#[cfg(feature = "shared")]
pub type PHIDP_BUTTON_ARRAY_DATA = *mut HIDP_BUTTON_ARRAY_DATA;
#[cfg(all(feature = "hidusage", feature = "shared"))]
pub type PHIDP_BUTTON_CAPS = *mut HIDP_BUTTON_CAPS;
#[cfg(all(feature = "hidusage", feature = "shared"))]
pub type PHIDP_CAPS = *mut HIDP_CAPS;
#[cfg(feature = "shared")]
pub type PHIDP_DATA = *mut HIDP_DATA;
#[cfg(feature = "shared")]
pub type PHIDP_EXTENDED_ATTRIBUTES = *mut HIDP_EXTENDED_ATTRIBUTES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHIDP_INSERT_SCANCODES = Option<unsafe extern "C" fn(context: super::PVOID, newscancodes: super::PCHAR, length: super::ULONG) -> super::BOOLEAN>;
#[cfg(feature = "shared")]
pub type PHIDP_KEYBOARD_MODIFIER_STATE = *mut HIDP_KEYBOARD_MODIFIER_STATE;
#[cfg(all(feature = "hidusage", feature = "shared", feature = "winnt"))]
pub type PHIDP_LINK_COLLECTION_NODE = *mut HIDP_LINK_COLLECTION_NODE;
pub type PHIDP_PREPARSED_DATA = *mut _HIDP_PREPARSED_DATA;
pub type PHIDP_REPORT_DESCRIPTOR = *mut u8;
#[cfg(feature = "shared")]
pub type PHIDP_UNKNOWN_TOKEN = *mut HIDP_UNKNOWN_TOKEN;
#[cfg(all(feature = "hidusage", feature = "shared"))]
pub type PHIDP_VALUE_CAPS = *mut HIDP_VALUE_CAPS;
#[cfg(feature = "hidusage")]
pub type PUSAGE_AND_PAGE = *mut USAGE_AND_PAGE;
#[repr(C, packed(4))]
#[cfg(feature = "hidusage")]
#[derive(Clone, Copy, Default)]
pub struct USAGE_AND_PAGE {
    pub Usage: super::USAGE,
    pub UsagePage: super::USAGE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _HIDP_PREPARSED_DATA(pub u8);
