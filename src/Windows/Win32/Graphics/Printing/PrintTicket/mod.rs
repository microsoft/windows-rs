#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EDefaultDevmodeType(pub i32);
pub const kUserDefaultDevmode: EDefaultDevmodeType = EDefaultDevmodeType(0i32);
pub const kPrinterDefaultDevmode: EDefaultDevmodeType = EDefaultDevmodeType(1i32);
impl ::std::convert::From<i32> for EDefaultDevmodeType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EDefaultDevmodeType {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EPrintTicketScope(pub i32);
pub const kPTPageScope: EPrintTicketScope = EPrintTicketScope(0i32);
pub const kPTDocumentScope: EPrintTicketScope = EPrintTicketScope(1i32);
pub const kPTJobScope: EPrintTicketScope = EPrintTicketScope(2i32);
impl ::std::convert::From<i32> for EPrintTicketScope {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EPrintTicketScope {
    type Abi = Self;
    type DefaultType = Self;
}
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`*"]
pub const E_DELTA_PRINTTICKET_FORMAT: u32 = 2147745797u32;
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`*"]
pub const E_PRINTCAPABILITIES_FORMAT: u32 = 2147745796u32;
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`*"]
pub const E_PRINTDEVICECAPABILITIES_FORMAT: u32 = 2147745798u32;
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`*"]
pub const E_PRINTTICKET_FORMAT: u32 = 2147745795u32;
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`*"]
pub const PRINTTICKET_ISTREAM_APIS: u32 = 1u32;
#[cfg(feature = "Win32_Storage_Xps")]
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Storage_Xps`*"]
#[inline]
pub unsafe fn PTCloseProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>>(hprovider: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTCloseProvider(hprovider: super::super::super::Storage::Xps::HPTPROVIDER) -> ::windows::runtime::HRESULT;
        }
        PTCloseProvider(hprovider.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn PTConvertDevModeToPrintTicket<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(hprovider: Param0, cbdevmode: u32, pdevmode: *const super::super::Gdi::DEVMODEA, scope: EPrintTicketScope, pprintticket: Param4) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTConvertDevModeToPrintTicket(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, cbdevmode: u32, pdevmode: *const super::super::Gdi::DEVMODEA, scope: EPrintTicketScope, pprintticket: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        PTConvertDevModeToPrintTicket(hprovider.into_param().abi(), ::std::mem::transmute(cbdevmode), ::std::mem::transmute(pdevmode), ::std::mem::transmute(scope), pprintticket.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn PTConvertPrintTicketToDevMode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(hprovider: Param0, pprintticket: Param1, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut *mut super::super::Gdi::DEVMODEA, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTConvertPrintTicketToDevMode(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: ::windows::runtime::RawPtr, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut *mut super::super::Gdi::DEVMODEA, pbstrerrormessage: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
        }
        PTConvertPrintTicketToDevMode(hprovider.into_param().abi(), pprintticket.into_param().abi(), ::std::mem::transmute(basedevmodetype), ::std::mem::transmute(scope), ::std::mem::transmute(pcbdevmode), ::std::mem::transmute(ppdevmode), ::std::mem::transmute(pbstrerrormessage)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn PTGetPrintCapabilities<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(hprovider: Param0, pprintticket: Param1, pcapabilities: Param2, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTGetPrintCapabilities(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: ::windows::runtime::RawPtr, pcapabilities: ::windows::runtime::RawPtr, pbstrerrormessage: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
        }
        PTGetPrintCapabilities(hprovider.into_param().abi(), pprintticket.into_param().abi(), pcapabilities.into_param().abi(), ::std::mem::transmute(pbstrerrormessage)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn PTGetPrintDeviceCapabilities<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(hprovider: Param0, pprintticket: Param1, pdevicecapabilities: Param2, pbstrerrormessage: *mut super::super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTGetPrintDeviceCapabilities(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: ::windows::runtime::RawPtr, pdevicecapabilities: ::windows::runtime::RawPtr, pbstrerrormessage: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
        }
        PTGetPrintDeviceCapabilities(hprovider.into_param().abi(), pprintticket.into_param().abi(), pdevicecapabilities.into_param().abi(), ::std::mem::transmute(pbstrerrormessage)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn PTGetPrintDeviceResources<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param3: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(
    hprovider: Param0,
    pszlocalename: Param1,
    pprintticket: Param2,
    pdeviceresources: Param3,
    pbstrerrormessage: *mut super::super::super::Foundation::BSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTGetPrintDeviceResources(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pszlocalename: super::super::super::Foundation::PWSTR, pprintticket: ::windows::runtime::RawPtr, pdeviceresources: ::windows::runtime::RawPtr, pbstrerrormessage: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
        }
        PTGetPrintDeviceResources(hprovider.into_param().abi(), pszlocalename.into_param().abi(), pprintticket.into_param().abi(), pdeviceresources.into_param().abi(), ::std::mem::transmute(pbstrerrormessage)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn PTMergeAndValidatePrintTicket<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Xps::HPTPROVIDER>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>, Param4: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(
    hprovider: Param0,
    pbaseticket: Param1,
    pdeltaticket: Param2,
    scope: EPrintTicketScope,
    presultticket: Param4,
    pbstrerrormessage: *mut super::super::super::Foundation::BSTR,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTMergeAndValidatePrintTicket(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pbaseticket: ::windows::runtime::RawPtr, pdeltaticket: ::windows::runtime::RawPtr, scope: EPrintTicketScope, presultticket: ::windows::runtime::RawPtr, pbstrerrormessage: *mut ::std::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
        }
        PTMergeAndValidatePrintTicket(hprovider.into_param().abi(), pbaseticket.into_param().abi(), pdeltaticket.into_param().abi(), ::std::mem::transmute(scope), presultticket.into_param().abi(), ::std::mem::transmute(pbstrerrormessage)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`*"]
#[inline]
pub unsafe fn PTOpenProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pszprintername: Param0, dwversion: u32) -> ::windows::runtime::Result<super::super::super::Storage::Xps::HPTPROVIDER> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTOpenProvider(pszprintername: super::super::super::Foundation::PWSTR, dwversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::super::Storage::Xps::HPTPROVIDER as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PTOpenProvider(pszprintername.into_param().abi(), ::std::mem::transmute(dwversion), &mut result__).from_abi::<super::super::super::Storage::Xps::HPTPROVIDER>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`*"]
#[inline]
pub unsafe fn PTOpenProviderEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pszprintername: Param0, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER, pusedversion: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTOpenProviderEx(pszprintername: super::super::super::Foundation::PWSTR, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER, pusedversion: *mut u32) -> ::windows::runtime::HRESULT;
        }
        PTOpenProviderEx(pszprintername.into_param().abi(), ::std::mem::transmute(dwmaxversion), ::std::mem::transmute(dwprefversion), ::std::mem::transmute(phprovider), ::std::mem::transmute(pusedversion)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn PTQuerySchemaVersionSupport<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(pszprintername: Param0) -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTQuerySchemaVersionSupport(pszprintername: super::super::super::Foundation::PWSTR, pmaxversion: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PTQuerySchemaVersionSupport(pszprintername.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`*"]
#[inline]
pub unsafe fn PTReleaseMemory(pbuffer: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PTReleaseMemory(pbuffer: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        PTReleaseMemory(::std::mem::transmute(pbuffer)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`*"]
pub const S_PT_CONFLICT_RESOLVED: u32 = 262146u32;
#[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`*"]
pub const S_PT_NO_CONFLICT: u32 = 262145u32;
