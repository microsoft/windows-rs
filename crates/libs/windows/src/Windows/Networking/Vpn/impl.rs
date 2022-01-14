#[cfg(feature = "implement_exclusive")]
pub trait IVpnAppId_Impl: Sized {
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
impl IVpnAppId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnAppId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnAppId_Vtbl {
        unsafe extern "system" fn Type<Impl: IVpnAppId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnAppIdType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetType<Impl: IVpnAppId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnAppIdType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn Value<Impl: IVpnAppId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IVpnAppId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IVpnAppIdFactory_Impl: Sized {
    fn Create(&mut self, r#type: VpnAppIdType, value: &::windows::core::HSTRING) -> ::windows::core::Result<VpnAppId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnAppIdFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnAppIdFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnAppIdFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnAppIdFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnAppIdFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IVpnAppIdFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VpnAppIdType, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnChannel_Impl: Sized {
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
impl IVpnChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannel_Vtbl {
        unsafe extern "system" fn AssociateTransport<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AssociateTransport(&*(&mainoutertunneltransport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&optionaloutertunneltransport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, routescope: ::windows::core::RawPtr, namespacescope: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, optimizeforlowcostnetwork: bool, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Stop<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn RequestCredentials<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, isretry: bool, issinglesignoncredential: bool, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestVpnPacketBuffer<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VpnDataPathType, vpnpacketbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestVpnPacketBuffer(r#type, ::core::mem::transmute_copy(&vpnpacketbuffer)).into()
        }
        unsafe extern "system" fn LogDiagnosticMessage<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LogDiagnosticMessage(&*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Configuration<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ActivityChange<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveActivityChange<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActivityChange(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetPlugInContext<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlugInContext(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlugInContext<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SystemHealth<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestCustomPrompt<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, customprompt: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestCustomPrompt(&*(&customprompt as *const <super::super::Foundation::Collections::IVectorView<IVpnCustomPrompt> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<IVpnCustomPrompt> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetErrorMessage<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetErrorMessage(&*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetAllowedSslTlsVersions<Impl: IVpnChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tunneltransport: *mut ::core::ffi::c_void, usetls12: bool) -> ::windows::core::HRESULT {
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
pub trait IVpnChannel2_Impl: Sized {
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
impl IVpnChannel2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannel2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannel2_Vtbl {
        unsafe extern "system" fn StartWithMainTransport<Impl: IVpnChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assigneddomainname: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn StartExistingTransports<Impl: IVpnChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assigneddomainname: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ActivityStateChange<Impl: IVpnChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveActivityStateChange<Impl: IVpnChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveActivityStateChange(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetVpnSendPacketBuffer<Impl: IVpnChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetVpnReceivePacketBuffer<Impl: IVpnChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestCustomPromptAsync<Impl: IVpnChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, custompromptelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestCredentialsWithCertificateAsync<Impl: IVpnChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, credoptions: u32, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestCredentialsWithOptionsAsync<Impl: IVpnChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, credoptions: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestCredentialsSimpleAsync<Impl: IVpnChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TerminateConnection<Impl: IVpnChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TerminateConnection(&*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartWithTrafficFilter<Impl: IVpnChannel2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assignednamespace: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void, assignedtrafficfilters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnChannel4_Impl: Sized {
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
impl IVpnChannel4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannel4_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannel4_Vtbl {
        unsafe extern "system" fn AddAndAssociateTransport<Impl: IVpnChannel4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAndAssociateTransport(&*(&transport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartWithMultipleTransports<Impl: IVpnChannel4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, assignedclientipv4addresses: ::windows::core::RawPtr, assignedclientipv6addresses: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assignednamespace: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool, transports: ::windows::core::RawPtr, assignedtrafficfilters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ReplaceAndAssociateTransport<Impl: IVpnChannel4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReplaceAndAssociateTransport(&*(&transport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartReconnectingTransport<Impl: IVpnChannel4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartReconnectingTransport(&*(&transport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetSlotTypeForTransportContext<Impl: IVpnChannel4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut super::Sockets::ControlChannelTriggerStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CurrentRequestTransportContext<Impl: IVpnChannel4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IVpnChannel5_Impl: Sized {
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
impl IVpnChannel5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannel5_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannel5_Vtbl {
        unsafe extern "system" fn AppendVpnReceivePacketBuffer<Impl: IVpnChannel5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, decapsulatedpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AppendVpnReceivePacketBuffer(&*(&decapsulatedpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendVpnSendPacketBuffer<Impl: IVpnChannel5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, encapsulatedpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AppendVpnSendPacketBuffer(&*(&encapsulatedpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FlushVpnReceivePacketBuffers<Impl: IVpnChannel5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushVpnReceivePacketBuffers().into()
        }
        unsafe extern "system" fn FlushVpnSendPacketBuffers<Impl: IVpnChannel5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IVpnChannel6_Impl: Sized {
    fn ActivateForeground(&mut self, packagerelativeappid: &::windows::core::HSTRING, sharedcontext: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnChannel6 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannel6";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnChannel6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannel6_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannel6_Vtbl {
        unsafe extern "system" fn ActivateForeground<Impl: IVpnChannel6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, packagerelativeappid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sharedcontext: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnChannelActivityEventArgs_Impl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<VpnChannelActivityEventType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannelActivityEventArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelActivityEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannelActivityEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelActivityEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannelActivityEventArgs_Vtbl {
        unsafe extern "system" fn Type<Impl: IVpnChannelActivityEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnChannelActivityEventType) -> ::windows::core::HRESULT {
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
pub trait IVpnChannelActivityStateChangedArgs_Impl: Sized {
    fn ActivityState(&mut self) -> ::windows::core::Result<VpnChannelActivityEventType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannelActivityStateChangedArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelActivityStateChangedArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannelActivityStateChangedArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelActivityStateChangedArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannelActivityStateChangedArgs_Vtbl {
        unsafe extern "system" fn ActivityState<Impl: IVpnChannelActivityStateChangedArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnChannelActivityEventType) -> ::windows::core::HRESULT {
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
pub trait IVpnChannelConfiguration_Impl: Sized {
    fn ServerServiceName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServerHostNameList(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>>;
    fn CustomField(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnChannelConfiguration {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelConfiguration";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnChannelConfiguration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelConfiguration_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannelConfiguration_Vtbl {
        unsafe extern "system" fn ServerServiceName<Impl: IVpnChannelConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ServerHostNameList<Impl: IVpnChannelConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CustomField<Impl: IVpnChannelConfiguration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IVpnChannelConfiguration2_Impl: Sized {
    fn ServerUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnChannelConfiguration2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelConfiguration2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnChannelConfiguration2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelConfiguration2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannelConfiguration2_Vtbl {
        unsafe extern "system" fn ServerUris<Impl: IVpnChannelConfiguration2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnChannelStatics_Impl: Sized {
    fn ProcessEventAsync(&mut self, thirdpartyplugin: &::core::option::Option<::windows::core::IInspectable>, event: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVpnChannelStatics {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelStatics";
}
impl IVpnChannelStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnChannelStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnChannelStatics_Vtbl {
        unsafe extern "system" fn ProcessEventAsync<Impl: IVpnChannelStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thirdpartyplugin: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IVpnCredential_Impl: Sized {
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
impl IVpnCredential_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCredential_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCredential_Vtbl {
        unsafe extern "system" fn PasskeyCredential<Impl: IVpnCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CertificateCredential<Impl: IVpnCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AdditionalPin<Impl: IVpnCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OldPasswordCredential<Impl: IVpnCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnCustomCheckBox_Impl: Sized + IVpnCustomPrompt_Impl {
    fn SetInitialCheckState(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InitialCheckState(&mut self) -> ::windows::core::Result<bool>;
    fn Checked(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomCheckBox {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomCheckBox";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomCheckBox_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomCheckBox_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomCheckBox_Vtbl {
        unsafe extern "system" fn SetInitialCheckState<Impl: IVpnCustomCheckBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialCheckState(value).into()
        }
        unsafe extern "system" fn InitialCheckState<Impl: IVpnCustomCheckBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Checked<Impl: IVpnCustomCheckBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IVpnCustomComboBox_Impl: Sized + IVpnCustomPrompt_Impl {
    fn SetOptionsText(&mut self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn OptionsText(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Selected(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnCustomComboBox {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomComboBox";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnCustomComboBox_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomComboBox_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomComboBox_Vtbl {
        unsafe extern "system" fn SetOptionsText<Impl: IVpnCustomComboBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOptionsText(&*(&value as *const <super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OptionsText<Impl: IVpnCustomComboBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Selected<Impl: IVpnCustomComboBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IVpnCustomEditBox_Impl: Sized + IVpnCustomPrompt_Impl {
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
impl IVpnCustomEditBox_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomEditBox_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomEditBox_Vtbl {
        unsafe extern "system" fn SetDefaultText<Impl: IVpnCustomEditBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultText<Impl: IVpnCustomEditBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNoEcho<Impl: IVpnCustomEditBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNoEcho(value).into()
        }
        unsafe extern "system" fn NoEcho<Impl: IVpnCustomEditBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Text<Impl: IVpnCustomEditBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IVpnCustomErrorBox_Impl: Sized + IVpnCustomPrompt_Impl {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomErrorBox {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomErrorBox";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomErrorBox_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomErrorBox_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomErrorBox_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnCustomErrorBox, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnCustomErrorBox as ::windows::core::Interface>::IID
    }
}
pub trait IVpnCustomPrompt_Impl: Sized {
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
impl IVpnCustomPrompt_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPrompt_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPrompt_Vtbl {
        unsafe extern "system" fn SetLabel<Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Label<Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCompulsory<Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompulsory(value).into()
        }
        unsafe extern "system" fn Compulsory<Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetBordered<Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBordered(value).into()
        }
        unsafe extern "system" fn Bordered<Impl: IVpnCustomPrompt_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IVpnCustomPromptBooleanInput_Impl: Sized + IVpnCustomPromptElement_Impl {
    fn SetInitialValue(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn InitialValue(&mut self) -> ::windows::core::Result<bool>;
    fn Value(&mut self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomPromptBooleanInput {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptBooleanInput";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomPromptBooleanInput_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptBooleanInput_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPromptBooleanInput_Vtbl {
        unsafe extern "system" fn SetInitialValue<Impl: IVpnCustomPromptBooleanInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInitialValue(value).into()
        }
        unsafe extern "system" fn InitialValue<Impl: IVpnCustomPromptBooleanInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IVpnCustomPromptBooleanInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IVpnCustomPromptElement_Impl: Sized {
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
impl IVpnCustomPromptElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPromptElement_Vtbl {
        unsafe extern "system" fn SetDisplayName<Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCompulsory<Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompulsory(value).into()
        }
        unsafe extern "system" fn Compulsory<Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEmphasized<Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmphasized(value).into()
        }
        unsafe extern "system" fn Emphasized<Impl: IVpnCustomPromptElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IVpnCustomPromptOptionSelector_Impl: Sized + IVpnCustomPromptElement_Impl {
    fn Options(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SelectedIndex(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnCustomPromptOptionSelector {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptOptionSelector";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnCustomPromptOptionSelector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptOptionSelector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPromptOptionSelector_Vtbl {
        unsafe extern "system" fn Options<Impl: IVpnCustomPromptOptionSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SelectedIndex<Impl: IVpnCustomPromptOptionSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IVpnCustomPromptText_Impl: Sized + IVpnCustomPromptElement_Impl {
    fn SetText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Text(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomPromptText {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptText";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomPromptText_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptText_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPromptText_Vtbl {
        unsafe extern "system" fn SetText<Impl: IVpnCustomPromptText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Text<Impl: IVpnCustomPromptText_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IVpnCustomPromptTextInput_Impl: Sized + IVpnCustomPromptElement_Impl {
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
impl IVpnCustomPromptTextInput_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomPromptTextInput_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomPromptTextInput_Vtbl {
        unsafe extern "system" fn SetPlaceholderText<Impl: IVpnCustomPromptTextInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlaceholderText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaceholderText<Impl: IVpnCustomPromptTextInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIsTextHidden<Impl: IVpnCustomPromptTextInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsTextHidden(value).into()
        }
        unsafe extern "system" fn IsTextHidden<Impl: IVpnCustomPromptTextInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Text<Impl: IVpnCustomPromptTextInput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IVpnCustomTextBox_Impl: Sized + IVpnCustomPrompt_Impl {
    fn SetDisplayText(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayText(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomTextBox {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomTextBox";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomTextBox_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnCustomTextBox_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnCustomTextBox_Vtbl {
        unsafe extern "system" fn SetDisplayText<Impl: IVpnCustomTextBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayText<Impl: IVpnCustomTextBox_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IVpnDomainNameAssignment_Impl: Sized {
    fn DomainNameList(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>;
    fn SetProxyAutoConfigurationUri(&mut self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ProxyAutoConfigurationUri(&mut self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnDomainNameAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameAssignment";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnDomainNameAssignment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnDomainNameAssignment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnDomainNameAssignment_Vtbl {
        unsafe extern "system" fn DomainNameList<Impl: IVpnDomainNameAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProxyAutoConfigurationUri<Impl: IVpnDomainNameAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyAutoConfigurationUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyAutoConfigurationUri<Impl: IVpnDomainNameAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnDomainNameInfo_Impl: Sized {
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
impl IVpnDomainNameInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnDomainNameInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnDomainNameInfo_Vtbl {
        unsafe extern "system" fn SetDomainName<Impl: IVpnDomainNameInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomainName(&*(&value as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DomainName<Impl: IVpnDomainNameInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDomainNameType<Impl: IVpnDomainNameInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnDomainNameType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomainNameType(value).into()
        }
        unsafe extern "system" fn DomainNameType<Impl: IVpnDomainNameInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnDomainNameType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DnsServers<Impl: IVpnDomainNameInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn WebProxyServers<Impl: IVpnDomainNameInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnDomainNameInfo2_Impl: Sized {
    fn WebProxyUris(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnDomainNameInfo2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameInfo2";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnDomainNameInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnDomainNameInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnDomainNameInfo2_Vtbl {
        unsafe extern "system" fn WebProxyUris<Impl: IVpnDomainNameInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnDomainNameInfoFactory_Impl: Sized {
    fn CreateVpnDomainNameInfo(&mut self, name: &::windows::core::HSTRING, nametype: VpnDomainNameType, dnsserverlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>, proxyserverlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>) -> ::windows::core::Result<VpnDomainNameInfo>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVpnDomainNameInfoFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameInfoFactory";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnDomainNameInfoFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnDomainNameInfoFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnDomainNameInfoFactory_Vtbl {
        unsafe extern "system" fn CreateVpnDomainNameInfo<Impl: IVpnDomainNameInfoFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, nametype: VpnDomainNameType, dnsserverlist: ::windows::core::RawPtr, proxyserverlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnForegroundActivatedEventArgs_Impl: Sized {
    fn ProfileName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SharedContext(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn ActivationOperation(&mut self) -> ::windows::core::Result<VpnForegroundActivationOperation>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnForegroundActivatedEventArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnForegroundActivatedEventArgs";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnForegroundActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnForegroundActivatedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnForegroundActivatedEventArgs_Vtbl {
        unsafe extern "system" fn ProfileName<Impl: IVpnForegroundActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SharedContext<Impl: IVpnForegroundActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ActivationOperation<Impl: IVpnForegroundActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnForegroundActivationOperation_Impl: Sized {
    fn Complete(&mut self, result: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnForegroundActivationOperation {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnForegroundActivationOperation";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnForegroundActivationOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnForegroundActivationOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnForegroundActivationOperation_Vtbl {
        unsafe extern "system" fn Complete<Impl: IVpnForegroundActivationOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnInterfaceId_Impl: Sized {
    fn GetAddressInfo(&mut self, id: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnInterfaceId {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnInterfaceId";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnInterfaceId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnInterfaceId_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnInterfaceId_Vtbl {
        unsafe extern "system" fn GetAddressInfo<Impl: IVpnInterfaceId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id_array_size: *mut u32, id: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAddressInfo(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&id), id_array_size).as_array()).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IVpnInterfaceId, BASE_OFFSET>(), GetAddressInfo: GetAddressInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVpnInterfaceId as ::windows::core::Interface>::IID
    }
}
pub trait IVpnInterfaceIdFactory_Impl: Sized {
    fn CreateVpnInterfaceId(&mut self, address: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<VpnInterfaceId>;
}
impl ::windows::core::RuntimeName for IVpnInterfaceIdFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnInterfaceIdFactory";
}
impl IVpnInterfaceIdFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnInterfaceIdFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnInterfaceIdFactory_Vtbl {
        unsafe extern "system" fn CreateVpnInterfaceId<Impl: IVpnInterfaceIdFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address_array_size: u32, address: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnManagementAgent_Impl: Sized {
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
impl IVpnManagementAgent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnManagementAgent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnManagementAgent_Vtbl {
        unsafe extern "system" fn AddProfileFromXmlAsync<Impl: IVpnManagementAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddProfileFromObjectAsync<Impl: IVpnManagementAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdateProfileFromXmlAsync<Impl: IVpnManagementAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UpdateProfileFromObjectAsync<Impl: IVpnManagementAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetProfilesAsync<Impl: IVpnManagementAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DeleteProfileAsync<Impl: IVpnManagementAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectProfileAsync<Impl: IVpnManagementAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectProfileWithPasswordCredentialAsync<Impl: IVpnManagementAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, passwordcredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DisconnectProfileAsync<Impl: IVpnManagementAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnNamespaceAssignment_Impl: Sized {
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
impl IVpnNamespaceAssignment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnNamespaceAssignment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnNamespaceAssignment_Vtbl {
        unsafe extern "system" fn SetNamespaceList<Impl: IVpnNamespaceAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamespaceList(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnNamespaceInfo> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnNamespaceInfo> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NamespaceList<Impl: IVpnNamespaceAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProxyAutoConfigUri<Impl: IVpnNamespaceAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyAutoConfigUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyAutoConfigUri<Impl: IVpnNamespaceAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnNamespaceInfo_Impl: Sized {
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
impl IVpnNamespaceInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnNamespaceInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnNamespaceInfo_Vtbl {
        unsafe extern "system" fn SetNamespace<Impl: IVpnNamespaceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamespace(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Namespace<Impl: IVpnNamespaceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDnsServers<Impl: IVpnNamespaceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDnsServers(&*(&value as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DnsServers<Impl: IVpnNamespaceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetWebProxyServers<Impl: IVpnNamespaceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWebProxyServers(&*(&value as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WebProxyServers<Impl: IVpnNamespaceInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnNamespaceInfoFactory_Impl: Sized {
    fn CreateVpnNamespaceInfo(&mut self, name: &::windows::core::HSTRING, dnsserverlist: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>, proxyserverlist: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>) -> ::windows::core::Result<VpnNamespaceInfo>;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for IVpnNamespaceInfoFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNamespaceInfoFactory";
}
#[cfg(feature = "Foundation_Collections")]
impl IVpnNamespaceInfoFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnNamespaceInfoFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnNamespaceInfoFactory_Vtbl {
        unsafe extern "system" fn CreateVpnNamespaceInfo<Impl: IVpnNamespaceInfoFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dnsserverlist: ::windows::core::RawPtr, proxyserverlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnNativeProfile_Impl: Sized + IVpnProfile_Impl {
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
impl IVpnNativeProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnNativeProfile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnNativeProfile_Vtbl {
        unsafe extern "system" fn Servers<Impl: IVpnNativeProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoutingPolicyType<Impl: IVpnNativeProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnRoutingPolicyType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRoutingPolicyType<Impl: IVpnNativeProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnRoutingPolicyType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoutingPolicyType(value).into()
        }
        unsafe extern "system" fn NativeProtocolType<Impl: IVpnNativeProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnNativeProtocolType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetNativeProtocolType<Impl: IVpnNativeProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnNativeProtocolType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNativeProtocolType(value).into()
        }
        unsafe extern "system" fn UserAuthenticationMethod<Impl: IVpnNativeProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnAuthenticationMethod) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetUserAuthenticationMethod<Impl: IVpnNativeProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnAuthenticationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserAuthenticationMethod(value).into()
        }
        unsafe extern "system" fn TunnelAuthenticationMethod<Impl: IVpnNativeProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnAuthenticationMethod) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTunnelAuthenticationMethod<Impl: IVpnNativeProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnAuthenticationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTunnelAuthenticationMethod(value).into()
        }
        unsafe extern "system" fn EapConfiguration<Impl: IVpnNativeProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetEapConfiguration<Impl: IVpnNativeProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IVpnNativeProfile2_Impl: Sized {
    fn RequireVpnClientAppUI(&mut self) -> ::windows::core::Result<bool>;
    fn SetRequireVpnClientAppUI(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ConnectionStatus(&mut self) -> ::windows::core::Result<VpnManagementConnectionStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnNativeProfile2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNativeProfile2";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnNativeProfile2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnNativeProfile2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnNativeProfile2_Vtbl {
        unsafe extern "system" fn RequireVpnClientAppUI<Impl: IVpnNativeProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRequireVpnClientAppUI<Impl: IVpnNativeProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequireVpnClientAppUI(value).into()
        }
        unsafe extern "system" fn ConnectionStatus<Impl: IVpnNativeProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnManagementConnectionStatus) -> ::windows::core::HRESULT {
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
pub trait IVpnPacketBuffer_Impl: Sized {
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
impl IVpnPacketBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPacketBuffer_Vtbl {
        unsafe extern "system" fn Buffer<Impl: IVpnPacketBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetStatus<Impl: IVpnPacketBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnPacketBufferStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn Status<Impl: IVpnPacketBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnPacketBufferStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetTransportAffinity<Impl: IVpnPacketBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransportAffinity(value).into()
        }
        unsafe extern "system" fn TransportAffinity<Impl: IVpnPacketBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IVpnPacketBuffer2_Impl: Sized {
    fn AppId(&mut self) -> ::windows::core::Result<VpnAppId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnPacketBuffer2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBuffer2";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnPacketBuffer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBuffer2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPacketBuffer2_Vtbl {
        unsafe extern "system" fn AppId<Impl: IVpnPacketBuffer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnPacketBuffer3_Impl: Sized {
    fn SetTransportContext(&mut self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn TransportContext(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnPacketBuffer3 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBuffer3";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnPacketBuffer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBuffer3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPacketBuffer3_Vtbl {
        unsafe extern "system" fn SetTransportContext<Impl: IVpnPacketBuffer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransportContext(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransportContext<Impl: IVpnPacketBuffer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IVpnPacketBufferFactory_Impl: Sized {
    fn CreateVpnPacketBuffer(&mut self, parentbuffer: &::core::option::Option<VpnPacketBuffer>, offset: u32, length: u32) -> ::windows::core::Result<VpnPacketBuffer>;
}
impl ::windows::core::RuntimeName for IVpnPacketBufferFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBufferFactory";
}
impl IVpnPacketBufferFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBufferFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPacketBufferFactory_Vtbl {
        unsafe extern "system" fn CreateVpnPacketBuffer<Impl: IVpnPacketBufferFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentbuffer: ::windows::core::RawPtr, offset: u32, length: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnPacketBufferList_Impl: Sized + super::super::Foundation::Collections::IIterable_Impl<VpnPacketBuffer> {
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
impl IVpnPacketBufferList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBufferList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPacketBufferList_Vtbl {
        unsafe extern "system" fn Append<Impl: IVpnPacketBufferList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(&*(&nextvpnpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddAtBegin<Impl: IVpnPacketBufferList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAtBegin(&*(&nextvpnpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAtEnd<Impl: IVpnPacketBufferList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveAtBegin<Impl: IVpnPacketBufferList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Clear<Impl: IVpnPacketBufferList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn SetStatus<Impl: IVpnPacketBufferList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnPacketBufferStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn Status<Impl: IVpnPacketBufferList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnPacketBufferStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Size<Impl: IVpnPacketBufferList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
pub trait IVpnPacketBufferList2_Impl: Sized + super::super::Foundation::Collections::IIterable_Impl<VpnPacketBuffer> {
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
impl IVpnPacketBufferList2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPacketBufferList2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPacketBufferList2_Vtbl {
        unsafe extern "system" fn AddLeadingPacket<Impl: IVpnPacketBufferList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLeadingPacket(&*(&nextvpnpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveLeadingPacket<Impl: IVpnPacketBufferList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddTrailingPacket<Impl: IVpnPacketBufferList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTrailingPacket(&*(&nextvpnpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveTrailingPacket<Impl: IVpnPacketBufferList2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnPickedCredential_Impl: Sized {
    fn PasskeyCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn AdditionalPin(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OldPasswordCredential(&mut self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnPickedCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPickedCredential";
}
#[cfg(all(feature = "Security_Credentials", feature = "implement_exclusive"))]
impl IVpnPickedCredential_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPickedCredential_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPickedCredential_Vtbl {
        unsafe extern "system" fn PasskeyCredential<Impl: IVpnPickedCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AdditionalPin<Impl: IVpnPickedCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn OldPasswordCredential<Impl: IVpnPickedCredential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnPlugIn_Impl: Sized {
    fn Connect(&mut self, channel: &::core::option::Option<VpnChannel>) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self, channel: &::core::option::Option<VpnChannel>) -> ::windows::core::Result<()>;
    fn GetKeepAlivePayload(&mut self, channel: &::core::option::Option<VpnChannel>, keepalivepacket: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn Encapsulate(&mut self, channel: &::core::option::Option<VpnChannel>, packets: &::core::option::Option<VpnPacketBufferList>, encapulatedpackets: &::core::option::Option<VpnPacketBufferList>) -> ::windows::core::Result<()>;
    fn Decapsulate(&mut self, channel: &::core::option::Option<VpnChannel>, encapbuffer: &::core::option::Option<VpnPacketBuffer>, decapsulatedpackets: &::core::option::Option<VpnPacketBufferList>, controlpacketstosend: &::core::option::Option<VpnPacketBufferList>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVpnPlugIn {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPlugIn";
}
impl IVpnPlugIn_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPlugIn_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPlugIn_Vtbl {
        unsafe extern "system" fn Connect<Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disconnect<Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetKeepAlivePayload<Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, keepalivepacket: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKeepAlivePayload(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&keepalivepacket)).into()
        }
        unsafe extern "system" fn Encapsulate<Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, packets: ::windows::core::RawPtr, encapulatedpackets: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Encapsulate(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType), &*(&packets as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType), &*(&encapulatedpackets as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Decapsulate<Impl: IVpnPlugIn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, encapbuffer: ::windows::core::RawPtr, decapsulatedpackets: ::windows::core::RawPtr, controlpacketstosend: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnPlugInProfile_Impl: Sized + IVpnProfile_Impl {
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
impl IVpnPlugInProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPlugInProfile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPlugInProfile_Vtbl {
        unsafe extern "system" fn ServerUris<Impl: IVpnPlugInProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CustomConfiguration<Impl: IVpnPlugInProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCustomConfiguration<Impl: IVpnPlugInProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomConfiguration(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VpnPluginPackageFamilyName<Impl: IVpnPlugInProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetVpnPluginPackageFamilyName<Impl: IVpnPlugInProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
pub trait IVpnPlugInProfile2_Impl: Sized + IVpnProfile_Impl {
    fn RequireVpnClientAppUI(&mut self) -> ::windows::core::Result<bool>;
    fn SetRequireVpnClientAppUI(&mut self, value: bool) -> ::windows::core::Result<()>;
    fn ConnectionStatus(&mut self) -> ::windows::core::Result<VpnManagementConnectionStatus>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnPlugInProfile2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPlugInProfile2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnPlugInProfile2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnPlugInProfile2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnPlugInProfile2_Vtbl {
        unsafe extern "system" fn RequireVpnClientAppUI<Impl: IVpnPlugInProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRequireVpnClientAppUI<Impl: IVpnPlugInProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequireVpnClientAppUI(value).into()
        }
        unsafe extern "system" fn ConnectionStatus<Impl: IVpnPlugInProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnManagementConnectionStatus) -> ::windows::core::HRESULT {
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
pub trait IVpnProfile_Impl: Sized {
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
impl IVpnProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnProfile_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnProfile_Vtbl {
        unsafe extern "system" fn ProfileName<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProfileName<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProfileName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppTriggers<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Routes<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn DomainNameInfoList<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TrafficFilters<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RememberCredentials<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRememberCredentials<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRememberCredentials(value).into()
        }
        unsafe extern "system" fn AlwaysOn<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAlwaysOn<Impl: IVpnProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IVpnRoute_Impl: Sized {
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
impl IVpnRoute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnRoute_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnRoute_Vtbl {
        unsafe extern "system" fn SetAddress<Impl: IVpnRoute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddress(&*(&value as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Address<Impl: IVpnRoute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPrefixSize<Impl: IVpnRoute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrefixSize(value).into()
        }
        unsafe extern "system" fn PrefixSize<Impl: IVpnRoute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
pub trait IVpnRouteAssignment_Impl: Sized {
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
impl IVpnRouteAssignment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnRouteAssignment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnRouteAssignment_Vtbl {
        unsafe extern "system" fn SetIpv4InclusionRoutes<Impl: IVpnRouteAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpv4InclusionRoutes(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetIpv6InclusionRoutes<Impl: IVpnRouteAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpv6InclusionRoutes(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Ipv4InclusionRoutes<Impl: IVpnRouteAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Ipv6InclusionRoutes<Impl: IVpnRouteAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetIpv4ExclusionRoutes<Impl: IVpnRouteAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpv4ExclusionRoutes(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetIpv6ExclusionRoutes<Impl: IVpnRouteAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIpv6ExclusionRoutes(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Ipv4ExclusionRoutes<Impl: IVpnRouteAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Ipv6ExclusionRoutes<Impl: IVpnRouteAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetExcludeLocalSubnets<Impl: IVpnRouteAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExcludeLocalSubnets(value).into()
        }
        unsafe extern "system" fn ExcludeLocalSubnets<Impl: IVpnRouteAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
pub trait IVpnRouteFactory_Impl: Sized {
    fn CreateVpnRoute(&mut self, address: &::core::option::Option<super::HostName>, prefixsize: u8) -> ::windows::core::Result<VpnRoute>;
}
impl ::windows::core::RuntimeName for IVpnRouteFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnRouteFactory";
}
impl IVpnRouteFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnRouteFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnRouteFactory_Vtbl {
        unsafe extern "system" fn CreateVpnRoute<Impl: IVpnRouteFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: ::windows::core::RawPtr, prefixsize: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnSystemHealth_Impl: Sized {
    fn StatementOfHealth(&mut self) -> ::windows::core::Result<super::super::Storage::Streams::Buffer>;
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnSystemHealth {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnSystemHealth";
}
#[cfg(all(feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IVpnSystemHealth_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnSystemHealth_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnSystemHealth_Vtbl {
        unsafe extern "system" fn StatementOfHealth<Impl: IVpnSystemHealth_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IVpnTrafficFilter_Impl: Sized {
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
impl IVpnTrafficFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnTrafficFilter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnTrafficFilter_Vtbl {
        unsafe extern "system" fn AppId<Impl: IVpnTrafficFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAppId<Impl: IVpnTrafficFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAppId(&*(&value as *const <VpnAppId as ::windows::core::Abi>::Abi as *const <VpnAppId as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppClaims<Impl: IVpnTrafficFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Protocol<Impl: IVpnTrafficFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnIPProtocol) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetProtocol<Impl: IVpnTrafficFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnIPProtocol) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtocol(value).into()
        }
        unsafe extern "system" fn LocalPortRanges<Impl: IVpnTrafficFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemotePortRanges<Impl: IVpnTrafficFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LocalAddressRanges<Impl: IVpnTrafficFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteAddressRanges<Impl: IVpnTrafficFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RoutingPolicyType<Impl: IVpnTrafficFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut VpnRoutingPolicyType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetRoutingPolicyType<Impl: IVpnTrafficFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: VpnRoutingPolicyType) -> ::windows::core::HRESULT {
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
pub trait IVpnTrafficFilterAssignment_Impl: Sized {
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
impl IVpnTrafficFilterAssignment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnTrafficFilterAssignment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnTrafficFilterAssignment_Vtbl {
        unsafe extern "system" fn TrafficFilterList<Impl: IVpnTrafficFilterAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AllowOutbound<Impl: IVpnTrafficFilterAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowOutbound<Impl: IVpnTrafficFilterAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowOutbound(value).into()
        }
        unsafe extern "system" fn AllowInbound<Impl: IVpnTrafficFilterAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetAllowInbound<Impl: IVpnTrafficFilterAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
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
pub trait IVpnTrafficFilterFactory_Impl: Sized {
    fn Create(&mut self, appid: &::core::option::Option<VpnAppId>) -> ::windows::core::Result<VpnTrafficFilter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnTrafficFilterFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnTrafficFilterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnTrafficFilterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVpnTrafficFilterFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVpnTrafficFilterFactory_Vtbl {
        unsafe extern "system" fn Create<Impl: IVpnTrafficFilterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, appid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
