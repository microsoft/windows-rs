#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "objidlbase"))]
#[inline]
pub unsafe fn MFCreateEncryptedMediaExtensionsStoreActivate<P0, P1, P2>(pmphost: P0, objectstream: P1, classid: P2) -> windows_core::Result<super::IMFActivate>
where
    P0: windows_core::Param<super::IMFPMPHostApp>,
    P1: windows_core::Param<super::IStream>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateEncryptedMediaExtensionsStoreActivate(pmphost : *mut core::ffi::c_void, objectstream : *mut core::ffi::c_void, classid : windows_core::PCWSTR, activate : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateEncryptedMediaExtensionsStoreActivate(pmphost.param().abi(), objectstream.param().abi(), classid.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
windows_core::imp::define_interface!(IMFContentDecryptionModule, IMFContentDecryptionModule_Vtbl, 0x87be986c_10be_4943_bf48_4b54ce1983a2);
windows_core::imp::interface_hierarchy!(IMFContentDecryptionModule, windows_core::IUnknown);
impl IMFContentDecryptionModule {
    #[cfg(all(feature = "mfidl", feature = "mfobjects"))]
    pub unsafe fn SetContentEnabler<P0, P1>(&self, contentenabler: P0, result: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFContentEnabler>,
        P1: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContentEnabler)(windows_core::Interface::as_raw(self), contentenabler.param().abi(), result.param().abi()) }
    }
    #[cfg(feature = "mfmediaengine")]
    pub unsafe fn GetSuspendNotify(&self) -> windows_core::Result<super::IMFCdmSuspendNotify> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSuspendNotify)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfidl")]
    pub unsafe fn SetPMPHostApp<P0>(&self, pmphostapp: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFPMPHostApp>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPMPHostApp)(windows_core::Interface::as_raw(self), pmphostapp.param().abi()) }
    }
    #[cfg(feature = "mfidl")]
    pub unsafe fn CreateSession<P1>(&self, sessiontype: super::MF_MEDIAKEYSESSION_TYPE, callbacks: P1) -> windows_core::Result<IMFContentDecryptionModuleSession>
    where
        P1: windows_core::Param<IMFContentDecryptionModuleSessionCallbacks>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSession)(windows_core::Interface::as_raw(self), sessiontype, callbacks.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetServerCertificate(&self, certificate: &[super::BYTE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetServerCertificate)(windows_core::Interface::as_raw(self), certificate.as_ptr(), certificate.len().try_into().unwrap()) }
    }
    #[cfg(all(feature = "mfidl", feature = "shared"))]
    pub unsafe fn CreateTrustedInput(&self, contentinitdata: &[super::BYTE]) -> windows_core::Result<super::IMFTrustedInput> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTrustedInput)(windows_core::Interface::as_raw(self), contentinitdata.as_ptr(), contentinitdata.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetProtectionSystemIds(&self, systemids: *mut *mut windows_core::GUID, count: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProtectionSystemIds)(windows_core::Interface::as_raw(self), systemids as _, count as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFContentDecryptionModule_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mfidl", feature = "mfobjects"))]
    pub SetContentEnabler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "mfobjects")))]
    SetContentEnabler: usize,
    #[cfg(feature = "mfmediaengine")]
    pub GetSuspendNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfmediaengine"))]
    GetSuspendNotify: usize,
    #[cfg(feature = "mfidl")]
    pub SetPMPHostApp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    SetPMPHostApp: usize,
    #[cfg(feature = "mfidl")]
    pub CreateSession: unsafe extern "system" fn(*mut core::ffi::c_void, super::MF_MEDIAKEYSESSION_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    CreateSession: usize,
    #[cfg(feature = "shared")]
    pub SetServerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetServerCertificate: usize,
    #[cfg(all(feature = "mfidl", feature = "shared"))]
    pub CreateTrustedInput: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "shared")))]
    CreateTrustedInput: usize,
    #[cfg(feature = "shared")]
    pub GetProtectionSystemIds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut windows_core::GUID, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetProtectionSystemIds: usize,
}
#[cfg(all(feature = "mfidl", feature = "mfmediaengine", feature = "mfobjects", feature = "shared"))]
pub trait IMFContentDecryptionModule_Impl: windows_core::IUnknownImpl {
    fn SetContentEnabler(&self, contentenabler: windows_core::Ref<super::IMFContentEnabler>, result: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<()>;
    fn GetSuspendNotify(&self) -> windows_core::Result<super::IMFCdmSuspendNotify>;
    fn SetPMPHostApp(&self, pmphostapp: windows_core::Ref<super::IMFPMPHostApp>) -> windows_core::Result<()>;
    fn CreateSession(&self, sessiontype: super::MF_MEDIAKEYSESSION_TYPE, callbacks: windows_core::Ref<IMFContentDecryptionModuleSessionCallbacks>) -> windows_core::Result<IMFContentDecryptionModuleSession>;
    fn SetServerCertificate(&self, certificate: *const super::BYTE, certificatesize: super::DWORD) -> windows_core::Result<()>;
    fn CreateTrustedInput(&self, contentinitdata: *const super::BYTE, contentinitdatasize: super::DWORD) -> windows_core::Result<super::IMFTrustedInput>;
    fn GetProtectionSystemIds(&self, systemids: *mut *mut windows_core::GUID, count: *mut super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfidl", feature = "mfmediaengine", feature = "mfobjects", feature = "shared"))]
impl IMFContentDecryptionModule_Vtbl {
    pub const fn new<Identity: IMFContentDecryptionModule_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetContentEnabler<Identity: IMFContentDecryptionModule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentenabler: *mut core::ffi::c_void, result: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentDecryptionModule_Impl::SetContentEnabler(this, core::mem::transmute_copy(&contentenabler), core::mem::transmute_copy(&result)).into()
            }
        }
        unsafe extern "system" fn GetSuspendNotify<Identity: IMFContentDecryptionModule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notify: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFContentDecryptionModule_Impl::GetSuspendNotify(this) {
                    Ok(ok__) => {
                        notify.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPMPHostApp<Identity: IMFContentDecryptionModule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmphostapp: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentDecryptionModule_Impl::SetPMPHostApp(this, core::mem::transmute_copy(&pmphostapp)).into()
            }
        }
        unsafe extern "system" fn CreateSession<Identity: IMFContentDecryptionModule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessiontype: super::MF_MEDIAKEYSESSION_TYPE, callbacks: *mut core::ffi::c_void, session: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFContentDecryptionModule_Impl::CreateSession(this, core::mem::transmute_copy(&sessiontype), core::mem::transmute_copy(&callbacks)) {
                    Ok(ok__) => {
                        session.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServerCertificate<Identity: IMFContentDecryptionModule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certificate: *const super::BYTE, certificatesize: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentDecryptionModule_Impl::SetServerCertificate(this, core::mem::transmute_copy(&certificate), core::mem::transmute_copy(&certificatesize)).into()
            }
        }
        unsafe extern "system" fn CreateTrustedInput<Identity: IMFContentDecryptionModule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentinitdata: *const super::BYTE, contentinitdatasize: super::DWORD, trustedinput: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFContentDecryptionModule_Impl::CreateTrustedInput(this, core::mem::transmute_copy(&contentinitdata), core::mem::transmute_copy(&contentinitdatasize)) {
                    Ok(ok__) => {
                        trustedinput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProtectionSystemIds<Identity: IMFContentDecryptionModule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, systemids: *mut *mut windows_core::GUID, count: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentDecryptionModule_Impl::GetProtectionSystemIds(this, core::mem::transmute_copy(&systemids), core::mem::transmute_copy(&count)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetContentEnabler: SetContentEnabler::<Identity, OFFSET>,
            GetSuspendNotify: GetSuspendNotify::<Identity, OFFSET>,
            SetPMPHostApp: SetPMPHostApp::<Identity, OFFSET>,
            CreateSession: CreateSession::<Identity, OFFSET>,
            SetServerCertificate: SetServerCertificate::<Identity, OFFSET>,
            CreateTrustedInput: CreateTrustedInput::<Identity, OFFSET>,
            GetProtectionSystemIds: GetProtectionSystemIds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFContentDecryptionModule as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfidl", feature = "mfmediaengine", feature = "mfobjects", feature = "shared"))]
impl windows_core::RuntimeName for IMFContentDecryptionModule {}
windows_core::imp::define_interface!(IMFContentDecryptionModuleAccess, IMFContentDecryptionModuleAccess_Vtbl, 0xa853d1f4_e2a0_4303_9edc_f1a68ee43136);
windows_core::imp::interface_hierarchy!(IMFContentDecryptionModuleAccess, windows_core::IUnknown);
impl IMFContentDecryptionModuleAccess {
    #[cfg(feature = "propsys")]
    pub unsafe fn CreateContentDecryptionModule<P0>(&self, contentdecryptionmoduleproperties: P0) -> windows_core::Result<IMFContentDecryptionModule>
    where
        P0: windows_core::Param<super::IPropertyStore>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateContentDecryptionModule)(windows_core::Interface::as_raw(self), contentdecryptionmoduleproperties.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "propsys")]
    pub unsafe fn GetConfiguration(&self) -> windows_core::Result<super::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConfiguration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetKeySystem(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeySystem)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFContentDecryptionModuleAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "propsys")]
    pub CreateContentDecryptionModule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    CreateContentDecryptionModule: usize,
    #[cfg(feature = "propsys")]
    pub GetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    GetConfiguration: usize,
    pub GetKeySystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "propsys")]
