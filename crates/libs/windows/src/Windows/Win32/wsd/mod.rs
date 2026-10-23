#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WSDAllocateLinkedMemory(pparent: *mut core::ffi::c_void, cbsize: super::size_t) -> *mut core::ffi::c_void {
    windows_core::link!("wsdapi.dll" "system" fn WSDAllocateLinkedMemory(pparent : *mut core::ffi::c_void, cbsize : super::size_t) -> *mut core::ffi::c_void);
    unsafe { WSDAllocateLinkedMemory(pparent as _, cbsize) }
}
#[inline]
pub unsafe fn WSDAttachLinkedMemory(pparent: *mut core::ffi::c_void, pchild: *mut core::ffi::c_void) {
    windows_core::link!("wsdapi.dll" "system" fn WSDAttachLinkedMemory(pparent : *mut core::ffi::c_void, pchild : *mut core::ffi::c_void));
    unsafe { WSDAttachLinkedMemory(pparent as _, pchild as _) }
}
#[inline]
pub unsafe fn WSDCreateHttpAddress() -> windows_core::Result<IWSDHttpAddress> {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateHttpAddress(ppaddress : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateHttpAddress(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateHttpMessageParameters() -> windows_core::Result<IWSDHttpMessageParameters> {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateHttpMessageParameters(pptxparams : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateHttpMessageParameters(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateOutboundAttachment() -> windows_core::Result<IWSDOutboundAttachment> {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateOutboundAttachment(ppattachment : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateOutboundAttachment(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateUdpAddress() -> windows_core::Result<IWSDUdpAddress> {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateUdpAddress(ppaddress : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateUdpAddress(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateUdpMessageParameters(pptxparams: *mut Option<IWSDUdpMessageParameters>) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateUdpMessageParameters(pptxparams : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WSDCreateUdpMessageParameters(pptxparams as _) }
}
#[inline]
pub unsafe fn WSDDetachLinkedMemory(pvoid: *mut core::ffi::c_void) {
    windows_core::link!("wsdapi.dll" "system" fn WSDDetachLinkedMemory(pvoid : *mut core::ffi::c_void));
    unsafe { WSDDetachLinkedMemory(pvoid as _) }
}
#[inline]
pub unsafe fn WSDFreeLinkedMemory(pvoid: *mut core::ffi::c_void) {
    windows_core::link!("wsdapi.dll" "system" fn WSDFreeLinkedMemory(pvoid : *mut core::ffi::c_void));
    unsafe { WSDFreeLinkedMemory(pvoid as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSDGetConfigurationOption(dwoption: super::DWORD, pvoid: super::LPVOID, cboutbuffer: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDGetConfigurationOption(dwoption : super::DWORD, pvoid : super::LPVOID, cboutbuffer : super::DWORD) -> windows_core::HRESULT);
    unsafe { WSDGetConfigurationOption(dwoption, pvoid as _, cboutbuffer) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WSDSetConfigurationOption(dwoption: super::DWORD, pvoid: super::LPVOID, cbinbuffer: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDSetConfigurationOption(dwoption : super::DWORD, pvoid : super::LPVOID, cbinbuffer : super::DWORD) -> windows_core::HRESULT);
    unsafe { WSDSetConfigurationOption(dwoption, pvoid, cbinbuffer) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WSDUriDecode(source: &[u16], destout: *mut windows_core::PWSTR, cchdestout: Option<*mut super::DWORD>) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDUriDecode(source : windows_core::PCWSTR, cchsource : super::DWORD, destout : *mut windows_core::PWSTR, cchdestout : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { WSDUriDecode(core::mem::transmute(source.as_ptr()), source.len().try_into().unwrap(), destout as _, cchdestout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WSDUriEncode(source: &[u16], destout: *mut windows_core::PWSTR, cchdestout: Option<*mut super::DWORD>) -> windows_core::HRESULT {
    windows_core::link!("wsdapi.dll" "system" fn WSDUriEncode(source : windows_core::PCWSTR, cchsource : super::DWORD, destout : *mut windows_core::PWSTR, cchdestout : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { WSDUriEncode(core::mem::transmute(source.as_ptr()), source.len().try_into().unwrap(), destout as _, cchdestout.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WSDXMLCreateContext() -> windows_core::Result<IWSDXMLContext> {
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLCreateContext(ppcontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDXMLCreateContext(&mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WSDXMLGetNameFromBuiltinNamespace<P0, P1>(psznamespace: P0, pszname: P1) -> windows_core::Result<*mut WSDXML_NAME>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDXMLGetNameFromBuiltinNamespace(psznamespace : windows_core::PCWSTR, pszname : windows_core::PCWSTR, ppname : *mut *mut WSDXML_NAME) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDXMLGetNameFromBuiltinNamespace(psznamespace.param().abi(), pszname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "shared")]
pub const DPP_AppSequence: super::DWORD = 524299;
#[cfg(feature = "shared")]
pub const DPP_Bye: super::DWORD = 524289;
#[cfg(feature = "shared")]
pub const DPP_DiscoveryProxy: super::DWORD = 524305;
#[cfg(feature = "shared")]
pub const DPP_Element: super::DWORD = 524307;
#[cfg(feature = "shared")]
pub const DPP_Hello: super::DWORD = 524288;
#[cfg(feature = "shared")]
pub const DPP_Id: super::DWORD = 524309;
#[cfg(feature = "shared")]
pub const DPP_InstanceId: super::DWORD = 524300;
#[cfg(feature = "shared")]
pub const DPP_KeyId: super::DWORD = 524313;
#[cfg(feature = "shared")]
pub const DPP_MatchBy: super::DWORD = 524303;
#[cfg(feature = "shared")]
pub const DPP_MessageNumber: super::DWORD = 524302;
#[cfg(feature = "shared")]
pub const DPP_MetadataVersion: super::DWORD = 524297;
#[cfg(feature = "shared")]
pub const DPP_Probe: super::DWORD = 524290;
#[cfg(feature = "shared")]
pub const DPP_ProbeMatch: super::DWORD = 524292;
#[cfg(feature = "shared")]
pub const DPP_ProbeMatches: super::DWORD = 524291;
#[cfg(feature = "shared")]
pub const DPP_Refs: super::DWORD = 524314;
#[cfg(feature = "shared")]
pub const DPP_Resolve: super::DWORD = 524293;
#[cfg(feature = "shared")]
pub const DPP_ResolveMatch: super::DWORD = 524294;
#[cfg(feature = "shared")]
pub const DPP_ResolveMatches: super::DWORD = 524308;
#[cfg(feature = "shared")]
pub const DPP_Scheme: super::DWORD = 524312;
#[cfg(feature = "shared")]
pub const DPP_Scopes: super::DWORD = 524296;
#[cfg(feature = "shared")]
pub const DPP_Security: super::DWORD = 524310;
#[cfg(feature = "shared")]
pub const DPP_SequenceId: super::DWORD = 524301;
#[cfg(feature = "shared")]
pub const DPP_Sig: super::DWORD = 524311;
#[cfg(feature = "shared")]
pub const DPP_TargetService: super::DWORD = 524306;
#[cfg(feature = "shared")]
pub const DPP_Transport: super::DWORD = 524298;
#[cfg(feature = "shared")]
pub const DPP_Types: super::DWORD = 524295;
#[cfg(feature = "shared")]
pub const DPP_XAddrs: super::DWORD = 524304;
pub type DeviceDiscoveryMechanism = i32;
pub const DirectedDiscovery: DeviceDiscoveryMechanism = 1;
windows_core::imp::define_interface!(IWSDAddress, IWSDAddress_Vtbl, 0xb9574c6c_12a6_4f74_93a1_3318ff605759);
windows_core::imp::interface_hierarchy!(IWSDAddress, windows_core::IUnknown);
impl IWSDAddress {
    #[cfg(feature = "shared")]
    pub unsafe fn Serialize(&self, pszbuffer: windows_core::PWSTR, cchlength: super::DWORD, fsafe: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), pszbuffer, cchlength, fsafe.into()) }
    }
    pub unsafe fn Deserialize<P0>(&self, pszbuffer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Deserialize)(windows_core::Interface::as_raw(self), pszbuffer.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAddress_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, super::DWORD, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Serialize: usize,
    pub Deserialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWSDAddress_Impl: windows_core::IUnknownImpl {
    fn Serialize(&self, pszbuffer: windows_core::PWSTR, cchlength: super::DWORD, fsafe: windows_core::BOOL) -> windows_core::Result<()>;
    fn Deserialize(&self, pszbuffer: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWSDAddress_Vtbl {
    pub const fn new<Identity: IWSDAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Serialize<Identity: IWSDAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbuffer: windows_core::PWSTR, cchlength: super::DWORD, fsafe: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAddress_Impl::Serialize(this, core::mem::transmute_copy(&pszbuffer), core::mem::transmute_copy(&cchlength), core::mem::transmute_copy(&fsafe)).into()
            }
        }
        unsafe extern "system" fn Deserialize<Identity: IWSDAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbuffer: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAddress_Impl::Deserialize(this, core::mem::transmute(&pszbuffer)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Serialize: Serialize::<Identity, OFFSET>,
            Deserialize: Deserialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDAddress as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWSDAddress {}
windows_core::imp::define_interface!(IWSDAttachment, IWSDAttachment_Vtbl, 0x5d55a616_9df8_4b09_b156_9ba351a48b76);
windows_core::imp::interface_hierarchy!(IWSDAttachment, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAttachment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IWSDAttachment_Impl: windows_core::IUnknownImpl {}
impl IWSDAttachment_Vtbl {
    pub const fn new<Identity: IWSDAttachment_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDAttachment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDAttachment {}
windows_core::imp::define_interface!(IWSDDeviceHostNotify, IWSDDeviceHostNotify_Vtbl, 0xb5bee9f9_eeda_41fe_96f7_f45e14990fb0);
windows_core::imp::interface_hierarchy!(IWSDDeviceHostNotify, windows_core::IUnknown);
impl IWSDDeviceHostNotify {
    pub unsafe fn GetService<P0>(&self, pszserviceid: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), pszserviceid.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHostNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWSDDeviceHostNotify_Impl: windows_core::IUnknownImpl {
    fn GetService(&self, pszserviceid: &windows_core::PCWSTR) -> windows_core::Result<windows_core::IUnknown>;
}
impl IWSDDeviceHostNotify_Vtbl {
    pub const fn new<Identity: IWSDDeviceHostNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetService<Identity: IWSDDeviceHostNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceHostNotify_Impl::GetService(this, core::mem::transmute(&pszserviceid)) {
                    Ok(ok__) => {
                        ppservice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetService: GetService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDDeviceHostNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDDeviceHostNotify {}
windows_core::imp::define_interface!(IWSDEventingStatus, IWSDEventingStatus_Vtbl, 0x49b17f52_637a_407a_ae99_fbe82a4d38c0);
windows_core::imp::interface_hierarchy!(IWSDEventingStatus, windows_core::IUnknown);
impl IWSDEventingStatus {
    pub unsafe fn SubscriptionRenewed<P0>(&self, pszsubscriptionaction: P0)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SubscriptionRenewed)(windows_core::Interface::as_raw(self), pszsubscriptionaction.param().abi());
        }
    }
    pub unsafe fn SubscriptionRenewalFailed<P0>(&self, pszsubscriptionaction: P0, hr: windows_core::HRESULT)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SubscriptionRenewalFailed)(windows_core::Interface::as_raw(self), pszsubscriptionaction.param().abi(), hr);
        }
    }
    pub unsafe fn SubscriptionEnded<P0>(&self, pszsubscriptionaction: P0)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SubscriptionEnded)(windows_core::Interface::as_raw(self), pszsubscriptionaction.param().abi());
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEventingStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SubscriptionRenewed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR),
    pub SubscriptionRenewalFailed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::HRESULT),
    pub SubscriptionEnded: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR),
}
pub trait IWSDEventingStatus_Impl: windows_core::IUnknownImpl {
    fn SubscriptionRenewed(&self, pszsubscriptionaction: &windows_core::PCWSTR);
    fn SubscriptionRenewalFailed(&self, pszsubscriptionaction: &windows_core::PCWSTR, hr: windows_core::HRESULT);
    fn SubscriptionEnded(&self, pszsubscriptionaction: &windows_core::PCWSTR);
}
impl IWSDEventingStatus_Vtbl {
    pub const fn new<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SubscriptionRenewed<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubscriptionaction: windows_core::PCWSTR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEventingStatus_Impl::SubscriptionRenewed(this, core::mem::transmute(&pszsubscriptionaction));
            }
        }
        unsafe extern "system" fn SubscriptionRenewalFailed<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubscriptionaction: windows_core::PCWSTR, hr: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEventingStatus_Impl::SubscriptionRenewalFailed(this, core::mem::transmute(&pszsubscriptionaction), core::mem::transmute_copy(&hr));
            }
        }
        unsafe extern "system" fn SubscriptionEnded<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubscriptionaction: windows_core::PCWSTR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEventingStatus_Impl::SubscriptionEnded(this, core::mem::transmute(&pszsubscriptionaction));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SubscriptionRenewed: SubscriptionRenewed::<Identity, OFFSET>,
            SubscriptionRenewalFailed: SubscriptionRenewalFailed::<Identity, OFFSET>,
            SubscriptionEnded: SubscriptionEnded::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDEventingStatus as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDEventingStatus {}
windows_core::imp::define_interface!(IWSDHttpAddress, IWSDHttpAddress_Vtbl, 0xd09ac7bd_2a3e_4b85_8605_2737ff3e4ea0);
impl core::ops::Deref for IWSDHttpAddress {
    type Target = IWSDTransportAddress;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDHttpAddress, windows_core::IUnknown, IWSDAddress, IWSDTransportAddress);
impl IWSDHttpAddress {
    pub unsafe fn GetSecure(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSecure)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetSecure(&self, fsecure: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSecure)(windows_core::Interface::as_raw(self), fsecure.into()) }
    }
    pub unsafe fn GetPath(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPath<P0>(&self, pszpath: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPath)(windows_core::Interface::as_raw(self), pszpath.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAddress_Vtbl {
    pub base__: IWSDTransportAddress_Vtbl,
    pub GetSecure: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSecure: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWSDHttpAddress_Impl: IWSDTransportAddress_Impl {
    fn GetSecure(&self) -> windows_core::Result<()>;
    fn SetSecure(&self, fsecure: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetPath(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetPath(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWSDHttpAddress_Vtbl {
    pub const fn new<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSecure<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpAddress_Impl::GetSecure(this).into()
            }
        }
        unsafe extern "system" fn SetSecure<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsecure: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpAddress_Impl::SetSecure(this, core::mem::transmute_copy(&fsecure)).into()
            }
        }
        unsafe extern "system" fn GetPath<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpath: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDHttpAddress_Impl::GetPath(this) {
                    Ok(ok__) => {
                        ppszpath.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPath<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpAddress_Impl::SetPath(this, core::mem::transmute(&pszpath)).into()
            }
        }
        Self {
            base__: IWSDTransportAddress_Vtbl::new::<Identity, OFFSET>(),
            GetSecure: GetSecure::<Identity, OFFSET>,
            SetSecure: SetSecure::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDHttpAddress as windows_core::Interface>::IID || iid == &<IWSDAddress as windows_core::Interface>::IID || iid == &<IWSDTransportAddress as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWSDHttpAddress {}
windows_core::imp::define_interface!(IWSDHttpAuthParameters, IWSDHttpAuthParameters_Vtbl, 0x0b476df0_8dac_480d_b05c_99781a5884aa);
windows_core::imp::interface_hierarchy!(IWSDHttpAuthParameters, windows_core::IUnknown);
impl IWSDHttpAuthParameters {
    #[cfg(feature = "winnt")]
    pub unsafe fn GetClientAccessToken(&self, phtoken: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClientAccessToken)(windows_core::Interface::as_raw(self), phtoken as _) }
    }
    pub unsafe fn GetAuthType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAuthType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpAuthParameters_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub GetClientAccessToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetClientAccessToken: usize,
    pub GetAuthType: unsafe extern "system" fn(*mut core::ffi::c_void, PWSD_SECURITY_HTTP_AUTH_SCHEMES) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait IWSDHttpAuthParameters_Impl: windows_core::IUnknownImpl {
    fn GetClientAccessToken(&self, phtoken: *mut super::HANDLE) -> windows_core::Result<()>;
    fn GetAuthType(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "winnt")]
impl IWSDHttpAuthParameters_Vtbl {
    pub const fn new<Identity: IWSDHttpAuthParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientAccessToken<Identity: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phtoken: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpAuthParameters_Impl::GetClientAccessToken(this, core::mem::transmute_copy(&phtoken)).into()
            }
        }
        unsafe extern "system" fn GetAuthType<Identity: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthtype: PWSD_SECURITY_HTTP_AUTH_SCHEMES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDHttpAuthParameters_Impl::GetAuthType(this) {
                    Ok(ok__) => {
                        pauthtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientAccessToken: GetClientAccessToken::<Identity, OFFSET>,
            GetAuthType: GetAuthType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDHttpAuthParameters as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IWSDHttpAuthParameters {}
windows_core::imp::define_interface!(IWSDHttpMessageParameters, IWSDHttpMessageParameters_Vtbl, 0x540bd122_5c83_4dec_b396_ea62a2697fdf);
impl core::ops::Deref for IWSDHttpMessageParameters {
    type Target = IWSDMessageParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDHttpMessageParameters, windows_core::IUnknown, IWSDMessageParameters);
impl IWSDHttpMessageParameters {
    pub unsafe fn SetInboundHttpHeaders<P0>(&self, pszheaders: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInboundHttpHeaders)(windows_core::Interface::as_raw(self), pszheaders.param().abi()) }
    }
    pub unsafe fn GetInboundHttpHeaders(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInboundHttpHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetOutboundHttpHeaders<P0>(&self, pszheaders: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutboundHttpHeaders)(windows_core::Interface::as_raw(self), pszheaders.param().abi()) }
    }
    pub unsafe fn GetOutboundHttpHeaders(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutboundHttpHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetID<P0>(&self, pszid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetID)(windows_core::Interface::as_raw(self), pszid.param().abi()) }
    }
    pub unsafe fn GetID(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetContext<P0>(&self, pcontext: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContext)(windows_core::Interface::as_raw(self), pcontext.param().abi()) }
    }
    pub unsafe fn GetContext(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDHttpMessageParameters_Vtbl {
    pub base__: IWSDMessageParameters_Vtbl,
    pub SetInboundHttpHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetInboundHttpHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetOutboundHttpHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetOutboundHttpHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWSDHttpMessageParameters_Impl: IWSDMessageParameters_Impl {
    fn SetInboundHttpHeaders(&self, pszheaders: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetInboundHttpHeaders(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetOutboundHttpHeaders(&self, pszheaders: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetOutboundHttpHeaders(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetID(&self, pszid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetID(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetContext(&self, pcontext: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetContext(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl IWSDHttpMessageParameters_Vtbl {
    pub const fn new<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInboundHttpHeaders<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszheaders: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpMessageParameters_Impl::SetInboundHttpHeaders(this, core::mem::transmute(&pszheaders)).into()
            }
        }
        unsafe extern "system" fn GetInboundHttpHeaders<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszheaders: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDHttpMessageParameters_Impl::GetInboundHttpHeaders(this) {
                    Ok(ok__) => {
                        ppszheaders.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOutboundHttpHeaders<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszheaders: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpMessageParameters_Impl::SetOutboundHttpHeaders(this, core::mem::transmute(&pszheaders)).into()
            }
        }
        unsafe extern "system" fn GetOutboundHttpHeaders<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszheaders: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDHttpMessageParameters_Impl::GetOutboundHttpHeaders(this) {
                    Ok(ok__) => {
                        ppszheaders.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetID<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpMessageParameters_Impl::SetID(this, core::mem::transmute(&pszid)).into()
            }
        }
        unsafe extern "system" fn GetID<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszid: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDHttpMessageParameters_Impl::GetID(this) {
                    Ok(ok__) => {
                        ppszid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContext<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpMessageParameters_Impl::SetContext(this, core::mem::transmute_copy(&pcontext)).into()
            }
        }
        unsafe extern "system" fn GetContext<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDHttpMessageParameters_Impl::GetContext(this) {
                    Ok(ok__) => {
                        ppcontext.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clear<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDHttpMessageParameters_Impl::Clear(this).into()
            }
        }
        Self {
            base__: IWSDMessageParameters_Vtbl::new::<Identity, OFFSET>(),
            SetInboundHttpHeaders: SetInboundHttpHeaders::<Identity, OFFSET>,
            GetInboundHttpHeaders: GetInboundHttpHeaders::<Identity, OFFSET>,
            SetOutboundHttpHeaders: SetOutboundHttpHeaders::<Identity, OFFSET>,
            GetOutboundHttpHeaders: GetOutboundHttpHeaders::<Identity, OFFSET>,
            SetID: SetID::<Identity, OFFSET>,
            GetID: GetID::<Identity, OFFSET>,
            SetContext: SetContext::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDHttpMessageParameters as windows_core::Interface>::IID || iid == &<IWSDMessageParameters as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDHttpMessageParameters {}
windows_core::imp::define_interface!(IWSDInboundAttachment, IWSDInboundAttachment_Vtbl, 0x5bd6ca65_233c_4fb8_9f7a_2641619655c9);
impl core::ops::Deref for IWSDInboundAttachment {
    type Target = IWSDAttachment;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDInboundAttachment, windows_core::IUnknown, IWSDAttachment);
impl IWSDInboundAttachment {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Read(&self, pbuffer: *mut super::BYTE, dwbytestoread: super::DWORD, pdwnumberofbytesread: super::LPDWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pbuffer as _, dwbytestoread, pdwnumberofbytesread as _) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDInboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, super::DWORD, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Read: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IWSDInboundAttachment_Impl: IWSDAttachment_Impl {
    fn Read(&self, pbuffer: *mut super::BYTE, dwbytestoread: super::DWORD, pdwnumberofbytesread: super::LPDWORD) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IWSDInboundAttachment_Vtbl {
    pub const fn new<Identity: IWSDInboundAttachment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut super::BYTE, dwbytestoread: super::DWORD, pdwnumberofbytesread: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDInboundAttachment_Impl::Read(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwbytestoread), core::mem::transmute_copy(&pdwnumberofbytesread)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDInboundAttachment_Impl::Close(this).into()
            }
        }
        Self { base__: IWSDAttachment_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDInboundAttachment as windows_core::Interface>::IID || iid == &<IWSDAttachment as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IWSDInboundAttachment {}
windows_core::imp::define_interface!(IWSDMessageParameters, IWSDMessageParameters_Vtbl, 0x1fafe8a2_e6fc_4b80_b6cf_b7d45c416d7c);
windows_core::imp::interface_hierarchy!(IWSDMessageParameters, windows_core::IUnknown);
impl IWSDMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> windows_core::Result<IWSDAddress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetLocalAddress<P0>(&self, paddress: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDAddress>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLocalAddress)(windows_core::Interface::as_raw(self), paddress.param().abi()) }
    }
    pub unsafe fn GetRemoteAddress(&self) -> windows_core::Result<IWSDAddress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRemoteAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetRemoteAddress<P0>(&self, paddress: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDAddress>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteAddress)(windows_core::Interface::as_raw(self), paddress.param().abi()) }
    }
    pub unsafe fn GetLowerParameters(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLowerParameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMessageParameters_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLocalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLocalAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRemoteAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRemoteAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLowerParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWSDMessageParameters_Impl: windows_core::IUnknownImpl {
    fn GetLocalAddress(&self) -> windows_core::Result<IWSDAddress>;
    fn SetLocalAddress(&self, paddress: windows_core::Ref<IWSDAddress>) -> windows_core::Result<()>;
    fn GetRemoteAddress(&self) -> windows_core::Result<IWSDAddress>;
    fn SetRemoteAddress(&self, paddress: windows_core::Ref<IWSDAddress>) -> windows_core::Result<()>;
    fn GetLowerParameters(&self) -> windows_core::Result<IWSDMessageParameters>;
}
impl IWSDMessageParameters_Vtbl {
    pub const fn new<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLocalAddress<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDMessageParameters_Impl::GetLocalAddress(this) {
                    Ok(ok__) => {
                        ppaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLocalAddress<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDMessageParameters_Impl::SetLocalAddress(this, core::mem::transmute_copy(&paddress)).into()
            }
        }
        unsafe extern "system" fn GetRemoteAddress<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDMessageParameters_Impl::GetRemoteAddress(this) {
                    Ok(ok__) => {
                        ppaddress.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteAddress<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDMessageParameters_Impl::SetRemoteAddress(this, core::mem::transmute_copy(&paddress)).into()
            }
        }
        unsafe extern "system" fn GetLowerParameters<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptxparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDMessageParameters_Impl::GetLowerParameters(this) {
                    Ok(ok__) => {
                        pptxparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLocalAddress: GetLocalAddress::<Identity, OFFSET>,
            SetLocalAddress: SetLocalAddress::<Identity, OFFSET>,
            GetRemoteAddress: GetRemoteAddress::<Identity, OFFSET>,
            SetRemoteAddress: SetRemoteAddress::<Identity, OFFSET>,
            GetLowerParameters: GetLowerParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDMessageParameters as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDMessageParameters {}
windows_core::imp::define_interface!(IWSDOutboundAttachment, IWSDOutboundAttachment_Vtbl, 0xaa302f8d_5a22_4ba5_b392_aa8486f4c15d);
impl core::ops::Deref for IWSDOutboundAttachment {
    type Target = IWSDAttachment;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDOutboundAttachment, windows_core::IUnknown, IWSDAttachment);
impl IWSDOutboundAttachment {
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub unsafe fn Write(&self, pbuffer: &[super::BYTE]) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pbuffer.as_ptr(), pbuffer.len().try_into().unwrap(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDOutboundAttachment_Vtbl {
    pub base__: IWSDAttachment_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared"))]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::DWORD, super::LPDWORD) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared")))]
    Write: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub trait IWSDOutboundAttachment_Impl: IWSDAttachment_Impl {
    fn Write(&self, pbuffer: *const super::BYTE, dwbytestowrite: super::DWORD) -> windows_core::Result<u32>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl IWSDOutboundAttachment_Vtbl {
    pub const fn new<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Write<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *const super::BYTE, dwbytestowrite: super::DWORD, pdwnumberofbyteswritten: super::LPDWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDOutboundAttachment_Impl::Write(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwbytestowrite)) {
                    Ok(ok__) => {
                        pdwnumberofbyteswritten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDOutboundAttachment_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDOutboundAttachment_Impl::Abort(this).into()
            }
        }
        Self {
            base__: IWSDAttachment_Vtbl::new::<Identity, OFFSET>(),
            Write: Write::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDOutboundAttachment as windows_core::Interface>::IID || iid == &<IWSDAttachment as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl windows_core::RuntimeName for IWSDOutboundAttachment {}
windows_core::imp::define_interface!(IWSDSSLClientCertificate, IWSDSSLClientCertificate_Vtbl, 0xde105e87_a0da_418e_98ad_27b9eed87bdc);
windows_core::imp::interface_hierarchy!(IWSDSSLClientCertificate, windows_core::IUnknown);
impl IWSDSSLClientCertificate {
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
    pub unsafe fn GetClientCertificate(&self, ppcertcontext: *mut super::PCCERT_CONTEXT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClientCertificate)(windows_core::Interface::as_raw(self), ppcertcontext as _) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetMappedAccessToken(&self, phtoken: *mut super::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMappedAccessToken)(windows_core::Interface::as_raw(self), phtoken as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSSLClientCertificate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
    pub GetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::PCCERT_CONTEXT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "wincrypt")))]
    GetClientCertificate: usize,
    #[cfg(feature = "winnt")]
    pub GetMappedAccessToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetMappedAccessToken: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt", feature = "winnt"))]
pub trait IWSDSSLClientCertificate_Impl: windows_core::IUnknownImpl {
    fn GetClientCertificate(&self, ppcertcontext: *mut super::PCCERT_CONTEXT) -> windows_core::Result<()>;
    fn GetMappedAccessToken(&self, phtoken: *mut super::HANDLE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt", feature = "winnt"))]
impl IWSDSSLClientCertificate_Vtbl {
    pub const fn new<Identity: IWSDSSLClientCertificate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientCertificate<Identity: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcertcontext: *mut super::PCCERT_CONTEXT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDSSLClientCertificate_Impl::GetClientCertificate(this, core::mem::transmute_copy(&ppcertcontext)).into()
            }
        }
        unsafe extern "system" fn GetMappedAccessToken<Identity: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phtoken: *mut super::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDSSLClientCertificate_Impl::GetMappedAccessToken(this, core::mem::transmute_copy(&phtoken)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientCertificate: GetClientCertificate::<Identity, OFFSET>,
            GetMappedAccessToken: GetMappedAccessToken::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDSSLClientCertificate as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDSSLClientCertificate {}
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
windows_core::imp::define_interface!(IWSDSignatureProperty, IWSDSignatureProperty_Vtbl, 0x03ce20aa_71c4_45e2_b32e_3766c61c790f);
windows_core::imp::interface_hierarchy!(IWSDSignatureProperty, windows_core::IUnknown);
impl IWSDSignatureProperty {
    pub unsafe fn IsMessageSigned(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMessageSigned)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsMessageSignatureTrusted(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMessageSignatureTrusted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetKeyInfo(&self, pbkeyinfo: Option<*mut super::BYTE>, pdwkeyinfosize: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetKeyInfo)(windows_core::Interface::as_raw(self), pbkeyinfo.unwrap_or(core::mem::zeroed()) as _, pdwkeyinfosize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSignature(&self, pbsignature: Option<*mut super::BYTE>, pdwsignaturesize: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSignature)(windows_core::Interface::as_raw(self), pbsignature.unwrap_or(core::mem::zeroed()) as _, pdwsignaturesize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSignedInfoHash(&self, pbsignedinfohash: Option<*mut super::BYTE>, pdwhashsize: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSignedInfoHash)(windows_core::Interface::as_raw(self), pbsignedinfohash.unwrap_or(core::mem::zeroed()) as _, pdwhashsize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSignatureProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsMessageSigned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsMessageSignatureTrusted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetKeyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetKeyInfo: usize,
    #[cfg(feature = "shared")]
    pub GetSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSignature: usize,
    #[cfg(feature = "shared")]
    pub GetSignedInfoHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::BYTE, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSignedInfoHash: usize,
}
#[cfg(feature = "shared")]
pub trait IWSDSignatureProperty_Impl: windows_core::IUnknownImpl {
    fn IsMessageSigned(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsMessageSignatureTrusted(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetKeyInfo(&self, pbkeyinfo: *mut super::BYTE, pdwkeyinfosize: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetSignature(&self, pbsignature: *mut super::BYTE, pdwsignaturesize: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetSignedInfoHash(&self, pbsignedinfohash: *mut super::BYTE, pdwhashsize: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWSDSignatureProperty_Vtbl {
    pub const fn new<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsMessageSigned<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsigned: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDSignatureProperty_Impl::IsMessageSigned(this) {
                    Ok(ok__) => {
                        pbsigned.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsMessageSignatureTrusted<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsignaturetrusted: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDSignatureProperty_Impl::IsMessageSignatureTrusted(this) {
                    Ok(ok__) => {
                        pbsignaturetrusted.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetKeyInfo<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbkeyinfo: *mut super::BYTE, pdwkeyinfosize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDSignatureProperty_Impl::GetKeyInfo(this, core::mem::transmute_copy(&pbkeyinfo), core::mem::transmute_copy(&pdwkeyinfosize)).into()
            }
        }
        unsafe extern "system" fn GetSignature<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsignature: *mut super::BYTE, pdwsignaturesize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDSignatureProperty_Impl::GetSignature(this, core::mem::transmute_copy(&pbsignature), core::mem::transmute_copy(&pdwsignaturesize)).into()
            }
        }
        unsafe extern "system" fn GetSignedInfoHash<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsignedinfohash: *mut super::BYTE, pdwhashsize: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDSignatureProperty_Impl::GetSignedInfoHash(this, core::mem::transmute_copy(&pbsignedinfohash), core::mem::transmute_copy(&pdwhashsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsMessageSigned: IsMessageSigned::<Identity, OFFSET>,
            IsMessageSignatureTrusted: IsMessageSignatureTrusted::<Identity, OFFSET>,
            GetKeyInfo: GetKeyInfo::<Identity, OFFSET>,
            GetSignature: GetSignature::<Identity, OFFSET>,
            GetSignedInfoHash: GetSignedInfoHash::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDSignatureProperty as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWSDSignatureProperty {}
windows_core::imp::define_interface!(IWSDTransportAddress, IWSDTransportAddress_Vtbl, 0x70d23498_4ee6_4340_a3df_d845d2235467);
impl core::ops::Deref for IWSDTransportAddress {
    type Target = IWSDAddress;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDTransportAddress, windows_core::IUnknown, IWSDAddress);
impl IWSDTransportAddress {
    #[cfg(feature = "shared")]
    pub unsafe fn GetPort(&self) -> windows_core::Result<super::WORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetPortA(&self, wport: super::WORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPortA)(windows_core::Interface::as_raw(self), wport) }
    }
    pub unsafe fn GetTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransportAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTransportAddressEx(&self, fsafe: bool) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransportAddressEx)(windows_core::Interface::as_raw(self), fsafe.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTransportAddress<P0>(&self, pszaddress: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTransportAddress)(windows_core::Interface::as_raw(self), pszaddress.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDTransportAddress_Vtbl {
    pub base__: IWSDAddress_Vtbl,
    #[cfg(feature = "shared")]
    pub GetPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPort: usize,
    #[cfg(feature = "shared")]
    pub SetPortA: unsafe extern "system" fn(*mut core::ffi::c_void, super::WORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetPortA: usize,
    pub GetTransportAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetTransportAddressEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetTransportAddress: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IWSDTransportAddress_Impl: IWSDAddress_Impl {
    fn GetPort(&self) -> windows_core::Result<super::WORD>;
    fn SetPortA(&self, wport: super::WORD) -> windows_core::Result<()>;
    fn GetTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetTransportAddressEx(&self, fsafe: windows_core::BOOL) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetTransportAddress(&self, pszaddress: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWSDTransportAddress_Vtbl {
    pub const fn new<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPort<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwport: *mut super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDTransportAddress_Impl::GetPort(this) {
                    Ok(ok__) => {
                        pwport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPortA<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wport: super::WORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDTransportAddress_Impl::SetPortA(this, core::mem::transmute_copy(&wport)).into()
            }
        }
        unsafe extern "system" fn GetTransportAddress<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszaddress: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDTransportAddress_Impl::GetTransportAddress(this) {
                    Ok(ok__) => {
                        ppszaddress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransportAddressEx<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsafe: windows_core::BOOL, ppszaddress: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDTransportAddress_Impl::GetTransportAddressEx(this, core::mem::transmute_copy(&fsafe)) {
                    Ok(ok__) => {
                        ppszaddress.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransportAddress<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszaddress: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDTransportAddress_Impl::SetTransportAddress(this, core::mem::transmute(&pszaddress)).into()
            }
        }
        Self {
            base__: IWSDAddress_Vtbl::new::<Identity, OFFSET>(),
            GetPort: GetPort::<Identity, OFFSET>,
            SetPortA: SetPortA::<Identity, OFFSET>,
            GetTransportAddress: GetTransportAddress::<Identity, OFFSET>,
            GetTransportAddressEx: GetTransportAddressEx::<Identity, OFFSET>,
            SetTransportAddress: SetTransportAddress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDTransportAddress as windows_core::Interface>::IID || iid == &<IWSDAddress as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWSDTransportAddress {}
windows_core::imp::define_interface!(IWSDUdpAddress, IWSDUdpAddress_Vtbl, 0x74d6124a_a441_4f78_a1eb_97a8d1996893);
impl core::ops::Deref for IWSDUdpAddress {
    type Target = IWSDTransportAddress;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDUdpAddress, windows_core::IUnknown, IWSDAddress, IWSDTransportAddress);
impl IWSDUdpAddress {
    #[cfg(all(feature = "shared", feature = "ws2"))]
    pub unsafe fn SetSockaddr(&self, psockaddr: *const super::SOCKADDR_STORAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSockaddr)(windows_core::Interface::as_raw(self), psockaddr) }
    }
    #[cfg(all(feature = "shared", feature = "ws2"))]
    pub unsafe fn GetSockaddr(&self, psockaddr: *mut super::SOCKADDR_STORAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSockaddr)(windows_core::Interface::as_raw(self), psockaddr as _) }
    }
    pub unsafe fn SetExclusive(&self, fexclusive: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExclusive)(windows_core::Interface::as_raw(self), fexclusive.into()) }
    }
    pub unsafe fn GetExclusive(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExclusive)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetMessageType(&self, messagetype: WSDUdpMessageType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMessageType)(windows_core::Interface::as_raw(self), messagetype) }
    }
    pub unsafe fn GetMessageType(&self) -> windows_core::Result<WSDUdpMessageType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMessageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetTTL(&self, dwttl: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTTL)(windows_core::Interface::as_raw(self), dwttl) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTTL(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTTL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAlias(&self, palias: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAlias)(windows_core::Interface::as_raw(self), palias) }
    }
    pub unsafe fn GetAlias(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlias)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpAddress_Vtbl {
    pub base__: IWSDTransportAddress_Vtbl,
    #[cfg(all(feature = "shared", feature = "ws2"))]
    pub SetSockaddr: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SOCKADDR_STORAGE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "ws2")))]
    SetSockaddr: usize,
    #[cfg(all(feature = "shared", feature = "ws2"))]
    pub GetSockaddr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SOCKADDR_STORAGE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "ws2")))]
    GetSockaddr: usize,
    pub SetExclusive: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetExclusive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(*mut core::ffi::c_void, WSDUdpMessageType) -> windows_core::HRESULT,
    pub GetMessageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSDUdpMessageType) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetTTL: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetTTL: usize,
    #[cfg(feature = "shared")]
    pub GetTTL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTTL: usize,
    pub SetAlias: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetAlias: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "ws2"))]
pub trait IWSDUdpAddress_Impl: IWSDTransportAddress_Impl {
    fn SetSockaddr(&self, psockaddr: *const super::SOCKADDR_STORAGE) -> windows_core::Result<()>;
    fn GetSockaddr(&self, psockaddr: *mut super::SOCKADDR_STORAGE) -> windows_core::Result<()>;
    fn SetExclusive(&self, fexclusive: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetExclusive(&self) -> windows_core::Result<()>;
    fn SetMessageType(&self, messagetype: WSDUdpMessageType) -> windows_core::Result<()>;
    fn GetMessageType(&self) -> windows_core::Result<WSDUdpMessageType>;
    fn SetTTL(&self, dwttl: super::DWORD) -> windows_core::Result<()>;
    fn GetTTL(&self) -> windows_core::Result<super::DWORD>;
    fn SetAlias(&self, palias: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetAlias(&self) -> windows_core::Result<windows_core::GUID>;
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl IWSDUdpAddress_Vtbl {
    pub const fn new<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSockaddr<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psockaddr: *const super::SOCKADDR_STORAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::SetSockaddr(this, core::mem::transmute_copy(&psockaddr)).into()
            }
        }
        unsafe extern "system" fn GetSockaddr<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psockaddr: *mut super::SOCKADDR_STORAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::GetSockaddr(this, core::mem::transmute_copy(&psockaddr)).into()
            }
        }
        unsafe extern "system" fn SetExclusive<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fexclusive: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::SetExclusive(this, core::mem::transmute_copy(&fexclusive)).into()
            }
        }
        unsafe extern "system" fn GetExclusive<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::GetExclusive(this).into()
            }
        }
        unsafe extern "system" fn SetMessageType<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetype: WSDUdpMessageType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::SetMessageType(this, core::mem::transmute_copy(&messagetype)).into()
            }
        }
        unsafe extern "system" fn GetMessageType<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessagetype: *mut WSDUdpMessageType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDUdpAddress_Impl::GetMessageType(this) {
                    Ok(ok__) => {
                        pmessagetype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTTL<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwttl: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::SetTTL(this, core::mem::transmute_copy(&dwttl)).into()
            }
        }
        unsafe extern "system" fn GetTTL<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwttl: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDUdpAddress_Impl::GetTTL(this) {
                    Ok(ok__) => {
                        pdwttl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAlias<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palias: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::SetAlias(this, core::mem::transmute_copy(&palias)).into()
            }
        }
        unsafe extern "system" fn GetAlias<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palias: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDUdpAddress_Impl::GetAlias(this) {
                    Ok(ok__) => {
                        palias.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWSDTransportAddress_Vtbl::new::<Identity, OFFSET>(),
            SetSockaddr: SetSockaddr::<Identity, OFFSET>,
            GetSockaddr: GetSockaddr::<Identity, OFFSET>,
            SetExclusive: SetExclusive::<Identity, OFFSET>,
            GetExclusive: GetExclusive::<Identity, OFFSET>,
            SetMessageType: SetMessageType::<Identity, OFFSET>,
            GetMessageType: GetMessageType::<Identity, OFFSET>,
            SetTTL: SetTTL::<Identity, OFFSET>,
            GetTTL: GetTTL::<Identity, OFFSET>,
            SetAlias: SetAlias::<Identity, OFFSET>,
            GetAlias: GetAlias::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDUdpAddress as windows_core::Interface>::IID || iid == &<IWSDAddress as windows_core::Interface>::IID || iid == &<IWSDTransportAddress as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "ws2"))]
impl windows_core::RuntimeName for IWSDUdpAddress {}
windows_core::imp::define_interface!(IWSDUdpMessageParameters, IWSDUdpMessageParameters_Vtbl, 0x9934149f_8f0c_447b_aa0b_73124b0ca7f0);
impl core::ops::Deref for IWSDUdpMessageParameters {
    type Target = IWSDMessageParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDUdpMessageParameters, windows_core::IUnknown, IWSDMessageParameters);
impl IWSDUdpMessageParameters {
    #[cfg(feature = "shared")]
    pub unsafe fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRetransmitParams)(windows_core::Interface::as_raw(self), pparams) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetRetransmitParams(&self, pparams: *mut WSDUdpRetransmitParams) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRetransmitParams)(windows_core::Interface::as_raw(self), pparams as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpMessageParameters_Vtbl {
    pub base__: IWSDMessageParameters_Vtbl,
    #[cfg(feature = "shared")]
    pub SetRetransmitParams: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSDUdpRetransmitParams) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRetransmitParams: usize,
    #[cfg(feature = "shared")]
    pub GetRetransmitParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSDUdpRetransmitParams) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetRetransmitParams: usize,
}
#[cfg(feature = "shared")]
pub trait IWSDUdpMessageParameters_Impl: IWSDMessageParameters_Impl {
    fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> windows_core::Result<()>;
    fn GetRetransmitParams(&self, pparams: *mut WSDUdpRetransmitParams) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IWSDUdpMessageParameters_Vtbl {
    pub const fn new<Identity: IWSDUdpMessageParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRetransmitParams<Identity: IWSDUdpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparams: *const WSDUdpRetransmitParams) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpMessageParameters_Impl::SetRetransmitParams(this, core::mem::transmute_copy(&pparams)).into()
            }
        }
        unsafe extern "system" fn GetRetransmitParams<Identity: IWSDUdpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparams: *mut WSDUdpRetransmitParams) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpMessageParameters_Impl::GetRetransmitParams(this, core::mem::transmute_copy(&pparams)).into()
            }
        }
        Self {
            base__: IWSDMessageParameters_Vtbl::new::<Identity, OFFSET>(),
            SetRetransmitParams: SetRetransmitParams::<Identity, OFFSET>,
            GetRetransmitParams: GetRetransmitParams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDUdpMessageParameters as windows_core::Interface>::IID || iid == &<IWSDMessageParameters as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IWSDUdpMessageParameters {}
windows_core::imp::define_interface!(IWSDXMLContext, IWSDXMLContext_Vtbl, 0x75d8f3ee_3e5a_43b4_a15a_bcf6887460c0);
windows_core::imp::interface_hierarchy!(IWSDXMLContext, windows_core::IUnknown);
impl IWSDXMLContext {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn AddNamespace<P0, P1>(&self, pszuri: P0, pszsuggestedprefix: P1, ppnamespace: Option<*mut *mut WSDXML_NAMESPACE>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddNamespace)(windows_core::Interface::as_raw(self), pszuri.param().abi(), pszsuggestedprefix.param().abi(), ppnamespace.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn AddNameToNamespace<P0, P1>(&self, pszuri: P0, pszname: P1, ppname: Option<*mut *mut WSDXML_NAME>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddNameToNamespace)(windows_core::Interface::as_raw(self), pszuri.param().abi(), pszname.param().abi(), ppname.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetNamespaces(&self, pnamespaces: &[PCWSDXML_NAMESPACE], blayernumber: super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNamespaces)(windows_core::Interface::as_raw(self), pnamespaces.as_ptr(), pnamespaces.len().try_into().unwrap(), blayernumber) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn SetTypes(&self, ptypes: &[PCWSDXML_TYPE], blayernumber: super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTypes)(windows_core::Interface::as_raw(self), ptypes.as_ptr(), ptypes.len().try_into().unwrap(), blayernumber) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDXMLContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub AddNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut WSDXML_NAMESPACE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    AddNamespace: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub AddNameToNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut WSDXML_NAME) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    AddNameToNamespace: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetNamespaces: unsafe extern "system" fn(*mut core::ffi::c_void, *const PCWSDXML_NAMESPACE, super::WORD, super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetNamespaces: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub SetTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *const PCWSDXML_TYPE, super::DWORD, super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    SetTypes: usize,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub trait IWSDXMLContext_Impl: windows_core::IUnknownImpl {
    fn AddNamespace(&self, pszuri: &windows_core::PCWSTR, pszsuggestedprefix: &windows_core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> windows_core::Result<()>;
    fn AddNameToNamespace(&self, pszuri: &windows_core::PCWSTR, pszname: &windows_core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> windows_core::Result<()>;
    fn SetNamespaces(&self, pnamespaces: *const PCWSDXML_NAMESPACE, wnamespacescount: super::WORD, blayernumber: super::BYTE) -> windows_core::Result<()>;
    fn SetTypes(&self, ptypes: *const PCWSDXML_TYPE, dwtypescount: super::DWORD, blayernumber: super::BYTE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl IWSDXMLContext_Vtbl {
    pub const fn new<Identity: IWSDXMLContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddNamespace<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuri: windows_core::PCWSTR, pszsuggestedprefix: windows_core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDXMLContext_Impl::AddNamespace(this, core::mem::transmute(&pszuri), core::mem::transmute(&pszsuggestedprefix), core::mem::transmute_copy(&ppnamespace)).into()
            }
        }
        unsafe extern "system" fn AddNameToNamespace<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuri: windows_core::PCWSTR, pszname: windows_core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDXMLContext_Impl::AddNameToNamespace(this, core::mem::transmute(&pszuri), core::mem::transmute(&pszname), core::mem::transmute_copy(&ppname)).into()
            }
        }
        unsafe extern "system" fn SetNamespaces<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespaces: *const PCWSDXML_NAMESPACE, wnamespacescount: super::WORD, blayernumber: super::BYTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDXMLContext_Impl::SetNamespaces(this, core::mem::transmute_copy(&pnamespaces), core::mem::transmute_copy(&wnamespacescount), core::mem::transmute_copy(&blayernumber)).into()
            }
        }
        unsafe extern "system" fn SetTypes<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypes: *const PCWSDXML_TYPE, dwtypescount: super::DWORD, blayernumber: super::BYTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDXMLContext_Impl::SetTypes(this, core::mem::transmute_copy(&ptypes), core::mem::transmute_copy(&dwtypescount), core::mem::transmute_copy(&blayernumber)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddNamespace: AddNamespace::<Identity, OFFSET>,
            AddNameToNamespace: AddNameToNamespace::<Identity, OFFSET>,
            SetNamespaces: SetNamespaces::<Identity, OFFSET>,
            SetTypes: SetTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDXMLContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IWSDXMLContext {}
pub const MulticastDiscovery: DeviceDiscoveryMechanism = 0;
pub const NAMESPACE_ENCODING_ADDRESSING: i32 = 1;
pub const NAMESPACE_ENCODING_DEVPROF: i32 = 5;
pub const NAMESPACE_ENCODING_DISCOVERY: i32 = 2;
pub const NAMESPACE_ENCODING_DISCOVERY11: i32 = 8;
pub const NAMESPACE_ENCODING_ENVELOPE: i32 = 7;
pub const NAMESPACE_ENCODING_EVENTING: i32 = 0;
pub const NAMESPACE_ENCODING_INCLUDE: i32 = 3;
pub const NAMESPACE_ENCODING_MEX: i32 = 6;
pub const NAMESPACE_ENCODING_XML: i32 = 4;
pub const ONE_WAY: WSDUdpMessageType = 0;
pub const OpAnyElement: i32 = 6;
pub const OpAnyElements: i32 = 7;
pub const OpAnyNumber: i32 = 17;
pub const OpAnyText: i32 = 8;
pub const OpAnything: i32 = 16;
pub const OpAttribute_: i32 = 9;
pub const OpBeginAll: i32 = 14;
pub const OpBeginAnyElement: i32 = 3;
pub const OpBeginChoice: i32 = 10;
pub const OpBeginElement_: i32 = 2;
pub const OpBeginSequence: i32 = 12;
pub const OpElement_: i32 = 5;
pub const OpEndAll: i32 = 15;
pub const OpEndChoice: i32 = 11;
pub const OpEndElement: i32 = 4;
pub const OpEndOfTable: i32 = 1;
pub const OpEndSequence: i32 = 13;
pub const OpFormatBool_: i32 = 20;
pub const OpFormatDateTime_: i32 = 40;
pub const OpFormatDom_: i32 = 30;
pub const OpFormatDouble_: i32 = 42;
pub const OpFormatDuration_: i32 = 39;
pub const OpFormatDynamicType_: i32 = 37;
pub const OpFormatFloat_: i32 = 41;
pub const OpFormatInt16_: i32 = 22;
pub const OpFormatInt32_: i32 = 23;
pub const OpFormatInt64_: i32 = 24;
pub const OpFormatInt8_: i32 = 21;
pub const OpFormatListInsertTail_: i32 = 35;
pub const OpFormatLookupType_: i32 = 38;
pub const OpFormatMax: i32 = 46;
pub const OpFormatName_: i32 = 34;
pub const OpFormatStruct_: i32 = 31;
pub const OpFormatType_: i32 = 36;
pub const OpFormatUInt16_: i32 = 26;
pub const OpFormatUInt32_: i32 = 27;
pub const OpFormatUInt64_: i32 = 28;
pub const OpFormatUInt8_: i32 = 25;
pub const OpFormatUnicodeString_: i32 = 29;
pub const OpFormatUri_: i32 = 32;
pub const OpFormatUuidUri_: i32 = 33;
pub const OpFormatXMLDeclaration_: i32 = 45;
pub const OpNone: i32 = 0;
pub const OpOneOrMore: i32 = 18;
pub const OpOptional: i32 = 19;
pub const OpProcess_: i32 = 43;
pub const OpQualifiedAttribute_: i32 = 44;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCWSDXML_NAMESPACE = *const _WSDXML_NAMESPACE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCWSDXML_TYPE = *const _WSDXML_TYPE;
#[cfg(feature = "shared")]
pub type PWSD_CONFIG_ADDRESSES = *mut WSD_CONFIG_ADDRESSES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWSD_CONFIG_PARAM = *mut WSD_CONFIG_PARAM;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
pub type PWSD_SECURITY_CERT_VALIDATION = *mut WSD_SECURITY_CERT_VALIDATION;
pub type PWSD_SECURITY_HTTP_AUTH_SCHEMES = *mut u32;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
pub type PWSD_SECURITY_SIGNATURE_VALIDATION = *mut WSD_SECURITY_SIGNATURE_VALIDATION;
#[cfg(feature = "shared")]
pub const SOAP_Actor: super::DWORD = 458757;
#[cfg(feature = "shared")]
pub const SOAP_Body: super::DWORD = 458754;
#[cfg(feature = "shared")]
pub const SOAP_Code: super::DWORD = 458759;
#[cfg(feature = "shared")]
pub const SOAP_DataEncodingUnknown: super::DWORD = 458772;
#[cfg(feature = "shared")]
pub const SOAP_Detail: super::DWORD = 458766;
#[cfg(feature = "shared")]
pub const SOAP_Element: super::DWORD = 458775;
#[cfg(feature = "shared")]
pub const SOAP_Envelope: super::DWORD = 458752;
#[cfg(feature = "shared")]
pub const SOAP_Fault: super::DWORD = 458758;
#[cfg(feature = "shared")]
pub const SOAP_Header: super::DWORD = 458753;
#[cfg(feature = "shared")]
pub const SOAP_MustUnderstand: super::DWORD = 458755;
#[cfg(feature = "shared")]
pub const SOAP_MustUnderstand_1: super::DWORD = 458771;
#[cfg(feature = "shared")]
pub const SOAP_Node: super::DWORD = 458764;
#[cfg(feature = "shared")]
pub const SOAP_NotUnderstood: super::DWORD = 458769;
#[cfg(feature = "shared")]
pub const SOAP_Reason: super::DWORD = 458762;
#[cfg(feature = "shared")]
pub const SOAP_Receiver: super::DWORD = 458774;
#[cfg(feature = "shared")]
pub const SOAP_Relay: super::DWORD = 458776;
#[cfg(feature = "shared")]
pub const SOAP_Role: super::DWORD = 458756;
#[cfg(feature = "shared")]
pub const SOAP_Role_1: super::DWORD = 458765;
#[cfg(feature = "shared")]
pub const SOAP_Sender: super::DWORD = 458773;
#[cfg(feature = "shared")]
pub const SOAP_Subcode: super::DWORD = 458760;
#[cfg(feature = "shared")]
pub const SOAP_SupportedEnvelope: super::DWORD = 458768;
#[cfg(feature = "shared")]
pub const SOAP_Text: super::DWORD = 458763;
#[cfg(feature = "shared")]
pub const SOAP_Upgrade: super::DWORD = 458767;
#[cfg(feature = "shared")]
pub const SOAP_Value: super::DWORD = 458761;
#[cfg(feature = "shared")]
pub const SOAP_VersionMismatch: super::DWORD = 458770;
pub const SecureDirectedDiscovery: DeviceDiscoveryMechanism = 2;
pub const TWO_WAY: WSDUdpMessageType = 1;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_BYE: super::DWORD = 38;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_HELLO: super::DWORD = 39;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_PROBE: super::DWORD = 37;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_PROBE_MATCH: super::DWORD = 35;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_PROBE_MATCHES: super::DWORD = 36;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_RESOLVE: super::DWORD = 34;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_RESOLVE_MATCH: super::DWORD = 32;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD11_RESOLVE_MATCHES: super::DWORD = 33;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_APP_SEQUENCE: super::DWORD = 9;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_BYE: super::DWORD = 19;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_ENDPOINT_REFERENCE: super::DWORD = 24;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_DELIVERY_MODE: super::DWORD = 30;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_DELIVERY_MODE_PUSH: super::DWORD = 29;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_EXPIRES: super::DWORD = 28;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_FILTER: super::DWORD = 27;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_EVENTING_FILTER_ACTION: super::DWORD = 26;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_HEADER_RELATESTO: super::DWORD = 10;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_HELLO: super::DWORD = 21;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_HOST_METADATA: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_LOCALIZED_STRING: super::DWORD = 31;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_METADATA_SECTION: super::DWORD = 25;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_PROBE: super::DWORD = 18;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_PROBE_MATCH: super::DWORD = 16;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_PROBE_MATCHES: super::DWORD = 17;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_REFERENCE_PARAMETERS: super::DWORD = 22;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_REFERENCE_PROPERTIES: super::DWORD = 23;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_RELATIONSHIP_METADATA: super::DWORD = 0;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_RESOLVE: super::DWORD = 15;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_RESOLVE_MATCH: super::DWORD = 14;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_RESOLVE_MATCHES: super::DWORD = 13;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SCOPES: super::DWORD = 20;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SERVICE_METADATA: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_FAULT: super::DWORD = 8;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_FAULT_CODE: super::DWORD = 7;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_FAULT_REASON: super::DWORD = 5;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_FAULT_SUBCODE: super::DWORD = 6;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_HEADER: super::DWORD = 11;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_SOAP_MESSAGE: super::DWORD = 12;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_THIS_DEVICE_METADATA: super::DWORD = 3;
#[cfg(feature = "shared")]
pub const TYPE_ENCODING_WSD_THIS_MODEL_METADATA: super::DWORD = 4;
#[cfg(feature = "shared")]
pub const WSA_Action: super::DWORD = 65550;
#[cfg(feature = "shared")]
pub const WSA_ActionNotSupported: super::DWORD = 65555;
#[cfg(feature = "shared")]
pub const WSA_Address: super::DWORD = 65537;
#[cfg(feature = "shared")]
pub const WSA_DestinationUnreachable: super::DWORD = 65554;
#[cfg(feature = "shared")]
pub const WSA_Element: super::DWORD = 65558;
#[cfg(feature = "shared")]
pub const WSA_EndpointReference: super::DWORD = 65536;
#[cfg(feature = "shared")]
pub const WSA_EndpointUnavailable: super::DWORD = 65556;
#[cfg(feature = "shared")]
pub const WSA_FaultTo: super::DWORD = 65548;
#[cfg(feature = "shared")]
pub const WSA_From: super::DWORD = 65547;
#[cfg(feature = "shared")]
pub const WSA_InvalidMessageInformationHeader: super::DWORD = 65552;
#[cfg(feature = "shared")]
pub const WSA_MessageID: super::DWORD = 65542;
#[cfg(feature = "shared")]
pub const WSA_MessageInformationHeaderRequired: super::DWORD = 65553;
#[cfg(feature = "shared")]
pub const WSA_PortName: super::DWORD = 65541;
#[cfg(feature = "shared")]
pub const WSA_PortType: super::DWORD = 65539;
#[cfg(feature = "shared")]
pub const WSA_Recipient: super::DWORD = 65551;
#[cfg(feature = "shared")]
pub const WSA_ReferenceParameters: super::DWORD = 65538;
#[cfg(feature = "shared")]
pub const WSA_ReferenceProperties: super::DWORD = 65557;
#[cfg(feature = "shared")]
pub const WSA_RelatesTo: super::DWORD = 65543;
#[cfg(feature = "shared")]
pub const WSA_RelationshipType: super::DWORD = 65544;
#[cfg(feature = "shared")]
pub const WSA_Reply: super::DWORD = 65545;
#[cfg(feature = "shared")]
pub const WSA_ReplyAfter: super::DWORD = 65560;
#[cfg(feature = "shared")]
pub const WSA_ReplyTo: super::DWORD = 65546;
#[cfg(feature = "shared")]
pub const WSA_RetryAfter: super::DWORD = 65559;
#[cfg(feature = "shared")]
pub const WSA_ServiceName: super::DWORD = 65540;
#[cfg(feature = "shared")]
pub const WSA_To: super::DWORD = 65549;
pub const WSDAPI_ADDRESSFAMILY_IPV4: i32 = 1;
pub const WSDAPI_ADDRESSFAMILY_IPV6: i32 = 2;
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: i32 = 1;
pub const WSDAPI_OPTION_MAX_INBOUND_MESSAGE_SIZE: i32 = 1;
pub const WSDAPI_OPTION_TRACE_XML_TO_DEBUGGER: i32 = 2;
pub const WSDAPI_OPTION_TRACE_XML_TO_FILE: i32 = 3;
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: i32 = 0;
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: i32 = 2;
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: i32 = 16;
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: i32 = 1;
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: i32 = 8;
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: i32 = 4;
pub const WSDET_INCOMING_FAULT: WSDEventType = 2;
pub const WSDET_INCOMING_MESSAGE: WSDEventType = 1;
pub const WSDET_NONE: WSDEventType = 0;
pub const WSDET_RESPONSE_TIMEOUT: WSDEventType = 4;
pub const WSDET_TRANSMISSION_FAILURE: WSDEventType = 3;
pub type WSDEventType = i32;
#[cfg(feature = "shared")]
pub const WSDISCO_AppSequence: super::DWORD = 131083;
#[cfg(feature = "shared")]
pub const WSDISCO_Bye: super::DWORD = 131073;
#[cfg(feature = "shared")]
pub const WSDISCO_DiscoveryProxy: super::DWORD = 131089;
#[cfg(feature = "shared")]
pub const WSDISCO_Element: super::DWORD = 131091;
#[cfg(feature = "shared")]
pub const WSDISCO_Hello: super::DWORD = 131072;
#[cfg(feature = "shared")]
pub const WSDISCO_Id: super::DWORD = 131093;
#[cfg(feature = "shared")]
pub const WSDISCO_InstanceId: super::DWORD = 131084;
#[cfg(feature = "shared")]
pub const WSDISCO_KeyId: super::DWORD = 131097;
#[cfg(feature = "shared")]
pub const WSDISCO_MatchBy: super::DWORD = 131087;
#[cfg(feature = "shared")]
pub const WSDISCO_MessageNumber: super::DWORD = 131086;
#[cfg(feature = "shared")]
pub const WSDISCO_MetadataVersion: super::DWORD = 131081;
#[cfg(feature = "shared")]
pub const WSDISCO_Probe: super::DWORD = 131074;
#[cfg(feature = "shared")]
pub const WSDISCO_ProbeMatch: super::DWORD = 131076;
#[cfg(feature = "shared")]
pub const WSDISCO_ProbeMatches: super::DWORD = 131075;
#[cfg(feature = "shared")]
pub const WSDISCO_Refs: super::DWORD = 131098;
#[cfg(feature = "shared")]
pub const WSDISCO_Resolve: super::DWORD = 131077;
#[cfg(feature = "shared")]
pub const WSDISCO_ResolveMatch: super::DWORD = 131078;
#[cfg(feature = "shared")]
pub const WSDISCO_ResolveMatches: super::DWORD = 131092;
#[cfg(feature = "shared")]
pub const WSDISCO_Scheme: super::DWORD = 131096;
#[cfg(feature = "shared")]
pub const WSDISCO_Scopes: super::DWORD = 131080;
#[cfg(feature = "shared")]
pub const WSDISCO_Security: super::DWORD = 131094;
#[cfg(feature = "shared")]
pub const WSDISCO_SequenceId: super::DWORD = 131085;
#[cfg(feature = "shared")]
pub const WSDISCO_Sig: super::DWORD = 131095;
#[cfg(feature = "shared")]
pub const WSDISCO_TargetService: super::DWORD = 131090;
#[cfg(feature = "shared")]
pub const WSDISCO_Transport: super::DWORD = 131082;
#[cfg(feature = "shared")]
pub const WSDISCO_Types: super::DWORD = 131079;
#[cfg(feature = "shared")]
pub const WSDISCO_XAddrs: super::DWORD = 131088;
pub type WSDUdpMessageType = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: super::ULONG,
    pub ulRepeat: super::ULONG,
    pub ulRepeatMinDelay: super::ULONG,
    pub ulRepeatMaxDelay: super::ULONG,
    pub ulRepeatUpperDelay: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_NAME = _WSDXML_NAME;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_NAMESPACE = _WSDXML_NAMESPACE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_PREFIX_MAPPING = _WSDXML_PREFIX_MAPPING;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSDXML_TYPE = _WSDXML_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_APP_SEQUENCE = _WSD_APP_SEQUENCE;
#[cfg(feature = "shared")]
pub const WSD_Actions: super::DWORD = 327682;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut Option<IWSDAddress>,
    pub dwAddressCount: super::DWORD,
}
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 10;
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 9;
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 1;
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: super::PVOID,
    pub dwConfigDataSize: super::DWORD,
}
pub type WSD_CONFIG_PARAM_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_DATETIME {
    pub isPositive: windows_core::BOOL,
    pub year: super::ULONG,
    pub month: super::UCHAR,
    pub day: super::UCHAR,
    pub hour: super::UCHAR,
    pub minute: super::UCHAR,
    pub second: super::UCHAR,
    pub millisecond: super::UINT,
    pub TZIsLocal: windows_core::BOOL,
    pub TZIsPositive: windows_core::BOOL,
    pub TZHour: super::UCHAR,
    pub TZMinute: super::UCHAR,
}
pub const WSD_DEFAULT_EVENTING_ADDRESS: windows_core::PCWSTR = windows_core::w!("http://*:5357/");
pub const WSD_DEFAULT_HOSTING_ADDRESS: windows_core::PCWSTR = windows_core::w!("http://*:5357/");
pub const WSD_DEFAULT_SECURE_HOSTING_ADDRESS: windows_core::PCWSTR = windows_core::w!("https://*:5358/");
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_DURATION {
    pub isPositive: windows_core::BOOL,
    pub year: super::ULONG,
    pub month: super::ULONG,
    pub day: super::ULONG,
    pub hour: super::ULONG,
    pub minute: super::ULONG,
    pub second: super::ULONG,
    pub millisecond: super::ULONG,
}
#[cfg(feature = "shared")]
pub const WSD_Data: super::DWORD = 327701;
#[cfg(feature = "shared")]
pub const WSD_Device: super::DWORD = 327680;
#[cfg(feature = "shared")]
pub type WSD_EVENTING_EXPIRES = _WSD_EVENTING_EXPIRES;
#[cfg(feature = "winnt")]
pub type WSD_EVENTING_FILTER = _WSD_EVENTING_FILTER;
#[cfg(feature = "winnt")]
pub type WSD_EVENTING_FILTER_ACTION = _WSD_EVENTING_FILTER_ACTION;
#[cfg(feature = "shared")]
pub const WSD_Element: super::DWORD = 327683;
#[cfg(feature = "shared")]
pub const WSD_FilterActionNotSupported: super::DWORD = 327681;
#[cfg(feature = "shared")]
pub const WSD_FirmwareVersion: super::DWORD = 327693;
#[cfg(feature = "shared")]
pub const WSD_FriendlyName: super::DWORD = 327692;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_HEADER_RELATESTO = _WSD_HEADER_RELATESTO;
#[cfg(feature = "shared")]
pub const WSD_Host: super::DWORD = 327697;
#[cfg(feature = "shared")]
pub const WSD_Hosted: super::DWORD = 327698;
#[cfg(feature = "winnt")]
pub type WSD_LOCALIZED_STRING = _WSD_LOCALIZED_STRING;
#[cfg(feature = "winnt")]
pub type WSD_LOCALIZED_STRING_LIST = _WSD_LOCALIZED_STRING_LIST;
#[cfg(feature = "shared")]
pub const WSD_Manufacturer: super::DWORD = 327685;
#[cfg(feature = "shared")]
pub const WSD_ManufacturerUrl: super::DWORD = 327686;
#[cfg(feature = "shared")]
pub const WSD_ModelName: super::DWORD = 327687;
#[cfg(feature = "shared")]
pub const WSD_ModelNumber: super::DWORD = 327688;
#[cfg(feature = "shared")]
pub const WSD_ModelUrl: super::DWORD = 327689;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_NAME_LIST = _WSD_NAME_LIST;
pub type WSD_PROTOCOL_TYPE = i32;
pub const WSD_PT_ALL: WSD_PROTOCOL_TYPE = 255;
pub const WSD_PT_HTTP: WSD_PROTOCOL_TYPE = 2;
pub const WSD_PT_HTTPS: WSD_PROTOCOL_TYPE = 4;
pub const WSD_PT_NONE: WSD_PROTOCOL_TYPE = 0;
pub const WSD_PT_UDP: WSD_PROTOCOL_TYPE = 1;
#[cfg(feature = "shared")]
pub const WSD_PresentationUrl: super::DWORD = 327690;
#[cfg(feature = "shared")]
pub const WSD_Relationship: super::DWORD = 327699;
#[cfg(feature = "winnt")]
pub type WSD_SCOPES = _WSD_SCOPES;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut super::PCCERT_CONTEXT,
    pub dwCertMatchArrayCount: super::DWORD,
    pub hCertMatchStore: super::HCERTSTORE,
    pub hCertIssuerStore: super::HCERTSTORE,
    pub dwCertCheckOptions: super::DWORD,
    pub pszCNGHashAlgId: windows_core::PCWSTR,
    pub pbCertHash: *mut super::BYTE,
    pub dwCertHashSize: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut super::PCCERT_CONTEXT,
    pub dwCertMatchArrayCount: super::DWORD,
    pub hCertMatchStore: super::HCERTSTORE,
    pub hCertIssuerStore: super::HCERTSTORE,
    pub dwCertCheckOptions: super::DWORD,
}
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = 7;
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = 8;
pub type WSD_SECURITY_HTTP_AUTH_SCHEMES = u32;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: i32 = 1;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: i32 = 2;
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 12;
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 11;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "wincrypt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut super::PCCERT_CONTEXT,
    pub dwSigningCertArrayCount: super::DWORD,
    pub hSigningCertStore: super::HCERTSTORE,
    pub dwFlags: super::DWORD,
}
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 3;
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 5;
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = 6;
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 4;
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 13;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SOAP_FAULT_CODE = _WSD_SOAP_FAULT_CODE;
#[cfg(feature = "winnt")]
pub type WSD_SOAP_FAULT_REASON = _WSD_SOAP_FAULT_REASON;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WSD_SOAP_FAULT_SUBCODE = _WSD_SOAP_FAULT_SUBCODE;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WSD_SYNCHRONOUS_RESPONSE_CONTEXT {
    pub hr: windows_core::HRESULT,
    pub eventHandle: super::HANDLE,
    pub messageParameters: core::mem::ManuallyDrop<Option<IWSDMessageParameters>>,
    pub results: *mut core::ffi::c_void,
}
#[cfg(feature = "shared")]
pub const WSD_SerialNumber: super::DWORD = 327694;
#[cfg(feature = "shared")]
pub const WSD_ServiceId: super::DWORD = 327696;
#[cfg(feature = "shared")]
pub const WSD_ThisDevice: super::DWORD = 327691;
#[cfg(feature = "shared")]
pub const WSD_ThisModel: super::DWORD = 327684;
#[cfg(feature = "shared")]
pub const WSD_Type: super::DWORD = 327700;
#[cfg(feature = "shared")]
pub const WSD_Types: super::DWORD = 327695;
#[cfg(feature = "winnt")]
pub type WSD_URI_LIST = _WSD_URI_LIST;
#[cfg(feature = "shared")]
pub const WSE_Any: super::DWORD = 27;
#[cfg(feature = "shared")]
pub const WSE_Any_1: super::DWORD = 29;
#[cfg(feature = "shared")]
pub const WSE_Data: super::DWORD = 19;
#[cfg(feature = "shared")]
pub const WSE_DateTime: super::DWORD = 22;
#[cfg(feature = "shared")]
pub const WSE_Delivery: super::DWORD = 20;
#[cfg(feature = "shared")]
pub const WSE_DeliveryModeRequestedUnavailable: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const WSE_Dialect: super::DWORD = 24;
#[cfg(feature = "shared")]
pub const WSE_Duration: super::DWORD = 21;
#[cfg(feature = "shared")]
pub const WSE_EndTo: super::DWORD = 15;
#[cfg(feature = "shared")]
pub const WSE_EventSink: super::DWORD = 36;
#[cfg(feature = "shared")]
pub const WSE_EventSourceUnableToProcess: super::DWORD = 6;
#[cfg(feature = "shared")]
pub const WSE_Eventing: super::DWORD = 11;
#[cfg(feature = "shared")]
pub const WSE_Expires: super::DWORD = 23;
#[cfg(feature = "shared")]
pub const WSE_Filter: super::DWORD = 26;
#[cfg(feature = "shared")]
pub const WSE_FilterAction: super::DWORD = 25;
#[cfg(feature = "shared")]
pub const WSE_FilteringNotSupported: super::DWORD = 4;
#[cfg(feature = "shared")]
pub const WSE_FilteringRequestedUnavailable: super::DWORD = 5;
#[cfg(feature = "shared")]
pub const WSE_GetStatus: super::DWORD = 32;
#[cfg(feature = "shared")]
pub const WSE_GetStatusResponse: super::DWORD = 33;
#[cfg(feature = "shared")]
pub const WSE_Identifier: super::DWORD = 0;
#[cfg(feature = "shared")]
pub const WSE_InvalidExpirationTime: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const WSE_InvalidMessage: super::DWORD = 8;
#[cfg(feature = "shared")]
pub const WSE_Mode: super::DWORD = 16;
#[cfg(feature = "shared")]
pub const WSE_NotifyTo: super::DWORD = 18;
#[cfg(feature = "shared")]
pub const WSE_Push: super::DWORD = 17;
#[cfg(feature = "shared")]
pub const WSE_Reason: super::DWORD = 40;
#[cfg(feature = "shared")]
pub const WSE_Renew: super::DWORD = 30;
#[cfg(feature = "shared")]
pub const WSE_RenewResponse: super::DWORD = 31;
#[cfg(feature = "shared")]
pub const WSE_ReturnValue: super::DWORD = 14;
#[cfg(feature = "shared")]
pub const WSE_Status: super::DWORD = 38;
#[cfg(feature = "shared")]
pub const WSE_String: super::DWORD = 39;
#[cfg(feature = "shared")]
pub const WSE_Subscribe: super::DWORD = 12;
#[cfg(feature = "shared")]
pub const WSE_SubscribeResponse: super::DWORD = 13;
#[cfg(feature = "shared")]
pub const WSE_SubscriptionEnd: super::DWORD = 37;
#[cfg(feature = "shared")]
pub const WSE_SubscriptionManager: super::DWORD = 28;
#[cfg(feature = "shared")]
pub const WSE_SupportedDeliveryMode: super::DWORD = 10;
#[cfg(feature = "shared")]
pub const WSE_SupportedDialect: super::DWORD = 9;
#[cfg(feature = "shared")]
pub const WSE_UnableToRenew: super::DWORD = 7;
#[cfg(feature = "shared")]
pub const WSE_Unsubscribe: super::DWORD = 34;
#[cfg(feature = "shared")]
pub const WSE_UnsubscribeResponse: super::DWORD = 35;
#[cfg(feature = "shared")]
pub const WSE_UnsupportedExpirationTime: super::DWORD = 3;
#[cfg(feature = "shared")]
pub const WSX_Data: super::DWORD = 393223;
#[cfg(feature = "shared")]
pub const WSX_Dialect: super::DWORD = 393221;
#[cfg(feature = "shared")]
pub const WSX_Element: super::DWORD = 393226;
#[cfg(feature = "shared")]
pub const WSX_GetMetadata: super::DWORD = 393217;
#[cfg(feature = "shared")]
pub const WSX_GetMetadataResponse: super::DWORD = 393218;
#[cfg(feature = "shared")]
pub const WSX_Identifier: super::DWORD = 393222;
#[cfg(feature = "shared")]
pub const WSX_Location: super::DWORD = 393225;
#[cfg(feature = "shared")]
pub const WSX_Metadata: super::DWORD = 393227;
#[cfg(feature = "shared")]
pub const WSX_MetadataReference: super::DWORD = 393224;
#[cfg(feature = "shared")]
pub const WSX_MetadataSection: super::DWORD = 393220;
#[cfg(feature = "shared")]
pub const WSX_Mex: super::DWORD = 393216;
#[cfg(feature = "shared")]
pub const WSX_ReturnValue: super::DWORD = 393219;
#[cfg(feature = "shared")]
pub const XML_Lang: super::DWORD = 262144;
#[cfg(feature = "shared")]
pub const XML_Space: super::DWORD = 262145;
#[cfg(feature = "shared")]
pub const XOP_Href: super::DWORD = 196609;
#[cfg(feature = "shared")]
pub const XOP_Include: super::DWORD = 196608;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSDXML_NAME {
    pub Space: *mut WSDXML_NAMESPACE,
    pub LocalName: *mut super::WCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSDXML_NAMESPACE {
    pub Uri: *const super::WCHAR,
    pub PreferredPrefix: *const super::WCHAR,
    pub Names: *mut WSDXML_NAME,
    pub NamesCount: super::WORD,
    pub Encoding: super::WORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSDXML_PREFIX_MAPPING {
    pub Refs: super::DWORD,
    pub Next: *mut WSDXML_PREFIX_MAPPING,
    pub Space: *mut WSDXML_NAMESPACE,
    pub Prefix: *mut super::WCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSDXML_TYPE {
    pub Uri: *const super::WCHAR,
    pub Table: *const super::BYTE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_APP_SEQUENCE {
    pub InstanceId: super::ULONGLONG,
    pub SequenceId: *const super::WCHAR,
    pub MessageNumber: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_EVENTING_EXPIRES {
    pub Duration: *mut WSD_DURATION,
    pub DateTime: *mut WSD_DATETIME,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_EVENTING_FILTER {
    pub Dialect: *const super::WCHAR,
    pub FilterAction: *mut WSD_EVENTING_FILTER_ACTION,
    pub Data: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_EVENTING_FILTER_ACTION {
    pub Actions: *mut WSD_URI_LIST,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_HEADER_RELATESTO {
    pub RelationshipType: *mut WSDXML_NAME,
    pub MessageID: *const super::WCHAR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_LOCALIZED_STRING {
    pub lang: *const super::WCHAR,
    pub String: *const super::WCHAR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_LOCALIZED_STRING_LIST {
    pub Next: *mut WSD_LOCALIZED_STRING_LIST,
    pub Element: *mut WSD_LOCALIZED_STRING,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_NAME_LIST {
    pub Next: *mut WSD_NAME_LIST,
    pub Element: *mut WSDXML_NAME,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_SCOPES {
    pub MatchBy: *const super::WCHAR,
    pub Scopes: *mut WSD_URI_LIST,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_SOAP_FAULT_CODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_SOAP_FAULT_REASON {
    pub Text: *mut WSD_LOCALIZED_STRING_LIST,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_SOAP_FAULT_SUBCODE {
    pub Value: *mut WSDXML_NAME,
    pub Subcode: *mut WSD_SOAP_FAULT_SUBCODE,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _WSD_URI_LIST {
    pub Next: *mut WSD_URI_LIST,
    pub Element: *const super::WCHAR,
}
