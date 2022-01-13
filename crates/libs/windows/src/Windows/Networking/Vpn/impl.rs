#[cfg(feature = "implement_exclusive")]
pub trait IVpnAppIdImpl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<VpnAppIdType>;
    fn SetType(&mut self, value: VpnAppIdType) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnAppId {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnAppId";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnAppIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnAppIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnAppIdVtbl {
        unsafe extern "system" fn Type<Impl: IVpnAppIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnAppIdType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IVpnAppIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnAppIdType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn Value<Impl: IVpnAppIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IVpnAppIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnAppId, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnAppId as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnAppIdFactoryImpl: Sized {
    fn Create(&mut self, r#type: VpnAppIdType, value: &::windows::core::HSTRING) -> ::windows::core::Result<VpnAppId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnAppIdFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnAppIdFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnAppIdFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnAppIdFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnAppIdFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IVpnAppIdFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VpnAppIdType, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(r#type, &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnAppIdFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnAppIdFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IVpnChannelImpl: Sized {
    fn AssociateTransport(&mut self, mainoutertunneltransport: &::core::option::Option<::windows::core::IInspectable>, optionaloutertunneltransport: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Start(&mut self, assignedclientipv4list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, assignedclientipv6list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, routescope: &::core::option::Option<VpnRouteAssignment>, namespacescope: &::core::option::Option<VpnNamespaceAssignment>, mtusize: u32, maxframesize: u32, optimizeforlowcostnetwork: bool, mainoutertunneltransport: &::core::option::Option<::windows::core::IInspectable>, optionaloutertunneltransport: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn RequestCredentials(&mut self, credtype: VpnCredentialType, isretry: bool, issinglesignoncredential: bool, certificate: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<VpnPickedCredential>;
    fn RequestVpnPacketBuffer(&mut self, r#type: VpnDataPathType, vpnpacketbuffer: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn LogDiagnosticMessage(&mut self, message: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn Configuration(&mut self) -> ::windows::core::Result<VpnChannelConfiguration>;
    fn ActivityChange(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivityChange(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetPlugInContext(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn PlugInContext(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SystemHealth(&mut self) -> ::windows::core::Result<VpnSystemHealth>;
    fn RequestCustomPrompt(&mut self, customprompt: &::core::option::Option<super::super::Foundation::Collections::IVectorView<IVpnCustomPrompt>>) -> ::windows::core::Result<()>;
    fn SetErrorMessage(&mut self, message: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetAllowedSslTlsVersions(&mut self, tunneltransport: &::core::option::Option<::windows::core::IInspectable>, usetls12: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnChannel {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannel";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IVpnChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannelVtbl {
        unsafe extern "system" fn AssociateTransport<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AssociateTransport(&*(&mainoutertunneltransport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&optionaloutertunneltransport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, routescope: ::windows::core::RawPtr, namespacescope: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, optimizeforlowcostnetwork: bool, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .Start(
                    &*(&assignedclientipv4list as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                    &*(&assignedclientipv6list as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                    &*(&vpninterfaceid as *const <VpnInterfaceId as ::windows::core::Abi>::Abi as *const <VpnInterfaceId as ::windows::core::DefaultType>::DefaultType),
                    &*(&routescope as *const <VpnRouteAssignment as ::windows::core::Abi>::Abi as *const <VpnRouteAssignment as ::windows::core::DefaultType>::DefaultType),
                    &*(&namespacescope as *const <VpnNamespaceAssignment as ::windows::core::Abi>::Abi as *const <VpnNamespaceAssignment as ::windows::core::DefaultType>::DefaultType),
                    mtusize,
                    maxframesize,
                    optimizeforlowcostnetwork,
                    &*(&mainoutertunneltransport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&optionaloutertunneltransport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn Stop<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn RequestCredentials<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, isretry: bool, issinglesignoncredential: bool, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestCredentials(credtype, isretry, issinglesignoncredential, &*(&certificate as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestVpnPacketBuffer<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VpnDataPathType, vpnpacketbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestVpnPacketBuffer(r#type, ::core::mem::transmute_copy(&vpnpacketbuffer)).into()
        }
        unsafe extern "system" fn LogDiagnosticMessage<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogDiagnosticMessage(&*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Configuration<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityChange<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityChange(&*(&handler as *const <super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivityChange<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActivityChange(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetPlugInContext<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlugInContext(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlugInContext<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlugInContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemHealth<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SystemHealth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCustomPrompt<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customprompt: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestCustomPrompt(&*(&customprompt as *const <super::super::Foundation::Collections::IVectorView<IVpnCustomPrompt> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<IVpnCustomPrompt> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetErrorMessage<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorMessage(&*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetAllowedSslTlsVersions<Impl: IVpnChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tunneltransport: *mut ::core::ffi::c_void, usetls12: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowedSslTlsVersions(&*(&tunneltransport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), usetls12).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnChannel, BASE_OFFSET>(),
            AssociateTransport: AssociateTransport::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            RequestCredentials: RequestCredentials::<Impl, IMPL_OFFSET>,
            RequestVpnPacketBuffer: RequestVpnPacketBuffer::<Impl, IMPL_OFFSET>,
            LogDiagnosticMessage: LogDiagnosticMessage::<Impl, IMPL_OFFSET>,
            Id: Id::<Impl, IMPL_OFFSET>,
            Configuration: Configuration::<Impl, IMPL_OFFSET>,
            ActivityChange: ActivityChange::<Impl, IMPL_OFFSET>,
            RemoveActivityChange: RemoveActivityChange::<Impl, IMPL_OFFSET>,
            SetPlugInContext: SetPlugInContext::<Impl, IMPL_OFFSET>,
            PlugInContext: PlugInContext::<Impl, IMPL_OFFSET>,
            SystemHealth: SystemHealth::<Impl, IMPL_OFFSET>,
            RequestCustomPrompt: RequestCustomPrompt::<Impl, IMPL_OFFSET>,
            SetErrorMessage: SetErrorMessage::<Impl, IMPL_OFFSET>,
            SetAllowedSslTlsVersions: SetAllowedSslTlsVersions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
pub trait IVpnChannel2Impl: Sized {
    fn StartWithMainTransport(&mut self, assignedclientipv4list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, assignedclientipv6list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, assignedroutes: &::core::option::Option<VpnRouteAssignment>, assigneddomainname: &::core::option::Option<VpnDomainNameAssignment>, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn StartExistingTransports(&mut self, assignedclientipv4list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, assignedclientipv6list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, assignedroutes: &::core::option::Option<VpnRouteAssignment>, assigneddomainname: &::core::option::Option<VpnDomainNameAssignment>, mtusize: u32, maxframesize: u32, reserved: bool) -> ::windows::core::Result<()>;
    fn ActivityStateChange(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityStateChangedArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivityStateChange(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetVpnSendPacketBuffer(&mut self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn GetVpnReceivePacketBuffer(&mut self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn RequestCustomPromptAsync(&mut self, custompromptelement: &::core::option::Option<super::super::Foundation::Collections::IVectorView<IVpnCustomPromptElement>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestCredentialsWithCertificateAsync(&mut self, credtype: VpnCredentialType, credoptions: u32, certificate: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>>;
    fn RequestCredentialsWithOptionsAsync(&mut self, credtype: VpnCredentialType, credoptions: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>>;
    fn RequestCredentialsSimpleAsync(&mut self, credtype: VpnCredentialType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>>;
    fn TerminateConnection(&mut self, message: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StartWithTrafficFilter(&mut self, assignedclientipv4list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, assignedclientipv6list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, assignedroutes: &::core::option::Option<VpnRouteAssignment>, assignednamespace: &::core::option::Option<VpnDomainNameAssignment>, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: &::core::option::Option<::windows::core::IInspectable>, optionaloutertunneltransport: &::core::option::Option<::windows::core::IInspectable>, assignedtrafficfilters: &::core::option::Option<VpnTrafficFilterAssignment>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnChannel2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannel2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Security_Cryptography_Certificates", feature = "implement_exclusive"))]
impl IVpnChannel2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannel2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannel2Vtbl {
        unsafe extern "system" fn StartWithMainTransport<Impl: IVpnChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assigneddomainname: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .StartWithMainTransport(
                    &*(&assignedclientipv4list as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                    &*(&assignedclientipv6list as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                    &*(&vpninterfaceid as *const <VpnInterfaceId as ::windows::core::Abi>::Abi as *const <VpnInterfaceId as ::windows::core::DefaultType>::DefaultType),
                    &*(&assignedroutes as *const <VpnRouteAssignment as ::windows::core::Abi>::Abi as *const <VpnRouteAssignment as ::windows::core::DefaultType>::DefaultType),
                    &*(&assigneddomainname as *const <VpnDomainNameAssignment as ::windows::core::Abi>::Abi as *const <VpnDomainNameAssignment as ::windows::core::DefaultType>::DefaultType),
                    mtusize,
                    maxframesize,
                    reserved,
                    &*(&mainoutertunneltransport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn StartExistingTransports<Impl: IVpnChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assigneddomainname: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .StartExistingTransports(
                    &*(&assignedclientipv4list as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                    &*(&assignedclientipv6list as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                    &*(&vpninterfaceid as *const <VpnInterfaceId as ::windows::core::Abi>::Abi as *const <VpnInterfaceId as ::windows::core::DefaultType>::DefaultType),
                    &*(&assignedroutes as *const <VpnRouteAssignment as ::windows::core::Abi>::Abi as *const <VpnRouteAssignment as ::windows::core::DefaultType>::DefaultType),
                    &*(&assigneddomainname as *const <VpnDomainNameAssignment as ::windows::core::Abi>::Abi as *const <VpnDomainNameAssignment as ::windows::core::DefaultType>::DefaultType),
                    mtusize,
                    maxframesize,
                    reserved,
                )
                .into()
        }
        unsafe extern "system" fn ActivityStateChange<Impl: IVpnChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityStateChange(&*(&handler as *const <super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityStateChangedArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityStateChangedArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivityStateChange<Impl: IVpnChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActivityStateChange(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetVpnSendPacketBuffer<Impl: IVpnChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVpnSendPacketBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVpnReceivePacketBuffer<Impl: IVpnChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVpnReceivePacketBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCustomPromptAsync<Impl: IVpnChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, custompromptelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestCustomPromptAsync(&*(&custompromptelement as *const <super::super::Foundation::Collections::IVectorView<IVpnCustomPromptElement> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<IVpnCustomPromptElement> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCredentialsWithCertificateAsync<Impl: IVpnChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, credoptions: u32, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestCredentialsWithCertificateAsync(credtype, credoptions, &*(&certificate as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCredentialsWithOptionsAsync<Impl: IVpnChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, credoptions: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestCredentialsWithOptionsAsync(credtype, credoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCredentialsSimpleAsync<Impl: IVpnChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestCredentialsSimpleAsync(credtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateConnection<Impl: IVpnChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TerminateConnection(&*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartWithTrafficFilter<Impl: IVpnChannel2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assignednamespace: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void, assignedtrafficfilters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .StartWithTrafficFilter(
                    &*(&assignedclientipv4list as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                    &*(&assignedclientipv6list as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                    &*(&vpninterfaceid as *const <VpnInterfaceId as ::windows::core::Abi>::Abi as *const <VpnInterfaceId as ::windows::core::DefaultType>::DefaultType),
                    &*(&assignedroutes as *const <VpnRouteAssignment as ::windows::core::Abi>::Abi as *const <VpnRouteAssignment as ::windows::core::DefaultType>::DefaultType),
                    &*(&assignednamespace as *const <VpnDomainNameAssignment as ::windows::core::Abi>::Abi as *const <VpnDomainNameAssignment as ::windows::core::DefaultType>::DefaultType),
                    mtusize,
                    maxframesize,
                    reserved,
                    &*(&mainoutertunneltransport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&optionaloutertunneltransport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&assignedtrafficfilters as *const <VpnTrafficFilterAssignment as ::windows::core::Abi>::Abi as *const <VpnTrafficFilterAssignment as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnChannel2, BASE_OFFSET>(),
            StartWithMainTransport: StartWithMainTransport::<Impl, IMPL_OFFSET>,
            StartExistingTransports: StartExistingTransports::<Impl, IMPL_OFFSET>,
            ActivityStateChange: ActivityStateChange::<Impl, IMPL_OFFSET>,
            RemoveActivityStateChange: RemoveActivityStateChange::<Impl, IMPL_OFFSET>,
            GetVpnSendPacketBuffer: GetVpnSendPacketBuffer::<Impl, IMPL_OFFSET>,
            GetVpnReceivePacketBuffer: GetVpnReceivePacketBuffer::<Impl, IMPL_OFFSET>,
            RequestCustomPromptAsync: RequestCustomPromptAsync::<Impl, IMPL_OFFSET>,
            RequestCredentialsWithCertificateAsync: RequestCredentialsWithCertificateAsync::<Impl, IMPL_OFFSET>,
            RequestCredentialsWithOptionsAsync: RequestCredentialsWithOptionsAsync::<Impl, IMPL_OFFSET>,
            RequestCredentialsSimpleAsync: RequestCredentialsSimpleAsync::<Impl, IMPL_OFFSET>,
            TerminateConnection: TerminateConnection::<Impl, IMPL_OFFSET>,
            StartWithTrafficFilter: StartWithTrafficFilter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnChannel2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "implement_exclusive"))]
pub trait IVpnChannel4Impl: Sized {
    fn AddAndAssociateTransport(&mut self, transport: &::core::option::Option<::windows::core::IInspectable>, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn StartWithMultipleTransports(&mut self, assignedclientipv4addresses: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>, assignedclientipv6addresses: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, assignedroutes: &::core::option::Option<VpnRouteAssignment>, assignednamespace: &::core::option::Option<VpnDomainNameAssignment>, mtusize: u32, maxframesize: u32, reserved: bool, transports: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>, assignedtrafficfilters: &::core::option::Option<VpnTrafficFilterAssignment>) -> ::windows::core::Result<()>;
    fn ReplaceAndAssociateTransport(&mut self, transport: &::core::option::Option<::windows::core::IInspectable>, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn StartReconnectingTransport(&mut self, transport: &::core::option::Option<::windows::core::IInspectable>, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetSlotTypeForTransportContext(&mut self, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::Sockets::ControlChannelTriggerStatus>;
    fn CurrentRequestTransportContext(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnChannel4 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannel4";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Networking_Sockets", feature = "implement_exclusive"))]
impl IVpnChannel4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannel4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannel4Vtbl {
        unsafe extern "system" fn AddAndAssociateTransport<Impl: IVpnChannel4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAndAssociateTransport(&*(&transport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartWithMultipleTransports<Impl: IVpnChannel4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assignedclientipv4addresses: ::windows::core::RawPtr, assignedclientipv6addresses: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assignednamespace: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool, transports: ::windows::core::RawPtr, assignedtrafficfilters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .StartWithMultipleTransports(
                    &*(&assignedclientipv4addresses as *const <super::super::Foundation::Collections::IIterable<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                    &*(&assignedclientipv6addresses as *const <super::super::Foundation::Collections::IIterable<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                    &*(&vpninterfaceid as *const <VpnInterfaceId as ::windows::core::Abi>::Abi as *const <VpnInterfaceId as ::windows::core::DefaultType>::DefaultType),
                    &*(&assignedroutes as *const <VpnRouteAssignment as ::windows::core::Abi>::Abi as *const <VpnRouteAssignment as ::windows::core::DefaultType>::DefaultType),
                    &*(&assignednamespace as *const <VpnDomainNameAssignment as ::windows::core::Abi>::Abi as *const <VpnDomainNameAssignment as ::windows::core::DefaultType>::DefaultType),
                    mtusize,
                    maxframesize,
                    reserved,
                    &*(&transports as *const <super::super::Foundation::Collections::IIterable<::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType),
                    &*(&assignedtrafficfilters as *const <VpnTrafficFilterAssignment as ::windows::core::Abi>::Abi as *const <VpnTrafficFilterAssignment as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn ReplaceAndAssociateTransport<Impl: IVpnChannel4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReplaceAndAssociateTransport(&*(&transport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartReconnectingTransport<Impl: IVpnChannel4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartReconnectingTransport(&*(&transport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetSlotTypeForTransportContext<Impl: IVpnChannel4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut super::Sockets::ControlChannelTriggerStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSlotTypeForTransportContext(&*(&context as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRequestTransportContext<Impl: IVpnChannel4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRequestTransportContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnChannel4, BASE_OFFSET>(),
            AddAndAssociateTransport: AddAndAssociateTransport::<Impl, IMPL_OFFSET>,
            StartWithMultipleTransports: StartWithMultipleTransports::<Impl, IMPL_OFFSET>,
            ReplaceAndAssociateTransport: ReplaceAndAssociateTransport::<Impl, IMPL_OFFSET>,
            StartReconnectingTransport: StartReconnectingTransport::<Impl, IMPL_OFFSET>,
            GetSlotTypeForTransportContext: GetSlotTypeForTransportContext::<Impl, IMPL_OFFSET>,
            CurrentRequestTransportContext: CurrentRequestTransportContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnChannel4 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannel5Impl: Sized {
    fn AppendVpnReceivePacketBuffer(&mut self, decapsulatedpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn AppendVpnSendPacketBuffer(&mut self, encapsulatedpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn FlushVpnReceivePacketBuffers(&mut self) -> ::windows::core::Result<()>;
    fn FlushVpnSendPacketBuffers(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannel5 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannel5";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannel5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannel5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannel5Vtbl {
        unsafe extern "system" fn AppendVpnReceivePacketBuffer<Impl: IVpnChannel5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, decapsulatedpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AppendVpnReceivePacketBuffer(&*(&decapsulatedpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendVpnSendPacketBuffer<Impl: IVpnChannel5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encapsulatedpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AppendVpnSendPacketBuffer(&*(&encapsulatedpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FlushVpnReceivePacketBuffers<Impl: IVpnChannel5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushVpnReceivePacketBuffers().into()
        }
        unsafe extern "system" fn FlushVpnSendPacketBuffers<Impl: IVpnChannel5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushVpnSendPacketBuffers().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnChannel5, BASE_OFFSET>(),
            AppendVpnReceivePacketBuffer: AppendVpnReceivePacketBuffer::<Impl, IMPL_OFFSET>,
            AppendVpnSendPacketBuffer: AppendVpnSendPacketBuffer::<Impl, IMPL_OFFSET>,
            FlushVpnReceivePacketBuffers: FlushVpnReceivePacketBuffers::<Impl, IMPL_OFFSET>,
            FlushVpnSendPacketBuffers: FlushVpnSendPacketBuffers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnChannel5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnChannel6Impl: Sized {
    fn ActivateForeground(&mut self, packagerelativeappid: &::windows::core::HSTRING, sharedcontext: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnChannel6 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannel6";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnChannel6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannel6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannel6Vtbl {
        unsafe extern "system" fn ActivateForeground<Impl: IVpnChannel6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagerelativeappid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sharedcontext: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateForeground(&*(&packagerelativeappid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&sharedcontext as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnChannel6, BASE_OFFSET>(),
            ActivateForeground: ActivateForeground::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnChannel6 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannelActivityEventArgsImpl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<VpnChannelActivityEventType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannelActivityEventArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelActivityEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannelActivityEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelActivityEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannelActivityEventArgsVtbl {
        unsafe extern "system" fn Type<Impl: IVpnChannelActivityEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnChannelActivityEventType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnChannelActivityEventArgs, BASE_OFFSET>(), Type: Type::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnChannelActivityEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannelActivityStateChangedArgsImpl: Sized {
    fn ActivityState(&mut self) -> ::windows::core::Result<VpnChannelActivityEventType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannelActivityStateChangedArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelActivityStateChangedArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannelActivityStateChangedArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelActivityStateChangedArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannelActivityStateChangedArgsVtbl {
        unsafe extern "system" fn ActivityState<Impl: IVpnChannelActivityStateChangedArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnChannelActivityEventType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnChannelActivityStateChangedArgs, BASE_OFFSET>(),
            ActivityState: ActivityState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnChannelActivityStateChangedArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnChannelConfigurationImpl: Sized {
    fn ServerServiceName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServerHostNameList(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>>;
    fn CustomField(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnChannelConfiguration {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelConfiguration";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnChannelConfigurationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelConfigurationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannelConfigurationVtbl {
        unsafe extern "system" fn ServerServiceName<Impl: IVpnChannelConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerHostNameList<Impl: IVpnChannelConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerHostNameList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomField<Impl: IVpnChannelConfigurationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomField() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnChannelConfiguration, BASE_OFFSET>(),
            ServerServiceName: ServerServiceName::<Impl, IMPL_OFFSET>,
            ServerHostNameList: ServerHostNameList::<Impl, IMPL_OFFSET>,
            CustomField: CustomField::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnChannelConfiguration as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnChannelConfiguration2Impl: Sized {
    fn ServerUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnChannelConfiguration2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelConfiguration2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnChannelConfiguration2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelConfiguration2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannelConfiguration2Vtbl {
        unsafe extern "system" fn ServerUris<Impl: IVpnChannelConfiguration2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnChannelConfiguration2, BASE_OFFSET>(), ServerUris: ServerUris::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnChannelConfiguration2 as ::windows::core::Interface>::IID
    }
}
pub trait IVpnChannelStaticsImpl: Sized {
    fn ProcessEventAsync(&mut self, thirdpartyplugin: &::core::option::Option<::windows::core::IInspectable>, event: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVpnChannelStatics {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelStatics";
}
impl IVpnChannelStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannelStaticsVtbl {
        unsafe extern "system" fn ProcessEventAsync<Impl: IVpnChannelStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thirdpartyplugin: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessEventAsync(&*(&thirdpartyplugin as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&event as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnChannelStatics, BASE_OFFSET>(),
            ProcessEventAsync: ProcessEventAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnChannelStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
pub trait IVpnCredentialImpl: Sized {
    fn PasskeyCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn CertificateCredential(&mut self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn AdditionalPin(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OldPasswordCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl ::windows::core::RuntimeName for IVpnCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCredential";
}
#[cfg(all(feature = "Security_Credentials", feature = "Security_Cryptography_Certificates"))]
impl IVpnCredentialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCredentialImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCredentialVtbl {
        unsafe extern "system" fn PasskeyCredential<Impl: IVpnCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasskeyCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CertificateCredential<Impl: IVpnCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CertificateCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdditionalPin<Impl: IVpnCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdditionalPin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldPasswordCredential<Impl: IVpnCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldPasswordCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCredential, BASE_OFFSET>(),
            PasskeyCredential: PasskeyCredential::<Impl, IMPL_OFFSET>,
            CertificateCredential: CertificateCredential::<Impl, IMPL_OFFSET>,
            AdditionalPin: AdditionalPin::<Impl, IMPL_OFFSET>,
            OldPasswordCredential: OldPasswordCredential::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCredential as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomCheckBoxImpl: Sized + IVpnCustomPromptImpl {
    fn SetInitialCheckState(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InitialCheckState(&mut self) -> ::windows::core::Result<bool>;
    fn Checked(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomCheckBox {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomCheckBox";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomCheckBoxVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomCheckBoxImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomCheckBoxVtbl {
        unsafe extern "system" fn SetInitialCheckState<Impl: IVpnCustomCheckBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialCheckState(value).into()
        }
        unsafe extern "system" fn InitialCheckState<Impl: IVpnCustomCheckBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialCheckState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Checked<Impl: IVpnCustomCheckBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Checked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomCheckBox, BASE_OFFSET>(),
            SetInitialCheckState: SetInitialCheckState::<Impl, IMPL_OFFSET>,
            InitialCheckState: InitialCheckState::<Impl, IMPL_OFFSET>,
            Checked: Checked::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomCheckBox as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnCustomComboBoxImpl: Sized + IVpnCustomPromptImpl {
    fn SetOptionsText(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn OptionsText(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Selected(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnCustomComboBox {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomComboBox";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnCustomComboBoxVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomComboBoxImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomComboBoxVtbl {
        unsafe extern "system" fn SetOptionsText<Impl: IVpnCustomComboBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOptionsText(&*(&value as *const <super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OptionsText<Impl: IVpnCustomComboBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OptionsText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Selected<Impl: IVpnCustomComboBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Selected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomComboBox, BASE_OFFSET>(),
            SetOptionsText: SetOptionsText::<Impl, IMPL_OFFSET>,
            OptionsText: OptionsText::<Impl, IMPL_OFFSET>,
            Selected: Selected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomComboBox as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomEditBoxImpl: Sized + IVpnCustomPromptImpl {
    fn SetDefaultText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DefaultText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNoEcho(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn NoEcho(&mut self) -> ::windows::core::Result<bool>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomEditBox {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomEditBox";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomEditBoxVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomEditBoxImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomEditBoxVtbl {
        unsafe extern "system" fn SetDefaultText<Impl: IVpnCustomEditBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultText<Impl: IVpnCustomEditBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DefaultText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoEcho<Impl: IVpnCustomEditBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoEcho(value).into()
        }
        unsafe extern "system" fn NoEcho<Impl: IVpnCustomEditBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NoEcho() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: IVpnCustomEditBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomEditBox, BASE_OFFSET>(),
            SetDefaultText: SetDefaultText::<Impl, IMPL_OFFSET>,
            DefaultText: DefaultText::<Impl, IMPL_OFFSET>,
            SetNoEcho: SetNoEcho::<Impl, IMPL_OFFSET>,
            NoEcho: NoEcho::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomEditBox as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomErrorBoxImpl: Sized + IVpnCustomPromptImpl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomErrorBox {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomErrorBox";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomErrorBoxVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomErrorBoxImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomErrorBoxVtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomErrorBox, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomErrorBox as ::windows::core::Interface>::IID
    }
}
pub trait IVpnCustomPromptImpl: Sized {
    fn SetLabel(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Label(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompulsory(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Compulsory(&mut self) -> ::windows::core::Result<bool>;
    fn SetBordered(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Bordered(&mut self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IVpnCustomPrompt {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPrompt";
}
impl IVpnCustomPromptVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPromptVtbl {
        unsafe extern "system" fn SetLabel<Impl: IVpnCustomPromptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Label<Impl: IVpnCustomPromptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompulsory<Impl: IVpnCustomPromptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompulsory(value).into()
        }
        unsafe extern "system" fn Compulsory<Impl: IVpnCustomPromptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compulsory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBordered<Impl: IVpnCustomPromptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBordered(value).into()
        }
        unsafe extern "system" fn Bordered<Impl: IVpnCustomPromptImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Bordered() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomPrompt, BASE_OFFSET>(),
            SetLabel: SetLabel::<Impl, IMPL_OFFSET>,
            Label: Label::<Impl, IMPL_OFFSET>,
            SetCompulsory: SetCompulsory::<Impl, IMPL_OFFSET>,
            Compulsory: Compulsory::<Impl, IMPL_OFFSET>,
            SetBordered: SetBordered::<Impl, IMPL_OFFSET>,
            Bordered: Bordered::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomPrompt as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomPromptBooleanInputImpl: Sized + IVpnCustomPromptElementImpl {
    fn SetInitialValue(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InitialValue(&mut self) -> ::windows::core::Result<bool>;
    fn Value(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomPromptBooleanInput {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptBooleanInput";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomPromptBooleanInputVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptBooleanInputImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPromptBooleanInputVtbl {
        unsafe extern "system" fn SetInitialValue<Impl: IVpnCustomPromptBooleanInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialValue(value).into()
        }
        unsafe extern "system" fn InitialValue<Impl: IVpnCustomPromptBooleanInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitialValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IVpnCustomPromptBooleanInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomPromptBooleanInput, BASE_OFFSET>(),
            SetInitialValue: SetInitialValue::<Impl, IMPL_OFFSET>,
            InitialValue: InitialValue::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomPromptBooleanInput as ::windows::core::Interface>::IID
    }
}
pub trait IVpnCustomPromptElementImpl: Sized {
    fn SetDisplayName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompulsory(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Compulsory(&mut self) -> ::windows::core::Result<bool>;
    fn SetEmphasized(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn Emphasized(&mut self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IVpnCustomPromptElement {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptElement";
}
impl IVpnCustomPromptElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptElementImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPromptElementVtbl {
        unsafe extern "system" fn SetDisplayName<Impl: IVpnCustomPromptElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IVpnCustomPromptElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompulsory<Impl: IVpnCustomPromptElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompulsory(value).into()
        }
        unsafe extern "system" fn Compulsory<Impl: IVpnCustomPromptElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Compulsory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmphasized<Impl: IVpnCustomPromptElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmphasized(value).into()
        }
        unsafe extern "system" fn Emphasized<Impl: IVpnCustomPromptElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Emphasized() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomPromptElement, BASE_OFFSET>(),
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            SetCompulsory: SetCompulsory::<Impl, IMPL_OFFSET>,
            Compulsory: Compulsory::<Impl, IMPL_OFFSET>,
            SetEmphasized: SetEmphasized::<Impl, IMPL_OFFSET>,
            Emphasized: Emphasized::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomPromptElement as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnCustomPromptOptionSelectorImpl: Sized + IVpnCustomPromptElementImpl {
    fn Options(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SelectedIndex(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnCustomPromptOptionSelector {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptOptionSelector";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnCustomPromptOptionSelectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptOptionSelectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPromptOptionSelectorVtbl {
        unsafe extern "system" fn Options<Impl: IVpnCustomPromptOptionSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedIndex<Impl: IVpnCustomPromptOptionSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectedIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomPromptOptionSelector, BASE_OFFSET>(),
            Options: Options::<Impl, IMPL_OFFSET>,
            SelectedIndex: SelectedIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomPromptOptionSelector as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomPromptTextImpl: Sized + IVpnCustomPromptElementImpl {
    fn SetText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomPromptText {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptText";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomPromptTextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptTextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPromptTextVtbl {
        unsafe extern "system" fn SetText<Impl: IVpnCustomPromptTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Text<Impl: IVpnCustomPromptTextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomPromptText, BASE_OFFSET>(),
            SetText: SetText::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomPromptText as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomPromptTextInputImpl: Sized + IVpnCustomPromptElementImpl {
    fn SetPlaceholderText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PlaceholderText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIsTextHidden(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn IsTextHidden(&mut self) -> ::windows::core::Result<bool>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomPromptTextInput {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptTextInput";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomPromptTextInputVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptTextInputImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPromptTextInputVtbl {
        unsafe extern "system" fn SetPlaceholderText<Impl: IVpnCustomPromptTextInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaceholderText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaceholderText<Impl: IVpnCustomPromptTextInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlaceholderText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTextHidden<Impl: IVpnCustomPromptTextInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTextHidden(value).into()
        }
        unsafe extern "system" fn IsTextHidden<Impl: IVpnCustomPromptTextInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTextHidden() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: IVpnCustomPromptTextInputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomPromptTextInput, BASE_OFFSET>(),
            SetPlaceholderText: SetPlaceholderText::<Impl, IMPL_OFFSET>,
            PlaceholderText: PlaceholderText::<Impl, IMPL_OFFSET>,
            SetIsTextHidden: SetIsTextHidden::<Impl, IMPL_OFFSET>,
            IsTextHidden: IsTextHidden::<Impl, IMPL_OFFSET>,
            Text: Text::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomPromptTextInput as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomTextBoxImpl: Sized + IVpnCustomPromptImpl {
    fn SetDisplayText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomTextBox {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomTextBox";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomTextBoxVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomTextBoxImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomTextBoxVtbl {
        unsafe extern "system" fn SetDisplayText<Impl: IVpnCustomTextBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayText<Impl: IVpnCustomTextBoxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomTextBox, BASE_OFFSET>(),
            SetDisplayText: SetDisplayText::<Impl, IMPL_OFFSET>,
            DisplayText: DisplayText::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomTextBox as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnDomainNameAssignmentImpl: Sized {
    fn DomainNameList(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>;
    fn SetProxyAutoConfigurationUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ProxyAutoConfigurationUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnDomainNameAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameAssignment";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnDomainNameAssignmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnDomainNameAssignmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnDomainNameAssignmentVtbl {
        unsafe extern "system" fn DomainNameList<Impl: IVpnDomainNameAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainNameList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyAutoConfigurationUri<Impl: IVpnDomainNameAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyAutoConfigurationUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyAutoConfigurationUri<Impl: IVpnDomainNameAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyAutoConfigurationUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnDomainNameAssignment, BASE_OFFSET>(),
            DomainNameList: DomainNameList::<Impl, IMPL_OFFSET>,
            SetProxyAutoConfigurationUri: SetProxyAutoConfigurationUri::<Impl, IMPL_OFFSET>,
            ProxyAutoConfigurationUri: ProxyAutoConfigurationUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnDomainNameAssignment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnDomainNameInfoImpl: Sized {
    fn SetDomainName(&mut self, value: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn DomainName(&mut self) -> ::windows::core::Result<super::HostName>;
    fn SetDomainNameType(&mut self, value: VpnDomainNameType) -> ::windows::core::Result<()>;
    fn DomainNameType(&mut self) -> ::windows::core::Result<VpnDomainNameType>;
    fn DnsServers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>>;
    fn WebProxyServers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnDomainNameInfo {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnDomainNameInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnDomainNameInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnDomainNameInfoVtbl {
        unsafe extern "system" fn SetDomainName<Impl: IVpnDomainNameInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomainName(&*(&value as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DomainName<Impl: IVpnDomainNameInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomainNameType<Impl: IVpnDomainNameInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnDomainNameType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomainNameType(value).into()
        }
        unsafe extern "system" fn DomainNameType<Impl: IVpnDomainNameInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnDomainNameType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainNameType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DnsServers<Impl: IVpnDomainNameInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DnsServers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebProxyServers<Impl: IVpnDomainNameInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebProxyServers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnDomainNameInfo, BASE_OFFSET>(),
            SetDomainName: SetDomainName::<Impl, IMPL_OFFSET>,
            DomainName: DomainName::<Impl, IMPL_OFFSET>,
            SetDomainNameType: SetDomainNameType::<Impl, IMPL_OFFSET>,
            DomainNameType: DomainNameType::<Impl, IMPL_OFFSET>,
            DnsServers: DnsServers::<Impl, IMPL_OFFSET>,
            WebProxyServers: WebProxyServers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnDomainNameInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnDomainNameInfo2Impl: Sized {
    fn WebProxyUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnDomainNameInfo2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameInfo2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnDomainNameInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnDomainNameInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnDomainNameInfo2Vtbl {
        unsafe extern "system" fn WebProxyUris<Impl: IVpnDomainNameInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebProxyUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnDomainNameInfo2, BASE_OFFSET>(), WebProxyUris: WebProxyUris::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnDomainNameInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnDomainNameInfoFactoryImpl: Sized {
    fn CreateVpnDomainNameInfo(&mut self, name: &::windows::core::HSTRING, nametype: VpnDomainNameType, dnsserverlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>, proxyserverlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>) -> ::windows::core::Result<VpnDomainNameInfo>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVpnDomainNameInfoFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameInfoFactory";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnDomainNameInfoFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnDomainNameInfoFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnDomainNameInfoFactoryVtbl {
        unsafe extern "system" fn CreateVpnDomainNameInfo<Impl: IVpnDomainNameInfoFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, nametype: VpnDomainNameType, dnsserverlist: ::windows::core::RawPtr, proxyserverlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVpnDomainNameInfo(
                &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                nametype,
                &*(&dnsserverlist as *const <super::super::Foundation::Collections::IIterable<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                &*(&proxyserverlist as *const <super::super::Foundation::Collections::IIterable<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IIterable<super::HostName> as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnDomainNameInfoFactory, BASE_OFFSET>(),
            CreateVpnDomainNameInfo: CreateVpnDomainNameInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnDomainNameInfoFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnForegroundActivatedEventArgsImpl: Sized {
    fn ProfileName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SharedContext(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn ActivationOperation(&mut self) -> ::windows::core::Result<VpnForegroundActivationOperation>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnForegroundActivatedEventArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnForegroundActivatedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnForegroundActivatedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnForegroundActivatedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnForegroundActivatedEventArgsVtbl {
        unsafe extern "system" fn ProfileName<Impl: IVpnForegroundActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProfileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharedContext<Impl: IVpnForegroundActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SharedContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivationOperation<Impl: IVpnForegroundActivatedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivationOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnForegroundActivatedEventArgs, BASE_OFFSET>(),
            ProfileName: ProfileName::<Impl, IMPL_OFFSET>,
            SharedContext: SharedContext::<Impl, IMPL_OFFSET>,
            ActivationOperation: ActivationOperation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnForegroundActivatedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnForegroundActivationOperationImpl: Sized {
    fn Complete(&mut self, result: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnForegroundActivationOperation {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnForegroundActivationOperation";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnForegroundActivationOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnForegroundActivationOperationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnForegroundActivationOperationVtbl {
        unsafe extern "system" fn Complete<Impl: IVpnForegroundActivationOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete(&*(&result as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnForegroundActivationOperation, BASE_OFFSET>(),
            Complete: Complete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnForegroundActivationOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnInterfaceIdImpl: Sized {
    fn GetAddressInfo(&mut self, id: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnInterfaceId {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnInterfaceId";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnInterfaceIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnInterfaceIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnInterfaceIdVtbl {
        unsafe extern "system" fn GetAddressInfo<Impl: IVpnInterfaceIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id_array_size: *mut u32, id: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAddressInfo(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&id), id_array_size).as_array()).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnInterfaceId, BASE_OFFSET>(), GetAddressInfo: GetAddressInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnInterfaceId as ::windows::core::Interface>::IID
    }
}
pub trait IVpnInterfaceIdFactoryImpl: Sized {
    fn CreateVpnInterfaceId(&mut self, address: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<VpnInterfaceId>;
}
impl ::windows::core::RuntimeName for IVpnInterfaceIdFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnInterfaceIdFactory";
}
impl IVpnInterfaceIdFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnInterfaceIdFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnInterfaceIdFactoryVtbl {
        unsafe extern "system" fn CreateVpnInterfaceId<Impl: IVpnInterfaceIdFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address_array_size: u32, address: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVpnInterfaceId(::core::slice::from_raw_parts(::core::mem::transmute_copy(&address), address_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnInterfaceIdFactory, BASE_OFFSET>(),
            CreateVpnInterfaceId: CreateVpnInterfaceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnInterfaceIdFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IVpnManagementAgentImpl: Sized {
    fn AddProfileFromXmlAsync(&mut self, xml: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn AddProfileFromObjectAsync(&mut self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn UpdateProfileFromXmlAsync(&mut self, xml: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn UpdateProfileFromObjectAsync(&mut self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn GetProfilesAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IVpnProfile>>>;
    fn DeleteProfileAsync(&mut self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn ConnectProfileAsync(&mut self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn ConnectProfileWithPasswordCredentialAsync(&mut self, profile: &::core::option::Option<IVpnProfile>, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn DisconnectProfileAsync(&mut self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnManagementAgent {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnManagementAgent";
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IVpnManagementAgentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnManagementAgentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnManagementAgentVtbl {
        unsafe extern "system" fn AddProfileFromXmlAsync<Impl: IVpnManagementAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddProfileFromXmlAsync(&*(&xml as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddProfileFromObjectAsync<Impl: IVpnManagementAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddProfileFromObjectAsync(&*(&profile as *const <IVpnProfile as ::windows::core::Abi>::Abi as *const <IVpnProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateProfileFromXmlAsync<Impl: IVpnManagementAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateProfileFromXmlAsync(&*(&xml as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateProfileFromObjectAsync<Impl: IVpnManagementAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateProfileFromObjectAsync(&*(&profile as *const <IVpnProfile as ::windows::core::Abi>::Abi as *const <IVpnProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfilesAsync<Impl: IVpnManagementAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfilesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProfileAsync<Impl: IVpnManagementAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteProfileAsync(&*(&profile as *const <IVpnProfile as ::windows::core::Abi>::Abi as *const <IVpnProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectProfileAsync<Impl: IVpnManagementAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectProfileAsync(&*(&profile as *const <IVpnProfile as ::windows::core::Abi>::Abi as *const <IVpnProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectProfileWithPasswordCredentialAsync<Impl: IVpnManagementAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, passwordcredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectProfileWithPasswordCredentialAsync(&*(&profile as *const <IVpnProfile as ::windows::core::Abi>::Abi as *const <IVpnProfile as ::windows::core::DefaultType>::DefaultType), &*(&passwordcredential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectProfileAsync<Impl: IVpnManagementAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisconnectProfileAsync(&*(&profile as *const <IVpnProfile as ::windows::core::Abi>::Abi as *const <IVpnProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnManagementAgent, BASE_OFFSET>(),
            AddProfileFromXmlAsync: AddProfileFromXmlAsync::<Impl, IMPL_OFFSET>,
            AddProfileFromObjectAsync: AddProfileFromObjectAsync::<Impl, IMPL_OFFSET>,
            UpdateProfileFromXmlAsync: UpdateProfileFromXmlAsync::<Impl, IMPL_OFFSET>,
            UpdateProfileFromObjectAsync: UpdateProfileFromObjectAsync::<Impl, IMPL_OFFSET>,
            GetProfilesAsync: GetProfilesAsync::<Impl, IMPL_OFFSET>,
            DeleteProfileAsync: DeleteProfileAsync::<Impl, IMPL_OFFSET>,
            ConnectProfileAsync: ConnectProfileAsync::<Impl, IMPL_OFFSET>,
            ConnectProfileWithPasswordCredentialAsync: ConnectProfileWithPasswordCredentialAsync::<Impl, IMPL_OFFSET>,
            DisconnectProfileAsync: DisconnectProfileAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnManagementAgent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnNamespaceAssignmentImpl: Sized {
    fn SetNamespaceList(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnNamespaceInfo>>) -> ::windows::core::Result<()>;
    fn NamespaceList(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnNamespaceInfo>>;
    fn SetProxyAutoConfigUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ProxyAutoConfigUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnNamespaceAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNamespaceAssignment";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnNamespaceAssignmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnNamespaceAssignmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnNamespaceAssignmentVtbl {
        unsafe extern "system" fn SetNamespaceList<Impl: IVpnNamespaceAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamespaceList(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnNamespaceInfo> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnNamespaceInfo> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NamespaceList<Impl: IVpnNamespaceAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NamespaceList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyAutoConfigUri<Impl: IVpnNamespaceAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyAutoConfigUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyAutoConfigUri<Impl: IVpnNamespaceAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyAutoConfigUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnNamespaceAssignment, BASE_OFFSET>(),
            SetNamespaceList: SetNamespaceList::<Impl, IMPL_OFFSET>,
            NamespaceList: NamespaceList::<Impl, IMPL_OFFSET>,
            SetProxyAutoConfigUri: SetProxyAutoConfigUri::<Impl, IMPL_OFFSET>,
            ProxyAutoConfigUri: ProxyAutoConfigUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnNamespaceAssignment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnNamespaceInfoImpl: Sized {
    fn SetNamespace(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Namespace(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDnsServers(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>) -> ::windows::core::Result<()>;
    fn DnsServers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>>;
    fn SetWebProxyServers(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>) -> ::windows::core::Result<()>;
    fn WebProxyServers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnNamespaceInfo {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNamespaceInfo";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnNamespaceInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnNamespaceInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnNamespaceInfoVtbl {
        unsafe extern "system" fn SetNamespace<Impl: IVpnNamespaceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamespace(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Namespace<Impl: IVpnNamespaceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Namespace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDnsServers<Impl: IVpnNamespaceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDnsServers(&*(&value as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DnsServers<Impl: IVpnNamespaceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DnsServers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWebProxyServers<Impl: IVpnNamespaceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWebProxyServers(&*(&value as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WebProxyServers<Impl: IVpnNamespaceInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WebProxyServers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnNamespaceInfo, BASE_OFFSET>(),
            SetNamespace: SetNamespace::<Impl, IMPL_OFFSET>,
            Namespace: Namespace::<Impl, IMPL_OFFSET>,
            SetDnsServers: SetDnsServers::<Impl, IMPL_OFFSET>,
            DnsServers: DnsServers::<Impl, IMPL_OFFSET>,
            SetWebProxyServers: SetWebProxyServers::<Impl, IMPL_OFFSET>,
            WebProxyServers: WebProxyServers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnNamespaceInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnNamespaceInfoFactoryImpl: Sized {
    fn CreateVpnNamespaceInfo(&mut self, name: &::windows::core::HSTRING, dnsserverlist: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>, proxyserverlist: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>) -> ::windows::core::Result<VpnNamespaceInfo>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVpnNamespaceInfoFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNamespaceInfoFactory";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnNamespaceInfoFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnNamespaceInfoFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnNamespaceInfoFactoryVtbl {
        unsafe extern "system" fn CreateVpnNamespaceInfo<Impl: IVpnNamespaceInfoFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dnsserverlist: ::windows::core::RawPtr, proxyserverlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVpnNamespaceInfo(
                &*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&dnsserverlist as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::DefaultType>::DefaultType),
                &*(&proxyserverlist as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::DefaultType>::DefaultType),
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnNamespaceInfoFactory, BASE_OFFSET>(),
            CreateVpnNamespaceInfo: CreateVpnNamespaceInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnNamespaceInfoFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnNativeProfileImpl: Sized + IVpnProfileImpl {
    fn Servers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RoutingPolicyType(&mut self) -> ::windows::core::Result<VpnRoutingPolicyType>;
    fn SetRoutingPolicyType(&mut self, value: VpnRoutingPolicyType) -> ::windows::core::Result<()>;
    fn NativeProtocolType(&mut self) -> ::windows::core::Result<VpnNativeProtocolType>;
    fn SetNativeProtocolType(&mut self, value: VpnNativeProtocolType) -> ::windows::core::Result<()>;
    fn UserAuthenticationMethod(&mut self) -> ::windows::core::Result<VpnAuthenticationMethod>;
    fn SetUserAuthenticationMethod(&mut self, value: VpnAuthenticationMethod) -> ::windows::core::Result<()>;
    fn TunnelAuthenticationMethod(&mut self) -> ::windows::core::Result<VpnAuthenticationMethod>;
    fn SetTunnelAuthenticationMethod(&mut self, value: VpnAuthenticationMethod) -> ::windows::core::Result<()>;
    fn EapConfiguration(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEapConfiguration(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnNativeProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNativeProfile";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnNativeProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnNativeProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnNativeProfileVtbl {
        unsafe extern "system" fn Servers<Impl: IVpnNativeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Servers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingPolicyType<Impl: IVpnNativeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnRoutingPolicyType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoutingPolicyType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoutingPolicyType<Impl: IVpnNativeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnRoutingPolicyType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoutingPolicyType(value).into()
        }
        unsafe extern "system" fn NativeProtocolType<Impl: IVpnNativeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnNativeProtocolType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NativeProtocolType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNativeProtocolType<Impl: IVpnNativeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnNativeProtocolType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNativeProtocolType(value).into()
        }
        unsafe extern "system" fn UserAuthenticationMethod<Impl: IVpnNativeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnAuthenticationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserAuthenticationMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserAuthenticationMethod<Impl: IVpnNativeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnAuthenticationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserAuthenticationMethod(value).into()
        }
        unsafe extern "system" fn TunnelAuthenticationMethod<Impl: IVpnNativeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnAuthenticationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TunnelAuthenticationMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTunnelAuthenticationMethod<Impl: IVpnNativeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnAuthenticationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTunnelAuthenticationMethod(value).into()
        }
        unsafe extern "system" fn EapConfiguration<Impl: IVpnNativeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EapConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEapConfiguration<Impl: IVpnNativeProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEapConfiguration(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnNativeProfile, BASE_OFFSET>(),
            Servers: Servers::<Impl, IMPL_OFFSET>,
            RoutingPolicyType: RoutingPolicyType::<Impl, IMPL_OFFSET>,
            SetRoutingPolicyType: SetRoutingPolicyType::<Impl, IMPL_OFFSET>,
            NativeProtocolType: NativeProtocolType::<Impl, IMPL_OFFSET>,
            SetNativeProtocolType: SetNativeProtocolType::<Impl, IMPL_OFFSET>,
            UserAuthenticationMethod: UserAuthenticationMethod::<Impl, IMPL_OFFSET>,
            SetUserAuthenticationMethod: SetUserAuthenticationMethod::<Impl, IMPL_OFFSET>,
            TunnelAuthenticationMethod: TunnelAuthenticationMethod::<Impl, IMPL_OFFSET>,
            SetTunnelAuthenticationMethod: SetTunnelAuthenticationMethod::<Impl, IMPL_OFFSET>,
            EapConfiguration: EapConfiguration::<Impl, IMPL_OFFSET>,
            SetEapConfiguration: SetEapConfiguration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnNativeProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnNativeProfile2Impl: Sized {
    fn RequireVpnClientAppUI(&mut self) -> ::windows::core::Result<bool>;
    fn SetRequireVpnClientAppUI(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ConnectionStatus(&mut self) -> ::windows::core::Result<VpnManagementConnectionStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnNativeProfile2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNativeProfile2";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnNativeProfile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnNativeProfile2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnNativeProfile2Vtbl {
        unsafe extern "system" fn RequireVpnClientAppUI<Impl: IVpnNativeProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequireVpnClientAppUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequireVpnClientAppUI<Impl: IVpnNativeProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequireVpnClientAppUI(value).into()
        }
        unsafe extern "system" fn ConnectionStatus<Impl: IVpnNativeProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnManagementConnectionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnNativeProfile2, BASE_OFFSET>(),
            RequireVpnClientAppUI: RequireVpnClientAppUI::<Impl, IMPL_OFFSET>,
            SetRequireVpnClientAppUI: SetRequireVpnClientAppUI::<Impl, IMPL_OFFSET>,
            ConnectionStatus: ConnectionStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnNativeProfile2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IVpnPacketBufferImpl: Sized {
    fn Buffer(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::Buffer>;
    fn SetStatus(&mut self, value: VpnPacketBufferStatus) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<VpnPacketBufferStatus>;
    fn SetTransportAffinity(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn TransportAffinity(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnPacketBuffer {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBuffer";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IVpnPacketBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPacketBufferVtbl {
        unsafe extern "system" fn Buffer<Impl: IVpnPacketBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Buffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IVpnPacketBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnPacketBufferStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn Status<Impl: IVpnPacketBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnPacketBufferStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransportAffinity<Impl: IVpnPacketBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransportAffinity(value).into()
        }
        unsafe extern "system" fn TransportAffinity<Impl: IVpnPacketBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportAffinity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPacketBuffer, BASE_OFFSET>(),
            Buffer: Buffer::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            SetTransportAffinity: SetTransportAffinity::<Impl, IMPL_OFFSET>,
            TransportAffinity: TransportAffinity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPacketBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPacketBuffer2Impl: Sized {
    fn AppId(&mut self) -> ::windows::core::Result<VpnAppId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnPacketBuffer2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBuffer2";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnPacketBuffer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBuffer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPacketBuffer2Vtbl {
        unsafe extern "system" fn AppId<Impl: IVpnPacketBuffer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPacketBuffer2, BASE_OFFSET>(), AppId: AppId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPacketBuffer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPacketBuffer3Impl: Sized {
    fn SetTransportContext(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn TransportContext(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnPacketBuffer3 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBuffer3";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnPacketBuffer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBuffer3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPacketBuffer3Vtbl {
        unsafe extern "system" fn SetTransportContext<Impl: IVpnPacketBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransportContext(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransportContext<Impl: IVpnPacketBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransportContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPacketBuffer3, BASE_OFFSET>(),
            SetTransportContext: SetTransportContext::<Impl, IMPL_OFFSET>,
            TransportContext: TransportContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPacketBuffer3 as ::windows::core::Interface>::IID
    }
}
pub trait IVpnPacketBufferFactoryImpl: Sized {
    fn CreateVpnPacketBuffer(&mut self, parentbuffer: &::core::option::Option<VpnPacketBuffer>, offset: u32, length: u32) -> ::windows::core::Result<VpnPacketBuffer>;
}
impl ::windows::core::RuntimeName for IVpnPacketBufferFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBufferFactory";
}
impl IVpnPacketBufferFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBufferFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPacketBufferFactoryVtbl {
        unsafe extern "system" fn CreateVpnPacketBuffer<Impl: IVpnPacketBufferFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentbuffer: ::windows::core::RawPtr, offset: u32, length: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVpnPacketBuffer(&*(&parentbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType), offset, length) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPacketBufferFactory, BASE_OFFSET>(),
            CreateVpnPacketBuffer: CreateVpnPacketBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPacketBufferFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnPacketBufferListImpl: Sized + IIterableImpl<VpnPacketBuffer> {
    fn Append(&mut self, nextvpnpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn AddAtBegin(&mut self, nextvpnpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn RemoveAtEnd(&mut self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn RemoveAtBegin(&mut self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn SetStatus(&mut self, value: VpnPacketBufferStatus) -> ::windows::core::Result<()>;
    fn Status(&mut self) -> ::windows::core::Result<VpnPacketBufferStatus>;
    fn Size(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnPacketBufferList {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBufferList";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnPacketBufferListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBufferListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPacketBufferListVtbl {
        unsafe extern "system" fn Append<Impl: IVpnPacketBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(&*(&nextvpnpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddAtBegin<Impl: IVpnPacketBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAtBegin(&*(&nextvpnpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAtEnd<Impl: IVpnPacketBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAtEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAtBegin<Impl: IVpnPacketBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAtBegin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IVpnPacketBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn SetStatus<Impl: IVpnPacketBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnPacketBufferStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn Status<Impl: IVpnPacketBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnPacketBufferStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Size<Impl: IVpnPacketBufferListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPacketBufferList, BASE_OFFSET>(),
            Append: Append::<Impl, IMPL_OFFSET>,
            AddAtBegin: AddAtBegin::<Impl, IMPL_OFFSET>,
            RemoveAtEnd: RemoveAtEnd::<Impl, IMPL_OFFSET>,
            RemoveAtBegin: RemoveAtBegin::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            SetStatus: SetStatus::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            Size: Size::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPacketBufferList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnPacketBufferList2Impl: Sized + IIterableImpl<VpnPacketBuffer> {
    fn AddLeadingPacket(&mut self, nextvpnpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn RemoveLeadingPacket(&mut self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn AddTrailingPacket(&mut self, nextvpnpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn RemoveTrailingPacket(&mut self) -> ::windows::core::Result<VpnPacketBuffer>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnPacketBufferList2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBufferList2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnPacketBufferList2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBufferList2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPacketBufferList2Vtbl {
        unsafe extern "system" fn AddLeadingPacket<Impl: IVpnPacketBufferList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLeadingPacket(&*(&nextvpnpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveLeadingPacket<Impl: IVpnPacketBufferList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveLeadingPacket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrailingPacket<Impl: IVpnPacketBufferList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTrailingPacket(&*(&nextvpnpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveTrailingPacket<Impl: IVpnPacketBufferList2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveTrailingPacket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPacketBufferList2, BASE_OFFSET>(),
            AddLeadingPacket: AddLeadingPacket::<Impl, IMPL_OFFSET>,
            RemoveLeadingPacket: RemoveLeadingPacket::<Impl, IMPL_OFFSET>,
            AddTrailingPacket: AddTrailingPacket::<Impl, IMPL_OFFSET>,
            RemoveTrailingPacket: RemoveTrailingPacket::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPacketBufferList2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
pub trait IVpnPickedCredentialImpl: Sized {
    fn PasskeyCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn AdditionalPin(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OldPasswordCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnPickedCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPickedCredential";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IVpnPickedCredentialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPickedCredentialImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPickedCredentialVtbl {
        unsafe extern "system" fn PasskeyCredential<Impl: IVpnPickedCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PasskeyCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdditionalPin<Impl: IVpnPickedCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdditionalPin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldPasswordCredential<Impl: IVpnPickedCredentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OldPasswordCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPickedCredential, BASE_OFFSET>(),
            PasskeyCredential: PasskeyCredential::<Impl, IMPL_OFFSET>,
            AdditionalPin: AdditionalPin::<Impl, IMPL_OFFSET>,
            OldPasswordCredential: OldPasswordCredential::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPickedCredential as ::windows::core::Interface>::IID
    }
}
pub trait IVpnPlugInImpl: Sized {
    fn Connect(&mut self, channel: &::core::option::Option<VpnChannel>) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self, channel: &::core::option::Option<VpnChannel>) -> ::windows::core::Result<()>;
    fn GetKeepAlivePayload(&mut self, channel: &::core::option::Option<VpnChannel>, keepalivepacket: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn Encapsulate(&mut self, channel: &::core::option::Option<VpnChannel>, packets: &::core::option::Option<VpnPacketBufferList>, encapulatedpackets: &::core::option::Option<VpnPacketBufferList>) -> ::windows::core::Result<()>;
    fn Decapsulate(&mut self, channel: &::core::option::Option<VpnChannel>, encapbuffer: &::core::option::Option<VpnPacketBuffer>, decapsulatedpackets: &::core::option::Option<VpnPacketBufferList>, controlpacketstosend: &::core::option::Option<VpnPacketBufferList>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVpnPlugIn {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPlugIn";
}
impl IVpnPlugInVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPlugInImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPlugInVtbl {
        unsafe extern "system" fn Connect<Impl: IVpnPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disconnect<Impl: IVpnPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetKeepAlivePayload<Impl: IVpnPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, keepalivepacket: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKeepAlivePayload(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&keepalivepacket)).into()
        }
        unsafe extern "system" fn Encapsulate<Impl: IVpnPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, packets: ::windows::core::RawPtr, encapulatedpackets: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Encapsulate(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType), &*(&packets as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType), &*(&encapulatedpackets as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Decapsulate<Impl: IVpnPlugInImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, encapbuffer: ::windows::core::RawPtr, decapsulatedpackets: ::windows::core::RawPtr, controlpacketstosend: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .Decapsulate(
                    &*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType),
                    &*(&encapbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType),
                    &*(&decapsulatedpackets as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType),
                    &*(&controlpacketstosend as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPlugIn, BASE_OFFSET>(),
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            GetKeepAlivePayload: GetKeepAlivePayload::<Impl, IMPL_OFFSET>,
            Encapsulate: Encapsulate::<Impl, IMPL_OFFSET>,
            Decapsulate: Decapsulate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPlugIn as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnPlugInProfileImpl: Sized + IVpnProfileImpl {
    fn ServerUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn CustomConfiguration(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCustomConfiguration(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn VpnPluginPackageFamilyName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVpnPluginPackageFamilyName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnPlugInProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPlugInProfile";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnPlugInProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPlugInProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPlugInProfileVtbl {
        unsafe extern "system" fn ServerUris<Impl: IVpnPlugInProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomConfiguration<Impl: IVpnPlugInProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomConfiguration<Impl: IVpnPlugInProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomConfiguration(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VpnPluginPackageFamilyName<Impl: IVpnPlugInProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VpnPluginPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVpnPluginPackageFamilyName<Impl: IVpnPlugInProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVpnPluginPackageFamilyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPlugInProfile, BASE_OFFSET>(),
            ServerUris: ServerUris::<Impl, IMPL_OFFSET>,
            CustomConfiguration: CustomConfiguration::<Impl, IMPL_OFFSET>,
            SetCustomConfiguration: SetCustomConfiguration::<Impl, IMPL_OFFSET>,
            VpnPluginPackageFamilyName: VpnPluginPackageFamilyName::<Impl, IMPL_OFFSET>,
            SetVpnPluginPackageFamilyName: SetVpnPluginPackageFamilyName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPlugInProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnPlugInProfile2Impl: Sized + IVpnProfileImpl {
    fn RequireVpnClientAppUI(&mut self) -> ::windows::core::Result<bool>;
    fn SetRequireVpnClientAppUI(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ConnectionStatus(&mut self) -> ::windows::core::Result<VpnManagementConnectionStatus>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnPlugInProfile2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPlugInProfile2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnPlugInProfile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPlugInProfile2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPlugInProfile2Vtbl {
        unsafe extern "system" fn RequireVpnClientAppUI<Impl: IVpnPlugInProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequireVpnClientAppUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequireVpnClientAppUI<Impl: IVpnPlugInProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequireVpnClientAppUI(value).into()
        }
        unsafe extern "system" fn ConnectionStatus<Impl: IVpnPlugInProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnManagementConnectionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnPlugInProfile2, BASE_OFFSET>(),
            RequireVpnClientAppUI: RequireVpnClientAppUI::<Impl, IMPL_OFFSET>,
            SetRequireVpnClientAppUI: SetRequireVpnClientAppUI::<Impl, IMPL_OFFSET>,
            ConnectionStatus: ConnectionStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnPlugInProfile2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IVpnProfileImpl: Sized {
    fn ProfileName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetProfileName(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AppTriggers(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnAppId>>;
    fn Routes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn DomainNameInfoList(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>;
    fn TrafficFilters(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>;
    fn RememberCredentials(&mut self) -> ::windows::core::Result<bool>;
    fn SetRememberCredentials(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AlwaysOn(&mut self) -> ::windows::core::Result<bool>;
    fn SetAlwaysOn(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVpnProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnProfile";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnProfileVtbl {
        unsafe extern "system" fn ProfileName<Impl: IVpnProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProfileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfileName<Impl: IVpnProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProfileName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppTriggers<Impl: IVpnProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppTriggers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Routes<Impl: IVpnProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Routes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainNameInfoList<Impl: IVpnProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainNameInfoList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrafficFilters<Impl: IVpnProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrafficFilters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RememberCredentials<Impl: IVpnProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RememberCredentials() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRememberCredentials<Impl: IVpnProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRememberCredentials(value).into()
        }
        unsafe extern "system" fn AlwaysOn<Impl: IVpnProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AlwaysOn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysOn<Impl: IVpnProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlwaysOn(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnProfile, BASE_OFFSET>(),
            ProfileName: ProfileName::<Impl, IMPL_OFFSET>,
            SetProfileName: SetProfileName::<Impl, IMPL_OFFSET>,
            AppTriggers: AppTriggers::<Impl, IMPL_OFFSET>,
            Routes: Routes::<Impl, IMPL_OFFSET>,
            DomainNameInfoList: DomainNameInfoList::<Impl, IMPL_OFFSET>,
            TrafficFilters: TrafficFilters::<Impl, IMPL_OFFSET>,
            RememberCredentials: RememberCredentials::<Impl, IMPL_OFFSET>,
            SetRememberCredentials: SetRememberCredentials::<Impl, IMPL_OFFSET>,
            AlwaysOn: AlwaysOn::<Impl, IMPL_OFFSET>,
            SetAlwaysOn: SetAlwaysOn::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnRouteImpl: Sized {
    fn SetAddress(&mut self, value: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn Address(&mut self) -> ::windows::core::Result<super::HostName>;
    fn SetPrefixSize(&mut self, value: u8) -> ::windows::core::Result<()>;
    fn PrefixSize(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnRoute {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnRoute";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnRouteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnRouteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnRouteVtbl {
        unsafe extern "system" fn SetAddress<Impl: IVpnRouteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddress(&*(&value as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Address<Impl: IVpnRouteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefixSize<Impl: IVpnRouteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrefixSize(value).into()
        }
        unsafe extern "system" fn PrefixSize<Impl: IVpnRouteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrefixSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnRoute, BASE_OFFSET>(),
            SetAddress: SetAddress::<Impl, IMPL_OFFSET>,
            Address: Address::<Impl, IMPL_OFFSET>,
            SetPrefixSize: SetPrefixSize::<Impl, IMPL_OFFSET>,
            PrefixSize: PrefixSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnRoute as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnRouteAssignmentImpl: Sized {
    fn SetIpv4InclusionRoutes(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnRoute>>) -> ::windows::core::Result<()>;
    fn SetIpv6InclusionRoutes(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnRoute>>) -> ::windows::core::Result<()>;
    fn Ipv4InclusionRoutes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn Ipv6InclusionRoutes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn SetIpv4ExclusionRoutes(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnRoute>>) -> ::windows::core::Result<()>;
    fn SetIpv6ExclusionRoutes(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnRoute>>) -> ::windows::core::Result<()>;
    fn Ipv4ExclusionRoutes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn Ipv6ExclusionRoutes(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn SetExcludeLocalSubnets(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ExcludeLocalSubnets(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnRouteAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnRouteAssignment";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnRouteAssignmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnRouteAssignmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnRouteAssignmentVtbl {
        unsafe extern "system" fn SetIpv4InclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpv4InclusionRoutes(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetIpv6InclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpv6InclusionRoutes(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Ipv4InclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ipv4InclusionRoutes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ipv6InclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ipv6InclusionRoutes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpv4ExclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpv4ExclusionRoutes(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetIpv6ExclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpv6ExclusionRoutes(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Ipv4ExclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ipv4ExclusionRoutes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ipv6ExclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Ipv6ExclusionRoutes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExcludeLocalSubnets<Impl: IVpnRouteAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExcludeLocalSubnets(value).into()
        }
        unsafe extern "system" fn ExcludeLocalSubnets<Impl: IVpnRouteAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExcludeLocalSubnets() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnRouteAssignment, BASE_OFFSET>(),
            SetIpv4InclusionRoutes: SetIpv4InclusionRoutes::<Impl, IMPL_OFFSET>,
            SetIpv6InclusionRoutes: SetIpv6InclusionRoutes::<Impl, IMPL_OFFSET>,
            Ipv4InclusionRoutes: Ipv4InclusionRoutes::<Impl, IMPL_OFFSET>,
            Ipv6InclusionRoutes: Ipv6InclusionRoutes::<Impl, IMPL_OFFSET>,
            SetIpv4ExclusionRoutes: SetIpv4ExclusionRoutes::<Impl, IMPL_OFFSET>,
            SetIpv6ExclusionRoutes: SetIpv6ExclusionRoutes::<Impl, IMPL_OFFSET>,
            Ipv4ExclusionRoutes: Ipv4ExclusionRoutes::<Impl, IMPL_OFFSET>,
            Ipv6ExclusionRoutes: Ipv6ExclusionRoutes::<Impl, IMPL_OFFSET>,
            SetExcludeLocalSubnets: SetExcludeLocalSubnets::<Impl, IMPL_OFFSET>,
            ExcludeLocalSubnets: ExcludeLocalSubnets::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnRouteAssignment as ::windows::core::Interface>::IID
    }
}
pub trait IVpnRouteFactoryImpl: Sized {
    fn CreateVpnRoute(&mut self, address: &::core::option::Option<super::HostName>, prefixsize: u8) -> ::windows::core::Result<VpnRoute>;
}
impl ::windows::core::RuntimeName for IVpnRouteFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnRouteFactory";
}
impl IVpnRouteFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnRouteFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnRouteFactoryVtbl {
        unsafe extern "system" fn CreateVpnRoute<Impl: IVpnRouteFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: ::windows::core::RawPtr, prefixsize: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVpnRoute(&*(&address as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), prefixsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnRouteFactory, BASE_OFFSET>(), CreateVpnRoute: CreateVpnRoute::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnRouteFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IVpnSystemHealthImpl: Sized {
    fn StatementOfHealth(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::Buffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnSystemHealth {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnSystemHealth";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IVpnSystemHealthVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnSystemHealthImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnSystemHealthVtbl {
        unsafe extern "system" fn StatementOfHealth<Impl: IVpnSystemHealthImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StatementOfHealth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnSystemHealth, BASE_OFFSET>(),
            StatementOfHealth: StatementOfHealth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnSystemHealth as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnTrafficFilterImpl: Sized {
    fn AppId(&mut self) -> ::windows::core::Result<VpnAppId>;
    fn SetAppId(&mut self, value: &::core::option::Option<VpnAppId>) -> ::windows::core::Result<()>;
    fn AppClaims(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Protocol(&mut self) -> ::windows::core::Result<VpnIPProtocol>;
    fn SetProtocol(&mut self, value: VpnIPProtocol) -> ::windows::core::Result<()>;
    fn LocalPortRanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RemotePortRanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn LocalAddressRanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RemoteAddressRanges(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RoutingPolicyType(&mut self) -> ::windows::core::Result<VpnRoutingPolicyType>;
    fn SetRoutingPolicyType(&mut self, value: VpnRoutingPolicyType) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnTrafficFilter {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnTrafficFilter";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnTrafficFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnTrafficFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnTrafficFilterVtbl {
        unsafe extern "system" fn AppId<Impl: IVpnTrafficFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppId<Impl: IVpnTrafficFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppId(&*(&value as *const <VpnAppId as ::windows::core::Abi>::Abi as *const <VpnAppId as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppClaims<Impl: IVpnTrafficFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppClaims() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Impl: IVpnTrafficFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnIPProtocol) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProtocol<Impl: IVpnTrafficFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnIPProtocol) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtocol(value).into()
        }
        unsafe extern "system" fn LocalPortRanges<Impl: IVpnTrafficFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPortRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemotePortRanges<Impl: IVpnTrafficFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemotePortRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAddressRanges<Impl: IVpnTrafficFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalAddressRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteAddressRanges<Impl: IVpnTrafficFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteAddressRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingPolicyType<Impl: IVpnTrafficFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnRoutingPolicyType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoutingPolicyType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoutingPolicyType<Impl: IVpnTrafficFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnRoutingPolicyType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoutingPolicyType(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnTrafficFilter, BASE_OFFSET>(),
            AppId: AppId::<Impl, IMPL_OFFSET>,
            SetAppId: SetAppId::<Impl, IMPL_OFFSET>,
            AppClaims: AppClaims::<Impl, IMPL_OFFSET>,
            Protocol: Protocol::<Impl, IMPL_OFFSET>,
            SetProtocol: SetProtocol::<Impl, IMPL_OFFSET>,
            LocalPortRanges: LocalPortRanges::<Impl, IMPL_OFFSET>,
            RemotePortRanges: RemotePortRanges::<Impl, IMPL_OFFSET>,
            LocalAddressRanges: LocalAddressRanges::<Impl, IMPL_OFFSET>,
            RemoteAddressRanges: RemoteAddressRanges::<Impl, IMPL_OFFSET>,
            RoutingPolicyType: RoutingPolicyType::<Impl, IMPL_OFFSET>,
            SetRoutingPolicyType: SetRoutingPolicyType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnTrafficFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnTrafficFilterAssignmentImpl: Sized {
    fn TrafficFilterList(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>;
    fn AllowOutbound(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowOutbound(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn AllowInbound(&mut self) -> ::windows::core::Result<bool>;
    fn SetAllowInbound(&mut self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnTrafficFilterAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnTrafficFilterAssignment";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnTrafficFilterAssignmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnTrafficFilterAssignmentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnTrafficFilterAssignmentVtbl {
        unsafe extern "system" fn TrafficFilterList<Impl: IVpnTrafficFilterAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrafficFilterList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowOutbound<Impl: IVpnTrafficFilterAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowOutbound() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutbound<Impl: IVpnTrafficFilterAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowOutbound(value).into()
        }
        unsafe extern "system" fn AllowInbound<Impl: IVpnTrafficFilterAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInbound() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInbound<Impl: IVpnTrafficFilterAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowInbound(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnTrafficFilterAssignment, BASE_OFFSET>(),
            TrafficFilterList: TrafficFilterList::<Impl, IMPL_OFFSET>,
            AllowOutbound: AllowOutbound::<Impl, IMPL_OFFSET>,
            SetAllowOutbound: SetAllowOutbound::<Impl, IMPL_OFFSET>,
            AllowInbound: AllowInbound::<Impl, IMPL_OFFSET>,
            SetAllowInbound: SetAllowInbound::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnTrafficFilterAssignment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnTrafficFilterFactoryImpl: Sized {
    fn Create(&mut self, appid: &::core::option::Option<VpnAppId>) -> ::windows::core::Result<VpnTrafficFilter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnTrafficFilterFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnTrafficFilterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnTrafficFilterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnTrafficFilterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnTrafficFilterFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IVpnTrafficFilterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&appid as *const <VpnAppId as ::windows::core::Abi>::Abi as *const <VpnAppId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnTrafficFilterFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnTrafficFilterFactory as ::windows::core::Interface>::IID
    }
}