pub trait IMFContentDecryptionModuleAccess_Impl: windows_core::IUnknownImpl {
    fn CreateContentDecryptionModule(&self, contentdecryptionmoduleproperties: windows_core::Ref<super::IPropertyStore>) -> windows_core::Result<IMFContentDecryptionModule>;
    fn GetConfiguration(&self) -> windows_core::Result<super::IPropertyStore>;
    fn GetKeySystem(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(feature = "propsys")]
impl IMFContentDecryptionModuleAccess_Vtbl {
    pub const fn new<Identity: IMFContentDecryptionModuleAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateContentDecryptionModule<Identity: IMFContentDecryptionModuleAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentdecryptionmoduleproperties: *mut core::ffi::c_void, contentdecryptionmodule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFContentDecryptionModuleAccess_Impl::CreateContentDecryptionModule(this, core::mem::transmute_copy(&contentdecryptionmoduleproperties)) {
                    Ok(ok__) => {
                        contentdecryptionmodule.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConfiguration<Identity: IMFContentDecryptionModuleAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configuration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFContentDecryptionModuleAccess_Impl::GetConfiguration(this) {
                    Ok(ok__) => {
                        configuration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetKeySystem<Identity: IMFContentDecryptionModuleAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keysystem: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFContentDecryptionModuleAccess_Impl::GetKeySystem(this) {
                    Ok(ok__) => {
                        keysystem.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateContentDecryptionModule: CreateContentDecryptionModule::<Identity, OFFSET>,
            GetConfiguration: GetConfiguration::<Identity, OFFSET>,
            GetKeySystem: GetKeySystem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFContentDecryptionModuleAccess as windows_core::Interface>::IID
    }
}
#[cfg(feature = "propsys")]
impl windows_core::RuntimeName for IMFContentDecryptionModuleAccess {}
windows_core::imp::define_interface!(IMFContentDecryptionModuleFactory, IMFContentDecryptionModuleFactory_Vtbl, 0x7d5abf16_4cbb_4e08_b977_9ba59049943e);
windows_core::imp::interface_hierarchy!(IMFContentDecryptionModuleFactory, windows_core::IUnknown);
impl IMFContentDecryptionModuleFactory {
    pub unsafe fn IsTypeSupported<P0, P1>(&self, keysystem: P0, contenttype: P1) -> windows_core::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsTypeSupported)(windows_core::Interface::as_raw(self), keysystem.param().abi(), contenttype.param().abi()) }
    }
    #[cfg(all(feature = "propsys", feature = "shared"))]
    pub unsafe fn CreateContentDecryptionModuleAccess<P0>(&self, keysystem: P0, configurations: *const Option<super::IPropertyStore>, numconfigurations: super::DWORD) -> windows_core::Result<IMFContentDecryptionModuleAccess>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateContentDecryptionModuleAccess)(windows_core::Interface::as_raw(self), keysystem.param().abi(), core::mem::transmute(configurations), numconfigurations, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFContentDecryptionModuleFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsTypeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::BOOL,
    #[cfg(all(feature = "propsys", feature = "shared"))]
    pub CreateContentDecryptionModuleAccess: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const *mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "propsys", feature = "shared")))]
    CreateContentDecryptionModuleAccess: usize,
}
#[cfg(all(feature = "propsys", feature = "shared"))]
pub trait IMFContentDecryptionModuleFactory_Impl: windows_core::IUnknownImpl {
    fn IsTypeSupported(&self, keysystem: &windows_core::PCWSTR, contenttype: &windows_core::PCWSTR) -> windows_core::BOOL;
    fn CreateContentDecryptionModuleAccess(&self, keysystem: &windows_core::PCWSTR, configurations: *const Option<super::IPropertyStore>, numconfigurations: super::DWORD) -> windows_core::Result<IMFContentDecryptionModuleAccess>;
}
#[cfg(all(feature = "propsys", feature = "shared"))]
impl IMFContentDecryptionModuleFactory_Vtbl {
    pub const fn new<Identity: IMFContentDecryptionModuleFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsTypeSupported<Identity: IMFContentDecryptionModuleFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keysystem: windows_core::PCWSTR, contenttype: windows_core::PCWSTR) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentDecryptionModuleFactory_Impl::IsTypeSupported(this, core::mem::transmute(&keysystem), core::mem::transmute(&contenttype))
            }
        }
        unsafe extern "system" fn CreateContentDecryptionModuleAccess<Identity: IMFContentDecryptionModuleFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keysystem: windows_core::PCWSTR, configurations: *const *mut core::ffi::c_void, numconfigurations: super::DWORD, contentdecryptionmoduleaccess: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFContentDecryptionModuleFactory_Impl::CreateContentDecryptionModuleAccess(this, core::mem::transmute(&keysystem), core::mem::transmute_copy(&configurations), core::mem::transmute_copy(&numconfigurations)) {
                    Ok(ok__) => {
                        contentdecryptionmoduleaccess.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsTypeSupported: IsTypeSupported::<Identity, OFFSET>,
            CreateContentDecryptionModuleAccess: CreateContentDecryptionModuleAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFContentDecryptionModuleFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "propsys", feature = "shared"))]
