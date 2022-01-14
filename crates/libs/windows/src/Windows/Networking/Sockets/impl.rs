#[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation", feature = "implement_exclusive"))]
pub trait IControlChannelTrigger_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn ControlChannelTriggerId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServerKeepAliveIntervalInMinutes(&mut self) -> ::windows::core::Result<u32>;
    fn SetServerKeepAliveIntervalInMinutes(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn CurrentKeepAliveIntervalInMinutes(&mut self) -> ::windows::core::Result<u32>;
    fn TransportObject(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn KeepAliveTrigger(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger>;
    fn PushNotificationTrigger(&mut self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger>;
    fn UsingTransport(&mut self, transport: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn WaitForPushEnabled(&mut self) -> ::windows::core::Result<ControlChannelTriggerStatus>;
    fn DecreaseNetworkKeepAliveInterval(&mut self) -> ::windows::core::Result<()>;
    fn FlushTransport(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IControlChannelTrigger {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTrigger";
}
#[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation", feature = "implement_exclusive"))]
impl IControlChannelTrigger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTrigger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlChannelTrigger_Vtbl {
        unsafe extern "system" fn ControlChannelTriggerId<Impl: IControlChannelTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServerKeepAliveIntervalInMinutes<Impl: IControlChannelTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetServerKeepAliveIntervalInMinutes<Impl: IControlChannelTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerKeepAliveIntervalInMinutes(value).into()
        }
        unsafe extern "system" fn CurrentKeepAliveIntervalInMinutes<Impl: IControlChannelTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TransportObject<Impl: IControlChannelTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KeepAliveTrigger<Impl: IControlChannelTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PushNotificationTrigger<Impl: IControlChannelTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UsingTransport<Impl: IControlChannelTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UsingTransport(&*(&transport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WaitForPushEnabled<Impl: IControlChannelTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ControlChannelTriggerStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DecreaseNetworkKeepAliveInterval<Impl: IControlChannelTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DecreaseNetworkKeepAliveInterval().into()
        }
        unsafe extern "system" fn FlushTransport<Impl: IControlChannelTrigger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushTransport().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IControlChannelTrigger, BASE_OFFSET>(),
            ControlChannelTriggerId: ControlChannelTriggerId::<Impl, IMPL_OFFSET>,
            ServerKeepAliveIntervalInMinutes: ServerKeepAliveIntervalInMinutes::<Impl, IMPL_OFFSET>,
            SetServerKeepAliveIntervalInMinutes: SetServerKeepAliveIntervalInMinutes::<Impl, IMPL_OFFSET>,
            CurrentKeepAliveIntervalInMinutes: CurrentKeepAliveIntervalInMinutes::<Impl, IMPL_OFFSET>,
            TransportObject: TransportObject::<Impl, IMPL_OFFSET>,
            KeepAliveTrigger: KeepAliveTrigger::<Impl, IMPL_OFFSET>,
            PushNotificationTrigger: PushNotificationTrigger::<Impl, IMPL_OFFSET>,
            UsingTransport: UsingTransport::<Impl, IMPL_OFFSET>,
            WaitForPushEnabled: WaitForPushEnabled::<Impl, IMPL_OFFSET>,
            DecreaseNetworkKeepAliveInterval: DecreaseNetworkKeepAliveInterval::<Impl, IMPL_OFFSET>,
            FlushTransport: FlushTransport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChannelTrigger as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlChannelTrigger2_Impl: Sized {
    fn IsWakeFromLowPowerSupported(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IControlChannelTrigger2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTrigger2";
}
#[cfg(feature = "implement_exclusive")]
impl IControlChannelTrigger2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTrigger2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlChannelTrigger2_Vtbl {
        unsafe extern "system" fn IsWakeFromLowPowerSupported<Impl: IControlChannelTrigger2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IControlChannelTrigger2, BASE_OFFSET>(),
            IsWakeFromLowPowerSupported: IsWakeFromLowPowerSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChannelTrigger2 as ::windows::core::Interface>::IID
    }
}
pub trait IControlChannelTriggerEventDetails_Impl: Sized {
    fn ControlChannelTrigger(&mut self) -> ::windows::core::Result<ControlChannelTrigger>;
}
impl ::windows::core::RuntimeName for IControlChannelTriggerEventDetails {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTriggerEventDetails";
}
impl IControlChannelTriggerEventDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTriggerEventDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlChannelTriggerEventDetails_Vtbl {
        unsafe extern "system" fn ControlChannelTrigger<Impl: IControlChannelTriggerEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IControlChannelTriggerEventDetails, BASE_OFFSET>(),
            ControlChannelTrigger: ControlChannelTrigger::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChannelTriggerEventDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IControlChannelTriggerFactory_Impl: Sized {
    fn CreateControlChannelTrigger(&mut self, channelid: &::windows::core::HSTRING, serverkeepaliveintervalinminutes: u32) -> ::windows::core::Result<ControlChannelTrigger>;
    fn CreateControlChannelTriggerEx(&mut self, channelid: &::windows::core::HSTRING, serverkeepaliveintervalinminutes: u32, resourcerequesttype: ControlChannelTriggerResourceType) -> ::windows::core::Result<ControlChannelTrigger>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IControlChannelTriggerFactory {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTriggerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IControlChannelTriggerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTriggerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlChannelTriggerFactory_Vtbl {
        unsafe extern "system" fn CreateControlChannelTrigger<Impl: IControlChannelTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serverkeepaliveintervalinminutes: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateControlChannelTriggerEx<Impl: IControlChannelTriggerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, serverkeepaliveintervalinminutes: u32, resourcerequesttype: ControlChannelTriggerResourceType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IControlChannelTriggerFactory, BASE_OFFSET>(),
            CreateControlChannelTrigger: CreateControlChannelTrigger::<Impl, IMPL_OFFSET>,
            CreateControlChannelTriggerEx: CreateControlChannelTriggerEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChannelTriggerFactory as ::windows::core::Interface>::IID
    }
}
pub trait IControlChannelTriggerResetEventDetails_Impl: Sized {
    fn ResetReason(&mut self) -> ::windows::core::Result<ControlChannelTriggerResetReason>;
    fn HardwareSlotReset(&mut self) -> ::windows::core::Result<bool>;
    fn SoftwareSlotReset(&mut self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IControlChannelTriggerResetEventDetails {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTriggerResetEventDetails";
}
impl IControlChannelTriggerResetEventDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTriggerResetEventDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlChannelTriggerResetEventDetails_Vtbl {
        unsafe extern "system" fn ResetReason<Impl: IControlChannelTriggerResetEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ControlChannelTriggerResetReason) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HardwareSlotReset<Impl: IControlChannelTriggerResetEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SoftwareSlotReset<Impl: IControlChannelTriggerResetEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IControlChannelTriggerResetEventDetails, BASE_OFFSET>(),
            ResetReason: ResetReason::<Impl, IMPL_OFFSET>,
            HardwareSlotReset: HardwareSlotReset::<Impl, IMPL_OFFSET>,
            SoftwareSlotReset: SoftwareSlotReset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChannelTriggerResetEventDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDatagramSocket_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn Control(&mut self) -> ::windows::core::Result<DatagramSocketControl>;
    fn Information(&mut self) -> ::windows::core::Result<DatagramSocketInformation>;
    fn OutputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn ConnectAsync(&mut self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectWithEndpointPairAsync(&mut self, endpointpair: &::core::option::Option<super::EndpointPair>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BindServiceNameAsync(&mut self, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BindEndpointAsync(&mut self, localhostname: &::core::option::Option<super::HostName>, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn JoinMulticastGroup(&mut self, host: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn GetOutputStreamAsync(&mut self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>>;
    fn GetOutputStreamWithEndpointPairAsync(&mut self, endpointpair: &::core::option::Option<super::EndpointPair>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IOutputStream>>;
    fn MessageReceived(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DatagramSocket, DatagramSocketMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDatagramSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDatagramSocket_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocket_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocket_Vtbl {
        unsafe extern "system" fn Control<Impl: IDatagramSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Information<Impl: IDatagramSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OutputStream<Impl: IDatagramSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectAsync<Impl: IDatagramSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectWithEndpointPairAsync<Impl: IDatagramSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpointpair: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BindServiceNameAsync<Impl: IDatagramSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BindEndpointAsync<Impl: IDatagramSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localhostname: ::windows::core::RawPtr, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn JoinMulticastGroup<Impl: IDatagramSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, host: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).JoinMulticastGroup(&*(&host as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetOutputStreamAsync<Impl: IDatagramSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetOutputStreamWithEndpointPairAsync<Impl: IDatagramSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpointpair: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MessageReceived<Impl: IDatagramSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMessageReceived<Impl: IDatagramSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceived(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDatagramSocket, BASE_OFFSET>(),
            Control: Control::<Impl, IMPL_OFFSET>,
            Information: Information::<Impl, IMPL_OFFSET>,
            OutputStream: OutputStream::<Impl, IMPL_OFFSET>,
            ConnectAsync: ConnectAsync::<Impl, IMPL_OFFSET>,
            ConnectWithEndpointPairAsync: ConnectWithEndpointPairAsync::<Impl, IMPL_OFFSET>,
            BindServiceNameAsync: BindServiceNameAsync::<Impl, IMPL_OFFSET>,
            BindEndpointAsync: BindEndpointAsync::<Impl, IMPL_OFFSET>,
            JoinMulticastGroup: JoinMulticastGroup::<Impl, IMPL_OFFSET>,
            GetOutputStreamAsync: GetOutputStreamAsync::<Impl, IMPL_OFFSET>,
            GetOutputStreamWithEndpointPairAsync: GetOutputStreamWithEndpointPairAsync::<Impl, IMPL_OFFSET>,
            MessageReceived: MessageReceived::<Impl, IMPL_OFFSET>,
            RemoveMessageReceived: RemoveMessageReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IDatagramSocket2_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn BindServiceNameAndAdapterAsync(&mut self, localservicename: &::windows::core::HSTRING, adapter: &::core::option::Option<super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDatagramSocket2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocket2";
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IDatagramSocket2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocket2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocket2_Vtbl {
        unsafe extern "system" fn BindServiceNameAndAdapterAsync<Impl: IDatagramSocket2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDatagramSocket2, BASE_OFFSET>(),
            BindServiceNameAndAdapterAsync: BindServiceNameAndAdapterAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocket2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDatagramSocket3_Impl: Sized {
    fn CancelIOAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn EnableTransferOwnership(&mut self, taskid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnableTransferOwnershipWithConnectedStandbyAction(&mut self, taskid: &::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()>;
    fn TransferOwnership(&mut self, socketid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContext(&mut self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContextAndKeepAliveTime(&mut self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>, keepalivetime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDatagramSocket3 {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocket3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IDatagramSocket3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocket3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocket3_Vtbl {
        unsafe extern "system" fn CancelIOAsync<Impl: IDatagramSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnableTransferOwnership<Impl: IDatagramSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableTransferOwnership(&*(&taskid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableTransferOwnershipWithConnectedStandbyAction<Impl: IDatagramSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableTransferOwnershipWithConnectedStandbyAction(&*(&taskid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), connectedstandbyaction).into()
        }
        unsafe extern "system" fn TransferOwnership<Impl: IDatagramSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferOwnership(&*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferOwnershipWithContext<Impl: IDatagramSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferOwnershipWithContext(&*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <SocketActivityContext as ::windows::core::Abi>::Abi as *const <SocketActivityContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferOwnershipWithContextAndKeepAliveTime<Impl: IDatagramSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, keepalivetime: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .TransferOwnershipWithContextAndKeepAliveTime(
                    &*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&data as *const <SocketActivityContext as ::windows::core::Abi>::Abi as *const <SocketActivityContext as ::windows::core::DefaultType>::DefaultType),
                    &*(&keepalivetime as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDatagramSocket3, BASE_OFFSET>(),
            CancelIOAsync: CancelIOAsync::<Impl, IMPL_OFFSET>,
            EnableTransferOwnership: EnableTransferOwnership::<Impl, IMPL_OFFSET>,
            EnableTransferOwnershipWithConnectedStandbyAction: EnableTransferOwnershipWithConnectedStandbyAction::<Impl, IMPL_OFFSET>,
            TransferOwnership: TransferOwnership::<Impl, IMPL_OFFSET>,
            TransferOwnershipWithContext: TransferOwnershipWithContext::<Impl, IMPL_OFFSET>,
            TransferOwnershipWithContextAndKeepAliveTime: TransferOwnershipWithContextAndKeepAliveTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocket3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketControl_Impl: Sized {
    fn QualityOfService(&mut self) -> ::windows::core::Result<SocketQualityOfService>;
    fn SetQualityOfService(&mut self, value: SocketQualityOfService) -> ::windows::core::Result<()>;
    fn OutboundUnicastHopLimit(&mut self) -> ::windows::core::Result<u8>;
    fn SetOutboundUnicastHopLimit(&mut self, value: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatagramSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocketControl";
}
#[cfg(feature = "implement_exclusive")]
impl IDatagramSocketControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocketControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocketControl_Vtbl {
        unsafe extern "system" fn QualityOfService<Impl: IDatagramSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketQualityOfService) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetQualityOfService<Impl: IDatagramSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocketQualityOfService) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQualityOfService(value).into()
        }
        unsafe extern "system" fn OutboundUnicastHopLimit<Impl: IDatagramSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOutboundUnicastHopLimit<Impl: IDatagramSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundUnicastHopLimit(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDatagramSocketControl, BASE_OFFSET>(),
            QualityOfService: QualityOfService::<Impl, IMPL_OFFSET>,
            SetQualityOfService: SetQualityOfService::<Impl, IMPL_OFFSET>,
            OutboundUnicastHopLimit: OutboundUnicastHopLimit::<Impl, IMPL_OFFSET>,
            SetOutboundUnicastHopLimit: SetOutboundUnicastHopLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocketControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketControl2_Impl: Sized {
    fn InboundBufferSizeInBytes(&mut self) -> ::windows::core::Result<u32>;
    fn SetInboundBufferSizeInBytes(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn DontFragment(&mut self) -> ::windows::core::Result<bool>;
    fn SetDontFragment(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatagramSocketControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocketControl2";
}
#[cfg(feature = "implement_exclusive")]
impl IDatagramSocketControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocketControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocketControl2_Vtbl {
        unsafe extern "system" fn InboundBufferSizeInBytes<Impl: IDatagramSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetInboundBufferSizeInBytes<Impl: IDatagramSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInboundBufferSizeInBytes(value).into()
        }
        unsafe extern "system" fn DontFragment<Impl: IDatagramSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDontFragment<Impl: IDatagramSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDontFragment(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDatagramSocketControl2, BASE_OFFSET>(),
            InboundBufferSizeInBytes: InboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            SetInboundBufferSizeInBytes: SetInboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            DontFragment: DontFragment::<Impl, IMPL_OFFSET>,
            SetDontFragment: SetDontFragment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocketControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketControl3_Impl: Sized {
    fn MulticastOnly(&mut self) -> ::windows::core::Result<bool>;
    fn SetMulticastOnly(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatagramSocketControl3 {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocketControl3";
}
#[cfg(feature = "implement_exclusive")]
impl IDatagramSocketControl3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocketControl3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocketControl3_Vtbl {
        unsafe extern "system" fn MulticastOnly<Impl: IDatagramSocketControl3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMulticastOnly<Impl: IDatagramSocketControl3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMulticastOnly(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDatagramSocketControl3, BASE_OFFSET>(),
            MulticastOnly: MulticastOnly::<Impl, IMPL_OFFSET>,
            SetMulticastOnly: SetMulticastOnly::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocketControl3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDatagramSocketInformation_Impl: Sized {
    fn LocalAddress(&mut self) -> ::windows::core::Result<super::HostName>;
    fn LocalPort(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteAddress(&mut self) -> ::windows::core::Result<super::HostName>;
    fn RemotePort(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDatagramSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocketInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IDatagramSocketInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocketInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocketInformation_Vtbl {
        unsafe extern "system" fn LocalAddress<Impl: IDatagramSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocalPort<Impl: IDatagramSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteAddress<Impl: IDatagramSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemotePort<Impl: IDatagramSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDatagramSocketInformation, BASE_OFFSET>(),
            LocalAddress: LocalAddress::<Impl, IMPL_OFFSET>,
            LocalPort: LocalPort::<Impl, IMPL_OFFSET>,
            RemoteAddress: RemoteAddress::<Impl, IMPL_OFFSET>,
            RemotePort: RemotePort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocketInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IDatagramSocketMessageReceivedEventArgs_Impl: Sized {
    fn RemoteAddress(&mut self) -> ::windows::core::Result<super::HostName>;
    fn RemotePort(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAddress(&mut self) -> ::windows::core::Result<super::HostName>;
    fn GetDataReader(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::DataReader>;
    fn GetDataStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDatagramSocketMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocketMessageReceivedEventArgs";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IDatagramSocketMessageReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocketMessageReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocketMessageReceivedEventArgs_Vtbl {
        unsafe extern "system" fn RemoteAddress<Impl: IDatagramSocketMessageReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemotePort<Impl: IDatagramSocketMessageReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocalAddress<Impl: IDatagramSocketMessageReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDataReader<Impl: IDatagramSocketMessageReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDataStream<Impl: IDatagramSocketMessageReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDatagramSocketMessageReceivedEventArgs, BASE_OFFSET>(),
            RemoteAddress: RemoteAddress::<Impl, IMPL_OFFSET>,
            RemotePort: RemotePort::<Impl, IMPL_OFFSET>,
            LocalAddress: LocalAddress::<Impl, IMPL_OFFSET>,
            GetDataReader: GetDataReader::<Impl, IMPL_OFFSET>,
            GetDataStream: GetDataStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocketMessageReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IDatagramSocketStatics_Impl: Sized {
    fn GetEndpointPairsAsync(&mut self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>;
    fn GetEndpointPairsWithSortOptionsAsync(&mut self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING, sortoptions: super::HostNameSortOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IDatagramSocketStatics {
    const NAME: &'static str = "Windows.Networking.Sockets.IDatagramSocketStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IDatagramSocketStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDatagramSocketStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDatagramSocketStatics_Vtbl {
        unsafe extern "system" fn GetEndpointPairsAsync<Impl: IDatagramSocketStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetEndpointPairsWithSortOptionsAsync<Impl: IDatagramSocketStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sortoptions: super::HostNameSortOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDatagramSocketStatics, BASE_OFFSET>(),
            GetEndpointPairsAsync: GetEndpointPairsAsync::<Impl, IMPL_OFFSET>,
            GetEndpointPairsWithSortOptionsAsync: GetEndpointPairsWithSortOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDatagramSocketStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMessageWebSocket_Impl: Sized + super::super::Foundation::IClosable_Impl + IWebSocket_Impl {
    fn Control(&mut self) -> ::windows::core::Result<MessageWebSocketControl>;
    fn Information(&mut self) -> ::windows::core::Result<MessageWebSocketInformation>;
    fn MessageReceived(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MessageWebSocket, MessageWebSocketMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMessageWebSocket_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocket_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocket_Vtbl {
        unsafe extern "system" fn Control<Impl: IMessageWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Information<Impl: IMessageWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn MessageReceived<Impl: IMessageWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMessageReceived<Impl: IMessageWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceived(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMessageWebSocket, BASE_OFFSET>(),
            Control: Control::<Impl, IMPL_OFFSET>,
            Information: Information::<Impl, IMPL_OFFSET>,
            MessageReceived: MessageReceived::<Impl, IMPL_OFFSET>,
            RemoveMessageReceived: RemoveMessageReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMessageWebSocket2_Impl: Sized + super::super::Foundation::IClosable_Impl + IMessageWebSocket_Impl + IWebSocket_Impl {
    fn ServerCustomValidationRequested(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<MessageWebSocket, WebSocketServerCustomValidationRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerCustomValidationRequested(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocket2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocket2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMessageWebSocket2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocket2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocket2_Vtbl {
        unsafe extern "system" fn ServerCustomValidationRequested<Impl: IMessageWebSocket2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveServerCustomValidationRequested<Impl: IMessageWebSocket2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveServerCustomValidationRequested(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMessageWebSocket2, BASE_OFFSET>(),
            ServerCustomValidationRequested: ServerCustomValidationRequested::<Impl, IMPL_OFFSET>,
            RemoveServerCustomValidationRequested: RemoveServerCustomValidationRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocket2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMessageWebSocket3_Impl: Sized {
    fn SendNonfinalFrameAsync(&mut self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn SendFinalFrameAsync(&mut self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocket3 {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocket3";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMessageWebSocket3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocket3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocket3_Vtbl {
        unsafe extern "system" fn SendNonfinalFrameAsync<Impl: IMessageWebSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SendFinalFrameAsync<Impl: IMessageWebSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMessageWebSocket3, BASE_OFFSET>(),
            SendNonfinalFrameAsync: SendNonfinalFrameAsync::<Impl, IMPL_OFFSET>,
            SendFinalFrameAsync: SendFinalFrameAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocket3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IMessageWebSocketControl_Impl: Sized + IWebSocketControl_Impl {
    fn MaxMessageSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxMessageSize(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn MessageType(&mut self) -> ::windows::core::Result<SocketMessageType>;
    fn SetMessageType(&mut self, value: SocketMessageType) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocketControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IMessageWebSocketControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocketControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocketControl_Vtbl {
        unsafe extern "system" fn MaxMessageSize<Impl: IMessageWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMaxMessageSize<Impl: IMessageWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxMessageSize(value).into()
        }
        unsafe extern "system" fn MessageType<Impl: IMessageWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketMessageType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMessageType<Impl: IMessageWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocketMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageType(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMessageWebSocketControl, BASE_OFFSET>(),
            MaxMessageSize: MaxMessageSize::<Impl, IMPL_OFFSET>,
            SetMaxMessageSize: SetMaxMessageSize::<Impl, IMPL_OFFSET>,
            MessageType: MessageType::<Impl, IMPL_OFFSET>,
            SetMessageType: SetMessageType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocketControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IMessageWebSocketControl2_Impl: Sized {
    fn DesiredUnsolicitedPongInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDesiredUnsolicitedPongInterval(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ActualUnsolicitedPongInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ReceiveMode(&mut self) -> ::windows::core::Result<MessageWebSocketReceiveMode>;
    fn SetReceiveMode(&mut self, value: MessageWebSocketReceiveMode) -> ::windows::core::Result<()>;
    fn ClientCertificate(&mut self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetClientCertificate(&mut self, value: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocketControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocketControl2";
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IMessageWebSocketControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocketControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocketControl2_Vtbl {
        unsafe extern "system" fn DesiredUnsolicitedPongInterval<Impl: IMessageWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredUnsolicitedPongInterval<Impl: IMessageWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredUnsolicitedPongInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualUnsolicitedPongInterval<Impl: IMessageWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReceiveMode<Impl: IMessageWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut MessageWebSocketReceiveMode) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReceiveMode<Impl: IMessageWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: MessageWebSocketReceiveMode) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReceiveMode(value).into()
        }
        unsafe extern "system" fn ClientCertificate<Impl: IMessageWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClientCertificate<Impl: IMessageWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCertificate(&*(&value as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMessageWebSocketControl2, BASE_OFFSET>(),
            DesiredUnsolicitedPongInterval: DesiredUnsolicitedPongInterval::<Impl, IMPL_OFFSET>,
            SetDesiredUnsolicitedPongInterval: SetDesiredUnsolicitedPongInterval::<Impl, IMPL_OFFSET>,
            ActualUnsolicitedPongInterval: ActualUnsolicitedPongInterval::<Impl, IMPL_OFFSET>,
            ReceiveMode: ReceiveMode::<Impl, IMPL_OFFSET>,
            SetReceiveMode: SetReceiveMode::<Impl, IMPL_OFFSET>,
            ClientCertificate: ClientCertificate::<Impl, IMPL_OFFSET>,
            SetClientCertificate: SetClientCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocketControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMessageWebSocketMessageReceivedEventArgs_Impl: Sized {
    fn MessageType(&mut self) -> ::windows::core::Result<SocketMessageType>;
    fn GetDataReader(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::DataReader>;
    fn GetDataStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocketMessageReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMessageWebSocketMessageReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocketMessageReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocketMessageReceivedEventArgs_Vtbl {
        unsafe extern "system" fn MessageType<Impl: IMessageWebSocketMessageReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketMessageType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDataReader<Impl: IMessageWebSocketMessageReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDataStream<Impl: IMessageWebSocketMessageReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMessageWebSocketMessageReceivedEventArgs, BASE_OFFSET>(),
            MessageType: MessageType::<Impl, IMPL_OFFSET>,
            GetDataReader: GetDataReader::<Impl, IMPL_OFFSET>,
            GetDataStream: GetDataStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocketMessageReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IMessageWebSocketMessageReceivedEventArgs2_Impl: Sized + IMessageWebSocketMessageReceivedEventArgs_Impl {
    fn IsMessageComplete(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMessageWebSocketMessageReceivedEventArgs2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IMessageWebSocketMessageReceivedEventArgs2";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IMessageWebSocketMessageReceivedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageWebSocketMessageReceivedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageWebSocketMessageReceivedEventArgs2_Vtbl {
        unsafe extern "system" fn IsMessageComplete<Impl: IMessageWebSocketMessageReceivedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMessageWebSocketMessageReceivedEventArgs2, BASE_OFFSET>(),
            IsMessageComplete: IsMessageComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageWebSocketMessageReceivedEventArgs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IServerMessageWebSocket_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn MessageReceived(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, MessageWebSocketMessageReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMessageReceived(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Control(&mut self) -> ::windows::core::Result<ServerMessageWebSocketControl>;
    fn Information(&mut self) -> ::windows::core::Result<ServerMessageWebSocketInformation>;
    fn OutputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn Closed(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ServerMessageWebSocket, WebSocketClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CloseWithStatus(&mut self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IServerMessageWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IServerMessageWebSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IServerMessageWebSocket_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerMessageWebSocket_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerMessageWebSocket_Vtbl {
        unsafe extern "system" fn MessageReceived<Impl: IServerMessageWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveMessageReceived<Impl: IServerMessageWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMessageReceived(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Control<Impl: IServerMessageWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Information<Impl: IServerMessageWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OutputStream<Impl: IServerMessageWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Closed<Impl: IServerMessageWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveClosed<Impl: IServerMessageWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CloseWithStatus<Impl: IServerMessageWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, code: u16, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseWithStatus(code, &*(&reason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IServerMessageWebSocket, BASE_OFFSET>(),
            MessageReceived: MessageReceived::<Impl, IMPL_OFFSET>,
            RemoveMessageReceived: RemoveMessageReceived::<Impl, IMPL_OFFSET>,
            Control: Control::<Impl, IMPL_OFFSET>,
            Information: Information::<Impl, IMPL_OFFSET>,
            OutputStream: OutputStream::<Impl, IMPL_OFFSET>,
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
            CloseWithStatus: CloseWithStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerMessageWebSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IServerMessageWebSocketControl_Impl: Sized {
    fn MessageType(&mut self) -> ::windows::core::Result<SocketMessageType>;
    fn SetMessageType(&mut self, value: SocketMessageType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IServerMessageWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IServerMessageWebSocketControl";
}
#[cfg(feature = "implement_exclusive")]
impl IServerMessageWebSocketControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerMessageWebSocketControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerMessageWebSocketControl_Vtbl {
        unsafe extern "system" fn MessageType<Impl: IServerMessageWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketMessageType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMessageType<Impl: IServerMessageWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocketMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageType(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IServerMessageWebSocketControl, BASE_OFFSET>(),
            MessageType: MessageType::<Impl, IMPL_OFFSET>,
            SetMessageType: SetMessageType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerMessageWebSocketControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IServerMessageWebSocketInformation_Impl: Sized {
    fn BandwidthStatistics(&mut self) -> ::windows::core::Result<BandwidthStatistics>;
    fn Protocol(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAddress(&mut self) -> ::windows::core::Result<super::HostName>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IServerMessageWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IServerMessageWebSocketInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IServerMessageWebSocketInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerMessageWebSocketInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerMessageWebSocketInformation_Vtbl {
        unsafe extern "system" fn BandwidthStatistics<Impl: IServerMessageWebSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Protocol<Impl: IServerMessageWebSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocalAddress<Impl: IServerMessageWebSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IServerMessageWebSocketInformation, BASE_OFFSET>(),
            BandwidthStatistics: BandwidthStatistics::<Impl, IMPL_OFFSET>,
            Protocol: Protocol::<Impl, IMPL_OFFSET>,
            LocalAddress: LocalAddress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerMessageWebSocketInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IServerStreamWebSocket_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn Information(&mut self) -> ::windows::core::Result<ServerStreamWebSocketInformation>;
    fn InputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn OutputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn Closed(&mut self, value: &::core::option::Option<super::super::Foundation::TypedEventHandler<ServerStreamWebSocket, WebSocketClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CloseWithStatus(&mut self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IServerStreamWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IServerStreamWebSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IServerStreamWebSocket_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerStreamWebSocket_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerStreamWebSocket_Vtbl {
        unsafe extern "system" fn Information<Impl: IServerStreamWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InputStream<Impl: IServerStreamWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OutputStream<Impl: IServerStreamWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Closed<Impl: IServerStreamWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveClosed<Impl: IServerStreamWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CloseWithStatus<Impl: IServerStreamWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, code: u16, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseWithStatus(code, &*(&reason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IServerStreamWebSocket, BASE_OFFSET>(),
            Information: Information::<Impl, IMPL_OFFSET>,
            InputStream: InputStream::<Impl, IMPL_OFFSET>,
            OutputStream: OutputStream::<Impl, IMPL_OFFSET>,
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
            CloseWithStatus: CloseWithStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerStreamWebSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IServerStreamWebSocketInformation_Impl: Sized {
    fn BandwidthStatistics(&mut self) -> ::windows::core::Result<BandwidthStatistics>;
    fn Protocol(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LocalAddress(&mut self) -> ::windows::core::Result<super::HostName>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IServerStreamWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IServerStreamWebSocketInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IServerStreamWebSocketInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerStreamWebSocketInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerStreamWebSocketInformation_Vtbl {
        unsafe extern "system" fn BandwidthStatistics<Impl: IServerStreamWebSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Protocol<Impl: IServerStreamWebSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocalAddress<Impl: IServerStreamWebSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IServerStreamWebSocketInformation, BASE_OFFSET>(),
            BandwidthStatistics: BandwidthStatistics::<Impl, IMPL_OFFSET>,
            Protocol: Protocol::<Impl, IMPL_OFFSET>,
            LocalAddress: LocalAddress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerStreamWebSocketInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISocketActivityContext_Impl: Sized {
    fn Data(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocketActivityContext {
    const NAME: &'static str = "Windows.Networking.Sockets.ISocketActivityContext";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISocketActivityContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocketActivityContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocketActivityContext_Vtbl {
        unsafe extern "system" fn Data<Impl: ISocketActivityContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISocketActivityContext, BASE_OFFSET>(), Data: Data::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocketActivityContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait ISocketActivityContextFactory_Impl: Sized {
    fn Create(&mut self, data: &::core::option::Option<super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<SocketActivityContext>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocketActivityContextFactory {
    const NAME: &'static str = "Windows.Networking.Sockets.ISocketActivityContextFactory";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ISocketActivityContextFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocketActivityContextFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocketActivityContextFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: ISocketActivityContextFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISocketActivityContextFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocketActivityContextFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketActivityInformation_Impl: Sized {
    fn TaskId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Id(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SocketKind(&mut self) -> ::windows::core::Result<SocketActivityKind>;
    fn Context(&mut self) -> ::windows::core::Result<SocketActivityContext>;
    fn DatagramSocket(&mut self) -> ::windows::core::Result<DatagramSocket>;
    fn StreamSocket(&mut self) -> ::windows::core::Result<StreamSocket>;
    fn StreamSocketListener(&mut self) -> ::windows::core::Result<StreamSocketListener>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISocketActivityInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.ISocketActivityInformation";
}
#[cfg(feature = "implement_exclusive")]
impl ISocketActivityInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocketActivityInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocketActivityInformation_Vtbl {
        unsafe extern "system" fn TaskId<Impl: ISocketActivityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Id<Impl: ISocketActivityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SocketKind<Impl: ISocketActivityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketActivityKind) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Context<Impl: ISocketActivityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DatagramSocket<Impl: ISocketActivityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StreamSocket<Impl: ISocketActivityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StreamSocketListener<Impl: ISocketActivityInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISocketActivityInformation, BASE_OFFSET>(),
            TaskId: TaskId::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            SocketKind: SocketKind::<Impl, IMPL_OFFSET>,
            Context: Context::<Impl, IMPL_OFFSET>,
            DatagramSocket: DatagramSocket::<Impl, IMPL_OFFSET>,
            StreamSocket: StreamSocket::<Impl, IMPL_OFFSET>,
            StreamSocketListener: StreamSocketListener::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocketActivityInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait ISocketActivityInformationStatics_Impl: Sized {
    fn AllSockets(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, SocketActivityInformation>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for ISocketActivityInformationStatics {
    const NAME: &'static str = "Windows.Networking.Sockets.ISocketActivityInformationStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ISocketActivityInformationStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocketActivityInformationStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocketActivityInformationStatics_Vtbl {
        unsafe extern "system" fn AllSockets<Impl: ISocketActivityInformationStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISocketActivityInformationStatics, BASE_OFFSET>(),
            AllSockets: AllSockets::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocketActivityInformationStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketActivityTriggerDetails_Impl: Sized {
    fn Reason(&mut self) -> ::windows::core::Result<SocketActivityTriggerReason>;
    fn SocketInformation(&mut self) -> ::windows::core::Result<SocketActivityInformation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISocketActivityTriggerDetails {
    const NAME: &'static str = "Windows.Networking.Sockets.ISocketActivityTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl ISocketActivityTriggerDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocketActivityTriggerDetails_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocketActivityTriggerDetails_Vtbl {
        unsafe extern "system" fn Reason<Impl: ISocketActivityTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketActivityTriggerReason) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SocketInformation<Impl: ISocketActivityTriggerDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ISocketActivityTriggerDetails, BASE_OFFSET>(),
            Reason: Reason::<Impl, IMPL_OFFSET>,
            SocketInformation: SocketInformation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocketActivityTriggerDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ISocketErrorStatics_Impl: Sized {
    fn GetStatus(&mut self, hresult: i32) -> ::windows::core::Result<SocketErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ISocketErrorStatics {
    const NAME: &'static str = "Windows.Networking.Sockets.ISocketErrorStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ISocketErrorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISocketErrorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISocketErrorStatics_Vtbl {
        unsafe extern "system" fn GetStatus<Impl: ISocketErrorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut SocketErrorStatus) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, ISocketErrorStatics, BASE_OFFSET>(), GetStatus: GetStatus::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISocketErrorStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IStreamSocket_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn Control(&mut self) -> ::windows::core::Result<StreamSocketControl>;
    fn Information(&mut self) -> ::windows::core::Result<StreamSocketInformation>;
    fn InputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn OutputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn ConnectWithEndpointPairAsync(&mut self, endpointpair: &::core::option::Option<super::EndpointPair>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectAsync(&mut self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectWithEndpointPairAndProtectionLevelAsync(&mut self, endpointpair: &::core::option::Option<super::EndpointPair>, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectWithProtectionLevelAsync(&mut self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UpgradeToSslAsync(&mut self, protectionlevel: SocketProtectionLevel, validationhostname: &::core::option::Option<super::HostName>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IStreamSocket_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocket_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocket_Vtbl {
        unsafe extern "system" fn Control<Impl: IStreamSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Information<Impl: IStreamSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InputStream<Impl: IStreamSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OutputStream<Impl: IStreamSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectWithEndpointPairAsync<Impl: IStreamSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpointpair: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectAsync<Impl: IStreamSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectWithEndpointPairAndProtectionLevelAsync<Impl: IStreamSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, endpointpair: ::windows::core::RawPtr, protectionlevel: SocketProtectionLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectWithProtectionLevelAsync<Impl: IStreamSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpgradeToSslAsync<Impl: IStreamSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protectionlevel: SocketProtectionLevel, validationhostname: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocket, BASE_OFFSET>(),
            Control: Control::<Impl, IMPL_OFFSET>,
            Information: Information::<Impl, IMPL_OFFSET>,
            InputStream: InputStream::<Impl, IMPL_OFFSET>,
            OutputStream: OutputStream::<Impl, IMPL_OFFSET>,
            ConnectWithEndpointPairAsync: ConnectWithEndpointPairAsync::<Impl, IMPL_OFFSET>,
            ConnectAsync: ConnectAsync::<Impl, IMPL_OFFSET>,
            ConnectWithEndpointPairAndProtectionLevelAsync: ConnectWithEndpointPairAndProtectionLevelAsync::<Impl, IMPL_OFFSET>,
            ConnectWithProtectionLevelAsync: ConnectWithProtectionLevelAsync::<Impl, IMPL_OFFSET>,
            UpgradeToSslAsync: UpgradeToSslAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IStreamSocket2_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn ConnectWithProtectionLevelAndAdapterAsync(&mut self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel, adapter: &::core::option::Option<super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocket2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocket2";
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IStreamSocket2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocket2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocket2_Vtbl {
        unsafe extern "system" fn ConnectWithProtectionLevelAndAdapterAsync<Impl: IStreamSocket2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocket2, BASE_OFFSET>(),
            ConnectWithProtectionLevelAndAdapterAsync: ConnectWithProtectionLevelAndAdapterAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocket2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreamSocket3_Impl: Sized {
    fn CancelIOAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn EnableTransferOwnership(&mut self, taskid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnableTransferOwnershipWithConnectedStandbyAction(&mut self, taskid: &::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()>;
    fn TransferOwnership(&mut self, socketid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContext(&mut self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContextAndKeepAliveTime(&mut self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>, keepalivetime: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocket3 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocket3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStreamSocket3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocket3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocket3_Vtbl {
        unsafe extern "system" fn CancelIOAsync<Impl: IStreamSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnableTransferOwnership<Impl: IStreamSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableTransferOwnership(&*(&taskid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableTransferOwnershipWithConnectedStandbyAction<Impl: IStreamSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableTransferOwnershipWithConnectedStandbyAction(&*(&taskid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), connectedstandbyaction).into()
        }
        unsafe extern "system" fn TransferOwnership<Impl: IStreamSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferOwnership(&*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferOwnershipWithContext<Impl: IStreamSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferOwnershipWithContext(&*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <SocketActivityContext as ::windows::core::Abi>::Abi as *const <SocketActivityContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferOwnershipWithContextAndKeepAliveTime<Impl: IStreamSocket3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr, keepalivetime: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .TransferOwnershipWithContextAndKeepAliveTime(
                    &*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                    &*(&data as *const <SocketActivityContext as ::windows::core::Abi>::Abi as *const <SocketActivityContext as ::windows::core::DefaultType>::DefaultType),
                    &*(&keepalivetime as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocket3, BASE_OFFSET>(),
            CancelIOAsync: CancelIOAsync::<Impl, IMPL_OFFSET>,
            EnableTransferOwnership: EnableTransferOwnership::<Impl, IMPL_OFFSET>,
            EnableTransferOwnershipWithConnectedStandbyAction: EnableTransferOwnershipWithConnectedStandbyAction::<Impl, IMPL_OFFSET>,
            TransferOwnership: TransferOwnership::<Impl, IMPL_OFFSET>,
            TransferOwnershipWithContext: TransferOwnershipWithContext::<Impl, IMPL_OFFSET>,
            TransferOwnershipWithContextAndKeepAliveTime: TransferOwnershipWithContextAndKeepAliveTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocket3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketControl_Impl: Sized {
    fn NoDelay(&mut self) -> ::windows::core::Result<bool>;
    fn SetNoDelay(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn KeepAlive(&mut self) -> ::windows::core::Result<bool>;
    fn SetKeepAlive(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn OutboundBufferSizeInBytes(&mut self) -> ::windows::core::Result<u32>;
    fn SetOutboundBufferSizeInBytes(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn QualityOfService(&mut self) -> ::windows::core::Result<SocketQualityOfService>;
    fn SetQualityOfService(&mut self, value: SocketQualityOfService) -> ::windows::core::Result<()>;
    fn OutboundUnicastHopLimit(&mut self) -> ::windows::core::Result<u8>;
    fn SetOutboundUnicastHopLimit(&mut self, value: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreamSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketControl";
}
#[cfg(feature = "implement_exclusive")]
impl IStreamSocketControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketControl_Vtbl {
        unsafe extern "system" fn NoDelay<Impl: IStreamSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNoDelay<Impl: IStreamSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoDelay(value).into()
        }
        unsafe extern "system" fn KeepAlive<Impl: IStreamSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeepAlive<Impl: IStreamSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeepAlive(value).into()
        }
        unsafe extern "system" fn OutboundBufferSizeInBytes<Impl: IStreamSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOutboundBufferSizeInBytes<Impl: IStreamSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundBufferSizeInBytes(value).into()
        }
        unsafe extern "system" fn QualityOfService<Impl: IStreamSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketQualityOfService) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetQualityOfService<Impl: IStreamSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocketQualityOfService) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQualityOfService(value).into()
        }
        unsafe extern "system" fn OutboundUnicastHopLimit<Impl: IStreamSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOutboundUnicastHopLimit<Impl: IStreamSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundUnicastHopLimit(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketControl, BASE_OFFSET>(),
            NoDelay: NoDelay::<Impl, IMPL_OFFSET>,
            SetNoDelay: SetNoDelay::<Impl, IMPL_OFFSET>,
            KeepAlive: KeepAlive::<Impl, IMPL_OFFSET>,
            SetKeepAlive: SetKeepAlive::<Impl, IMPL_OFFSET>,
            OutboundBufferSizeInBytes: OutboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            SetOutboundBufferSizeInBytes: SetOutboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            QualityOfService: QualityOfService::<Impl, IMPL_OFFSET>,
            SetQualityOfService: SetQualityOfService::<Impl, IMPL_OFFSET>,
            OutboundUnicastHopLimit: OutboundUnicastHopLimit::<Impl, IMPL_OFFSET>,
            SetOutboundUnicastHopLimit: SetOutboundUnicastHopLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IStreamSocketControl2_Impl: Sized {
    fn IgnorableServerCertificateErrors(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketControl2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IStreamSocketControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketControl2_Vtbl {
        unsafe extern "system" fn IgnorableServerCertificateErrors<Impl: IStreamSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketControl2, BASE_OFFSET>(),
            IgnorableServerCertificateErrors: IgnorableServerCertificateErrors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IStreamSocketControl3_Impl: Sized {
    fn SerializeConnectionAttempts(&mut self) -> ::windows::core::Result<bool>;
    fn SetSerializeConnectionAttempts(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ClientCertificate(&mut self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetClientCertificate(&mut self, value: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketControl3 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketControl3";
}
#[cfg(all(feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IStreamSocketControl3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketControl3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketControl3_Vtbl {
        unsafe extern "system" fn SerializeConnectionAttempts<Impl: IStreamSocketControl3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetSerializeConnectionAttempts<Impl: IStreamSocketControl3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSerializeConnectionAttempts(value).into()
        }
        unsafe extern "system" fn ClientCertificate<Impl: IStreamSocketControl3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClientCertificate<Impl: IStreamSocketControl3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCertificate(&*(&value as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketControl3, BASE_OFFSET>(),
            SerializeConnectionAttempts: SerializeConnectionAttempts::<Impl, IMPL_OFFSET>,
            SetSerializeConnectionAttempts: SetSerializeConnectionAttempts::<Impl, IMPL_OFFSET>,
            ClientCertificate: ClientCertificate::<Impl, IMPL_OFFSET>,
            SetClientCertificate: SetClientCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketControl3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketControl4_Impl: Sized {
    fn MinProtectionLevel(&mut self) -> ::windows::core::Result<SocketProtectionLevel>;
    fn SetMinProtectionLevel(&mut self, value: SocketProtectionLevel) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreamSocketControl4 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketControl4";
}
#[cfg(feature = "implement_exclusive")]
impl IStreamSocketControl4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketControl4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketControl4_Vtbl {
        unsafe extern "system" fn MinProtectionLevel<Impl: IStreamSocketControl4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketProtectionLevel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetMinProtectionLevel<Impl: IStreamSocketControl4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocketProtectionLevel) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinProtectionLevel(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketControl4, BASE_OFFSET>(),
            MinProtectionLevel: MinProtectionLevel::<Impl, IMPL_OFFSET>,
            SetMinProtectionLevel: SetMinProtectionLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketControl4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IStreamSocketInformation_Impl: Sized {
    fn LocalAddress(&mut self) -> ::windows::core::Result<super::HostName>;
    fn LocalPort(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteHostName(&mut self) -> ::windows::core::Result<super::HostName>;
    fn RemoteAddress(&mut self) -> ::windows::core::Result<super::HostName>;
    fn RemoteServiceName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemotePort(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RoundTripTimeStatistics(&mut self) -> ::windows::core::Result<RoundTripTimeStatistics>;
    fn BandwidthStatistics(&mut self) -> ::windows::core::Result<BandwidthStatistics>;
    fn ProtectionLevel(&mut self) -> ::windows::core::Result<SocketProtectionLevel>;
    fn SessionKey(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketInformation";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IStreamSocketInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketInformation_Vtbl {
        unsafe extern "system" fn LocalAddress<Impl: IStreamSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocalPort<Impl: IStreamSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteHostName<Impl: IStreamSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteAddress<Impl: IStreamSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteServiceName<Impl: IStreamSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemotePort<Impl: IStreamSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoundTripTimeStatistics<Impl: IStreamSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RoundTripTimeStatistics) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BandwidthStatistics<Impl: IStreamSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProtectionLevel<Impl: IStreamSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketProtectionLevel) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SessionKey<Impl: IStreamSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketInformation, BASE_OFFSET>(),
            LocalAddress: LocalAddress::<Impl, IMPL_OFFSET>,
            LocalPort: LocalPort::<Impl, IMPL_OFFSET>,
            RemoteHostName: RemoteHostName::<Impl, IMPL_OFFSET>,
            RemoteAddress: RemoteAddress::<Impl, IMPL_OFFSET>,
            RemoteServiceName: RemoteServiceName::<Impl, IMPL_OFFSET>,
            RemotePort: RemotePort::<Impl, IMPL_OFFSET>,
            RoundTripTimeStatistics: RoundTripTimeStatistics::<Impl, IMPL_OFFSET>,
            BandwidthStatistics: BandwidthStatistics::<Impl, IMPL_OFFSET>,
            ProtectionLevel: ProtectionLevel::<Impl, IMPL_OFFSET>,
            SessionKey: SessionKey::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IStreamSocketInformation2_Impl: Sized {
    fn ServerCertificateErrorSeverity(&mut self) -> ::windows::core::Result<SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerCertificate(&mut self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerIntermediateCertificates(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketInformation2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketInformation2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IStreamSocketInformation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketInformation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketInformation2_Vtbl {
        unsafe extern "system" fn ServerCertificateErrorSeverity<Impl: IStreamSocketInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServerCertificateErrors<Impl: IStreamSocketInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServerCertificate<Impl: IStreamSocketInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServerIntermediateCertificates<Impl: IStreamSocketInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketInformation2, BASE_OFFSET>(),
            ServerCertificateErrorSeverity: ServerCertificateErrorSeverity::<Impl, IMPL_OFFSET>,
            ServerCertificateErrors: ServerCertificateErrors::<Impl, IMPL_OFFSET>,
            ServerCertificate: ServerCertificate::<Impl, IMPL_OFFSET>,
            ServerIntermediateCertificates: ServerIntermediateCertificates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreamSocketListener_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn Control(&mut self) -> ::windows::core::Result<StreamSocketListenerControl>;
    fn Information(&mut self) -> ::windows::core::Result<StreamSocketListenerInformation>;
    fn BindServiceNameAsync(&mut self, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BindEndpointAsync(&mut self, localhostname: &::core::option::Option<super::HostName>, localservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn ConnectionReceived(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StreamSocketListener, StreamSocketListenerConnectionReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionReceived(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketListener {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListener";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStreamSocketListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListener_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListener_Vtbl {
        unsafe extern "system" fn Control<Impl: IStreamSocketListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Information<Impl: IStreamSocketListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BindServiceNameAsync<Impl: IStreamSocketListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BindEndpointAsync<Impl: IStreamSocketListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localhostname: ::windows::core::RawPtr, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectionReceived<Impl: IStreamSocketListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveConnectionReceived<Impl: IStreamSocketListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConnectionReceived(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketListener, BASE_OFFSET>(),
            Control: Control::<Impl, IMPL_OFFSET>,
            Information: Information::<Impl, IMPL_OFFSET>,
            BindServiceNameAsync: BindServiceNameAsync::<Impl, IMPL_OFFSET>,
            BindEndpointAsync: BindEndpointAsync::<Impl, IMPL_OFFSET>,
            ConnectionReceived: ConnectionReceived::<Impl, IMPL_OFFSET>,
            RemoveConnectionReceived: RemoveConnectionReceived::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListener as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
pub trait IStreamSocketListener2_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn BindServiceNameWithProtectionLevelAsync(&mut self, localservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn BindServiceNameWithProtectionLevelAndAdapterAsync(&mut self, localservicename: &::windows::core::HSTRING, protectionlevel: SocketProtectionLevel, adapter: &::core::option::Option<super::Connectivity::NetworkAdapter>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketListener2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListener2";
}
#[cfg(all(feature = "Foundation", feature = "Networking_Connectivity", feature = "implement_exclusive"))]
impl IStreamSocketListener2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListener2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListener2_Vtbl {
        unsafe extern "system" fn BindServiceNameWithProtectionLevelAsync<Impl: IStreamSocketListener2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BindServiceNameWithProtectionLevelAndAdapterAsync<Impl: IStreamSocketListener2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, localservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, protectionlevel: SocketProtectionLevel, adapter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketListener2, BASE_OFFSET>(),
            BindServiceNameWithProtectionLevelAsync: BindServiceNameWithProtectionLevelAsync::<Impl, IMPL_OFFSET>,
            BindServiceNameWithProtectionLevelAndAdapterAsync: BindServiceNameWithProtectionLevelAndAdapterAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListener2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IStreamSocketListener3_Impl: Sized {
    fn CancelIOAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn EnableTransferOwnership(&mut self, taskid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnableTransferOwnershipWithConnectedStandbyAction(&mut self, taskid: &::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::Result<()>;
    fn TransferOwnership(&mut self, socketid: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TransferOwnershipWithContext(&mut self, socketid: &::windows::core::HSTRING, data: &::core::option::Option<SocketActivityContext>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketListener3 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListener3";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IStreamSocketListener3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListener3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListener3_Vtbl {
        unsafe extern "system" fn CancelIOAsync<Impl: IStreamSocketListener3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn EnableTransferOwnership<Impl: IStreamSocketListener3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableTransferOwnership(&*(&taskid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EnableTransferOwnershipWithConnectedStandbyAction<Impl: IStreamSocketListener3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, taskid: ::windows::core::GUID, connectedstandbyaction: SocketActivityConnectedStandbyAction) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableTransferOwnershipWithConnectedStandbyAction(&*(&taskid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), connectedstandbyaction).into()
        }
        unsafe extern "system" fn TransferOwnership<Impl: IStreamSocketListener3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferOwnership(&*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransferOwnershipWithContext<Impl: IStreamSocketListener3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, socketid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransferOwnershipWithContext(&*(&socketid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <SocketActivityContext as ::windows::core::Abi>::Abi as *const <SocketActivityContext as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketListener3, BASE_OFFSET>(),
            CancelIOAsync: CancelIOAsync::<Impl, IMPL_OFFSET>,
            EnableTransferOwnership: EnableTransferOwnership::<Impl, IMPL_OFFSET>,
            EnableTransferOwnershipWithConnectedStandbyAction: EnableTransferOwnershipWithConnectedStandbyAction::<Impl, IMPL_OFFSET>,
            TransferOwnership: TransferOwnership::<Impl, IMPL_OFFSET>,
            TransferOwnershipWithContext: TransferOwnershipWithContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListener3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketListenerConnectionReceivedEventArgs_Impl: Sized {
    fn Socket(&mut self) -> ::windows::core::Result<StreamSocket>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreamSocketListenerConnectionReceivedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListenerConnectionReceivedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IStreamSocketListenerConnectionReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListenerConnectionReceivedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListenerConnectionReceivedEventArgs_Vtbl {
        unsafe extern "system" fn Socket<Impl: IStreamSocketListenerConnectionReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketListenerConnectionReceivedEventArgs, BASE_OFFSET>(),
            Socket: Socket::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListenerConnectionReceivedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketListenerControl_Impl: Sized {
    fn QualityOfService(&mut self) -> ::windows::core::Result<SocketQualityOfService>;
    fn SetQualityOfService(&mut self, value: SocketQualityOfService) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreamSocketListenerControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListenerControl";
}
#[cfg(feature = "implement_exclusive")]
impl IStreamSocketListenerControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListenerControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListenerControl_Vtbl {
        unsafe extern "system" fn QualityOfService<Impl: IStreamSocketListenerControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketQualityOfService) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetQualityOfService<Impl: IStreamSocketListenerControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: SocketQualityOfService) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQualityOfService(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketListenerControl, BASE_OFFSET>(),
            QualityOfService: QualityOfService::<Impl, IMPL_OFFSET>,
            SetQualityOfService: SetQualityOfService::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListenerControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketListenerControl2_Impl: Sized {
    fn NoDelay(&mut self) -> ::windows::core::Result<bool>;
    fn SetNoDelay(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn KeepAlive(&mut self) -> ::windows::core::Result<bool>;
    fn SetKeepAlive(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn OutboundBufferSizeInBytes(&mut self) -> ::windows::core::Result<u32>;
    fn SetOutboundBufferSizeInBytes(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn OutboundUnicastHopLimit(&mut self) -> ::windows::core::Result<u8>;
    fn SetOutboundUnicastHopLimit(&mut self, value: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreamSocketListenerControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListenerControl2";
}
#[cfg(feature = "implement_exclusive")]
impl IStreamSocketListenerControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListenerControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListenerControl2_Vtbl {
        unsafe extern "system" fn NoDelay<Impl: IStreamSocketListenerControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNoDelay<Impl: IStreamSocketListenerControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoDelay(value).into()
        }
        unsafe extern "system" fn KeepAlive<Impl: IStreamSocketListenerControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetKeepAlive<Impl: IStreamSocketListenerControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetKeepAlive(value).into()
        }
        unsafe extern "system" fn OutboundBufferSizeInBytes<Impl: IStreamSocketListenerControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOutboundBufferSizeInBytes<Impl: IStreamSocketListenerControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundBufferSizeInBytes(value).into()
        }
        unsafe extern "system" fn OutboundUnicastHopLimit<Impl: IStreamSocketListenerControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOutboundUnicastHopLimit<Impl: IStreamSocketListenerControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundUnicastHopLimit(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketListenerControl2, BASE_OFFSET>(),
            NoDelay: NoDelay::<Impl, IMPL_OFFSET>,
            SetNoDelay: SetNoDelay::<Impl, IMPL_OFFSET>,
            KeepAlive: KeepAlive::<Impl, IMPL_OFFSET>,
            SetKeepAlive: SetKeepAlive::<Impl, IMPL_OFFSET>,
            OutboundBufferSizeInBytes: OutboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            SetOutboundBufferSizeInBytes: SetOutboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            OutboundUnicastHopLimit: OutboundUnicastHopLimit::<Impl, IMPL_OFFSET>,
            SetOutboundUnicastHopLimit: SetOutboundUnicastHopLimit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListenerControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IStreamSocketListenerInformation_Impl: Sized {
    fn LocalPort(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IStreamSocketListenerInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketListenerInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IStreamSocketListenerInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketListenerInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketListenerInformation_Vtbl {
        unsafe extern "system" fn LocalPort<Impl: IStreamSocketListenerInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketListenerInformation, BASE_OFFSET>(),
            LocalPort: LocalPort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketListenerInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IStreamSocketStatics_Impl: Sized {
    fn GetEndpointPairsAsync(&mut self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>;
    fn GetEndpointPairsWithSortOptionsAsync(&mut self, remotehostname: &::core::option::Option<super::HostName>, remoteservicename: &::windows::core::HSTRING, sortoptions: super::HostNameSortOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::EndpointPair>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamSocketStatics {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamSocketStatics";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IStreamSocketStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamSocketStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamSocketStatics_Vtbl {
        unsafe extern "system" fn GetEndpointPairsAsync<Impl: IStreamSocketStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetEndpointPairsWithSortOptionsAsync<Impl: IStreamSocketStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotehostname: ::windows::core::RawPtr, remoteservicename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sortoptions: super::HostNameSortOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamSocketStatics, BASE_OFFSET>(),
            GetEndpointPairsAsync: GetEndpointPairsAsync::<Impl, IMPL_OFFSET>,
            GetEndpointPairsWithSortOptionsAsync: GetEndpointPairsWithSortOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamSocketStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IStreamWebSocket_Impl: Sized + super::super::Foundation::IClosable_Impl + IWebSocket_Impl {
    fn Control(&mut self) -> ::windows::core::Result<StreamWebSocketControl>;
    fn Information(&mut self) -> ::windows::core::Result<StreamWebSocketInformation>;
    fn InputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamWebSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IStreamWebSocket_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamWebSocket_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamWebSocket_Vtbl {
        unsafe extern "system" fn Control<Impl: IStreamWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Information<Impl: IStreamWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn InputStream<Impl: IStreamWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamWebSocket, BASE_OFFSET>(),
            Control: Control::<Impl, IMPL_OFFSET>,
            Information: Information::<Impl, IMPL_OFFSET>,
            InputStream: InputStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamWebSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IStreamWebSocket2_Impl: Sized + super::super::Foundation::IClosable_Impl + IStreamWebSocket_Impl + IWebSocket_Impl {
    fn ServerCustomValidationRequested(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<StreamWebSocket, WebSocketServerCustomValidationRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerCustomValidationRequested(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamWebSocket2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamWebSocket2";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IStreamWebSocket2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamWebSocket2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamWebSocket2_Vtbl {
        unsafe extern "system" fn ServerCustomValidationRequested<Impl: IStreamWebSocket2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveServerCustomValidationRequested<Impl: IStreamWebSocket2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveServerCustomValidationRequested(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamWebSocket2, BASE_OFFSET>(),
            ServerCustomValidationRequested: ServerCustomValidationRequested::<Impl, IMPL_OFFSET>,
            RemoveServerCustomValidationRequested: RemoveServerCustomValidationRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamWebSocket2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IStreamWebSocketControl_Impl: Sized + IWebSocketControl_Impl {
    fn NoDelay(&mut self) -> ::windows::core::Result<bool>;
    fn SetNoDelay(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamWebSocketControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IStreamWebSocketControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamWebSocketControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamWebSocketControl_Vtbl {
        unsafe extern "system" fn NoDelay<Impl: IStreamWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNoDelay<Impl: IStreamWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoDelay(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamWebSocketControl, BASE_OFFSET>(),
            NoDelay: NoDelay::<Impl, IMPL_OFFSET>,
            SetNoDelay: SetNoDelay::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamWebSocketControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IStreamWebSocketControl2_Impl: Sized {
    fn DesiredUnsolicitedPongInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetDesiredUnsolicitedPongInterval(&mut self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn ActualUnsolicitedPongInterval(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn ClientCertificate(&mut self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetClientCertificate(&mut self, value: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IStreamWebSocketControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IStreamWebSocketControl2";
}
#[cfg(all(feature = "Foundation", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IStreamWebSocketControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamWebSocketControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamWebSocketControl2_Vtbl {
        unsafe extern "system" fn DesiredUnsolicitedPongInterval<Impl: IStreamWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredUnsolicitedPongInterval<Impl: IStreamWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDesiredUnsolicitedPongInterval(&*(&value as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ActualUnsolicitedPongInterval<Impl: IStreamWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ClientCertificate<Impl: IStreamWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetClientCertificate<Impl: IStreamWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientCertificate(&*(&value as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IStreamWebSocketControl2, BASE_OFFSET>(),
            DesiredUnsolicitedPongInterval: DesiredUnsolicitedPongInterval::<Impl, IMPL_OFFSET>,
            SetDesiredUnsolicitedPongInterval: SetDesiredUnsolicitedPongInterval::<Impl, IMPL_OFFSET>,
            ActualUnsolicitedPongInterval: ActualUnsolicitedPongInterval::<Impl, IMPL_OFFSET>,
            ClientCertificate: ClientCertificate::<Impl, IMPL_OFFSET>,
            SetClientCertificate: SetClientCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamWebSocketControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IWebSocket_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn OutputStream(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream>;
    fn ConnectAsync(&mut self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn SetRequestHeader(&mut self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Closed(&mut self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<IWebSocket, WebSocketClosedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(&mut self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CloseWithStatus(&mut self, code: u16, reason: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IWebSocket {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocket";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IWebSocket_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocket_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocket_Vtbl {
        unsafe extern "system" fn OutputStream<Impl: IWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectAsync<Impl: IWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRequestHeader<Impl: IWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestHeader(&*(&headername as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&headervalue as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Closed<Impl: IWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveClosed<Impl: IWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&eventcookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CloseWithStatus<Impl: IWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, code: u16, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseWithStatus(code, &*(&reason as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebSocket, BASE_OFFSET>(),
            OutputStream: OutputStream::<Impl, IMPL_OFFSET>,
            ConnectAsync: ConnectAsync::<Impl, IMPL_OFFSET>,
            SetRequestHeader: SetRequestHeader::<Impl, IMPL_OFFSET>,
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
            CloseWithStatus: CloseWithStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWebSocketClosedEventArgs_Impl: Sized {
    fn Code(&mut self) -> ::windows::core::Result<u16>;
    fn Reason(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWebSocketClosedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketClosedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IWebSocketClosedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketClosedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketClosedEventArgs_Vtbl {
        unsafe extern "system" fn Code<Impl: IWebSocketClosedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Reason<Impl: IWebSocketClosedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebSocketClosedEventArgs, BASE_OFFSET>(),
            Code: Code::<Impl, IMPL_OFFSET>,
            Reason: Reason::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketClosedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
pub trait IWebSocketControl_Impl: Sized {
    fn OutboundBufferSizeInBytes(&mut self) -> ::windows::core::Result<u32>;
    fn SetOutboundBufferSizeInBytes(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn ServerCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&mut self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn ProxyCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&mut self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn SupportedProtocols(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
impl ::windows::core::RuntimeName for IWebSocketControl {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketControl";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
impl IWebSocketControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketControl_Vtbl {
        unsafe extern "system" fn OutboundBufferSizeInBytes<Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOutboundBufferSizeInBytes<Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundBufferSizeInBytes(value).into()
        }
        unsafe extern "system" fn ServerCredential<Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetServerCredential<Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServerCredential(&*(&value as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyCredential<Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProxyCredential<Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyCredential(&*(&value as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SupportedProtocols<Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebSocketControl, BASE_OFFSET>(),
            OutboundBufferSizeInBytes: OutboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            SetOutboundBufferSizeInBytes: SetOutboundBufferSizeInBytes::<Impl, IMPL_OFFSET>,
            ServerCredential: ServerCredential::<Impl, IMPL_OFFSET>,
            SetServerCredential: SetServerCredential::<Impl, IMPL_OFFSET>,
            ProxyCredential: ProxyCredential::<Impl, IMPL_OFFSET>,
            SetProxyCredential: SetProxyCredential::<Impl, IMPL_OFFSET>,
            SupportedProtocols: SupportedProtocols::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
pub trait IWebSocketControl2_Impl: Sized + IWebSocketControl_Impl {
    fn IgnorableServerCertificateErrors(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl ::windows::core::RuntimeName for IWebSocketControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketControl2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl IWebSocketControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketControl2_Vtbl {
        unsafe extern "system" fn IgnorableServerCertificateErrors<Impl: IWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebSocketControl2, BASE_OFFSET>(),
            IgnorableServerCertificateErrors: IgnorableServerCertificateErrors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketControl2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Web", feature = "implement_exclusive"))]
pub trait IWebSocketErrorStatics_Impl: Sized {
    fn GetStatus(&mut self, hresult: i32) -> ::windows::core::Result<super::super::Web::WebErrorStatus>;
}
#[cfg(all(feature = "Web", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebSocketErrorStatics {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketErrorStatics";
}
#[cfg(all(feature = "Web", feature = "implement_exclusive"))]
impl IWebSocketErrorStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketErrorStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketErrorStatics_Vtbl {
        unsafe extern "system" fn GetStatus<Impl: IWebSocketErrorStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: i32, result__: *mut super::super::Web::WebErrorStatus) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IWebSocketErrorStatics, BASE_OFFSET>(), GetStatus: GetStatus::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketErrorStatics as ::windows::core::Interface>::IID
    }
}
pub trait IWebSocketInformation_Impl: Sized {
    fn LocalAddress(&mut self) -> ::windows::core::Result<super::HostName>;
    fn BandwidthStatistics(&mut self) -> ::windows::core::Result<BandwidthStatistics>;
    fn Protocol(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketInformation";
}
impl IWebSocketInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketInformation_Vtbl {
        unsafe extern "system" fn LocalAddress<Impl: IWebSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn BandwidthStatistics<Impl: IWebSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Protocol<Impl: IWebSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebSocketInformation, BASE_OFFSET>(),
            LocalAddress: LocalAddress::<Impl, IMPL_OFFSET>,
            BandwidthStatistics: BandwidthStatistics::<Impl, IMPL_OFFSET>,
            Protocol: Protocol::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
pub trait IWebSocketInformation2_Impl: Sized + IWebSocketInformation_Impl {
    fn ServerCertificate(&mut self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerCertificateErrorSeverity(&mut self) -> ::windows::core::Result<SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerIntermediateCertificates(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
impl ::windows::core::RuntimeName for IWebSocketInformation2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketInformation2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
impl IWebSocketInformation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketInformation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketInformation2_Vtbl {
        unsafe extern "system" fn ServerCertificate<Impl: IWebSocketInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServerCertificateErrorSeverity<Impl: IWebSocketInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServerCertificateErrors<Impl: IWebSocketInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServerIntermediateCertificates<Impl: IWebSocketInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebSocketInformation2, BASE_OFFSET>(),
            ServerCertificate: ServerCertificate::<Impl, IMPL_OFFSET>,
            ServerCertificateErrorSeverity: ServerCertificateErrorSeverity::<Impl, IMPL_OFFSET>,
            ServerCertificateErrors: ServerCertificateErrors::<Impl, IMPL_OFFSET>,
            ServerIntermediateCertificates: ServerIntermediateCertificates::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketInformation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IWebSocketServerCustomValidationRequestedEventArgs_Impl: Sized {
    fn ServerCertificate(&mut self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn ServerCertificateErrorSeverity(&mut self) -> ::windows::core::Result<SocketSslErrorSeverity>;
    fn ServerCertificateErrors(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn ServerIntermediateCertificates(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
    fn Reject(&mut self) -> ::windows::core::Result<()>;
    fn GetDeferral(&mut self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWebSocketServerCustomValidationRequestedEventArgs {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketServerCustomValidationRequestedEventArgs";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IWebSocketServerCustomValidationRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketServerCustomValidationRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWebSocketServerCustomValidationRequestedEventArgs_Vtbl {
        unsafe extern "system" fn ServerCertificate<Impl: IWebSocketServerCustomValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServerCertificateErrorSeverity<Impl: IWebSocketServerCustomValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServerCertificateErrors<Impl: IWebSocketServerCustomValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServerIntermediateCertificates<Impl: IWebSocketServerCustomValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Reject<Impl: IWebSocketServerCustomValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reject().into()
        }
        unsafe extern "system" fn GetDeferral<Impl: IWebSocketServerCustomValidationRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebSocketServerCustomValidationRequestedEventArgs, BASE_OFFSET>(),
            ServerCertificate: ServerCertificate::<Impl, IMPL_OFFSET>,
            ServerCertificateErrorSeverity: ServerCertificateErrorSeverity::<Impl, IMPL_OFFSET>,
            ServerCertificateErrors: ServerCertificateErrors::<Impl, IMPL_OFFSET>,
            ServerIntermediateCertificates: ServerIntermediateCertificates::<Impl, IMPL_OFFSET>,
            Reject: Reject::<Impl, IMPL_OFFSET>,
            GetDeferral: GetDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketServerCustomValidationRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
