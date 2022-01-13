#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppServiceCatalogStaticsImpl: Sized {
    fn FindAppServiceProvidersAsync(&mut self, appservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::AppInfo>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppServiceCatalogStatics {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IAppServiceCatalogStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppServiceCatalogStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceCatalogStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceCatalogStaticsVtbl {
        unsafe extern "system" fn FindAppServiceProvidersAsync<Impl: IAppServiceCatalogStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindAppServiceProvidersAsync(&*(&appservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppServiceCatalogStatics, BASE_OFFSET>(),
            FindAppServiceProvidersAsync: FindAppServiceProvidersAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceCatalogStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceClosedEventArgsImpl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<AppServiceClosedStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppServiceClosedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IAppServiceClosedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppServiceClosedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceClosedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceClosedEventArgsVtbl {
        unsafe extern "system" fn Status<Impl: IAppServiceClosedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppServiceClosedStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppServiceClosedEventArgs, BASE_OFFSET>(), Status: Status::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceClosedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppServiceConnectionImpl: Sized {
    fn AppServiceName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetAppServiceName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetPackageFamilyName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn OpenAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>>;
    fn SendMessageAsync(&mut self, message: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppServiceResponse>>;
    fn RequestReceived(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceRequestReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRequestReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ServiceClosed(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServiceClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppServiceConnection {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IAppServiceConnection";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppServiceConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceConnectionVtbl {
        unsafe extern "system" fn AppServiceName<Impl: IAppServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppServiceName<Impl: IAppServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppServiceName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PackageFamilyName<Impl: IAppServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPackageFamilyName<Impl: IAppServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPackageFamilyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OpenAsync<Impl: IAppServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendMessageAsync<Impl: IAppServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendMessageAsync(&*(&message as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestReceived<Impl: IAppServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestReceived(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceRequestReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceRequestReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRequestReceived<Impl: IAppServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRequestReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServiceClosed<Impl: IAppServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceClosed(&*(&handler as *const <super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<AppServiceConnection, AppServiceClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveServiceClosed<Impl: IAppServiceConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveServiceClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppServiceConnection, BASE_OFFSET>(),
            AppServiceName: AppServiceName::<Impl, IMPL_OFFSET>,
            SetAppServiceName: SetAppServiceName::<Impl, IMPL_OFFSET>,
            PackageFamilyName: PackageFamilyName::<Impl, IMPL_OFFSET>,
            SetPackageFamilyName: SetPackageFamilyName::<Impl, IMPL_OFFSET>,
            OpenAsync: OpenAsync::<Impl, IMPL_OFFSET>,
            SendMessageAsync: SendMessageAsync::<Impl, IMPL_OFFSET>,
            RequestReceived: RequestReceived::<Impl, IMPL_OFFSET>,
            RemoveRequestReceived: RemoveRequestReceived::<Impl, IMPL_OFFSET>,
            ServiceClosed: ServiceClosed::<Impl, IMPL_OFFSET>,
            RemoveServiceClosed: RemoveServiceClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "System_RemoteSystems", feature = "implement_exclusive"))]
pub trait IAppServiceConnection2Impl: Sized {
    fn OpenRemoteAsync(&mut self, remotesystemconnectionrequest: &::core::option::Option<super::super::System::RemoteSystems::RemoteSystemConnectionRequest>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppServiceConnectionStatus>>;
    fn User(&mut self) -> ::windows::core::Result<super::super::System::User>;
    fn SetUser(&mut self, value: &::core::option::Option<super::super::System::User>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "System_RemoteSystems", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppServiceConnection2 {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IAppServiceConnection2";
}
#[cfg(all(feature = "Foundation", feature = "System", feature = "System_RemoteSystems", feature = "implement_exclusive"))]
impl IAppServiceConnection2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceConnection2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceConnection2Vtbl {
        unsafe extern "system" fn OpenRemoteAsync<Impl: IAppServiceConnection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotesystemconnectionrequest: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenRemoteAsync(&*(&remotesystemconnectionrequest as *const <super::super::System::RemoteSystems::RemoteSystemConnectionRequest as ::windows::core::Abi>::Abi as *const <super::super::System::RemoteSystems::RemoteSystemConnectionRequest as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn User<Impl: IAppServiceConnection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).User() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUser<Impl: IAppServiceConnection2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUser(&*(&value as *const <super::super::System::User as ::windows::core::Abi>::Abi as *const <super::super::System::User as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppServiceConnection2, BASE_OFFSET>(),
            OpenRemoteAsync: OpenRemoteAsync::<Impl, IMPL_OFFSET>,
            User: User::<Impl, IMPL_OFFSET>,
            SetUser: SetUser::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceConnection2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems", feature = "implement_exclusive"))]
pub trait IAppServiceConnectionStaticsImpl: Sized {
    fn SendStatelessMessageAsync(&mut self, connection: &::core::option::Option<AppServiceConnection>, connectionrequest: &::core::option::Option<super::super::System::RemoteSystems::RemoteSystemConnectionRequest>, message: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<StatelessAppServiceResponse>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppServiceConnectionStatics {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IAppServiceConnectionStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "System_RemoteSystems", feature = "implement_exclusive"))]
impl IAppServiceConnectionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceConnectionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceConnectionStaticsVtbl {
        unsafe extern "system" fn SendStatelessMessageAsync<Impl: IAppServiceConnectionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connection: ::windows::core::RawPtr, connectionrequest: ::windows::core::RawPtr, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendStatelessMessageAsync(
                &*(&connection as *const <AppServiceConnection as ::windows::core::Abi>::Abi as *const <AppServiceConnection as ::windows::core::DefaultType>::DefaultType),
                &*(&connectionrequest as *const <super::super::System::RemoteSystems::RemoteSystemConnectionRequest as ::windows::core::Abi>::Abi as *const <super::super::System::RemoteSystems::RemoteSystemConnectionRequest as ::windows::core::DefaultType>::DefaultType),
                &*(&message as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppServiceConnectionStatics, BASE_OFFSET>(),
            SendStatelessMessageAsync: SendStatelessMessageAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceConnectionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceDeferralImpl: Sized {
    fn Complete(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppServiceDeferral {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IAppServiceDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IAppServiceDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IAppServiceDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAppServiceDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppServiceRequestImpl: Sized {
    fn Message(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn SendResponseAsync(&mut self, message: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppServiceResponseStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppServiceRequest {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IAppServiceRequest";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppServiceRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceRequestImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceRequestVtbl {
        unsafe extern "system" fn Message<Impl: IAppServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendResponseAsync<Impl: IAppServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendResponseAsync(&*(&message as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppServiceRequest, BASE_OFFSET>(),
            Message: Message::<Impl, IMPL_OFFSET>,
            SendResponseAsync: SendResponseAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceRequestReceivedEventArgsImpl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<AppServiceRequest>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<AppServiceDeferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppServiceRequestReceivedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IAppServiceRequestReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IAppServiceRequestReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceRequestReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceRequestReceivedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IAppServiceRequestReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IAppServiceRequestReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppServiceRequestReceivedEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceRequestReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IAppServiceResponseImpl: Sized {
    fn Message(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn Status(&mut self) -> ::windows::core::Result<AppServiceResponseStatus>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppServiceResponse {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IAppServiceResponse";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IAppServiceResponseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceResponseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceResponseVtbl {
        unsafe extern "system" fn Message<Impl: IAppServiceResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IAppServiceResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AppServiceResponseStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppServiceResponse, BASE_OFFSET>(),
            Message: Message::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceResponse as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceTriggerDetailsImpl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CallerPackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppServiceConnection(&mut self) -> ::windows::core::Result<AppServiceConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppServiceTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IAppServiceTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IAppServiceTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceTriggerDetailsVtbl {
        unsafe extern "system" fn Name<Impl: IAppServiceTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CallerPackageFamilyName<Impl: IAppServiceTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallerPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppServiceConnection<Impl: IAppServiceTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppServiceConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppServiceTriggerDetails, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            CallerPackageFamilyName: CallerPackageFamilyName::<Impl, IMPL_OFFSET>,
            AppServiceConnection: AppServiceConnection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceTriggerDetails2Impl: Sized {
    fn IsRemoteSystemConnection(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppServiceTriggerDetails2 {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IAppServiceTriggerDetails2";
}
#[cfg(feature = "implement_exclusive")]
impl IAppServiceTriggerDetails2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceTriggerDetails2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceTriggerDetails2Vtbl {
        unsafe extern "system" fn IsRemoteSystemConnection<Impl: IAppServiceTriggerDetails2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRemoteSystemConnection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppServiceTriggerDetails2, BASE_OFFSET>(),
            IsRemoteSystemConnection: IsRemoteSystemConnection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceTriggerDetails2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IAppServiceTriggerDetails3Impl: Sized {
    fn CheckCallerForCapabilityAsync(&mut self, capabilityname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IAppServiceTriggerDetails3 {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IAppServiceTriggerDetails3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IAppServiceTriggerDetails3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceTriggerDetails3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceTriggerDetails3Vtbl {
        unsafe extern "system" fn CheckCallerForCapabilityAsync<Impl: IAppServiceTriggerDetails3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capabilityname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckCallerForCapabilityAsync(&*(&capabilityname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppServiceTriggerDetails3, BASE_OFFSET>(),
            CheckCallerForCapabilityAsync: CheckCallerForCapabilityAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceTriggerDetails3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppServiceTriggerDetails4Impl: Sized {
    fn CallerRemoteConnectionToken(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAppServiceTriggerDetails4 {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IAppServiceTriggerDetails4";
}
#[cfg(feature = "implement_exclusive")]
impl IAppServiceTriggerDetails4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppServiceTriggerDetails4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppServiceTriggerDetails4Vtbl {
        unsafe extern "system" fn CallerRemoteConnectionToken<Impl: IAppServiceTriggerDetails4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CallerRemoteConnectionToken() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAppServiceTriggerDetails4, BASE_OFFSET>(),
            CallerRemoteConnectionToken: CallerRemoteConnectionToken::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppServiceTriggerDetails4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStatelessAppServiceResponseImpl: Sized {
    fn Message(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn Status(&mut self) -> ::windows::core::Result<StatelessAppServiceResponseStatus>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStatelessAppServiceResponse {
    const NAME: &'static str = "Windows.ApplicationModel.AppService.IStatelessAppServiceResponse";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStatelessAppServiceResponseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStatelessAppServiceResponseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStatelessAppServiceResponseVtbl {
        unsafe extern "system" fn Message<Impl: IStatelessAppServiceResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IStatelessAppServiceResponseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut StatelessAppServiceResponseStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStatelessAppServiceResponse, BASE_OFFSET>(),
            Message: Message::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStatelessAppServiceResponse as ::windows::core::Interface>::IID
    }
}
