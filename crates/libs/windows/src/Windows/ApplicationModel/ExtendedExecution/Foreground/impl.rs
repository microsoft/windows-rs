#[cfg(feature = "implement_exclusive")]
pub trait IExtendedExecutionForegroundRevokedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionForegroundRevokedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExtendedExecutionForegroundRevokedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundRevokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IExtendedExecutionForegroundRevokedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendedExecutionForegroundRevokedEventArgsImpl, const OFFSET: isize>() -> IExtendedExecutionForegroundRevokedEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IExtendedExecutionForegroundRevokedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ExtendedExecutionForegroundRevokedReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IExtendedExecutionForegroundRevokedEventArgs>, ::windows::core::GetTrustLevel, Reason::<Impl, OFFSET>)
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IExtendedExecutionForegroundSessionImpl: Sized + IClosableImpl {
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Revoked(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionForegroundRevokedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveRevoked(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestExtensionAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<ExtendedExecutionForegroundResult>>;
    fn Reason(&self) -> ::windows::core::Result<ExtendedExecutionForegroundReason>;
    fn SetReason(&self, value: ExtendedExecutionForegroundReason) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IExtendedExecutionForegroundSession {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.Foreground.IExtendedExecutionForegroundSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IExtendedExecutionForegroundSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendedExecutionForegroundSessionImpl, const OFFSET: isize>() -> IExtendedExecutionForegroundSessionVtbl {
        unsafe extern "system" fn Description<Impl: IExtendedExecutionForegroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IExtendedExecutionForegroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Revoked<Impl: IExtendedExecutionForegroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Revoked(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionForegroundRevokedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionForegroundRevokedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRevoked<Impl: IExtendedExecutionForegroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRevoked(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestExtensionAsync<Impl: IExtendedExecutionForegroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestExtensionAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reason<Impl: IExtendedExecutionForegroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ExtendedExecutionForegroundReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reason() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReason<Impl: IExtendedExecutionForegroundSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ExtendedExecutionForegroundReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReason(value).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IExtendedExecutionForegroundSession>,
            ::windows::core::GetTrustLevel,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            Revoked::<Impl, OFFSET>,
            RemoveRevoked::<Impl, OFFSET>,
            RequestExtensionAsync::<Impl, OFFSET>,
            Reason::<Impl, OFFSET>,
            SetReason::<Impl, OFFSET>,
        )
    }
}
