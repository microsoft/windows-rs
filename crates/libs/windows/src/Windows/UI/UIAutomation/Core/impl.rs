#[cfg(feature = "implement_exclusive")]
pub trait IAutomationRemoteOperationResult_Impl: Sized {
    fn Status(&mut self) -> ::windows::core::Result<AutomationRemoteOperationStatus>;
    fn ExtendedError(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn ErrorLocation(&mut self) -> ::windows::core::Result<i32>;
    fn HasOperand(&mut self, operandid: &AutomationRemoteOperationOperandId) -> ::windows::core::Result<bool>;
    fn GetOperand(&mut self, operandid: &AutomationRemoteOperationOperandId) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationRemoteOperationResult {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IAutomationRemoteOperationResult";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationRemoteOperationResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationRemoteOperationResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationRemoteOperationResult_Vtbl {
        unsafe extern "system" fn Status<Impl: IAutomationRemoteOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AutomationRemoteOperationStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExtendedError<Impl: IAutomationRemoteOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ErrorLocation<Impl: IAutomationRemoteOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn HasOperand<Impl: IAutomationRemoteOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetOperand<Impl: IAutomationRemoteOperationResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationRemoteOperationResult, BASE_OFFSET>(),
            Status: Status::<Impl, IMPL_OFFSET>,
            ExtendedError: ExtendedError::<Impl, IMPL_OFFSET>,
            ErrorLocation: ErrorLocation::<Impl, IMPL_OFFSET>,
            HasOperand: HasOperand::<Impl, IMPL_OFFSET>,
            GetOperand: GetOperand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationRemoteOperationResult as ::windows::core::Interface>::IID
    }
}
pub trait ICoreAutomationConnectionBoundObjectProvider_Impl: Sized {
    fn IsComThreadingRequired(&mut self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ICoreAutomationConnectionBoundObjectProvider {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationConnectionBoundObjectProvider";
}
impl ICoreAutomationConnectionBoundObjectProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationConnectionBoundObjectProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreAutomationConnectionBoundObjectProvider_Vtbl {
        unsafe extern "system" fn IsComThreadingRequired<Impl: ICoreAutomationConnectionBoundObjectProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAutomationConnectionBoundObjectProvider, BASE_OFFSET>(),
            IsComThreadingRequired: IsComThreadingRequired::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAutomationConnectionBoundObjectProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAutomationRegistrarStatics_Impl: Sized {
    fn RegisterAnnotationType(&mut self, guid: &::windows::core::GUID) -> ::windows::core::Result<AutomationAnnotationTypeRegistration>;
    fn UnregisterAnnotationType(&mut self, registration: &AutomationAnnotationTypeRegistration) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreAutomationRegistrarStatics {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRegistrarStatics";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreAutomationRegistrarStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationRegistrarStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreAutomationRegistrarStatics_Vtbl {
        unsafe extern "system" fn RegisterAnnotationType<Impl: ICoreAutomationRegistrarStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID, result__: *mut AutomationAnnotationTypeRegistration) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UnregisterAnnotationType<Impl: ICoreAutomationRegistrarStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registration: AutomationAnnotationTypeRegistration) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterAnnotationType(&*(&registration as *const <AutomationAnnotationTypeRegistration as ::windows::core::Abi>::Abi as *const <AutomationAnnotationTypeRegistration as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAutomationRegistrarStatics, BASE_OFFSET>(),
            RegisterAnnotationType: RegisterAnnotationType::<Impl, IMPL_OFFSET>,
            UnregisterAnnotationType: UnregisterAnnotationType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAutomationRegistrarStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAutomationRemoteOperation_Impl: Sized {
    fn IsOpcodeSupported(&mut self, opcode: u32) -> ::windows::core::Result<bool>;
    fn ImportElement(&mut self, operandid: &AutomationRemoteOperationOperandId, element: &::core::option::Option<super::AutomationElement>) -> ::windows::core::Result<()>;
    fn ImportTextRange(&mut self, operandid: &AutomationRemoteOperationOperandId, textrange: &::core::option::Option<super::AutomationTextRange>) -> ::windows::core::Result<()>;
    fn AddToResults(&mut self, operandid: &AutomationRemoteOperationOperandId) -> ::windows::core::Result<()>;
    fn Execute(&mut self, bytecodebuffer: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<AutomationRemoteOperationResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreAutomationRemoteOperation {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreAutomationRemoteOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationRemoteOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreAutomationRemoteOperation_Vtbl {
        unsafe extern "system" fn IsOpcodeSupported<Impl: ICoreAutomationRemoteOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, opcode: u32, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ImportElement<Impl: ICoreAutomationRemoteOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImportElement(&*(&operandid as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType), &*(&element as *const <super::AutomationElement as ::windows::core::Abi>::Abi as *const <super::AutomationElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ImportTextRange<Impl: ICoreAutomationRemoteOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, textrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImportTextRange(&*(&operandid as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType), &*(&textrange as *const <super::AutomationTextRange as ::windows::core::Abi>::Abi as *const <super::AutomationTextRange as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddToResults<Impl: ICoreAutomationRemoteOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToResults(&*(&operandid as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Execute<Impl: ICoreAutomationRemoteOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytecodeBuffer_array_size: u32, bytecodebuffer: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAutomationRemoteOperation, BASE_OFFSET>(),
            IsOpcodeSupported: IsOpcodeSupported::<Impl, IMPL_OFFSET>,
            ImportElement: ImportElement::<Impl, IMPL_OFFSET>,
            ImportTextRange: ImportTextRange::<Impl, IMPL_OFFSET>,
            AddToResults: AddToResults::<Impl, IMPL_OFFSET>,
            Execute: Execute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAutomationRemoteOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAutomationRemoteOperation2_Impl: Sized {
    fn ImportConnectionBoundObject(&mut self, operandid: &AutomationRemoteOperationOperandId, connectionboundobject: &::core::option::Option<super::AutomationConnectionBoundObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreAutomationRemoteOperation2 {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperation2";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreAutomationRemoteOperation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationRemoteOperation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreAutomationRemoteOperation2_Vtbl {
        unsafe extern "system" fn ImportConnectionBoundObject<Impl: ICoreAutomationRemoteOperation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operandid: AutomationRemoteOperationOperandId, connectionboundobject: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImportConnectionBoundObject(&*(&operandid as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType), &*(&connectionboundobject as *const <super::AutomationConnectionBoundObject as ::windows::core::Abi>::Abi as *const <super::AutomationConnectionBoundObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAutomationRemoteOperation2, BASE_OFFSET>(),
            ImportConnectionBoundObject: ImportConnectionBoundObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAutomationRemoteOperation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait ICoreAutomationRemoteOperationContext_Impl: Sized {
    fn GetOperand(&mut self, id: &AutomationRemoteOperationOperandId) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetOperand(&mut self, id: &AutomationRemoteOperationOperandId, operand: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn SetOperand2(&mut self, id: &AutomationRemoteOperationOperandId, operand: &::core::option::Option<::windows::core::IInspectable>, operandinterfaceid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for ICoreAutomationRemoteOperationContext {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationContext";
}
#[cfg(feature = "implement_exclusive")]
impl ICoreAutomationRemoteOperationContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationRemoteOperationContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreAutomationRemoteOperationContext_Vtbl {
        unsafe extern "system" fn GetOperand<Impl: ICoreAutomationRemoteOperationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetOperand<Impl: ICoreAutomationRemoteOperationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOperand(&*(&id as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType), &*(&operand as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetOperand2<Impl: ICoreAutomationRemoteOperationContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: AutomationRemoteOperationOperandId, operand: *mut ::core::ffi::c_void, operandinterfaceid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetOperand2(
                    &*(&id as *const <AutomationRemoteOperationOperandId as ::windows::core::Abi>::Abi as *const <AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType),
                    &*(&operand as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&operandinterfaceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAutomationRemoteOperationContext, BASE_OFFSET>(),
            GetOperand: GetOperand::<Impl, IMPL_OFFSET>,
            SetOperand: SetOperand::<Impl, IMPL_OFFSET>,
            SetOperand2: SetOperand2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAutomationRemoteOperationContext as ::windows::core::Interface>::IID
    }
}
pub trait ICoreAutomationRemoteOperationExtensionProvider_Impl: Sized {
    fn CallExtension(&mut self, extensionid: &::windows::core::GUID, context: &::core::option::Option<CoreAutomationRemoteOperationContext>, operandids: &[<AutomationRemoteOperationOperandId as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn IsExtensionSupported(&mut self, extensionid: &::windows::core::GUID) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ICoreAutomationRemoteOperationExtensionProvider {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationExtensionProvider";
}
impl ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
        unsafe extern "system" fn CallExtension<Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensionid: ::windows::core::GUID, context: ::windows::core::RawPtr, operandIds_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CallExtension(&*(&extensionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <CoreAutomationRemoteOperationContext as ::windows::core::Abi>::Abi as *const <CoreAutomationRemoteOperationContext as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&operandids), operandIds_array_size as _)).into()
        }
        unsafe extern "system" fn IsExtensionSupported<Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensionid: ::windows::core::GUID, result__: *mut bool) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAutomationRemoteOperationExtensionProvider, BASE_OFFSET>(),
            CallExtension: CallExtension::<Impl, IMPL_OFFSET>,
            IsExtensionSupported: IsExtensionSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAutomationRemoteOperationExtensionProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRemoteAutomationClientSession_Impl: Sized {
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn CreateWindowAsync(&mut self, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<RemoteAutomationWindow>>;
    fn SessionId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ConnectionRequested(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationConnectionRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveConnectionRequested(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Disconnected(&mut self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<RemoteAutomationClientSession, RemoteAutomationDisconnectedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisconnected(&mut self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteAutomationClientSession {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IRemoteAutomationClientSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRemoteAutomationClientSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteAutomationClientSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteAutomationClientSession_Vtbl {
        unsafe extern "system" fn Start<Impl: IRemoteAutomationClientSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IRemoteAutomationClientSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn CreateWindowAsync<Impl: IRemoteAutomationClientSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, remotewindowid: u64, remoteprocessid: u32, parentautomationelement: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SessionId<Impl: IRemoteAutomationClientSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ConnectionRequested<Impl: IRemoteAutomationClientSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveConnectionRequested<Impl: IRemoteAutomationClientSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConnectionRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Disconnected<Impl: IRemoteAutomationClientSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveDisconnected<Impl: IRemoteAutomationClientSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDisconnected(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteAutomationClientSession, BASE_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            CreateWindowAsync: CreateWindowAsync::<Impl, IMPL_OFFSET>,
            SessionId: SessionId::<Impl, IMPL_OFFSET>,
            ConnectionRequested: ConnectionRequested::<Impl, IMPL_OFFSET>,
            RemoveConnectionRequested: RemoveConnectionRequested::<Impl, IMPL_OFFSET>,
            Disconnected: Disconnected::<Impl, IMPL_OFFSET>,
            RemoveDisconnected: RemoveDisconnected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteAutomationClientSession as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationClientSessionFactory_Impl: Sized {
    fn CreateInstance(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteAutomationClientSession>;
    fn CreateInstance2(&mut self, name: &::windows::core::HSTRING, sessionid: &::windows::core::GUID) -> ::windows::core::Result<RemoteAutomationClientSession>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteAutomationClientSessionFactory {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IRemoteAutomationClientSessionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteAutomationClientSessionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteAutomationClientSessionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteAutomationClientSessionFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IRemoteAutomationClientSessionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateInstance2<Impl: IRemoteAutomationClientSessionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, sessionid: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteAutomationClientSessionFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            CreateInstance2: CreateInstance2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteAutomationClientSessionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationConnectionRequestedEventArgs_Impl: Sized {
    fn LocalPipeName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteProcessId(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteAutomationConnectionRequestedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IRemoteAutomationConnectionRequestedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteAutomationConnectionRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteAutomationConnectionRequestedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteAutomationConnectionRequestedEventArgs_Vtbl {
        unsafe extern "system" fn LocalPipeName<Impl: IRemoteAutomationConnectionRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoteProcessId<Impl: IRemoteAutomationConnectionRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteAutomationConnectionRequestedEventArgs, BASE_OFFSET>(),
            LocalPipeName: LocalPipeName::<Impl, IMPL_OFFSET>,
            RemoteProcessId: RemoteProcessId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteAutomationConnectionRequestedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationDisconnectedEventArgs_Impl: Sized {
    fn LocalPipeName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteAutomationDisconnectedEventArgs {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IRemoteAutomationDisconnectedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteAutomationDisconnectedEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteAutomationDisconnectedEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteAutomationDisconnectedEventArgs_Vtbl {
        unsafe extern "system" fn LocalPipeName<Impl: IRemoteAutomationDisconnectedEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteAutomationDisconnectedEventArgs, BASE_OFFSET>(),
            LocalPipeName: LocalPipeName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteAutomationDisconnectedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteAutomationServerStatics_Impl: Sized {
    fn ReportSession(&mut self, sessionid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IRemoteAutomationServerStatics {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IRemoteAutomationServerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IRemoteAutomationServerStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteAutomationServerStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteAutomationServerStatics_Vtbl {
        unsafe extern "system" fn ReportSession<Impl: IRemoteAutomationServerStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportSession(&*(&sessionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteAutomationServerStatics, BASE_OFFSET>(),
            ReportSession: ReportSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteAutomationServerStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IRemoteAutomationWindow_Impl: Sized {
    fn AutomationProvider(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn UnregisterAsync(&mut self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IRemoteAutomationWindow {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.IRemoteAutomationWindow";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IRemoteAutomationWindow_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRemoteAutomationWindow_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRemoteAutomationWindow_Vtbl {
        unsafe extern "system" fn AutomationProvider<Impl: IRemoteAutomationWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UnregisterAsync<Impl: IRemoteAutomationWindow_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IRemoteAutomationWindow, BASE_OFFSET>(),
            AutomationProvider: AutomationProvider::<Impl, IMPL_OFFSET>,
            UnregisterAsync: UnregisterAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRemoteAutomationWindow as ::windows::core::Interface>::IID
    }
}
