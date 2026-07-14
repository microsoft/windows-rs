#[inline]
pub unsafe fn SLAcquireGenuineTicket<P2, P3, P4>(ppticketblob: *mut *mut core::ffi::c_void, pcbticketblob: *mut u32, pwsztemplateid: P2, pwszserverurl: P3, pwszclienttoken: P4) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slcext.dll" "system" fn SLAcquireGenuineTicket(ppticketblob : *mut *mut core::ffi::c_void, pcbticketblob : *mut u32, pwsztemplateid : windows_core::PCWSTR, pwszserverurl : windows_core::PCWSTR, pwszclienttoken : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SLAcquireGenuineTicket(ppticketblob as _, pcbticketblob as _, pwsztemplateid.param().abi(), pwszserverurl.param().abi(), pwszclienttoken.param().abi()) }
}
#[inline]
pub unsafe fn SLActivateProduct<P5>(hslc: HSLC, pproductskuid: *const SLID, cbappspecificdata: Option<u32>, pvappspecificdata: Option<*const core::ffi::c_void>, pactivationinfo: Option<*const SL_ACTIVATION_INFO_HEADER>, pwszproxyserver: P5, wproxyport: Option<u16>) -> windows_core::HRESULT
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slcext.dll" "system" fn SLActivateProduct(hslc : HSLC, pproductskuid : *const SLID, cbappspecificdata : u32, pvappspecificdata : *const core::ffi::c_void, pactivationinfo : *const SL_ACTIVATION_INFO_HEADER, pwszproxyserver : windows_core::PCWSTR, wproxyport : u16) -> windows_core::HRESULT);
    unsafe { SLActivateProduct(hslc, pproductskuid, cbappspecificdata.unwrap_or(core::mem::zeroed()) as _, pvappspecificdata.unwrap_or(core::mem::zeroed()) as _, pactivationinfo.unwrap_or(core::mem::zeroed()) as _, pwszproxyserver.param().abi(), wproxyport.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SLClose(hslc: HSLC) -> windows_core::HRESULT {
    windows_core::link!("slc.dll" "system" fn SLClose(hslc : HSLC) -> windows_core::HRESULT);
    unsafe { SLClose(hslc) }
}
#[inline]
pub unsafe fn SLConsumeRight<P3>(hslc: HSLC, pappid: *const SLID, pproductskuid: Option<*const SLID>, pwszrightname: P3, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLConsumeRight(hslc : HSLC, pappid : *const SLID, pproductskuid : *const SLID, pwszrightname : windows_core::PCWSTR, pvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SLConsumeRight(hslc, pappid, pproductskuid.unwrap_or(core::mem::zeroed()) as _, pwszrightname.param().abi(), pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SLDepositOfflineConfirmationId<P2, P3>(hslc: HSLC, pproductskuid: *const SLID, pwszinstallationid: P2, pwszconfirmationid: P3) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLDepositOfflineConfirmationId(hslc : HSLC, pproductskuid : *const SLID, pwszinstallationid : windows_core::PCWSTR, pwszconfirmationid : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SLDepositOfflineConfirmationId(hslc, pproductskuid, pwszinstallationid.param().abi(), pwszconfirmationid.param().abi()) }
}
#[inline]
pub unsafe fn SLDepositOfflineConfirmationIdEx<P3, P4>(hslc: HSLC, pproductskuid: Option<*const SLID>, pactivationinfo: Option<*const SL_ACTIVATION_INFO_HEADER>, pwszinstallationid: P3, pwszconfirmationid: P4) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLDepositOfflineConfirmationIdEx(hslc : HSLC, pproductskuid : *const SLID, pactivationinfo : *const SL_ACTIVATION_INFO_HEADER, pwszinstallationid : windows_core::PCWSTR, pwszconfirmationid : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { SLDepositOfflineConfirmationIdEx(hslc, pproductskuid.unwrap_or(core::mem::zeroed()) as _, pactivationinfo.unwrap_or(core::mem::zeroed()) as _, pwszinstallationid.param().abi(), pwszconfirmationid.param().abi()) }
}
#[inline]
pub unsafe fn SLFireEvent<P1>(hslc: HSLC, pwszeventid: P1, papplicationid: *const SLID) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLFireEvent(hslc : HSLC, pwszeventid : windows_core::PCWSTR, papplicationid : *const SLID) -> windows_core::HRESULT);
    unsafe { SLFireEvent(hslc, pwszeventid.param().abi(), papplicationid) }
}
#[inline]
pub unsafe fn SLGenerateOfflineInstallationId(hslc: HSLC, pproductskuid: *const SLID) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("slc.dll" "system" fn SLGenerateOfflineInstallationId(hslc : HSLC, pproductskuid : *const SLID, ppwszinstallationid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGenerateOfflineInstallationId(hslc, pproductskuid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLGenerateOfflineInstallationIdEx(hslc: HSLC, pproductskuid: Option<*const SLID>, pactivationinfo: Option<*const SL_ACTIVATION_INFO_HEADER>) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("slc.dll" "system" fn SLGenerateOfflineInstallationIdEx(hslc : HSLC, pproductskuid : *const SLID, pactivationinfo : *const SL_ACTIVATION_INFO_HEADER, ppwszinstallationid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGenerateOfflineInstallationIdEx(hslc, pproductskuid.unwrap_or(core::mem::zeroed()) as _, pactivationinfo.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SLGetApplicationInformation<P2>(hslc: HSLC, papplicationid: *const SLID, pwszvaluename: P2, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut super::minwindef::PBYTE) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLGetApplicationInformation(hslc : HSLC, papplicationid : *const SLID, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_core::HRESULT);
    unsafe { SLGetApplicationInformation(hslc, papplicationid, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _) }
}
#[inline]
pub unsafe fn SLGetGenuineInformation<P1>(pqueryid: *const SLID, pwszvaluename: P1, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut *mut u8) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLGetGenuineInformation(pqueryid : *const SLID, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { SLGetGenuineInformation(pqueryid, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _) }
}
#[inline]
pub unsafe fn SLGetInstalledProductKeyIds(hslc: HSLC, pproductskuid: *const SLID, pnproductkeyids: *mut u32, ppproductkeyids: *mut *mut SLID) -> windows_core::HRESULT {
    windows_core::link!("slc.dll" "system" fn SLGetInstalledProductKeyIds(hslc : HSLC, pproductskuid : *const SLID, pnproductkeyids : *mut u32, ppproductkeyids : *mut *mut SLID) -> windows_core::HRESULT);
    unsafe { SLGetInstalledProductKeyIds(hslc, pproductskuid, pnproductkeyids as _, ppproductkeyids as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SLGetLicense(hslc: HSLC, plicensefileid: *const SLID, pcblicensefile: *mut u32, ppblicensefile: *mut super::minwindef::PBYTE) -> windows_core::HRESULT {
    windows_core::link!("slc.dll" "system" fn SLGetLicense(hslc : HSLC, plicensefileid : *const SLID, pcblicensefile : *mut u32, ppblicensefile : *mut super::minwindef::PBYTE) -> windows_core::HRESULT);
    unsafe { SLGetLicense(hslc, plicensefileid, pcblicensefile as _, ppblicensefile as _) }
}
#[inline]
pub unsafe fn SLGetLicenseFileId(hslc: HSLC, pblicenseblob: &[u8]) -> windows_core::Result<SLID> {
    windows_core::link!("slc.dll" "system" fn SLGetLicenseFileId(hslc : HSLC, cblicenseblob : u32, pblicenseblob : *const u8, plicensefileid : *mut SLID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGetLicenseFileId(hslc, pblicenseblob.len().try_into().unwrap(), pblicenseblob.as_ptr(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SLGetLicenseInformation<P2>(hslc: HSLC, psllicenseid: *const SLID, pwszvaluename: P2, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut super::minwindef::PBYTE) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLGetLicenseInformation(hslc : HSLC, psllicenseid : *const SLID, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_core::HRESULT);
    unsafe { SLGetLicenseInformation(hslc, psllicenseid, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _) }
}
#[inline]
pub unsafe fn SLGetLicensingStatusInformation<P3>(hslc: HSLC, pappid: Option<*const SLID>, pproductskuid: Option<*const SLID>, pwszrightname: P3, pnstatuscount: *mut u32, pplicensingstatus: *mut *mut SL_LICENSING_STATUS) -> windows_core::HRESULT
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLGetLicensingStatusInformation(hslc : HSLC, pappid : *const SLID, pproductskuid : *const SLID, pwszrightname : windows_core::PCWSTR, pnstatuscount : *mut u32, pplicensingstatus : *mut *mut SL_LICENSING_STATUS) -> windows_core::HRESULT);
    unsafe { SLGetLicensingStatusInformation(hslc, pappid.unwrap_or(core::mem::zeroed()) as _, pproductskuid.unwrap_or(core::mem::zeroed()) as _, pwszrightname.param().abi(), pnstatuscount as _, pplicensingstatus as _) }
}
#[inline]
pub unsafe fn SLGetPKeyId<P1, P2>(hslc: HSLC, pwszpkeyalgorithm: P1, pwszpkeystring: P2, pbpkeyspecificdata: Option<&[u8]>) -> windows_core::Result<SLID>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLGetPKeyId(hslc : HSLC, pwszpkeyalgorithm : windows_core::PCWSTR, pwszpkeystring : windows_core::PCWSTR, cbpkeyspecificdata : u32, pbpkeyspecificdata : *const u8, ppkeyid : *mut SLID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGetPKeyId(hslc, pwszpkeyalgorithm.param().abi(), pwszpkeystring.param().abi(), pbpkeyspecificdata.map_or(0, |slice| slice.len().try_into().unwrap()), pbpkeyspecificdata.map_or(core::ptr::null(), |slice| slice.as_ptr()), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SLGetPKeyInformation<P2>(hslc: HSLC, ppkeyid: *const SLID, pwszvaluename: P2, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut super::minwindef::PBYTE) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLGetPKeyInformation(hslc : HSLC, ppkeyid : *const SLID, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_core::HRESULT);
    unsafe { SLGetPKeyInformation(hslc, ppkeyid, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SLGetPolicyInformation<P1>(hslc: HSLC, pwszvaluename: P1, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut super::minwindef::PBYTE) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLGetPolicyInformation(hslc : HSLC, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_core::HRESULT);
    unsafe { SLGetPolicyInformation(hslc, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _) }
}
#[inline]
pub unsafe fn SLGetPolicyInformationDWORD<P1>(hslc: HSLC, pwszvaluename: P1) -> windows_core::Result<u32>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLGetPolicyInformationDWORD(hslc : HSLC, pwszvaluename : windows_core::PCWSTR, pdwvalue : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGetPolicyInformationDWORD(hslc, pwszvaluename.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SLGetProductSkuInformation<P2>(hslc: HSLC, pproductskuid: *const SLID, pwszvaluename: P2, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut super::minwindef::PBYTE) -> windows_core::HRESULT
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLGetProductSkuInformation(hslc : HSLC, pproductskuid : *const SLID, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_core::HRESULT);
    unsafe { SLGetProductSkuInformation(hslc, pproductskuid, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _) }
}
#[inline]
pub unsafe fn SLGetReferralInformation<P3>(hslc: HSLC, ereferraltype: SLREFERRALTYPE, pskuorappid: *const SLID, pwszvaluename: P3) -> windows_core::Result<windows_core::PWSTR>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slcext.dll" "system" fn SLGetReferralInformation(hslc : HSLC, ereferraltype : SLREFERRALTYPE, pskuorappid : *const SLID, pwszvaluename : windows_core::PCWSTR, ppwszvalue : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGetReferralInformation(hslc, ereferraltype, pskuorappid, pwszvaluename.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLGetSLIDList(hslc: HSLC, equeryidtype: SLIDTYPE, pqueryid: Option<*const SLID>, ereturnidtype: SLIDTYPE, pnreturnids: *mut u32, ppreturnids: *mut *mut SLID) -> windows_core::HRESULT {
    windows_core::link!("slc.dll" "system" fn SLGetSLIDList(hslc : HSLC, equeryidtype : SLIDTYPE, pqueryid : *const SLID, ereturnidtype : SLIDTYPE, pnreturnids : *mut u32, ppreturnids : *mut *mut SLID) -> windows_core::HRESULT);
    unsafe { SLGetSLIDList(hslc, equeryidtype, pqueryid.unwrap_or(core::mem::zeroed()) as _, ereturnidtype, pnreturnids as _, ppreturnids as _) }
}
#[inline]
pub unsafe fn SLGetServerStatus<P0, P1, P2>(pwszserverurl: P0, pwszacquisitiontype: P1, pwszproxyserver: P2, wproxyport: Option<u16>) -> windows_core::Result<windows_core::HRESULT>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slcext.dll" "system" fn SLGetServerStatus(pwszserverurl : windows_core::PCWSTR, pwszacquisitiontype : windows_core::PCWSTR, pwszproxyserver : windows_core::PCWSTR, wproxyport : u16, phrstatus : *mut windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGetServerStatus(pwszserverurl.param().abi(), pwszacquisitiontype.param().abi(), pwszproxyserver.param().abi(), wproxyport.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SLGetServiceInformation<P1>(hslc: HSLC, pwszvaluename: P1, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut super::minwindef::PBYTE) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLGetServiceInformation(hslc : HSLC, pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_core::HRESULT);
    unsafe { SLGetServiceInformation(hslc, pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SLGetWindowsInformation<P0>(pwszvaluename: P0, pedatatype: Option<*mut SLDATATYPE>, pcbvalue: *mut u32, ppbvalue: *mut super::minwindef::PBYTE) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLGetWindowsInformation(pwszvaluename : windows_core::PCWSTR, pedatatype : *mut SLDATATYPE, pcbvalue : *mut u32, ppbvalue : *mut super::minwindef::PBYTE) -> windows_core::HRESULT);
    unsafe { SLGetWindowsInformation(pwszvaluename.param().abi(), pedatatype.unwrap_or(core::mem::zeroed()) as _, pcbvalue as _, ppbvalue as _) }
}
#[inline]
pub unsafe fn SLGetWindowsInformationDWORD<P0>(pwszvaluename: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLGetWindowsInformationDWORD(pwszvaluename : windows_core::PCWSTR, pdwvalue : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLGetWindowsInformationDWORD(pwszvaluename.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLInstallLicense(hslc: HSLC, pblicenseblob: &[u8]) -> windows_core::Result<SLID> {
    windows_core::link!("slc.dll" "system" fn SLInstallLicense(hslc : HSLC, cblicenseblob : u32, pblicenseblob : *const u8, plicensefileid : *mut SLID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLInstallLicense(hslc, pblicenseblob.len().try_into().unwrap(), pblicenseblob.as_ptr(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLInstallProofOfPurchase<P1, P2>(hslc: HSLC, pwszpkeyalgorithm: P1, pwszpkeystring: P2, pbpkeyspecificdata: Option<&[u8]>) -> windows_core::Result<SLID>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLInstallProofOfPurchase(hslc : HSLC, pwszpkeyalgorithm : windows_core::PCWSTR, pwszpkeystring : windows_core::PCWSTR, cbpkeyspecificdata : u32, pbpkeyspecificdata : *const u8, ppkeyid : *mut SLID) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLInstallProofOfPurchase(hslc, pwszpkeyalgorithm.param().abi(), pwszpkeystring.param().abi(), pbpkeyspecificdata.map_or(0, |slice| slice.len().try_into().unwrap()), pbpkeyspecificdata.map_or(core::ptr::null(), |slice| slice.as_ptr()), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLIsGenuineLocal(pappid: *const SLID, pgenuinestate: *mut SL_GENUINE_STATE, puioptions: Option<*mut SL_NONGENUINE_UI_OPTIONS>) -> windows_core::HRESULT {
    windows_core::link!("slwga.dll" "system" fn SLIsGenuineLocal(pappid : *const SLID, pgenuinestate : *mut SL_GENUINE_STATE, puioptions : *mut SL_NONGENUINE_UI_OPTIONS) -> windows_core::HRESULT);
    unsafe { SLIsGenuineLocal(pappid, pgenuinestate as _, puioptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn SLOpen() -> windows_core::Result<HSLC> {
    windows_core::link!("slc.dll" "system" fn SLOpen(phslc : *mut HSLC) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        SLOpen(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn SLQueryLicenseValueFromApp<P0>(valuename: P0, valuetype: Option<*mut u32>, databuffer: Option<*mut core::ffi::c_void>, datasize: u32, resultdatasize: *mut u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-slapi-l1-1-0.dll" "system" fn SLQueryLicenseValueFromApp(valuename : windows_core::PCWSTR, valuetype : *mut u32, databuffer : *mut core::ffi::c_void, datasize : u32, resultdatasize : *mut u32) -> windows_core::HRESULT);
    unsafe { SLQueryLicenseValueFromApp(valuename.param().abi(), valuetype.unwrap_or(core::mem::zeroed()) as _, databuffer.unwrap_or(core::mem::zeroed()) as _, datasize, resultdatasize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SLRegisterEvent<P1>(hslc: Option<HSLC>, pwszeventid: P1, papplicationid: *const SLID, hevent: super::winnt::HANDLE) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLRegisterEvent(hslc : HSLC, pwszeventid : windows_core::PCWSTR, papplicationid : *const SLID, hevent : super::winnt::HANDLE) -> windows_core::HRESULT);
    unsafe { SLRegisterEvent(hslc.unwrap_or(core::mem::zeroed()) as _, pwszeventid.param().abi(), papplicationid, hevent) }
}
#[inline]
pub unsafe fn SLSetCurrentProductKey(hslc: HSLC, pproductskuid: *const SLID, pproductkeyid: *const SLID) -> windows_core::HRESULT {
    windows_core::link!("slc.dll" "system" fn SLSetCurrentProductKey(hslc : HSLC, pproductskuid : *const SLID, pproductkeyid : *const SLID) -> windows_core::HRESULT);
    unsafe { SLSetCurrentProductKey(hslc, pproductskuid, pproductkeyid) }
}
#[inline]
pub unsafe fn SLSetGenuineInformation<P1>(pqueryid: *const SLID, pwszvaluename: P1, edatatype: SLDATATYPE, pbvalue: Option<&[u8]>) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLSetGenuineInformation(pqueryid : *const SLID, pwszvaluename : windows_core::PCWSTR, edatatype : SLDATATYPE, cbvalue : u32, pbvalue : *const u8) -> windows_core::HRESULT);
    unsafe { SLSetGenuineInformation(pqueryid, pwszvaluename.param().abi(), edatatype, pbvalue.map_or(0, |slice| slice.len().try_into().unwrap()), pbvalue.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
}
#[inline]
pub unsafe fn SLUninstallLicense(hslc: HSLC, plicensefileid: *const SLID) -> windows_core::HRESULT {
    windows_core::link!("slc.dll" "system" fn SLUninstallLicense(hslc : HSLC, plicensefileid : *const SLID) -> windows_core::HRESULT);
    unsafe { SLUninstallLicense(hslc, plicensefileid) }
}
#[inline]
pub unsafe fn SLUninstallProofOfPurchase(hslc: HSLC, ppkeyid: *const SLID) -> windows_core::HRESULT {
    windows_core::link!("slc.dll" "system" fn SLUninstallProofOfPurchase(hslc : HSLC, ppkeyid : *const SLID) -> windows_core::HRESULT);
    unsafe { SLUninstallProofOfPurchase(hslc, ppkeyid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SLUnregisterEvent<P1>(hslc: Option<HSLC>, pwszeventid: P1, papplicationid: *const SLID, hevent: super::winnt::HANDLE) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("slc.dll" "system" fn SLUnregisterEvent(hslc : HSLC, pwszeventid : windows_core::PCWSTR, papplicationid : *const SLID, hevent : super::winnt::HANDLE) -> windows_core::HRESULT);
    unsafe { SLUnregisterEvent(hslc.unwrap_or(core::mem::zeroed()) as _, pwszeventid.param().abi(), papplicationid, hevent) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSLC(pub *mut core::ffi::c_void);
impl Default for HSLC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HSLP(pub *mut core::ffi::c_void);
impl Default for HSLP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ID_CAP_SLAPI: windows_core::PCWSTR = windows_core::w!("slapiQueryLicenseValue");
pub type PSL_SYSTEM_POLICY_INFORMATION = *mut SL_SYSTEM_POLICY_INFORMATION;
pub type SLDATATYPE = i32;
pub type SLID = windows_core::GUID;
pub type SLIDTYPE = i32;
pub type SLLICENSINGSTATUS = i32;
pub type SLREFERRALTYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SL_ACTIVATION_INFO_HEADER {
    pub cbSize: u32,
    pub r#type: SL_ACTIVATION_TYPE,
}
pub type SL_ACTIVATION_TYPE = i32;
pub const SL_ACTIVATION_TYPE_ACTIVE_DIRECTORY: SL_ACTIVATION_TYPE = 1;
pub const SL_ACTIVATION_TYPE_DEFAULT: SL_ACTIVATION_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SL_AD_ACTIVATION_INFO {
    pub header: SL_ACTIVATION_INFO_HEADER,
    pub pwszProductKey: windows_core::PCWSTR,
    pub pwszActivationObjectName: windows_core::PCWSTR,
}
pub const SL_DATA_BINARY: SLDATATYPE = 3;
pub const SL_DATA_DWORD: SLDATATYPE = 4;
pub const SL_DATA_MULTI_SZ: SLDATATYPE = 7;
pub const SL_DATA_NONE: SLDATATYPE = 0;
pub const SL_DATA_SUM: SLDATATYPE = 100;
pub const SL_DATA_SZ: SLDATATYPE = 1;
pub const SL_DEFAULT_MIGRATION_ENCRYPTOR_URI: windows_core::PCWSTR = windows_core::w!("msft:spp/migrationencryptor/tokenact/1.0");
pub const SL_EVENT_LICENSING_STATE_CHANGED: windows_core::PCWSTR = windows_core::w!("msft:rm/event/licensingstatechanged");
pub const SL_EVENT_POLICY_CHANGED: windows_core::PCWSTR = windows_core::w!("msft:rm/event/policychanged");
pub const SL_EVENT_USER_NOTIFICATION: windows_core::PCWSTR = windows_core::w!("msft:rm/event/usernotification");
pub type SL_GENUINE_STATE = i32;
pub const SL_GEN_STATE_INVALID_LICENSE: SL_GENUINE_STATE = 1;
pub const SL_GEN_STATE_IS_GENUINE: SL_GENUINE_STATE = 0;
pub const SL_GEN_STATE_LAST: SL_GENUINE_STATE = 4;
pub const SL_GEN_STATE_OFFLINE: SL_GENUINE_STATE = 3;
pub const SL_GEN_STATE_TAMPERED: SL_GENUINE_STATE = 2;
pub const SL_ID_ALL_LICENSES: SLIDTYPE = 5;
pub const SL_ID_ALL_LICENSE_FILES: SLIDTYPE = 6;
pub const SL_ID_APPLICATION: SLIDTYPE = 0;
pub const SL_ID_LAST: SLIDTYPE = 8;
pub const SL_ID_LICENSE: SLIDTYPE = 3;
pub const SL_ID_LICENSE_FILE: SLIDTYPE = 2;
pub const SL_ID_PKEY: SLIDTYPE = 4;
pub const SL_ID_PRODUCT_SKU: SLIDTYPE = 1;
pub const SL_ID_STORE_TOKEN: SLIDTYPE = 7;
pub const SL_INFO_KEY_ACTIVE_PLUGINS: windows_core::PCWSTR = windows_core::w!("ActivePlugins");
pub const SL_INFO_KEY_AUTHOR: windows_core::PCWSTR = windows_core::w!("Author");
pub const SL_INFO_KEY_BIOS_OA2_MINOR_VERSION: windows_core::PCWSTR = windows_core::w!("BiosOA2MinorVersion");
pub const SL_INFO_KEY_BIOS_PKEY: windows_core::PCWSTR = windows_core::w!("BiosProductKey");
pub const SL_INFO_KEY_BIOS_PKEY_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("BiosProductKeyDescription");
pub const SL_INFO_KEY_BIOS_PKEY_PKPN: windows_core::PCWSTR = windows_core::w!("BiosProductKeyPkPn");
pub const SL_INFO_KEY_BIOS_SLIC_STATE: windows_core::PCWSTR = windows_core::w!("BiosSlicState");
pub const SL_INFO_KEY_CHANNEL: windows_core::PCWSTR = windows_core::w!("Channel");
pub const SL_INFO_KEY_DESCRIPTION: windows_core::PCWSTR = windows_core::w!("Description");
pub const SL_INFO_KEY_DIGITAL_PID: windows_core::PCWSTR = windows_core::w!("DigitalPID");
pub const SL_INFO_KEY_DIGITAL_PID2: windows_core::PCWSTR = windows_core::w!("DigitalPID2");
pub const SL_INFO_KEY_IS_KMS: windows_core::PCWSTR = windows_core::w!("IsKeyManagementService");
pub const SL_INFO_KEY_IS_PRS: windows_core::PCWSTR = windows_core::w!("IsPRS");
pub const SL_INFO_KEY_KMS_CURRENT_COUNT: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceCurrentCount");
pub const SL_INFO_KEY_KMS_FAILED_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceFailedRequests");
pub const SL_INFO_KEY_KMS_LICENSED_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceLicensedRequests");
pub const SL_INFO_KEY_KMS_NON_GENUINE_GRACE_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceNonGenuineGraceRequests");
pub const SL_INFO_KEY_KMS_NOTIFICATION_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceNotificationRequests");
pub const SL_INFO_KEY_KMS_OOB_GRACE_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceOOBGraceRequests");
pub const SL_INFO_KEY_KMS_OOT_GRACE_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceOOTGraceRequests");
pub const SL_INFO_KEY_KMS_REQUIRED_CLIENT_COUNT: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceRequiredClientCount");
pub const SL_INFO_KEY_KMS_TOTAL_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceTotalRequests");
pub const SL_INFO_KEY_KMS_UNLICENSED_REQUESTS: windows_core::PCWSTR = windows_core::w!("KeyManagementServiceUnlicensedRequests");
pub const SL_INFO_KEY_LICENSE_TYPE: windows_core::PCWSTR = windows_core::w!("LicenseType");
pub const SL_INFO_KEY_LICENSOR_URL: windows_core::PCWSTR = windows_core::w!("LicensorUrl");
pub const SL_INFO_KEY_NAME: windows_core::PCWSTR = windows_core::w!("Name");
pub const SL_INFO_KEY_PARTIAL_PRODUCT_KEY: windows_core::PCWSTR = windows_core::w!("PartialProductKey");
pub const SL_INFO_KEY_PRODUCT_KEY_ACTIVATION_URL: windows_core::PCWSTR = windows_core::w!("PKCURL");
pub const SL_INFO_KEY_PRODUCT_SKU_ID: windows_core::PCWSTR = windows_core::w!("ProductSkuId");
pub const SL_INFO_KEY_RIGHT_ACCOUNT_ACTIVATION_URL: windows_core::PCWSTR = windows_core::w!("RACURL");
pub const SL_INFO_KEY_SECURE_PROCESSOR_ACTIVATION_URL: windows_core::PCWSTR = windows_core::w!("SPCURL");
pub const SL_INFO_KEY_SECURE_STORE_ID: windows_core::PCWSTR = windows_core::w!("SecureStoreId");
pub const SL_INFO_KEY_SYSTEM_STATE: windows_core::PCWSTR = windows_core::w!("SystemState");
pub const SL_INFO_KEY_USE_LICENSE_ACTIVATION_URL: windows_core::PCWSTR = windows_core::w!("EULURL");
pub const SL_INFO_KEY_VERSION: windows_core::PCWSTR = windows_core::w!("Version");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SL_LICENSING_STATUS {
    pub SkuId: SLID,
    pub eStatus: SLLICENSINGSTATUS,
    pub dwGraceTime: u32,
    pub dwTotalGraceDays: u32,
    pub hrReason: windows_core::HRESULT,
    pub qwValidityExpiration: u64,
}
pub const SL_LICENSING_STATUS_IN_GRACE_PERIOD: SLLICENSINGSTATUS = 2;
pub const SL_LICENSING_STATUS_LAST: SLLICENSINGSTATUS = 4;
pub const SL_LICENSING_STATUS_LICENSED: SLLICENSINGSTATUS = 1;
pub const SL_LICENSING_STATUS_NOTIFICATION: SLLICENSINGSTATUS = 3;
pub const SL_LICENSING_STATUS_UNLICENSED: SLLICENSINGSTATUS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SL_NONGENUINE_UI_OPTIONS {
    pub cbSize: u32,
    pub pComponentId: *const SLID,
    pub hResultUI: windows_core::HRESULT,
}
impl Default for SL_NONGENUINE_UI_OPTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SL_PKEY_DETECT: windows_core::PCWSTR = windows_core::w!("msft:rm/algorithm/pkey/detect");
pub const SL_PKEY_MS2005: windows_core::PCWSTR = windows_core::w!("msft:rm/algorithm/pkey/2005");
pub const SL_PKEY_MS2009: windows_core::PCWSTR = windows_core::w!("msft:rm/algorithm/pkey/2009");
pub const SL_POLICY_EVALUATION_MODE_ENABLED: windows_core::PCWSTR = windows_core::w!("Security-SPP-EvaluationModeEnabled");
pub const SL_PROP_ACTIVATION_VALIDATION_IN_PROGRESS: windows_core::PCWSTR = windows_core::w!("SL_ACTIVATION_VALIDATION_IN_PROGRESS");
pub const SL_PROP_BRT_COMMIT: windows_core::PCWSTR = windows_core::w!("SL_BRT_COMMIT");
pub const SL_PROP_BRT_DATA: windows_core::PCWSTR = windows_core::w!("SL_BRT_DATA");
pub const SL_PROP_GENUINE_RESULT: windows_core::PCWSTR = windows_core::w!("SL_GENUINE_RESULT");
pub const SL_PROP_GET_GENUINE_AUTHZ: windows_core::PCWSTR = windows_core::w!("SL_GET_GENUINE_AUTHZ");
pub const SL_PROP_GET_GENUINE_SERVER_AUTHZ: windows_core::PCWSTR = windows_core::w!("SL_GET_GENUINE_SERVER_AUTHZ");
pub const SL_PROP_LAST_ACT_ATTEMPT_HRESULT: windows_core::PCWSTR = windows_core::w!("SL_LAST_ACT_ATTEMPT_HRESULT");
pub const SL_PROP_LAST_ACT_ATTEMPT_SERVER_FLAGS: windows_core::PCWSTR = windows_core::w!("SL_LAST_ACT_ATTEMPT_SERVER_FLAGS");
pub const SL_PROP_LAST_ACT_ATTEMPT_TIME: windows_core::PCWSTR = windows_core::w!("SL_LAST_ACT_ATTEMPT_TIME");
pub const SL_PROP_NONGENUINE_GRACE_FLAG: windows_core::PCWSTR = windows_core::w!("SL_NONGENUINE_GRACE_FLAG");
pub const SL_REARM_REBOOT_REQUIRED: u32 = 1;
pub const SL_REFERRALTYPE_APPID: SLREFERRALTYPE = 1;
pub const SL_REFERRALTYPE_BEST_MATCH: SLREFERRALTYPE = 4;
pub const SL_REFERRALTYPE_OVERRIDE_APPID: SLREFERRALTYPE = 3;
pub const SL_REFERRALTYPE_OVERRIDE_SKUID: SLREFERRALTYPE = 2;
pub const SL_REFERRALTYPE_SKUID: SLREFERRALTYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SL_SYSTEM_POLICY_INFORMATION {
    pub Reserved1: [*mut core::ffi::c_void; 2],
    pub Reserved2: [u32; 3],
}
impl Default for SL_SYSTEM_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SL_SYSTEM_STATE_REBOOT_POLICY_FOUND: u32 = 1;
pub const SL_SYSTEM_STATE_TAMPERED: u32 = 2;
pub const SPP_MIGRATION_GATHER_ACTIVATED_WINDOWS_STATE: u32 = 2;
pub const SPP_MIGRATION_GATHER_ALL: u32 = 4294967295;
pub const SPP_MIGRATION_GATHER_MIGRATABLE_APPS: u32 = 1;
