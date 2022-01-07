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
    pub const fn new<Impl: IAutomationConnectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationConnectionVtbl {
        unsafe extern "system" fn IsRemoteSystem<Impl: IAutomationConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRemoteSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppUserModelId<Impl: IAutomationConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppUserModelId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecutableFileName<Impl: IAutomationConnectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecutableFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationConnection>, base.5, IsRemoteSystem::<Impl, OFFSET>, AppUserModelId::<Impl, OFFSET>, ExecutableFileName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationConnectionBoundObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationConnectionBoundObjectVtbl {
        unsafe extern "system" fn Connection<Impl: IAutomationConnectionBoundObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationConnectionBoundObject>, base.5, Connection::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationElementVtbl {
        unsafe extern "system" fn IsRemoteSystem<Impl: IAutomationElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsRemoteSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppUserModelId<Impl: IAutomationElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AppUserModelId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExecutableFileName<Impl: IAutomationElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ExecutableFileName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationElement>, base.5, IsRemoteSystem::<Impl, OFFSET>, AppUserModelId::<Impl, OFFSET>, ExecutableFileName::<Impl, OFFSET>)
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
    pub const fn new<Impl: IAutomationTextRangeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAutomationTextRangeVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAutomationTextRange>, base.5)
    }
}
