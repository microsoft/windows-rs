#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EDefaultDevmodeType(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const kUserDefaultDevmode: EDefaultDevmodeType = EDefaultDevmodeType(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const kPrinterDefaultDevmode: EDefaultDevmodeType = EDefaultDevmodeType(1i32);
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
unsafe impl ::windows::core::Abi for EDefaultDevmodeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EDefaultDevmodeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EDefaultDevmodeType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct EPrintTicketScope(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const kPTPageScope: EPrintTicketScope = EPrintTicketScope(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const kPTDocumentScope: EPrintTicketScope = EPrintTicketScope(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const kPTJobScope: EPrintTicketScope = EPrintTicketScope(2i32);
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
unsafe impl ::windows::core::Abi for EPrintTicketScope {
    type Abi = Self;
}
impl ::core::fmt::Debug for EPrintTicketScope {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EPrintTicketScope").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const E_DELTA_PRINTTICKET_FORMAT: u32 = 2147745797u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const E_PRINTCAPABILITIES_FORMAT: u32 = 2147745796u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const E_PRINTDEVICECAPABILITIES_FORMAT: u32 = 2147745798u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const E_PRINTTICKET_FORMAT: u32 = 2147745795u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const PRINTTICKET_ISTREAM_APIS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Storage_Xps\"`*"]
#[cfg(feature = "Win32_Storage_Xps")]
#[inline]
pub unsafe fn PTCloseProvider<'a, P0>(hprovider: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::super::Storage::Xps::HPTPROVIDER>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PTCloseProvider(hprovider: super::super::super::Storage::Xps::HPTPROVIDER) -> ::windows::core::HRESULT;
    }
    PTCloseProvider(hprovider.into()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTConvertDevModeToPrintTicket<'a, P0, P1>(hprovider: P0, cbdevmode: u32, pdevmode: *const super::super::Gdi::DEVMODEA, scope: EPrintTicketScope, pprintticket: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::super::Storage::Xps::HPTPROVIDER>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IStream>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PTConvertDevModeToPrintTicket(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, cbdevmode: u32, pdevmode: *const super::super::Gdi::DEVMODEA, scope: EPrintTicketScope, pprintticket: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    PTConvertDevModeToPrintTicket(hprovider.into(), cbdevmode, ::core::mem::transmute(pdevmode), scope, pprintticket.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTConvertPrintTicketToDevMode<'a, P0, P1>(hprovider: P0, pprintticket: P1, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut *mut super::super::Gdi::DEVMODEA, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::super::Storage::Xps::HPTPROVIDER>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IStream>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PTConvertPrintTicketToDevMode(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: *mut ::core::ffi::c_void, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut *mut super::super::Gdi::DEVMODEA, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT;
    }
    PTConvertPrintTicketToDevMode(hprovider.into(), pprintticket.into().abi(), basedevmodetype, scope, ::core::mem::transmute(pcbdevmode), ::core::mem::transmute(ppdevmode), ::core::mem::transmute(pbstrerrormessage)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTGetPrintCapabilities<'a, P0, P1, P2>(hprovider: P0, pprintticket: P1, pcapabilities: P2) -> ::windows::core::Result<super::super::super::Foundation::BSTR>
where
    P0: ::std::convert::Into<super::super::super::Storage::Xps::HPTPROVIDER>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IStream>>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IStream>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PTGetPrintCapabilities(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: *mut ::core::ffi::c_void, pcapabilities: *mut ::core::ffi::c_void, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PTGetPrintCapabilities(hprovider.into(), pprintticket.into().abi(), pcapabilities.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BSTR>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTGetPrintDeviceCapabilities<'a, P0, P1, P2>(hprovider: P0, pprintticket: P1, pdevicecapabilities: P2) -> ::windows::core::Result<super::super::super::Foundation::BSTR>
where
    P0: ::std::convert::Into<super::super::super::Storage::Xps::HPTPROVIDER>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IStream>>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IStream>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PTGetPrintDeviceCapabilities(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: *mut ::core::ffi::c_void, pdevicecapabilities: *mut ::core::ffi::c_void, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PTGetPrintDeviceCapabilities(hprovider.into(), pprintticket.into().abi(), pdevicecapabilities.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BSTR>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTGetPrintDeviceResources<'a, P0, P1, P2, P3>(hprovider: P0, pszlocalename: P1, pprintticket: P2, pdeviceresources: P3) -> ::windows::core::Result<super::super::super::Foundation::BSTR>
where
    P0: ::std::convert::Into<super::super::super::Storage::Xps::HPTPROVIDER>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IStream>>,
    P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IStream>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PTGetPrintDeviceResources(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pszlocalename: ::windows::core::PCWSTR, pprintticket: *mut ::core::ffi::c_void, pdeviceresources: *mut ::core::ffi::c_void, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PTGetPrintDeviceResources(hprovider.into(), pszlocalename.into(), pprintticket.into().abi(), pdeviceresources.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BSTR>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTMergeAndValidatePrintTicket<'a, P0, P1, P2, P3>(hprovider: P0, pbaseticket: P1, pdeltaticket: P2, scope: EPrintTicketScope, presultticket: P3) -> ::windows::core::Result<super::super::super::Foundation::BSTR>
where
    P0: ::std::convert::Into<super::super::super::Storage::Xps::HPTPROVIDER>,
    P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IStream>>,
    P2: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IStream>>,
    P3: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IStream>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PTMergeAndValidatePrintTicket(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pbaseticket: *mut ::core::ffi::c_void, pdeltaticket: *mut ::core::ffi::c_void, scope: EPrintTicketScope, presultticket: *mut ::core::ffi::c_void, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PTMergeAndValidatePrintTicket(hprovider.into(), pbaseticket.into().abi(), pdeltaticket.into().abi(), scope, presultticket.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BSTR>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Storage_Xps\"`*"]
#[cfg(feature = "Win32_Storage_Xps")]
#[inline]
pub unsafe fn PTOpenProvider<'a, P0>(pszprintername: P0, dwversion: u32) -> ::windows::core::Result<super::super::super::Storage::Xps::HPTPROVIDER>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PTOpenProvider(pszprintername: ::windows::core::PCWSTR, dwversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PTOpenProvider(pszprintername.into(), dwversion, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Storage::Xps::HPTPROVIDER>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Storage_Xps\"`*"]
#[cfg(feature = "Win32_Storage_Xps")]
#[inline]
pub unsafe fn PTOpenProviderEx<'a, P0>(pszprintername: P0, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER, pusedversion: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PTOpenProviderEx(pszprintername: ::windows::core::PCWSTR, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER, pusedversion: *mut u32) -> ::windows::core::HRESULT;
    }
    PTOpenProviderEx(pszprintername.into(), dwmaxversion, dwprefversion, ::core::mem::transmute(phprovider), ::core::mem::transmute(pusedversion)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
#[inline]
pub unsafe fn PTQuerySchemaVersionSupport<'a, P0>(pszprintername: P0) -> ::windows::core::Result<u32>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PTQuerySchemaVersionSupport(pszprintername: ::windows::core::PCWSTR, pmaxversion: *mut u32) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    PTQuerySchemaVersionSupport(pszprintername.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
#[inline]
pub unsafe fn PTReleaseMemory(pbuffer: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn PTReleaseMemory(pbuffer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    PTReleaseMemory(::core::mem::transmute(pbuffer)).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const S_PT_CONFLICT_RESOLVED: u32 = 262146u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const S_PT_NO_CONFLICT: u32 = 262145u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
