#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMAcquireAdvisories<P1, P2>(hlicensestorage: super::msdrmdefs::DRMHSESSION, wszlicense: P1, wszurl: P2, pvcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMAcquireAdvisories(hlicensestorage : super::msdrmdefs::DRMHSESSION, wszlicense : windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DRMAcquireAdvisories(hlicensestorage, wszlicense.param().abi(), wszurl.param().abi(), pvcontext as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMAcquireIssuanceLicenseTemplate<P5>(hclient: super::msdrmdefs::DRMHSESSION, uflags: u32, pvreserved: *mut core::ffi::c_void, pwsztemplateids: Option<&[windows_core::PCWSTR]>, wszurl: P5, pvcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMAcquireIssuanceLicenseTemplate(hclient : super::msdrmdefs::DRMHSESSION, uflags : u32, pvreserved : *mut core::ffi::c_void, ctemplates : u32, pwsztemplateids : *const windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DRMAcquireIssuanceLicenseTemplate(hclient, uflags, pvreserved as _, pwsztemplateids.map_or(0, |slice| slice.len().try_into().unwrap()), pwsztemplateids.map_or(core::ptr::null(), |slice| slice.as_ptr()), wszurl.param().abi(), pvcontext as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMAcquireLicense<P2, P3, P4, P5>(hsession: super::msdrmdefs::DRMHSESSION, uflags: u32, wszgroupidentitycredential: P2, wszrequestedrights: P3, wszcustomdata: P4, wszurl: P5, pvcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMAcquireLicense(hsession : super::msdrmdefs::DRMHSESSION, uflags : u32, wszgroupidentitycredential : windows_core::PCWSTR, wszrequestedrights : windows_core::PCWSTR, wszcustomdata : windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DRMAcquireLicense(hsession, uflags, wszgroupidentitycredential.param().abi(), wszrequestedrights.param().abi(), wszcustomdata.param().abi(), wszurl.param().abi(), pvcontext as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "windef"))]
#[inline]
pub unsafe fn DRMActivate(hclient: super::msdrmdefs::DRMHSESSION, uflags: u32, ulangid: u32, pactservinfo: *mut super::msdrmdefs::DRM_ACTSERV_INFO, pvcontext: *mut core::ffi::c_void, hparentwnd: super::windef::HWND) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMActivate(hclient : super::msdrmdefs::DRMHSESSION, uflags : u32, ulangid : u32, pactservinfo : *mut super::msdrmdefs::DRM_ACTSERV_INFO, pvcontext : *mut core::ffi::c_void, hparentwnd : super::windef::HWND) -> windows_core::HRESULT);
    unsafe { DRMActivate(hclient, uflags, ulangid, pactservinfo as _, pvcontext as _, hparentwnd) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMAddLicense<P2>(hlicensestorage: super::msdrmdefs::DRMHSESSION, uflags: u32, wszlicense: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMAddLicense(hlicensestorage : super::msdrmdefs::DRMHSESSION, uflags : u32, wszlicense : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMAddLicense(hlicensestorage, uflags, wszlicense.param().abi()) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMAddRightWithUser(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, hright: super::msdrmdefs::DRMPUBHANDLE, huser: super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMAddRightWithUser(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, hright : super::msdrmdefs::DRMPUBHANDLE, huser : super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMAddRightWithUser(hissuancelicense, hright, huser) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMAttest<P1>(henablingprincipal: super::msdrmdefs::DRMHANDLE, wszdata: P1, etype: super::msdrmdefs::DRMATTESTTYPE, pcattestedblob: *mut u32, wszattestedblob: windows_core::PWSTR) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMAttest(henablingprincipal : super::msdrmdefs::DRMHANDLE, wszdata : windows_core::PCWSTR, etype : super::msdrmdefs::DRMATTESTTYPE, pcattestedblob : *mut u32, wszattestedblob : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMAttest(henablingprincipal, wszdata.param().abi(), etype, pcattestedblob as _, wszattestedblob) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCheckSecurity(henv: super::msdrmdefs::DRMENVHANDLE, clevel: u32) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMCheckSecurity(henv : super::msdrmdefs::DRMENVHANDLE, clevel : u32) -> windows_core::HRESULT);
    unsafe { DRMCheckSecurity(henv, clevel) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMClearAllRights(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMClearAllRights(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMClearAllRights(hissuancelicense) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCloseEnvironmentHandle(henv: super::msdrmdefs::DRMENVHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMCloseEnvironmentHandle(henv : super::msdrmdefs::DRMENVHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCloseEnvironmentHandle(henv) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCloseHandle(handle: super::msdrmdefs::DRMHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMCloseHandle(handle : super::msdrmdefs::DRMHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCloseHandle(handle) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMClosePubHandle(hpub: super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMClosePubHandle(hpub : super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMClosePubHandle(hpub) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCloseQueryHandle(hquery: super::msdrmdefs::DRMQUERYHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMCloseQueryHandle(hquery : super::msdrmdefs::DRMQUERYHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCloseQueryHandle(hquery) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCloseSession(hsession: super::msdrmdefs::DRMHSESSION) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMCloseSession(hsession : super::msdrmdefs::DRMHSESSION) -> windows_core::HRESULT);
    unsafe { DRMCloseSession(hsession) }
}
#[inline]
pub unsafe fn DRMConstructCertificateChain(rgwszcertificates: &[windows_core::PCWSTR], pcchain: *mut u32, wszchain: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMConstructCertificateChain(ccertificates : u32, rgwszcertificates : *const windows_core::PCWSTR, pcchain : *mut u32, wszchain : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMConstructCertificateChain(rgwszcertificates.len().try_into().unwrap(), rgwszcertificates.as_ptr(), pcchain as _, wszchain.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCreateBoundLicense<P2>(henv: super::msdrmdefs::DRMENVHANDLE, pparams: *mut super::msdrmdefs::DRMBOUNDLICENSEPARAMS, wszlicensechain: P2, phboundlicense: *mut super::msdrmdefs::DRMHANDLE, pherrorlog: *mut super::msdrmdefs::DRMHANDLE) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateBoundLicense(henv : super::msdrmdefs::DRMENVHANDLE, pparams : *mut super::msdrmdefs::DRMBOUNDLICENSEPARAMS, wszlicensechain : windows_core::PCWSTR, phboundlicense : *mut super::msdrmdefs::DRMHANDLE, pherrorlog : *mut super::msdrmdefs::DRMHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCreateBoundLicense(henv, pparams as _, wszlicensechain.param().abi(), phboundlicense as _, pherrorlog as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCreateClientSession<P2, P3>(pfncallback: super::msdrmdefs::DRMCALLBACK, ucallbackversion: u32, wszgroupidprovidertype: P2, wszgroupid: P3) -> windows_core::Result<super::msdrmdefs::DRMHSESSION>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateClientSession(pfncallback : super::msdrmdefs::DRMCALLBACK, ucallbackversion : u32, wszgroupidprovidertype : windows_core::PCWSTR, wszgroupid : windows_core::PCWSTR, phclient : *mut super::msdrmdefs::DRMHSESSION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMCreateClientSession(pfncallback, ucallbackversion, wszgroupidprovidertype.param().abi(), wszgroupid.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCreateEnablingBitsDecryptor<P1, P3>(hboundlicense: super::msdrmdefs::DRMHANDLE, wszright: P1, hauxlib: super::msdrmdefs::DRMHANDLE, wszauxplug: P3) -> windows_core::Result<super::msdrmdefs::DRMHANDLE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsDecryptor(hboundlicense : super::msdrmdefs::DRMHANDLE, wszright : windows_core::PCWSTR, hauxlib : super::msdrmdefs::DRMHANDLE, wszauxplug : windows_core::PCWSTR, phdecryptor : *mut super::msdrmdefs::DRMHANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMCreateEnablingBitsDecryptor(hboundlicense, wszright.param().abi(), hauxlib, wszauxplug.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCreateEnablingBitsEncryptor<P1, P3>(hboundlicense: super::msdrmdefs::DRMHANDLE, wszright: P1, hauxlib: super::msdrmdefs::DRMHANDLE, wszauxplug: P3) -> windows_core::Result<super::msdrmdefs::DRMHANDLE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsEncryptor(hboundlicense : super::msdrmdefs::DRMHANDLE, wszright : windows_core::PCWSTR, hauxlib : super::msdrmdefs::DRMHANDLE, wszauxplug : windows_core::PCWSTR, phencryptor : *mut super::msdrmdefs::DRMHANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMCreateEnablingBitsEncryptor(hboundlicense, wszright.param().abi(), hauxlib, wszauxplug.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCreateEnablingPrincipal<P2, P4>(henv: super::msdrmdefs::DRMENVHANDLE, hlibrary: super::msdrmdefs::DRMHANDLE, wszobject: P2, pidprincipal: *mut super::msdrmdefs::DRMID, wszcredentials: P4, phenablingprincipal: *mut super::msdrmdefs::DRMHANDLE) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateEnablingPrincipal(henv : super::msdrmdefs::DRMENVHANDLE, hlibrary : super::msdrmdefs::DRMHANDLE, wszobject : windows_core::PCWSTR, pidprincipal : *mut super::msdrmdefs::DRMID, wszcredentials : windows_core::PCWSTR, phenablingprincipal : *mut super::msdrmdefs::DRMHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCreateEnablingPrincipal(henv, hlibrary, wszobject.param().abi(), pidprincipal as _, wszcredentials.param().abi(), phenablingprincipal as _) }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
#[inline]
pub unsafe fn DRMCreateIssuanceLicense<P2, P3, P5>(psttimefrom: *mut super::minwinbase::SYSTEMTIME, psttimeuntil: *mut super::minwinbase::SYSTEMTIME, wszreferralinfoname: P2, wszreferralinfourl: P3, howner: super::msdrmdefs::DRMPUBHANDLE, wszissuancelicense: P5, hboundlicense: super::msdrmdefs::DRMHANDLE, phissuancelicense: *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateIssuanceLicense(psttimefrom : *mut super::minwinbase::SYSTEMTIME, psttimeuntil : *mut super::minwinbase::SYSTEMTIME, wszreferralinfoname : windows_core::PCWSTR, wszreferralinfourl : windows_core::PCWSTR, howner : super::msdrmdefs::DRMPUBHANDLE, wszissuancelicense : windows_core::PCWSTR, hboundlicense : super::msdrmdefs::DRMHANDLE, phissuancelicense : *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCreateIssuanceLicense(psttimefrom as _, psttimeuntil as _, wszreferralinfoname.param().abi(), wszreferralinfourl.param().abi(), howner, wszissuancelicense.param().abi(), hboundlicense, phissuancelicense as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCreateLicenseStorageSession<P4>(henv: super::msdrmdefs::DRMENVHANDLE, hdefaultlibrary: super::msdrmdefs::DRMHANDLE, hclient: super::msdrmdefs::DRMHSESSION, uflags: u32, wszissuancelicense: P4) -> windows_core::Result<super::msdrmdefs::DRMHSESSION>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateLicenseStorageSession(henv : super::msdrmdefs::DRMENVHANDLE, hdefaultlibrary : super::msdrmdefs::DRMHANDLE, hclient : super::msdrmdefs::DRMHSESSION, uflags : u32, wszissuancelicense : windows_core::PCWSTR, phlicensestorage : *mut super::msdrmdefs::DRMHSESSION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMCreateLicenseStorageSession(henv, hdefaultlibrary, hclient, uflags, wszissuancelicense.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
#[inline]
pub unsafe fn DRMCreateRight<P0>(wszrightname: P0, pstfrom: *mut super::minwinbase::SYSTEMTIME, pstuntil: *mut super::minwinbase::SYSTEMTIME, cextendedinfo: u32, pwszextendedinfoname: Option<*const windows_core::PCWSTR>, pwszextendedinfovalue: Option<*const windows_core::PCWSTR>, phright: *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateRight(wszrightname : windows_core::PCWSTR, pstfrom : *mut super::minwinbase::SYSTEMTIME, pstuntil : *mut super::minwinbase::SYSTEMTIME, cextendedinfo : u32, pwszextendedinfoname : *const windows_core::PCWSTR, pwszextendedinfovalue : *const windows_core::PCWSTR, phright : *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCreateRight(wszrightname.param().abi(), pstfrom as _, pstuntil as _, cextendedinfo, pwszextendedinfoname.unwrap_or(core::mem::zeroed()) as _, pwszextendedinfovalue.unwrap_or(core::mem::zeroed()) as _, phright as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCreateUser<P0, P1, P2>(wszusername: P0, wszuserid: P1, wszuseridtype: P2) -> windows_core::Result<super::msdrmdefs::DRMPUBHANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateUser(wszusername : windows_core::PCWSTR, wszuserid : windows_core::PCWSTR, wszuseridtype : windows_core::PCWSTR, phuser : *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMCreateUser(wszusername.param().abi(), wszuserid.param().abi(), wszuseridtype.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DRMDecode<P0, P1>(wszalgid: P0, wszencodedstring: P1, pudecodeddatalen: *mut u32, pbdecodeddata: *mut u8) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMDecode(wszalgid : windows_core::PCWSTR, wszencodedstring : windows_core::PCWSTR, pudecodeddatalen : *mut u32, pbdecodeddata : *mut u8) -> windows_core::HRESULT);
    unsafe { DRMDecode(wszalgid.param().abi(), wszencodedstring.param().abi(), pudecodeddatalen as _, pbdecodeddata as _) }
}
#[inline]
pub unsafe fn DRMDeconstructCertificateChain<P0>(wszchain: P0, iwhich: u32, pccert: *mut u32, wszcert: Option<windows_core::PWSTR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMDeconstructCertificateChain(wszchain : windows_core::PCWSTR, iwhich : u32, pccert : *mut u32, wszcert : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMDeconstructCertificateChain(wszchain.param().abi(), iwhich, pccert as _, wszcert.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMDecrypt(hcryptoprovider: super::msdrmdefs::DRMHANDLE, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMDecrypt(hcryptoprovider : super::msdrmdefs::DRMHANDLE, iposition : u32, cnuminbytes : u32, pbindata : *mut u8, pcnumoutbytes : *mut u32, pboutdata : *mut u8) -> windows_core::HRESULT);
    unsafe { DRMDecrypt(hcryptoprovider, iposition, cnuminbytes, pbindata as _, pcnumoutbytes as _, pboutdata as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMDeleteLicense<P1>(hsession: super::msdrmdefs::DRMHSESSION, wszlicenseid: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMDeleteLicense(hsession : super::msdrmdefs::DRMHSESSION, wszlicenseid : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMDeleteLicense(hsession, wszlicenseid.param().abi()) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMDuplicateEnvironmentHandle(htocopy: super::msdrmdefs::DRMENVHANDLE) -> windows_core::Result<super::msdrmdefs::DRMENVHANDLE> {
    windows_core::link!("msdrm.dll" "system" fn DRMDuplicateEnvironmentHandle(htocopy : super::msdrmdefs::DRMENVHANDLE, phcopy : *mut super::msdrmdefs::DRMENVHANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMDuplicateEnvironmentHandle(htocopy, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMDuplicateHandle(htocopy: super::msdrmdefs::DRMHANDLE) -> windows_core::Result<super::msdrmdefs::DRMHANDLE> {
    windows_core::link!("msdrm.dll" "system" fn DRMDuplicateHandle(htocopy : super::msdrmdefs::DRMHANDLE, phcopy : *mut super::msdrmdefs::DRMHANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMDuplicateHandle(htocopy, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMDuplicatePubHandle(hpubin: super::msdrmdefs::DRMPUBHANDLE) -> windows_core::Result<super::msdrmdefs::DRMPUBHANDLE> {
    windows_core::link!("msdrm.dll" "system" fn DRMDuplicatePubHandle(hpubin : super::msdrmdefs::DRMPUBHANDLE, phpubout : *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMDuplicatePubHandle(hpubin, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMDuplicateSession(hsessionin: super::msdrmdefs::DRMHSESSION) -> windows_core::Result<super::msdrmdefs::DRMHSESSION> {
    windows_core::link!("msdrm.dll" "system" fn DRMDuplicateSession(hsessionin : super::msdrmdefs::DRMHSESSION, phsessionout : *mut super::msdrmdefs::DRMHSESSION) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMDuplicateSession(hsessionin, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DRMEncode<P0>(wszalgid: P0, udatalen: u32, pbdecodeddata: *mut u8, puencodedstringlen: *mut u32, wszencodedstring: Option<windows_core::PWSTR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMEncode(wszalgid : windows_core::PCWSTR, udatalen : u32, pbdecodeddata : *mut u8, puencodedstringlen : *mut u32, wszencodedstring : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMEncode(wszalgid.param().abi(), udatalen, pbdecodeddata as _, puencodedstringlen as _, wszencodedstring.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMEncrypt(hcryptoprovider: super::msdrmdefs::DRMHANDLE, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMEncrypt(hcryptoprovider : super::msdrmdefs::DRMHANDLE, iposition : u32, cnuminbytes : u32, pbindata : *mut u8, pcnumoutbytes : *mut u32, pboutdata : *mut u8) -> windows_core::HRESULT);
    unsafe { DRMEncrypt(hcryptoprovider, iposition, cnuminbytes, pbindata as _, pcnumoutbytes as _, pboutdata as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMEnumerateLicense(hsession: super::msdrmdefs::DRMHSESSION, uflags: u32, uindex: u32, pfsharedflag: *mut windows_core::BOOL, pucertificatedatalen: *mut u32, wszcertificatedata: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMEnumerateLicense(hsession : super::msdrmdefs::DRMHSESSION, uflags : u32, uindex : u32, pfsharedflag : *mut windows_core::BOOL, pucertificatedatalen : *mut u32, wszcertificatedata : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMEnumerateLicense(hsession, uflags, uindex, pfsharedflag as _, pucertificatedatalen as _, wszcertificatedata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetApplicationSpecificData(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, uindex: u32, punamelength: *mut u32, wszname: Option<windows_core::PWSTR>, puvaluelength: *mut u32, wszvalue: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetApplicationSpecificData(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, uindex : u32, punamelength : *mut u32, wszname : windows_core::PWSTR, puvaluelength : *mut u32, wszvalue : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetApplicationSpecificData(hissuancelicense, uindex, punamelength as _, wszname.unwrap_or(core::mem::zeroed()) as _, puvaluelength as _, wszvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetBoundLicenseAttribute<P1>(hqueryroot: super::msdrmdefs::DRMHANDLE, wszattribute: P1, iwhich: u32, peencoding: *mut super::msdrmdefs::DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttribute(hqueryroot : super::msdrmdefs::DRMHANDLE, wszattribute : windows_core::PCWSTR, iwhich : u32, peencoding : *mut super::msdrmdefs::DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_core::HRESULT);
    unsafe { DRMGetBoundLicenseAttribute(hqueryroot, wszattribute.param().abi(), iwhich, peencoding as _, pcbuffer as _, pbbuffer as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetBoundLicenseAttributeCount<P1>(hqueryroot: super::msdrmdefs::DRMHANDLE, wszattribute: P1) -> windows_core::Result<u32>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttributeCount(hqueryroot : super::msdrmdefs::DRMHANDLE, wszattribute : windows_core::PCWSTR, pcattributes : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMGetBoundLicenseAttributeCount(hqueryroot, wszattribute.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetBoundLicenseObject<P1>(hqueryroot: super::msdrmdefs::DRMHANDLE, wszsubobjecttype: P1, iwhich: u32) -> windows_core::Result<super::msdrmdefs::DRMHANDLE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObject(hqueryroot : super::msdrmdefs::DRMHANDLE, wszsubobjecttype : windows_core::PCWSTR, iwhich : u32, phsubobject : *mut super::msdrmdefs::DRMHANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMGetBoundLicenseObject(hqueryroot, wszsubobjecttype.param().abi(), iwhich, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetBoundLicenseObjectCount<P1>(hqueryroot: super::msdrmdefs::DRMHANDLE, wszsubobjecttype: P1) -> windows_core::Result<u32>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObjectCount(hqueryroot : super::msdrmdefs::DRMHANDLE, wszsubobjecttype : windows_core::PCWSTR, pcsubobjects : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMGetBoundLicenseObjectCount(hqueryroot, wszsubobjecttype.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DRMGetCertificateChainCount<P0>(wszchain: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetCertificateChainCount(wszchain : windows_core::PCWSTR, pccertcount : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMGetCertificateChainCount(wszchain.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetClientVersion(pdrmclientversioninfo: *mut super::msdrmdefs::DRM_CLIENT_VERSION_INFO) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetClientVersion(pdrmclientversioninfo : *mut super::msdrmdefs::DRM_CLIENT_VERSION_INFO) -> windows_core::HRESULT);
    unsafe { DRMGetClientVersion(pdrmclientversioninfo as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetEnvironmentInfo<P1>(handle: super::msdrmdefs::DRMENVHANDLE, wszattribute: P1, peencoding: *mut super::msdrmdefs::DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetEnvironmentInfo(handle : super::msdrmdefs::DRMENVHANDLE, wszattribute : windows_core::PCWSTR, peencoding : *mut super::msdrmdefs::DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_core::HRESULT);
    unsafe { DRMGetEnvironmentInfo(handle, wszattribute.param().abi(), peencoding as _, pcbuffer as _, pbbuffer as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetInfo<P1>(handle: super::msdrmdefs::DRMHANDLE, wszattribute: P1, peencoding: *const super::msdrmdefs::DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetInfo(handle : super::msdrmdefs::DRMHANDLE, wszattribute : windows_core::PCWSTR, peencoding : *const super::msdrmdefs::DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_core::HRESULT);
    unsafe { DRMGetInfo(handle, wszattribute.param().abi(), peencoding, pcbuffer as _, pbbuffer as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetIntervalTime(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE) -> windows_core::Result<u32> {
    windows_core::link!("msdrm.dll" "system" fn DRMGetIntervalTime(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, pcdays : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMGetIntervalTime(hissuancelicense, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
#[inline]
pub unsafe fn DRMGetIssuanceLicenseInfo(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, psttimefrom: *mut super::minwinbase::SYSTEMTIME, psttimeuntil: *mut super::minwinbase::SYSTEMTIME, uflags: u32, pudistributionpointnamelength: *mut u32, wszdistributionpointname: Option<windows_core::PWSTR>, pudistributionpointurllength: *mut u32, wszdistributionpointurl: Option<windows_core::PWSTR>, phowner: *mut super::msdrmdefs::DRMPUBHANDLE, pfofficial: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseInfo(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, psttimefrom : *mut super::minwinbase::SYSTEMTIME, psttimeuntil : *mut super::minwinbase::SYSTEMTIME, uflags : u32, pudistributionpointnamelength : *mut u32, wszdistributionpointname : windows_core::PWSTR, pudistributionpointurllength : *mut u32, wszdistributionpointurl : windows_core::PWSTR, phowner : *mut super::msdrmdefs::DRMPUBHANDLE, pfofficial : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { DRMGetIssuanceLicenseInfo(hissuancelicense, psttimefrom as _, psttimeuntil as _, uflags, pudistributionpointnamelength as _, wszdistributionpointname.unwrap_or(core::mem::zeroed()) as _, pudistributionpointurllength as _, wszdistributionpointurl.unwrap_or(core::mem::zeroed()) as _, phowner as _, pfofficial as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetIssuanceLicenseTemplate(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, puissuancelicensetemplatelength: *mut u32, wszissuancelicensetemplate: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseTemplate(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, puissuancelicensetemplatelength : *mut u32, wszissuancelicensetemplate : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetIssuanceLicenseTemplate(hissuancelicense, puissuancelicensetemplatelength as _, wszissuancelicensetemplate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetMetaData(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, pucontentidlength: *mut u32, wszcontentid: Option<windows_core::PWSTR>, pucontentidtypelength: *mut u32, wszcontentidtype: Option<windows_core::PWSTR>, puskuidlength: *mut u32, wszskuid: Option<windows_core::PWSTR>, puskuidtypelength: *mut u32, wszskuidtype: Option<windows_core::PWSTR>, pucontenttypelength: *mut u32, wszcontenttype: Option<windows_core::PWSTR>, pucontentnamelength: *mut u32, wszcontentname: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetMetaData(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, pucontentidlength : *mut u32, wszcontentid : windows_core::PWSTR, pucontentidtypelength : *mut u32, wszcontentidtype : windows_core::PWSTR, puskuidlength : *mut u32, wszskuid : windows_core::PWSTR, puskuidtypelength : *mut u32, wszskuidtype : windows_core::PWSTR, pucontenttypelength : *mut u32, wszcontenttype : windows_core::PWSTR, pucontentnamelength : *mut u32, wszcontentname : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetMetaData(hissuancelicense, pucontentidlength as _, wszcontentid.unwrap_or(core::mem::zeroed()) as _, pucontentidtypelength as _, wszcontentidtype.unwrap_or(core::mem::zeroed()) as _, puskuidlength as _, wszskuid.unwrap_or(core::mem::zeroed()) as _, puskuidtypelength as _, wszskuidtype.unwrap_or(core::mem::zeroed()) as _, pucontenttypelength as _, wszcontenttype.unwrap_or(core::mem::zeroed()) as _, pucontentnamelength as _, wszcontentname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetNameAndDescription(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, uindex: u32, pulcid: *mut u32, punamelength: *mut u32, wszname: Option<windows_core::PWSTR>, pudescriptionlength: *mut u32, wszdescription: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetNameAndDescription(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, uindex : u32, pulcid : *mut u32, punamelength : *mut u32, wszname : windows_core::PWSTR, pudescriptionlength : *mut u32, wszdescription : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetNameAndDescription(hissuancelicense, uindex, pulcid as _, punamelength as _, wszname.unwrap_or(core::mem::zeroed()) as _, pudescriptionlength as _, wszdescription.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetOwnerLicense(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, puownerlicenselength: *mut u32, wszownerlicense: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetOwnerLicense(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, puownerlicenselength : *mut u32, wszownerlicense : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetOwnerLicense(hissuancelicense, puownerlicenselength as _, wszownerlicense.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "msdrmdefs"))]
#[inline]
pub unsafe fn DRMGetProcAddress<P1>(hlibrary: super::msdrmdefs::DRMHANDLE, wszprocname: P1) -> windows_core::Result<super::minwindef::FARPROC>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetProcAddress(hlibrary : super::msdrmdefs::DRMHANDLE, wszprocname : windows_core::PCWSTR, ppfnprocaddress : *mut super::minwindef::FARPROC) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMGetProcAddress(hlibrary, wszprocname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
#[inline]
pub unsafe fn DRMGetRevocationPoint(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, puidlength: *mut u32, wszid: Option<windows_core::PWSTR>, puidtypelength: *mut u32, wszidtype: Option<windows_core::PWSTR>, puurllength: *mut u32, wszrl: Option<windows_core::PWSTR>, pstfrequency: *mut super::minwinbase::SYSTEMTIME, punamelength: *mut u32, wszname: Option<windows_core::PWSTR>, pupublickeylength: *mut u32, wszpublickey: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetRevocationPoint(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, puidlength : *mut u32, wszid : windows_core::PWSTR, puidtypelength : *mut u32, wszidtype : windows_core::PWSTR, puurllength : *mut u32, wszrl : windows_core::PWSTR, pstfrequency : *mut super::minwinbase::SYSTEMTIME, punamelength : *mut u32, wszname : windows_core::PWSTR, pupublickeylength : *mut u32, wszpublickey : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetRevocationPoint(hissuancelicense, puidlength as _, wszid.unwrap_or(core::mem::zeroed()) as _, puidtypelength as _, wszidtype.unwrap_or(core::mem::zeroed()) as _, puurllength as _, wszrl.unwrap_or(core::mem::zeroed()) as _, pstfrequency as _, punamelength as _, wszname.unwrap_or(core::mem::zeroed()) as _, pupublickeylength as _, wszpublickey.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetRightExtendedInfo(hright: super::msdrmdefs::DRMPUBHANDLE, uindex: u32, puextendedinfonamelength: *mut u32, wszextendedinfoname: Option<windows_core::PWSTR>, puextendedinfovaluelength: *mut u32, wszextendedinfovalue: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetRightExtendedInfo(hright : super::msdrmdefs::DRMPUBHANDLE, uindex : u32, puextendedinfonamelength : *mut u32, wszextendedinfoname : windows_core::PWSTR, puextendedinfovaluelength : *mut u32, wszextendedinfovalue : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetRightExtendedInfo(hright, uindex, puextendedinfonamelength as _, wszextendedinfoname.unwrap_or(core::mem::zeroed()) as _, puextendedinfovaluelength as _, wszextendedinfovalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
#[inline]
pub unsafe fn DRMGetRightInfo(hright: super::msdrmdefs::DRMPUBHANDLE, purightnamelength: *mut u32, wszrightname: Option<windows_core::PWSTR>, pstfrom: *mut super::minwinbase::SYSTEMTIME, pstuntil: *mut super::minwinbase::SYSTEMTIME) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetRightInfo(hright : super::msdrmdefs::DRMPUBHANDLE, purightnamelength : *mut u32, wszrightname : windows_core::PWSTR, pstfrom : *mut super::minwinbase::SYSTEMTIME, pstuntil : *mut super::minwinbase::SYSTEMTIME) -> windows_core::HRESULT);
    unsafe { DRMGetRightInfo(hright, purightnamelength as _, wszrightname.unwrap_or(core::mem::zeroed()) as _, pstfrom as _, pstuntil as _) }
}
#[inline]
pub unsafe fn DRMGetSecurityProvider(uflags: u32, putypelen: *mut u32, wsztype: Option<windows_core::PWSTR>, pupathlen: *mut u32, wszpath: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetSecurityProvider(uflags : u32, putypelen : *mut u32, wsztype : windows_core::PWSTR, pupathlen : *mut u32, wszpath : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetSecurityProvider(uflags, putypelen as _, wsztype.unwrap_or(core::mem::zeroed()) as _, pupathlen as _, wszpath.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetServiceLocation<P3>(hclient: super::msdrmdefs::DRMHSESSION, uservicetype: u32, uservicelocation: u32, wszissuancelicense: P3, puserviceurllength: *mut u32, wszserviceurl: Option<windows_core::PWSTR>) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetServiceLocation(hclient : super::msdrmdefs::DRMHSESSION, uservicetype : u32, uservicelocation : u32, wszissuancelicense : windows_core::PCWSTR, puserviceurllength : *mut u32, wszserviceurl : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetServiceLocation(hclient, uservicetype, uservicelocation, wszissuancelicense.param().abi(), puserviceurllength as _, wszserviceurl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicense<P5, P6, P8>(henv: super::msdrmdefs::DRMENVHANDLE, hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, uflags: u32, pbsymkey: *mut u8, cbsymkey: u32, wszsymkeytype: P5, wszclientlicensorcertificate: P6, pfncallback: super::msdrmdefs::DRMCALLBACK, wszurl: P8, pvcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicense(henv : super::msdrmdefs::DRMENVHANDLE, hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, uflags : u32, pbsymkey : *mut u8, cbsymkey : u32, wszsymkeytype : windows_core::PCWSTR, wszclientlicensorcertificate : windows_core::PCWSTR, pfncallback : super::msdrmdefs::DRMCALLBACK, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DRMGetSignedIssuanceLicense(henv, hissuancelicense, uflags, pbsymkey as _, cbsymkey, wszsymkeytype.param().abi(), wszclientlicensorcertificate.param().abi(), pfncallback, wszurl.param().abi(), pvcontext as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicenseEx<P5>(henv: super::msdrmdefs::DRMENVHANDLE, hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, uflags: u32, pbsymkey: Option<&[u8]>, wszsymkeytype: P5, pvreserved: Option<*const core::ffi::c_void>, henablingprincipal: super::msdrmdefs::DRMHANDLE, hboundlicenseclc: super::msdrmdefs::DRMHANDLE, pfncallback: super::msdrmdefs::DRMCALLBACK, pvcontext: *const core::ffi::c_void) -> windows_core::HRESULT
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicenseEx(henv : super::msdrmdefs::DRMENVHANDLE, hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, uflags : u32, pbsymkey : *const u8, cbsymkey : u32, wszsymkeytype : windows_core::PCWSTR, pvreserved : *const core::ffi::c_void, henablingprincipal : super::msdrmdefs::DRMHANDLE, hboundlicenseclc : super::msdrmdefs::DRMHANDLE, pfncallback : super::msdrmdefs::DRMCALLBACK, pvcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DRMGetSignedIssuanceLicenseEx(henv, hissuancelicense, uflags, pbsymkey.map_or(core::ptr::null(), |slice| slice.as_ptr()), pbsymkey.map_or(0, |slice| slice.len().try_into().unwrap()), wszsymkeytype.param().abi(), pvreserved.unwrap_or(core::mem::zeroed()) as _, henablingprincipal, hboundlicenseclc, pfncallback, pvcontext) }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
#[inline]
pub unsafe fn DRMGetTime(henv: super::msdrmdefs::DRMENVHANDLE, etimeridtype: super::msdrmdefs::DRMTIMETYPE) -> windows_core::Result<super::minwinbase::SYSTEMTIME> {
    windows_core::link!("msdrm.dll" "system" fn DRMGetTime(henv : super::msdrmdefs::DRMENVHANDLE, etimeridtype : super::msdrmdefs::DRMTIMETYPE, potimeobject : *mut super::minwinbase::SYSTEMTIME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMGetTime(henv, etimeridtype, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttribute<P1>(hqueryroot: super::msdrmdefs::DRMQUERYHANDLE, wszattributetype: P1, iwhich: u32, peencoding: *mut super::msdrmdefs::DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttribute(hqueryroot : super::msdrmdefs::DRMQUERYHANDLE, wszattributetype : windows_core::PCWSTR, iwhich : u32, peencoding : *mut super::msdrmdefs::DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_core::HRESULT);
    unsafe { DRMGetUnboundLicenseAttribute(hqueryroot, wszattributetype.param().abi(), iwhich, peencoding as _, pcbuffer as _, pbbuffer as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttributeCount<P1>(hqueryroot: super::msdrmdefs::DRMQUERYHANDLE, wszattributetype: P1) -> windows_core::Result<u32>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttributeCount(hqueryroot : super::msdrmdefs::DRMQUERYHANDLE, wszattributetype : windows_core::PCWSTR, pcattributes : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMGetUnboundLicenseAttributeCount(hqueryroot, wszattributetype.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetUnboundLicenseObject<P1>(hqueryroot: super::msdrmdefs::DRMQUERYHANDLE, wszsubobjecttype: P1, iindex: u32) -> windows_core::Result<super::msdrmdefs::DRMQUERYHANDLE>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObject(hqueryroot : super::msdrmdefs::DRMQUERYHANDLE, wszsubobjecttype : windows_core::PCWSTR, iindex : u32, phsubquery : *mut super::msdrmdefs::DRMQUERYHANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMGetUnboundLicenseObject(hqueryroot, wszsubobjecttype.param().abi(), iindex, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetUnboundLicenseObjectCount<P1>(hqueryroot: super::msdrmdefs::DRMQUERYHANDLE, wszsubobjecttype: P1) -> windows_core::Result<u32>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObjectCount(hqueryroot : super::msdrmdefs::DRMQUERYHANDLE, wszsubobjecttype : windows_core::PCWSTR, pcsubobjects : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMGetUnboundLicenseObjectCount(hqueryroot, wszsubobjecttype.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetUsagePolicy(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, uindex: u32, peusagepolicytype: *mut super::msdrmdefs::DRM_USAGEPOLICY_TYPE, pfexclusion: *mut windows_core::BOOL, punamelength: *mut u32, wszname: Option<windows_core::PWSTR>, puminversionlength: *mut u32, wszminversion: Option<windows_core::PWSTR>, pumaxversionlength: *mut u32, wszmaxversion: Option<windows_core::PWSTR>, pupublickeylength: *mut u32, wszpublickey: Option<windows_core::PWSTR>, pudigestalgorithmlength: *mut u32, wszdigestalgorithm: Option<windows_core::PWSTR>, pcbdigest: *mut u32, pbdigest: *mut u8) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetUsagePolicy(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, uindex : u32, peusagepolicytype : *mut super::msdrmdefs::DRM_USAGEPOLICY_TYPE, pfexclusion : *mut windows_core::BOOL, punamelength : *mut u32, wszname : windows_core::PWSTR, puminversionlength : *mut u32, wszminversion : windows_core::PWSTR, pumaxversionlength : *mut u32, wszmaxversion : windows_core::PWSTR, pupublickeylength : *mut u32, wszpublickey : windows_core::PWSTR, pudigestalgorithmlength : *mut u32, wszdigestalgorithm : windows_core::PWSTR, pcbdigest : *mut u32, pbdigest : *mut u8) -> windows_core::HRESULT);
    unsafe { DRMGetUsagePolicy(hissuancelicense, uindex, peusagepolicytype as _, pfexclusion as _, punamelength as _, wszname.unwrap_or(core::mem::zeroed()) as _, puminversionlength as _, wszminversion.unwrap_or(core::mem::zeroed()) as _, pumaxversionlength as _, wszmaxversion.unwrap_or(core::mem::zeroed()) as _, pupublickeylength as _, wszpublickey.unwrap_or(core::mem::zeroed()) as _, pudigestalgorithmlength as _, wszdigestalgorithm.unwrap_or(core::mem::zeroed()) as _, pcbdigest as _, pbdigest as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetUserInfo(huser: super::msdrmdefs::DRMPUBHANDLE, puusernamelength: *mut u32, wszusername: Option<windows_core::PWSTR>, puuseridlength: *mut u32, wszuserid: Option<windows_core::PWSTR>, puuseridtypelength: *mut u32, wszuseridtype: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetUserInfo(huser : super::msdrmdefs::DRMPUBHANDLE, puusernamelength : *mut u32, wszusername : windows_core::PWSTR, puuseridlength : *mut u32, wszuserid : windows_core::PWSTR, puuseridtypelength : *mut u32, wszuseridtype : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetUserInfo(huser, puusernamelength as _, wszusername.unwrap_or(core::mem::zeroed()) as _, puuseridlength as _, wszuserid.unwrap_or(core::mem::zeroed()) as _, puuseridtypelength as _, wszuseridtype.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetUserRights(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, huser: super::msdrmdefs::DRMPUBHANDLE, uindex: u32) -> windows_core::Result<super::msdrmdefs::DRMPUBHANDLE> {
    windows_core::link!("msdrm.dll" "system" fn DRMGetUserRights(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, huser : super::msdrmdefs::DRMPUBHANDLE, uindex : u32, phright : *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMGetUserRights(hissuancelicense, huser, uindex, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMGetUsers(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, uindex: u32) -> windows_core::Result<super::msdrmdefs::DRMPUBHANDLE> {
    windows_core::link!("msdrm.dll" "system" fn DRMGetUsers(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, uindex : u32, phuser : *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMGetUsers(hissuancelicense, uindex, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMInitEnvironment<P2, P3, P4>(esecurityprovidertype: super::msdrmdefs::DRMSECURITYPROVIDERTYPE, especification: super::msdrmdefs::DRMSPECTYPE, wszsecurityprovider: P2, wszmanifestcredentials: P3, wszmachinecredentials: P4, phenv: *mut super::msdrmdefs::DRMENVHANDLE, phdefaultlibrary: *mut super::msdrmdefs::DRMHANDLE) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMInitEnvironment(esecurityprovidertype : super::msdrmdefs::DRMSECURITYPROVIDERTYPE, especification : super::msdrmdefs::DRMSPECTYPE, wszsecurityprovider : windows_core::PCWSTR, wszmanifestcredentials : windows_core::PCWSTR, wszmachinecredentials : windows_core::PCWSTR, phenv : *mut super::msdrmdefs::DRMENVHANDLE, phdefaultlibrary : *mut super::msdrmdefs::DRMHANDLE) -> windows_core::HRESULT);
    unsafe { DRMInitEnvironment(esecurityprovidertype, especification, wszsecurityprovider.param().abi(), wszmanifestcredentials.param().abi(), wszmachinecredentials.param().abi(), phenv as _, phdefaultlibrary as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMIsActivated(hclient: super::msdrmdefs::DRMHSESSION, uflags: u32) -> windows_core::Result<super::msdrmdefs::DRM_ACTSERV_INFO> {
    windows_core::link!("msdrm.dll" "system" fn DRMIsActivated(hclient : super::msdrmdefs::DRMHSESSION, uflags : u32, pactservinfo : *mut super::msdrmdefs::DRM_ACTSERV_INFO) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMIsActivated(hclient, uflags, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DRMIsWindowProtected(hwnd: super::windef::HWND) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("msdrm.dll" "system" fn DRMIsWindowProtected(hwnd : super::windef::HWND, pfprotected : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMIsWindowProtected(hwnd, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMLoadLibrary<P2, P3>(henv: super::msdrmdefs::DRMENVHANDLE, especification: super::msdrmdefs::DRMSPECTYPE, wszlibraryprovider: P2, wszcredentials: P3) -> windows_core::Result<super::msdrmdefs::DRMHANDLE>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMLoadLibrary(henv : super::msdrmdefs::DRMENVHANDLE, especification : super::msdrmdefs::DRMSPECTYPE, wszlibraryprovider : windows_core::PCWSTR, wszcredentials : windows_core::PCWSTR, phlibrary : *mut super::msdrmdefs::DRMHANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMLoadLibrary(henv, especification, wszlibraryprovider.param().abi(), wszcredentials.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMParseUnboundLicense<P0>(wszcertificate: P0) -> windows_core::Result<super::msdrmdefs::DRMQUERYHANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMParseUnboundLicense(wszcertificate : windows_core::PCWSTR, phqueryroot : *mut super::msdrmdefs::DRMQUERYHANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DRMParseUnboundLicense(wszcertificate.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn DRMRegisterContent(fregister: bool) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMRegisterContent(fregister : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { DRMRegisterContent(fregister.into()) }
}
#[cfg(all(feature = "msdrmdefs", feature = "windef"))]
#[inline]
pub unsafe fn DRMRegisterProtectedWindow(henv: super::msdrmdefs::DRMENVHANDLE, hwnd: super::windef::HWND) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMRegisterProtectedWindow(henv : super::msdrmdefs::DRMENVHANDLE, hwnd : super::windef::HWND) -> windows_core::HRESULT);
    unsafe { DRMRegisterProtectedWindow(henv, hwnd) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMRegisterRevocationList<P1>(henv: super::msdrmdefs::DRMENVHANDLE, wszrevocationlist: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMRegisterRevocationList(henv : super::msdrmdefs::DRMENVHANDLE, wszrevocationlist : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMRegisterRevocationList(henv, wszrevocationlist.param().abi()) }
}
#[inline]
pub unsafe fn DRMRepair() -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMRepair() -> windows_core::HRESULT);
    unsafe { DRMRepair() }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMSetApplicationSpecificData<P2, P3>(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, fdelete: bool, wszname: P2, wszvalue: P3) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMSetApplicationSpecificData(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, fdelete : windows_core::BOOL, wszname : windows_core::PCWSTR, wszvalue : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMSetApplicationSpecificData(hissuancelicense, fdelete.into(), wszname.param().abi(), wszvalue.param().abi()) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMSetGlobalOptions(eglobaloptions: super::msdrmdefs::DRMGLOBALOPTIONS, pvdata: *mut core::ffi::c_void, dwlen: u32) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMSetGlobalOptions(eglobaloptions : super::msdrmdefs::DRMGLOBALOPTIONS, pvdata : *mut core::ffi::c_void, dwlen : u32) -> windows_core::HRESULT);
    unsafe { DRMSetGlobalOptions(eglobaloptions, pvdata as _, dwlen) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMSetIntervalTime(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, cdays: u32) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMSetIntervalTime(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, cdays : u32) -> windows_core::HRESULT);
    unsafe { DRMSetIntervalTime(hissuancelicense, cdays) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMSetMetaData<P1, P2, P3, P4, P5, P6>(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, wszcontentid: P1, wszcontentidtype: P2, wszskuid: P3, wszskuidtype: P4, wszcontenttype: P5, wszcontentname: P6) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMSetMetaData(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, wszcontentid : windows_core::PCWSTR, wszcontentidtype : windows_core::PCWSTR, wszskuid : windows_core::PCWSTR, wszskuidtype : windows_core::PCWSTR, wszcontenttype : windows_core::PCWSTR, wszcontentname : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMSetMetaData(hissuancelicense, wszcontentid.param().abi(), wszcontentidtype.param().abi(), wszskuid.param().abi(), wszskuidtype.param().abi(), wszcontenttype.param().abi(), wszcontentname.param().abi()) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMSetNameAndDescription<P3, P4>(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, fdelete: bool, lcid: u32, wszname: P3, wszdescription: P4) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMSetNameAndDescription(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, fdelete : windows_core::BOOL, lcid : u32, wszname : windows_core::PCWSTR, wszdescription : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMSetNameAndDescription(hissuancelicense, fdelete.into(), lcid, wszname.param().abi(), wszdescription.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
#[inline]
pub unsafe fn DRMSetRevocationPoint<P2, P3, P4, P6, P7>(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, fdelete: bool, wszid: P2, wszidtype: P3, wszurl: P4, pstfrequency: *mut super::minwinbase::SYSTEMTIME, wszname: P6, wszpublickey: P7) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMSetRevocationPoint(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, fdelete : windows_core::BOOL, wszid : windows_core::PCWSTR, wszidtype : windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pstfrequency : *mut super::minwinbase::SYSTEMTIME, wszname : windows_core::PCWSTR, wszpublickey : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMSetRevocationPoint(hissuancelicense, fdelete.into(), wszid.param().abi(), wszidtype.param().abi(), wszurl.param().abi(), pstfrequency as _, wszname.param().abi(), wszpublickey.param().abi()) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMSetUsagePolicy<P4, P5, P6, P7, P8>(hissuancelicense: super::msdrmdefs::DRMPUBHANDLE, eusagepolicytype: super::msdrmdefs::DRM_USAGEPOLICY_TYPE, fdelete: bool, fexclusion: bool, wszname: P4, wszminversion: P5, wszmaxversion: P6, wszpublickey: P7, wszdigestalgorithm: P8, pbdigest: *mut u8, cbdigest: u32) -> windows_core::HRESULT
where
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMSetUsagePolicy(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, eusagepolicytype : super::msdrmdefs::DRM_USAGEPOLICY_TYPE, fdelete : windows_core::BOOL, fexclusion : windows_core::BOOL, wszname : windows_core::PCWSTR, wszminversion : windows_core::PCWSTR, wszmaxversion : windows_core::PCWSTR, wszpublickey : windows_core::PCWSTR, wszdigestalgorithm : windows_core::PCWSTR, pbdigest : *mut u8, cbdigest : u32) -> windows_core::HRESULT);
    unsafe { DRMSetUsagePolicy(hissuancelicense, eusagepolicytype, fdelete.into(), fexclusion.into(), wszname.param().abi(), wszminversion.param().abi(), wszmaxversion.param().abi(), wszpublickey.param().abi(), wszdigestalgorithm.param().abi(), pbdigest as _, cbdigest) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMVerify<P0>(wszdata: P0, pcattesteddata: *mut u32, wszattesteddata: Option<windows_core::PWSTR>, petype: *mut super::msdrmdefs::DRMATTESTTYPE, pcprincipal: *mut u32, wszprincipal: Option<windows_core::PWSTR>, pcmanifest: *mut u32, wszmanifest: Option<windows_core::PWSTR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMVerify(wszdata : windows_core::PCWSTR, pcattesteddata : *mut u32, wszattesteddata : windows_core::PWSTR, petype : *mut super::msdrmdefs::DRMATTESTTYPE, pcprincipal : *mut u32, wszprincipal : windows_core::PWSTR, pcmanifest : *mut u32, wszmanifest : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMVerify(wszdata.param().abi(), pcattesteddata as _, wszattesteddata.unwrap_or(core::mem::zeroed()) as _, petype as _, pcprincipal as _, wszprincipal.unwrap_or(core::mem::zeroed()) as _, pcmanifest as _, wszmanifest.unwrap_or(core::mem::zeroed()) as _) }
}
