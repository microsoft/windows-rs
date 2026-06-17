#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IAsyncOperation<TResult>(windows_core::IUnknown, core::marker::PhantomData<TResult>)
where
    TResult: windows_core::RuntimeType + 'static;
impl<TResult: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IUnknown> for IAsyncOperation<TResult>
{
}
impl<TResult: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_core::IInspectable> for IAsyncOperation<TResult>
{
}
unsafe impl<TResult: windows_core::RuntimeType + 'static> windows_core::Interface
    for IAsyncOperation<TResult>
{
    type Vtable = IAsyncOperation_Vtbl<TResult>;
    const IID: windows_core::GUID =
        windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TResult: windows_core::RuntimeType + 'static> windows_core::RuntimeType
    for IAsyncOperation<TResult>
{
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2}")
        .push_slice(b";")
        .push_other(TResult::SIGNATURE)
        .push_slice(b")");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
        .push_slice(b"Windows.Foundation.IAsyncOperation`1<")
        .push_other(TResult::NAME)
        .push_slice(b">");
}
impl<TResult: windows_core::RuntimeType + 'static>
    windows_core::imp::CanInto<windows_future::IAsyncInfo> for IAsyncOperation<TResult>
{
    const QUERY: bool = true;
}
impl<TResult: windows_core::RuntimeType + 'static> IAsyncOperation<TResult> {
    pub fn SetCompleted<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_future::AsyncOperationCompletedHandler<TResult>>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetCompleted)(
                windows_core::Interface::as_raw(self),
                handler.param().abi(),
            )
            .ok()
        }
    }
    pub fn Completed(
        &self,
    ) -> windows_core::Result<windows_future::AsyncOperationCompletedHandler<TResult>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Completed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResults(&self) -> windows_core::Result<TResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResults)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_future::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<windows_future::AsyncStatus> {
        let this = &windows_core::Interface::cast::<windows_future::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<windows_future::IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_future::IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_future::IAsyncInfo>(self)?;
        unsafe {
            (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this))
                .ok()
        }
    }
}
unsafe impl<TResult: windows_core::RuntimeType + 'static> Send for IAsyncOperation<TResult> {}
unsafe impl<TResult: windows_core::RuntimeType + 'static> Sync for IAsyncOperation<TResult> {}
impl<TResult: windows_core::RuntimeType + 'static> windows_core::RuntimeName
    for IAsyncOperation<TResult>
{
    const NAME: &'static str = "Windows.Foundation.IAsyncOperation";
    const RUNTIME_CLASS_NAME: windows_core::imp::ConstBuffer =
        <Self as windows_core::RuntimeType>::NAME;
}
pub trait IAsyncOperation_Impl<TResult>: windows_future::IAsyncInfo_Impl
where
    TResult: windows_core::RuntimeType + 'static,
{
    fn SetCompleted(
        &self,
        handler: windows_core::Ref<windows_future::AsyncOperationCompletedHandler<TResult>>,
    ) -> windows_core::Result<()>;
    fn Completed(
        &self,
    ) -> windows_core::Result<windows_future::AsyncOperationCompletedHandler<TResult>>;
    fn GetResults(&self) -> windows_core::Result<TResult>;
}
impl<TResult: windows_core::RuntimeType + 'static> IAsyncOperation_Vtbl<TResult> {
    pub const fn new<Identity: IAsyncOperation_Impl<TResult>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCompleted<
            TResult: windows_core::RuntimeType + 'static,
            Identity: IAsyncOperation_Impl<TResult>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            handler: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncOperation_Impl::SetCompleted(this, core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn Completed<
            TResult: windows_core::RuntimeType + 'static,
            Identity: IAsyncOperation_Impl<TResult>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncOperation_Impl::Completed(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetResults<
            TResult: windows_core::RuntimeType + 'static,
            Identity: IAsyncOperation_Impl<TResult>,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            result__: *mut windows_core::AbiType<TResult>,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncOperation_Impl::GetResults(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<
                Identity,
                IAsyncOperation<TResult>,
                OFFSET,
            >(),
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
#[repr(C)]
pub struct IAsyncOperation_Vtbl<TResult>
where
    TResult: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetResults: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::AbiType<TResult>,
    ) -> windows_core::HRESULT,
    TResult: core::marker::PhantomData<TResult>,
}
