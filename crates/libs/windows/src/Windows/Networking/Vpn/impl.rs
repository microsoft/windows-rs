#[cfg(feature = "implement_exclusive")]
pub trait IVpnAppIdImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<VpnAppIdType>;
    fn SetType(&self, value: VpnAppIdType) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnAppId {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnAppId";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnAppIdVtbl {
    pub const fn new<Impl: IVpnAppIdImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnAppIdVtbl {
        unsafe extern "system" fn Type<Impl: IVpnAppIdImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnAppIdType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IVpnAppIdImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VpnAppIdType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetType(value).into()
        }
        unsafe extern "system" fn Value<Impl: IVpnAppIdImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IVpnAppIdImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnAppId>, base.5, Type::<Impl, OFFSET>, SetType::<Impl, OFFSET>, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnAppIdFactoryImpl: Sized {
    fn Create(&self, r#type: VpnAppIdType, value: &::windows::core::HSTRING) -> ::windows::core::Result<VpnAppId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnAppIdFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnAppIdFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnAppIdFactoryVtbl {
    pub const fn new<Impl: IVpnAppIdFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnAppIdFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IVpnAppIdFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: VpnAppIdType, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(r#type, &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnAppIdFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannelImpl: Sized {
    fn AssociateTransport(&self, mainoutertunneltransport: &::core::option::Option<::windows::core::IInspectable>, optionaloutertunneltransport: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Start(&self, assignedclientipv4list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, assignedclientipv6list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, routescope: &::core::option::Option<VpnRouteAssignment>, namespacescope: &::core::option::Option<VpnNamespaceAssignment>, mtusize: u32, maxframesize: u32, optimizeforlowcostnetwork: bool, mainoutertunneltransport: &::core::option::Option<::windows::core::IInspectable>, optionaloutertunneltransport: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn RequestCredentials(&self, credtype: VpnCredentialType, isretry: bool, issinglesignoncredential: bool, certificate: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<VpnPickedCredential>;
    fn RequestVpnPacketBuffer(&self, r#type: VpnDataPathType, vpnpacketbuffer: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn LogDiagnosticMessage(&self, message: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Configuration(&self) -> ::windows::core::Result<VpnChannelConfiguration>;
    fn ActivityChange(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivityChange(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetPlugInContext(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn PlugInContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SystemHealth(&self) -> ::windows::core::Result<VpnSystemHealth>;
    fn RequestCustomPrompt(&self, customprompt: &::core::option::Option<super::super::Foundation::Collections::IVectorView<IVpnCustomPrompt>>) -> ::windows::core::Result<()>;
    fn SetErrorMessage(&self, message: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetAllowedSslTlsVersions(&self, tunneltransport: &::core::option::Option<::windows::core::IInspectable>, usetls12: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannel {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannel";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannelVtbl {
    pub const fn new<Impl: IVpnChannelImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnChannelVtbl {
        unsafe extern "system" fn AssociateTransport<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AssociateTransport(&*(&mainoutertunneltransport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&optionaloutertunneltransport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, routescope: ::windows::core::RawPtr, namespacescope: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, optimizeforlowcostnetwork: bool, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn Stop<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn RequestCredentials<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, isretry: bool, issinglesignoncredential: bool, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestCredentials(credtype, isretry, issinglesignoncredential, &*(&certificate as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestVpnPacketBuffer<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: VpnDataPathType, vpnpacketbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RequestVpnPacketBuffer(r#type, ::core::mem::transmute_copy(&vpnpacketbuffer)).into()
        }
        unsafe extern "system" fn LogDiagnosticMessage<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).LogDiagnosticMessage(&*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Id<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configuration<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Configuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityChange<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivityChange(&*(&handler as *const <super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivityChange<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveActivityChange(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetPlugInContext<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlugInContext(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlugInContext<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlugInContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SystemHealth<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SystemHealth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCustomPrompt<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customprompt: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RequestCustomPrompt(&*(&customprompt as *const <super::super::Foundation::Collections::IVectorView<IVpnCustomPrompt> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<IVpnCustomPrompt> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetErrorMessage<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetErrorMessage(&*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetAllowedSslTlsVersions<Impl: IVpnChannelImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tunneltransport: *mut ::core::ffi::c_void, usetls12: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowedSslTlsVersions(&*(&tunneltransport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), usetls12).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IVpnChannel>,
            base.5,
            AssociateTransport::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            RequestCredentials::<Impl, OFFSET>,
            RequestVpnPacketBuffer::<Impl, OFFSET>,
            LogDiagnosticMessage::<Impl, OFFSET>,
            Id::<Impl, OFFSET>,
            Configuration::<Impl, OFFSET>,
            ActivityChange::<Impl, OFFSET>,
            RemoveActivityChange::<Impl, OFFSET>,
            SetPlugInContext::<Impl, OFFSET>,
            PlugInContext::<Impl, OFFSET>,
            SystemHealth::<Impl, OFFSET>,
            RequestCustomPrompt::<Impl, OFFSET>,
            SetErrorMessage::<Impl, OFFSET>,
            SetAllowedSslTlsVersions::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannel2Impl: Sized {
    fn StartWithMainTransport(&self, assignedclientipv4list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, assignedclientipv6list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, assignedroutes: &::core::option::Option<VpnRouteAssignment>, assigneddomainname: &::core::option::Option<VpnDomainNameAssignment>, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn StartExistingTransports(&self, assignedclientipv4list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, assignedclientipv6list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, assignedroutes: &::core::option::Option<VpnRouteAssignment>, assigneddomainname: &::core::option::Option<VpnDomainNameAssignment>, mtusize: u32, maxframesize: u32, reserved: bool) -> ::windows::core::Result<()>;
    fn ActivityStateChange(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityStateChangedArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivityStateChange(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetVpnSendPacketBuffer(&self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn GetVpnReceivePacketBuffer(&self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn RequestCustomPromptAsync(&self, custompromptelement: &::core::option::Option<super::super::Foundation::Collections::IVectorView<IVpnCustomPromptElement>>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RequestCredentialsWithCertificateAsync(&self, credtype: VpnCredentialType, credoptions: u32, certificate: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>>;
    fn RequestCredentialsWithOptionsAsync(&self, credtype: VpnCredentialType, credoptions: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>>;
    fn RequestCredentialsSimpleAsync(&self, credtype: VpnCredentialType) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnCredential>>;
    fn TerminateConnection(&self, message: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StartWithTrafficFilter(&self, assignedclientipv4list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, assignedclientipv6list: &::core::option::Option<super::super::Foundation::Collections::IVectorView<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, assignedroutes: &::core::option::Option<VpnRouteAssignment>, assignednamespace: &::core::option::Option<VpnDomainNameAssignment>, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: &::core::option::Option<::windows::core::IInspectable>, optionaloutertunneltransport: &::core::option::Option<::windows::core::IInspectable>, assignedtrafficfilters: &::core::option::Option<VpnTrafficFilterAssignment>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannel2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannel2";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannel2Vtbl {
    pub const fn new<Impl: IVpnChannel2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnChannel2Vtbl {
        unsafe extern "system" fn StartWithMainTransport<Impl: IVpnChannel2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assigneddomainname: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn StartExistingTransports<Impl: IVpnChannel2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assigneddomainname: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn ActivityStateChange<Impl: IVpnChannel2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivityStateChange(&*(&handler as *const <super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityStateChangedArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<VpnChannel, VpnChannelActivityStateChangedArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveActivityStateChange<Impl: IVpnChannel2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveActivityStateChange(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetVpnSendPacketBuffer<Impl: IVpnChannel2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVpnSendPacketBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVpnReceivePacketBuffer<Impl: IVpnChannel2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVpnReceivePacketBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCustomPromptAsync<Impl: IVpnChannel2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, custompromptelement: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestCustomPromptAsync(&*(&custompromptelement as *const <super::super::Foundation::Collections::IVectorView<IVpnCustomPromptElement> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<IVpnCustomPromptElement> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCredentialsWithCertificateAsync<Impl: IVpnChannel2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, credoptions: u32, certificate: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestCredentialsWithCertificateAsync(credtype, credoptions, &*(&certificate as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::Abi>::Abi as *const <super::super::Security::Cryptography::Certificates::Certificate as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCredentialsWithOptionsAsync<Impl: IVpnChannel2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, credoptions: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestCredentialsWithOptionsAsync(credtype, credoptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestCredentialsSimpleAsync<Impl: IVpnChannel2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, credtype: VpnCredentialType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestCredentialsSimpleAsync(credtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TerminateConnection<Impl: IVpnChannel2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).TerminateConnection(&*(&message as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartWithTrafficFilter<Impl: IVpnChannel2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, assignedclientipv4list: ::windows::core::RawPtr, assignedclientipv6list: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assignednamespace: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool, mainoutertunneltransport: *mut ::core::ffi::c_void, optionaloutertunneltransport: *mut ::core::ffi::c_void, assignedtrafficfilters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IVpnChannel2>,
            base.5,
            StartWithMainTransport::<Impl, OFFSET>,
            StartExistingTransports::<Impl, OFFSET>,
            ActivityStateChange::<Impl, OFFSET>,
            RemoveActivityStateChange::<Impl, OFFSET>,
            GetVpnSendPacketBuffer::<Impl, OFFSET>,
            GetVpnReceivePacketBuffer::<Impl, OFFSET>,
            RequestCustomPromptAsync::<Impl, OFFSET>,
            RequestCredentialsWithCertificateAsync::<Impl, OFFSET>,
            RequestCredentialsWithOptionsAsync::<Impl, OFFSET>,
            RequestCredentialsSimpleAsync::<Impl, OFFSET>,
            TerminateConnection::<Impl, OFFSET>,
            StartWithTrafficFilter::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannel4Impl: Sized {
    fn AddAndAssociateTransport(&self, transport: &::core::option::Option<::windows::core::IInspectable>, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn StartWithMultipleTransports(&self, assignedclientipv4addresses: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>, assignedclientipv6addresses: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>, vpninterfaceid: &::core::option::Option<VpnInterfaceId>, assignedroutes: &::core::option::Option<VpnRouteAssignment>, assignednamespace: &::core::option::Option<VpnDomainNameAssignment>, mtusize: u32, maxframesize: u32, reserved: bool, transports: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>, assignedtrafficfilters: &::core::option::Option<VpnTrafficFilterAssignment>) -> ::windows::core::Result<()>;
    fn ReplaceAndAssociateTransport(&self, transport: &::core::option::Option<::windows::core::IInspectable>, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn StartReconnectingTransport(&self, transport: &::core::option::Option<::windows::core::IInspectable>, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetSlotTypeForTransportContext(&self, context: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::Sockets::ControlChannelTriggerStatus>;
    fn CurrentRequestTransportContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannel4 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannel4";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannel4Vtbl {
    pub const fn new<Impl: IVpnChannel4Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnChannel4Vtbl {
        unsafe extern "system" fn AddAndAssociateTransport<Impl: IVpnChannel4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddAndAssociateTransport(&*(&transport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartWithMultipleTransports<Impl: IVpnChannel4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, assignedclientipv4addresses: ::windows::core::RawPtr, assignedclientipv6addresses: ::windows::core::RawPtr, vpninterfaceid: ::windows::core::RawPtr, assignedroutes: ::windows::core::RawPtr, assignednamespace: ::windows::core::RawPtr, mtusize: u32, maxframesize: u32, reserved: bool, transports: ::windows::core::RawPtr, assignedtrafficfilters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn ReplaceAndAssociateTransport<Impl: IVpnChannel4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ReplaceAndAssociateTransport(&*(&transport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn StartReconnectingTransport<Impl: IVpnChannel4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transport: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).StartReconnectingTransport(&*(&transport as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetSlotTypeForTransportContext<Impl: IVpnChannel4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut super::Sockets::ControlChannelTriggerStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSlotTypeForTransportContext(&*(&context as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRequestTransportContext<Impl: IVpnChannel4Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CurrentRequestTransportContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnChannel4>, base.5, AddAndAssociateTransport::<Impl, OFFSET>, StartWithMultipleTransports::<Impl, OFFSET>, ReplaceAndAssociateTransport::<Impl, OFFSET>, StartReconnectingTransport::<Impl, OFFSET>, GetSlotTypeForTransportContext::<Impl, OFFSET>, CurrentRequestTransportContext::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannel5Impl: Sized {
    fn AppendVpnReceivePacketBuffer(&self, decapsulatedpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn AppendVpnSendPacketBuffer(&self, encapsulatedpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn FlushVpnReceivePacketBuffers(&self) -> ::windows::core::Result<()>;
    fn FlushVpnSendPacketBuffers(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannel5 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannel5";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannel5Vtbl {
    pub const fn new<Impl: IVpnChannel5Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnChannel5Vtbl {
        unsafe extern "system" fn AppendVpnReceivePacketBuffer<Impl: IVpnChannel5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, decapsulatedpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AppendVpnReceivePacketBuffer(&*(&decapsulatedpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppendVpnSendPacketBuffer<Impl: IVpnChannel5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encapsulatedpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AppendVpnSendPacketBuffer(&*(&encapsulatedpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn FlushVpnReceivePacketBuffers<Impl: IVpnChannel5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FlushVpnReceivePacketBuffers().into()
        }
        unsafe extern "system" fn FlushVpnSendPacketBuffers<Impl: IVpnChannel5Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).FlushVpnSendPacketBuffers().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnChannel5>, base.5, AppendVpnReceivePacketBuffer::<Impl, OFFSET>, AppendVpnSendPacketBuffer::<Impl, OFFSET>, FlushVpnReceivePacketBuffers::<Impl, OFFSET>, FlushVpnSendPacketBuffers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannel6Impl: Sized {
    fn ActivateForeground(&self, packagerelativeappid: &::windows::core::HSTRING, sharedcontext: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannel6 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannel6";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannel6Vtbl {
    pub const fn new<Impl: IVpnChannel6Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnChannel6Vtbl {
        unsafe extern "system" fn ActivateForeground<Impl: IVpnChannel6Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagerelativeappid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sharedcontext: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivateForeground(&*(&packagerelativeappid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&sharedcontext as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnChannel6>, base.5, ActivateForeground::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannelActivityEventArgsImpl: Sized {
    fn Type(&self) -> ::windows::core::Result<VpnChannelActivityEventType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannelActivityEventArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelActivityEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannelActivityEventArgsVtbl {
    pub const fn new<Impl: IVpnChannelActivityEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnChannelActivityEventArgsVtbl {
        unsafe extern "system" fn Type<Impl: IVpnChannelActivityEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnChannelActivityEventType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnChannelActivityEventArgs>, base.5, Type::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannelActivityStateChangedArgsImpl: Sized {
    fn ActivityState(&self) -> ::windows::core::Result<VpnChannelActivityEventType>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannelActivityStateChangedArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelActivityStateChangedArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannelActivityStateChangedArgsVtbl {
    pub const fn new<Impl: IVpnChannelActivityStateChangedArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnChannelActivityStateChangedArgsVtbl {
        unsafe extern "system" fn ActivityState<Impl: IVpnChannelActivityStateChangedArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnChannelActivityEventType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivityState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnChannelActivityStateChangedArgs>, base.5, ActivityState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannelConfigurationImpl: Sized {
    fn ServerServiceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ServerHostNameList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::HostName>>;
    fn CustomField(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannelConfiguration {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelConfiguration";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannelConfigurationVtbl {
    pub const fn new<Impl: IVpnChannelConfigurationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnChannelConfigurationVtbl {
        unsafe extern "system" fn ServerServiceName<Impl: IVpnChannelConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerServiceName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerHostNameList<Impl: IVpnChannelConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerHostNameList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomField<Impl: IVpnChannelConfigurationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CustomField() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnChannelConfiguration>, base.5, ServerServiceName::<Impl, OFFSET>, ServerHostNameList::<Impl, OFFSET>, CustomField::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnChannelConfiguration2Impl: Sized {
    fn ServerUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Uri>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnChannelConfiguration2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelConfiguration2";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnChannelConfiguration2Vtbl {
    pub const fn new<Impl: IVpnChannelConfiguration2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnChannelConfiguration2Vtbl {
        unsafe extern "system" fn ServerUris<Impl: IVpnChannelConfiguration2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnChannelConfiguration2>, base.5, ServerUris::<Impl, OFFSET>)
    }
}
pub trait IVpnChannelStaticsImpl: Sized {
    fn ProcessEventAsync(&self, thirdpartyplugin: &::core::option::Option<::windows::core::IInspectable>, event: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVpnChannelStatics {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnChannelStatics";
}
impl IVpnChannelStaticsVtbl {
    pub const fn new<Impl: IVpnChannelStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnChannelStaticsVtbl {
        unsafe extern "system" fn ProcessEventAsync<Impl: IVpnChannelStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, thirdpartyplugin: *mut ::core::ffi::c_void, event: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).ProcessEventAsync(&*(&thirdpartyplugin as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&event as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnChannelStatics>, base.5, ProcessEventAsync::<Impl, OFFSET>)
    }
}
pub trait IVpnCredentialImpl: Sized {
    fn PasskeyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn CertificateCredential(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn AdditionalPin(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OldPasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
}
impl ::windows::core::RuntimeName for IVpnCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCredential";
}
impl IVpnCredentialVtbl {
    pub const fn new<Impl: IVpnCredentialImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnCredentialVtbl {
        unsafe extern "system" fn PasskeyCredential<Impl: IVpnCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PasskeyCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CertificateCredential<Impl: IVpnCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CertificateCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdditionalPin<Impl: IVpnCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdditionalPin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldPasswordCredential<Impl: IVpnCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldPasswordCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnCredential>, base.5, PasskeyCredential::<Impl, OFFSET>, CertificateCredential::<Impl, OFFSET>, AdditionalPin::<Impl, OFFSET>, OldPasswordCredential::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomCheckBoxImpl: Sized + IVpnCustomPromptImpl {
    fn SetInitialCheckState(&self, value: bool) -> ::windows::core::Result<()>;
    fn InitialCheckState(&self) -> ::windows::core::Result<bool>;
    fn Checked(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomCheckBox {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomCheckBox";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomCheckBoxVtbl {
    pub const fn new<Impl: IVpnCustomCheckBoxImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnCustomCheckBoxVtbl {
        unsafe extern "system" fn SetInitialCheckState<Impl: IVpnCustomCheckBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInitialCheckState(value).into()
        }
        unsafe extern "system" fn InitialCheckState<Impl: IVpnCustomCheckBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitialCheckState() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Checked<Impl: IVpnCustomCheckBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Checked() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnCustomCheckBox>, base.5, SetInitialCheckState::<Impl, OFFSET>, InitialCheckState::<Impl, OFFSET>, Checked::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomComboBoxImpl: Sized + IVpnCustomPromptImpl {
    fn SetOptionsText(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>) -> ::windows::core::Result<()>;
    fn OptionsText(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Selected(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomComboBox {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomComboBox";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomComboBoxVtbl {
    pub const fn new<Impl: IVpnCustomComboBoxImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnCustomComboBoxVtbl {
        unsafe extern "system" fn SetOptionsText<Impl: IVpnCustomComboBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetOptionsText(&*(&value as *const <super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OptionsText<Impl: IVpnCustomComboBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OptionsText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Selected<Impl: IVpnCustomComboBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Selected() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnCustomComboBox>, base.5, SetOptionsText::<Impl, OFFSET>, OptionsText::<Impl, OFFSET>, Selected::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomEditBoxImpl: Sized + IVpnCustomPromptImpl {
    fn SetDefaultText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DefaultText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNoEcho(&self, value: bool) -> ::windows::core::Result<()>;
    fn NoEcho(&self) -> ::windows::core::Result<bool>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomEditBox {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomEditBox";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomEditBoxVtbl {
    pub const fn new<Impl: IVpnCustomEditBoxImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnCustomEditBoxVtbl {
        unsafe extern "system" fn SetDefaultText<Impl: IVpnCustomEditBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDefaultText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DefaultText<Impl: IVpnCustomEditBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DefaultText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNoEcho<Impl: IVpnCustomEditBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNoEcho(value).into()
        }
        unsafe extern "system" fn NoEcho<Impl: IVpnCustomEditBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NoEcho() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: IVpnCustomEditBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnCustomEditBox>, base.5, SetDefaultText::<Impl, OFFSET>, DefaultText::<Impl, OFFSET>, SetNoEcho::<Impl, OFFSET>, NoEcho::<Impl, OFFSET>, Text::<Impl, OFFSET>)
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
    pub const fn new<Impl: IVpnCustomErrorBoxImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnCustomErrorBoxVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnCustomErrorBox>, base.5)
    }
}
pub trait IVpnCustomPromptImpl: Sized {
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()>;
    fn Compulsory(&self) -> ::windows::core::Result<bool>;
    fn SetBordered(&self, value: bool) -> ::windows::core::Result<()>;
    fn Bordered(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IVpnCustomPrompt {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPrompt";
}
impl IVpnCustomPromptVtbl {
    pub const fn new<Impl: IVpnCustomPromptImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnCustomPromptVtbl {
        unsafe extern "system" fn SetLabel<Impl: IVpnCustomPromptImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetLabel(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Label<Impl: IVpnCustomPromptImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Label() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompulsory<Impl: IVpnCustomPromptImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCompulsory(value).into()
        }
        unsafe extern "system" fn Compulsory<Impl: IVpnCustomPromptImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compulsory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBordered<Impl: IVpnCustomPromptImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetBordered(value).into()
        }
        unsafe extern "system" fn Bordered<Impl: IVpnCustomPromptImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Bordered() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnCustomPrompt>, base.5, SetLabel::<Impl, OFFSET>, Label::<Impl, OFFSET>, SetCompulsory::<Impl, OFFSET>, Compulsory::<Impl, OFFSET>, SetBordered::<Impl, OFFSET>, Bordered::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomPromptBooleanInputImpl: Sized + IVpnCustomPromptElementImpl {
    fn SetInitialValue(&self, value: bool) -> ::windows::core::Result<()>;
    fn InitialValue(&self) -> ::windows::core::Result<bool>;
    fn Value(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomPromptBooleanInput {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptBooleanInput";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomPromptBooleanInputVtbl {
    pub const fn new<Impl: IVpnCustomPromptBooleanInputImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnCustomPromptBooleanInputVtbl {
        unsafe extern "system" fn SetInitialValue<Impl: IVpnCustomPromptBooleanInputImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetInitialValue(value).into()
        }
        unsafe extern "system" fn InitialValue<Impl: IVpnCustomPromptBooleanInputImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitialValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IVpnCustomPromptBooleanInputImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnCustomPromptBooleanInput>, base.5, SetInitialValue::<Impl, OFFSET>, InitialValue::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
pub trait IVpnCustomPromptElementImpl: Sized {
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompulsory(&self, value: bool) -> ::windows::core::Result<()>;
    fn Compulsory(&self) -> ::windows::core::Result<bool>;
    fn SetEmphasized(&self, value: bool) -> ::windows::core::Result<()>;
    fn Emphasized(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IVpnCustomPromptElement {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptElement";
}
impl IVpnCustomPromptElementVtbl {
    pub const fn new<Impl: IVpnCustomPromptElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnCustomPromptElementVtbl {
        unsafe extern "system" fn SetDisplayName<Impl: IVpnCustomPromptElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisplayName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayName<Impl: IVpnCustomPromptElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompulsory<Impl: IVpnCustomPromptElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCompulsory(value).into()
        }
        unsafe extern "system" fn Compulsory<Impl: IVpnCustomPromptElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compulsory() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmphasized<Impl: IVpnCustomPromptElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEmphasized(value).into()
        }
        unsafe extern "system" fn Emphasized<Impl: IVpnCustomPromptElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Emphasized() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnCustomPromptElement>, base.5, SetDisplayName::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, SetCompulsory::<Impl, OFFSET>, Compulsory::<Impl, OFFSET>, SetEmphasized::<Impl, OFFSET>, Emphasized::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomPromptOptionSelectorImpl: Sized + IVpnCustomPromptElementImpl {
    fn Options(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SelectedIndex(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomPromptOptionSelector {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptOptionSelector";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomPromptOptionSelectorVtbl {
    pub const fn new<Impl: IVpnCustomPromptOptionSelectorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnCustomPromptOptionSelectorVtbl {
        unsafe extern "system" fn Options<Impl: IVpnCustomPromptOptionSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectedIndex<Impl: IVpnCustomPromptOptionSelectorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SelectedIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnCustomPromptOptionSelector>, base.5, Options::<Impl, OFFSET>, SelectedIndex::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomPromptTextImpl: Sized + IVpnCustomPromptElementImpl {
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomPromptText {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptText";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomPromptTextVtbl {
    pub const fn new<Impl: IVpnCustomPromptTextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnCustomPromptTextVtbl {
        unsafe extern "system" fn SetText<Impl: IVpnCustomPromptTextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Text<Impl: IVpnCustomPromptTextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnCustomPromptText>, base.5, SetText::<Impl, OFFSET>, Text::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomPromptTextInputImpl: Sized + IVpnCustomPromptElementImpl {
    fn SetPlaceholderText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PlaceholderText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIsTextHidden(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsTextHidden(&self) -> ::windows::core::Result<bool>;
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomPromptTextInput {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomPromptTextInput";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomPromptTextInputVtbl {
    pub const fn new<Impl: IVpnCustomPromptTextInputImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnCustomPromptTextInputVtbl {
        unsafe extern "system" fn SetPlaceholderText<Impl: IVpnCustomPromptTextInputImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPlaceholderText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PlaceholderText<Impl: IVpnCustomPromptTextInputImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PlaceholderText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsTextHidden<Impl: IVpnCustomPromptTextInputImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIsTextHidden(value).into()
        }
        unsafe extern "system" fn IsTextHidden<Impl: IVpnCustomPromptTextInputImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsTextHidden() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Text<Impl: IVpnCustomPromptTextInputImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Text() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnCustomPromptTextInput>, base.5, SetPlaceholderText::<Impl, OFFSET>, PlaceholderText::<Impl, OFFSET>, SetIsTextHidden::<Impl, OFFSET>, IsTextHidden::<Impl, OFFSET>, Text::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnCustomTextBoxImpl: Sized + IVpnCustomPromptImpl {
    fn SetDisplayText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnCustomTextBox {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnCustomTextBox";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnCustomTextBoxVtbl {
    pub const fn new<Impl: IVpnCustomTextBoxImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnCustomTextBoxVtbl {
        unsafe extern "system" fn SetDisplayText<Impl: IVpnCustomTextBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDisplayText(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DisplayText<Impl: IVpnCustomTextBoxImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisplayText() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnCustomTextBox>, base.5, SetDisplayText::<Impl, OFFSET>, DisplayText::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnDomainNameAssignmentImpl: Sized {
    fn DomainNameList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>;
    fn SetProxyAutoConfigurationUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ProxyAutoConfigurationUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnDomainNameAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameAssignment";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnDomainNameAssignmentVtbl {
    pub const fn new<Impl: IVpnDomainNameAssignmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnDomainNameAssignmentVtbl {
        unsafe extern "system" fn DomainNameList<Impl: IVpnDomainNameAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DomainNameList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyAutoConfigurationUri<Impl: IVpnDomainNameAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProxyAutoConfigurationUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyAutoConfigurationUri<Impl: IVpnDomainNameAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProxyAutoConfigurationUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnDomainNameAssignment>, base.5, DomainNameList::<Impl, OFFSET>, SetProxyAutoConfigurationUri::<Impl, OFFSET>, ProxyAutoConfigurationUri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnDomainNameInfoImpl: Sized {
    fn SetDomainName(&self, value: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn DomainName(&self) -> ::windows::core::Result<super::HostName>;
    fn SetDomainNameType(&self, value: VpnDomainNameType) -> ::windows::core::Result<()>;
    fn DomainNameType(&self) -> ::windows::core::Result<VpnDomainNameType>;
    fn DnsServers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>>;
    fn WebProxyServers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnDomainNameInfo {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnDomainNameInfoVtbl {
    pub const fn new<Impl: IVpnDomainNameInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnDomainNameInfoVtbl {
        unsafe extern "system" fn SetDomainName<Impl: IVpnDomainNameInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDomainName(&*(&value as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DomainName<Impl: IVpnDomainNameInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DomainName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomainNameType<Impl: IVpnDomainNameInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VpnDomainNameType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDomainNameType(value).into()
        }
        unsafe extern "system" fn DomainNameType<Impl: IVpnDomainNameInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnDomainNameType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DomainNameType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DnsServers<Impl: IVpnDomainNameInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DnsServers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WebProxyServers<Impl: IVpnDomainNameInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebProxyServers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnDomainNameInfo>, base.5, SetDomainName::<Impl, OFFSET>, DomainName::<Impl, OFFSET>, SetDomainNameType::<Impl, OFFSET>, DomainNameType::<Impl, OFFSET>, DnsServers::<Impl, OFFSET>, WebProxyServers::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnDomainNameInfo2Impl: Sized {
    fn WebProxyUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnDomainNameInfo2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameInfo2";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnDomainNameInfo2Vtbl {
    pub const fn new<Impl: IVpnDomainNameInfo2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnDomainNameInfo2Vtbl {
        unsafe extern "system" fn WebProxyUris<Impl: IVpnDomainNameInfo2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebProxyUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnDomainNameInfo2>, base.5, WebProxyUris::<Impl, OFFSET>)
    }
}
pub trait IVpnDomainNameInfoFactoryImpl: Sized {
    fn CreateVpnDomainNameInfo(&self, name: &::windows::core::HSTRING, nametype: VpnDomainNameType, dnsserverlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>, proxyserverlist: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::HostName>>) -> ::windows::core::Result<VpnDomainNameInfo>;
}
impl ::windows::core::RuntimeName for IVpnDomainNameInfoFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnDomainNameInfoFactory";
}
impl IVpnDomainNameInfoFactoryVtbl {
    pub const fn new<Impl: IVpnDomainNameInfoFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnDomainNameInfoFactoryVtbl {
        unsafe extern "system" fn CreateVpnDomainNameInfo<Impl: IVpnDomainNameInfoFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, nametype: VpnDomainNameType, dnsserverlist: ::windows::core::RawPtr, proxyserverlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnDomainNameInfoFactory>, base.5, CreateVpnDomainNameInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnForegroundActivatedEventArgsImpl: Sized {
    fn ProfileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SharedContext(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
    fn ActivationOperation(&self) -> ::windows::core::Result<VpnForegroundActivationOperation>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnForegroundActivatedEventArgs {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnForegroundActivatedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnForegroundActivatedEventArgsVtbl {
    pub const fn new<Impl: IVpnForegroundActivatedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnForegroundActivatedEventArgsVtbl {
        unsafe extern "system" fn ProfileName<Impl: IVpnForegroundActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProfileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SharedContext<Impl: IVpnForegroundActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SharedContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivationOperation<Impl: IVpnForegroundActivatedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivationOperation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnForegroundActivatedEventArgs>, base.5, ProfileName::<Impl, OFFSET>, SharedContext::<Impl, OFFSET>, ActivationOperation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnForegroundActivationOperationImpl: Sized {
    fn Complete(&self, result: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnForegroundActivationOperation {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnForegroundActivationOperation";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnForegroundActivationOperationVtbl {
    pub const fn new<Impl: IVpnForegroundActivationOperationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnForegroundActivationOperationVtbl {
        unsafe extern "system" fn Complete<Impl: IVpnForegroundActivationOperationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Complete(&*(&result as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::ValueSet as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnForegroundActivationOperation>, base.5, Complete::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnInterfaceIdImpl: Sized {
    fn GetAddressInfo(&self, id: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnInterfaceId {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnInterfaceId";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnInterfaceIdVtbl {
    pub const fn new<Impl: IVpnInterfaceIdImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnInterfaceIdVtbl {
        unsafe extern "system" fn GetAddressInfo<Impl: IVpnInterfaceIdImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id_array_size: *mut u32, id: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetAddressInfo(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&id), id_array_size).as_array()).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnInterfaceId>, base.5, GetAddressInfo::<Impl, OFFSET>)
    }
}
pub trait IVpnInterfaceIdFactoryImpl: Sized {
    fn CreateVpnInterfaceId(&self, address: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<VpnInterfaceId>;
}
impl ::windows::core::RuntimeName for IVpnInterfaceIdFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnInterfaceIdFactory";
}
impl IVpnInterfaceIdFactoryVtbl {
    pub const fn new<Impl: IVpnInterfaceIdFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnInterfaceIdFactoryVtbl {
        unsafe extern "system" fn CreateVpnInterfaceId<Impl: IVpnInterfaceIdFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address_array_size: u32, address: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVpnInterfaceId(::core::slice::from_raw_parts(::core::mem::transmute_copy(&address), address_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnInterfaceIdFactory>, base.5, CreateVpnInterfaceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnManagementAgentImpl: Sized {
    fn AddProfileFromXmlAsync(&self, xml: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn AddProfileFromObjectAsync(&self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn UpdateProfileFromXmlAsync(&self, xml: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn UpdateProfileFromObjectAsync(&self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn GetProfilesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IVpnProfile>>>;
    fn DeleteProfileAsync(&self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn ConnectProfileAsync(&self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn ConnectProfileWithPasswordCredentialAsync(&self, profile: &::core::option::Option<IVpnProfile>, passwordcredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
    fn DisconnectProfileAsync(&self, profile: &::core::option::Option<IVpnProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<VpnManagementErrorStatus>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnManagementAgent {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnManagementAgent";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnManagementAgentVtbl {
    pub const fn new<Impl: IVpnManagementAgentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnManagementAgentVtbl {
        unsafe extern "system" fn AddProfileFromXmlAsync<Impl: IVpnManagementAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddProfileFromXmlAsync(&*(&xml as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddProfileFromObjectAsync<Impl: IVpnManagementAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddProfileFromObjectAsync(&*(&profile as *const <IVpnProfile as ::windows::core::Abi>::Abi as *const <IVpnProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateProfileFromXmlAsync<Impl: IVpnManagementAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateProfileFromXmlAsync(&*(&xml as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateProfileFromObjectAsync<Impl: IVpnManagementAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateProfileFromObjectAsync(&*(&profile as *const <IVpnProfile as ::windows::core::Abi>::Abi as *const <IVpnProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProfilesAsync<Impl: IVpnManagementAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProfilesAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProfileAsync<Impl: IVpnManagementAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteProfileAsync(&*(&profile as *const <IVpnProfile as ::windows::core::Abi>::Abi as *const <IVpnProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectProfileAsync<Impl: IVpnManagementAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectProfileAsync(&*(&profile as *const <IVpnProfile as ::windows::core::Abi>::Abi as *const <IVpnProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectProfileWithPasswordCredentialAsync<Impl: IVpnManagementAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, passwordcredential: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectProfileWithPasswordCredentialAsync(&*(&profile as *const <IVpnProfile as ::windows::core::Abi>::Abi as *const <IVpnProfile as ::windows::core::DefaultType>::DefaultType), &*(&passwordcredential as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::Abi>::Abi as *const <super::super::Security::Credentials::PasswordCredential as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectProfileAsync<Impl: IVpnManagementAgentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profile: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisconnectProfileAsync(&*(&profile as *const <IVpnProfile as ::windows::core::Abi>::Abi as *const <IVpnProfile as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnManagementAgent>, base.5, AddProfileFromXmlAsync::<Impl, OFFSET>, AddProfileFromObjectAsync::<Impl, OFFSET>, UpdateProfileFromXmlAsync::<Impl, OFFSET>, UpdateProfileFromObjectAsync::<Impl, OFFSET>, GetProfilesAsync::<Impl, OFFSET>, DeleteProfileAsync::<Impl, OFFSET>, ConnectProfileAsync::<Impl, OFFSET>, ConnectProfileWithPasswordCredentialAsync::<Impl, OFFSET>, DisconnectProfileAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnNamespaceAssignmentImpl: Sized {
    fn SetNamespaceList(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnNamespaceInfo>>) -> ::windows::core::Result<()>;
    fn NamespaceList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnNamespaceInfo>>;
    fn SetProxyAutoConfigUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ProxyAutoConfigUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnNamespaceAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNamespaceAssignment";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnNamespaceAssignmentVtbl {
    pub const fn new<Impl: IVpnNamespaceAssignmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnNamespaceAssignmentVtbl {
        unsafe extern "system" fn SetNamespaceList<Impl: IVpnNamespaceAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNamespaceList(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnNamespaceInfo> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnNamespaceInfo> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn NamespaceList<Impl: IVpnNamespaceAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NamespaceList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyAutoConfigUri<Impl: IVpnNamespaceAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProxyAutoConfigUri(&*(&value as *const <super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProxyAutoConfigUri<Impl: IVpnNamespaceAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProxyAutoConfigUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnNamespaceAssignment>, base.5, SetNamespaceList::<Impl, OFFSET>, NamespaceList::<Impl, OFFSET>, SetProxyAutoConfigUri::<Impl, OFFSET>, ProxyAutoConfigUri::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnNamespaceInfoImpl: Sized {
    fn SetNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Namespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDnsServers(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>) -> ::windows::core::Result<()>;
    fn DnsServers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>>;
    fn SetWebProxyServers(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>) -> ::windows::core::Result<()>;
    fn WebProxyServers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::HostName>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnNamespaceInfo {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNamespaceInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnNamespaceInfoVtbl {
    pub const fn new<Impl: IVpnNamespaceInfoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnNamespaceInfoVtbl {
        unsafe extern "system" fn SetNamespace<Impl: IVpnNamespaceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNamespace(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Namespace<Impl: IVpnNamespaceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Namespace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDnsServers<Impl: IVpnNamespaceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDnsServers(&*(&value as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DnsServers<Impl: IVpnNamespaceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DnsServers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWebProxyServers<Impl: IVpnNamespaceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetWebProxyServers(&*(&value as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<super::HostName> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn WebProxyServers<Impl: IVpnNamespaceInfoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WebProxyServers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnNamespaceInfo>, base.5, SetNamespace::<Impl, OFFSET>, Namespace::<Impl, OFFSET>, SetDnsServers::<Impl, OFFSET>, DnsServers::<Impl, OFFSET>, SetWebProxyServers::<Impl, OFFSET>, WebProxyServers::<Impl, OFFSET>)
    }
}
pub trait IVpnNamespaceInfoFactoryImpl: Sized {
    fn CreateVpnNamespaceInfo(&self, name: &::windows::core::HSTRING, dnsserverlist: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>, proxyserverlist: &::core::option::Option<super::super::Foundation::Collections::IVector<super::HostName>>) -> ::windows::core::Result<VpnNamespaceInfo>;
}
impl ::windows::core::RuntimeName for IVpnNamespaceInfoFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNamespaceInfoFactory";
}
impl IVpnNamespaceInfoFactoryVtbl {
    pub const fn new<Impl: IVpnNamespaceInfoFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnNamespaceInfoFactoryVtbl {
        unsafe extern "system" fn CreateVpnNamespaceInfo<Impl: IVpnNamespaceInfoFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, dnsserverlist: ::windows::core::RawPtr, proxyserverlist: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnNamespaceInfoFactory>, base.5, CreateVpnNamespaceInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnNativeProfileImpl: Sized + IVpnProfileImpl {
    fn Servers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RoutingPolicyType(&self) -> ::windows::core::Result<VpnRoutingPolicyType>;
    fn SetRoutingPolicyType(&self, value: VpnRoutingPolicyType) -> ::windows::core::Result<()>;
    fn NativeProtocolType(&self) -> ::windows::core::Result<VpnNativeProtocolType>;
    fn SetNativeProtocolType(&self, value: VpnNativeProtocolType) -> ::windows::core::Result<()>;
    fn UserAuthenticationMethod(&self) -> ::windows::core::Result<VpnAuthenticationMethod>;
    fn SetUserAuthenticationMethod(&self, value: VpnAuthenticationMethod) -> ::windows::core::Result<()>;
    fn TunnelAuthenticationMethod(&self) -> ::windows::core::Result<VpnAuthenticationMethod>;
    fn SetTunnelAuthenticationMethod(&self, value: VpnAuthenticationMethod) -> ::windows::core::Result<()>;
    fn EapConfiguration(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEapConfiguration(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnNativeProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNativeProfile";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnNativeProfileVtbl {
    pub const fn new<Impl: IVpnNativeProfileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnNativeProfileVtbl {
        unsafe extern "system" fn Servers<Impl: IVpnNativeProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Servers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingPolicyType<Impl: IVpnNativeProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnRoutingPolicyType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoutingPolicyType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoutingPolicyType<Impl: IVpnNativeProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VpnRoutingPolicyType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRoutingPolicyType(value).into()
        }
        unsafe extern "system" fn NativeProtocolType<Impl: IVpnNativeProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnNativeProtocolType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NativeProtocolType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNativeProtocolType<Impl: IVpnNativeProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VpnNativeProtocolType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetNativeProtocolType(value).into()
        }
        unsafe extern "system" fn UserAuthenticationMethod<Impl: IVpnNativeProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnAuthenticationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserAuthenticationMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserAuthenticationMethod<Impl: IVpnNativeProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VpnAuthenticationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetUserAuthenticationMethod(value).into()
        }
        unsafe extern "system" fn TunnelAuthenticationMethod<Impl: IVpnNativeProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnAuthenticationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TunnelAuthenticationMethod() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTunnelAuthenticationMethod<Impl: IVpnNativeProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VpnAuthenticationMethod) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTunnelAuthenticationMethod(value).into()
        }
        unsafe extern "system" fn EapConfiguration<Impl: IVpnNativeProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EapConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEapConfiguration<Impl: IVpnNativeProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetEapConfiguration(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IVpnNativeProfile>,
            base.5,
            Servers::<Impl, OFFSET>,
            RoutingPolicyType::<Impl, OFFSET>,
            SetRoutingPolicyType::<Impl, OFFSET>,
            NativeProtocolType::<Impl, OFFSET>,
            SetNativeProtocolType::<Impl, OFFSET>,
            UserAuthenticationMethod::<Impl, OFFSET>,
            SetUserAuthenticationMethod::<Impl, OFFSET>,
            TunnelAuthenticationMethod::<Impl, OFFSET>,
            SetTunnelAuthenticationMethod::<Impl, OFFSET>,
            EapConfiguration::<Impl, OFFSET>,
            SetEapConfiguration::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnNativeProfile2Impl: Sized {
    fn RequireVpnClientAppUI(&self) -> ::windows::core::Result<bool>;
    fn SetRequireVpnClientAppUI(&self, value: bool) -> ::windows::core::Result<()>;
    fn ConnectionStatus(&self) -> ::windows::core::Result<VpnManagementConnectionStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnNativeProfile2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnNativeProfile2";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnNativeProfile2Vtbl {
    pub const fn new<Impl: IVpnNativeProfile2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnNativeProfile2Vtbl {
        unsafe extern "system" fn RequireVpnClientAppUI<Impl: IVpnNativeProfile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequireVpnClientAppUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequireVpnClientAppUI<Impl: IVpnNativeProfile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRequireVpnClientAppUI(value).into()
        }
        unsafe extern "system" fn ConnectionStatus<Impl: IVpnNativeProfile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnManagementConnectionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnNativeProfile2>, base.5, RequireVpnClientAppUI::<Impl, OFFSET>, SetRequireVpnClientAppUI::<Impl, OFFSET>, ConnectionStatus::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPacketBufferImpl: Sized {
    fn Buffer(&self) -> ::windows::core::Result<super::super::Storage::Streams::Buffer>;
    fn SetStatus(&self, value: VpnPacketBufferStatus) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<VpnPacketBufferStatus>;
    fn SetTransportAffinity(&self, value: u32) -> ::windows::core::Result<()>;
    fn TransportAffinity(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnPacketBuffer {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBuffer";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnPacketBufferVtbl {
    pub const fn new<Impl: IVpnPacketBufferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnPacketBufferVtbl {
        unsafe extern "system" fn Buffer<Impl: IVpnPacketBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Buffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Impl: IVpnPacketBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VpnPacketBufferStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn Status<Impl: IVpnPacketBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnPacketBufferStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransportAffinity<Impl: IVpnPacketBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransportAffinity(value).into()
        }
        unsafe extern "system" fn TransportAffinity<Impl: IVpnPacketBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransportAffinity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnPacketBuffer>, base.5, Buffer::<Impl, OFFSET>, SetStatus::<Impl, OFFSET>, Status::<Impl, OFFSET>, SetTransportAffinity::<Impl, OFFSET>, TransportAffinity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPacketBuffer2Impl: Sized {
    fn AppId(&self) -> ::windows::core::Result<VpnAppId>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnPacketBuffer2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBuffer2";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnPacketBuffer2Vtbl {
    pub const fn new<Impl: IVpnPacketBuffer2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnPacketBuffer2Vtbl {
        unsafe extern "system" fn AppId<Impl: IVpnPacketBuffer2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnPacketBuffer2>, base.5, AppId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPacketBuffer3Impl: Sized {
    fn SetTransportContext(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn TransportContext(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnPacketBuffer3 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBuffer3";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnPacketBuffer3Vtbl {
    pub const fn new<Impl: IVpnPacketBuffer3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnPacketBuffer3Vtbl {
        unsafe extern "system" fn SetTransportContext<Impl: IVpnPacketBuffer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTransportContext(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn TransportContext<Impl: IVpnPacketBuffer3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TransportContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnPacketBuffer3>, base.5, SetTransportContext::<Impl, OFFSET>, TransportContext::<Impl, OFFSET>)
    }
}
pub trait IVpnPacketBufferFactoryImpl: Sized {
    fn CreateVpnPacketBuffer(&self, parentbuffer: &::core::option::Option<VpnPacketBuffer>, offset: u32, length: u32) -> ::windows::core::Result<VpnPacketBuffer>;
}
impl ::windows::core::RuntimeName for IVpnPacketBufferFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBufferFactory";
}
impl IVpnPacketBufferFactoryVtbl {
    pub const fn new<Impl: IVpnPacketBufferFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnPacketBufferFactoryVtbl {
        unsafe extern "system" fn CreateVpnPacketBuffer<Impl: IVpnPacketBufferFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parentbuffer: ::windows::core::RawPtr, offset: u32, length: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVpnPacketBuffer(&*(&parentbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType), offset, length) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnPacketBufferFactory>, base.5, CreateVpnPacketBuffer::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnPacketBufferListImpl: Sized + IIterableImpl<VpnPacketBuffer> {
    fn Append(&self, nextvpnpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn AddAtBegin(&self, nextvpnpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn RemoveAtEnd(&self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn RemoveAtBegin(&self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn SetStatus(&self, value: VpnPacketBufferStatus) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<VpnPacketBufferStatus>;
    fn Size(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnPacketBufferList {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBufferList";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnPacketBufferListVtbl {
    pub const fn new<Impl: IVpnPacketBufferListImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnPacketBufferListVtbl {
        unsafe extern "system" fn Append<Impl: IVpnPacketBufferListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Append(&*(&nextvpnpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddAtBegin<Impl: IVpnPacketBufferListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddAtBegin(&*(&nextvpnpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveAtEnd<Impl: IVpnPacketBufferListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAtEnd() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAtBegin<Impl: IVpnPacketBufferListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAtBegin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IVpnPacketBufferListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn SetStatus<Impl: IVpnPacketBufferListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VpnPacketBufferStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetStatus(value).into()
        }
        unsafe extern "system" fn Status<Impl: IVpnPacketBufferListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnPacketBufferStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Size<Impl: IVpnPacketBufferListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Size() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnPacketBufferList>, base.5, Append::<Impl, OFFSET>, AddAtBegin::<Impl, OFFSET>, RemoveAtEnd::<Impl, OFFSET>, RemoveAtBegin::<Impl, OFFSET>, Clear::<Impl, OFFSET>, SetStatus::<Impl, OFFSET>, Status::<Impl, OFFSET>, Size::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IVpnPacketBufferList2Impl: Sized + IIterableImpl<VpnPacketBuffer> {
    fn AddLeadingPacket(&self, nextvpnpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn RemoveLeadingPacket(&self) -> ::windows::core::Result<VpnPacketBuffer>;
    fn AddTrailingPacket(&self, nextvpnpacketbuffer: &::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn RemoveTrailingPacket(&self) -> ::windows::core::Result<VpnPacketBuffer>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IVpnPacketBufferList2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPacketBufferList2";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IVpnPacketBufferList2Vtbl {
    pub const fn new<Impl: IVpnPacketBufferList2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnPacketBufferList2Vtbl {
        unsafe extern "system" fn AddLeadingPacket<Impl: IVpnPacketBufferList2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddLeadingPacket(&*(&nextvpnpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveLeadingPacket<Impl: IVpnPacketBufferList2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveLeadingPacket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrailingPacket<Impl: IVpnPacketBufferList2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nextvpnpacketbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).AddTrailingPacket(&*(&nextvpnpacketbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RemoveTrailingPacket<Impl: IVpnPacketBufferList2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveTrailingPacket() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnPacketBufferList2>, base.5, AddLeadingPacket::<Impl, OFFSET>, RemoveLeadingPacket::<Impl, OFFSET>, AddTrailingPacket::<Impl, OFFSET>, RemoveTrailingPacket::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPickedCredentialImpl: Sized {
    fn PasskeyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn AdditionalPin(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn OldPasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnPickedCredential {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPickedCredential";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnPickedCredentialVtbl {
    pub const fn new<Impl: IVpnPickedCredentialImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnPickedCredentialVtbl {
        unsafe extern "system" fn PasskeyCredential<Impl: IVpnPickedCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PasskeyCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdditionalPin<Impl: IVpnPickedCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdditionalPin() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OldPasswordCredential<Impl: IVpnPickedCredentialImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OldPasswordCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnPickedCredential>, base.5, PasskeyCredential::<Impl, OFFSET>, AdditionalPin::<Impl, OFFSET>, OldPasswordCredential::<Impl, OFFSET>)
    }
}
pub trait IVpnPlugInImpl: Sized {
    fn Connect(&self, channel: &::core::option::Option<VpnChannel>) -> ::windows::core::Result<()>;
    fn Disconnect(&self, channel: &::core::option::Option<VpnChannel>) -> ::windows::core::Result<()>;
    fn GetKeepAlivePayload(&self, channel: &::core::option::Option<VpnChannel>, keepalivepacket: &mut ::core::option::Option<VpnPacketBuffer>) -> ::windows::core::Result<()>;
    fn Encapsulate(&self, channel: &::core::option::Option<VpnChannel>, packets: &::core::option::Option<VpnPacketBufferList>, encapulatedpackets: &::core::option::Option<VpnPacketBufferList>) -> ::windows::core::Result<()>;
    fn Decapsulate(&self, channel: &::core::option::Option<VpnChannel>, encapbuffer: &::core::option::Option<VpnPacketBuffer>, decapsulatedpackets: &::core::option::Option<VpnPacketBufferList>, controlpacketstosend: &::core::option::Option<VpnPacketBufferList>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVpnPlugIn {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPlugIn";
}
impl IVpnPlugInVtbl {
    pub const fn new<Impl: IVpnPlugInImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnPlugInVtbl {
        unsafe extern "system" fn Connect<Impl: IVpnPlugInImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Connect(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disconnect<Impl: IVpnPlugInImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Disconnect(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetKeepAlivePayload<Impl: IVpnPlugInImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, keepalivepacket: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetKeepAlivePayload(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&keepalivepacket)).into()
        }
        unsafe extern "system" fn Encapsulate<Impl: IVpnPlugInImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, packets: ::windows::core::RawPtr, encapulatedpackets: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Encapsulate(&*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType), &*(&packets as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType), &*(&encapulatedpackets as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Decapsulate<Impl: IVpnPlugInImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, channel: ::windows::core::RawPtr, encapbuffer: ::windows::core::RawPtr, decapsulatedpackets: ::windows::core::RawPtr, controlpacketstosend: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this)
                .Decapsulate(
                    &*(&channel as *const <VpnChannel as ::windows::core::Abi>::Abi as *const <VpnChannel as ::windows::core::DefaultType>::DefaultType),
                    &*(&encapbuffer as *const <VpnPacketBuffer as ::windows::core::Abi>::Abi as *const <VpnPacketBuffer as ::windows::core::DefaultType>::DefaultType),
                    &*(&decapsulatedpackets as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType),
                    &*(&controlpacketstosend as *const <VpnPacketBufferList as ::windows::core::Abi>::Abi as *const <VpnPacketBufferList as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnPlugIn>, base.5, Connect::<Impl, OFFSET>, Disconnect::<Impl, OFFSET>, GetKeepAlivePayload::<Impl, OFFSET>, Encapsulate::<Impl, OFFSET>, Decapsulate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPlugInProfileImpl: Sized + IVpnProfileImpl {
    fn ServerUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn CustomConfiguration(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCustomConfiguration(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn VpnPluginPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVpnPluginPackageFamilyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnPlugInProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPlugInProfile";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnPlugInProfileVtbl {
    pub const fn new<Impl: IVpnPlugInProfileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnPlugInProfileVtbl {
        unsafe extern "system" fn ServerUris<Impl: IVpnPlugInProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerUris() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CustomConfiguration<Impl: IVpnPlugInProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CustomConfiguration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomConfiguration<Impl: IVpnPlugInProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetCustomConfiguration(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn VpnPluginPackageFamilyName<Impl: IVpnPlugInProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VpnPluginPackageFamilyName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVpnPluginPackageFamilyName<Impl: IVpnPlugInProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetVpnPluginPackageFamilyName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnPlugInProfile>, base.5, ServerUris::<Impl, OFFSET>, CustomConfiguration::<Impl, OFFSET>, SetCustomConfiguration::<Impl, OFFSET>, VpnPluginPackageFamilyName::<Impl, OFFSET>, SetVpnPluginPackageFamilyName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnPlugInProfile2Impl: Sized + IVpnProfileImpl {
    fn RequireVpnClientAppUI(&self) -> ::windows::core::Result<bool>;
    fn SetRequireVpnClientAppUI(&self, value: bool) -> ::windows::core::Result<()>;
    fn ConnectionStatus(&self) -> ::windows::core::Result<VpnManagementConnectionStatus>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnPlugInProfile2 {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnPlugInProfile2";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnPlugInProfile2Vtbl {
    pub const fn new<Impl: IVpnPlugInProfile2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnPlugInProfile2Vtbl {
        unsafe extern "system" fn RequireVpnClientAppUI<Impl: IVpnPlugInProfile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequireVpnClientAppUI() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequireVpnClientAppUI<Impl: IVpnPlugInProfile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRequireVpnClientAppUI(value).into()
        }
        unsafe extern "system" fn ConnectionStatus<Impl: IVpnPlugInProfile2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnManagementConnectionStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectionStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnPlugInProfile2>, base.5, RequireVpnClientAppUI::<Impl, OFFSET>, SetRequireVpnClientAppUI::<Impl, OFFSET>, ConnectionStatus::<Impl, OFFSET>)
    }
}
pub trait IVpnProfileImpl: Sized {
    fn ProfileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetProfileName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AppTriggers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnAppId>>;
    fn Routes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn DomainNameInfoList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnDomainNameInfo>>;
    fn TrafficFilters(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>;
    fn RememberCredentials(&self) -> ::windows::core::Result<bool>;
    fn SetRememberCredentials(&self, value: bool) -> ::windows::core::Result<()>;
    fn AlwaysOn(&self) -> ::windows::core::Result<bool>;
    fn SetAlwaysOn(&self, value: bool) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IVpnProfile {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnProfile";
}
impl IVpnProfileVtbl {
    pub const fn new<Impl: IVpnProfileImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnProfileVtbl {
        unsafe extern "system" fn ProfileName<Impl: IVpnProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProfileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProfileName<Impl: IVpnProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProfileName(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppTriggers<Impl: IVpnProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppTriggers() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Routes<Impl: IVpnProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Routes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainNameInfoList<Impl: IVpnProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DomainNameInfoList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrafficFilters<Impl: IVpnProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrafficFilters() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RememberCredentials<Impl: IVpnProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RememberCredentials() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRememberCredentials<Impl: IVpnProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRememberCredentials(value).into()
        }
        unsafe extern "system" fn AlwaysOn<Impl: IVpnProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AlwaysOn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlwaysOn<Impl: IVpnProfileImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAlwaysOn(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnProfile>, base.5, ProfileName::<Impl, OFFSET>, SetProfileName::<Impl, OFFSET>, AppTriggers::<Impl, OFFSET>, Routes::<Impl, OFFSET>, DomainNameInfoList::<Impl, OFFSET>, TrafficFilters::<Impl, OFFSET>, RememberCredentials::<Impl, OFFSET>, SetRememberCredentials::<Impl, OFFSET>, AlwaysOn::<Impl, OFFSET>, SetAlwaysOn::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnRouteImpl: Sized {
    fn SetAddress(&self, value: &::core::option::Option<super::HostName>) -> ::windows::core::Result<()>;
    fn Address(&self) -> ::windows::core::Result<super::HostName>;
    fn SetPrefixSize(&self, value: u8) -> ::windows::core::Result<()>;
    fn PrefixSize(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnRoute {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnRoute";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnRouteVtbl {
    pub const fn new<Impl: IVpnRouteImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnRouteVtbl {
        unsafe extern "system" fn SetAddress<Impl: IVpnRouteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAddress(&*(&value as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Address<Impl: IVpnRouteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Address() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrefixSize<Impl: IVpnRouteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPrefixSize(value).into()
        }
        unsafe extern "system" fn PrefixSize<Impl: IVpnRouteImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PrefixSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnRoute>, base.5, SetAddress::<Impl, OFFSET>, Address::<Impl, OFFSET>, SetPrefixSize::<Impl, OFFSET>, PrefixSize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnRouteAssignmentImpl: Sized {
    fn SetIpv4InclusionRoutes(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnRoute>>) -> ::windows::core::Result<()>;
    fn SetIpv6InclusionRoutes(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnRoute>>) -> ::windows::core::Result<()>;
    fn Ipv4InclusionRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn Ipv6InclusionRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn SetIpv4ExclusionRoutes(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnRoute>>) -> ::windows::core::Result<()>;
    fn SetIpv6ExclusionRoutes(&self, value: &::core::option::Option<super::super::Foundation::Collections::IVector<VpnRoute>>) -> ::windows::core::Result<()>;
    fn Ipv4ExclusionRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn Ipv6ExclusionRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnRoute>>;
    fn SetExcludeLocalSubnets(&self, value: bool) -> ::windows::core::Result<()>;
    fn ExcludeLocalSubnets(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnRouteAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnRouteAssignment";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnRouteAssignmentVtbl {
    pub const fn new<Impl: IVpnRouteAssignmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnRouteAssignmentVtbl {
        unsafe extern "system" fn SetIpv4InclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIpv4InclusionRoutes(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetIpv6InclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIpv6InclusionRoutes(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Ipv4InclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ipv4InclusionRoutes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ipv6InclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ipv6InclusionRoutes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIpv4ExclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIpv4ExclusionRoutes(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetIpv6ExclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetIpv6ExclusionRoutes(&*(&value as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IVector<VpnRoute> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Ipv4ExclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ipv4ExclusionRoutes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ipv6ExclusionRoutes<Impl: IVpnRouteAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Ipv6ExclusionRoutes() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExcludeLocalSubnets<Impl: IVpnRouteAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetExcludeLocalSubnets(value).into()
        }
        unsafe extern "system" fn ExcludeLocalSubnets<Impl: IVpnRouteAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExcludeLocalSubnets() {
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
            ::windows::core::GetRuntimeClassName::<IVpnRouteAssignment>,
            base.5,
            SetIpv4InclusionRoutes::<Impl, OFFSET>,
            SetIpv6InclusionRoutes::<Impl, OFFSET>,
            Ipv4InclusionRoutes::<Impl, OFFSET>,
            Ipv6InclusionRoutes::<Impl, OFFSET>,
            SetIpv4ExclusionRoutes::<Impl, OFFSET>,
            SetIpv6ExclusionRoutes::<Impl, OFFSET>,
            Ipv4ExclusionRoutes::<Impl, OFFSET>,
            Ipv6ExclusionRoutes::<Impl, OFFSET>,
            SetExcludeLocalSubnets::<Impl, OFFSET>,
            ExcludeLocalSubnets::<Impl, OFFSET>,
        )
    }
}
pub trait IVpnRouteFactoryImpl: Sized {
    fn CreateVpnRoute(&self, address: &::core::option::Option<super::HostName>, prefixsize: u8) -> ::windows::core::Result<VpnRoute>;
}
impl ::windows::core::RuntimeName for IVpnRouteFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnRouteFactory";
}
impl IVpnRouteFactoryVtbl {
    pub const fn new<Impl: IVpnRouteFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnRouteFactoryVtbl {
        unsafe extern "system" fn CreateVpnRoute<Impl: IVpnRouteFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: ::windows::core::RawPtr, prefixsize: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateVpnRoute(&*(&address as *const <super::HostName as ::windows::core::Abi>::Abi as *const <super::HostName as ::windows::core::DefaultType>::DefaultType), prefixsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnRouteFactory>, base.5, CreateVpnRoute::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnSystemHealthImpl: Sized {
    fn StatementOfHealth(&self) -> ::windows::core::Result<super::super::Storage::Streams::Buffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnSystemHealth {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnSystemHealth";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnSystemHealthVtbl {
    pub const fn new<Impl: IVpnSystemHealthImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnSystemHealthVtbl {
        unsafe extern "system" fn StatementOfHealth<Impl: IVpnSystemHealthImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).StatementOfHealth() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnSystemHealth>, base.5, StatementOfHealth::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnTrafficFilterImpl: Sized {
    fn AppId(&self) -> ::windows::core::Result<VpnAppId>;
    fn SetAppId(&self, value: &::core::option::Option<VpnAppId>) -> ::windows::core::Result<()>;
    fn AppClaims(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Protocol(&self) -> ::windows::core::Result<VpnIPProtocol>;
    fn SetProtocol(&self, value: VpnIPProtocol) -> ::windows::core::Result<()>;
    fn LocalPortRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RemotePortRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn LocalAddressRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RemoteAddressRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn RoutingPolicyType(&self) -> ::windows::core::Result<VpnRoutingPolicyType>;
    fn SetRoutingPolicyType(&self, value: VpnRoutingPolicyType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnTrafficFilter {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnTrafficFilter";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnTrafficFilterVtbl {
    pub const fn new<Impl: IVpnTrafficFilterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnTrafficFilterVtbl {
        unsafe extern "system" fn AppId<Impl: IVpnTrafficFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAppId<Impl: IVpnTrafficFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAppId(&*(&value as *const <VpnAppId as ::windows::core::Abi>::Abi as *const <VpnAppId as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AppClaims<Impl: IVpnTrafficFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppClaims() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Protocol<Impl: IVpnTrafficFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnIPProtocol) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Protocol() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProtocol<Impl: IVpnTrafficFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VpnIPProtocol) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetProtocol(value).into()
        }
        unsafe extern "system" fn LocalPortRanges<Impl: IVpnTrafficFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalPortRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemotePortRanges<Impl: IVpnTrafficFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemotePortRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalAddressRanges<Impl: IVpnTrafficFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).LocalAddressRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteAddressRanges<Impl: IVpnTrafficFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoteAddressRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoutingPolicyType<Impl: IVpnTrafficFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut VpnRoutingPolicyType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RoutingPolicyType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoutingPolicyType<Impl: IVpnTrafficFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: VpnRoutingPolicyType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetRoutingPolicyType(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnTrafficFilter>, base.5, AppId::<Impl, OFFSET>, SetAppId::<Impl, OFFSET>, AppClaims::<Impl, OFFSET>, Protocol::<Impl, OFFSET>, SetProtocol::<Impl, OFFSET>, LocalPortRanges::<Impl, OFFSET>, RemotePortRanges::<Impl, OFFSET>, LocalAddressRanges::<Impl, OFFSET>, RemoteAddressRanges::<Impl, OFFSET>, RoutingPolicyType::<Impl, OFFSET>, SetRoutingPolicyType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnTrafficFilterAssignmentImpl: Sized {
    fn TrafficFilterList(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<VpnTrafficFilter>>;
    fn AllowOutbound(&self) -> ::windows::core::Result<bool>;
    fn SetAllowOutbound(&self, value: bool) -> ::windows::core::Result<()>;
    fn AllowInbound(&self) -> ::windows::core::Result<bool>;
    fn SetAllowInbound(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnTrafficFilterAssignment {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnTrafficFilterAssignment";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnTrafficFilterAssignmentVtbl {
    pub const fn new<Impl: IVpnTrafficFilterAssignmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnTrafficFilterAssignmentVtbl {
        unsafe extern "system" fn TrafficFilterList<Impl: IVpnTrafficFilterAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TrafficFilterList() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowOutbound<Impl: IVpnTrafficFilterAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowOutbound() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowOutbound<Impl: IVpnTrafficFilterAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowOutbound(value).into()
        }
        unsafe extern "system" fn AllowInbound<Impl: IVpnTrafficFilterAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AllowInbound() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInbound<Impl: IVpnTrafficFilterAssignmentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetAllowInbound(value).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnTrafficFilterAssignment>, base.5, TrafficFilterList::<Impl, OFFSET>, AllowOutbound::<Impl, OFFSET>, SetAllowOutbound::<Impl, OFFSET>, AllowInbound::<Impl, OFFSET>, SetAllowInbound::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IVpnTrafficFilterFactoryImpl: Sized {
    fn Create(&self, appid: &::core::option::Option<VpnAppId>) -> ::windows::core::Result<VpnTrafficFilter>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IVpnTrafficFilterFactory {
    const NAME: &'static str = "Windows.Networking.Vpn.IVpnTrafficFilterFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IVpnTrafficFilterFactoryVtbl {
    pub const fn new<Impl: IVpnTrafficFilterFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVpnTrafficFilterFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IVpnTrafficFilterFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&appid as *const <VpnAppId as ::windows::core::Abi>::Abi as *const <VpnAppId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVpnTrafficFilterFactory>, base.5, Create::<Impl, OFFSET>)
    }
}
