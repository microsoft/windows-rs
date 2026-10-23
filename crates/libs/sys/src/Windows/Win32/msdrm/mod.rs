#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAcquireAdvisories(hlicensestorage : super::DRMHSESSION, wszlicense : windows_sys::core::PCWSTR, wszurl : windows_sys::core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMAcquireIssuanceLicenseTemplate(hclient : super::DRMHSESSION, uflags : super::UINT, pvreserved : *mut core::ffi::c_void, ctemplates : super::UINT, pwsztemplateids : *const windows_sys::core::PCWSTR, wszurl : windows_sys::core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMAcquireLicense(hsession : super::DRMHSESSION, uflags : super::UINT, wszgroupidentitycredential : windows_sys::core::PCWSTR, wszrequestedrights : windows_sys::core::PCWSTR, wszcustomdata : windows_sys::core::PCWSTR, wszurl : windows_sys::core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMAddLicense(hlicensestorage : super::DRMHSESSION, uflags : super::UINT, wszlicense : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMAddRightWithUser(hissuancelicense : super::DRMPUBHANDLE, hright : super::DRMPUBHANDLE, huser : super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMAttest(henablingprincipal : super::DRMHANDLE, wszdata : windows_sys::core::PCWSTR, etype : super::DRMATTESTTYPE, pcattestedblob : *mut super::UINT, wszattestedblob : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMCheckSecurity(henv : super::DRMENVHANDLE, clevel : super::UINT) -> windows_sys::core::HRESULT);
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
#[cfg(feature = "shared")]
windows_link::link!("msdrm.dll" "system" fn DRMConstructCertificateChain(ccertificates : super::UINT, rgwszcertificates : *const windows_sys::core::PCWSTR, pcchain : *mut super::UINT, wszchain : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMCreateClientSession(pfncallback : super::DRMCALLBACK, ucallbackversion : super::UINT, wszgroupidprovidertype : windows_sys::core::PCWSTR, wszgroupid : windows_sys::core::PCWSTR, phclient : *mut super::DRMHSESSION) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsDecryptor(hboundlicense : super::DRMHANDLE, wszright : windows_sys::core::PCWSTR, hauxlib : super::DRMHANDLE, wszauxplug : windows_sys::core::PCWSTR, phdecryptor : *mut super::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateEnablingBitsEncryptor(hboundlicense : super::DRMHANDLE, wszright : windows_sys::core::PCWSTR, hauxlib : super::DRMHANDLE, wszauxplug : windows_sys::core::PCWSTR, phencryptor : *mut super::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMCreateIssuanceLicense(psttimefrom : *mut super::SYSTEMTIME, psttimeuntil : *mut super::SYSTEMTIME, wszreferralinfoname : windows_sys::core::PCWSTR, wszreferralinfourl : windows_sys::core::PCWSTR, howner : super::DRMPUBHANDLE, wszissuancelicense : windows_sys::core::PCWSTR, hboundlicense : super::DRMHANDLE, phissuancelicense : *mut super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMCreateLicenseStorageSession(henv : super::DRMENVHANDLE, hdefaultlibrary : super::DRMHANDLE, hclient : super::DRMHSESSION, uflags : super::UINT, wszissuancelicense : windows_sys::core::PCWSTR, phlicensestorage : *mut super::DRMHSESSION) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMCreateRight(wszrightname : windows_sys::core::PCWSTR, pstfrom : *mut super::SYSTEMTIME, pstuntil : *mut super::SYSTEMTIME, cextendedinfo : super::UINT, pwszextendedinfoname : *const windows_sys::core::PCWSTR, pwszextendedinfovalue : *const windows_sys::core::PCWSTR, phright : *mut super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMCreateUser(wszusername : windows_sys::core::PCWSTR, wszuserid : windows_sys::core::PCWSTR, wszuseridtype : windows_sys::core::PCWSTR, phuser : *mut super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("msdrm.dll" "system" fn DRMDecode(wszalgid : windows_sys::core::PCWSTR, wszencodedstring : windows_sys::core::PCWSTR, pudecodeddatalen : *mut super::UINT, pbdecodeddata : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("msdrm.dll" "system" fn DRMDeconstructCertificateChain(wszchain : windows_sys::core::PCWSTR, iwhich : super::UINT, pccert : *mut super::UINT, wszcert : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMDecrypt(hcryptoprovider : super::DRMHANDLE, iposition : super::UINT, cnuminbytes : super::UINT, pbindata : *mut super::BYTE, pcnumoutbytes : *mut super::UINT, pboutdata : *mut super::BYTE) -> windows_sys::core::HRESULT);
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
#[cfg(feature = "shared")]
windows_link::link!("msdrm.dll" "system" fn DRMEncode(wszalgid : windows_sys::core::PCWSTR, udatalen : super::UINT, pbdecodeddata : *mut super::BYTE, puencodedstringlen : *mut super::UINT, wszencodedstring : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMEncrypt(hcryptoprovider : super::DRMHANDLE, iposition : super::UINT, cnuminbytes : super::UINT, pbindata : *mut super::BYTE, pcnumoutbytes : *mut super::UINT, pboutdata : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMEnumerateLicense(hsession : super::DRMHSESSION, uflags : super::UINT, uindex : super::UINT, pfsharedflag : *mut windows_sys::core::BOOL, pucertificatedatalen : *mut super::UINT, wszcertificatedata : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetApplicationSpecificData(hissuancelicense : super::DRMPUBHANDLE, uindex : super::UINT, punamelength : *mut super::UINT, wszname : windows_sys::core::PWSTR, puvaluelength : *mut super::UINT, wszvalue : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttribute(hqueryroot : super::DRMHANDLE, wszattribute : windows_sys::core::PCWSTR, iwhich : super::UINT, peencoding : *mut super::DRMENCODINGTYPE, pcbuffer : *mut super::UINT, pbbuffer : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetBoundLicenseAttributeCount(hqueryroot : super::DRMHANDLE, wszattribute : windows_sys::core::PCWSTR, pcattributes : *mut super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObject(hqueryroot : super::DRMHANDLE, wszsubobjecttype : windows_sys::core::PCWSTR, iwhich : super::UINT, phsubobject : *mut super::DRMHANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetBoundLicenseObjectCount(hqueryroot : super::DRMHANDLE, wszsubobjecttype : windows_sys::core::PCWSTR, pcsubobjects : *mut super::UINT) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("msdrm.dll" "system" fn DRMGetCertificateChainCount(wszchain : windows_sys::core::PCWSTR, pccertcount : *mut super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetEnvironmentInfo(handle : super::DRMENVHANDLE, wszattribute : windows_sys::core::PCWSTR, peencoding : *mut super::DRMENCODINGTYPE, pcbuffer : *mut super::UINT, pbbuffer : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetInfo(handle : super::DRMHANDLE, wszattribute : windows_sys::core::PCWSTR, peencoding : *const super::DRMENCODINGTYPE, pcbuffer : *mut super::UINT, pbbuffer : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetIntervalTime(hissuancelicense : super::DRMPUBHANDLE, pcdays : *mut super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseInfo(hissuancelicense : super::DRMPUBHANDLE, psttimefrom : *mut super::SYSTEMTIME, psttimeuntil : *mut super::SYSTEMTIME, uflags : super::UINT, pudistributionpointnamelength : *mut super::UINT, wszdistributionpointname : windows_sys::core::PWSTR, pudistributionpointurllength : *mut super::UINT, wszdistributionpointurl : windows_sys::core::PWSTR, phowner : *mut super::DRMPUBHANDLE, pfofficial : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetIssuanceLicenseTemplate(hissuancelicense : super::DRMPUBHANDLE, puissuancelicensetemplatelength : *mut super::UINT, wszissuancelicensetemplate : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetMetaData(hissuancelicense : super::DRMPUBHANDLE, pucontentidlength : *mut super::UINT, wszcontentid : windows_sys::core::PWSTR, pucontentidtypelength : *mut super::UINT, wszcontentidtype : windows_sys::core::PWSTR, puskuidlength : *mut super::UINT, wszskuid : windows_sys::core::PWSTR, puskuidtypelength : *mut super::UINT, wszskuidtype : windows_sys::core::PWSTR, pucontenttypelength : *mut super::UINT, wszcontenttype : windows_sys::core::PWSTR, pucontentnamelength : *mut super::UINT, wszcontentname : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetNameAndDescription(hissuancelicense : super::DRMPUBHANDLE, uindex : super::UINT, pulcid : *mut super::UINT, punamelength : *mut super::UINT, wszname : windows_sys::core::PWSTR, pudescriptionlength : *mut super::UINT, wszdescription : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetOwnerLicense(hissuancelicense : super::DRMPUBHANDLE, puownerlicenselength : *mut super::UINT, wszownerlicense : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetProcAddress(hlibrary : super::DRMHANDLE, wszprocname : windows_sys::core::PCWSTR, ppfnprocaddress : *mut super::FARPROC) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetRevocationPoint(hissuancelicense : super::DRMPUBHANDLE, puidlength : *mut super::UINT, wszid : windows_sys::core::PWSTR, puidtypelength : *mut super::UINT, wszidtype : windows_sys::core::PWSTR, puurllength : *mut super::UINT, wszrl : windows_sys::core::PWSTR, pstfrequency : *mut super::SYSTEMTIME, punamelength : *mut super::UINT, wszname : windows_sys::core::PWSTR, pupublickeylength : *mut super::UINT, wszpublickey : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetRightExtendedInfo(hright : super::DRMPUBHANDLE, uindex : super::UINT, puextendedinfonamelength : *mut super::UINT, wszextendedinfoname : windows_sys::core::PWSTR, puextendedinfovaluelength : *mut super::UINT, wszextendedinfovalue : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetRightInfo(hright : super::DRMPUBHANDLE, purightnamelength : *mut super::UINT, wszrightname : windows_sys::core::PWSTR, pstfrom : *mut super::SYSTEMTIME, pstuntil : *mut super::SYSTEMTIME) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("msdrm.dll" "system" fn DRMGetSecurityProvider(uflags : super::UINT, putypelen : *mut super::UINT, wsztype : windows_sys::core::PWSTR, pupathlen : *mut super::UINT, wszpath : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetServiceLocation(hclient : super::DRMHSESSION, uservicetype : super::UINT, uservicelocation : super::UINT, wszissuancelicense : windows_sys::core::PCWSTR, puserviceurllength : *mut super::UINT, wszserviceurl : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicense(henv : super::DRMENVHANDLE, hissuancelicense : super::DRMPUBHANDLE, uflags : super::UINT, pbsymkey : *mut super::BYTE, cbsymkey : super::UINT, wszsymkeytype : windows_sys::core::PCWSTR, wszclientlicensorcertificate : windows_sys::core::PCWSTR, pfncallback : super::DRMCALLBACK, wszurl : windows_sys::core::PCWSTR, pvcontext : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared", feature = "winnt"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetSignedIssuanceLicenseEx(henv : super::DRMENVHANDLE, hissuancelicense : super::DRMPUBHANDLE, uflags : super::UINT, pbsymkey : *const super::BYTE, cbsymkey : super::UINT, wszsymkeytype : windows_sys::core::PCWSTR, pvreserved : super::PVOID, henablingprincipal : super::DRMHANDLE, hboundlicenseclc : super::DRMHANDLE, pfncallback : super::DRMCALLBACK, pvcontext : super::PVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetTime(henv : super::DRMENVHANDLE, etimeridtype : super::DRMTIMETYPE, potimeobject : *mut super::SYSTEMTIME) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttribute(hqueryroot : super::DRMQUERYHANDLE, wszattributetype : windows_sys::core::PCWSTR, iwhich : super::UINT, peencoding : *mut super::DRMENCODINGTYPE, pcbuffer : *mut super::UINT, pbbuffer : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseAttributeCount(hqueryroot : super::DRMQUERYHANDLE, wszattributetype : windows_sys::core::PCWSTR, pcattributes : *mut super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObject(hqueryroot : super::DRMQUERYHANDLE, wszsubobjecttype : windows_sys::core::PCWSTR, iindex : super::UINT, phsubquery : *mut super::DRMQUERYHANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetUnboundLicenseObjectCount(hqueryroot : super::DRMQUERYHANDLE, wszsubobjecttype : windows_sys::core::PCWSTR, pcsubobjects : *mut super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetUsagePolicy(hissuancelicense : super::DRMPUBHANDLE, uindex : super::UINT, peusagepolicytype : *mut super::DRM_USAGEPOLICY_TYPE, pfexclusion : *mut windows_sys::core::BOOL, punamelength : *mut super::UINT, wszname : windows_sys::core::PWSTR, puminversionlength : *mut super::UINT, wszminversion : windows_sys::core::PWSTR, pumaxversionlength : *mut super::UINT, wszmaxversion : windows_sys::core::PWSTR, pupublickeylength : *mut super::UINT, wszpublickey : windows_sys::core::PWSTR, pudigestalgorithmlength : *mut super::UINT, wszdigestalgorithm : windows_sys::core::PWSTR, pcbdigest : *mut super::UINT, pbdigest : *mut super::BYTE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetUserInfo(huser : super::DRMPUBHANDLE, puusernamelength : *mut super::UINT, wszusername : windows_sys::core::PWSTR, puuseridlength : *mut super::UINT, wszuserid : windows_sys::core::PWSTR, puuseridtypelength : *mut super::UINT, wszuseridtype : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetUserRights(hissuancelicense : super::DRMPUBHANDLE, huser : super::DRMPUBHANDLE, uindex : super::UINT, phright : *mut super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMGetUsers(hissuancelicense : super::DRMPUBHANDLE, uindex : super::UINT, phuser : *mut super::DRMPUBHANDLE) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMInitEnvironment(esecurityprovidertype : super::DRMSECURITYPROVIDERTYPE, especification : super::DRMSPECTYPE, wszsecurityprovider : windows_sys::core::PCWSTR, wszmanifestcredentials : windows_sys::core::PCWSTR, wszmachinecredentials : windows_sys::core::PCWSTR, phenv : *mut super::DRMENVHANDLE, phdefaultlibrary : *mut super::DRMHANDLE) -> windows_sys::core::HRESULT);
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
#[cfg(all(feature = "minwindef", feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMSetGlobalOptions(eglobaloptions : super::DRMGLOBALOPTIONS, pvdata : super::LPVOID, dwlen : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMSetIntervalTime(hissuancelicense : super::DRMPUBHANDLE, cdays : super::UINT) -> windows_sys::core::HRESULT);
#[cfg(feature = "msdrmdefs")]
windows_link::link!("msdrm.dll" "system" fn DRMSetMetaData(hissuancelicense : super::DRMPUBHANDLE, wszcontentid : windows_sys::core::PCWSTR, wszcontentidtype : windows_sys::core::PCWSTR, wszskuid : windows_sys::core::PCWSTR, wszskuidtype : windows_sys::core::PCWSTR, wszcontenttype : windows_sys::core::PCWSTR, wszcontentname : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMSetNameAndDescription(hissuancelicense : super::DRMPUBHANDLE, fdelete : windows_sys::core::BOOL, lcid : super::UINT, wszname : windows_sys::core::PCWSTR, wszdescription : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwinbase", feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMSetRevocationPoint(hissuancelicense : super::DRMPUBHANDLE, fdelete : windows_sys::core::BOOL, wszid : windows_sys::core::PCWSTR, wszidtype : windows_sys::core::PCWSTR, wszurl : windows_sys::core::PCWSTR, pstfrequency : *mut super::SYSTEMTIME, wszname : windows_sys::core::PCWSTR, wszpublickey : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMSetUsagePolicy(hissuancelicense : super::DRMPUBHANDLE, eusagepolicytype : super::DRM_USAGEPOLICY_TYPE, fdelete : windows_sys::core::BOOL, fexclusion : windows_sys::core::BOOL, wszname : windows_sys::core::PCWSTR, wszminversion : windows_sys::core::PCWSTR, wszmaxversion : windows_sys::core::PCWSTR, wszpublickey : windows_sys::core::PCWSTR, wszdigestalgorithm : windows_sys::core::PCWSTR, pbdigest : *mut super::BYTE, cbdigest : super::UINT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "msdrmdefs", feature = "shared"))]
windows_link::link!("msdrm.dll" "system" fn DRMVerify(wszdata : windows_sys::core::PCWSTR, pcattesteddata : *mut super::UINT, wszattesteddata : windows_sys::core::PWSTR, petype : *mut super::DRMATTESTTYPE, pcprincipal : *mut super::UINT, wszprincipal : windows_sys::core::PWSTR, pcmanifest : *mut super::UINT, wszmanifest : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
