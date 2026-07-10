windows_core::imp::define_interface!(IWbemDecoupledBasicEventProvider, IWbemDecoupledBasicEventProvider_Vtbl, 0x86336d20_ca11_4786_9ef1_bc8a946b42fc);
impl core::ops::Deref for IWbemDecoupledBasicEventProvider {
    type Target = IWbemDecoupledRegistrar;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWbemDecoupledBasicEventProvider, windows_core::IUnknown, IWbemDecoupledRegistrar);
impl IWbemDecoupledBasicEventProvider {
    #[cfg(feature = "wbemcli")]
    pub unsafe fn GetSink<P1>(&self, a_flags: i32, a_context: P1) -> windows_core::Result<super::wbemcli::IWbemObjectSink>
    where
        P1: windows_core::Param<super::wbemcli::IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSink)(windows_core::Interface::as_raw(self), a_flags, a_context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wbemcli")]
    pub unsafe fn GetService<P1>(&self, a_flags: i32, a_context: P1) -> windows_core::Result<super::wbemcli::IWbemServices>
    where
        P1: windows_core::Param<super::wbemcli::IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), a_flags, a_context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemDecoupledBasicEventProvider_Vtbl {
    pub base__: IWbemDecoupledRegistrar_Vtbl,
    #[cfg(feature = "wbemcli")]
    pub GetSink: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    GetSink: usize,
    #[cfg(feature = "wbemcli")]
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    GetService: usize,
}
#[cfg(feature = "wbemcli")]
pub trait IWbemDecoupledBasicEventProvider_Impl: IWbemDecoupledRegistrar_Impl {
    fn GetSink(&self, a_flags: i32, a_context: windows_core::Ref<super::wbemcli::IWbemContext>) -> windows_core::Result<super::wbemcli::IWbemObjectSink>;
    fn GetService(&self, a_flags: i32, a_context: windows_core::Ref<super::wbemcli::IWbemContext>) -> windows_core::Result<super::wbemcli::IWbemServices>;
}
#[cfg(feature = "wbemcli")]
impl IWbemDecoupledBasicEventProvider_Vtbl {
    pub const fn new<Identity: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSink<Identity: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, a_flags: i32, a_context: *mut core::ffi::c_void, a_sink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemDecoupledBasicEventProvider_Impl::GetSink(this, core::mem::transmute_copy(&a_flags), core::mem::transmute_copy(&a_context)) {
                    Ok(ok__) => {
                        a_sink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetService<Identity: IWbemDecoupledBasicEventProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, a_flags: i32, a_context: *mut core::ffi::c_void, a_service: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemDecoupledBasicEventProvider_Impl::GetService(this, core::mem::transmute_copy(&a_flags), core::mem::transmute_copy(&a_context)) {
                    Ok(ok__) => {
                        a_service.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWbemDecoupledRegistrar_Vtbl::new::<Identity, OFFSET>(),
            GetSink: GetSink::<Identity, OFFSET>,
            GetService: GetService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemDecoupledBasicEventProvider as windows_core::Interface>::IID || iid == &<IWbemDecoupledRegistrar as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wbemcli")]
impl windows_core::RuntimeName for IWbemDecoupledBasicEventProvider {}
windows_core::imp::define_interface!(IWbemDecoupledRegistrar, IWbemDecoupledRegistrar_Vtbl, 0x1005cbcf_e64f_4646_bcd3_3a089d8a84b4);
windows_core::imp::interface_hierarchy!(IWbemDecoupledRegistrar, windows_core::IUnknown);
impl IWbemDecoupledRegistrar {
    #[cfg(feature = "wbemcli")]
    pub unsafe fn Register<P1, P2, P3, P4, P5, P6>(&self, a_flags: i32, a_context: P1, a_user: P2, a_locale: P3, a_scope: P4, a_registration: P5, piunknown: P6) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::wbemcli::IWbemContext>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), a_flags, a_context.param().abi(), a_user.param().abi(), a_locale.param().abi(), a_scope.param().abi(), a_registration.param().abi(), piunknown.param().abi()) }
    }
    pub unsafe fn UnRegister(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnRegister)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemDecoupledRegistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wbemcli")]
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    Register: usize,
    pub UnRegister: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "wbemcli")]
