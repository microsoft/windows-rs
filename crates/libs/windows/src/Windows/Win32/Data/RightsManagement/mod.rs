#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMACTSERVINFOVERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRMATTESTTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMATTESTTYPE_FULLENVIRONMENT: DRMATTESTTYPE = DRMATTESTTYPE(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMATTESTTYPE_HASHONLY: DRMATTESTTYPE = DRMATTESTTYPE(1i32);
impl ::core::marker::Copy for DRMATTESTTYPE {}
impl ::core::clone::Clone for DRMATTESTTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRMATTESTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRMATTESTTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRMATTESTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMATTESTTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAcquireAdvisories<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hlicensestorage: u32, wszlicense: Param1, wszurl: Param2, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMAcquireAdvisories(hlicensestorage: u32, wszlicense: ::windows::core::PCWSTR, wszurl: ::windows::core::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DRMAcquireAdvisories(::core::mem::transmute(hlicensestorage), wszlicense.into_param().abi(), wszurl.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAcquireIssuanceLicenseTemplate<'a, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hclient: u32, uflags: u32, pvreserved: *mut ::core::ffi::c_void, pwsztemplateids: &[::windows::core::PWSTR], wszurl: Param5, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMAcquireIssuanceLicenseTemplate(hclient: u32, uflags: u32, pvreserved: *mut ::core::ffi::c_void, ctemplates: u32, pwsztemplateids: *const ::windows::core::PWSTR, wszurl: ::windows::core::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DRMAcquireIssuanceLicenseTemplate(::core::mem::transmute(hclient), ::core::mem::transmute(uflags), ::core::mem::transmute(pvreserved), pwsztemplateids.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pwsztemplateids)), wszurl.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAcquireLicense<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hsession: u32, uflags: u32, wszgroupidentitycredential: Param2, wszrequestedrights: Param3, wszcustomdata: Param4, wszurl: Param5, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMAcquireLicense(hsession: u32, uflags: u32, wszgroupidentitycredential: ::windows::core::PCWSTR, wszrequestedrights: ::windows::core::PCWSTR, wszcustomdata: ::windows::core::PCWSTR, wszurl: ::windows::core::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DRMAcquireLicense(::core::mem::transmute(hsession), ::core::mem::transmute(uflags), wszgroupidentitycredential.into_param().abi(), wszrequestedrights.into_param().abi(), wszcustomdata.into_param().abi(), wszurl.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMActivate<'a, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hclient: u32, uflags: u32, ulangid: u32, pactservinfo: *mut DRM_ACTSERV_INFO, pvcontext: *mut ::core::ffi::c_void, hparentwnd: Param5) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMActivate(hclient: u32, uflags: u32, ulangid: u32, pactservinfo: *mut DRM_ACTSERV_INFO, pvcontext: *mut ::core::ffi::c_void, hparentwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
        }
        DRMActivate(::core::mem::transmute(hclient), ::core::mem::transmute(uflags), ::core::mem::transmute(ulangid), ::core::mem::transmute(pactservinfo), ::core::mem::transmute(pvcontext), hparentwnd.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAddLicense<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hlicensestorage: u32, uflags: u32, wszlicense: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMAddLicense(hlicensestorage: u32, uflags: u32, wszlicense: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        DRMAddLicense(::core::mem::transmute(hlicensestorage), ::core::mem::transmute(uflags), wszlicense.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAddRightWithUser(hissuancelicense: u32, hright: u32, huser: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMAddRightWithUser(hissuancelicense: u32, hright: u32, huser: u32) -> ::windows::core::HRESULT;
        }
        DRMAddRightWithUser(::core::mem::transmute(hissuancelicense), ::core::mem::transmute(hright), ::core::mem::transmute(huser)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAttest<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(henablingprincipal: u32, wszdata: Param1, etype: DRMATTESTTYPE, pcattestedblob: *mut u32, wszattestedblob: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMAttest(henablingprincipal: u32, wszdata: ::windows::core::PCWSTR, etype: DRMATTESTTYPE, pcattestedblob: *mut u32, wszattestedblob: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMAttest(::core::mem::transmute(henablingprincipal), wszdata.into_param().abi(), ::core::mem::transmute(etype), ::core::mem::transmute(pcattestedblob), ::core::mem::transmute(wszattestedblob)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMBINDINGFLAGS_IGNORE_VALIDITY_INTERVALS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub struct DRMBOUNDLICENSEPARAMS {
    pub uVersion: u32,
    pub hEnablingPrincipal: u32,
    pub hSecureStore: u32,
    pub wszRightsRequested: ::windows::core::PWSTR,
    pub wszRightsGroup: ::windows::core::PWSTR,
    pub idResource: DRMID,
    pub cAuthenticatorCount: u32,
    pub rghAuthenticators: *mut u32,
    pub wszDefaultEnablingPrincipalCredentials: ::windows::core::PWSTR,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DRMBOUNDLICENSEPARAMS {}
impl ::core::clone::Clone for DRMBOUNDLICENSEPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRMBOUNDLICENSEPARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRMBOUNDLICENSEPARAMS")
            .field("uVersion", &self.uVersion)
            .field("hEnablingPrincipal", &self.hEnablingPrincipal)
            .field("hSecureStore", &self.hSecureStore)
            .field("wszRightsRequested", &self.wszRightsRequested)
            .field("wszRightsGroup", &self.wszRightsGroup)
            .field("idResource", &self.idResource)
            .field("cAuthenticatorCount", &self.cAuthenticatorCount)
            .field("rghAuthenticators", &self.rghAuthenticators)
            .field("wszDefaultEnablingPrincipalCredentials", &self.wszDefaultEnablingPrincipalCredentials)
            .field("dwFlags", &self.dwFlags)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for DRMBOUNDLICENSEPARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRMBOUNDLICENSEPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRMBOUNDLICENSEPARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRMBOUNDLICENSEPARAMS {}
impl ::core::default::Default for DRMBOUNDLICENSEPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMBOUNDLICENSEPARAMSVERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub type DRMCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: DRM_STATUS_MSG, param1: ::windows::core::HRESULT, param2: *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMCALLBACKVERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMCLIENTSTRUCTVERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCheckSecurity(henv: u32, clevel: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCheckSecurity(henv: u32, clevel: u32) -> ::windows::core::HRESULT;
        }
        DRMCheckSecurity(::core::mem::transmute(henv), ::core::mem::transmute(clevel)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMClearAllRights(hissuancelicense: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMClearAllRights(hissuancelicense: u32) -> ::windows::core::HRESULT;
        }
        DRMClearAllRights(::core::mem::transmute(hissuancelicense)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCloseEnvironmentHandle(henv: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCloseEnvironmentHandle(henv: u32) -> ::windows::core::HRESULT;
        }
        DRMCloseEnvironmentHandle(::core::mem::transmute(henv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCloseHandle(handle: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCloseHandle(handle: u32) -> ::windows::core::HRESULT;
        }
        DRMCloseHandle(::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMClosePubHandle(hpub: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMClosePubHandle(hpub: u32) -> ::windows::core::HRESULT;
        }
        DRMClosePubHandle(::core::mem::transmute(hpub)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCloseQueryHandle(hquery: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCloseQueryHandle(hquery: u32) -> ::windows::core::HRESULT;
        }
        DRMCloseQueryHandle(::core::mem::transmute(hquery)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCloseSession(hsession: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCloseSession(hsession: u32) -> ::windows::core::HRESULT;
        }
        DRMCloseSession(::core::mem::transmute(hsession)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMConstructCertificateChain(rgwszcertificates: &[::windows::core::PWSTR], pcchain: *mut u32, wszchain: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMConstructCertificateChain(ccertificates: u32, rgwszcertificates: *const ::windows::core::PWSTR, pcchain: *mut u32, wszchain: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMConstructCertificateChain(rgwszcertificates.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgwszcertificates)), ::core::mem::transmute(pcchain), ::core::mem::transmute(wszchain)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateBoundLicense<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(henv: u32, pparams: *mut DRMBOUNDLICENSEPARAMS, wszlicensechain: Param2, phboundlicense: *mut u32, pherrorlog: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCreateBoundLicense(henv: u32, pparams: *mut DRMBOUNDLICENSEPARAMS, wszlicensechain: ::windows::core::PCWSTR, phboundlicense: *mut u32, pherrorlog: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMCreateBoundLicense(::core::mem::transmute(henv), ::core::mem::transmute(pparams), wszlicensechain.into_param().abi(), ::core::mem::transmute(phboundlicense), ::core::mem::transmute(pherrorlog)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateClientSession<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pfncallback: DRMCALLBACK, ucallbackversion: u32, wszgroupidprovidertype: Param2, wszgroupid: Param3, phclient: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCreateClientSession(pfncallback: ::windows::core::RawPtr, ucallbackversion: u32, wszgroupidprovidertype: ::windows::core::PCWSTR, wszgroupid: ::windows::core::PCWSTR, phclient: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMCreateClientSession(::core::mem::transmute(pfncallback), ::core::mem::transmute(ucallbackversion), wszgroupidprovidertype.into_param().abi(), wszgroupid.into_param().abi(), ::core::mem::transmute(phclient)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateEnablingBitsDecryptor<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hboundlicense: u32, wszright: Param1, hauxlib: u32, wszauxplug: Param3, phdecryptor: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCreateEnablingBitsDecryptor(hboundlicense: u32, wszright: ::windows::core::PCWSTR, hauxlib: u32, wszauxplug: ::windows::core::PCWSTR, phdecryptor: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMCreateEnablingBitsDecryptor(::core::mem::transmute(hboundlicense), wszright.into_param().abi(), ::core::mem::transmute(hauxlib), wszauxplug.into_param().abi(), ::core::mem::transmute(phdecryptor)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateEnablingBitsEncryptor<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hboundlicense: u32, wszright: Param1, hauxlib: u32, wszauxplug: Param3, phencryptor: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCreateEnablingBitsEncryptor(hboundlicense: u32, wszright: ::windows::core::PCWSTR, hauxlib: u32, wszauxplug: ::windows::core::PCWSTR, phencryptor: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMCreateEnablingBitsEncryptor(::core::mem::transmute(hboundlicense), wszright.into_param().abi(), ::core::mem::transmute(hauxlib), wszauxplug.into_param().abi(), ::core::mem::transmute(phencryptor)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateEnablingPrincipal<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(henv: u32, hlibrary: u32, wszobject: Param2, pidprincipal: *mut DRMID, wszcredentials: Param4, phenablingprincipal: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCreateEnablingPrincipal(henv: u32, hlibrary: u32, wszobject: ::windows::core::PCWSTR, pidprincipal: *mut DRMID, wszcredentials: ::windows::core::PCWSTR, phenablingprincipal: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMCreateEnablingPrincipal(::core::mem::transmute(henv), ::core::mem::transmute(hlibrary), wszobject.into_param().abi(), ::core::mem::transmute(pidprincipal), wszcredentials.into_param().abi(), ::core::mem::transmute(phenablingprincipal)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMCreateIssuanceLicense<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, wszreferralinfoname: Param2, wszreferralinfourl: Param3, howner: u32, wszissuancelicense: Param5, hboundlicense: u32, phissuancelicense: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCreateIssuanceLicense(psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, wszreferralinfoname: ::windows::core::PCWSTR, wszreferralinfourl: ::windows::core::PCWSTR, howner: u32, wszissuancelicense: ::windows::core::PCWSTR, hboundlicense: u32, phissuancelicense: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMCreateIssuanceLicense(::core::mem::transmute(psttimefrom), ::core::mem::transmute(psttimeuntil), wszreferralinfoname.into_param().abi(), wszreferralinfourl.into_param().abi(), ::core::mem::transmute(howner), wszissuancelicense.into_param().abi(), ::core::mem::transmute(hboundlicense), ::core::mem::transmute(phissuancelicense)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateLicenseStorageSession<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(henv: u32, hdefaultlibrary: u32, hclient: u32, uflags: u32, wszissuancelicense: Param4, phlicensestorage: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCreateLicenseStorageSession(henv: u32, hdefaultlibrary: u32, hclient: u32, uflags: u32, wszissuancelicense: ::windows::core::PCWSTR, phlicensestorage: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMCreateLicenseStorageSession(::core::mem::transmute(henv), ::core::mem::transmute(hdefaultlibrary), ::core::mem::transmute(hclient), ::core::mem::transmute(uflags), wszissuancelicense.into_param().abi(), ::core::mem::transmute(phlicensestorage)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMCreateRight<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(wszrightname: Param0, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME, cextendedinfo: u32, pwszextendedinfoname: *const ::windows::core::PWSTR, pwszextendedinfovalue: *const ::windows::core::PWSTR, phright: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCreateRight(wszrightname: ::windows::core::PCWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME, cextendedinfo: u32, pwszextendedinfoname: *const ::windows::core::PWSTR, pwszextendedinfovalue: *const ::windows::core::PWSTR, phright: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMCreateRight(wszrightname.into_param().abi(), ::core::mem::transmute(pstfrom), ::core::mem::transmute(pstuntil), ::core::mem::transmute(cextendedinfo), ::core::mem::transmute(pwszextendedinfoname), ::core::mem::transmute(pwszextendedinfovalue), ::core::mem::transmute(phright)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateUser<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(wszusername: Param0, wszuserid: Param1, wszuseridtype: Param2, phuser: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMCreateUser(wszusername: ::windows::core::PCWSTR, wszuserid: ::windows::core::PCWSTR, wszuseridtype: ::windows::core::PCWSTR, phuser: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMCreateUser(wszusername.into_param().abi(), wszuserid.into_param().abi(), wszuseridtype.into_param().abi(), ::core::mem::transmute(phuser)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDecode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(wszalgid: Param0, wszencodedstring: Param1, pudecodeddatalen: *mut u32, pbdecodeddata: *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMDecode(wszalgid: ::windows::core::PCWSTR, wszencodedstring: ::windows::core::PCWSTR, pudecodeddatalen: *mut u32, pbdecodeddata: *mut u8) -> ::windows::core::HRESULT;
        }
        DRMDecode(wszalgid.into_param().abi(), wszencodedstring.into_param().abi(), ::core::mem::transmute(pudecodeddatalen), ::core::mem::transmute(pbdecodeddata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDeconstructCertificateChain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(wszchain: Param0, iwhich: u32, pccert: *mut u32, wszcert: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMDeconstructCertificateChain(wszchain: ::windows::core::PCWSTR, iwhich: u32, pccert: *mut u32, wszcert: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMDeconstructCertificateChain(wszchain.into_param().abi(), ::core::mem::transmute(iwhich), ::core::mem::transmute(pccert), ::core::mem::transmute(wszcert)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDecrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMDecrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows::core::HRESULT;
        }
        DRMDecrypt(::core::mem::transmute(hcryptoprovider), ::core::mem::transmute(iposition), ::core::mem::transmute(cnuminbytes), ::core::mem::transmute(pbindata), ::core::mem::transmute(pcnumoutbytes), ::core::mem::transmute(pboutdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDeleteLicense<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hsession: u32, wszlicenseid: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMDeleteLicense(hsession: u32, wszlicenseid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        DRMDeleteLicense(::core::mem::transmute(hsession), wszlicenseid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDuplicateEnvironmentHandle(htocopy: u32, phcopy: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMDuplicateEnvironmentHandle(htocopy: u32, phcopy: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMDuplicateEnvironmentHandle(::core::mem::transmute(htocopy), ::core::mem::transmute(phcopy)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDuplicateHandle(htocopy: u32, phcopy: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMDuplicateHandle(htocopy: u32, phcopy: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMDuplicateHandle(::core::mem::transmute(htocopy), ::core::mem::transmute(phcopy)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDuplicatePubHandle(hpubin: u32, phpubout: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMDuplicatePubHandle(hpubin: u32, phpubout: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMDuplicatePubHandle(::core::mem::transmute(hpubin), ::core::mem::transmute(phpubout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDuplicateSession(hsessionin: u32, phsessionout: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMDuplicateSession(hsessionin: u32, phsessionout: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMDuplicateSession(::core::mem::transmute(hsessionin), ::core::mem::transmute(phsessionout)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRMENCODINGTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENCODINGTYPE_BASE64: DRMENCODINGTYPE = DRMENCODINGTYPE(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENCODINGTYPE_STRING: DRMENCODINGTYPE = DRMENCODINGTYPE(1i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENCODINGTYPE_LONG: DRMENCODINGTYPE = DRMENCODINGTYPE(2i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENCODINGTYPE_TIME: DRMENCODINGTYPE = DRMENCODINGTYPE(3i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENCODINGTYPE_UINT: DRMENCODINGTYPE = DRMENCODINGTYPE(4i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENCODINGTYPE_RAW: DRMENCODINGTYPE = DRMENCODINGTYPE(5i32);
impl ::core::marker::Copy for DRMENCODINGTYPE {}
impl ::core::clone::Clone for DRMENCODINGTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRMENCODINGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRMENCODINGTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRMENCODINGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMENCODINGTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMENVHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMEncode<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(wszalgid: Param0, udatalen: u32, pbdecodeddata: *mut u8, puencodedstringlen: *mut u32, wszencodedstring: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMEncode(wszalgid: ::windows::core::PCWSTR, udatalen: u32, pbdecodeddata: *mut u8, puencodedstringlen: *mut u32, wszencodedstring: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMEncode(wszalgid.into_param().abi(), ::core::mem::transmute(udatalen), ::core::mem::transmute(pbdecodeddata), ::core::mem::transmute(puencodedstringlen), ::core::mem::transmute(wszencodedstring)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMEncrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMEncrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows::core::HRESULT;
        }
        DRMEncrypt(::core::mem::transmute(hcryptoprovider), ::core::mem::transmute(iposition), ::core::mem::transmute(cnuminbytes), ::core::mem::transmute(pbindata), ::core::mem::transmute(pcnumoutbytes), ::core::mem::transmute(pboutdata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMEnumerateLicense(hsession: u32, uflags: u32, uindex: u32, pfsharedflag: *mut super::super::Foundation::BOOL, pucertificatedatalen: *mut u32, wszcertificatedata: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMEnumerateLicense(hsession: u32, uflags: u32, uindex: u32, pfsharedflag: *mut super::super::Foundation::BOOL, pucertificatedatalen: *mut u32, wszcertificatedata: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMEnumerateLicense(::core::mem::transmute(hsession), ::core::mem::transmute(uflags), ::core::mem::transmute(uindex), ::core::mem::transmute(pfsharedflag), ::core::mem::transmute(pucertificatedatalen), ::core::mem::transmute(wszcertificatedata)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRMGLOBALOPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMGLOBALOPTIONS_USE_WINHTTP: DRMGLOBALOPTIONS = DRMGLOBALOPTIONS(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMGLOBALOPTIONS_USE_SERVERSECURITYPROCESSOR: DRMGLOBALOPTIONS = DRMGLOBALOPTIONS(1i32);
impl ::core::marker::Copy for DRMGLOBALOPTIONS {}
impl ::core::clone::Clone for DRMGLOBALOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRMGLOBALOPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRMGLOBALOPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRMGLOBALOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMGLOBALOPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetApplicationSpecificData(hissuancelicense: u32, uindex: u32, punamelength: *mut u32, wszname: ::windows::core::PWSTR, puvaluelength: *mut u32, wszvalue: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetApplicationSpecificData(hissuancelicense: u32, uindex: u32, punamelength: *mut u32, wszname: ::windows::core::PWSTR, puvaluelength: *mut u32, wszvalue: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMGetApplicationSpecificData(::core::mem::transmute(hissuancelicense), ::core::mem::transmute(uindex), ::core::mem::transmute(punamelength), ::core::mem::transmute(wszname), ::core::mem::transmute(puvaluelength), ::core::mem::transmute(wszvalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetBoundLicenseAttribute<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hqueryroot: u32, wszattribute: Param1, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetBoundLicenseAttribute(hqueryroot: u32, wszattribute: ::windows::core::PCWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT;
        }
        DRMGetBoundLicenseAttribute(::core::mem::transmute(hqueryroot), wszattribute.into_param().abi(), ::core::mem::transmute(iwhich), ::core::mem::transmute(peencoding), ::core::mem::transmute(pcbuffer), ::core::mem::transmute(pbbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetBoundLicenseAttributeCount<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hqueryroot: u32, wszattribute: Param1, pcattributes: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetBoundLicenseAttributeCount(hqueryroot: u32, wszattribute: ::windows::core::PCWSTR, pcattributes: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMGetBoundLicenseAttributeCount(::core::mem::transmute(hqueryroot), wszattribute.into_param().abi(), ::core::mem::transmute(pcattributes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetBoundLicenseObject<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hqueryroot: u32, wszsubobjecttype: Param1, iwhich: u32, phsubobject: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetBoundLicenseObject(hqueryroot: u32, wszsubobjecttype: ::windows::core::PCWSTR, iwhich: u32, phsubobject: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMGetBoundLicenseObject(::core::mem::transmute(hqueryroot), wszsubobjecttype.into_param().abi(), ::core::mem::transmute(iwhich), ::core::mem::transmute(phsubobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetBoundLicenseObjectCount<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hqueryroot: u32, wszsubobjecttype: Param1, pcsubobjects: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetBoundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: ::windows::core::PCWSTR, pcsubobjects: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMGetBoundLicenseObjectCount(::core::mem::transmute(hqueryroot), wszsubobjecttype.into_param().abi(), ::core::mem::transmute(pcsubobjects)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetCertificateChainCount<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(wszchain: Param0, pccertcount: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetCertificateChainCount(wszchain: ::windows::core::PCWSTR, pccertcount: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMGetCertificateChainCount(wszchain.into_param().abi(), ::core::mem::transmute(pccertcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetClientVersion(pdrmclientversioninfo: *mut DRM_CLIENT_VERSION_INFO) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetClientVersion(pdrmclientversioninfo: *mut DRM_CLIENT_VERSION_INFO) -> ::windows::core::HRESULT;
        }
        DRMGetClientVersion(::core::mem::transmute(pdrmclientversioninfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetEnvironmentInfo<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(handle: u32, wszattribute: Param1, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetEnvironmentInfo(handle: u32, wszattribute: ::windows::core::PCWSTR, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT;
        }
        DRMGetEnvironmentInfo(::core::mem::transmute(handle), wszattribute.into_param().abi(), ::core::mem::transmute(peencoding), ::core::mem::transmute(pcbuffer), ::core::mem::transmute(pbbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetInfo<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(handle: u32, wszattribute: Param1, peencoding: *const DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetInfo(handle: u32, wszattribute: ::windows::core::PCWSTR, peencoding: *const DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT;
        }
        DRMGetInfo(::core::mem::transmute(handle), wszattribute.into_param().abi(), ::core::mem::transmute(peencoding), ::core::mem::transmute(pcbuffer), ::core::mem::transmute(pbbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetIntervalTime(hissuancelicense: u32, pcdays: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetIntervalTime(hissuancelicense: u32, pcdays: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMGetIntervalTime(::core::mem::transmute(hissuancelicense), ::core::mem::transmute(pcdays)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetIssuanceLicenseInfo(hissuancelicense: u32, psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, uflags: u32, pudistributionpointnamelength: *mut u32, wszdistributionpointname: ::windows::core::PWSTR, pudistributionpointurllength: *mut u32, wszdistributionpointurl: ::windows::core::PWSTR, phowner: *mut u32, pfofficial: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetIssuanceLicenseInfo(hissuancelicense: u32, psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, uflags: u32, pudistributionpointnamelength: *mut u32, wszdistributionpointname: ::windows::core::PWSTR, pudistributionpointurllength: *mut u32, wszdistributionpointurl: ::windows::core::PWSTR, phowner: *mut u32, pfofficial: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        DRMGetIssuanceLicenseInfo(::core::mem::transmute(hissuancelicense), ::core::mem::transmute(psttimefrom), ::core::mem::transmute(psttimeuntil), ::core::mem::transmute(uflags), ::core::mem::transmute(pudistributionpointnamelength), ::core::mem::transmute(wszdistributionpointname), ::core::mem::transmute(pudistributionpointurllength), ::core::mem::transmute(wszdistributionpointurl), ::core::mem::transmute(phowner), ::core::mem::transmute(pfofficial)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetIssuanceLicenseTemplate(hissuancelicense: u32, puissuancelicensetemplatelength: *mut u32, wszissuancelicensetemplate: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetIssuanceLicenseTemplate(hissuancelicense: u32, puissuancelicensetemplatelength: *mut u32, wszissuancelicensetemplate: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMGetIssuanceLicenseTemplate(::core::mem::transmute(hissuancelicense), ::core::mem::transmute(puissuancelicensetemplatelength), ::core::mem::transmute(wszissuancelicensetemplate)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetMetaData(hissuancelicense: u32, pucontentidlength: *mut u32, wszcontentid: ::windows::core::PWSTR, pucontentidtypelength: *mut u32, wszcontentidtype: ::windows::core::PWSTR, puskuidlength: *mut u32, wszskuid: ::windows::core::PWSTR, puskuidtypelength: *mut u32, wszskuidtype: ::windows::core::PWSTR, pucontenttypelength: *mut u32, wszcontenttype: ::windows::core::PWSTR, pucontentnamelength: *mut u32, wszcontentname: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetMetaData(hissuancelicense: u32, pucontentidlength: *mut u32, wszcontentid: ::windows::core::PWSTR, pucontentidtypelength: *mut u32, wszcontentidtype: ::windows::core::PWSTR, puskuidlength: *mut u32, wszskuid: ::windows::core::PWSTR, puskuidtypelength: *mut u32, wszskuidtype: ::windows::core::PWSTR, pucontenttypelength: *mut u32, wszcontenttype: ::windows::core::PWSTR, pucontentnamelength: *mut u32, wszcontentname: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMGetMetaData(
            ::core::mem::transmute(hissuancelicense),
            ::core::mem::transmute(pucontentidlength),
            ::core::mem::transmute(wszcontentid),
            ::core::mem::transmute(pucontentidtypelength),
            ::core::mem::transmute(wszcontentidtype),
            ::core::mem::transmute(puskuidlength),
            ::core::mem::transmute(wszskuid),
            ::core::mem::transmute(puskuidtypelength),
            ::core::mem::transmute(wszskuidtype),
            ::core::mem::transmute(pucontenttypelength),
            ::core::mem::transmute(wszcontenttype),
            ::core::mem::transmute(pucontentnamelength),
            ::core::mem::transmute(wszcontentname),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetNameAndDescription(hissuancelicense: u32, uindex: u32, pulcid: *mut u32, punamelength: *mut u32, wszname: ::windows::core::PWSTR, pudescriptionlength: *mut u32, wszdescription: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetNameAndDescription(hissuancelicense: u32, uindex: u32, pulcid: *mut u32, punamelength: *mut u32, wszname: ::windows::core::PWSTR, pudescriptionlength: *mut u32, wszdescription: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMGetNameAndDescription(::core::mem::transmute(hissuancelicense), ::core::mem::transmute(uindex), ::core::mem::transmute(pulcid), ::core::mem::transmute(punamelength), ::core::mem::transmute(wszname), ::core::mem::transmute(pudescriptionlength), ::core::mem::transmute(wszdescription)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetOwnerLicense(hissuancelicense: u32, puownerlicenselength: *mut u32, wszownerlicense: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetOwnerLicense(hissuancelicense: u32, puownerlicenselength: *mut u32, wszownerlicense: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMGetOwnerLicense(::core::mem::transmute(hissuancelicense), ::core::mem::transmute(puownerlicenselength), ::core::mem::transmute(wszownerlicense)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetProcAddress<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hlibrary: u32, wszprocname: Param1, ppfnprocaddress: *mut super::super::Foundation::FARPROC) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetProcAddress(hlibrary: u32, wszprocname: ::windows::core::PCWSTR, ppfnprocaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        DRMGetProcAddress(::core::mem::transmute(hlibrary), wszprocname.into_param().abi(), ::core::mem::transmute(ppfnprocaddress)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetRevocationPoint(hissuancelicense: u32, puidlength: *mut u32, wszid: ::windows::core::PWSTR, puidtypelength: *mut u32, wszidtype: ::windows::core::PWSTR, puurllength: *mut u32, wszrl: ::windows::core::PWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, punamelength: *mut u32, wszname: ::windows::core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetRevocationPoint(hissuancelicense: u32, puidlength: *mut u32, wszid: ::windows::core::PWSTR, puidtypelength: *mut u32, wszidtype: ::windows::core::PWSTR, puurllength: *mut u32, wszrl: ::windows::core::PWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, punamelength: *mut u32, wszname: ::windows::core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMGetRevocationPoint(::core::mem::transmute(hissuancelicense), ::core::mem::transmute(puidlength), ::core::mem::transmute(wszid), ::core::mem::transmute(puidtypelength), ::core::mem::transmute(wszidtype), ::core::mem::transmute(puurllength), ::core::mem::transmute(wszrl), ::core::mem::transmute(pstfrequency), ::core::mem::transmute(punamelength), ::core::mem::transmute(wszname), ::core::mem::transmute(pupublickeylength), ::core::mem::transmute(wszpublickey)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetRightExtendedInfo(hright: u32, uindex: u32, puextendedinfonamelength: *mut u32, wszextendedinfoname: ::windows::core::PWSTR, puextendedinfovaluelength: *mut u32, wszextendedinfovalue: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetRightExtendedInfo(hright: u32, uindex: u32, puextendedinfonamelength: *mut u32, wszextendedinfoname: ::windows::core::PWSTR, puextendedinfovaluelength: *mut u32, wszextendedinfovalue: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMGetRightExtendedInfo(::core::mem::transmute(hright), ::core::mem::transmute(uindex), ::core::mem::transmute(puextendedinfonamelength), ::core::mem::transmute(wszextendedinfoname), ::core::mem::transmute(puextendedinfovaluelength), ::core::mem::transmute(wszextendedinfovalue)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetRightInfo(hright: u32, purightnamelength: *mut u32, wszrightname: ::windows::core::PWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetRightInfo(hright: u32, purightnamelength: *mut u32, wszrightname: ::windows::core::PWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT;
        }
        DRMGetRightInfo(::core::mem::transmute(hright), ::core::mem::transmute(purightnamelength), ::core::mem::transmute(wszrightname), ::core::mem::transmute(pstfrom), ::core::mem::transmute(pstuntil)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetSecurityProvider(uflags: u32, putypelen: *mut u32, wsztype: ::windows::core::PWSTR, pupathlen: *mut u32, wszpath: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetSecurityProvider(uflags: u32, putypelen: *mut u32, wsztype: ::windows::core::PWSTR, pupathlen: *mut u32, wszpath: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMGetSecurityProvider(::core::mem::transmute(uflags), ::core::mem::transmute(putypelen), ::core::mem::transmute(wsztype), ::core::mem::transmute(pupathlen), ::core::mem::transmute(wszpath)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetServiceLocation<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hclient: u32, uservicetype: u32, uservicelocation: u32, wszissuancelicense: Param3, puserviceurllength: *mut u32, wszserviceurl: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetServiceLocation(hclient: u32, uservicetype: u32, uservicelocation: u32, wszissuancelicense: ::windows::core::PCWSTR, puserviceurllength: *mut u32, wszserviceurl: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMGetServiceLocation(::core::mem::transmute(hclient), ::core::mem::transmute(uservicetype), ::core::mem::transmute(uservicelocation), wszissuancelicense.into_param().abi(), ::core::mem::transmute(puserviceurllength), ::core::mem::transmute(wszserviceurl)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicense<'a, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param8: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *mut u8, cbsymkey: u32, wszsymkeytype: Param5, wszclientlicensorcertificate: Param6, pfncallback: DRMCALLBACK, wszurl: Param8, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetSignedIssuanceLicense(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *mut u8, cbsymkey: u32, wszsymkeytype: ::windows::core::PCWSTR, wszclientlicensorcertificate: ::windows::core::PCWSTR, pfncallback: ::windows::core::RawPtr, wszurl: ::windows::core::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DRMGetSignedIssuanceLicense(::core::mem::transmute(henv), ::core::mem::transmute(hissuancelicense), ::core::mem::transmute(uflags), ::core::mem::transmute(pbsymkey), ::core::mem::transmute(cbsymkey), wszsymkeytype.into_param().abi(), wszclientlicensorcertificate.into_param().abi(), ::core::mem::transmute(pfncallback), wszurl.into_param().abi(), ::core::mem::transmute(pvcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicenseEx<'a, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *const u8, cbsymkey: u32, wszsymkeytype: Param5, pvreserved: *const ::core::ffi::c_void, henablingprincipal: u32, hboundlicenseclc: u32, pfncallback: DRMCALLBACK, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetSignedIssuanceLicenseEx(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *const u8, cbsymkey: u32, wszsymkeytype: ::windows::core::PCWSTR, pvreserved: *const ::core::ffi::c_void, henablingprincipal: u32, hboundlicenseclc: u32, pfncallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DRMGetSignedIssuanceLicenseEx(::core::mem::transmute(henv), ::core::mem::transmute(hissuancelicense), ::core::mem::transmute(uflags), ::core::mem::transmute(pbsymkey), ::core::mem::transmute(cbsymkey), wszsymkeytype.into_param().abi(), ::core::mem::transmute(pvreserved), ::core::mem::transmute(henablingprincipal), ::core::mem::transmute(hboundlicenseclc), ::core::mem::transmute(pfncallback), ::core::mem::transmute(pvcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetTime(henv: u32, etimeridtype: DRMTIMETYPE, potimeobject: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetTime(henv: u32, etimeridtype: DRMTIMETYPE, potimeobject: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT;
        }
        DRMGetTime(::core::mem::transmute(henv), ::core::mem::transmute(etimeridtype), ::core::mem::transmute(potimeobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttribute<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hqueryroot: u32, wszattributetype: Param1, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetUnboundLicenseAttribute(hqueryroot: u32, wszattributetype: ::windows::core::PCWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT;
        }
        DRMGetUnboundLicenseAttribute(::core::mem::transmute(hqueryroot), wszattributetype.into_param().abi(), ::core::mem::transmute(iwhich), ::core::mem::transmute(peencoding), ::core::mem::transmute(pcbuffer), ::core::mem::transmute(pbbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttributeCount<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hqueryroot: u32, wszattributetype: Param1, pcattributes: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetUnboundLicenseAttributeCount(hqueryroot: u32, wszattributetype: ::windows::core::PCWSTR, pcattributes: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMGetUnboundLicenseAttributeCount(::core::mem::transmute(hqueryroot), wszattributetype.into_param().abi(), ::core::mem::transmute(pcattributes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUnboundLicenseObject<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hqueryroot: u32, wszsubobjecttype: Param1, iindex: u32, phsubquery: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetUnboundLicenseObject(hqueryroot: u32, wszsubobjecttype: ::windows::core::PCWSTR, iindex: u32, phsubquery: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMGetUnboundLicenseObject(::core::mem::transmute(hqueryroot), wszsubobjecttype.into_param().abi(), ::core::mem::transmute(iindex), ::core::mem::transmute(phsubquery)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUnboundLicenseObjectCount<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hqueryroot: u32, wszsubobjecttype: Param1, pcsubobjects: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetUnboundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: ::windows::core::PCWSTR, pcsubobjects: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMGetUnboundLicenseObjectCount(::core::mem::transmute(hqueryroot), wszsubobjecttype.into_param().abi(), ::core::mem::transmute(pcsubobjects)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetUsagePolicy(hissuancelicense: u32, uindex: u32, peusagepolicytype: *mut DRM_USAGEPOLICY_TYPE, pfexclusion: *mut super::super::Foundation::BOOL, punamelength: *mut u32, wszname: ::windows::core::PWSTR, puminversionlength: *mut u32, wszminversion: ::windows::core::PWSTR, pumaxversionlength: *mut u32, wszmaxversion: ::windows::core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows::core::PWSTR, pudigestalgorithmlength: *mut u32, wszdigestalgorithm: ::windows::core::PWSTR, pcbdigest: *mut u32, pbdigest: *mut u8) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetUsagePolicy(hissuancelicense: u32, uindex: u32, peusagepolicytype: *mut DRM_USAGEPOLICY_TYPE, pfexclusion: *mut super::super::Foundation::BOOL, punamelength: *mut u32, wszname: ::windows::core::PWSTR, puminversionlength: *mut u32, wszminversion: ::windows::core::PWSTR, pumaxversionlength: *mut u32, wszmaxversion: ::windows::core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows::core::PWSTR, pudigestalgorithmlength: *mut u32, wszdigestalgorithm: ::windows::core::PWSTR, pcbdigest: *mut u32, pbdigest: *mut u8) -> ::windows::core::HRESULT;
        }
        DRMGetUsagePolicy(
            ::core::mem::transmute(hissuancelicense),
            ::core::mem::transmute(uindex),
            ::core::mem::transmute(peusagepolicytype),
            ::core::mem::transmute(pfexclusion),
            ::core::mem::transmute(punamelength),
            ::core::mem::transmute(wszname),
            ::core::mem::transmute(puminversionlength),
            ::core::mem::transmute(wszminversion),
            ::core::mem::transmute(pumaxversionlength),
            ::core::mem::transmute(wszmaxversion),
            ::core::mem::transmute(pupublickeylength),
            ::core::mem::transmute(wszpublickey),
            ::core::mem::transmute(pudigestalgorithmlength),
            ::core::mem::transmute(wszdigestalgorithm),
            ::core::mem::transmute(pcbdigest),
            ::core::mem::transmute(pbdigest),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUserInfo(huser: u32, puusernamelength: *mut u32, wszusername: ::windows::core::PWSTR, puuseridlength: *mut u32, wszuserid: ::windows::core::PWSTR, puuseridtypelength: *mut u32, wszuseridtype: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetUserInfo(huser: u32, puusernamelength: *mut u32, wszusername: ::windows::core::PWSTR, puuseridlength: *mut u32, wszuserid: ::windows::core::PWSTR, puuseridtypelength: *mut u32, wszuseridtype: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMGetUserInfo(::core::mem::transmute(huser), ::core::mem::transmute(puusernamelength), ::core::mem::transmute(wszusername), ::core::mem::transmute(puuseridlength), ::core::mem::transmute(wszuserid), ::core::mem::transmute(puuseridtypelength), ::core::mem::transmute(wszuseridtype)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUserRights(hissuancelicense: u32, huser: u32, uindex: u32, phright: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetUserRights(hissuancelicense: u32, huser: u32, uindex: u32, phright: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMGetUserRights(::core::mem::transmute(hissuancelicense), ::core::mem::transmute(huser), ::core::mem::transmute(uindex), ::core::mem::transmute(phright)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUsers(hissuancelicense: u32, uindex: u32, phuser: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMGetUsers(hissuancelicense: u32, uindex: u32, phuser: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMGetUsers(::core::mem::transmute(hissuancelicense), ::core::mem::transmute(uindex), ::core::mem::transmute(phuser)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMHSESSION_INVALID: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub struct DRMID {
    pub uVersion: u32,
    pub wszIDType: ::windows::core::PWSTR,
    pub wszID: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DRMID {}
impl ::core::clone::Clone for DRMID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRMID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRMID").field("uVersion", &self.uVersion).field("wszIDType", &self.wszIDType).field("wszID", &self.wszID).finish()
    }
}
unsafe impl ::windows::core::Abi for DRMID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRMID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRMID>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRMID {}
impl ::core::default::Default for DRMID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMIDVERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMInitEnvironment<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(esecurityprovidertype: DRMSECURITYPROVIDERTYPE, especification: DRMSPECTYPE, wszsecurityprovider: Param2, wszmanifestcredentials: Param3, wszmachinecredentials: Param4, phenv: *mut u32, phdefaultlibrary: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMInitEnvironment(esecurityprovidertype: DRMSECURITYPROVIDERTYPE, especification: DRMSPECTYPE, wszsecurityprovider: ::windows::core::PCWSTR, wszmanifestcredentials: ::windows::core::PCWSTR, wszmachinecredentials: ::windows::core::PCWSTR, phenv: *mut u32, phdefaultlibrary: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMInitEnvironment(::core::mem::transmute(esecurityprovidertype), ::core::mem::transmute(especification), wszsecurityprovider.into_param().abi(), wszmanifestcredentials.into_param().abi(), wszmachinecredentials.into_param().abi(), ::core::mem::transmute(phenv), ::core::mem::transmute(phdefaultlibrary)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMIsActivated(hclient: u32, uflags: u32, pactservinfo: *mut DRM_ACTSERV_INFO) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMIsActivated(hclient: u32, uflags: u32, pactservinfo: *mut DRM_ACTSERV_INFO) -> ::windows::core::HRESULT;
        }
        DRMIsActivated(::core::mem::transmute(hclient), ::core::mem::transmute(uflags), ::core::mem::transmute(pactservinfo)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMIsWindowProtected<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(hwnd: Param0, pfprotected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMIsWindowProtected(hwnd: super::super::Foundation::HWND, pfprotected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        DRMIsWindowProtected(hwnd.into_param().abi(), ::core::mem::transmute(pfprotected)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMLICENSEACQDATAVERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMLoadLibrary<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(henv: u32, especification: DRMSPECTYPE, wszlibraryprovider: Param2, wszcredentials: Param3, phlibrary: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMLoadLibrary(henv: u32, especification: DRMSPECTYPE, wszlibraryprovider: ::windows::core::PCWSTR, wszcredentials: ::windows::core::PCWSTR, phlibrary: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMLoadLibrary(::core::mem::transmute(henv), ::core::mem::transmute(especification), wszlibraryprovider.into_param().abi(), wszcredentials.into_param().abi(), ::core::mem::transmute(phlibrary)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMPUBHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMParseUnboundLicense<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(wszcertificate: Param0, phqueryroot: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMParseUnboundLicense(wszcertificate: ::windows::core::PCWSTR, phqueryroot: *mut u32) -> ::windows::core::HRESULT;
        }
        DRMParseUnboundLicense(wszcertificate.into_param().abi(), ::core::mem::transmute(phqueryroot)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMQUERYHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMRegisterContent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(fregister: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMRegisterContent(fregister: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        DRMRegisterContent(fregister.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMRegisterProtectedWindow<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(henv: u32, hwnd: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMRegisterProtectedWindow(henv: u32, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
        }
        DRMRegisterProtectedWindow(::core::mem::transmute(henv), hwnd.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMRegisterRevocationList<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(henv: u32, wszrevocationlist: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMRegisterRevocationList(henv: u32, wszrevocationlist: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        DRMRegisterRevocationList(::core::mem::transmute(henv), wszrevocationlist.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMRepair() -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMRepair() -> ::windows::core::HRESULT;
        }
        DRMRepair().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRMSECURITYPROVIDERTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMSECURITYPROVIDERTYPE_SOFTWARESECREP: DRMSECURITYPROVIDERTYPE = DRMSECURITYPROVIDERTYPE(0i32);
impl ::core::marker::Copy for DRMSECURITYPROVIDERTYPE {}
impl ::core::clone::Clone for DRMSECURITYPROVIDERTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRMSECURITYPROVIDERTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRMSECURITYPROVIDERTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRMSECURITYPROVIDERTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMSECURITYPROVIDERTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRMSPECTYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMSPECTYPE_UNKNOWN: DRMSPECTYPE = DRMSPECTYPE(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMSPECTYPE_FILENAME: DRMSPECTYPE = DRMSPECTYPE(1i32);
impl ::core::marker::Copy for DRMSPECTYPE {}
impl ::core::clone::Clone for DRMSPECTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRMSPECTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRMSPECTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRMSPECTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMSPECTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMSetApplicationSpecificData<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hissuancelicense: u32, fdelete: Param1, wszname: Param2, wszvalue: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMSetApplicationSpecificData(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, wszname: ::windows::core::PCWSTR, wszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        DRMSetApplicationSpecificData(::core::mem::transmute(hissuancelicense), fdelete.into_param().abi(), wszname.into_param().abi(), wszvalue.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMSetGlobalOptions(eglobaloptions: DRMGLOBALOPTIONS, pvdata: *mut ::core::ffi::c_void, dwlen: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMSetGlobalOptions(eglobaloptions: DRMGLOBALOPTIONS, pvdata: *mut ::core::ffi::c_void, dwlen: u32) -> ::windows::core::HRESULT;
        }
        DRMSetGlobalOptions(::core::mem::transmute(eglobaloptions), ::core::mem::transmute(pvdata), ::core::mem::transmute(dwlen)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMSetIntervalTime(hissuancelicense: u32, cdays: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMSetIntervalTime(hissuancelicense: u32, cdays: u32) -> ::windows::core::HRESULT;
        }
        DRMSetIntervalTime(::core::mem::transmute(hissuancelicense), ::core::mem::transmute(cdays)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMSetMetaData<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hissuancelicense: u32, wszcontentid: Param1, wszcontentidtype: Param2, wszskuid: Param3, wszskuidtype: Param4, wszcontenttype: Param5, wszcontentname: Param6) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMSetMetaData(hissuancelicense: u32, wszcontentid: ::windows::core::PCWSTR, wszcontentidtype: ::windows::core::PCWSTR, wszskuid: ::windows::core::PCWSTR, wszskuidtype: ::windows::core::PCWSTR, wszcontenttype: ::windows::core::PCWSTR, wszcontentname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        DRMSetMetaData(::core::mem::transmute(hissuancelicense), wszcontentid.into_param().abi(), wszcontentidtype.into_param().abi(), wszskuid.into_param().abi(), wszskuidtype.into_param().abi(), wszcontenttype.into_param().abi(), wszcontentname.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMSetNameAndDescription<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hissuancelicense: u32, fdelete: Param1, lcid: u32, wszname: Param3, wszdescription: Param4) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMSetNameAndDescription(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, lcid: u32, wszname: ::windows::core::PCWSTR, wszdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        DRMSetNameAndDescription(::core::mem::transmute(hissuancelicense), fdelete.into_param().abi(), ::core::mem::transmute(lcid), wszname.into_param().abi(), wszdescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMSetRevocationPoint<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param7: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hissuancelicense: u32, fdelete: Param1, wszid: Param2, wszidtype: Param3, wszurl: Param4, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, wszname: Param6, wszpublickey: Param7) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMSetRevocationPoint(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, wszid: ::windows::core::PCWSTR, wszidtype: ::windows::core::PCWSTR, wszurl: ::windows::core::PCWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, wszname: ::windows::core::PCWSTR, wszpublickey: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        DRMSetRevocationPoint(::core::mem::transmute(hissuancelicense), fdelete.into_param().abi(), wszid.into_param().abi(), wszidtype.into_param().abi(), wszurl.into_param().abi(), ::core::mem::transmute(pstfrequency), wszname.into_param().abi(), wszpublickey.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMSetUsagePolicy<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param7: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param8: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hissuancelicense: u32, eusagepolicytype: DRM_USAGEPOLICY_TYPE, fdelete: Param2, fexclusion: Param3, wszname: Param4, wszminversion: Param5, wszmaxversion: Param6, wszpublickey: Param7, wszdigestalgorithm: Param8, pbdigest: *mut u8, cbdigest: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMSetUsagePolicy(hissuancelicense: u32, eusagepolicytype: DRM_USAGEPOLICY_TYPE, fdelete: super::super::Foundation::BOOL, fexclusion: super::super::Foundation::BOOL, wszname: ::windows::core::PCWSTR, wszminversion: ::windows::core::PCWSTR, wszmaxversion: ::windows::core::PCWSTR, wszpublickey: ::windows::core::PCWSTR, wszdigestalgorithm: ::windows::core::PCWSTR, pbdigest: *mut u8, cbdigest: u32) -> ::windows::core::HRESULT;
        }
        DRMSetUsagePolicy(::core::mem::transmute(hissuancelicense), ::core::mem::transmute(eusagepolicytype), fdelete.into_param().abi(), fexclusion.into_param().abi(), wszname.into_param().abi(), wszminversion.into_param().abi(), wszmaxversion.into_param().abi(), wszpublickey.into_param().abi(), wszdigestalgorithm.into_param().abi(), ::core::mem::transmute(pbdigest), ::core::mem::transmute(cbdigest)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRMTIMETYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMTIMETYPE_SYSTEMUTC: DRMTIMETYPE = DRMTIMETYPE(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMTIMETYPE_SYSTEMLOCAL: DRMTIMETYPE = DRMTIMETYPE(1i32);
impl ::core::marker::Copy for DRMTIMETYPE {}
impl ::core::clone::Clone for DRMTIMETYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRMTIMETYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRMTIMETYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRMTIMETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRMTIMETYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMVerify<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(wszdata: Param0, pcattesteddata: *mut u32, wszattesteddata: ::windows::core::PWSTR, petype: *mut DRMATTESTTYPE, pcprincipal: *mut u32, wszprincipal: ::windows::core::PWSTR, pcmanifest: *mut u32, wszmanifest: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DRMVerify(wszdata: ::windows::core::PCWSTR, pcattesteddata: *mut u32, wszattesteddata: ::windows::core::PWSTR, petype: *mut DRMATTESTTYPE, pcprincipal: *mut u32, wszprincipal: ::windows::core::PWSTR, pcmanifest: *mut u32, wszmanifest: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        DRMVerify(wszdata.into_param().abi(), ::core::mem::transmute(pcattesteddata), ::core::mem::transmute(wszattesteddata), ::core::mem::transmute(petype), ::core::mem::transmute(pcprincipal), ::core::mem::transmute(wszprincipal), ::core::mem::transmute(pcmanifest), ::core::mem::transmute(wszmanifest)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_CANCEL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_DELAYED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_GROUPIDENTITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_MACHINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_SHARED_GROUPIDENTITY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_SILENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ACTIVATE_TEMPORARY: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub struct DRM_ACTSERV_INFO {
    pub uVersion: u32,
    pub wszPubKey: ::windows::core::PWSTR,
    pub wszURL: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DRM_ACTSERV_INFO {}
impl ::core::clone::Clone for DRM_ACTSERV_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_ACTSERV_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_ACTSERV_INFO").field("uVersion", &self.uVersion).field("wszPubKey", &self.wszPubKey).field("wszURL", &self.wszURL).finish()
    }
}
unsafe impl ::windows::core::Abi for DRM_ACTSERV_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_ACTSERV_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_ACTSERV_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_ACTSERV_INFO {}
impl ::core::default::Default for DRM_ACTSERV_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ADD_LICENSE_NOPERSIST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_ADD_LICENSE_PERSIST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AILT_CANCEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AILT_NONSILENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AILT_OBTAIN_ALL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AL_CANCEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AL_FETCHNOADVISORY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AL_NONSILENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AL_NOPERSIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AL_NOUI: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_AUTO_GENERATE_KEY: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub struct DRM_CLIENT_VERSION_INFO {
    pub uStructVersion: u32,
    pub dwVersion: [u32; 4],
    pub wszHierarchy: [u16; 256],
    pub wszProductId: [u16; 256],
    pub wszProductDescription: [u16; 256],
}
impl ::core::marker::Copy for DRM_CLIENT_VERSION_INFO {}
impl ::core::clone::Clone for DRM_CLIENT_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_CLIENT_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_CLIENT_VERSION_INFO").field("uStructVersion", &self.uStructVersion).field("dwVersion", &self.dwVersion).field("wszHierarchy", &self.wszHierarchy).field("wszProductId", &self.wszProductId).field("wszProductDescription", &self.wszProductDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for DRM_CLIENT_VERSION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_CLIENT_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_CLIENT_VERSION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_CLIENT_VERSION_INFO {}
impl ::core::default::Default for DRM_CLIENT_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_DEFAULTGROUPIDTYPE_PASSPORT: &'static str = "PassportAuthProvider";
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_DEFAULTGROUPIDTYPE_WINDOWSAUTH: &'static str = "WindowsAuthProvider";
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRM_DISTRIBUTION_POINT_INFO(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_DISTRIBUTION_POINT_LICENSE_ACQUISITION: DRM_DISTRIBUTION_POINT_INFO = DRM_DISTRIBUTION_POINT_INFO(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_DISTRIBUTION_POINT_PUBLISHING: DRM_DISTRIBUTION_POINT_INFO = DRM_DISTRIBUTION_POINT_INFO(1i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_DISTRIBUTION_POINT_REFERRAL_INFO: DRM_DISTRIBUTION_POINT_INFO = DRM_DISTRIBUTION_POINT_INFO(2i32);
impl ::core::marker::Copy for DRM_DISTRIBUTION_POINT_INFO {}
impl ::core::clone::Clone for DRM_DISTRIBUTION_POINT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRM_DISTRIBUTION_POINT_INFO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRM_DISTRIBUTION_POINT_INFO {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRM_DISTRIBUTION_POINT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRM_DISTRIBUTION_POINT_INFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_CLIENTLICENSOR: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_CLIENTLICENSOR_LID: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_EUL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_EUL_LID: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_EXPIRED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_GROUPIDENTITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_GROUPIDENTITY_LID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_GROUPIDENTITY_NAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE_LID: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_ISSUERNAME: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_MACHINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_REVOCATIONLIST: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_REVOCATIONLIST_LID: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_SPECIFIED_CLIENTLICENSOR: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_EL_SPECIFIED_GROUPIDENTITY: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub struct DRM_LICENSE_ACQ_DATA {
    pub uVersion: u32,
    pub wszURL: ::windows::core::PWSTR,
    pub wszLocalFilename: ::windows::core::PWSTR,
    pub pbPostData: *mut u8,
    pub dwPostDataSize: u32,
    pub wszFriendlyName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DRM_LICENSE_ACQ_DATA {}
impl ::core::clone::Clone for DRM_LICENSE_ACQ_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DRM_LICENSE_ACQ_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRM_LICENSE_ACQ_DATA").field("uVersion", &self.uVersion).field("wszURL", &self.wszURL).field("wszLocalFilename", &self.wszLocalFilename).field("pbPostData", &self.pbPostData).field("dwPostDataSize", &self.dwPostDataSize).field("wszFriendlyName", &self.wszFriendlyName).finish()
    }
}
unsafe impl ::windows::core::Abi for DRM_LICENSE_ACQ_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DRM_LICENSE_ACQ_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DRM_LICENSE_ACQ_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DRM_LICENSE_ACQ_DATA {}
impl ::core::default::Default for DRM_LICENSE_ACQ_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_LOCKBOXTYPE_BLACKBOX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_LOCKBOXTYPE_DEFAULT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_LOCKBOXTYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_LOCKBOXTYPE_WHITEBOX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_OWNER_LICENSE_NOPERSIST: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_REUSE_KEY: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVER_ISSUANCELICENSE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_LOCATION_ENTERPRISE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_LOCATION_INTERNET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_TYPE_ACTIVATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_TYPE_CERTIFICATION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_TYPE_CLIENTLICENSOR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_TYPE_PUBLISHING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SERVICE_TYPE_SILENT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SIGN_CANCEL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SIGN_OFFLINE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_SIGN_ONLINE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRM_STATUS_MSG(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_ACTIVATE_MACHINE: DRM_STATUS_MSG = DRM_STATUS_MSG(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_ACTIVATE_GROUPIDENTITY: DRM_STATUS_MSG = DRM_STATUS_MSG(1i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_ACQUIRE_LICENSE: DRM_STATUS_MSG = DRM_STATUS_MSG(2i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_ACQUIRE_ADVISORY: DRM_STATUS_MSG = DRM_STATUS_MSG(3i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_SIGN_ISSUANCE_LICENSE: DRM_STATUS_MSG = DRM_STATUS_MSG(4i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_ACQUIRE_CLIENTLICENSOR: DRM_STATUS_MSG = DRM_STATUS_MSG(5i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_MSG_ACQUIRE_ISSUANCE_LICENSE_TEMPLATE: DRM_STATUS_MSG = DRM_STATUS_MSG(6i32);
impl ::core::marker::Copy for DRM_STATUS_MSG {}
impl ::core::clone::Clone for DRM_STATUS_MSG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRM_STATUS_MSG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRM_STATUS_MSG {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRM_STATUS_MSG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRM_STATUS_MSG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DRM_USAGEPOLICY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_USAGEPOLICY_TYPE_BYNAME: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_USAGEPOLICY_TYPE_BYPUBLICKEY: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_USAGEPOLICY_TYPE_BYDIGEST: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_USAGEPOLICY_TYPE_OSEXCLUSION: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(3i32);
impl ::core::marker::Copy for DRM_USAGEPOLICY_TYPE {}
impl ::core::clone::Clone for DRM_USAGEPOLICY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DRM_USAGEPOLICY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DRM_USAGEPOLICY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DRM_USAGEPOLICY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DRM_USAGEPOLICY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const MSDRM_CLIENT_ZONE: u32 = 52992u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const MSDRM_POLICY_ZONE: u32 = 37632u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
