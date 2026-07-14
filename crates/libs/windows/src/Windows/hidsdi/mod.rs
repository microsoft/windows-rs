#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_FlushQueue(hiddeviceobject: super::winnt::HANDLE) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_FlushQueue(hiddeviceobject : super::winnt::HANDLE) -> bool);
    unsafe { HidD_FlushQueue(hiddeviceobject) }
}
#[cfg(feature = "hidpi")]
#[inline]
pub unsafe fn HidD_FreePreparsedData(preparseddata: *const super::hidpi::_HIDP_PREPARSED_DATA) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_FreePreparsedData(preparseddata : *const super::hidpi::_HIDP_PREPARSED_DATA) -> bool);
    unsafe { HidD_FreePreparsedData(preparseddata) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_GetAttributes(hiddeviceobject: super::winnt::HANDLE, attributes: *mut HIDD_ATTRIBUTES) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetAttributes(hiddeviceobject : super::winnt::HANDLE, attributes : *mut HIDD_ATTRIBUTES) -> bool);
    unsafe { HidD_GetAttributes(hiddeviceobject, attributes as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_GetConfiguration(hiddeviceobject: super::winnt::HANDLE, configuration: *mut HIDD_CONFIGURATION, configurationlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetConfiguration(hiddeviceobject : super::winnt::HANDLE, configuration : *mut HIDD_CONFIGURATION, configurationlength : u32) -> bool);
    unsafe { HidD_GetConfiguration(hiddeviceobject, configuration as _, configurationlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_GetFeature(hiddeviceobject: super::winnt::HANDLE, reportbuffer: *mut core::ffi::c_void, reportbufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetFeature(hiddeviceobject : super::winnt::HANDLE, reportbuffer : *mut core::ffi::c_void, reportbufferlength : u32) -> bool);
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
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_GetIndexedString(hiddeviceobject: super::winnt::HANDLE, stringindex: u32, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetIndexedString(hiddeviceobject : super::winnt::HANDLE, stringindex : u32, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetIndexedString(hiddeviceobject, stringindex, buffer as _, bufferlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_GetInputReport(hiddeviceobject: super::winnt::HANDLE, reportbuffer: *mut core::ffi::c_void, reportbufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetInputReport(hiddeviceobject : super::winnt::HANDLE, reportbuffer : *mut core::ffi::c_void, reportbufferlength : u32) -> bool);
    unsafe { HidD_GetInputReport(hiddeviceobject, reportbuffer as _, reportbufferlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_GetManufacturerString(hiddeviceobject: super::winnt::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetManufacturerString(hiddeviceobject : super::winnt::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetManufacturerString(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_GetMsGenreDescriptor(hiddeviceobject: super::winnt::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetMsGenreDescriptor(hiddeviceobject : super::winnt::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetMsGenreDescriptor(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_GetNumInputBuffers(hiddeviceobject: super::winnt::HANDLE, numberbuffers: *mut u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetNumInputBuffers(hiddeviceobject : super::winnt::HANDLE, numberbuffers : *mut u32) -> bool);
    unsafe { HidD_GetNumInputBuffers(hiddeviceobject, numberbuffers as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_GetPhysicalDescriptor(hiddeviceobject: super::winnt::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetPhysicalDescriptor(hiddeviceobject : super::winnt::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetPhysicalDescriptor(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(all(feature = "hidpi", feature = "winnt"))]
#[inline]
pub unsafe fn HidD_GetPreparsedData(hiddeviceobject: super::winnt::HANDLE, preparseddata: *mut super::hidpi::PHIDP_PREPARSED_DATA) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetPreparsedData(hiddeviceobject : super::winnt::HANDLE, preparseddata : *mut super::hidpi::PHIDP_PREPARSED_DATA) -> bool);
    unsafe { HidD_GetPreparsedData(hiddeviceobject, preparseddata as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_GetProductString(hiddeviceobject: super::winnt::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetProductString(hiddeviceobject : super::winnt::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetProductString(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_GetSerialNumberString(hiddeviceobject: super::winnt::HANDLE, buffer: *mut core::ffi::c_void, bufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_GetSerialNumberString(hiddeviceobject : super::winnt::HANDLE, buffer : *mut core::ffi::c_void, bufferlength : u32) -> bool);
    unsafe { HidD_GetSerialNumberString(hiddeviceobject, buffer as _, bufferlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_SetConfiguration(hiddeviceobject: super::winnt::HANDLE, configuration: *const HIDD_CONFIGURATION, configurationlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_SetConfiguration(hiddeviceobject : super::winnt::HANDLE, configuration : *const HIDD_CONFIGURATION, configurationlength : u32) -> bool);
    unsafe { HidD_SetConfiguration(hiddeviceobject, configuration, configurationlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_SetFeature(hiddeviceobject: super::winnt::HANDLE, reportbuffer: *const core::ffi::c_void, reportbufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_SetFeature(hiddeviceobject : super::winnt::HANDLE, reportbuffer : *const core::ffi::c_void, reportbufferlength : u32) -> bool);
    unsafe { HidD_SetFeature(hiddeviceobject, reportbuffer, reportbufferlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_SetNumInputBuffers(hiddeviceobject: super::winnt::HANDLE, numberbuffers: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_SetNumInputBuffers(hiddeviceobject : super::winnt::HANDLE, numberbuffers : u32) -> bool);
    unsafe { HidD_SetNumInputBuffers(hiddeviceobject, numberbuffers) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HidD_SetOutputReport(hiddeviceobject: super::winnt::HANDLE, reportbuffer: *const core::ffi::c_void, reportbufferlength: u32) -> bool {
    windows_core::link!("hid.dll" "system" fn HidD_SetOutputReport(hiddeviceobject : super::winnt::HANDLE, reportbuffer : *const core::ffi::c_void, reportbufferlength : u32) -> bool);
    unsafe { HidD_SetOutputReport(hiddeviceobject, reportbuffer, reportbufferlength) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HIDD_ATTRIBUTES {
    pub Size: u32,
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
