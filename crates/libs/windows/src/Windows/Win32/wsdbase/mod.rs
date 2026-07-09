#[inline]
pub unsafe fn WSDCreateHttpAddress() -> windows_core::Result<IWSDHttpAddress> {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateHttpAddress(ppaddress : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateHttpAddress(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateHttpMessageParameters() -> windows_core::Result<IWSDHttpMessageParameters> {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateHttpMessageParameters(pptxparams : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateHttpMessageParameters(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateUdpAddress() -> windows_core::Result<IWSDUdpAddress> {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateUdpAddress(ppaddress : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateUdpAddress(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn WSDCreateUdpMessageParameters() -> windows_core::Result<IWSDUdpMessageParameters> {
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateUdpMessageParameters(pptxparams : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateUdpMessageParameters(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
windows_core::imp::define_interface!(IWSDAddress, IWSDAddress_Vtbl, 0xb9574c6c_12a6_4f74_93a1_3318ff605759);
windows_core::imp::interface_hierarchy!(IWSDAddress, windows_core::IUnknown);
impl IWSDAddress {
    pub unsafe fn Serialize(&self, pszbuffer: &mut [u16], fsafe: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Serialize)(windows_core::Interface::as_raw(self), core::mem::transmute(pszbuffer.as_ptr()), pszbuffer.len().try_into().unwrap(), fsafe.into()) }
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
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub Deserialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IWSDAddress_Impl: windows_core::IUnknownImpl {
    fn Serialize(&self, pszbuffer: windows_core::PWSTR, cchlength: u32, fsafe: windows_core::BOOL) -> windows_core::Result<()>;
    fn Deserialize(&self, pszbuffer: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IWSDAddress_Vtbl {
    pub const fn new<Identity: IWSDAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Serialize<Identity: IWSDAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbuffer: windows_core::PWSTR, cchlength: u32, fsafe: windows_core::BOOL) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IWSDAddress {}
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
pub trait IWSDHttpAddress_Impl: IWSDTransportAddress_Impl {
    fn GetSecure(&self) -> windows_core::Result<()>;
    fn SetSecure(&self, fsecure: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetPath(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetPath(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
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
impl windows_core::RuntimeName for IWSDHttpAddress {}
windows_core::imp::define_interface!(IWSDHttpAuthParameters, IWSDHttpAuthParameters_Vtbl, 0x0b476df0_8dac_480d_b05c_99781a5884aa);
windows_core::imp::interface_hierarchy!(IWSDHttpAuthParameters, windows_core::IUnknown);
impl IWSDHttpAuthParameters {
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetClientAccessToken(&self) -> windows_core::Result<super::winnt::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClientAccessToken)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
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
    #[cfg(feature = "Win32_winnt")]
    pub GetClientAccessToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetClientAccessToken: usize,
    pub GetAuthType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_winnt")]
pub trait IWSDHttpAuthParameters_Impl: windows_core::IUnknownImpl {
    fn GetClientAccessToken(&self) -> windows_core::Result<super::winnt::HANDLE>;
    fn GetAuthType(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "Win32_winnt")]
impl IWSDHttpAuthParameters_Vtbl {
    pub const fn new<Identity: IWSDHttpAuthParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientAccessToken<Identity: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phtoken: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDHttpAuthParameters_Impl::GetClientAccessToken(this) {
                    Ok(ok__) => {
                        phtoken.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAuthType<Identity: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthtype: *mut u32) -> windows_core::HRESULT {
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
#[cfg(feature = "Win32_winnt")]
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
            (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
windows_core::imp::define_interface!(IWSDMessageParameters, IWSDMessageParameters_Vtbl, 0x1fafe8a2_e6fc_4b80_b6cf_b7d45c416d7c);
windows_core::imp::interface_hierarchy!(IWSDMessageParameters, windows_core::IUnknown);
impl IWSDMessageParameters {
    pub unsafe fn GetLocalAddress(&self) -> windows_core::Result<IWSDAddress> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLocalAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(self).GetRemoteAddress)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(self).GetLowerParameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
windows_core::imp::define_interface!(IWSDSSLClientCertificate, IWSDSSLClientCertificate_Vtbl, 0xde105e87_a0da_418e_98ad_27b9eed87bdc);
windows_core::imp::interface_hierarchy!(IWSDSSLClientCertificate, windows_core::IUnknown);
impl IWSDSSLClientCertificate {
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
    pub unsafe fn GetClientCertificate(&self, ppcertcontext: *mut super::wincrypt::PCCERT_CONTEXT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClientCertificate)(windows_core::Interface::as_raw(self), ppcertcontext as _) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn GetMappedAccessToken(&self) -> windows_core::Result<super::winnt::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMappedAccessToken)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSSLClientCertificate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
    pub GetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wincrypt::PCCERT_CONTEXT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_wincrypt")))]
    GetClientCertificate: usize,
    #[cfg(feature = "Win32_winnt")]
    pub GetMappedAccessToken: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    GetMappedAccessToken: usize,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
pub trait IWSDSSLClientCertificate_Impl: windows_core::IUnknownImpl {
    fn GetClientCertificate(&self, ppcertcontext: *mut super::wincrypt::PCCERT_CONTEXT) -> windows_core::Result<()>;
    fn GetMappedAccessToken(&self) -> windows_core::Result<super::winnt::HANDLE>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
impl IWSDSSLClientCertificate_Vtbl {
    pub const fn new<Identity: IWSDSSLClientCertificate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientCertificate<Identity: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcertcontext: *mut super::wincrypt::PCCERT_CONTEXT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDSSLClientCertificate_Impl::GetClientCertificate(this, core::mem::transmute_copy(&ppcertcontext)).into()
            }
        }
        unsafe extern "system" fn GetMappedAccessToken<Identity: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phtoken: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDSSLClientCertificate_Impl::GetMappedAccessToken(this) {
                    Ok(ok__) => {
                        phtoken.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
impl windows_core::RuntimeName for IWSDSSLClientCertificate {}
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
    pub unsafe fn GetKeyInfo(&self, pbkeyinfo: Option<*mut u8>, pdwkeyinfosize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetKeyInfo)(windows_core::Interface::as_raw(self), pbkeyinfo.unwrap_or(core::mem::zeroed()) as _, pdwkeyinfosize as _) }
    }
    pub unsafe fn GetSignature(&self, pbsignature: Option<*mut u8>, pdwsignaturesize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSignature)(windows_core::Interface::as_raw(self), pbsignature.unwrap_or(core::mem::zeroed()) as _, pdwsignaturesize as _) }
    }
    pub unsafe fn GetSignedInfoHash(&self, pbsignedinfohash: Option<*mut u8>, pdwhashsize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSignedInfoHash)(windows_core::Interface::as_raw(self), pbsignedinfohash.unwrap_or(core::mem::zeroed()) as _, pdwhashsize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDSignatureProperty_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsMessageSigned: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsMessageSignatureTrusted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetKeyInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetSignedInfoHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IWSDSignatureProperty_Impl: windows_core::IUnknownImpl {
    fn IsMessageSigned(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsMessageSignatureTrusted(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetKeyInfo(&self, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> windows_core::Result<()>;
    fn GetSignature(&self, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> windows_core::Result<()>;
    fn GetSignedInfoHash(&self, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> windows_core::Result<()>;
}
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
        unsafe extern "system" fn GetKeyInfo<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDSignatureProperty_Impl::GetKeyInfo(this, core::mem::transmute_copy(&pbkeyinfo), core::mem::transmute_copy(&pdwkeyinfosize)).into()
            }
        }
        unsafe extern "system" fn GetSignature<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDSignatureProperty_Impl::GetSignature(this, core::mem::transmute_copy(&pbsignature), core::mem::transmute_copy(&pdwsignaturesize)).into()
            }
        }
        unsafe extern "system" fn GetSignedInfoHash<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> windows_core::HRESULT {
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
    pub unsafe fn GetPort(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPort)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPort(&self, wport: u16) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPort)(windows_core::Interface::as_raw(self), wport) }
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
    pub GetPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub SetPort: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub GetTransportAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetTransportAddressEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetTransportAddress: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IWSDTransportAddress_Impl: IWSDAddress_Impl {
    fn GetPort(&self) -> windows_core::Result<u16>;
    fn SetPort(&self, wport: u16) -> windows_core::Result<()>;
    fn GetTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetTransportAddressEx(&self, fsafe: windows_core::BOOL) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetTransportAddress(&self, pszaddress: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IWSDTransportAddress_Vtbl {
    pub const fn new<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPort<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwport: *mut u16) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetPort<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wport: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDTransportAddress_Impl::SetPort(this, core::mem::transmute_copy(&wport)).into()
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
            SetPort: SetPort::<Identity, OFFSET>,
            GetTransportAddress: GetTransportAddress::<Identity, OFFSET>,
            GetTransportAddressEx: GetTransportAddressEx::<Identity, OFFSET>,
            SetTransportAddress: SetTransportAddress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDTransportAddress as windows_core::Interface>::IID || iid == &<IWSDAddress as windows_core::Interface>::IID
    }
}
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
    #[cfg(feature = "Win32_ws2def")]
    pub unsafe fn SetSockaddr(&self, psockaddr: *const super::ws2def::SOCKADDR_STORAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSockaddr)(windows_core::Interface::as_raw(self), psockaddr) }
    }
    #[cfg(feature = "Win32_ws2def")]
    pub unsafe fn GetSockaddr(&self, psockaddr: *mut super::ws2def::SOCKADDR_STORAGE) -> windows_core::HRESULT {
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
    pub unsafe fn SetTTL(&self, dwttl: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTTL)(windows_core::Interface::as_raw(self), dwttl) }
    }
    pub unsafe fn GetTTL(&self) -> windows_core::Result<u32> {
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
    #[cfg(feature = "Win32_ws2def")]
    pub SetSockaddr: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::ws2def::SOCKADDR_STORAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_ws2def"))]
    SetSockaddr: usize,
    #[cfg(feature = "Win32_ws2def")]
    pub GetSockaddr: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ws2def::SOCKADDR_STORAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_ws2def"))]
    GetSockaddr: usize,
    pub SetExclusive: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetExclusive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMessageType: unsafe extern "system" fn(*mut core::ffi::c_void, WSDUdpMessageType) -> windows_core::HRESULT,
    pub GetMessageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSDUdpMessageType) -> windows_core::HRESULT,
    pub SetTTL: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetTTL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAlias: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetAlias: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_ws2def")]
pub trait IWSDUdpAddress_Impl: IWSDTransportAddress_Impl {
    fn SetSockaddr(&self, psockaddr: *const super::ws2def::SOCKADDR_STORAGE) -> windows_core::Result<()>;
    fn GetSockaddr(&self, psockaddr: *mut super::ws2def::SOCKADDR_STORAGE) -> windows_core::Result<()>;
    fn SetExclusive(&self, fexclusive: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetExclusive(&self) -> windows_core::Result<()>;
    fn SetMessageType(&self, messagetype: WSDUdpMessageType) -> windows_core::Result<()>;
    fn GetMessageType(&self) -> windows_core::Result<WSDUdpMessageType>;
    fn SetTTL(&self, dwttl: u32) -> windows_core::Result<()>;
    fn GetTTL(&self) -> windows_core::Result<u32>;
    fn SetAlias(&self, palias: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetAlias(&self) -> windows_core::Result<windows_core::GUID>;
}
#[cfg(feature = "Win32_ws2def")]
impl IWSDUdpAddress_Vtbl {
    pub const fn new<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSockaddr<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psockaddr: *const super::ws2def::SOCKADDR_STORAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::SetSockaddr(this, core::mem::transmute_copy(&psockaddr)).into()
            }
        }
        unsafe extern "system" fn GetSockaddr<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psockaddr: *mut super::ws2def::SOCKADDR_STORAGE) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetTTL<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwttl: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDUdpAddress_Impl::SetTTL(this, core::mem::transmute_copy(&dwttl)).into()
            }
        }
        unsafe extern "system" fn GetTTL<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwttl: *mut u32) -> windows_core::HRESULT {
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
#[cfg(feature = "Win32_ws2def")]
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
    pub unsafe fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRetransmitParams)(windows_core::Interface::as_raw(self), pparams) }
    }
    pub unsafe fn GetRetransmitParams(&self, pparams: *mut WSDUdpRetransmitParams) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRetransmitParams)(windows_core::Interface::as_raw(self), pparams as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDUdpMessageParameters_Vtbl {
    pub base__: IWSDMessageParameters_Vtbl,
    pub SetRetransmitParams: unsafe extern "system" fn(*mut core::ffi::c_void, *const WSDUdpRetransmitParams) -> windows_core::HRESULT,
    pub GetRetransmitParams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WSDUdpRetransmitParams) -> windows_core::HRESULT,
}
pub trait IWSDUdpMessageParameters_Impl: IWSDMessageParameters_Impl {
    fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> windows_core::Result<()>;
    fn GetRetransmitParams(&self, pparams: *mut WSDUdpRetransmitParams) -> windows_core::Result<()>;
}
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
impl windows_core::RuntimeName for IWSDUdpMessageParameters {}
pub const ONE_WAY: WSDUdpMessageType = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWSD_CONFIG_ADDRESSES(pub *mut WSD_CONFIG_ADDRESSES);
impl PWSD_CONFIG_ADDRESSES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWSD_CONFIG_ADDRESSES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWSD_CONFIG_PARAM(pub *mut WSD_CONFIG_PARAM);
impl PWSD_CONFIG_PARAM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWSD_CONFIG_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWSD_SECURITY_CERT_VALIDATION(pub *mut WSD_SECURITY_CERT_VALIDATION);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
impl PWSD_SECURITY_CERT_VALIDATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
impl Default for PWSD_SECURITY_CERT_VALIDATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWSD_SECURITY_HTTP_AUTH_SCHEMES(pub *mut u32);
impl PWSD_SECURITY_HTTP_AUTH_SCHEMES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWSD_SECURITY_HTTP_AUTH_SCHEMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWSD_SECURITY_SIGNATURE_VALIDATION(pub *mut WSD_SECURITY_SIGNATURE_VALIDATION);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
impl PWSD_SECURITY_SIGNATURE_VALIDATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
impl Default for PWSD_SECURITY_SIGNATURE_VALIDATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TWO_WAY: WSDUdpMessageType = 1;
pub const WSDAPI_ADDRESSFAMILY_IPV4: u32 = 1;
pub const WSDAPI_ADDRESSFAMILY_IPV6: u32 = 2;
pub const WSDAPI_COMPACTSIG_ACCEPT_ALL_MESSAGES: u32 = 1;
pub const WSDAPI_SSL_CERT_APPLY_DEFAULT_CHECKS: u32 = 0;
pub const WSDAPI_SSL_CERT_IGNORE_EXPIRY: u32 = 2;
pub const WSDAPI_SSL_CERT_IGNORE_INVALID_CN: u32 = 16;
pub const WSDAPI_SSL_CERT_IGNORE_REVOCATION: u32 = 1;
pub const WSDAPI_SSL_CERT_IGNORE_UNKNOWN_CA: u32 = 8;
pub const WSDAPI_SSL_CERT_IGNORE_WRONG_USAGE: u32 = 4;
pub type WSDUdpMessageType = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WSDUdpRetransmitParams {
    pub ulSendDelay: u32,
    pub ulRepeat: u32,
    pub ulRepeatMinDelay: u32,
    pub ulRepeatMaxDelay: u32,
    pub ulRepeatUpperDelay: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSD_CONFIG_ADDRESSES {
    pub addresses: *mut Option<IWSDAddress>,
    pub dwAddressCount: u32,
}
impl Default for WSD_CONFIG_ADDRESSES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_CONFIG_DEVICE_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 10;
pub const WSD_CONFIG_HOSTING_ADDRESSES: WSD_CONFIG_PARAM_TYPE = 9;
pub const WSD_CONFIG_MAX_INBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 1;
pub const WSD_CONFIG_MAX_OUTBOUND_MESSAGE_SIZE: WSD_CONFIG_PARAM_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSD_CONFIG_PARAM {
    pub configParamType: WSD_CONFIG_PARAM_TYPE,
    pub pConfigData: *mut core::ffi::c_void,
    pub dwConfigDataSize: u32,
}
impl Default for WSD_CONFIG_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WSD_CONFIG_PARAM_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSD_SECURITY_CERT_VALIDATION {
    pub certMatchArray: *mut super::wincrypt::PCCERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::wincrypt::HCERTSTORE,
    pub hCertIssuerStore: super::wincrypt::HCERTSTORE,
    pub dwCertCheckOptions: u32,
    pub pszCNGHashAlgId: windows_core::PCWSTR,
    pub pbCertHash: *mut u8,
    pub dwCertHashSize: u32,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
impl Default for WSD_SECURITY_CERT_VALIDATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSD_SECURITY_CERT_VALIDATION_V1 {
    pub certMatchArray: *mut super::wincrypt::PCCERT_CONTEXT,
    pub dwCertMatchArrayCount: u32,
    pub hCertMatchStore: super::wincrypt::HCERTSTORE,
    pub hCertIssuerStore: super::wincrypt::HCERTSTORE,
    pub dwCertCheckOptions: u32,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
impl Default for WSD_SECURITY_CERT_VALIDATION_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_SECURITY_COMPACTSIG_SIGNING_CERT: WSD_CONFIG_PARAM_TYPE = 7;
pub const WSD_SECURITY_COMPACTSIG_VALIDATION: WSD_CONFIG_PARAM_TYPE = 8;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct WSD_SECURITY_HTTP_AUTH_SCHEMES(pub u32);
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NEGOTIATE: u32 = 1;
pub const WSD_SECURITY_HTTP_AUTH_SCHEME_NTLM: u32 = 2;
pub const WSD_SECURITY_REQUIRE_CLIENT_CERT_OR_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 12;
pub const WSD_SECURITY_REQUIRE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 11;
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WSD_SECURITY_SIGNATURE_VALIDATION {
    pub signingCertArray: *mut super::wincrypt::PCCERT_CONTEXT,
    pub dwSigningCertArrayCount: u32,
    pub hSigningCertStore: super::wincrypt::HCERTSTORE,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt"))]
impl Default for WSD_SECURITY_SIGNATURE_VALIDATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WSD_SECURITY_SSL_CERT_FOR_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 3;
pub const WSD_SECURITY_SSL_CLIENT_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 5;
pub const WSD_SECURITY_SSL_NEGOTIATE_CLIENT_CERT: WSD_CONFIG_PARAM_TYPE = 6;
pub const WSD_SECURITY_SSL_SERVER_CERT_VALIDATION: WSD_CONFIG_PARAM_TYPE = 4;
pub const WSD_SECURITY_USE_HTTP_CLIENT_AUTH: WSD_CONFIG_PARAM_TYPE = 13;
