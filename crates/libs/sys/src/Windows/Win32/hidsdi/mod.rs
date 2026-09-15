#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_FlushQueue(hiddeviceobject : super::HANDLE) -> super::BOOLEAN);
#[cfg(all(feature = "hidpi", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_FreePreparsedData(preparseddata : super::PHIDP_PREPARSED_DATA) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetAttributes(hiddeviceobject : super::HANDLE, attributes : PHIDD_ATTRIBUTES) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetConfiguration(hiddeviceobject : super::HANDLE, configuration : PHIDD_CONFIGURATION, configurationlength : u32) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetFeature(hiddeviceobject : super::HANDLE, reportbuffer : *mut core::ffi::c_void, reportbufferlength : u32) -> super::BOOLEAN);
#[cfg(feature = "guiddef")]
windows_link::link!("hid.dll" "system" fn HidD_GetHidGuid(hidguid : super::LPGUID));
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetIndexedString(hiddeviceobject : super::HANDLE, stringindex : u32, buffer : *mut core::ffi::c_void, bufferlength : u32) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetInputReport(hiddeviceobject : super::HANDLE, reportbuffer : *mut core::ffi::c_void, reportbufferlength : u32) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetManufacturerString(hiddeviceobject : super::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetMsGenreDescriptor(hiddeviceobject : super::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> super::BOOLEAN);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetNumInputBuffers(hiddeviceobject : super::HANDLE, numberbuffers : super::PULONG) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetPhysicalDescriptor(hiddeviceobject : super::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> super::BOOLEAN);
#[cfg(all(feature = "hidpi", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetPreparsedData(hiddeviceobject : super::HANDLE, preparseddata : *mut super::PHIDP_PREPARSED_DATA) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetProductString(hiddeviceobject : super::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetSerialNumberString(hiddeviceobject : super::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_SetConfiguration(hiddeviceobject : super::HANDLE, configuration : PHIDD_CONFIGURATION, configurationlength : u32) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_SetFeature(hiddeviceobject : super::HANDLE, reportbuffer : *const core::ffi::c_void, reportbufferlength : u32) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_SetNumInputBuffers(hiddeviceobject : super::HANDLE, numberbuffers : u32) -> super::BOOLEAN);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_SetOutputReport(hiddeviceobject : super::HANDLE, reportbuffer : *const core::ffi::c_void, reportbufferlength : u32) -> super::BOOLEAN);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HIDD_ATTRIBUTES {
    pub Size: u32,
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct HIDD_CONFIGURATION {
    pub cookie: *mut core::ffi::c_void,
    pub size: u32,
    pub RingBufferSize: u32,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct HIDD_CONFIGURATION {
    pub cookie: *mut core::ffi::c_void,
    pub size: u32,
    pub RingBufferSize: u32,
}
pub type PHIDD_ATTRIBUTES = *mut HIDD_ATTRIBUTES;
pub type PHIDD_CONFIGURATION = *mut HIDD_CONFIGURATION;
