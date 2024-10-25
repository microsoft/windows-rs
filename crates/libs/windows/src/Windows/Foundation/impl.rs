pub trait IAsyncAction_Impl: Sized + windows_core::IUnknownImpl + IAsyncInfo_Impl {
    fn SetCompleted(&self, handler: Option<&AsyncActionCompletedHandler>) -> windows_core::Result<()>;
    fn Completed(&self) -> windows_core::Result<AsyncActionCompletedHandler>;
    fn GetResults(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAsyncAction {
    const NAME: &'static str = "Windows.Foundation.IAsyncAction";
}
impl IAsyncAction_Vtbl {
    pub const fn new<Identity: IAsyncAction_Impl, const OFFSET: isize>() -> IAsyncAction_Vtbl {
        unsafe extern "system" fn SetCompleted<Identity: IAsyncAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncAction_Impl::SetCompleted(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn Completed<Identity: IAsyncAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAsyncAction_Impl::Completed(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResults<Identity: IAsyncAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncAction_Impl::GetResults(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAsyncAction, OFFSET>(),
            SetCompleted: SetCompleted::<Identity, OFFSET>,
            Completed: Completed::<Identity, OFFSET>,
            GetResults: GetResults::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncAction as windows_core::Interface>::IID
    }
}
pub trait IAsyncActionWithProgress_Impl<TProgress>: Sized + windows_core::IUnknownImpl + IAsyncInfo_Impl
where
    TProgress: windows_core::RuntimeType + 'static,
{
    fn SetProgress(&self, handler: Option<&AsyncActionProgressHandler<TProgress>>) -> windows_core::Result<()>;
    fn Progress(&self) -> windows_core::Result<AsyncActionProgressHandler<TProgress>>;
    fn SetCompleted(&self, handler: Option<&AsyncActionWithProgressCompletedHandler<TProgress>>) -> windows_core::Result<()>;
    fn Completed(&self) -> windows_core::Result<AsyncActionWithProgressCompletedHandler<TProgress>>;
    fn GetResults(&self) -> windows_core::Result<()>;
}
impl<TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IAsyncActionWithProgress<TProgress> {
    const NAME: &'static str = "Windows.Foundation.IAsyncActionWithProgress";
}
impl<TProgress: windows_core::RuntimeType + 'static> IAsyncActionWithProgress_Vtbl<TProgress> {
    pub const fn new<Identity: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>() -> IAsyncActionWithProgress_Vtbl<TProgress> {
        unsafe extern "system" fn SetProgress<TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncActionWithProgress_Impl::SetProgress(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn Progress<TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAsyncActionWithProgress_Impl::Progress(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompleted<TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncActionWithProgress_Impl::SetCompleted(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn Completed<TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAsyncActionWithProgress_Impl::Completed(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResults<TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncActionWithProgress_Impl::GetResults(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAsyncActionWithProgress<TProgress>, OFFSET>(),
            SetProgress: SetProgress::<TProgress, Identity, OFFSET>,
            Progress: Progress::<TProgress, Identity, OFFSET>,
            SetCompleted: SetCompleted::<TProgress, Identity, OFFSET>,
            Completed: Completed::<TProgress, Identity, OFFSET>,
            GetResults: GetResults::<TProgress, Identity, OFFSET>,
            TProgress: core::marker::PhantomData::<TProgress>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncActionWithProgress<TProgress> as windows_core::Interface>::IID
    }
}
pub trait IAsyncInfo_Impl: Sized + windows_core::IUnknownImpl {
    fn Id(&self) -> windows_core::Result<u32>;
    fn Status(&self) -> windows_core::Result<AsyncStatus>;
    fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IAsyncInfo {
    const NAME: &'static str = "Windows.Foundation.IAsyncInfo";
}
impl IAsyncInfo_Vtbl {
    pub const fn new<Identity: IAsyncInfo_Impl, const OFFSET: isize>() -> IAsyncInfo_Vtbl {
        unsafe extern "system" fn Id<Identity: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAsyncInfo_Impl::Id(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut AsyncStatus) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAsyncInfo_Impl::Status(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ErrorCode<Identity: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAsyncInfo_Impl::ErrorCode(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncInfo_Impl::Cancel(this).into()
        }
        unsafe extern "system" fn Close<Identity: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncInfo_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAsyncInfo, OFFSET>(),
            Id: Id::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            ErrorCode: ErrorCode::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncInfo as windows_core::Interface>::IID
    }
}
pub trait IAsyncOperation_Impl<TResult>: Sized + windows_core::IUnknownImpl + IAsyncInfo_Impl
where
    TResult: windows_core::RuntimeType + 'static,
{
    fn SetCompleted(&self, handler: Option<&AsyncOperationCompletedHandler<TResult>>) -> windows_core::Result<()>;
    fn Completed(&self) -> windows_core::Result<AsyncOperationCompletedHandler<TResult>>;
    fn GetResults(&self) -> windows_core::Result<TResult>;
}
impl<TResult: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IAsyncOperation<TResult> {
    const NAME: &'static str = "Windows.Foundation.IAsyncOperation";
}
impl<TResult: windows_core::RuntimeType + 'static> IAsyncOperation_Vtbl<TResult> {
    pub const fn new<Identity: IAsyncOperation_Impl<TResult>, const OFFSET: isize>() -> IAsyncOperation_Vtbl<TResult> {
        unsafe extern "system" fn SetCompleted<TResult: windows_core::RuntimeType + 'static, Identity: IAsyncOperation_Impl<TResult>, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncOperation_Impl::SetCompleted(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn Completed<TResult: windows_core::RuntimeType + 'static, Identity: IAsyncOperation_Impl<TResult>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAsyncOperation_Impl::Completed(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResults<TResult: windows_core::RuntimeType + 'static, Identity: IAsyncOperation_Impl<TResult>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<TResult>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAsyncOperation_Impl::GetResults(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAsyncOperation<TResult>, OFFSET>(),
            SetCompleted: SetCompleted::<TResult, Identity, OFFSET>,
            Completed: Completed::<TResult, Identity, OFFSET>,
            GetResults: GetResults::<TResult, Identity, OFFSET>,
            TResult: core::marker::PhantomData::<TResult>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncOperation<TResult> as windows_core::Interface>::IID
    }
}
pub trait IAsyncOperationWithProgress_Impl<TResult, TProgress>: Sized + windows_core::IUnknownImpl + IAsyncInfo_Impl
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static,
{
    fn SetProgress(&self, handler: Option<&AsyncOperationProgressHandler<TResult, TProgress>>) -> windows_core::Result<()>;
    fn Progress(&self) -> windows_core::Result<AsyncOperationProgressHandler<TResult, TProgress>>;
    fn SetCompleted(&self, handler: Option<&AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>) -> windows_core::Result<()>;
    fn Completed(&self) -> windows_core::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>;
    fn GetResults(&self) -> windows_core::Result<TResult>;
}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IAsyncOperationWithProgress<TResult, TProgress> {
    const NAME: &'static str = "Windows.Foundation.IAsyncOperationWithProgress";
}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> IAsyncOperationWithProgress_Vtbl<TResult, TProgress> {
    pub const fn new<Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>() -> IAsyncOperationWithProgress_Vtbl<TResult, TProgress> {
        unsafe extern "system" fn SetProgress<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncOperationWithProgress_Impl::SetProgress(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn Progress<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAsyncOperationWithProgress_Impl::Progress(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCompleted<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAsyncOperationWithProgress_Impl::SetCompleted(this, windows_core::from_raw_borrowed(&handler)).into()
        }
        unsafe extern "system" fn Completed<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAsyncOperationWithProgress_Impl::Completed(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResults<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<TResult>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAsyncOperationWithProgress_Impl::GetResults(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAsyncOperationWithProgress<TResult, TProgress>, OFFSET>(),
            SetProgress: SetProgress::<TResult, TProgress, Identity, OFFSET>,
            Progress: Progress::<TResult, TProgress, Identity, OFFSET>,
            SetCompleted: SetCompleted::<TResult, TProgress, Identity, OFFSET>,
            Completed: Completed::<TResult, TProgress, Identity, OFFSET>,
            GetResults: GetResults::<TResult, TProgress, Identity, OFFSET>,
            TResult: core::marker::PhantomData::<TResult>,
            TProgress: core::marker::PhantomData::<TProgress>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAsyncOperationWithProgress<TResult, TProgress> as windows_core::Interface>::IID
    }
}
pub trait IClosable_Impl: Sized + windows_core::IUnknownImpl {
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IClosable {
    const NAME: &'static str = "Windows.Foundation.IClosable";
}
impl IClosable_Vtbl {
    pub const fn new<Identity: IClosable_Impl, const OFFSET: isize>() -> IClosable_Vtbl {
        unsafe extern "system" fn Close<Identity: IClosable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IClosable_Impl::Close(this).into()
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IClosable, OFFSET>(), Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClosable as windows_core::Interface>::IID
    }
}
pub trait IGetActivationFactory_Impl: Sized + windows_core::IUnknownImpl {
    fn GetActivationFactory(&self, activatableclassid: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable>;
}
impl windows_core::RuntimeName for IGetActivationFactory {
    const NAME: &'static str = "Windows.Foundation.IGetActivationFactory";
}
impl IGetActivationFactory_Vtbl {
    pub const fn new<Identity: IGetActivationFactory_Impl, const OFFSET: isize>() -> IGetActivationFactory_Vtbl {
        unsafe extern "system" fn GetActivationFactory<Identity: IGetActivationFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activatableclassid: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IGetActivationFactory_Impl::GetActivationFactory(this, core::mem::transmute(&activatableclassid)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGetActivationFactory, OFFSET>(),
            GetActivationFactory: GetActivationFactory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGetActivationFactory as windows_core::Interface>::IID
    }
}
pub trait IMemoryBuffer_Impl: Sized + windows_core::IUnknownImpl + IClosable_Impl {
    fn CreateReference(&self) -> windows_core::Result<IMemoryBufferReference>;
}
impl windows_core::RuntimeName for IMemoryBuffer {
    const NAME: &'static str = "Windows.Foundation.IMemoryBuffer";
}
impl IMemoryBuffer_Vtbl {
    pub const fn new<Identity: IMemoryBuffer_Impl, const OFFSET: isize>() -> IMemoryBuffer_Vtbl {
        unsafe extern "system" fn CreateReference<Identity: IMemoryBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMemoryBuffer_Impl::CreateReference(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMemoryBuffer, OFFSET>(), CreateReference: CreateReference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMemoryBuffer as windows_core::Interface>::IID
    }
}
pub trait IMemoryBufferReference_Impl: Sized + windows_core::IUnknownImpl + IClosable_Impl {
    fn Capacity(&self) -> windows_core::Result<u32>;
    fn Closed(&self, handler: Option<&TypedEventHandler<IMemoryBufferReference, windows_core::IInspectable>>) -> windows_core::Result<EventRegistrationToken>;
    fn RemoveClosed(&self, cookie: &EventRegistrationToken) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IMemoryBufferReference {
    const NAME: &'static str = "Windows.Foundation.IMemoryBufferReference";
}
impl IMemoryBufferReference_Vtbl {
    pub const fn new<Identity: IMemoryBufferReference_Impl, const OFFSET: isize>() -> IMemoryBufferReference_Vtbl {
        unsafe extern "system" fn Capacity<Identity: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMemoryBufferReference_Impl::Capacity(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Closed<Identity: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMemoryBufferReference_Impl::Closed(this, windows_core::from_raw_borrowed(&handler)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveClosed<Identity: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: EventRegistrationToken) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMemoryBufferReference_Impl::RemoveClosed(this, core::mem::transmute(&cookie)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMemoryBufferReference, OFFSET>(),
            Capacity: Capacity::<Identity, OFFSET>,
            Closed: Closed::<Identity, OFFSET>,
            RemoveClosed: RemoveClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMemoryBufferReference as windows_core::Interface>::IID
    }
}
pub trait IPropertyValue_Impl: Sized + windows_core::IUnknownImpl {
    fn Type(&self) -> windows_core::Result<PropertyType>;
    fn IsNumericScalar(&self) -> windows_core::Result<bool>;
    fn GetUInt8(&self) -> windows_core::Result<u8>;
    fn GetInt16(&self) -> windows_core::Result<i16>;
    fn GetUInt16(&self) -> windows_core::Result<u16>;
    fn GetInt32(&self) -> windows_core::Result<i32>;
    fn GetUInt32(&self) -> windows_core::Result<u32>;
    fn GetInt64(&self) -> windows_core::Result<i64>;
    fn GetUInt64(&self) -> windows_core::Result<u64>;
    fn GetSingle(&self) -> windows_core::Result<f32>;
    fn GetDouble(&self) -> windows_core::Result<f64>;
    fn GetChar16(&self) -> windows_core::Result<u16>;
    fn GetBoolean(&self) -> windows_core::Result<bool>;
    fn GetString(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn GetGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetDateTime(&self) -> windows_core::Result<DateTime>;
    fn GetTimeSpan(&self) -> windows_core::Result<TimeSpan>;
    fn GetPoint(&self) -> windows_core::Result<Point>;
    fn GetSize(&self) -> windows_core::Result<Size>;
    fn GetRect(&self) -> windows_core::Result<Rect>;
    fn GetUInt8Array(&self, value: &mut windows_core::Array<u8>) -> windows_core::Result<()>;
    fn GetInt16Array(&self, value: &mut windows_core::Array<i16>) -> windows_core::Result<()>;
    fn GetUInt16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()>;
    fn GetInt32Array(&self, value: &mut windows_core::Array<i32>) -> windows_core::Result<()>;
    fn GetUInt32Array(&self, value: &mut windows_core::Array<u32>) -> windows_core::Result<()>;
    fn GetInt64Array(&self, value: &mut windows_core::Array<i64>) -> windows_core::Result<()>;
    fn GetUInt64Array(&self, value: &mut windows_core::Array<u64>) -> windows_core::Result<()>;
    fn GetSingleArray(&self, value: &mut windows_core::Array<f32>) -> windows_core::Result<()>;
    fn GetDoubleArray(&self, value: &mut windows_core::Array<f64>) -> windows_core::Result<()>;
    fn GetChar16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()>;
    fn GetBooleanArray(&self, value: &mut windows_core::Array<bool>) -> windows_core::Result<()>;
    fn GetStringArray(&self, value: &mut windows_core::Array<windows_core::HSTRING>) -> windows_core::Result<()>;
    fn GetInspectableArray(&self, value: &mut windows_core::Array<windows_core::IInspectable>) -> windows_core::Result<()>;
    fn GetGuidArray(&self, value: &mut windows_core::Array<windows_core::GUID>) -> windows_core::Result<()>;
    fn GetDateTimeArray(&self, value: &mut windows_core::Array<DateTime>) -> windows_core::Result<()>;
    fn GetTimeSpanArray(&self, value: &mut windows_core::Array<TimeSpan>) -> windows_core::Result<()>;
    fn GetPointArray(&self, value: &mut windows_core::Array<Point>) -> windows_core::Result<()>;
    fn GetSizeArray(&self, value: &mut windows_core::Array<Size>) -> windows_core::Result<()>;
    fn GetRectArray(&self, value: &mut windows_core::Array<Rect>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPropertyValue {
    const NAME: &'static str = "Windows.Foundation.IPropertyValue";
}
impl IPropertyValue_Vtbl {
    pub const fn new<Identity: IPropertyValue_Impl, const OFFSET: isize>() -> IPropertyValue_Vtbl {
        unsafe extern "system" fn Type<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PropertyType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::Type(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNumericScalar<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::IsNumericScalar(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt8<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetUInt8(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInt16<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetInt16(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt16<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetUInt16(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInt32<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetInt32(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt32<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetUInt32(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInt64<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetInt64(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt64<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetUInt64(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSingle<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetSingle(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDouble<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetDouble(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChar16<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetChar16(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBoolean<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetBoolean(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetString<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetString(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGuid<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetGuid(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDateTime<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut DateTime) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetDateTime(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeSpan<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut TimeSpan) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetTimeSpan(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPoint<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Point) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetPoint(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSize<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Size) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetSize(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRect<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Rect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertyValue_Impl::GetRect(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUInt8Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetUInt8Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInt16Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetInt16Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetUInt16Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetUInt16Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInt32Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetInt32Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetUInt32Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetUInt32Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInt64Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetInt64Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetUInt64Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetUInt64Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetSingleArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetSingleArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetDoubleArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetDoubleArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetChar16Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetChar16Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetBooleanArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut bool) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetBooleanArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetStringArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetStringArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetInspectableArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetInspectableArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetGuidArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetGuidArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetDateTimeArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut DateTime) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetDateTimeArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetTimeSpanArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut TimeSpan) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetTimeSpanArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetPointArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Point) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetPointArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetSizeArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Size) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetSizeArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        unsafe extern "system" fn GetRectArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Rect) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertyValue_Impl::GetRectArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPropertyValue, OFFSET>(),
            Type: Type::<Identity, OFFSET>,
            IsNumericScalar: IsNumericScalar::<Identity, OFFSET>,
            GetUInt8: GetUInt8::<Identity, OFFSET>,
            GetInt16: GetInt16::<Identity, OFFSET>,
            GetUInt16: GetUInt16::<Identity, OFFSET>,
            GetInt32: GetInt32::<Identity, OFFSET>,
            GetUInt32: GetUInt32::<Identity, OFFSET>,
            GetInt64: GetInt64::<Identity, OFFSET>,
            GetUInt64: GetUInt64::<Identity, OFFSET>,
            GetSingle: GetSingle::<Identity, OFFSET>,
            GetDouble: GetDouble::<Identity, OFFSET>,
            GetChar16: GetChar16::<Identity, OFFSET>,
            GetBoolean: GetBoolean::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetGuid: GetGuid::<Identity, OFFSET>,
            GetDateTime: GetDateTime::<Identity, OFFSET>,
            GetTimeSpan: GetTimeSpan::<Identity, OFFSET>,
            GetPoint: GetPoint::<Identity, OFFSET>,
            GetSize: GetSize::<Identity, OFFSET>,
            GetRect: GetRect::<Identity, OFFSET>,
            GetUInt8Array: GetUInt8Array::<Identity, OFFSET>,
            GetInt16Array: GetInt16Array::<Identity, OFFSET>,
            GetUInt16Array: GetUInt16Array::<Identity, OFFSET>,
            GetInt32Array: GetInt32Array::<Identity, OFFSET>,
            GetUInt32Array: GetUInt32Array::<Identity, OFFSET>,
            GetInt64Array: GetInt64Array::<Identity, OFFSET>,
            GetUInt64Array: GetUInt64Array::<Identity, OFFSET>,
            GetSingleArray: GetSingleArray::<Identity, OFFSET>,
            GetDoubleArray: GetDoubleArray::<Identity, OFFSET>,
            GetChar16Array: GetChar16Array::<Identity, OFFSET>,
            GetBooleanArray: GetBooleanArray::<Identity, OFFSET>,
            GetStringArray: GetStringArray::<Identity, OFFSET>,
            GetInspectableArray: GetInspectableArray::<Identity, OFFSET>,
            GetGuidArray: GetGuidArray::<Identity, OFFSET>,
            GetDateTimeArray: GetDateTimeArray::<Identity, OFFSET>,
            GetTimeSpanArray: GetTimeSpanArray::<Identity, OFFSET>,
            GetPointArray: GetPointArray::<Identity, OFFSET>,
            GetSizeArray: GetSizeArray::<Identity, OFFSET>,
            GetRectArray: GetRectArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertyValue as windows_core::Interface>::IID
    }
}
pub trait IReference_Impl<T>: Sized + windows_core::IUnknownImpl + IPropertyValue_Impl
where
    T: windows_core::RuntimeType + 'static,
{
    fn Value(&self) -> windows_core::Result<T>;
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IReference<T> {
    const NAME: &'static str = "Windows.Foundation.IReference";
}
impl<T: windows_core::RuntimeType + 'static> IReference_Vtbl<T> {
    pub const fn new<Identity: IReference_Impl<T>, const OFFSET: isize>() -> IReference_Vtbl<T> {
        unsafe extern "system" fn Value<T: windows_core::RuntimeType + 'static, Identity: IReference_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<T>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IReference_Impl::Value(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IReference<T>, OFFSET>(),
            Value: Value::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReference<T> as windows_core::Interface>::IID
    }
}
pub trait IReferenceArray_Impl<T>: Sized + windows_core::IUnknownImpl + IPropertyValue_Impl
where
    T: windows_core::RuntimeType + 'static,
{
    fn Value(&self) -> windows_core::Result<windows_core::Array<T>>;
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IReferenceArray<T> {
    const NAME: &'static str = "Windows.Foundation.IReferenceArray";
}
impl<T: windows_core::RuntimeType + 'static> IReferenceArray_Vtbl<T> {
    pub const fn new<Identity: IReferenceArray_Impl<T>, const OFFSET: isize>() -> IReferenceArray_Vtbl<T> {
        unsafe extern "system" fn Value<T: windows_core::RuntimeType + 'static, Identity: IReferenceArray_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut windows_core::AbiType<T>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IReferenceArray_Impl::Value(this) {
                Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    result__.write(ok_data__);
                    result_size__.write(ok_data_len__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IReferenceArray<T>, OFFSET>(),
            Value: Value::<T, Identity, OFFSET>,
            T: core::marker::PhantomData::<T>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceArray<T> as windows_core::Interface>::IID
    }
}
pub trait IStringable_Impl: Sized + windows_core::IUnknownImpl {
    fn ToString(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IStringable {
    const NAME: &'static str = "Windows.Foundation.IStringable";
}
impl IStringable_Vtbl {
    pub const fn new<Identity: IStringable_Impl, const OFFSET: isize>() -> IStringable_Vtbl {
        unsafe extern "system" fn ToString<Identity: IStringable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStringable_Impl::ToString(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IStringable, OFFSET>(), ToString: ToString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStringable as windows_core::Interface>::IID
    }
}
pub trait IWwwFormUrlDecoderEntry_Impl: Sized + windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Value(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IWwwFormUrlDecoderEntry {
    const NAME: &'static str = "Windows.Foundation.IWwwFormUrlDecoderEntry";
}
impl IWwwFormUrlDecoderEntry_Vtbl {
    pub const fn new<Identity: IWwwFormUrlDecoderEntry_Impl, const OFFSET: isize>() -> IWwwFormUrlDecoderEntry_Vtbl {
        unsafe extern "system" fn Name<Identity: IWwwFormUrlDecoderEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWwwFormUrlDecoderEntry_Impl::Name(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: IWwwFormUrlDecoderEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWwwFormUrlDecoderEntry_Impl::Value(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWwwFormUrlDecoderEntry, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWwwFormUrlDecoderEntry as windows_core::Interface>::IID
    }
}
