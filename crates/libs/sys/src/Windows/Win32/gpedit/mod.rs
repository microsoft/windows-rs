#[cfg(feature = "windef")]
windows_link::link!("gpedit.dll" "system" fn BrowseForGPO(lpbrowseinfo : *mut GPOBROWSEINFO) -> windows_sys::core::HRESULT);
windows_link::link!("gpedit.dll" "system" fn CreateGPOLink(lpgpo : windows_sys::core::PCWSTR, lpcontainer : windows_sys::core::PCWSTR, fhighpriority : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("gpedit.dll" "system" fn DeleteAllGPOLinks(lpcontainer : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("gpedit.dll" "system" fn DeleteGPOLink(lpgpo : windows_sys::core::PCWSTR, lpcontainer : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("gpedit.dll" "system" fn ExportRSoPData(lpnamespace : windows_sys::core::PCWSTR, lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("gpedit.dll" "system" fn ImportRSoPData(lpnamespace : windows_sys::core::PCWSTR, lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
pub const CLSID_GPESnapIn: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8fc0b734_a0e1_11d1_a7d3_0000f87571e3);
pub const CLSID_GroupPolicyObject: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xea502722_a23d_11d1_a7d3_0000f87571e3);
pub const CLSID_RSOPSnapIn: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6dc3804b_7212_458d_adb0_9a07e2ae1fa2);
pub const GPHintDomain: GROUP_POLICY_HINT_TYPE = 3;
pub const GPHintMachine: GROUP_POLICY_HINT_TYPE = 1;
pub const GPHintOrganizationalUnit: GROUP_POLICY_HINT_TYPE = 4;
pub const GPHintSite: GROUP_POLICY_HINT_TYPE = 2;
pub const GPHintUnknown: GROUP_POLICY_HINT_TYPE = 0;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct GPOBROWSEINFO {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub hwndOwner: super::windef::HWND,
    pub lpTitle: windows_sys::core::PWSTR,
    pub lpInitialOU: windows_sys::core::PWSTR,
    pub lpDSPath: windows_sys::core::PWSTR,
    pub dwDSPathSize: u32,
    pub lpName: windows_sys::core::PWSTR,
    pub dwNameSize: u32,
    pub gpoType: GROUP_POLICY_OBJECT_TYPE,
    pub gpoHint: GROUP_POLICY_HINT_TYPE,
}
#[cfg(feature = "windef")]
impl Default for GPOBROWSEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GPOTypeDS: GROUP_POLICY_OBJECT_TYPE = 2;
pub const GPOTypeLocal: GROUP_POLICY_OBJECT_TYPE = 0;
pub const GPOTypeLocalGroup: GROUP_POLICY_OBJECT_TYPE = 4;
pub const GPOTypeLocalUser: GROUP_POLICY_OBJECT_TYPE = 3;
pub const GPOTypeRemote: GROUP_POLICY_OBJECT_TYPE = 1;
pub const GPO_BROWSE_DISABLENEW: u32 = 1;
pub const GPO_BROWSE_INITTOALL: u32 = 16;
pub const GPO_BROWSE_NOCOMPUTERS: u32 = 2;
pub const GPO_BROWSE_NODSGPOS: u32 = 4;
pub const GPO_BROWSE_NOUSERGPOS: u32 = 32;
pub const GPO_BROWSE_OPENBUTTON: u32 = 8;
pub const GPO_BROWSE_SENDAPPLYONEDIT: u32 = 64;
pub const GPO_OPEN_LOAD_REGISTRY: u32 = 1;
pub const GPO_OPEN_READ_ONLY: u32 = 2;
pub const GPO_OPTION_DISABLE_MACHINE: u32 = 2;
pub const GPO_OPTION_DISABLE_USER: u32 = 1;
pub const GPO_SECTION_MACHINE: u32 = 2;
pub const GPO_SECTION_ROOT: u32 = 0;
pub const GPO_SECTION_USER: u32 = 1;
pub type GROUP_POLICY_HINT_TYPE = i32;
pub type GROUP_POLICY_OBJECT_TYPE = i32;
#[cfg(feature = "windef")]
pub type LPGPOBROWSEINFO = *mut GPOBROWSEINFO;
pub const NODEID_Machine: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8fc0b737_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_MachineSWSettings: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8fc0b73a_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_RSOPMachine: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbd4c1a2e_0b7a_4a62_a6b0_c0577539c97e);
pub const NODEID_RSOPMachineSWSettings: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6a76273e_eb8e_45db_94c5_25663a5f2c1a);
pub const NODEID_RSOPUser: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xab87364f_0cec_4cd8_9bf8_898f34628fb8);
pub const NODEID_RSOPUserSWSettings: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe52c5ce3_fd27_4402_84de_d9a5f2858910);
pub const NODEID_User: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8fc0b738_a0e1_11d1_a7d3_0000f87571e3);
pub const NODEID_UserSWSettings: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8fc0b73c_a0e1_11d1_a7d3_0000f87571e3);
pub type PGROUP_POLICY_HINT_TYPE = *mut GROUP_POLICY_HINT_TYPE;
pub type PGROUP_POLICY_OBJECT_TYPE = *mut GROUP_POLICY_OBJECT_TYPE;
pub const RSOP_INFO_FLAG_DIAGNOSTIC_MODE: u32 = 1;
