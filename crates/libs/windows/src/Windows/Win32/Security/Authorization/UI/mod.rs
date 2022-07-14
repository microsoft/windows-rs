#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const CFSTR_ACLUI_SID_INFO_LIST: &str = "CFSTR_ACLUI_SID_INFO_LIST";
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`, `\"Win32_UI_Controls\"`*"]
#[cfg(feature = "Win32_UI_Controls")]
#[inline]
pub unsafe fn CreateSecurityPage<'a, P0>(psi: P0) -> ::windows::core::Result<super::super::super::UI::Controls::HPROPSHEETPAGE>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, ISecurityInformation>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateSecurityPage(psi: *mut ::core::ffi::c_void) -> super::super::super::UI::Controls::HPROPSHEETPAGE;
    }
    let result__ = CreateSecurityPage(psi.into().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const DOBJ_COND_NTACLS: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const DOBJ_RES_CONT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const DOBJ_RES_ROOT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const DOBJ_RIBBON_LAUNCH: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const DOBJ_VOL_NTACLS: i32 = 4i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct EFFPERM_RESULT_LIST {
    pub fEvaluated: super::super::super::Foundation::BOOLEAN,
    pub cObjectTypeListLength: u32,
    pub pObjectTypeList: *mut super::super::OBJECT_TYPE_LIST,
    pub pGrantedAccessList: *mut u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for EFFPERM_RESULT_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for EFFPERM_RESULT_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for EFFPERM_RESULT_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EFFPERM_RESULT_LIST").field("fEvaluated", &self.fEvaluated).field("cObjectTypeListLength", &self.cObjectTypeListLength).field("pObjectTypeList", &self.pObjectTypeList).field("pGrantedAccessList", &self.pGrantedAccessList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for EFFPERM_RESULT_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for EFFPERM_RESULT_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<EFFPERM_RESULT_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for EFFPERM_RESULT_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for EFFPERM_RESULT_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EditSecurity<'a, P0, P1>(hwndowner: P0, psi: P1) -> super::super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, ISecurityInformation>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EditSecurity(hwndowner: super::super::super::Foundation::HWND, psi: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL;
    }
    EditSecurity(hwndowner.into(), psi.into().abi())
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EditSecurityAdvanced<'a, P0, P1>(hwndowner: P0, psi: P1, usipage: SI_PAGE_TYPE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, ISecurityInformation>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EditSecurityAdvanced(hwndowner: super::super::super::Foundation::HWND, psi: *mut ::core::ffi::c_void, usipage: SI_PAGE_TYPE) -> ::windows::core::HRESULT;
    }
    EditSecurityAdvanced(hwndowner.into(), psi.into().abi(), usipage).ok()
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
#[repr(transparent)]
pub struct IEffectivePermission(::windows::core::IUnknown);
impl IEffectivePermission {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetEffectivePermission<'a, P0, P1, P2>(&self, pguidobjecttype: *const ::windows::core::GUID, pusersid: P0, pszservername: P1, psd: P2, ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST, pcobjecttypelistlength: *mut u32, ppgrantedaccesslist: *mut *mut u32, pcgrantedaccesslistlength: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::PSID>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
        P2: ::std::convert::Into<super::super::PSECURITY_DESCRIPTOR>,
    {
        (::windows::core::Interface::vtable(self).GetEffectivePermission)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidobjecttype), pusersid.into(), pszservername.into(), psd.into(), ::core::mem::transmute(ppobjecttypelist), ::core::mem::transmute(pcobjecttypelistlength), ::core::mem::transmute(ppgrantedaccesslist), ::core::mem::transmute(pcgrantedaccesslistlength)).ok()
    }
}
impl ::core::convert::From<IEffectivePermission> for ::windows::core::IUnknown {
    fn from(value: IEffectivePermission) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEffectivePermission> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEffectivePermission) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEffectivePermission> for ::windows::core::IUnknown {
    fn from(value: &IEffectivePermission) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEffectivePermission {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEffectivePermission {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEffectivePermission {}
impl ::core::fmt::Debug for IEffectivePermission {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEffectivePermission").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEffectivePermission {
    type Vtable = IEffectivePermission_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3853dc76_9f35_407c_88a1_d19344365fbc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEffectivePermission_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetEffectivePermission: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows::core::GUID, pusersid: super::super::super::Foundation::PSID, pszservername: ::windows::core::PCWSTR, psd: super::super::PSECURITY_DESCRIPTOR, ppobjecttypelist: *mut *mut super::super::OBJECT_TYPE_LIST, pcobjecttypelistlength: *mut u32, ppgrantedaccesslist: *mut *mut u32, pcgrantedaccesslistlength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetEffectivePermission: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
#[repr(transparent)]
pub struct IEffectivePermission2(::windows::core::IUnknown);
impl IEffectivePermission2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ComputeEffectivePermissionWithSecondarySecurity<'a, P0, P1, P2>(
        &self,
        psid: P0,
        pdevicesid: P1,
        pszservername: P2,
        psecurityobjects: *mut SECURITY_OBJECT,
        dwsecurityobjectcount: u32,
        pusergroups: *const super::super::TOKEN_GROUPS,
        pauthzusergroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pdevicegroups: *const super::super::TOKEN_GROUPS,
        pauthzdevicegroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pauthzuserclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzuserclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        pauthzdeviceclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzdeviceclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        peffpermresultlists: *mut EFFPERM_RESULT_LIST,
    ) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::PSID>,
        P1: ::std::convert::Into<super::super::super::Foundation::PSID>,
        P2: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ComputeEffectivePermissionWithSecondarySecurity)(
            ::windows::core::Interface::as_raw(self),
            psid.into(),
            pdevicesid.into(),
            pszservername.into(),
            ::core::mem::transmute(psecurityobjects),
            ::core::mem::transmute(dwsecurityobjectcount),
            ::core::mem::transmute(pusergroups),
            ::core::mem::transmute(pauthzusergroupsoperations),
            ::core::mem::transmute(pdevicegroups),
            ::core::mem::transmute(pauthzdevicegroupsoperations),
            ::core::mem::transmute(pauthzuserclaims),
            ::core::mem::transmute(pauthzuserclaimsoperations),
            ::core::mem::transmute(pauthzdeviceclaims),
            ::core::mem::transmute(pauthzdeviceclaimsoperations),
            ::core::mem::transmute(peffpermresultlists),
        )
        .ok()
    }
}
impl ::core::convert::From<IEffectivePermission2> for ::windows::core::IUnknown {
    fn from(value: IEffectivePermission2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEffectivePermission2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEffectivePermission2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEffectivePermission2> for ::windows::core::IUnknown {
    fn from(value: &IEffectivePermission2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEffectivePermission2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEffectivePermission2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEffectivePermission2 {}
impl ::core::fmt::Debug for IEffectivePermission2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEffectivePermission2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEffectivePermission2 {
    type Vtable = IEffectivePermission2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x941fabca_dd47_4fca_90bb_b0e10255f20d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEffectivePermission2_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ComputeEffectivePermissionWithSecondarySecurity: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        psid: super::super::super::Foundation::PSID,
        pdevicesid: super::super::super::Foundation::PSID,
        pszservername: ::windows::core::PCWSTR,
        psecurityobjects: *mut SECURITY_OBJECT,
        dwsecurityobjectcount: u32,
        pusergroups: *const super::super::TOKEN_GROUPS,
        pauthzusergroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pdevicegroups: *const super::super::TOKEN_GROUPS,
        pauthzdevicegroupsoperations: *const super::AUTHZ_SID_OPERATION,
        pauthzuserclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzuserclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        pauthzdeviceclaims: *const super::AUTHZ_SECURITY_ATTRIBUTES_INFORMATION,
        pauthzdeviceclaimsoperations: *const super::AUTHZ_SECURITY_ATTRIBUTE_OPERATION,
        peffpermresultlists: *mut EFFPERM_RESULT_LIST,
    ) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ComputeEffectivePermissionWithSecondarySecurity: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
#[repr(transparent)]
pub struct ISecurityInformation(::windows::core::IUnknown);
impl ISecurityInformation {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetObjectInformation(&self, pobjectinfo: *mut SI_OBJECT_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetObjectInformation)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pobjectinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSecurity<'a, P0>(&self, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut super::super::PSECURITY_DESCRIPTOR, fdefault: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).GetSecurity)(::windows::core::Interface::as_raw(self), requestedinformation, ::core::mem::transmute(ppsecuritydescriptor), fdefault.into()).ok()
    }
    pub unsafe fn SetSecurity<'a, P0>(&self, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::PSECURITY_DESCRIPTOR>,
    {
        (::windows::core::Interface::vtable(self).SetSecurity)(::windows::core::Interface::as_raw(self), securityinformation, psecuritydescriptor.into()).ok()
    }
    pub unsafe fn GetAccessRights(&self, pguidobjecttype: *const ::windows::core::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAccessRights)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidobjecttype), dwflags, ::core::mem::transmute(ppaccess), ::core::mem::transmute(pcaccesses), ::core::mem::transmute(pidefaultaccess)).ok()
    }
    pub unsafe fn MapGeneric(&self, pguidobjecttype: *const ::windows::core::GUID, paceflags: *mut u8, pmask: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).MapGeneric)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidobjecttype), ::core::mem::transmute(paceflags), ::core::mem::transmute(pmask)).ok()
    }
    pub unsafe fn GetInheritTypes(&self, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInheritTypes)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppinherittypes), ::core::mem::transmute(pcinherittypes)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Controls\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub unsafe fn PropertySheetPageCallback<'a, P0>(&self, hwnd: P0, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).PropertySheetPageCallback)(::windows::core::Interface::as_raw(self), hwnd.into(), umsg, upage).ok()
    }
}
impl ::core::convert::From<ISecurityInformation> for ::windows::core::IUnknown {
    fn from(value: ISecurityInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISecurityInformation> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISecurityInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISecurityInformation> for ::windows::core::IUnknown {
    fn from(value: &ISecurityInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISecurityInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISecurityInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISecurityInformation {}
impl ::core::fmt::Debug for ISecurityInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISecurityInformation {
    type Vtable = ISecurityInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x965fc360_16ff_11d0_91cb_00aa00bbb723);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityInformation_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetObjectInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobjectinfo: *mut SI_OBJECT_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetObjectInformation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedinformation: super::super::OBJECT_SECURITY_INFORMATION, ppsecuritydescriptor: *mut super::super::PSECURITY_DESCRIPTOR, fdefault: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSecurity: usize,
    pub SetSecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, securityinformation: super::super::OBJECT_SECURITY_INFORMATION, psecuritydescriptor: super::super::PSECURITY_DESCRIPTOR) -> ::windows::core::HRESULT,
    pub GetAccessRights: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows::core::GUID, dwflags: SECURITY_INFO_PAGE_FLAGS, ppaccess: *mut *mut SI_ACCESS, pcaccesses: *mut u32, pidefaultaccess: *mut u32) -> ::windows::core::HRESULT,
    pub MapGeneric: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidobjecttype: *const ::windows::core::GUID, paceflags: *mut u8, pmask: *mut u32) -> ::windows::core::HRESULT,
    pub GetInheritTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinherittypes: *mut *mut SI_INHERIT_TYPE, pcinherittypes: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub PropertySheetPageCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: super::super::super::UI::Controls::PSPCB_MESSAGE, upage: SI_PAGE_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls")))]
    PropertySheetPageCallback: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
#[repr(transparent)]
pub struct ISecurityInformation2(::windows::core::IUnknown);
impl ISecurityInformation2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsDaclCanonical(&self, pdacl: *mut super::super::ACL) -> super::super::super::Foundation::BOOL {
        (::windows::core::Interface::vtable(self).IsDaclCanonical)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdacl))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn LookupSids(&self, csids: u32, rgpsids: *mut super::super::super::Foundation::PSID, ppdo: *mut ::core::option::Option<super::super::super::System::Com::IDataObject>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LookupSids)(::windows::core::Interface::as_raw(self), csids, ::core::mem::transmute(rgpsids), ::core::mem::transmute(ppdo)).ok()
    }
}
impl ::core::convert::From<ISecurityInformation2> for ::windows::core::IUnknown {
    fn from(value: ISecurityInformation2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISecurityInformation2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISecurityInformation2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISecurityInformation2> for ::windows::core::IUnknown {
    fn from(value: &ISecurityInformation2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISecurityInformation2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISecurityInformation2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISecurityInformation2 {}
impl ::core::fmt::Debug for ISecurityInformation2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityInformation2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISecurityInformation2 {
    type Vtable = ISecurityInformation2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3ccfdb4_6f88_11d2_a3ce_00c04fb1782a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityInformation2_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsDaclCanonical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdacl: *mut super::super::ACL) -> super::super::super::Foundation::BOOL,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsDaclCanonical: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub LookupSids: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, csids: u32, rgpsids: *mut super::super::super::Foundation::PSID, ppdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    LookupSids: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
#[repr(transparent)]
pub struct ISecurityInformation3(::windows::core::IUnknown);
impl ISecurityInformation3 {
    pub unsafe fn GetFullResourceName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFullResourceName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenElevatedEditor<'a, P0>(&self, hwnd: P0, upage: SI_PAGE_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).OpenElevatedEditor)(::windows::core::Interface::as_raw(self), hwnd.into(), upage).ok()
    }
}
impl ::core::convert::From<ISecurityInformation3> for ::windows::core::IUnknown {
    fn from(value: ISecurityInformation3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISecurityInformation3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISecurityInformation3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISecurityInformation3> for ::windows::core::IUnknown {
    fn from(value: &ISecurityInformation3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISecurityInformation3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISecurityInformation3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISecurityInformation3 {}
impl ::core::fmt::Debug for ISecurityInformation3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityInformation3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISecurityInformation3 {
    type Vtable = ISecurityInformation3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2cdc9cc_31bd_4f8f_8c8b_b641af516a1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityInformation3_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetFullResourceName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszresourcename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub OpenElevatedEditor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, upage: SI_PAGE_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OpenElevatedEditor: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
#[repr(transparent)]
pub struct ISecurityInformation4(::windows::core::IUnknown);
impl ISecurityInformation4 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSecondarySecurity(&self, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSecondarySecurity)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psecurityobjects), ::core::mem::transmute(psecurityobjectcount)).ok()
    }
}
impl ::core::convert::From<ISecurityInformation4> for ::windows::core::IUnknown {
    fn from(value: ISecurityInformation4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISecurityInformation4> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISecurityInformation4) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISecurityInformation4> for ::windows::core::IUnknown {
    fn from(value: &ISecurityInformation4) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISecurityInformation4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISecurityInformation4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISecurityInformation4 {}
impl ::core::fmt::Debug for ISecurityInformation4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityInformation4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISecurityInformation4 {
    type Vtable = ISecurityInformation4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea961070_cd14_4621_ace4_f63c03e583e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityInformation4_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSecondarySecurity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psecurityobjects: *mut *mut SECURITY_OBJECT, psecurityobjectcount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSecondarySecurity: usize,
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
#[repr(transparent)]
pub struct ISecurityObjectTypeInfo(::windows::core::IUnknown);
impl ISecurityObjectTypeInfo {
    pub unsafe fn GetInheritSource(&self, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInheritSource)(::windows::core::Interface::as_raw(self), si, ::core::mem::transmute(pacl), ::core::mem::transmute(ppinheritarray)).ok()
    }
}
impl ::core::convert::From<ISecurityObjectTypeInfo> for ::windows::core::IUnknown {
    fn from(value: ISecurityObjectTypeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISecurityObjectTypeInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISecurityObjectTypeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISecurityObjectTypeInfo> for ::windows::core::IUnknown {
    fn from(value: &ISecurityObjectTypeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISecurityObjectTypeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISecurityObjectTypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISecurityObjectTypeInfo {}
impl ::core::fmt::Debug for ISecurityObjectTypeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISecurityObjectTypeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISecurityObjectTypeInfo {
    type Vtable = ISecurityObjectTypeInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc3066eb_79ef_444b_9111_d18a75ebf2fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISecurityObjectTypeInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetInheritSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, si: u32, pacl: *mut super::super::ACL, ppinheritarray: *mut *mut super::INHERITED_FROMA) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECURITY_INFO_PAGE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_ADVANCED: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_EDIT_AUDITS: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_EDIT_PROPERTIES: SECURITY_INFO_PAGE_FLAGS = SECURITY_INFO_PAGE_FLAGS(128u32);
impl ::core::marker::Copy for SECURITY_INFO_PAGE_FLAGS {}
impl ::core::clone::Clone for SECURITY_INFO_PAGE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SECURITY_INFO_PAGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SECURITY_INFO_PAGE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SECURITY_INFO_PAGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_INFO_PAGE_FLAGS").field(&self.0).finish()
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
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_OBJECT {
    pub pwszName: ::windows::core::PWSTR,
    pub pData: *mut ::core::ffi::c_void,
    pub cbData: u32,
    pub pData2: *mut ::core::ffi::c_void,
    pub cbData2: u32,
    pub Id: u32,
    pub fWellKnown: super::super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_OBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECURITY_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_OBJECT").field("pwszName", &self.pwszName).field("pData", &self.pData).field("cbData", &self.cbData).field("pData2", &self.pData2).field("cbData2", &self.cbData2).field("Id", &self.Id).field("fWellKnown", &self.fWellKnown).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECURITY_OBJECT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECURITY_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SECURITY_OBJECT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECURITY_OBJECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECURITY_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SECURITY_OBJECT_ID_CENTRAL_ACCESS_RULE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SECURITY_OBJECT_ID_CENTRAL_POLICY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SECURITY_OBJECT_ID_OBJECT_SD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SECURITY_OBJECT_ID_SHARE: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SID_INFO {
    pub pSid: super::super::super::Foundation::PSID,
    pub pwzCommonName: ::windows::core::PWSTR,
    pub pwzClass: ::windows::core::PWSTR,
    pub pwzUPN: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SID_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SID_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SID_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SID_INFO").field("pSid", &self.pSid).field("pwzCommonName", &self.pwzCommonName).field("pwzClass", &self.pwzClass).field("pwzUPN", &self.pwzUPN).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SID_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SID_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SID_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SID_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SID_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SID_INFO_LIST {
    pub cItems: u32,
    pub aSidInfo: [SID_INFO; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SID_INFO_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SID_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SID_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SID_INFO_LIST").field("cItems", &self.cItems).field("aSidInfo", &self.aSidInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SID_INFO_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SID_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SID_INFO_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SID_INFO_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SID_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub struct SI_ACCESS {
    pub pguid: *const ::windows::core::GUID,
    pub mask: u32,
    pub pszName: ::windows::core::PCWSTR,
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
unsafe impl ::windows::core::Abi for SI_ACCESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SI_ACCESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SI_ACCESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for SI_ACCESS {}
impl ::core::default::Default for SI_ACCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_ACCESS_CONTAINER: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_ACCESS_GENERAL: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_ACCESS_PROPERTY: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_ACCESS_SPECIFIC: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_CONTAINER: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_EDIT_OWNER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_EDIT_PERMS: i32 = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub struct SI_INHERIT_TYPE {
    pub pguid: *const ::windows::core::GUID,
    pub dwFlags: super::super::ACE_FLAGS,
    pub pszName: ::windows::core::PCWSTR,
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
unsafe impl ::windows::core::Abi for SI_INHERIT_TYPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SI_INHERIT_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SI_INHERIT_TYPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SI_INHERIT_TYPE {}
impl ::core::default::Default for SI_INHERIT_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_NO_ACL_PROTECT: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_NO_TREE_APPLY: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_OBJECT_GUID: i32 = 65536i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SI_OBJECT_INFO {
    pub dwFlags: SI_OBJECT_INFO_FLAGS,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub pszServerName: ::windows::core::PWSTR,
    pub pszObjectName: ::windows::core::PWSTR,
    pub pszPageTitle: ::windows::core::PWSTR,
    pub guidObjectType: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SI_OBJECT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SI_OBJECT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SI_OBJECT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SI_OBJECT_INFO").field("dwFlags", &self.dwFlags).field("hInstance", &self.hInstance).field("pszServerName", &self.pszServerName).field("pszObjectName", &self.pszObjectName).field("pszPageTitle", &self.pszPageTitle).field("guidObjectType", &self.guidObjectType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SI_OBJECT_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SI_OBJECT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SI_OBJECT_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SI_OBJECT_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SI_OBJECT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SI_OBJECT_INFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_AUDITS_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(33554432u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_DISABLE_DENY_ACE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(2147483648u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_EDIT_EFFECTIVE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_ENABLE_CENTRAL_POLICY: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(1073741824u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_ENABLE_EDIT_ATTRIBUTE_CONDITION: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(536870912u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_MAY_WRITE: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_NO_ADDITIONAL_PERMISSION: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_OWNER_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(67108864u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_PERMS_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_RESET_DACL: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_RESET_OWNER: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_RESET_SACL: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_SCOPE_ELEVATION_REQUIRED: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(134217728u32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_VIEW_ONLY: SI_OBJECT_INFO_FLAGS = SI_OBJECT_INFO_FLAGS(4194304u32);
impl ::core::marker::Copy for SI_OBJECT_INFO_FLAGS {}
impl ::core::clone::Clone for SI_OBJECT_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SI_OBJECT_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SI_OBJECT_INFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SI_OBJECT_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SI_OBJECT_INFO_FLAGS").field(&self.0).finish()
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
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_OWNER_READONLY: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_OWNER_RECURSE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SI_PAGE_ACTIVATED(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_SHOW_DEFAULT: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(0i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_SHOW_PERM_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_SHOW_AUDIT_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_SHOW_OWNER_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_SHOW_EFFECTIVE_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(4i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_SHOW_SHARE_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(5i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_SHOW_CENTRAL_POLICY_ACTIVATED: SI_PAGE_ACTIVATED = SI_PAGE_ACTIVATED(6i32);
impl ::core::marker::Copy for SI_PAGE_ACTIVATED {}
impl ::core::clone::Clone for SI_PAGE_ACTIVATED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SI_PAGE_ACTIVATED {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SI_PAGE_ACTIVATED {
    type Abi = Self;
}
impl ::core::fmt::Debug for SI_PAGE_ACTIVATED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SI_PAGE_ACTIVATED").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_PAGE_TITLE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SI_PAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_PAGE_PERM: SI_PAGE_TYPE = SI_PAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_PAGE_ADVPERM: SI_PAGE_TYPE = SI_PAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_PAGE_AUDIT: SI_PAGE_TYPE = SI_PAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_PAGE_OWNER: SI_PAGE_TYPE = SI_PAGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_PAGE_EFFECTIVE: SI_PAGE_TYPE = SI_PAGE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_PAGE_TAKEOWNERSHIP: SI_PAGE_TYPE = SI_PAGE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_PAGE_SHARE: SI_PAGE_TYPE = SI_PAGE_TYPE(6i32);
impl ::core::marker::Copy for SI_PAGE_TYPE {}
impl ::core::clone::Clone for SI_PAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SI_PAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SI_PAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SI_PAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SI_PAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_READONLY: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_RESET: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_RESET_DACL_TREE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_RESET_SACL_TREE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Security_Authorization_UI\"`*"]
pub const SI_SERVER_IS_DC: i32 = 4096i32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
