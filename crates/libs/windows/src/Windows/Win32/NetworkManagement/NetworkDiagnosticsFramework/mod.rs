#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[inline]
pub unsafe fn NdfCancelIncident(handle: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfCancelIncident ( handle : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    NdfCancelIncident(handle).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[inline]
pub unsafe fn NdfCloseIncident(handle: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfCloseIncident ( handle : *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    NdfCloseIncident(handle).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[inline]
pub unsafe fn NdfCreateConnectivityIncident(handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfCreateConnectivityIncident ( handle : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    NdfCreateConnectivityIncident(handle).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[inline]
pub unsafe fn NdfCreateDNSIncident<P0>(hostname: P0, querytype: u16, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfCreateDNSIncident ( hostname : :: windows::core::PCWSTR , querytype : u16 , handle : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    NdfCreateDNSIncident(hostname.into_param().abi(), querytype, handle).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn NdfCreateGroupingIncident<P0, P1, P2, P3, P4>(cloudname: P0, groupname: P1, identity: P2, invitation: P3, addresses: ::core::option::Option<*const super::super::Networking::WinSock::SOCKET_ADDRESS_LIST>, appid: P4, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P4: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfCreateGroupingIncident ( cloudname : :: windows::core::PCWSTR , groupname : :: windows::core::PCWSTR , identity : :: windows::core::PCWSTR , invitation : :: windows::core::PCWSTR , addresses : *const super::super::Networking::WinSock:: SOCKET_ADDRESS_LIST , appid : :: windows::core::PCWSTR , handle : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    NdfCreateGroupingIncident(cloudname.into_param().abi(), groupname.into_param().abi(), identity.into_param().abi(), invitation.into_param().abi(), ::core::mem::transmute(addresses.unwrap_or(::std::ptr::null())), appid.into_param().abi(), handle).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfCreateIncident<P0>(helperclassname: P0, attributes: &[HELPER_ATTRIBUTE], handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfCreateIncident ( helperclassname : :: windows::core::PCWSTR , celt : u32 , attributes : *const HELPER_ATTRIBUTE , handle : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    NdfCreateIncident(helperclassname.into_param().abi(), attributes.len() as _, ::core::mem::transmute(attributes.as_ptr()), handle).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[inline]
pub unsafe fn NdfCreateNetConnectionIncident(handle: *mut *mut ::core::ffi::c_void, id: ::windows::core::GUID) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfCreateNetConnectionIncident ( handle : *mut *mut ::core::ffi::c_void , id : :: windows::core::GUID ) -> :: windows::core::HRESULT );
    NdfCreateNetConnectionIncident(handle, ::core::mem::transmute(id)).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfCreatePnrpIncident<P0, P1, P2, P3>(cloudname: P0, peername: P1, diagnosepublish: P2, appid: P3, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfCreatePnrpIncident ( cloudname : :: windows::core::PCWSTR , peername : :: windows::core::PCWSTR , diagnosepublish : super::super::Foundation:: BOOL , appid : :: windows::core::PCWSTR , handle : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    NdfCreatePnrpIncident(cloudname.into_param().abi(), peername.into_param().abi(), diagnosepublish.into_param().abi(), appid.into_param().abi(), handle).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[inline]
pub unsafe fn NdfCreateSharingIncident<P0>(uncpath: P0, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfCreateSharingIncident ( uncpath : :: windows::core::PCWSTR , handle : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    NdfCreateSharingIncident(uncpath.into_param().abi(), handle).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[inline]
pub unsafe fn NdfCreateWebIncident<P0>(url: P0, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfCreateWebIncident ( url : :: windows::core::PCWSTR , handle : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    NdfCreateWebIncident(url.into_param().abi(), handle).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfCreateWebIncidentEx<P0, P1, P2>(url: P0, usewinhttp: P1, modulename: P2, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfCreateWebIncidentEx ( url : :: windows::core::PCWSTR , usewinhttp : super::super::Foundation:: BOOL , modulename : :: windows::core::PCWSTR , handle : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    NdfCreateWebIncidentEx(url.into_param().abi(), usewinhttp.into_param().abi(), modulename.into_param().abi(), handle).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NdfCreateWinSockIncident<P0, P1, P2>(sock: P0, host: P1, port: u16, appid: P2, userid: ::core::option::Option<*const super::super::Security::SID>, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Networking::WinSock::SOCKET>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfCreateWinSockIncident ( sock : super::super::Networking::WinSock:: SOCKET , host : :: windows::core::PCWSTR , port : u16 , appid : :: windows::core::PCWSTR , userid : *const super::super::Security:: SID , handle : *mut *mut ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    NdfCreateWinSockIncident(sock.into_param().abi(), host.into_param().abi(), port, appid.into_param().abi(), ::core::mem::transmute(userid.unwrap_or(::std::ptr::null())), handle).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[inline]
pub unsafe fn NdfDiagnoseIncident(handle: *const ::core::ffi::c_void, rootcausecount: *mut u32, rootcauses: *mut *mut RootCauseInfo, dwwait: u32, dwflags: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfDiagnoseIncident ( handle : *const ::core::ffi::c_void , rootcausecount : *mut u32 , rootcauses : *mut *mut RootCauseInfo , dwwait : u32 , dwflags : u32 ) -> :: windows::core::HRESULT );
    NdfDiagnoseIncident(handle, rootcausecount, rootcauses, dwwait, dwflags).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfExecuteDiagnosis<P0>(handle: *const ::core::ffi::c_void, hwnd: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfExecuteDiagnosis ( handle : *const ::core::ffi::c_void , hwnd : super::super::Foundation:: HWND ) -> :: windows::core::HRESULT );
    NdfExecuteDiagnosis(handle, hwnd.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[inline]
pub unsafe fn NdfGetTraceFile(handle: *const ::core::ffi::c_void) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfGetTraceFile ( handle : *const ::core::ffi::c_void , tracefilelocation : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    NdfGetTraceFile(handle, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[inline]
pub unsafe fn NdfRepairIncident(handle: *const ::core::ffi::c_void, repairex: *const RepairInfoEx, dwwait: u32) -> ::windows::core::Result<()> {
    ::windows::imp::link ! ( "ndfapi.dll""system" fn NdfRepairIncident ( handle : *const ::core::ffi::c_void , repairex : *const RepairInfoEx , dwwait : u32 ) -> :: windows::core::HRESULT );
    NdfRepairIncident(handle, repairex, dwwait).ok()
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[repr(transparent)]
pub struct INetDiagExtensibleHelper(::windows::core::IUnknown);
impl INetDiagExtensibleHelper {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResolveAttributes(&self, rgkeyattributes: &[HELPER_ATTRIBUTE], pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResolveAttributes)(::windows::core::Interface::as_raw(self), rgkeyattributes.len() as _, ::core::mem::transmute(rgkeyattributes.as_ptr()), pcelt, prgmatchvalues).ok()
    }
}
::windows::imp::interface_hierarchy!(INetDiagExtensibleHelper, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetDiagExtensibleHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetDiagExtensibleHelper {}
impl ::core::fmt::Debug for INetDiagExtensibleHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetDiagExtensibleHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetDiagExtensibleHelper {
    type Vtable = INetDiagExtensibleHelper_Vtbl;
}
impl ::core::clone::Clone for INetDiagExtensibleHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetDiagExtensibleHelper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0b35748_ebf5_11d8_bbe9_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagExtensibleHelper_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ResolveAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ResolveAttributes: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[repr(transparent)]
pub struct INetDiagHelper(::windows::core::IUnknown);
impl INetDiagHelper {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize(&self, rgattributes: &[HELPER_ATTRIBUTE]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), rgattributes.len() as _, ::core::mem::transmute(rgattributes.as_ptr())).ok()
    }
    pub unsafe fn GetDiagnosticsInfo(&self) -> ::windows::core::Result<*mut DiagnosticsInfo> {
        let mut result__ = ::windows::core::zeroed::<*mut DiagnosticsInfo>();
        (::windows::core::Interface::vtable(self).GetDiagnosticsInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetKeyAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetKeyAttributes)(::windows::core::Interface::as_raw(self), pcelt, pprgattributes).ok()
    }
    pub unsafe fn LowHealth<P0>(&self, pwszinstancedescription: P0, ppwszdescription: *mut ::windows::core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).LowHealth)(::windows::core::Interface::as_raw(self), pwszinstancedescription.into_param().abi(), ppwszdescription, pdeferredtime, pstatus).ok()
    }
    pub unsafe fn HighUtilization<P0>(&self, pwszinstancedescription: P0, ppwszdescription: *mut ::windows::core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).HighUtilization)(::windows::core::Interface::as_raw(self), pwszinstancedescription.into_param().abi(), ppwszdescription, pdeferredtime, pstatus).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLowerHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLowerHypotheses)(::windows::core::Interface::as_raw(self), pcelt, pprghypotheses).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDownStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDownStreamHypotheses)(::windows::core::Interface::as_raw(self), pcelt, pprghypotheses).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHigherHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHigherHypotheses)(::windows::core::Interface::as_raw(self), pcelt, pprghypotheses).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUpStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUpStreamHypotheses)(::windows::core::Interface::as_raw(self), pcelt, pprghypotheses).ok()
    }
    pub unsafe fn Repair(&self, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Repair)(::windows::core::Interface::as_raw(self), pinfo, pdeferredtime, pstatus).ok()
    }
    pub unsafe fn Validate(&self, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Validate)(::windows::core::Interface::as_raw(self), problem, pdeferredtime, pstatus).ok()
    }
    pub unsafe fn GetRepairInfo(&self, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRepairInfo)(::windows::core::Interface::as_raw(self), problem, pcelt, ppinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLifeTime(&self) -> ::windows::core::Result<LIFE_TIME> {
        let mut result__ = ::windows::core::zeroed::<LIFE_TIME>();
        (::windows::core::Interface::vtable(self).GetLifeTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLifeTime(&self, lifetime: LIFE_TIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLifeTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(lifetime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCacheTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::FILETIME>();
        (::windows::core::Interface::vtable(self).GetCacheTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributes)(::windows::core::Interface::as_raw(self), pcelt, pprgattributes).ok()
    }
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cancel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Cleanup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Cleanup)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(INetDiagHelper, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetDiagHelper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetDiagHelper {}
impl ::core::fmt::Debug for INetDiagHelper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetDiagHelper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetDiagHelper {
    type Vtable = INetDiagHelper_Vtbl;
}
impl ::core::clone::Clone for INetDiagHelper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetDiagHelper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0b35746_ebf5_11d8_bbe9_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelper_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    pub GetDiagnosticsInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinfo: *mut *mut DiagnosticsInfo) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetKeyAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetKeyAttributes: usize,
    pub LowHealth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinstancedescription: ::windows::core::PCWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT,
    pub HighUtilization: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwszinstancedescription: ::windows::core::PCWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLowerHypotheses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLowerHypotheses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDownStreamHypotheses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDownStreamHypotheses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHigherHypotheses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHigherHypotheses: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUpStreamHypotheses: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUpStreamHypotheses: usize,
    pub Repair: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::HRESULT,
    pub Validate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::HRESULT,
    pub GetRepairInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLifeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plifetime: *mut LIFE_TIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLifeTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLifeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lifetime: LIFE_TIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLifeTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCacheTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcachetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCacheTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributes: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Cleanup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[repr(transparent)]
pub struct INetDiagHelperEx(::windows::core::IUnknown);
impl INetDiagHelperEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReconfirmLowHealth(&self, presults: &[HypothesisResult], ppwszupdateddescription: *mut ::windows::core::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReconfirmLowHealth)(::windows::core::Interface::as_raw(self), presults.len() as _, ::core::mem::transmute(presults.as_ptr()), ppwszupdateddescription, pupdatedstatus).ok()
    }
    pub unsafe fn SetUtilities<P0>(&self, putilities: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<INetDiagHelperUtilFactory>,
    {
        (::windows::core::Interface::vtable(self).SetUtilities)(::windows::core::Interface::as_raw(self), putilities.into_param().abi()).ok()
    }
    pub unsafe fn ReproduceFailure(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReproduceFailure)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(INetDiagHelperEx, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetDiagHelperEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetDiagHelperEx {}
impl ::core::fmt::Debug for INetDiagHelperEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetDiagHelperEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetDiagHelperEx {
    type Vtable = INetDiagHelperEx_Vtbl;
}
impl ::core::clone::Clone for INetDiagHelperEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetDiagHelperEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x972dab4d_e4e3_4fc6_ae54_5f65ccde4a15);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelperEx_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ReconfirmLowHealth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut ::windows::core::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReconfirmLowHealth: usize,
    pub SetUtilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, putilities: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ReproduceFailure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[repr(transparent)]
pub struct INetDiagHelperInfo(::windows::core::IUnknown);
impl INetDiagHelperInfo {
    pub unsafe fn GetAttributeInfo(&self, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttributeInfo)(::windows::core::Interface::as_raw(self), pcelt, pprgattributeinfos).ok()
    }
}
::windows::imp::interface_hierarchy!(INetDiagHelperInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetDiagHelperInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetDiagHelperInfo {}
impl ::core::fmt::Debug for INetDiagHelperInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetDiagHelperInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetDiagHelperInfo {
    type Vtable = INetDiagHelperInfo_Vtbl;
}
impl ::core::clone::Clone for INetDiagHelperInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetDiagHelperInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0b35747_ebf5_11d8_bbe9_505054503030);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelperInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetAttributeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[repr(transparent)]
pub struct INetDiagHelperUtilFactory(::windows::core::IUnknown);
impl INetDiagHelperUtilFactory {
    pub unsafe fn CreateUtilityInstance<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows::core::Interface::vtable(self).CreateUtilityInstance)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(INetDiagHelperUtilFactory, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INetDiagHelperUtilFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INetDiagHelperUtilFactory {}
impl ::core::fmt::Debug for INetDiagHelperUtilFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INetDiagHelperUtilFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INetDiagHelperUtilFactory {
    type Vtable = INetDiagHelperUtilFactory_Vtbl;
}
impl ::core::clone::Clone for INetDiagHelperUtilFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INetDiagHelperUtilFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x104613fb_bc57_4178_95ba_88809698354a);
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelperUtilFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateUtilityInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const DF_IMPERSONATION: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const DF_TRACELESS: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const NDF_ADD_CAPTURE_TRACE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const NDF_APPLY_INCLUSION_LIST_FILTER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const NDF_ERROR_START: u32 = 63744u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const NDF_E_BAD_PARAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895611i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const NDF_E_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895614i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const NDF_E_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895612i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const NDF_E_LENGTH_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895616i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const NDF_E_NOHELPERCLASS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895615i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const NDF_E_PROBLEM_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895608i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const NDF_E_UNKNOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895609i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const NDF_E_VALIDATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895610i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const NDF_INBOUND_FLAG_EDGETRAVERSAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const NDF_INBOUND_FLAG_HEALTHCHECK: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RCF_ISCONFIRMED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RCF_ISLEAF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RCF_ISTHIRDPARTY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RF_CONTACT_ADMIN: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RF_INFORMATION_ONLY: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RF_REPRO: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RF_RESERVED: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RF_RESERVED_CA: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RF_RESERVED_LNI: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RF_SHOW_EVENTS: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RF_UI_ONLY: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RF_USER_ACTION: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RF_USER_CONFIRMATION: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RF_VALIDATE_HELPTOPIC: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RF_WORKAROUND: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ATTRIBUTE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_INVALID: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_BOOLEAN: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_INT8: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_UINT8: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_INT16: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_UINT16: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_INT32: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_UINT32: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_INT64: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_UINT64: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_STRING: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_GUID: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_LIFE_TIME: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_SOCKADDR: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const AT_OCTET_STRING: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(14i32);
impl ::core::marker::Copy for ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ATTRIBUTE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ATTRIBUTE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ATTRIBUTE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DIAGNOSIS_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const DS_NOT_IMPLEMENTED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const DS_CONFIRMED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const DS_REJECTED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const DS_INDETERMINATE: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const DS_DEFERRED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const DS_PASSTHROUGH: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(5i32);
impl ::core::marker::Copy for DIAGNOSIS_STATUS {}
impl ::core::clone::Clone for DIAGNOSIS_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DIAGNOSIS_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DIAGNOSIS_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DIAGNOSIS_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DIAGNOSIS_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PROBLEM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const PT_INVALID: PROBLEM_TYPE = PROBLEM_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const PT_LOW_HEALTH: PROBLEM_TYPE = PROBLEM_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const PT_LOWER_HEALTH: PROBLEM_TYPE = PROBLEM_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const PT_DOWN_STREAM_HEALTH: PROBLEM_TYPE = PROBLEM_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const PT_HIGH_UTILIZATION: PROBLEM_TYPE = PROBLEM_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const PT_HIGHER_UTILIZATION: PROBLEM_TYPE = PROBLEM_TYPE(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const PT_UP_STREAM_UTILIZATION: PROBLEM_TYPE = PROBLEM_TYPE(32i32);
impl ::core::marker::Copy for PROBLEM_TYPE {}
impl ::core::clone::Clone for PROBLEM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROBLEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PROBLEM_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PROBLEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROBLEM_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REPAIR_RISK(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RR_NOROLLBACK: REPAIR_RISK = REPAIR_RISK(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RR_ROLLBACK: REPAIR_RISK = REPAIR_RISK(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RR_NORISK: REPAIR_RISK = REPAIR_RISK(2i32);
impl ::core::marker::Copy for REPAIR_RISK {}
impl ::core::clone::Clone for REPAIR_RISK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REPAIR_RISK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REPAIR_RISK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REPAIR_RISK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPAIR_RISK").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REPAIR_SCOPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RS_SYSTEM: REPAIR_SCOPE = REPAIR_SCOPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RS_USER: REPAIR_SCOPE = REPAIR_SCOPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RS_APPLICATION: REPAIR_SCOPE = REPAIR_SCOPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RS_PROCESS: REPAIR_SCOPE = REPAIR_SCOPE(3i32);
impl ::core::marker::Copy for REPAIR_SCOPE {}
impl ::core::clone::Clone for REPAIR_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REPAIR_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REPAIR_SCOPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REPAIR_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPAIR_SCOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct REPAIR_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RS_NOT_IMPLEMENTED: REPAIR_STATUS = REPAIR_STATUS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RS_REPAIRED: REPAIR_STATUS = REPAIR_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RS_UNREPAIRED: REPAIR_STATUS = REPAIR_STATUS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RS_DEFERRED: REPAIR_STATUS = REPAIR_STATUS(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const RS_USER_ACTION: REPAIR_STATUS = REPAIR_STATUS(4i32);
impl ::core::marker::Copy for REPAIR_STATUS {}
impl ::core::clone::Clone for REPAIR_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REPAIR_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for REPAIR_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for REPAIR_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REPAIR_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UI_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const UIT_INVALID: UI_INFO_TYPE = UI_INFO_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const UIT_NONE: UI_INFO_TYPE = UI_INFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const UIT_SHELL_COMMAND: UI_INFO_TYPE = UI_INFO_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const UIT_HELP_PANE: UI_INFO_TYPE = UI_INFO_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub const UIT_DUI: UI_INFO_TYPE = UI_INFO_TYPE(4i32);
impl ::core::marker::Copy for UI_INFO_TYPE {}
impl ::core::clone::Clone for UI_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UI_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for UI_INFO_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for UI_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UI_INFO_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub struct DIAG_SOCKADDR {
    pub family: u16,
    pub data: [u8; 126],
}
impl ::core::marker::Copy for DIAG_SOCKADDR {}
impl ::core::clone::Clone for DIAG_SOCKADDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DIAG_SOCKADDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DIAG_SOCKADDR").field("family", &self.family).field("data", &self.data).finish()
    }
}
impl ::windows::core::TypeKind for DIAG_SOCKADDR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DIAG_SOCKADDR {
    fn eq(&self, other: &Self) -> bool {
        self.family == other.family && self.data == other.data
    }
}
impl ::core::cmp::Eq for DIAG_SOCKADDR {}
impl ::core::default::Default for DIAG_SOCKADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub struct DiagnosticsInfo {
    pub cost: i32,
    pub flags: u32,
}
impl ::core::marker::Copy for DiagnosticsInfo {}
impl ::core::clone::Clone for DiagnosticsInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DiagnosticsInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DiagnosticsInfo").field("cost", &self.cost).field("flags", &self.flags).finish()
    }
}
impl ::windows::core::TypeKind for DiagnosticsInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DiagnosticsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for DiagnosticsInfo {}
impl ::core::default::Default for DiagnosticsInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HELPER_ATTRIBUTE {
    pub pwszName: ::windows::core::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
    pub Anonymous: HELPER_ATTRIBUTE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HELPER_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HELPER_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for HELPER_ATTRIBUTE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HELPER_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union HELPER_ATTRIBUTE_0 {
    pub Boolean: super::super::Foundation::BOOL,
    pub Char: u8,
    pub Byte: u8,
    pub Short: i16,
    pub Word: u16,
    pub Int: i32,
    pub DWord: u32,
    pub Int64: i64,
    pub UInt64: u64,
    pub PWStr: ::windows::core::PWSTR,
    pub Guid: ::windows::core::GUID,
    pub LifeTime: LIFE_TIME,
    pub Address: DIAG_SOCKADDR,
    pub OctetString: OCTET_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HELPER_ATTRIBUTE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HELPER_ATTRIBUTE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for HELPER_ATTRIBUTE_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HELPER_ATTRIBUTE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HYPOTHESIS {
    pub pwszClassName: ::windows::core::PWSTR,
    pub pwszDescription: ::windows::core::PWSTR,
    pub celt: u32,
    pub rgAttributes: *mut HELPER_ATTRIBUTE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HYPOTHESIS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HYPOTHESIS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HYPOTHESIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HYPOTHESIS").field("pwszClassName", &self.pwszClassName).field("pwszDescription", &self.pwszDescription).field("celt", &self.celt).field("rgAttributes", &self.rgAttributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for HYPOTHESIS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HYPOTHESIS {
    fn eq(&self, other: &Self) -> bool {
        self.pwszClassName == other.pwszClassName && self.pwszDescription == other.pwszDescription && self.celt == other.celt && self.rgAttributes == other.rgAttributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HYPOTHESIS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HYPOTHESIS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub struct HelperAttributeInfo {
    pub pwszName: ::windows::core::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
}
impl ::core::marker::Copy for HelperAttributeInfo {}
impl ::core::clone::Clone for HelperAttributeInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HelperAttributeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HelperAttributeInfo").field("pwszName", &self.pwszName).field("type", &self.r#type).finish()
    }
}
impl ::windows::core::TypeKind for HelperAttributeInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HelperAttributeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.pwszName == other.pwszName && self.r#type == other.r#type
    }
}
impl ::core::cmp::Eq for HelperAttributeInfo {}
impl ::core::default::Default for HelperAttributeInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HypothesisResult {
    pub hypothesis: HYPOTHESIS,
    pub pathStatus: DIAGNOSIS_STATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HypothesisResult {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HypothesisResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HypothesisResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HypothesisResult").field("hypothesis", &self.hypothesis).field("pathStatus", &self.pathStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for HypothesisResult {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HypothesisResult {
    fn eq(&self, other: &Self) -> bool {
        self.hypothesis == other.hypothesis && self.pathStatus == other.pathStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HypothesisResult {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HypothesisResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LIFE_TIME {
    pub startTime: super::super::Foundation::FILETIME,
    pub endTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LIFE_TIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LIFE_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LIFE_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LIFE_TIME").field("startTime", &self.startTime).field("endTime", &self.endTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for LIFE_TIME {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LIFE_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.startTime == other.startTime && self.endTime == other.endTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LIFE_TIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LIFE_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub struct OCTET_STRING {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl ::core::marker::Copy for OCTET_STRING {}
impl ::core::clone::Clone for OCTET_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OCTET_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OCTET_STRING").field("dwLength", &self.dwLength).field("lpValue", &self.lpValue).finish()
    }
}
impl ::windows::core::TypeKind for OCTET_STRING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for OCTET_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for OCTET_STRING {}
impl ::core::default::Default for OCTET_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub struct RepairInfo {
    pub guid: ::windows::core::GUID,
    pub pwszClassName: ::windows::core::PWSTR,
    pub pwszDescription: ::windows::core::PWSTR,
    pub sidType: u32,
    pub cost: i32,
    pub flags: u32,
    pub scope: REPAIR_SCOPE,
    pub risk: REPAIR_RISK,
    pub UiInfo: UiInfo,
    pub rootCauseIndex: i32,
}
impl ::core::marker::Copy for RepairInfo {}
impl ::core::clone::Clone for RepairInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for RepairInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RepairInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub struct RepairInfoEx {
    pub repair: RepairInfo,
    pub repairRank: u16,
}
impl ::core::marker::Copy for RepairInfoEx {}
impl ::core::clone::Clone for RepairInfoEx {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for RepairInfoEx {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RepairInfoEx {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub struct RootCauseInfo {
    pub pwszDescription: ::windows::core::PWSTR,
    pub rootCauseID: ::windows::core::GUID,
    pub rootCauseFlags: u32,
    pub networkInterfaceID: ::windows::core::GUID,
    pub pRepairs: *mut RepairInfoEx,
    pub repairCount: u16,
}
impl ::core::marker::Copy for RootCauseInfo {}
impl ::core::clone::Clone for RootCauseInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RootCauseInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RootCauseInfo").field("pwszDescription", &self.pwszDescription).field("rootCauseID", &self.rootCauseID).field("rootCauseFlags", &self.rootCauseFlags).field("networkInterfaceID", &self.networkInterfaceID).field("pRepairs", &self.pRepairs).field("repairCount", &self.repairCount).finish()
    }
}
impl ::windows::core::TypeKind for RootCauseInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RootCauseInfo {
    fn eq(&self, other: &Self) -> bool {
        self.pwszDescription == other.pwszDescription && self.rootCauseID == other.rootCauseID && self.rootCauseFlags == other.rootCauseFlags && self.networkInterfaceID == other.networkInterfaceID && self.pRepairs == other.pRepairs && self.repairCount == other.repairCount
    }
}
impl ::core::cmp::Eq for RootCauseInfo {}
impl ::core::default::Default for RootCauseInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub struct ShellCommandInfo {
    pub pwszOperation: ::windows::core::PWSTR,
    pub pwszFile: ::windows::core::PWSTR,
    pub pwszParameters: ::windows::core::PWSTR,
    pub pwszDirectory: ::windows::core::PWSTR,
    pub nShowCmd: u32,
}
impl ::core::marker::Copy for ShellCommandInfo {}
impl ::core::clone::Clone for ShellCommandInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ShellCommandInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ShellCommandInfo").field("pwszOperation", &self.pwszOperation).field("pwszFile", &self.pwszFile).field("pwszParameters", &self.pwszParameters).field("pwszDirectory", &self.pwszDirectory).field("nShowCmd", &self.nShowCmd).finish()
    }
}
impl ::windows::core::TypeKind for ShellCommandInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ShellCommandInfo {
    fn eq(&self, other: &Self) -> bool {
        self.pwszOperation == other.pwszOperation && self.pwszFile == other.pwszFile && self.pwszParameters == other.pwszParameters && self.pwszDirectory == other.pwszDirectory && self.nShowCmd == other.nShowCmd
    }
}
impl ::core::cmp::Eq for ShellCommandInfo {}
impl ::core::default::Default for ShellCommandInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub struct UiInfo {
    pub r#type: UI_INFO_TYPE,
    pub Anonymous: UiInfo_0,
}
impl ::core::marker::Copy for UiInfo {}
impl ::core::clone::Clone for UiInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for UiInfo {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for UiInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkDiagnosticsFramework\"`*"]
pub union UiInfo_0 {
    pub pwzNull: ::windows::core::PWSTR,
    pub ShellInfo: ShellCommandInfo,
    pub pwzHelpUrl: ::windows::core::PWSTR,
    pub pwzDui: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for UiInfo_0 {}
impl ::core::clone::Clone for UiInfo_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for UiInfo_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for UiInfo_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
