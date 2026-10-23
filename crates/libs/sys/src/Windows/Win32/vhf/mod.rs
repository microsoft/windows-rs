#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type EVT_VHF_ASYNC_OPERATION = Option<unsafe extern "C" fn(vhfclientcontext: super::PVOID, vhfoperationhandle: VHFOPERATIONHANDLE, vhfoperationcontext: super::PVOID, hidtransferpacket: super::PHID_XFER_PACKET)>;
#[cfg(feature = "winnt")]
pub type EVT_VHF_CLEANUP = Option<unsafe extern "C" fn(vhfclientcontext: super::PVOID)>;
#[cfg(feature = "winnt")]
pub type EVT_VHF_READY_FOR_NEXT_READ_REPORT = Option<unsafe extern "C" fn(vhfclientcontext: super::PVOID)>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PEVT_VHF_ASYNC_OPERATION = Option<unsafe extern "C" fn(vhfclientcontext: super::PVOID, vhfoperationhandle: VHFOPERATIONHANDLE, vhfoperationcontext: super::PVOID, hidtransferpacket: super::PHID_XFER_PACKET)>;
#[cfg(feature = "winnt")]
pub type PEVT_VHF_CLEANUP = Option<unsafe extern "C" fn(vhfclientcontext: super::PVOID)>;
#[cfg(feature = "winnt")]
pub type PEVT_VHF_READY_FOR_NEXT_READ_REPORT = Option<unsafe extern "C" fn(vhfclientcontext: super::PVOID)>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PVHF_CONFIG = *mut VHF_CONFIG;
pub type VHFHANDLE = *mut core::ffi::c_void;
pub type VHFOPERATIONHANDLE = *mut core::ffi::c_void;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct VHF_CONFIG {
    pub Size: super::ULONG,
    pub VhfClientContext: super::PVOID,
    pub OperationContextSize: super::ULONG,
    pub FileHandle: super::HANDLE,
    pub VendorID: super::USHORT,
    pub ProductID: super::USHORT,
    pub VersionNumber: super::USHORT,
    pub ContainerID: windows_sys::core::GUID,
    pub InstanceIDLength: super::USHORT,
    pub InstanceID: windows_sys::core::PWSTR,
    pub ReportDescriptorLength: super::USHORT,
    pub ReportDescriptor: super::PUCHAR,
    pub EvtVhfReadyForNextReadReport: PEVT_VHF_READY_FOR_NEXT_READ_REPORT,
    pub EvtVhfAsyncOperationGetFeature: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationSetFeature: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationWriteReport: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationGetInputReport: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfCleanup: PEVT_VHF_CLEANUP,
    pub HardwareIDsLength: super::USHORT,
    pub HardwareIDs: windows_sys::core::PWSTR,
}
