windows_link::link!("prntvpt.dll" "system" fn PTCloseProvider(hprovider : HPTPROVIDER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_wingdi"))]
windows_link::link!("prntvpt.dll" "system" fn PTConvertDevModeToPrintTicket(hprovider : HPTPROVIDER, cbdevmode : u32, pdevmode : super::wingdi::PDEVMODE, scope : EPrintTicketScope, pprintticket : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_wingdi"))]
windows_link::link!("prntvpt.dll" "system" fn PTConvertPrintTicketToDevMode(hprovider : HPTPROVIDER, pprintticket : *mut core::ffi::c_void, basedevmodetype : EDefaultDevmodeType, scope : EPrintTicketScope, pcbdevmode : *mut u32, ppdevmode : *mut super::wingdi::PDEVMODE, pbstrerrormessage : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidlbase")]
windows_link::link!("prntvpt.dll" "system" fn PTGetPrintCapabilities(hprovider : HPTPROVIDER, pprintticket : *mut core::ffi::c_void, pcapabilities : *mut core::ffi::c_void, pbstrerrormessage : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidlbase")]
windows_link::link!("prntvpt.dll" "system" fn PTGetPrintDeviceCapabilities(hprovider : HPTPROVIDER, pprintticket : *mut core::ffi::c_void, pdevicecapabilities : *mut core::ffi::c_void, pbstrerrormessage : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidlbase")]
windows_link::link!("prntvpt.dll" "system" fn PTGetPrintDeviceResources(hprovider : HPTPROVIDER, pszlocalename : windows_sys::core::PCWSTR, pprintticket : *mut core::ffi::c_void, pdeviceresources : *mut core::ffi::c_void, pbstrerrormessage : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidlbase")]
windows_link::link!("prntvpt.dll" "system" fn PTMergeAndValidatePrintTicket(hprovider : HPTPROVIDER, pbaseticket : *mut core::ffi::c_void, pdeltaticket : *mut core::ffi::c_void, scope : EPrintTicketScope, presultticket : *mut core::ffi::c_void, pbstrerrormessage : *mut windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
windows_link::link!("prntvpt.dll" "system" fn PTOpenProvider(pszprintername : windows_sys::core::PCWSTR, dwversion : u32, phprovider : *mut HPTPROVIDER) -> windows_sys::core::HRESULT);
windows_link::link!("prntvpt.dll" "system" fn PTOpenProviderEx(pszprintername : windows_sys::core::PCWSTR, dwmaxversion : u32, dwprefversion : u32, phprovider : *mut HPTPROVIDER, pusedversion : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("prntvpt.dll" "system" fn PTQuerySchemaVersionSupport(pszprintername : windows_sys::core::PCWSTR, pmaxversion : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("prntvpt.dll" "system" fn PTReleaseMemory(pbuffer : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
pub type EDefaultDevmodeType = i32;
pub type EPrintTicketScope = i32;
pub const E_DELTA_PRINTTICKET_FORMAT: u32 = 2147745797;
pub const E_PRINTCAPABILITIES_FORMAT: u32 = 2147745796;
pub const E_PRINTDEVICECAPABILITIES_FORMAT: u32 = 2147745798;
pub const E_PRINTTICKET_FORMAT: u32 = 2147745795;
pub type HPTPROVIDER = *mut core::ffi::c_void;
pub const PRINTTICKET_ISTREAM_APIS: u32 = 1;
pub const S_PT_CONFLICT_RESOLVED: u32 = 262146;
pub const S_PT_NO_CONFLICT: u32 = 262145;
pub const kPTDocumentScope: EPrintTicketScope = 1;
pub const kPTJobScope: EPrintTicketScope = 2;
pub const kPTPageScope: EPrintTicketScope = 0;
pub const kPrinterDefaultDevmode: EDefaultDevmodeType = 1;
pub const kUserDefaultDevmode: EDefaultDevmodeType = 0;
