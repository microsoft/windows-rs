#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAcquireAdvisories(hlicensestorage : super::DRMHSESSION, wszlicense : windows_sys::core::PCWSTR, wszurl : windows_sys::core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAcquireIssuanceLicenseTemplate(hclient : super::DRMHSESSION, uflags : u32, pvreserved : *mut core::ffi::c_void, ctemplates : u32, pwsztemplateids : *const windows_sys::core::PCWSTR, wszurl : windows_sys::core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAcquireLicense(hsession : super::DRMHSESSION, uflags : u32, wszgroupidentitycredential : windows_sys::core::PCWSTR, wszrequestedrights : windows_sys::core::PCWSTR, wszcustomdata : windows_sys::core::PCWSTR, wszurl : windows_sys::core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "windef"))]
windows_link::link!("msdrm.dll" "system" fn DRMActivate(hclient : super::DRMHSESSION, uflags : u32, ulangid : u32, pactservinfo : *mut super::DRM_ACTSERV_INFO, pvcontext : *mut core::ffi::c_void, hparentwnd : super::HWND) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAddLicense(hlicensestorage : super::DRMHSESSION, uflags : u32, wszlicense : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAddRightWithUser(hissuancelicense : super::DRMPUBHANDLE, hright : super::DRMPUBHANDLE, huser : super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAttest(henablingprincipal : super::DRMHANDLE, wszdata : windows_sys::core::PCWSTR, etype : super::DRMATTESTTYPE, pcattestedblob : *mut u32, wszattestedblob : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCheckSecurity(henv : super::DRMENVHANDLE, clevel : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMClearAllRights(hissuancelicense : super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCloseEnvironmentHandle(henv : super::DRMENVHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCloseHandle(handle : super::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMClosePubHandle(hpub : super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCloseQueryHandle(hquery : super::DRMQUERYHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCloseSession(hsession : super::DRMHSESSION) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMConstructCertificateChain(ccertificates : u32, rgwszcertificates : *const windows_sys::core::PCWSTR, pcchain : *mut u32, wszchain : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateBoundLicense(henv : super::DRMENVHANDLE, pparams : *mut super::DRMBOUNDLICENSEPARAMS, wszlicensechain : windows_sys::core::PCWSTR, phboundlicense : *mut super::DRMHANDLE, pherrorlog : *mut super::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateClientSession(pfncallback : super::DRMCALLBACK, ucallbackversion : u32, wszgroupidprovidertype : windows_sys::core::PCWSTR, wszgroupid : windows_sys::core::PCWSTR, phclient : *mut super::DRMHSESSION) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsDecryptor(hboundlicense : super::DRMHANDLE, wszright : windows_sys::core::PCWSTR, hauxlib : super::DRMHANDLE, wszauxplug : windows_sys::core::PCWSTR, phdecryptor : *mut super::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsEncryptor(hboundlicense : super::DRMHANDLE, wszright : windows_sys::core::PCWSTR, hauxlib : super::DRMHANDLE, wszauxplug : windows_sys::core::PCWSTR, phencryptor : *mut super::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateEnablingPrincipal(henv : super::DRMENVHANDLE, hlibrary : super::DRMHANDLE, wszobject : windows_sys::core::PCWSTR, pidprincipal : *mut super::DRMID, wszcredentials : windows_sys::core::PCWSTR, phenablingprincipal : *mut super::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMCreateIssuanceLicense(psttimefrom : *mut super::SYSTEMTIME, psttimeuntil : *mut super::SYSTEMTIME, wszreferralinfoname : windows_sys::core::PCWSTR, wszreferralinfourl : windows_sys::core::PCWSTR, howner : super::DRMPUBHANDLE, wszissuancelicense : windows_sys::core::PCWSTR, hboundlicense : super::DRMHANDLE, phissuancelicense : *mut super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateLicenseStorageSession(henv : super::DRMENVHANDLE, hdefaultlibrary : super::DRMHANDLE, hclient : super::DRMHSESSION, uflags : u32, wszissuancelicense : windows_sys::core::PCWSTR, phlicensestorage : *mut super::DRMHSESSION) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMCreateRight(wszrightname : windows_sys::core::PCWSTR, pstfrom : *mut super::SYSTEMTIME, pstuntil : *mut super::SYSTEMTIME, cextendedinfo : u32, pwszextendedinfoname : *const windows_sys::core::PCWSTR, pwszextendedinfovalue : *const windows_sys::core::PCWSTR, phright : *mut super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateUser(wszusername : windows_sys::core::PCWSTR, wszuserid : windows_sys::core::PCWSTR, wszuseridtype : windows_sys::core::PCWSTR, phuser : *mut super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMDecode(wszalgid : windows_sys::core::PCWSTR, wszencodedstring : windows_sys::core::PCWSTR, pudecodeddatalen : *mut u32, pbdecodeddata : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMDeconstructCertificateChain(wszchain : windows_sys::core::PCWSTR, iwhich : u32, pccert : *mut u32, wszcert : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMDecrypt(hcryptoprovider : super::DRMHANDLE, iposition : u32, cnuminbytes : u32, pbindata : *mut u8, pcnumoutbytes : *mut u32, pboutdata : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMDeleteLicense(hsession : super::DRMHSESSION, wszlicenseid : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMDuplicateEnvironmentHandle(htocopy : super::DRMENVHANDLE, phcopy : *mut super::DRMENVHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMDuplicateHandle(htocopy : super::DRMHANDLE, phcopy : *mut super::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMDuplicatePubHandle(hpubin : super::DRMPUBHANDLE, phpubout : *mut super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMDuplicateSession(hsessionin : super::DRMHSESSION, phsessionout : *mut super::DRMHSESSION) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMEncode(wszalgid : windows_sys::core::PCWSTR, udatalen : u32, pbdecodeddata : *mut u8, puencodedstringlen : *mut u32, wszencodedstring : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMEncrypt(hcryptoprovider : super::DRMHANDLE, iposition : u32, cnuminbytes : u32, pbindata : *mut u8, pcnumoutbytes : *mut u32, pboutdata : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMEnumerateLicense(hsession : super::DRMHSESSION, uflags : u32, uindex : u32, pfsharedflag : *mut windows_sys::core::BOOL, pucertificatedatalen : *mut u32, wszcertificatedata : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetApplicationSpecificData(hissuancelicense : super::DRMPUBHANDLE, uindex : u32, punamelength : *mut u32, wszname : windows_sys::core::PWSTR, puvaluelength : *mut u32, wszvalue : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttribute(hqueryroot : super::DRMHANDLE, wszattribute : windows_sys::core::PCWSTR, iwhich : u32, peencoding : *mut super::DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttributeCount(hqueryroot : super::DRMHANDLE, wszattribute : windows_sys::core::PCWSTR, pcattributes : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObject(hqueryroot : super::DRMHANDLE, wszsubobjecttype : windows_sys::core::PCWSTR, iwhich : u32, phsubobject : *mut super::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObjectCount(hqueryroot : super::DRMHANDLE, wszsubobjecttype : windows_sys::core::PCWSTR, pcsubobjects : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMGetCertificateChainCount(wszchain : windows_sys::core::PCWSTR, pccertcount : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetClientVersion(pdrmclientversioninfo : *mut super::DRM_CLIENT_VERSION_INFO) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetEnvironmentInfo(handle : super::DRMENVHANDLE, wszattribute : windows_sys::core::PCWSTR, peencoding : *mut super::DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetInfo(handle : super::DRMHANDLE, wszattribute : windows_sys::core::PCWSTR, peencoding : *const super::DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetIntervalTime(hissuancelicense : super::DRMPUBHANDLE, pcdays : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseInfo(hissuancelicense : super::DRMPUBHANDLE, psttimefrom : *mut super::SYSTEMTIME, psttimeuntil : *mut super::SYSTEMTIME, uflags : u32, pudistributionpointnamelength : *mut u32, wszdistributionpointname : windows_sys::core::PWSTR, pudistributionpointurllength : *mut u32, wszdistributionpointurl : windows_sys::core::PWSTR, phowner : *mut super::DRMPUBHANDLE, pfofficial : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseTemplate(hissuancelicense : super::DRMPUBHANDLE, puissuancelicensetemplatelength : *mut u32, wszissuancelicensetemplate : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetMetaData(hissuancelicense : super::DRMPUBHANDLE, pucontentidlength : *mut u32, wszcontentid : windows_sys::core::PWSTR, pucontentidtypelength : *mut u32, wszcontentidtype : windows_sys::core::PWSTR, puskuidlength : *mut u32, wszskuid : windows_sys::core::PWSTR, puskuidtypelength : *mut u32, wszskuidtype : windows_sys::core::PWSTR, pucontenttypelength : *mut u32, wszcontenttype : windows_sys::core::PWSTR, pucontentnamelength : *mut u32, wszcontentname : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetNameAndDescription(hissuancelicense : super::DRMPUBHANDLE, uindex : u32, pulcid : *mut u32, punamelength : *mut u32, wszname : windows_sys::core::PWSTR, pudescriptionlength : *mut u32, wszdescription : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetOwnerLicense(hissuancelicense : super::DRMPUBHANDLE, puownerlicenselength : *mut u32, wszownerlicense : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetProcAddress(hlibrary : super::DRMHANDLE, wszprocname : windows_sys::core::PCWSTR, ppfnprocaddress : *mut super::FARPROC) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetRevocationPoint(hissuancelicense : super::DRMPUBHANDLE, puidlength : *mut u32, wszid : windows_sys::core::PWSTR, puidtypelength : *mut u32, wszidtype : windows_sys::core::PWSTR, puurllength : *mut u32, wszrl : windows_sys::core::PWSTR, pstfrequency : *mut super::SYSTEMTIME, punamelength : *mut u32, wszname : windows_sys::core::PWSTR, pupublickeylength : *mut u32, wszpublickey : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetRightExtendedInfo(hright : super::DRMPUBHANDLE, uindex : u32, puextendedinfonamelength : *mut u32, wszextendedinfoname : windows_sys::core::PWSTR, puextendedinfovaluelength : *mut u32, wszextendedinfovalue : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetRightInfo(hright : super::DRMPUBHANDLE, purightnamelength : *mut u32, wszrightname : windows_sys::core::PWSTR, pstfrom : *mut super::SYSTEMTIME, pstuntil : *mut super::SYSTEMTIME) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMGetSecurityProvider(uflags : u32, putypelen : *mut u32, wsztype : windows_sys::core::PWSTR, pupathlen : *mut u32, wszpath : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetServiceLocation(hclient : super::DRMHSESSION, uservicetype : u32, uservicelocation : u32, wszissuancelicense : windows_sys::core::PCWSTR, puserviceurllength : *mut u32, wszserviceurl : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicense(henv : super::DRMENVHANDLE, hissuancelicense : super::DRMPUBHANDLE, uflags : u32, pbsymkey : *mut u8, cbsymkey : u32, wszsymkeytype : windows_sys::core::PCWSTR, wszclientlicensorcertificate : windows_sys::core::PCWSTR, pfncallback : super::DRMCALLBACK, wszurl : windows_sys::core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicenseEx(henv : super::DRMENVHANDLE, hissuancelicense : super::DRMPUBHANDLE, uflags : u32, pbsymkey : *const u8, cbsymkey : u32, wszsymkeytype : windows_sys::core::PCWSTR, pvreserved : *const core::ffi::c_void, henablingprincipal : super::DRMHANDLE, hboundlicenseclc : super::DRMHANDLE, pfncallback : super::DRMCALLBACK, pvcontext : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetTime(henv : super::DRMENVHANDLE, etimeridtype : super::DRMTIMETYPE, potimeobject : *mut super::SYSTEMTIME) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttribute(hqueryroot : super::DRMQUERYHANDLE, wszattributetype : windows_sys::core::PCWSTR, iwhich : u32, peencoding : *mut super::DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttributeCount(hqueryroot : super::DRMQUERYHANDLE, wszattributetype : windows_sys::core::PCWSTR, pcattributes : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObject(hqueryroot : super::DRMQUERYHANDLE, wszsubobjecttype : windows_sys::core::PCWSTR, iindex : u32, phsubquery : *mut super::DRMQUERYHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObjectCount(hqueryroot : super::DRMQUERYHANDLE, wszsubobjecttype : windows_sys::core::PCWSTR, pcsubobjects : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUsagePolicy(hissuancelicense : super::DRMPUBHANDLE, uindex : u32, peusagepolicytype : *mut super::DRM_USAGEPOLICY_TYPE, pfexclusion : *mut windows_sys::core::BOOL, punamelength : *mut u32, wszname : windows_sys::core::PWSTR, puminversionlength : *mut u32, wszminversion : windows_sys::core::PWSTR, pumaxversionlength : *mut u32, wszmaxversion : windows_sys::core::PWSTR, pupublickeylength : *mut u32, wszpublickey : windows_sys::core::PWSTR, pudigestalgorithmlength : *mut u32, wszdigestalgorithm : windows_sys::core::PWSTR, pcbdigest : *mut u32, pbdigest : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUserInfo(huser : super::DRMPUBHANDLE, puusernamelength : *mut u32, wszusername : windows_sys::core::PWSTR, puuseridlength : *mut u32, wszuserid : windows_sys::core::PWSTR, puuseridtypelength : *mut u32, wszuseridtype : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUserRights(hissuancelicense : super::DRMPUBHANDLE, huser : super::DRMPUBHANDLE, uindex : u32, phright : *mut super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUsers(hissuancelicense : super::DRMPUBHANDLE, uindex : u32, phuser : *mut super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMInitEnvironment(esecurityprovidertype : super::DRMSECURITYPROVIDERTYPE, especification : super::DRMSPECTYPE, wszsecurityprovider : windows_sys::core::PCWSTR, wszmanifestcredentials : windows_sys::core::PCWSTR, wszmachinecredentials : windows_sys::core::PCWSTR, phenv : *mut super::DRMENVHANDLE, phdefaultlibrary : *mut super::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMIsActivated(hclient : super::DRMHSESSION, uflags : u32, pactservinfo : *mut super::DRM_ACTSERV_INFO) -> windows_sys::core::HRESULT);
#[cfg(feature = "windef")]
windows_link::link!("msdrm.dll" "system" fn DRMIsWindowProtected(hwnd : super::HWND, pfprotected : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMLoadLibrary(henv : super::DRMENVHANDLE, especification : super::DRMSPECTYPE, wszlibraryprovider : windows_sys::core::PCWSTR, wszcredentials : windows_sys::core::PCWSTR, phlibrary : *mut super::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMParseUnboundLicense(wszcertificate : windows_sys::core::PCWSTR, phqueryroot : *mut super::DRMQUERYHANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMRegisterContent(fregister : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "windef"))]
windows_link::link!("msdrm.dll" "system" fn DRMRegisterProtectedWindow(henv : super::DRMENVHANDLE, hwnd : super::HWND) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMRegisterRevocationList(henv : super::DRMENVHANDLE, wszrevocationlist : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMRepair() -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMSetApplicationSpecificData(hissuancelicense : super::DRMPUBHANDLE, fdelete : windows_sys::core::BOOL, wszname : windows_sys::core::PCWSTR, wszvalue : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMSetGlobalOptions(eglobaloptions : super::DRMGLOBALOPTIONS, pvdata : *mut core::ffi::c_void, dwlen : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMSetIntervalTime(hissuancelicense : super::DRMPUBHANDLE, cdays : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMSetMetaData(hissuancelicense : super::DRMPUBHANDLE, wszcontentid : windows_sys::core::PCWSTR, wszcontentidtype : windows_sys::core::PCWSTR, wszskuid : windows_sys::core::PCWSTR, wszskuidtype : windows_sys::core::PCWSTR, wszcontenttype : windows_sys::core::PCWSTR, wszcontentname : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMSetNameAndDescription(hissuancelicense : super::DRMPUBHANDLE, fdelete : windows_sys::core::BOOL, lcid : u32, wszname : windows_sys::core::PCWSTR, wszdescription : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMSetRevocationPoint(hissuancelicense : super::DRMPUBHANDLE, fdelete : windows_sys::core::BOOL, wszid : windows_sys::core::PCWSTR, wszidtype : windows_sys::core::PCWSTR, wszurl : windows_sys::core::PCWSTR, pstfrequency : *mut super::SYSTEMTIME, wszname : windows_sys::core::PCWSTR, wszpublickey : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMSetUsagePolicy(hissuancelicense : super::DRMPUBHANDLE, eusagepolicytype : super::DRM_USAGEPOLICY_TYPE, fdelete : windows_sys::core::BOOL, fexclusion : windows_sys::core::BOOL, wszname : windows_sys::core::PCWSTR, wszminversion : windows_sys::core::PCWSTR, wszmaxversion : windows_sys::core::PCWSTR, wszpublickey : windows_sys::core::PCWSTR, wszdigestalgorithm : windows_sys::core::PCWSTR, pbdigest : *mut u8, cbdigest : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMVerify(wszdata : windows_sys::core::PCWSTR, pcattesteddata : *mut u32, wszattesteddata : windows_sys::core::PWSTR, petype : *mut super::DRMATTESTTYPE, pcprincipal : *mut u32, wszprincipal : windows_sys::core::PWSTR, pcmanifest : *mut u32, wszmanifest : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
