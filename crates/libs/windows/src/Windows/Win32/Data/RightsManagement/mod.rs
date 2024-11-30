#[inline]
pub unsafe fn DRMAcquireAdvisories<P1, P2>(hlicensestorage: u32, wszlicense: P1, wszurl: P2, pvcontext: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMAcquireAdvisories(hlicensestorage : u32, wszlicense : windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    DRMAcquireAdvisories(core::mem::transmute(hlicensestorage), wszlicense.param().abi(), wszurl.param().abi(), core::mem::transmute(pvcontext)).ok()
}
#[inline]
pub unsafe fn DRMAcquireIssuanceLicenseTemplate<P5>(hclient: u32, uflags: u32, pvreserved: *mut core::ffi::c_void, pwsztemplateids: Option<&[windows_core::PCWSTR]>, wszurl: P5, pvcontext: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMAcquireIssuanceLicenseTemplate(hclient : u32, uflags : u32, pvreserved : *mut core::ffi::c_void, ctemplates : u32, pwsztemplateids : *const windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    DRMAcquireIssuanceLicenseTemplate(core::mem::transmute(hclient), core::mem::transmute(uflags), core::mem::transmute(pvreserved), pwsztemplateids.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pwsztemplateids.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), wszurl.param().abi(), core::mem::transmute(pvcontext)).ok()
}
#[inline]
pub unsafe fn DRMAcquireLicense<P2, P3, P4, P5>(hsession: u32, uflags: u32, wszgroupidentitycredential: P2, wszrequestedrights: P3, wszcustomdata: P4, wszurl: P5, pvcontext: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMAcquireLicense(hsession : u32, uflags : u32, wszgroupidentitycredential : windows_core::PCWSTR, wszrequestedrights : windows_core::PCWSTR, wszcustomdata : windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    DRMAcquireLicense(core::mem::transmute(hsession), core::mem::transmute(uflags), wszgroupidentitycredential.param().abi(), wszrequestedrights.param().abi(), wszcustomdata.param().abi(), wszurl.param().abi(), core::mem::transmute(pvcontext)).ok()
}
#[inline]
pub unsafe fn DRMActivate<P5>(hclient: u32, uflags: u32, ulangid: u32, pactservinfo: *mut DRM_ACTSERV_INFO, pvcontext: *mut core::ffi::c_void, hparentwnd: P5) -> windows_core::Result<()>
where
    P5: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMActivate(hclient : u32, uflags : u32, ulangid : u32, pactservinfo : *mut DRM_ACTSERV_INFO, pvcontext : *mut core::ffi::c_void, hparentwnd : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    DRMActivate(core::mem::transmute(hclient), core::mem::transmute(uflags), core::mem::transmute(ulangid), core::mem::transmute(pactservinfo), core::mem::transmute(pvcontext), hparentwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMAddLicense<P2>(hlicensestorage: u32, uflags: u32, wszlicense: P2) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMAddLicense(hlicensestorage : u32, uflags : u32, wszlicense : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMAddLicense(core::mem::transmute(hlicensestorage), core::mem::transmute(uflags), wszlicense.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMAddRightWithUser(hissuancelicense: u32, hright: u32, huser: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMAddRightWithUser(hissuancelicense : u32, hright : u32, huser : u32) -> windows_core::HRESULT);
    DRMAddRightWithUser(core::mem::transmute(hissuancelicense), core::mem::transmute(hright), core::mem::transmute(huser)).ok()
}
#[inline]
pub unsafe fn DRMAttest<P1>(henablingprincipal: u32, wszdata: P1, etype: DRMATTESTTYPE, pcattestedblob: *mut u32, wszattestedblob: windows_core::PWSTR) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMAttest(henablingprincipal : u32, wszdata : windows_core::PCWSTR, etype : DRMATTESTTYPE, pcattestedblob : *mut u32, wszattestedblob : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMAttest(core::mem::transmute(henablingprincipal), wszdata.param().abi(), core::mem::transmute(etype), core::mem::transmute(pcattestedblob), core::mem::transmute(wszattestedblob)).ok()
}
#[inline]
pub unsafe fn DRMCheckSecurity(henv: u32, clevel: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMCheckSecurity(henv : u32, clevel : u32) -> windows_core::HRESULT);
    DRMCheckSecurity(core::mem::transmute(henv), core::mem::transmute(clevel)).ok()
}
#[inline]
pub unsafe fn DRMClearAllRights(hissuancelicense: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMClearAllRights(hissuancelicense : u32) -> windows_core::HRESULT);
    DRMClearAllRights(core::mem::transmute(hissuancelicense)).ok()
}
#[inline]
pub unsafe fn DRMCloseEnvironmentHandle(henv: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMCloseEnvironmentHandle(henv : u32) -> windows_core::HRESULT);
    DRMCloseEnvironmentHandle(core::mem::transmute(henv)).ok()
}
#[inline]
pub unsafe fn DRMCloseHandle(handle: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMCloseHandle(handle : u32) -> windows_core::HRESULT);
    DRMCloseHandle(core::mem::transmute(handle)).ok()
}
#[inline]
pub unsafe fn DRMClosePubHandle(hpub: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMClosePubHandle(hpub : u32) -> windows_core::HRESULT);
    DRMClosePubHandle(core::mem::transmute(hpub)).ok()
}
#[inline]
pub unsafe fn DRMCloseQueryHandle(hquery: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMCloseQueryHandle(hquery : u32) -> windows_core::HRESULT);
    DRMCloseQueryHandle(core::mem::transmute(hquery)).ok()
}
#[inline]
pub unsafe fn DRMCloseSession(hsession: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMCloseSession(hsession : u32) -> windows_core::HRESULT);
    DRMCloseSession(core::mem::transmute(hsession)).ok()
}
#[inline]
pub unsafe fn DRMConstructCertificateChain(rgwszcertificates: &[windows_core::PCWSTR], pcchain: *mut u32, wszchain: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMConstructCertificateChain(ccertificates : u32, rgwszcertificates : *const windows_core::PCWSTR, pcchain : *mut u32, wszchain : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMConstructCertificateChain(rgwszcertificates.len().try_into().unwrap(), core::mem::transmute(rgwszcertificates.as_ptr()), core::mem::transmute(pcchain), core::mem::transmute(wszchain)).ok()
}
#[inline]
pub unsafe fn DRMCreateBoundLicense<P2>(henv: u32, pparams: *mut DRMBOUNDLICENSEPARAMS, wszlicensechain: P2, phboundlicense: *mut u32, pherrorlog: *mut u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateBoundLicense(henv : u32, pparams : *mut DRMBOUNDLICENSEPARAMS, wszlicensechain : windows_core::PCWSTR, phboundlicense : *mut u32, pherrorlog : *mut u32) -> windows_core::HRESULT);
    DRMCreateBoundLicense(core::mem::transmute(henv), core::mem::transmute(pparams), wszlicensechain.param().abi(), core::mem::transmute(phboundlicense), core::mem::transmute(pherrorlog)).ok()
}
#[inline]
pub unsafe fn DRMCreateClientSession<P2, P3>(pfncallback: DRMCALLBACK, ucallbackversion: u32, wszgroupidprovidertype: P2, wszgroupid: P3, phclient: *mut u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateClientSession(pfncallback : DRMCALLBACK, ucallbackversion : u32, wszgroupidprovidertype : windows_core::PCWSTR, wszgroupid : windows_core::PCWSTR, phclient : *mut u32) -> windows_core::HRESULT);
    DRMCreateClientSession(core::mem::transmute(pfncallback), core::mem::transmute(ucallbackversion), wszgroupidprovidertype.param().abi(), wszgroupid.param().abi(), core::mem::transmute(phclient)).ok()
}
#[inline]
pub unsafe fn DRMCreateEnablingBitsDecryptor<P1, P3>(hboundlicense: u32, wszright: P1, hauxlib: u32, wszauxplug: P3, phdecryptor: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsDecryptor(hboundlicense : u32, wszright : windows_core::PCWSTR, hauxlib : u32, wszauxplug : windows_core::PCWSTR, phdecryptor : *mut u32) -> windows_core::HRESULT);
    DRMCreateEnablingBitsDecryptor(core::mem::transmute(hboundlicense), wszright.param().abi(), core::mem::transmute(hauxlib), wszauxplug.param().abi(), core::mem::transmute(phdecryptor)).ok()
}
#[inline]
pub unsafe fn DRMCreateEnablingBitsEncryptor<P1, P3>(hboundlicense: u32, wszright: P1, hauxlib: u32, wszauxplug: P3, phencryptor: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsEncryptor(hboundlicense : u32, wszright : windows_core::PCWSTR, hauxlib : u32, wszauxplug : windows_core::PCWSTR, phencryptor : *mut u32) -> windows_core::HRESULT);
    DRMCreateEnablingBitsEncryptor(core::mem::transmute(hboundlicense), wszright.param().abi(), core::mem::transmute(hauxlib), wszauxplug.param().abi(), core::mem::transmute(phencryptor)).ok()
}
#[inline]
pub unsafe fn DRMCreateEnablingPrincipal<P2, P4>(henv: u32, hlibrary: u32, wszobject: P2, pidprincipal: *mut DRMID, wszcredentials: P4, phenablingprincipal: *mut u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateEnablingPrincipal(henv : u32, hlibrary : u32, wszobject : windows_core::PCWSTR, pidprincipal : *mut DRMID, wszcredentials : windows_core::PCWSTR, phenablingprincipal : *mut u32) -> windows_core::HRESULT);
    DRMCreateEnablingPrincipal(core::mem::transmute(henv), core::mem::transmute(hlibrary), wszobject.param().abi(), core::mem::transmute(pidprincipal), wszcredentials.param().abi(), core::mem::transmute(phenablingprincipal)).ok()
}
#[inline]
pub unsafe fn DRMCreateIssuanceLicense<P2, P3, P5>(psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, wszreferralinfoname: P2, wszreferralinfourl: P3, howner: u32, wszissuancelicense: P5, hboundlicense: u32, phissuancelicense: *mut u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateIssuanceLicense(psttimefrom : *mut super::super::Foundation:: SYSTEMTIME, psttimeuntil : *mut super::super::Foundation:: SYSTEMTIME, wszreferralinfoname : windows_core::PCWSTR, wszreferralinfourl : windows_core::PCWSTR, howner : u32, wszissuancelicense : windows_core::PCWSTR, hboundlicense : u32, phissuancelicense : *mut u32) -> windows_core::HRESULT);
    DRMCreateIssuanceLicense(core::mem::transmute(psttimefrom), core::mem::transmute(psttimeuntil), wszreferralinfoname.param().abi(), wszreferralinfourl.param().abi(), core::mem::transmute(howner), wszissuancelicense.param().abi(), core::mem::transmute(hboundlicense), core::mem::transmute(phissuancelicense)).ok()
}
#[inline]
pub unsafe fn DRMCreateLicenseStorageSession<P4>(henv: u32, hdefaultlibrary: u32, hclient: u32, uflags: u32, wszissuancelicense: P4, phlicensestorage: *mut u32) -> windows_core::Result<()>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateLicenseStorageSession(henv : u32, hdefaultlibrary : u32, hclient : u32, uflags : u32, wszissuancelicense : windows_core::PCWSTR, phlicensestorage : *mut u32) -> windows_core::HRESULT);
    DRMCreateLicenseStorageSession(core::mem::transmute(henv), core::mem::transmute(hdefaultlibrary), core::mem::transmute(hclient), core::mem::transmute(uflags), wszissuancelicense.param().abi(), core::mem::transmute(phlicensestorage)).ok()
}
#[inline]
pub unsafe fn DRMCreateRight<P0>(wszrightname: P0, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME, cextendedinfo: u32, pwszextendedinfoname: Option<*const windows_core::PCWSTR>, pwszextendedinfovalue: Option<*const windows_core::PCWSTR>, phright: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateRight(wszrightname : windows_core::PCWSTR, pstfrom : *mut super::super::Foundation:: SYSTEMTIME, pstuntil : *mut super::super::Foundation:: SYSTEMTIME, cextendedinfo : u32, pwszextendedinfoname : *const windows_core::PCWSTR, pwszextendedinfovalue : *const windows_core::PCWSTR, phright : *mut u32) -> windows_core::HRESULT);
    DRMCreateRight(wszrightname.param().abi(), core::mem::transmute(pstfrom), core::mem::transmute(pstuntil), core::mem::transmute(cextendedinfo), core::mem::transmute(pwszextendedinfoname.unwrap_or(core::ptr::null())), core::mem::transmute(pwszextendedinfovalue.unwrap_or(core::ptr::null())), core::mem::transmute(phright)).ok()
}
#[inline]
pub unsafe fn DRMCreateUser<P0, P1, P2>(wszusername: P0, wszuserid: P1, wszuseridtype: P2, phuser: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMCreateUser(wszusername : windows_core::PCWSTR, wszuserid : windows_core::PCWSTR, wszuseridtype : windows_core::PCWSTR, phuser : *mut u32) -> windows_core::HRESULT);
    DRMCreateUser(wszusername.param().abi(), wszuserid.param().abi(), wszuseridtype.param().abi(), core::mem::transmute(phuser)).ok()
}
#[inline]
pub unsafe fn DRMDecode<P0, P1>(wszalgid: P0, wszencodedstring: P1, pudecodeddatalen: *mut u32, pbdecodeddata: *mut u8) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMDecode(wszalgid : windows_core::PCWSTR, wszencodedstring : windows_core::PCWSTR, pudecodeddatalen : *mut u32, pbdecodeddata : *mut u8) -> windows_core::HRESULT);
    DRMDecode(wszalgid.param().abi(), wszencodedstring.param().abi(), core::mem::transmute(pudecodeddatalen), core::mem::transmute(pbdecodeddata)).ok()
}
#[inline]
pub unsafe fn DRMDeconstructCertificateChain<P0>(wszchain: P0, iwhich: u32, pccert: *mut u32, wszcert: windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMDeconstructCertificateChain(wszchain : windows_core::PCWSTR, iwhich : u32, pccert : *mut u32, wszcert : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMDeconstructCertificateChain(wszchain.param().abi(), core::mem::transmute(iwhich), core::mem::transmute(pccert), core::mem::transmute(wszcert)).ok()
}
#[inline]
pub unsafe fn DRMDecrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMDecrypt(hcryptoprovider : u32, iposition : u32, cnuminbytes : u32, pbindata : *mut u8, pcnumoutbytes : *mut u32, pboutdata : *mut u8) -> windows_core::HRESULT);
    DRMDecrypt(core::mem::transmute(hcryptoprovider), core::mem::transmute(iposition), core::mem::transmute(cnuminbytes), core::mem::transmute(pbindata), core::mem::transmute(pcnumoutbytes), core::mem::transmute(pboutdata)).ok()
}
#[inline]
pub unsafe fn DRMDeleteLicense<P1>(hsession: u32, wszlicenseid: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMDeleteLicense(hsession : u32, wszlicenseid : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMDeleteLicense(core::mem::transmute(hsession), wszlicenseid.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMDuplicateEnvironmentHandle(htocopy: u32, phcopy: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMDuplicateEnvironmentHandle(htocopy : u32, phcopy : *mut u32) -> windows_core::HRESULT);
    DRMDuplicateEnvironmentHandle(core::mem::transmute(htocopy), core::mem::transmute(phcopy)).ok()
}
#[inline]
pub unsafe fn DRMDuplicateHandle(htocopy: u32, phcopy: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMDuplicateHandle(htocopy : u32, phcopy : *mut u32) -> windows_core::HRESULT);
    DRMDuplicateHandle(core::mem::transmute(htocopy), core::mem::transmute(phcopy)).ok()
}
#[inline]
pub unsafe fn DRMDuplicatePubHandle(hpubin: u32, phpubout: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMDuplicatePubHandle(hpubin : u32, phpubout : *mut u32) -> windows_core::HRESULT);
    DRMDuplicatePubHandle(core::mem::transmute(hpubin), core::mem::transmute(phpubout)).ok()
}
#[inline]
pub unsafe fn DRMDuplicateSession(hsessionin: u32, phsessionout: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMDuplicateSession(hsessionin : u32, phsessionout : *mut u32) -> windows_core::HRESULT);
    DRMDuplicateSession(core::mem::transmute(hsessionin), core::mem::transmute(phsessionout)).ok()
}
#[inline]
pub unsafe fn DRMEncode<P0>(wszalgid: P0, udatalen: u32, pbdecodeddata: *mut u8, puencodedstringlen: *mut u32, wszencodedstring: windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMEncode(wszalgid : windows_core::PCWSTR, udatalen : u32, pbdecodeddata : *mut u8, puencodedstringlen : *mut u32, wszencodedstring : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMEncode(wszalgid.param().abi(), core::mem::transmute(udatalen), core::mem::transmute(pbdecodeddata), core::mem::transmute(puencodedstringlen), core::mem::transmute(wszencodedstring)).ok()
}
#[inline]
pub unsafe fn DRMEncrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMEncrypt(hcryptoprovider : u32, iposition : u32, cnuminbytes : u32, pbindata : *mut u8, pcnumoutbytes : *mut u32, pboutdata : *mut u8) -> windows_core::HRESULT);
    DRMEncrypt(core::mem::transmute(hcryptoprovider), core::mem::transmute(iposition), core::mem::transmute(cnuminbytes), core::mem::transmute(pbindata), core::mem::transmute(pcnumoutbytes), core::mem::transmute(pboutdata)).ok()
}
#[inline]
pub unsafe fn DRMEnumerateLicense(hsession: u32, uflags: u32, uindex: u32, pfsharedflag: *mut super::super::Foundation::BOOL, pucertificatedatalen: *mut u32, wszcertificatedata: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMEnumerateLicense(hsession : u32, uflags : u32, uindex : u32, pfsharedflag : *mut super::super::Foundation:: BOOL, pucertificatedatalen : *mut u32, wszcertificatedata : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMEnumerateLicense(core::mem::transmute(hsession), core::mem::transmute(uflags), core::mem::transmute(uindex), core::mem::transmute(pfsharedflag), core::mem::transmute(pucertificatedatalen), core::mem::transmute(wszcertificatedata)).ok()
}
#[inline]
pub unsafe fn DRMGetApplicationSpecificData(hissuancelicense: u32, uindex: u32, punamelength: *mut u32, wszname: windows_core::PWSTR, puvaluelength: *mut u32, wszvalue: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetApplicationSpecificData(hissuancelicense : u32, uindex : u32, punamelength : *mut u32, wszname : windows_core::PWSTR, puvaluelength : *mut u32, wszvalue : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetApplicationSpecificData(core::mem::transmute(hissuancelicense), core::mem::transmute(uindex), core::mem::transmute(punamelength), core::mem::transmute(wszname), core::mem::transmute(puvaluelength), core::mem::transmute(wszvalue)).ok()
}
#[inline]
pub unsafe fn DRMGetBoundLicenseAttribute<P1>(hqueryroot: u32, wszattribute: P1, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttribute(hqueryroot : u32, wszattribute : windows_core::PCWSTR, iwhich : u32, peencoding : *mut DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_core::HRESULT);
    DRMGetBoundLicenseAttribute(core::mem::transmute(hqueryroot), wszattribute.param().abi(), core::mem::transmute(iwhich), core::mem::transmute(peencoding), core::mem::transmute(pcbuffer), core::mem::transmute(pbbuffer)).ok()
}
#[inline]
pub unsafe fn DRMGetBoundLicenseAttributeCount<P1>(hqueryroot: u32, wszattribute: P1, pcattributes: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttributeCount(hqueryroot : u32, wszattribute : windows_core::PCWSTR, pcattributes : *mut u32) -> windows_core::HRESULT);
    DRMGetBoundLicenseAttributeCount(core::mem::transmute(hqueryroot), wszattribute.param().abi(), core::mem::transmute(pcattributes)).ok()
}
#[inline]
pub unsafe fn DRMGetBoundLicenseObject<P1>(hqueryroot: u32, wszsubobjecttype: P1, iwhich: u32, phsubobject: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObject(hqueryroot : u32, wszsubobjecttype : windows_core::PCWSTR, iwhich : u32, phsubobject : *mut u32) -> windows_core::HRESULT);
    DRMGetBoundLicenseObject(core::mem::transmute(hqueryroot), wszsubobjecttype.param().abi(), core::mem::transmute(iwhich), core::mem::transmute(phsubobject)).ok()
}
#[inline]
pub unsafe fn DRMGetBoundLicenseObjectCount<P1>(hqueryroot: u32, wszsubobjecttype: P1, pcsubobjects: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObjectCount(hqueryroot : u32, wszsubobjecttype : windows_core::PCWSTR, pcsubobjects : *mut u32) -> windows_core::HRESULT);
    DRMGetBoundLicenseObjectCount(core::mem::transmute(hqueryroot), wszsubobjecttype.param().abi(), core::mem::transmute(pcsubobjects)).ok()
}
#[inline]
pub unsafe fn DRMGetCertificateChainCount<P0>(wszchain: P0, pccertcount: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetCertificateChainCount(wszchain : windows_core::PCWSTR, pccertcount : *mut u32) -> windows_core::HRESULT);
    DRMGetCertificateChainCount(wszchain.param().abi(), core::mem::transmute(pccertcount)).ok()
}
#[inline]
pub unsafe fn DRMGetClientVersion(pdrmclientversioninfo: *mut DRM_CLIENT_VERSION_INFO) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetClientVersion(pdrmclientversioninfo : *mut DRM_CLIENT_VERSION_INFO) -> windows_core::HRESULT);
    DRMGetClientVersion(core::mem::transmute(pdrmclientversioninfo)).ok()
}
#[inline]
pub unsafe fn DRMGetEnvironmentInfo<P1>(handle: u32, wszattribute: P1, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetEnvironmentInfo(handle : u32, wszattribute : windows_core::PCWSTR, peencoding : *mut DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_core::HRESULT);
    DRMGetEnvironmentInfo(core::mem::transmute(handle), wszattribute.param().abi(), core::mem::transmute(peencoding), core::mem::transmute(pcbuffer), core::mem::transmute(pbbuffer)).ok()
}
#[inline]
pub unsafe fn DRMGetInfo<P1>(handle: u32, wszattribute: P1, peencoding: *const DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetInfo(handle : u32, wszattribute : windows_core::PCWSTR, peencoding : *const DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_core::HRESULT);
    DRMGetInfo(core::mem::transmute(handle), wszattribute.param().abi(), core::mem::transmute(peencoding), core::mem::transmute(pcbuffer), core::mem::transmute(pbbuffer)).ok()
}
#[inline]
pub unsafe fn DRMGetIntervalTime(hissuancelicense: u32, pcdays: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetIntervalTime(hissuancelicense : u32, pcdays : *mut u32) -> windows_core::HRESULT);
    DRMGetIntervalTime(core::mem::transmute(hissuancelicense), core::mem::transmute(pcdays)).ok()
}
#[inline]
pub unsafe fn DRMGetIssuanceLicenseInfo(hissuancelicense: u32, psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, uflags: u32, pudistributionpointnamelength: *mut u32, wszdistributionpointname: windows_core::PWSTR, pudistributionpointurllength: *mut u32, wszdistributionpointurl: windows_core::PWSTR, phowner: *mut u32, pfofficial: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseInfo(hissuancelicense : u32, psttimefrom : *mut super::super::Foundation:: SYSTEMTIME, psttimeuntil : *mut super::super::Foundation:: SYSTEMTIME, uflags : u32, pudistributionpointnamelength : *mut u32, wszdistributionpointname : windows_core::PWSTR, pudistributionpointurllength : *mut u32, wszdistributionpointurl : windows_core::PWSTR, phowner : *mut u32, pfofficial : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    DRMGetIssuanceLicenseInfo(core::mem::transmute(hissuancelicense), core::mem::transmute(psttimefrom), core::mem::transmute(psttimeuntil), core::mem::transmute(uflags), core::mem::transmute(pudistributionpointnamelength), core::mem::transmute(wszdistributionpointname), core::mem::transmute(pudistributionpointurllength), core::mem::transmute(wszdistributionpointurl), core::mem::transmute(phowner), core::mem::transmute(pfofficial)).ok()
}
#[inline]
pub unsafe fn DRMGetIssuanceLicenseTemplate(hissuancelicense: u32, puissuancelicensetemplatelength: *mut u32, wszissuancelicensetemplate: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseTemplate(hissuancelicense : u32, puissuancelicensetemplatelength : *mut u32, wszissuancelicensetemplate : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetIssuanceLicenseTemplate(core::mem::transmute(hissuancelicense), core::mem::transmute(puissuancelicensetemplatelength), core::mem::transmute(wszissuancelicensetemplate)).ok()
}
#[inline]
pub unsafe fn DRMGetMetaData(hissuancelicense: u32, pucontentidlength: *mut u32, wszcontentid: windows_core::PWSTR, pucontentidtypelength: *mut u32, wszcontentidtype: windows_core::PWSTR, puskuidlength: *mut u32, wszskuid: windows_core::PWSTR, puskuidtypelength: *mut u32, wszskuidtype: windows_core::PWSTR, pucontenttypelength: *mut u32, wszcontenttype: windows_core::PWSTR, pucontentnamelength: *mut u32, wszcontentname: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetMetaData(hissuancelicense : u32, pucontentidlength : *mut u32, wszcontentid : windows_core::PWSTR, pucontentidtypelength : *mut u32, wszcontentidtype : windows_core::PWSTR, puskuidlength : *mut u32, wszskuid : windows_core::PWSTR, puskuidtypelength : *mut u32, wszskuidtype : windows_core::PWSTR, pucontenttypelength : *mut u32, wszcontenttype : windows_core::PWSTR, pucontentnamelength : *mut u32, wszcontentname : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetMetaData(
        core::mem::transmute(hissuancelicense),
        core::mem::transmute(pucontentidlength),
        core::mem::transmute(wszcontentid),
        core::mem::transmute(pucontentidtypelength),
        core::mem::transmute(wszcontentidtype),
        core::mem::transmute(puskuidlength),
        core::mem::transmute(wszskuid),
        core::mem::transmute(puskuidtypelength),
        core::mem::transmute(wszskuidtype),
        core::mem::transmute(pucontenttypelength),
        core::mem::transmute(wszcontenttype),
        core::mem::transmute(pucontentnamelength),
        core::mem::transmute(wszcontentname),
    )
    .ok()
}
#[inline]
pub unsafe fn DRMGetNameAndDescription(hissuancelicense: u32, uindex: u32, pulcid: *mut u32, punamelength: *mut u32, wszname: windows_core::PWSTR, pudescriptionlength: *mut u32, wszdescription: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetNameAndDescription(hissuancelicense : u32, uindex : u32, pulcid : *mut u32, punamelength : *mut u32, wszname : windows_core::PWSTR, pudescriptionlength : *mut u32, wszdescription : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetNameAndDescription(core::mem::transmute(hissuancelicense), core::mem::transmute(uindex), core::mem::transmute(pulcid), core::mem::transmute(punamelength), core::mem::transmute(wszname), core::mem::transmute(pudescriptionlength), core::mem::transmute(wszdescription)).ok()
}
#[inline]
pub unsafe fn DRMGetOwnerLicense(hissuancelicense: u32, puownerlicenselength: *mut u32, wszownerlicense: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetOwnerLicense(hissuancelicense : u32, puownerlicenselength : *mut u32, wszownerlicense : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetOwnerLicense(core::mem::transmute(hissuancelicense), core::mem::transmute(puownerlicenselength), core::mem::transmute(wszownerlicense)).ok()
}
#[inline]
pub unsafe fn DRMGetProcAddress<P1>(hlibrary: u32, wszprocname: P1, ppfnprocaddress: *mut super::super::Foundation::FARPROC) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetProcAddress(hlibrary : u32, wszprocname : windows_core::PCWSTR, ppfnprocaddress : *mut super::super::Foundation:: FARPROC) -> windows_core::HRESULT);
    DRMGetProcAddress(core::mem::transmute(hlibrary), wszprocname.param().abi(), core::mem::transmute(ppfnprocaddress)).ok()
}
#[inline]
pub unsafe fn DRMGetRevocationPoint(hissuancelicense: u32, puidlength: *mut u32, wszid: windows_core::PWSTR, puidtypelength: *mut u32, wszidtype: windows_core::PWSTR, puurllength: *mut u32, wszrl: windows_core::PWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, punamelength: *mut u32, wszname: windows_core::PWSTR, pupublickeylength: *mut u32, wszpublickey: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetRevocationPoint(hissuancelicense : u32, puidlength : *mut u32, wszid : windows_core::PWSTR, puidtypelength : *mut u32, wszidtype : windows_core::PWSTR, puurllength : *mut u32, wszrl : windows_core::PWSTR, pstfrequency : *mut super::super::Foundation:: SYSTEMTIME, punamelength : *mut u32, wszname : windows_core::PWSTR, pupublickeylength : *mut u32, wszpublickey : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetRevocationPoint(core::mem::transmute(hissuancelicense), core::mem::transmute(puidlength), core::mem::transmute(wszid), core::mem::transmute(puidtypelength), core::mem::transmute(wszidtype), core::mem::transmute(puurllength), core::mem::transmute(wszrl), core::mem::transmute(pstfrequency), core::mem::transmute(punamelength), core::mem::transmute(wszname), core::mem::transmute(pupublickeylength), core::mem::transmute(wszpublickey)).ok()
}
#[inline]
pub unsafe fn DRMGetRightExtendedInfo(hright: u32, uindex: u32, puextendedinfonamelength: *mut u32, wszextendedinfoname: windows_core::PWSTR, puextendedinfovaluelength: *mut u32, wszextendedinfovalue: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetRightExtendedInfo(hright : u32, uindex : u32, puextendedinfonamelength : *mut u32, wszextendedinfoname : windows_core::PWSTR, puextendedinfovaluelength : *mut u32, wszextendedinfovalue : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetRightExtendedInfo(core::mem::transmute(hright), core::mem::transmute(uindex), core::mem::transmute(puextendedinfonamelength), core::mem::transmute(wszextendedinfoname), core::mem::transmute(puextendedinfovaluelength), core::mem::transmute(wszextendedinfovalue)).ok()
}
#[inline]
pub unsafe fn DRMGetRightInfo(hright: u32, purightnamelength: *mut u32, wszrightname: windows_core::PWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetRightInfo(hright : u32, purightnamelength : *mut u32, wszrightname : windows_core::PWSTR, pstfrom : *mut super::super::Foundation:: SYSTEMTIME, pstuntil : *mut super::super::Foundation:: SYSTEMTIME) -> windows_core::HRESULT);
    DRMGetRightInfo(core::mem::transmute(hright), core::mem::transmute(purightnamelength), core::mem::transmute(wszrightname), core::mem::transmute(pstfrom), core::mem::transmute(pstuntil)).ok()
}
#[inline]
pub unsafe fn DRMGetSecurityProvider(uflags: u32, putypelen: *mut u32, wsztype: windows_core::PWSTR, pupathlen: *mut u32, wszpath: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetSecurityProvider(uflags : u32, putypelen : *mut u32, wsztype : windows_core::PWSTR, pupathlen : *mut u32, wszpath : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetSecurityProvider(core::mem::transmute(uflags), core::mem::transmute(putypelen), core::mem::transmute(wsztype), core::mem::transmute(pupathlen), core::mem::transmute(wszpath)).ok()
}
#[inline]
pub unsafe fn DRMGetServiceLocation<P3>(hclient: u32, uservicetype: u32, uservicelocation: u32, wszissuancelicense: P3, puserviceurllength: *mut u32, wszserviceurl: windows_core::PWSTR) -> windows_core::Result<()>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetServiceLocation(hclient : u32, uservicetype : u32, uservicelocation : u32, wszissuancelicense : windows_core::PCWSTR, puserviceurllength : *mut u32, wszserviceurl : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetServiceLocation(core::mem::transmute(hclient), core::mem::transmute(uservicetype), core::mem::transmute(uservicelocation), wszissuancelicense.param().abi(), core::mem::transmute(puserviceurllength), core::mem::transmute(wszserviceurl)).ok()
}
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicense<P5, P6, P8>(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *mut u8, cbsymkey: u32, wszsymkeytype: P5, wszclientlicensorcertificate: P6, pfncallback: DRMCALLBACK, wszurl: P8, pvcontext: *mut core::ffi::c_void) -> windows_core::Result<()>
where
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicense(henv : u32, hissuancelicense : u32, uflags : u32, pbsymkey : *mut u8, cbsymkey : u32, wszsymkeytype : windows_core::PCWSTR, wszclientlicensorcertificate : windows_core::PCWSTR, pfncallback : DRMCALLBACK, wszurl : windows_core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_core::HRESULT);
    DRMGetSignedIssuanceLicense(core::mem::transmute(henv), core::mem::transmute(hissuancelicense), core::mem::transmute(uflags), core::mem::transmute(pbsymkey), core::mem::transmute(cbsymkey), wszsymkeytype.param().abi(), wszclientlicensorcertificate.param().abi(), core::mem::transmute(pfncallback), wszurl.param().abi(), core::mem::transmute(pvcontext)).ok()
}
#[inline]
pub unsafe fn DRMGetSignedIssuanceLicenseEx<P5>(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: Option<&[u8]>, wszsymkeytype: P5, pvreserved: Option<*const core::ffi::c_void>, henablingprincipal: u32, hboundlicenseclc: u32, pfncallback: DRMCALLBACK, pvcontext: *const core::ffi::c_void) -> windows_core::Result<()>
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicenseEx(henv : u32, hissuancelicense : u32, uflags : u32, pbsymkey : *const u8, cbsymkey : u32, wszsymkeytype : windows_core::PCWSTR, pvreserved : *const core::ffi::c_void, henablingprincipal : u32, hboundlicenseclc : u32, pfncallback : DRMCALLBACK, pvcontext : *const core::ffi::c_void) -> windows_core::HRESULT);
    DRMGetSignedIssuanceLicenseEx(
        core::mem::transmute(henv),
        core::mem::transmute(hissuancelicense),
        core::mem::transmute(uflags),
        core::mem::transmute(pbsymkey.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        pbsymkey.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
        wszsymkeytype.param().abi(),
        core::mem::transmute(pvreserved.unwrap_or(core::ptr::null())),
        core::mem::transmute(henablingprincipal),
        core::mem::transmute(hboundlicenseclc),
        core::mem::transmute(pfncallback),
        core::mem::transmute(pvcontext),
    )
    .ok()
}
#[inline]
pub unsafe fn DRMGetTime(henv: u32, etimeridtype: DRMTIMETYPE, potimeobject: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetTime(henv : u32, etimeridtype : DRMTIMETYPE, potimeobject : *mut super::super::Foundation:: SYSTEMTIME) -> windows_core::HRESULT);
    DRMGetTime(core::mem::transmute(henv), core::mem::transmute(etimeridtype), core::mem::transmute(potimeobject)).ok()
}
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttribute<P1>(hqueryroot: u32, wszattributetype: P1, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttribute(hqueryroot : u32, wszattributetype : windows_core::PCWSTR, iwhich : u32, peencoding : *mut DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_core::HRESULT);
    DRMGetUnboundLicenseAttribute(core::mem::transmute(hqueryroot), wszattributetype.param().abi(), core::mem::transmute(iwhich), core::mem::transmute(peencoding), core::mem::transmute(pcbuffer), core::mem::transmute(pbbuffer)).ok()
}
#[inline]
pub unsafe fn DRMGetUnboundLicenseAttributeCount<P1>(hqueryroot: u32, wszattributetype: P1, pcattributes: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttributeCount(hqueryroot : u32, wszattributetype : windows_core::PCWSTR, pcattributes : *mut u32) -> windows_core::HRESULT);
    DRMGetUnboundLicenseAttributeCount(core::mem::transmute(hqueryroot), wszattributetype.param().abi(), core::mem::transmute(pcattributes)).ok()
}
#[inline]
pub unsafe fn DRMGetUnboundLicenseObject<P1>(hqueryroot: u32, wszsubobjecttype: P1, iindex: u32, phsubquery: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObject(hqueryroot : u32, wszsubobjecttype : windows_core::PCWSTR, iindex : u32, phsubquery : *mut u32) -> windows_core::HRESULT);
    DRMGetUnboundLicenseObject(core::mem::transmute(hqueryroot), wszsubobjecttype.param().abi(), core::mem::transmute(iindex), core::mem::transmute(phsubquery)).ok()
}
#[inline]
pub unsafe fn DRMGetUnboundLicenseObjectCount<P1>(hqueryroot: u32, wszsubobjecttype: P1, pcsubobjects: *mut u32) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObjectCount(hqueryroot : u32, wszsubobjecttype : windows_core::PCWSTR, pcsubobjects : *mut u32) -> windows_core::HRESULT);
    DRMGetUnboundLicenseObjectCount(core::mem::transmute(hqueryroot), wszsubobjecttype.param().abi(), core::mem::transmute(pcsubobjects)).ok()
}
#[inline]
pub unsafe fn DRMGetUsagePolicy(hissuancelicense: u32, uindex: u32, peusagepolicytype: *mut DRM_USAGEPOLICY_TYPE, pfexclusion: *mut super::super::Foundation::BOOL, punamelength: *mut u32, wszname: windows_core::PWSTR, puminversionlength: *mut u32, wszminversion: windows_core::PWSTR, pumaxversionlength: *mut u32, wszmaxversion: windows_core::PWSTR, pupublickeylength: *mut u32, wszpublickey: windows_core::PWSTR, pudigestalgorithmlength: *mut u32, wszdigestalgorithm: windows_core::PWSTR, pcbdigest: *mut u32, pbdigest: *mut u8) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUsagePolicy(hissuancelicense : u32, uindex : u32, peusagepolicytype : *mut DRM_USAGEPOLICY_TYPE, pfexclusion : *mut super::super::Foundation:: BOOL, punamelength : *mut u32, wszname : windows_core::PWSTR, puminversionlength : *mut u32, wszminversion : windows_core::PWSTR, pumaxversionlength : *mut u32, wszmaxversion : windows_core::PWSTR, pupublickeylength : *mut u32, wszpublickey : windows_core::PWSTR, pudigestalgorithmlength : *mut u32, wszdigestalgorithm : windows_core::PWSTR, pcbdigest : *mut u32, pbdigest : *mut u8) -> windows_core::HRESULT);
    DRMGetUsagePolicy(
        core::mem::transmute(hissuancelicense),
        core::mem::transmute(uindex),
        core::mem::transmute(peusagepolicytype),
        core::mem::transmute(pfexclusion),
        core::mem::transmute(punamelength),
        core::mem::transmute(wszname),
        core::mem::transmute(puminversionlength),
        core::mem::transmute(wszminversion),
        core::mem::transmute(pumaxversionlength),
        core::mem::transmute(wszmaxversion),
        core::mem::transmute(pupublickeylength),
        core::mem::transmute(wszpublickey),
        core::mem::transmute(pudigestalgorithmlength),
        core::mem::transmute(wszdigestalgorithm),
        core::mem::transmute(pcbdigest),
        core::mem::transmute(pbdigest),
    )
    .ok()
}
#[inline]
pub unsafe fn DRMGetUserInfo(huser: u32, puusernamelength: *mut u32, wszusername: windows_core::PWSTR, puuseridlength: *mut u32, wszuserid: windows_core::PWSTR, puuseridtypelength: *mut u32, wszuseridtype: windows_core::PWSTR) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUserInfo(huser : u32, puusernamelength : *mut u32, wszusername : windows_core::PWSTR, puuseridlength : *mut u32, wszuserid : windows_core::PWSTR, puuseridtypelength : *mut u32, wszuseridtype : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMGetUserInfo(core::mem::transmute(huser), core::mem::transmute(puusernamelength), core::mem::transmute(wszusername), core::mem::transmute(puuseridlength), core::mem::transmute(wszuserid), core::mem::transmute(puuseridtypelength), core::mem::transmute(wszuseridtype)).ok()
}
#[inline]
pub unsafe fn DRMGetUserRights(hissuancelicense: u32, huser: u32, uindex: u32, phright: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUserRights(hissuancelicense : u32, huser : u32, uindex : u32, phright : *mut u32) -> windows_core::HRESULT);
    DRMGetUserRights(core::mem::transmute(hissuancelicense), core::mem::transmute(huser), core::mem::transmute(uindex), core::mem::transmute(phright)).ok()
}
#[inline]
pub unsafe fn DRMGetUsers(hissuancelicense: u32, uindex: u32, phuser: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMGetUsers(hissuancelicense : u32, uindex : u32, phuser : *mut u32) -> windows_core::HRESULT);
    DRMGetUsers(core::mem::transmute(hissuancelicense), core::mem::transmute(uindex), core::mem::transmute(phuser)).ok()
}
#[inline]
pub unsafe fn DRMInitEnvironment<P2, P3, P4>(esecurityprovidertype: DRMSECURITYPROVIDERTYPE, especification: DRMSPECTYPE, wszsecurityprovider: P2, wszmanifestcredentials: P3, wszmachinecredentials: P4, phenv: *mut u32, phdefaultlibrary: *mut u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMInitEnvironment(esecurityprovidertype : DRMSECURITYPROVIDERTYPE, especification : DRMSPECTYPE, wszsecurityprovider : windows_core::PCWSTR, wszmanifestcredentials : windows_core::PCWSTR, wszmachinecredentials : windows_core::PCWSTR, phenv : *mut u32, phdefaultlibrary : *mut u32) -> windows_core::HRESULT);
    DRMInitEnvironment(core::mem::transmute(esecurityprovidertype), core::mem::transmute(especification), wszsecurityprovider.param().abi(), wszmanifestcredentials.param().abi(), wszmachinecredentials.param().abi(), core::mem::transmute(phenv), core::mem::transmute(phdefaultlibrary)).ok()
}
#[inline]
pub unsafe fn DRMIsActivated(hclient: u32, uflags: u32, pactservinfo: *mut DRM_ACTSERV_INFO) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMIsActivated(hclient : u32, uflags : u32, pactservinfo : *mut DRM_ACTSERV_INFO) -> windows_core::HRESULT);
    DRMIsActivated(core::mem::transmute(hclient), core::mem::transmute(uflags), core::mem::transmute(pactservinfo)).ok()
}
#[inline]
pub unsafe fn DRMIsWindowProtected<P0>(hwnd: P0, pfprotected: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMIsWindowProtected(hwnd : super::super::Foundation:: HWND, pfprotected : *mut super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    DRMIsWindowProtected(hwnd.param().abi(), core::mem::transmute(pfprotected)).ok()
}
#[inline]
pub unsafe fn DRMLoadLibrary<P2, P3>(henv: u32, especification: DRMSPECTYPE, wszlibraryprovider: P2, wszcredentials: P3, phlibrary: *mut u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMLoadLibrary(henv : u32, especification : DRMSPECTYPE, wszlibraryprovider : windows_core::PCWSTR, wszcredentials : windows_core::PCWSTR, phlibrary : *mut u32) -> windows_core::HRESULT);
    DRMLoadLibrary(core::mem::transmute(henv), core::mem::transmute(especification), wszlibraryprovider.param().abi(), wszcredentials.param().abi(), core::mem::transmute(phlibrary)).ok()
}
#[inline]
pub unsafe fn DRMParseUnboundLicense<P0>(wszcertificate: P0, phqueryroot: *mut u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMParseUnboundLicense(wszcertificate : windows_core::PCWSTR, phqueryroot : *mut u32) -> windows_core::HRESULT);
    DRMParseUnboundLicense(wszcertificate.param().abi(), core::mem::transmute(phqueryroot)).ok()
}
#[inline]
pub unsafe fn DRMRegisterContent<P0>(fregister: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMRegisterContent(fregister : super::super::Foundation:: BOOL) -> windows_core::HRESULT);
    DRMRegisterContent(fregister.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMRegisterProtectedWindow<P1>(henv: u32, hwnd: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::Foundation::HWND>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMRegisterProtectedWindow(henv : u32, hwnd : super::super::Foundation:: HWND) -> windows_core::HRESULT);
    DRMRegisterProtectedWindow(core::mem::transmute(henv), hwnd.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMRegisterRevocationList<P1>(henv: u32, wszrevocationlist: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMRegisterRevocationList(henv : u32, wszrevocationlist : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMRegisterRevocationList(core::mem::transmute(henv), wszrevocationlist.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMRepair() -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMRepair() -> windows_core::HRESULT);
    DRMRepair().ok()
}
#[inline]
pub unsafe fn DRMSetApplicationSpecificData<P1, P2, P3>(hissuancelicense: u32, fdelete: P1, wszname: P2, wszvalue: P3) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMSetApplicationSpecificData(hissuancelicense : u32, fdelete : super::super::Foundation:: BOOL, wszname : windows_core::PCWSTR, wszvalue : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMSetApplicationSpecificData(core::mem::transmute(hissuancelicense), fdelete.param().abi(), wszname.param().abi(), wszvalue.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMSetGlobalOptions(eglobaloptions: DRMGLOBALOPTIONS, pvdata: *mut core::ffi::c_void, dwlen: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMSetGlobalOptions(eglobaloptions : DRMGLOBALOPTIONS, pvdata : *mut core::ffi::c_void, dwlen : u32) -> windows_core::HRESULT);
    DRMSetGlobalOptions(core::mem::transmute(eglobaloptions), core::mem::transmute(pvdata), core::mem::transmute(dwlen)).ok()
}
#[inline]
pub unsafe fn DRMSetIntervalTime(hissuancelicense: u32, cdays: u32) -> windows_core::Result<()> {
    windows_targets::link!("msdrm.dll" "system" fn DRMSetIntervalTime(hissuancelicense : u32, cdays : u32) -> windows_core::HRESULT);
    DRMSetIntervalTime(core::mem::transmute(hissuancelicense), core::mem::transmute(cdays)).ok()
}
#[inline]
pub unsafe fn DRMSetMetaData<P1, P2, P3, P4, P5, P6>(hissuancelicense: u32, wszcontentid: P1, wszcontentidtype: P2, wszskuid: P3, wszskuidtype: P4, wszcontenttype: P5, wszcontentname: P6) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMSetMetaData(hissuancelicense : u32, wszcontentid : windows_core::PCWSTR, wszcontentidtype : windows_core::PCWSTR, wszskuid : windows_core::PCWSTR, wszskuidtype : windows_core::PCWSTR, wszcontenttype : windows_core::PCWSTR, wszcontentname : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMSetMetaData(core::mem::transmute(hissuancelicense), wszcontentid.param().abi(), wszcontentidtype.param().abi(), wszskuid.param().abi(), wszskuidtype.param().abi(), wszcontenttype.param().abi(), wszcontentname.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMSetNameAndDescription<P1, P3, P4>(hissuancelicense: u32, fdelete: P1, lcid: u32, wszname: P3, wszdescription: P4) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMSetNameAndDescription(hissuancelicense : u32, fdelete : super::super::Foundation:: BOOL, lcid : u32, wszname : windows_core::PCWSTR, wszdescription : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMSetNameAndDescription(core::mem::transmute(hissuancelicense), fdelete.param().abi(), core::mem::transmute(lcid), wszname.param().abi(), wszdescription.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMSetRevocationPoint<P1, P2, P3, P4, P6, P7>(hissuancelicense: u32, fdelete: P1, wszid: P2, wszidtype: P3, wszurl: P4, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, wszname: P6, wszpublickey: P7) -> windows_core::Result<()>
where
    P1: windows_core::Param<super::super::Foundation::BOOL>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMSetRevocationPoint(hissuancelicense : u32, fdelete : super::super::Foundation:: BOOL, wszid : windows_core::PCWSTR, wszidtype : windows_core::PCWSTR, wszurl : windows_core::PCWSTR, pstfrequency : *mut super::super::Foundation:: SYSTEMTIME, wszname : windows_core::PCWSTR, wszpublickey : windows_core::PCWSTR) -> windows_core::HRESULT);
    DRMSetRevocationPoint(core::mem::transmute(hissuancelicense), fdelete.param().abi(), wszid.param().abi(), wszidtype.param().abi(), wszurl.param().abi(), core::mem::transmute(pstfrequency), wszname.param().abi(), wszpublickey.param().abi()).ok()
}
#[inline]
pub unsafe fn DRMSetUsagePolicy<P2, P3, P4, P5, P6, P7, P8>(hissuancelicense: u32, eusagepolicytype: DRM_USAGEPOLICY_TYPE, fdelete: P2, fexclusion: P3, wszname: P4, wszminversion: P5, wszmaxversion: P6, wszpublickey: P7, wszdigestalgorithm: P8, pbdigest: *mut u8, cbdigest: u32) -> windows_core::Result<()>
where
    P2: windows_core::Param<super::super::Foundation::BOOL>,
    P3: windows_core::Param<super::super::Foundation::BOOL>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
    P7: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMSetUsagePolicy(hissuancelicense : u32, eusagepolicytype : DRM_USAGEPOLICY_TYPE, fdelete : super::super::Foundation:: BOOL, fexclusion : super::super::Foundation:: BOOL, wszname : windows_core::PCWSTR, wszminversion : windows_core::PCWSTR, wszmaxversion : windows_core::PCWSTR, wszpublickey : windows_core::PCWSTR, wszdigestalgorithm : windows_core::PCWSTR, pbdigest : *mut u8, cbdigest : u32) -> windows_core::HRESULT);
    DRMSetUsagePolicy(core::mem::transmute(hissuancelicense), core::mem::transmute(eusagepolicytype), fdelete.param().abi(), fexclusion.param().abi(), wszname.param().abi(), wszminversion.param().abi(), wszmaxversion.param().abi(), wszpublickey.param().abi(), wszdigestalgorithm.param().abi(), core::mem::transmute(pbdigest), core::mem::transmute(cbdigest)).ok()
}
#[inline]
pub unsafe fn DRMVerify<P0>(wszdata: P0, pcattesteddata: *mut u32, wszattesteddata: windows_core::PWSTR, petype: *mut DRMATTESTTYPE, pcprincipal: *mut u32, wszprincipal: windows_core::PWSTR, pcmanifest: *mut u32, wszmanifest: windows_core::PWSTR) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("msdrm.dll" "system" fn DRMVerify(wszdata : windows_core::PCWSTR, pcattesteddata : *mut u32, wszattesteddata : windows_core::PWSTR, petype : *mut DRMATTESTTYPE, pcprincipal : *mut u32, wszprincipal : windows_core::PWSTR, pcmanifest : *mut u32, wszmanifest : windows_core::PWSTR) -> windows_core::HRESULT);
    DRMVerify(wszdata.param().abi(), core::mem::transmute(pcattesteddata), core::mem::transmute(wszattesteddata), core::mem::transmute(petype), core::mem::transmute(pcprincipal), core::mem::transmute(wszprincipal), core::mem::transmute(pcmanifest), core::mem::transmute(wszmanifest)).ok()
}
pub type DRMCALLBACK = Option<unsafe extern "system" fn(param0: DRM_STATUS_MSG, param1: windows_core::HRESULT, param2: *mut core::ffi::c_void, param3: *mut core::ffi::c_void) -> windows_core::HRESULT>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRMATTESTTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRMENCODINGTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRMGLOBALOPTIONS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRMSECURITYPROVIDERTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRMSPECTYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRMTIMETYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRM_DISTRIBUTION_POINT_INFO(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRM_STATUS_MSG(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DRM_USAGEPOLICY_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRMBOUNDLICENSEPARAMS {
    pub uVersion: u32,
    pub hEnablingPrincipal: u32,
    pub hSecureStore: u32,
    pub wszRightsRequested: windows_core::PWSTR,
    pub wszRightsGroup: windows_core::PWSTR,
    pub idResource: DRMID,
    pub cAuthenticatorCount: u32,
    pub rghAuthenticators: *mut u32,
    pub wszDefaultEnablingPrincipalCredentials: windows_core::PWSTR,
    pub dwFlags: u32,
}
impl Default for DRMBOUNDLICENSEPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DRMBOUNDLICENSEPARAMS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRMID {
    pub uVersion: u32,
    pub wszIDType: windows_core::PWSTR,
    pub wszID: windows_core::PWSTR,
}
impl Default for DRMID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DRMID {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRM_ACTSERV_INFO {
    pub uVersion: u32,
    pub wszPubKey: windows_core::PWSTR,
    pub wszURL: windows_core::PWSTR,
}
impl Default for DRM_ACTSERV_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DRM_ACTSERV_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRM_CLIENT_VERSION_INFO {
    pub uStructVersion: u32,
    pub dwVersion: [u32; 4],
    pub wszHierarchy: [u16; 256],
    pub wszProductId: [u16; 256],
    pub wszProductDescription: [u16; 256],
}
impl Default for DRM_CLIENT_VERSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DRM_CLIENT_VERSION_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRM_LICENSE_ACQ_DATA {
    pub uVersion: u32,
    pub wszURL: windows_core::PWSTR,
    pub wszLocalFilename: windows_core::PWSTR,
    pub pbPostData: *mut u8,
    pub dwPostDataSize: u32,
    pub wszFriendlyName: windows_core::PWSTR,
}
impl Default for DRM_LICENSE_ACQ_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for DRM_LICENSE_ACQ_DATA {
    type TypeKind = windows_core::CopyType;
}
pub const DRMACTSERVINFOVERSION: u32 = 0u32;
pub const DRMATTESTTYPE_FULLENVIRONMENT: DRMATTESTTYPE = DRMATTESTTYPE(0i32);
pub const DRMATTESTTYPE_HASHONLY: DRMATTESTTYPE = DRMATTESTTYPE(1i32);
pub const DRMBINDINGFLAGS_IGNORE_VALIDITY_INTERVALS: u32 = 1u32;
pub const DRMBOUNDLICENSEPARAMSVERSION: u32 = 1u32;
pub const DRMCALLBACKVERSION: u32 = 1u32;
pub const DRMCLIENTSTRUCTVERSION: u32 = 1u32;
pub const DRMENCODINGTYPE_BASE64: DRMENCODINGTYPE = DRMENCODINGTYPE(0i32);
pub const DRMENCODINGTYPE_LONG: DRMENCODINGTYPE = DRMENCODINGTYPE(2i32);
pub const DRMENCODINGTYPE_RAW: DRMENCODINGTYPE = DRMENCODINGTYPE(5i32);
pub const DRMENCODINGTYPE_STRING: DRMENCODINGTYPE = DRMENCODINGTYPE(1i32);
pub const DRMENCODINGTYPE_TIME: DRMENCODINGTYPE = DRMENCODINGTYPE(3i32);
pub const DRMENCODINGTYPE_UINT: DRMENCODINGTYPE = DRMENCODINGTYPE(4i32);
pub const DRMENVHANDLE_INVALID: u32 = 0u32;
pub const DRMGLOBALOPTIONS_USE_SERVERSECURITYPROCESSOR: DRMGLOBALOPTIONS = DRMGLOBALOPTIONS(1i32);
pub const DRMGLOBALOPTIONS_USE_WINHTTP: DRMGLOBALOPTIONS = DRMGLOBALOPTIONS(0i32);
pub const DRMHANDLE_INVALID: u32 = 0u32;
pub const DRMHSESSION_INVALID: u32 = 0u32;
pub const DRMIDVERSION: u32 = 0u32;
pub const DRMLICENSEACQDATAVERSION: u32 = 0u32;
pub const DRMPUBHANDLE_INVALID: u32 = 0u32;
pub const DRMQUERYHANDLE_INVALID: u32 = 0u32;
pub const DRMSECURITYPROVIDERTYPE_SOFTWARESECREP: DRMSECURITYPROVIDERTYPE = DRMSECURITYPROVIDERTYPE(0i32);
pub const DRMSPECTYPE_FILENAME: DRMSPECTYPE = DRMSPECTYPE(1i32);
pub const DRMSPECTYPE_UNKNOWN: DRMSPECTYPE = DRMSPECTYPE(0i32);
pub const DRMTIMETYPE_SYSTEMLOCAL: DRMTIMETYPE = DRMTIMETYPE(1i32);
pub const DRMTIMETYPE_SYSTEMUTC: DRMTIMETYPE = DRMTIMETYPE(0i32);
pub const DRM_ACTIVATE_CANCEL: u32 = 8u32;
pub const DRM_ACTIVATE_DELAYED: u32 = 64u32;
pub const DRM_ACTIVATE_GROUPIDENTITY: u32 = 2u32;
pub const DRM_ACTIVATE_MACHINE: u32 = 1u32;
pub const DRM_ACTIVATE_SHARED_GROUPIDENTITY: u32 = 32u32;
pub const DRM_ACTIVATE_SILENT: u32 = 16u32;
pub const DRM_ACTIVATE_TEMPORARY: u32 = 4u32;
pub const DRM_ADD_LICENSE_NOPERSIST: u32 = 0u32;
pub const DRM_ADD_LICENSE_PERSIST: u32 = 1u32;
pub const DRM_AILT_CANCEL: u32 = 4u32;
pub const DRM_AILT_NONSILENT: u32 = 1u32;
pub const DRM_AILT_OBTAIN_ALL: u32 = 2u32;
pub const DRM_AL_CANCEL: u32 = 4u32;
pub const DRM_AL_FETCHNOADVISORY: u32 = 8u32;
pub const DRM_AL_NONSILENT: u32 = 1u32;
pub const DRM_AL_NOPERSIST: u32 = 2u32;
pub const DRM_AL_NOUI: u32 = 16u32;
pub const DRM_AUTO_GENERATE_KEY: u32 = 16u32;
pub const DRM_DEFAULTGROUPIDTYPE_PASSPORT: windows_core::PCWSTR = windows_core::w!("PassportAuthProvider");
pub const DRM_DEFAULTGROUPIDTYPE_WINDOWSAUTH: windows_core::PCWSTR = windows_core::w!("WindowsAuthProvider");
pub const DRM_DISTRIBUTION_POINT_LICENSE_ACQUISITION: DRM_DISTRIBUTION_POINT_INFO = DRM_DISTRIBUTION_POINT_INFO(0i32);
pub const DRM_DISTRIBUTION_POINT_PUBLISHING: DRM_DISTRIBUTION_POINT_INFO = DRM_DISTRIBUTION_POINT_INFO(1i32);
pub const DRM_DISTRIBUTION_POINT_REFERRAL_INFO: DRM_DISTRIBUTION_POINT_INFO = DRM_DISTRIBUTION_POINT_INFO(2i32);
pub const DRM_EL_CLIENTLICENSOR: u32 = 128u32;
pub const DRM_EL_CLIENTLICENSOR_LID: u32 = 256u32;
pub const DRM_EL_EUL: u32 = 32u32;
pub const DRM_EL_EUL_LID: u32 = 64u32;
pub const DRM_EL_EXPIRED: u32 = 4096u32;
pub const DRM_EL_GROUPIDENTITY: u32 = 2u32;
pub const DRM_EL_GROUPIDENTITY_LID: u32 = 8u32;
pub const DRM_EL_GROUPIDENTITY_NAME: u32 = 4u32;
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE: u32 = 16384u32;
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE_LID: u32 = 32768u32;
pub const DRM_EL_ISSUERNAME: u32 = 8192u32;
pub const DRM_EL_MACHINE: u32 = 1u32;
pub const DRM_EL_REVOCATIONLIST: u32 = 1024u32;
pub const DRM_EL_REVOCATIONLIST_LID: u32 = 2048u32;
pub const DRM_EL_SPECIFIED_CLIENTLICENSOR: u32 = 512u32;
pub const DRM_EL_SPECIFIED_GROUPIDENTITY: u32 = 16u32;
pub const DRM_LOCKBOXTYPE_BLACKBOX: u32 = 2u32;
pub const DRM_LOCKBOXTYPE_DEFAULT: u32 = 2u32;
pub const DRM_LOCKBOXTYPE_NONE: u32 = 0u32;
pub const DRM_LOCKBOXTYPE_WHITEBOX: u32 = 1u32;
pub const DRM_MSG_ACQUIRE_ADVISORY: DRM_STATUS_MSG = DRM_STATUS_MSG(3i32);
pub const DRM_MSG_ACQUIRE_CLIENTLICENSOR: DRM_STATUS_MSG = DRM_STATUS_MSG(5i32);
pub const DRM_MSG_ACQUIRE_ISSUANCE_LICENSE_TEMPLATE: DRM_STATUS_MSG = DRM_STATUS_MSG(6i32);
pub const DRM_MSG_ACQUIRE_LICENSE: DRM_STATUS_MSG = DRM_STATUS_MSG(2i32);
pub const DRM_MSG_ACTIVATE_GROUPIDENTITY: DRM_STATUS_MSG = DRM_STATUS_MSG(1i32);
pub const DRM_MSG_ACTIVATE_MACHINE: DRM_STATUS_MSG = DRM_STATUS_MSG(0i32);
pub const DRM_MSG_SIGN_ISSUANCE_LICENSE: DRM_STATUS_MSG = DRM_STATUS_MSG(4i32);
pub const DRM_OWNER_LICENSE_NOPERSIST: u32 = 32u32;
pub const DRM_REUSE_KEY: u32 = 64u32;
pub const DRM_SERVER_ISSUANCELICENSE: u32 = 8u32;
pub const DRM_SERVICE_LOCATION_ENTERPRISE: u32 = 2u32;
pub const DRM_SERVICE_LOCATION_INTERNET: u32 = 1u32;
pub const DRM_SERVICE_TYPE_ACTIVATION: u32 = 1u32;
pub const DRM_SERVICE_TYPE_CERTIFICATION: u32 = 2u32;
pub const DRM_SERVICE_TYPE_CLIENTLICENSOR: u32 = 8u32;
pub const DRM_SERVICE_TYPE_PUBLISHING: u32 = 4u32;
pub const DRM_SERVICE_TYPE_SILENT: u32 = 16u32;
pub const DRM_SIGN_CANCEL: u32 = 4u32;
pub const DRM_SIGN_OFFLINE: u32 = 2u32;
pub const DRM_SIGN_ONLINE: u32 = 1u32;
pub const DRM_USAGEPOLICY_TYPE_BYDIGEST: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(2i32);
pub const DRM_USAGEPOLICY_TYPE_BYNAME: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(0i32);
pub const DRM_USAGEPOLICY_TYPE_BYPUBLICKEY: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(1i32);
pub const DRM_USAGEPOLICY_TYPE_OSEXCLUSION: DRM_USAGEPOLICY_TYPE = DRM_USAGEPOLICY_TYPE(3i32);
pub const MSDRM_CLIENT_ZONE: u32 = 52992u32;
pub const MSDRM_POLICY_ZONE: u32 = 37632u32;
