#[cfg(all(feature = "hidclass", feature = "minwindef"))]
pub type EVT_VHF_ASYNC_OPERATION = Option<unsafe extern "system" fn(vhfclientcontext: *const core::ffi::c_void, vhfoperationhandle: VHFOPERATIONHANDLE, vhfoperationcontext: *const core::ffi::c_void, hidtransferpacket: *const super::hidclass::HID_XFER_PACKET)>;
pub type EVT_VHF_CLEANUP = Option<unsafe extern "system" fn(vhfclientcontext: *const core::ffi::c_void)>;
pub type EVT_VHF_READY_FOR_NEXT_READ_REPORT = Option<unsafe extern "system" fn(vhfclientcontext: *const core::ffi::c_void)>;
#[cfg(all(feature = "hidclass", feature = "minwindef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEVT_VHF_ASYNC_OPERATION(pub *mut EVT_VHF_ASYNC_OPERATION);
#[cfg(all(feature = "hidclass", feature = "minwindef"))]
impl PEVT_VHF_ASYNC_OPERATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "hidclass", feature = "minwindef"))]
impl Default for PEVT_VHF_ASYNC_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEVT_VHF_CLEANUP(pub *mut EVT_VHF_CLEANUP);
impl PEVT_VHF_CLEANUP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEVT_VHF_CLEANUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEVT_VHF_READY_FOR_NEXT_READ_REPORT(pub *mut EVT_VHF_READY_FOR_NEXT_READ_REPORT);
impl PEVT_VHF_READY_FOR_NEXT_READ_REPORT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEVT_VHF_READY_FOR_NEXT_READ_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "hidclass", feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PVHF_CONFIG(pub *mut VHF_CONFIG);
#[cfg(all(feature = "hidclass", feature = "minwindef", feature = "winnt"))]
impl PVHF_CONFIG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "hidclass", feature = "minwindef", feature = "winnt"))]
impl Default for PVHF_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VHFHANDLE(pub *mut core::ffi::c_void);
impl VHFHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for VHFHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VHFOPERATIONHANDLE(pub *mut core::ffi::c_void);
impl VHFOPERATIONHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for VHFOPERATIONHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "hidclass", feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VHF_CONFIG {
    pub Size: u32,
    pub VhfClientContext: *mut core::ffi::c_void,
    pub OperationContextSize: u32,
    pub FileHandle: super::winnt::HANDLE,
    pub VendorID: u16,
    pub ProductID: u16,
    pub VersionNumber: u16,
    pub ContainerID: windows_core::GUID,
    pub InstanceIDLength: u16,
    pub InstanceID: windows_core::PWSTR,
    pub ReportDescriptorLength: u16,
    pub ReportDescriptor: super::minwindef::PUCHAR,
    pub EvtVhfReadyForNextReadReport: PEVT_VHF_READY_FOR_NEXT_READ_REPORT,
    pub EvtVhfAsyncOperationGetFeature: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationSetFeature: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationWriteReport: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfAsyncOperationGetInputReport: PEVT_VHF_ASYNC_OPERATION,
    pub EvtVhfCleanup: PEVT_VHF_CLEANUP,
    pub HardwareIDsLength: u16,
    pub HardwareIDs: windows_core::PWSTR,
}
#[cfg(all(feature = "hidclass", feature = "minwindef", feature = "winnt"))]
impl Default for VHF_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
