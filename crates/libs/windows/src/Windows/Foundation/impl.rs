pub trait IAsyncActionImpl: Sized + IAsyncInfoImpl {
    fn SetCompleted(&self, handler: &::core::option::Option<AsyncActionCompletedHandler>) -> ::windows::core::Result<()>;
    fn Completed(&self) -> ::windows::core::Result<AsyncActionCompletedHandler>;
    fn GetResults(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAsyncAction {
    const NAME: &'static str = "Windows.Foundation.IAsyncAction";
}
impl IAsyncActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncActionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncActionVtbl {
        unsafe extern "system" fn SetCompleted<Impl: IAsyncActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompleted(&*(&handler as *const <AsyncActionCompletedHandler as ::windows::core::Abi>::Abi as *const <AsyncActionCompletedHandler as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<Impl: IAsyncActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetResults<Impl: IAsyncActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IAsyncActionWithProgressImpl<TProgress>: Sized + IAsyncInfoImpl
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
impl<TProgress: ::windows::core::RuntimeType + 'static> IAsyncActionWithProgressVtbl<TProgress> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncActionWithProgressImpl<TProgress>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncActionWithProgressVtbl<TProgress> {
        unsafe extern "system" fn SetProgress<TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncActionWithProgressImpl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgress(&*(&handler as *const <AsyncActionProgressHandler<TProgress> as ::windows::core::Abi>::Abi as *const <AsyncActionProgressHandler<TProgress> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Progress<TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncActionWithProgressImpl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCompleted<TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncActionWithProgressImpl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompleted(&*(&handler as *const <AsyncActionWithProgressCompletedHandler<TProgress> as ::windows::core::Abi>::Abi as *const <AsyncActionWithProgressCompletedHandler<TProgress> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncActionWithProgressImpl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetResults<TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncActionWithProgressImpl<TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IAsyncInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn Status(&self) -> ::windows::core::Result<AsyncStatus>;
    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAsyncInfo {
    const NAME: &'static str = "Windows.Foundation.IAsyncInfo";
}
impl IAsyncInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncInfoVtbl {
        unsafe extern "system" fn Id<Impl: IAsyncInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Status<Impl: IAsyncInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut AsyncStatus) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ErrorCode<Impl: IAsyncInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Cancel<Impl: IAsyncInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn Close<Impl: IAsyncInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait IAsyncOperationImpl<TResult>: Sized + IAsyncInfoImpl
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
impl<TResult: ::windows::core::RuntimeType + 'static> IAsyncOperationVtbl<TResult> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperationImpl<TResult>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncOperationVtbl<TResult> {
        unsafe extern "system" fn SetCompleted<TResult: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperationImpl<TResult>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompleted(&*(&handler as *const <AsyncOperationCompletedHandler<TResult> as ::windows::core::Abi>::Abi as *const <AsyncOperationCompletedHandler<TResult> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<TResult: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperationImpl<TResult>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetResults<TResult: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperationImpl<TResult>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
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
pub trait IAsyncOperationWithProgressImpl<TResult, TProgress>: Sized + IAsyncInfoImpl
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
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> IAsyncOperationWithProgressVtbl<TResult, TProgress> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncOperationWithProgressImpl<TResult, TProgress>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncOperationWithProgressVtbl<TResult, TProgress> {
        unsafe extern "system" fn SetProgress<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperationWithProgressImpl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgress(&*(&handler as *const <AsyncOperationProgressHandler<TResult, TProgress> as ::windows::core::Abi>::Abi as *const <AsyncOperationProgressHandler<TResult, TProgress> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Progress<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperationWithProgressImpl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetCompleted<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperationWithProgressImpl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCompleted(&*(&handler as *const <AsyncOperationWithProgressCompletedHandler<TResult, TProgress> as ::windows::core::Abi>::Abi as *const <AsyncOperationWithProgressCompletedHandler<TResult, TProgress> as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Completed<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperationWithProgressImpl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetResults<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, Impl: IAsyncOperationWithProgressImpl<TResult, TProgress>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
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
pub trait IClosableImpl: Sized {
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IClosable {
    const NAME: &'static str = "Windows.Foundation.IClosable";
}
impl IClosableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClosableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClosableVtbl {
        unsafe extern "system" fn Close<Impl: IClosableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IClosable, BASE_OFFSET>(), Close: Close::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClosable as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeferralImpl: Sized + IClosableImpl {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeferral {
    const NAME: &'static str = "Windows.Foundation.IDeferral";
}
#[cfg(feature = "implement_exclusive")]
impl IDeferralVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeferralImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeferralVtbl {
        unsafe extern "system" fn Complete<Impl: IDeferralImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Complete().into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDeferral, BASE_OFFSET>(), Complete: Complete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeferral as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IDeferralFactoryImpl: Sized {
    fn Create(&self, handler: &::core::option::Option<DeferralCompletedHandler>) -> ::windows::core::Result<Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IDeferralFactory {
    const NAME: &'static str = "Windows.Foundation.IDeferralFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IDeferralFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeferralFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeferralFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IDeferralFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&handler as *const <DeferralCompletedHandler as ::windows::core::Abi>::Abi as *const <DeferralCompletedHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IDeferralFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeferralFactory as ::windows::core::Interface>::IID
    }
}
pub trait IGetActivationFactoryImpl: Sized {
    fn GetActivationFactory(&self, activatableclassid: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
impl ::windows::core::RuntimeName for IGetActivationFactory {
    const NAME: &'static str = "Windows.Foundation.IGetActivationFactory";
}
impl IGetActivationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetActivationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetActivationFactoryVtbl {
        unsafe extern "system" fn GetActivationFactory<Impl: IGetActivationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
#[cfg(feature = "implement_exclusive")]
pub trait IGuidHelperStaticsImpl: Sized {
    fn CreateNewGuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Empty(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Equals(&self, target: &::windows::core::GUID, value: &::windows::core::GUID) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IGuidHelperStatics {
    const NAME: &'static str = "Windows.Foundation.IGuidHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IGuidHelperStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGuidHelperStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGuidHelperStaticsVtbl {
        unsafe extern "system" fn CreateNewGuid<Impl: IGuidHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNewGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Empty<Impl: IGuidHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Empty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: IGuidHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: &::windows::core::GUID, value: &::windows::core::GUID, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Equals(&*(target as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IGuidHelperStatics, BASE_OFFSET>(),
            CreateNewGuid: CreateNewGuid::<Impl, IMPL_OFFSET>,
            Empty: Empty::<Impl, IMPL_OFFSET>,
            Equals: Equals::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGuidHelperStatics as ::windows::core::Interface>::IID
    }
}
pub trait IMemoryBufferImpl: Sized + IClosableImpl {
    fn CreateReference(&self) -> ::windows::core::Result<IMemoryBufferReference>;
}
impl ::windows::core::RuntimeName for IMemoryBuffer {
    const NAME: &'static str = "Windows.Foundation.IMemoryBuffer";
}
impl IMemoryBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryBufferVtbl {
        unsafe extern "system" fn CreateReference<Impl: IMemoryBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
#[cfg(feature = "implement_exclusive")]
pub trait IMemoryBufferFactoryImpl: Sized {
    fn Create(&self, capacity: u32) -> ::windows::core::Result<MemoryBuffer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMemoryBufferFactory {
    const NAME: &'static str = "Windows.Foundation.IMemoryBufferFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMemoryBufferFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryBufferFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryBufferFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IMemoryBufferFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, capacity: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(capacity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMemoryBufferFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMemoryBufferFactory as ::windows::core::Interface>::IID
    }
}
pub trait IMemoryBufferReferenceImpl: Sized + IClosableImpl {
    fn Capacity(&self) -> ::windows::core::Result<u32>;
    fn Closed(&self, handler: &::core::option::Option<TypedEventHandler<IMemoryBufferReference, ::windows::core::IInspectable>>) -> ::windows::core::Result<EventRegistrationToken>;
    fn RemoveClosed(&self, cookie: &EventRegistrationToken) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMemoryBufferReference {
    const NAME: &'static str = "Windows.Foundation.IMemoryBufferReference";
}
impl IMemoryBufferReferenceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMemoryBufferReferenceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMemoryBufferReferenceVtbl {
        unsafe extern "system" fn Capacity<Impl: IMemoryBufferReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Closed<Impl: IMemoryBufferReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut EventRegistrationToken) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RemoveClosed<Impl: IMemoryBufferReferenceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: EventRegistrationToken) -> ::windows::core::HRESULT {
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
pub trait IPropertyValueImpl: Sized {
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
impl IPropertyValueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyValueVtbl {
        unsafe extern "system" fn Type<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PropertyType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsNumericScalar<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetUInt8<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u8) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetInt16<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetUInt16<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetInt32<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetUInt32<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetInt64<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetUInt64<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSingle<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDouble<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetChar16<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetBoolean<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetString<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetGuid<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDateTime<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut DateTime) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetTimeSpan<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut TimeSpan) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetPoint<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Point) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetSize<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Size) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetRect<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut Rect) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetUInt8Array<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUInt8Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInt16Array<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInt16Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetUInt16Array<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUInt16Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInt32Array<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInt32Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetUInt32Array<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUInt32Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInt64Array<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInt64Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetUInt64Array<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUInt64Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetSingleArray<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSingleArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetDoubleArray<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDoubleArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetChar16Array<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChar16Array(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetBooleanArray<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBooleanArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetStringArray<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStringArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInspectableArray<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInspectableArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetGuidArray<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGuidArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetDateTimeArray<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDateTimeArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetTimeSpanArray<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTimeSpanArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetPointArray<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPointArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetSizeArray<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSizeArray(::windows::core::ArrayProxy::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetRectArray<Impl: IPropertyValueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Rect) -> ::windows::core::HRESULT {
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
#[cfg(feature = "implement_exclusive")]
pub trait IPropertyValueStaticsImpl: Sized {
    fn CreateEmpty(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt8(&self, value: u8) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInt16(&self, value: i16) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt16(&self, value: u16) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInt32(&self, value: i32) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt32(&self, value: u32) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInt64(&self, value: i64) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt64(&self, value: u64) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateSingle(&self, value: f32) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateDouble(&self, value: f64) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateChar16(&self, value: u16) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateBoolean(&self, value: bool) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInspectable(&self, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateGuid(&self, value: &::windows::core::GUID) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateDateTime(&self, value: &DateTime) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateTimeSpan(&self, value: &TimeSpan) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreatePoint(&self, value: &Point) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateSize(&self, value: &Size) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateRect(&self, value: &Rect) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt8Array(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInt16Array(&self, value: &[<i16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt16Array(&self, value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInt32Array(&self, value: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt32Array(&self, value: &[<u32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInt64Array(&self, value: &[<i64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateUInt64Array(&self, value: &[<u64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateSingleArray(&self, value: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateDoubleArray(&self, value: &[<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateChar16Array(&self, value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateBooleanArray(&self, value: &[<bool as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateStringArray(&self, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateInspectableArray(&self, value: &[<::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateGuidArray(&self, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateDateTimeArray(&self, value: &[<DateTime as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateTimeSpanArray(&self, value: &[<TimeSpan as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreatePointArray(&self, value: &[<Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateSizeArray(&self, value: &[<Size as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateRectArray(&self, value: &[<Rect as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPropertyValueStatics {
    const NAME: &'static str = "Windows.Foundation.IPropertyValueStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPropertyValueStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyValueStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyValueStaticsVtbl {
        unsafe extern "system" fn CreateEmpty<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEmpty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUInt8<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUInt8(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInt16<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInt16(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUInt16<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUInt16(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInt32<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInt32(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUInt32<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUInt32(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInt64<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInt64(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUInt64<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUInt64(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSingle<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSingle(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDouble<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDouble(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateChar16<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateChar16(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBoolean<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBoolean(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateString<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInspectable<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInspectable(&*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGuid<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGuid(&*(&value as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDateTime<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDateTime(&*(&value as *const <DateTime as ::windows::core::Abi>::Abi as *const <DateTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTimeSpan<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTimeSpan(&*(&value as *const <TimeSpan as ::windows::core::Abi>::Abi as *const <TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePoint<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePoint(&*(&value as *const <Point as ::windows::core::Abi>::Abi as *const <Point as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSize<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSize(&*(&value as *const <Size as ::windows::core::Abi>::Abi as *const <Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRect<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRect(&*(&value as *const <Rect as ::windows::core::Abi>::Abi as *const <Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUInt8Array<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u8, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUInt8Array(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInt16Array<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const i16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInt16Array(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUInt16Array<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUInt16Array(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInt32Array<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInt32Array(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUInt32Array<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUInt32Array(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInt64Array<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const i64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInt64Array(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUInt64Array<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUInt64Array(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSingleArray<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const f32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSingleArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDoubleArray<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const f64, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDoubleArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateChar16Array<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const u16, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateChar16Array(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateBooleanArray<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateBooleanArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateStringArray<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateStringArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInspectableArray<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInspectableArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateGuidArray<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const ::windows::core::GUID, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateGuidArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDateTimeArray<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const DateTime, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDateTimeArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTimeSpanArray<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTimeSpanArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePointArray<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePointArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSizeArray<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const Size, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSizeArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRectArray<Impl: IPropertyValueStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value_array_size: u32, value: *const Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRectArray(::core::slice::from_raw_parts(::core::mem::transmute_copy(&value), value_array_size as _)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPropertyValueStatics, BASE_OFFSET>(),
            CreateEmpty: CreateEmpty::<Impl, IMPL_OFFSET>,
            CreateUInt8: CreateUInt8::<Impl, IMPL_OFFSET>,
            CreateInt16: CreateInt16::<Impl, IMPL_OFFSET>,
            CreateUInt16: CreateUInt16::<Impl, IMPL_OFFSET>,
            CreateInt32: CreateInt32::<Impl, IMPL_OFFSET>,
            CreateUInt32: CreateUInt32::<Impl, IMPL_OFFSET>,
            CreateInt64: CreateInt64::<Impl, IMPL_OFFSET>,
            CreateUInt64: CreateUInt64::<Impl, IMPL_OFFSET>,
            CreateSingle: CreateSingle::<Impl, IMPL_OFFSET>,
            CreateDouble: CreateDouble::<Impl, IMPL_OFFSET>,
            CreateChar16: CreateChar16::<Impl, IMPL_OFFSET>,
            CreateBoolean: CreateBoolean::<Impl, IMPL_OFFSET>,
            CreateString: CreateString::<Impl, IMPL_OFFSET>,
            CreateInspectable: CreateInspectable::<Impl, IMPL_OFFSET>,
            CreateGuid: CreateGuid::<Impl, IMPL_OFFSET>,
            CreateDateTime: CreateDateTime::<Impl, IMPL_OFFSET>,
            CreateTimeSpan: CreateTimeSpan::<Impl, IMPL_OFFSET>,
            CreatePoint: CreatePoint::<Impl, IMPL_OFFSET>,
            CreateSize: CreateSize::<Impl, IMPL_OFFSET>,
            CreateRect: CreateRect::<Impl, IMPL_OFFSET>,
            CreateUInt8Array: CreateUInt8Array::<Impl, IMPL_OFFSET>,
            CreateInt16Array: CreateInt16Array::<Impl, IMPL_OFFSET>,
            CreateUInt16Array: CreateUInt16Array::<Impl, IMPL_OFFSET>,
            CreateInt32Array: CreateInt32Array::<Impl, IMPL_OFFSET>,
            CreateUInt32Array: CreateUInt32Array::<Impl, IMPL_OFFSET>,
            CreateInt64Array: CreateInt64Array::<Impl, IMPL_OFFSET>,
            CreateUInt64Array: CreateUInt64Array::<Impl, IMPL_OFFSET>,
            CreateSingleArray: CreateSingleArray::<Impl, IMPL_OFFSET>,
            CreateDoubleArray: CreateDoubleArray::<Impl, IMPL_OFFSET>,
            CreateChar16Array: CreateChar16Array::<Impl, IMPL_OFFSET>,
            CreateBooleanArray: CreateBooleanArray::<Impl, IMPL_OFFSET>,
            CreateStringArray: CreateStringArray::<Impl, IMPL_OFFSET>,
            CreateInspectableArray: CreateInspectableArray::<Impl, IMPL_OFFSET>,
            CreateGuidArray: CreateGuidArray::<Impl, IMPL_OFFSET>,
            CreateDateTimeArray: CreateDateTimeArray::<Impl, IMPL_OFFSET>,
            CreateTimeSpanArray: CreateTimeSpanArray::<Impl, IMPL_OFFSET>,
            CreatePointArray: CreatePointArray::<Impl, IMPL_OFFSET>,
            CreateSizeArray: CreateSizeArray::<Impl, IMPL_OFFSET>,
            CreateRectArray: CreateRectArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyValueStatics as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceImpl<T>: Sized + IPropertyValueImpl
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn Value(&self) -> ::windows::core::Result<T>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IReference<T> {
    const NAME: &'static str = "Windows.Foundation.IReference";
}
impl<T: ::windows::core::RuntimeType + 'static> IReferenceVtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceImpl<T>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceVtbl<T> {
        unsafe extern "system" fn Value<T: ::windows::core::RuntimeType + 'static, Impl: IReferenceImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
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
pub trait IReferenceArrayImpl<T>: Sized + IPropertyValueImpl
where
    T: ::windows::core::RuntimeType + 'static,
{
    fn Value(&self) -> ::windows::core::Result<::windows::core::Array<T>>;
}
impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeName for IReferenceArray<T> {
    const NAME: &'static str = "Windows.Foundation.IReferenceArray";
}
impl<T: ::windows::core::RuntimeType + 'static> IReferenceArrayVtbl<T> {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceArrayImpl<T>, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceArrayVtbl<T> {
        unsafe extern "system" fn Value<T: ::windows::core::RuntimeType + 'static, Impl: IReferenceArrayImpl<T>, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
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
pub trait IStringableImpl: Sized {
    fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IStringable {
    const NAME: &'static str = "Windows.Foundation.IStringable";
}
impl IStringableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStringableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStringableVtbl {
        unsafe extern "system" fn ToString<Impl: IStringableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
#[cfg(feature = "implement_exclusive")]
pub trait IUriEscapeStaticsImpl: Sized {
    fn UnescapeComponent(&self, tounescape: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EscapeComponent(&self, toescape: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUriEscapeStatics {
    const NAME: &'static str = "Windows.Foundation.IUriEscapeStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IUriEscapeStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriEscapeStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUriEscapeStaticsVtbl {
        unsafe extern "system" fn UnescapeComponent<Impl: IUriEscapeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tounescape: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnescapeComponent(&*(&tounescape as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EscapeComponent<Impl: IUriEscapeStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, toescape: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EscapeComponent(&*(&toescape as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUriEscapeStatics, BASE_OFFSET>(),
            UnescapeComponent: UnescapeComponent::<Impl, IMPL_OFFSET>,
            EscapeComponent: EscapeComponent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUriEscapeStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUriRuntimeClassImpl: Sized {
    fn AbsoluteUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Extension(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Fragment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Host(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Password(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Query(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn QueryParsed(&self) -> ::windows::core::Result<WwwFormUrlDecoder>;
    fn RawUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SchemeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Port(&self) -> ::windows::core::Result<i32>;
    fn Suspicious(&self) -> ::windows::core::Result<bool>;
    fn Equals(&self, puri: &::core::option::Option<Uri>) -> ::windows::core::Result<bool>;
    fn CombineUri(&self, relativeuri: &::windows::core::HSTRING) -> ::windows::core::Result<Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUriRuntimeClass {
    const NAME: &'static str = "Windows.Foundation.IUriRuntimeClass";
}
#[cfg(feature = "implement_exclusive")]
impl IUriRuntimeClassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriRuntimeClassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUriRuntimeClassVtbl {
        unsafe extern "system" fn AbsoluteUri<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbsoluteUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayUri<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Domain<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Domain() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Extension<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Extension() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Fragment<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Fragment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Host<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Host() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Password<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Password() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Path<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryParsed<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryParsed() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RawUri<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RawUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SchemeName<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SchemeName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserName<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Port<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Port() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Suspicious<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Suspicious() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Equals<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Equals(&*(&puri as *const <Uri as ::windows::core::Abi>::Abi as *const <Uri as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CombineUri<Impl: IUriRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, relativeuri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CombineUri(&*(&relativeuri as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUriRuntimeClass, BASE_OFFSET>(),
            AbsoluteUri: AbsoluteUri::<Impl, IMPL_OFFSET>,
            DisplayUri: DisplayUri::<Impl, IMPL_OFFSET>,
            Domain: Domain::<Impl, IMPL_OFFSET>,
            Extension: Extension::<Impl, IMPL_OFFSET>,
            Fragment: Fragment::<Impl, IMPL_OFFSET>,
            Host: Host::<Impl, IMPL_OFFSET>,
            Password: Password::<Impl, IMPL_OFFSET>,
            Path: Path::<Impl, IMPL_OFFSET>,
            Query: Query::<Impl, IMPL_OFFSET>,
            QueryParsed: QueryParsed::<Impl, IMPL_OFFSET>,
            RawUri: RawUri::<Impl, IMPL_OFFSET>,
            SchemeName: SchemeName::<Impl, IMPL_OFFSET>,
            UserName: UserName::<Impl, IMPL_OFFSET>,
            Port: Port::<Impl, IMPL_OFFSET>,
            Suspicious: Suspicious::<Impl, IMPL_OFFSET>,
            Equals: Equals::<Impl, IMPL_OFFSET>,
            CombineUri: CombineUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUriRuntimeClass as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUriRuntimeClassFactoryImpl: Sized {
    fn CreateUri(&self, uri: &::windows::core::HSTRING) -> ::windows::core::Result<Uri>;
    fn CreateWithRelativeUri(&self, baseuri: &::windows::core::HSTRING, relativeuri: &::windows::core::HSTRING) -> ::windows::core::Result<Uri>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUriRuntimeClassFactory {
    const NAME: &'static str = "Windows.Foundation.IUriRuntimeClassFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IUriRuntimeClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriRuntimeClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUriRuntimeClassFactoryVtbl {
        unsafe extern "system" fn CreateUri<Impl: IUriRuntimeClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateUri(&*(&uri as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithRelativeUri<Impl: IUriRuntimeClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseuri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, relativeuri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWithRelativeUri(&*(&baseuri as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&relativeuri as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUriRuntimeClassFactory, BASE_OFFSET>(),
            CreateUri: CreateUri::<Impl, IMPL_OFFSET>,
            CreateWithRelativeUri: CreateWithRelativeUri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUriRuntimeClassFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IUriRuntimeClassWithAbsoluteCanonicalUriImpl: Sized {
    fn AbsoluteCanonicalUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayIri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IUriRuntimeClassWithAbsoluteCanonicalUri {
    const NAME: &'static str = "Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri";
}
#[cfg(feature = "implement_exclusive")]
impl IUriRuntimeClassWithAbsoluteCanonicalUriVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriRuntimeClassWithAbsoluteCanonicalUriImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUriRuntimeClassWithAbsoluteCanonicalUriVtbl {
        unsafe extern "system" fn AbsoluteCanonicalUri<Impl: IUriRuntimeClassWithAbsoluteCanonicalUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbsoluteCanonicalUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayIri<Impl: IUriRuntimeClassWithAbsoluteCanonicalUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayIri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUriRuntimeClassWithAbsoluteCanonicalUri, BASE_OFFSET>(),
            AbsoluteCanonicalUri: AbsoluteCanonicalUri::<Impl, IMPL_OFFSET>,
            DisplayIri: DisplayIri::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUriRuntimeClassWithAbsoluteCanonicalUri as ::windows::core::Interface>::IID
    }
}
pub trait IWwwFormUrlDecoderEntryImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IWwwFormUrlDecoderEntry {
    const NAME: &'static str = "Windows.Foundation.IWwwFormUrlDecoderEntry";
}
impl IWwwFormUrlDecoderEntryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWwwFormUrlDecoderEntryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWwwFormUrlDecoderEntryVtbl {
        unsafe extern "system" fn Name<Impl: IWwwFormUrlDecoderEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Value<Impl: IWwwFormUrlDecoderEntryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
pub trait IWwwFormUrlDecoderRuntimeClassImpl: Sized + IIterableImpl<IWwwFormUrlDecoderEntry> + IVectorViewImpl<IWwwFormUrlDecoderEntry> {
    fn GetFirstValueByName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IWwwFormUrlDecoderRuntimeClass {
    const NAME: &'static str = "Windows.Foundation.IWwwFormUrlDecoderRuntimeClass";
}
#[cfg(all(feature = "Foundation_Collections", feature = "implement_exclusive"))]
impl IWwwFormUrlDecoderRuntimeClassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWwwFormUrlDecoderRuntimeClassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWwwFormUrlDecoderRuntimeClassVtbl {
        unsafe extern "system" fn GetFirstValueByName<Impl: IWwwFormUrlDecoderRuntimeClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstValueByName(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWwwFormUrlDecoderRuntimeClass, BASE_OFFSET>(),
            GetFirstValueByName: GetFirstValueByName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWwwFormUrlDecoderRuntimeClass as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IWwwFormUrlDecoderRuntimeClassFactoryImpl: Sized {
    fn CreateWwwFormUrlDecoder(&self, query: &::windows::core::HSTRING) -> ::windows::core::Result<WwwFormUrlDecoder>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IWwwFormUrlDecoderRuntimeClassFactory {
    const NAME: &'static str = "Windows.Foundation.IWwwFormUrlDecoderRuntimeClassFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IWwwFormUrlDecoderRuntimeClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWwwFormUrlDecoderRuntimeClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWwwFormUrlDecoderRuntimeClassFactoryVtbl {
        unsafe extern "system" fn CreateWwwFormUrlDecoder<Impl: IWwwFormUrlDecoderRuntimeClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, query: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateWwwFormUrlDecoder(&*(&query as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IWwwFormUrlDecoderRuntimeClassFactory, BASE_OFFSET>(),
            CreateWwwFormUrlDecoder: CreateWwwFormUrlDecoder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWwwFormUrlDecoderRuntimeClassFactory as ::windows::core::Interface>::IID
    }
}
