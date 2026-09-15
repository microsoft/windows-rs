#[cfg(feature = "minwindef")]
pub type EVT_VHF_ASYNC_OPERATION = Option<unsafe extern "C" fn(vhfclientcontext: *const core::ffi::c_void, vhfoperationhandle: VHFOPERATIONHANDLE, vhfoperationcontext: *const core::ffi::c_void, hidtransferpacket: PHID_XFER_PACKET)>;
pub type EVT_VHF_CLEANUP = Option<unsafe extern "C" fn(vhfclientcontext: *const core::ffi::c_void)>;
pub type EVT_VHF_READY_FOR_NEXT_READ_REPORT = Option<unsafe extern "C" fn(vhfclientcontext: *const core::ffi::c_void)>;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HID_XFER_PACKET {
    pub reportBuffer: super::PUCHAR,
    pub reportBufferLen: u32,
    pub reportId: u8,
}
pub type PEVT_VHF_ASYNC_OPERATION = *mut u8;
pub type PEVT_VHF_CLEANUP = *mut u8;
pub type PEVT_VHF_READY_FOR_NEXT_READ_REPORT = *mut u8;
#[cfg(feature = "minwindef")]
pub type PHID_XFER_PACKET = *mut HID_XFER_PACKET;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PVHF_CONFIG = *mut VHF_CONFIG;
pub type VHFHANDLE = *mut core::ffi::c_void;
pub type VHFOPERATIONHANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VHF_CONFIG {
    pub Size: u32,
    pub VhfClientContext: *mut core::ffi::c_void,
    pub OperationContextSize: u32,
    pub FileHandle: super::HANDLE,
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
    pub ContainerID: windows_core::GUID,
    pub InstanceIDLength: u16,
    pub InstanceID: windows_core::PWSTR,
    pub ReportDescriptorLength: u16,
    pub ReportDescriptor: super::PUCHAR,
    pub EvtVhfReadyForNextReadReport: PEVT_VHF_READY_FOR_NEXT_READ_REPORT,
    pub EvtVhfAsyncOperationGetFeature: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationSetFeature: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationWriteReport: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationGetInputReport: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfCleanup: PEVT_VHF_CLEANUP,
    pub HardwareIDsLength: u16,
    pub HardwareIDs: windows_core::PWSTR,
}
