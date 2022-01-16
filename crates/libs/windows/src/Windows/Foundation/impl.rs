pub trait IAsyncAction_Impl: Sized + IAsyncInfo_Impl {
    fn SetCompleted(&mut self, handler: &::core::option::Option<AsyncActionCompletedHandler>) -> ::windows::core::Result<()>;
    fn Completed(&mut self) -> ::windows::core::Result<AsyncActionCompletedHandler>;
    fn GetResults(&mut self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAsyncAction {
    const NAME: &'static str = "Windows.Foundation.IAsyncAction";
}
impl IAsyncAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncAction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncAction_Vtbl {
        unsafe extern "system" fn SetCompleted<Impl: IAsyncAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompleted(&*(&handler as *const <AsyncActionCompletedHandler as ::windows::core::Abi>::Abi as *const <AsyncActionCompletedHandler as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<Impl: IAsyncAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResults<Impl: IAsyncAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResults().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsyncAction, BASE_OFFSET>(),
            SetCompleted: SetCompleted::<Impl, IMPL_OFFSET>,
            Completed: Completed::<Impl, IMPL_OFFSET>,
            GetResults: GetResults::<Impl, IMPL_OFFSET>,
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
    fn SetProgress(&mut self, handler: &::core::option::Option<AsyncActionProgressHandler<TProgress>>) -> ::windows::core::Result<()>;
    fn Progress(&mut self) -> ::windows::core::Result<AsyncActionProgressHandler<TProgress>>;
    fn SetCompleted(&mut self, handler: &::core::option::Option<AsyncActionWithProgressCompletedHandler<TProgress>>) -> ::windows::core::Result<()>;
    fn Completed(&mut self) -> ::windows::core::Result<AsyncActionWithProgressCompletedHandler<TProgress>>;
    fn GetResults(&mut self) -> ::windows::core::Result<()>;
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IAsyncActionWithProgress<TProgress> {
    const NAME: &'static str = "Windows.Foundation.IAsyncActionWithProgress";
}
impl<TProgress: ::windows::core::RuntimeType + 'static> IAsyncActionWithProgress_Vtbl<TProgress> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncActionWithProgress_Impl<TProgress>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncActionWithProgress_Vtbl<TProgress> {
        unsafe extern "system" fn SetProgress<TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgress(&*(&handler as *const <AsyncActionProgressHandler<TProgress> as ::windows::core::Abi>::Abi as *const <AsyncActionProgressHandler<TProgress> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Progress<TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompleted<TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompleted(&*(&handler as *const <AsyncActionWithProgressCompletedHandler<TProgress> as ::windows::core::Abi>::Abi as *const <AsyncActionWithProgressCompletedHandler<TProgress> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResults<TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResults().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsyncActionWithProgress<TProgress>, BASE_OFFSET>(),
            SetProgress: SetProgress::<TProgress, Impl, IMPL_OFFSET>,
            Progress: Progress::<TProgress, Impl, IMPL_OFFSET>,
            SetCompleted: SetCompleted::<TProgress, Impl, IMPL_OFFSET>,
            Completed: Completed::<TProgress, Impl, IMPL_OFFSET>,
            GetResults: GetResults::<TProgress, Impl, IMPL_OFFSET>,
            TProgress: ::core::marker::PhantomData::<TProgress>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncActionWithProgress<TProgress> as ::windows::core::Interface>::IID
    }
}
pub trait IAsyncInfo_Impl: Sized {
    fn Id(&mut self) -> ::windows::core::Result<u32>;
    fn Status(&mut self) -> ::windows::core::Result<AsyncStatus>;
    fn ErrorCode(&mut self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAsyncInfo {
    const NAME: &'static str = "Windows.Foundation.IAsyncInfo";
}
impl IAsyncInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncInfo_Vtbl {
        unsafe extern "system" fn Id<Impl: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Id() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Impl: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AsyncStatus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Status() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Impl: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ErrorCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Close<Impl: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsyncInfo, BASE_OFFSET>(),
            Id: Id::<Impl, IMPL_OFFSET>,
            Status: Status::<Impl, IMPL_OFFSET>,
            ErrorCode: ErrorCode::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
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
    fn SetCompleted(&mut self, handler: &::core::option::Option<AsyncOperationCompletedHandler<TResult>>) -> ::windows::core::Result<()>;
    fn Completed(&mut self) -> ::windows::core::Result<AsyncOperationCompletedHandler<TResult>>;
    fn GetResults(&mut self) -> ::windows::core::Result<TResult>;
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IAsyncOperation<TResult> {
    const NAME: &'static str = "Windows.Foundation.IAsyncOperation";
}
impl<TResult: ::windows::core::RuntimeType + 'static> IAsyncOperation_Vtbl<TResult> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperation_Impl<TResult>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncOperation_Vtbl<TResult> {
        unsafe extern "system" fn SetCompleted<TResult: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperation_Impl<TResult>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompleted(&*(&handler as *const <AsyncOperationCompletedHandler<TResult> as ::windows::core::Abi>::Abi as *const <AsyncOperationCompletedHandler<TResult> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<TResult: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperation_Impl<TResult>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResults<TResult: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperation_Impl<TResult>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsyncOperation<TResult>, BASE_OFFSET>(),
            SetCompleted: SetCompleted::<TResult, Impl, IMPL_OFFSET>,
            Completed: Completed::<TResult, Impl, IMPL_OFFSET>,
            GetResults: GetResults::<TResult, Impl, IMPL_OFFSET>,
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
    fn SetProgress(&mut self, handler: &::core::option::Option<AsyncOperationProgressHandler<TResult, TProgress>>) -> ::windows::core::Result<()>;
    fn Progress(&mut self) -> ::windows::core::Result<AsyncOperationProgressHandler<TResult, TProgress>>;
    fn SetCompleted(&mut self, handler: &::core::option::Option<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>) -> ::windows::core::Result<()>;
    fn Completed(&mut self) -> ::windows::core::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>;
    fn GetResults(&mut self) -> ::windows::core::Result<TResult>;
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IAsyncOperationWithProgress<TResult, TProgress> {
    const NAME: &'static str = "Windows.Foundation.IAsyncOperationWithProgress";
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> IAsyncOperationWithProgress_Vtbl<TResult, TProgress> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncOperationWithProgress_Vtbl<TResult, TProgress> {
        unsafe extern "system" fn SetProgress<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgress(&*(&handler as *const <AsyncOperationProgressHandler<TResult, TProgress> as ::windows::core::Abi>::Abi as *const <AsyncOperationProgressHandler<TResult, TProgress> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Progress<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Progress() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompleted<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompleted(&*(&handler as *const <AsyncOperationWithProgressCompletedHandler<TResult, TProgress> as ::windows::core::Abi>::Abi as *const <AsyncOperationWithProgressCompletedHandler<TResult, TProgress> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Completed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResults<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IAsyncOperationWithProgress<TResult, TProgress>, BASE_OFFSET>(),
            SetProgress: SetProgress::<TResult, TProgress, Impl, IMPL_OFFSET>,
            Progress: Progress::<TResult, TProgress, Impl, IMPL_OFFSET>,
            SetCompleted: SetCompleted::<TResult, TProgress, Impl, IMPL_OFFSET>,
            Completed: Completed::<TResult, TProgress, Impl, IMPL_OFFSET>,
            GetResults: GetResults::<TResult, TProgress, Impl, IMPL_OFFSET>,
            TResult: ::core::marker::PhantomData::<TResult>,
            TProgress: ::core::marker::PhantomData::<TProgress>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncOperationWithProgress<TResult, TProgress> as ::windows::core::Interface>::IID
    }
}
pub trait IClosable_Impl: Sized {
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IClosable {
    const NAME: &'static str = "Windows.Foundation.IClosable";
}
impl IClosable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClosable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClosable_Vtbl {
        unsafe extern "system" fn Close<Impl: IClosable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IClosable, BASE_OFFSET>(), Close: Close::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClosable as ::windows::core::Interface>::IID
    }
}
pub trait IGetActivationFactory_Impl: Sized {
    fn GetActivationFactory(&mut self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IGetActivationFactory {
    const NAME: &'static str = "Windows.Foundation.IGetActivationFactory";
}
impl IGetActivationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetActivationFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetActivationFactory_Vtbl {
        unsafe extern "system" fn GetActivationFactory<Impl: IGetActivationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivationFactory(&*(&activatableclassid as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGetActivationFactory, BASE_OFFSET>(),
            GetActivationFactory: GetActivationFactory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetActivationFactory as ::windows::core::Interface>::IID
    }
}
pub trait IMemoryBuffer_Impl: Sized + IClosable_Impl {
    fn CreateReference(&mut self) -> ::windows::core::Result<IMemoryBufferReference>;
}
impl ::windows::core::RuntimeName for IMemoryBuffer {
    const NAME: &'static str = "Windows.Foundation.IMemoryBuffer";
}
impl IMemoryBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryBuffer_Vtbl {
        unsafe extern "system" fn CreateReference<Impl: IMemoryBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateReference() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMemoryBuffer, BASE_OFFSET>(), CreateReference: CreateReference::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IMemoryBufferReference_Impl: Sized + IClosable_Impl {
    fn Capacity(&mut self) -> ::windows::core::Result<u32>;
    fn Closed(&mut self, handler: &::core::option::Option<TypedEventHandler<IMemoryBufferReference, ::windows::core::IInspectable>>) -> ::windows::core::Result<EventRegistrationToken>;
    fn RemoveClosed(&mut self, cookie: &EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMemoryBufferReference {
    const NAME: &'static str = "Windows.Foundation.IMemoryBufferReference";
}
impl IMemoryBufferReference_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryBufferReference_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryBufferReference_Vtbl {
        unsafe extern "system" fn Capacity<Impl: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Capacity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Closed<Impl: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Closed(&*(&handler as *const <TypedEventHandler<IMemoryBufferReference, ::windows::core::IInspectable> as ::windows::core::Abi>::Abi as *const <TypedEventHandler<IMemoryBufferReference, ::windows::core::IInspectable> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Impl: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveClosed(&*(&cookie as *const <EventRegistrationToken as ::windows::core::Abi>::Abi as *const <EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMemoryBufferReference, BASE_OFFSET>(),
            Capacity: Capacity::<Impl, IMPL_OFFSET>,
            Closed: Closed::<Impl, IMPL_OFFSET>,
            RemoveClosed: RemoveClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryBufferReference as ::windows::core::Interface>::IID
    }
}
pub trait IPropertyValue_Impl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<PropertyType>;
    fn IsNumericScalar(&mut self) -> ::windows::core::Result<bool>;
    fn GetUInt8(&mut self) -> ::windows::core::Result<u8>;
    fn GetInt16(&mut self) -> ::windows::core::Result<i16>;
    fn GetUInt16(&mut self) -> ::windows::core::Result<u16>;
    fn GetInt32(&mut self) -> ::windows::core::Result<i32>;
    fn GetUInt32(&mut self) -> ::windows::core::Result<u32>;
    fn GetInt64(&mut self) -> ::windows::core::Result<i64>;
    fn GetUInt64(&mut self) -> ::windows::core::Result<u64>;
    fn GetSingle(&mut self) -> ::windows::core::Result<f32>;
    fn GetDouble(&mut self) -> ::windows::core::Result<f64>;
    fn GetChar16(&mut self) -> ::windows::core::Result<u16>;
    fn GetBoolean(&mut self) -> ::windows::core::Result<bool>;
    fn GetString(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetGuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetDateTime(&mut self) -> ::windows::core::Result<DateTime>;
    fn GetTimeSpan(&mut self) -> ::windows::core::Result<TimeSpan>;
    fn GetPoint(&mut self) -> ::windows::core::Result<Point>;
    fn GetSize(&mut self) -> ::windows::core::Result<Size>;
    fn GetRect(&mut self) -> ::windows::core::Result<Rect>;
    fn GetUInt8Array(&mut self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()>;
    fn GetInt16Array(&mut self, value: &mut ::windows::core::Array<i16>) -> ::windows::core::Result<()>;
    fn GetUInt16Array(&mut self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()>;
    fn GetInt32Array(&mut self, value: &mut ::windows::core::Array<i32>) -> ::windows::core::Result<()>;
    fn GetUInt32Array(&mut self, value: &mut ::windows::core::Array<u32>) -> ::windows::core::Result<()>;
    fn GetInt64Array(&mut self, value: &mut ::windows::core::Array<i64>) -> ::windows::core::Result<()>;
    fn GetUInt64Array(&mut self, value: &mut ::windows::core::Array<u64>) -> ::windows::core::Result<()>;
    fn GetSingleArray(&mut self, value: &mut ::windows::core::Array<f32>) -> ::windows::core::Result<()>;
    fn GetDoubleArray(&mut self, value: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()>;
    fn GetChar16Array(&mut self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()>;
    fn GetBooleanArray(&mut self, value: &mut ::windows::core::Array<bool>) -> ::windows::core::Result<()>;
    fn GetStringArray(&mut self, value: &mut ::windows::core::Array<::windows::core::HSTRING>) -> ::windows::core::Result<()>;
    fn GetInspectableArray(&mut self, value: &mut ::windows::core::Array<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn GetGuidArray(&mut self, value: &mut ::windows::core::Array<::windows::core::GUID>) -> ::windows::core::Result<()>;
    fn GetDateTimeArray(&mut self, value: &mut ::windows::core::Array<DateTime>) -> ::windows::core::Result<()>;
    fn GetTimeSpanArray(&mut self, value: &mut ::windows::core::Array<TimeSpan>) -> ::windows::core::Result<()>;
    fn GetPointArray(&mut self, value: &mut ::windows::core::Array<Point>) -> ::windows::core::Result<()>;
    fn GetSizeArray(&mut self, value: &mut ::windows::core::Array<Size>) -> ::windows::core::Result<()>;
    fn GetRectArray(&mut self, value: &mut ::windows::core::Array<Rect>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPropertyValue {
    const NAME: &'static str = "Windows.Foundation.IPropertyValue";
}
impl IPropertyValue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValue_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyValue_Vtbl {
        unsafe extern "system" fn Type<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PropertyType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNumericScalar<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNumericScalar() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt8<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUInt8() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInt16<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInt16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt16<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUInt16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInt32<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInt32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt32<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUInt32() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInt64<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInt64() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt64<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUInt64() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSingle<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSingle() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDouble<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDouble() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChar16<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChar16() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoolean<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBoolean() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuid<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDateTime<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDateTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeSpan<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimeSpan() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPoint<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRect<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt8Array<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUInt8Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInt16Array<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInt16Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetUInt16Array<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUInt16Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInt32Array<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInt32Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetUInt32Array<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUInt32Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInt64Array<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInt64Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetUInt64Array<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUInt64Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetSingleArray<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSingleArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetDoubleArray<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDoubleArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetChar16Array<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChar16Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetBooleanArray<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBooleanArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetStringArray<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStringArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInspectableArray<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInspectableArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetGuidArray<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGuidArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetDateTimeArray<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDateTimeArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetTimeSpanArray<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTimeSpanArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetPointArray<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPointArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetSizeArray<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSizeArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetRectArray<Impl: IPropertyValue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRectArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPropertyValue, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            IsNumericScalar: IsNumericScalar::<Impl, IMPL_OFFSET>,
            GetUInt8: GetUInt8::<Impl, IMPL_OFFSET>,
            GetInt16: GetInt16::<Impl, IMPL_OFFSET>,
            GetUInt16: GetUInt16::<Impl, IMPL_OFFSET>,
            GetInt32: GetInt32::<Impl, IMPL_OFFSET>,
            GetUInt32: GetUInt32::<Impl, IMPL_OFFSET>,
            GetInt64: GetInt64::<Impl, IMPL_OFFSET>,
            GetUInt64: GetUInt64::<Impl, IMPL_OFFSET>,
            GetSingle: GetSingle::<Impl, IMPL_OFFSET>,
            GetDouble: GetDouble::<Impl, IMPL_OFFSET>,
            GetChar16: GetChar16::<Impl, IMPL_OFFSET>,
            GetBoolean: GetBoolean::<Impl, IMPL_OFFSET>,
            GetString: GetString::<Impl, IMPL_OFFSET>,
            GetGuid: GetGuid::<Impl, IMPL_OFFSET>,
            GetDateTime: GetDateTime::<Impl, IMPL_OFFSET>,
            GetTimeSpan: GetTimeSpan::<Impl, IMPL_OFFSET>,
            GetPoint: GetPoint::<Impl, IMPL_OFFSET>,
            GetSize: GetSize::<Impl, IMPL_OFFSET>,
            GetRect: GetRect::<Impl, IMPL_OFFSET>,
            GetUInt8Array: GetUInt8Array::<Impl, IMPL_OFFSET>,
            GetInt16Array: GetInt16Array::<Impl, IMPL_OFFSET>,
            GetUInt16Array: GetUInt16Array::<Impl, IMPL_OFFSET>,
            GetInt32Array: GetInt32Array::<Impl, IMPL_OFFSET>,
            GetUInt32Array: GetUInt32Array::<Impl, IMPL_OFFSET>,
            GetInt64Array: GetInt64Array::<Impl, IMPL_OFFSET>,
            GetUInt64Array: GetUInt64Array::<Impl, IMPL_OFFSET>,
            GetSingleArray: GetSingleArray::<Impl, IMPL_OFFSET>,
            GetDoubleArray: GetDoubleArray::<Impl, IMPL_OFFSET>,
            GetChar16Array: GetChar16Array::<Impl, IMPL_OFFSET>,
            GetBooleanArray: GetBooleanArray::<Impl, IMPL_OFFSET>,
            GetStringArray: GetStringArray::<Impl, IMPL_OFFSET>,
            GetInspectableArray: GetInspectableArray::<Impl, IMPL_OFFSET>,
            GetGuidArray: GetGuidArray::<Impl, IMPL_OFFSET>,
            GetDateTimeArray: GetDateTimeArray::<Impl, IMPL_OFFSET>,
            GetTimeSpanArray: GetTimeSpanArray::<Impl, IMPL_OFFSET>,
            GetPointArray: GetPointArray::<Impl, IMPL_OFFSET>,
            GetSizeArray: GetSizeArray::<Impl, IMPL_OFFSET>,
            GetRectArray: GetRectArray::<Impl, IMPL_OFFSET>,
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
    fn Value(&mut self) -> ::windows::core::Result<T>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IReference<T> {
    const NAME: &'static str = "Windows.Foundation.IReference";
}
impl<T: ::windows::core::RuntimeType + 'static> IReference_Vtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReference_Impl<T>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReference_Vtbl<T> {
        unsafe extern "system" fn Value<T: ::windows::core::RuntimeType + 'static, Impl: IReference_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IReference<T>, BASE_OFFSET>(),
            Value: Value::<T, Impl, IMPL_OFFSET>,
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
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::Array<T>>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IReferenceArray<T> {
    const NAME: &'static str = "Windows.Foundation.IReferenceArray";
}
impl<T: ::windows::core::RuntimeType + 'static> IReferenceArray_Vtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceArray_Impl<T>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceArray_Vtbl<T> {
        unsafe extern "system" fn Value<T: ::windows::core::RuntimeType + 'static, Impl: IReferenceArray_Impl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IReferenceArray<T>, BASE_OFFSET>(),
            Value: Value::<T, Impl, IMPL_OFFSET>,
            T: ::core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceArray<T> as ::windows::core::Interface>::IID
    }
}
pub trait IStringable_Impl: Sized {
    fn ToString(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IStringable {
    const NAME: &'static str = "Windows.Foundation.IStringable";
}
impl IStringable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStringable_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStringable_Vtbl {
        unsafe extern "system" fn ToString<Impl: IStringable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToString() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IStringable, BASE_OFFSET>(), ToString: ToString::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStringable as ::windows::core::Interface>::IID
    }
}
pub trait IWwwFormUrlDecoderEntry_Impl: Sized {
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IWwwFormUrlDecoderEntry {
    const NAME: &'static str = "Windows.Foundation.IWwwFormUrlDecoderEntry";
}
impl IWwwFormUrlDecoderEntry_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWwwFormUrlDecoderEntry_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWwwFormUrlDecoderEntry_Vtbl {
        unsafe extern "system" fn Name<Impl: IWwwFormUrlDecoderEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IWwwFormUrlDecoderEntry_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
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
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWwwFormUrlDecoderEntry, BASE_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWwwFormUrlDecoderEntry as ::windows::core::Interface>::IID
    }
}
