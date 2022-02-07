pub trait IAsyncAction_Impl: Sized + IAsyncInfo_Impl {
    fn SetCompleted(&self, handler: &::core::option::Option<AsyncActionCompletedHandler>) -> ::windows::core::Result<()>;
    fn Completed(&self) -> ::windows::core::Result<AsyncActionCompletedHandler>;
    fn GetResults(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAsyncAction {
    const NAME: &'static str = "Windows.Foundation.IAsyncAction";
}
impl IAsyncAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncAction_Impl, const OFFSET: isize>() -> IAsyncAction_Vtbl {
        unsafe extern "system" fn SetCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompleted(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn Completed<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Completed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResults<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetResults().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsyncAction, OFFSET>(),
            SetCompleted: SetCompleted::<Identity, Impl, OFFSET>,
            Completed: Completed::<Identity, Impl, OFFSET>,
            GetResults: GetResults::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncAction as ::windows::core::Interface>::IID
    }
}
pub trait IAsyncActionWithProgress_Impl<TProgress>: Sized + IAsyncInfo_Impl
where
    TProgress: ::windows::core::RuntimeType + 'static,
{
    fn SetProgress(&self, handler: &::core::option::Option<AsyncActionProgressHandler<TProgress>>) -> ::windows::core::Result<()>;
    fn Progress(&self) -> ::windows::core::Result<AsyncActionProgressHandler<TProgress>>;
    fn SetCompleted(&self, handler: &::core::option::Option<AsyncActionWithProgressCompletedHandler<TProgress>>) -> ::windows::core::Result<()>;
    fn Completed(&self) -> ::windows::core::Result<AsyncActionWithProgressCompletedHandler<TProgress>>;
    fn GetResults(&self) -> ::windows::core::Result<()>;
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IAsyncActionWithProgress<TProgress> {
    const NAME: &'static str = "Windows.Foundation.IAsyncActionWithProgress";
}
impl<TProgress: ::windows::core::RuntimeType + 'static> IAsyncActionWithProgress_Vtbl<TProgress> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>() -> IAsyncActionWithProgress_Vtbl<TProgress> {
        unsafe extern "system" fn SetProgress<TProgress: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProgress(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn Progress<TProgress: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompleted<TProgress: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompleted(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn Completed<TProgress: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Completed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResults<TProgress: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetResults().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsyncActionWithProgress<TProgress>, OFFSET>(),
            SetProgress: SetProgress::<TProgress, Identity, Impl, OFFSET>,
            Progress: Progress::<TProgress, Identity, Impl, OFFSET>,
            SetCompleted: SetCompleted::<TProgress, Identity, Impl, OFFSET>,
            Completed: Completed::<TProgress, Identity, Impl, OFFSET>,
            GetResults: GetResults::<TProgress, Identity, Impl, OFFSET>,
            TProgress: ::core::marker::PhantomData::<TProgress>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncActionWithProgress<TProgress> as ::windows::core::Interface>::IID
    }
}
pub trait IAsyncInfo_Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Status(&self) -> ::windows::core::Result<AsyncStatus>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAsyncInfo {
    const NAME: &'static str = "Windows.Foundation.IAsyncInfo";
}
impl IAsyncInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncInfo_Impl, const OFFSET: isize>() -> IAsyncInfo_Vtbl {
        unsafe extern "system" fn Id<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AsyncStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsyncInfo, OFFSET>(),
            Id: Id::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            ErrorCode: ErrorCode::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncInfo as ::windows::core::Interface>::IID
    }
}
pub trait IAsyncOperation_Impl<TResult>: Sized + IAsyncInfo_Impl
where
    TResult: ::windows::core::RuntimeType + 'static,
{
    fn SetCompleted(&self, handler: &::core::option::Option<AsyncOperationCompletedHandler<TResult>>) -> ::windows::core::Result<()>;
    fn Completed(&self) -> ::windows::core::Result<AsyncOperationCompletedHandler<TResult>>;
    fn GetResults(&self) -> ::windows::core::Result<TResult>;
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IAsyncOperation<TResult> {
    const NAME: &'static str = "Windows.Foundation.IAsyncOperation";
}
impl<TResult: ::windows::core::RuntimeType + 'static> IAsyncOperation_Vtbl<TResult> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperation_Impl<TResult>, const OFFSET: isize>() -> IAsyncOperation_Vtbl<TResult> {
        unsafe extern "system" fn SetCompleted<TResult: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperation_Impl<TResult>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompleted(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn Completed<TResult: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperation_Impl<TResult>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Completed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResults<TResult: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperation_Impl<TResult>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResults() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsyncOperation<TResult>, OFFSET>(),
            SetCompleted: SetCompleted::<TResult, Identity, Impl, OFFSET>,
            Completed: Completed::<TResult, Identity, Impl, OFFSET>,
            GetResults: GetResults::<TResult, Identity, Impl, OFFSET>,
            TResult: ::core::marker::PhantomData::<TResult>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncOperation<TResult> as ::windows::core::Interface>::IID
    }
}
pub trait IAsyncOperationWithProgress_Impl<TResult, TProgress>: Sized + IAsyncInfo_Impl
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static,
{
    fn SetProgress(&self, handler: &::core::option::Option<AsyncOperationProgressHandler<TResult, TProgress>>) -> ::windows::core::Result<()>;
    fn Progress(&self) -> ::windows::core::Result<AsyncOperationProgressHandler<TResult, TProgress>>;
    fn SetCompleted(&self, handler: &::core::option::Option<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>) -> ::windows::core::Result<()>;
    fn Completed(&self) -> ::windows::core::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>;
    fn GetResults(&self) -> ::windows::core::Result<TResult>;
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IAsyncOperationWithProgress<TResult, TProgress> {
    const NAME: &'static str = "Windows.Foundation.IAsyncOperationWithProgress";
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> IAsyncOperationWithProgress_Vtbl<TResult, TProgress> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>() -> IAsyncOperationWithProgress_Vtbl<TResult, TProgress> {
        unsafe extern "system" fn SetProgress<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProgress(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn Progress<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompleted<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCompleted(::core::mem::transmute(&handler)).into()
        }
        unsafe extern "system" fn Completed<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Completed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResults<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResults() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsyncOperationWithProgress<TResult, TProgress>, OFFSET>(),
            SetProgress: SetProgress::<TResult, TProgress, Identity, Impl, OFFSET>,
            Progress: Progress::<TResult, TProgress, Identity, Impl, OFFSET>,
            SetCompleted: SetCompleted::<TResult, TProgress, Identity, Impl, OFFSET>,
            Completed: Completed::<TResult, TProgress, Identity, Impl, OFFSET>,
            GetResults: GetResults::<TResult, TProgress, Identity, Impl, OFFSET>,
            TResult: ::core::marker::PhantomData::<TResult>,
            TProgress: ::core::marker::PhantomData::<TProgress>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncOperationWithProgress<TResult, TProgress> as ::windows::core::Interface>::IID
    }
}
pub trait IClosable_Impl: Sized {
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IClosable {
    const NAME: &'static str = "Windows.Foundation.IClosable";
}
impl IClosable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClosable_Impl, const OFFSET: isize>() -> IClosable_Vtbl {
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IClosable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IClosable, OFFSET>(), Close: Close::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClosable as ::windows::core::Interface>::IID
    }
}
pub trait IGetActivationFactory_Impl: Sized {
    fn GetActivationFactory(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IGetActivationFactory {
    const NAME: &'static str = "Windows.Foundation.IGetActivationFactory";
}
impl IGetActivationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetActivationFactory_Impl, const OFFSET: isize>() -> IGetActivationFactory_Vtbl {
        unsafe extern "system" fn GetActivationFactory<Identity: ::windows::core::IUnknownImpl, Impl: IGetActivationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetActivationFactory(::core::mem::transmute(&activatableclassid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGetActivationFactory, OFFSET>(),
            GetActivationFactory: GetActivationFactory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetActivationFactory as ::windows::core::Interface>::IID
    }
}
pub trait IMemoryBuffer_Impl: Sized + IClosable_Impl {
    fn CreateReference(&self) -> ::windows::core::Result<IMemoryBufferReference>;
}
impl ::windows::core::RuntimeName for IMemoryBuffer {
    const NAME: &'static str = "Windows.Foundation.IMemoryBuffer";
}
impl IMemoryBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryBuffer_Impl, const OFFSET: isize>() -> IMemoryBuffer_Vtbl {
        unsafe extern "system" fn CreateReference<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMemoryBuffer, OFFSET>(), CreateReference: CreateReference::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IMemoryBufferReference_Impl: Sized + IClosable_Impl {
    fn Capacity(&self) -> ::windows::core::Result<u32>;
    fn Closed(&self, handler: &::core::option::Option<TypedEventHandler<IMemoryBufferReference, ::windows::core::IInspectable>>) -> ::windows::core::Result<EventRegistrationToken>;
    fn RemoveClosed(&self, cookie: &EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMemoryBufferReference {
    const NAME: &'static str = "Windows.Foundation.IMemoryBufferReference";
}
impl IMemoryBufferReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryBufferReference_Impl, const OFFSET: isize>() -> IMemoryBufferReference_Vtbl {
        unsafe extern "system" fn Capacity<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Capacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Closed<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Closed(::core::mem::transmute(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveClosed(::core::mem::transmute(&cookie)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMemoryBufferReference, OFFSET>(),
            Capacity: Capacity::<Identity, Impl, OFFSET>,
            Closed: Closed::<Identity, Impl, OFFSET>,
            RemoveClosed: RemoveClosed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryBufferReference as ::windows::core::Interface>::IID
    }
}
pub trait IPropertyValue_Impl: Sized {
    fn Type(&self) -> ::windows::core::Result<PropertyType>;
    fn IsNumericScalar(&self) -> ::windows::core::Result<bool>;
    fn GetUInt8(&self) -> ::windows::core::Result<u8>;
    fn GetInt16(&self) -> ::windows::core::Result<i16>;
    fn GetUInt16(&self) -> ::windows::core::Result<u16>;
    fn GetInt32(&self) -> ::windows::core::Result<i32>;
    fn GetUInt32(&self) -> ::windows::core::Result<u32>;
    fn GetInt64(&self) -> ::windows::core::Result<i64>;
    fn GetUInt64(&self) -> ::windows::core::Result<u64>;
    fn GetSingle(&self) -> ::windows::core::Result<f32>;
    fn GetDouble(&self) -> ::windows::core::Result<f64>;
    fn GetChar16(&self) -> ::windows::core::Result<u16>;
    fn GetBoolean(&self) -> ::windows::core::Result<bool>;
    fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDateTime(&self) -> ::windows::core::Result<DateTime>;
    fn GetTimeSpan(&self) -> ::windows::core::Result<TimeSpan>;
    fn GetPoint(&self) -> ::windows::core::Result<Point>;
    fn GetSize(&self) -> ::windows::core::Result<Size>;
    fn GetRect(&self) -> ::windows::core::Result<Rect>;
    fn GetUInt8Array(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn GetInt16Array(&self, value: &mut ::windows::core::Array<i16>) -> ::windows::core::Result<()>;
    fn GetUInt16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()>;
    fn GetInt32Array(&self, value: &mut ::windows::core::Array<i32>) -> ::windows::core::Result<()>;
    fn GetUInt32Array(&self, value: &mut ::windows::core::Array<u32>) -> ::windows::core::Result<()>;
    fn GetInt64Array(&self, value: &mut ::windows::core::Array<i64>) -> ::windows::core::Result<()>;
    fn GetUInt64Array(&self, value: &mut ::windows::core::Array<u64>) -> ::windows::core::Result<()>;
    fn GetSingleArray(&self, value: &mut ::windows::core::Array<f32>) -> ::windows::core::Result<()>;
    fn GetDoubleArray(&self, value: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()>;
    fn GetChar16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()>;
    fn GetBooleanArray(&self, value: &mut ::windows::core::Array<bool>) -> ::windows::core::Result<()>;
    fn GetStringArray(&self, value: &mut ::windows::core::Array<::windows::core::HSTRING>) -> ::windows::core::Result<()>;
    fn GetInspectableArray(&self, value: &mut ::windows::core::Array<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetGuidArray(&self, value: &mut ::windows::core::Array<::windows::core::GUID>) -> ::windows::core::Result<()>;
    fn GetDateTimeArray(&self, value: &mut ::windows::core::Array<DateTime>) -> ::windows::core::Result<()>;
    fn GetTimeSpanArray(&self, value: &mut ::windows::core::Array<TimeSpan>) -> ::windows::core::Result<()>;
    fn GetPointArray(&self, value: &mut ::windows::core::Array<Point>) -> ::windows::core::Result<()>;
    fn GetSizeArray(&self, value: &mut ::windows::core::Array<Size>) -> ::windows::core::Result<()>;
    fn GetRectArray(&self, value: &mut ::windows::core::Array<Rect>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPropertyValue {
    const NAME: &'static str = "Windows.Foundation.IPropertyValue";
}
impl IPropertyValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>() -> IPropertyValue_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PropertyType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNumericScalar<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsNumericScalar() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt8<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUInt8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInt16<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInt16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt16<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUInt16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInt32<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInt32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt32<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUInt32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInt64<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInt64() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt64<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUInt64() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSingle<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSingle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDouble<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDouble() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChar16<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChar16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoolean<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBoolean() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuid<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDateTime<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeSpan<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTimeSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPoint<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRect<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt8Array<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUInt8Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInt16Array<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInt16Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetUInt16Array<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUInt16Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInt32Array<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInt32Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetUInt32Array<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUInt32Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInt64Array<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInt64Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetUInt64Array<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUInt64Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetSingleArray<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSingleArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetDoubleArray<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDoubleArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetChar16Array<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetChar16Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetBooleanArray<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBooleanArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetStringArray<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStringArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInspectableArray<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInspectableArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetGuidArray<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGuidArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetDateTimeArray<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDateTimeArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetTimeSpanArray<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTimeSpanArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetPointArray<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPointArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetSizeArray<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSizeArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetRectArray<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRectArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPropertyValue, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            IsNumericScalar: IsNumericScalar::<Identity, Impl, OFFSET>,
            GetUInt8: GetUInt8::<Identity, Impl, OFFSET>,
            GetInt16: GetInt16::<Identity, Impl, OFFSET>,
            GetUInt16: GetUInt16::<Identity, Impl, OFFSET>,
            GetInt32: GetInt32::<Identity, Impl, OFFSET>,
            GetUInt32: GetUInt32::<Identity, Impl, OFFSET>,
            GetInt64: GetInt64::<Identity, Impl, OFFSET>,
            GetUInt64: GetUInt64::<Identity, Impl, OFFSET>,
            GetSingle: GetSingle::<Identity, Impl, OFFSET>,
            GetDouble: GetDouble::<Identity, Impl, OFFSET>,
            GetChar16: GetChar16::<Identity, Impl, OFFSET>,
            GetBoolean: GetBoolean::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetGuid: GetGuid::<Identity, Impl, OFFSET>,
            GetDateTime: GetDateTime::<Identity, Impl, OFFSET>,
            GetTimeSpan: GetTimeSpan::<Identity, Impl, OFFSET>,
            GetPoint: GetPoint::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetRect: GetRect::<Identity, Impl, OFFSET>,
            GetUInt8Array: GetUInt8Array::<Identity, Impl, OFFSET>,
            GetInt16Array: GetInt16Array::<Identity, Impl, OFFSET>,
            GetUInt16Array: GetUInt16Array::<Identity, Impl, OFFSET>,
            GetInt32Array: GetInt32Array::<Identity, Impl, OFFSET>,
            GetUInt32Array: GetUInt32Array::<Identity, Impl, OFFSET>,
            GetInt64Array: GetInt64Array::<Identity, Impl, OFFSET>,
            GetUInt64Array: GetUInt64Array::<Identity, Impl, OFFSET>,
            GetSingleArray: GetSingleArray::<Identity, Impl, OFFSET>,
            GetDoubleArray: GetDoubleArray::<Identity, Impl, OFFSET>,
            GetChar16Array: GetChar16Array::<Identity, Impl, OFFSET>,
            GetBooleanArray: GetBooleanArray::<Identity, Impl, OFFSET>,
            GetStringArray: GetStringArray::<Identity, Impl, OFFSET>,
            GetInspectableArray: GetInspectableArray::<Identity, Impl, OFFSET>,
            GetGuidArray: GetGuidArray::<Identity, Impl, OFFSET>,
            GetDateTimeArray: GetDateTimeArray::<Identity, Impl, OFFSET>,
            GetTimeSpanArray: GetTimeSpanArray::<Identity, Impl, OFFSET>,
            GetPointArray: GetPointArray::<Identity, Impl, OFFSET>,
            GetSizeArray: GetSizeArray::<Identity, Impl, OFFSET>,
            GetRectArray: GetRectArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyValue as ::windows::core::Interface>::IID
    }
}
pub trait IReference_Impl<T>: Sized + IPropertyValue_Impl
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn Value(&self) -> ::windows::core::Result<T>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IReference<T> {
    const NAME: &'static str = "Windows.Foundation.IReference";
}
impl<T: ::windows::core::RuntimeType + 'static> IReference_Vtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReference_Impl<T>, const OFFSET: isize>() -> IReference_Vtbl<T> {
        unsafe extern "system" fn Value<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IReference_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IReference<T>, OFFSET>(),
            Value: Value::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReference<T> as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceArray_Impl<T>: Sized + IPropertyValue_Impl
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn Value(&self) -> ::windows::core::Result<::windows::core::Array<T>>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IReferenceArray<T> {
    const NAME: &'static str = "Windows.Foundation.IReferenceArray";
}
impl<T: ::windows::core::RuntimeType + 'static> IReferenceArray_Vtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceArray_Impl<T>, const OFFSET: isize>() -> IReferenceArray_Vtbl<T> {
        unsafe extern "system" fn Value<T: ::windows::core::RuntimeType + 'static, Identity: ::windows::core::IUnknownImpl, Impl: IReferenceArray_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IReferenceArray<T>, OFFSET>(),
            Value: Value::<T, Identity, Impl, OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceArray<T> as ::windows::core::Interface>::IID
    }
}
pub trait IStringable_Impl: Sized {
    fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IStringable {
    const NAME: &'static str = "Windows.Foundation.IStringable";
}
impl IStringable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStringable_Impl, const OFFSET: isize>() -> IStringable_Vtbl {
        unsafe extern "system" fn ToString<Identity: ::windows::core::IUnknownImpl, Impl: IStringable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ToString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStringable, OFFSET>(), ToString: ToString::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStringable as ::windows::core::Interface>::IID
    }
}
pub trait IWwwFormUrlDecoderEntry_Impl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IWwwFormUrlDecoderEntry {
    const NAME: &'static str = "Windows.Foundation.IWwwFormUrlDecoderEntry";
}
impl IWwwFormUrlDecoderEntry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWwwFormUrlDecoderEntry_Impl, const OFFSET: isize>() -> IWwwFormUrlDecoderEntry_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IWwwFormUrlDecoderEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: IWwwFormUrlDecoderEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWwwFormUrlDecoderEntry, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWwwFormUrlDecoderEntry as ::windows::core::Interface>::IID
    }
}
