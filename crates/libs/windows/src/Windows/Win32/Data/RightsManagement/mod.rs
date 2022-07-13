#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMACTSERVINFOVERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
pub unsafe fn DRMAcquireAdvisories<'a, P0, P1>(hlicensestorage: u32, wszlicense: P0, wszurl: P1, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMAcquireAdvisories(hlicensestorage: u32, wszlicense: ::windows::core::PCWSTR, wszurl: ::windows::core::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DRMAcquireAdvisories(hlicensestorage, wszlicense.into(), wszurl.into(), ::core::mem::transmute(pvcontext)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAcquireIssuanceLicenseTemplate<'a, P0>(hclient: u32, uflags: u32, pvreserved: *mut ::core::ffi::c_void, pwsztemplateids: &[::windows::core::PWSTR], wszurl: P0, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMAcquireIssuanceLicenseTemplate(hclient: u32, uflags: u32, pvreserved: *mut ::core::ffi::c_void, ctemplates: u32, pwsztemplateids: *const ::windows::core::PWSTR, wszurl: ::windows::core::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DRMAcquireIssuanceLicenseTemplate(hclient, uflags, ::core::mem::transmute(pvreserved), pwsztemplateids.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pwsztemplateids)), wszurl.into(), ::core::mem::transmute(pvcontext)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAcquireLicense<'a, P0, P1, P2, P3>(hsession: u32, uflags: u32, wszgroupidentitycredential: P0, wszrequestedrights: P1, wszcustomdata: P2, wszurl: P3, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMAcquireLicense(hsession: u32, uflags: u32, wszgroupidentitycredential: ::windows::core::PCWSTR, wszrequestedrights: ::windows::core::PCWSTR, wszcustomdata: ::windows::core::PCWSTR, wszurl: ::windows::core::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DRMAcquireLicense(hsession, uflags, wszgroupidentitycredential.into(), wszrequestedrights.into(), wszcustomdata.into(), wszurl.into(), ::core::mem::transmute(pvcontext)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMActivate<'a, P0>(hclient: u32, uflags: u32, ulangid: u32, pactservinfo: *mut DRM_ACTSERV_INFO, pvcontext: *mut ::core::ffi::c_void, hparentwnd: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMActivate(hclient: u32, uflags: u32, ulangid: u32, pactservinfo: *mut DRM_ACTSERV_INFO, pvcontext: *mut ::core::ffi::c_void, hparentwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
    }
    DRMActivate(hclient, uflags, ulangid, ::core::mem::transmute(pactservinfo), ::core::mem::transmute(pvcontext), hparentwnd.into()).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAddLicense<'a, P0>(hlicensestorage: u32, uflags: u32, wszlicense: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMAddLicense(hlicensestorage: u32, uflags: u32, wszlicense: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    DRMAddLicense(hlicensestorage, uflags, wszlicense.into()).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAddRightWithUser(hissuancelicense: u32, hright: u32, huser: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMAddRightWithUser(hissuancelicense: u32, hright: u32, huser: u32) -> ::windows::core::HRESULT;
    }
    DRMAddRightWithUser(hissuancelicense, hright, huser).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMAttest<'a, P0>(henablingprincipal: u32, wszdata: P0, etype: DRMATTESTTYPE, pcattestedblob: *mut u32, wszattestedblob: ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMAttest(henablingprincipal: u32, wszdata: ::windows::core::PCWSTR, etype: DRMATTESTTYPE, pcattestedblob: *mut u32, wszattestedblob: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMAttest(henablingprincipal, wszdata.into(), etype, ::core::mem::transmute(pcattestedblob), ::core::mem::transmute(wszattestedblob)).ok()
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
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCheckSecurity(henv: u32, clevel: u32) -> ::windows::core::HRESULT;
    }
    DRMCheckSecurity(henv, clevel).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMClearAllRights(hissuancelicense: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMClearAllRights(hissuancelicense: u32) -> ::windows::core::HRESULT;
    }
    DRMClearAllRights(hissuancelicense).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCloseEnvironmentHandle(henv: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCloseEnvironmentHandle(henv: u32) -> ::windows::core::HRESULT;
    }
    DRMCloseEnvironmentHandle(henv).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCloseHandle(handle: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCloseHandle(handle: u32) -> ::windows::core::HRESULT;
    }
    DRMCloseHandle(handle).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMClosePubHandle(hpub: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMClosePubHandle(hpub: u32) -> ::windows::core::HRESULT;
    }
    DRMClosePubHandle(hpub).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCloseQueryHandle(hquery: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCloseQueryHandle(hquery: u32) -> ::windows::core::HRESULT;
    }
    DRMCloseQueryHandle(hquery).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCloseSession(hsession: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCloseSession(hsession: u32) -> ::windows::core::HRESULT;
    }
    DRMCloseSession(hsession).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMConstructCertificateChain(rgwszcertificates: &[::windows::core::PWSTR], pcchain: *mut u32, wszchain: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMConstructCertificateChain(ccertificates: u32, rgwszcertificates: *const ::windows::core::PWSTR, pcchain: *mut u32, wszchain: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMConstructCertificateChain(rgwszcertificates.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(rgwszcertificates)), ::core::mem::transmute(pcchain), ::core::mem::transmute(wszchain)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateBoundLicense<'a, P0>(henv: u32, pparams: *mut DRMBOUNDLICENSEPARAMS, wszlicensechain: P0, phboundlicense: *mut u32, pherrorlog: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateBoundLicense(henv: u32, pparams: *mut DRMBOUNDLICENSEPARAMS, wszlicensechain: ::windows::core::PCWSTR, phboundlicense: *mut u32, pherrorlog: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateBoundLicense(henv, ::core::mem::transmute(pparams), wszlicensechain.into(), ::core::mem::transmute(phboundlicense), ::core::mem::transmute(pherrorlog)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateClientSession<'a, P0, P1>(pfncallback: DRMCALLBACK, ucallbackversion: u32, wszgroupidprovidertype: P0, wszgroupid: P1, phclient: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateClientSession(pfncallback: *mut ::core::ffi::c_void, ucallbackversion: u32, wszgroupidprovidertype: ::windows::core::PCWSTR, wszgroupid: ::windows::core::PCWSTR, phclient: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateClientSession(::core::mem::transmute(pfncallback), ucallbackversion, wszgroupidprovidertype.into(), wszgroupid.into(), ::core::mem::transmute(phclient)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateEnablingBitsDecryptor<'a, P0, P1>(hboundlicense: u32, wszright: P0, hauxlib: u32, wszauxplug: P1, phdecryptor: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateEnablingBitsDecryptor(hboundlicense: u32, wszright: ::windows::core::PCWSTR, hauxlib: u32, wszauxplug: ::windows::core::PCWSTR, phdecryptor: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateEnablingBitsDecryptor(hboundlicense, wszright.into(), hauxlib, wszauxplug.into(), ::core::mem::transmute(phdecryptor)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateEnablingBitsEncryptor<'a, P0, P1>(hboundlicense: u32, wszright: P0, hauxlib: u32, wszauxplug: P1, phencryptor: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateEnablingBitsEncryptor(hboundlicense: u32, wszright: ::windows::core::PCWSTR, hauxlib: u32, wszauxplug: ::windows::core::PCWSTR, phencryptor: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateEnablingBitsEncryptor(hboundlicense, wszright.into(), hauxlib, wszauxplug.into(), ::core::mem::transmute(phencryptor)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateEnablingPrincipal<'a, P0, P1>(henv: u32, hlibrary: u32, wszobject: P0, pidprincipal: *mut DRMID, wszcredentials: P1, phenablingprincipal: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateEnablingPrincipal(henv: u32, hlibrary: u32, wszobject: ::windows::core::PCWSTR, pidprincipal: *mut DRMID, wszcredentials: ::windows::core::PCWSTR, phenablingprincipal: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateEnablingPrincipal(henv, hlibrary, wszobject.into(), ::core::mem::transmute(pidprincipal), wszcredentials.into(), ::core::mem::transmute(phenablingprincipal)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMCreateIssuanceLicense<'a, P0, P1, P2>(psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, wszreferralinfoname: P0, wszreferralinfourl: P1, howner: u32, wszissuancelicense: P2, hboundlicense: u32, phissuancelicense: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateIssuanceLicense(psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, wszreferralinfoname: ::windows::core::PCWSTR, wszreferralinfourl: ::windows::core::PCWSTR, howner: u32, wszissuancelicense: ::windows::core::PCWSTR, hboundlicense: u32, phissuancelicense: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateIssuanceLicense(::core::mem::transmute(psttimefrom), ::core::mem::transmute(psttimeuntil), wszreferralinfoname.into(), wszreferralinfourl.into(), howner, wszissuancelicense.into(), hboundlicense, ::core::mem::transmute(phissuancelicense)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateLicenseStorageSession<'a, P0>(henv: u32, hdefaultlibrary: u32, hclient: u32, uflags: u32, wszissuancelicense: P0, phlicensestorage: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateLicenseStorageSession(henv: u32, hdefaultlibrary: u32, hclient: u32, uflags: u32, wszissuancelicense: ::windows::core::PCWSTR, phlicensestorage: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateLicenseStorageSession(henv, hdefaultlibrary, hclient, uflags, wszissuancelicense.into(), ::core::mem::transmute(phlicensestorage)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMCreateRight<'a, P0>(wszrightname: P0, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME, cextendedinfo: u32, pwszextendedinfoname: *const ::windows::core::PWSTR, pwszextendedinfovalue: *const ::windows::core::PWSTR, phright: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateRight(wszrightname: ::windows::core::PCWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME, cextendedinfo: u32, pwszextendedinfoname: *const ::windows::core::PWSTR, pwszextendedinfovalue: *const ::windows::core::PWSTR, phright: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateRight(wszrightname.into(), ::core::mem::transmute(pstfrom), ::core::mem::transmute(pstuntil), ::core::mem::transmute(cextendedinfo), ::core::mem::transmute(pwszextendedinfoname), ::core::mem::transmute(pwszextendedinfovalue), ::core::mem::transmute(phright)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMCreateUser<'a, P0, P1, P2>(wszusername: P0, wszuserid: P1, wszuseridtype: P2, phuser: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMCreateUser(wszusername: ::windows::core::PCWSTR, wszuserid: ::windows::core::PCWSTR, wszuseridtype: ::windows::core::PCWSTR, phuser: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMCreateUser(wszusername.into(), wszuserid.into(), wszuseridtype.into(), ::core::mem::transmute(phuser)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDecode<'a, P0, P1>(wszalgid: P0, wszencodedstring: P1, pudecodeddatalen: *mut u32, pbdecodeddata: *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDecode(wszalgid: ::windows::core::PCWSTR, wszencodedstring: ::windows::core::PCWSTR, pudecodeddatalen: *mut u32, pbdecodeddata: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMDecode(wszalgid.into(), wszencodedstring.into(), ::core::mem::transmute(pudecodeddatalen), ::core::mem::transmute(pbdecodeddata)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDeconstructCertificateChain<'a, P0>(wszchain: P0, iwhich: u32, pccert: *mut u32, wszcert: ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDeconstructCertificateChain(wszchain: ::windows::core::PCWSTR, iwhich: u32, pccert: *mut u32, wszcert: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMDeconstructCertificateChain(wszchain.into(), iwhich, ::core::mem::transmute(pccert), ::core::mem::transmute(wszcert)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDecrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDecrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMDecrypt(hcryptoprovider, iposition, cnuminbytes, ::core::mem::transmute(pbindata), ::core::mem::transmute(pcnumoutbytes), ::core::mem::transmute(pboutdata)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDeleteLicense<'a, P0>(hsession: u32, wszlicenseid: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDeleteLicense(hsession: u32, wszlicenseid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    DRMDeleteLicense(hsession, wszlicenseid.into()).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDuplicateEnvironmentHandle(htocopy: u32, phcopy: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDuplicateEnvironmentHandle(htocopy: u32, phcopy: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMDuplicateEnvironmentHandle(htocopy, ::core::mem::transmute(phcopy)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDuplicateHandle(htocopy: u32, phcopy: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDuplicateHandle(htocopy: u32, phcopy: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMDuplicateHandle(htocopy, ::core::mem::transmute(phcopy)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDuplicatePubHandle(hpubin: u32, phpubout: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDuplicatePubHandle(hpubin: u32, phpubout: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMDuplicatePubHandle(hpubin, ::core::mem::transmute(phpubout)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMDuplicateSession(hsessionin: u32, phsessionout: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMDuplicateSession(hsessionin: u32, phsessionout: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMDuplicateSession(hsessionin, ::core::mem::transmute(phsessionout)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
pub unsafe fn DRMEncode<'a, P0>(wszalgid: P0, udatalen: u32, pbdecodeddata: *mut u8, puencodedstringlen: *mut u32, wszencodedstring: ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMEncode(wszalgid: ::windows::core::PCWSTR, udatalen: u32, pbdecodeddata: *mut u8, puencodedstringlen: *mut u32, wszencodedstring: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMEncode(wszalgid.into(), udatalen, ::core::mem::transmute(pbdecodeddata), ::core::mem::transmute(puencodedstringlen), ::core::mem::transmute(wszencodedstring)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMEncrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMEncrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMEncrypt(hcryptoprovider, iposition, cnuminbytes, ::core::mem::transmute(pbindata), ::core::mem::transmute(pcnumoutbytes), ::core::mem::transmute(pboutdata)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMEnumerateLicense(hsession: u32, uflags: u32, uindex: u32, pfsharedflag: *mut super::super::Foundation::BOOL, pucertificatedatalen: *mut u32, wszcertificatedata: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMEnumerateLicense(hsession: u32, uflags: u32, uindex: u32, pfsharedflag: *mut super::super::Foundation::BOOL, pucertificatedatalen: *mut u32, wszcertificatedata: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMEnumerateLicense(hsession, uflags, uindex, ::core::mem::transmute(pfsharedflag), ::core::mem::transmute(pucertificatedatalen), ::core::mem::transmute(wszcertificatedata)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetApplicationSpecificData(hissuancelicense: u32, uindex: u32, punamelength: *mut u32, wszname: ::windows::core::PWSTR, puvaluelength: *mut u32, wszvalue: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetApplicationSpecificData(hissuancelicense, uindex, ::core::mem::transmute(punamelength), ::core::mem::transmute(wszname), ::core::mem::transmute(puvaluelength), ::core::mem::transmute(wszvalue)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetBoundLicenseAttribute<'a, P0>(hqueryroot: u32, wszattribute: P0, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetBoundLicenseAttribute(hqueryroot: u32, wszattribute: ::windows::core::PCWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMGetBoundLicenseAttribute(hqueryroot, wszattribute.into(), iwhich, ::core::mem::transmute(peencoding), ::core::mem::transmute(pcbuffer), ::core::mem::transmute(pbbuffer)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetBoundLicenseAttributeCount<'a, P0>(hqueryroot: u32, wszattribute: P0, pcattributes: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetBoundLicenseAttributeCount(hqueryroot: u32, wszattribute: ::windows::core::PCWSTR, pcattributes: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetBoundLicenseAttributeCount(hqueryroot, wszattribute.into(), ::core::mem::transmute(pcattributes)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetBoundLicenseObject<'a, P0>(hqueryroot: u32, wszsubobjecttype: P0, iwhich: u32, phsubobject: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetBoundLicenseObject(hqueryroot: u32, wszsubobjecttype: ::windows::core::PCWSTR, iwhich: u32, phsubobject: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetBoundLicenseObject(hqueryroot, wszsubobjecttype.into(), iwhich, ::core::mem::transmute(phsubobject)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetBoundLicenseObjectCount<'a, P0>(hqueryroot: u32, wszsubobjecttype: P0, pcsubobjects: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetBoundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: ::windows::core::PCWSTR, pcsubobjects: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetBoundLicenseObjectCount(hqueryroot, wszsubobjecttype.into(), ::core::mem::transmute(pcsubobjects)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetCertificateChainCount<'a, P0>(wszchain: P0, pccertcount: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetCertificateChainCount(wszchain: ::windows::core::PCWSTR, pccertcount: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetCertificateChainCount(wszchain.into(), ::core::mem::transmute(pccertcount)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetClientVersion(pdrmclientversioninfo: *mut DRM_CLIENT_VERSION_INFO) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetClientVersion(pdrmclientversioninfo: *mut DRM_CLIENT_VERSION_INFO) -> ::windows::core::HRESULT;
    }
    DRMGetClientVersion(::core::mem::transmute(pdrmclientversioninfo)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetEnvironmentInfo<'a, P0>(handle: u32, wszattribute: P0, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetEnvironmentInfo(handle: u32, wszattribute: ::windows::core::PCWSTR, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMGetEnvironmentInfo(handle, wszattribute.into(), ::core::mem::transmute(peencoding), ::core::mem::transmute(pcbuffer), ::core::mem::transmute(pbbuffer)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetInfo<'a, P0>(handle: u32, wszattribute: P0, peencoding: *const DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetInfo(handle: u32, wszattribute: ::windows::core::PCWSTR, peencoding: *const DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMGetInfo(handle, wszattribute.into(), ::core::mem::transmute(peencoding), ::core::mem::transmute(pcbuffer), ::core::mem::transmute(pbbuffer)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetIntervalTime(hissuancelicense: u32, pcdays: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetIntervalTime(hissuancelicense: u32, pcdays: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetIntervalTime(hissuancelicense, ::core::mem::transmute(pcdays)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetIssuanceLicenseInfo(hissuancelicense: u32, psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, uflags: u32, pudistributionpointnamelength: *mut u32, wszdistributionpointname: ::windows::core::PWSTR, pudistributionpointurllength: *mut u32, wszdistributionpointurl: ::windows::core::PWSTR, phowner: *mut u32, pfofficial: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetIssuanceLicenseInfo(hissuancelicense: u32, psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, uflags: u32, pudistributionpointnamelength: *mut u32, wszdistributionpointname: ::windows::core::PWSTR, pudistributionpointurllength: *mut u32, wszdistributionpointurl: ::windows::core::PWSTR, phowner: *mut u32, pfofficial: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    DRMGetIssuanceLicenseInfo(hissuancelicense, ::core::mem::transmute(psttimefrom), ::core::mem::transmute(psttimeuntil), uflags, ::core::mem::transmute(pudistributionpointnamelength), ::core::mem::transmute(wszdistributionpointname), ::core::mem::transmute(pudistributionpointurllength), ::core::mem::transmute(wszdistributionpointurl), ::core::mem::transmute(phowner), ::core::mem::transmute(pfofficial)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetIssuanceLicenseTemplate(hissuancelicense: u32, puissuancelicensetemplatelength: *mut u32, wszissuancelicensetemplate: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetIssuanceLicenseTemplate(hissuancelicense: u32, puissuancelicensetemplatelength: *mut u32, wszissuancelicensetemplate: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetIssuanceLicenseTemplate(hissuancelicense, ::core::mem::transmute(puissuancelicensetemplatelength), ::core::mem::transmute(wszissuancelicensetemplate)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetMetaData(hissuancelicense: u32, pucontentidlength: *mut u32, wszcontentid: ::windows::core::PWSTR, pucontentidtypelength: *mut u32, wszcontentidtype: ::windows::core::PWSTR, puskuidlength: *mut u32, wszskuid: ::windows::core::PWSTR, puskuidtypelength: *mut u32, wszskuidtype: ::windows::core::PWSTR, pucontenttypelength: *mut u32, wszcontenttype: ::windows::core::PWSTR, pucontentnamelength: *mut u32, wszcontentname: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetMetaData(hissuancelicense: u32, pucontentidlength: *mut u32, wszcontentid: ::windows::core::PWSTR, pucontentidtypelength: *mut u32, wszcontentidtype: ::windows::core::PWSTR, puskuidlength: *mut u32, wszskuid: ::windows::core::PWSTR, puskuidtypelength: *mut u32, wszskuidtype: ::windows::core::PWSTR, pucontenttypelength: *mut u32, wszcontenttype: ::windows::core::PWSTR, pucontentnamelength: *mut u32, wszcontentname: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetMetaData(
        hissuancelicense,
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
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetNameAndDescription(hissuancelicense: u32, uindex: u32, pulcid: *mut u32, punamelength: *mut u32, wszname: ::windows::core::PWSTR, pudescriptionlength: *mut u32, wszdescription: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetNameAndDescription(hissuancelicense: u32, uindex: u32, pulcid: *mut u32, punamelength: *mut u32, wszname: ::windows::core::PWSTR, pudescriptionlength: *mut u32, wszdescription: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetNameAndDescription(hissuancelicense, uindex, ::core::mem::transmute(pulcid), ::core::mem::transmute(punamelength), ::core::mem::transmute(wszname), ::core::mem::transmute(pudescriptionlength), ::core::mem::transmute(wszdescription)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetOwnerLicense(hissuancelicense: u32, puownerlicenselength: *mut u32, wszownerlicense: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetOwnerLicense(hissuancelicense: u32, puownerlicenselength: *mut u32, wszownerlicense: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetOwnerLicense(hissuancelicense, ::core::mem::transmute(puownerlicenselength), ::core::mem::transmute(wszownerlicense)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetProcAddress<'a, P0>(hlibrary: u32, wszprocname: P0, ppfnprocaddress: *mut super::super::Foundation::FARPROC) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetProcAddress(hlibrary: u32, wszprocname: ::windows::core::PCWSTR, ppfnprocaddress: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DRMGetProcAddress(hlibrary, wszprocname.into(), ::core::mem::transmute(ppfnprocaddress)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetRevocationPoint(hissuancelicense: u32, puidlength: *mut u32, wszid: ::windows::core::PWSTR, puidtypelength: *mut u32, wszidtype: ::windows::core::PWSTR, puurllength: *mut u32, wszrl: ::windows::core::PWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, punamelength: *mut u32, wszname: ::windows::core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetRevocationPoint(hissuancelicense: u32, puidlength: *mut u32, wszid: ::windows::core::PWSTR, puidtypelength: *mut u32, wszidtype: ::windows::core::PWSTR, puurllength: *mut u32, wszrl: ::windows::core::PWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, punamelength: *mut u32, wszname: ::windows::core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetRevocationPoint(hissuancelicense, ::core::mem::transmute(puidlength), ::core::mem::transmute(wszid), ::core::mem::transmute(puidtypelength), ::core::mem::transmute(wszidtype), ::core::mem::transmute(puurllength), ::core::mem::transmute(wszrl), ::core::mem::transmute(pstfrequency), ::core::mem::transmute(punamelength), ::core::mem::transmute(wszname), ::core::mem::transmute(pupublickeylength), ::core::mem::transmute(wszpublickey)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetRightExtendedInfo(hright: u32, uindex: u32, puextendedinfonamelength: *mut u32, wszextendedinfoname: ::windows::core::PWSTR, puextendedinfovaluelength: *mut u32, wszextendedinfovalue: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetRightExtendedInfo(hright: u32, uindex: u32, puextendedinfonamelength: *mut u32, wszextendedinfoname: ::windows::core::PWSTR, puextendedinfovaluelength: *mut u32, wszextendedinfovalue: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetRightExtendedInfo(hright, uindex, ::core::mem::transmute(puextendedinfonamelength), ::core::mem::transmute(wszextendedinfoname), ::core::mem::transmute(puextendedinfovaluelength), ::core::mem::transmute(wszextendedinfovalue)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetRightInfo(hright: u32, purightnamelength: *mut u32, wszrightname: ::windows::core::PWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetRightInfo(hright: u32, purightnamelength: *mut u32, wszrightname: ::windows::core::PWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT;
    }
    DRMGetRightInfo(hright, ::core::mem::transmute(purightnamelength), ::core::mem::transmute(wszrightname), ::core::mem::transmute(pstfrom), ::core::mem::transmute(pstuntil)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetSecurityProvider(uflags: u32, putypelen: *mut u32, wsztype: ::windows::core::PWSTR, pupathlen: *mut u32, wszpath: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetSecurityProvider(uflags: u32, putypelen: *mut u32, wsztype: ::windows::core::PWSTR, pupathlen: *mut u32, wszpath: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetSecurityProvider(uflags, ::core::mem::transmute(putypelen), ::core::mem::transmute(wsztype), ::core::mem::transmute(pupathlen), ::core::mem::transmute(wszpath)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetServiceLocation<'a, P0>(hclient: u32, uservicetype: u32, uservicelocation: u32, wszissuancelicense: P0, puserviceurllength: *mut u32, wszserviceurl: ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetServiceLocation(hclient: u32, uservicetype: u32, uservicelocation: u32, wszissuancelicense: ::windows::core::PCWSTR, puserviceurllength: *mut u32, wszserviceurl: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetServiceLocation(hclient, uservicetype, uservicelocation, wszissuancelicense.into(), ::core::mem::transmute(puserviceurllength), ::core::mem::transmute(wszserviceurl)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicense<'a, P0, P1, P2>(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *mut u8, cbsymkey: u32, wszsymkeytype: P0, wszclientlicensorcertificate: P1, pfncallback: DRMCALLBACK, wszurl: P2, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetSignedIssuanceLicense(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *mut u8, cbsymkey: u32, wszsymkeytype: ::windows::core::PCWSTR, wszclientlicensorcertificate: ::windows::core::PCWSTR, pfncallback: *mut ::core::ffi::c_void, wszurl: ::windows::core::PCWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DRMGetSignedIssuanceLicense(henv, hissuancelicense, uflags, ::core::mem::transmute(pbsymkey), cbsymkey, wszsymkeytype.into(), wszclientlicensorcertificate.into(), ::core::mem::transmute(pfncallback), wszurl.into(), ::core::mem::transmute(pvcontext)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicenseEx<'a, P0>(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *const u8, cbsymkey: u32, wszsymkeytype: P0, pvreserved: *const ::core::ffi::c_void, henablingprincipal: u32, hboundlicenseclc: u32, pfncallback: DRMCALLBACK, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetSignedIssuanceLicenseEx(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *const u8, cbsymkey: u32, wszsymkeytype: ::windows::core::PCWSTR, pvreserved: *const ::core::ffi::c_void, henablingprincipal: u32, hboundlicenseclc: u32, pfncallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    DRMGetSignedIssuanceLicenseEx(henv, hissuancelicense, uflags, ::core::mem::transmute(pbsymkey), cbsymkey, wszsymkeytype.into(), ::core::mem::transmute(pvreserved), henablingprincipal, hboundlicenseclc, ::core::mem::transmute(pfncallback), ::core::mem::transmute(pvcontext)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetTime(henv: u32, etimeridtype: DRMTIMETYPE, potimeobject: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetTime(henv: u32, etimeridtype: DRMTIMETYPE, potimeobject: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT;
    }
    DRMGetTime(henv, etimeridtype, ::core::mem::transmute(potimeobject)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttribute<'a, P0>(hqueryroot: u32, wszattributetype: P0, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUnboundLicenseAttribute(hqueryroot: u32, wszattributetype: ::windows::core::PCWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMGetUnboundLicenseAttribute(hqueryroot, wszattributetype.into(), iwhich, ::core::mem::transmute(peencoding), ::core::mem::transmute(pcbuffer), ::core::mem::transmute(pbbuffer)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttributeCount<'a, P0>(hqueryroot: u32, wszattributetype: P0, pcattributes: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUnboundLicenseAttributeCount(hqueryroot: u32, wszattributetype: ::windows::core::PCWSTR, pcattributes: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetUnboundLicenseAttributeCount(hqueryroot, wszattributetype.into(), ::core::mem::transmute(pcattributes)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUnboundLicenseObject<'a, P0>(hqueryroot: u32, wszsubobjecttype: P0, iindex: u32, phsubquery: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUnboundLicenseObject(hqueryroot: u32, wszsubobjecttype: ::windows::core::PCWSTR, iindex: u32, phsubquery: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetUnboundLicenseObject(hqueryroot, wszsubobjecttype.into(), iindex, ::core::mem::transmute(phsubquery)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUnboundLicenseObjectCount<'a, P0>(hqueryroot: u32, wszsubobjecttype: P0, pcsubobjects: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUnboundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: ::windows::core::PCWSTR, pcsubobjects: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetUnboundLicenseObjectCount(hqueryroot, wszsubobjecttype.into(), ::core::mem::transmute(pcsubobjects)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMGetUsagePolicy(hissuancelicense: u32, uindex: u32, peusagepolicytype: *mut DRM_USAGEPOLICY_TYPE, pfexclusion: *mut super::super::Foundation::BOOL, punamelength: *mut u32, wszname: ::windows::core::PWSTR, puminversionlength: *mut u32, wszminversion: ::windows::core::PWSTR, pumaxversionlength: *mut u32, wszmaxversion: ::windows::core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows::core::PWSTR, pudigestalgorithmlength: *mut u32, wszdigestalgorithm: ::windows::core::PWSTR, pcbdigest: *mut u32, pbdigest: *mut u8) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUsagePolicy(hissuancelicense: u32, uindex: u32, peusagepolicytype: *mut DRM_USAGEPOLICY_TYPE, pfexclusion: *mut super::super::Foundation::BOOL, punamelength: *mut u32, wszname: ::windows::core::PWSTR, puminversionlength: *mut u32, wszminversion: ::windows::core::PWSTR, pumaxversionlength: *mut u32, wszmaxversion: ::windows::core::PWSTR, pupublickeylength: *mut u32, wszpublickey: ::windows::core::PWSTR, pudigestalgorithmlength: *mut u32, wszdigestalgorithm: ::windows::core::PWSTR, pcbdigest: *mut u32, pbdigest: *mut u8) -> ::windows::core::HRESULT;
    }
    DRMGetUsagePolicy(
        hissuancelicense,
        uindex,
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
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUserInfo(huser: u32, puusernamelength: *mut u32, wszusername: ::windows::core::PWSTR, puuseridlength: *mut u32, wszuserid: ::windows::core::PWSTR, puuseridtypelength: *mut u32, wszuseridtype: ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUserInfo(huser: u32, puusernamelength: *mut u32, wszusername: ::windows::core::PWSTR, puuseridlength: *mut u32, wszuserid: ::windows::core::PWSTR, puuseridtypelength: *mut u32, wszuseridtype: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMGetUserInfo(huser, ::core::mem::transmute(puusernamelength), ::core::mem::transmute(wszusername), ::core::mem::transmute(puuseridlength), ::core::mem::transmute(wszuserid), ::core::mem::transmute(puuseridtypelength), ::core::mem::transmute(wszuseridtype)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUserRights(hissuancelicense: u32, huser: u32, uindex: u32, phright: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUserRights(hissuancelicense: u32, huser: u32, uindex: u32, phright: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetUserRights(hissuancelicense, huser, uindex, ::core::mem::transmute(phright)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMGetUsers(hissuancelicense: u32, uindex: u32, phuser: *mut u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMGetUsers(hissuancelicense: u32, uindex: u32, phuser: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMGetUsers(hissuancelicense, uindex, ::core::mem::transmute(phuser)).ok()
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
pub unsafe fn DRMInitEnvironment<'a, P0, P1, P2>(esecurityprovidertype: DRMSECURITYPROVIDERTYPE, especification: DRMSPECTYPE, wszsecurityprovider: P0, wszmanifestcredentials: P1, wszmachinecredentials: P2, phenv: *mut u32, phdefaultlibrary: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMInitEnvironment(esecurityprovidertype: DRMSECURITYPROVIDERTYPE, especification: DRMSPECTYPE, wszsecurityprovider: ::windows::core::PCWSTR, wszmanifestcredentials: ::windows::core::PCWSTR, wszmachinecredentials: ::windows::core::PCWSTR, phenv: *mut u32, phdefaultlibrary: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMInitEnvironment(esecurityprovidertype, especification, wszsecurityprovider.into(), wszmanifestcredentials.into(), wszmachinecredentials.into(), ::core::mem::transmute(phenv), ::core::mem::transmute(phdefaultlibrary)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMIsActivated(hclient: u32, uflags: u32, pactservinfo: *mut DRM_ACTSERV_INFO) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMIsActivated(hclient: u32, uflags: u32, pactservinfo: *mut DRM_ACTSERV_INFO) -> ::windows::core::HRESULT;
    }
    DRMIsActivated(hclient, uflags, ::core::mem::transmute(pactservinfo)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMIsWindowProtected<'a, P0>(hwnd: P0, pfprotected: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMIsWindowProtected(hwnd: super::super::Foundation::HWND, pfprotected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    DRMIsWindowProtected(hwnd.into(), ::core::mem::transmute(pfprotected)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMLICENSEACQDATAVERSION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMLoadLibrary<'a, P0, P1>(henv: u32, especification: DRMSPECTYPE, wszlibraryprovider: P0, wszcredentials: P1, phlibrary: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMLoadLibrary(henv: u32, especification: DRMSPECTYPE, wszlibraryprovider: ::windows::core::PCWSTR, wszcredentials: ::windows::core::PCWSTR, phlibrary: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMLoadLibrary(henv, especification, wszlibraryprovider.into(), wszcredentials.into(), ::core::mem::transmute(phlibrary)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMPUBHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMParseUnboundLicense<'a, P0>(wszcertificate: P0, phqueryroot: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMParseUnboundLicense(wszcertificate: ::windows::core::PCWSTR, phqueryroot: *mut u32) -> ::windows::core::HRESULT;
    }
    DRMParseUnboundLicense(wszcertificate.into(), ::core::mem::transmute(phqueryroot)).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRMQUERYHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMRegisterContent<'a, P0>(fregister: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMRegisterContent(fregister: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
    }
    DRMRegisterContent(fregister.into()).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMRegisterProtectedWindow<'a, P0>(henv: u32, hwnd: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMRegisterProtectedWindow(henv: u32, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
    }
    DRMRegisterProtectedWindow(henv, hwnd.into()).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMRegisterRevocationList<'a, P0>(henv: u32, wszrevocationlist: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMRegisterRevocationList(henv: u32, wszrevocationlist: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    DRMRegisterRevocationList(henv, wszrevocationlist.into()).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMRepair() -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMRepair() -> ::windows::core::HRESULT;
    }
    DRMRepair().ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
pub unsafe fn DRMSetApplicationSpecificData<'a, P0, P1, P2>(hissuancelicense: u32, fdelete: P0, wszname: P1, wszvalue: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetApplicationSpecificData(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, wszname: ::windows::core::PCWSTR, wszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    DRMSetApplicationSpecificData(hissuancelicense, fdelete.into(), wszname.into(), wszvalue.into()).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMSetGlobalOptions(eglobaloptions: DRMGLOBALOPTIONS, pvdata: *mut ::core::ffi::c_void, dwlen: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetGlobalOptions(eglobaloptions: DRMGLOBALOPTIONS, pvdata: *mut ::core::ffi::c_void, dwlen: u32) -> ::windows::core::HRESULT;
    }
    DRMSetGlobalOptions(eglobaloptions, ::core::mem::transmute(pvdata), dwlen).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMSetIntervalTime(hissuancelicense: u32, cdays: u32) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetIntervalTime(hissuancelicense: u32, cdays: u32) -> ::windows::core::HRESULT;
    }
    DRMSetIntervalTime(hissuancelicense, cdays).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[inline]
pub unsafe fn DRMSetMetaData<'a, P0, P1, P2, P3, P4, P5>(hissuancelicense: u32, wszcontentid: P0, wszcontentidtype: P1, wszskuid: P2, wszskuidtype: P3, wszcontenttype: P4, wszcontentname: P5) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
    P4: ::std::convert::Into<::windows::core::PCWSTR>,
    P5: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetMetaData(hissuancelicense: u32, wszcontentid: ::windows::core::PCWSTR, wszcontentidtype: ::windows::core::PCWSTR, wszskuid: ::windows::core::PCWSTR, wszskuidtype: ::windows::core::PCWSTR, wszcontenttype: ::windows::core::PCWSTR, wszcontentname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    DRMSetMetaData(hissuancelicense, wszcontentid.into(), wszcontentidtype.into(), wszskuid.into(), wszskuidtype.into(), wszcontenttype.into(), wszcontentname.into()).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMSetNameAndDescription<'a, P0, P1, P2>(hissuancelicense: u32, fdelete: P0, lcid: u32, wszname: P1, wszdescription: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetNameAndDescription(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, lcid: u32, wszname: ::windows::core::PCWSTR, wszdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    DRMSetNameAndDescription(hissuancelicense, fdelete.into(), lcid, wszname.into(), wszdescription.into()).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMSetRevocationPoint<'a, P0, P1, P2, P3, P4, P5>(hissuancelicense: u32, fdelete: P0, wszid: P1, wszidtype: P2, wszurl: P3, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, wszname: P4, wszpublickey: P5) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
    P4: ::std::convert::Into<::windows::core::PCWSTR>,
    P5: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetRevocationPoint(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, wszid: ::windows::core::PCWSTR, wszidtype: ::windows::core::PCWSTR, wszurl: ::windows::core::PCWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, wszname: ::windows::core::PCWSTR, wszpublickey: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    DRMSetRevocationPoint(hissuancelicense, fdelete.into(), wszid.into(), wszidtype.into(), wszurl.into(), ::core::mem::transmute(pstfrequency), wszname.into(), wszpublickey.into()).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DRMSetUsagePolicy<'a, P0, P1, P2, P3, P4, P5, P6>(hissuancelicense: u32, eusagepolicytype: DRM_USAGEPOLICY_TYPE, fdelete: P0, fexclusion: P1, wszname: P2, wszminversion: P3, wszmaxversion: P4, wszpublickey: P5, wszdigestalgorithm: P6, pbdigest: *mut u8, cbdigest: u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
    P4: ::std::convert::Into<::windows::core::PCWSTR>,
    P5: ::std::convert::Into<::windows::core::PCWSTR>,
    P6: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMSetUsagePolicy(hissuancelicense: u32, eusagepolicytype: DRM_USAGEPOLICY_TYPE, fdelete: super::super::Foundation::BOOL, fexclusion: super::super::Foundation::BOOL, wszname: ::windows::core::PCWSTR, wszminversion: ::windows::core::PCWSTR, wszmaxversion: ::windows::core::PCWSTR, wszpublickey: ::windows::core::PCWSTR, wszdigestalgorithm: ::windows::core::PCWSTR, pbdigest: *mut u8, cbdigest: u32) -> ::windows::core::HRESULT;
    }
    DRMSetUsagePolicy(hissuancelicense, eusagepolicytype, fdelete.into(), fexclusion.into(), wszname.into(), wszminversion.into(), wszmaxversion.into(), wszpublickey.into(), wszdigestalgorithm.into(), ::core::mem::transmute(pbdigest), cbdigest).ok()
}
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
pub unsafe fn DRMVerify<'a, P0>(wszdata: P0, pcattesteddata: *mut u32, wszattesteddata: ::windows::core::PWSTR, petype: *mut DRMATTESTTYPE, pcprincipal: *mut u32, wszprincipal: ::windows::core::PWSTR, pcmanifest: *mut u32, wszmanifest: ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DRMVerify(wszdata: ::windows::core::PCWSTR, pcattesteddata: *mut u32, wszattesteddata: ::windows::core::PWSTR, petype: *mut DRMATTESTTYPE, pcprincipal: *mut u32, wszprincipal: ::windows::core::PWSTR, pcmanifest: *mut u32, wszmanifest: ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    DRMVerify(wszdata.into(), ::core::mem::transmute(pcattesteddata), ::core::mem::transmute(wszattesteddata), ::core::mem::transmute(petype), ::core::mem::transmute(pcprincipal), ::core::mem::transmute(wszprincipal), ::core::mem::transmute(pcmanifest), ::core::mem::transmute(wszmanifest)).ok()
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
pub const DRM_DEFAULTGROUPIDTYPE_PASSPORT: &str = "PassportAuthProvider";
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
pub const DRM_DEFAULTGROUPIDTYPE_WINDOWSAUTH: &str = "WindowsAuthProvider";
#[doc = "*Required features: `\"Win32_Data_RightsManagement\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
