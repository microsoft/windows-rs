#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ActivateAudioInterfaceAsync<P0, P3>(deviceinterfacepath: P0, riid: *const windows_core::GUID, activationparams: Option<*const super::propidlbase::PROPVARIANT>, completionhandler: P3) -> windows_core::Result<IActivateAudioInterfaceAsyncOperation>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<IActivateAudioInterfaceCompletionHandler>,
{
    windows_core::link!("mmdevapi.dll" "system" fn ActivateAudioInterfaceAsync(deviceinterfacepath : windows_core::PCWSTR, riid : *const windows_core::GUID, activationparams : *const super::propidlbase::PROPVARIANT, completionhandler : *mut core::ffi::c_void, activationoperation : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        ActivateAudioInterfaceAsync(deviceinterfacepath.param().abi(), riid, activationparams.unwrap_or(core::mem::zeroed()) as _, completionhandler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
pub type AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = i32;
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_DEFAULT: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = 0;
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_ENUM_COUNT: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = 3;
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_USER: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = 1;
pub const AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE_VOLATILE: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AudioExtensionParams {
    pub AddPageParam: super::minwindef::LPARAM,
    pub pEndpoint: core::mem::ManuallyDrop<Option<IMMDevice>>,
    pub pPnpInterface: core::mem::ManuallyDrop<Option<IMMDevice>>,
    pub pPnpDevnode: core::mem::ManuallyDrop<Option<IMMDevice>>,
}
pub const DEVICE_STATEMASK_ALL: u32 = 15;
pub const DEVICE_STATE_ACTIVE: u32 = 1;
pub const DEVICE_STATE_DISABLED: u32 = 2;
pub const DEVICE_STATE_NOTPRESENT: u32 = 4;
pub const DEVICE_STATE_UNPLUGGED: u32 = 8;
pub const DEVINTERFACE_AUDIO_CAPTURE: windows_core::GUID = windows_core::GUID::from_u128(0x2eef81be_33fa_4800_9670_1cd474972c3f);
pub const DEVINTERFACE_AUDIO_RENDER: windows_core::GUID = windows_core::GUID::from_u128(0xe6327cad_dcec_4949_ae8a_991e976a79d2);
pub const DEVINTERFACE_MIDI_INPUT: windows_core::GUID = windows_core::GUID::from_u128(0x504be32c_ccf6_4d2c_b73f_6f8b3747e22b);
pub const DEVINTERFACE_MIDI_OUTPUT: windows_core::GUID = windows_core::GUID::from_u128(0x6dc23320_ab33_4ce4_80d4_bbb3ebbf2814);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DIRECTX_AUDIO_ACTIVATION_PARAMS {
    pub cbDirectXAudioActivationParams: u32,
    pub guidAudioSession: windows_core::GUID,
    pub dwAudioStreamFlags: u32,
}
pub const DigitalAudioDisplayDevice: EndpointFormFactor = 9;
pub type EDataFlow = i32;
pub const EDataFlow_enum_count: EDataFlow = 3;
pub const ENDPOINT_SYSFX_DISABLED: u32 = 1;
pub const ENDPOINT_SYSFX_ENABLED: u32 = 0;
pub type ERole = i32;
pub const ERole_enum_count: ERole = 3;
pub const E_NOTFOUND: i32 = -2147023728;
pub const E_UNSUPPORTED_TYPE: i32 = -2147023266;
pub type EndpointFormFactor = i32;
pub const EndpointFormFactor_enum_count: EndpointFormFactor = 11;
pub const HDMI: u32 = 9;
pub const Handset: EndpointFormFactor = 6;
pub const Headphones: EndpointFormFactor = 3;
pub const Headset: EndpointFormFactor = 5;
windows_core::imp::define_interface!(IActivateAudioInterfaceAsyncOperation, IActivateAudioInterfaceAsyncOperation_Vtbl, 0x72a22d78_cde4_431d_b8cc_843a71199b6d);
windows_core::imp::interface_hierarchy!(IActivateAudioInterfaceAsyncOperation, windows_core::IUnknown);
impl IActivateAudioInterfaceAsyncOperation {
    pub unsafe fn GetActivateResult(&self, activateresult: *mut windows_core::HRESULT, activatedinterface: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetActivateResult)(windows_core::Interface::as_raw(self), activateresult as _, core::mem::transmute(activatedinterface)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivateAudioInterfaceAsyncOperation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetActivateResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActivateAudioInterfaceAsyncOperation_Impl: windows_core::IUnknownImpl {
    fn GetActivateResult(&self, activateresult: *mut windows_core::HRESULT, activatedinterface: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IActivateAudioInterfaceAsyncOperation_Vtbl {
    pub const fn new<Identity: IActivateAudioInterfaceAsyncOperation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetActivateResult<Identity: IActivateAudioInterfaceAsyncOperation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activateresult: *mut windows_core::HRESULT, activatedinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActivateAudioInterfaceAsyncOperation_Impl::GetActivateResult(this, core::mem::transmute_copy(&activateresult), core::mem::transmute_copy(&activatedinterface)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetActivateResult: GetActivateResult::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivateAudioInterfaceAsyncOperation as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActivateAudioInterfaceAsyncOperation {}
windows_core::imp::define_interface!(IActivateAudioInterfaceCompletionHandler, IActivateAudioInterfaceCompletionHandler_Vtbl, 0x41d949ab_9862_444a_80f6_c261334da5eb);
windows_core::imp::interface_hierarchy!(IActivateAudioInterfaceCompletionHandler, windows_core::IUnknown);
impl IActivateAudioInterfaceCompletionHandler {
    pub unsafe fn ActivateCompleted<P0>(&self, activateoperation: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IActivateAudioInterfaceAsyncOperation>,
    {
        unsafe { (windows_core::Interface::vtable(self).ActivateCompleted)(windows_core::Interface::as_raw(self), activateoperation.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivateAudioInterfaceCompletionHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ActivateCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IActivateAudioInterfaceCompletionHandler_Impl: windows_core::IUnknownImpl {
    fn ActivateCompleted(&self, activateoperation: windows_core::Ref<IActivateAudioInterfaceAsyncOperation>) -> windows_core::Result<()>;
}
impl IActivateAudioInterfaceCompletionHandler_Vtbl {
    pub const fn new<Identity: IActivateAudioInterfaceCompletionHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateCompleted<Identity: IActivateAudioInterfaceCompletionHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activateoperation: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IActivateAudioInterfaceCompletionHandler_Impl::ActivateCompleted(this, core::mem::transmute_copy(&activateoperation)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ActivateCompleted: ActivateCompleted::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivateAudioInterfaceCompletionHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IActivateAudioInterfaceCompletionHandler {}
windows_core::imp::define_interface!(IAudioSystemEffectsPropertyChangeNotificationClient, IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl, 0x20049d40_56d5_400e_a2ef_385599feed49);
windows_core::imp::interface_hierarchy!(IAudioSystemEffectsPropertyChangeNotificationClient, windows_core::IUnknown);
impl IAudioSystemEffectsPropertyChangeNotificationClient {
    #[cfg(feature = "wtypes")]
    pub unsafe fn OnPropertyChanged(&self, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnPropertyChanged)(windows_core::Interface::as_raw(self), r#type, core::mem::transmute(key)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub OnPropertyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, super::wtypes::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    OnPropertyChanged: usize,
}
#[cfg(feature = "wtypes")]
pub trait IAudioSystemEffectsPropertyChangeNotificationClient_Impl: windows_core::IUnknownImpl {
    fn OnPropertyChanged(&self, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: &super::wtypes::PROPERTYKEY) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypes")]
impl IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl {
    pub const fn new<Identity: IAudioSystemEffectsPropertyChangeNotificationClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnPropertyChanged<Identity: IAudioSystemEffectsPropertyChangeNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSystemEffectsPropertyChangeNotificationClient_Impl::OnPropertyChanged(this, core::mem::transmute_copy(&r#type), core::mem::transmute(&key)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnPropertyChanged: OnPropertyChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSystemEffectsPropertyChangeNotificationClient as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IAudioSystemEffectsPropertyChangeNotificationClient {}
windows_core::imp::define_interface!(IAudioSystemEffectsPropertyStore, IAudioSystemEffectsPropertyStore_Vtbl, 0x302ae7f9_d7e0_43e4_971b_1f8293613d2a);
windows_core::imp::interface_hierarchy!(IAudioSystemEffectsPropertyStore, windows_core::IUnknown);
impl IAudioSystemEffectsPropertyStore {
    #[cfg(feature = "propsys")]
    pub unsafe fn OpenDefaultPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenDefaultPropertyStore)(windows_core::Interface::as_raw(self), stgmaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "propsys")]
    pub unsafe fn OpenUserPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenUserPropertyStore)(windows_core::Interface::as_raw(self), stgmaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "propsys")]
    pub unsafe fn OpenVolatilePropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenVolatilePropertyStore)(windows_core::Interface::as_raw(self), stgmaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ResetUserPropertyStore(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetUserPropertyStore)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ResetVolatilePropertyStore(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetVolatilePropertyStore)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RegisterPropertyChangeNotification<P0>(&self, callback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAudioSystemEffectsPropertyChangeNotificationClient>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterPropertyChangeNotification)(windows_core::Interface::as_raw(self), callback.param().abi()) }
    }
    pub unsafe fn UnregisterPropertyChangeNotification<P0>(&self, callback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAudioSystemEffectsPropertyChangeNotificationClient>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterPropertyChangeNotification)(windows_core::Interface::as_raw(self), callback.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSystemEffectsPropertyStore_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "propsys")]
    pub OpenDefaultPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    OpenDefaultPropertyStore: usize,
    #[cfg(feature = "propsys")]
    pub OpenUserPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    OpenUserPropertyStore: usize,
    #[cfg(feature = "propsys")]
    pub OpenVolatilePropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    OpenVolatilePropertyStore: usize,
    pub ResetUserPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResetVolatilePropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterPropertyChangeNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterPropertyChangeNotification: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "propsys")]
pub trait IAudioSystemEffectsPropertyStore_Impl: windows_core::IUnknownImpl {
    fn OpenDefaultPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn OpenUserPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn OpenVolatilePropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn ResetUserPropertyStore(&self) -> windows_core::Result<()>;
    fn ResetVolatilePropertyStore(&self) -> windows_core::Result<()>;
    fn RegisterPropertyChangeNotification(&self, callback: windows_core::Ref<IAudioSystemEffectsPropertyChangeNotificationClient>) -> windows_core::Result<()>;
    fn UnregisterPropertyChangeNotification(&self, callback: windows_core::Ref<IAudioSystemEffectsPropertyChangeNotificationClient>) -> windows_core::Result<()>;
}
#[cfg(feature = "propsys")]
impl IAudioSystemEffectsPropertyStore_Vtbl {
    pub const fn new<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenDefaultPropertyStore<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSystemEffectsPropertyStore_Impl::OpenDefaultPropertyStore(this, core::mem::transmute_copy(&stgmaccess)) {
                    Ok(ok__) => {
                        propstore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenUserPropertyStore<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSystemEffectsPropertyStore_Impl::OpenUserPropertyStore(this, core::mem::transmute_copy(&stgmaccess)) {
                    Ok(ok__) => {
                        propstore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenVolatilePropertyStore<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioSystemEffectsPropertyStore_Impl::OpenVolatilePropertyStore(this, core::mem::transmute_copy(&stgmaccess)) {
                    Ok(ok__) => {
                        propstore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResetUserPropertyStore<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSystemEffectsPropertyStore_Impl::ResetUserPropertyStore(this).into()
            }
        }
        unsafe extern "system" fn ResetVolatilePropertyStore<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSystemEffectsPropertyStore_Impl::ResetVolatilePropertyStore(this).into()
            }
        }
        unsafe extern "system" fn RegisterPropertyChangeNotification<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSystemEffectsPropertyStore_Impl::RegisterPropertyChangeNotification(this, core::mem::transmute_copy(&callback)).into()
            }
        }
        unsafe extern "system" fn UnregisterPropertyChangeNotification<Identity: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSystemEffectsPropertyStore_Impl::UnregisterPropertyChangeNotification(this, core::mem::transmute_copy(&callback)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenDefaultPropertyStore: OpenDefaultPropertyStore::<Identity, OFFSET>,
            OpenUserPropertyStore: OpenUserPropertyStore::<Identity, OFFSET>,
            OpenVolatilePropertyStore: OpenVolatilePropertyStore::<Identity, OFFSET>,
            ResetUserPropertyStore: ResetUserPropertyStore::<Identity, OFFSET>,
            ResetVolatilePropertyStore: ResetVolatilePropertyStore::<Identity, OFFSET>,
            RegisterPropertyChangeNotification: RegisterPropertyChangeNotification::<Identity, OFFSET>,
            UnregisterPropertyChangeNotification: UnregisterPropertyChangeNotification::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSystemEffectsPropertyStore as windows_core::Interface>::IID
    }
}
#[cfg(feature = "propsys")]
impl windows_core::RuntimeName for IAudioSystemEffectsPropertyStore {}
windows_core::imp::define_interface!(IMMDevice, IMMDevice_Vtbl, 0xd666063f_1587_4e43_81f1_b948e807363f);
windows_core::imp::interface_hierarchy!(IMMDevice, windows_core::IUnknown);
impl IMMDevice {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Activate<T>(&self, dwclsctx: u32, pactivationparams: Option<*const super::propidlbase::PROPVARIANT>) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), &T::IID, dwclsctx, pactivationparams.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "propsys")]
    pub unsafe fn OpenPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenPropertyStore)(windows_core::Interface::as_raw(self), stgmaccess, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetState(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const super::propidlbase::PROPVARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    Activate: usize,
    #[cfg(feature = "propsys")]
    pub OpenPropertyStore: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    OpenPropertyStore: usize,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "propsys", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMMDevice_Impl: windows_core::IUnknownImpl {
    fn Activate(&self, iid: *const windows_core::GUID, dwclsctx: u32, pactivationparams: *const super::propidlbase::PROPVARIANT, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OpenPropertyStore(&self, stgmaccess: u32) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn GetId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetState(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "propsys", feature = "wtypes", feature = "wtypesbase"))]
impl IMMDevice_Vtbl {
    pub const fn new<Identity: IMMDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: IMMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, dwclsctx: u32, pactivationparams: *const super::propidlbase::PROPVARIANT, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMDevice_Impl::Activate(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&dwclsctx), core::mem::transmute_copy(&pactivationparams), core::mem::transmute_copy(&ppinterface)).into()
            }
        }
        unsafe extern "system" fn OpenPropertyStore<Identity: IMMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stgmaccess: u32, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDevice_Impl::OpenPropertyStore(this, core::mem::transmute_copy(&stgmaccess)) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetId<Identity: IMMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstrid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDevice_Impl::GetId(this) {
                    Ok(ok__) => {
                        ppstrid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetState<Identity: IMMDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDevice_Impl::GetState(this) {
                    Ok(ok__) => {
                        pdwstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Identity, OFFSET>,
            GetId: GetId::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMDevice as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "propsys", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMMDevice {}
windows_core::imp::define_interface!(IMMDeviceActivator, IMMDeviceActivator_Vtbl, 0x3b0d0ea4_d0a9_4b0e_935b_09516746fac0);
windows_core::imp::interface_hierarchy!(IMMDeviceActivator, windows_core::IUnknown);
impl IMMDeviceActivator {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Activate<P1, T>(&self, pdevice: P1, pactivationparams: Option<*const super::propidlbase::PROPVARIANT>) -> windows_core::Result<T>
    where
        P1: windows_core::Param<IMMDevice>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), &T::IID, pdevice.param().abi(), pactivationparams.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceActivator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    Activate: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMMDeviceActivator_Impl: windows_core::IUnknownImpl {
    fn Activate(&self, iid: *const windows_core::GUID, pdevice: windows_core::Ref<IMMDevice>, pactivationparams: *const super::propidlbase::PROPVARIANT, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMMDeviceActivator_Vtbl {
    pub const fn new<Identity: IMMDeviceActivator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: IMMDeviceActivator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, pdevice: *mut core::ffi::c_void, pactivationparams: *const super::propidlbase::PROPVARIANT, ppinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMDeviceActivator_Impl::Activate(this, core::mem::transmute_copy(&iid), core::mem::transmute_copy(&pdevice), core::mem::transmute_copy(&pactivationparams), core::mem::transmute_copy(&ppinterface)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Activate: Activate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMDeviceActivator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMMDeviceActivator {}
windows_core::imp::define_interface!(IMMDeviceCollection, IMMDeviceCollection_Vtbl, 0x0bd7a1be_7a1a_44db_8397_cc5392387b5e);
windows_core::imp::interface_hierarchy!(IMMDeviceCollection, windows_core::IUnknown);
impl IMMDeviceCollection {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Item(&self, ndevice: u32) -> windows_core::Result<IMMDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), ndevice, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMMDeviceCollection_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Item(&self, ndevice: u32) -> windows_core::Result<IMMDevice>;
}
impl IMMDeviceCollection_Vtbl {
    pub const fn new<Identity: IMMDeviceCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IMMDeviceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdevices: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDeviceCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcdevices.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IMMDeviceCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ndevice: u32, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDeviceCollection_Impl::Item(this, core::mem::transmute_copy(&ndevice)) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, Item: Item::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMDeviceCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMMDeviceCollection {}
windows_core::imp::define_interface!(IMMDeviceEnumerator, IMMDeviceEnumerator_Vtbl, 0xa95664d2_9614_4f35_a746_de8db63617e6);
windows_core::imp::interface_hierarchy!(IMMDeviceEnumerator, windows_core::IUnknown);
impl IMMDeviceEnumerator {
    pub unsafe fn EnumAudioEndpoints(&self, dataflow: EDataFlow, dwstatemask: u32) -> windows_core::Result<IMMDeviceCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAudioEndpoints)(windows_core::Interface::as_raw(self), dataflow, dwstatemask, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDefaultAudioEndpoint(&self, dataflow: EDataFlow, role: ERole) -> windows_core::Result<IMMDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultAudioEndpoint)(windows_core::Interface::as_raw(self), dataflow, role, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDevice<P0>(&self, pwstrid: P0) -> windows_core::Result<IMMDevice>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), pwstrid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterEndpointNotificationCallback<P0>(&self, pclient: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMMNotificationClient>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterEndpointNotificationCallback)(windows_core::Interface::as_raw(self), pclient.param().abi()) }
    }
    pub unsafe fn UnregisterEndpointNotificationCallback<P0>(&self, pclient: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMMNotificationClient>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterEndpointNotificationCallback)(windows_core::Interface::as_raw(self), pclient.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMDeviceEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EnumAudioEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, EDataFlow, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDefaultAudioEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, EDataFlow, ERole, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterEndpointNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterEndpointNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMMDeviceEnumerator_Impl: windows_core::IUnknownImpl {
    fn EnumAudioEndpoints(&self, dataflow: EDataFlow, dwstatemask: u32) -> windows_core::Result<IMMDeviceCollection>;
    fn GetDefaultAudioEndpoint(&self, dataflow: EDataFlow, role: ERole) -> windows_core::Result<IMMDevice>;
    fn GetDevice(&self, pwstrid: &windows_core::PCWSTR) -> windows_core::Result<IMMDevice>;
    fn RegisterEndpointNotificationCallback(&self, pclient: windows_core::Ref<IMMNotificationClient>) -> windows_core::Result<()>;
    fn UnregisterEndpointNotificationCallback(&self, pclient: windows_core::Ref<IMMNotificationClient>) -> windows_core::Result<()>;
}
impl IMMDeviceEnumerator_Vtbl {
    pub const fn new<Identity: IMMDeviceEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumAudioEndpoints<Identity: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dataflow: EDataFlow, dwstatemask: u32, ppdevices: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDeviceEnumerator_Impl::EnumAudioEndpoints(this, core::mem::transmute_copy(&dataflow), core::mem::transmute_copy(&dwstatemask)) {
                    Ok(ok__) => {
                        ppdevices.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDefaultAudioEndpoint<Identity: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dataflow: EDataFlow, role: ERole, ppendpoint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDeviceEnumerator_Impl::GetDefaultAudioEndpoint(this, core::mem::transmute_copy(&dataflow), core::mem::transmute_copy(&role)) {
                    Ok(ok__) => {
                        ppendpoint.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDevice<Identity: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrid: windows_core::PCWSTR, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMDeviceEnumerator_Impl::GetDevice(this, core::mem::transmute(&pwstrid)) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterEndpointNotificationCallback<Identity: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclient: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMDeviceEnumerator_Impl::RegisterEndpointNotificationCallback(this, core::mem::transmute_copy(&pclient)).into()
            }
        }
        unsafe extern "system" fn UnregisterEndpointNotificationCallback<Identity: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclient: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMDeviceEnumerator_Impl::UnregisterEndpointNotificationCallback(this, core::mem::transmute_copy(&pclient)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EnumAudioEndpoints: EnumAudioEndpoints::<Identity, OFFSET>,
            GetDefaultAudioEndpoint: GetDefaultAudioEndpoint::<Identity, OFFSET>,
            GetDevice: GetDevice::<Identity, OFFSET>,
            RegisterEndpointNotificationCallback: RegisterEndpointNotificationCallback::<Identity, OFFSET>,
            UnregisterEndpointNotificationCallback: UnregisterEndpointNotificationCallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMDeviceEnumerator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMMDeviceEnumerator {}
windows_core::imp::define_interface!(IMMEndpoint, IMMEndpoint_Vtbl, 0x1be09788_6894_4089_8586_9a2a6c265ac5);
windows_core::imp::interface_hierarchy!(IMMEndpoint, windows_core::IUnknown);
impl IMMEndpoint {
    pub unsafe fn GetDataFlow(&self) -> windows_core::Result<EDataFlow> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataFlow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMEndpoint_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDataFlow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut EDataFlow) -> windows_core::HRESULT,
}
pub trait IMMEndpoint_Impl: windows_core::IUnknownImpl {
    fn GetDataFlow(&self) -> windows_core::Result<EDataFlow>;
}
impl IMMEndpoint_Vtbl {
    pub const fn new<Identity: IMMEndpoint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDataFlow<Identity: IMMEndpoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataflow: *mut EDataFlow) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMMEndpoint_Impl::GetDataFlow(this) {
                    Ok(ok__) => {
                        pdataflow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDataFlow: GetDataFlow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMEndpoint as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMMEndpoint {}
windows_core::imp::define_interface!(IMMNotificationClient, IMMNotificationClient_Vtbl, 0x7991eec9_7e89_4d85_8390_6c703cec60c0);
windows_core::imp::interface_hierarchy!(IMMNotificationClient, windows_core::IUnknown);
impl IMMNotificationClient {
    pub unsafe fn OnDeviceStateChanged<P0>(&self, pwstrdeviceid: P0, dwnewstate: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDeviceStateChanged)(windows_core::Interface::as_raw(self), pwstrdeviceid.param().abi(), dwnewstate) }
    }
    pub unsafe fn OnDeviceAdded<P0>(&self, pwstrdeviceid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDeviceAdded)(windows_core::Interface::as_raw(self), pwstrdeviceid.param().abi()) }
    }
    pub unsafe fn OnDeviceRemoved<P0>(&self, pwstrdeviceid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDeviceRemoved)(windows_core::Interface::as_raw(self), pwstrdeviceid.param().abi()) }
    }
    pub unsafe fn OnDefaultDeviceChanged<P2>(&self, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDefaultDeviceChanged)(windows_core::Interface::as_raw(self), flow, role, pwstrdefaultdeviceid.param().abi()) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn OnPropertyValueChanged<P0>(&self, pwstrdeviceid: P0, key: super::wtypes::PROPERTYKEY) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnPropertyValueChanged)(windows_core::Interface::as_raw(self), pwstrdeviceid.param().abi(), core::mem::transmute(key)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMNotificationClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDeviceStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub OnDeviceAdded: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnDeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub OnDefaultDeviceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, EDataFlow, ERole, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub OnPropertyValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::wtypes::PROPERTYKEY) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    OnPropertyValueChanged: usize,
}
#[cfg(feature = "wtypes")]
pub trait IMMNotificationClient_Impl: windows_core::IUnknownImpl {
    fn OnDeviceStateChanged(&self, pwstrdeviceid: &windows_core::PCWSTR, dwnewstate: u32) -> windows_core::Result<()>;
    fn OnDeviceAdded(&self, pwstrdeviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnDeviceRemoved(&self, pwstrdeviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnDefaultDeviceChanged(&self, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn OnPropertyValueChanged(&self, pwstrdeviceid: &windows_core::PCWSTR, key: &super::wtypes::PROPERTYKEY) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypes")]
impl IMMNotificationClient_Vtbl {
    pub const fn new<Identity: IMMNotificationClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDeviceStateChanged<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrdeviceid: windows_core::PCWSTR, dwnewstate: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMNotificationClient_Impl::OnDeviceStateChanged(this, core::mem::transmute(&pwstrdeviceid), core::mem::transmute_copy(&dwnewstate)).into()
            }
        }
        unsafe extern "system" fn OnDeviceAdded<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrdeviceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMNotificationClient_Impl::OnDeviceAdded(this, core::mem::transmute(&pwstrdeviceid)).into()
            }
        }
        unsafe extern "system" fn OnDeviceRemoved<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrdeviceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMNotificationClient_Impl::OnDeviceRemoved(this, core::mem::transmute(&pwstrdeviceid)).into()
            }
        }
        unsafe extern "system" fn OnDefaultDeviceChanged<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMNotificationClient_Impl::OnDefaultDeviceChanged(this, core::mem::transmute_copy(&flow), core::mem::transmute_copy(&role), core::mem::transmute(&pwstrdefaultdeviceid)).into()
            }
        }
        unsafe extern "system" fn OnPropertyValueChanged<Identity: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwstrdeviceid: windows_core::PCWSTR, key: super::wtypes::PROPERTYKEY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMMNotificationClient_Impl::OnPropertyValueChanged(this, core::mem::transmute(&pwstrdeviceid), core::mem::transmute(&key)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDeviceStateChanged: OnDeviceStateChanged::<Identity, OFFSET>,
            OnDeviceAdded: OnDeviceAdded::<Identity, OFFSET>,
            OnDeviceRemoved: OnDeviceRemoved::<Identity, OFFSET>,
            OnDefaultDeviceChanged: OnDefaultDeviceChanged::<Identity, OFFSET>,
            OnPropertyValueChanged: OnPropertyValueChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMMNotificationClient as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IMMNotificationClient {}
pub const LineLevel: EndpointFormFactor = 2;
pub const MMDeviceEnumerator: windows_core::GUID = windows_core::GUID::from_u128(0xbcde0395_e52f_467c_8e3d_c4579291692e);
pub const Microphone: EndpointFormFactor = 4;
pub type PDIRECTX_AUDIO_ACTIVATION_PARAMS = *mut DIRECTX_AUDIO_ACTIVATION_PARAMS;
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpointLogo_IconEffects: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 0 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpointLogo_IconPath: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf1ab780d_2010_4ed3_a3a6_8b87f0f0c476), pid: 1 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpointSettings_LaunchContract: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 1 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpointSettings_MenuText: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x14242002_0320_4de4_9555_a7d82b73c286), pid: 0 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_Association: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 2 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_ControlPanelPageProvider: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 1 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_Default_VolumeInDb: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 9 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_Disable_SysFx: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 5 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_FormFactor: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 0 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_FullRangeSpeakers: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 6 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_GUID: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 4 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_JackSubType: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 8 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_Max_VolumeInDb: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 10 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_Min_VolumeInDb: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 11 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_PhysicalSpeakers: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 3 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEndpoint_Supports_EventDriven_Mode: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0x1da5d803_d492_4edd_8c23_e0c0ffee7f0e), pid: 7 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEngine_DeviceFormat: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xf19f064d_082c_4e27_bc73_6882a1bb8e4c), pid: 0 };
#[cfg(feature = "wtypes")]
pub const PKEY_AudioEngine_OEMFormat: super::wtypes::PROPERTYKEY = super::wtypes::PROPERTYKEY { fmtid: windows_core::GUID::from_u128(0xe4870e26_3cc5_4cd2_ba46_ca0a9a70ed04), pid: 3 };
pub const RemoteNetworkDevice: EndpointFormFactor = 0;
pub const SPDIF: EndpointFormFactor = 8;
pub const Speakers: EndpointFormFactor = 1;
pub const UnknownDigitalPassthrough: EndpointFormFactor = 7;
pub const UnknownFormFactor: EndpointFormFactor = 10;
pub const eAll: EDataFlow = 2;
pub const eCapture: EDataFlow = 1;
pub const eCommunications: ERole = 2;
pub const eConsole: ERole = 0;
pub const eMultimedia: ERole = 1;
pub const eRender: EDataFlow = 0;
