#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAccountsSettingsPaneInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, accountssettingspane: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ShowManageAccountsForWindowAsync(&self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ShowAddAccountForWindowAsync(&self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IAccountsSettingsPaneInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccountsSettingsPaneInterop_Impl, const OFFSET: isize>() -> IAccountsSettingsPaneInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccountsSettingsPaneInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, accountssettingspane: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&accountssettingspane)).into()
        }
        unsafe extern "system" fn ShowManageAccountsForWindowAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccountsSettingsPaneInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowManageAccountsForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncaction)).into()
        }
        unsafe extern "system" fn ShowAddAccountForWindowAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccountsSettingsPaneInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowAddAccountForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncaction)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IAccountsSettingsPaneInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
            ShowManageAccountsForWindowAsync: ShowManageAccountsForWindowAsync::<Identity, Impl, OFFSET>,
            ShowAddAccountForWindowAsync: ShowAddAccountForWindowAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccountsSettingsPaneInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait IActivationFactory_Impl: Sized {
    fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IActivationFactory {}
impl IActivationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivationFactory_Impl, const OFFSET: isize>() -> IActivationFactory_Vtbl {
        unsafe extern "system" fn ActivateInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(instance, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IActivationFactory, OFFSET>(),
            ActivateInstance: ActivateInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait IAgileReference_Impl: Sized {
    fn Resolve(&self, riid: *const ::windows::core::GUID, ppvobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAgileReference {}
impl IAgileReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAgileReference_Impl, const OFFSET: isize>() -> IAgileReference_Vtbl {
        unsafe extern "system" fn Resolve<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAgileReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resolve(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobjectreference)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Resolve: Resolve::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAgileReference as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait IApartmentShutdown_Impl: Sized {
    fn OnUninitialize(&self, ui64apartmentidentifier: u64);
}
impl ::windows::core::RuntimeName for IApartmentShutdown {}
impl IApartmentShutdown_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IApartmentShutdown_Impl, const OFFSET: isize>() -> IApartmentShutdown_Vtbl {
        unsafe extern "system" fn OnUninitialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IApartmentShutdown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ui64apartmentidentifier: u64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUninitialize(::core::mem::transmute_copy(&ui64apartmentidentifier))
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUninitialize: OnUninitialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApartmentShutdown as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait IAppServiceConnectionExtendedExecution_Impl: Sized {
    fn OpenForExtendedExecutionAsync(&self, riid: *const ::windows::core::GUID, operation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAppServiceConnectionExtendedExecution {}
impl IAppServiceConnectionExtendedExecution_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppServiceConnectionExtendedExecution_Impl, const OFFSET: isize>() -> IAppServiceConnectionExtendedExecution_Vtbl {
        unsafe extern "system" fn OpenForExtendedExecutionAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppServiceConnectionExtendedExecution_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, operation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenForExtendedExecutionAsync(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&operation)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OpenForExtendedExecutionAsync: OpenForExtendedExecutionAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceConnectionExtendedExecution as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait IBufferByteAccess_Impl: Sized {
    fn Buffer(&self) -> ::windows::core::Result<*mut u8>;
}
impl ::windows::core::RuntimeName for IBufferByteAccess {}
impl IBufferByteAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBufferByteAccess_Impl, const OFFSET: isize>() -> IBufferByteAccess_Vtbl {
        unsafe extern "system" fn Buffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBufferByteAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Buffer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Buffer: Buffer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBufferByteAccess as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait ICastingController_Impl: Sized {
    fn Initialize(&self, castingengine: ::core::option::Option<&::windows::core::IUnknown>, castingsource: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Connect(&self) -> ::windows::core::Result<()>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
    fn Advise(&self, eventhandler: ::core::option::Option<&ICastingEventHandler>) -> ::windows::core::Result<u32>;
    fn UnAdvise(&self, cookie: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICastingController {}
impl ICastingController_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICastingController_Impl, const OFFSET: isize>() -> ICastingController_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICastingController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, castingengine: *mut ::core::ffi::c_void, castingsource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows::core::from_raw_borrowed(&castingengine), ::windows::core::from_raw_borrowed(&castingsource)).into()
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICastingController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Connect().into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICastingController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disconnect().into()
        }
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICastingController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Advise(::windows::core::from_raw_borrowed(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnAdvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICastingController_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnAdvise(::core::mem::transmute_copy(&cookie)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            UnAdvise: UnAdvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingController as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait ICastingEventHandler_Impl: Sized {
    fn OnStateChanged(&self, newstate: CASTING_CONNECTION_STATE) -> ::windows::core::Result<()>;
    fn OnError(&self, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICastingEventHandler {}
impl ICastingEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICastingEventHandler_Impl, const OFFSET: isize>() -> ICastingEventHandler_Vtbl {
        unsafe extern "system" fn OnStateChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICastingEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: CASTING_CONNECTION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStateChanged(::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn OnError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICastingEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnError(::core::mem::transmute_copy(&errorstatus), ::core::mem::transmute(&errormessage)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStateChanged: OnStateChanged::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingEventHandler as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ICastingSourceInfo_Impl: Sized {
    fn GetController(&self) -> ::windows::core::Result<ICastingController>;
    fn GetProperties(&self) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::INamedPropertyStore>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows::core::RuntimeName for ICastingSourceInfo {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ICastingSourceInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICastingSourceInfo_Impl, const OFFSET: isize>() -> ICastingSourceInfo_Vtbl {
        unsafe extern "system" fn GetController<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICastingSourceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controller: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetController() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(controller, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICastingSourceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, props: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(props, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetController: GetController::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingSourceInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait ICoreInputInterop_Impl: Sized {
    fn SetInputSource(&self, value: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetMessageHandled(&self, value: u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICoreInputInterop {}
impl ICoreInputInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputInterop_Impl, const OFFSET: isize>() -> ICoreInputInterop_Vtbl {
        unsafe extern "system" fn SetInputSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInputSource(::windows::core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn SetMessageHandled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreInputInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMessageHandled(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInputSource: SetInputSource::<Identity, Impl, OFFSET>,
            SetMessageHandled: SetMessageHandled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreInputInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait ICoreWindowAdapterInterop_Impl: Sized {
    fn AppActivationClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn ApplicationViewClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn CoreApplicationViewClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn HoloViewClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn PositionerClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SystemNavigationClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn TitleBarClientAdapter(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SetWindowClientAdapter(&self, value: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICoreWindowAdapterInterop {}
impl ICoreWindowAdapterInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>() -> ICoreWindowAdapterInterop_Vtbl {
        unsafe extern "system" fn AppActivationClientAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppActivationClientAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationViewClientAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationViewClientAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CoreApplicationViewClientAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CoreApplicationViewClientAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoloViewClientAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HoloViewClientAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PositionerClientAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PositionerClientAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemNavigationClientAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SystemNavigationClientAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TitleBarClientAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TitleBarClientAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWindowClientAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWindowClientAdapter(::windows::core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ICoreWindowAdapterInterop, OFFSET>(),
            AppActivationClientAdapter: AppActivationClientAdapter::<Identity, Impl, OFFSET>,
            ApplicationViewClientAdapter: ApplicationViewClientAdapter::<Identity, Impl, OFFSET>,
            CoreApplicationViewClientAdapter: CoreApplicationViewClientAdapter::<Identity, Impl, OFFSET>,
            HoloViewClientAdapter: HoloViewClientAdapter::<Identity, Impl, OFFSET>,
            PositionerClientAdapter: PositionerClientAdapter::<Identity, Impl, OFFSET>,
            SystemNavigationClientAdapter: SystemNavigationClientAdapter::<Identity, Impl, OFFSET>,
            TitleBarClientAdapter: TitleBarClientAdapter::<Identity, Impl, OFFSET>,
            SetWindowClientAdapter: SetWindowClientAdapter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowAdapterInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICoreWindowComponentInterop_Impl: Sized {
    fn ConfigureComponentInput(&self, hostviewinstanceid: u32, hwndhost: super::super::Foundation::HWND, inputsourcevisual: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetViewInstanceId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICoreWindowComponentInterop {}
#[cfg(feature = "Win32_Foundation")]
impl ICoreWindowComponentInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowComponentInterop_Impl, const OFFSET: isize>() -> ICoreWindowComponentInterop_Vtbl {
        unsafe extern "system" fn ConfigureComponentInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowComponentInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hostviewinstanceid: u32, hwndhost: super::super::Foundation::HWND, inputsourcevisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConfigureComponentInput(::core::mem::transmute_copy(&hostviewinstanceid), ::core::mem::transmute_copy(&hwndhost), ::windows::core::from_raw_borrowed(&inputsourcevisual)).into()
        }
        unsafe extern "system" fn GetViewInstanceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowComponentInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, componentviewinstanceid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetViewInstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(componentviewinstanceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConfigureComponentInput: ConfigureComponentInput::<Identity, Impl, OFFSET>,
            GetViewInstanceId: GetViewInstanceId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowComponentInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICoreWindowInterop_Impl: Sized {
    fn WindowHandle(&self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn SetMessageHandled(&self, value: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICoreWindowInterop {}
#[cfg(feature = "Win32_Foundation")]
impl ICoreWindowInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowInterop_Impl, const OFFSET: isize>() -> ICoreWindowInterop_Vtbl {
        unsafe extern "system" fn WindowHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WindowHandle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hwnd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageHandled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreWindowInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMessageHandled(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WindowHandle: WindowHandle::<Identity, Impl, OFFSET>,
            SetMessageHandled: SetMessageHandled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreWindowInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait ICorrelationVectorInformation_Impl: Sized {
    fn LastCorrelationVectorForThread(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NextCorrelationVectorForThread(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNextCorrelationVectorForThread(&self, cv: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICorrelationVectorInformation {}
impl ICorrelationVectorInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorrelationVectorInformation_Impl, const OFFSET: isize>() -> ICorrelationVectorInformation_Vtbl {
        unsafe extern "system" fn LastCorrelationVectorForThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorrelationVectorInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cv: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastCorrelationVectorForThread() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cv, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextCorrelationVectorForThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorrelationVectorInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cv: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NextCorrelationVectorForThread() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cv, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNextCorrelationVectorForThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorrelationVectorInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cv: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNextCorrelationVectorForThread(::core::mem::transmute(&cv)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ICorrelationVectorInformation, OFFSET>(),
            LastCorrelationVectorForThread: LastCorrelationVectorForThread::<Identity, Impl, OFFSET>,
            NextCorrelationVectorForThread: NextCorrelationVectorForThread::<Identity, Impl, OFFSET>,
            SetNextCorrelationVectorForThread: SetNextCorrelationVectorForThread::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorrelationVectorInformation as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait ICorrelationVectorSource_Impl: Sized {
    fn CorrelationVector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for ICorrelationVectorSource {}
impl ICorrelationVectorSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorrelationVectorSource_Impl, const OFFSET: isize>() -> ICorrelationVectorSource_Vtbl {
        unsafe extern "system" fn CorrelationVector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICorrelationVectorSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cv: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CorrelationVector() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cv, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CorrelationVector: CorrelationVector::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorrelationVectorSource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDragDropManagerInterop_Impl: Sized {
    fn GetForWindow(&self, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDragDropManagerInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IDragDropManagerInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDragDropManagerInterop_Impl, const OFFSET: isize>() -> IDragDropManagerInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDragDropManagerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForWindow(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IDragDropManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragDropManagerInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHolographicSpaceInterop_Impl: Sized {
    fn CreateForWindow(&self, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, holographicspace: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IHolographicSpaceInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IHolographicSpaceInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHolographicSpaceInterop_Impl, const OFFSET: isize>() -> IHolographicSpaceInterop_Vtbl {
        unsafe extern "system" fn CreateForWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHolographicSpaceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, holographicspace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateForWindow(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&holographicspace)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IHolographicSpaceInterop, OFFSET>(),
            CreateForWindow: CreateForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicSpaceInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInputPaneInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, inputpane: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IInputPaneInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IInputPaneInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInputPaneInterop_Impl, const OFFSET: isize>() -> IInputPaneInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInputPaneInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, inputpane: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&inputpane)).into()
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IInputPaneInterop, OFFSET>(), GetForWindow: GetForWindow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputPaneInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait ILanguageExceptionErrorInfo_Impl: Sized {
    fn GetLanguageException(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for ILanguageExceptionErrorInfo {}
impl ILanguageExceptionErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILanguageExceptionErrorInfo_Impl, const OFFSET: isize>() -> ILanguageExceptionErrorInfo_Vtbl {
        unsafe extern "system" fn GetLanguageException<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILanguageExceptionErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageexception: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLanguageException() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(languageexception, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetLanguageException: GetLanguageException::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageExceptionErrorInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait ILanguageExceptionErrorInfo2_Impl: Sized + ILanguageExceptionErrorInfo_Impl {
    fn GetPreviousLanguageExceptionErrorInfo(&self) -> ::windows::core::Result<ILanguageExceptionErrorInfo2>;
    fn CapturePropagationContext(&self, languageexception: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetPropagationContextHead(&self) -> ::windows::core::Result<ILanguageExceptionErrorInfo2>;
}
impl ::windows::core::RuntimeName for ILanguageExceptionErrorInfo2 {}
impl ILanguageExceptionErrorInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILanguageExceptionErrorInfo2_Impl, const OFFSET: isize>() -> ILanguageExceptionErrorInfo2_Vtbl {
        unsafe extern "system" fn GetPreviousLanguageExceptionErrorInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILanguageExceptionErrorInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previouslanguageexceptionerrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreviousLanguageExceptionErrorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previouslanguageexceptionerrorinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturePropagationContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILanguageExceptionErrorInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageexception: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CapturePropagationContext(::windows::core::from_raw_borrowed(&languageexception)).into()
        }
        unsafe extern "system" fn GetPropagationContextHead<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILanguageExceptionErrorInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propagatedlanguageexceptionerrorinfohead: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropagationContextHead() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propagatedlanguageexceptionerrorinfohead, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ILanguageExceptionErrorInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPreviousLanguageExceptionErrorInfo: GetPreviousLanguageExceptionErrorInfo::<Identity, Impl, OFFSET>,
            CapturePropagationContext: CapturePropagationContext::<Identity, Impl, OFFSET>,
            GetPropagationContextHead: GetPropagationContextHead::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageExceptionErrorInfo2 as ::windows::core::ComInterface>::IID || iid == &<ILanguageExceptionErrorInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait ILanguageExceptionStackBackTrace_Impl: Sized {
    fn GetStackBackTrace(&self, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ILanguageExceptionStackBackTrace {}
impl ILanguageExceptionStackBackTrace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILanguageExceptionStackBackTrace_Impl, const OFFSET: isize>() -> ILanguageExceptionStackBackTrace_Vtbl {
        unsafe extern "system" fn GetStackBackTrace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILanguageExceptionStackBackTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStackBackTrace(::core::mem::transmute_copy(&maxframestocapture), ::core::mem::transmute_copy(&stackbacktrace), ::core::mem::transmute_copy(&framescaptured)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetStackBackTrace: GetStackBackTrace::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageExceptionStackBackTrace as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait ILanguageExceptionTransform_Impl: Sized {
    fn GetTransformedRestrictedErrorInfo(&self) -> ::windows::core::Result<IRestrictedErrorInfo>;
}
impl ::windows::core::RuntimeName for ILanguageExceptionTransform {}
impl ILanguageExceptionTransform_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILanguageExceptionTransform_Impl, const OFFSET: isize>() -> ILanguageExceptionTransform_Vtbl {
        unsafe extern "system" fn GetTransformedRestrictedErrorInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILanguageExceptionTransform_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictederrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransformedRestrictedErrorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(restrictederrorinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTransformedRestrictedErrorInfo: GetTransformedRestrictedErrorInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageExceptionTransform as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait IMemoryBufferByteAccess_Impl: Sized {
    fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMemoryBufferByteAccess {}
impl IMemoryBufferByteAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMemoryBufferByteAccess_Impl, const OFFSET: isize>() -> IMemoryBufferByteAccess_Vtbl {
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMemoryBufferByteAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBuffer(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&capacity)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetBuffer: GetBuffer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryBufferByteAccess as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait IMessageDispatcher_Impl: Sized {
    fn PumpMessages(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMessageDispatcher {}
impl IMessageDispatcher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageDispatcher_Impl, const OFFSET: isize>() -> IMessageDispatcher_Vtbl {
        unsafe extern "system" fn PumpMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageDispatcher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PumpMessages().into()
        }
        Self { base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IMessageDispatcher, OFFSET>(), PumpMessages: PumpMessages::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageDispatcher as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPlayToManagerInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, playtomanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ShowPlayToUIForWindow(&self, appwindow: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPlayToManagerInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IPlayToManagerInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPlayToManagerInterop_Impl, const OFFSET: isize>() -> IPlayToManagerInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPlayToManagerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, playtomanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&playtomanager)).into()
        }
        unsafe extern "system" fn ShowPlayToUIForWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPlayToManagerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowPlayToUIForWindow(::core::mem::transmute_copy(&appwindow)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IPlayToManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
            ShowPlayToUIForWindow: ShowPlayToUIForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToManagerInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait IRestrictedErrorInfo_Impl: Sized {
    fn GetErrorDetails(&self, description: *mut ::windows::core::BSTR, error: *mut ::windows::core::HRESULT, restricteddescription: *mut ::windows::core::BSTR, capabilitysid: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetReference(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
impl ::windows::core::RuntimeName for IRestrictedErrorInfo {}
impl IRestrictedErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRestrictedErrorInfo_Impl, const OFFSET: isize>() -> IRestrictedErrorInfo_Vtbl {
        unsafe extern "system" fn GetErrorDetails<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRestrictedErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, error: *mut ::windows::core::HRESULT, restricteddescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, capabilitysid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetErrorDetails(::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&error), ::core::mem::transmute_copy(&restricteddescription), ::core::mem::transmute_copy(&capabilitysid)).into()
        }
        unsafe extern "system" fn GetReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRestrictedErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetErrorDetails: GetErrorDetails::<Identity, Impl, OFFSET>,
            GetReference: GetReference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRestrictedErrorInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IShareWindowCommandEventArgsInterop_Impl: Sized {
    fn GetWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IShareWindowCommandEventArgsInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IShareWindowCommandEventArgsInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IShareWindowCommandEventArgsInterop_Impl, const OFFSET: isize>() -> IShareWindowCommandEventArgsInterop_Vtbl {
        unsafe extern "system" fn GetWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IShareWindowCommandEventArgsInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWindow: GetWindow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareWindowCommandEventArgsInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IShareWindowCommandSourceInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, sharewindowcommandsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IShareWindowCommandSourceInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IShareWindowCommandSourceInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IShareWindowCommandSourceInterop_Impl, const OFFSET: isize>() -> IShareWindowCommandSourceInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IShareWindowCommandSourceInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, sharewindowcommandsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&sharewindowcommandsource)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetForWindow: GetForWindow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareWindowCommandSourceInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialInteractionManagerInterop_Impl: Sized {
    fn GetForWindow(&self, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, spatialinteractionmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISpatialInteractionManagerInterop {}
#[cfg(feature = "Win32_Foundation")]
impl ISpatialInteractionManagerInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialInteractionManagerInterop_Impl, const OFFSET: isize>() -> ISpatialInteractionManagerInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialInteractionManagerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, spatialinteractionmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForWindow(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&spatialinteractionmanager)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ISpatialInteractionManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionManagerInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISystemMediaTransportControlsInterop_Impl: Sized {
    fn GetForWindow(&self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, mediatransportcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsInterop {}
#[cfg(feature = "Win32_Foundation")]
impl ISystemMediaTransportControlsInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISystemMediaTransportControlsInterop_Impl, const OFFSET: isize>() -> ISystemMediaTransportControlsInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISystemMediaTransportControlsInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, mediatransportcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&mediatransportcontrol)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, ISystemMediaTransportControlsInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIViewSettingsInterop_Impl: Sized {
    fn GetForWindow(&self, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUIViewSettingsInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IUIViewSettingsInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUIViewSettingsInterop_Impl, const OFFSET: isize>() -> IUIViewSettingsInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUIViewSettingsInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForWindow(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IUIViewSettingsInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIViewSettingsInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUserActivityInterop_Impl: Sized {
    fn CreateSessionForWindow(&self, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUserActivityInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IUserActivityInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserActivityInterop_Impl, const OFFSET: isize>() -> IUserActivityInterop_Vtbl {
        unsafe extern "system" fn CreateSessionForWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserActivityInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateSessionForWindow(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IUserActivityInterop, OFFSET>(),
            CreateSessionForWindow: CreateSessionForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUserActivityRequestManagerInterop_Impl: Sized {
    fn GetForWindow(&self, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUserActivityRequestManagerInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IUserActivityRequestManagerInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserActivityRequestManagerInterop_Impl, const OFFSET: isize>() -> IUserActivityRequestManagerInterop_Vtbl {
        unsafe extern "system" fn GetForWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserActivityRequestManagerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetForWindow(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IUserActivityRequestManagerInterop, OFFSET>(),
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityRequestManagerInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait IUserActivitySourceHostInterop_Impl: Sized {
    fn SetActivitySourceHost(&self, activitysourcehost: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IUserActivitySourceHostInterop {}
impl IUserActivitySourceHostInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserActivitySourceHostInterop_Impl, const OFFSET: isize>() -> IUserActivitySourceHostInterop_Vtbl {
        unsafe extern "system" fn SetActivitySourceHost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserActivitySourceHostInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activitysourcehost: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActivitySourceHost(::core::mem::transmute(&activitysourcehost)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IUserActivitySourceHostInterop, OFFSET>(),
            SetActivitySourceHost: SetActivitySourceHost::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivitySourceHostInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUserConsentVerifierInterop_Impl: Sized {
    fn RequestVerificationForWindowAsync(&self, appwindow: super::super::Foundation::HWND, message: &::windows::core::HSTRING, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUserConsentVerifierInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IUserConsentVerifierInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserConsentVerifierInterop_Impl, const OFFSET: isize>() -> IUserConsentVerifierInterop_Vtbl {
        unsafe extern "system" fn RequestVerificationForWindowAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUserConsentVerifierInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, message: ::std::mem::MaybeUninit<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestVerificationForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&message), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IUserConsentVerifierInterop, OFFSET>(),
            RequestVerificationForWindowAsync: RequestVerificationForWindowAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserConsentVerifierInterop as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait IWeakReference_Impl: Sized {
    fn Resolve(&self, riid: *const ::windows::core::GUID, objectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWeakReference {}
impl IWeakReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWeakReference_Impl, const OFFSET: isize>() -> IWeakReference_Vtbl {
        unsafe extern "system" fn Resolve<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWeakReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, objectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resolve(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&objectreference)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Resolve: Resolve::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWeakReference as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"implement\"`*"]
pub trait IWeakReferenceSource_Impl: Sized {
    fn GetWeakReference(&self) -> ::windows::core::Result<IWeakReference>;
}
impl ::windows::core::RuntimeName for IWeakReferenceSource {}
impl IWeakReferenceSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWeakReferenceSource_Impl, const OFFSET: isize>() -> IWeakReferenceSource_Vtbl {
        unsafe extern "system" fn GetWeakReference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWeakReferenceSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weakreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWeakReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(weakreference, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetWeakReference: GetWeakReference::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWeakReferenceSource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_WinRT\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWebAuthenticationCoreManagerInterop_Impl: Sized {
    fn RequestTokenForWindowAsync(&self, appwindow: super::super::Foundation::HWND, request: ::core::option::Option<&::windows::core::IInspectable>, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RequestTokenWithWebAccountForWindowAsync(&self, appwindow: super::super::Foundation::HWND, request: ::core::option::Option<&::windows::core::IInspectable>, webaccount: ::core::option::Option<&::windows::core::IInspectable>, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWebAuthenticationCoreManagerInterop {}
#[cfg(feature = "Win32_Foundation")]
impl IWebAuthenticationCoreManagerInterop_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAuthenticationCoreManagerInterop_Impl, const OFFSET: isize>() -> IWebAuthenticationCoreManagerInterop_Vtbl {
        unsafe extern "system" fn RequestTokenForWindowAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAuthenticationCoreManagerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestTokenForWindowAsync(::core::mem::transmute_copy(&appwindow), ::windows::core::from_raw_borrowed(&request), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncinfo)).into()
        }
        unsafe extern "system" fn RequestTokenWithWebAccountForWindowAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWebAuthenticationCoreManagerInterop_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestTokenWithWebAccountForWindowAsync(::core::mem::transmute_copy(&appwindow), ::windows::core::from_raw_borrowed(&request), ::windows::core::from_raw_borrowed(&webaccount), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncinfo)).into()
        }
        Self {
            base__: ::windows::core::IInspectable_Vtbl::new::<Identity, IWebAuthenticationCoreManagerInterop, OFFSET>(),
            RequestTokenForWindowAsync: RequestTokenForWindowAsync::<Identity, Impl, OFFSET>,
            RequestTokenWithWebAccountForWindowAsync: RequestTokenWithWebAccountForWindowAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAuthenticationCoreManagerInterop as ::windows::core::ComInterface>::IID
    }
}
