#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Storage_Xps`*"]
    #[cfg(feature = "Win32_Storage_Xps")]
    pub fn PTCloseProvider();
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
    pub fn PTConvertDevModeToPrintTicket();
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
    pub fn PTConvertPrintTicketToDevMode();
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
    pub fn PTGetPrintCapabilities();
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
    pub fn PTGetPrintDeviceCapabilities();
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
    pub fn PTGetPrintDeviceResources();
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps", feature = "Win32_System_Com"))]
    pub fn PTMergeAndValidatePrintTicket();
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
    pub fn PTOpenProvider();
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`, `Win32_Storage_Xps`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
    pub fn PTOpenProviderEx();
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PTQuerySchemaVersionSupport();
    #[doc = "*Required features: `Win32_Graphics_Printing_PrintTicket`*"]
    pub fn PTReleaseMemory();
}
