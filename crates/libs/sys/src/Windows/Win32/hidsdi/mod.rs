#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_FlushQueue(hiddeviceobject : super::HANDLE) -> super::BOOLEAN);
#[cfg(all(feature = "hidpi", feature = "shared"))]
windows_link::link!("hid.dll" "system" fn HidD_FreePreparsedData(preparseddata : super::PHIDP_PREPARSED_DATA) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetAttributes(hiddeviceobject : super::HANDLE, attributes : PHIDD_ATTRIBUTES) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetConfiguration(hiddeviceobject : super::HANDLE, configuration : PHIDD_CONFIGURATION, configurationlength : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetFeature(hiddeviceobject : super::HANDLE, reportbuffer : super::PVOID, reportbufferlength : super::ULONG) -> super::BOOLEAN);
#[cfg(feature = "guiddef")]
windows_link::link!("hid.dll" "system" fn HidD_GetHidGuid(hidguid : super::LPGUID));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetIndexedString(hiddeviceobject : super::HANDLE, stringindex : super::ULONG, buffer : super::PVOID, bufferlength : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetInputReport(hiddeviceobject : super::HANDLE, reportbuffer : super::PVOID, reportbufferlength : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetManufacturerString(hiddeviceobject : super::HANDLE, buffer : super::PVOID, bufferlength : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetMsGenreDescriptor(hiddeviceobject : super::HANDLE, buffer : super::PVOID, bufferlength : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetNumInputBuffers(hiddeviceobject : super::HANDLE, numberbuffers : super::PULONG) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetPhysicalDescriptor(hiddeviceobject : super::HANDLE, buffer : super::PVOID, bufferlength : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "hidpi", feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetPreparsedData(hiddeviceobject : super::HANDLE, preparseddata : *mut super::PHIDP_PREPARSED_DATA) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetProductString(hiddeviceobject : super::HANDLE, buffer : super::PVOID, bufferlength : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetSerialNumberString(hiddeviceobject : super::HANDLE, buffer : super::PVOID, bufferlength : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_SetConfiguration(hiddeviceobject : super::HANDLE, configuration : PHIDD_CONFIGURATION, configurationlength : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_SetFeature(hiddeviceobject : super::HANDLE, reportbuffer : super::PVOID, reportbufferlength : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_SetNumInputBuffers(hiddeviceobject : super::HANDLE, numberbuffers : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_SetOutputReport(hiddeviceobject : super::HANDLE, reportbuffer : super::PVOID, reportbufferlength : super::ULONG) -> super::BOOLEAN);
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
