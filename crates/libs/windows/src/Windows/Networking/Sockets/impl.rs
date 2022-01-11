#[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IControlChannelTriggerImpl: Sized + IClosableImpl {
    fn ControlChannelTriggerId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServerKeepAliveIntervalInMinutes(&self) -> ::windows::core::Result<u32>;
    fn SetServerKeepAliveIntervalInMinutes(&self, value: u32) -> ::windows::core::Result<()>;
    fn CurrentKeepAliveIntervalInMinutes(&self) -> ::windows::core::Result<u32>;
    fn TransportObject(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn KeepAliveTrigger(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger>;
    fn PushNotificationTrigger(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger>;
    fn UsingTransport(&self, transport: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn WaitForPushEnabled(&self) -> ::windows::core::Result<ControlChannelTriggerStatus>;
    fn DecreaseNetworkKeepAliveInterval(&self) -> ::windows::core::Result<()>;
    fn FlushTransport(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IControlChannelTrigger {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTrigger";
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation", feature = "implement_exclusive"))]
impl IControlChannelTriggerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTriggerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlChannelTriggerVtbl {
        unsafe extern "system" fn ControlChannelTriggerId<Impl: IControlChannelTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlChannelTriggerId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerKeepAliveIntervalInMinutes<Impl: IControlChannelTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerKeepAliveIntervalInMinutes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerKeepAliveIntervalInMinutes<Impl: IControlChannelTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerKeepAliveIntervalInMinutes(value).into()
        }
        unsafe extern "system" fn CurrentKeepAliveIntervalInMinutes<Impl: IControlChannelTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentKeepAliveIntervalInMinutes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransportObject<Impl: IControlChannelTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportObject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeepAliveTrigger<Impl: IControlChannelTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeepAliveTrigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushNotificationTrigger<Impl: IControlChannelTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushNotificationTrigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsingTransport<Impl: IControlChannelTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UsingTransport(&*(&transport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WaitForPushEnabled<Impl: IControlChannelTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ControlChannelTriggerStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForPushEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DecreaseNetworkKeepAliveInterval<Impl: IControlChannelTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecreaseNetworkKeepAliveInterval().into()
        }
        unsafe extern "system" fn FlushTransport<Impl: IControlChannelTriggerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushTransport().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IControlChannelTrigger>,
            ::windows::core::GetTrustLevel,
            ControlChannelTriggerId::<Impl, IMPL_OFFSET>,
            ServerKeepAliveIntervalInMinutes::<Impl, IMPL_OFFSET>,
            SetServerKeepAliveIntervalInMinutes::<Impl, IMPL_OFFSET>,
            CurrentKeepAliveIntervalInMinutes::<Impl, IMPL_OFFSET>,
            TransportObject::<Impl, IMPL_OFFSET>,
            KeepAliveTrigger::<Impl, IMPL_OFFSET>,
            PushNotificationTrigger::<Impl, IMPL_OFFSET>,
            UsingTransport::<Impl, IMPL_OFFSET>,
            WaitForPushEnabled::<Impl, IMPL_OFFSET>,
            DecreaseNetworkKeepAliveInterval::<Impl, IMPL_OFFSET>,
            FlushTransport::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChannelTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlChannelTrigger2Impl: Sized {
    fn IsWakeFromLowPowerSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IControlChannelTrigger2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTrigger2";
}
#[cfg(feature = "implement_exclusive")]
impl IControlChannelTrigger2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTrigger2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlChannelTrigger2Vtbl {
        unsafe extern "system" fn IsWakeFromLowPowerSupported<Impl: IControlChannelTrigger2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWakeFromLowPowerSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IControlChannelTrigger2>, ::windows::core::GetTrustLevel, IsWakeFromLowPowerSupported::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChannelTrigger2 as ::windows::core::Interface>::IID
    }
}
pub trait IControlChannelTriggerEventDetailsImpl: Sized {
    fn ControlChannelTrigger(&self) -> ::windows::core::Result<ControlChannelTrigger>;
}
impl ::windows::core::RuntimeName for IControlChannelTriggerEventDetails {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTriggerEventDetails";
}
impl IControlChannelTriggerEventDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTriggerEventDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlChannelTriggerEventDetailsVtbl {
        unsafe extern "system" fn ControlChannelTrigger<Impl: IControlChannelTriggerEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ControlChannelTrigger() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IControlChannelTriggerEventDetails>, ::windows::core::GetTrustLevel, ControlChannelTrigger::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChannelTriggerEventDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlChannelTriggerFactoryImpl: Sized {
    fn CreateControlChannelTrigger(&self, channelid: &::windows::core::HSTRING, serverkeepaliveintervalinminutes: u32) -> ::windows::core::Result<ControlChannelTrigger>;
    fn CreateControlChannelTriggerEx(&self, channelid: &::windows::core::HSTRING, serverkeepaliveintervalinminutes: u32, resourcerequesttype: ControlChannelTriggerResourceType) -> ::windows::core::Result<ControlChannelTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IControlChannelTriggerFactory {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IControlChannelTriggerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTriggerFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlChannelTriggerFactoryVtbl {
        unsafe extern "system" fn CreateControlChannelTrigger<Impl: IControlChannelTriggerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serverkeepaliveintervalinminutes: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateControlChannelTrigger(&*(&channelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), serverkeepaliveintervalinminutes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateControlChannelTriggerEx<Impl: IControlChannelTriggerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serverkeepaliveintervalinminutes: u32, resourcerequesttype: ControlChannelTriggerResourceType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateControlChannelTriggerEx(&*(&channelid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), serverkeepaliveintervalinminutes, resourcerequesttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IControlChannelTriggerFactory>, ::windows::core::GetTrustLevel, CreateControlChannelTrigger::<Impl, IMPL_OFFSET>, CreateControlChannelTriggerEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChannelTriggerFactory as ::windows::core::Interface>::IID
    }
}
pub trait IControlChannelTriggerResetEventDetailsImpl: Sized {
    fn ResetReason(&self) -> ::windows::core::Result<ControlChannelTriggerResetReason>;
    fn HardwareSlotReset(&self) -> ::windows::core::Result<bool>;
    fn SoftwareSlotReset(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IControlChannelTriggerResetEventDetails {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTriggerResetEventDetails";
}
impl IControlChannelTriggerResetEventDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTriggerResetEventDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlChannelTriggerResetEventDetailsVtbl {
        unsafe extern "system" fn ResetReason<Impl: IControlChannelTriggerResetEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ControlChannelTriggerResetReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareSlotReset<Impl: IControlChannelTriggerResetEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HardwareSlotReset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoftwareSlotReset<Impl: IControlChannelTriggerResetEventDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SoftwareSlotReset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IControlChannelTriggerResetEventDetails>, ::windows::core::GetTrustLevel, ResetReason::<Impl, IMPL_OFFSET>, HardwareSlotReset::<Impl, IMPL_OFFSET>, SoftwareSlotReset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChannelTriggerResetEventDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDatagramSocketImpl: Sized + IClosableImpl {
    fn Control(&self) -> ::windows::core::Result<DatagramSocketControl>;
    fn Information(&self) -> ::windows::core::Result<DatagramSocketInformation>;
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn ConnectAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectWithEndpointPairAsync(&self, endpointpair: &::core::option::Option<super::EndpointPair>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BindServiceNameAsync(&self, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BindEndpointAsync(&self, localhostname: &::core::option::Option<super::HostName>, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn JoinMulticastGroup(&self, host: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn GetOutputStreamAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>>;
    fn GetOutputStreamWithEndpointPairAsync(&self, endpointpair: &::core::option::Option<super::EndpointPair>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>>;
    fn MessageReceived(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DatagramSocket, DatagramSocketMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDatagramSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDatagramSocketVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocketImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocketVtbl {
        unsafe extern "system" fn Control<Impl: IDatagramSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Control() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Information<Impl: IDatagramSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Information() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputStream<Impl: IDatagramSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectAsync<Impl: IDatagramSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectAsync(&*(&remotehostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), &*(&remoteservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectWithEndpointPairAsync<Impl: IDatagramSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpointpair: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectWithEndpointPairAsync(&*(&endpointpair as *const <super::EndpointPair as ::windows::core::Abi>::Abi as *const <super::EndpointPair as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindServiceNameAsync<Impl: IDatagramSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindServiceNameAsync(&*(&localservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindEndpointAsync<Impl: IDatagramSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localhostname: ::windows::core::RawPtr, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindEndpointAsync(&*(&localhostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), &*(&localservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JoinMulticastGroup<Impl: IDatagramSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, host: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).JoinMulticastGroup(&*(&host as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetOutputStreamAsync<Impl: IDatagramSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputStreamAsync(&*(&remotehostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), &*(&remoteservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamWithEndpointPairAsync<Impl: IDatagramSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpointpair: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputStreamWithEndpointPairAsync(&*(&endpointpair as *const <super::EndpointPair as ::windows::core::Abi>::Abi as *const <super::EndpointPair as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageReceived<Impl: IDatagramSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageReceived(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<DatagramSocket, DatagramSocketMessageReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<DatagramSocket, DatagramSocketMessageReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMessageReceived<Impl: IDatagramSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceived(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDatagramSocket>,
            ::windows::core::GetTrustLevel,
            Control::<Impl, IMPL_OFFSET>,
            Information::<Impl, IMPL_OFFSET>,
            OutputStream::<Impl, IMPL_OFFSET>,
            ConnectAsync::<Impl, IMPL_OFFSET>,
            ConnectWithEndpointPairAsync::<Impl, IMPL_OFFSET>,
            BindServiceNameAsync::<Impl, IMPL_OFFSET>,
            BindEndpointAsync::<Impl, IMPL_OFFSET>,
            JoinMulticastGroup::<Impl, IMPL_OFFSET>,
            GetOutputStreamAsync::<Impl, IMPL_OFFSET>,
            GetOutputStreamWithEndpointPairAsync::<Impl, IMPL_OFFSET>,
            MessageReceived::<Impl, IMPL_OFFSET>,
            RemoveMessageReceived::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IDatagramSocket2Impl: Sized + IClosableImpl {
    fn BindServiceNameAndAdapterAsync(&self, localservicename: &::windows::core::HSTRING, adapter: &::core::option::Option<super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDatagramSocket2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocket2";
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IDatagramSocket2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocket2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocket2Vtbl {
        unsafe extern "system" fn BindServiceNameAndAdapterAsync<Impl: IDatagramSocket2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindServiceNameAndAdapterAsync(&*(&localservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&adapter as *const <super::Connectivity::NetworkAdapter as ::windows::core::Abi>::Abi as *const <super::Connectivity::NetworkAdapter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDatagramSocket2>, ::windows::core::GetTrustLevel, BindServiceNameAndAdapterAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocket2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDatagramSocket3Impl: Sized {
    fn CancelIOAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn EnableTransferOwnership(&self, taskid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskid: &::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()>;
    fn TransferOwnership(&self, socketid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContext(&self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContextAndKeepAliveTime(&self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>, keepalivetime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDatagramSocket3 {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocket3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDatagramSocket3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocket3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocket3Vtbl {
        unsafe extern "system" fn CancelIOAsync<Impl: IDatagramSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelIOAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableTransferOwnership<Impl: IDatagramSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableTransferOwnership(&*(&taskid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableTransferOwnershipWithConnectedStandbyAction<Impl: IDatagramSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableTransferOwnershipWithConnectedStandbyAction(&*(&taskid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), connectedstandbyaction).into()
        }
        unsafe extern "system" fn TransferOwnership<Impl: IDatagramSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferOwnership(&*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferOwnershipWithContext<Impl: IDatagramSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferOwnershipWithContext(&*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <SocketActivityContext as ::windows::core::Abi>::Abi as *const <SocketActivityContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferOwnershipWithContextAndKeepAliveTime<Impl: IDatagramSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, keepalivetime: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .TransferOwnershipWithContextAndKeepAliveTime(
                    &*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&data as *const <SocketActivityContext as ::windows::core::Abi>::Abi as *const <SocketActivityContext as ::windows::core::DefaultType>::DefaultType),
                    &*(&keepalivetime as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDatagramSocket3>,
            ::windows::core::GetTrustLevel,
            CancelIOAsync::<Impl, IMPL_OFFSET>,
            EnableTransferOwnership::<Impl, IMPL_OFFSET>,
            EnableTransferOwnershipWithConnectedStandbyAction::<Impl, IMPL_OFFSET>,
            TransferOwnership::<Impl, IMPL_OFFSET>,
            TransferOwnershipWithContext::<Impl, IMPL_OFFSET>,
            TransferOwnershipWithContextAndKeepAliveTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocket3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketControlImpl: Sized {
    fn QualityOfService(&self) -> ::windows::core::Result<SocketQualityOfService>;
    fn SetQualityOfService(&self, value: SocketQualityOfService) -> ::windows::core::Result<()>;
    fn OutboundUnicastHopLimit(&self) -> ::windows::core::Result<u8>;
    fn SetOutboundUnicastHopLimit(&self, value: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatagramSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocketControl";
}
#[cfg(feature = "implement_exclusive")]
impl IDatagramSocketControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocketControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocketControlVtbl {
        unsafe extern "system" fn QualityOfService<Impl: IDatagramSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketQualityOfService) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QualityOfService() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQualityOfService<Impl: IDatagramSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocketQualityOfService) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQualityOfService(value).into()
        }
        unsafe extern "system" fn OutboundUnicastHopLimit<Impl: IDatagramSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutboundUnicastHopLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundUnicastHopLimit<Impl: IDatagramSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundUnicastHopLimit(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDatagramSocketControl>, ::windows::core::GetTrustLevel, QualityOfService::<Impl, IMPL_OFFSET>, SetQualityOfService::<Impl, IMPL_OFFSET>, OutboundUnicastHopLimit::<Impl, IMPL_OFFSET>, SetOutboundUnicastHopLimit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocketControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketControl2Impl: Sized {
    fn InboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn SetInboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()>;
    fn DontFragment(&self) -> ::windows::core::Result<bool>;
    fn SetDontFragment(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatagramSocketControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocketControl2";
}
#[cfg(feature = "implement_exclusive")]
impl IDatagramSocketControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocketControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocketControl2Vtbl {
        unsafe extern "system" fn InboundBufferSizeInBytes<Impl: IDatagramSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InboundBufferSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInboundBufferSizeInBytes<Impl: IDatagramSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInboundBufferSizeInBytes(value).into()
        }
        unsafe extern "system" fn DontFragment<Impl: IDatagramSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DontFragment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDontFragment<Impl: IDatagramSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDontFragment(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDatagramSocketControl2>, ::windows::core::GetTrustLevel, InboundBufferSizeInBytes::<Impl, IMPL_OFFSET>, SetInboundBufferSizeInBytes::<Impl, IMPL_OFFSET>, DontFragment::<Impl, IMPL_OFFSET>, SetDontFragment::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocketControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketControl3Impl: Sized {
    fn MulticastOnly(&self) -> ::windows::core::Result<bool>;
    fn SetMulticastOnly(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatagramSocketControl3 {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocketControl3";
}
#[cfg(feature = "implement_exclusive")]
impl IDatagramSocketControl3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocketControl3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocketControl3Vtbl {
        unsafe extern "system" fn MulticastOnly<Impl: IDatagramSocketControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MulticastOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMulticastOnly<Impl: IDatagramSocketControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMulticastOnly(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDatagramSocketControl3>, ::windows::core::GetTrustLevel, MulticastOnly::<Impl, IMPL_OFFSET>, SetMulticastOnly::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocketControl3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketInformationImpl: Sized {
    fn LocalAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatagramSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocketInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IDatagramSocketInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocketInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocketInformationVtbl {
        unsafe extern "system" fn LocalAddress<Impl: IDatagramSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPort<Impl: IDatagramSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteAddress<Impl: IDatagramSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemotePort<Impl: IDatagramSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDatagramSocketInformation>, ::windows::core::GetTrustLevel, LocalAddress::<Impl, IMPL_OFFSET>, LocalPort::<Impl, IMPL_OFFSET>, RemoteAddress::<Impl, IMPL_OFFSET>, RemotePort::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocketInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDatagramSocketMessageReceivedEventArgsImpl: Sized {
    fn RemoteAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn GetDataReader(&self) -> ::windows::core::Result<super::super::Storage::Streams::DataReader>;
    fn GetDataStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDatagramSocketMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocketMessageReceivedEventArgs";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDatagramSocketMessageReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocketMessageReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocketMessageReceivedEventArgsVtbl {
        unsafe extern "system" fn RemoteAddress<Impl: IDatagramSocketMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemotePort<Impl: IDatagramSocketMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAddress<Impl: IDatagramSocketMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataReader<Impl: IDatagramSocketMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataStream<Impl: IDatagramSocketMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDatagramSocketMessageReceivedEventArgs>, ::windows::core::GetTrustLevel, RemoteAddress::<Impl, IMPL_OFFSET>, RemotePort::<Impl, IMPL_OFFSET>, LocalAddress::<Impl, IMPL_OFFSET>, GetDataReader::<Impl, IMPL_OFFSET>, GetDataStream::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocketMessageReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDatagramSocketStaticsImpl: Sized {
    fn GetEndpointPairsAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>;
    fn GetEndpointPairsWithSortOptionsAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING, sortoptions: super::HostNameSortOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDatagramSocketStatics {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocketStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDatagramSocketStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocketStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocketStaticsVtbl {
        unsafe extern "system" fn GetEndpointPairsAsync<Impl: IDatagramSocketStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndpointPairsAsync(&*(&remotehostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), &*(&remoteservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndpointPairsWithSortOptionsAsync<Impl: IDatagramSocketStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sortoptions: super::HostNameSortOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndpointPairsWithSortOptionsAsync(&*(&remotehostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), &*(&remoteservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), sortoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDatagramSocketStatics>, ::windows::core::GetTrustLevel, GetEndpointPairsAsync::<Impl, IMPL_OFFSET>, GetEndpointPairsWithSortOptionsAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocketStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMessageWebSocketImpl: Sized + IClosableImpl + IWebSocketImpl {
    fn Control(&self) -> ::windows::core::Result<MessageWebSocketControl>;
    fn Information(&self) -> ::windows::core::Result<MessageWebSocketInformation>;
    fn MessageReceived(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MessageWebSocket, MessageWebSocketMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMessageWebSocketVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocketImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocketVtbl {
        unsafe extern "system" fn Control<Impl: IMessageWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Control() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Information<Impl: IMessageWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Information() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessageReceived<Impl: IMessageWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageReceived(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<MessageWebSocket, MessageWebSocketMessageReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MessageWebSocket, MessageWebSocketMessageReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMessageReceived<Impl: IMessageWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceived(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMessageWebSocket>, ::windows::core::GetTrustLevel, Control::<Impl, IMPL_OFFSET>, Information::<Impl, IMPL_OFFSET>, MessageReceived::<Impl, IMPL_OFFSET>, RemoveMessageReceived::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMessageWebSocket2Impl: Sized + IClosableImpl + IMessageWebSocketImpl + IWebSocketImpl {
    fn ServerCustomValidationRequested(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MessageWebSocket, WebSocketServerCustomValidationRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerCustomValidationRequested(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocket2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocket2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMessageWebSocket2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocket2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocket2Vtbl {
        unsafe extern "system" fn ServerCustomValidationRequested<Impl: IMessageWebSocket2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCustomValidationRequested(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<MessageWebSocket, WebSocketServerCustomValidationRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<MessageWebSocket, WebSocketServerCustomValidationRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveServerCustomValidationRequested<Impl: IMessageWebSocket2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveServerCustomValidationRequested(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMessageWebSocket2>, ::windows::core::GetTrustLevel, ServerCustomValidationRequested::<Impl, IMPL_OFFSET>, RemoveServerCustomValidationRequested::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocket2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMessageWebSocket3Impl: Sized {
    fn SendNonfinalFrameAsync(&self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn SendFinalFrameAsync(&self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocket3 {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocket3";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMessageWebSocket3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocket3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocket3Vtbl {
        unsafe extern "system" fn SendNonfinalFrameAsync<Impl: IMessageWebSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendNonfinalFrameAsync(&*(&data as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendFinalFrameAsync<Impl: IMessageWebSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendFinalFrameAsync(&*(&data as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMessageWebSocket3>, ::windows::core::GetTrustLevel, SendNonfinalFrameAsync::<Impl, IMPL_OFFSET>, SendFinalFrameAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocket3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IMessageWebSocketControlImpl: Sized + IWebSocketControlImpl {
    fn MaxMessageSize(&self) -> ::windows::core::Result<u32>;
    fn SetMaxMessageSize(&self, value: u32) -> ::windows::core::Result<()>;
    fn MessageType(&self) -> ::windows::core::Result<SocketMessageType>;
    fn SetMessageType(&self, value: SocketMessageType) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocketControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IMessageWebSocketControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocketControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocketControlVtbl {
        unsafe extern "system" fn MaxMessageSize<Impl: IMessageWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxMessageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxMessageSize<Impl: IMessageWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxMessageSize(value).into()
        }
        unsafe extern "system" fn MessageType<Impl: IMessageWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageType<Impl: IMessageWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocketMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageType(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMessageWebSocketControl>, ::windows::core::GetTrustLevel, MaxMessageSize::<Impl, IMPL_OFFSET>, SetMaxMessageSize::<Impl, IMPL_OFFSET>, MessageType::<Impl, IMPL_OFFSET>, SetMessageType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocketControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IMessageWebSocketControl2Impl: Sized {
    fn DesiredUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDesiredUnsolicitedPongInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ActualUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ReceiveMode(&self) -> ::windows::core::Result<MessageWebSocketReceiveMode>;
    fn SetReceiveMode(&self, value: MessageWebSocketReceiveMode) -> ::windows::core::Result<()>;
    fn ClientCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetClientCertificate(&self, value: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocketControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocketControl2";
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IMessageWebSocketControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocketControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocketControl2Vtbl {
        unsafe extern "system" fn DesiredUnsolicitedPongInterval<Impl: IMessageWebSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredUnsolicitedPongInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredUnsolicitedPongInterval<Impl: IMessageWebSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredUnsolicitedPongInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualUnsolicitedPongInterval<Impl: IMessageWebSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualUnsolicitedPongInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReceiveMode<Impl: IMessageWebSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MessageWebSocketReceiveMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReceiveMode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveMode<Impl: IMessageWebSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MessageWebSocketReceiveMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReceiveMode(value).into()
        }
        unsafe extern "system" fn ClientCertificate<Impl: IMessageWebSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientCertificate<Impl: IMessageWebSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCertificate(&*(&value as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMessageWebSocketControl2>,
            ::windows::core::GetTrustLevel,
            DesiredUnsolicitedPongInterval::<Impl, IMPL_OFFSET>,
            SetDesiredUnsolicitedPongInterval::<Impl, IMPL_OFFSET>,
            ActualUnsolicitedPongInterval::<Impl, IMPL_OFFSET>,
            ReceiveMode::<Impl, IMPL_OFFSET>,
            SetReceiveMode::<Impl, IMPL_OFFSET>,
            ClientCertificate::<Impl, IMPL_OFFSET>,
            SetClientCertificate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocketControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMessageWebSocketMessageReceivedEventArgsImpl: Sized {
    fn MessageType(&self) -> ::windows::core::Result<SocketMessageType>;
    fn GetDataReader(&self) -> ::windows::core::Result<super::super::Storage::Streams::DataReader>;
    fn GetDataStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocketMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMessageWebSocketMessageReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocketMessageReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocketMessageReceivedEventArgsVtbl {
        unsafe extern "system" fn MessageType<Impl: IMessageWebSocketMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataReader<Impl: IMessageWebSocketMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataReader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataStream<Impl: IMessageWebSocketMessageReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMessageWebSocketMessageReceivedEventArgs>, ::windows::core::GetTrustLevel, MessageType::<Impl, IMPL_OFFSET>, GetDataReader::<Impl, IMPL_OFFSET>, GetDataStream::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocketMessageReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMessageWebSocketMessageReceivedEventArgs2Impl: Sized + IMessageWebSocketMessageReceivedEventArgsImpl {
    fn IsMessageComplete(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocketMessageReceivedEventArgs2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMessageWebSocketMessageReceivedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocketMessageReceivedEventArgs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocketMessageReceivedEventArgs2Vtbl {
        unsafe extern "system" fn IsMessageComplete<Impl: IMessageWebSocketMessageReceivedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMessageComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMessageWebSocketMessageReceivedEventArgs2>, ::windows::core::GetTrustLevel, IsMessageComplete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocketMessageReceivedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IServerMessageWebSocketImpl: Sized + IClosableImpl {
    fn MessageReceived(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, MessageWebSocketMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Control(&self) -> ::windows::core::Result<ServerMessageWebSocketControl>;
    fn Information(&self) -> ::windows::core::Result<ServerMessageWebSocketInformation>;
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn Closed(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, WebSocketClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CloseWithStatus(&self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IServerMessageWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IServerMessageWebSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IServerMessageWebSocketVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerMessageWebSocketImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerMessageWebSocketVtbl {
        unsafe extern "system" fn MessageReceived<Impl: IServerMessageWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageReceived(&*(&value as *const <super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, MessageWebSocketMessageReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, MessageWebSocketMessageReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMessageReceived<Impl: IServerMessageWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Control<Impl: IServerMessageWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Control() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Information<Impl: IServerMessageWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Information() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputStream<Impl: IServerMessageWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Closed<Impl: IServerMessageWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&value as *const <super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, WebSocketClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, WebSocketClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IServerMessageWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CloseWithStatus<Impl: IServerMessageWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, code: u16, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseWithStatus(code, &*(&reason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IServerMessageWebSocket>,
            ::windows::core::GetTrustLevel,
            MessageReceived::<Impl, IMPL_OFFSET>,
            RemoveMessageReceived::<Impl, IMPL_OFFSET>,
            Control::<Impl, IMPL_OFFSET>,
            Information::<Impl, IMPL_OFFSET>,
            OutputStream::<Impl, IMPL_OFFSET>,
            Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed::<Impl, IMPL_OFFSET>,
            CloseWithStatus::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerMessageWebSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IServerMessageWebSocketControlImpl: Sized {
    fn MessageType(&self) -> ::windows::core::Result<SocketMessageType>;
    fn SetMessageType(&self, value: SocketMessageType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IServerMessageWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IServerMessageWebSocketControl";
}
#[cfg(feature = "implement_exclusive")]
impl IServerMessageWebSocketControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerMessageWebSocketControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerMessageWebSocketControlVtbl {
        unsafe extern "system" fn MessageType<Impl: IServerMessageWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessageType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageType<Impl: IServerMessageWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocketMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageType(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServerMessageWebSocketControl>, ::windows::core::GetTrustLevel, MessageType::<Impl, IMPL_OFFSET>, SetMessageType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerMessageWebSocketControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IServerMessageWebSocketInformationImpl: Sized {
    fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics>;
    fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAddress(&self) -> ::windows::core::Result<super::HostName>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IServerMessageWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IServerMessageWebSocketInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IServerMessageWebSocketInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerMessageWebSocketInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerMessageWebSocketInformationVtbl {
        unsafe extern "system" fn BandwidthStatistics<Impl: IServerMessageWebSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BandwidthStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Impl: IServerMessageWebSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAddress<Impl: IServerMessageWebSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServerMessageWebSocketInformation>, ::windows::core::GetTrustLevel, BandwidthStatistics::<Impl, IMPL_OFFSET>, Protocol::<Impl, IMPL_OFFSET>, LocalAddress::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerMessageWebSocketInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IServerStreamWebSocketImpl: Sized + IClosableImpl {
    fn Information(&self) -> ::windows::core::Result<ServerStreamWebSocketInformation>;
    fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn Closed(&self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ServerStreamWebSocket, WebSocketClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CloseWithStatus(&self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IServerStreamWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IServerStreamWebSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IServerStreamWebSocketVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerStreamWebSocketImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerStreamWebSocketVtbl {
        unsafe extern "system" fn Information<Impl: IServerStreamWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Information() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputStream<Impl: IServerStreamWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputStream<Impl: IServerStreamWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Closed<Impl: IServerStreamWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&value as *const <super::super::Foundation::TypedEventHandler<ServerStreamWebSocket, WebSocketClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<ServerStreamWebSocket, WebSocketClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IServerStreamWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CloseWithStatus<Impl: IServerStreamWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, code: u16, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseWithStatus(code, &*(&reason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IServerStreamWebSocket>,
            ::windows::core::GetTrustLevel,
            Information::<Impl, IMPL_OFFSET>,
            InputStream::<Impl, IMPL_OFFSET>,
            OutputStream::<Impl, IMPL_OFFSET>,
            Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed::<Impl, IMPL_OFFSET>,
            CloseWithStatus::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerStreamWebSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IServerStreamWebSocketInformationImpl: Sized {
    fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics>;
    fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAddress(&self) -> ::windows::core::Result<super::HostName>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IServerStreamWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IServerStreamWebSocketInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IServerStreamWebSocketInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerStreamWebSocketInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerStreamWebSocketInformationVtbl {
        unsafe extern "system" fn BandwidthStatistics<Impl: IServerStreamWebSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BandwidthStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Impl: IServerStreamWebSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAddress<Impl: IServerStreamWebSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServerStreamWebSocketInformation>, ::windows::core::GetTrustLevel, BandwidthStatistics::<Impl, IMPL_OFFSET>, Protocol::<Impl, IMPL_OFFSET>, LocalAddress::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerStreamWebSocketInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISocketActivityContextImpl: Sized {
    fn Data(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocketActivityContext {
    const NAME: &'static str = "Windows.Networking.Sockets.ISocketActivityContext";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISocketActivityContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocketActivityContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocketActivityContextVtbl {
        unsafe extern "system" fn Data<Impl: ISocketActivityContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISocketActivityContext>, ::windows::core::GetTrustLevel, Data::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocketActivityContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISocketActivityContextFactoryImpl: Sized {
    fn Create(&self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<SocketActivityContext>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocketActivityContextFactory {
    const NAME: &'static str = "Windows.Networking.Sockets.ISocketActivityContextFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISocketActivityContextFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocketActivityContextFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocketActivityContextFactoryVtbl {
        unsafe extern "system" fn Create<Impl: ISocketActivityContextFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&data as *const <super::super::Storage::Streams::IBuffer as ::windows::core::Abi>::Abi as *const <super::super::Storage::Streams::IBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISocketActivityContextFactory>, ::windows::core::GetTrustLevel, Create::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocketActivityContextFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketActivityInformationImpl: Sized {
    fn TaskId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SocketKind(&self) -> ::windows::core::Result<SocketActivityKind>;
    fn Context(&self) -> ::windows::core::Result<SocketActivityContext>;
    fn DatagramSocket(&self) -> ::windows::core::Result<DatagramSocket>;
    fn StreamSocket(&self) -> ::windows::core::Result<StreamSocket>;
    fn StreamSocketListener(&self) -> ::windows::core::Result<StreamSocketListener>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISocketActivityInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.ISocketActivityInformation";
}
#[cfg(feature = "implement_exclusive")]
impl ISocketActivityInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocketActivityInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocketActivityInformationVtbl {
        unsafe extern "system" fn TaskId<Impl: ISocketActivityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TaskId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Id<Impl: ISocketActivityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SocketKind<Impl: ISocketActivityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketActivityKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SocketKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Context<Impl: ISocketActivityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Context() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DatagramSocket<Impl: ISocketActivityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DatagramSocket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreamSocket<Impl: ISocketActivityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamSocket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StreamSocketListener<Impl: ISocketActivityInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StreamSocketListener() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISocketActivityInformation>,
            ::windows::core::GetTrustLevel,
            TaskId::<Impl, IMPL_OFFSET>,
            Id::<Impl, IMPL_OFFSET>,
            SocketKind::<Impl, IMPL_OFFSET>,
            Context::<Impl, IMPL_OFFSET>,
            DatagramSocket::<Impl, IMPL_OFFSET>,
            StreamSocket::<Impl, IMPL_OFFSET>,
            StreamSocketListener::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocketActivityInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISocketActivityInformationStaticsImpl: Sized {
    fn AllSockets(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SocketActivityInformation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocketActivityInformationStatics {
    const NAME: &'static str = "Windows.Networking.Sockets.ISocketActivityInformationStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISocketActivityInformationStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocketActivityInformationStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocketActivityInformationStaticsVtbl {
        unsafe extern "system" fn AllSockets<Impl: ISocketActivityInformationStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllSockets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISocketActivityInformationStatics>, ::windows::core::GetTrustLevel, AllSockets::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocketActivityInformationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketActivityTriggerDetailsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<SocketActivityTriggerReason>;
    fn SocketInformation(&self) -> ::windows::core::Result<SocketActivityInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISocketActivityTriggerDetails {
    const NAME: &'static str = "Windows.Networking.Sockets.ISocketActivityTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl ISocketActivityTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocketActivityTriggerDetailsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocketActivityTriggerDetailsVtbl {
        unsafe extern "system" fn Reason<Impl: ISocketActivityTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketActivityTriggerReason) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SocketInformation<Impl: ISocketActivityTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SocketInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISocketActivityTriggerDetails>, ::windows::core::GetTrustLevel, Reason::<Impl, IMPL_OFFSET>, SocketInformation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocketActivityTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketErrorStaticsImpl: Sized {
    fn GetStatus(&self, hresult: i32) -> ::windows::core::Result<SocketErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISocketErrorStatics {
    const NAME: &'static str = "Windows.Networking.Sockets.ISocketErrorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISocketErrorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocketErrorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocketErrorStaticsVtbl {
        unsafe extern "system" fn GetStatus<Impl: ISocketErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut SocketErrorStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(hresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISocketErrorStatics>, ::windows::core::GetTrustLevel, GetStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocketErrorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IStreamSocketImpl: Sized + IClosableImpl {
    fn Control(&self) -> ::windows::core::Result<StreamSocketControl>;
    fn Information(&self) -> ::windows::core::Result<StreamSocketInformation>;
    fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn ConnectWithEndpointPairAsync(&self, endpointpair: &::core::option::Option<super::EndpointPair>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectWithEndpointPairAndProtectionLevelAsync(&self, endpointpair: &::core::option::Option<super::EndpointPair>, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectWithProtectionLevelAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpgradeToSslAsync(&self, protectionlevel: SocketProtectionLevel, validationhostname: &::core::option::Option<super::HostName>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IStreamSocketVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketVtbl {
        unsafe extern "system" fn Control<Impl: IStreamSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Control() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Information<Impl: IStreamSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Information() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputStream<Impl: IStreamSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OutputStream<Impl: IStreamSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectWithEndpointPairAsync<Impl: IStreamSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpointpair: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectWithEndpointPairAsync(&*(&endpointpair as *const <super::EndpointPair as ::windows::core::Abi>::Abi as *const <super::EndpointPair as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectAsync<Impl: IStreamSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectAsync(&*(&remotehostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), &*(&remoteservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectWithEndpointPairAndProtectionLevelAsync<Impl: IStreamSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpointpair: ::windows::core::RawPtr, protectionlevel: SocketProtectionLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectWithEndpointPairAndProtectionLevelAsync(&*(&endpointpair as *const <super::EndpointPair as ::windows::core::Abi>::Abi as *const <super::EndpointPair as ::windows::core::DefaultType>::DefaultType), protectionlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectWithProtectionLevelAsync<Impl: IStreamSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectWithProtectionLevelAsync(&*(&remotehostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), &*(&remoteservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), protectionlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpgradeToSslAsync<Impl: IStreamSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protectionlevel: SocketProtectionLevel, validationhostname: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpgradeToSslAsync(protectionlevel, &*(&validationhostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStreamSocket>,
            ::windows::core::GetTrustLevel,
            Control::<Impl, IMPL_OFFSET>,
            Information::<Impl, IMPL_OFFSET>,
            InputStream::<Impl, IMPL_OFFSET>,
            OutputStream::<Impl, IMPL_OFFSET>,
            ConnectWithEndpointPairAsync::<Impl, IMPL_OFFSET>,
            ConnectAsync::<Impl, IMPL_OFFSET>,
            ConnectWithEndpointPairAndProtectionLevelAsync::<Impl, IMPL_OFFSET>,
            ConnectWithProtectionLevelAsync::<Impl, IMPL_OFFSET>,
            UpgradeToSslAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IStreamSocket2Impl: Sized + IClosableImpl {
    fn ConnectWithProtectionLevelAndAdapterAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel, adapter: &::core::option::Option<super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocket2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocket2";
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IStreamSocket2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocket2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocket2Vtbl {
        unsafe extern "system" fn ConnectWithProtectionLevelAndAdapterAsync<Impl: IStreamSocket2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectWithProtectionLevelAndAdapterAsync(
                &*(&remotehostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType),
                &*(&remoteservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                protectionlevel,
                &*(&adapter as *const <super::Connectivity::NetworkAdapter as ::windows::core::Abi>::Abi as *const <super::Connectivity::NetworkAdapter as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamSocket2>, ::windows::core::GetTrustLevel, ConnectWithProtectionLevelAndAdapterAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocket2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreamSocket3Impl: Sized {
    fn CancelIOAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn EnableTransferOwnership(&self, taskid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskid: &::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()>;
    fn TransferOwnership(&self, socketid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContext(&self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContextAndKeepAliveTime(&self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>, keepalivetime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocket3 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocket3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStreamSocket3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocket3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocket3Vtbl {
        unsafe extern "system" fn CancelIOAsync<Impl: IStreamSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelIOAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableTransferOwnership<Impl: IStreamSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableTransferOwnership(&*(&taskid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableTransferOwnershipWithConnectedStandbyAction<Impl: IStreamSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableTransferOwnershipWithConnectedStandbyAction(&*(&taskid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), connectedstandbyaction).into()
        }
        unsafe extern "system" fn TransferOwnership<Impl: IStreamSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferOwnership(&*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferOwnershipWithContext<Impl: IStreamSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferOwnershipWithContext(&*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <SocketActivityContext as ::windows::core::Abi>::Abi as *const <SocketActivityContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferOwnershipWithContextAndKeepAliveTime<Impl: IStreamSocket3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, keepalivetime: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .TransferOwnershipWithContextAndKeepAliveTime(
                    &*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&data as *const <SocketActivityContext as ::windows::core::Abi>::Abi as *const <SocketActivityContext as ::windows::core::DefaultType>::DefaultType),
                    &*(&keepalivetime as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStreamSocket3>,
            ::windows::core::GetTrustLevel,
            CancelIOAsync::<Impl, IMPL_OFFSET>,
            EnableTransferOwnership::<Impl, IMPL_OFFSET>,
            EnableTransferOwnershipWithConnectedStandbyAction::<Impl, IMPL_OFFSET>,
            TransferOwnership::<Impl, IMPL_OFFSET>,
            TransferOwnershipWithContext::<Impl, IMPL_OFFSET>,
            TransferOwnershipWithContextAndKeepAliveTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocket3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketControlImpl: Sized {
    fn NoDelay(&self) -> ::windows::core::Result<bool>;
    fn SetNoDelay(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeepAlive(&self) -> ::windows::core::Result<bool>;
    fn SetKeepAlive(&self, value: bool) -> ::windows::core::Result<()>;
    fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()>;
    fn QualityOfService(&self) -> ::windows::core::Result<SocketQualityOfService>;
    fn SetQualityOfService(&self, value: SocketQualityOfService) -> ::windows::core::Result<()>;
    fn OutboundUnicastHopLimit(&self) -> ::windows::core::Result<u8>;
    fn SetOutboundUnicastHopLimit(&self, value: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreamSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketControl";
}
#[cfg(feature = "implement_exclusive")]
impl IStreamSocketControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketControlVtbl {
        unsafe extern "system" fn NoDelay<Impl: IStreamSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NoDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoDelay<Impl: IStreamSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoDelay(value).into()
        }
        unsafe extern "system" fn KeepAlive<Impl: IStreamSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeepAlive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepAlive<Impl: IStreamSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeepAlive(value).into()
        }
        unsafe extern "system" fn OutboundBufferSizeInBytes<Impl: IStreamSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutboundBufferSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundBufferSizeInBytes<Impl: IStreamSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundBufferSizeInBytes(value).into()
        }
        unsafe extern "system" fn QualityOfService<Impl: IStreamSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketQualityOfService) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QualityOfService() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQualityOfService<Impl: IStreamSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocketQualityOfService) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQualityOfService(value).into()
        }
        unsafe extern "system" fn OutboundUnicastHopLimit<Impl: IStreamSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutboundUnicastHopLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundUnicastHopLimit<Impl: IStreamSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundUnicastHopLimit(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStreamSocketControl>,
            ::windows::core::GetTrustLevel,
            NoDelay::<Impl, IMPL_OFFSET>,
            SetNoDelay::<Impl, IMPL_OFFSET>,
            KeepAlive::<Impl, IMPL_OFFSET>,
            SetKeepAlive::<Impl, IMPL_OFFSET>,
            OutboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            SetOutboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            QualityOfService::<Impl, IMPL_OFFSET>,
            SetQualityOfService::<Impl, IMPL_OFFSET>,
            OutboundUnicastHopLimit::<Impl, IMPL_OFFSET>,
            SetOutboundUnicastHopLimit::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IStreamSocketControl2Impl: Sized {
    fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketControl2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IStreamSocketControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketControl2Vtbl {
        unsafe extern "system" fn IgnorableServerCertificateErrors<Impl: IStreamSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IgnorableServerCertificateErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamSocketControl2>, ::windows::core::GetTrustLevel, IgnorableServerCertificateErrors::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IStreamSocketControl3Impl: Sized {
    fn SerializeConnectionAttempts(&self) -> ::windows::core::Result<bool>;
    fn SetSerializeConnectionAttempts(&self, value: bool) -> ::windows::core::Result<()>;
    fn ClientCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetClientCertificate(&self, value: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketControl3 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketControl3";
}
#[cfg(all(feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IStreamSocketControl3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketControl3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketControl3Vtbl {
        unsafe extern "system" fn SerializeConnectionAttempts<Impl: IStreamSocketControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SerializeConnectionAttempts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSerializeConnectionAttempts<Impl: IStreamSocketControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSerializeConnectionAttempts(value).into()
        }
        unsafe extern "system" fn ClientCertificate<Impl: IStreamSocketControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientCertificate<Impl: IStreamSocketControl3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCertificate(&*(&value as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamSocketControl3>, ::windows::core::GetTrustLevel, SerializeConnectionAttempts::<Impl, IMPL_OFFSET>, SetSerializeConnectionAttempts::<Impl, IMPL_OFFSET>, ClientCertificate::<Impl, IMPL_OFFSET>, SetClientCertificate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketControl3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketControl4Impl: Sized {
    fn MinProtectionLevel(&self) -> ::windows::core::Result<SocketProtectionLevel>;
    fn SetMinProtectionLevel(&self, value: SocketProtectionLevel) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreamSocketControl4 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketControl4";
}
#[cfg(feature = "implement_exclusive")]
impl IStreamSocketControl4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketControl4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketControl4Vtbl {
        unsafe extern "system" fn MinProtectionLevel<Impl: IStreamSocketControl4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinProtectionLevel<Impl: IStreamSocketControl4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocketProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinProtectionLevel(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamSocketControl4>, ::windows::core::GetTrustLevel, MinProtectionLevel::<Impl, IMPL_OFFSET>, SetMinProtectionLevel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketControl4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IStreamSocketInformationImpl: Sized {
    fn LocalAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteHostName(&self) -> ::windows::core::Result<super::HostName>;
    fn RemoteAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn RemoteServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemotePort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoundTripTimeStatistics(&self) -> ::windows::core::Result<RoundTripTimeStatistics>;
    fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics>;
    fn ProtectionLevel(&self) -> ::windows::core::Result<SocketProtectionLevel>;
    fn SessionKey(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketInformation";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IStreamSocketInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketInformationVtbl {
        unsafe extern "system" fn LocalAddress<Impl: IStreamSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalPort<Impl: IStreamSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteHostName<Impl: IStreamSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteHostName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteAddress<Impl: IStreamSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteServiceName<Impl: IStreamSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemotePort<Impl: IStreamSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoundTripTimeStatistics<Impl: IStreamSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RoundTripTimeStatistics) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoundTripTimeStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BandwidthStatistics<Impl: IStreamSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BandwidthStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtectionLevel<Impl: IStreamSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionKey<Impl: IStreamSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionKey() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStreamSocketInformation>,
            ::windows::core::GetTrustLevel,
            LocalAddress::<Impl, IMPL_OFFSET>,
            LocalPort::<Impl, IMPL_OFFSET>,
            RemoteHostName::<Impl, IMPL_OFFSET>,
            RemoteAddress::<Impl, IMPL_OFFSET>,
            RemoteServiceName::<Impl, IMPL_OFFSET>,
            RemotePort::<Impl, IMPL_OFFSET>,
            RoundTripTimeStatistics::<Impl, IMPL_OFFSET>,
            BandwidthStatistics::<Impl, IMPL_OFFSET>,
            ProtectionLevel::<Impl, IMPL_OFFSET>,
            SessionKey::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IStreamSocketInformation2Impl: Sized {
    fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketInformation2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketInformation2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IStreamSocketInformation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketInformation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketInformation2Vtbl {
        unsafe extern "system" fn ServerCertificateErrorSeverity<Impl: IStreamSocketInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCertificateErrorSeverity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificateErrors<Impl: IStreamSocketInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCertificateErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificate<Impl: IStreamSocketInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerIntermediateCertificates<Impl: IStreamSocketInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerIntermediateCertificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamSocketInformation2>, ::windows::core::GetTrustLevel, ServerCertificateErrorSeverity::<Impl, IMPL_OFFSET>, ServerCertificateErrors::<Impl, IMPL_OFFSET>, ServerCertificate::<Impl, IMPL_OFFSET>, ServerIntermediateCertificates::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreamSocketListenerImpl: Sized + IClosableImpl {
    fn Control(&self) -> ::windows::core::Result<StreamSocketListenerControl>;
    fn Information(&self) -> ::windows::core::Result<StreamSocketListenerInformation>;
    fn BindServiceNameAsync(&self, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BindEndpointAsync(&self, localhostname: &::core::option::Option<super::HostName>, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectionReceived(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StreamSocketListener, StreamSocketListenerConnectionReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionReceived(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketListener {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListener";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStreamSocketListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListenerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListenerVtbl {
        unsafe extern "system" fn Control<Impl: IStreamSocketListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Control() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Information<Impl: IStreamSocketListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Information() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindServiceNameAsync<Impl: IStreamSocketListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindServiceNameAsync(&*(&localservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindEndpointAsync<Impl: IStreamSocketListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localhostname: ::windows::core::RawPtr, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindEndpointAsync(&*(&localhostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), &*(&localservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionReceived<Impl: IStreamSocketListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionReceived(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<StreamSocketListener, StreamSocketListenerConnectionReceivedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<StreamSocketListener, StreamSocketListenerConnectionReceivedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionReceived<Impl: IStreamSocketListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConnectionReceived(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStreamSocketListener>,
            ::windows::core::GetTrustLevel,
            Control::<Impl, IMPL_OFFSET>,
            Information::<Impl, IMPL_OFFSET>,
            BindServiceNameAsync::<Impl, IMPL_OFFSET>,
            BindEndpointAsync::<Impl, IMPL_OFFSET>,
            ConnectionReceived::<Impl, IMPL_OFFSET>,
            RemoveConnectionReceived::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListener as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IStreamSocketListener2Impl: Sized + IClosableImpl {
    fn BindServiceNameWithProtectionLevelAsync(&self, localservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BindServiceNameWithProtectionLevelAndAdapterAsync(&self, localservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel, adapter: &::core::option::Option<super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketListener2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListener2";
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IStreamSocketListener2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListener2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListener2Vtbl {
        unsafe extern "system" fn BindServiceNameWithProtectionLevelAsync<Impl: IStreamSocketListener2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindServiceNameWithProtectionLevelAsync(&*(&localservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), protectionlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindServiceNameWithProtectionLevelAndAdapterAsync<Impl: IStreamSocketListener2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindServiceNameWithProtectionLevelAndAdapterAsync(&*(&localservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), protectionlevel, &*(&adapter as *const <super::Connectivity::NetworkAdapter as ::windows::core::Abi>::Abi as *const <super::Connectivity::NetworkAdapter as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamSocketListener2>, ::windows::core::GetTrustLevel, BindServiceNameWithProtectionLevelAsync::<Impl, IMPL_OFFSET>, BindServiceNameWithProtectionLevelAndAdapterAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListener2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreamSocketListener3Impl: Sized {
    fn CancelIOAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn EnableTransferOwnership(&self, taskid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnableTransferOwnershipWithConnectedStandbyAction(&self, taskid: &::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()>;
    fn TransferOwnership(&self, socketid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContext(&self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketListener3 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListener3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStreamSocketListener3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListener3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListener3Vtbl {
        unsafe extern "system" fn CancelIOAsync<Impl: IStreamSocketListener3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelIOAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableTransferOwnership<Impl: IStreamSocketListener3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableTransferOwnership(&*(&taskid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableTransferOwnershipWithConnectedStandbyAction<Impl: IStreamSocketListener3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableTransferOwnershipWithConnectedStandbyAction(&*(&taskid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), connectedstandbyaction).into()
        }
        unsafe extern "system" fn TransferOwnership<Impl: IStreamSocketListener3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferOwnership(&*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferOwnershipWithContext<Impl: IStreamSocketListener3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferOwnershipWithContext(&*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <SocketActivityContext as ::windows::core::Abi>::Abi as *const <SocketActivityContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStreamSocketListener3>,
            ::windows::core::GetTrustLevel,
            CancelIOAsync::<Impl, IMPL_OFFSET>,
            EnableTransferOwnership::<Impl, IMPL_OFFSET>,
            EnableTransferOwnershipWithConnectedStandbyAction::<Impl, IMPL_OFFSET>,
            TransferOwnership::<Impl, IMPL_OFFSET>,
            TransferOwnershipWithContext::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListener3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketListenerConnectionReceivedEventArgsImpl: Sized {
    fn Socket(&self) -> ::windows::core::Result<StreamSocket>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreamSocketListenerConnectionReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListenerConnectionReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IStreamSocketListenerConnectionReceivedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListenerConnectionReceivedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListenerConnectionReceivedEventArgsVtbl {
        unsafe extern "system" fn Socket<Impl: IStreamSocketListenerConnectionReceivedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Socket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamSocketListenerConnectionReceivedEventArgs>, ::windows::core::GetTrustLevel, Socket::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListenerConnectionReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketListenerControlImpl: Sized {
    fn QualityOfService(&self) -> ::windows::core::Result<SocketQualityOfService>;
    fn SetQualityOfService(&self, value: SocketQualityOfService) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreamSocketListenerControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListenerControl";
}
#[cfg(feature = "implement_exclusive")]
impl IStreamSocketListenerControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListenerControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListenerControlVtbl {
        unsafe extern "system" fn QualityOfService<Impl: IStreamSocketListenerControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketQualityOfService) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QualityOfService() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQualityOfService<Impl: IStreamSocketListenerControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocketQualityOfService) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQualityOfService(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamSocketListenerControl>, ::windows::core::GetTrustLevel, QualityOfService::<Impl, IMPL_OFFSET>, SetQualityOfService::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListenerControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketListenerControl2Impl: Sized {
    fn NoDelay(&self) -> ::windows::core::Result<bool>;
    fn SetNoDelay(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeepAlive(&self) -> ::windows::core::Result<bool>;
    fn SetKeepAlive(&self, value: bool) -> ::windows::core::Result<()>;
    fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()>;
    fn OutboundUnicastHopLimit(&self) -> ::windows::core::Result<u8>;
    fn SetOutboundUnicastHopLimit(&self, value: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreamSocketListenerControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListenerControl2";
}
#[cfg(feature = "implement_exclusive")]
impl IStreamSocketListenerControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListenerControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListenerControl2Vtbl {
        unsafe extern "system" fn NoDelay<Impl: IStreamSocketListenerControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NoDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoDelay<Impl: IStreamSocketListenerControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoDelay(value).into()
        }
        unsafe extern "system" fn KeepAlive<Impl: IStreamSocketListenerControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeepAlive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeepAlive<Impl: IStreamSocketListenerControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeepAlive(value).into()
        }
        unsafe extern "system" fn OutboundBufferSizeInBytes<Impl: IStreamSocketListenerControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutboundBufferSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundBufferSizeInBytes<Impl: IStreamSocketListenerControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundBufferSizeInBytes(value).into()
        }
        unsafe extern "system" fn OutboundUnicastHopLimit<Impl: IStreamSocketListenerControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutboundUnicastHopLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundUnicastHopLimit<Impl: IStreamSocketListenerControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundUnicastHopLimit(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStreamSocketListenerControl2>,
            ::windows::core::GetTrustLevel,
            NoDelay::<Impl, IMPL_OFFSET>,
            SetNoDelay::<Impl, IMPL_OFFSET>,
            KeepAlive::<Impl, IMPL_OFFSET>,
            SetKeepAlive::<Impl, IMPL_OFFSET>,
            OutboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            SetOutboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            OutboundUnicastHopLimit::<Impl, IMPL_OFFSET>,
            SetOutboundUnicastHopLimit::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListenerControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketListenerInformationImpl: Sized {
    fn LocalPort(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreamSocketListenerInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListenerInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IStreamSocketListenerInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListenerInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListenerInformationVtbl {
        unsafe extern "system" fn LocalPort<Impl: IStreamSocketListenerInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamSocketListenerInformation>, ::windows::core::GetTrustLevel, LocalPort::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListenerInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStreamSocketStaticsImpl: Sized {
    fn GetEndpointPairsAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>;
    fn GetEndpointPairsWithSortOptionsAsync(&self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING, sortoptions: super::HostNameSortOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketStatics {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStreamSocketStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketStaticsVtbl {
        unsafe extern "system" fn GetEndpointPairsAsync<Impl: IStreamSocketStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndpointPairsAsync(&*(&remotehostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), &*(&remoteservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndpointPairsWithSortOptionsAsync<Impl: IStreamSocketStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sortoptions: super::HostNameSortOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndpointPairsWithSortOptionsAsync(&*(&remotehostname as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), &*(&remoteservicename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), sortoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamSocketStatics>, ::windows::core::GetTrustLevel, GetEndpointPairsAsync::<Impl, IMPL_OFFSET>, GetEndpointPairsWithSortOptionsAsync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IStreamWebSocketImpl: Sized + IClosableImpl + IWebSocketImpl {
    fn Control(&self) -> ::windows::core::Result<StreamWebSocketControl>;
    fn Information(&self) -> ::windows::core::Result<StreamWebSocketInformation>;
    fn InputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamWebSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IStreamWebSocketVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamWebSocketImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamWebSocketVtbl {
        unsafe extern "system" fn Control<Impl: IStreamWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Control() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Information<Impl: IStreamWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Information() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InputStream<Impl: IStreamWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamWebSocket>, ::windows::core::GetTrustLevel, Control::<Impl, IMPL_OFFSET>, Information::<Impl, IMPL_OFFSET>, InputStream::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamWebSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IStreamWebSocket2Impl: Sized + IClosableImpl + IStreamWebSocketImpl + IWebSocketImpl {
    fn ServerCustomValidationRequested(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StreamWebSocket, WebSocketServerCustomValidationRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerCustomValidationRequested(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamWebSocket2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamWebSocket2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IStreamWebSocket2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamWebSocket2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamWebSocket2Vtbl {
        unsafe extern "system" fn ServerCustomValidationRequested<Impl: IStreamWebSocket2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCustomValidationRequested(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<StreamWebSocket, WebSocketServerCustomValidationRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<StreamWebSocket, WebSocketServerCustomValidationRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveServerCustomValidationRequested<Impl: IStreamWebSocket2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveServerCustomValidationRequested(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamWebSocket2>, ::windows::core::GetTrustLevel, ServerCustomValidationRequested::<Impl, IMPL_OFFSET>, RemoveServerCustomValidationRequested::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamWebSocket2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IStreamWebSocketControlImpl: Sized + IWebSocketControlImpl {
    fn NoDelay(&self) -> ::windows::core::Result<bool>;
    fn SetNoDelay(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamWebSocketControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IStreamWebSocketControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamWebSocketControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamWebSocketControlVtbl {
        unsafe extern "system" fn NoDelay<Impl: IStreamWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NoDelay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoDelay<Impl: IStreamWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoDelay(value).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamWebSocketControl>, ::windows::core::GetTrustLevel, NoDelay::<Impl, IMPL_OFFSET>, SetNoDelay::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamWebSocketControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IStreamWebSocketControl2Impl: Sized {
    fn DesiredUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDesiredUnsolicitedPongInterval(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ActualUnsolicitedPongInterval(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ClientCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetClientCertificate(&self, value: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamWebSocketControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamWebSocketControl2";
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IStreamWebSocketControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamWebSocketControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamWebSocketControl2Vtbl {
        unsafe extern "system" fn DesiredUnsolicitedPongInterval<Impl: IStreamWebSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DesiredUnsolicitedPongInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredUnsolicitedPongInterval<Impl: IStreamWebSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredUnsolicitedPongInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualUnsolicitedPongInterval<Impl: IStreamWebSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActualUnsolicitedPongInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientCertificate<Impl: IStreamWebSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientCertificate<Impl: IStreamWebSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCertificate(&*(&value as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IStreamWebSocketControl2>,
            ::windows::core::GetTrustLevel,
            DesiredUnsolicitedPongInterval::<Impl, IMPL_OFFSET>,
            SetDesiredUnsolicitedPongInterval::<Impl, IMPL_OFFSET>,
            ActualUnsolicitedPongInterval::<Impl, IMPL_OFFSET>,
            ClientCertificate::<Impl, IMPL_OFFSET>,
            SetClientCertificate::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamWebSocketControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IWebSocketImpl: Sized + IClosableImpl {
    fn OutputStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn ConnectAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetRequestHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Closed(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CloseWithStatus(&self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IWebSocketVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketVtbl {
        unsafe extern "system" fn OutputStream<Impl: IWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectAsync<Impl: IWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectAsync(&*(&uri as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestHeader<Impl: IWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&headervalue as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&eventhandler as *const <super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CloseWithStatus<Impl: IWebSocketImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, code: u16, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseWithStatus(code, &*(&reason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebSocket>, ::windows::core::GetTrustLevel, OutputStream::<Impl, IMPL_OFFSET>, ConnectAsync::<Impl, IMPL_OFFSET>, SetRequestHeader::<Impl, IMPL_OFFSET>, Closed::<Impl, IMPL_OFFSET>, RemoveClosed::<Impl, IMPL_OFFSET>, CloseWithStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebSocketClosedEventArgsImpl: Sized {
    fn Code(&self) -> ::windows::core::Result<u16>;
    fn Reason(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebSocketClosedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketClosedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWebSocketClosedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketClosedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketClosedEventArgsVtbl {
        unsafe extern "system" fn Code<Impl: IWebSocketClosedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Code() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reason<Impl: IWebSocketClosedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebSocketClosedEventArgs>, ::windows::core::GetTrustLevel, Code::<Impl, IMPL_OFFSET>, Reason::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketClosedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
pub trait IWebSocketControlImpl: Sized {
    fn OutboundBufferSizeInBytes(&self) -> ::windows::core::Result<u32>;
    fn SetOutboundBufferSizeInBytes(&self, value: u32) -> ::windows::core::Result<()>;
    fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn SupportedProtocols(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
impl ::windows::core::RuntimeName for IWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
impl IWebSocketControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketControlVtbl {
        unsafe extern "system" fn OutboundBufferSizeInBytes<Impl: IWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OutboundBufferSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundBufferSizeInBytes<Impl: IWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundBufferSizeInBytes(value).into()
        }
        unsafe extern "system" fn ServerCredential<Impl: IWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerCredential<Impl: IWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerCredential(&*(&value as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyCredential<Impl: IWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyCredential<Impl: IWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyCredential(&*(&value as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SupportedProtocols<Impl: IWebSocketControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedProtocols() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWebSocketControl>,
            ::windows::core::GetTrustLevel,
            OutboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            SetOutboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            ServerCredential::<Impl, IMPL_OFFSET>,
            SetServerCredential::<Impl, IMPL_OFFSET>,
            ProxyCredential::<Impl, IMPL_OFFSET>,
            SetProxyCredential::<Impl, IMPL_OFFSET>,
            SupportedProtocols::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
pub trait IWebSocketControl2Impl: Sized + IWebSocketControlImpl {
    fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl ::windows::core::RuntimeName for IWebSocketControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketControl2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl IWebSocketControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControl2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketControl2Vtbl {
        unsafe extern "system" fn IgnorableServerCertificateErrors<Impl: IWebSocketControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IgnorableServerCertificateErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebSocketControl2>, ::windows::core::GetTrustLevel, IgnorableServerCertificateErrors::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Web", feature = "implement_exclusive"))]
pub trait IWebSocketErrorStaticsImpl: Sized {
    fn GetStatus(&self, hresult: i32) -> ::windows::core::Result<super::super::Web::WebErrorStatus>;
}
#[cfg(all(feature = "Web", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebSocketErrorStatics {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketErrorStatics";
}
#[cfg(all(feature = "Web", feature = "implement_exclusive"))]
impl IWebSocketErrorStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketErrorStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketErrorStaticsVtbl {
        unsafe extern "system" fn GetStatus<Impl: IWebSocketErrorStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut super::super::Web::WebErrorStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus(hresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebSocketErrorStatics>, ::windows::core::GetTrustLevel, GetStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketErrorStatics as ::windows::core::Interface>::IID
    }
}
pub trait IWebSocketInformationImpl: Sized {
    fn LocalAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics>;
    fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketInformation";
}
impl IWebSocketInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketInformationVtbl {
        unsafe extern "system" fn LocalAddress<Impl: IWebSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BandwidthStatistics<Impl: IWebSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BandwidthStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Impl: IWebSocketInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebSocketInformation>, ::windows::core::GetTrustLevel, LocalAddress::<Impl, IMPL_OFFSET>, BandwidthStatistics::<Impl, IMPL_OFFSET>, Protocol::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
pub trait IWebSocketInformation2Impl: Sized + IWebSocketInformationImpl {
    fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
impl ::windows::core::RuntimeName for IWebSocketInformation2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketInformation2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
impl IWebSocketInformation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketInformation2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketInformation2Vtbl {
        unsafe extern "system" fn ServerCertificate<Impl: IWebSocketInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificateErrorSeverity<Impl: IWebSocketInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCertificateErrorSeverity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificateErrors<Impl: IWebSocketInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCertificateErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerIntermediateCertificates<Impl: IWebSocketInformation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerIntermediateCertificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWebSocketInformation2>, ::windows::core::GetTrustLevel, ServerCertificate::<Impl, IMPL_OFFSET>, ServerCertificateErrorSeverity::<Impl, IMPL_OFFSET>, ServerCertificateErrors::<Impl, IMPL_OFFSET>, ServerIntermediateCertificates::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IWebSocketServerCustomValidationRequestedEventArgsImpl: Sized {
    fn ServerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerCertificateErrorSeverity(&self) -> ::windows::core::Result<SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
    fn Reject(&self) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebSocketServerCustomValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketServerCustomValidationRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IWebSocketServerCustomValidationRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketServerCustomValidationRequestedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketServerCustomValidationRequestedEventArgsVtbl {
        unsafe extern "system" fn ServerCertificate<Impl: IWebSocketServerCustomValidationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificateErrorSeverity<Impl: IWebSocketServerCustomValidationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCertificateErrorSeverity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificateErrors<Impl: IWebSocketServerCustomValidationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerCertificateErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerIntermediateCertificates<Impl: IWebSocketServerCustomValidationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerIntermediateCertificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reject<Impl: IWebSocketServerCustomValidationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reject().into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IWebSocketServerCustomValidationRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWebSocketServerCustomValidationRequestedEventArgs>,
            ::windows::core::GetTrustLevel,
            ServerCertificate::<Impl, IMPL_OFFSET>,
            ServerCertificateErrorSeverity::<Impl, IMPL_OFFSET>,
            ServerCertificateErrors::<Impl, IMPL_OFFSET>,
            ServerIntermediateCertificates::<Impl, IMPL_OFFSET>,
            Reject::<Impl, IMPL_OFFSET>,
            GetDeferral::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketServerCustomValidationRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