pub trait IWbemDecoupledRegistrar_Impl: windows_core::IUnknownImpl {
    fn Register(&self, a_flags: i32, a_context: windows_core::Ref<super::wbemcli::IWbemContext>, a_user: &windows_core::PCWSTR, a_locale: &windows_core::PCWSTR, a_scope: &windows_core::PCWSTR, a_registration: &windows_core::PCWSTR, piunknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn UnRegister(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "wbemcli")]
impl IWbemDecoupledRegistrar_Vtbl {
    pub const fn new<Identity: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Register<Identity: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, a_flags: i32, a_context: *mut core::ffi::c_void, a_user: windows_core::PCWSTR, a_locale: windows_core::PCWSTR, a_scope: windows_core::PCWSTR, a_registration: windows_core::PCWSTR, piunknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemDecoupledRegistrar_Impl::Register(this, core::mem::transmute_copy(&a_flags), core::mem::transmute_copy(&a_context), core::mem::transmute(&a_user), core::mem::transmute(&a_locale), core::mem::transmute(&a_scope), core::mem::transmute(&a_registration), core::mem::transmute_copy(&piunknown)).into()
            }
        }
        unsafe extern "system" fn UnRegister<Identity: IWbemDecoupledRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemDecoupledRegistrar_Impl::UnRegister(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, OFFSET>,
            UnRegister: UnRegister::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemDecoupledRegistrar as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wbemcli")]
impl windows_core::RuntimeName for IWbemDecoupledRegistrar {}
windows_core::imp::define_interface!(IWbemEventConsumerProvider, IWbemEventConsumerProvider_Vtbl, 0xe246107a_b06e_11d0_ad61_00c04fd8fdff);
windows_core::imp::interface_hierarchy!(IWbemEventConsumerProvider, windows_core::IUnknown);
impl IWbemEventConsumerProvider {
    #[cfg(feature = "wbemcli")]
    pub unsafe fn FindConsumer<P0>(&self, plogicalconsumer: P0) -> windows_core::Result<IWbemUnboundObjectSink>
    where
        P0: windows_core::Param<super::wbemcli::IWbemClassObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindConsumer)(windows_core::Interface::as_raw(self), plogicalconsumer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventConsumerProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wbemcli")]
    pub FindConsumer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    FindConsumer: usize,
}
#[cfg(feature = "wbemcli")]
pub trait IWbemEventConsumerProvider_Impl: windows_core::IUnknownImpl {
    fn FindConsumer(&self, plogicalconsumer: windows_core::Ref<super::wbemcli::IWbemClassObject>) -> windows_core::Result<IWbemUnboundObjectSink>;
}
#[cfg(feature = "wbemcli")]
impl IWbemEventConsumerProvider_Vtbl {
    pub const fn new<Identity: IWbemEventConsumerProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindConsumer<Identity: IWbemEventConsumerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogicalconsumer: *mut core::ffi::c_void, ppconsumer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemEventConsumerProvider_Impl::FindConsumer(this, core::mem::transmute_copy(&plogicalconsumer)) {
                    Ok(ok__) => {
                        ppconsumer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindConsumer: FindConsumer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventConsumerProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wbemcli")]
impl windows_core::RuntimeName for IWbemEventConsumerProvider {}
windows_core::imp::define_interface!(IWbemEventProvider, IWbemEventProvider_Vtbl, 0xe245105b_b06e_11d0_ad61_00c04fd8fdff);
windows_core::imp::interface_hierarchy!(IWbemEventProvider, windows_core::IUnknown);
impl IWbemEventProvider {
    #[cfg(feature = "wbemcli")]
    pub unsafe fn ProvideEvents<P0>(&self, psink: P0, lflags: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::wbemcli::IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProvideEvents)(windows_core::Interface::as_raw(self), psink.param().abi(), lflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wbemcli")]
    pub ProvideEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    ProvideEvents: usize,
}
#[cfg(feature = "wbemcli")]
pub trait IWbemEventProvider_Impl: windows_core::IUnknownImpl {
    fn ProvideEvents(&self, psink: windows_core::Ref<super::wbemcli::IWbemObjectSink>, lflags: i32) -> windows_core::Result<()>;
}
#[cfg(feature = "wbemcli")]
impl IWbemEventProvider_Vtbl {
    pub const fn new<Identity: IWbemEventProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProvideEvents<Identity: IWbemEventProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventProvider_Impl::ProvideEvents(this, core::mem::transmute_copy(&psink), core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ProvideEvents: ProvideEvents::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wbemcli")]
impl windows_core::RuntimeName for IWbemEventProvider {}
windows_core::imp::define_interface!(IWbemEventProviderQuerySink, IWbemEventProviderQuerySink_Vtbl, 0x580acaf8_fa1c_11d0_ad72_00c04fd8fdff);
windows_core::imp::interface_hierarchy!(IWbemEventProviderQuerySink, windows_core::IUnknown);
impl IWbemEventProviderQuerySink {
    pub unsafe fn NewQuery(&self, dwid: u32, wszquerylanguage: WBEM_WSTR, wszquery: WBEM_WSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NewQuery)(windows_core::Interface::as_raw(self), dwid, core::mem::transmute(wszquerylanguage), core::mem::transmute(wszquery)) }
    }
    pub unsafe fn CancelQuery(&self, dwid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelQuery)(windows_core::Interface::as_raw(self), dwid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProviderQuerySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NewQuery: unsafe extern "system" fn(*mut core::ffi::c_void, u32, WBEM_WSTR, WBEM_WSTR) -> windows_core::HRESULT,
    pub CancelQuery: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IWbemEventProviderQuerySink_Impl: windows_core::IUnknownImpl {
    fn NewQuery(&self, dwid: u32, wszquerylanguage: &WBEM_WSTR, wszquery: &WBEM_WSTR) -> windows_core::Result<()>;
    fn CancelQuery(&self, dwid: u32) -> windows_core::Result<()>;
}
impl IWbemEventProviderQuerySink_Vtbl {
    pub const fn new<Identity: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NewQuery<Identity: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: u32, wszquerylanguage: WBEM_WSTR, wszquery: WBEM_WSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventProviderQuerySink_Impl::NewQuery(this, core::mem::transmute_copy(&dwid), core::mem::transmute(&wszquerylanguage), core::mem::transmute(&wszquery)).into()
            }
        }
        unsafe extern "system" fn CancelQuery<Identity: IWbemEventProviderQuerySink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventProviderQuerySink_Impl::CancelQuery(this, core::mem::transmute_copy(&dwid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NewQuery: NewQuery::<Identity, OFFSET>,
            CancelQuery: CancelQuery::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventProviderQuerySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemEventProviderQuerySink {}
windows_core::imp::define_interface!(IWbemEventProviderSecurity, IWbemEventProviderSecurity_Vtbl, 0x631f7d96_d993_11d2_b339_00105a1f4aaf);
windows_core::imp::interface_hierarchy!(IWbemEventProviderSecurity, windows_core::IUnknown);
impl IWbemEventProviderSecurity {
    pub unsafe fn AccessCheck(&self, wszquerylanguage: WBEM_CWSTR, wszquery: WBEM_CWSTR, lsidlength: i32, psid: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AccessCheck)(windows_core::Interface::as_raw(self), core::mem::transmute(wszquerylanguage), core::mem::transmute(wszquery), lsidlength, psid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventProviderSecurity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AccessCheck: unsafe extern "system" fn(*mut core::ffi::c_void, WBEM_CWSTR, WBEM_CWSTR, i32, *const u8) -> windows_core::HRESULT,
}
pub trait IWbemEventProviderSecurity_Impl: windows_core::IUnknownImpl {
    fn AccessCheck(&self, wszquerylanguage: &WBEM_CWSTR, wszquery: &WBEM_CWSTR, lsidlength: i32, psid: *const u8) -> windows_core::Result<()>;
}
impl IWbemEventProviderSecurity_Vtbl {
    pub const fn new<Identity: IWbemEventProviderSecurity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AccessCheck<Identity: IWbemEventProviderSecurity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszquerylanguage: WBEM_CWSTR, wszquery: WBEM_CWSTR, lsidlength: i32, psid: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventProviderSecurity_Impl::AccessCheck(this, core::mem::transmute(&wszquerylanguage), core::mem::transmute(&wszquery), core::mem::transmute_copy(&lsidlength), core::mem::transmute_copy(&psid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AccessCheck: AccessCheck::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventProviderSecurity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemEventProviderSecurity {}
#[cfg(feature = "wbemcli")]
windows_core::imp::define_interface!(IWbemEventSink, IWbemEventSink_Vtbl, 0x3ae0080a_7e3a_4366_bf89_0feedc931659);
#[cfg(feature = "wbemcli")]
impl core::ops::Deref for IWbemEventSink {
    type Target = super::wbemcli::IWbemObjectSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "wbemcli")]
windows_core::imp::interface_hierarchy!(IWbemEventSink, windows_core::IUnknown, super::wbemcli::IWbemObjectSink);
#[cfg(feature = "wbemcli")]
impl IWbemEventSink {
    pub unsafe fn SetSinkSecurity(&self, lsdlength: i32, psd: *const u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSinkSecurity)(windows_core::Interface::as_raw(self), lsdlength, psd) }
    }
    pub unsafe fn IsActive(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsActive)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetRestrictedSink<P2>(&self, lnumqueries: i32, awszqueries: *const windows_core::PCWSTR, pcallback: P2) -> windows_core::Result<Self>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictedSink)(windows_core::Interface::as_raw(self), lnumqueries, awszqueries, pcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBatchingParameters(&self, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBatchingParameters)(windows_core::Interface::as_raw(self), lflags, dwmaxbuffersize, dwmaxsendlatency) }
    }
}
#[cfg(feature = "wbemcli")]
#[repr(C)]
#[doc(hidden)]
pub struct IWbemEventSink_Vtbl {
    pub base__: super::wbemcli::IWbemObjectSink_Vtbl,
    pub SetSinkSecurity: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const u8) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRestrictedSink: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::PCWSTR, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBatchingParameters: unsafe extern "system" fn(*mut core::ffi::c_void, i32, u32, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "wbemcli")]
pub trait IWbemEventSink_Impl: super::wbemcli::IWbemObjectSink_Impl {
    fn SetSinkSecurity(&self, lsdlength: i32, psd: *const u8) -> windows_core::Result<()>;
    fn IsActive(&self) -> windows_core::Result<()>;
    fn GetRestrictedSink(&self, lnumqueries: i32, awszqueries: *const windows_core::PCWSTR, pcallback: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<IWbemEventSink>;
    fn SetBatchingParameters(&self, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "wbemcli")]
impl IWbemEventSink_Vtbl {
    pub const fn new<Identity: IWbemEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSinkSecurity<Identity: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsdlength: i32, psd: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventSink_Impl::SetSinkSecurity(this, core::mem::transmute_copy(&lsdlength), core::mem::transmute_copy(&psd)).into()
            }
        }
        unsafe extern "system" fn IsActive<Identity: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventSink_Impl::IsActive(this).into()
            }
        }
        unsafe extern "system" fn GetRestrictedSink<Identity: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumqueries: i32, awszqueries: *const windows_core::PCWSTR, pcallback: *mut core::ffi::c_void, ppsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemEventSink_Impl::GetRestrictedSink(this, core::mem::transmute_copy(&lnumqueries), core::mem::transmute_copy(&awszqueries), core::mem::transmute_copy(&pcallback)) {
                    Ok(ok__) => {
                        ppsink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBatchingParameters<Identity: IWbemEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, dwmaxbuffersize: u32, dwmaxsendlatency: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemEventSink_Impl::SetBatchingParameters(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&dwmaxbuffersize), core::mem::transmute_copy(&dwmaxsendlatency)).into()
            }
        }
        Self {
            base__: super::wbemcli::IWbemObjectSink_Vtbl::new::<Identity, OFFSET>(),
            SetSinkSecurity: SetSinkSecurity::<Identity, OFFSET>,
            IsActive: IsActive::<Identity, OFFSET>,
            GetRestrictedSink: GetRestrictedSink::<Identity, OFFSET>,
            SetBatchingParameters: SetBatchingParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemEventSink as windows_core::Interface>::IID || iid == &<super::wbemcli::IWbemObjectSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wbemcli")]
impl windows_core::RuntimeName for IWbemEventSink {}
windows_core::imp::define_interface!(IWbemHiPerfProvider, IWbemHiPerfProvider_Vtbl, 0x49353c93_516b_11d1_aea6_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemHiPerfProvider, windows_core::IUnknown);
impl IWbemHiPerfProvider {
    #[cfg(feature = "wbemcli")]
    pub unsafe fn QueryInstances<P0, P3>(&self, pnamespace: P0, wszclass: *const u16, lflags: i32, pctx: P3, psink: &Option<super::wbemcli::IWbemObjectSink>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::wbemcli::IWbemServices>,
        P3: windows_core::Param<super::wbemcli::IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).QueryInstances)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), wszclass, lflags, pctx.param().abi(), core::mem::transmute_copy(psink)) }
    }
    #[cfg(feature = "wbemcli")]
    pub unsafe fn CreateRefresher<P0>(&self, pnamespace: P0, lflags: i32) -> windows_core::Result<super::wbemcli::IWbemRefresher>
    where
        P0: windows_core::Param<super::wbemcli::IWbemServices>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRefresher)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), lflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wbemcli")]
    pub unsafe fn CreateRefreshableObject<P0, P1, P2, P4>(&self, pnamespace: P0, ptemplate: P1, prefresher: P2, lflags: i32, pcontext: P4, pprefreshable: *mut Option<super::wbemcli::IWbemObjectAccess>, plid: *mut i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::wbemcli::IWbemServices>,
        P1: windows_core::Param<super::wbemcli::IWbemObjectAccess>,
        P2: windows_core::Param<super::wbemcli::IWbemRefresher>,
        P4: windows_core::Param<super::wbemcli::IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateRefreshableObject)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), ptemplate.param().abi(), prefresher.param().abi(), lflags, pcontext.param().abi(), core::mem::transmute(pprefreshable), plid as _) }
    }
    #[cfg(feature = "wbemcli")]
    pub unsafe fn StopRefreshing<P0>(&self, prefresher: P0, lid: i32, lflags: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::wbemcli::IWbemRefresher>,
    {
        unsafe { (windows_core::Interface::vtable(self).StopRefreshing)(windows_core::Interface::as_raw(self), prefresher.param().abi(), lid, lflags) }
    }
    #[cfg(feature = "wbemcli")]
    pub unsafe fn CreateRefreshableEnum<P0, P1, P2, P4, P5>(&self, pnamespace: P0, wszclass: P1, prefresher: P2, lflags: i32, pcontext: P4, phiperfenum: P5) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::wbemcli::IWbemServices>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::wbemcli::IWbemRefresher>,
        P4: windows_core::Param<super::wbemcli::IWbemContext>,
        P5: windows_core::Param<super::wbemcli::IWbemHiPerfEnum>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRefreshableEnum)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), wszclass.param().abi(), prefresher.param().abi(), lflags, pcontext.param().abi(), phiperfenum.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wbemcli")]
    pub unsafe fn GetObjects<P0, P4>(&self, pnamespace: P0, apobj: &mut [Option<super::wbemcli::IWbemObjectAccess>], lflags: i32, pcontext: P4) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::wbemcli::IWbemServices>,
        P4: windows_core::Param<super::wbemcli::IWbemContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetObjects)(windows_core::Interface::as_raw(self), pnamespace.param().abi(), apobj.len().try_into().unwrap(), core::mem::transmute(apobj.as_ptr()), lflags, pcontext.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemHiPerfProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wbemcli")]
    pub QueryInstances: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u16, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    QueryInstances: usize,
    #[cfg(feature = "wbemcli")]
    pub CreateRefresher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    CreateRefresher: usize,
    #[cfg(feature = "wbemcli")]
    pub CreateRefreshableObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    CreateRefreshableObject: usize,
    #[cfg(feature = "wbemcli")]
    pub StopRefreshing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    StopRefreshing: usize,
    #[cfg(feature = "wbemcli")]
    pub CreateRefreshableEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    CreateRefreshableEnum: usize,
    #[cfg(feature = "wbemcli")]
    pub GetObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    GetObjects: usize,
}
#[cfg(feature = "wbemcli")]
pub trait IWbemHiPerfProvider_Impl: windows_core::IUnknownImpl {
    fn QueryInstances(&self, pnamespace: windows_core::Ref<super::wbemcli::IWbemServices>, wszclass: *const u16, lflags: i32, pctx: windows_core::Ref<super::wbemcli::IWbemContext>, psink: windows_core::OutRef<super::wbemcli::IWbemObjectSink>) -> windows_core::Result<()>;
    fn CreateRefresher(&self, pnamespace: windows_core::Ref<super::wbemcli::IWbemServices>, lflags: i32) -> windows_core::Result<super::wbemcli::IWbemRefresher>;
    fn CreateRefreshableObject(&self, pnamespace: windows_core::Ref<super::wbemcli::IWbemServices>, ptemplate: windows_core::Ref<super::wbemcli::IWbemObjectAccess>, prefresher: windows_core::Ref<super::wbemcli::IWbemRefresher>, lflags: i32, pcontext: windows_core::Ref<super::wbemcli::IWbemContext>, pprefreshable: windows_core::OutRef<super::wbemcli::IWbemObjectAccess>, plid: *mut i32) -> windows_core::Result<()>;
    fn StopRefreshing(&self, prefresher: windows_core::Ref<super::wbemcli::IWbemRefresher>, lid: i32, lflags: i32) -> windows_core::Result<()>;
    fn CreateRefreshableEnum(&self, pnamespace: windows_core::Ref<super::wbemcli::IWbemServices>, wszclass: &windows_core::PCWSTR, prefresher: windows_core::Ref<super::wbemcli::IWbemRefresher>, lflags: i32, pcontext: windows_core::Ref<super::wbemcli::IWbemContext>, phiperfenum: windows_core::Ref<super::wbemcli::IWbemHiPerfEnum>) -> windows_core::Result<i32>;
    fn GetObjects(&self, pnamespace: windows_core::Ref<super::wbemcli::IWbemServices>, lnumobjects: i32, apobj: *mut Option<super::wbemcli::IWbemObjectAccess>, lflags: i32, pcontext: windows_core::Ref<super::wbemcli::IWbemContext>) -> windows_core::Result<()>;
}
#[cfg(feature = "wbemcli")]
impl IWbemHiPerfProvider_Vtbl {
    pub const fn new<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryInstances<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszclass: *const u16, lflags: i32, pctx: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfProvider_Impl::QueryInstances(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute_copy(&wszclass), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute(&psink)).into()
            }
        }
        unsafe extern "system" fn CreateRefresher<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, lflags: i32, pprefresher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemHiPerfProvider_Impl::CreateRefresher(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute_copy(&lflags)) {
                    Ok(ok__) => {
                        pprefresher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRefreshableObject<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void, prefresher: *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void, pprefreshable: *mut *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfProvider_Impl::CreateRefreshableObject(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute_copy(&ptemplate), core::mem::transmute_copy(&prefresher), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pprefreshable), core::mem::transmute_copy(&plid)).into()
            }
        }
        unsafe extern "system" fn StopRefreshing<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prefresher: *mut core::ffi::c_void, lid: i32, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfProvider_Impl::StopRefreshing(this, core::mem::transmute_copy(&prefresher), core::mem::transmute_copy(&lid), core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn CreateRefreshableEnum<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, wszclass: windows_core::PCWSTR, prefresher: *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void, phiperfenum: *mut core::ffi::c_void, plid: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemHiPerfProvider_Impl::CreateRefreshableEnum(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute(&wszclass), core::mem::transmute_copy(&prefresher), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&phiperfenum)) {
                    Ok(ok__) => {
                        plid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjects<Identity: IWbemHiPerfProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespace: *mut core::ffi::c_void, lnumobjects: i32, apobj: *mut *mut core::ffi::c_void, lflags: i32, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemHiPerfProvider_Impl::GetObjects(this, core::mem::transmute_copy(&pnamespace), core::mem::transmute_copy(&lnumobjects), core::mem::transmute_copy(&apobj), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pcontext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryInstances: QueryInstances::<Identity, OFFSET>,
            CreateRefresher: CreateRefresher::<Identity, OFFSET>,
            CreateRefreshableObject: CreateRefreshableObject::<Identity, OFFSET>,
            StopRefreshing: StopRefreshing::<Identity, OFFSET>,
            CreateRefreshableEnum: CreateRefreshableEnum::<Identity, OFFSET>,
            GetObjects: GetObjects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemHiPerfProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wbemcli")]
impl windows_core::RuntimeName for IWbemHiPerfProvider {}
windows_core::imp::define_interface!(IWbemPropertyProvider, IWbemPropertyProvider_Vtbl, 0xce61e841_65bc_11d0_b6bd_00aa003240c7);
windows_core::imp::interface_hierarchy!(IWbemPropertyProvider, windows_core::IUnknown);
impl IWbemPropertyProvider {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetProperty(&self, lflags: i32, strlocale: &windows_core::BSTR, strclassmapping: &windows_core::BSTR, strinstmapping: &windows_core::BSTR, strpropmapping: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(strlocale), core::mem::transmute_copy(strclassmapping), core::mem::transmute_copy(strinstmapping), core::mem::transmute_copy(strpropmapping), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PutProperty(&self, lflags: i32, strlocale: &windows_core::BSTR, strclassmapping: &windows_core::BSTR, strinstmapping: &windows_core::BSTR, strpropmapping: &windows_core::BSTR, pvvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PutProperty)(windows_core::Interface::as_raw(self), lflags, core::mem::transmute_copy(strlocale), core::mem::transmute_copy(strclassmapping), core::mem::transmute_copy(strinstmapping), core::mem::transmute_copy(strpropmapping), core::mem::transmute(pvvalue)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPropertyProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetProperty: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub PutProperty: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    PutProperty: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWbemPropertyProvider_Impl: windows_core::IUnknownImpl {
    fn GetProperty(&self, lflags: i32, strlocale: &windows_core::BSTR, strclassmapping: &windows_core::BSTR, strinstmapping: &windows_core::BSTR, strpropmapping: &windows_core::BSTR) -> windows_core::Result<super::oaidl::VARIANT>;
    fn PutProperty(&self, lflags: i32, strlocale: &windows_core::BSTR, strclassmapping: &windows_core::BSTR, strinstmapping: &windows_core::BSTR, strpropmapping: &windows_core::BSTR, pvvalue: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IWbemPropertyProvider_Vtbl {
    pub const fn new<Identity: IWbemPropertyProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperty<Identity: IWbemPropertyProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strlocale: *mut core::ffi::c_void, strclassmapping: *mut core::ffi::c_void, strinstmapping: *mut core::ffi::c_void, strpropmapping: *mut core::ffi::c_void, pvvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPropertyProvider_Impl::GetProperty(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&strlocale), core::mem::transmute(&strclassmapping), core::mem::transmute(&strinstmapping), core::mem::transmute(&strpropmapping)) {
                    Ok(ok__) => {
                        pvvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PutProperty<Identity: IWbemPropertyProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, strlocale: *mut core::ffi::c_void, strclassmapping: *mut core::ffi::c_void, strinstmapping: *mut core::ffi::c_void, strpropmapping: *mut core::ffi::c_void, pvvalue: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPropertyProvider_Impl::PutProperty(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&strlocale), core::mem::transmute(&strclassmapping), core::mem::transmute(&strinstmapping), core::mem::transmute(&strpropmapping), core::mem::transmute_copy(&pvvalue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, OFFSET>,
            PutProperty: PutProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemPropertyProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWbemPropertyProvider {}
windows_core::imp::define_interface!(IWbemProviderIdentity, IWbemProviderIdentity_Vtbl, 0x631f7d97_d993_11d2_b339_00105a1f4aaf);
windows_core::imp::interface_hierarchy!(IWbemProviderIdentity, windows_core::IUnknown);
impl IWbemProviderIdentity {
    #[cfg(feature = "wbemcli")]
    pub unsafe fn SetRegistrationObject<P1>(&self, lflags: i32, pprovreg: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::wbemcli::IWbemClassObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRegistrationObject)(windows_core::Interface::as_raw(self), lflags, pprovreg.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderIdentity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wbemcli")]
    pub SetRegistrationObject: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    SetRegistrationObject: usize,
}
#[cfg(feature = "wbemcli")]
pub trait IWbemProviderIdentity_Impl: windows_core::IUnknownImpl {
    fn SetRegistrationObject(&self, lflags: i32, pprovreg: windows_core::Ref<super::wbemcli::IWbemClassObject>) -> windows_core::Result<()>;
}
#[cfg(feature = "wbemcli")]
impl IWbemProviderIdentity_Vtbl {
    pub const fn new<Identity: IWbemProviderIdentity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRegistrationObject<Identity: IWbemProviderIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pprovreg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemProviderIdentity_Impl::SetRegistrationObject(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pprovreg)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRegistrationObject: SetRegistrationObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemProviderIdentity as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wbemcli")]
impl windows_core::RuntimeName for IWbemProviderIdentity {}
windows_core::imp::define_interface!(IWbemProviderInit, IWbemProviderInit_Vtbl, 0x1be41572_91dd_11d1_aeb2_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemProviderInit, windows_core::IUnknown);
impl IWbemProviderInit {
    #[cfg(feature = "wbemcli")]
    pub unsafe fn Initialize<P0, P2, P3, P4, P5, P6>(&self, wszuser: P0, lflags: i32, wsznamespace: P2, wszlocale: P3, pnamespace: P4, pctx: P5, pinitsink: P6) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<super::wbemcli::IWbemServices>,
        P5: windows_core::Param<super::wbemcli::IWbemContext>,
        P6: windows_core::Param<IWbemProviderInitSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), wszuser.param().abi(), lflags, wsznamespace.param().abi(), wszlocale.param().abi(), pnamespace.param().abi(), pctx.param().abi(), pinitsink.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderInit_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wbemcli")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32, windows_core::PCWSTR, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    Initialize: usize,
}
#[cfg(feature = "wbemcli")]
pub trait IWbemProviderInit_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, wszuser: &windows_core::PCWSTR, lflags: i32, wsznamespace: &windows_core::PCWSTR, wszlocale: &windows_core::PCWSTR, pnamespace: windows_core::Ref<super::wbemcli::IWbemServices>, pctx: windows_core::Ref<super::wbemcli::IWbemContext>, pinitsink: windows_core::Ref<IWbemProviderInitSink>) -> windows_core::Result<()>;
}
#[cfg(feature = "wbemcli")]
impl IWbemProviderInit_Vtbl {
    pub const fn new<Identity: IWbemProviderInit_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWbemProviderInit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszuser: windows_core::PCWSTR, lflags: i32, wsznamespace: windows_core::PCWSTR, wszlocale: windows_core::PCWSTR, pnamespace: *mut core::ffi::c_void, pctx: *mut core::ffi::c_void, pinitsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemProviderInit_Impl::Initialize(this, core::mem::transmute(&wszuser), core::mem::transmute_copy(&lflags), core::mem::transmute(&wsznamespace), core::mem::transmute(&wszlocale), core::mem::transmute_copy(&pnamespace), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&pinitsink)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemProviderInit as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wbemcli")]
impl windows_core::RuntimeName for IWbemProviderInit {}
windows_core::imp::define_interface!(IWbemProviderInitSink, IWbemProviderInitSink_Vtbl, 0x1be41571_91dd_11d1_aeb2_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemProviderInitSink, windows_core::IUnknown);
impl IWbemProviderInitSink {
    pub unsafe fn SetStatus(&self, lstatus: i32, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStatus)(windows_core::Interface::as_raw(self), lstatus, lflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemProviderInitSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
pub trait IWbemProviderInitSink_Impl: windows_core::IUnknownImpl {
    fn SetStatus(&self, lstatus: i32, lflags: i32) -> windows_core::Result<()>;
}
impl IWbemProviderInitSink_Vtbl {
    pub const fn new<Identity: IWbemProviderInitSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetStatus<Identity: IWbemProviderInitSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lstatus: i32, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemProviderInitSink_Impl::SetStatus(this, core::mem::transmute_copy(&lstatus), core::mem::transmute_copy(&lflags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetStatus: SetStatus::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemProviderInitSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemProviderInitSink {}
windows_core::imp::define_interface!(IWbemUnboundObjectSink, IWbemUnboundObjectSink_Vtbl, 0xe246107b_b06e_11d0_ad61_00c04fd8fdff);
windows_core::imp::interface_hierarchy!(IWbemUnboundObjectSink, windows_core::IUnknown);
impl IWbemUnboundObjectSink {
    #[cfg(feature = "wbemcli")]
    pub unsafe fn IndicateToConsumer<P0>(&self, plogicalconsumer: P0, lnumobjects: i32, apobjects: *const Option<super::wbemcli::IWbemClassObject>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::wbemcli::IWbemClassObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).IndicateToConsumer)(windows_core::Interface::as_raw(self), plogicalconsumer.param().abi(), lnumobjects, core::mem::transmute(apobjects)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemUnboundObjectSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wbemcli")]
    pub IndicateToConsumer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *const *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wbemcli"))]
    IndicateToConsumer: usize,
}
#[cfg(feature = "wbemcli")]
pub trait IWbemUnboundObjectSink_Impl: windows_core::IUnknownImpl {
    fn IndicateToConsumer(&self, plogicalconsumer: windows_core::Ref<super::wbemcli::IWbemClassObject>, lnumobjects: i32, apobjects: *const Option<super::wbemcli::IWbemClassObject>) -> windows_core::Result<()>;
}
#[cfg(feature = "wbemcli")]
impl IWbemUnboundObjectSink_Vtbl {
    pub const fn new<Identity: IWbemUnboundObjectSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IndicateToConsumer<Identity: IWbemUnboundObjectSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plogicalconsumer: *mut core::ffi::c_void, lnumobjects: i32, apobjects: *const *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemUnboundObjectSink_Impl::IndicateToConsumer(this, core::mem::transmute_copy(&plogicalconsumer), core::mem::transmute_copy(&lnumobjects), core::mem::transmute_copy(&apobjects)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IndicateToConsumer: IndicateToConsumer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemUnboundObjectSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wbemcli")]
impl windows_core::RuntimeName for IWbemUnboundObjectSink {}
pub type WBEM_BATCH_TYPE = i32;
pub type WBEM_CWSTR = windows_core::PCWSTR;
pub type WBEM_EXTRA_RETURN_CODES = i32;
pub const WBEM_E_RESOURCE_CONTENTION: WBEM_EXTRA_RETURN_CODES = -2147209214;
pub const WBEM_E_RETRY_LATER: WBEM_EXTRA_RETURN_CODES = -2147209215;
pub const WBEM_FLAG_BATCH_IF_NEEDED: WBEM_BATCH_TYPE = 0;
pub const WBEM_FLAG_MUST_BATCH: WBEM_BATCH_TYPE = 1;
pub const WBEM_FLAG_MUST_NOT_BATCH: WBEM_BATCH_TYPE = 2;
pub const WBEM_FLAG_OWNER_UPDATE: WBEM_PROVIDER_FLAGS = 65536;
pub type WBEM_PROVIDER_FLAGS = i32;
pub type WBEM_PROVIDER_REQUIREMENTS_TYPE = i32;
pub const WBEM_REQUIREMENTS_RECHECK_SUBSCRIPTIONS: WBEM_PROVIDER_REQUIREMENTS_TYPE = 2;
pub const WBEM_REQUIREMENTS_START_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = 0;
pub const WBEM_REQUIREMENTS_STOP_POSTFILTER: WBEM_PROVIDER_REQUIREMENTS_TYPE = 1;
pub const WBEM_S_INDIRECTLY_UPDATED: WBEM_EXTRA_RETURN_CODES = 274434;
pub const WBEM_S_INITIALIZED: WBEM_EXTRA_RETURN_CODES = 0;
pub const WBEM_S_LIMITED_SERVICE: WBEM_EXTRA_RETURN_CODES = 274433;
pub const WBEM_S_SUBJECT_TO_SDS: WBEM_EXTRA_RETURN_CODES = 274435;
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub type WBEM_VARIANT = super::oaidl::VARIANT;
pub type WBEM_WSTR = windows_core::PWSTR;
pub const WbemAdministrativeLocator: windows_core::GUID = windows_core::GUID::from_u128(0xcb8555cc_9128_11d1_ad9b_00c04fd8fdff);
pub const WbemAuthenticatedLocator: windows_core::GUID = windows_core::GUID::from_u128(0xcd184336_9128_11d1_ad9b_00c04fd8fdff);
pub const WbemDecoupledBasicEventProvider: windows_core::GUID = windows_core::GUID::from_u128(0xf5f75737_2843_4f22_933d_c76a97cda62f);
pub const WbemDecoupledRegistrar: windows_core::GUID = windows_core::GUID::from_u128(0x4cfc7932_0f9d_4bef_9c32_8ea2a6b56fcb);
pub const WbemUnauthenticatedLocator: windows_core::GUID = windows_core::GUID::from_u128(0x443e7b79_de31_11d2_b340_00104bcc4b4a);
