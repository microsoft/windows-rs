#[inline]
pub unsafe fn SaferCloseLevel(hlevelhandle: SAFER_LEVEL_HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SaferCloseLevel(hlevelhandle : SAFER_LEVEL_HANDLE) -> windows_core::BOOL);
    unsafe { SaferCloseLevel(hlevelhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SaferComputeTokenFromLevel(levelhandle: SAFER_LEVEL_HANDLE, inaccesstoken: Option<super::winnt::HANDLE>, outaccesstoken: *mut super::winnt::HANDLE, dwflags: u32, lpreserved: Option<*mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SaferComputeTokenFromLevel(levelhandle : SAFER_LEVEL_HANDLE, inaccesstoken : super::winnt::HANDLE, outaccesstoken : *mut super::winnt::HANDLE, dwflags : u32, lpreserved : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SaferComputeTokenFromLevel(levelhandle, inaccesstoken.unwrap_or(core::mem::zeroed()) as _, outaccesstoken as _, dwflags, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SaferCreateLevel(dwscopeid: u32, dwlevelid: u32, openflags: u32, plevelhandle: *mut SAFER_LEVEL_HANDLE, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SaferCreateLevel(dwscopeid : u32, dwlevelid : u32, openflags : u32, plevelhandle : *mut SAFER_LEVEL_HANDLE, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SaferCreateLevel(dwscopeid, dwlevelid, openflags, plevelhandle as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SaferGetLevelInformation(levelhandle: SAFER_LEVEL_HANDLE, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: Option<*mut core::ffi::c_void>, dwinbuffersize: u32, lpdwoutbuffersize: *mut u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SaferGetLevelInformation(levelhandle : SAFER_LEVEL_HANDLE, dwinfotype : SAFER_OBJECT_INFO_CLASS, lpquerybuffer : *mut core::ffi::c_void, dwinbuffersize : u32, lpdwoutbuffersize : *mut u32) -> windows_core::BOOL);
    unsafe { SaferGetLevelInformation(levelhandle, dwinfotype, lpquerybuffer.unwrap_or(core::mem::zeroed()) as _, dwinbuffersize, lpdwoutbuffersize as _) }
}
#[inline]
pub unsafe fn SaferGetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *mut core::ffi::c_void, infobufferretsize: *mut u32, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SaferGetPolicyInformation(dwscopeid : u32, saferpolicyinfoclass : SAFER_POLICY_INFO_CLASS, infobuffersize : u32, infobuffer : *mut core::ffi::c_void, infobufferretsize : *mut u32, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SaferGetPolicyInformation(dwscopeid, saferpolicyinfoclass, infobuffersize, infobuffer as _, infobufferretsize as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn SaferIdentifyLevel(pcodeproperties: Option<&[SAFER_CODE_PROPERTIES_V2]>, plevelhandle: *mut SAFER_LEVEL_HANDLE, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SaferIdentifyLevel(dwnumproperties : u32, pcodeproperties : *const SAFER_CODE_PROPERTIES_V2, plevelhandle : *mut SAFER_LEVEL_HANDLE, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SaferIdentifyLevel(pcodeproperties.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pcodeproperties.map_or(core::ptr::null(), |slice| slice.as_ptr())), plevelhandle as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SaferRecordEventLogEntry<P1>(hlevel: SAFER_LEVEL_HANDLE, sztargetpath: P1, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn SaferRecordEventLogEntry(hlevel : SAFER_LEVEL_HANDLE, sztargetpath : windows_core::PCWSTR, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SaferRecordEventLogEntry(hlevel, sztargetpath.param().abi(), lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SaferSetLevelInformation(levelhandle: SAFER_LEVEL_HANDLE, dwinfotype: SAFER_OBJECT_INFO_CLASS, lpquerybuffer: *const core::ffi::c_void, dwinbuffersize: u32) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SaferSetLevelInformation(levelhandle : SAFER_LEVEL_HANDLE, dwinfotype : SAFER_OBJECT_INFO_CLASS, lpquerybuffer : *const core::ffi::c_void, dwinbuffersize : u32) -> windows_core::BOOL);
    unsafe { SaferSetLevelInformation(levelhandle, dwinfotype, lpquerybuffer, dwinbuffersize) }
}
#[inline]
pub unsafe fn SaferSetPolicyInformation(dwscopeid: u32, saferpolicyinfoclass: SAFER_POLICY_INFO_CLASS, infobuffersize: u32, infobuffer: *const core::ffi::c_void, lpreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn SaferSetPolicyInformation(dwscopeid : u32, saferpolicyinfoclass : SAFER_POLICY_INFO_CLASS, infobuffersize : u32, infobuffer : *const core::ffi::c_void, lpreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SaferSetPolicyInformation(dwscopeid, saferpolicyinfoclass, infobuffersize, infobuffer, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SaferiIsExecutableFileType<P0>(szfullpathname: P0, bfromshellexecute: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn SaferiIsExecutableFileType(szfullpathname : windows_core::PCWSTR, bfromshellexecute : bool) -> windows_core::BOOL);
    unsafe { SaferiIsExecutableFileType(szfullpathname.param().abi(), bfromshellexecute) }
}
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PSAFER_CODE_PROPERTIES = *mut SAFER_CODE_PROPERTIES_V2;
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PSAFER_CODE_PROPERTIES_V1 = *mut SAFER_CODE_PROPERTIES_V1;
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type PSAFER_CODE_PROPERTIES_V2 = *mut SAFER_CODE_PROPERTIES_V2;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PSAFER_HASH_IDENTIFICATION = *mut SAFER_HASH_IDENTIFICATION;
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
pub type PSAFER_HASH_IDENTIFICATION2 = *mut SAFER_HASH_IDENTIFICATION2;
#[cfg(feature = "minwindef")]
pub type PSAFER_IDENTIFICATION_HEADER = *mut SAFER_IDENTIFICATION_HEADER;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PSAFER_PATHNAME_IDENTIFICATION = *mut SAFER_PATHNAME_IDENTIFICATION;
#[cfg(feature = "minwindef")]
pub type PSAFER_URLZONE_IDENTIFICATION = *mut SAFER_URLZONE_IDENTIFICATION;
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef", feature = "winnt"))]
pub type SAFER_CODE_PROPERTIES = SAFER_CODE_PROPERTIES_V2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFER_CODE_PROPERTIES_V1 {
    pub cbSize: u32,
    pub dwCheckFlags: u32,
    pub ImagePath: windows_core::PCWSTR,
    pub hImageFileHandle: super::winnt::HANDLE,
    pub UrlZoneId: u32,
    pub ImageHash: [u8; 64],
    pub dwImageHashSize: u32,
    pub ImageSize: i64,
    pub HashAlgorithm: super::wincrypt::ALG_ID,
    pub pByteBlock: super::minwindef::LPBYTE,
    pub hWndParent: super::windef::HWND,
    pub dwWVTUIChoice: u32,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef", feature = "winnt"))]
impl Default for SAFER_CODE_PROPERTIES_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFER_CODE_PROPERTIES_V2 {
    pub cbSize: u32,
    pub dwCheckFlags: u32,
    pub ImagePath: windows_core::PCWSTR,
    pub hImageFileHandle: super::winnt::HANDLE,
    pub UrlZoneId: u32,
    pub ImageHash: [u8; 64],
    pub dwImageHashSize: u32,
    pub ImageSize: i64,
    pub HashAlgorithm: super::wincrypt::ALG_ID,
    pub pByteBlock: super::minwindef::LPBYTE,
    pub hWndParent: super::windef::HWND,
    pub dwWVTUIChoice: u32,
    pub PackageMoniker: windows_core::PCWSTR,
    pub PackagePublisher: windows_core::PCWSTR,
    pub PackageName: windows_core::PCWSTR,
    pub PackageVersion: u64,
    pub PackageIsFramework: windows_core::BOOL,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt", feature = "windef", feature = "winnt"))]
impl Default for SAFER_CODE_PROPERTIES_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SAFER_CRITERIA_APPX_PACKAGE: u32 = 32;
pub const SAFER_CRITERIA_AUTHENTICODE: u32 = 8;
pub const SAFER_CRITERIA_IMAGEHASH: u32 = 4;
pub const SAFER_CRITERIA_IMAGEPATH: u32 = 1;
pub const SAFER_CRITERIA_IMAGEPATH_NT: u32 = 4096;
pub const SAFER_CRITERIA_NOSIGNEDHASH: u32 = 2;
pub const SAFER_CRITERIA_URLZONE: u32 = 16;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFER_HASH_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub Description: [u16; 256],
    pub FriendlyName: [u16; 256],
    pub HashSize: u32,
    pub ImageHash: [u8; 64],
    pub HashAlgorithm: super::wincrypt::ALG_ID,
    pub ImageSize: i64,
    pub dwSaferFlags: u32,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for SAFER_HASH_IDENTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFER_HASH_IDENTIFICATION2 {
    pub hashIdentification: SAFER_HASH_IDENTIFICATION,
    pub HashSize: u32,
    pub ImageHash: [u8; 64],
    pub HashAlgorithm: super::wincrypt::ALG_ID,
}
#[cfg(all(feature = "minwindef", feature = "wincrypt"))]
impl Default for SAFER_HASH_IDENTIFICATION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SAFER_IDENTIFICATION_HEADER {
    pub dwIdentificationType: SAFER_IDENTIFICATION_TYPES,
    pub cbStructSize: u32,
    pub IdentificationGuid: windows_core::GUID,
    pub lastModified: super::minwindef::FILETIME,
}
pub type SAFER_IDENTIFICATION_TYPES = i32;
pub const SAFER_LEVELID_CONSTRAINED: u32 = 65536;
pub const SAFER_LEVELID_DISALLOWED: u32 = 0;
pub const SAFER_LEVELID_FULLYTRUSTED: u32 = 262144;
pub const SAFER_LEVELID_NORMALUSER: u32 = 131072;
pub const SAFER_LEVELID_UNTRUSTED: u32 = 4096;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SAFER_LEVEL_HANDLE(pub *mut core::ffi::c_void);
impl SAFER_LEVEL_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for SAFER_LEVEL_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SAFER_LEVEL_OPEN: u32 = 1;
pub const SAFER_MAX_DESCRIPTION_SIZE: u32 = 256;
pub const SAFER_MAX_FRIENDLYNAME_SIZE: u32 = 256;
pub const SAFER_MAX_HASH_SIZE: u32 = 64;
pub type SAFER_OBJECT_INFO_CLASS = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SAFER_PATHNAME_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub Description: [u16; 256],
    pub ImageName: super::winnt::PWCHAR,
    pub dwSaferFlags: u32,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for SAFER_PATHNAME_IDENTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SAFER_POLICY_BLOCK_CLIENT_UI: u32 = 8192;
pub const SAFER_POLICY_HASH_DUPLICATE: u32 = 262144;
pub type SAFER_POLICY_INFO_CLASS = i32;
pub const SAFER_POLICY_JOBID_CONSTRAINED: u32 = 67108864;
pub const SAFER_POLICY_JOBID_MASK: u32 = 4278190080;
pub const SAFER_POLICY_JOBID_UNTRUSTED: u32 = 50331648;
pub const SAFER_POLICY_ONLY_AUDIT: u32 = 4096;
pub const SAFER_POLICY_ONLY_EXES: u32 = 65536;
pub const SAFER_POLICY_SANDBOX_INERT: u32 = 131072;
pub const SAFER_POLICY_UIFLAGS_HIDDEN: u32 = 4;
pub const SAFER_POLICY_UIFLAGS_INFORMATION_PROMPT: u32 = 1;
pub const SAFER_POLICY_UIFLAGS_MASK: u32 = 255;
pub const SAFER_POLICY_UIFLAGS_OPTION_PROMPT: u32 = 2;
pub const SAFER_SCOPEID_MACHINE: u32 = 1;
pub const SAFER_SCOPEID_USER: u32 = 2;
pub const SAFER_TOKEN_COMPARE_ONLY: u32 = 2;
pub const SAFER_TOKEN_MAKE_INERT: u32 = 4;
pub const SAFER_TOKEN_NULL_IF_EQUAL: u32 = 1;
pub const SAFER_TOKEN_WANT_FLAGS: u32 = 8;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SAFER_URLZONE_IDENTIFICATION {
    pub header: SAFER_IDENTIFICATION_HEADER,
    pub UrlZoneId: u32,
    pub dwSaferFlags: u32,
}
pub const SRP_POLICY_APPX: windows_core::PCWSTR = windows_core::w!("APPX");
pub const SRP_POLICY_DLL: windows_core::PCWSTR = windows_core::w!("DLL");
pub const SRP_POLICY_EXE: windows_core::PCWSTR = windows_core::w!("EXE");
pub const SRP_POLICY_MANAGEDINSTALLER: windows_core::PCWSTR = windows_core::w!("MANAGEDINSTALLER");
pub const SRP_POLICY_MSI: windows_core::PCWSTR = windows_core::w!("MSI");
pub const SRP_POLICY_NOV2: windows_core::PCWSTR = windows_core::w!("IGNORESRPV2");
pub const SRP_POLICY_SCRIPT: windows_core::PCWSTR = windows_core::w!("SCRIPT");
pub const SRP_POLICY_SHELL: windows_core::PCWSTR = windows_core::w!("SHELL");
pub const SRP_POLICY_WLDPCONFIGCI: windows_core::PCWSTR = windows_core::w!("WLDPCONFIGCI");
pub const SRP_POLICY_WLDPMSI: windows_core::PCWSTR = windows_core::w!("WLDPMSI");
pub const SRP_POLICY_WLDPSCRIPT: windows_core::PCWSTR = windows_core::w!("WLDPSCRIPT");
pub const SaferIdentityDefault: SAFER_IDENTIFICATION_TYPES = 0;
pub const SaferIdentityTypeCertificate: SAFER_IDENTIFICATION_TYPES = 4;
pub const SaferIdentityTypeImageHash: SAFER_IDENTIFICATION_TYPES = 2;
pub const SaferIdentityTypeImageName: SAFER_IDENTIFICATION_TYPES = 1;
pub const SaferIdentityTypeUrlZone: SAFER_IDENTIFICATION_TYPES = 3;
pub const SaferObjectAllIdentificationGuids: SAFER_OBJECT_INFO_CLASS = 14;
pub const SaferObjectBuiltin: SAFER_OBJECT_INFO_CLASS = 5;
pub const SaferObjectDefaultOwner: SAFER_OBJECT_INFO_CLASS = 10;
pub const SaferObjectDeletedPrivileges: SAFER_OBJECT_INFO_CLASS = 9;
pub const SaferObjectDescription: SAFER_OBJECT_INFO_CLASS = 4;
pub const SaferObjectDisableMaxPrivilege: SAFER_OBJECT_INFO_CLASS = 7;
pub const SaferObjectDisallowed: SAFER_OBJECT_INFO_CLASS = 6;
pub const SaferObjectExtendedError: SAFER_OBJECT_INFO_CLASS = 16;
pub const SaferObjectFriendlyName: SAFER_OBJECT_INFO_CLASS = 3;
pub const SaferObjectInvertDeletedPrivileges: SAFER_OBJECT_INFO_CLASS = 8;
pub const SaferObjectLevelId: SAFER_OBJECT_INFO_CLASS = 1;
pub const SaferObjectRestrictedSidsAdded: SAFER_OBJECT_INFO_CLASS = 13;
pub const SaferObjectRestrictedSidsInverted: SAFER_OBJECT_INFO_CLASS = 12;
pub const SaferObjectScopeId: SAFER_OBJECT_INFO_CLASS = 2;
pub const SaferObjectSidsToDisable: SAFER_OBJECT_INFO_CLASS = 11;
pub const SaferObjectSingleIdentification: SAFER_OBJECT_INFO_CLASS = 15;
pub const SaferPolicyAuthenticodeEnabled: SAFER_POLICY_INFO_CLASS = 7;
pub const SaferPolicyDefaultLevel: SAFER_POLICY_INFO_CLASS = 3;
pub const SaferPolicyDefaultLevelFlags: SAFER_POLICY_INFO_CLASS = 6;
pub const SaferPolicyEnableTransparentEnforcement: SAFER_POLICY_INFO_CLASS = 2;
pub const SaferPolicyEvaluateUserScope: SAFER_POLICY_INFO_CLASS = 4;
pub const SaferPolicyLevelList: SAFER_POLICY_INFO_CLASS = 1;
pub const SaferPolicyScopeFlags: SAFER_POLICY_INFO_CLASS = 5;
