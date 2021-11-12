#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_UI_Controls")]
    pub fn CreateSecurityPage(psi: ISecurityInformation) -> super::super::super::UI::Controls::HPROPSHEETPAGE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditSecurity(hwndowner: super::super::super::Foundation::HWND, psi: ISecurityInformation) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditSecurityAdvanced(hwndowner: super::super::super::Foundation::HWND, psi: ISecurityInformation, usipage: SI_PAGE_TYPE) -> ::windows_sys::core::HRESULT;
}
pub const DOBJ_COND_NTACLS: i32 = 8i32;
pub const DOBJ_RES_CONT: i32 = 1i32;
pub const DOBJ_RES_ROOT: i32 = 2i32;
pub const DOBJ_RIBBON_LAUNCH: i32 = 16i32;
pub const DOBJ_VOL_NTACLS: i32 = 4i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct EFFPERM_RESULT_LIST(i32);
#[repr(transparent)]
pub struct IEffectivePermission(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEffectivePermission2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecurityInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecurityInformation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecurityInformation3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecurityInformation4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISecurityObjectTypeInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SECURITY_INFO_PAGE_FLAGS(pub u32);
pub const SI_ADVANCED: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(16u32);
pub const SI_EDIT_AUDITS: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(2u32);
pub const SI_EDIT_PROPERTIES: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(128u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SECURITY_OBJECT(i32);
pub const SECURITY_OBJECT_ID_CENTRAL_ACCESS_RULE: u32 = 4u32;
pub const SECURITY_OBJECT_ID_CENTRAL_POLICY: u32 = 3u32;
pub const SECURITY_OBJECT_ID_OBJECT_SD: u32 = 1u32;
pub const SECURITY_OBJECT_ID_SHARE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SID_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SID_INFO_LIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SI_ACCESS(i32);
pub const SI_ACCESS_CONTAINER: i32 = 262144i32;
pub const SI_ACCESS_GENERAL: i32 = 131072i32;
pub const SI_ACCESS_PROPERTY: i32 = 524288i32;
pub const SI_ACCESS_SPECIFIC: i32 = 65536i32;
pub const SI_CONTAINER: i32 = 4i32;
pub const SI_EDIT_OWNER: i32 = 1i32;
pub const SI_EDIT_PERMS: i32 = 0i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SI_INHERIT_TYPE(i32);
pub const SI_NO_ACL_PROTECT: i32 = 512i32;
pub const SI_NO_TREE_APPLY: i32 = 1024i32;
pub const SI_OBJECT_GUID: i32 = 65536i32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SI_OBJECT_INFO(i32);
#[repr(transparent)]
pub struct SI_OBJECT_INFO_FLAGS(pub u32);
pub const SI_AUDITS_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(33554432u32);
pub const SI_DISABLE_DENY_ACE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(2147483648u32);
pub const SI_EDIT_EFFECTIVE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(131072u32);
pub const SI_ENABLE_CENTRAL_POLICY: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(1073741824u32);
pub const SI_ENABLE_EDIT_ATTRIBUTE_CONDITION: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(536870912u32);
pub const SI_MAY_WRITE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(268435456u32);
pub const SI_NO_ADDITIONAL_PERMISSION: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(2097152u32);
pub const SI_OWNER_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(67108864u32);
pub const SI_PERMS_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(16777216u32);
pub const SI_RESET_DACL: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(262144u32);
pub const SI_RESET_OWNER: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(1048576u32);
pub const SI_RESET_SACL: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(524288u32);
pub const SI_SCOPE_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(134217728u32);
pub const SI_VIEW_ONLY: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(4194304u32);
pub const SI_OWNER_READONLY: i32 = 64i32;
pub const SI_OWNER_RECURSE: i32 = 256i32;
#[repr(transparent)]
pub struct SI_PAGE_ACTIVATED(pub i32);
pub const SI_SHOW_DEFAULT: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(0i32);
pub const SI_SHOW_PERM_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(1i32);
pub const SI_SHOW_AUDIT_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(2i32);
pub const SI_SHOW_OWNER_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(3i32);
pub const SI_SHOW_EFFECTIVE_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(4i32);
pub const SI_SHOW_SHARE_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(5i32);
pub const SI_SHOW_CENTRAL_POLICY_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(6i32);
pub const SI_PAGE_TITLE: i32 = 2048i32;
#[repr(transparent)]
pub struct SI_PAGE_TYPE(pub i32);
pub const SI_PAGE_PERM: SI_PAGE_TYPE = SI_PAGE_TYPE(0i32);
pub const SI_PAGE_ADVPERM: SI_PAGE_TYPE = SI_PAGE_TYPE(1i32);
pub const SI_PAGE_AUDIT: SI_PAGE_TYPE = SI_PAGE_TYPE(2i32);
pub const SI_PAGE_OWNER: SI_PAGE_TYPE = SI_PAGE_TYPE(3i32);
pub const SI_PAGE_EFFECTIVE: SI_PAGE_TYPE = SI_PAGE_TYPE(4i32);
pub const SI_PAGE_TAKEOWNERSHIP: SI_PAGE_TYPE = SI_PAGE_TYPE(5i32);
pub const SI_PAGE_SHARE: SI_PAGE_TYPE = SI_PAGE_TYPE(6i32);
pub const SI_READONLY: i32 = 8i32;
pub const SI_RESET: i32 = 32i32;
pub const SI_RESET_DACL_TREE: i32 = 16384i32;
pub const SI_RESET_SACL_TREE: i32 = 32768i32;
pub const SI_SERVER_IS_DC: i32 = 4096i32;
