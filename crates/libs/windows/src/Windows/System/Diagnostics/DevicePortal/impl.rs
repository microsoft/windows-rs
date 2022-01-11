#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDevicePortalConnectionImpl: Sized {
    fn Closed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionClosedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestReceived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionRequestReceivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRequestReceived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDevicePortalConnection {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.IDevicePortalConnection";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDevicePortalConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePortalConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePortalConnectionVtbl {
        unsafe extern "system" fn Closed<Impl: IDevicePortalConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IDevicePortalConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestReceived<Impl: IDevicePortalConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestReceived(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionRequestReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<DevicePortalConnection, DevicePortalConnectionRequestReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRequestReceived<Impl: IDevicePortalConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRequestReceived(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePortalConnection, BASE_OFFSET>(),
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
            RequestReceived: RequestReceived::<Impl, IMPL_OFFSET>,
            RemoveRequestReceived: RemoveRequestReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePortalConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDevicePortalConnectionClosedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<DevicePortalConnectionClosedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDevicePortalConnectionClosedEventArgs {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionClosedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IDevicePortalConnectionClosedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePortalConnectionClosedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePortalConnectionClosedEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IDevicePortalConnectionClosedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DevicePortalConnectionClosedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePortalConnectionClosedEventArgs, BASE_OFFSET>(),
            Reason: Reason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePortalConnectionClosedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IDevicePortalConnectionRequestReceivedEventArgsImpl: Sized {
    fn RequestMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpRequestMessage>;
    fn ResponseMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage>;
}
#[cfg(all(feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDevicePortalConnectionRequestReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionRequestReceivedEventArgs";
}
#[cfg(all(feature = "Web_Http", feature = "implement_exclusive"))]
impl IDevicePortalConnectionRequestReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePortalConnectionRequestReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePortalConnectionRequestReceivedEventArgsVtbl {
        unsafe extern "system" fn RequestMessage<Impl: IDevicePortalConnectionRequestReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResponseMessage<Impl: IDevicePortalConnectionRequestReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResponseMessage() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePortalConnectionRequestReceivedEventArgs, BASE_OFFSET>(),
            RequestMessage: RequestMessage::<Impl, IMPL_OFFSET>,
            ResponseMessage: ResponseMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePortalConnectionRequestReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
pub trait IDevicePortalConnectionStaticsImpl: Sized {
    fn GetForAppServiceConnection(&self, appserviceconnection: &::core::option::Option<super::super::super::ApplicationModel::AppService::AppServiceConnection>) -> ::windows::core::Result<DevicePortalConnection>;
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDevicePortalConnectionStatics {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.IDevicePortalConnectionStatics";
}
#[cfg(all(feature = "ApplicationModel_AppService", feature = "implement_exclusive"))]
impl IDevicePortalConnectionStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePortalConnectionStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePortalConnectionStaticsVtbl {
        unsafe extern "system" fn GetForAppServiceConnection<Impl: IDevicePortalConnectionStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appserviceconnection: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForAppServiceConnection(&*(&appserviceconnection as *const <super::super::super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::Abi>::Abi as *const <super::super::super::ApplicationModel::AppService::AppServiceConnection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePortalConnectionStatics, BASE_OFFSET>(),
            GetForAppServiceConnection: GetForAppServiceConnection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePortalConnectionStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Networking_Sockets", feature = "Web_Http", feature = "implement_exclusive"))]
pub trait IDevicePortalWebSocketConnectionImpl: Sized {
    fn GetServerMessageWebSocketForRequest(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket>;
    fn GetServerMessageWebSocketForRequest2(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket>;
    fn GetServerMessageWebSocketForRequest3(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: &::windows::core::HSTRING, outboundbuffersizeinbytes: u32, maxmessagesize: u32, receivemode: super::super::super::Networking::Sockets::MessageWebSocketReceiveMode) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerMessageWebSocket>;
    fn GetServerStreamWebSocketForRequest(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket>;
    fn GetServerStreamWebSocketForRequest2(&self, request: &::core::option::Option<super::super::super::Web::Http::HttpRequestMessage>, protocol: &::windows::core::HSTRING, outboundbuffersizeinbytes: u32, nodelay: bool) -> ::windows::core::Result<super::super::super::Networking::Sockets::ServerStreamWebSocket>;
}
#[cfg(all(feature = "Networking_Sockets", feature = "Web_Http", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDevicePortalWebSocketConnection {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnection";
}
#[cfg(all(feature = "Networking_Sockets", feature = "Web_Http", feature = "implement_exclusive"))]
impl IDevicePortalWebSocketConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePortalWebSocketConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePortalWebSocketConnectionVtbl {
        unsafe extern "system" fn GetServerMessageWebSocketForRequest<Impl: IDevicePortalWebSocketConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServerMessageWebSocketForRequest(&*(&request as *const <super::super::super::Web::Http::HttpRequestMessage as ::windows::core::Abi>::Abi as *const <super::super::super::Web::Http::HttpRequestMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServerMessageWebSocketForRequest2<Impl: IDevicePortalWebSocketConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServerMessageWebSocketForRequest2(&*(&request as *const <super::super::super::Web::Http::HttpRequestMessage as ::windows::core::Abi>::Abi as *const <super::super::super::Web::Http::HttpRequestMessage as ::windows::core::DefaultType>::DefaultType), messagetype, &*(&protocol as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServerMessageWebSocketForRequest3<Impl: IDevicePortalWebSocketConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, messagetype: super::super::super::Networking::Sockets::SocketMessageType, protocol: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, outboundbuffersizeinbytes: u32, maxmessagesize: u32, receivemode: super::super::super::Networking::Sockets::MessageWebSocketReceiveMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServerMessageWebSocketForRequest3(&*(&request as *const <super::super::super::Web::Http::HttpRequestMessage as ::windows::core::Abi>::Abi as *const <super::super::super::Web::Http::HttpRequestMessage as ::windows::core::DefaultType>::DefaultType), messagetype, &*(&protocol as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), outboundbuffersizeinbytes, maxmessagesize, receivemode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServerStreamWebSocketForRequest<Impl: IDevicePortalWebSocketConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServerStreamWebSocketForRequest(&*(&request as *const <super::super::super::Web::Http::HttpRequestMessage as ::windows::core::Abi>::Abi as *const <super::super::super::Web::Http::HttpRequestMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServerStreamWebSocketForRequest2<Impl: IDevicePortalWebSocketConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, request: ::windows::core::RawPtr, protocol: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, outboundbuffersizeinbytes: u32, nodelay: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServerStreamWebSocketForRequest2(&*(&request as *const <super::super::super::Web::Http::HttpRequestMessage as ::windows::core::Abi>::Abi as *const <super::super::super::Web::Http::HttpRequestMessage as ::windows::core::DefaultType>::DefaultType), &*(&protocol as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), outboundbuffersizeinbytes, nodelay) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePortalWebSocketConnection, BASE_OFFSET>(),
            GetServerMessageWebSocketForRequest: GetServerMessageWebSocketForRequest::<Impl, IMPL_OFFSET>,
            GetServerMessageWebSocketForRequest2: GetServerMessageWebSocketForRequest2::<Impl, IMPL_OFFSET>,
            GetServerMessageWebSocketForRequest3: GetServerMessageWebSocketForRequest3::<Impl, IMPL_OFFSET>,
            GetServerStreamWebSocketForRequest: GetServerStreamWebSocketForRequest::<Impl, IMPL_OFFSET>,
            GetServerStreamWebSocketForRequest2: GetServerStreamWebSocketForRequest2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePortalWebSocketConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDevicePortalWebSocketConnectionRequestReceivedEventArgsImpl: Sized {
    fn IsWebSocketUpgradeRequest(&self) -> ::windows::core::Result<bool>;
    fn WebSocketProtocolsRequested(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDevicePortalWebSocketConnectionRequestReceivedEventArgs {
    const NAME: &'static str = "Windows.System.Diagnostics.DevicePortal.IDevicePortalWebSocketConnectionRequestReceivedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDevicePortalWebSocketConnectionRequestReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDevicePortalWebSocketConnectionRequestReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDevicePortalWebSocketConnectionRequestReceivedEventArgsVtbl {
        unsafe extern "system" fn IsWebSocketUpgradeRequest<Impl: IDevicePortalWebSocketConnectionRequestReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWebSocketUpgradeRequest() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebSocketProtocolsRequested<Impl: IDevicePortalWebSocketConnectionRequestReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebSocketProtocolsRequested() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IDevicePortalWebSocketConnectionRequestReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDevicePortalWebSocketConnectionRequestReceivedEventArgs, BASE_OFFSET>(),
            IsWebSocketUpgradeRequest: IsWebSocketUpgradeRequest::<Impl, IMPL_OFFSET>,
            WebSocketProtocolsRequested: WebSocketProtocolsRequested::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDevicePortalWebSocketConnectionRequestReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
