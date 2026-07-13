#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_FlushQueue(hiddeviceobject : super::winnt::HANDLE) -> bool);
#[cfg(feature = "hidpi")]
windows_link::link!("hid.dll" "system" fn HidD_FreePreparsedData(preparseddata : *const super::hidpi::_HIDP_PREPARSED_DATA) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetAttributes(hiddeviceobject : super::winnt::HANDLE, attributes : *mut HIDD_ATTRIBUTES) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetConfiguration(hiddeviceobject : super::winnt::HANDLE, configuration : *mut HIDD_CONFIGURATION, configurationlength : u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetFeature(hiddeviceobject : super::winnt::HANDLE, reportbuffer : *mut core::ffi::c_void, reportbufferlength : u32) -> bool);
windows_link::link!("hid.dll" "system" fn HidD_GetHidGuid(hidguid : *mut windows_sys::core::GUID));
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetIndexedString(hiddeviceobject : super::winnt::HANDLE, stringindex : u32, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetInputReport(hiddeviceobject : super::winnt::HANDLE, reportbuffer : *mut core::ffi::c_void, reportbufferlength : u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetManufacturerString(hiddeviceobject : super::winnt::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetMsGenreDescriptor(hiddeviceobject : super::winnt::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetNumInputBuffers(hiddeviceobject : super::winnt::HANDLE, numberbuffers : *mut u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetPhysicalDescriptor(hiddeviceobject : super::winnt::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
#[cfg(all(feature = "hidpi", feature = "winnt"))]
windows_link::link!("hid.dll" "system" fn HidD_GetPreparsedData(hiddeviceobject : super::winnt::HANDLE, preparseddata : *mut super::hidpi::PHIDP_PREPARSED_DATA) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetProductString(hiddeviceobject : super::winnt::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_GetSerialNumberString(hiddeviceobject : super::winnt::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_SetConfiguration(hiddeviceobject : super::winnt::HANDLE, configuration : *const HIDD_CONFIGURATION, configurationlength : u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_SetFeature(hiddeviceobject : super::winnt::HANDLE, reportbuffer : *const core::ffi::c_void, reportbufferlength : u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_SetNumInputBuffers(hiddeviceobject : super::winnt::HANDLE, numberbuffers : u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("hid.dll" "system" fn HidD_SetOutputReport(hiddeviceobject : super::winnt::HANDLE, reportbuffer : *const core::ffi::c_void, reportbufferlength : u32) -> bool);
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
#[derive(Clone, Copy)]
pub struct HIDD_CONFIGURATION {
    pub cookie: *mut core::ffi::c_void,
    pub size: u32,
    pub RingBufferSize: u32,
}
#[cfg(target_arch = "x86")]
impl Default for HIDD_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct HIDD_CONFIGURATION {
    pub cookie: *mut core::ffi::c_void,
    pub size: u32,
    pub RingBufferSize: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for HIDD_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PHIDD_ATTRIBUTES = *mut HIDD_ATTRIBUTES;
pub type PHIDD_CONFIGURATION = *mut HIDD_CONFIGURATION;
