#[cfg(feature = "implement_exclusive")]
pub trait IAutomationConnectionImpl: Sized {
    fn IsRemoteSystem(&self) -> ::windows::core::Result<bool>;
    fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExecutableFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationConnection {
    const NAME: &'static str = "Windows.UI.UIAutomation.IAutomationConnection";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationConnectionImpl, const OFFSET: isize>() -> IAutomationConnectionVtbl {
        unsafe extern "system" fn IsRemoteSystem<Impl: IAutomationConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AppUserModelId<Impl: IAutomationConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExecutableFileName<Impl: IAutomationConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationConnection>, ::windows::core::GetTrustLevel, IsRemoteSystem::<Impl, OFFSET>, AppUserModelId::<Impl, OFFSET>, ExecutableFileName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationConnectionBoundObjectImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<AutomationConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationConnectionBoundObject {
    const NAME: &'static str = "Windows.UI.UIAutomation.IAutomationConnectionBoundObject";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationConnectionBoundObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationConnectionBoundObjectImpl, const OFFSET: isize>() -> IAutomationConnectionBoundObjectVtbl {
        unsafe extern "system" fn Connection<Impl: IAutomationConnectionBoundObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationConnectionBoundObject>, ::windows::core::GetTrustLevel, Connection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationElementImpl: Sized {
    fn IsRemoteSystem(&self) -> ::windows::core::Result<bool>;
    fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExecutableFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationElement {
    const NAME: &'static str = "Windows.UI.UIAutomation.IAutomationElement";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationElementVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationElementImpl, const OFFSET: isize>() -> IAutomationElementVtbl {
        unsafe extern "system" fn IsRemoteSystem<Impl: IAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AppUserModelId<Impl: IAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ExecutableFileName<Impl: IAutomationElementImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationElement>, ::windows::core::GetTrustLevel, IsRemoteSystem::<Impl, OFFSET>, AppUserModelId::<Impl, OFFSET>, ExecutableFileName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IAutomationTextRangeImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IAutomationTextRange {
    const NAME: &'static str = "Windows.UI.UIAutomation.IAutomationTextRange";
}
#[cfg(feature = "implement_exclusive")]
impl IAutomationTextRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAutomationTextRangeImpl, const OFFSET: isize>() -> IAutomationTextRangeVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAutomationTextRange>, ::windows::core::GetTrustLevel)
    }
}
