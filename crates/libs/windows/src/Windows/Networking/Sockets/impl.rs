pub trait IControlChannelTriggerEventDetails_Impl: Sized {
    fn ControlChannelTrigger(&self) -> ::windows::core::Result<ControlChannelTrigger>;
}
impl ::windows::core::RuntimeName for IControlChannelTriggerEventDetails {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTriggerEventDetails";
}
impl IControlChannelTriggerEventDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTriggerEventDetails_Impl, const OFFSET: isize>() -> IControlChannelTriggerEventDetails_Vtbl {
        unsafe extern "system" fn ControlChannelTrigger<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTriggerEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IControlChannelTriggerEventDetails, OFFSET>(),
            ControlChannelTrigger: ControlChannelTrigger::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChannelTriggerEventDetails as ::windows::core::Interface>::IID
    }
}
pub trait IControlChannelTriggerResetEventDetails_Impl: Sized {
    fn ResetReason(&self) -> ::windows::core::Result<ControlChannelTriggerResetReason>;
    fn HardwareSlotReset(&self) -> ::windows::core::Result<bool>;
    fn SoftwareSlotReset(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IControlChannelTriggerResetEventDetails {
    const NAME: &'static str = "Windows.Networking.Sockets.IControlChannelTriggerResetEventDetails";
}
impl IControlChannelTriggerResetEventDetails_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTriggerResetEventDetails_Impl, const OFFSET: isize>() -> IControlChannelTriggerResetEventDetails_Vtbl {
        unsafe extern "system" fn ResetReason<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTriggerResetEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ControlChannelTriggerResetReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResetReason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HardwareSlotReset<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTriggerResetEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).HardwareSlotReset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SoftwareSlotReset<Identity: ::windows::core::IUnknownImpl, Impl: IControlChannelTriggerResetEventDetails_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IControlChannelTriggerResetEventDetails, OFFSET>(),
            ResetReason: ResetReason::<Identity, Impl, OFFSET>,
            HardwareSlotReset: HardwareSlotReset::<Identity, Impl, OFFSET>,
            SoftwareSlotReset: SoftwareSlotReset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChannelTriggerResetEventDetails as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IWebSocket_Impl: Sized + super::super::Foundation::IClosable_Impl {
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
impl IWebSocket_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocket_Impl, const OFFSET: isize>() -> IWebSocket_Vtbl {
        unsafe extern "system" fn OutputStream<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OutputStream() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectAsync<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConnectAsync(::core::mem::transmute(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestHeader<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRequestHeader(::core::mem::transmute(&headername), ::core::mem::transmute(&headervalue)).into()
        }
        unsafe extern "system" fn Closed<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Closed(::core::mem::transmute(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveClosed(::core::mem::transmute(&eventcookie)).into()
        }
        unsafe extern "system" fn CloseWithStatus<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocket_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, code: u16, reason: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseWithStatus(code, ::core::mem::transmute(&reason)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebSocket, OFFSET>(),
            OutputStream: OutputStream::<Identity, Impl, OFFSET>,
            ConnectAsync: ConnectAsync::<Identity, Impl, OFFSET>,
            SetRequestHeader: SetRequestHeader::<Identity, Impl, OFFSET>,
            Closed: Closed::<Identity, Impl, OFFSET>,
            RemoveClosed: RemoveClosed::<Identity, Impl, OFFSET>,
            CloseWithStatus: CloseWithStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocket as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials"))]
pub trait IWebSocketControl_Impl: Sized {
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
impl IWebSocketControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControl_Impl, const OFFSET: isize>() -> IWebSocketControl_Vtbl {
        unsafe extern "system" fn OutboundBufferSizeInBytes<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OutboundBufferSizeInBytes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundBufferSizeInBytes<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutboundBufferSizeInBytes(value).into()
        }
        unsafe extern "system" fn ServerCredential<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServerCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerCredential<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServerCredential(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ProxyCredential<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProxyCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyCredential<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProxyCredential(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SupportedProtocols<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebSocketControl, OFFSET>(),
            OutboundBufferSizeInBytes: OutboundBufferSizeInBytes::<Identity, Impl, OFFSET>,
            SetOutboundBufferSizeInBytes: SetOutboundBufferSizeInBytes::<Identity, Impl, OFFSET>,
            ServerCredential: ServerCredential::<Identity, Impl, OFFSET>,
            SetServerCredential: SetServerCredential::<Identity, Impl, OFFSET>,
            ProxyCredential: ProxyCredential::<Identity, Impl, OFFSET>,
            SetProxyCredential: SetProxyCredential::<Identity, Impl, OFFSET>,
            SupportedProtocols: SupportedProtocols::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
pub trait IWebSocketControl2_Impl: Sized + IWebSocketControl_Impl {
    fn IgnorableServerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl ::windows::core::RuntimeName for IWebSocketControl2 {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketControl2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl IWebSocketControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControl2_Impl, const OFFSET: isize>() -> IWebSocketControl2_Vtbl {
        unsafe extern "system" fn IgnorableServerCertificateErrors<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebSocketControl2, OFFSET>(),
            IgnorableServerCertificateErrors: IgnorableServerCertificateErrors::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketControl2 as ::windows::core::Interface>::IID
    }
}
pub trait IWebSocketInformation_Impl: Sized {
    fn LocalAddress(&self) -> ::windows::core::Result<super::HostName>;
    fn BandwidthStatistics(&self) -> ::windows::core::Result<BandwidthStatistics>;
    fn Protocol(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IWebSocketInformation {
    const NAME: &'static str = "Windows.Networking.Sockets.IWebSocketInformation";
}
impl IWebSocketInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketInformation_Impl, const OFFSET: isize>() -> IWebSocketInformation_Vtbl {
        unsafe extern "system" fn LocalAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LocalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BandwidthStatistics<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BandwidthStatistics) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BandwidthStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebSocketInformation, OFFSET>(),
            LocalAddress: LocalAddress::<Identity, Impl, OFFSET>,
            BandwidthStatistics: BandwidthStatistics::<Identity, Impl, OFFSET>,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates"))]
pub trait IWebSocketInformation2_Impl: Sized + IWebSocketInformation_Impl {
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
impl IWebSocketInformation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketInformation2_Impl, const OFFSET: isize>() -> IWebSocketInformation2_Vtbl {
        unsafe extern "system" fn ServerCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServerCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificateErrorSeverity<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SocketSslErrorSeverity) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServerCertificateErrorSeverity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerCertificateErrors<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServerCertificateErrors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerIntermediateCertificates<Identity: ::windows::core::IUnknownImpl, Impl: IWebSocketInformation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWebSocketInformation2, OFFSET>(),
            ServerCertificate: ServerCertificate::<Identity, Impl, OFFSET>,
            ServerCertificateErrorSeverity: ServerCertificateErrorSeverity::<Identity, Impl, OFFSET>,
            ServerCertificateErrors: ServerCertificateErrors::<Identity, Impl, OFFSET>,
            ServerIntermediateCertificates: ServerIntermediateCertificates::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWebSocketInformation2 as ::windows::core::Interface>::IID
    }
}
