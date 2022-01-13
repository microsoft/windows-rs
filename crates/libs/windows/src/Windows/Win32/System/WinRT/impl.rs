#[cfg(feature = "Win32_Foundation")]
pub trait IAccountsSettingsPaneInteropImpl: Sized {
    fn GetForWindow(&mut self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, accountssettingspane: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ShowManageAccountsForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ShowAddAccountForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAccountsSettingsPaneInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IAccountsSettingsPaneInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccountsSettingsPaneInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccountsSettingsPaneInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IAccountsSettingsPaneInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, accountssettingspane: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&accountssettingspane)).into()
        }
        unsafe extern "system" fn ShowManageAccountsForWindowAsync<Impl: IAccountsSettingsPaneInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowManageAccountsForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncaction)).into()
        }
        unsafe extern "system" fn ShowAddAccountForWindowAsync<Impl: IAccountsSettingsPaneInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowAddAccountForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncaction)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAccountsSettingsPaneInterop, BASE_OFFSET>(),
            GetForWindow: GetForWindow::<Impl, IMPL_OFFSET>,
            ShowManageAccountsForWindowAsync: ShowManageAccountsForWindowAsync::<Impl, IMPL_OFFSET>,
            ShowAddAccountForWindowAsync: ShowAddAccountForWindowAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccountsSettingsPaneInterop as ::windows::core::Interface>::IID
    }
}
pub trait IActivationFactoryImpl: Sized {
    fn ActivateInstance(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IActivationFactory {
    const NAME: &'static str = "";
}
impl IActivationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationFactoryVtbl {
        unsafe extern "system" fn ActivateInstance<Impl: IActivationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *instance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IActivationFactory, BASE_OFFSET>(),
            ActivateInstance: ActivateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationFactory as ::windows::core::Interface>::IID
    }
}
pub trait IAgileReferenceImpl: Sized {
    fn Resolve(&mut self, riid: *const ::windows::core::GUID, ppvobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IAgileReferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAgileReferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAgileReferenceVtbl {
        unsafe extern "system" fn Resolve<Impl: IAgileReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resolve(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobjectreference)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Resolve: Resolve::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAgileReference as ::windows::core::Interface>::IID
    }
}
pub trait IApartmentShutdownImpl: Sized {
    fn OnUninitialize(&mut self, ui64apartmentidentifier: u64);
}
impl IApartmentShutdownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApartmentShutdownImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApartmentShutdownVtbl {
        unsafe extern "system" fn OnUninitialize<Impl: IApartmentShutdownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ui64apartmentidentifier: u64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUninitialize(::core::mem::transmute_copy(&ui64apartmentidentifier))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnUninitialize: OnUninitialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApartmentShutdown as ::windows::core::Interface>::IID
    }
}
pub trait IAppServiceConnectionExtendedExecutionImpl: Sized {
    fn OpenForExtendedExecutionAsync(&mut self, riid: *const ::windows::core::GUID, operation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IAppServiceConnectionExtendedExecutionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceConnectionExtendedExecutionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceConnectionExtendedExecutionVtbl {
        unsafe extern "system" fn OpenForExtendedExecutionAsync<Impl: IAppServiceConnectionExtendedExecutionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, operation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenForExtendedExecutionAsync(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&operation)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OpenForExtendedExecutionAsync: OpenForExtendedExecutionAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceConnectionExtendedExecution as ::windows::core::Interface>::IID
    }
}
pub trait IBufferByteAccessImpl: Sized {
    fn Buffer(&mut self) -> ::windows::core::Result<*mut u8>;
}
impl IBufferByteAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBufferByteAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBufferByteAccessVtbl {
        unsafe extern "system" fn Buffer<Impl: IBufferByteAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buffer() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Buffer: Buffer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBufferByteAccess as ::windows::core::Interface>::IID
    }
}
pub trait ICastingControllerImpl: Sized {
    fn Initialize(&mut self, castingengine: ::core::option::Option<::windows::core::IUnknown>, castingsource: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Connect(&mut self) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
    fn Advise(&mut self, eventhandler: ::core::option::Option<ICastingEventHandler>) -> ::windows::core::Result<u32>;
    fn UnAdvise(&mut self, cookie: u32) -> ::windows::core::Result<()>;
}
impl ICastingControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICastingControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICastingControllerVtbl {
        unsafe extern "system" fn Initialize<Impl: ICastingControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, castingengine: *mut ::core::ffi::c_void, castingsource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&castingengine), ::core::mem::transmute(&castingsource)).into()
        }
        unsafe extern "system" fn Connect<Impl: ICastingControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect().into()
        }
        unsafe extern "system" fn Disconnect<Impl: ICastingControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn Advise<Impl: ICastingControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Advise(::core::mem::transmute(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *cookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnAdvise<Impl: ICastingControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnAdvise(::core::mem::transmute_copy(&cookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            Advise: Advise::<Impl, IMPL_OFFSET>,
            UnAdvise: UnAdvise::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingController as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICastingEventHandlerImpl: Sized {
    fn OnStateChanged(&mut self, newstate: CASTING_CONNECTION_STATE) -> ::windows::core::Result<()>;
    fn OnError(&mut self, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICastingEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICastingEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICastingEventHandlerVtbl {
        unsafe extern "system" fn OnStateChanged<Impl: ICastingEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: CASTING_CONNECTION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStateChanged(::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn OnError<Impl: ICastingEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnError(::core::mem::transmute_copy(&errorstatus), ::core::mem::transmute_copy(&errormessage)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnStateChanged: OnStateChanged::<Impl, IMPL_OFFSET>,
            OnError: OnError::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ICastingSourceInfoImpl: Sized {
    fn GetController(&mut self) -> ::windows::core::Result<ICastingController>;
    fn GetProperties(&mut self) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::INamedPropertyStore>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ICastingSourceInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICastingSourceInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICastingSourceInfoVtbl {
        unsafe extern "system" fn GetController<Impl: ICastingSourceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, controller: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetController() {
                ::core::result::Result::Ok(ok__) => {
                    *controller = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Impl: ICastingSourceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, props: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *props = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetController: GetController::<Impl, IMPL_OFFSET>,
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICastingSourceInfo as ::windows::core::Interface>::IID
    }
}
pub trait ICorrelationVectorInformationImpl: Sized {
    fn LastCorrelationVectorForThread(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NextCorrelationVectorForThread(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNextCorrelationVectorForThread(&mut self, cv: ::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICorrelationVectorInformation {
    const NAME: &'static str = "";
}
impl ICorrelationVectorInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICorrelationVectorInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICorrelationVectorInformationVtbl {
        unsafe extern "system" fn LastCorrelationVectorForThread<Impl: ICorrelationVectorInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cv: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastCorrelationVectorForThread() {
                ::core::result::Result::Ok(ok__) => {
                    *cv = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextCorrelationVectorForThread<Impl: ICorrelationVectorInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cv: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextCorrelationVectorForThread() {
                ::core::result::Result::Ok(ok__) => {
                    *cv = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNextCorrelationVectorForThread<Impl: ICorrelationVectorInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cv: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNextCorrelationVectorForThread(::core::mem::transmute_copy(&cv)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICorrelationVectorInformation, BASE_OFFSET>(),
            LastCorrelationVectorForThread: LastCorrelationVectorForThread::<Impl, IMPL_OFFSET>,
            NextCorrelationVectorForThread: NextCorrelationVectorForThread::<Impl, IMPL_OFFSET>,
            SetNextCorrelationVectorForThread: SetNextCorrelationVectorForThread::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorrelationVectorInformation as ::windows::core::Interface>::IID
    }
}
pub trait ICorrelationVectorSourceImpl: Sized {
    fn CorrelationVector(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ICorrelationVectorSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICorrelationVectorSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICorrelationVectorSourceVtbl {
        unsafe extern "system" fn CorrelationVector<Impl: ICorrelationVectorSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cv: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CorrelationVector() {
                ::core::result::Result::Ok(ok__) => {
                    *cv = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CorrelationVector: CorrelationVector::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICorrelationVectorSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDragDropManagerInteropImpl: Sized {
    fn GetForWindow(&mut self, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDragDropManagerInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IDragDropManagerInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDragDropManagerInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDragDropManagerInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IDragDropManagerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDragDropManagerInterop, BASE_OFFSET>(),
            GetForWindow: GetForWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDragDropManagerInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHolographicSpaceInteropImpl: Sized {
    fn CreateForWindow(&mut self, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, holographicspace: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IHolographicSpaceInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IHolographicSpaceInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolographicSpaceInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolographicSpaceInteropVtbl {
        unsafe extern "system" fn CreateForWindow<Impl: IHolographicSpaceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, holographicspace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateForWindow(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&holographicspace)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHolographicSpaceInterop, BASE_OFFSET>(),
            CreateForWindow: CreateForWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolographicSpaceInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IInputPaneInteropImpl: Sized {
    fn GetForWindow(&mut self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, inputpane: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IInputPaneInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IInputPaneInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInputPaneInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInputPaneInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IInputPaneInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, inputpane: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&inputpane)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IInputPaneInterop, BASE_OFFSET>(), GetForWindow: GetForWindow::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInputPaneInterop as ::windows::core::Interface>::IID
    }
}
pub trait ILanguageExceptionErrorInfoImpl: Sized {
    fn GetLanguageException(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ILanguageExceptionErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanguageExceptionErrorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILanguageExceptionErrorInfoVtbl {
        unsafe extern "system" fn GetLanguageException<Impl: ILanguageExceptionErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageexception: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguageException() {
                ::core::result::Result::Ok(ok__) => {
                    *languageexception = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetLanguageException: GetLanguageException::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageExceptionErrorInfo as ::windows::core::Interface>::IID
    }
}
pub trait ILanguageExceptionErrorInfo2Impl: Sized + ILanguageExceptionErrorInfoImpl {
    fn GetPreviousLanguageExceptionErrorInfo(&mut self) -> ::windows::core::Result<ILanguageExceptionErrorInfo2>;
    fn CapturePropagationContext(&mut self, languageexception: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetPropagationContextHead(&mut self) -> ::windows::core::Result<ILanguageExceptionErrorInfo2>;
}
impl ILanguageExceptionErrorInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanguageExceptionErrorInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILanguageExceptionErrorInfo2Vtbl {
        unsafe extern "system" fn GetPreviousLanguageExceptionErrorInfo<Impl: ILanguageExceptionErrorInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, previouslanguageexceptionerrorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreviousLanguageExceptionErrorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *previouslanguageexceptionerrorinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturePropagationContext<Impl: ILanguageExceptionErrorInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, languageexception: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CapturePropagationContext(::core::mem::transmute(&languageexception)).into()
        }
        unsafe extern "system" fn GetPropagationContextHead<Impl: ILanguageExceptionErrorInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propagatedlanguageexceptionerrorinfohead: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropagationContextHead() {
                ::core::result::Result::Ok(ok__) => {
                    *propagatedlanguageexceptionerrorinfohead = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ILanguageExceptionErrorInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPreviousLanguageExceptionErrorInfo: GetPreviousLanguageExceptionErrorInfo::<Impl, IMPL_OFFSET>,
            CapturePropagationContext: CapturePropagationContext::<Impl, IMPL_OFFSET>,
            GetPropagationContextHead: GetPropagationContextHead::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageExceptionErrorInfo2 as ::windows::core::Interface>::IID
    }
}
pub trait ILanguageExceptionStackBackTraceImpl: Sized {
    fn GetStackBackTrace(&mut self, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows::core::Result<()>;
}
impl ILanguageExceptionStackBackTraceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanguageExceptionStackBackTraceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILanguageExceptionStackBackTraceVtbl {
        unsafe extern "system" fn GetStackBackTrace<Impl: ILanguageExceptionStackBackTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStackBackTrace(::core::mem::transmute_copy(&maxframestocapture), ::core::mem::transmute_copy(&stackbacktrace), ::core::mem::transmute_copy(&framescaptured)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetStackBackTrace: GetStackBackTrace::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageExceptionStackBackTrace as ::windows::core::Interface>::IID
    }
}
pub trait ILanguageExceptionTransformImpl: Sized {
    fn GetTransformedRestrictedErrorInfo(&mut self) -> ::windows::core::Result<IRestrictedErrorInfo>;
}
impl ILanguageExceptionTransformVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILanguageExceptionTransformImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILanguageExceptionTransformVtbl {
        unsafe extern "system" fn GetTransformedRestrictedErrorInfo<Impl: ILanguageExceptionTransformImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restrictederrorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransformedRestrictedErrorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *restrictederrorinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetTransformedRestrictedErrorInfo: GetTransformedRestrictedErrorInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILanguageExceptionTransform as ::windows::core::Interface>::IID
    }
}
pub trait IMemoryBufferByteAccessImpl: Sized {
    fn GetBuffer(&mut self, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::Result<()>;
}
impl IMemoryBufferByteAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryBufferByteAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryBufferByteAccessVtbl {
        unsafe extern "system" fn GetBuffer<Impl: IMemoryBufferByteAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBuffer(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&capacity)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetBuffer: GetBuffer::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryBufferByteAccess as ::windows::core::Interface>::IID
    }
}
pub trait IMessageDispatcherImpl: Sized {
    fn PumpMessages(&mut self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMessageDispatcher {
    const NAME: &'static str = "";
}
impl IMessageDispatcherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageDispatcherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageDispatcherVtbl {
        unsafe extern "system" fn PumpMessages<Impl: IMessageDispatcherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PumpMessages().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMessageDispatcher, BASE_OFFSET>(), PumpMessages: PumpMessages::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageDispatcher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPlayToManagerInteropImpl: Sized {
    fn GetForWindow(&mut self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, playtomanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn ShowPlayToUIForWindow(&mut self, appwindow: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPlayToManagerInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IPlayToManagerInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlayToManagerInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlayToManagerInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IPlayToManagerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, playtomanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&playtomanager)).into()
        }
        unsafe extern "system" fn ShowPlayToUIForWindow<Impl: IPlayToManagerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowPlayToUIForWindow(::core::mem::transmute_copy(&appwindow)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPlayToManagerInterop, BASE_OFFSET>(),
            GetForWindow: GetForWindow::<Impl, IMPL_OFFSET>,
            ShowPlayToUIForWindow: ShowPlayToUIForWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlayToManagerInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRestrictedErrorInfoImpl: Sized {
    fn GetErrorDetails(&mut self, description: *mut super::super::Foundation::BSTR, error: *mut ::windows::core::HRESULT, restricteddescription: *mut super::super::Foundation::BSTR, capabilitysid: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetReference(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRestrictedErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRestrictedErrorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRestrictedErrorInfoVtbl {
        unsafe extern "system" fn GetErrorDetails<Impl: IRestrictedErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR, error: *mut ::windows::core::HRESULT, restricteddescription: *mut super::super::Foundation::BSTR, capabilitysid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetErrorDetails(::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&error), ::core::mem::transmute_copy(&restricteddescription), ::core::mem::transmute_copy(&capabilitysid)).into()
        }
        unsafe extern "system" fn GetReference<Impl: IRestrictedErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reference: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReference() {
                ::core::result::Result::Ok(ok__) => {
                    *reference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetErrorDetails: GetErrorDetails::<Impl, IMPL_OFFSET>,
            GetReference: GetReference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRestrictedErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRoMetaDataLocatorImpl: Sized {
    fn Locate(&mut self, nameelement: super::super::Foundation::PWSTR, metadatadestination: ::core::option::Option<IRoSimpleMetaDataBuilder>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRoMetaDataLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRoMetaDataLocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRoMetaDataLocatorVtbl {
        unsafe extern "system" fn Locate<Impl: IRoMetaDataLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nameelement: super::super::Foundation::PWSTR, metadatadestination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Locate(::core::mem::transmute_copy(&nameelement), ::core::mem::transmute(&metadatadestination)).into()
        }
        Self { Locate: Locate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRoMetaDataLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRoSimpleMetaDataBuilderImpl: Sized {
    fn SetWinRtInterface(&mut self, iid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetDelegate(&mut self, iid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetInterfaceGroupSimpleDefault(&mut self, name: super::super::Foundation::PWSTR, defaultinterfacename: super::super::Foundation::PWSTR, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetInterfaceGroupParameterizedDefault(&mut self, name: super::super::Foundation::PWSTR, elementcount: u32, defaultinterfacenameelements: *const super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetRuntimeClassSimpleDefault(&mut self, name: super::super::Foundation::PWSTR, defaultinterfacename: super::super::Foundation::PWSTR, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetRuntimeClassParameterizedDefault(&mut self, name: super::super::Foundation::PWSTR, elementcount: u32, defaultinterfacenameelements: *const super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetStruct(&mut self, name: super::super::Foundation::PWSTR, numfields: u32, fieldtypenames: *const super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetEnum(&mut self, name: super::super::Foundation::PWSTR, basetype: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetParameterizedInterface(&mut self, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::Result<()>;
    fn SetParameterizedDelegate(&mut self, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRoSimpleMetaDataBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRoSimpleMetaDataBuilderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRoSimpleMetaDataBuilderVtbl {
        unsafe extern "system" fn SetWinRtInterface<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWinRtInterface(::core::mem::transmute_copy(&iid)).into()
        }
        unsafe extern "system" fn SetDelegate<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDelegate(::core::mem::transmute_copy(&iid)).into()
        }
        unsafe extern "system" fn SetInterfaceGroupSimpleDefault<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, defaultinterfacename: super::super::Foundation::PWSTR, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterfaceGroupSimpleDefault(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&defaultinterfacename), ::core::mem::transmute_copy(&defaultinterfaceiid)).into()
        }
        unsafe extern "system" fn SetInterfaceGroupParameterizedDefault<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, elementcount: u32, defaultinterfacenameelements: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterfaceGroupParameterizedDefault(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&elementcount), ::core::mem::transmute_copy(&defaultinterfacenameelements)).into()
        }
        unsafe extern "system" fn SetRuntimeClassSimpleDefault<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, defaultinterfacename: super::super::Foundation::PWSTR, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRuntimeClassSimpleDefault(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&defaultinterfacename), ::core::mem::transmute_copy(&defaultinterfaceiid)).into()
        }
        unsafe extern "system" fn SetRuntimeClassParameterizedDefault<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, elementcount: u32, defaultinterfacenameelements: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRuntimeClassParameterizedDefault(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&elementcount), ::core::mem::transmute_copy(&defaultinterfacenameelements)).into()
        }
        unsafe extern "system" fn SetStruct<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, numfields: u32, fieldtypenames: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStruct(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&numfields), ::core::mem::transmute_copy(&fieldtypenames)).into()
        }
        unsafe extern "system" fn SetEnum<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, basetype: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnum(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&basetype)).into()
        }
        unsafe extern "system" fn SetParameterizedInterface<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameterizedInterface(::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&numargs)).into()
        }
        unsafe extern "system" fn SetParameterizedDelegate<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetParameterizedDelegate(::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&numargs)).into()
        }
        Self {
            SetWinRtInterface: SetWinRtInterface::<Impl, IMPL_OFFSET>,
            SetDelegate: SetDelegate::<Impl, IMPL_OFFSET>,
            SetInterfaceGroupSimpleDefault: SetInterfaceGroupSimpleDefault::<Impl, IMPL_OFFSET>,
            SetInterfaceGroupParameterizedDefault: SetInterfaceGroupParameterizedDefault::<Impl, IMPL_OFFSET>,
            SetRuntimeClassSimpleDefault: SetRuntimeClassSimpleDefault::<Impl, IMPL_OFFSET>,
            SetRuntimeClassParameterizedDefault: SetRuntimeClassParameterizedDefault::<Impl, IMPL_OFFSET>,
            SetStruct: SetStruct::<Impl, IMPL_OFFSET>,
            SetEnum: SetEnum::<Impl, IMPL_OFFSET>,
            SetParameterizedInterface: SetParameterizedInterface::<Impl, IMPL_OFFSET>,
            SetParameterizedDelegate: SetParameterizedDelegate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRoSimpleMetaDataBuilder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IShareWindowCommandEventArgsInteropImpl: Sized {
    fn GetWindow(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl IShareWindowCommandEventArgsInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareWindowCommandEventArgsInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShareWindowCommandEventArgsInteropVtbl {
        unsafe extern "system" fn GetWindow<Impl: IShareWindowCommandEventArgsInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetWindow: GetWindow::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareWindowCommandEventArgsInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IShareWindowCommandSourceInteropImpl: Sized {
    fn GetForWindow(&mut self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, sharewindowcommandsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IShareWindowCommandSourceInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IShareWindowCommandSourceInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IShareWindowCommandSourceInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IShareWindowCommandSourceInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, sharewindowcommandsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&sharewindowcommandsource)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetForWindow: GetForWindow::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IShareWindowCommandSourceInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialInteractionManagerInteropImpl: Sized {
    fn GetForWindow(&mut self, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, spatialinteractionmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISpatialInteractionManagerInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl ISpatialInteractionManagerInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialInteractionManagerInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialInteractionManagerInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: ISpatialInteractionManagerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, spatialinteractionmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&spatialinteractionmanager)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISpatialInteractionManagerInterop, BASE_OFFSET>(),
            GetForWindow: GetForWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialInteractionManagerInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISystemMediaTransportControlsInteropImpl: Sized {
    fn GetForWindow(&mut self, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, mediatransportcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl ISystemMediaTransportControlsInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemMediaTransportControlsInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemMediaTransportControlsInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: ISystemMediaTransportControlsInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, mediatransportcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&mediatransportcontrol)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISystemMediaTransportControlsInterop, BASE_OFFSET>(),
            GetForWindow: GetForWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemMediaTransportControlsInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUIViewSettingsInteropImpl: Sized {
    fn GetForWindow(&mut self, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUIViewSettingsInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IUIViewSettingsInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIViewSettingsInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIViewSettingsInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IUIViewSettingsInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUIViewSettingsInterop, BASE_OFFSET>(), GetForWindow: GetForWindow::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIViewSettingsInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUserActivityInteropImpl: Sized {
    fn CreateSessionForWindow(&mut self, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUserActivityInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IUserActivityInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityInteropVtbl {
        unsafe extern "system" fn CreateSessionForWindow<Impl: IUserActivityInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateSessionForWindow(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityInterop, BASE_OFFSET>(),
            CreateSessionForWindow: CreateSessionForWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUserActivityRequestManagerInteropImpl: Sized {
    fn GetForWindow(&mut self, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUserActivityRequestManagerInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IUserActivityRequestManagerInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityRequestManagerInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityRequestManagerInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IUserActivityRequestManagerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetForWindow(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityRequestManagerInterop, BASE_OFFSET>(),
            GetForWindow: GetForWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityRequestManagerInterop as ::windows::core::Interface>::IID
    }
}
pub trait IUserActivitySourceHostInteropImpl: Sized {
    fn SetActivitySourceHost(&mut self, activitysourcehost: ::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IUserActivitySourceHostInterop {
    const NAME: &'static str = "";
}
impl IUserActivitySourceHostInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivitySourceHostInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivitySourceHostInteropVtbl {
        unsafe extern "system" fn SetActivitySourceHost<Impl: IUserActivitySourceHostInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activitysourcehost: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActivitySourceHost(::core::mem::transmute_copy(&activitysourcehost)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivitySourceHostInterop, BASE_OFFSET>(),
            SetActivitySourceHost: SetActivitySourceHost::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivitySourceHostInterop as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUserConsentVerifierInteropImpl: Sized {
    fn RequestVerificationForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, message: ::windows::core::HSTRING, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUserConsentVerifierInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IUserConsentVerifierInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserConsentVerifierInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserConsentVerifierInteropVtbl {
        unsafe extern "system" fn RequestVerificationForWindowAsync<Impl: IUserConsentVerifierInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestVerificationForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&message), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUserConsentVerifierInterop, BASE_OFFSET>(),
            RequestVerificationForWindowAsync: RequestVerificationForWindowAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserConsentVerifierInterop as ::windows::core::Interface>::IID
    }
}
pub trait IWeakReferenceImpl: Sized {
    fn Resolve(&mut self, riid: *const ::windows::core::GUID, objectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IWeakReferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWeakReferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWeakReferenceVtbl {
        unsafe extern "system" fn Resolve<Impl: IWeakReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, objectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resolve(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&objectreference)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Resolve: Resolve::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWeakReference as ::windows::core::Interface>::IID
    }
}
pub trait IWeakReferenceSourceImpl: Sized {
    fn GetWeakReference(&mut self) -> ::windows::core::Result<IWeakReference>;
}
impl IWeakReferenceSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWeakReferenceSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWeakReferenceSourceVtbl {
        unsafe extern "system" fn GetWeakReference<Impl: IWeakReferenceSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, weakreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWeakReference() {
                ::core::result::Result::Ok(ok__) => {
                    *weakreference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetWeakReference: GetWeakReference::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWeakReferenceSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWebAuthenticationCoreManagerInteropImpl: Sized {
    fn RequestTokenForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, request: ::core::option::Option<::windows::core::IInspectable>, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RequestTokenWithWebAccountForWindowAsync(&mut self, appwindow: super::super::Foundation::HWND, request: ::core::option::Option<::windows::core::IInspectable>, webaccount: ::core::option::Option<::windows::core::IInspectable>, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWebAuthenticationCoreManagerInterop {
    const NAME: &'static str = "";
}
#[cfg(feature = "Win32_Foundation")]
impl IWebAuthenticationCoreManagerInteropVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebAuthenticationCoreManagerInteropImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebAuthenticationCoreManagerInteropVtbl {
        unsafe extern "system" fn RequestTokenForWindowAsync<Impl: IWebAuthenticationCoreManagerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestTokenForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&request), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncinfo)).into()
        }
        unsafe extern "system" fn RequestTokenWithWebAccountForWindowAsync<Impl: IWebAuthenticationCoreManagerInteropImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestTokenWithWebAccountForWindowAsync(::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&request), ::core::mem::transmute(&webaccount), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncinfo)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebAuthenticationCoreManagerInterop, BASE_OFFSET>(),
            RequestTokenForWindowAsync: RequestTokenForWindowAsync::<Impl, IMPL_OFFSET>,
            RequestTokenWithWebAccountForWindowAsync: RequestTokenWithWebAccountForWindowAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebAuthenticationCoreManagerInterop as ::windows::core::Interface>::IID
    }
}
