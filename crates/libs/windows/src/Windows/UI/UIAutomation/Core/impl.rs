pub trait ICoreAutomationConnectionBoundObjectProvider_Impl: Sized {
    fn IsComThreadingRequired(&mut self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ICoreAutomationConnectionBoundObjectProvider {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationConnectionBoundObjectProvider";
}
impl ICoreAutomationConnectionBoundObjectProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationConnectionBoundObjectProvider_Impl, const OFFSET: isize>() -> ICoreAutomationConnectionBoundObjectProvider_Vtbl {
        unsafe extern "system" fn IsComThreadingRequired<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationConnectionBoundObjectProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAutomationConnectionBoundObjectProvider, OFFSET>(),
            IsComThreadingRequired: IsComThreadingRequired::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAutomationConnectionBoundObjectProvider as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>() -> ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
        unsafe extern "system" fn CallExtension<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensionid: ::windows::core::GUID, context: ::windows::core::RawPtr, operandIds_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CallExtension(&*(&extensionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <CoreAutomationRemoteOperationContext as ::windows::core::Abi>::Abi as *const <CoreAutomationRemoteOperationContext as ::windows::core::DefaultType>::DefaultType), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&operandids), operandIds_array_size as _)).into()
        }
        unsafe extern "system" fn IsExtensionSupported<Identity: ::windows::core::IUnknownImpl, Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensionid: ::windows::core::GUID, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAutomationRemoteOperationExtensionProvider, OFFSET>(),
            CallExtension: CallExtension::<Identity, Impl, OFFSET>,
            IsExtensionSupported: IsExtensionSupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAutomationRemoteOperationExtensionProvider as ::windows::core::Interface>::IID
    }
}
