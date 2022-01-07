#[cfg(feature = "implement_exclusive")]
pub trait IExtendedExecutionRevokedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionRevokedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExtendedExecutionRevokedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.IExtendedExecutionRevokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IExtendedExecutionRevokedEventArgsVtbl {
    pub const fn new<Impl: IExtendedExecutionRevokedEventArgsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IExtendedExecutionRevokedEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IExtendedExecutionRevokedEventArgsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ExtendedExecutionRevokedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IExtendedExecutionRevokedEventArgs>, base.5, Reason::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IExtendedExecutionSessionImpl: Sized + IClosableImpl {
    fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionReason>;
    fn SetReason(&self, value: ExtendedExecutionReason) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PercentProgress(&self) -> ::windows::core::Result<u32>;
    fn SetPercentProgress(&self, value: u32) -> ::windows::core::Result<()>;
    fn Revoked(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionRevokedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRevoked(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestExtensionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ExtendedExecutionResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IExtendedExecutionSession {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.IExtendedExecutionSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IExtendedExecutionSessionVtbl {
    pub const fn new<Impl: IExtendedExecutionSessionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IExtendedExecutionSessionVtbl {
        unsafe extern "system" fn Reason<Impl: IExtendedExecutionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ExtendedExecutionReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReason<Impl: IExtendedExecutionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ExtendedExecutionReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetReason(value).into()
        }
        unsafe extern "system" fn Description<Impl: IExtendedExecutionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IExtendedExecutionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PercentProgress<Impl: IExtendedExecutionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PercentProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPercentProgress<Impl: IExtendedExecutionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetPercentProgress(value).into()
        }
        unsafe extern "system" fn Revoked<Impl: IExtendedExecutionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Revoked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionRevokedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionRevokedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRevoked<Impl: IExtendedExecutionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).RemoveRevoked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestExtensionAsync<Impl: IExtendedExecutionSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RequestExtensionAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IExtendedExecutionSession>, base.5, Reason::<Impl, OFFSET>, SetReason::<Impl, OFFSET>, Description::<Impl, OFFSET>, SetDescription::<Impl, OFFSET>, PercentProgress::<Impl, OFFSET>, SetPercentProgress::<Impl, OFFSET>, Revoked::<Impl, OFFSET>, RemoveRevoked::<Impl, OFFSET>, RequestExtensionAsync::<Impl, OFFSET>)
    }
}
