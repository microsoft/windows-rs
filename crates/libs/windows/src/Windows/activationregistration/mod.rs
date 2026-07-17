pub type ActivationType = i32;
pub const ActivationType_InProcess: ActivationType = 0;
pub const ActivationType_OutOfProcess: ActivationType = 1;
pub const ActivationType_RemoteProcess: ActivationType = 2;
windows_core::imp::define_interface!(IActivatableClassRegistration, IActivatableClassRegistration_Vtbl, 0x9bbcae23_3dd6_49c3_b63c_1c587e7a6a67);
windows_core::imp::interface_hierarchy!(IActivatableClassRegistration, windows_core::IUnknown, windows_core::IInspectable);
impl IActivatableClassRegistration {
    pub unsafe fn get_ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ActivatableClassId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ActivationType(&self) -> windows_core::Result<ActivationType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ActivationType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_RegistrationScope(&self) -> windows_core::Result<RegistrationScope> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RegistrationScope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "inspectable")]
    pub unsafe fn get_RegisteredTrustLevel(&self) -> windows_core::Result<RegisteredTrustLevel> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RegisteredTrustLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Attributes(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Attributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatableClassRegistration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub get_ActivatableClassId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ActivationType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ActivationType) -> windows_core::HRESULT,
    pub get_RegistrationScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RegistrationScope) -> windows_core::HRESULT,
    #[cfg(feature = "inspectable")]
    pub get_RegisteredTrustLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RegisteredTrustLevel) -> windows_core::HRESULT,
    #[cfg(not(feature = "inspectable"))]
    get_RegisteredTrustLevel: usize,
    pub get_Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "inspectable")]
pub trait IActivatableClassRegistration_Impl: windows_core::IUnknownImpl {
    fn get_ActivatableClassId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn get_ActivationType(&self) -> windows_core::Result<ActivationType>;
    fn get_RegistrationScope(&self) -> windows_core::Result<RegistrationScope>;
    fn get_RegisteredTrustLevel(&self) -> windows_core::Result<RegisteredTrustLevel>;
    fn get_Attributes(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::HSTRING, windows_core::IInspectable>>;
}
#[cfg(feature = "inspectable")]
impl IActivatableClassRegistration_Vtbl {
    pub const fn new<Identity: IActivatableClassRegistration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ActivatableClassId<Identity: IActivatableClassRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activatableclassid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivatableClassRegistration_Impl::get_ActivatableClassId(this) {
                    Ok(ok__) => {
                        activatableclassid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ActivationType<Identity: IActivatableClassRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activationtype: *mut ActivationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivatableClassRegistration_Impl::get_ActivationType(this) {
                    Ok(ok__) => {
                        activationtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RegistrationScope<Identity: IActivatableClassRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, registrationscope: *mut RegistrationScope) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivatableClassRegistration_Impl::get_RegistrationScope(this) {
                    Ok(ok__) => {
                        registrationscope.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RegisteredTrustLevel<Identity: IActivatableClassRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, registeredtrustlevel: *mut RegisteredTrustLevel) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivatableClassRegistration_Impl::get_RegisteredTrustLevel(this) {
                    Ok(ok__) => {
                        registeredtrustlevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Attributes<Identity: IActivatableClassRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IActivatableClassRegistration_Impl::get_Attributes(this) {
                    Ok(ok__) => {
                        attributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IActivatableClassRegistration, OFFSET>(),
            get_ActivatableClassId: get_ActivatableClassId::<Identity, OFFSET>,
            get_ActivationType: get_ActivationType::<Identity, OFFSET>,
            get_RegistrationScope: get_RegistrationScope::<Identity, OFFSET>,
            get_RegisteredTrustLevel: get_RegisteredTrustLevel::<Identity, OFFSET>,
            get_Attributes: get_Attributes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IActivatableClassRegistration as windows_core::Interface>::IID
    }
}
#[cfg(feature = "inspectable")]
impl windows_core::RuntimeName for IActivatableClassRegistration {}
#[cfg(feature = "inspectable")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RegisteredTrustLevel(pub super::inspectable::TrustLevel);
pub type RegistrationScope = i32;
pub const RegistrationScope_InboxApp: RegistrationScope = 2;
pub const RegistrationScope_PerMachine: RegistrationScope = 0;
pub const RegistrationScope_PerUser: RegistrationScope = 1;
