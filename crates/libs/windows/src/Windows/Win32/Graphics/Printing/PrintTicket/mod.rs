#[doc = "Required features: `\"Win32_Storage_Xps\"`"]
#[cfg(feature = "Win32_Storage_Xps")]
#[inline]
pub unsafe fn PTCloseProvider<P0>(hprovider: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Storage::Xps::HPTPROVIDER>,
{
    ::windows_targets::link!("prntvpt.dll" "system" fn PTCloseProvider(hprovider : super::super::super::Storage::Xps:: HPTPROVIDER) -> ::windows_core::HRESULT);
    PTCloseProvider(hprovider.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTConvertDevModeToPrintTicket<P0, P1>(hprovider: P0, cbdevmode: u32, pdevmode: *const super::super::Gdi::DEVMODEA, scope: EPrintTicketScope, pprintticket: P1) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Storage::Xps::HPTPROVIDER>,
    P1: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
{
    ::windows_targets::link!("prntvpt.dll" "system" fn PTConvertDevModeToPrintTicket(hprovider : super::super::super::Storage::Xps:: HPTPROVIDER, cbdevmode : u32, pdevmode : *const super::super::Gdi:: DEVMODEA, scope : EPrintTicketScope, pprintticket : * mut::core::ffi::c_void) -> ::windows_core::HRESULT);
    PTConvertDevModeToPrintTicket(hprovider.into_param().abi(), cbdevmode, pdevmode, scope, pprintticket.into_param().abi()).ok()
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTConvertPrintTicketToDevMode<P0, P1>(hprovider: P0, pprintticket: P1, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut *mut super::super::Gdi::DEVMODEA, pbstrerrormessage: ::core::option::Option<*mut ::windows_core::BSTR>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Storage::Xps::HPTPROVIDER>,
    P1: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
{
    ::windows_targets::link!("prntvpt.dll" "system" fn PTConvertPrintTicketToDevMode(hprovider : super::super::super::Storage::Xps:: HPTPROVIDER, pprintticket : * mut::core::ffi::c_void, basedevmodetype : EDefaultDevmodeType, scope : EPrintTicketScope, pcbdevmode : *mut u32, ppdevmode : *mut *mut super::super::Gdi:: DEVMODEA, pbstrerrormessage : *mut ::std::mem::MaybeUninit <::windows_core::BSTR >) -> ::windows_core::HRESULT);
    PTConvertPrintTicketToDevMode(hprovider.into_param().abi(), pprintticket.into_param().abi(), basedevmodetype, scope, pcbdevmode, ppdevmode, ::core::mem::transmute(pbstrerrormessage.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTGetPrintCapabilities<P0, P1, P2>(hprovider: P0, pprintticket: P1, pcapabilities: P2, pbstrerrormessage: ::core::option::Option<*mut ::windows_core::BSTR>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Storage::Xps::HPTPROVIDER>,
    P1: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
    P2: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
{
    ::windows_targets::link!("prntvpt.dll" "system" fn PTGetPrintCapabilities(hprovider : super::super::super::Storage::Xps:: HPTPROVIDER, pprintticket : * mut::core::ffi::c_void, pcapabilities : * mut::core::ffi::c_void, pbstrerrormessage : *mut ::std::mem::MaybeUninit <::windows_core::BSTR >) -> ::windows_core::HRESULT);
    PTGetPrintCapabilities(hprovider.into_param().abi(), pprintticket.into_param().abi(), pcapabilities.into_param().abi(), ::core::mem::transmute(pbstrerrormessage.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTGetPrintDeviceCapabilities<P0, P1, P2>(hprovider: P0, pprintticket: P1, pdevicecapabilities: P2, pbstrerrormessage: ::core::option::Option<*mut ::windows_core::BSTR>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Storage::Xps::HPTPROVIDER>,
    P1: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
    P2: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
{
    ::windows_targets::link!("prntvpt.dll" "system" fn PTGetPrintDeviceCapabilities(hprovider : super::super::super::Storage::Xps:: HPTPROVIDER, pprintticket : * mut::core::ffi::c_void, pdevicecapabilities : * mut::core::ffi::c_void, pbstrerrormessage : *mut ::std::mem::MaybeUninit <::windows_core::BSTR >) -> ::windows_core::HRESULT);
    PTGetPrintDeviceCapabilities(hprovider.into_param().abi(), pprintticket.into_param().abi(), pdevicecapabilities.into_param().abi(), ::core::mem::transmute(pbstrerrormessage.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTGetPrintDeviceResources<P0, P1, P2, P3>(hprovider: P0, pszlocalename: P1, pprintticket: P2, pdeviceresources: P3, pbstrerrormessage: ::core::option::Option<*mut ::windows_core::BSTR>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Storage::Xps::HPTPROVIDER>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
    P3: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
{
    ::windows_targets::link!("prntvpt.dll" "system" fn PTGetPrintDeviceResources(hprovider : super::super::super::Storage::Xps:: HPTPROVIDER, pszlocalename : ::windows_core::PCWSTR, pprintticket : * mut::core::ffi::c_void, pdeviceresources : * mut::core::ffi::c_void, pbstrerrormessage : *mut ::std::mem::MaybeUninit <::windows_core::BSTR >) -> ::windows_core::HRESULT);
    PTGetPrintDeviceResources(hprovider.into_param().abi(), pszlocalename.into_param().abi(), pprintticket.into_param().abi(), pdeviceresources.into_param().abi(), ::core::mem::transmute(pbstrerrormessage.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTMergeAndValidatePrintTicket<P0, P1, P2, P3>(hprovider: P0, pbaseticket: P1, pdeltaticket: P2, scope: EPrintTicketScope, presultticket: P3, pbstrerrormessage: ::core::option::Option<*mut ::windows_core::BSTR>) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::super::Storage::Xps::HPTPROVIDER>,
    P1: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
    P2: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
    P3: ::windows_core::IntoParam<super::super::super::System::Com::IStream>,
{
    ::windows_targets::link!("prntvpt.dll" "system" fn PTMergeAndValidatePrintTicket(hprovider : super::super::super::Storage::Xps:: HPTPROVIDER, pbaseticket : * mut::core::ffi::c_void, pdeltaticket : * mut::core::ffi::c_void, scope : EPrintTicketScope, presultticket : * mut::core::ffi::c_void, pbstrerrormessage : *mut ::std::mem::MaybeUninit <::windows_core::BSTR >) -> ::windows_core::HRESULT);
    PTMergeAndValidatePrintTicket(hprovider.into_param().abi(), pbaseticket.into_param().abi(), pdeltaticket.into_param().abi(), scope, presultticket.into_param().abi(), ::core::mem::transmute(pbstrerrormessage.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`"]
#[cfg(feature = "Win32_Storage_Xps")]
#[inline]
pub unsafe fn PTOpenProvider<P0>(pszprintername: P0, dwversion: u32) -> ::windows_core::Result<super::super::super::Storage::Xps::HPTPROVIDER>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("prntvpt.dll" "system" fn PTOpenProvider(pszprintername : ::windows_core::PCWSTR, dwversion : u32, phprovider : *mut super::super::super::Storage::Xps:: HPTPROVIDER) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PTOpenProvider(pszprintername.into_param().abi(), dwversion, &mut result__).from_abi(result__)
}
#[doc = "Required features: `\"Win32_Storage_Xps\"`"]
#[cfg(feature = "Win32_Storage_Xps")]
#[inline]
pub unsafe fn PTOpenProviderEx<P0>(pszprintername: P0, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER, pusedversion: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("prntvpt.dll" "system" fn PTOpenProviderEx(pszprintername : ::windows_core::PCWSTR, dwmaxversion : u32, dwprefversion : u32, phprovider : *mut super::super::super::Storage::Xps:: HPTPROVIDER, pusedversion : *mut u32) -> ::windows_core::HRESULT);
    PTOpenProviderEx(pszprintername.into_param().abi(), dwmaxversion, dwprefversion, phprovider, pusedversion).ok()
}
#[inline]
pub unsafe fn PTQuerySchemaVersionSupport<P0>(pszprintername: P0) -> ::windows_core::Result<u32>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("prntvpt.dll" "system" fn PTQuerySchemaVersionSupport(pszprintername : ::windows_core::PCWSTR, pmaxversion : *mut u32) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    PTQuerySchemaVersionSupport(pszprintername.into_param().abi(), &mut result__).from_abi(result__)
}
#[inline]
pub unsafe fn PTReleaseMemory(pbuffer: *const ::core::ffi::c_void) -> ::windows_core::Result<()> {
    ::windows_targets::link!("prntvpt.dll" "system" fn PTReleaseMemory(pbuffer : *const ::core::ffi::c_void) -> ::windows_core::HRESULT);
    PTReleaseMemory(pbuffer).ok()
}
pub const E_DELTA_PRINTTICKET_FORMAT: u32 = 2147745797u32;
pub const E_PRINTCAPABILITIES_FORMAT: u32 = 2147745796u32;
pub const E_PRINTDEVICECAPABILITIES_FORMAT: u32 = 2147745798u32;
pub const E_PRINTTICKET_FORMAT: u32 = 2147745795u32;
pub const PRINTTICKET_ISTREAM_APIS: u32 = 1u32;
pub const S_PT_CONFLICT_RESOLVED: u32 = 262146u32;
pub const S_PT_NO_CONFLICT: u32 = 262145u32;
pub const kPTDocumentScope: EPrintTicketScope = EPrintTicketScope(1i32);
pub const kPTJobScope: EPrintTicketScope = EPrintTicketScope(2i32);
pub const kPTPageScope: EPrintTicketScope = EPrintTicketScope(0i32);
pub const kPrinterDefaultDevmode: EDefaultDevmodeType = EDefaultDevmodeType(1i32);
pub const kUserDefaultDevmode: EDefaultDevmodeType = EDefaultDevmodeType(0i32);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EDefaultDevmodeType(pub i32);
impl ::core::marker::Copy for EDefaultDevmodeType {}
impl ::core::clone::Clone for EDefaultDevmodeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EDefaultDevmodeType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EDefaultDevmodeType {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EDefaultDevmodeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDefaultDevmodeType").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EPrintTicketScope(pub i32);
impl ::core::marker::Copy for EPrintTicketScope {}
impl ::core::clone::Clone for EPrintTicketScope {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EPrintTicketScope {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for EPrintTicketScope {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for EPrintTicketScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPrintTicketScope").field(&self.0).finish()
    }
}
