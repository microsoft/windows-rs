pub trait ICoreAutomationConnectionBoundObjectProvider_Impl: Sized {
    fn IsComThreadingRequired(&self) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ICoreAutomationConnectionBoundObjectProvider {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationConnectionBoundObjectProvider";
}
impl ICoreAutomationConnectionBoundObjectProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreAutomationConnectionBoundObjectProvider_Impl, const OFFSET: isize>() -> ICoreAutomationConnectionBoundObjectProvider_Vtbl {
        unsafe extern "system" fn IsComThreadingRequired<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreAutomationConnectionBoundObjectProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsComThreadingRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAutomationConnectionBoundObjectProvider, OFFSET>(),
            IsComThreadingRequired: IsComThreadingRequired::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAutomationConnectionBoundObjectProvider as ::windows::core::Interface>::IID
    }
}
pub trait ICoreAutomationRemoteOperationExtensionProvider_Impl: Sized {
    fn CallExtension(&self, extensionid: &::windows::core::GUID, context: &::core::option::Option<CoreAutomationRemoteOperationContext>, operandids: &[AutomationRemoteOperationOperandId]) -> ::windows::core::Result<()>;
    fn IsExtensionSupported(&self, extensionid: &::windows::core::GUID) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for ICoreAutomationRemoteOperationExtensionProvider {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationExtensionProvider";
}
impl ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>() -> ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
        unsafe extern "system" fn CallExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensionid: ::windows::core::GUID, context: *mut ::core::ffi::c_void, operandIds_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CallExtension(::core::mem::transmute(&extensionid), ::core::mem::transmute(&context), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&operandids), operandIds_array_size as _)).into()
        }
        unsafe extern "system" fn IsExtensionSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensionid: ::windows::core::GUID, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsExtensionSupported(::core::mem::transmute(&extensionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IInspectableVtbl::new::<Identity, ICoreAutomationRemoteOperationExtensionProvider, OFFSET>(),
            CallExtension: CallExtension::<Identity, Impl, OFFSET>,
            IsExtensionSupported: IsExtensionSupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICoreAutomationRemoteOperationExtensionProvider as ::windows::core::Interface>::IID
    }
}
