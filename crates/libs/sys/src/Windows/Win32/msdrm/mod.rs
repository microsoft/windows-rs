#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAcquireAdvisories(hlicensestorage : super::msdrmdefs::DRMHSESSION, wszlicense : windows_sys::core::PCWSTR, wszurl : windows_sys::core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAcquireIssuanceLicenseTemplate(hclient : super::msdrmdefs::DRMHSESSION, uflags : u32, pvreserved : *mut core::ffi::c_void, ctemplates : u32, pwsztemplateids : *const windows_sys::core::PCWSTR, wszurl : windows_sys::core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAcquireLicense(hsession : super::msdrmdefs::DRMHSESSION, uflags : u32, wszgroupidentitycredential : windows_sys::core::PCWSTR, wszrequestedrights : windows_sys::core::PCWSTR, wszcustomdata : windows_sys::core::PCWSTR, wszurl : windows_sys::core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_msdrmdefs", feature = "Win32_windef"))]
windows_link::link!("msdrm.dll" "system" fn DRMActivate(hclient : super::msdrmdefs::DRMHSESSION, uflags : u32, ulangid : u32, pactservinfo : *mut super::msdrmdefs::DRM_ACTSERV_INFO, pvcontext : *mut core::ffi::c_void, hparentwnd : super::windef::HWND) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAddLicense(hlicensestorage : super::msdrmdefs::DRMHSESSION, uflags : u32, wszlicense : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAddRightWithUser(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, hright : super::msdrmdefs::DRMPUBHANDLE, huser : super::msdrmdefs::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAttest(henablingprincipal : super::msdrmdefs::DRMHANDLE, wszdata : windows_sys::core::PCWSTR, etype : super::msdrmdefs::DRMATTESTTYPE, pcattestedblob : *mut u32, wszattestedblob : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCheckSecurity(henv : super::msdrmdefs::DRMENVHANDLE, clevel : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMClearAllRights(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCloseEnvironmentHandle(henv : super::msdrmdefs::DRMENVHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCloseHandle(handle : super::msdrmdefs::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMClosePubHandle(hpub : super::msdrmdefs::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCloseQueryHandle(hquery : super::msdrmdefs::DRMQUERYHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCloseSession(hsession : super::msdrmdefs::DRMHSESSION) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMConstructCertificateChain(ccertificates : u32, rgwszcertificates : *const windows_sys::core::PCWSTR, pcchain : *mut u32, wszchain : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateBoundLicense(henv : super::msdrmdefs::DRMENVHANDLE, pparams : *mut super::msdrmdefs::DRMBOUNDLICENSEPARAMS, wszlicensechain : windows_sys::core::PCWSTR, phboundlicense : *mut super::msdrmdefs::DRMHANDLE, pherrorlog : *mut super::msdrmdefs::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateClientSession(pfncallback : super::msdrmdefs::DRMCALLBACK, ucallbackversion : u32, wszgroupidprovidertype : windows_sys::core::PCWSTR, wszgroupid : windows_sys::core::PCWSTR, phclient : *mut super::msdrmdefs::DRMHSESSION) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsDecryptor(hboundlicense : super::msdrmdefs::DRMHANDLE, wszright : windows_sys::core::PCWSTR, hauxlib : super::msdrmdefs::DRMHANDLE, wszauxplug : windows_sys::core::PCWSTR, phdecryptor : *mut super::msdrmdefs::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsEncryptor(hboundlicense : super::msdrmdefs::DRMHANDLE, wszright : windows_sys::core::PCWSTR, hauxlib : super::msdrmdefs::DRMHANDLE, wszauxplug : windows_sys::core::PCWSTR, phencryptor : *mut super::msdrmdefs::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateEnablingPrincipal(henv : super::msdrmdefs::DRMENVHANDLE, hlibrary : super::msdrmdefs::DRMHANDLE, wszobject : windows_sys::core::PCWSTR, pidprincipal : *mut super::msdrmdefs::DRMID, wszcredentials : windows_sys::core::PCWSTR, phenablingprincipal : *mut super::msdrmdefs::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMCreateIssuanceLicense(psttimefrom : *mut super::minwinbase::SYSTEMTIME, psttimeuntil : *mut super::minwinbase::SYSTEMTIME, wszreferralinfoname : windows_sys::core::PCWSTR, wszreferralinfourl : windows_sys::core::PCWSTR, howner : super::msdrmdefs::DRMPUBHANDLE, wszissuancelicense : windows_sys::core::PCWSTR, hboundlicense : super::msdrmdefs::DRMHANDLE, phissuancelicense : *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateLicenseStorageSession(henv : super::msdrmdefs::DRMENVHANDLE, hdefaultlibrary : super::msdrmdefs::DRMHANDLE, hclient : super::msdrmdefs::DRMHSESSION, uflags : u32, wszissuancelicense : windows_sys::core::PCWSTR, phlicensestorage : *mut super::msdrmdefs::DRMHSESSION) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMCreateRight(wszrightname : windows_sys::core::PCWSTR, pstfrom : *mut super::minwinbase::SYSTEMTIME, pstuntil : *mut super::minwinbase::SYSTEMTIME, cextendedinfo : u32, pwszextendedinfoname : *const windows_sys::core::PCWSTR, pwszextendedinfovalue : *const windows_sys::core::PCWSTR, phright : *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateUser(wszusername : windows_sys::core::PCWSTR, wszuserid : windows_sys::core::PCWSTR, wszuseridtype : windows_sys::core::PCWSTR, phuser : *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMDecode(wszalgid : windows_sys::core::PCWSTR, wszencodedstring : windows_sys::core::PCWSTR, pudecodeddatalen : *mut u32, pbdecodeddata : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMDeconstructCertificateChain(wszchain : windows_sys::core::PCWSTR, iwhich : u32, pccert : *mut u32, wszcert : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMDecrypt(hcryptoprovider : super::msdrmdefs::DRMHANDLE, iposition : u32, cnuminbytes : u32, pbindata : *mut u8, pcnumoutbytes : *mut u32, pboutdata : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMDeleteLicense(hsession : super::msdrmdefs::DRMHSESSION, wszlicenseid : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMDuplicateEnvironmentHandle(htocopy : super::msdrmdefs::DRMENVHANDLE, phcopy : *mut super::msdrmdefs::DRMENVHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMDuplicateHandle(htocopy : super::msdrmdefs::DRMHANDLE, phcopy : *mut super::msdrmdefs::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMDuplicatePubHandle(hpubin : super::msdrmdefs::DRMPUBHANDLE, phpubout : *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMDuplicateSession(hsessionin : super::msdrmdefs::DRMHSESSION, phsessionout : *mut super::msdrmdefs::DRMHSESSION) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMEncode(wszalgid : windows_sys::core::PCWSTR, udatalen : u32, pbdecodeddata : *mut u8, puencodedstringlen : *mut u32, wszencodedstring : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMEncrypt(hcryptoprovider : super::msdrmdefs::DRMHANDLE, iposition : u32, cnuminbytes : u32, pbindata : *mut u8, pcnumoutbytes : *mut u32, pboutdata : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMEnumerateLicense(hsession : super::msdrmdefs::DRMHSESSION, uflags : u32, uindex : u32, pfsharedflag : *mut windows_sys::core::BOOL, pucertificatedatalen : *mut u32, wszcertificatedata : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetApplicationSpecificData(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, uindex : u32, punamelength : *mut u32, wszname : windows_sys::core::PWSTR, puvaluelength : *mut u32, wszvalue : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttribute(hqueryroot : super::msdrmdefs::DRMHANDLE, wszattribute : windows_sys::core::PCWSTR, iwhich : u32, peencoding : *mut super::msdrmdefs::DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttributeCount(hqueryroot : super::msdrmdefs::DRMHANDLE, wszattribute : windows_sys::core::PCWSTR, pcattributes : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObject(hqueryroot : super::msdrmdefs::DRMHANDLE, wszsubobjecttype : windows_sys::core::PCWSTR, iwhich : u32, phsubobject : *mut super::msdrmdefs::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObjectCount(hqueryroot : super::msdrmdefs::DRMHANDLE, wszsubobjecttype : windows_sys::core::PCWSTR, pcsubobjects : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMGetCertificateChainCount(wszchain : windows_sys::core::PCWSTR, pccertcount : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetClientVersion(pdrmclientversioninfo : *mut super::msdrmdefs::DRM_CLIENT_VERSION_INFO) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetEnvironmentInfo(handle : super::msdrmdefs::DRMENVHANDLE, wszattribute : windows_sys::core::PCWSTR, peencoding : *mut super::msdrmdefs::DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetInfo(handle : super::msdrmdefs::DRMHANDLE, wszattribute : windows_sys::core::PCWSTR, peencoding : *const super::msdrmdefs::DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetIntervalTime(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, pcdays : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseInfo(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, psttimefrom : *mut super::minwinbase::SYSTEMTIME, psttimeuntil : *mut super::minwinbase::SYSTEMTIME, uflags : u32, pudistributionpointnamelength : *mut u32, wszdistributionpointname : windows_sys::core::PWSTR, pudistributionpointurllength : *mut u32, wszdistributionpointurl : windows_sys::core::PWSTR, phowner : *mut super::msdrmdefs::DRMPUBHANDLE, pfofficial : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseTemplate(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, puissuancelicensetemplatelength : *mut u32, wszissuancelicensetemplate : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetMetaData(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, pucontentidlength : *mut u32, wszcontentid : windows_sys::core::PWSTR, pucontentidtypelength : *mut u32, wszcontentidtype : windows_sys::core::PWSTR, puskuidlength : *mut u32, wszskuid : windows_sys::core::PWSTR, puskuidtypelength : *mut u32, wszskuidtype : windows_sys::core::PWSTR, pucontenttypelength : *mut u32, wszcontenttype : windows_sys::core::PWSTR, pucontentnamelength : *mut u32, wszcontentname : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetNameAndDescription(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, uindex : u32, pulcid : *mut u32, punamelength : *mut u32, wszname : windows_sys::core::PWSTR, pudescriptionlength : *mut u32, wszdescription : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetOwnerLicense(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, puownerlicenselength : *mut u32, wszownerlicense : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetProcAddress(hlibrary : super::msdrmdefs::DRMHANDLE, wszprocname : windows_sys::core::PCWSTR, ppfnprocaddress : *mut super::minwindef::FARPROC) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetRevocationPoint(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, puidlength : *mut u32, wszid : windows_sys::core::PWSTR, puidtypelength : *mut u32, wszidtype : windows_sys::core::PWSTR, puurllength : *mut u32, wszrl : windows_sys::core::PWSTR, pstfrequency : *mut super::minwinbase::SYSTEMTIME, punamelength : *mut u32, wszname : windows_sys::core::PWSTR, pupublickeylength : *mut u32, wszpublickey : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetRightExtendedInfo(hright : super::msdrmdefs::DRMPUBHANDLE, uindex : u32, puextendedinfonamelength : *mut u32, wszextendedinfoname : windows_sys::core::PWSTR, puextendedinfovaluelength : *mut u32, wszextendedinfovalue : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetRightInfo(hright : super::msdrmdefs::DRMPUBHANDLE, purightnamelength : *mut u32, wszrightname : windows_sys::core::PWSTR, pstfrom : *mut super::minwinbase::SYSTEMTIME, pstuntil : *mut super::minwinbase::SYSTEMTIME) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMGetSecurityProvider(uflags : u32, putypelen : *mut u32, wsztype : windows_sys::core::PWSTR, pupathlen : *mut u32, wszpath : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetServiceLocation(hclient : super::msdrmdefs::DRMHSESSION, uservicetype : u32, uservicelocation : u32, wszissuancelicense : windows_sys::core::PCWSTR, puserviceurllength : *mut u32, wszserviceurl : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicense(henv : super::msdrmdefs::DRMENVHANDLE, hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, uflags : u32, pbsymkey : *mut u8, cbsymkey : u32, wszsymkeytype : windows_sys::core::PCWSTR, wszclientlicensorcertificate : windows_sys::core::PCWSTR, pfncallback : super::msdrmdefs::DRMCALLBACK, wszurl : windows_sys::core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicenseEx(henv : super::msdrmdefs::DRMENVHANDLE, hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, uflags : u32, pbsymkey : *const u8, cbsymkey : u32, wszsymkeytype : windows_sys::core::PCWSTR, pvreserved : *const core::ffi::c_void, henablingprincipal : super::msdrmdefs::DRMHANDLE, hboundlicenseclc : super::msdrmdefs::DRMHANDLE, pfncallback : super::msdrmdefs::DRMCALLBACK, pvcontext : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetTime(henv : super::msdrmdefs::DRMENVHANDLE, etimeridtype : super::msdrmdefs::DRMTIMETYPE, potimeobject : *mut super::minwinbase::SYSTEMTIME) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttribute(hqueryroot : super::msdrmdefs::DRMQUERYHANDLE, wszattributetype : windows_sys::core::PCWSTR, iwhich : u32, peencoding : *mut super::msdrmdefs::DRMENCODINGTYPE, pcbuffer : *mut u32, pbbuffer : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttributeCount(hqueryroot : super::msdrmdefs::DRMQUERYHANDLE, wszattributetype : windows_sys::core::PCWSTR, pcattributes : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObject(hqueryroot : super::msdrmdefs::DRMQUERYHANDLE, wszsubobjecttype : windows_sys::core::PCWSTR, iindex : u32, phsubquery : *mut super::msdrmdefs::DRMQUERYHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObjectCount(hqueryroot : super::msdrmdefs::DRMQUERYHANDLE, wszsubobjecttype : windows_sys::core::PCWSTR, pcsubobjects : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUsagePolicy(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, uindex : u32, peusagepolicytype : *mut super::msdrmdefs::DRM_USAGEPOLICY_TYPE, pfexclusion : *mut windows_sys::core::BOOL, punamelength : *mut u32, wszname : windows_sys::core::PWSTR, puminversionlength : *mut u32, wszminversion : windows_sys::core::PWSTR, pumaxversionlength : *mut u32, wszmaxversion : windows_sys::core::PWSTR, pupublickeylength : *mut u32, wszpublickey : windows_sys::core::PWSTR, pudigestalgorithmlength : *mut u32, wszdigestalgorithm : windows_sys::core::PWSTR, pcbdigest : *mut u32, pbdigest : *mut u8) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUserInfo(huser : super::msdrmdefs::DRMPUBHANDLE, puusernamelength : *mut u32, wszusername : windows_sys::core::PWSTR, puuseridlength : *mut u32, wszuserid : windows_sys::core::PWSTR, puuseridtypelength : *mut u32, wszuseridtype : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUserRights(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, huser : super::msdrmdefs::DRMPUBHANDLE, uindex : u32, phright : *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMGetUsers(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, uindex : u32, phuser : *mut super::msdrmdefs::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMInitEnvironment(esecurityprovidertype : super::msdrmdefs::DRMSECURITYPROVIDERTYPE, especification : super::msdrmdefs::DRMSPECTYPE, wszsecurityprovider : windows_sys::core::PCWSTR, wszmanifestcredentials : windows_sys::core::PCWSTR, wszmachinecredentials : windows_sys::core::PCWSTR, phenv : *mut super::msdrmdefs::DRMENVHANDLE, phdefaultlibrary : *mut super::msdrmdefs::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMIsActivated(hclient : super::msdrmdefs::DRMHSESSION, uflags : u32, pactservinfo : *mut super::msdrmdefs::DRM_ACTSERV_INFO) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_windef")]
windows_link::link!("msdrm.dll" "system" fn DRMIsWindowProtected(hwnd : super::windef::HWND, pfprotected : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMLoadLibrary(henv : super::msdrmdefs::DRMENVHANDLE, especification : super::msdrmdefs::DRMSPECTYPE, wszlibraryprovider : windows_sys::core::PCWSTR, wszcredentials : windows_sys::core::PCWSTR, phlibrary : *mut super::msdrmdefs::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMParseUnboundLicense(wszcertificate : windows_sys::core::PCWSTR, phqueryroot : *mut super::msdrmdefs::DRMQUERYHANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMRegisterContent(fregister : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_msdrmdefs", feature = "Win32_windef"))]
windows_link::link!("msdrm.dll" "system" fn DRMRegisterProtectedWindow(henv : super::msdrmdefs::DRMENVHANDLE, hwnd : super::windef::HWND) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMRegisterRevocationList(henv : super::msdrmdefs::DRMENVHANDLE, wszrevocationlist : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("msdrm.dll" "system" fn DRMRepair() -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMSetApplicationSpecificData(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, fdelete : windows_sys::core::BOOL, wszname : windows_sys::core::PCWSTR, wszvalue : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMSetGlobalOptions(eglobaloptions : super::msdrmdefs::DRMGLOBALOPTIONS, pvdata : *mut core::ffi::c_void, dwlen : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMSetIntervalTime(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, cdays : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMSetMetaData(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, wszcontentid : windows_sys::core::PCWSTR, wszcontentidtype : windows_sys::core::PCWSTR, wszskuid : windows_sys::core::PCWSTR, wszskuidtype : windows_sys::core::PCWSTR, wszcontenttype : windows_sys::core::PCWSTR, wszcontentname : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMSetNameAndDescription(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, fdelete : windows_sys::core::BOOL, lcid : u32, wszname : windows_sys::core::PCWSTR, wszdescription : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_msdrmdefs"))]
windows_link::link!("msdrm.dll" "system" fn DRMSetRevocationPoint(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, fdelete : windows_sys::core::BOOL, wszid : windows_sys::core::PCWSTR, wszidtype : windows_sys::core::PCWSTR, wszurl : windows_sys::core::PCWSTR, pstfrequency : *mut super::minwinbase::SYSTEMTIME, wszname : windows_sys::core::PCWSTR, wszpublickey : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMSetUsagePolicy(hissuancelicense : super::msdrmdefs::DRMPUBHANDLE, eusagepolicytype : super::msdrmdefs::DRM_USAGEPOLICY_TYPE, fdelete : windows_sys::core::BOOL, fexclusion : windows_sys::core::BOOL, wszname : windows_sys::core::PCWSTR, wszminversion : windows_sys::core::PCWSTR, wszmaxversion : windows_sys::core::PCWSTR, wszpublickey : windows_sys::core::PCWSTR, wszdigestalgorithm : windows_sys::core::PCWSTR, pbdigest : *mut u8, cbdigest : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMVerify(wszdata : windows_sys::core::PCWSTR, pcattesteddata : *mut u32, wszattesteddata : windows_sys::core::PWSTR, petype : *mut super::msdrmdefs::DRMATTESTTYPE, pcprincipal : *mut u32, wszprincipal : windows_sys::core::PWSTR, pcmanifest : *mut u32, wszmanifest : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
