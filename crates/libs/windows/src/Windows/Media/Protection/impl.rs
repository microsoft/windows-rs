#[cfg(feature = "implement_exclusive")]
pub trait IComponentLoadFailedEventArgsImpl: Sized {
    fn Information(&self) -> ::windows::core::Result<RevocationAndRenewalInformation>;
    fn Completion(&self) -> ::windows::core::Result<MediaProtectionServiceCompletion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComponentLoadFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.IComponentLoadFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IComponentLoadFailedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentLoadFailedEventArgsImpl, const OFFSET: isize>() -> IComponentLoadFailedEventArgsVtbl {
        unsafe extern "system" fn Information<Impl: IComponentLoadFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Completion<Impl: IComponentLoadFailedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComponentLoadFailedEventArgs>, ::windows::core::GetTrustLevel, Information::<Impl, OFFSET>, Completion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IComponentRenewalStaticsImpl: Sized {
    fn RenewSystemComponentsAsync(&self, information: &::core::option::Option<RevocationAndRenewalInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<RenewalStatus, u32>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComponentRenewalStatics {
    const NAME: &'static str = "Windows.Media.Protection.IComponentRenewalStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IComponentRenewalStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentRenewalStaticsImpl, const OFFSET: isize>() -> IComponentRenewalStaticsVtbl {
        unsafe extern "system" fn RenewSystemComponentsAsync<Impl: IComponentRenewalStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, information: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenewSystemComponentsAsync(&*(&information as *const <RevocationAndRenewalInformation as ::windows::core::Abi>::Abi as *const <RevocationAndRenewalInformation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComponentRenewalStatics>, ::windows::core::GetTrustLevel, RenewSystemComponentsAsync::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHdcpSessionImpl: Sized + IClosableImpl {
    fn IsEffectiveProtectionAtLeast(&self, protection: HdcpProtection) -> ::windows::core::Result<bool>;
    fn GetEffectiveProtection(&self) -> ::windows::core::Result<super::super::Foundation::IReference<HdcpProtection>>;
    fn SetDesiredMinProtectionAsync(&self, protection: HdcpProtection) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HdcpSetProtectionResult>>;
    fn ProtectionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HdcpSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProtectionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHdcpSession {
    const NAME: &'static str = "Windows.Media.Protection.IHdcpSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHdcpSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHdcpSessionImpl, const OFFSET: isize>() -> IHdcpSessionVtbl {
        unsafe extern "system" fn IsEffectiveProtectionAtLeast<Impl: IHdcpSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protection: HdcpProtection, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEffectiveProtectionAtLeast(protection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEffectiveProtection<Impl: IHdcpSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEffectiveProtection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredMinProtectionAsync<Impl: IHdcpSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protection: HdcpProtection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDesiredMinProtectionAsync(protection) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProtectionChanged<Impl: IHdcpSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionChanged(&*(&handler as *const <super::super::Foundation::TypedEventHandler<HdcpSession, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<HdcpSession, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProtectionChanged<Impl: IHdcpSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProtectionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHdcpSession>, ::windows::core::GetTrustLevel, IsEffectiveProtectionAtLeast::<Impl, OFFSET>, GetEffectiveProtection::<Impl, OFFSET>, SetDesiredMinProtectionAsync::<Impl, OFFSET>, ProtectionChanged::<Impl, OFFSET>, RemoveProtectionChanged::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaProtectionManagerImpl: Sized {
    fn ServiceRequested(&self, handler: &::core::option::Option<ServiceRequestedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServiceRequested(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RebootNeeded(&self, handler: &::core::option::Option<RebootNeededEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRebootNeeded(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ComponentLoadFailed(&self, handler: &::core::option::Option<ComponentLoadFailedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveComponentLoadFailed(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaProtectionManager {
    const NAME: &'static str = "Windows.Media.Protection.IMediaProtectionManager";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaProtectionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProtectionManagerImpl, const OFFSET: isize>() -> IMediaProtectionManagerVtbl {
        unsafe extern "system" fn ServiceRequested<Impl: IMediaProtectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceRequested(&*(&handler as *const <ServiceRequestedEventHandler as ::windows::core::Abi>::Abi as *const <ServiceRequestedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveServiceRequested<Impl: IMediaProtectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveServiceRequested(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RebootNeeded<Impl: IMediaProtectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RebootNeeded(&*(&handler as *const <RebootNeededEventHandler as ::windows::core::Abi>::Abi as *const <RebootNeededEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRebootNeeded<Impl: IMediaProtectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRebootNeeded(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ComponentLoadFailed<Impl: IMediaProtectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComponentLoadFailed(&*(&handler as *const <ComponentLoadFailedEventHandler as ::windows::core::Abi>::Abi as *const <ComponentLoadFailedEventHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveComponentLoadFailed<Impl: IMediaProtectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveComponentLoadFailed(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Properties<Impl: IMediaProtectionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IMediaProtectionManager>,
            ::windows::core::GetTrustLevel,
            ServiceRequested::<Impl, OFFSET>,
            RemoveServiceRequested::<Impl, OFFSET>,
            RebootNeeded::<Impl, OFFSET>,
            RemoveRebootNeeded::<Impl, OFFSET>,
            ComponentLoadFailed::<Impl, OFFSET>,
            RemoveComponentLoadFailed::<Impl, OFFSET>,
            Properties::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaProtectionPMPServerImpl: Sized {
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaProtectionPMPServer {
    const NAME: &'static str = "Windows.Media.Protection.IMediaProtectionPMPServer";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaProtectionPMPServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProtectionPMPServerImpl, const OFFSET: isize>() -> IMediaProtectionPMPServerVtbl {
        unsafe extern "system" fn Properties<Impl: IMediaProtectionPMPServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Properties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaProtectionPMPServer>, ::windows::core::GetTrustLevel, Properties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaProtectionPMPServerFactoryImpl: Sized {
    fn CreatePMPServer(&self, pproperties: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<MediaProtectionPMPServer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaProtectionPMPServerFactory {
    const NAME: &'static str = "Windows.Media.Protection.IMediaProtectionPMPServerFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaProtectionPMPServerFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProtectionPMPServerFactoryImpl, const OFFSET: isize>() -> IMediaProtectionPMPServerFactoryVtbl {
        unsafe extern "system" fn CreatePMPServer<Impl: IMediaProtectionPMPServerFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePMPServer(&*(&pproperties as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Collections::IPropertySet as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaProtectionPMPServerFactory>, ::windows::core::GetTrustLevel, CreatePMPServer::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaProtectionServiceCompletionImpl: Sized {
    fn Complete(&self, success: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaProtectionServiceCompletion {
    const NAME: &'static str = "Windows.Media.Protection.IMediaProtectionServiceCompletion";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaProtectionServiceCompletionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProtectionServiceCompletionImpl, const OFFSET: isize>() -> IMediaProtectionServiceCompletionVtbl {
        unsafe extern "system" fn Complete<Impl: IMediaProtectionServiceCompletionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, success: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete(success).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaProtectionServiceCompletion>, ::windows::core::GetTrustLevel, Complete::<Impl, OFFSET>)
    }
}
pub trait IMediaProtectionServiceRequestImpl: Sized {
    fn ProtectionSystem(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Type(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IMediaProtectionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.IMediaProtectionServiceRequest";
}
impl IMediaProtectionServiceRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProtectionServiceRequestImpl, const OFFSET: isize>() -> IMediaProtectionServiceRequestVtbl {
        unsafe extern "system" fn ProtectionSystem<Impl: IMediaProtectionServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProtectionSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IMediaProtectionServiceRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMediaProtectionServiceRequest>, ::windows::core::GetTrustLevel, ProtectionSystem::<Impl, OFFSET>, Type::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionCapabilitiesImpl: Sized {
    fn IsTypeSupported(&self, r#type: &::windows::core::HSTRING, keysystem: &::windows::core::HSTRING) -> ::windows::core::Result<ProtectionCapabilityResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectionCapabilities {
    const NAME: &'static str = "Windows.Media.Protection.IProtectionCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectionCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionCapabilitiesImpl, const OFFSET: isize>() -> IProtectionCapabilitiesVtbl {
        unsafe extern "system" fn IsTypeSupported<Impl: IProtectionCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keysystem: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ProtectionCapabilityResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTypeSupported(&*(&r#type as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&keysystem as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProtectionCapabilities>, ::windows::core::GetTrustLevel, IsTypeSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevocationAndRenewalInformationImpl: Sized {
    fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<RevocationAndRenewalItem>>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevocationAndRenewalInformation {
    const NAME: &'static str = "Windows.Media.Protection.IRevocationAndRenewalInformation";
}
#[cfg(feature = "implement_exclusive")]
impl IRevocationAndRenewalInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRevocationAndRenewalInformationImpl, const OFFSET: isize>() -> IRevocationAndRenewalInformationVtbl {
        unsafe extern "system" fn Items<Impl: IRevocationAndRenewalInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Items() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRevocationAndRenewalInformation>, ::windows::core::GetTrustLevel, Items::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevocationAndRenewalItemImpl: Sized {
    fn Reasons(&self) -> ::windows::core::Result<RevocationAndRenewalReasons>;
    fn HeaderHash(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicKeyHash(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RenewalId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevocationAndRenewalItem {
    const NAME: &'static str = "Windows.Media.Protection.IRevocationAndRenewalItem";
}
#[cfg(feature = "implement_exclusive")]
impl IRevocationAndRenewalItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRevocationAndRenewalItemImpl, const OFFSET: isize>() -> IRevocationAndRenewalItemVtbl {
        unsafe extern "system" fn Reasons<Impl: IRevocationAndRenewalItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RevocationAndRenewalReasons) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reasons() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HeaderHash<Impl: IRevocationAndRenewalItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HeaderHash() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublicKeyHash<Impl: IRevocationAndRenewalItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublicKeyHash() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IRevocationAndRenewalItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RenewalId<Impl: IRevocationAndRenewalItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenewalId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRevocationAndRenewalItem>, ::windows::core::GetTrustLevel, Reasons::<Impl, OFFSET>, HeaderHash::<Impl, OFFSET>, PublicKeyHash::<Impl, OFFSET>, Name::<Impl, OFFSET>, RenewalId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IServiceRequestedEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<IMediaProtectionServiceRequest>;
    fn Completion(&self) -> ::windows::core::Result<MediaProtectionServiceCompletion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IServiceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.IServiceRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IServiceRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceRequestedEventArgsImpl, const OFFSET: isize>() -> IServiceRequestedEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IServiceRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Completion<Impl: IServiceRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completion() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceRequestedEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, Completion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IServiceRequestedEventArgs2Impl: Sized {
    fn MediaPlaybackItem(&self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IServiceRequestedEventArgs2 {
    const NAME: &'static str = "Windows.Media.Protection.IServiceRequestedEventArgs2";
}
#[cfg(feature = "implement_exclusive")]
impl IServiceRequestedEventArgs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceRequestedEventArgs2Impl, const OFFSET: isize>() -> IServiceRequestedEventArgs2Vtbl {
        unsafe extern "system" fn MediaPlaybackItem<Impl: IServiceRequestedEventArgs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaPlaybackItem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceRequestedEventArgs2>, ::windows::core::GetTrustLevel, MediaPlaybackItem::<Impl, OFFSET>)
    }
}
