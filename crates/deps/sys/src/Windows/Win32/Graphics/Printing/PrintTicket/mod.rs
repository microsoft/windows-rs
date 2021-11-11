#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Storage_Xps`*"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub fn PTCloseProvider(hprovider: super::super::super::Storage::Xps::HPTPROVIDER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
    pub fn PTConvertDevModeToPrintTicket(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, cbdevmode: u32, pdevmode: *const super::super::Gdi::DEVMODEA, scope: EPrintTicketScope, pprintticket: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
    pub fn PTConvertPrintTicketToDevMode(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: ::windows::runtime::RawPtr, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut *mut super::super::Gdi::DEVMODEA, pbstrerrormessage: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
    pub fn PTGetPrintCapabilities(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: ::windows::runtime::RawPtr, pcapabilities: ::windows::runtime::RawPtr, pbstrerrormessage: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
    pub fn PTGetPrintDeviceCapabilities(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pprintticket: ::windows::runtime::RawPtr, pdevicecapabilities: ::windows::runtime::RawPtr, pbstrerrormessage: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
    pub fn PTGetPrintDeviceResources(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pszlocalename: super::super::super::Foundation::PWSTR, pprintticket: ::windows::runtime::RawPtr, pdeviceresources: ::windows::runtime::RawPtr, pbstrerrormessage: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
    pub fn PTMergeAndValidatePrintTicket(hprovider: super::super::super::Storage::Xps::HPTPROVIDER, pbaseticket: ::windows::runtime::RawPtr, pdeltaticket: ::windows::runtime::RawPtr, scope: EPrintTicketScope, presultticket: ::windows::runtime::RawPtr, pbstrerrormessage: *mut ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
    pub fn PTOpenProvider(pszprintername: super::super::super::Foundation::PWSTR, dwversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
    pub fn PTOpenProviderEx(pszprintername: super::super::super::Foundation::PWSTR, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut super::super::super::Storage::Xps::HPTPROVIDER, pusedversion: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PTQuerySchemaVersionSupport(pszprintername: super::super::super::Foundation::PWSTR, pmaxversion: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`*"]
    pub fn PTReleaseMemory(pbuffer: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
}
