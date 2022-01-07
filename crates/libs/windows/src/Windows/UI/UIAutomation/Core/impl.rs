#[cfg(feature = "implement_exclusive")]
pub trait IAutomationRemoteOperationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AutomationRemoteOperationStatus>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn ErrorLocation(&self) -> ::windows::core::Result<i32>;
    fn HasOperand(&self, operandid: &AutomationRemoteOperationOperandId) -> ::windows::core::Result<bool>;
    fn GetOperand(&self, operandid: &AutomationRemoteOperationOperandId) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationRemoteOperationResult {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IAutomationRemoteOperationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationRemoteOperationResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationRemoteOperationResultImpl, const OFFSET: isize>() -> IAutomationRemoteOperationResultVtbl {
        unsafe extern "system" fn Status<Impl: IAutomationRemoteOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationRemoteOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: IAutomationRemoteOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedError() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorLocation<Impl: IAutomationRemoteOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorLocation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasOperand<Impl: IAutomationRemoteOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasOperand(&*(&operandid as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOperand<Impl: IAutomationRemoteOperationResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOperand(&*(&operandid as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationRemoteOperationResult>, ::windows::core::GetTrustLevel, Status::<Impl, OFFSET>, ExtendedError::<Impl, OFFSET>, ErrorLocation::<Impl, OFFSET>, HasOperand::<Impl, OFFSET>, GetOperand::<Impl, OFFSET>)
    }
}
pub trait ICoreAutomationConnectionBoundObjectProviderImpl: Sized {
    fn IsComThreadingRequired(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ICoreAutomationConnectionBoundObjectProvider {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationConnectionBoundObjectProvider";
}
impl ICoreAutomationConnectionBoundObjectProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationConnectionBoundObjectProviderImpl, const OFFSET: isize>() -> ICoreAutomationConnectionBoundObjectProviderVtbl {
        unsafe extern "system" fn IsComThreadingRequired<Impl: ICoreAutomationConnectionBoundObjectProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsComThreadingRequired() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreAutomationConnectionBoundObjectProvider>, ::windows::core::GetTrustLevel, IsComThreadingRequired::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAutomationRegistrarStaticsImpl: Sized {
    fn RegisterAnnotationType(&self, guid: &::windows::core::GUID) -> ::windows::core::Result<AutomationAnnotationTypeRegistration>;
    fn UnregisterAnnotationType(&self, registration: &AutomationAnnotationTypeRegistration) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreAutomationRegistrarStatics {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRegistrarStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreAutomationRegistrarStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationRegistrarStaticsImpl, const OFFSET: isize>() -> ICoreAutomationRegistrarStaticsVtbl {
        unsafe extern "system" fn RegisterAnnotationType<Impl: ICoreAutomationRegistrarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID, result__: *mut AutomationAnnotationTypeRegistration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterAnnotationType(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterAnnotationType<Impl: ICoreAutomationRegistrarStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registration: AutomationAnnotationTypeRegistration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterAnnotationType(&*(&registration as *const <AutomationAnnotationTypeRegistration as ::windows::core::Abi>::Abi as *const <AutomationAnnotationTypeRegistration as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreAutomationRegistrarStatics>, ::windows::core::GetTrustLevel, RegisterAnnotationType::<Impl, OFFSET>, UnregisterAnnotationType::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAutomationRemoteOperationImpl: Sized {
    fn IsOpcodeSupported(&self, opcode: u32) -> ::windows::core::Result<bool>;
    fn ImportElement(&self, operandid: &AutomationRemoteOperationOperandId, element: &::core::option::Option<super::AutomationElement>) -> ::windows::core::Result<()>;
    fn ImportTextRange(&self, operandid: &AutomationRemoteOperationOperandId, textrange: &::core::option::Option<super::AutomationTextRange>) -> ::windows::core::Result<()>;
    fn AddToResults(&self, operandid: &AutomationRemoteOperationOperandId) -> ::windows::core::Result<()>;
    fn Execute(&self, bytecodebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<AutomationRemoteOperationResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreAutomationRemoteOperation {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreAutomationRemoteOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationRemoteOperationImpl, const OFFSET: isize>() -> ICoreAutomationRemoteOperationVtbl {
        unsafe extern "system" fn IsOpcodeSupported<Impl: ICoreAutomationRemoteOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opcode: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpcodeSupported(opcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportElement<Impl: ICoreAutomationRemoteOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImportElement(&*(&operandid as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType), &*(&element as *const <super::AutomationElement as ::windows::core::Abi>::Abi as *const <super::AutomationElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImportTextRange<Impl: ICoreAutomationRemoteOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, textrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImportTextRange(&*(&operandid as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType), &*(&textrange as *const <super::AutomationTextRange as ::windows::core::Abi>::Abi as *const <super::AutomationTextRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddToResults<Impl: ICoreAutomationRemoteOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToResults(&*(&operandid as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Execute<Impl: ICoreAutomationRemoteOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytecodeBuffer_array_size: u32, bytecodebuffer: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Execute(::core::slice::from_raw_parts(::core::mem::transmute_copy(&bytecodebuffer), bytecodeBuffer_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreAutomationRemoteOperation>, ::windows::core::GetTrustLevel, IsOpcodeSupported::<Impl, OFFSET>, ImportElement::<Impl, OFFSET>, ImportTextRange::<Impl, OFFSET>, AddToResults::<Impl, OFFSET>, Execute::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAutomationRemoteOperation2Impl: Sized {
    fn ImportConnectionBoundObject(&self, operandid: &AutomationRemoteOperationOperandId, connectionboundobject: &::core::option::Option<super::AutomationConnectionBoundObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreAutomationRemoteOperation2 {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation2";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreAutomationRemoteOperation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationRemoteOperation2Impl, const OFFSET: isize>() -> ICoreAutomationRemoteOperation2Vtbl {
        unsafe extern "system" fn ImportConnectionBoundObject<Impl: ICoreAutomationRemoteOperation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, connectionboundobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImportConnectionBoundObject(&*(&operandid as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType), &*(&connectionboundobject as *const <super::AutomationConnectionBoundObject as ::windows::core::Abi>::Abi as *const <super::AutomationConnectionBoundObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreAutomationRemoteOperation2>, ::windows::core::GetTrustLevel, ImportConnectionBoundObject::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAutomationRemoteOperationContextImpl: Sized {
    fn GetOperand(&self, id: &AutomationRemoteOperationOperandId) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetOperand(&self, id: &AutomationRemoteOperationOperandId, operand: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SetOperand2(&self, id: &AutomationRemoteOperationOperandId, operand: &::core::option::Option<::windows::core::IInspectable>, operandinterfaceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreAutomationRemoteOperationContext {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationContext";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreAutomationRemoteOperationContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationRemoteOperationContextImpl, const OFFSET: isize>() -> ICoreAutomationRemoteOperationContextVtbl {
        unsafe extern "system" fn GetOperand<Impl: ICoreAutomationRemoteOperationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOperand(&*(&id as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperand<Impl: ICoreAutomationRemoteOperationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOperand(&*(&id as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType), &*(&operand as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetOperand2<Impl: ICoreAutomationRemoteOperationContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void, operandinterfaceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetOperand2(
                    &*(&id as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType),
                    &*(&operand as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&operandinterfaceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreAutomationRemoteOperationContext>, ::windows::core::GetTrustLevel, GetOperand::<Impl, OFFSET>, SetOperand::<Impl, OFFSET>, SetOperand2::<Impl, OFFSET>)
    }
}
pub trait ICoreAutomationRemoteOperationExtensionProviderImpl: Sized {
    fn CallExtension(&self, extensionid: &::windows::core::GUID, context: &::core::option::Option<CoreAutomationRemoteOperationContext>, operandids: &[<AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn IsExtensionSupported(&self, extensionid: &::windows::core::GUID) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ICoreAutomationRemoteOperationExtensionProvider {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationExtensionProvider";
}
impl ICoreAutomationRemoteOperationExtensionProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationRemoteOperationExtensionProviderImpl, const OFFSET: isize>() -> ICoreAutomationRemoteOperationExtensionProviderVtbl {
        unsafe extern "system" fn CallExtension<Impl: ICoreAutomationRemoteOperationExtensionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensionid: ::windows::core::GUID, context: ::windows::core::RawPtr, operandIds_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CallExtension(&*(&extensionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <CoreAutomationRemoteOperationContext as ::windows::core::Abi>::Abi as *const <CoreAutomationRemoteOperationContext as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&operandids), operandIds_array_size as _)).into()
        }
        unsafe extern "system" fn IsExtensionSupported<Impl: ICoreAutomationRemoteOperationExtensionProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensionid: ::windows::core::GUID, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsExtensionSupported(&*(&extensionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICoreAutomationRemoteOperationExtensionProvider>, ::windows::core::GetTrustLevel, CallExtension::<Impl, OFFSET>, IsExtensionSupported::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationClientSessionImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn CreateWindowAsync(&self, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RemoteAutomationWindow>>;
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ConnectionRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationConnectionRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Disconnected(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationDisconnectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisconnected(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteAutomationClientSession {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IRemoteAutomationClientSession";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteAutomationClientSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteAutomationClientSessionImpl, const OFFSET: isize>() -> IRemoteAutomationClientSessionVtbl {
        unsafe extern "system" fn Start<Impl: IRemoteAutomationClientSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IRemoteAutomationClientSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn CreateWindowAsync<Impl: IRemoteAutomationClientSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWindowAsync(remotewindowid, remoteprocessid, &*(&parentautomationelement as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionId<Impl: IRemoteAutomationClientSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectionRequested<Impl: IRemoteAutomationClientSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectionRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationConnectionRequestedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationConnectionRequestedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConnectionRequested<Impl: IRemoteAutomationClientSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConnectionRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disconnected<Impl: IRemoteAutomationClientSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnected(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationDisconnectedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationDisconnectedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDisconnected<Impl: IRemoteAutomationClientSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDisconnected(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRemoteAutomationClientSession>,
            ::windows::core::GetTrustLevel,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            CreateWindowAsync::<Impl, OFFSET>,
            SessionId::<Impl, OFFSET>,
            ConnectionRequested::<Impl, OFFSET>,
            RemoveConnectionRequested::<Impl, OFFSET>,
            Disconnected::<Impl, OFFSET>,
            RemoveDisconnected::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationClientSessionFactoryImpl: Sized {
    fn CreateInstance(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteAutomationClientSession>;
    fn CreateInstance2(&self, name: &::windows::core::HSTRING, sessionid: &::windows::core::GUID) -> ::windows::core::Result<RemoteAutomationClientSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteAutomationClientSessionFactory {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IRemoteAutomationClientSessionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteAutomationClientSessionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteAutomationClientSessionFactoryImpl, const OFFSET: isize>() -> IRemoteAutomationClientSessionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRemoteAutomationClientSessionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance2<Impl: IRemoteAutomationClientSessionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance2(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&sessionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteAutomationClientSessionFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>, CreateInstance2::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationConnectionRequestedEventArgsImpl: Sized {
    fn LocalPipeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteProcessId(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteAutomationConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IRemoteAutomationConnectionRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteAutomationConnectionRequestedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteAutomationConnectionRequestedEventArgsImpl, const OFFSET: isize>() -> IRemoteAutomationConnectionRequestedEventArgsVtbl {
        unsafe extern "system" fn LocalPipeName<Impl: IRemoteAutomationConnectionRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPipeName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoteProcessId<Impl: IRemoteAutomationConnectionRequestedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoteProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteAutomationConnectionRequestedEventArgs>, ::windows::core::GetTrustLevel, LocalPipeName::<Impl, OFFSET>, RemoteProcessId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationDisconnectedEventArgsImpl: Sized {
    fn LocalPipeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteAutomationDisconnectedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IRemoteAutomationDisconnectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteAutomationDisconnectedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteAutomationDisconnectedEventArgsImpl, const OFFSET: isize>() -> IRemoteAutomationDisconnectedEventArgsVtbl {
        unsafe extern "system" fn LocalPipeName<Impl: IRemoteAutomationDisconnectedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LocalPipeName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteAutomationDisconnectedEventArgs>, ::windows::core::GetTrustLevel, LocalPipeName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationServerStaticsImpl: Sized {
    fn ReportSession(&self, sessionid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteAutomationServerStatics {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IRemoteAutomationServerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteAutomationServerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteAutomationServerStaticsImpl, const OFFSET: isize>() -> IRemoteAutomationServerStaticsVtbl {
        unsafe extern "system" fn ReportSession<Impl: IRemoteAutomationServerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportSession(&*(&sessionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteAutomationServerStatics>, ::windows::core::GetTrustLevel, ReportSession::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationWindowImpl: Sized {
    fn AutomationProvider(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn UnregisterAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteAutomationWindow {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IRemoteAutomationWindow";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteAutomationWindowVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteAutomationWindowImpl, const OFFSET: isize>() -> IRemoteAutomationWindowVtbl {
        unsafe extern "system" fn AutomationProvider<Impl: IRemoteAutomationWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutomationProvider() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterAsync<Impl: IRemoteAutomationWindowImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IRemoteAutomationWindow>, ::windows::core::GetTrustLevel, AutomationProvider::<Impl, OFFSET>, UnregisterAsync::<Impl, OFFSET>)
    }
}
