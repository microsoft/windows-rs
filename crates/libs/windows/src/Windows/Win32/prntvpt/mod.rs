#[inline]
pub unsafe fn PTCloseProvider(hprovider: HPTPROVIDER) -> windows_core::HRESULT {
    windows_core::link!("prntvpt.dll" "system" fn PTCloseProvider(hprovider : HPTPROVIDER) -> windows_core::HRESULT);
    unsafe { PTCloseProvider(hprovider) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_wingdi"))]
#[inline]
pub unsafe fn PTConvertDevModeToPrintTicket(hprovider: HPTPROVIDER, cbdevmode: u32, pdevmode: super::wingdi::PDEVMODE, scope: EPrintTicketScope, pprintticket: &Option<super::objidlbase::IStream>) -> windows_core::HRESULT {
    windows_core::link!("prntvpt.dll" "system" fn PTConvertDevModeToPrintTicket(hprovider : HPTPROVIDER, cbdevmode : u32, pdevmode : super::wingdi::PDEVMODE, scope : EPrintTicketScope, pprintticket : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTConvertDevModeToPrintTicket(hprovider, cbdevmode, pdevmode, scope, core::mem::transmute_copy(pprintticket)) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_windef", feature = "Win32_wingdi"))]
#[inline]
pub unsafe fn PTConvertPrintTicketToDevMode<P1>(hprovider: HPTPROVIDER, pprintticket: P1, basedevmodetype: EDefaultDevmodeType, scope: EPrintTicketScope, pcbdevmode: *mut u32, ppdevmode: *mut super::wingdi::PDEVMODE, pbstrerrormessage: Option<*mut windows_core::BSTR>) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTConvertPrintTicketToDevMode(hprovider : HPTPROVIDER, pprintticket : *mut core::ffi::c_void, basedevmodetype : EDefaultDevmodeType, scope : EPrintTicketScope, pcbdevmode : *mut u32, ppdevmode : *mut super::wingdi::PDEVMODE, pbstrerrormessage : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTConvertPrintTicketToDevMode(hprovider, pprintticket.param().abi(), basedevmodetype, scope, pcbdevmode as _, ppdevmode as _, pbstrerrormessage.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn PTGetPrintCapabilities<P1>(hprovider: HPTPROVIDER, pprintticket: P1, pcapabilities: &Option<super::objidlbase::IStream>, pbstrerrormessage: Option<*mut windows_core::BSTR>) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTGetPrintCapabilities(hprovider : HPTPROVIDER, pprintticket : *mut core::ffi::c_void, pcapabilities : *mut core::ffi::c_void, pbstrerrormessage : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTGetPrintCapabilities(hprovider, pprintticket.param().abi(), core::mem::transmute_copy(pcapabilities), pbstrerrormessage.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn PTGetPrintDeviceCapabilities<P1>(hprovider: HPTPROVIDER, pprintticket: P1, pdevicecapabilities: &Option<super::objidlbase::IStream>, pbstrerrormessage: Option<*mut windows_core::BSTR>) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTGetPrintDeviceCapabilities(hprovider : HPTPROVIDER, pprintticket : *mut core::ffi::c_void, pdevicecapabilities : *mut core::ffi::c_void, pbstrerrormessage : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTGetPrintDeviceCapabilities(hprovider, pprintticket.param().abi(), core::mem::transmute_copy(pdevicecapabilities), pbstrerrormessage.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn PTGetPrintDeviceResources<P1, P2>(hprovider: HPTPROVIDER, pszlocalename: P1, pprintticket: P2, pdeviceresources: &Option<super::objidlbase::IStream>, pbstrerrormessage: Option<*mut windows_core::BSTR>) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTGetPrintDeviceResources(hprovider : HPTPROVIDER, pszlocalename : windows_core::PCWSTR, pprintticket : *mut core::ffi::c_void, pdeviceresources : *mut core::ffi::c_void, pbstrerrormessage : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTGetPrintDeviceResources(hprovider, pszlocalename.param().abi(), pprintticket.param().abi(), core::mem::transmute_copy(pdeviceresources), pbstrerrormessage.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn PTMergeAndValidatePrintTicket<P1, P2>(hprovider: HPTPROVIDER, pbaseticket: P1, pdeltaticket: P2, scope: EPrintTicketScope, presultticket: &Option<super::objidlbase::IStream>, pbstrerrormessage: Option<*mut windows_core::BSTR>) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::objidlbase::IStream>,
    P2: windows_core::Param<super::objidlbase::IStream>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTMergeAndValidatePrintTicket(hprovider : HPTPROVIDER, pbaseticket : *mut core::ffi::c_void, pdeltaticket : *mut core::ffi::c_void, scope : EPrintTicketScope, presultticket : *mut core::ffi::c_void, pbstrerrormessage : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTMergeAndValidatePrintTicket(hprovider, pbaseticket.param().abi(), pdeltaticket.param().abi(), scope, core::mem::transmute_copy(presultticket), pbstrerrormessage.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn PTOpenProvider<P0>(pszprintername: P0, dwversion: u32) -> windows_core::Result<HPTPROVIDER>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTOpenProvider(pszprintername : windows_core::PCWSTR, dwversion : u32, phprovider : *mut HPTPROVIDER) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PTOpenProvider(pszprintername.param().abi(), dwversion, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn PTOpenProviderEx<P0>(pszprintername: P0, dwmaxversion: u32, dwprefversion: u32, phprovider: *mut HPTPROVIDER, pusedversion: *mut u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTOpenProviderEx(pszprintername : windows_core::PCWSTR, dwmaxversion : u32, dwprefversion : u32, phprovider : *mut HPTPROVIDER, pusedversion : *mut u32) -> windows_core::HRESULT);
    unsafe { PTOpenProviderEx(pszprintername.param().abi(), dwmaxversion, dwprefversion, phprovider as _, pusedversion as _) }
}
#[inline]
pub unsafe fn PTQuerySchemaVersionSupport<P0>(pszprintername: P0) -> windows_core::Result<u32>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("prntvpt.dll" "system" fn PTQuerySchemaVersionSupport(pszprintername : windows_core::PCWSTR, pmaxversion : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        PTQuerySchemaVersionSupport(pszprintername.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn PTReleaseMemory(pbuffer: *const core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("prntvpt.dll" "system" fn PTReleaseMemory(pbuffer : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { PTReleaseMemory(pbuffer) }
}
pub type EDefaultDevmodeType = i32;
pub type EPrintTicketScope = i32;
pub const E_DELTA_PRINTTICKET_FORMAT: u32 = 2147745797;
pub const E_PRINTCAPABILITIES_FORMAT: u32 = 2147745796;
pub const E_PRINTDEVICECAPABILITIES_FORMAT: u32 = 2147745798;
pub const E_PRINTTICKET_FORMAT: u32 = 2147745795;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HPTPROVIDER(pub *mut core::ffi::c_void);
impl HPTPROVIDER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HPTPROVIDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTTICKET_ISTREAM_APIS: u32 = 1;
pub const S_PT_CONFLICT_RESOLVED: u32 = 262146;
pub const S_PT_NO_CONFLICT: u32 = 262145;
pub const kPTDocumentScope: EPrintTicketScope = 1;
pub const kPTJobScope: EPrintTicketScope = 2;
pub const kPTPageScope: EPrintTicketScope = 0;
pub const kPrinterDefaultDevmode: EDefaultDevmodeType = 1;
pub const kUserDefaultDevmode: EDefaultDevmodeType = 0;
