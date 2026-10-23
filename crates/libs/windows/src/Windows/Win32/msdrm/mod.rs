#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMAcquireAdvisories<P1, P2>(hlicensestorage: super::DRMHSESSION, wszlicense: P1, wszurl: P2, pvcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMAcquireAdvisories(hlicensestorage : super::DRMHSESSION, wszlicense : windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DRMAcquireAdvisories(hlicensestorage, wszlicense.param().abi(), wszurl.param().abi(), pvcontext as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMAcquireIssuanceLicenseTemplate<P5>(hclient: super::DRMHSESSION, uflags: super::UINT, pvreserved: *mut core::ffi::c_void, ctemplates: super::UINT, pwsztemplateids: Option<*const windows_core::PCWSTR>, wszurl: P5, pvcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMAcquireIssuanceLicenseTemplate(hclient : super::DRMHSESSION, uflags : super::UINT, pvreserved : *mut core::ffi::c_void, ctemplates : super::UINT, pwsztemplateids : *const windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DRMAcquireIssuanceLicenseTemplate(hclient, uflags, pvreserved as _, ctemplates, pwsztemplateids.unwrap_or(core::mem::zeroed()) as _, wszurl.param().abi(), pvcontext as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMAcquireLicense<P2, P3, P4, P5>(hsession: super::DRMHSESSION, uflags: super::UINT, wszgroupidentitycredential: P2, wszrequestedrights: P3, wszcustomdata: P4, wszurl: P5, pvcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMAcquireLicense(hsession : super::DRMHSESSION, uflags : super::UINT, wszgroupidentitycredential : windows_core::PCWSTR, wszrequestedrights : windows_core::PCWSTR, wszcustomdata : windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DRMAcquireLicense(hsession, uflags, wszgroupidentitycredential.param().abi(), wszrequestedrights.param().abi(), wszcustomdata.param().abi(), wszurl.param().abi(), pvcontext as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMAddLicense<P2>(hlicensestorage: super::DRMHSESSION, uflags: super::UINT, wszlicense: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMAddLicense(hlicensestorage : super::DRMHSESSION, uflags : super::UINT, wszlicense : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMAddLicense(hlicensestorage, uflags, wszlicense.param().abi()) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMAddRightWithUser(hissuancelicense: super::DRMPUBHANDLE, hright: super::DRMPUBHANDLE, huser: super::DRMPUBHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMAddRightWithUser(hissuancelicense : super::DRMPUBHANDLE, hright : super::DRMPUBHANDLE, huser : super::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMAddRightWithUser(hissuancelicense, hright, huser) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMAttest<P1>(henablingprincipal: super::DRMHANDLE, wszdata: P1, etype: super::DRMATTESTTYPE, pcattestedblob: *mut super::UINT, wszattestedblob: windows_core::PWSTR) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMAttest(henablingprincipal : super::DRMHANDLE, wszdata : windows_core::PCWSTR, etype : super::DRMATTESTTYPE, pcattestedblob : *mut super::UINT, wszattestedblob : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMAttest(henablingprincipal, wszdata.param().abi(), etype, pcattestedblob as _, wszattestedblob) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMCheckSecurity(henv: super::DRMENVHANDLE, clevel: super::UINT) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMCheckSecurity(henv : super::DRMENVHANDLE, clevel : super::UINT) -> windows_core::HRESULT);
    unsafe { DRMCheckSecurity(henv, clevel) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMClearAllRights(hissuancelicense: super::DRMPUBHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMClearAllRights(hissuancelicense : super::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMClearAllRights(hissuancelicense) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCloseEnvironmentHandle(henv: super::DRMENVHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMCloseEnvironmentHandle(henv : super::DRMENVHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCloseEnvironmentHandle(henv) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCloseHandle(handle: super::DRMHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMCloseHandle(handle : super::DRMHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCloseHandle(handle) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMClosePubHandle(hpub: super::DRMPUBHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMClosePubHandle(hpub : super::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMClosePubHandle(hpub) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCloseQueryHandle(hquery: super::DRMQUERYHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMCloseQueryHandle(hquery : super::DRMQUERYHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCloseQueryHandle(hquery) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCloseSession(hsession: super::DRMHSESSION) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMCloseSession(hsession : super::DRMHSESSION) -> windows_core::HRESULT);
    unsafe { DRMCloseSession(hsession) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DRMConstructCertificateChain(ccertificates: super::UINT, rgwszcertificates: *const windows_core::PCWSTR, pcchain: *mut super::UINT, wszchain: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMConstructCertificateChain(ccertificates : super::UINT, rgwszcertificates : *const windows_core::PCWSTR, pcchain : *mut super::UINT, wszchain : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMConstructCertificateChain(ccertificates, rgwszcertificates, pcchain as _, wszchain.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMCreateClientSession<P2, P3>(pfncallback: super::DRMCALLBACK, ucallbackversion: super::UINT, wszgroupidprovidertype: P2, wszgroupid: P3, phclient: *mut super::DRMHSESSION) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateClientSession(pfncallback : super::DRMCALLBACK, ucallbackversion : super::UINT, wszgroupidprovidertype : windows_core::PCWSTR, wszgroupid : windows_core::PCWSTR, phclient : *mut super::DRMHSESSION) -> windows_core::HRESULT);
    unsafe { DRMCreateClientSession(pfncallback, ucallbackversion, wszgroupidprovidertype.param().abi(), wszgroupid.param().abi(), phclient as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCreateEnablingBitsDecryptor<P1, P3>(hboundlicense: super::DRMHANDLE, wszright: P1, hauxlib: super::DRMHANDLE, wszauxplug: P3, phdecryptor: *mut super::DRMHANDLE) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsDecryptor(hboundlicense : super::DRMHANDLE, wszright : windows_core::PCWSTR, hauxlib : super::DRMHANDLE, wszauxplug : windows_core::PCWSTR, phdecryptor : *mut super::DRMHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCreateEnablingBitsDecryptor(hboundlicense, wszright.param().abi(), hauxlib, wszauxplug.param().abi(), phdecryptor as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCreateEnablingBitsEncryptor<P1, P3>(hboundlicense: super::DRMHANDLE, wszright: P1, hauxlib: super::DRMHANDLE, wszauxplug: P3, phencryptor: *mut super::DRMHANDLE) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsEncryptor(hboundlicense : super::DRMHANDLE, wszright : windows_core::PCWSTR, hauxlib : super::DRMHANDLE, wszauxplug : windows_core::PCWSTR, phencryptor : *mut super::DRMHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCreateEnablingBitsEncryptor(hboundlicense, wszright.param().abi(), hauxlib, wszauxplug.param().abi(), phencryptor as _) }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMCreateIssuanceLicense<P2, P3, P5>(psttimefrom: *mut super::SYSTEMTIME, psttimeuntil: *mut super::SYSTEMTIME, wszreferralinfoname: P2, wszreferralinfourl: P3, howner: super::DRMPUBHANDLE, wszissuancelicense: P5, hboundlicense: super::DRMHANDLE, phissuancelicense: *mut super::DRMPUBHANDLE) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateIssuanceLicense(psttimefrom : *mut super::SYSTEMTIME, psttimeuntil : *mut super::SYSTEMTIME, wszreferralinfoname : windows_core::PCWSTR, wszreferralinfourl : windows_core::PCWSTR, howner : super::DRMPUBHANDLE, wszissuancelicense : windows_core::PCWSTR, hboundlicense : super::DRMHANDLE, phissuancelicense : *mut super::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCreateIssuanceLicense(psttimefrom as _, psttimeuntil as _, wszreferralinfoname.param().abi(), wszreferralinfourl.param().abi(), howner, wszissuancelicense.param().abi(), hboundlicense, phissuancelicense as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMCreateLicenseStorageSession<P4>(henv: super::DRMENVHANDLE, hdefaultlibrary: super::DRMHANDLE, hclient: super::DRMHSESSION, uflags: super::UINT, wszissuancelicense: P4, phlicensestorage: *mut super::DRMHSESSION) -> windows_core::HRESULT
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateLicenseStorageSession(henv : super::DRMENVHANDLE, hdefaultlibrary : super::DRMHANDLE, hclient : super::DRMHSESSION, uflags : super::UINT, wszissuancelicense : windows_core::PCWSTR, phlicensestorage : *mut super::DRMHSESSION) -> windows_core::HRESULT);
    unsafe { DRMCreateLicenseStorageSession(henv, hdefaultlibrary, hclient, uflags, wszissuancelicense.param().abi(), phlicensestorage as _) }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMCreateRight<P0>(wszrightname: P0, pstfrom: *mut super::SYSTEMTIME, pstuntil: *mut super::SYSTEMTIME, cextendedinfo: super::UINT, pwszextendedinfoname: Option<*const windows_core::PCWSTR>, pwszextendedinfovalue: Option<*const windows_core::PCWSTR>, phright: *mut super::DRMPUBHANDLE) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateRight(wszrightname : windows_core::PCWSTR, pstfrom : *mut super::SYSTEMTIME, pstuntil : *mut super::SYSTEMTIME, cextendedinfo : super::UINT, pwszextendedinfoname : *const windows_core::PCWSTR, pwszextendedinfovalue : *const windows_core::PCWSTR, phright : *mut super::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCreateRight(wszrightname.param().abi(), pstfrom as _, pstuntil as _, cextendedinfo, pwszextendedinfoname.unwrap_or(core::mem::zeroed()) as _, pwszextendedinfovalue.unwrap_or(core::mem::zeroed()) as _, phright as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMCreateUser<P0, P1, P2>(wszusername: P0, wszuserid: P1, wszuseridtype: P2, phuser: *mut super::DRMPUBHANDLE) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMCreateUser(wszusername : windows_core::PCWSTR, wszuserid : windows_core::PCWSTR, wszuseridtype : windows_core::PCWSTR, phuser : *mut super::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMCreateUser(wszusername.param().abi(), wszuserid.param().abi(), wszuseridtype.param().abi(), phuser as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DRMDecode<P0, P1>(wszalgid: P0, wszencodedstring: P1, pudecodeddatalen: *mut super::UINT, pbdecodeddata: *mut super::BYTE) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMDecode(wszalgid : windows_core::PCWSTR, wszencodedstring : windows_core::PCWSTR, pudecodeddatalen : *mut super::UINT, pbdecodeddata : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { DRMDecode(wszalgid.param().abi(), wszencodedstring.param().abi(), pudecodeddatalen as _, pbdecodeddata as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DRMDeconstructCertificateChain<P0>(wszchain: P0, iwhich: super::UINT, pccert: *mut super::UINT, wszcert: Option<windows_core::PWSTR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMDeconstructCertificateChain(wszchain : windows_core::PCWSTR, iwhich : super::UINT, pccert : *mut super::UINT, wszcert : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMDeconstructCertificateChain(wszchain.param().abi(), iwhich, pccert as _, wszcert.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMDecrypt(hcryptoprovider: super::DRMHANDLE, iposition: super::UINT, cnuminbytes: super::UINT, pbindata: *mut super::BYTE, pcnumoutbytes: *mut super::UINT, pboutdata: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMDecrypt(hcryptoprovider : super::DRMHANDLE, iposition : super::UINT, cnuminbytes : super::UINT, pbindata : *mut super::BYTE, pcnumoutbytes : *mut super::UINT, pboutdata : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { DRMDecrypt(hcryptoprovider, iposition, cnuminbytes, pbindata as _, pcnumoutbytes as _, pboutdata as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMDeleteLicense<P1>(hsession: super::DRMHSESSION, wszlicenseid: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMDeleteLicense(hsession : super::DRMHSESSION, wszlicenseid : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMDeleteLicense(hsession, wszlicenseid.param().abi()) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMDuplicateEnvironmentHandle(htocopy: super::DRMENVHANDLE, phcopy: *mut super::DRMENVHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMDuplicateEnvironmentHandle(htocopy : super::DRMENVHANDLE, phcopy : *mut super::DRMENVHANDLE) -> windows_core::HRESULT);
    unsafe { DRMDuplicateEnvironmentHandle(htocopy, phcopy as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMDuplicateHandle(htocopy: super::DRMHANDLE, phcopy: *mut super::DRMHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMDuplicateHandle(htocopy : super::DRMHANDLE, phcopy : *mut super::DRMHANDLE) -> windows_core::HRESULT);
    unsafe { DRMDuplicateHandle(htocopy, phcopy as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMDuplicatePubHandle(hpubin: super::DRMPUBHANDLE, phpubout: *mut super::DRMPUBHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMDuplicatePubHandle(hpubin : super::DRMPUBHANDLE, phpubout : *mut super::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMDuplicatePubHandle(hpubin, phpubout as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMDuplicateSession(hsessionin: super::DRMHSESSION, phsessionout: *mut super::DRMHSESSION) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMDuplicateSession(hsessionin : super::DRMHSESSION, phsessionout : *mut super::DRMHSESSION) -> windows_core::HRESULT);
    unsafe { DRMDuplicateSession(hsessionin, phsessionout as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DRMEncode<P0>(wszalgid: P0, udatalen: super::UINT, pbdecodeddata: *mut super::BYTE, puencodedstringlen: *mut super::UINT, wszencodedstring: Option<windows_core::PWSTR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMEncode(wszalgid : windows_core::PCWSTR, udatalen : super::UINT, pbdecodeddata : *mut super::BYTE, puencodedstringlen : *mut super::UINT, wszencodedstring : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMEncode(wszalgid.param().abi(), udatalen, pbdecodeddata as _, puencodedstringlen as _, wszencodedstring.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMEncrypt(hcryptoprovider: super::DRMHANDLE, iposition: super::UINT, cnuminbytes: super::UINT, pbindata: *mut super::BYTE, pcnumoutbytes: *mut super::UINT, pboutdata: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMEncrypt(hcryptoprovider : super::DRMHANDLE, iposition : super::UINT, cnuminbytes : super::UINT, pbindata : *mut super::BYTE, pcnumoutbytes : *mut super::UINT, pboutdata : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { DRMEncrypt(hcryptoprovider, iposition, cnuminbytes, pbindata as _, pcnumoutbytes as _, pboutdata as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMEnumerateLicense(hsession: super::DRMHSESSION, uflags: super::UINT, uindex: super::UINT, pfsharedflag: *mut windows_core::BOOL, pucertificatedatalen: *mut super::UINT, wszcertificatedata: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMEnumerateLicense(hsession : super::DRMHSESSION, uflags : super::UINT, uindex : super::UINT, pfsharedflag : *mut windows_core::BOOL, pucertificatedatalen : *mut super::UINT, wszcertificatedata : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMEnumerateLicense(hsession, uflags, uindex, pfsharedflag as _, pucertificatedatalen as _, wszcertificatedata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetApplicationSpecificData(hissuancelicense: super::DRMPUBHANDLE, uindex: super::UINT, punamelength: *mut super::UINT, wszname: Option<windows_core::PWSTR>, puvaluelength: *mut super::UINT, wszvalue: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetApplicationSpecificData(hissuancelicense : super::DRMPUBHANDLE, uindex : super::UINT, punamelength : *mut super::UINT, wszname : windows_core::PWSTR, puvaluelength : *mut super::UINT, wszvalue : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetApplicationSpecificData(hissuancelicense, uindex, punamelength as _, wszname.unwrap_or(core::mem::zeroed()) as _, puvaluelength as _, wszvalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetBoundLicenseAttribute<P1>(hqueryroot: super::DRMHANDLE, wszattribute: P1, iwhich: super::UINT, peencoding: *mut super::DRMENCODINGTYPE, pcbuffer: *mut super::UINT, pbbuffer: *mut super::BYTE) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttribute(hqueryroot : super::DRMHANDLE, wszattribute : windows_core::PCWSTR, iwhich : super::UINT, peencoding : *mut super::DRMENCODINGTYPE, pcbuffer : *mut super::UINT, pbbuffer : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { DRMGetBoundLicenseAttribute(hqueryroot, wszattribute.param().abi(), iwhich, peencoding as _, pcbuffer as _, pbbuffer as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetBoundLicenseAttributeCount<P1>(hqueryroot: super::DRMHANDLE, wszattribute: P1, pcattributes: *mut super::UINT) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttributeCount(hqueryroot : super::DRMHANDLE, wszattribute : windows_core::PCWSTR, pcattributes : *mut super::UINT) -> windows_core::HRESULT);
    unsafe { DRMGetBoundLicenseAttributeCount(hqueryroot, wszattribute.param().abi(), pcattributes as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetBoundLicenseObject<P1>(hqueryroot: super::DRMHANDLE, wszsubobjecttype: P1, iwhich: super::UINT, phsubobject: *mut super::DRMHANDLE) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObject(hqueryroot : super::DRMHANDLE, wszsubobjecttype : windows_core::PCWSTR, iwhich : super::UINT, phsubobject : *mut super::DRMHANDLE) -> windows_core::HRESULT);
    unsafe { DRMGetBoundLicenseObject(hqueryroot, wszsubobjecttype.param().abi(), iwhich, phsubobject as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetBoundLicenseObjectCount<P1>(hqueryroot: super::DRMHANDLE, wszsubobjecttype: P1, pcsubobjects: *mut super::UINT) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObjectCount(hqueryroot : super::DRMHANDLE, wszsubobjecttype : windows_core::PCWSTR, pcsubobjects : *mut super::UINT) -> windows_core::HRESULT);
    unsafe { DRMGetBoundLicenseObjectCount(hqueryroot, wszsubobjecttype.param().abi(), pcsubobjects as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DRMGetCertificateChainCount<P0>(wszchain: P0, pccertcount: *mut super::UINT) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetCertificateChainCount(wszchain : windows_core::PCWSTR, pccertcount : *mut super::UINT) -> windows_core::HRESULT);
    unsafe { DRMGetCertificateChainCount(wszchain.param().abi(), pccertcount as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetEnvironmentInfo<P1>(handle: super::DRMENVHANDLE, wszattribute: P1, peencoding: *mut super::DRMENCODINGTYPE, pcbuffer: *mut super::UINT, pbbuffer: *mut super::BYTE) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetEnvironmentInfo(handle : super::DRMENVHANDLE, wszattribute : windows_core::PCWSTR, peencoding : *mut super::DRMENCODINGTYPE, pcbuffer : *mut super::UINT, pbbuffer : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { DRMGetEnvironmentInfo(handle, wszattribute.param().abi(), peencoding as _, pcbuffer as _, pbbuffer as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetInfo<P1>(handle: super::DRMHANDLE, wszattribute: P1, peencoding: *const super::DRMENCODINGTYPE, pcbuffer: *mut super::UINT, pbbuffer: *mut super::BYTE) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetInfo(handle : super::DRMHANDLE, wszattribute : windows_core::PCWSTR, peencoding : *const super::DRMENCODINGTYPE, pcbuffer : *mut super::UINT, pbbuffer : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { DRMGetInfo(handle, wszattribute.param().abi(), peencoding, pcbuffer as _, pbbuffer as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetIntervalTime(hissuancelicense: super::DRMPUBHANDLE, pcdays: *mut super::UINT) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetIntervalTime(hissuancelicense : super::DRMPUBHANDLE, pcdays : *mut super::UINT) -> windows_core::HRESULT);
    unsafe { DRMGetIntervalTime(hissuancelicense, pcdays as _) }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetIssuanceLicenseInfo(hissuancelicense: super::DRMPUBHANDLE, psttimefrom: *mut super::SYSTEMTIME, psttimeuntil: *mut super::SYSTEMTIME, uflags: super::UINT, pudistributionpointnamelength: *mut super::UINT, wszdistributionpointname: Option<windows_core::PWSTR>, pudistributionpointurllength: *mut super::UINT, wszdistributionpointurl: Option<windows_core::PWSTR>, phowner: *mut super::DRMPUBHANDLE, pfofficial: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseInfo(hissuancelicense : super::DRMPUBHANDLE, psttimefrom : *mut super::SYSTEMTIME, psttimeuntil : *mut super::SYSTEMTIME, uflags : super::UINT, pudistributionpointnamelength : *mut super::UINT, wszdistributionpointname : windows_core::PWSTR, pudistributionpointurllength : *mut super::UINT, wszdistributionpointurl : windows_core::PWSTR, phowner : *mut super::DRMPUBHANDLE, pfofficial : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { DRMGetIssuanceLicenseInfo(hissuancelicense, psttimefrom as _, psttimeuntil as _, uflags, pudistributionpointnamelength as _, wszdistributionpointname.unwrap_or(core::mem::zeroed()) as _, pudistributionpointurllength as _, wszdistributionpointurl.unwrap_or(core::mem::zeroed()) as _, phowner as _, pfofficial as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetIssuanceLicenseTemplate(hissuancelicense: super::DRMPUBHANDLE, puissuancelicensetemplatelength: *mut super::UINT, wszissuancelicensetemplate: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseTemplate(hissuancelicense : super::DRMPUBHANDLE, puissuancelicensetemplatelength : *mut super::UINT, wszissuancelicensetemplate : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetIssuanceLicenseTemplate(hissuancelicense, puissuancelicensetemplatelength as _, wszissuancelicensetemplate.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetMetaData(hissuancelicense: super::DRMPUBHANDLE, pucontentidlength: *mut super::UINT, wszcontentid: Option<windows_core::PWSTR>, pucontentidtypelength: *mut super::UINT, wszcontentidtype: Option<windows_core::PWSTR>, puskuidlength: *mut super::UINT, wszskuid: Option<windows_core::PWSTR>, puskuidtypelength: *mut super::UINT, wszskuidtype: Option<windows_core::PWSTR>, pucontenttypelength: *mut super::UINT, wszcontenttype: Option<windows_core::PWSTR>, pucontentnamelength: *mut super::UINT, wszcontentname: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetMetaData(hissuancelicense : super::DRMPUBHANDLE, pucontentidlength : *mut super::UINT, wszcontentid : windows_core::PWSTR, pucontentidtypelength : *mut super::UINT, wszcontentidtype : windows_core::PWSTR, puskuidlength : *mut super::UINT, wszskuid : windows_core::PWSTR, puskuidtypelength : *mut super::UINT, wszskuidtype : windows_core::PWSTR, pucontenttypelength : *mut super::UINT, wszcontenttype : windows_core::PWSTR, pucontentnamelength : *mut super::UINT, wszcontentname : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetMetaData(hissuancelicense, pucontentidlength as _, wszcontentid.unwrap_or(core::mem::zeroed()) as _, pucontentidtypelength as _, wszcontentidtype.unwrap_or(core::mem::zeroed()) as _, puskuidlength as _, wszskuid.unwrap_or(core::mem::zeroed()) as _, puskuidtypelength as _, wszskuidtype.unwrap_or(core::mem::zeroed()) as _, pucontenttypelength as _, wszcontenttype.unwrap_or(core::mem::zeroed()) as _, pucontentnamelength as _, wszcontentname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetNameAndDescription(hissuancelicense: super::DRMPUBHANDLE, uindex: super::UINT, pulcid: *mut super::UINT, punamelength: *mut super::UINT, wszname: Option<windows_core::PWSTR>, pudescriptionlength: *mut super::UINT, wszdescription: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetNameAndDescription(hissuancelicense : super::DRMPUBHANDLE, uindex : super::UINT, pulcid : *mut super::UINT, punamelength : *mut super::UINT, wszname : windows_core::PWSTR, pudescriptionlength : *mut super::UINT, wszdescription : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetNameAndDescription(hissuancelicense, uindex, pulcid as _, punamelength as _, wszname.unwrap_or(core::mem::zeroed()) as _, pudescriptionlength as _, wszdescription.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetOwnerLicense(hissuancelicense: super::DRMPUBHANDLE, puownerlicenselength: *mut super::UINT, wszownerlicense: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetOwnerLicense(hissuancelicense : super::DRMPUBHANDLE, puownerlicenselength : *mut super::UINT, wszownerlicense : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetOwnerLicense(hissuancelicense, puownerlicenselength as _, wszownerlicense.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetProcAddress<P1>(hlibrary: super::DRMHANDLE, wszprocname: P1, ppfnprocaddress: *mut super::FARPROC) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetProcAddress(hlibrary : super::DRMHANDLE, wszprocname : windows_core::PCWSTR, ppfnprocaddress : *mut super::FARPROC) -> windows_core::HRESULT);
    unsafe { DRMGetProcAddress(hlibrary, wszprocname.param().abi(), ppfnprocaddress as _) }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetRevocationPoint(hissuancelicense: super::DRMPUBHANDLE, puidlength: *mut super::UINT, wszid: Option<windows_core::PWSTR>, puidtypelength: *mut super::UINT, wszidtype: Option<windows_core::PWSTR>, puurllength: *mut super::UINT, wszrl: Option<windows_core::PWSTR>, pstfrequency: *mut super::SYSTEMTIME, punamelength: *mut super::UINT, wszname: Option<windows_core::PWSTR>, pupublickeylength: *mut super::UINT, wszpublickey: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetRevocationPoint(hissuancelicense : super::DRMPUBHANDLE, puidlength : *mut super::UINT, wszid : windows_core::PWSTR, puidtypelength : *mut super::UINT, wszidtype : windows_core::PWSTR, puurllength : *mut super::UINT, wszrl : windows_core::PWSTR, pstfrequency : *mut super::SYSTEMTIME, punamelength : *mut super::UINT, wszname : windows_core::PWSTR, pupublickeylength : *mut super::UINT, wszpublickey : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetRevocationPoint(hissuancelicense, puidlength as _, wszid.unwrap_or(core::mem::zeroed()) as _, puidtypelength as _, wszidtype.unwrap_or(core::mem::zeroed()) as _, puurllength as _, wszrl.unwrap_or(core::mem::zeroed()) as _, pstfrequency as _, punamelength as _, wszname.unwrap_or(core::mem::zeroed()) as _, pupublickeylength as _, wszpublickey.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetRightExtendedInfo(hright: super::DRMPUBHANDLE, uindex: super::UINT, puextendedinfonamelength: *mut super::UINT, wszextendedinfoname: Option<windows_core::PWSTR>, puextendedinfovaluelength: *mut super::UINT, wszextendedinfovalue: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetRightExtendedInfo(hright : super::DRMPUBHANDLE, uindex : super::UINT, puextendedinfonamelength : *mut super::UINT, wszextendedinfoname : windows_core::PWSTR, puextendedinfovaluelength : *mut super::UINT, wszextendedinfovalue : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetRightExtendedInfo(hright, uindex, puextendedinfonamelength as _, wszextendedinfoname.unwrap_or(core::mem::zeroed()) as _, puextendedinfovaluelength as _, wszextendedinfovalue.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetRightInfo(hright: super::DRMPUBHANDLE, purightnamelength: *mut super::UINT, wszrightname: Option<windows_core::PWSTR>, pstfrom: *mut super::SYSTEMTIME, pstuntil: *mut super::SYSTEMTIME) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetRightInfo(hright : super::DRMPUBHANDLE, purightnamelength : *mut super::UINT, wszrightname : windows_core::PWSTR, pstfrom : *mut super::SYSTEMTIME, pstuntil : *mut super::SYSTEMTIME) -> windows_core::HRESULT);
    unsafe { DRMGetRightInfo(hright, purightnamelength as _, wszrightname.unwrap_or(core::mem::zeroed()) as _, pstfrom as _, pstuntil as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DRMGetSecurityProvider(uflags: super::UINT, putypelen: *mut super::UINT, wsztype: Option<windows_core::PWSTR>, pupathlen: *mut super::UINT, wszpath: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetSecurityProvider(uflags : super::UINT, putypelen : *mut super::UINT, wsztype : windows_core::PWSTR, pupathlen : *mut super::UINT, wszpath : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetSecurityProvider(uflags, putypelen as _, wsztype.unwrap_or(core::mem::zeroed()) as _, pupathlen as _, wszpath.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetServiceLocation<P3>(hclient: super::DRMHSESSION, uservicetype: super::UINT, uservicelocation: super::UINT, wszissuancelicense: P3, puserviceurllength: *mut super::UINT, wszserviceurl: Option<windows_core::PWSTR>) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetServiceLocation(hclient : super::DRMHSESSION, uservicetype : super::UINT, uservicelocation : super::UINT, wszissuancelicense : windows_core::PCWSTR, puserviceurllength : *mut super::UINT, wszserviceurl : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetServiceLocation(hclient, uservicetype, uservicelocation, wszissuancelicense.param().abi(), puserviceurllength as _, wszserviceurl.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicense<P5, P6, P8>(henv: super::DRMENVHANDLE, hissuancelicense: super::DRMPUBHANDLE, uflags: super::UINT, pbsymkey: *mut super::BYTE, cbsymkey: super::UINT, wszsymkeytype: P5, wszclientlicensorcertificate: P6, pfncallback: super::DRMCALLBACK, wszurl: P8, pvcontext: *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicense(henv : super::DRMENVHANDLE, hissuancelicense : super::DRMPUBHANDLE, uflags : super::UINT, pbsymkey : *mut super::BYTE, cbsymkey : super::UINT, wszsymkeytype : windows_core::PCWSTR, wszclientlicensorcertificate : windows_core::PCWSTR, pfncallback : super::DRMCALLBACK, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { DRMGetSignedIssuanceLicense(henv, hissuancelicense, uflags, pbsymkey as _, cbsymkey, wszsymkeytype.param().abi(), wszclientlicensorcertificate.param().abi(), pfncallback, wszurl.param().abi(), pvcontext as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicenseEx<P5>(henv: super::DRMENVHANDLE, hissuancelicense: super::DRMPUBHANDLE, uflags: super::UINT, pbsymkey: Option<*const super::BYTE>, cbsymkey: super::UINT, wszsymkeytype: P5, pvreserved: Option<super::PVOID>, henablingprincipal: super::DRMHANDLE, hboundlicenseclc: super::DRMHANDLE, pfncallback: super::DRMCALLBACK, pvcontext: super::PVOID) -> windows_core::HRESULT
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicenseEx(henv : super::DRMENVHANDLE, hissuancelicense : super::DRMPUBHANDLE, uflags : super::UINT, pbsymkey : *const super::BYTE, cbsymkey : super::UINT, wszsymkeytype : windows_core::PCWSTR, pvreserved : super::PVOID, henablingprincipal : super::DRMHANDLE, hboundlicenseclc : super::DRMHANDLE, pfncallback : super::DRMCALLBACK, pvcontext : super::PVOID) -> windows_core::HRESULT);
    unsafe { DRMGetSignedIssuanceLicenseEx(henv, hissuancelicense, uflags, pbsymkey.unwrap_or(core::mem::zeroed()) as _, cbsymkey, wszsymkeytype.param().abi(), pvreserved.unwrap_or(core::mem::zeroed()) as _, henablingprincipal, hboundlicenseclc, pfncallback, pvcontext) }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetTime(henv: super::DRMENVHANDLE, etimeridtype: super::DRMTIMETYPE, potimeobject: *mut super::SYSTEMTIME) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetTime(henv : super::DRMENVHANDLE, etimeridtype : super::DRMTIMETYPE, potimeobject : *mut super::SYSTEMTIME) -> windows_core::HRESULT);
    unsafe { DRMGetTime(henv, etimeridtype, potimeobject as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttribute<P1>(hqueryroot: super::DRMQUERYHANDLE, wszattributetype: P1, iwhich: super::UINT, peencoding: *mut super::DRMENCODINGTYPE, pcbuffer: *mut super::UINT, pbbuffer: *mut super::BYTE) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttribute(hqueryroot : super::DRMQUERYHANDLE, wszattributetype : windows_core::PCWSTR, iwhich : super::UINT, peencoding : *mut super::DRMENCODINGTYPE, pcbuffer : *mut super::UINT, pbbuffer : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { DRMGetUnboundLicenseAttribute(hqueryroot, wszattributetype.param().abi(), iwhich, peencoding as _, pcbuffer as _, pbbuffer as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttributeCount<P1>(hqueryroot: super::DRMQUERYHANDLE, wszattributetype: P1, pcattributes: *mut super::UINT) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttributeCount(hqueryroot : super::DRMQUERYHANDLE, wszattributetype : windows_core::PCWSTR, pcattributes : *mut super::UINT) -> windows_core::HRESULT);
    unsafe { DRMGetUnboundLicenseAttributeCount(hqueryroot, wszattributetype.param().abi(), pcattributes as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetUnboundLicenseObject<P1>(hqueryroot: super::DRMQUERYHANDLE, wszsubobjecttype: P1, iindex: super::UINT, phsubquery: *mut super::DRMQUERYHANDLE) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObject(hqueryroot : super::DRMQUERYHANDLE, wszsubobjecttype : windows_core::PCWSTR, iindex : super::UINT, phsubquery : *mut super::DRMQUERYHANDLE) -> windows_core::HRESULT);
    unsafe { DRMGetUnboundLicenseObject(hqueryroot, wszsubobjecttype.param().abi(), iindex, phsubquery as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetUnboundLicenseObjectCount<P1>(hqueryroot: super::DRMQUERYHANDLE, wszsubobjecttype: P1, pcsubobjects: *mut super::UINT) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObjectCount(hqueryroot : super::DRMQUERYHANDLE, wszsubobjecttype : windows_core::PCWSTR, pcsubobjects : *mut super::UINT) -> windows_core::HRESULT);
    unsafe { DRMGetUnboundLicenseObjectCount(hqueryroot, wszsubobjecttype.param().abi(), pcsubobjects as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetUsagePolicy(hissuancelicense: super::DRMPUBHANDLE, uindex: super::UINT, peusagepolicytype: *mut super::DRM_USAGEPOLICY_TYPE, pfexclusion: *mut windows_core::BOOL, punamelength: *mut super::UINT, wszname: Option<windows_core::PWSTR>, puminversionlength: *mut super::UINT, wszminversion: Option<windows_core::PWSTR>, pumaxversionlength: *mut super::UINT, wszmaxversion: Option<windows_core::PWSTR>, pupublickeylength: *mut super::UINT, wszpublickey: Option<windows_core::PWSTR>, pudigestalgorithmlength: *mut super::UINT, wszdigestalgorithm: Option<windows_core::PWSTR>, pcbdigest: *mut super::UINT, pbdigest: *mut super::BYTE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetUsagePolicy(hissuancelicense : super::DRMPUBHANDLE, uindex : super::UINT, peusagepolicytype : *mut super::DRM_USAGEPOLICY_TYPE, pfexclusion : *mut windows_core::BOOL, punamelength : *mut super::UINT, wszname : windows_core::PWSTR, puminversionlength : *mut super::UINT, wszminversion : windows_core::PWSTR, pumaxversionlength : *mut super::UINT, wszmaxversion : windows_core::PWSTR, pupublickeylength : *mut super::UINT, wszpublickey : windows_core::PWSTR, pudigestalgorithmlength : *mut super::UINT, wszdigestalgorithm : windows_core::PWSTR, pcbdigest : *mut super::UINT, pbdigest : *mut super::BYTE) -> windows_core::HRESULT);
    unsafe { DRMGetUsagePolicy(hissuancelicense, uindex, peusagepolicytype as _, pfexclusion as _, punamelength as _, wszname.unwrap_or(core::mem::zeroed()) as _, puminversionlength as _, wszminversion.unwrap_or(core::mem::zeroed()) as _, pumaxversionlength as _, wszmaxversion.unwrap_or(core::mem::zeroed()) as _, pupublickeylength as _, wszpublickey.unwrap_or(core::mem::zeroed()) as _, pudigestalgorithmlength as _, wszdigestalgorithm.unwrap_or(core::mem::zeroed()) as _, pcbdigest as _, pbdigest as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetUserInfo(huser: super::DRMPUBHANDLE, puusernamelength: *mut super::UINT, wszusername: Option<windows_core::PWSTR>, puuseridlength: *mut super::UINT, wszuserid: Option<windows_core::PWSTR>, puuseridtypelength: *mut super::UINT, wszuseridtype: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetUserInfo(huser : super::DRMPUBHANDLE, puusernamelength : *mut super::UINT, wszusername : windows_core::PWSTR, puuseridlength : *mut super::UINT, wszuserid : windows_core::PWSTR, puuseridtypelength : *mut super::UINT, wszuseridtype : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMGetUserInfo(huser, puusernamelength as _, wszusername.unwrap_or(core::mem::zeroed()) as _, puuseridlength as _, wszuserid.unwrap_or(core::mem::zeroed()) as _, puuseridtypelength as _, wszuseridtype.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetUserRights(hissuancelicense: super::DRMPUBHANDLE, huser: super::DRMPUBHANDLE, uindex: super::UINT, phright: *mut super::DRMPUBHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetUserRights(hissuancelicense : super::DRMPUBHANDLE, huser : super::DRMPUBHANDLE, uindex : super::UINT, phright : *mut super::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMGetUserRights(hissuancelicense, huser, uindex, phright as _) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMGetUsers(hissuancelicense: super::DRMPUBHANDLE, uindex: super::UINT, phuser: *mut super::DRMPUBHANDLE) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMGetUsers(hissuancelicense : super::DRMPUBHANDLE, uindex : super::UINT, phuser : *mut super::DRMPUBHANDLE) -> windows_core::HRESULT);
    unsafe { DRMGetUsers(hissuancelicense, uindex, phuser as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMInitEnvironment<P2, P3, P4>(esecurityprovidertype: super::DRMSECURITYPROVIDERTYPE, especification: super::DRMSPECTYPE, wszsecurityprovider: P2, wszmanifestcredentials: P3, wszmachinecredentials: P4, phenv: *mut super::DRMENVHANDLE, phdefaultlibrary: *mut super::DRMHANDLE) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMInitEnvironment(esecurityprovidertype : super::DRMSECURITYPROVIDERTYPE, especification : super::DRMSPECTYPE, wszsecurityprovider : windows_core::PCWSTR, wszmanifestcredentials : windows_core::PCWSTR, wszmachinecredentials : windows_core::PCWSTR, phenv : *mut super::DRMENVHANDLE, phdefaultlibrary : *mut super::DRMHANDLE) -> windows_core::HRESULT);
    unsafe { DRMInitEnvironment(esecurityprovidertype, especification, wszsecurityprovider.param().abi(), wszmanifestcredentials.param().abi(), wszmachinecredentials.param().abi(), phenv as _, phdefaultlibrary as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn DRMIsWindowProtected(hwnd: super::HWND, pfprotected: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMIsWindowProtected(hwnd : super::HWND, pfprotected : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { DRMIsWindowProtected(hwnd as _, pfprotected as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMLoadLibrary<P2, P3>(henv: super::DRMENVHANDLE, especification: super::DRMSPECTYPE, wszlibraryprovider: P2, wszcredentials: P3, phlibrary: *mut super::DRMHANDLE) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMLoadLibrary(henv : super::DRMENVHANDLE, especification : super::DRMSPECTYPE, wszlibraryprovider : windows_core::PCWSTR, wszcredentials : windows_core::PCWSTR, phlibrary : *mut super::DRMHANDLE) -> windows_core::HRESULT);
    unsafe { DRMLoadLibrary(henv, especification, wszlibraryprovider.param().abi(), wszcredentials.param().abi(), phlibrary as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMParseUnboundLicense<P0>(wszcertificate: P0, phqueryroot: *mut super::DRMQUERYHANDLE) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMParseUnboundLicense(wszcertificate : windows_core::PCWSTR, phqueryroot : *mut super::DRMQUERYHANDLE) -> windows_core::HRESULT);
    unsafe { DRMParseUnboundLicense(wszcertificate.param().abi(), phqueryroot as _) }
}
#[inline]
pub unsafe fn DRMRegisterContent(fregister: windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMRegisterContent(fregister : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { DRMRegisterContent(fregister) }
}
#[cfg(all(feature = "msdrmdefs", feature = "windef"))]
#[inline]
pub unsafe fn DRMRegisterProtectedWindow(henv: super::DRMENVHANDLE, hwnd: super::HWND) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMRegisterProtectedWindow(henv : super::DRMENVHANDLE, hwnd : super::HWND) -> windows_core::HRESULT);
    unsafe { DRMRegisterProtectedWindow(henv, hwnd as _) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMRegisterRevocationList<P1>(henv: super::DRMENVHANDLE, wszrevocationlist: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMRegisterRevocationList(henv : super::DRMENVHANDLE, wszrevocationlist : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMRegisterRevocationList(henv, wszrevocationlist.param().abi()) }
}
#[inline]
pub unsafe fn DRMRepair() -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMRepair() -> windows_core::HRESULT);
    unsafe { DRMRepair() }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMSetApplicationSpecificData<P2, P3>(hissuancelicense: super::DRMPUBHANDLE, fdelete: windows_core::BOOL, wszname: P2, wszvalue: P3) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMSetApplicationSpecificData(hissuancelicense : super::DRMPUBHANDLE, fdelete : windows_core::BOOL, wszname : windows_core::PCWSTR, wszvalue : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMSetApplicationSpecificData(hissuancelicense, fdelete, wszname.param().abi(), wszvalue.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMSetGlobalOptions(eglobaloptions: super::DRMGLOBALOPTIONS, pvdata: super::LPVOID, dwlen: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMSetGlobalOptions(eglobaloptions : super::DRMGLOBALOPTIONS, pvdata : super::LPVOID, dwlen : super::DWORD) -> windows_core::HRESULT);
    unsafe { DRMSetGlobalOptions(eglobaloptions, pvdata as _, dwlen) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMSetIntervalTime(hissuancelicense: super::DRMPUBHANDLE, cdays: super::UINT) -> windows_core::HRESULT {
    windows_core::link!("msdrm.dll" "system" fn DRMSetIntervalTime(hissuancelicense : super::DRMPUBHANDLE, cdays : super::UINT) -> windows_core::HRESULT);
    unsafe { DRMSetIntervalTime(hissuancelicense, cdays) }
}
#[cfg(feature = "msdrmdefs")]
#[inline]
pub unsafe fn DRMSetMetaData<P1, P2, P3, P4, P5, P6>(hissuancelicense: super::DRMPUBHANDLE, wszcontentid: P1, wszcontentidtype: P2, wszskuid: P3, wszskuidtype: P4, wszcontenttype: P5, wszcontentname: P6) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMSetMetaData(hissuancelicense : super::DRMPUBHANDLE, wszcontentid : windows_core::PCWSTR, wszcontentidtype : windows_core::PCWSTR, wszskuid : windows_core::PCWSTR, wszskuidtype : windows_core::PCWSTR, wszcontenttype : windows_core::PCWSTR, wszcontentname : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMSetMetaData(hissuancelicense, wszcontentid.param().abi(), wszcontentidtype.param().abi(), wszskuid.param().abi(), wszskuidtype.param().abi(), wszcontenttype.param().abi(), wszcontentname.param().abi()) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMSetNameAndDescription<P3, P4>(hissuancelicense: super::DRMPUBHANDLE, fdelete: windows_core::BOOL, lcid: super::UINT, wszname: P3, wszdescription: P4) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMSetNameAndDescription(hissuancelicense : super::DRMPUBHANDLE, fdelete : windows_core::BOOL, lcid : super::UINT, wszname : windows_core::PCWSTR, wszdescription : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMSetNameAndDescription(hissuancelicense, fdelete, lcid, wszname.param().abi(), wszdescription.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMSetRevocationPoint<P2, P3, P4, P6, P7>(hissuancelicense: super::DRMPUBHANDLE, fdelete: windows_core::BOOL, wszid: P2, wszidtype: P3, wszurl: P4, pstfrequency: *mut super::SYSTEMTIME, wszname: P6, wszpublickey: P7) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMSetRevocationPoint(hissuancelicense : super::DRMPUBHANDLE, fdelete : windows_core::BOOL, wszid : windows_core::PCWSTR, wszidtype : windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pstfrequency : *mut super::SYSTEMTIME, wszname : windows_core::PCWSTR, wszpublickey : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DRMSetRevocationPoint(hissuancelicense, fdelete, wszid.param().abi(), wszidtype.param().abi(), wszurl.param().abi(), pstfrequency as _, wszname.param().abi(), wszpublickey.param().abi()) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMSetUsagePolicy<P4, P5, P6, P7, P8>(hissuancelicense: super::DRMPUBHANDLE, eusagepolicytype: super::DRM_USAGEPOLICY_TYPE, fdelete: windows_core::BOOL, fexclusion: windows_core::BOOL, wszname: P4, wszminversion: P5, wszmaxversion: P6, wszpublickey: P7, wszdigestalgorithm: P8, pbdigest: *mut super::BYTE, cbdigest: super::UINT) -> windows_core::HRESULT
where
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMSetUsagePolicy(hissuancelicense : super::DRMPUBHANDLE, eusagepolicytype : super::DRM_USAGEPOLICY_TYPE, fdelete : windows_core::BOOL, fexclusion : windows_core::BOOL, wszname : windows_core::PCWSTR, wszminversion : windows_core::PCWSTR, wszmaxversion : windows_core::PCWSTR, wszpublickey : windows_core::PCWSTR, wszdigestalgorithm : windows_core::PCWSTR, pbdigest : *mut super::BYTE, cbdigest : super::UINT) -> windows_core::HRESULT);
    unsafe { DRMSetUsagePolicy(hissuancelicense, eusagepolicytype, fdelete, fexclusion, wszname.param().abi(), wszminversion.param().abi(), wszmaxversion.param().abi(), wszpublickey.param().abi(), wszdigestalgorithm.param().abi(), pbdigest as _, cbdigest) }
}
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
#[inline]
pub unsafe fn DRMVerify<P0>(wszdata: P0, pcattesteddata: *mut super::UINT, wszattesteddata: Option<windows_core::PWSTR>, petype: *mut super::DRMATTESTTYPE, pcprincipal: *mut super::UINT, wszprincipal: Option<windows_core::PWSTR>, pcmanifest: *mut super::UINT, wszmanifest: Option<windows_core::PWSTR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("msdrm.dll" "system" fn DRMVerify(wszdata : windows_core::PCWSTR, pcattesteddata : *mut super::UINT, wszattesteddata : windows_core::PWSTR, petype : *mut super::DRMATTESTTYPE, pcprincipal : *mut super::UINT, wszprincipal : windows_core::PWSTR, pcmanifest : *mut super::UINT, wszmanifest : windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { DRMVerify(wszdata.param().abi(), pcattesteddata as _, wszattesteddata.unwrap_or(core::mem::zeroed()) as _, petype as _, pcprincipal as _, wszprincipal.unwrap_or(core::mem::zeroed()) as _, pcmanifest as _, wszmanifest.unwrap_or(core::mem::zeroed()) as _) }
}
