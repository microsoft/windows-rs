pub trait ICoreAutomationConnectionBoundObjectProvider_Impl: Sized {
    fn IsComThreadingRequired(&self) -> ::windows_core::Result<bool>;
}
impl ::windows_core::RuntimeName for ICoreAutomationConnectionBoundObjectProvider {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationConnectionBoundObjectProvider";
}
impl ICoreAutomationConnectionBoundObjectProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreAutomationConnectionBoundObjectProvider_Impl, const OFFSET: isize>() -> ICoreAutomationConnectionBoundObjectProvider_Vtbl {
        unsafe extern "system" fn IsComThreadingRequired<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreAutomationConnectionBoundObjectProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsComThreadingRequired() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ICoreAutomationConnectionBoundObjectProvider, OFFSET>(),
            IsComThreadingRequired: IsComThreadingRequired::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ICoreAutomationConnectionBoundObjectProvider as ::windows_core::ComInterface>::IID
    }
}
pub trait ICoreAutomationRemoteOperationExtensionProvider_Impl: Sized {
    fn CallExtension(&self, extensionid: &::windows_core::GUID, context: ::core::option::Option<&CoreAutomationRemoteOperationContext>, operandids: &[AutomationRemoteOperationOperandId]) -> ::windows_core::Result<()>;
    fn IsExtensionSupported(&self, extensionid: &::windows_core::GUID) -> ::windows_core::Result<bool>;
}
impl ::windows_core::RuntimeName for ICoreAutomationRemoteOperationExtensionProvider {
    const NAME: &'static str = "Windows.UI.UIAutomation.Core.ICoreAutomationRemoteOperationExtensionProvider";
}
impl ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>() -> ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
        unsafe extern "system" fn CallExtension<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensionid: ::windows_core::GUID, context: *mut ::core::ffi::c_void, operandIds_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CallExtension(::core::mem::transmute(&extensionid), ::windows_core::from_raw_borrowed(&context), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&operandids), operandIds_array_size as usize)).into()
        }
        unsafe extern "system" fn IsExtensionSupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extensionid: ::windows_core::GUID, result__: *mut bool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsExtensionSupported(::core::mem::transmute(&extensionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IInspectable_Vtbl::new::<Identity, ICoreAutomationRemoteOperationExtensionProvider, OFFSET>(),
            CallExtension: CallExtension::<Identity, Impl, OFFSET>,
            IsExtensionSupported: IsExtensionSupported::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <ICoreAutomationRemoteOperationExtensionProvider as ::windows_core::ComInterface>::IID
    }
}
