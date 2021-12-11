#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMAcquireAdvisories(hlicensestorage: u32, wszlicense: super::super::Foundation::PWSTR, wszurl: super::super::Foundation::PWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMAcquireIssuanceLicenseTemplate(hclient: u32, uflags: u32, pvreserved: *mut ::core::ffi::c_void, ctemplates: u32, pwsztemplateids: *const super::super::Foundation::PWSTR, wszurl: super::super::Foundation::PWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMAcquireLicense(hsession: u32, uflags: u32, wszgroupidentitycredential: super::super::Foundation::PWSTR, wszrequestedrights: super::super::Foundation::PWSTR, wszcustomdata: super::super::Foundation::PWSTR, wszurl: super::super::Foundation::PWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMActivate(hclient: u32, uflags: u32, ulangid: u32, pactservinfo: *mut DRM_ACTSERV_INFO, pvcontext: *mut ::core::ffi::c_void, hparentwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMAddLicense(hlicensestorage: u32, uflags: u32, wszlicense: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMAddRightWithUser(hissuancelicense: u32, hright: u32, huser: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMAttest(henablingprincipal: u32, wszdata: super::super::Foundation::PWSTR, etype: DRMATTESTTYPE, pcattestedblob: *mut u32, wszattestedblob: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMCheckSecurity(henv: u32, clevel: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMClearAllRights(hissuancelicense: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMCloseEnvironmentHandle(henv: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMCloseHandle(handle: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMClosePubHandle(hpub: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMCloseQueryHandle(hquery: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMCloseSession(hsession: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMConstructCertificateChain(ccertificates: u32, rgwszcertificates: *const super::super::Foundation::PWSTR, pcchain: *mut u32, wszchain: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateBoundLicense(henv: u32, pparams: *mut DRMBOUNDLICENSEPARAMS, wszlicensechain: super::super::Foundation::PWSTR, phboundlicense: *mut u32, pherrorlog: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateClientSession(pfncallback: DRMCALLBACK, ucallbackversion: u32, wszgroupidprovidertype: super::super::Foundation::PWSTR, wszgroupid: super::super::Foundation::PWSTR, phclient: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateEnablingBitsDecryptor(hboundlicense: u32, wszright: super::super::Foundation::PWSTR, hauxlib: u32, wszauxplug: super::super::Foundation::PWSTR, phdecryptor: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateEnablingBitsEncryptor(hboundlicense: u32, wszright: super::super::Foundation::PWSTR, hauxlib: u32, wszauxplug: super::super::Foundation::PWSTR, phencryptor: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateEnablingPrincipal(henv: u32, hlibrary: u32, wszobject: super::super::Foundation::PWSTR, pidprincipal: *mut DRMID, wszcredentials: super::super::Foundation::PWSTR, phenablingprincipal: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateIssuanceLicense(psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, wszreferralinfoname: super::super::Foundation::PWSTR, wszreferralinfourl: super::super::Foundation::PWSTR, howner: u32, wszissuancelicense: super::super::Foundation::PWSTR, hboundlicense: u32, phissuancelicense: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateLicenseStorageSession(henv: u32, hdefaultlibrary: u32, hclient: u32, uflags: u32, wszissuancelicense: super::super::Foundation::PWSTR, phlicensestorage: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateRight(wszrightname: super::super::Foundation::PWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME, cextendedinfo: u32, pwszextendedinfoname: *const super::super::Foundation::PWSTR, pwszextendedinfovalue: *const super::super::Foundation::PWSTR, phright: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMCreateUser(wszusername: super::super::Foundation::PWSTR, wszuserid: super::super::Foundation::PWSTR, wszuseridtype: super::super::Foundation::PWSTR, phuser: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMDecode(wszalgid: super::super::Foundation::PWSTR, wszencodedstring: super::super::Foundation::PWSTR, pudecodeddatalen: *mut u32, pbdecodeddata: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMDeconstructCertificateChain(wszchain: super::super::Foundation::PWSTR, iwhich: u32, pccert: *mut u32, wszcert: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMDecrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMDeleteLicense(hsession: u32, wszlicenseid: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMDuplicateEnvironmentHandle(htocopy: u32, phcopy: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMDuplicateHandle(htocopy: u32, phcopy: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMDuplicatePubHandle(hpubin: u32, phpubout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMDuplicateSession(hsessionin: u32, phsessionout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMEncode(wszalgid: super::super::Foundation::PWSTR, udatalen: u32, pbdecodeddata: *mut u8, puencodedstringlen: *mut u32, wszencodedstring: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMEncrypt(hcryptoprovider: u32, iposition: u32, cnuminbytes: u32, pbindata: *mut u8, pcnumoutbytes: *mut u32, pboutdata: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMEnumerateLicense(hsession: u32, uflags: u32, uindex: u32, pfsharedflag: *mut super::super::Foundation::BOOL, pucertificatedatalen: *mut u32, wszcertificatedata: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetApplicationSpecificData(hissuancelicense: u32, uindex: u32, punamelength: *mut u32, wszname: super::super::Foundation::PWSTR, puvaluelength: *mut u32, wszvalue: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetBoundLicenseAttribute(hqueryroot: u32, wszattribute: super::super::Foundation::PWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetBoundLicenseAttributeCount(hqueryroot: u32, wszattribute: super::super::Foundation::PWSTR, pcattributes: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetBoundLicenseObject(hqueryroot: u32, wszsubobjecttype: super::super::Foundation::PWSTR, iwhich: u32, phsubobject: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetBoundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: super::super::Foundation::PWSTR, pcsubobjects: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetCertificateChainCount(wszchain: super::super::Foundation::PWSTR, pccertcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMGetClientVersion(pdrmclientversioninfo: *mut DRM_CLIENT_VERSION_INFO) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetEnvironmentInfo(handle: u32, wszattribute: super::super::Foundation::PWSTR, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetInfo(handle: u32, wszattribute: super::super::Foundation::PWSTR, peencoding: *const DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMGetIntervalTime(hissuancelicense: u32, pcdays: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetIssuanceLicenseInfo(hissuancelicense: u32, psttimefrom: *mut super::super::Foundation::SYSTEMTIME, psttimeuntil: *mut super::super::Foundation::SYSTEMTIME, uflags: u32, pudistributionpointnamelength: *mut u32, wszdistributionpointname: super::super::Foundation::PWSTR, pudistributionpointurllength: *mut u32, wszdistributionpointurl: super::super::Foundation::PWSTR, phowner: *mut u32, pfofficial: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetIssuanceLicenseTemplate(hissuancelicense: u32, puissuancelicensetemplatelength: *mut u32, wszissuancelicensetemplate: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetMetaData(hissuancelicense: u32, pucontentidlength: *mut u32, wszcontentid: super::super::Foundation::PWSTR, pucontentidtypelength: *mut u32, wszcontentidtype: super::super::Foundation::PWSTR, puskuidlength: *mut u32, wszskuid: super::super::Foundation::PWSTR, puskuidtypelength: *mut u32, wszskuidtype: super::super::Foundation::PWSTR, pucontenttypelength: *mut u32, wszcontenttype: super::super::Foundation::PWSTR, pucontentnamelength: *mut u32, wszcontentname: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetNameAndDescription(hissuancelicense: u32, uindex: u32, pulcid: *mut u32, punamelength: *mut u32, wszname: super::super::Foundation::PWSTR, pudescriptionlength: *mut u32, wszdescription: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetOwnerLicense(hissuancelicense: u32, puownerlicenselength: *mut u32, wszownerlicense: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetProcAddress(hlibrary: u32, wszprocname: super::super::Foundation::PWSTR, ppfnprocaddress: *mut super::super::Foundation::FARPROC) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetRevocationPoint(hissuancelicense: u32, puidlength: *mut u32, wszid: super::super::Foundation::PWSTR, puidtypelength: *mut u32, wszidtype: super::super::Foundation::PWSTR, puurllength: *mut u32, wszrl: super::super::Foundation::PWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, punamelength: *mut u32, wszname: super::super::Foundation::PWSTR, pupublickeylength: *mut u32, wszpublickey: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetRightExtendedInfo(hright: u32, uindex: u32, puextendedinfonamelength: *mut u32, wszextendedinfoname: super::super::Foundation::PWSTR, puextendedinfovaluelength: *mut u32, wszextendedinfovalue: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetRightInfo(hright: u32, purightnamelength: *mut u32, wszrightname: super::super::Foundation::PWSTR, pstfrom: *mut super::super::Foundation::SYSTEMTIME, pstuntil: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetSecurityProvider(uflags: u32, putypelen: *mut u32, wsztype: super::super::Foundation::PWSTR, pupathlen: *mut u32, wszpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetServiceLocation(hclient: u32, uservicetype: u32, uservicelocation: u32, wszissuancelicense: super::super::Foundation::PWSTR, puserviceurllength: *mut u32, wszserviceurl: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetSignedIssuanceLicense(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *mut u8, cbsymkey: u32, wszsymkeytype: super::super::Foundation::PWSTR, wszclientlicensorcertificate: super::super::Foundation::PWSTR, pfncallback: DRMCALLBACK, wszurl: super::super::Foundation::PWSTR, pvcontext: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetSignedIssuanceLicenseEx(henv: u32, hissuancelicense: u32, uflags: u32, pbsymkey: *const u8, cbsymkey: u32, wszsymkeytype: super::super::Foundation::PWSTR, pvreserved: *const ::core::ffi::c_void, henablingprincipal: u32, hboundlicenseclc: u32, pfncallback: DRMCALLBACK, pvcontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetTime(henv: u32, etimeridtype: DRMTIMETYPE, potimeobject: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetUnboundLicenseAttribute(hqueryroot: u32, wszattributetype: super::super::Foundation::PWSTR, iwhich: u32, peencoding: *mut DRMENCODINGTYPE, pcbuffer: *mut u32, pbbuffer: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetUnboundLicenseAttributeCount(hqueryroot: u32, wszattributetype: super::super::Foundation::PWSTR, pcattributes: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetUnboundLicenseObject(hqueryroot: u32, wszsubobjecttype: super::super::Foundation::PWSTR, iindex: u32, phsubquery: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetUnboundLicenseObjectCount(hqueryroot: u32, wszsubobjecttype: super::super::Foundation::PWSTR, pcsubobjects: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetUsagePolicy(hissuancelicense: u32, uindex: u32, peusagepolicytype: *mut DRM_USAGEPOLICY_TYPE, pfexclusion: *mut super::super::Foundation::BOOL, punamelength: *mut u32, wszname: super::super::Foundation::PWSTR, puminversionlength: *mut u32, wszminversion: super::super::Foundation::PWSTR, pumaxversionlength: *mut u32, wszmaxversion: super::super::Foundation::PWSTR, pupublickeylength: *mut u32, wszpublickey: super::super::Foundation::PWSTR, pudigestalgorithmlength: *mut u32, wszdigestalgorithm: super::super::Foundation::PWSTR, pcbdigest: *mut u32, pbdigest: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMGetUserInfo(huser: u32, puusernamelength: *mut u32, wszusername: super::super::Foundation::PWSTR, puuseridlength: *mut u32, wszuserid: super::super::Foundation::PWSTR, puuseridtypelength: *mut u32, wszuseridtype: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMGetUserRights(hissuancelicense: u32, huser: u32, uindex: u32, phright: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMGetUsers(hissuancelicense: u32, uindex: u32, phuser: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMInitEnvironment(esecurityprovidertype: DRMSECURITYPROVIDERTYPE, especification: DRMSPECTYPE, wszsecurityprovider: super::super::Foundation::PWSTR, wszmanifestcredentials: super::super::Foundation::PWSTR, wszmachinecredentials: super::super::Foundation::PWSTR, phenv: *mut u32, phdefaultlibrary: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMIsActivated(hclient: u32, uflags: u32, pactservinfo: *mut DRM_ACTSERV_INFO) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMIsWindowProtected(hwnd: super::super::Foundation::HWND, pfprotected: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMLoadLibrary(henv: u32, especification: DRMSPECTYPE, wszlibraryprovider: super::super::Foundation::PWSTR, wszcredentials: super::super::Foundation::PWSTR, phlibrary: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMParseUnboundLicense(wszcertificate: super::super::Foundation::PWSTR, phqueryroot: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMRegisterContent(fregister: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMRegisterProtectedWindow(henv: u32, hwnd: super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMRegisterRevocationList(henv: u32, wszrevocationlist: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMRepair() -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetApplicationSpecificData(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, wszname: super::super::Foundation::PWSTR, wszvalue: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMSetGlobalOptions(eglobaloptions: DRMGLOBALOPTIONS, pvdata: *mut ::core::ffi::c_void, dwlen: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
    pub fn DRMSetIntervalTime(hissuancelicense: u32, cdays: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetMetaData(hissuancelicense: u32, wszcontentid: super::super::Foundation::PWSTR, wszcontentidtype: super::super::Foundation::PWSTR, wszskuid: super::super::Foundation::PWSTR, wszskuidtype: super::super::Foundation::PWSTR, wszcontenttype: super::super::Foundation::PWSTR, wszcontentname: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetNameAndDescription(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, lcid: u32, wszname: super::super::Foundation::PWSTR, wszdescription: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetRevocationPoint(hissuancelicense: u32, fdelete: super::super::Foundation::BOOL, wszid: super::super::Foundation::PWSTR, wszidtype: super::super::Foundation::PWSTR, wszurl: super::super::Foundation::PWSTR, pstfrequency: *mut super::super::Foundation::SYSTEMTIME, wszname: super::super::Foundation::PWSTR, wszpublickey: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMSetUsagePolicy(hissuancelicense: u32, eusagepolicytype: DRM_USAGEPOLICY_TYPE, fdelete: super::super::Foundation::BOOL, fexclusion: super::super::Foundation::BOOL, wszname: super::super::Foundation::PWSTR, wszminversion: super::super::Foundation::PWSTR, wszmaxversion: super::super::Foundation::PWSTR, wszpublickey: super::super::Foundation::PWSTR, wszdigestalgorithm: super::super::Foundation::PWSTR, pbdigest: *mut u8, cbdigest: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DRMVerify(wszdata: super::super::Foundation::PWSTR, pcattesteddata: *mut u32, wszattesteddata: super::super::Foundation::PWSTR, petype: *mut DRMATTESTTYPE, pcprincipal: *mut u32, wszprincipal: super::super::Foundation::PWSTR, pcmanifest: *mut u32, wszmanifest: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMACTSERVINFOVERSION: u32 = 0u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub type DRMATTESTTYPE = i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMATTESTTYPE_FULLENVIRONMENT: DRMATTESTTYPE = 0i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMATTESTTYPE_HASHONLY: DRMATTESTTYPE = 1i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMBINDINGFLAGS_IGNORE_VALIDITY_INTERVALS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRMBOUNDLICENSEPARAMS {
    pub uVersion: u32,
    pub hEnablingPrincipal: u32,
    pub hSecureStore: u32,
    pub wszRightsRequested: super::super::Foundation::PWSTR,
    pub wszRightsGroup: super::super::Foundation::PWSTR,
    pub idResource: DRMID,
    pub cAuthenticatorCount: u32,
    pub rghAuthenticators: *mut u32,
    pub wszDefaultEnablingPrincipalCredentials: super::super::Foundation::PWSTR,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRMBOUNDLICENSEPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRMBOUNDLICENSEPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMBOUNDLICENSEPARAMSVERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub type DRMCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: DRM_STATUS_MSG, param1: ::windows_sys::core::HRESULT, param2: *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT>;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMCALLBACKVERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMCLIENTSTRUCTVERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub type DRMENCODINGTYPE = i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMENCODINGTYPE_BASE64: DRMENCODINGTYPE = 0i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMENCODINGTYPE_STRING: DRMENCODINGTYPE = 1i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMENCODINGTYPE_LONG: DRMENCODINGTYPE = 2i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMENCODINGTYPE_TIME: DRMENCODINGTYPE = 3i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMENCODINGTYPE_UINT: DRMENCODINGTYPE = 4i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMENCODINGTYPE_RAW: DRMENCODINGTYPE = 5i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMENVHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub type DRMGLOBALOPTIONS = i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMGLOBALOPTIONS_USE_WINHTTP: DRMGLOBALOPTIONS = 0i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMGLOBALOPTIONS_USE_SERVERSECURITYPROCESSOR: DRMGLOBALOPTIONS = 1i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMHSESSION_INVALID: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRMID {
    pub uVersion: u32,
    pub wszIDType: super::super::Foundation::PWSTR,
    pub wszID: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRMID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRMID {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMIDVERSION: u32 = 0u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMLICENSEACQDATAVERSION: u32 = 0u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMPUBHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMQUERYHANDLE_INVALID: u32 = 0u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub type DRMSECURITYPROVIDERTYPE = i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMSECURITYPROVIDERTYPE_SOFTWARESECREP: DRMSECURITYPROVIDERTYPE = 0i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub type DRMSPECTYPE = i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMSPECTYPE_UNKNOWN: DRMSPECTYPE = 0i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMSPECTYPE_FILENAME: DRMSPECTYPE = 1i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub type DRMTIMETYPE = i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMTIMETYPE_SYSTEMUTC: DRMTIMETYPE = 0i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRMTIMETYPE_SYSTEMLOCAL: DRMTIMETYPE = 1i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_ACTIVATE_CANCEL: u32 = 8u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_ACTIVATE_DELAYED: u32 = 64u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_ACTIVATE_GROUPIDENTITY: u32 = 2u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_ACTIVATE_MACHINE: u32 = 1u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_ACTIVATE_SHARED_GROUPIDENTITY: u32 = 32u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_ACTIVATE_SILENT: u32 = 16u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_ACTIVATE_TEMPORARY: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRM_ACTSERV_INFO {
    pub uVersion: u32,
    pub wszPubKey: super::super::Foundation::PWSTR,
    pub wszURL: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRM_ACTSERV_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRM_ACTSERV_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_ADD_LICENSE_NOPERSIST: u32 = 0u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_ADD_LICENSE_PERSIST: u32 = 1u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_AILT_CANCEL: u32 = 4u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_AILT_NONSILENT: u32 = 1u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_AILT_OBTAIN_ALL: u32 = 2u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_AL_CANCEL: u32 = 4u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_AL_FETCHNOADVISORY: u32 = 8u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_AL_NONSILENT: u32 = 1u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_AL_NOPERSIST: u32 = 2u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_AL_NOUI: u32 = 16u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_AUTO_GENERATE_KEY: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
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
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub type DRM_DISTRIBUTION_POINT_INFO = i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_DISTRIBUTION_POINT_LICENSE_ACQUISITION: DRM_DISTRIBUTION_POINT_INFO = 0i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_DISTRIBUTION_POINT_PUBLISHING: DRM_DISTRIBUTION_POINT_INFO = 1i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_DISTRIBUTION_POINT_REFERRAL_INFO: DRM_DISTRIBUTION_POINT_INFO = 2i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_CLIENTLICENSOR: u32 = 128u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_CLIENTLICENSOR_LID: u32 = 256u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_EUL: u32 = 32u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_EUL_LID: u32 = 64u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_EXPIRED: u32 = 4096u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_GROUPIDENTITY: u32 = 2u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_GROUPIDENTITY_LID: u32 = 8u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_GROUPIDENTITY_NAME: u32 = 4u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE: u32 = 16384u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_ISSUANCELICENSE_TEMPLATE_LID: u32 = 32768u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_ISSUERNAME: u32 = 8192u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_MACHINE: u32 = 1u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_REVOCATIONLIST: u32 = 1024u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_REVOCATIONLIST_LID: u32 = 2048u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_SPECIFIED_CLIENTLICENSOR: u32 = 512u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_EL_SPECIFIED_GROUPIDENTITY: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Data_RightsManagement', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRM_LICENSE_ACQ_DATA {
    pub uVersion: u32,
    pub wszURL: super::super::Foundation::PWSTR,
    pub wszLocalFilename: super::super::Foundation::PWSTR,
    pub pbPostData: *mut u8,
    pub dwPostDataSize: u32,
    pub wszFriendlyName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRM_LICENSE_ACQ_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRM_LICENSE_ACQ_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_LOCKBOXTYPE_BLACKBOX: u32 = 2u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_LOCKBOXTYPE_DEFAULT: u32 = 2u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_LOCKBOXTYPE_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_LOCKBOXTYPE_WHITEBOX: u32 = 1u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_OWNER_LICENSE_NOPERSIST: u32 = 32u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_REUSE_KEY: u32 = 64u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_SERVER_ISSUANCELICENSE: u32 = 8u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_SERVICE_LOCATION_ENTERPRISE: u32 = 2u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_SERVICE_LOCATION_INTERNET: u32 = 1u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_SERVICE_TYPE_ACTIVATION: u32 = 1u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_SERVICE_TYPE_CERTIFICATION: u32 = 2u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_SERVICE_TYPE_CLIENTLICENSOR: u32 = 8u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_SERVICE_TYPE_PUBLISHING: u32 = 4u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_SERVICE_TYPE_SILENT: u32 = 16u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_SIGN_CANCEL: u32 = 4u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_SIGN_OFFLINE: u32 = 2u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_SIGN_ONLINE: u32 = 1u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub type DRM_STATUS_MSG = i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_MSG_ACTIVATE_MACHINE: DRM_STATUS_MSG = 0i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_MSG_ACTIVATE_GROUPIDENTITY: DRM_STATUS_MSG = 1i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_MSG_ACQUIRE_LICENSE: DRM_STATUS_MSG = 2i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_MSG_ACQUIRE_ADVISORY: DRM_STATUS_MSG = 3i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_MSG_SIGN_ISSUANCE_LICENSE: DRM_STATUS_MSG = 4i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_MSG_ACQUIRE_CLIENTLICENSOR: DRM_STATUS_MSG = 5i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_MSG_ACQUIRE_ISSUANCE_LICENSE_TEMPLATE: DRM_STATUS_MSG = 6i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub type DRM_USAGEPOLICY_TYPE = i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_USAGEPOLICY_TYPE_BYNAME: DRM_USAGEPOLICY_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_USAGEPOLICY_TYPE_BYPUBLICKEY: DRM_USAGEPOLICY_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_USAGEPOLICY_TYPE_BYDIGEST: DRM_USAGEPOLICY_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const DRM_USAGEPOLICY_TYPE_OSEXCLUSION: DRM_USAGEPOLICY_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const MSDRM_CLIENT_ZONE: u32 = 52992u32;
#[doc = "*Required features: 'Win32_Data_RightsManagement'*"]
pub const MSDRM_POLICY_ZONE: u32 = 37632u32;
