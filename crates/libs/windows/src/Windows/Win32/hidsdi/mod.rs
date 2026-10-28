#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_FlushQueue(hiddeviceobject: super::HANDLE) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_FlushQueue(hiddeviceobject : super::HANDLE) -> super::BOOLEAN);
    unsafe { HidD_FlushQueue(hiddeviceobject) }
}
#[cfg(all(feature = "hidpi", feature = "shared"))]
#[inline]
pub unsafe fn HidD_FreePreparsedData(preparseddata: super::PHIDP_PREPARSED_DATA) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_FreePreparsedData(preparseddata : super::PHIDP_PREPARSED_DATA) -> super::BOOLEAN);
    unsafe { HidD_FreePreparsedData(preparseddata) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_GetAttributes(hiddeviceobject: super::HANDLE, attributes: PHIDD_ATTRIBUTES) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_GetAttributes(hiddeviceobject : super::HANDLE, attributes : PHIDD_ATTRIBUTES) -> super::BOOLEAN);
    unsafe { HidD_GetAttributes(hiddeviceobject, attributes as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_GetConfiguration(hiddeviceobject: super::HANDLE, configuration: PHIDD_CONFIGURATION, configurationlength: super::ULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_GetConfiguration(hiddeviceobject : super::HANDLE, configuration : PHIDD_CONFIGURATION, configurationlength : super::ULONG) -> super::BOOLEAN);
    unsafe { HidD_GetConfiguration(hiddeviceobject, configuration as _, configurationlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_GetFeature(hiddeviceobject: super::HANDLE, reportbuffer: super::PVOID, reportbufferlength: super::ULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_GetFeature(hiddeviceobject : super::HANDLE, reportbuffer : super::PVOID, reportbufferlength : super::ULONG) -> super::BOOLEAN);
    unsafe { HidD_GetFeature(hiddeviceobject, reportbuffer as _, reportbufferlength) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn HidD_GetHidGuid() -> windows_core::GUID {
    windows_core::link!("hid.dll" "system" fn HidD_GetHidGuid(hidguid : super::LPGUID));
    unsafe {
        let mut result__ = core::mem::zeroed();
        HidD_GetHidGuid(&mut result__);
        result__
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_GetIndexedString(hiddeviceobject: super::HANDLE, stringindex: super::ULONG, buffer: super::PVOID, bufferlength: super::ULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_GetIndexedString(hiddeviceobject : super::HANDLE, stringindex : super::ULONG, buffer : super::PVOID, bufferlength : super::ULONG) -> super::BOOLEAN);
    unsafe { HidD_GetIndexedString(hiddeviceobject, stringindex, buffer as _, bufferlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_GetInputReport(hiddeviceobject: super::HANDLE, reportbuffer: super::PVOID, reportbufferlength: super::ULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_GetInputReport(hiddeviceobject : super::HANDLE, reportbuffer : super::PVOID, reportbufferlength : super::ULONG) -> super::BOOLEAN);
    unsafe { HidD_GetInputReport(hiddeviceobject, reportbuffer as _, reportbufferlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_GetManufacturerString(hiddeviceobject: super::HANDLE, buffer: super::PVOID, bufferlength: super::ULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_GetManufacturerString(hiddeviceobject : super::HANDLE, buffer : super::PVOID, bufferlength : super::ULONG) -> super::BOOLEAN);
    unsafe { HidD_GetManufacturerString(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_GetMsGenreDescriptor(hiddeviceobject: super::HANDLE, buffer: super::PVOID, bufferlength: super::ULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_GetMsGenreDescriptor(hiddeviceobject : super::HANDLE, buffer : super::PVOID, bufferlength : super::ULONG) -> super::BOOLEAN);
    unsafe { HidD_GetMsGenreDescriptor(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_GetNumInputBuffers(hiddeviceobject: super::HANDLE, numberbuffers: super::PULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_GetNumInputBuffers(hiddeviceobject : super::HANDLE, numberbuffers : super::PULONG) -> super::BOOLEAN);
    unsafe { HidD_GetNumInputBuffers(hiddeviceobject, numberbuffers as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_GetPhysicalDescriptor(hiddeviceobject: super::HANDLE, buffer: super::PVOID, bufferlength: super::ULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_GetPhysicalDescriptor(hiddeviceobject : super::HANDLE, buffer : super::PVOID, bufferlength : super::ULONG) -> super::BOOLEAN);
    unsafe { HidD_GetPhysicalDescriptor(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(all(feature = "hidpi", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_GetPreparsedData(hiddeviceobject: super::HANDLE, preparseddata: *mut super::PHIDP_PREPARSED_DATA) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_GetPreparsedData(hiddeviceobject : super::HANDLE, preparseddata : *mut super::PHIDP_PREPARSED_DATA) -> super::BOOLEAN);
    unsafe { HidD_GetPreparsedData(hiddeviceobject, preparseddata as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_GetProductString(hiddeviceobject: super::HANDLE, buffer: super::PVOID, bufferlength: super::ULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_GetProductString(hiddeviceobject : super::HANDLE, buffer : super::PVOID, bufferlength : super::ULONG) -> super::BOOLEAN);
    unsafe { HidD_GetProductString(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_GetSerialNumberString(hiddeviceobject: super::HANDLE, buffer: super::PVOID, bufferlength: super::ULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_GetSerialNumberString(hiddeviceobject : super::HANDLE, buffer : super::PVOID, bufferlength : super::ULONG) -> super::BOOLEAN);
    unsafe { HidD_GetSerialNumberString(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_SetConfiguration(hiddeviceobject: super::HANDLE, configuration: PHIDD_CONFIGURATION, configurationlength: super::ULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_SetConfiguration(hiddeviceobject : super::HANDLE, configuration : PHIDD_CONFIGURATION, configurationlength : super::ULONG) -> super::BOOLEAN);
    unsafe { HidD_SetConfiguration(hiddeviceobject, configuration, configurationlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_SetFeature(hiddeviceobject: super::HANDLE, reportbuffer: super::PVOID, reportbufferlength: super::ULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_SetFeature(hiddeviceobject : super::HANDLE, reportbuffer : super::PVOID, reportbufferlength : super::ULONG) -> super::BOOLEAN);
    unsafe { HidD_SetFeature(hiddeviceobject, reportbuffer, reportbufferlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_SetNumInputBuffers(hiddeviceobject: super::HANDLE, numberbuffers: super::ULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_SetNumInputBuffers(hiddeviceobject : super::HANDLE, numberbuffers : super::ULONG) -> super::BOOLEAN);
    unsafe { HidD_SetNumInputBuffers(hiddeviceobject, numberbuffers) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_SetOutputReport(hiddeviceobject: super::HANDLE, reportbuffer: super::PVOID, reportbufferlength: super::ULONG) -> super::BOOLEAN {
    windows_core::link!("hid.dll" "system" fn HidD_SetOutputReport(hiddeviceobject : super::HANDLE, reportbuffer : super::PVOID, reportbufferlength : super::ULONG) -> super::BOOLEAN);
    unsafe { HidD_SetOutputReport(hiddeviceobject, reportbuffer, reportbufferlength) }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HIDD_ATTRIBUTES {
    pub Size: super::ULONG,
    pub VendorID: super::USHORT,
    pub ProductID: super::USHORT,
    pub VersionNumber: super::USHORT,
}
#[repr(C, packed(4))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HIDD_CONFIGURATION {
    pub cookie: super::PVOID,
    pub size: super::ULONG,
    pub RingBufferSize: super::ULONG,
}
#[cfg(feature = "shared")]
pub type PHIDD_ATTRIBUTES = *mut HIDD_ATTRIBUTES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHIDD_CONFIGURATION = *mut HIDD_CONFIGURATION;
