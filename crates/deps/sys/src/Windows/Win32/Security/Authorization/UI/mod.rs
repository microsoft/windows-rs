#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const DOBJ_COND_NTACLS: i32 = 8i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const DOBJ_RES_CONT: i32 = 1i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const DOBJ_RES_ROOT: i32 = 2i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const DOBJ_RIBBON_LAUNCH: i32 = 16i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const DOBJ_VOL_NTACLS: i32 = 4i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SECURITY_OBJECT_ID_CENTRAL_ACCESS_RULE: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SECURITY_OBJECT_ID_CENTRAL_POLICY: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SECURITY_OBJECT_ID_OBJECT_SD: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SECURITY_OBJECT_ID_SHARE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_ACCESS_CONTAINER: i32 = 262144i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_ACCESS_GENERAL: i32 = 131072i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_ACCESS_PROPERTY: i32 = 524288i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_ACCESS_SPECIFIC: i32 = 65536i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_CONTAINER: i32 = 4i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_EDIT_OWNER: i32 = 1i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_EDIT_PERMS: i32 = 0i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_NO_ACL_PROTECT: i32 = 512i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_NO_TREE_APPLY: i32 = 1024i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_OBJECT_GUID: i32 = 65536i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_OWNER_READONLY: i32 = 64i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_OWNER_RECURSE: i32 = 256i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_PAGE_TITLE: i32 = 2048i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_READONLY: i32 = 8i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_RESET: i32 = 32i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_RESET_DACL_TREE: i32 = 16384i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_RESET_SACL_TREE: i32 = 32768i32;
#[doc = "*Required features: `Win32_Security_Authorization_UI`*"]
pub const SI_SERVER_IS_DC: i32 = 4096i32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_UI_Controls`*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub fn CreateSecurityPage(psi: ISecurityInformation) -> super::super::super::UI::Controls::HPROPSHEETPAGE;
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditSecurity(hwndowner: super::super::super::Foundation::HWND, psi: ISecurityInformation) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_Security_Authorization_UI`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EditSecurityAdvanced(hwndowner: super::super::super::Foundation::HWND, psi: ISecurityInformation, usipage: SI_PAGE_TYPE) -> ::windows_sys::core::HRESULT;
}
