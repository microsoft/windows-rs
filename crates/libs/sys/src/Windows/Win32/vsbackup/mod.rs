windows_link::link!("vssapi.dll" "system" fn CreateVssBackupComponentsInternal(ppbackup : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("vssapi.dll" "system" fn CreateVssExamineWriterMetadataInternal(bstrxml : windows_sys::core::BSTR, ppmetadata : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "vss"))]
windows_link::link!("vssapi.dll" "system" fn GetProviderMgmtInterfaceInternal(providerid : super::VSS_ID, interfaceid : super::IID, ppitf : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "vss"))]
windows_link::link!("vssapi.dll" "system" fn IsVolumeSnapshottedInternal(pwszvolumename : super::VSS_PWSZ, pbsnapshotspresent : *mut windows_sys::core::BOOL, plsnapshotcapability : *mut super::LONG) -> windows_sys::core::HRESULT);
windows_link::link!("vssapi.dll" "system" fn ShouldBlockRevertInternal(wszvolumename : windows_sys::core::PCWSTR, pbblock : *mut bool) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "vss"))]
windows_link::link!("vssapi.dll" "system" fn VssFreeSnapshotPropertiesInternal(pprop : *const super::VSS_SNAPSHOT_PROP));
pub const IID_IVssBackupComponents: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x665c1d5f_c218_414d_a05d_7fef5f9d5c86);
pub const IID_IVssBackupComponentsEx: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x963f03ad_9e4c_4a34_ac15_e4b6174e5036);
pub const IID_IVssBackupComponentsEx2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xacfe2b3a_22c9_4ef8_bd03_2f9ca230084e);
pub const IID_IVssBackupComponentsEx3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc191bfbc_b602_4675_8bd1_67d642f529d5);
pub const IID_IVssBackupComponentsEx4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf434c2fd_b553_4961_a9f9_a8e90b673e53);
pub const IID_IVssExamineWriterMetadata: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x902fcf7f_b7fd_42f8_81f1_b2e400b1e5bd);
pub const IID_IVssExamineWriterMetadataEx: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0c0e5ec0_ca44_472b_b702_e652db1c0451);
pub const IID_IVssExamineWriterMetadataEx2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xce115780_a611_431b_b57f_c38303ab6aee);
#[cfg(feature = "vswriter")]
#[repr(C)]
pub struct IVssWriterComponentsExt {
    base0: *const core::ffi::c_void,
    base1: *const core::ffi::c_void,
}
#[cfg(feature = "vswriter")]
impl IVssWriterComponentsExt {
    pub fn base0(&self) -> *mut core::ffi::c_void {
        core::ptr::addr_of!(self.base0).cast_mut().cast()
    }
    pub fn base1(&self) -> *mut core::ffi::c_void {
        core::ptr::addr_of!(self.base1).cast_mut().cast()
    }
}
#[cfg(all(feature = "shared", feature = "vswriter"))]
pub type PVSSCOMPONENTINFO = *const VSS_COMPONENTINFO;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "vswriter"))]
#[derive(Clone, Copy, Default)]
pub struct VSS_COMPONENTINFO {
    pub r#type: super::VSS_COMPONENT_TYPE,
    pub bstrLogicalPath: windows_sys::core::BSTR,
    pub bstrComponentName: windows_sys::core::BSTR,
    pub bstrCaption: windows_sys::core::BSTR,
    pub pbIcon: *mut super::BYTE,
    pub cbIcon: super::UINT,
    pub bRestoreMetadata: bool,
    pub bNotifyOnBackupComplete: bool,
    pub bSelectable: bool,
    pub bSelectableForRestore: bool,
    pub dwComponentFlags: super::DWORD,
    pub cFileCount: super::UINT,
    pub cDatabases: super::UINT,
    pub cLogFiles: super::UINT,
    pub cDependencies: super::UINT,
}
pub const VSS_SW_BOOTABLE_STATE: i32 = 1;
