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
