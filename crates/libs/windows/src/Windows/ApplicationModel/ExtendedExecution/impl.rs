#[cfg(feature = "implement_exclusive")]
pub trait IExtendedExecutionRevokedEventArgsImpl: Sized {
    fn Reason(&mut self) -> ::windows::core::Result<ExtendedExecutionRevokedReason>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IExtendedExecutionRevokedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.IExtendedExecutionRevokedEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IExtendedExecutionRevokedEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendedExecutionRevokedEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendedExecutionRevokedEventArgsVtbl {
        unsafe extern "system" fn Reason<Impl: IExtendedExecutionRevokedEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ExtendedExecutionRevokedReason) -> ::windows::core::HRESULT {
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
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IExtendedExecutionRevokedEventArgs, BASE_OFFSET>(), Reason: Reason::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendedExecutionRevokedEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IExtendedExecutionSessionImpl: Sized + IClosableImpl {
    fn Reason(&mut self) -> ::windows::core::Result<ExtendedExecutionReason>;
    fn SetReason(&mut self, value: ExtendedExecutionReason) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDescription(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn PercentProgress(&mut self) -> ::windows::core::Result<u32>;
    fn SetPercentProgress(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Revoked(&mut self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionRevokedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRevoked(&mut self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RequestExtensionAsync(&mut self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ExtendedExecutionResult>>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IExtendedExecutionSession {
    const NAME: &'static str = "Windows.ApplicationModel.ExtendedExecution.IExtendedExecutionSession";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IExtendedExecutionSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExtendedExecutionSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExtendedExecutionSessionVtbl {
        unsafe extern "system" fn Reason<Impl: IExtendedExecutionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ExtendedExecutionReason) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetReason<Impl: IExtendedExecutionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ExtendedExecutionReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReason(value).into()
        }
        unsafe extern "system" fn Description<Impl: IExtendedExecutionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDescription<Impl: IExtendedExecutionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PercentProgress<Impl: IExtendedExecutionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PercentProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPercentProgress<Impl: IExtendedExecutionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPercentProgress(value).into()
        }
        unsafe extern "system" fn Revoked<Impl: IExtendedExecutionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Revoked(&*(&handler as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionRevokedEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TypedEventHandler<::windows::core::IInspectable, ExtendedExecutionRevokedEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveRevoked<Impl: IExtendedExecutionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRevoked(&*(&token as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn RequestExtensionAsync<Impl: IExtendedExecutionSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IExtendedExecutionSession, BASE_OFFSET>(),
            Reason: Reason::<Impl, IMPL_OFFSET>,
            SetReason: SetReason::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            PercentProgress: PercentProgress::<Impl, IMPL_OFFSET>,
            SetPercentProgress: SetPercentProgress::<Impl, IMPL_OFFSET>,
            Revoked: Revoked::<Impl, IMPL_OFFSET>,
            RemoveRevoked: RemoveRevoked::<Impl, IMPL_OFFSET>,
            RequestExtensionAsync: RequestExtensionAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExtendedExecutionSession as ::windows::core::Interface>::IID
    }
}
