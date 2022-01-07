pub trait IAccountsSettingsPaneInteropImpl: Sized {
    fn GetForWindow();
    fn ShowManageAccountsForWindowAsync();
    fn ShowAddAccountForWindowAsync();
}
impl ::windows::core::RuntimeName for IAccountsSettingsPaneInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IAccountsSettingsPaneInterop";
}
impl IAccountsSettingsPaneInteropVtbl {
    pub const fn new<Impl: IAccountsSettingsPaneInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccountsSettingsPaneInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IAccountsSettingsPaneInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, accountssettingspane: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&appwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&accountssettingspane)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowManageAccountsForWindowAsync<Impl: IAccountsSettingsPaneInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowManageAccountsForWindowAsync(&*(&appwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&asyncaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowAddAccountForWindowAsync<Impl: IAccountsSettingsPaneInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowAddAccountForWindowAsync(&*(&appwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&asyncaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccountsSettingsPaneInterop>, base.5, GetForWindow::<Impl, OFFSET>, ShowManageAccountsForWindowAsync::<Impl, OFFSET>, ShowAddAccountForWindowAsync::<Impl, OFFSET>)
    }
}
pub trait IActivationFactoryImpl: Sized {
    fn ActivateInstance();
}
impl ::windows::core::RuntimeName for IActivationFactory {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IActivationFactory";
}
impl IActivationFactoryVtbl {
    pub const fn new<Impl: IActivationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IActivationFactoryVtbl {
        unsafe extern "system" fn ActivateInstance<Impl: IActivationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, instance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivateInstance(::core::mem::transmute_copy(&instance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IActivationFactory>, base.5, ActivateInstance::<Impl, OFFSET>)
    }
}
pub trait IAgileReferenceImpl: Sized {
    fn Resolve();
}
impl ::windows::core::RuntimeName for IAgileReference {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IAgileReference";
}
impl IAgileReferenceVtbl {
    pub const fn new<Impl: IAgileReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAgileReferenceVtbl {
        unsafe extern "system" fn Resolve<Impl: IAgileReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resolve(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppvobjectreference as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAgileReference>, base.5, Resolve::<Impl, OFFSET>)
    }
}
pub trait IApartmentShutdownImpl: Sized {
    fn OnUninitialize();
}
impl ::windows::core::RuntimeName for IApartmentShutdown {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IApartmentShutdown";
}
impl IApartmentShutdownVtbl {
    pub const fn new<Impl: IApartmentShutdownImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IApartmentShutdownVtbl {
        unsafe extern "system" fn OnUninitialize<Impl: IApartmentShutdownImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ui64apartmentidentifier: u64) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).OnUninitialize(ui64apartmentidentifier).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IApartmentShutdown>, base.5, OnUninitialize::<Impl, OFFSET>)
    }
}
pub trait IAppServiceConnectionExtendedExecutionImpl: Sized {
    fn OpenForExtendedExecutionAsync();
}
impl ::windows::core::RuntimeName for IAppServiceConnectionExtendedExecution {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IAppServiceConnectionExtendedExecution";
}
impl IAppServiceConnectionExtendedExecutionVtbl {
    pub const fn new<Impl: IAppServiceConnectionExtendedExecutionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAppServiceConnectionExtendedExecutionVtbl {
        unsafe extern "system" fn OpenForExtendedExecutionAsync<Impl: IAppServiceConnectionExtendedExecutionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, operation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OpenForExtendedExecutionAsync(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&operation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAppServiceConnectionExtendedExecution>, base.5, OpenForExtendedExecutionAsync::<Impl, OFFSET>)
    }
}
pub trait IBufferByteAccessImpl: Sized {
    fn Buffer();
}
impl ::windows::core::RuntimeName for IBufferByteAccess {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IBufferByteAccess";
}
impl IBufferByteAccessVtbl {
    pub const fn new<Impl: IBufferByteAccessImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBufferByteAccessVtbl {
        unsafe extern "system" fn Buffer<Impl: IBufferByteAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Buffer(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBufferByteAccess>, base.5, Buffer::<Impl, OFFSET>)
    }
}
pub trait ICastingControllerImpl: Sized {
    fn Initialize();
    fn Connect();
    fn Disconnect();
    fn Advise();
    fn UnAdvise();
}
impl ::windows::core::RuntimeName for ICastingController {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ICastingController";
}
impl ICastingControllerVtbl {
    pub const fn new<Impl: ICastingControllerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICastingControllerVtbl {
        unsafe extern "system" fn Initialize<Impl: ICastingControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, castingengine: *mut ::core::ffi::c_void, castingsource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&castingengine as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&castingsource as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: ICastingControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Connect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: ICastingControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Impl: ICastingControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Advise(&*(&eventhandler as *const <ICastingEventHandler as ::windows::core::Abi>::Abi as *const <ICastingEventHandler as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnAdvise<Impl: ICastingControllerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnAdvise(cookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICastingController>, base.5, Initialize::<Impl, OFFSET>, Connect::<Impl, OFFSET>, Disconnect::<Impl, OFFSET>, Advise::<Impl, OFFSET>, UnAdvise::<Impl, OFFSET>)
    }
}
pub trait ICastingEventHandlerImpl: Sized {
    fn OnStateChanged();
    fn OnError();
}
impl ::windows::core::RuntimeName for ICastingEventHandler {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ICastingEventHandler";
}
impl ICastingEventHandlerVtbl {
    pub const fn new<Impl: ICastingEventHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICastingEventHandlerVtbl {
        unsafe extern "system" fn OnStateChanged<Impl: ICastingEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstate: CASTING_CONNECTION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnStateChanged(newstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnError<Impl: ICastingEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnError(errorstatus, &*(&errormessage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICastingEventHandler>, base.5, OnStateChanged::<Impl, OFFSET>, OnError::<Impl, OFFSET>)
    }
}
pub trait ICastingSourceInfoImpl: Sized {
    fn GetController();
    fn GetProperties();
}
impl ::windows::core::RuntimeName for ICastingSourceInfo {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ICastingSourceInfo";
}
impl ICastingSourceInfoVtbl {
    pub const fn new<Impl: ICastingSourceInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICastingSourceInfoVtbl {
        unsafe extern "system" fn GetController<Impl: ICastingSourceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, controller: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetController(::core::mem::transmute_copy(&controller)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Impl: ICastingSourceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, props: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute_copy(&props)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICastingSourceInfo>, base.5, GetController::<Impl, OFFSET>, GetProperties::<Impl, OFFSET>)
    }
}
pub trait ICorrelationVectorInformationImpl: Sized {
    fn LastCorrelationVectorForThread();
    fn NextCorrelationVectorForThread();
    fn SetNextCorrelationVectorForThread();
}
impl ::windows::core::RuntimeName for ICorrelationVectorInformation {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ICorrelationVectorInformation";
}
impl ICorrelationVectorInformationVtbl {
    pub const fn new<Impl: ICorrelationVectorInformationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICorrelationVectorInformationVtbl {
        unsafe extern "system" fn LastCorrelationVectorForThread<Impl: ICorrelationVectorInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cv: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LastCorrelationVectorForThread(::core::mem::transmute_copy(&cv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextCorrelationVectorForThread<Impl: ICorrelationVectorInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cv: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NextCorrelationVectorForThread(::core::mem::transmute_copy(&cv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNextCorrelationVectorForThread<Impl: ICorrelationVectorInformationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cv: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNextCorrelationVectorForThread(&*(&cv as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICorrelationVectorInformation>, base.5, LastCorrelationVectorForThread::<Impl, OFFSET>, NextCorrelationVectorForThread::<Impl, OFFSET>, SetNextCorrelationVectorForThread::<Impl, OFFSET>)
    }
}
pub trait ICorrelationVectorSourceImpl: Sized {
    fn CorrelationVector();
}
impl ::windows::core::RuntimeName for ICorrelationVectorSource {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ICorrelationVectorSource";
}
impl ICorrelationVectorSourceVtbl {
    pub const fn new<Impl: ICorrelationVectorSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICorrelationVectorSourceVtbl {
        unsafe extern "system" fn CorrelationVector<Impl: ICorrelationVectorSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cv: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CorrelationVector(::core::mem::transmute_copy(&cv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICorrelationVectorSource>, base.5, CorrelationVector::<Impl, OFFSET>)
    }
}
pub trait IDragDropManagerInteropImpl: Sized {
    fn GetForWindow();
}
impl ::windows::core::RuntimeName for IDragDropManagerInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IDragDropManagerInterop";
}
impl IDragDropManagerInteropVtbl {
    pub const fn new<Impl: IDragDropManagerInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDragDropManagerInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IDragDropManagerInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDragDropManagerInterop>, base.5, GetForWindow::<Impl, OFFSET>)
    }
}
pub trait IHolographicSpaceInteropImpl: Sized {
    fn CreateForWindow();
}
impl ::windows::core::RuntimeName for IHolographicSpaceInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IHolographicSpaceInterop";
}
impl IHolographicSpaceInteropVtbl {
    pub const fn new<Impl: IHolographicSpaceInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IHolographicSpaceInteropVtbl {
        unsafe extern "system" fn CreateForWindow<Impl: IHolographicSpaceInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, holographicspace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateForWindow(&*(&window as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&holographicspace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IHolographicSpaceInterop>, base.5, CreateForWindow::<Impl, OFFSET>)
    }
}
pub trait IInputPaneInteropImpl: Sized {
    fn GetForWindow();
}
impl ::windows::core::RuntimeName for IInputPaneInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IInputPaneInterop";
}
impl IInputPaneInteropVtbl {
    pub const fn new<Impl: IInputPaneInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IInputPaneInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IInputPaneInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, inputpane: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&appwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&inputpane)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IInputPaneInterop>, base.5, GetForWindow::<Impl, OFFSET>)
    }
}
pub trait ILanguageExceptionErrorInfoImpl: Sized {
    fn GetLanguageException();
}
impl ::windows::core::RuntimeName for ILanguageExceptionErrorInfo {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ILanguageExceptionErrorInfo";
}
impl ILanguageExceptionErrorInfoVtbl {
    pub const fn new<Impl: ILanguageExceptionErrorInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILanguageExceptionErrorInfoVtbl {
        unsafe extern "system" fn GetLanguageException<Impl: ILanguageExceptionErrorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageexception: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLanguageException(::core::mem::transmute_copy(&languageexception)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILanguageExceptionErrorInfo>, base.5, GetLanguageException::<Impl, OFFSET>)
    }
}
pub trait ILanguageExceptionErrorInfo2Impl: Sized + ILanguageExceptionErrorInfoImpl {
    fn GetPreviousLanguageExceptionErrorInfo();
    fn CapturePropagationContext();
    fn GetPropagationContextHead();
}
impl ::windows::core::RuntimeName for ILanguageExceptionErrorInfo2 {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ILanguageExceptionErrorInfo2";
}
impl ILanguageExceptionErrorInfo2Vtbl {
    pub const fn new<Impl: ILanguageExceptionErrorInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILanguageExceptionErrorInfo2Vtbl {
        unsafe extern "system" fn GetPreviousLanguageExceptionErrorInfo<Impl: ILanguageExceptionErrorInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previouslanguageexceptionerrorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreviousLanguageExceptionErrorInfo(::core::mem::transmute_copy(&previouslanguageexceptionerrorinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CapturePropagationContext<Impl: ILanguageExceptionErrorInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageexception: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CapturePropagationContext(&*(&languageexception as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropagationContextHead<Impl: ILanguageExceptionErrorInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propagatedlanguageexceptionerrorinfohead: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPropagationContextHead(::core::mem::transmute_copy(&propagatedlanguageexceptionerrorinfohead)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILanguageExceptionErrorInfo2>, base.5, GetPreviousLanguageExceptionErrorInfo::<Impl, OFFSET>, CapturePropagationContext::<Impl, OFFSET>, GetPropagationContextHead::<Impl, OFFSET>)
    }
}
pub trait ILanguageExceptionStackBackTraceImpl: Sized {
    fn GetStackBackTrace();
}
impl ::windows::core::RuntimeName for ILanguageExceptionStackBackTrace {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ILanguageExceptionStackBackTrace";
}
impl ILanguageExceptionStackBackTraceVtbl {
    pub const fn new<Impl: ILanguageExceptionStackBackTraceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILanguageExceptionStackBackTraceVtbl {
        unsafe extern "system" fn GetStackBackTrace<Impl: ILanguageExceptionStackBackTraceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStackBackTrace(maxframestocapture, stackbacktrace, ::core::mem::transmute_copy(&framescaptured)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILanguageExceptionStackBackTrace>, base.5, GetStackBackTrace::<Impl, OFFSET>)
    }
}
pub trait ILanguageExceptionTransformImpl: Sized {
    fn GetTransformedRestrictedErrorInfo();
}
impl ::windows::core::RuntimeName for ILanguageExceptionTransform {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ILanguageExceptionTransform";
}
impl ILanguageExceptionTransformVtbl {
    pub const fn new<Impl: ILanguageExceptionTransformImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ILanguageExceptionTransformVtbl {
        unsafe extern "system" fn GetTransformedRestrictedErrorInfo<Impl: ILanguageExceptionTransformImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restrictederrorinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTransformedRestrictedErrorInfo(::core::mem::transmute_copy(&restrictederrorinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ILanguageExceptionTransform>, base.5, GetTransformedRestrictedErrorInfo::<Impl, OFFSET>)
    }
}
pub trait IMemoryBufferByteAccessImpl: Sized {
    fn GetBuffer();
}
impl ::windows::core::RuntimeName for IMemoryBufferByteAccess {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IMemoryBufferByteAccess";
}
impl IMemoryBufferByteAccessVtbl {
    pub const fn new<Impl: IMemoryBufferByteAccessImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMemoryBufferByteAccessVtbl {
        unsafe extern "system" fn GetBuffer<Impl: IMemoryBufferByteAccessImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetBuffer(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&capacity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMemoryBufferByteAccess>, base.5, GetBuffer::<Impl, OFFSET>)
    }
}
pub trait IMessageDispatcherImpl: Sized {
    fn PumpMessages();
}
impl ::windows::core::RuntimeName for IMessageDispatcher {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IMessageDispatcher";
}
impl IMessageDispatcherVtbl {
    pub const fn new<Impl: IMessageDispatcherImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMessageDispatcherVtbl {
        unsafe extern "system" fn PumpMessages<Impl: IMessageDispatcherImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PumpMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMessageDispatcher>, base.5, PumpMessages::<Impl, OFFSET>)
    }
}
pub trait IPlayToManagerInteropImpl: Sized {
    fn GetForWindow();
    fn ShowPlayToUIForWindow();
}
impl ::windows::core::RuntimeName for IPlayToManagerInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IPlayToManagerInterop";
}
impl IPlayToManagerInteropVtbl {
    pub const fn new<Impl: IPlayToManagerInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPlayToManagerInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IPlayToManagerInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, playtomanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&appwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&playtomanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowPlayToUIForWindow<Impl: IPlayToManagerInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowPlayToUIForWindow(&*(&appwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPlayToManagerInterop>, base.5, GetForWindow::<Impl, OFFSET>, ShowPlayToUIForWindow::<Impl, OFFSET>)
    }
}
pub trait IRestrictedErrorInfoImpl: Sized {
    fn GetErrorDetails();
    fn GetReference();
}
impl ::windows::core::RuntimeName for IRestrictedErrorInfo {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IRestrictedErrorInfo";
}
impl IRestrictedErrorInfoVtbl {
    pub const fn new<Impl: IRestrictedErrorInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRestrictedErrorInfoVtbl {
        unsafe extern "system" fn GetErrorDetails<Impl: IRestrictedErrorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut super::super::Foundation::BSTR, error: *mut ::windows::core::HRESULT, restricteddescription: *mut super::super::Foundation::BSTR, capabilitysid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorDetails(::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&error), ::core::mem::transmute_copy(&restricteddescription), ::core::mem::transmute_copy(&capabilitysid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReference<Impl: IRestrictedErrorInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReference(::core::mem::transmute_copy(&reference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRestrictedErrorInfo>, base.5, GetErrorDetails::<Impl, OFFSET>, GetReference::<Impl, OFFSET>)
    }
}
pub trait IRoMetaDataLocatorImpl: Sized {
    fn Locate();
}
impl ::windows::core::RuntimeName for IRoMetaDataLocator {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IRoMetaDataLocator";
}
impl IRoMetaDataLocatorVtbl {
    pub const fn new<Impl: IRoMetaDataLocatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRoMetaDataLocatorVtbl {
        unsafe extern "system" fn Locate<Impl: IRoMetaDataLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nameelement: super::super::Foundation::PWSTR, metadatadestination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Locate(&*(&nameelement as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&metadatadestination as *const <IRoSimpleMetaDataBuilder as ::windows::core::Abi>::Abi as *const <IRoSimpleMetaDataBuilder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRoMetaDataLocator>, base.5, Locate::<Impl, OFFSET>)
    }
}
pub trait IRoSimpleMetaDataBuilderImpl: Sized {
    fn SetWinRtInterface();
    fn SetDelegate();
    fn SetInterfaceGroupSimpleDefault();
    fn SetInterfaceGroupParameterizedDefault();
    fn SetRuntimeClassSimpleDefault();
    fn SetRuntimeClassParameterizedDefault();
    fn SetStruct();
    fn SetEnum();
    fn SetParameterizedInterface();
    fn SetParameterizedDelegate();
}
impl ::windows::core::RuntimeName for IRoSimpleMetaDataBuilder {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IRoSimpleMetaDataBuilder";
}
impl IRoSimpleMetaDataBuilderVtbl {
    pub const fn new<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRoSimpleMetaDataBuilderVtbl {
        unsafe extern "system" fn SetWinRtInterface<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetWinRtInterface(&*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDelegate<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDelegate(&*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaceGroupSimpleDefault<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, defaultinterfacename: super::super::Foundation::PWSTR, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInterfaceGroupSimpleDefault(
                &*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&defaultinterfacename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&defaultinterfaceiid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaceGroupParameterizedDefault<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, elementcount: u32, defaultinterfacenameelements: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInterfaceGroupParameterizedDefault(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), elementcount, &*(&defaultinterfacenameelements as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuntimeClassSimpleDefault<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, defaultinterfacename: super::super::Foundation::PWSTR, defaultinterfaceiid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRuntimeClassSimpleDefault(
                &*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&defaultinterfacename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&defaultinterfaceiid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRuntimeClassParameterizedDefault<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, elementcount: u32, defaultinterfacenameelements: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRuntimeClassParameterizedDefault(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), elementcount, &*(&defaultinterfacenameelements as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStruct<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, numfields: u32, fieldtypenames: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetStruct(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), numfields, &*(&fieldtypenames as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnum<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, basetype: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetEnum(&*(&name as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&basetype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameterizedInterface<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetParameterizedInterface(&*(&piid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), numargs) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParameterizedDelegate<Impl: IRoSimpleMetaDataBuilderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: ::windows::core::GUID, numargs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetParameterizedDelegate(&*(&piid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), numargs) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IRoSimpleMetaDataBuilder>,
            base.5,
            SetWinRtInterface::<Impl, OFFSET>,
            SetDelegate::<Impl, OFFSET>,
            SetInterfaceGroupSimpleDefault::<Impl, OFFSET>,
            SetInterfaceGroupParameterizedDefault::<Impl, OFFSET>,
            SetRuntimeClassSimpleDefault::<Impl, OFFSET>,
            SetRuntimeClassParameterizedDefault::<Impl, OFFSET>,
            SetStruct::<Impl, OFFSET>,
            SetEnum::<Impl, OFFSET>,
            SetParameterizedInterface::<Impl, OFFSET>,
            SetParameterizedDelegate::<Impl, OFFSET>,
        )
    }
}
pub trait IShareWindowCommandEventArgsInteropImpl: Sized {
    fn GetWindow();
}
impl ::windows::core::RuntimeName for IShareWindowCommandEventArgsInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IShareWindowCommandEventArgsInterop";
}
impl IShareWindowCommandEventArgsInteropVtbl {
    pub const fn new<Impl: IShareWindowCommandEventArgsInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IShareWindowCommandEventArgsInteropVtbl {
        unsafe extern "system" fn GetWindow<Impl: IShareWindowCommandEventArgsInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWindow(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IShareWindowCommandEventArgsInterop>, base.5, GetWindow::<Impl, OFFSET>)
    }
}
pub trait IShareWindowCommandSourceInteropImpl: Sized {
    fn GetForWindow();
}
impl ::windows::core::RuntimeName for IShareWindowCommandSourceInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IShareWindowCommandSourceInterop";
}
impl IShareWindowCommandSourceInteropVtbl {
    pub const fn new<Impl: IShareWindowCommandSourceInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IShareWindowCommandSourceInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IShareWindowCommandSourceInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, sharewindowcommandsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&appwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&sharewindowcommandsource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IShareWindowCommandSourceInterop>, base.5, GetForWindow::<Impl, OFFSET>)
    }
}
pub trait ISpatialInteractionManagerInteropImpl: Sized {
    fn GetForWindow();
}
impl ::windows::core::RuntimeName for ISpatialInteractionManagerInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ISpatialInteractionManagerInterop";
}
impl ISpatialInteractionManagerInteropVtbl {
    pub const fn new<Impl: ISpatialInteractionManagerInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISpatialInteractionManagerInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: ISpatialInteractionManagerInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, spatialinteractionmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&window as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&spatialinteractionmanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISpatialInteractionManagerInterop>, base.5, GetForWindow::<Impl, OFFSET>)
    }
}
pub trait ISystemMediaTransportControlsInteropImpl: Sized {
    fn GetForWindow();
}
impl ::windows::core::RuntimeName for ISystemMediaTransportControlsInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.ISystemMediaTransportControlsInterop";
}
impl ISystemMediaTransportControlsInteropVtbl {
    pub const fn new<Impl: ISystemMediaTransportControlsInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISystemMediaTransportControlsInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: ISystemMediaTransportControlsInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, mediatransportcontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&appwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&mediatransportcontrol)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISystemMediaTransportControlsInterop>, base.5, GetForWindow::<Impl, OFFSET>)
    }
}
pub trait IUIViewSettingsInteropImpl: Sized {
    fn GetForWindow();
}
impl ::windows::core::RuntimeName for IUIViewSettingsInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IUIViewSettingsInterop";
}
impl IUIViewSettingsInteropVtbl {
    pub const fn new<Impl: IUIViewSettingsInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUIViewSettingsInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IUIViewSettingsInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUIViewSettingsInterop>, base.5, GetForWindow::<Impl, OFFSET>)
    }
}
pub trait IUserActivityInteropImpl: Sized {
    fn CreateSessionForWindow();
}
impl ::windows::core::RuntimeName for IUserActivityInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IUserActivityInterop";
}
impl IUserActivityInteropVtbl {
    pub const fn new<Impl: IUserActivityInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserActivityInteropVtbl {
        unsafe extern "system" fn CreateSessionForWindow<Impl: IUserActivityInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSessionForWindow(&*(&window as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserActivityInterop>, base.5, CreateSessionForWindow::<Impl, OFFSET>)
    }
}
pub trait IUserActivityRequestManagerInteropImpl: Sized {
    fn GetForWindow();
}
impl ::windows::core::RuntimeName for IUserActivityRequestManagerInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IUserActivityRequestManagerInterop";
}
impl IUserActivityRequestManagerInteropVtbl {
    pub const fn new<Impl: IUserActivityRequestManagerInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserActivityRequestManagerInteropVtbl {
        unsafe extern "system" fn GetForWindow<Impl: IUserActivityRequestManagerInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const ::windows::core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetForWindow(&*(&window as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserActivityRequestManagerInterop>, base.5, GetForWindow::<Impl, OFFSET>)
    }
}
pub trait IUserActivitySourceHostInteropImpl: Sized {
    fn SetActivitySourceHost();
}
impl ::windows::core::RuntimeName for IUserActivitySourceHostInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IUserActivitySourceHostInterop";
}
impl IUserActivitySourceHostInteropVtbl {
    pub const fn new<Impl: IUserActivitySourceHostInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserActivitySourceHostInteropVtbl {
        unsafe extern "system" fn SetActivitySourceHost<Impl: IUserActivitySourceHostInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activitysourcehost: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetActivitySourceHost(&*(&activitysourcehost as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserActivitySourceHostInterop>, base.5, SetActivitySourceHost::<Impl, OFFSET>)
    }
}
pub trait IUserConsentVerifierInteropImpl: Sized {
    fn RequestVerificationForWindowAsync();
}
impl ::windows::core::RuntimeName for IUserConsentVerifierInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IUserConsentVerifierInterop";
}
impl IUserConsentVerifierInteropVtbl {
    pub const fn new<Impl: IUserConsentVerifierInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IUserConsentVerifierInteropVtbl {
        unsafe extern "system" fn RequestVerificationForWindowAsync<Impl: IUserConsentVerifierInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, riid: *const ::windows::core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestVerificationForWindowAsync(
                &*(&appwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&asyncoperation),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IUserConsentVerifierInterop>, base.5, RequestVerificationForWindowAsync::<Impl, OFFSET>)
    }
}
pub trait IWeakReferenceImpl: Sized {
    fn Resolve();
}
impl ::windows::core::RuntimeName for IWeakReference {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IWeakReference";
}
impl IWeakReferenceVtbl {
    pub const fn new<Impl: IWeakReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWeakReferenceVtbl {
        unsafe extern "system" fn Resolve<Impl: IWeakReferenceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, objectreference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resolve(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&objectreference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWeakReference>, base.5, Resolve::<Impl, OFFSET>)
    }
}
pub trait IWeakReferenceSourceImpl: Sized {
    fn GetWeakReference();
}
impl ::windows::core::RuntimeName for IWeakReferenceSource {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IWeakReferenceSource";
}
impl IWeakReferenceSourceVtbl {
    pub const fn new<Impl: IWeakReferenceSourceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWeakReferenceSourceVtbl {
        unsafe extern "system" fn GetWeakReference<Impl: IWeakReferenceSourceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, weakreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetWeakReference(::core::mem::transmute_copy(&weakreference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWeakReferenceSource>, base.5, GetWeakReference::<Impl, OFFSET>)
    }
}
pub trait IWebAuthenticationCoreManagerInteropImpl: Sized {
    fn RequestTokenForWindowAsync();
    fn RequestTokenWithWebAccountForWindowAsync();
}
impl ::windows::core::RuntimeName for IWebAuthenticationCoreManagerInterop {
    const NAME: &'static str = "Windows.Win32.System.WinRT.IWebAuthenticationCoreManagerInterop";
}
impl IWebAuthenticationCoreManagerInteropVtbl {
    pub const fn new<Impl: IWebAuthenticationCoreManagerInteropImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWebAuthenticationCoreManagerInteropVtbl {
        unsafe extern "system" fn RequestTokenForWindowAsync<Impl: IWebAuthenticationCoreManagerInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestTokenForWindowAsync(
                &*(&appwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&request as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&asyncinfo),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestTokenWithWebAccountForWindowAsync<Impl: IWebAuthenticationCoreManagerInteropImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestTokenWithWebAccountForWindowAsync(
                &*(&appwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&request as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&webaccount as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&asyncinfo),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWebAuthenticationCoreManagerInterop>, base.5, RequestTokenForWindowAsync::<Impl, OFFSET>, RequestTokenWithWebAccountForWindowAsync::<Impl, OFFSET>)
    }
}
