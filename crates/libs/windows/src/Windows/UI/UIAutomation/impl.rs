#[cfg(feature = "implement_exclusive")]
pub trait IAutomationConnection_Impl: Sized {
    fn IsRemoteSystem(&mut self) -> ::windows::core::Result<bool>;
    fn AppUserModelId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExecutableFileName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationConnection {
    const NAME: &'static str = "Windows.UI.UIAutomation.IAutomationConnection";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationConnection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationConnection_Vtbl {
        unsafe extern "system" fn IsRemoteSystem<Impl: IAutomationConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRemoteSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppUserModelId<Impl: IAutomationConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppUserModelId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecutableFileName<Impl: IAutomationConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecutableFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationConnection, BASE_OFFSET>(),
            IsRemoteSystem: IsRemoteSystem::<Impl, IMPL_OFFSET>,
            AppUserModelId: AppUserModelId::<Impl, IMPL_OFFSET>,
            ExecutableFileName: ExecutableFileName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationConnection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationConnectionBoundObject_Impl: Sized {
    fn Connection(&mut self) -> ::windows::core::Result<AutomationConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationConnectionBoundObject {
    const NAME: &'static str = "Windows.UI.UIAutomation.IAutomationConnectionBoundObject";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationConnectionBoundObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationConnectionBoundObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationConnectionBoundObject_Vtbl {
        unsafe extern "system" fn Connection<Impl: IAutomationConnectionBoundObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationConnectionBoundObject, BASE_OFFSET>(),
            Connection: Connection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationConnectionBoundObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElement_Impl: Sized {
    fn IsRemoteSystem(&mut self) -> ::windows::core::Result<bool>;
    fn AppUserModelId(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExecutableFileName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElement {
    const NAME: &'static str = "Windows.UI.UIAutomation.IAutomationElement";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElement_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElement_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationElement_Vtbl {
        unsafe extern "system" fn IsRemoteSystem<Impl: IAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRemoteSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppUserModelId<Impl: IAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppUserModelId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecutableFileName<Impl: IAutomationElement_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExecutableFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationElement, BASE_OFFSET>(),
            IsRemoteSystem: IsRemoteSystem::<Impl, IMPL_OFFSET>,
            AppUserModelId: AppUserModelId::<Impl, IMPL_OFFSET>,
            ExecutableFileName: ExecutableFileName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationElement as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationTextRange_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationTextRange {
    const NAME: &'static str = "Windows.UI.UIAutomation.IAutomationTextRange";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationTextRange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationTextRange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAutomationTextRange_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IAutomationTextRange, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAutomationTextRange as ::windows::core::Interface>::IID
    }
}
