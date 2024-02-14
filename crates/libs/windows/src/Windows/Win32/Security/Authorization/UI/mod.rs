#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn CreateSecurityPage<P0>(psi: P0) -> ::windows_core::Result<super::super::super::UI::Controls::HPROPSHEETPAGE>
where
    P0: ::windows_core::IntoParam<ISecurityInformation>,
{
    ::windows_targets::link!("aclui.dll" "system" fn CreateSecurityPage(psi : * mut::core::ffi::c_void) -> super::super::super::UI::Controls:: HPROPSHEETPAGE);
    let result__ = CreateSecurityPage(psi.into_param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn EditSecurity<P0, P1>(hwndowner: P0, psi: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    P1: ::windows_core::IntoParam<ISecurityInformation>,
{
    ::windows_targets::link!("aclui.dll" "system" fn EditSecurity(hwndowner : super::super::super::Foundation:: HWND, psi : * mut::core::ffi::c_void) -> super::super::super::Foundation:: BOOL);
    EditSecurity(hwndowner.into_param().abi(), psi.into_param().abi()).ok()
}
#[inline]
pub unsafe fn EditSecurityAdvanced<P0, P1>(hwndowner: P0, psi: P1, usipage: SI_PAGE_TYPE) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    P1: ::windows_core::IntoParam<ISecurityInformation>,
{
    ::windows_targets::link!("aclui.dll" "system" fn EditSecurityAdvanced(hwndowner : super::super::super::Foundation:: HWND, psi : * mut::core::ffi::c_void, usipage : SI_PAGE_TYPE) -> ::windows_core::HRESULT);
    EditSecurityAdvanced(hwndowner.into_param().abi(), psi.into_param().abi(), usipage).ok()
}
::windows_core::imp::com_interface!(IEffectivePermission, IEffectivePermission_Vtbl, 0x3853dc76_9f35_407c_88a1_d19344365fbc);
::windows_core::imp::interface_hierarchy!(IEffectivePermission, ::windows_core::IUnknown);
impl IEffectivePermission {
    pub unsafe fn GetEffectivePermission<P0, P1, P2>(&self, pguidobjecttype: *const ::windows_core::GUID, pusersid: P0, pszservername: P1, psd: P2, ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST, pcobjecttypelistlength: *mut u32, ppgrantedaccesslist: *mut *mut u32, pcgrantedaccesslistlength: *mut u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::PSID>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::PSECURITY_DESCRIPTOR>,
    {
        (::windows_core::Interface::vtable(self).GetEffectivePermission)(::windows_core::Interface::as_raw(self), pguidobjecttype, pusersid.into_param().abi(), pszservername.into_param().abi(), psd.into_param().abi(), ppobjecttypelist, pcobjecttypelistlength, ppgrantedaccesslist, pcgrantedaccesslistlength).ok()
    }
}
#[repr(C)]
pub struct IEffectivePermission_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetEffectivePermission: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID, super::super::super::Foundation::PSID, ::windows_core::PCWSTR, super::super::PSECURITY_DESCRIPTOR, *mut *mut super::super::OBJECT_TYPE_LIST, *mut u32, *mut *mut u32, *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IEffectivePermission2, IEffectivePermission2_Vtbl, 0x941fabca_dd47_4fca_90bb_b0e10255f20d);
::windows_core::imp::interface_hierarchy!(IEffectivePermission2, ::windows_core::IUnknown);
impl IEffectivePermission2 {
    pub unsafe fn ComputeEffectivePermissionWithSecondarySecurity<P0, P1, P2>(
        &self,
        psid: P0,
        pdevicesid: P1,
        pszservername: P2,
        psecurityobjects: *mut SECURITY_OBJECT,
        dwsecurityobjectcount: u32,
        pusergroups: ::core::option::Option<*const super::super::TOKEN_GROUPS>,
        pauthzusergroupsoperations: ::core::option::Option<*const super::AUTHZ_SID_OPERATION>,
        pdevicegroups: ::core::option::Option<*const super::super::TOKEN_GROUPS>,
        pauthzdevicegroupsoperations: ::core::option::Option<*const super::AUTHZ_SID_OPERATION>,
        pauthzuserclaims: ::core::option::Option<*const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION>,
        pauthzuserclaimsoperations: ::core::option::Option<*const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION>,
        pauthzdeviceclaims: ::core::option::Option<*const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION>,
        pauthzdeviceclaimsoperations: ::core::option::Option<*const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION>,
        peffpermresultlists: *mut EFFPERM_RESULT_LIST,
    ) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::PSID>,
        P1: ::windows_core::IntoParam<super::super::super::Foundation::PSID>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).ComputeEffectivePermissionWithSecondarySecurity)(
            ::windows_core::Interface::as_raw(self),
            psid.into_param().abi(),
            pdevicesid.into_param().abi(),
            pszservername.into_param().abi(),
            psecurityobjects,
            dwsecurityobjectcount,
            ::core::mem::transmute(pusergroups.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pauthzusergroupsoperations.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pdevicegroups.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pauthzdevicegroupsoperations.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pauthzuserclaims.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pauthzuserclaimsoperations.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pauthzdeviceclaims.unwrap_or(::std::ptr::null())),
            ::core::mem::transmute(pauthzdeviceclaimsoperations.unwrap_or(::std::ptr::null())),
            peffpermresultlists,
        )
        .ok()
    }
}
#[repr(C)]
pub struct IEffectivePermission2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ComputeEffectivePermissionWithSecondarySecurity: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::PSID, super::super::super::Foundation::PSID, ::windows_core::PCWSTR, *mut SECURITY_OBJECT, u32, *const super::super::TOKEN_GROUPS, *const super::AUTHZ_SID_OPERATION, *const super::super::TOKEN_GROUPS, *const super::AUTHZ_SID_OPERATION, *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION, *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION, *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION, *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION, *mut EFFPERM_RESULT_LIST) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISecurityInformation, ISecurityInformation_Vtbl, 0x965fc360_16ff_11d0_91cb_00aa00bbb723);
::windows_core::imp::interface_hierarchy!(ISecurityInformation, ::windows_core::IUnknown);
impl ISecurityInformation {
    pub unsafe fn GetObjectInformation(&self, pobjectinfo: *mut SI_OBJECT_INFO) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetObjectInformation)(::windows_core::Interface::as_raw(self), pobjectinfo).ok()
    }
    pub unsafe fn GetSecurity<P0>(&self, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut super::super::PSECURITY_DESCRIPTOR, fdefault: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).GetSecurity)(::windows_core::Interface::as_raw(self), requestedinformation, ppsecuritydescriptor, fdefault.into_param().abi()).ok()
    }
    pub unsafe fn SetSecurity<P0>(&self, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::PSECURITY_DESCRIPTOR>,
    {
        (::windows_core::Interface::vtable(self).SetSecurity)(::windows_core::Interface::as_raw(self), securityinformation, psecuritydescriptor.into_param().abi()).ok()
    }
    pub unsafe fn GetAccessRights(&self, pguidobjecttype: *const ::windows_core::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAccessRights)(::windows_core::Interface::as_raw(self), pguidobjecttype, dwflags, ppaccess, pcaccesses, pidefaultaccess).ok()
    }
    pub unsafe fn MapGeneric(&self, pguidobjecttype: *const ::windows_core::GUID, paceflags: *mut u8, pmask: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).MapGeneric)(::windows_core::Interface::as_raw(self), pguidobjecttype, paceflags, pmask).ok()
    }
    pub unsafe fn GetInheritTypes(&self, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInheritTypes)(::windows_core::Interface::as_raw(self), ppinherittypes, pcinherittypes).ok()
    }
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn PropertySheetPageCallback<P0>(&self, hwnd: P0, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).PropertySheetPageCallback)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), umsg, upage).ok()
    }
}
#[repr(C)]
pub struct ISecurityInformation_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetObjectInformation: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut SI_OBJECT_INFO) -> ::windows_core::HRESULT,
    pub GetSecurity: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::OBJECT_SECURITY_INFORMATION, *mut super::super::PSECURITY_DESCRIPTOR, super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub SetSecurity: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::OBJECT_SECURITY_INFORMATION, super::super::PSECURITY_DESCRIPTOR) -> ::windows_core::HRESULT,
    pub GetAccessRights: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID, SECURITY_INFO_PAGE_FLAGS, *mut *mut SI_ACCESS, *mut u32, *mut u32) -> ::windows_core::HRESULT,
    pub MapGeneric: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const ::windows_core::GUID, *mut u8, *mut u32) -> ::windows_core::HRESULT,
    pub GetInheritTypes: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut SI_INHERIT_TYPE, *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_UI_Controls")]
    pub PropertySheetPageCallback: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::HWND, super::super::super::UI::Controls::PSPCB_MESSAGE, SI_PAGE_TYPE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    PropertySheetPageCallback: usize,
}
::windows_core::imp::com_interface!(ISecurityInformation2, ISecurityInformation2_Vtbl, 0xc3ccfdb4_6f88_11d2_a3ce_00c04fb1782a);
::windows_core::imp::interface_hierarchy!(ISecurityInformation2, ::windows_core::IUnknown);
impl ISecurityInformation2 {
    pub unsafe fn IsDaclCanonical(&self, pdacl: *const super::super::ACL) -> super::super::super::Foundation::BOOL {
        (::windows_core::Interface::vtable(self).IsDaclCanonical)(::windows_core::Interface::as_raw(self), pdacl)
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LookupSids(&self, csids: u32, rgpsids: *const super::super::super::Foundation::PSID) -> ::windows_core::Result<super::super::super::System::Com::IDataObject> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).LookupSids)(::windows_core::Interface::as_raw(self), csids, rgpsids, &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
    }
}
#[repr(C)]
pub struct ISecurityInformation2_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub IsDaclCanonical: unsafe extern "system" fn(*mut ::core::ffi::c_void, *const super::super::ACL) -> super::super::super::Foundation::BOOL,
    #[cfg(feature = "Win32_System_Com")]
    pub LookupSids: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *const super::super::super::Foundation::PSID, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LookupSids: usize,
}
::windows_core::imp::com_interface!(ISecurityInformation3, ISecurityInformation3_Vtbl, 0xe2cdc9cc_31bd_4f8f_8c8b_b641af516a1a);
::windows_core::imp::interface_hierarchy!(ISecurityInformation3, ::windows_core::IUnknown);
impl ISecurityInformation3 {
    pub unsafe fn GetFullResourceName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFullResourceName)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn OpenElevatedEditor<P0>(&self, hwnd: P0, upage: SI_PAGE_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
    {
        (::windows_core::Interface::vtable(self).OpenElevatedEditor)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), upage).ok()
    }
}
#[repr(C)]
pub struct ISecurityInformation3_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetFullResourceName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub OpenElevatedEditor: unsafe extern "system" fn(*mut ::core::ffi::c_void, super::super::super::Foundation::HWND, SI_PAGE_TYPE) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISecurityInformation4, ISecurityInformation4_Vtbl, 0xea961070_cd14_4621_ace4_f63c03e583e4);
::windows_core::imp::interface_hierarchy!(ISecurityInformation4, ::windows_core::IUnknown);
impl ISecurityInformation4 {
    pub unsafe fn GetSecondarySecurity(&self, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSecondarySecurity)(::windows_core::Interface::as_raw(self), psecurityobjects, psecurityobjectcount).ok()
    }
}
#[repr(C)]
pub struct ISecurityInformation4_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetSecondarySecurity: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut SECURITY_OBJECT, *mut u32) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(ISecurityObjectTypeInfo, ISecurityObjectTypeInfo_Vtbl, 0xfc3066eb_79ef_444b_9111_d18a75ebf2fa);
::windows_core::imp::interface_hierarchy!(ISecurityObjectTypeInfo, ::windows_core::IUnknown);
impl ISecurityObjectTypeInfo {
    pub unsafe fn GetInheritSource(&self, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetInheritSource)(::windows_core::Interface::as_raw(self), si, pacl, ppinheritarray).ok()
    }
}
#[repr(C)]
pub struct ISecurityObjectTypeInfo_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetInheritSource: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32, *mut super::super::ACL, *mut *mut super::INHERITED_FROMA) -> ::windows_core::HRESULT,
}
pub const CFSTR_ACLUI_SID_INFO_LIST: ::windows_core::PCWSTR = ::windows_core::w!("CFSTR_ACLUI_SID_INFO_LIST");
pub const DOBJ_COND_NTACLS: i32 = 8i32;
pub const DOBJ_RES_CONT: i32 = 1i32;
pub const DOBJ_RES_ROOT: i32 = 2i32;
pub const DOBJ_RIBBON_LAUNCH: i32 = 16i32;
pub const DOBJ_VOL_NTACLS: i32 = 4i32;
pub const SECURITY_OBJECT_ID_CENTRAL_ACCESS_RULE: u32 = 4u32;
pub const SECURITY_OBJECT_ID_CENTRAL_POLICY: u32 = 3u32;
pub const SECURITY_OBJECT_ID_OBJECT_SD: u32 = 1u32;
pub const SECURITY_OBJECT_ID_SHARE: u32 = 2u32;
pub const SI_ACCESS_CONTAINER: i32 = 262144i32;
pub const SI_ACCESS_GENERAL: i32 = 131072i32;
pub const SI_ACCESS_PROPERTY: i32 = 524288i32;
pub const SI_ACCESS_SPECIFIC: i32 = 65536i32;
pub const SI_ADVANCED: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(16u32);
pub const SI_AUDITS_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(33554432u32);
pub const SI_CONTAINER: i32 = 4i32;
pub const SI_DISABLE_DENY_ACE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(2147483648u32);
pub const SI_EDIT_AUDITS: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(2u32);
pub const SI_EDIT_EFFECTIVE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(131072u32);
pub const SI_EDIT_OWNER: i32 = 1i32;
pub const SI_EDIT_PERMS: i32 = 0i32;
pub const SI_EDIT_PROPERTIES: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(128u32);
pub const SI_ENABLE_CENTRAL_POLICY: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(1073741824u32);
pub const SI_ENABLE_EDIT_ATTRIBUTE_CONDITION: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(536870912u32);
pub const SI_MAY_WRITE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(268435456u32);
pub const SI_NO_ACL_PROTECT: i32 = 512i32;
pub const SI_NO_ADDITIONAL_PERMISSION: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(2097152u32);
pub const SI_NO_TREE_APPLY: i32 = 1024i32;
pub const SI_OBJECT_GUID: i32 = 65536i32;
pub const SI_OWNER_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(67108864u32);
pub const SI_OWNER_READONLY: i32 = 64i32;
pub const SI_OWNER_RECURSE: i32 = 256i32;
pub const SI_PAGE_ADVPERM: SI_PAGE_TYPE = SI_PAGE_TYPE(1i32);
pub const SI_PAGE_AUDIT: SI_PAGE_TYPE = SI_PAGE_TYPE(2i32);
pub const SI_PAGE_EFFECTIVE: SI_PAGE_TYPE = SI_PAGE_TYPE(4i32);
pub const SI_PAGE_OWNER: SI_PAGE_TYPE = SI_PAGE_TYPE(3i32);
pub const SI_PAGE_PERM: SI_PAGE_TYPE = SI_PAGE_TYPE(0i32);
pub const SI_PAGE_SHARE: SI_PAGE_TYPE = SI_PAGE_TYPE(6i32);
pub const SI_PAGE_TAKEOWNERSHIP: SI_PAGE_TYPE = SI_PAGE_TYPE(5i32);
pub const SI_PAGE_TITLE: i32 = 2048i32;
pub const SI_PERMS_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(16777216u32);
pub const SI_READONLY: i32 = 8i32;
pub const SI_RESET: i32 = 32i32;
pub const SI_RESET_DACL: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(262144u32);
pub const SI_RESET_DACL_TREE: i32 = 16384i32;
pub const SI_RESET_OWNER: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(1048576u32);
pub const SI_RESET_SACL: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(524288u32);
pub const SI_RESET_SACL_TREE: i32 = 32768i32;
pub const SI_SCOPE_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(134217728u32);
pub const SI_SERVER_IS_DC: i32 = 4096i32;
pub const SI_SHOW_AUDIT_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(2i32);
pub const SI_SHOW_CENTRAL_POLICY_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(6i32);
pub const SI_SHOW_DEFAULT: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(0i32);
pub const SI_SHOW_EFFECTIVE_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(4i32);
pub const SI_SHOW_OWNER_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(3i32);
pub const SI_SHOW_PERM_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(1i32);
pub const SI_SHOW_SHARE_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(5i32);
pub const SI_VIEW_ONLY: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(4194304u32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SECURITY_INFO_PAGE_FLAGS(pub u32);
impl ::windows_core::TypeKind for SECURITY_INFO_PAGE_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SECURITY_INFO_PAGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_INFO_PAGE_FLAGS").field(&self.0).finish()
    }
}
impl SECURITY_INFO_PAGE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SECURITY_INFO_PAGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SECURITY_INFO_PAGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SECURITY_INFO_PAGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SECURITY_INFO_PAGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SECURITY_INFO_PAGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SI_OBJECT_INFO_FLAGS(pub u32);
impl ::windows_core::TypeKind for SI_OBJECT_INFO_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SI_OBJECT_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SI_OBJECT_INFO_FLAGS").field(&self.0).finish()
    }
}
impl SI_OBJECT_INFO_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for SI_OBJECT_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SI_OBJECT_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SI_OBJECT_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SI_OBJECT_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SI_OBJECT_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SI_PAGE_ACTIVATED(pub i32);
impl ::windows_core::TypeKind for SI_PAGE_ACTIVATED {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SI_PAGE_ACTIVATED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SI_PAGE_ACTIVATED").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct SI_PAGE_TYPE(pub i32);
impl ::windows_core::TypeKind for SI_PAGE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for SI_PAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SI_PAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct EFFPERM_RESULT_LIST {
    pub fEvaluated: super::super::super::Foundation::BOOLEAN,
    pub cObjectTypeListLength: u32,
    pub pObjectTypeList: *mut super::super::OBJECT_TYPE_LIST,
    pub pGrantedAccessList: *mut u32,
}
impl ::core::marker::Copy for EFFPERM_RESULT_LIST {}
impl ::core::clone::Clone for EFFPERM_RESULT_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for EFFPERM_RESULT_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFFPERM_RESULT_LIST").field("fEvaluated", &self.fEvaluated).field("cObjectTypeListLength", &self.cObjectTypeListLength).field("pObjectTypeList", &self.pObjectTypeList).field("pGrantedAccessList", &self.pGrantedAccessList).finish()
    }
}
impl ::windows_core::TypeKind for EFFPERM_RESULT_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for EFFPERM_RESULT_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.fEvaluated == other.fEvaluated && self.cObjectTypeListLength == other.cObjectTypeListLength && self.pObjectTypeList == other.pObjectTypeList && self.pGrantedAccessList == other.pGrantedAccessList
    }
}
impl ::core::cmp::Eq for EFFPERM_RESULT_LIST {}
impl ::core::default::Default for EFFPERM_RESULT_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SECURITY_OBJECT {
    pub pwszName: ::windows_core::PWSTR,
    pub pData: *mut ::core::ffi::c_void,
    pub cbData: u32,
    pub pData2: *mut ::core::ffi::c_void,
    pub cbData2: u32,
    pub Id: u32,
    pub fWellKnown: super::super::super::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for SECURITY_OBJECT {}
impl ::core::clone::Clone for SECURITY_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SECURITY_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_OBJECT").field("pwszName", &self.pwszName).field("pData", &self.pData).field("cbData", &self.cbData).field("pData2", &self.pData2).field("cbData2", &self.cbData2).field("Id", &self.Id).field("fWellKnown", &self.fWellKnown).finish()
    }
}
impl ::windows_core::TypeKind for SECURITY_OBJECT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SECURITY_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.pwszName == other.pwszName && self.pData == other.pData && self.cbData == other.cbData && self.pData2 == other.pData2 && self.cbData2 == other.cbData2 && self.Id == other.Id && self.fWellKnown == other.fWellKnown
    }
}
impl ::core::cmp::Eq for SECURITY_OBJECT {}
impl ::core::default::Default for SECURITY_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SID_INFO {
    pub pSid: super::super::super::Foundation::PSID,
    pub pwzCommonName: ::windows_core::PWSTR,
    pub pwzClass: ::windows_core::PWSTR,
    pub pwzUPN: ::windows_core::PWSTR,
}
impl ::core::marker::Copy for SID_INFO {}
impl ::core::clone::Clone for SID_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SID_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SID_INFO").field("pSid", &self.pSid).field("pwzCommonName", &self.pwzCommonName).field("pwzClass", &self.pwzClass).field("pwzUPN", &self.pwzUPN).finish()
    }
}
impl ::windows_core::TypeKind for SID_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pSid == other.pSid && self.pwzCommonName == other.pwzCommonName && self.pwzClass == other.pwzClass && self.pwzUPN == other.pwzUPN
    }
}
impl ::core::cmp::Eq for SID_INFO {}
impl ::core::default::Default for SID_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SID_INFO_LIST {
    pub cItems: u32,
    pub aSidInfo: [SID_INFO; 1],
}
impl ::core::marker::Copy for SID_INFO_LIST {}
impl ::core::clone::Clone for SID_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SID_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SID_INFO_LIST").field("cItems", &self.cItems).field("aSidInfo", &self.aSidInfo).finish()
    }
}
impl ::windows_core::TypeKind for SID_INFO_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SID_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.cItems == other.cItems && self.aSidInfo == other.aSidInfo
    }
}
impl ::core::cmp::Eq for SID_INFO_LIST {}
impl ::core::default::Default for SID_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SI_ACCESS {
    pub pguid: *const ::windows_core::GUID,
    pub mask: u32,
    pub pszName: ::windows_core::PCWSTR,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for SI_ACCESS {}
impl ::core::clone::Clone for SI_ACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SI_ACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SI_ACCESS").field("pguid", &self.pguid).field("mask", &self.mask).field("pszName", &self.pszName).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows_core::TypeKind for SI_ACCESS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SI_ACCESS {
    fn eq(&self, other: &Self) -> bool {
        self.pguid == other.pguid && self.mask == other.mask && self.pszName == other.pszName && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for SI_ACCESS {}
impl ::core::default::Default for SI_ACCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SI_INHERIT_TYPE {
    pub pguid: *const ::windows_core::GUID,
    pub dwFlags: super::super::ACE_FLAGS,
    pub pszName: ::windows_core::PCWSTR,
}
impl ::core::marker::Copy for SI_INHERIT_TYPE {}
impl ::core::clone::Clone for SI_INHERIT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SI_INHERIT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SI_INHERIT_TYPE").field("pguid", &self.pguid).field("dwFlags", &self.dwFlags).field("pszName", &self.pszName).finish()
    }
}
impl ::windows_core::TypeKind for SI_INHERIT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SI_INHERIT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.pguid == other.pguid && self.dwFlags == other.dwFlags && self.pszName == other.pszName
    }
}
impl ::core::cmp::Eq for SI_INHERIT_TYPE {}
impl ::core::default::Default for SI_INHERIT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct SI_OBJECT_INFO {
    pub dwFlags: SI_OBJECT_INFO_FLAGS,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub pszServerName: ::windows_core::PWSTR,
    pub pszObjectName: ::windows_core::PWSTR,
    pub pszPageTitle: ::windows_core::PWSTR,
    pub guidObjectType: ::windows_core::GUID,
}
impl ::core::marker::Copy for SI_OBJECT_INFO {}
impl ::core::clone::Clone for SI_OBJECT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SI_OBJECT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SI_OBJECT_INFO").field("dwFlags", &self.dwFlags).field("hInstance", &self.hInstance).field("pszServerName", &self.pszServerName).field("pszObjectName", &self.pszObjectName).field("pszPageTitle", &self.pszPageTitle).field("guidObjectType", &self.guidObjectType).finish()
    }
}
impl ::windows_core::TypeKind for SI_OBJECT_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SI_OBJECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.hInstance == other.hInstance && self.pszServerName == other.pszServerName && self.pszObjectName == other.pszObjectName && self.pszPageTitle == other.pszPageTitle && self.guidObjectType == other.guidObjectType
    }
}
impl ::core::cmp::Eq for SI_OBJECT_INFO {}
impl ::core::default::Default for SI_OBJECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
