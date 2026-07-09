#[cfg(feature = "Win32_wsdxml")]
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider<P0>(pcontext: P0) -> windows_core::Result<IWSDiscoveryProvider>
where
    P0: windows_core::Param<super::wsdxml::IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider(pcontext : *mut core::ffi::c_void, ppprovider : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDiscoveryProvider(pcontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
#[inline]
pub unsafe fn WSDCreateDiscoveryProvider2<P0>(pcontext: P0, pconfigparams: Option<&[super::wsdbase::WSD_CONFIG_PARAM]>) -> windows_core::Result<IWSDiscoveryProvider>
where
    P0: windows_core::Param<super::wsdxml::IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryProvider2(pcontext : *mut core::ffi::c_void, pconfigparams : *const super::wsdbase::WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppprovider : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDiscoveryProvider2(pcontext.param().abi(), core::mem::transmute(pconfigparams.map_or(core::ptr::null(), |slice| slice.as_ptr())), pconfigparams.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "Win32_wsdxml")]
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher<P0>(pcontext: P0) -> windows_core::Result<IWSDiscoveryPublisher>
where
    P0: windows_core::Param<super::wsdxml::IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher(pcontext : *mut core::ffi::c_void, pppublisher : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDiscoveryPublisher(pcontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
#[inline]
pub unsafe fn WSDCreateDiscoveryPublisher2<P0>(pcontext: P0, pconfigparams: Option<&[super::wsdbase::WSD_CONFIG_PARAM]>) -> windows_core::Result<IWSDiscoveryPublisher>
where
    P0: windows_core::Param<super::wsdxml::IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDiscoveryPublisher2(pcontext : *mut core::ffi::c_void, pconfigparams : *const super::wsdbase::WSD_CONFIG_PARAM, dwconfigparamcount : u32, pppublisher : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDiscoveryPublisher2(pcontext.param().abi(), core::mem::transmute(pconfigparams.map_or(core::ptr::null(), |slice| slice.as_ptr())), pconfigparams.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
windows_core::imp::define_interface!(IWSDScopeMatchingRule, IWSDScopeMatchingRule_Vtbl, 0xfcafe424_fef5_481a_bd9f_33ce0574256f);
windows_core::imp::interface_hierarchy!(IWSDScopeMatchingRule, windows_core::IUnknown);
impl IWSDScopeMatchingRule {
    pub unsafe fn GetScopeRule(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScopeRule)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MatchScopes<P0, P1>(&self, pszscope1: P0, pszscope2: P1) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MatchScopes)(windows_core::Interface::as_raw(self), pszscope1.param().abi(), pszscope2.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDScopeMatchingRule_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetScopeRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub MatchScopes: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWSDScopeMatchingRule_Impl: windows_core::IUnknownImpl {
    fn GetScopeRule(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn MatchScopes(&self, pszscope1: &windows_core::PCWSTR, pszscope2: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
}
impl IWSDScopeMatchingRule_Vtbl {
    pub const fn new<Identity: IWSDScopeMatchingRule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetScopeRule<Identity: IWSDScopeMatchingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszscopematchingrule: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDScopeMatchingRule_Impl::GetScopeRule(this) {
                    Ok(ok__) => {
                        ppszscopematchingrule.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MatchScopes<Identity: IWSDScopeMatchingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszscope1: windows_core::PCWSTR, pszscope2: windows_core::PCWSTR, pfmatch: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDScopeMatchingRule_Impl::MatchScopes(this, core::mem::transmute(&pszscope1), core::mem::transmute(&pszscope2)) {
                    Ok(ok__) => {
                        pfmatch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetScopeRule: GetScopeRule::<Identity, OFFSET>,
            MatchScopes: MatchScopes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDScopeMatchingRule as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDScopeMatchingRule {}
windows_core::imp::define_interface!(IWSDiscoveredService, IWSDiscoveredService_Vtbl, 0x4bad8a3b_b374_4420_9632_aac945b374aa);
windows_core::imp::interface_hierarchy!(IWSDiscoveredService, windows_core::IUnknown);
impl IWSDiscoveredService {
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn GetEndpointReference(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_ENDPOINT_REFERENCE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEndpointReference)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn GetTypes(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_NAME_LIST> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wsdtypes")]
    pub unsafe fn GetScopes(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_URI_LIST> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScopes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wsdtypes")]
    pub unsafe fn GetXAddrs(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_URI_LIST> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXAddrs)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMetadataVersion(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wsdxmldom")]
    pub unsafe fn GetExtendedDiscoXML(&self, ppheaderany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT, ppbodyany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExtendedDiscoXML)(windows_core::Interface::as_raw(self), ppheaderany as _, ppbodyany as _) }
    }
    pub unsafe fn GetProbeResolveTag(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProbeResolveTag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRemoteTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRemoteTransportAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLocalTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalTransportAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLocalInterfaceGUID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalInterfaceGUID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInstanceId(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInstanceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveredService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub GetEndpointReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_ENDPOINT_REFERENCE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    GetEndpointReference: usize,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub GetTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_NAME_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    GetTypes: usize,
    #[cfg(feature = "Win32_wsdtypes")]
    pub GetScopes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_URI_LIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wsdtypes"))]
    GetScopes: usize,
    #[cfg(feature = "Win32_wsdtypes")]
    pub GetXAddrs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_URI_LIST) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wsdtypes"))]
    GetXAddrs: usize,
    pub GetMetadataVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wsdxmldom")]
    pub GetExtendedDiscoXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::wsdxmldom::WSDXML_ELEMENT, *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wsdxmldom"))]
    GetExtendedDiscoXML: usize,
    pub GetProbeResolveTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetRemoteTransportAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetLocalTransportAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetLocalInterfaceGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetInstanceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
pub trait IWSDiscoveredService_Impl: windows_core::IUnknownImpl {
    fn GetEndpointReference(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_ENDPOINT_REFERENCE>;
    fn GetTypes(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_NAME_LIST>;
    fn GetScopes(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_URI_LIST>;
    fn GetXAddrs(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_URI_LIST>;
    fn GetMetadataVersion(&self) -> windows_core::Result<u64>;
    fn GetExtendedDiscoXML(&self, ppheaderany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT, ppbodyany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn GetProbeResolveTag(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetRemoteTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetLocalTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetLocalInterfaceGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetInstanceId(&self) -> windows_core::Result<u64>;
}
#[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl IWSDiscoveredService_Vtbl {
    pub const fn new<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEndpointReference<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppendpointreference: *mut *mut super::wsdtypes::WSD_ENDPOINT_REFERENCE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetEndpointReference(this) {
                    Ok(ok__) => {
                        ppendpointreference.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTypes<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptypeslist: *mut *mut super::wsdtypes::WSD_NAME_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetTypes(this) {
                    Ok(ok__) => {
                        pptypeslist.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScopes<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppscopeslist: *mut *mut super::wsdtypes::WSD_URI_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetScopes(this) {
                    Ok(ok__) => {
                        ppscopeslist.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetXAddrs<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppxaddrslist: *mut *mut super::wsdtypes::WSD_URI_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetXAddrs(this) {
                    Ok(ok__) => {
                        ppxaddrslist.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMetadataVersion<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullmetadataversion: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetMetadataVersion(this) {
                    Ok(ok__) => {
                        pullmetadataversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExtendedDiscoXML<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppheaderany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT, ppbodyany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveredService_Impl::GetExtendedDiscoXML(this, core::mem::transmute_copy(&ppheaderany), core::mem::transmute_copy(&ppbodyany)).into()
            }
        }
        unsafe extern "system" fn GetProbeResolveTag<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsztag: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetProbeResolveTag(this) {
                    Ok(ok__) => {
                        ppsztag.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRemoteTransportAddress<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszremotetransportaddress: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetRemoteTransportAddress(this) {
                    Ok(ok__) => {
                        ppszremotetransportaddress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocalTransportAddress<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszlocaltransportaddress: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetLocalTransportAddress(this) {
                    Ok(ok__) => {
                        ppszlocaltransportaddress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLocalInterfaceGUID<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetLocalInterfaceGUID(this) {
                    Ok(ok__) => {
                        pguid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInstanceId<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullinstanceid: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveredService_Impl::GetInstanceId(this) {
                    Ok(ok__) => {
                        pullinstanceid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEndpointReference: GetEndpointReference::<Identity, OFFSET>,
            GetTypes: GetTypes::<Identity, OFFSET>,
            GetScopes: GetScopes::<Identity, OFFSET>,
            GetXAddrs: GetXAddrs::<Identity, OFFSET>,
            GetMetadataVersion: GetMetadataVersion::<Identity, OFFSET>,
            GetExtendedDiscoXML: GetExtendedDiscoXML::<Identity, OFFSET>,
            GetProbeResolveTag: GetProbeResolveTag::<Identity, OFFSET>,
            GetRemoteTransportAddress: GetRemoteTransportAddress::<Identity, OFFSET>,
            GetLocalTransportAddress: GetLocalTransportAddress::<Identity, OFFSET>,
            GetLocalInterfaceGUID: GetLocalInterfaceGUID::<Identity, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveredService as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl windows_core::RuntimeName for IWSDiscoveredService {}
windows_core::imp::define_interface!(IWSDiscoveryProvider, IWSDiscoveryProvider_Vtbl, 0x8ffc8e55_f0eb_480f_88b7_b435dd281d45);
windows_core::imp::interface_hierarchy!(IWSDiscoveryProvider, windows_core::IUnknown);
impl IWSDiscoveryProvider {
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAddressFamily)(windows_core::Interface::as_raw(self), dwaddressfamily) }
    }
    pub unsafe fn Attach<P0>(&self, psink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDiscoveryProviderNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).Attach)(windows_core::Interface::as_raw(self), psink.param().abi()) }
    }
    pub unsafe fn Detach(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Detach)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SearchById<P0, P1>(&self, pszid: P0, psztag: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SearchById)(windows_core::Interface::as_raw(self), pszid.param().abi(), psztag.param().abi()) }
    }
    pub unsafe fn SearchByAddress<P0, P1>(&self, pszaddress: P0, psztag: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SearchByAddress)(windows_core::Interface::as_raw(self), pszaddress.param().abi(), psztag.param().abi()) }
    }
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn SearchByType<P2, P3>(&self, ptypeslist: Option<*const super::wsdtypes::WSD_NAME_LIST>, pscopeslist: Option<*const super::wsdtypes::WSD_URI_LIST>, pszmatchby: P2, psztag: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SearchByType)(windows_core::Interface::as_raw(self), ptypeslist.unwrap_or(core::mem::zeroed()) as _, pscopeslist.unwrap_or(core::mem::zeroed()) as _, pszmatchby.param().abi(), psztag.param().abi()) }
    }
    #[cfg(feature = "Win32_wsdxml")]
    pub unsafe fn GetXMLContext(&self) -> windows_core::Result<super::wsdxml::IWSDXMLContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXMLContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAddressFamily: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Attach: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Detach: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchById: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SearchByAddress: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub SearchByType: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_NAME_LIST, *const super::wsdtypes::WSD_URI_LIST, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    SearchByType: usize,
    #[cfg(feature = "Win32_wsdxml")]
    pub GetXMLContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wsdxml"))]
    GetXMLContext: usize,
}
#[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
pub trait IWSDiscoveryProvider_Impl: windows_core::IUnknownImpl {
    fn SetAddressFamily(&self, dwaddressfamily: u32) -> windows_core::Result<()>;
    fn Attach(&self, psink: windows_core::Ref<IWSDiscoveryProviderNotify>) -> windows_core::Result<()>;
    fn Detach(&self) -> windows_core::Result<()>;
    fn SearchById(&self, pszid: &windows_core::PCWSTR, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SearchByAddress(&self, pszaddress: &windows_core::PCWSTR, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SearchByType(&self, ptypeslist: *const super::wsdtypes::WSD_NAME_LIST, pscopeslist: *const super::wsdtypes::WSD_URI_LIST, pszmatchby: &windows_core::PCWSTR, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetXMLContext(&self) -> windows_core::Result<super::wsdxml::IWSDXMLContext>;
}
#[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl IWSDiscoveryProvider_Vtbl {
    pub const fn new<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAddressFamily<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaddressfamily: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProvider_Impl::SetAddressFamily(this, core::mem::transmute_copy(&dwaddressfamily)).into()
            }
        }
        unsafe extern "system" fn Attach<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProvider_Impl::Attach(this, core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn Detach<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProvider_Impl::Detach(this).into()
            }
        }
        unsafe extern "system" fn SearchById<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProvider_Impl::SearchById(this, core::mem::transmute(&pszid), core::mem::transmute(&psztag)).into()
            }
        }
        unsafe extern "system" fn SearchByAddress<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszaddress: windows_core::PCWSTR, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProvider_Impl::SearchByAddress(this, core::mem::transmute(&pszaddress), core::mem::transmute(&psztag)).into()
            }
        }
        unsafe extern "system" fn SearchByType<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypeslist: *const super::wsdtypes::WSD_NAME_LIST, pscopeslist: *const super::wsdtypes::WSD_URI_LIST, pszmatchby: windows_core::PCWSTR, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProvider_Impl::SearchByType(this, core::mem::transmute_copy(&ptypeslist), core::mem::transmute_copy(&pscopeslist), core::mem::transmute(&pszmatchby), core::mem::transmute(&psztag)).into()
            }
        }
        unsafe extern "system" fn GetXMLContext<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveryProvider_Impl::GetXMLContext(this) {
                    Ok(ok__) => {
                        ppcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Identity, OFFSET>,
            Attach: Attach::<Identity, OFFSET>,
            Detach: Detach::<Identity, OFFSET>,
            SearchById: SearchById::<Identity, OFFSET>,
            SearchByAddress: SearchByAddress::<Identity, OFFSET>,
            SearchByType: SearchByType::<Identity, OFFSET>,
            GetXMLContext: GetXMLContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl windows_core::RuntimeName for IWSDiscoveryProvider {}
windows_core::imp::define_interface!(IWSDiscoveryProviderNotify, IWSDiscoveryProviderNotify_Vtbl, 0x73ee3ced_b6e6_4329_a546_3e8ad46563d2);
windows_core::imp::interface_hierarchy!(IWSDiscoveryProviderNotify, windows_core::IUnknown);
impl IWSDiscoveryProviderNotify {
    pub unsafe fn Add<P0>(&self, pservice: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDiscoveredService>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), pservice.param().abi()) }
    }
    pub unsafe fn Remove<P0>(&self, pservice: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDiscoveredService>,
    {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), pservice.param().abi()) }
    }
    pub unsafe fn SearchFailed<P1>(&self, hr: windows_core::HRESULT, psztag: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SearchFailed)(windows_core::Interface::as_raw(self), hr, psztag.param().abi()) }
    }
    pub unsafe fn SearchComplete<P0>(&self, psztag: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SearchComplete)(windows_core::Interface::as_raw(self), psztag.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryProviderNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SearchFailed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SearchComplete: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IWSDiscoveryProviderNotify_Impl: windows_core::IUnknownImpl {
    fn Add(&self, pservice: windows_core::Ref<IWSDiscoveredService>) -> windows_core::Result<()>;
    fn Remove(&self, pservice: windows_core::Ref<IWSDiscoveredService>) -> windows_core::Result<()>;
    fn SearchFailed(&self, hr: windows_core::HRESULT, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SearchComplete(&self, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IWSDiscoveryProviderNotify_Vtbl {
    pub const fn new<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Add<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProviderNotify_Impl::Add(this, core::mem::transmute_copy(&pservice)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProviderNotify_Impl::Remove(this, core::mem::transmute_copy(&pservice)).into()
            }
        }
        unsafe extern "system" fn SearchFailed<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProviderNotify_Impl::SearchFailed(this, core::mem::transmute_copy(&hr), core::mem::transmute(&psztag)).into()
            }
        }
        unsafe extern "system" fn SearchComplete<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryProviderNotify_Impl::SearchComplete(this, core::mem::transmute(&psztag)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            SearchFailed: SearchFailed::<Identity, OFFSET>,
            SearchComplete: SearchComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryProviderNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDiscoveryProviderNotify {}
windows_core::imp::define_interface!(IWSDiscoveryPublisher, IWSDiscoveryPublisher_Vtbl, 0xae01e1a8_3ff9_4148_8116_057cc616fe13);
windows_core::imp::interface_hierarchy!(IWSDiscoveryPublisher, windows_core::IUnknown);
impl IWSDiscoveryPublisher {
    pub unsafe fn SetAddressFamily(&self, dwaddressfamily: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAddressFamily)(windows_core::Interface::as_raw(self), dwaddressfamily) }
    }
    pub unsafe fn RegisterNotificationSink<P0>(&self, psink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDiscoveryPublisherNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterNotificationSink)(windows_core::Interface::as_raw(self), psink.param().abi()) }
    }
    pub unsafe fn UnRegisterNotificationSink<P0>(&self, psink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDiscoveryPublisherNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterNotificationSink)(windows_core::Interface::as_raw(self), psink.param().abi()) }
    }
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn Publish<P0, P4>(&self, pszid: P0, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P4, ptypeslist: Option<*const super::wsdtypes::WSD_NAME_LIST>, pscopeslist: Option<*const super::wsdtypes::WSD_URI_LIST>, pxaddrslist: Option<*const super::wsdtypes::WSD_URI_LIST>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Publish)(windows_core::Interface::as_raw(self), pszid.param().abi(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.param().abi(), ptypeslist.unwrap_or(core::mem::zeroed()) as _, pscopeslist.unwrap_or(core::mem::zeroed()) as _, pxaddrslist.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "Win32_wsdxmldom")]
    pub unsafe fn UnPublish<P0, P3>(&self, pszid: P0, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P3, pany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnPublish)(windows_core::Interface::as_raw(self), pszid.param().abi(), ullinstanceid, ullmessagenumber, pszsessionid.param().abi(), pany.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn MatchProbe<P1, P2, P6>(&self, pprobemessage: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: P1, pszid: P2, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P6, ptypeslist: Option<*const super::wsdtypes::WSD_NAME_LIST>, pscopeslist: Option<*const super::wsdtypes::WSD_URI_LIST>, pxaddrslist: Option<*const super::wsdtypes::WSD_URI_LIST>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::wsdbase::IWSDMessageParameters>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).MatchProbe)(windows_core::Interface::as_raw(self), pprobemessage, pmessageparameters.param().abi(), pszid.param().abi(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.param().abi(), ptypeslist.unwrap_or(core::mem::zeroed()) as _, pscopeslist.unwrap_or(core::mem::zeroed()) as _, pxaddrslist.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn MatchResolve<P1, P2, P6>(&self, presolvemessage: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: P1, pszid: P2, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P6, ptypeslist: Option<*const super::wsdtypes::WSD_NAME_LIST>, pscopeslist: Option<*const super::wsdtypes::WSD_URI_LIST>, pxaddrslist: Option<*const super::wsdtypes::WSD_URI_LIST>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::wsdbase::IWSDMessageParameters>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).MatchResolve)(windows_core::Interface::as_raw(self), presolvemessage, pmessageparameters.param().abi(), pszid.param().abi(), ullmetadataversion, ullinstanceid, ullmessagenumber, pszsessionid.param().abi(), ptypeslist.unwrap_or(core::mem::zeroed()) as _, pscopeslist.unwrap_or(core::mem::zeroed()) as _, pxaddrslist.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn PublishEx<P0, P4>(&self, pszid: P0, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P4, ptypeslist: Option<*const super::wsdtypes::WSD_NAME_LIST>, pscopeslist: Option<*const super::wsdtypes::WSD_URI_LIST>, pxaddrslist: Option<*const super::wsdtypes::WSD_URI_LIST>, pheaderany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, preferenceparameterany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, ppolicyany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, pendpointreferenceany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, pany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PublishEx)(
                windows_core::Interface::as_raw(self),
                pszid.param().abi(),
                ullmetadataversion,
                ullinstanceid,
                ullmessagenumber,
                pszsessionid.param().abi(),
                ptypeslist.unwrap_or(core::mem::zeroed()) as _,
                pscopeslist.unwrap_or(core::mem::zeroed()) as _,
                pxaddrslist.unwrap_or(core::mem::zeroed()) as _,
                pheaderany.unwrap_or(core::mem::zeroed()) as _,
                preferenceparameterany.unwrap_or(core::mem::zeroed()) as _,
                ppolicyany.unwrap_or(core::mem::zeroed()) as _,
                pendpointreferenceany.unwrap_or(core::mem::zeroed()) as _,
                pany.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn MatchProbeEx<P1, P2, P6>(&self, pprobemessage: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: P1, pszid: P2, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P6, ptypeslist: Option<*const super::wsdtypes::WSD_NAME_LIST>, pscopeslist: Option<*const super::wsdtypes::WSD_URI_LIST>, pxaddrslist: Option<*const super::wsdtypes::WSD_URI_LIST>, pheaderany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, preferenceparameterany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, ppolicyany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, pendpointreferenceany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, pany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::wsdbase::IWSDMessageParameters>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).MatchProbeEx)(
                windows_core::Interface::as_raw(self),
                pprobemessage,
                pmessageparameters.param().abi(),
                pszid.param().abi(),
                ullmetadataversion,
                ullinstanceid,
                ullmessagenumber,
                pszsessionid.param().abi(),
                ptypeslist.unwrap_or(core::mem::zeroed()) as _,
                pscopeslist.unwrap_or(core::mem::zeroed()) as _,
                pxaddrslist.unwrap_or(core::mem::zeroed()) as _,
                pheaderany.unwrap_or(core::mem::zeroed()) as _,
                preferenceparameterany.unwrap_or(core::mem::zeroed()) as _,
                ppolicyany.unwrap_or(core::mem::zeroed()) as _,
                pendpointreferenceany.unwrap_or(core::mem::zeroed()) as _,
                pany.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn MatchResolveEx<P1, P2, P6>(&self, presolvemessage: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: P1, pszid: P2, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: P6, ptypeslist: Option<*const super::wsdtypes::WSD_NAME_LIST>, pscopeslist: Option<*const super::wsdtypes::WSD_URI_LIST>, pxaddrslist: Option<*const super::wsdtypes::WSD_URI_LIST>, pheaderany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, preferenceparameterany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, ppolicyany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, pendpointreferenceany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, pany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::wsdbase::IWSDMessageParameters>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).MatchResolveEx)(
                windows_core::Interface::as_raw(self),
                presolvemessage,
                pmessageparameters.param().abi(),
                pszid.param().abi(),
                ullmetadataversion,
                ullinstanceid,
                ullmessagenumber,
                pszsessionid.param().abi(),
                ptypeslist.unwrap_or(core::mem::zeroed()) as _,
                pscopeslist.unwrap_or(core::mem::zeroed()) as _,
                pxaddrslist.unwrap_or(core::mem::zeroed()) as _,
                pheaderany.unwrap_or(core::mem::zeroed()) as _,
                preferenceparameterany.unwrap_or(core::mem::zeroed()) as _,
                ppolicyany.unwrap_or(core::mem::zeroed()) as _,
                pendpointreferenceany.unwrap_or(core::mem::zeroed()) as _,
                pany.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn RegisterScopeMatchingRule<P0>(&self, pscopematchingrule: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDScopeMatchingRule>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterScopeMatchingRule)(windows_core::Interface::as_raw(self), pscopematchingrule.param().abi()) }
    }
    pub unsafe fn UnRegisterScopeMatchingRule<P0>(&self, pscopematchingrule: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDScopeMatchingRule>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnRegisterScopeMatchingRule)(windows_core::Interface::as_raw(self), pscopematchingrule.param().abi()) }
    }
    #[cfg(feature = "Win32_wsdxml")]
    pub unsafe fn GetXMLContext(&self) -> windows_core::Result<super::wsdxml::IWSDXMLContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetXMLContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisher_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAddressFamily: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RegisterNotificationSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnRegisterNotificationSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub Publish: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u64, u64, u64, windows_core::PCWSTR, *const super::wsdtypes::WSD_NAME_LIST, *const super::wsdtypes::WSD_URI_LIST, *const super::wsdtypes::WSD_URI_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    Publish: usize,
    #[cfg(feature = "Win32_wsdxmldom")]
    pub UnPublish: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u64, u64, windows_core::PCWSTR, *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wsdxmldom"))]
    UnPublish: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub MatchProbe: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_SOAP_MESSAGE, *mut core::ffi::c_void, windows_core::PCWSTR, u64, u64, u64, windows_core::PCWSTR, *const super::wsdtypes::WSD_NAME_LIST, *const super::wsdtypes::WSD_URI_LIST, *const super::wsdtypes::WSD_URI_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    MatchProbe: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub MatchResolve: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_SOAP_MESSAGE, *mut core::ffi::c_void, windows_core::PCWSTR, u64, u64, u64, windows_core::PCWSTR, *const super::wsdtypes::WSD_NAME_LIST, *const super::wsdtypes::WSD_URI_LIST, *const super::wsdtypes::WSD_URI_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    MatchResolve: usize,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub PublishEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u64, u64, u64, windows_core::PCWSTR, *const super::wsdtypes::WSD_NAME_LIST, *const super::wsdtypes::WSD_URI_LIST, *const super::wsdtypes::WSD_URI_LIST, *const super::wsdxmldom::WSDXML_ELEMENT, *const super::wsdxmldom::WSDXML_ELEMENT, *const super::wsdxmldom::WSDXML_ELEMENT, *const super::wsdxmldom::WSDXML_ELEMENT, *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    PublishEx: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub MatchProbeEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_SOAP_MESSAGE, *mut core::ffi::c_void, windows_core::PCWSTR, u64, u64, u64, windows_core::PCWSTR, *const super::wsdtypes::WSD_NAME_LIST, *const super::wsdtypes::WSD_URI_LIST, *const super::wsdtypes::WSD_URI_LIST, *const super::wsdxmldom::WSDXML_ELEMENT, *const super::wsdxmldom::WSDXML_ELEMENT, *const super::wsdxmldom::WSDXML_ELEMENT, *const super::wsdxmldom::WSDXML_ELEMENT, *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    MatchProbeEx: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub MatchResolveEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_SOAP_MESSAGE, *mut core::ffi::c_void, windows_core::PCWSTR, u64, u64, u64, windows_core::PCWSTR, *const super::wsdtypes::WSD_NAME_LIST, *const super::wsdtypes::WSD_URI_LIST, *const super::wsdtypes::WSD_URI_LIST, *const super::wsdxmldom::WSDXML_ELEMENT, *const super::wsdxmldom::WSDXML_ELEMENT, *const super::wsdxmldom::WSDXML_ELEMENT, *const super::wsdxmldom::WSDXML_ELEMENT, *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    MatchResolveEx: usize,
    pub RegisterScopeMatchingRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnRegisterScopeMatchingRule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wsdxml")]
    pub GetXMLContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wsdxml"))]
    GetXMLContext: usize,
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
pub trait IWSDiscoveryPublisher_Impl: windows_core::IUnknownImpl {
    fn SetAddressFamily(&self, dwaddressfamily: u32) -> windows_core::Result<()>;
    fn RegisterNotificationSink(&self, psink: windows_core::Ref<IWSDiscoveryPublisherNotify>) -> windows_core::Result<()>;
    fn UnRegisterNotificationSink(&self, psink: windows_core::Ref<IWSDiscoveryPublisherNotify>) -> windows_core::Result<()>;
    fn Publish(&self, pszid: &windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const super::wsdtypes::WSD_NAME_LIST, pscopeslist: *const super::wsdtypes::WSD_URI_LIST, pxaddrslist: *const super::wsdtypes::WSD_URI_LIST) -> windows_core::Result<()>;
    fn UnPublish(&self, pszid: &windows_core::PCWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, pany: *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn MatchProbe(&self, pprobemessage: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: windows_core::Ref<super::wsdbase::IWSDMessageParameters>, pszid: &windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const super::wsdtypes::WSD_NAME_LIST, pscopeslist: *const super::wsdtypes::WSD_URI_LIST, pxaddrslist: *const super::wsdtypes::WSD_URI_LIST) -> windows_core::Result<()>;
    fn MatchResolve(&self, presolvemessage: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: windows_core::Ref<super::wsdbase::IWSDMessageParameters>, pszid: &windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const super::wsdtypes::WSD_NAME_LIST, pscopeslist: *const super::wsdtypes::WSD_URI_LIST, pxaddrslist: *const super::wsdtypes::WSD_URI_LIST) -> windows_core::Result<()>;
    fn PublishEx(&self, pszid: &windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const super::wsdtypes::WSD_NAME_LIST, pscopeslist: *const super::wsdtypes::WSD_URI_LIST, pxaddrslist: *const super::wsdtypes::WSD_URI_LIST, pheaderany: *const super::wsdxmldom::WSDXML_ELEMENT, preferenceparameterany: *const super::wsdxmldom::WSDXML_ELEMENT, ppolicyany: *const super::wsdxmldom::WSDXML_ELEMENT, pendpointreferenceany: *const super::wsdxmldom::WSDXML_ELEMENT, pany: *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn MatchProbeEx(&self, pprobemessage: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: windows_core::Ref<super::wsdbase::IWSDMessageParameters>, pszid: &windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const super::wsdtypes::WSD_NAME_LIST, pscopeslist: *const super::wsdtypes::WSD_URI_LIST, pxaddrslist: *const super::wsdtypes::WSD_URI_LIST, pheaderany: *const super::wsdxmldom::WSDXML_ELEMENT, preferenceparameterany: *const super::wsdxmldom::WSDXML_ELEMENT, ppolicyany: *const super::wsdxmldom::WSDXML_ELEMENT, pendpointreferenceany: *const super::wsdxmldom::WSDXML_ELEMENT, pany: *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn MatchResolveEx(&self, presolvemessage: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: windows_core::Ref<super::wsdbase::IWSDMessageParameters>, pszid: &windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const super::wsdtypes::WSD_NAME_LIST, pscopeslist: *const super::wsdtypes::WSD_URI_LIST, pxaddrslist: *const super::wsdtypes::WSD_URI_LIST, pheaderany: *const super::wsdxmldom::WSDXML_ELEMENT, preferenceparameterany: *const super::wsdxmldom::WSDXML_ELEMENT, ppolicyany: *const super::wsdxmldom::WSDXML_ELEMENT, pendpointreferenceany: *const super::wsdxmldom::WSDXML_ELEMENT, pany: *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn RegisterScopeMatchingRule(&self, pscopematchingrule: windows_core::Ref<IWSDScopeMatchingRule>) -> windows_core::Result<()>;
    fn UnRegisterScopeMatchingRule(&self, pscopematchingrule: windows_core::Ref<IWSDScopeMatchingRule>) -> windows_core::Result<()>;
    fn GetXMLContext(&self) -> windows_core::Result<super::wsdxml::IWSDXMLContext>;
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl IWSDiscoveryPublisher_Vtbl {
    pub const fn new<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAddressFamily<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaddressfamily: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::SetAddressFamily(this, core::mem::transmute_copy(&dwaddressfamily)).into()
            }
        }
        unsafe extern "system" fn RegisterNotificationSink<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::RegisterNotificationSink(this, core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn UnRegisterNotificationSink<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::UnRegisterNotificationSink(this, core::mem::transmute_copy(&psink)).into()
            }
        }
        unsafe extern "system" fn Publish<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: windows_core::PCWSTR, ptypeslist: *const super::wsdtypes::WSD_NAME_LIST, pscopeslist: *const super::wsdtypes::WSD_URI_LIST, pxaddrslist: *const super::wsdtypes::WSD_URI_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::Publish(this, core::mem::transmute(&pszid), core::mem::transmute_copy(&ullmetadataversion), core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&ullmessagenumber), core::mem::transmute(&pszsessionid), core::mem::transmute_copy(&ptypeslist), core::mem::transmute_copy(&pscopeslist), core::mem::transmute_copy(&pxaddrslist)).into()
            }
        }
        unsafe extern "system" fn UnPublish<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: windows_core::PCWSTR, pany: *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::UnPublish(this, core::mem::transmute(&pszid), core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&ullmessagenumber), core::mem::transmute(&pszsessionid), core::mem::transmute_copy(&pany)).into()
            }
        }
        unsafe extern "system" fn MatchProbe<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprobemessage: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: windows_core::PCWSTR, ptypeslist: *const super::wsdtypes::WSD_NAME_LIST, pscopeslist: *const super::wsdtypes::WSD_URI_LIST, pxaddrslist: *const super::wsdtypes::WSD_URI_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::MatchProbe(this, core::mem::transmute_copy(&pprobemessage), core::mem::transmute_copy(&pmessageparameters), core::mem::transmute(&pszid), core::mem::transmute_copy(&ullmetadataversion), core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&ullmessagenumber), core::mem::transmute(&pszsessionid), core::mem::transmute_copy(&ptypeslist), core::mem::transmute_copy(&pscopeslist), core::mem::transmute_copy(&pxaddrslist)).into()
            }
        }
        unsafe extern "system" fn MatchResolve<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presolvemessage: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: windows_core::PCWSTR, ptypeslist: *const super::wsdtypes::WSD_NAME_LIST, pscopeslist: *const super::wsdtypes::WSD_URI_LIST, pxaddrslist: *const super::wsdtypes::WSD_URI_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::MatchResolve(this, core::mem::transmute_copy(&presolvemessage), core::mem::transmute_copy(&pmessageparameters), core::mem::transmute(&pszid), core::mem::transmute_copy(&ullmetadataversion), core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&ullmessagenumber), core::mem::transmute(&pszsessionid), core::mem::transmute_copy(&ptypeslist), core::mem::transmute_copy(&pscopeslist), core::mem::transmute_copy(&pxaddrslist)).into()
            }
        }
        unsafe extern "system" fn PublishEx<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: windows_core::PCWSTR, ptypeslist: *const super::wsdtypes::WSD_NAME_LIST, pscopeslist: *const super::wsdtypes::WSD_URI_LIST, pxaddrslist: *const super::wsdtypes::WSD_URI_LIST, pheaderany: *const super::wsdxmldom::WSDXML_ELEMENT, preferenceparameterany: *const super::wsdxmldom::WSDXML_ELEMENT, ppolicyany: *const super::wsdxmldom::WSDXML_ELEMENT, pendpointreferenceany: *const super::wsdxmldom::WSDXML_ELEMENT, pany: *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::PublishEx(
                    this,
                    core::mem::transmute(&pszid),
                    core::mem::transmute_copy(&ullmetadataversion),
                    core::mem::transmute_copy(&ullinstanceid),
                    core::mem::transmute_copy(&ullmessagenumber),
                    core::mem::transmute(&pszsessionid),
                    core::mem::transmute_copy(&ptypeslist),
                    core::mem::transmute_copy(&pscopeslist),
                    core::mem::transmute_copy(&pxaddrslist),
                    core::mem::transmute_copy(&pheaderany),
                    core::mem::transmute_copy(&preferenceparameterany),
                    core::mem::transmute_copy(&ppolicyany),
                    core::mem::transmute_copy(&pendpointreferenceany),
                    core::mem::transmute_copy(&pany),
                )
                .into()
            }
        }
        unsafe extern "system" fn MatchProbeEx<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pprobemessage: *const super::wsdtypes::WSD_SOAP_MESSAGE,
            pmessageparameters: *mut core::ffi::c_void,
            pszid: windows_core::PCWSTR,
            ullmetadataversion: u64,
            ullinstanceid: u64,
            ullmessagenumber: u64,
            pszsessionid: windows_core::PCWSTR,
            ptypeslist: *const super::wsdtypes::WSD_NAME_LIST,
            pscopeslist: *const super::wsdtypes::WSD_URI_LIST,
            pxaddrslist: *const super::wsdtypes::WSD_URI_LIST,
            pheaderany: *const super::wsdxmldom::WSDXML_ELEMENT,
            preferenceparameterany: *const super::wsdxmldom::WSDXML_ELEMENT,
            ppolicyany: *const super::wsdxmldom::WSDXML_ELEMENT,
            pendpointreferenceany: *const super::wsdxmldom::WSDXML_ELEMENT,
            pany: *const super::wsdxmldom::WSDXML_ELEMENT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::MatchProbeEx(
                    this,
                    core::mem::transmute_copy(&pprobemessage),
                    core::mem::transmute_copy(&pmessageparameters),
                    core::mem::transmute(&pszid),
                    core::mem::transmute_copy(&ullmetadataversion),
                    core::mem::transmute_copy(&ullinstanceid),
                    core::mem::transmute_copy(&ullmessagenumber),
                    core::mem::transmute(&pszsessionid),
                    core::mem::transmute_copy(&ptypeslist),
                    core::mem::transmute_copy(&pscopeslist),
                    core::mem::transmute_copy(&pxaddrslist),
                    core::mem::transmute_copy(&pheaderany),
                    core::mem::transmute_copy(&preferenceparameterany),
                    core::mem::transmute_copy(&ppolicyany),
                    core::mem::transmute_copy(&pendpointreferenceany),
                    core::mem::transmute_copy(&pany),
                )
                .into()
            }
        }
        unsafe extern "system" fn MatchResolveEx<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            presolvemessage: *const super::wsdtypes::WSD_SOAP_MESSAGE,
            pmessageparameters: *mut core::ffi::c_void,
            pszid: windows_core::PCWSTR,
            ullmetadataversion: u64,
            ullinstanceid: u64,
            ullmessagenumber: u64,
            pszsessionid: windows_core::PCWSTR,
            ptypeslist: *const super::wsdtypes::WSD_NAME_LIST,
            pscopeslist: *const super::wsdtypes::WSD_URI_LIST,
            pxaddrslist: *const super::wsdtypes::WSD_URI_LIST,
            pheaderany: *const super::wsdxmldom::WSDXML_ELEMENT,
            preferenceparameterany: *const super::wsdxmldom::WSDXML_ELEMENT,
            ppolicyany: *const super::wsdxmldom::WSDXML_ELEMENT,
            pendpointreferenceany: *const super::wsdxmldom::WSDXML_ELEMENT,
            pany: *const super::wsdxmldom::WSDXML_ELEMENT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::MatchResolveEx(
                    this,
                    core::mem::transmute_copy(&presolvemessage),
                    core::mem::transmute_copy(&pmessageparameters),
                    core::mem::transmute(&pszid),
                    core::mem::transmute_copy(&ullmetadataversion),
                    core::mem::transmute_copy(&ullinstanceid),
                    core::mem::transmute_copy(&ullmessagenumber),
                    core::mem::transmute(&pszsessionid),
                    core::mem::transmute_copy(&ptypeslist),
                    core::mem::transmute_copy(&pscopeslist),
                    core::mem::transmute_copy(&pxaddrslist),
                    core::mem::transmute_copy(&pheaderany),
                    core::mem::transmute_copy(&preferenceparameterany),
                    core::mem::transmute_copy(&ppolicyany),
                    core::mem::transmute_copy(&pendpointreferenceany),
                    core::mem::transmute_copy(&pany),
                )
                .into()
            }
        }
        unsafe extern "system" fn RegisterScopeMatchingRule<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscopematchingrule: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::RegisterScopeMatchingRule(this, core::mem::transmute_copy(&pscopematchingrule)).into()
            }
        }
        unsafe extern "system" fn UnRegisterScopeMatchingRule<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscopematchingrule: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisher_Impl::UnRegisterScopeMatchingRule(this, core::mem::transmute_copy(&pscopematchingrule)).into()
            }
        }
        unsafe extern "system" fn GetXMLContext<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDiscoveryPublisher_Impl::GetXMLContext(this) {
                    Ok(ok__) => {
                        ppcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Identity, OFFSET>,
            RegisterNotificationSink: RegisterNotificationSink::<Identity, OFFSET>,
            UnRegisterNotificationSink: UnRegisterNotificationSink::<Identity, OFFSET>,
            Publish: Publish::<Identity, OFFSET>,
            UnPublish: UnPublish::<Identity, OFFSET>,
            MatchProbe: MatchProbe::<Identity, OFFSET>,
            MatchResolve: MatchResolve::<Identity, OFFSET>,
            PublishEx: PublishEx::<Identity, OFFSET>,
            MatchProbeEx: MatchProbeEx::<Identity, OFFSET>,
            MatchResolveEx: MatchResolveEx::<Identity, OFFSET>,
            RegisterScopeMatchingRule: RegisterScopeMatchingRule::<Identity, OFFSET>,
            UnRegisterScopeMatchingRule: UnRegisterScopeMatchingRule::<Identity, OFFSET>,
            GetXMLContext: GetXMLContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryPublisher as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl windows_core::RuntimeName for IWSDiscoveryPublisher {}
windows_core::imp::define_interface!(IWSDiscoveryPublisherNotify, IWSDiscoveryPublisherNotify_Vtbl, 0xe67651b0_337a_4b3c_9758_733388568251);
windows_core::imp::interface_hierarchy!(IWSDiscoveryPublisherNotify, windows_core::IUnknown);
impl IWSDiscoveryPublisherNotify {
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn ProbeHandler<P1>(&self, psoap: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::wsdbase::IWSDMessageParameters>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProbeHandler)(windows_core::Interface::as_raw(self), psoap, pmessageparameters.param().abi()) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn ResolveHandler<P1>(&self, psoap: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::wsdbase::IWSDMessageParameters>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResolveHandler)(windows_core::Interface::as_raw(self), psoap, pmessageparameters.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDiscoveryPublisherNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub ProbeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_SOAP_MESSAGE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    ProbeHandler: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub ResolveHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_SOAP_MESSAGE, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    ResolveHandler: usize,
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
pub trait IWSDiscoveryPublisherNotify_Impl: windows_core::IUnknownImpl {
    fn ProbeHandler(&self, psoap: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: windows_core::Ref<super::wsdbase::IWSDMessageParameters>) -> windows_core::Result<()>;
    fn ResolveHandler(&self, psoap: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: windows_core::Ref<super::wsdbase::IWSDMessageParameters>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl IWSDiscoveryPublisherNotify_Vtbl {
    pub const fn new<Identity: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProbeHandler<Identity: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psoap: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisherNotify_Impl::ProbeHandler(this, core::mem::transmute_copy(&psoap), core::mem::transmute_copy(&pmessageparameters)).into()
            }
        }
        unsafe extern "system" fn ResolveHandler<Identity: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psoap: *const super::wsdtypes::WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDiscoveryPublisherNotify_Impl::ResolveHandler(this, core::mem::transmute_copy(&psoap), core::mem::transmute_copy(&pmessageparameters)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProbeHandler: ProbeHandler::<Identity, OFFSET>,
            ResolveHandler: ResolveHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryPublisherNotify as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl windows_core::RuntimeName for IWSDiscoveryPublisherNotify {}
