#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORCloseHive<P0>(handle: P0) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORCloseHive ( handle : ORHKEY ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORCloseHive(handle.into_param().abi())
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORCloseKey<P0>(keyhandle: P0) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORCloseKey ( keyhandle : ORHKEY ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORCloseKey(keyhandle.into_param().abi())
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORCreateHive(horkey: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "offreg.dll""system" fn ORCreateHive ( horkey : *mut ORHKEY ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORCreateHive(horkey)
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ORCreateKey<P0, P1, P2, P3>(keyhandle: P0, lpsubkey: P1, lpclass: P2, dwoptions: u32, psecuritydescriptor: P3, phkresult: *mut ORHKEY, pdwdisposition: ::core::option::Option<*mut u32>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORCreateKey ( keyhandle : ORHKEY , lpsubkey : :: windows::core::PCWSTR , lpclass : :: windows::core::PCWSTR , dwoptions : u32 , psecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR , phkresult : *mut ORHKEY , pdwdisposition : *mut u32 ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORCreateKey(keyhandle.into_param().abi(), lpsubkey.into_param().abi(), lpclass.into_param().abi(), dwoptions, psecuritydescriptor.into_param().abi(), phkresult, ::core::mem::transmute(pdwdisposition.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORDeleteKey<P0, P1>(handle: P0, lpsubkey: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORDeleteKey ( handle : ORHKEY , lpsubkey : :: windows::core::PCWSTR ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORDeleteKey(handle.into_param().abi(), lpsubkey.into_param().abi())
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORDeleteValue<P0, P1>(handle: P0, lpvaluename: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORDeleteValue ( handle : ORHKEY , lpvaluename : :: windows::core::PCWSTR ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORDeleteValue(handle.into_param().abi(), lpvaluename.into_param().abi())
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OREnumKey<P0>(handle: P0, dwindex: u32, lpname: ::windows::core::PWSTR, lpcname: *mut u32, lpclass: ::windows::core::PWSTR, lpcclass: ::core::option::Option<*mut u32>, lpftlastwritetime: ::core::option::Option<*mut super::super::super::Win32::Foundation::FILETIME>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn OREnumKey ( handle : ORHKEY , dwindex : u32 , lpname : :: windows::core::PWSTR , lpcname : *mut u32 , lpclass : :: windows::core::PWSTR , lpcclass : *mut u32 , lpftlastwritetime : *mut super::super::super::Win32::Foundation:: FILETIME ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    OREnumKey(handle.into_param().abi(), dwindex, ::core::mem::transmute(lpname), lpcname, ::core::mem::transmute(lpclass), ::core::mem::transmute(lpcclass.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpftlastwritetime.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OREnumValue<P0>(handle: P0, dwindex: u32, lpvaluename: ::windows::core::PWSTR, lpcvaluename: *mut u32, lptype: ::core::option::Option<*mut u32>, lpdata: ::core::option::Option<*mut u8>, lpcbdata: ::core::option::Option<*mut u32>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn OREnumValue ( handle : ORHKEY , dwindex : u32 , lpvaluename : :: windows::core::PWSTR , lpcvaluename : *mut u32 , lptype : *mut u32 , lpdata : *mut u8 , lpcbdata : *mut u32 ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    OREnumValue(handle.into_param().abi(), dwindex, ::core::mem::transmute(lpvaluename), lpcvaluename, ::core::mem::transmute(lptype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(lpcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ORGetKeySecurity<P0>(handle: P0, securityinformation: u32, psecuritydescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, lpcbsecuritydescriptor: *mut u32) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORGetKeySecurity ( handle : ORHKEY , securityinformation : u32 , psecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR , lpcbsecuritydescriptor : *mut u32 ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORGetKeySecurity(handle.into_param().abi(), securityinformation, psecuritydescriptor, lpcbsecuritydescriptor)
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORGetValue<P0, P1, P2>(handle: P0, lpsubkey: P1, lpvalue: P2, pdwtype: ::core::option::Option<*mut u32>, pvdata: ::core::option::Option<*mut ::core::ffi::c_void>, pcbdata: ::core::option::Option<*mut u32>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORGetValue ( handle : ORHKEY , lpsubkey : :: windows::core::PCWSTR , lpvalue : :: windows::core::PCWSTR , pdwtype : *mut u32 , pvdata : *mut ::core::ffi::c_void , pcbdata : *mut u32 ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORGetValue(handle.into_param().abi(), lpsubkey.into_param().abi(), lpvalue.into_param().abi(), ::core::mem::transmute(pdwtype.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvdata.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbdata.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORGetVersion(pdwmajorversion: *mut u32, pdwminorversion: *mut u32) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "offreg.dll""system" fn ORGetVersion ( pdwmajorversion : *mut u32 , pdwminorversion : *mut u32 ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORGetVersion(pdwmajorversion, pdwminorversion)
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORGetVirtualFlags<P0>(handle: P0, pdwflags: *mut u32) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORGetVirtualFlags ( handle : ORHKEY , pdwflags : *mut u32 ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORGetVirtualFlags(handle.into_param().abi(), pdwflags)
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORMergeHives(hivehandles: &[ORHKEY], phkresult: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "offreg.dll""system" fn ORMergeHives ( hivehandles : *const ORHKEY , hivecount : u32 , phkresult : *mut ORHKEY ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORMergeHives(::core::mem::transmute(hivehandles.as_ptr()), hivehandles.len() as _, phkresult)
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OROpenHive<P0>(filepath: P0, horkey: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn OROpenHive ( filepath : :: windows::core::PCWSTR , horkey : *mut ORHKEY ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    OROpenHive(filepath.into_param().abi(), horkey)
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OROpenHiveByHandle<P0>(filehandle: P0, horkey: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<super::super::super::Win32::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn OROpenHiveByHandle ( filehandle : super::super::super::Win32::Foundation:: HANDLE , horkey : *mut ORHKEY ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    OROpenHiveByHandle(filehandle.into_param().abi(), horkey)
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OROpenKey<P0, P1>(handle: P0, lpsubkey: P1, phkresult: *mut ORHKEY) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn OROpenKey ( handle : ORHKEY , lpsubkey : :: windows::core::PCWSTR , phkresult : *mut ORHKEY ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    OROpenKey(handle.into_param().abi(), lpsubkey.into_param().abi(), phkresult)
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORQueryInfoKey<P0>(handle: P0, lpclass: ::windows::core::PWSTR, lpcclass: ::core::option::Option<*mut u32>, lpcsubkeys: ::core::option::Option<*mut u32>, lpcmaxsubkeylen: ::core::option::Option<*mut u32>, lpcmaxclasslen: ::core::option::Option<*mut u32>, lpcvalues: ::core::option::Option<*mut u32>, lpcmaxvaluenamelen: ::core::option::Option<*mut u32>, lpcmaxvaluelen: ::core::option::Option<*mut u32>, lpcbsecuritydescriptor: ::core::option::Option<*mut u32>, lpftlastwritetime: ::core::option::Option<*mut super::super::super::Win32::Foundation::FILETIME>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORQueryInfoKey ( handle : ORHKEY , lpclass : :: windows::core::PWSTR , lpcclass : *mut u32 , lpcsubkeys : *mut u32 , lpcmaxsubkeylen : *mut u32 , lpcmaxclasslen : *mut u32 , lpcvalues : *mut u32 , lpcmaxvaluenamelen : *mut u32 , lpcmaxvaluelen : *mut u32 , lpcbsecuritydescriptor : *mut u32 , lpftlastwritetime : *mut super::super::super::Win32::Foundation:: FILETIME ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORQueryInfoKey(
        handle.into_param().abi(),
        ::core::mem::transmute(lpclass),
        ::core::mem::transmute(lpcclass.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcsubkeys.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcmaxsubkeylen.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcmaxclasslen.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcvalues.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcmaxvaluenamelen.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcmaxvaluelen.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpcbsecuritydescriptor.unwrap_or(::std::ptr::null_mut())),
        ::core::mem::transmute(lpftlastwritetime.unwrap_or(::std::ptr::null_mut())),
    )
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORRenameKey<P0, P1>(handle: P0, lpnewname: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORRenameKey ( handle : ORHKEY , lpnewname : :: windows::core::PCWSTR ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORRenameKey(handle.into_param().abi(), lpnewname.into_param().abi())
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORSaveHive<P0, P1>(horkey: P0, hivepath: P1, osmajorversion: u32, osminorversion: u32) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORSaveHive ( horkey : ORHKEY , hivepath : :: windows::core::PCWSTR , osmajorversion : u32 , osminorversion : u32 ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORSaveHive(horkey.into_param().abi(), hivepath.into_param().abi(), osmajorversion, osminorversion)
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ORSetKeySecurity<P0, P1>(handle: P0, securityinformation: u32, psecuritydescriptor: P1) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
    P1: ::windows::core::IntoParam<super::super::super::Win32::Security::PSECURITY_DESCRIPTOR>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORSetKeySecurity ( handle : ORHKEY , securityinformation : u32 , psecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORSetKeySecurity(handle.into_param().abi(), securityinformation, psecuritydescriptor.into_param().abi())
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORSetValue<P0, P1>(handle: P0, lpvaluename: P1, dwtype: u32, lpdata: ::core::option::Option<&[u8]>) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORSetValue ( handle : ORHKEY , lpvaluename : :: windows::core::PCWSTR , dwtype : u32 , lpdata : *const u8 , cbdata : u32 ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORSetValue(handle.into_param().abi(), lpvaluename.into_param().abi(), dwtype, ::core::mem::transmute(lpdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpdata.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORSetVirtualFlags<P0>(handle: P0, dwflags: u32) -> super::super::super::Win32::Foundation::WIN32_ERROR
where
    P0: ::windows::core::IntoParam<ORHKEY>,
{
    ::windows::imp::link ! ( "offreg.dll""system" fn ORSetVirtualFlags ( handle : ORHKEY , dwflags : u32 ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORSetVirtualFlags(handle.into_param().abi(), dwflags)
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORShutdown() -> super::super::super::Win32::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "offreg.dll""system" fn ORShutdown ( ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORShutdown()
}
#[doc = "*Required features: `\"Wdk_System_OfflineRegistry\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ORStart() -> super::super::super::Win32::Foundation::WIN32_ERROR {
    ::windows::imp::link ! ( "offreg.dll""system" fn ORStart ( ) -> super::super::super::Win32::Foundation:: WIN32_ERROR );
    ORStart()
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ORHKEY(pub isize);
impl ORHKEY {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for ORHKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for ORHKEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for ORHKEY {}
impl ::core::fmt::Debug for ORHKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ORHKEY").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for ORHKEY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