impl windows_core::RuntimeName for IMFContentDecryptionModuleFactory {}
windows_core::imp::define_interface!(IMFContentDecryptionModuleSession, IMFContentDecryptionModuleSession_Vtbl, 0x4e233efd_1dd2_49e8_b577_d63eee4c0d33);
windows_core::imp::interface_hierarchy!(IMFContentDecryptionModuleSession, windows_core::IUnknown);
impl IMFContentDecryptionModuleSession {
    pub unsafe fn GetSessionId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSessionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetExpiration(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExpiration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "mfidl", feature = "shared"))]
    pub unsafe fn GetKeyStatuses(&self, keystatuses: *mut *mut super::MFMediaKeyStatus, numkeystatuses: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetKeyStatuses)(windows_core::Interface::as_raw(self), keystatuses as _, numkeystatuses as _) }
    }
    pub unsafe fn Load<P0>(&self, sessionid: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), sessionid.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GenerateRequest<P0>(&self, initdatatype: P0, initdata: &[super::BYTE]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GenerateRequest)(windows_core::Interface::as_raw(self), initdatatype.param().abi(), initdata.as_ptr(), initdata.len().try_into().unwrap()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Update(&self, response: &[super::BYTE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), response.as_ptr(), response.len().try_into().unwrap()) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Remove(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFContentDecryptionModuleSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetExpiration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    #[cfg(all(feature = "mfidl", feature = "shared"))]
    pub GetKeyStatuses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::MFMediaKeyStatus, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "shared")))]
    GetKeyStatuses: usize,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GenerateRequest: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::BYTE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GenerateRequest: usize,
    #[cfg(feature = "shared")]
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Update: usize,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfidl", feature = "shared"))]
pub trait IMFContentDecryptionModuleSession_Impl: windows_core::IUnknownImpl {
    fn GetSessionId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetExpiration(&self) -> windows_core::Result<f64>;
    fn GetKeyStatuses(&self, keystatuses: *mut *mut super::MFMediaKeyStatus, numkeystatuses: *mut super::UINT) -> windows_core::Result<()>;
    fn Load(&self, sessionid: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
    fn GenerateRequest(&self, initdatatype: &windows_core::PCWSTR, initdata: *const super::BYTE, initdatasize: super::DWORD) -> windows_core::Result<()>;
    fn Update(&self, response: *const super::BYTE, responsesize: super::DWORD) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Remove(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfidl", feature = "shared"))]
impl IMFContentDecryptionModuleSession_Vtbl {
    pub const fn new<Identity: IMFContentDecryptionModuleSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSessionId<Identity: IMFContentDecryptionModuleSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFContentDecryptionModuleSession_Impl::GetSessionId(this) {
                    Ok(ok__) => {
                        sessionid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetExpiration<Identity: IMFContentDecryptionModuleSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, expiration: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFContentDecryptionModuleSession_Impl::GetExpiration(this) {
                    Ok(ok__) => {
                        expiration.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetKeyStatuses<Identity: IMFContentDecryptionModuleSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keystatuses: *mut *mut super::MFMediaKeyStatus, numkeystatuses: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentDecryptionModuleSession_Impl::GetKeyStatuses(this, core::mem::transmute_copy(&keystatuses), core::mem::transmute_copy(&numkeystatuses)).into()
            }
        }
        unsafe extern "system" fn Load<Identity: IMFContentDecryptionModuleSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: windows_core::PCWSTR, loaded: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFContentDecryptionModuleSession_Impl::Load(this, core::mem::transmute(&sessionid)) {
                    Ok(ok__) => {
                        loaded.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GenerateRequest<Identity: IMFContentDecryptionModuleSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initdatatype: windows_core::PCWSTR, initdata: *const super::BYTE, initdatasize: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentDecryptionModuleSession_Impl::GenerateRequest(this, core::mem::transmute(&initdatatype), core::mem::transmute_copy(&initdata), core::mem::transmute_copy(&initdatasize)).into()
            }
        }
        unsafe extern "system" fn Update<Identity: IMFContentDecryptionModuleSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, response: *const super::BYTE, responsesize: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentDecryptionModuleSession_Impl::Update(this, core::mem::transmute_copy(&response), core::mem::transmute_copy(&responsesize)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IMFContentDecryptionModuleSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentDecryptionModuleSession_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IMFContentDecryptionModuleSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentDecryptionModuleSession_Impl::Remove(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSessionId: GetSessionId::<Identity, OFFSET>,
            GetExpiration: GetExpiration::<Identity, OFFSET>,
            GetKeyStatuses: GetKeyStatuses::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            GenerateRequest: GenerateRequest::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFContentDecryptionModuleSession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfidl", feature = "shared"))]
impl windows_core::RuntimeName for IMFContentDecryptionModuleSession {}
windows_core::imp::define_interface!(IMFContentDecryptionModuleSessionCallbacks, IMFContentDecryptionModuleSessionCallbacks_Vtbl, 0x3f96ee40_ad81_4096_8470_59a4b770f89a);
windows_core::imp::interface_hierarchy!(IMFContentDecryptionModuleSessionCallbacks, windows_core::IUnknown);
impl IMFContentDecryptionModuleSessionCallbacks {
    #[cfg(all(feature = "mfidl", feature = "shared"))]
    pub unsafe fn KeyMessage<P3>(&self, messagetype: super::MF_MEDIAKEYSESSION_MESSAGETYPE, message: &[super::BYTE], destinationurl: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).KeyMessage)(windows_core::Interface::as_raw(self), messagetype, message.as_ptr(), message.len().try_into().unwrap(), destinationurl.param().abi()) }
    }
    pub unsafe fn KeyStatusChanged(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).KeyStatusChanged)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFContentDecryptionModuleSessionCallbacks_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mfidl", feature = "shared"))]
    pub KeyMessage: unsafe extern "system" fn(*mut core::ffi::c_void, super::MF_MEDIAKEYSESSION_MESSAGETYPE, *const super::BYTE, super::DWORD, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "shared")))]
    KeyMessage: usize,
    pub KeyStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfidl", feature = "shared"))]
pub trait IMFContentDecryptionModuleSessionCallbacks_Impl: windows_core::IUnknownImpl {
    fn KeyMessage(&self, messagetype: super::MF_MEDIAKEYSESSION_MESSAGETYPE, message: *const super::BYTE, messagesize: super::DWORD, destinationurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn KeyStatusChanged(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfidl", feature = "shared"))]
impl IMFContentDecryptionModuleSessionCallbacks_Vtbl {
    pub const fn new<Identity: IMFContentDecryptionModuleSessionCallbacks_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KeyMessage<Identity: IMFContentDecryptionModuleSessionCallbacks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetype: super::MF_MEDIAKEYSESSION_MESSAGETYPE, message: *const super::BYTE, messagesize: super::DWORD, destinationurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentDecryptionModuleSessionCallbacks_Impl::KeyMessage(this, core::mem::transmute_copy(&messagetype), core::mem::transmute_copy(&message), core::mem::transmute_copy(&messagesize), core::mem::transmute(&destinationurl)).into()
            }
        }
        unsafe extern "system" fn KeyStatusChanged<Identity: IMFContentDecryptionModuleSessionCallbacks_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentDecryptionModuleSessionCallbacks_Impl::KeyStatusChanged(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KeyMessage: KeyMessage::<Identity, OFFSET>,
            KeyStatusChanged: KeyStatusChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFContentDecryptionModuleSessionCallbacks as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfidl", feature = "shared"))]
impl windows_core::RuntimeName for IMFContentDecryptionModuleSessionCallbacks {}
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MF_CONTENTDECRYPTIONMODULE_INPRIVATESTOREPATH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x730cb3ac_51dc_49da_a578_b95386b62afe), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MF_CONTENTDECRYPTIONMODULE_PMPSTORECONTEXT: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x6d2a2835_c3a9_4681_97f2_0af56be93446), pid: 3 };
pub const MF_CONTENTDECRYPTIONMODULE_SERVICE: windows_core::GUID = windows_core::GUID::from_u128(0x15320c45_ff80_484a_9dcb_0df894e69a01);
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MF_CONTENTDECRYPTIONMODULE_STOREPATH: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x77d993b9_ba61_4bb7_92c6_18c86a189c06), pid: 2 };
pub const MF_ENCRYPTEDMEDIAEXTENSIONS_ACTIVATABLE_CLASS_ID: windows_core::GUID = windows_core::GUID::from_u128(0x77631a31_e5e7_4785_bf17_20f57b224802);
pub const MF_ENCRYPTEDMEDIAEXTENSIONS_ACTIVATE: windows_core::GUID = windows_core::GUID::from_u128(0x2df7b51e_797b_4d06_be71_d14a52cf8421);
pub const MF_ENCRYPTEDMEDIAEXTENSIONS_INITIALIZATION_DATA: windows_core::GUID = windows_core::GUID::from_u128(0x3e73735c_e6c0_481d_8260_ee5db1343b5f);
