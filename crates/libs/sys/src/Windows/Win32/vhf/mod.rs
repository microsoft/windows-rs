#[cfg(all(feature = "hidclass", feature = "minwindef"))]
pub type EVT_VHF_ASYNC_OPERATION = Option<unsafe extern "system" fn(vhfclientcontext: *const core::ffi::c_void, vhfoperationhandle: VHFOPERATIONHANDLE, vhfoperationcontext: *const core::ffi::c_void, hidtransferpacket: *const super::hidclass::HID_XFER_PACKET)>;
pub type EVT_VHF_CLEANUP = Option<unsafe extern "system" fn(vhfclientcontext: *const core::ffi::c_void)>;
pub type EVT_VHF_READY_FOR_NEXT_READ_REPORT = Option<unsafe extern "system" fn(vhfclientcontext: *const core::ffi::c_void)>;
#[cfg(all(feature = "hidclass", feature = "minwindef"))]
pub type PEVT_VHF_ASYNC_OPERATION = *mut EVT_VHF_ASYNC_OPERATION;
pub type PEVT_VHF_CLEANUP = *mut EVT_VHF_CLEANUP;
pub type PEVT_VHF_READY_FOR_NEXT_READ_REPORT = *mut EVT_VHF_READY_FOR_NEXT_READ_REPORT;
#[cfg(all(feature = "hidclass", feature = "minwindef", feature = "winnt"))]
pub type PVHF_CONFIG = *mut VHF_CONFIG;
pub type VHFHANDLE = *mut core::ffi::c_void;
pub type VHFOPERATIONHANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(all(feature = "hidclass", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct VHF_CONFIG {
    pub Size: u32,
    pub VhfClientContext: *mut core::ffi::c_void,
    pub OperationContextSize: u32,
    pub FileHandle: super::winnt::HANDLE,
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
    pub ContainerID: windows_sys::core::GUID,
    pub InstanceIDLength: u16,
    pub InstanceID: windows_sys::core::PWSTR,
    pub ReportDescriptorLength: u16,
    pub ReportDescriptor: super::minwindef::PUCHAR,
    pub EvtVhfReadyForNextReadReport: PEVT_VHF_READY_FOR_NEXT_READ_REPORT,
    pub EvtVhfAsyncOperationGetFeature: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationSetFeature: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationWriteReport: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationGetInputReport: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfCleanup: PEVT_VHF_CLEANUP,
    pub HardwareIDsLength: u16,
    pub HardwareIDs: windows_sys::core::PWSTR,
}
#[cfg(all(feature = "hidclass", feature = "minwindef", feature = "winnt"))]
impl Default for VHF_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
