#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
pub unsafe fn PTCloseProvider<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>>(hprovider: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTCloseProvider(hprovider: super::super::super::Storage::Xps::HPTPROVIDER) -> ::windows::core::HRESULT;
        }
        PTCloseProvider(hprovider.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTConvertDevModeToPrintTicket<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>, Param4: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(hprovider: Param0, cbdevmode: u32, pdevmode: *const super::super::Gdi::DEVMODEA, scope: EPrintTicketScope, pprintticket: Param4) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTConvertDevModeToPrintTicket(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, cbdevmode: u32, pdevmode: *const super::super::Gdi::DEVMODEA, scope: EPrintTicketScope, pprintticket: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PTConvertDevModeToPrintTicket(hprovider.into_param().abi(), ::core::mem::transmute(cbdevmode), ::core::mem::transmute(pdevmode), ::core::mem::transmute(scope), pprintticket.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTConvertPrintTicketToDevMode<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(hprovider: Param0, pprintticket: Param1, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut *mut super::super::Gdi::DEVMODEA, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTConvertPrintTicketToDevMode(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: *mut ::core::ffi::c_void, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut *mut super::super::Gdi::DEVMODEA, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT;
        }
        PTConvertPrintTicketToDevMode(hprovider.into_param().abi(), pprintticket.into_param().abi(), ::core::mem::transmute(basedevmodetype), ::core::mem::transmute(scope), ::core::mem::transmute(pcbdevmode), ::core::mem::transmute(ppdevmode), ::core::mem::transmute(pbstrerrormessage)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTGetPrintCapabilities<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(hprovider: Param0, pprintticket: Param1, pcapabilities: Param2) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTGetPrintCapabilities(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: *mut ::core::ffi::c_void, pcapabilities: *mut ::core::ffi::c_void, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>>::zeroed();
        PTGetPrintCapabilities(hprovider.into_param().abi(), pprintticket.into_param().abi(), pcapabilities.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTGetPrintDeviceCapabilities<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(hprovider: Param0, pprintticket: Param1, pdevicecapabilities: Param2) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTGetPrintDeviceCapabilities(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: *mut ::core::ffi::c_void, pdevicecapabilities: *mut ::core::ffi::c_void, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>>::zeroed();
        PTGetPrintDeviceCapabilities(hprovider.into_param().abi(), pprintticket.into_param().abi(), pdevicecapabilities.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTGetPrintDeviceResources<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param3: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(hprovider: Param0, pszlocalename: Param1, pprintticket: Param2, pdeviceresources: Param3) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTGetPrintDeviceResources(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pszlocalename: ::windows::core::PCWSTR, pprintticket: *mut ::core::ffi::c_void, pdeviceresources: *mut ::core::ffi::c_void, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>>::zeroed();
        PTGetPrintDeviceResources(hprovider.into_param().abi(), pszlocalename.into_param().abi(), pprintticket.into_param().abi(), pdeviceresources.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Foundation\"`, `\"Win32_Storage_Xps\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[inline]
pub unsafe fn PTMergeAndValidatePrintTicket<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>, Param1: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>, Param4: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(hprovider: Param0, pbaseticket: Param1, pdeltaticket: Param2, scope: EPrintTicketScope, presultticket: Param4) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTMergeAndValidatePrintTicket(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pbaseticket: *mut ::core::ffi::c_void, pdeltaticket: *mut ::core::ffi::c_void, scope: EPrintTicketScope, presultticket: *mut ::core::ffi::c_void, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>>::zeroed();
        PTMergeAndValidatePrintTicket(hprovider.into_param().abi(), pbaseticket.into_param().abi(), pdeltaticket.into_param().abi(), ::core::mem::transmute(scope), presultticket.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Storage_Xps\"`*"]
#[cfg(feature = "Win32_Storage_Xps")]
#[inline]
pub unsafe fn PTOpenProvider<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszprintername: Param0, dwversion: u32) -> ::windows::core::Result<super::super::super::Storage::Xps::HPTPROVIDER> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTOpenProvider(pszprintername: ::windows::core::PCWSTR, dwversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Storage::Xps::HPTPROVIDER>::zeroed();
        PTOpenProvider(pszprintername.into_param().abi(), ::core::mem::transmute(dwversion), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Storage::Xps::HPTPROVIDER>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`, `\"Win32_Storage_Xps\"`*"]
#[cfg(feature = "Win32_Storage_Xps")]
#[inline]
pub unsafe fn PTOpenProviderEx<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszprintername: Param0, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER, pusedversion: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTOpenProviderEx(pszprintername: ::windows::core::PCWSTR, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER, pusedversion: *mut u32) -> ::windows::core::HRESULT;
        }
        PTOpenProviderEx(pszprintername.into_param().abi(), ::core::mem::transmute(dwmaxversion), ::core::mem::transmute(dwprefversion), ::core::mem::transmute(phprovider), ::core::mem::transmute(pusedversion)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
#[inline]
pub unsafe fn PTQuerySchemaVersionSupport<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszprintername: Param0) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTQuerySchemaVersionSupport(pszprintername: ::windows::core::PCWSTR, pmaxversion: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__ = ::core::mem::MaybeUninit::<u32>::zeroed();
        PTQuerySchemaVersionSupport(pszprintername.into_param().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
#[inline]
pub unsafe fn PTReleaseMemory(pbuffer: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTReleaseMemory(pbuffer: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        PTReleaseMemory(::core::mem::transmute(pbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const S_PT_CONFLICT_RESOLVED: u32 = 262146u32;
#[doc = "*Required features: `\"Win32_Graphics_Printing_PrintTicket\"`*"]
pub const S_PT_NO_CONFLICT: u32 = 262145u32;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
