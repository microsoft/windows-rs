#[cfg(feature = "implement_exclusive")]
pub trait IComponentLoadFailedEventArgs_Impl: Sized {
    fn Information(&mut self) -> ::windows::core::Result<RevocationAndRenewalInformation>;
    fn Completion(&mut self) -> ::windows::core::Result<MediaProtectionServiceCompletion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IComponentLoadFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.IComponentLoadFailedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IComponentLoadFailedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentLoadFailedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponentLoadFailedEventArgs_Vtbl {
        unsafe extern "system" fn Information<Impl: IComponentLoadFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Completion<Impl: IComponentLoadFailedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IComponentLoadFailedEventArgs, BASE_OFFSET>(),
            Information: Information::<Impl, IMPL_OFFSET>,
            Completion: Completion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentLoadFailedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IComponentRenewalStatics_Impl: Sized {
    fn RenewSystemComponentsAsync(&mut self, information: &::core::option::Option<RevocationAndRenewalInformation>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<RenewalStatus, u32>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IComponentRenewalStatics {
    const NAME: &'static str = "Windows.Media.Protection.IComponentRenewalStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IComponentRenewalStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentRenewalStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponentRenewalStatics_Vtbl {
        unsafe extern "system" fn RenewSystemComponentsAsync<Impl: IComponentRenewalStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, information: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IComponentRenewalStatics, BASE_OFFSET>(),
            RenewSystemComponentsAsync: RenewSystemComponentsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentRenewalStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHdcpSession_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn IsEffectiveProtectionAtLeast(&mut self, protection: HdcpProtection) -> ::windows::core::Result<bool>;
    fn GetEffectiveProtection(&mut self) -> ::windows::core::Result<super::super::Foundation::IReference<HdcpProtection>>;
    fn SetDesiredMinProtectionAsync(&mut self, protection: HdcpProtection) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<HdcpSetProtectionResult>>;
    fn ProtectionChanged(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HdcpSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveProtectionChanged(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IHdcpSession {
    const NAME: &'static str = "Windows.Media.Protection.IHdcpSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IHdcpSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHdcpSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHdcpSession_Vtbl {
        unsafe extern "system" fn IsEffectiveProtectionAtLeast<Impl: IHdcpSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protection: HdcpProtection, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetEffectiveProtection<Impl: IHdcpSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDesiredMinProtectionAsync<Impl: IHdcpSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protection: HdcpProtection, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ProtectionChanged<Impl: IHdcpSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveProtectionChanged<Impl: IHdcpSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProtectionChanged(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IHdcpSession, BASE_OFFSET>(),
            IsEffectiveProtectionAtLeast: IsEffectiveProtectionAtLeast::<Impl, IMPL_OFFSET>,
            GetEffectiveProtection: GetEffectiveProtection::<Impl, IMPL_OFFSET>,
            SetDesiredMinProtectionAsync: SetDesiredMinProtectionAsync::<Impl, IMPL_OFFSET>,
            ProtectionChanged: ProtectionChanged::<Impl, IMPL_OFFSET>,
            RemoveProtectionChanged: RemoveProtectionChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHdcpSession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaProtectionManager_Impl: Sized {
    fn ServiceRequested(&mut self, handler: &::core::option::Option<ServiceRequestedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveServiceRequested(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RebootNeeded(&mut self, handler: &::core::option::Option<RebootNeededEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRebootNeeded(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ComponentLoadFailed(&mut self, handler: &::core::option::Option<ComponentLoadFailedEventHandler>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveComponentLoadFailed(&mut self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaProtectionManager {
    const NAME: &'static str = "Windows.Media.Protection.IMediaProtectionManager";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaProtectionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProtectionManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaProtectionManager_Vtbl {
        unsafe extern "system" fn ServiceRequested<Impl: IMediaProtectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveServiceRequested<Impl: IMediaProtectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveServiceRequested(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RebootNeeded<Impl: IMediaProtectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveRebootNeeded<Impl: IMediaProtectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRebootNeeded(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ComponentLoadFailed<Impl: IMediaProtectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveComponentLoadFailed<Impl: IMediaProtectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveComponentLoadFailed(&*(&cookie as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Properties<Impl: IMediaProtectionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaProtectionManager, BASE_OFFSET>(),
            ServiceRequested: ServiceRequested::<Impl, IMPL_OFFSET>,
            RemoveServiceRequested: RemoveServiceRequested::<Impl, IMPL_OFFSET>,
            RebootNeeded: RebootNeeded::<Impl, IMPL_OFFSET>,
            RemoveRebootNeeded: RemoveRebootNeeded::<Impl, IMPL_OFFSET>,
            ComponentLoadFailed: ComponentLoadFailed::<Impl, IMPL_OFFSET>,
            RemoveComponentLoadFailed: RemoveComponentLoadFailed::<Impl, IMPL_OFFSET>,
            Properties: Properties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaProtectionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaProtectionPMPServer_Impl: Sized {
    fn Properties(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaProtectionPMPServer {
    const NAME: &'static str = "Windows.Media.Protection.IMediaProtectionPMPServer";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaProtectionPMPServer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProtectionPMPServer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaProtectionPMPServer_Vtbl {
        unsafe extern "system" fn Properties<Impl: IMediaProtectionPMPServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaProtectionPMPServer, BASE_OFFSET>(), Properties: Properties::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaProtectionPMPServer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IMediaProtectionPMPServerFactory_Impl: Sized {
    fn CreatePMPServer(&mut self, pproperties: &::core::option::Option<super::super::Foundation::Collections::IPropertySet>) -> ::windows::core::Result<MediaProtectionPMPServer>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IMediaProtectionPMPServerFactory {
    const NAME: &'static str = "Windows.Media.Protection.IMediaProtectionPMPServerFactory";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IMediaProtectionPMPServerFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProtectionPMPServerFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaProtectionPMPServerFactory_Vtbl {
        unsafe extern "system" fn CreatePMPServer<Impl: IMediaProtectionPMPServerFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaProtectionPMPServerFactory, BASE_OFFSET>(),
            CreatePMPServer: CreatePMPServer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaProtectionPMPServerFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaProtectionServiceCompletion_Impl: Sized {
    fn Complete(&mut self, success: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMediaProtectionServiceCompletion {
    const NAME: &'static str = "Windows.Media.Protection.IMediaProtectionServiceCompletion";
}
#[cfg(feature = "implement_exclusive")]
impl IMediaProtectionServiceCompletion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProtectionServiceCompletion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaProtectionServiceCompletion_Vtbl {
        unsafe extern "system" fn Complete<Impl: IMediaProtectionServiceCompletion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, success: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete(success).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaProtectionServiceCompletion, BASE_OFFSET>(),
            Complete: Complete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaProtectionServiceCompletion as ::windows::core::Interface>::IID
    }
}
pub trait IMediaProtectionServiceRequest_Impl: Sized {
    fn ProtectionSystem(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Type(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IMediaProtectionServiceRequest {
    const NAME: &'static str = "Windows.Media.Protection.IMediaProtectionServiceRequest";
}
impl IMediaProtectionServiceRequest_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMediaProtectionServiceRequest_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMediaProtectionServiceRequest_Vtbl {
        unsafe extern "system" fn ProtectionSystem<Impl: IMediaProtectionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Type<Impl: IMediaProtectionServiceRequest_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMediaProtectionServiceRequest, BASE_OFFSET>(),
            ProtectionSystem: ProtectionSystem::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMediaProtectionServiceRequest as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IProtectionCapabilities_Impl: Sized {
    fn IsTypeSupported(&mut self, r#type: &::windows::core::HSTRING, keysystem: &::windows::core::HSTRING) -> ::windows::core::Result<ProtectionCapabilityResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IProtectionCapabilities {
    const NAME: &'static str = "Windows.Media.Protection.IProtectionCapabilities";
}
#[cfg(feature = "implement_exclusive")]
impl IProtectionCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProtectionCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProtectionCapabilities_Vtbl {
        unsafe extern "system" fn IsTypeSupported<Impl: IProtectionCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, keysystem: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ProtectionCapabilityResult) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IProtectionCapabilities, BASE_OFFSET>(),
            IsTypeSupported: IsTypeSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProtectionCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IRevocationAndRenewalInformation_Impl: Sized {
    fn Items(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<RevocationAndRenewalItem>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRevocationAndRenewalInformation {
    const NAME: &'static str = "Windows.Media.Protection.IRevocationAndRenewalInformation";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IRevocationAndRenewalInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRevocationAndRenewalInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRevocationAndRenewalInformation_Vtbl {
        unsafe extern "system" fn Items<Impl: IRevocationAndRenewalInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IRevocationAndRenewalInformation, BASE_OFFSET>(), Items: Items::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRevocationAndRenewalInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRevocationAndRenewalItem_Impl: Sized {
    fn Reasons(&mut self) -> ::windows::core::Result<RevocationAndRenewalReasons>;
    fn HeaderHash(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PublicKeyHash(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RenewalId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRevocationAndRenewalItem {
    const NAME: &'static str = "Windows.Media.Protection.IRevocationAndRenewalItem";
}
#[cfg(feature = "implement_exclusive")]
impl IRevocationAndRenewalItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRevocationAndRenewalItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRevocationAndRenewalItem_Vtbl {
        unsafe extern "system" fn Reasons<Impl: IRevocationAndRenewalItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut RevocationAndRenewalReasons) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HeaderHash<Impl: IRevocationAndRenewalItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn PublicKeyHash<Impl: IRevocationAndRenewalItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Name<Impl: IRevocationAndRenewalItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RenewalId<Impl: IRevocationAndRenewalItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRevocationAndRenewalItem, BASE_OFFSET>(),
            Reasons: Reasons::<Impl, IMPL_OFFSET>,
            HeaderHash: HeaderHash::<Impl, IMPL_OFFSET>,
            PublicKeyHash: PublicKeyHash::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            RenewalId: RenewalId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRevocationAndRenewalItem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IServiceRequestedEventArgs_Impl: Sized {
    fn Request(&mut self) -> ::windows::core::Result<IMediaProtectionServiceRequest>;
    fn Completion(&mut self) -> ::windows::core::Result<MediaProtectionServiceCompletion>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IServiceRequestedEventArgs {
    const NAME: &'static str = "Windows.Media.Protection.IServiceRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IServiceRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceRequestedEventArgs_Vtbl {
        unsafe extern "system" fn Request<Impl: IServiceRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Completion<Impl: IServiceRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IServiceRequestedEventArgs, BASE_OFFSET>(),
            Request: Request::<Impl, IMPL_OFFSET>,
            Completion: Completion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Media_Playback", feature = "implement_exclusive"))]
pub trait IServiceRequestedEventArgs2_Impl: Sized {
    fn MediaPlaybackItem(&mut self) -> ::windows::core::Result<super::Playback::MediaPlaybackItem>;
}
#[cfg(all(feature = "Media_Playback", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IServiceRequestedEventArgs2 {
    const NAME: &'static str = "Windows.Media.Protection.IServiceRequestedEventArgs2";
}
#[cfg(all(feature = "Media_Playback", feature = "implement_exclusive"))]
impl IServiceRequestedEventArgs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceRequestedEventArgs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceRequestedEventArgs2_Vtbl {
        unsafe extern "system" fn MediaPlaybackItem<Impl: IServiceRequestedEventArgs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IServiceRequestedEventArgs2, BASE_OFFSET>(),
            MediaPlaybackItem: MediaPlaybackItem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceRequestedEventArgs2 as ::windows::core::Interface>::IID
    }
}
