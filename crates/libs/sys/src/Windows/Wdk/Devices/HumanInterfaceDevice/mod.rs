#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("vhfum.dll" "system" #[doc = "Required features: `\"Win32_Foundation\"`"] fn VhfAsyncOperationComplete(vhfoperationhandle : *const ::core::ffi::c_void, completionstatus : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("vhfum.dll" "system" #[doc = "Required features: `\"Win32_Foundation\"`"] fn VhfCreate(vhfconfig : *const VHF_CONFIG, vhfhandle : *mut *mut ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("vhfum.dll" "system" #[doc = "Required features: `\"Win32_Foundation\"`"] fn VhfDelete(vhfhandle : *const ::core::ffi::c_void, wait : super::super::super::Win32::Foundation:: BOOLEAN));
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("vhfum.dll" "system" #[doc = "Required features: `\"Win32_Foundation\"`"] fn VhfReadReportSubmit(vhfhandle : *const ::core::ffi::c_void, hidtransferpacket : *const HID_XFER_PACKET) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Foundation")]
::windows_targets::link!("vhfum.dll" "system" #[doc = "Required features: `\"Win32_Foundation\"`"] fn VhfStart(vhfhandle : *const ::core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[repr(C)]
pub struct HID_XFER_PACKET {
    pub reportBuffer: *mut u8,
    pub reportBufferLen: u32,
    pub reportId: u8,
}
impl ::core::marker::Copy for HID_XFER_PACKET {}
impl ::core::clone::Clone for HID_XFER_PACKET {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub struct VHF_CONFIG {
    pub Size: u32,
    pub VhfClientContext: *mut ::core::ffi::c_void,
    pub OperationContextSize: u32,
    pub FileHandle: super::super::super::Win32::Foundation::HANDLE,
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
    pub ContainerID: ::windows_sys::core::GUID,
    pub InstanceIDLength: u16,
    pub InstanceID: ::windows_sys::core::PWSTR,
    pub ReportDescriptorLength: u16,
    pub ReportDescriptor: *mut u8,
    pub EvtVhfReadyForNextReadReport: PEVT_VHF_READY_FOR_NEXT_READ_REPORT,
    pub EvtVhfAsyncOperationGetFeature: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationSetFeature: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationWriteReport: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationGetInputReport: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfCleanup: PEVT_VHF_CLEANUP,
    pub HardwareIDsLength: u16,
    pub HardwareIDs: ::windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VHF_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VHF_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
pub type EVT_VHF_ASYNC_OPERATION = ::core::option::Option<unsafe extern "system" fn(vhfclientcontext: *const ::core::ffi::c_void, vhfoperationhandle: *const ::core::ffi::c_void, vhfoperationcontext: *const ::core::ffi::c_void, hidtransferpacket: *const HID_XFER_PACKET)>;
pub type EVT_VHF_CLEANUP = ::core::option::Option<unsafe extern "system" fn(vhfclientcontext: *const ::core::ffi::c_void)>;
pub type EVT_VHF_READY_FOR_NEXT_READ_REPORT = ::core::option::Option<unsafe extern "system" fn(vhfclientcontext: *const ::core::ffi::c_void)>;
pub type PEVT_VHF_ASYNC_OPERATION = ::core::option::Option<unsafe extern "system" fn()>;
pub type PEVT_VHF_CLEANUP = ::core::option::Option<unsafe extern "system" fn()>;
pub type PEVT_VHF_READY_FOR_NEXT_READ_REPORT = ::core::option::Option<unsafe extern "system" fn()>;
