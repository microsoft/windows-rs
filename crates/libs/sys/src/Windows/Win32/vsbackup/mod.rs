windows_link::link!("vssapi.dll" "system" fn CreateVssBackupComponentsInternal(ppbackup : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("vssapi.dll" "system" fn CreateVssExamineWriterMetadataInternal(bstrxml : windows_sys::core::BSTR, ppmetadata : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "vss")]
windows_link::link!("vssapi.dll" "system" fn GetProviderMgmtInterfaceInternal(providerid : super::vss::VSS_ID, interfaceid : windows_sys::core::GUID, ppitf : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("vssapi.dll" "system" fn IsVolumeSnapshottedInternal(pwszvolumename : windows_sys::core::PCWSTR, pbsnapshotspresent : *mut windows_sys::core::BOOL, plsnapshotcapability : *mut i32) -> windows_sys::core::HRESULT);
windows_link::link!("vssapi.dll" "system" fn ShouldBlockRevertInternal(wszvolumename : windows_sys::core::PCWSTR, pbblock : *mut bool) -> windows_sys::core::HRESULT);
#[cfg(feature = "vss")]
windows_link::link!("vssapi.dll" "system" fn VssFreeSnapshotPropertiesInternal(pprop : *const super::vss::VSS_SNAPSHOT_PROP));
#[cfg(feature = "vswriter")]
pub type PVSSCOMPONENTINFO = *const VSS_COMPONENTINFO;
#[repr(C)]
#[cfg(feature = "vswriter")]
#[derive(Clone, Copy)]
pub struct VSS_COMPONENTINFO {
    pub r#type: super::vswriter::VSS_COMPONENT_TYPE,
    pub bstrLogicalPath: windows_sys::core::BSTR,
    pub bstrComponentName: windows_sys::core::BSTR,
    pub bstrCaption: windows_sys::core::BSTR,
    pub pbIcon: *mut u8,
    pub cbIcon: u32,
    pub bRestoreMetadata: bool,
    pub bNotifyOnBackupComplete: bool,
    pub bSelectable: bool,
    pub bSelectableForRestore: bool,
    pub dwComponentFlags: u32,
    pub cFileCount: u32,
    pub cDatabases: u32,
    pub cLogFiles: u32,
    pub cDependencies: u32,
}
#[cfg(feature = "vswriter")]
impl Default for VSS_COMPONENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VSS_SW_BOOTABLE_STATE: u32 = 1;
