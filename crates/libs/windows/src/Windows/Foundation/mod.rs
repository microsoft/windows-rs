#[cfg(feature = "Foundation_Collections")]
pub mod Collections;
#[cfg(feature = "Foundation_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "Foundation_Metadata")]
pub mod Metadata;
#[cfg(feature = "Foundation_Numerics")]
pub mod Numerics;
windows_core::imp::define_interface!(AsyncActionCompletedHandler, AsyncActionCompletedHandler_Vtbl, 0xa4ed5c81_76c9_40bd_8be6_b1d90fb20ae7);
impl windows_core::RuntimeType for AsyncActionCompletedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl AsyncActionCompletedHandler {
    pub fn new<F: FnMut(windows_core::Ref<'_, IAsyncAction>, AsyncStatus) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = AsyncActionCompletedHandlerBox { vtable: &AsyncActionCompletedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, asyncinfo: P0, asyncstatus: AsyncStatus) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncAction>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), asyncinfo.param().abi(), asyncstatus).ok() }
    }
}
#[repr(C)]
pub struct AsyncActionCompletedHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, asyncinfo: *mut core::ffi::c_void, asyncstatus: AsyncStatus) -> windows_core::HRESULT,
}
#[repr(C)]
struct AsyncActionCompletedHandlerBox<F: FnMut(windows_core::Ref<'_, IAsyncAction>, AsyncStatus) -> windows_core::Result<()> + Send + 'static> {
    vtable: *const AsyncActionCompletedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut(windows_core::Ref<'_, IAsyncAction>, AsyncStatus) -> windows_core::Result<()> + Send + 'static> AsyncActionCompletedHandlerBox<F> {
    const VTABLE: AsyncActionCompletedHandler_Vtbl = AsyncActionCompletedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <AsyncActionCompletedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, asyncinfo: *mut core::ffi::c_void, asyncstatus: AsyncStatus) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&asyncinfo), asyncstatus).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncActionProgressHandler<TProgress>(windows_core::IUnknown, core::marker::PhantomData<TProgress>)
where
    TProgress: windows_core::RuntimeType + 'static;
unsafe impl<TProgress: windows_core::RuntimeType + 'static> windows_core::Interface for AsyncActionProgressHandler<TProgress> {
    type Vtable = AsyncActionProgressHandler_Vtbl<TProgress>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeType for AsyncActionProgressHandler<TProgress> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({6d844858-0cff-4590-ae89-95a5a5c8b4b8}").push_slice(b";").push_other(TProgress::SIGNATURE).push_slice(b")");
}
impl<TProgress: windows_core::RuntimeType + 'static> AsyncActionProgressHandler<TProgress> {
    pub fn new<F: FnMut(windows_core::Ref<'_, IAsyncActionWithProgress<TProgress>>, windows_core::Ref<'_, TProgress>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = AsyncActionProgressHandlerBox { vtable: &AsyncActionProgressHandlerBox::<TProgress, F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, asyncinfo: P0, progressinfo: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncActionWithProgress<TProgress>>,
        P1: windows_core::Param<TProgress>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), asyncinfo.param().abi(), progressinfo.param().abi()).ok() }
    }
}
#[repr(C)]
pub struct AsyncActionProgressHandler_Vtbl<TProgress>
where
    TProgress: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, asyncinfo: *mut core::ffi::c_void, progressinfo: windows_core::AbiType<TProgress>) -> windows_core::HRESULT,
    TProgress: core::marker::PhantomData<TProgress>,
}
#[repr(C)]
struct AsyncActionProgressHandlerBox<TProgress, F: FnMut(windows_core::Ref<'_, IAsyncActionWithProgress<TProgress>>, windows_core::Ref<'_, TProgress>) -> windows_core::Result<()> + Send + 'static>
where
    TProgress: windows_core::RuntimeType + 'static,
{
    vtable: *const AsyncActionProgressHandler_Vtbl<TProgress>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<TProgress: windows_core::RuntimeType + 'static, F: FnMut(windows_core::Ref<'_, IAsyncActionWithProgress<TProgress>>, windows_core::Ref<'_, TProgress>) -> windows_core::Result<()> + Send + 'static> AsyncActionProgressHandlerBox<TProgress, F> {
    const VTABLE: AsyncActionProgressHandler_Vtbl<TProgress> = AsyncActionProgressHandler_Vtbl::<TProgress> {
        base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        TProgress: core::marker::PhantomData::<TProgress>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <AsyncActionProgressHandler<TProgress> as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, asyncinfo: *mut core::ffi::c_void, progressinfo: windows_core::AbiType<TProgress>) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&asyncinfo), core::mem::transmute_copy(&progressinfo)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncActionWithProgressCompletedHandler<TProgress>(windows_core::IUnknown, core::marker::PhantomData<TProgress>)
where
    TProgress: windows_core::RuntimeType + 'static;
unsafe impl<TProgress: windows_core::RuntimeType + 'static> windows_core::Interface for AsyncActionWithProgressCompletedHandler<TProgress> {
    type Vtable = AsyncActionWithProgressCompletedHandler_Vtbl<TProgress>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeType for AsyncActionWithProgressCompletedHandler<TProgress> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({9c029f91-cc84-44fd-ac26-0a6c4e555281}").push_slice(b";").push_other(TProgress::SIGNATURE).push_slice(b")");
}
impl<TProgress: windows_core::RuntimeType + 'static> AsyncActionWithProgressCompletedHandler<TProgress> {
    pub fn new<F: FnMut(windows_core::Ref<'_, IAsyncActionWithProgress<TProgress>>, AsyncStatus) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = AsyncActionWithProgressCompletedHandlerBox { vtable: &AsyncActionWithProgressCompletedHandlerBox::<TProgress, F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, asyncinfo: P0, asyncstatus: AsyncStatus) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncActionWithProgress<TProgress>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), asyncinfo.param().abi(), asyncstatus).ok() }
    }
}
#[repr(C)]
pub struct AsyncActionWithProgressCompletedHandler_Vtbl<TProgress>
where
    TProgress: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, asyncinfo: *mut core::ffi::c_void, asyncstatus: AsyncStatus) -> windows_core::HRESULT,
    TProgress: core::marker::PhantomData<TProgress>,
}
#[repr(C)]
struct AsyncActionWithProgressCompletedHandlerBox<TProgress, F: FnMut(windows_core::Ref<'_, IAsyncActionWithProgress<TProgress>>, AsyncStatus) -> windows_core::Result<()> + Send + 'static>
where
    TProgress: windows_core::RuntimeType + 'static,
{
    vtable: *const AsyncActionWithProgressCompletedHandler_Vtbl<TProgress>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<TProgress: windows_core::RuntimeType + 'static, F: FnMut(windows_core::Ref<'_, IAsyncActionWithProgress<TProgress>>, AsyncStatus) -> windows_core::Result<()> + Send + 'static> AsyncActionWithProgressCompletedHandlerBox<TProgress, F> {
    const VTABLE: AsyncActionWithProgressCompletedHandler_Vtbl<TProgress> = AsyncActionWithProgressCompletedHandler_Vtbl::<TProgress> {
        base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        TProgress: core::marker::PhantomData::<TProgress>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <AsyncActionWithProgressCompletedHandler<TProgress> as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, asyncinfo: *mut core::ffi::c_void, asyncstatus: AsyncStatus) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&asyncinfo), asyncstatus).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncOperationCompletedHandler<TResult>(windows_core::IUnknown, core::marker::PhantomData<TResult>)
where
    TResult: windows_core::RuntimeType + 'static;
unsafe impl<TResult: windows_core::RuntimeType + 'static> windows_core::Interface for AsyncOperationCompletedHandler<TResult> {
    type Vtable = AsyncOperationCompletedHandler_Vtbl<TResult>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TResult: windows_core::RuntimeType + 'static> windows_core::RuntimeType for AsyncOperationCompletedHandler<TResult> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({fcdcf02c-e5d8-4478-915a-4d90b74b83a5}").push_slice(b";").push_other(TResult::SIGNATURE).push_slice(b")");
}
impl<TResult: windows_core::RuntimeType + 'static> AsyncOperationCompletedHandler<TResult> {
    pub fn new<F: FnMut(windows_core::Ref<'_, IAsyncOperation<TResult>>, AsyncStatus) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = AsyncOperationCompletedHandlerBox { vtable: &AsyncOperationCompletedHandlerBox::<TResult, F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, asyncinfo: P0, asyncstatus: AsyncStatus) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncOperation<TResult>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), asyncinfo.param().abi(), asyncstatus).ok() }
    }
}
#[repr(C)]
pub struct AsyncOperationCompletedHandler_Vtbl<TResult>
where
    TResult: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, asyncinfo: *mut core::ffi::c_void, asyncstatus: AsyncStatus) -> windows_core::HRESULT,
    TResult: core::marker::PhantomData<TResult>,
}
#[repr(C)]
struct AsyncOperationCompletedHandlerBox<TResult, F: FnMut(windows_core::Ref<'_, IAsyncOperation<TResult>>, AsyncStatus) -> windows_core::Result<()> + Send + 'static>
where
    TResult: windows_core::RuntimeType + 'static,
{
    vtable: *const AsyncOperationCompletedHandler_Vtbl<TResult>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<TResult: windows_core::RuntimeType + 'static, F: FnMut(windows_core::Ref<'_, IAsyncOperation<TResult>>, AsyncStatus) -> windows_core::Result<()> + Send + 'static> AsyncOperationCompletedHandlerBox<TResult, F> {
    const VTABLE: AsyncOperationCompletedHandler_Vtbl<TResult> = AsyncOperationCompletedHandler_Vtbl::<TResult> {
        base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        TResult: core::marker::PhantomData::<TResult>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <AsyncOperationCompletedHandler<TResult> as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, asyncinfo: *mut core::ffi::c_void, asyncstatus: AsyncStatus) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&asyncinfo), asyncstatus).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncOperationProgressHandler<TResult, TProgress>(windows_core::IUnknown, core::marker::PhantomData<TResult>, core::marker::PhantomData<TProgress>)
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static;
unsafe impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> windows_core::Interface for AsyncOperationProgressHandler<TResult, TProgress> {
    type Vtable = AsyncOperationProgressHandler_Vtbl<TResult, TProgress>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeType for AsyncOperationProgressHandler<TResult, TProgress> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({55690902-0aab-421a-8778-f8ce5026d758}").push_slice(b";").push_other(TResult::SIGNATURE).push_slice(b";").push_other(TProgress::SIGNATURE).push_slice(b")");
}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> AsyncOperationProgressHandler<TResult, TProgress> {
    pub fn new<F: FnMut(windows_core::Ref<'_, IAsyncOperationWithProgress<TResult, TProgress>>, windows_core::Ref<'_, TProgress>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = AsyncOperationProgressHandlerBox { vtable: &AsyncOperationProgressHandlerBox::<TResult, TProgress, F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, asyncinfo: P0, progressinfo: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncOperationWithProgress<TResult, TProgress>>,
        P1: windows_core::Param<TProgress>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), asyncinfo.param().abi(), progressinfo.param().abi()).ok() }
    }
}
#[repr(C)]
pub struct AsyncOperationProgressHandler_Vtbl<TResult, TProgress>
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, asyncinfo: *mut core::ffi::c_void, progressinfo: windows_core::AbiType<TProgress>) -> windows_core::HRESULT,
    TResult: core::marker::PhantomData<TResult>,
    TProgress: core::marker::PhantomData<TProgress>,
}
#[repr(C)]
struct AsyncOperationProgressHandlerBox<TResult, TProgress, F: FnMut(windows_core::Ref<'_, IAsyncOperationWithProgress<TResult, TProgress>>, windows_core::Ref<'_, TProgress>) -> windows_core::Result<()> + Send + 'static>
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static,
{
    vtable: *const AsyncOperationProgressHandler_Vtbl<TResult, TProgress>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static, F: FnMut(windows_core::Ref<'_, IAsyncOperationWithProgress<TResult, TProgress>>, windows_core::Ref<'_, TProgress>) -> windows_core::Result<()> + Send + 'static> AsyncOperationProgressHandlerBox<TResult, TProgress, F> {
    const VTABLE: AsyncOperationProgressHandler_Vtbl<TResult, TProgress> = AsyncOperationProgressHandler_Vtbl::<TResult, TProgress> {
        base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        TResult: core::marker::PhantomData::<TResult>,
        TProgress: core::marker::PhantomData::<TProgress>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <AsyncOperationProgressHandler<TResult, TProgress> as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, asyncinfo: *mut core::ffi::c_void, progressinfo: windows_core::AbiType<TProgress>) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&asyncinfo), core::mem::transmute_copy(&progressinfo)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AsyncOperationWithProgressCompletedHandler<TResult, TProgress>(windows_core::IUnknown, core::marker::PhantomData<TResult>, core::marker::PhantomData<TProgress>)
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static;
unsafe impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> windows_core::Interface for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    type Vtable = AsyncOperationWithProgressCompletedHandler_Vtbl<TResult, TProgress>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeType for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({e85df41d-6aa7-46e3-a8e2-f009d840c627}").push_slice(b";").push_other(TResult::SIGNATURE).push_slice(b";").push_other(TProgress::SIGNATURE).push_slice(b")");
}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    pub fn new<F: FnMut(windows_core::Ref<'_, IAsyncOperationWithProgress<TResult, TProgress>>, AsyncStatus) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = AsyncOperationWithProgressCompletedHandlerBox { vtable: &AsyncOperationWithProgressCompletedHandlerBox::<TResult, TProgress, F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, asyncinfo: P0, asyncstatus: AsyncStatus) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IAsyncOperationWithProgress<TResult, TProgress>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), asyncinfo.param().abi(), asyncstatus).ok() }
    }
}
#[repr(C)]
pub struct AsyncOperationWithProgressCompletedHandler_Vtbl<TResult, TProgress>
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, asyncinfo: *mut core::ffi::c_void, asyncstatus: AsyncStatus) -> windows_core::HRESULT,
    TResult: core::marker::PhantomData<TResult>,
    TProgress: core::marker::PhantomData<TProgress>,
}
#[repr(C)]
struct AsyncOperationWithProgressCompletedHandlerBox<TResult, TProgress, F: FnMut(windows_core::Ref<'_, IAsyncOperationWithProgress<TResult, TProgress>>, AsyncStatus) -> windows_core::Result<()> + Send + 'static>
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static,
{
    vtable: *const AsyncOperationWithProgressCompletedHandler_Vtbl<TResult, TProgress>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static, F: FnMut(windows_core::Ref<'_, IAsyncOperationWithProgress<TResult, TProgress>>, AsyncStatus) -> windows_core::Result<()> + Send + 'static> AsyncOperationWithProgressCompletedHandlerBox<TResult, TProgress, F> {
    const VTABLE: AsyncOperationWithProgressCompletedHandler_Vtbl<TResult, TProgress> = AsyncOperationWithProgressCompletedHandler_Vtbl::<TResult, TProgress> {
        base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        TResult: core::marker::PhantomData::<TResult>,
        TProgress: core::marker::PhantomData::<TProgress>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <AsyncOperationWithProgressCompletedHandler<TResult, TProgress> as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, asyncinfo: *mut core::ffi::c_void, asyncstatus: AsyncStatus) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&asyncinfo), asyncstatus).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: Self = Self(2i32);
    pub const Completed: Self = Self(1i32);
    pub const Error: Self = Self(3i32);
    pub const Started: Self = Self(0i32);
}
impl windows_core::TypeKind for AsyncStatus {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AsyncStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.AsyncStatus;i4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DateTime {
    pub UniversalTime: i64,
}
impl windows_core::TypeKind for DateTime {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DateTime {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.DateTime;i8)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Deferral, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Deferral, IClosable);
impl Deferral {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Complete(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Complete)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Create<P0>(handler: P0) -> windows_core::Result<Deferral>
    where
        P0: windows_core::Param<DeferralCompletedHandler>,
    {
        Self::IDeferralFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IDeferralFactory<R, F: FnOnce(&IDeferralFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Deferral, IDeferralFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Deferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeferral>();
}
unsafe impl windows_core::Interface for Deferral {
    type Vtable = <IDeferral as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Deferral {
    const NAME: &'static str = "Windows.Foundation.Deferral";
}
unsafe impl Send for Deferral {}
unsafe impl Sync for Deferral {}
windows_core::imp::define_interface!(DeferralCompletedHandler, DeferralCompletedHandler_Vtbl, 0xed32a372_f3c8_4faa_9cfb_470148da3888);
impl windows_core::RuntimeType for DeferralCompletedHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl DeferralCompletedHandler {
    pub fn new<F: FnMut() -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = DeferralCompletedHandlerBox { vtable: &DeferralCompletedHandlerBox::<F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this)).ok() }
    }
}
#[repr(C)]
pub struct DeferralCompletedHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(C)]
struct DeferralCompletedHandlerBox<F: FnMut() -> windows_core::Result<()> + Send + 'static> {
    vtable: *const DeferralCompletedHandler_Vtbl,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<F: FnMut() -> windows_core::Result<()> + Send + 'static> DeferralCompletedHandlerBox<F> {
    const VTABLE: DeferralCompletedHandler_Vtbl = DeferralCompletedHandler_Vtbl { base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <DeferralCompletedHandler as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)().into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventHandler<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for EventHandler<T> {
    type Vtable = EventHandler_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for EventHandler<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({9de1c535-6ae1-11e0-84e1-18a905bcc53f}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> EventHandler<T> {
    pub fn new<F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, T>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = EventHandlerBox { vtable: &EventHandlerBox::<T, F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<T>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), args.param().abi()).ok() }
    }
}
#[repr(C)]
pub struct EventHandler_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: windows_core::AbiType<T>) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
#[repr(C)]
struct EventHandlerBox<T, F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, T>) -> windows_core::Result<()> + Send + 'static>
where
    T: windows_core::RuntimeType + 'static,
{
    vtable: *const EventHandler_Vtbl<T>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<T: windows_core::RuntimeType + 'static, F: FnMut(windows_core::Ref<'_, windows_core::IInspectable>, windows_core::Ref<'_, T>) -> windows_core::Result<()> + Send + 'static> EventHandlerBox<T, F> {
    const VTABLE: EventHandler_Vtbl<T> = EventHandler_Vtbl::<T> {
        base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        T: core::marker::PhantomData::<T>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <EventHandler<T> as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: windows_core::AbiType<T>) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&args)).into()
        }
    }
}
pub struct GuidHelper;
impl GuidHelper {
    pub fn CreateNewGuid() -> windows_core::Result<windows_core::GUID> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateNewGuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Empty() -> windows_core::Result<windows_core::GUID> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Empty)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn Equals(target: windows_core::GUID, value: windows_core::GUID) -> windows_core::Result<bool> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Equals)(windows_core::Interface::as_raw(this), &target, &value, &mut result__).map(|| result__)
        })
    }
    fn IGuidHelperStatics<R, F: FnOnce(&IGuidHelperStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GuidHelper, IGuidHelperStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for GuidHelper {
    const NAME: &'static str = "Windows.Foundation.GuidHelper";
}
windows_core::imp::define_interface!(IAsyncAction, IAsyncAction_Vtbl, 0x5a648006_843a_4da9_865b_9d26e5dfad7b);
impl windows_core::RuntimeType for IAsyncAction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IAsyncAction, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IAsyncAction, IAsyncInfo);
impl IAsyncAction {
    pub fn SetCompleted<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AsyncActionCompletedHandler>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCompleted)(windows_core::Interface::as_raw(this), handler.param().abi()).ok() }
    }
    pub fn Completed(&self) -> windows_core::Result<AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResults(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetResults)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<AsyncStatus> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
unsafe impl Send for IAsyncAction {}
unsafe impl Sync for IAsyncAction {}
impl windows_core::RuntimeName for IAsyncAction {
    const NAME: &'static str = "Windows.Foundation.IAsyncAction";
}
pub trait IAsyncAction_Impl: IAsyncInfo_Impl {
    fn SetCompleted(&self, handler: windows_core::Ref<'_, AsyncActionCompletedHandler>) -> windows_core::Result<()>;
    fn Completed(&self) -> windows_core::Result<AsyncActionCompletedHandler>;
    fn GetResults(&self) -> windows_core::Result<()>;
}
impl IAsyncAction_Vtbl {
    pub const fn new<Identity: IAsyncAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCompleted<Identity: IAsyncAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncAction_Impl::SetCompleted(this, core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn Completed<Identity: IAsyncAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn GetResults<Identity: IAsyncAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncAction_Impl::GetResults(this).into()
            }
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
#[repr(C)]
pub struct IAsyncAction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IAsyncActionWithProgress<TProgress>(windows_core::IUnknown, core::marker::PhantomData<TProgress>)
where
    TProgress: windows_core::RuntimeType + 'static;
impl<TProgress: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IAsyncActionWithProgress<TProgress> {}
impl<TProgress: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IAsyncActionWithProgress<TProgress> {}
unsafe impl<TProgress: windows_core::RuntimeType + 'static> windows_core::Interface for IAsyncActionWithProgress<TProgress> {
    type Vtable = IAsyncActionWithProgress_Vtbl<TProgress>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IAsyncActionWithProgress<TProgress> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({1f6db258-e803-48a1-9546-eb7353398884}").push_slice(b";").push_other(TProgress::SIGNATURE).push_slice(b")");
}
impl<TProgress: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IAsyncInfo> for IAsyncActionWithProgress<TProgress> {
    const QUERY: bool = true;
}
impl<TProgress: windows_core::RuntimeType + 'static> IAsyncActionWithProgress<TProgress> {
    pub fn SetProgress<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AsyncActionProgressHandler<TProgress>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProgress)(windows_core::Interface::as_raw(this), handler.param().abi()).ok() }
    }
    pub fn Progress(&self) -> windows_core::Result<AsyncActionProgressHandler<TProgress>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompleted<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AsyncActionWithProgressCompletedHandler<TProgress>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCompleted)(windows_core::Interface::as_raw(this), handler.param().abi()).ok() }
    }
    pub fn Completed(&self) -> windows_core::Result<AsyncActionWithProgressCompletedHandler<TProgress>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResults(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetResults)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<AsyncStatus> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
unsafe impl<TProgress: windows_core::RuntimeType + 'static> Send for IAsyncActionWithProgress<TProgress> {}
unsafe impl<TProgress: windows_core::RuntimeType + 'static> Sync for IAsyncActionWithProgress<TProgress> {}
impl<TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IAsyncActionWithProgress<TProgress> {
    const NAME: &'static str = "Windows.Foundation.IAsyncActionWithProgress";
}
pub trait IAsyncActionWithProgress_Impl<TProgress>: IAsyncInfo_Impl
where
    TProgress: windows_core::RuntimeType + 'static,
{
    fn SetProgress(&self, handler: windows_core::Ref<'_, AsyncActionProgressHandler<TProgress>>) -> windows_core::Result<()>;
    fn Progress(&self) -> windows_core::Result<AsyncActionProgressHandler<TProgress>>;
    fn SetCompleted(&self, handler: windows_core::Ref<'_, AsyncActionWithProgressCompletedHandler<TProgress>>) -> windows_core::Result<()>;
    fn Completed(&self) -> windows_core::Result<AsyncActionWithProgressCompletedHandler<TProgress>>;
    fn GetResults(&self) -> windows_core::Result<()>;
}
impl<TProgress: windows_core::RuntimeType + 'static> IAsyncActionWithProgress_Vtbl<TProgress> {
    pub const fn new<Identity: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProgress<TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncActionWithProgress_Impl::SetProgress(this, core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn Progress<TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn SetCompleted<TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncActionWithProgress_Impl::SetCompleted(this, core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn Completed<TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn GetResults<TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncActionWithProgress_Impl<TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncActionWithProgress_Impl::GetResults(this).into()
            }
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
#[repr(C)]
pub struct IAsyncActionWithProgress_Vtbl<TProgress>
where
    TProgress: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    TProgress: core::marker::PhantomData<TProgress>,
}
windows_core::imp::define_interface!(IAsyncInfo, IAsyncInfo_Vtbl, 0x00000036_0000_0000_c000_000000000046);
impl windows_core::RuntimeType for IAsyncInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IAsyncInfo, windows_core::IUnknown, windows_core::IInspectable);
impl IAsyncInfo {
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<AsyncStatus> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for IAsyncInfo {
    const NAME: &'static str = "Windows.Foundation.IAsyncInfo";
}
pub trait IAsyncInfo_Impl: windows_core::IUnknownImpl {
    fn Id(&self) -> windows_core::Result<u32>;
    fn Status(&self) -> windows_core::Result<AsyncStatus>;
    fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn Cancel(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl IAsyncInfo_Vtbl {
    pub const fn new<Identity: IAsyncInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Id<Identity: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncInfo_Impl::Id(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Status<Identity: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut AsyncStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncInfo_Impl::Status(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ErrorCode<Identity: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAsyncInfo_Impl::ErrorCode(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cancel<Identity: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncInfo_Impl::Cancel(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IAsyncInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncInfo_Impl::Close(this).into()
            }
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
#[repr(C)]
pub struct IAsyncInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Status: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AsyncStatus) -> windows_core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IAsyncOperation<TResult>(windows_core::IUnknown, core::marker::PhantomData<TResult>)
where
    TResult: windows_core::RuntimeType + 'static;
impl<TResult: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IAsyncOperation<TResult> {}
impl<TResult: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IAsyncOperation<TResult> {}
unsafe impl<TResult: windows_core::RuntimeType + 'static> windows_core::Interface for IAsyncOperation<TResult> {
    type Vtable = IAsyncOperation_Vtbl<TResult>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TResult: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IAsyncOperation<TResult> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({9fc2b0bb-e446-44e2-aa61-9cab8f636af2}").push_slice(b";").push_other(TResult::SIGNATURE).push_slice(b")");
}
impl<TResult: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IAsyncInfo> for IAsyncOperation<TResult> {
    const QUERY: bool = true;
}
impl<TResult: windows_core::RuntimeType + 'static> IAsyncOperation<TResult> {
    pub fn SetCompleted<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AsyncOperationCompletedHandler<TResult>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCompleted)(windows_core::Interface::as_raw(this), handler.param().abi()).ok() }
    }
    pub fn Completed(&self) -> windows_core::Result<AsyncOperationCompletedHandler<TResult>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResults(&self) -> windows_core::Result<TResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetResults)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<AsyncStatus> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
unsafe impl<TResult: windows_core::RuntimeType + 'static> Send for IAsyncOperation<TResult> {}
unsafe impl<TResult: windows_core::RuntimeType + 'static> Sync for IAsyncOperation<TResult> {}
impl<TResult: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IAsyncOperation<TResult> {
    const NAME: &'static str = "Windows.Foundation.IAsyncOperation";
}
pub trait IAsyncOperation_Impl<TResult>: IAsyncInfo_Impl
where
    TResult: windows_core::RuntimeType + 'static,
{
    fn SetCompleted(&self, handler: windows_core::Ref<'_, AsyncOperationCompletedHandler<TResult>>) -> windows_core::Result<()>;
    fn Completed(&self) -> windows_core::Result<AsyncOperationCompletedHandler<TResult>>;
    fn GetResults(&self) -> windows_core::Result<TResult>;
}
impl<TResult: windows_core::RuntimeType + 'static> IAsyncOperation_Vtbl<TResult> {
    pub const fn new<Identity: IAsyncOperation_Impl<TResult>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCompleted<TResult: windows_core::RuntimeType + 'static, Identity: IAsyncOperation_Impl<TResult>, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncOperation_Impl::SetCompleted(this, core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn Completed<TResult: windows_core::RuntimeType + 'static, Identity: IAsyncOperation_Impl<TResult>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn GetResults<TResult: windows_core::RuntimeType + 'static, Identity: IAsyncOperation_Impl<TResult>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<TResult>) -> windows_core::HRESULT {
            unsafe {
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
#[repr(C)]
pub struct IAsyncOperation_Vtbl<TResult>
where
    TResult: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::AbiType<TResult>) -> windows_core::HRESULT,
    TResult: core::marker::PhantomData<TResult>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IAsyncOperationWithProgress<TResult, TProgress>(windows_core::IUnknown, core::marker::PhantomData<TResult>, core::marker::PhantomData<TProgress>)
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static;
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IAsyncOperationWithProgress<TResult, TProgress> {}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IAsyncOperationWithProgress<TResult, TProgress> {}
unsafe impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> windows_core::Interface for IAsyncOperationWithProgress<TResult, TProgress> {
    type Vtable = IAsyncOperationWithProgress_Vtbl<TResult, TProgress>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IAsyncOperationWithProgress<TResult, TProgress> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({b5d036d7-e297-498f-ba60-0289e76e23dd}").push_slice(b";").push_other(TResult::SIGNATURE).push_slice(b";").push_other(TProgress::SIGNATURE).push_slice(b")");
}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IAsyncInfo> for IAsyncOperationWithProgress<TResult, TProgress> {
    const QUERY: bool = true;
}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> IAsyncOperationWithProgress<TResult, TProgress> {
    pub fn SetProgress<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AsyncOperationProgressHandler<TResult, TProgress>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProgress)(windows_core::Interface::as_raw(this), handler.param().abi()).ok() }
    }
    pub fn Progress(&self) -> windows_core::Result<AsyncOperationProgressHandler<TResult, TProgress>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Progress)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetCompleted<P0>(&self, handler: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetCompleted)(windows_core::Interface::as_raw(this), handler.param().abi()).ok() }
    }
    pub fn Completed(&self) -> windows_core::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Completed)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetResults(&self) -> windows_core::Result<TResult> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetResults)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Status(&self) -> windows_core::Result<AsyncStatus> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Status)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ErrorCode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Cancel)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
unsafe impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> Send for IAsyncOperationWithProgress<TResult, TProgress> {}
unsafe impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> Sync for IAsyncOperationWithProgress<TResult, TProgress> {}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IAsyncOperationWithProgress<TResult, TProgress> {
    const NAME: &'static str = "Windows.Foundation.IAsyncOperationWithProgress";
}
pub trait IAsyncOperationWithProgress_Impl<TResult, TProgress>: IAsyncInfo_Impl
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static,
{
    fn SetProgress(&self, handler: windows_core::Ref<'_, AsyncOperationProgressHandler<TResult, TProgress>>) -> windows_core::Result<()>;
    fn Progress(&self) -> windows_core::Result<AsyncOperationProgressHandler<TResult, TProgress>>;
    fn SetCompleted(&self, handler: windows_core::Ref<'_, AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>) -> windows_core::Result<()>;
    fn Completed(&self) -> windows_core::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>;
    fn GetResults(&self) -> windows_core::Result<TResult>;
}
impl<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static> IAsyncOperationWithProgress_Vtbl<TResult, TProgress> {
    pub const fn new<Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetProgress<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncOperationWithProgress_Impl::SetProgress(this, core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn Progress<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn SetCompleted<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAsyncOperationWithProgress_Impl::SetCompleted(this, core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn Completed<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn GetResults<TResult: windows_core::RuntimeType + 'static, TProgress: windows_core::RuntimeType + 'static, Identity: IAsyncOperationWithProgress_Impl<TResult, TProgress>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<TResult>) -> windows_core::HRESULT {
            unsafe {
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
#[repr(C)]
pub struct IAsyncOperationWithProgress_Vtbl<TResult, TProgress>
where
    TResult: windows_core::RuntimeType + 'static,
    TProgress: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Progress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Completed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetResults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::AbiType<TResult>) -> windows_core::HRESULT,
    TResult: core::marker::PhantomData<TResult>,
    TProgress: core::marker::PhantomData<TProgress>,
}
windows_core::imp::define_interface!(IClosable, IClosable_Vtbl, 0x30d5a829_7fa4_4026_83bb_d75bae4ea99e);
impl windows_core::RuntimeType for IClosable {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IClosable, windows_core::IUnknown, windows_core::IInspectable);
impl IClosable {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for IClosable {
    const NAME: &'static str = "Windows.Foundation.IClosable";
}
pub trait IClosable_Impl: windows_core::IUnknownImpl {
    fn Close(&self) -> windows_core::Result<()>;
}
impl IClosable_Vtbl {
    pub const fn new<Identity: IClosable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Close<Identity: IClosable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClosable_Impl::Close(this).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IClosable, OFFSET>(), Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClosable as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IClosable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeferral, IDeferral_Vtbl, 0xd6269732_3b7f_46a7_b40b_4fdca2a2c693);
impl windows_core::RuntimeType for IDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeferralFactory, IDeferralFactory_Vtbl, 0x65a1ecc5_3fb5_4832_8ca9_f061b281d13a);
impl windows_core::RuntimeType for IDeferralFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDeferralFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGetActivationFactory, IGetActivationFactory_Vtbl, 0x4edb8ee2_96dd_49a7_94f7_4607ddab8e3c);
impl windows_core::RuntimeType for IGetActivationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IGetActivationFactory, windows_core::IUnknown, windows_core::IInspectable);
impl IGetActivationFactory {
    pub fn GetActivationFactory(&self, activatableclassid: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetActivationFactory)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(activatableclassid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IGetActivationFactory {
    const NAME: &'static str = "Windows.Foundation.IGetActivationFactory";
}
pub trait IGetActivationFactory_Impl: windows_core::IUnknownImpl {
    fn GetActivationFactory(&self, activatableClassId: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable>;
}
impl IGetActivationFactory_Vtbl {
    pub const fn new<Identity: IGetActivationFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetActivationFactory<Identity: IGetActivationFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activatableclassid: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
#[repr(C)]
pub struct IGetActivationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetActivationFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGuidHelperStatics, IGuidHelperStatics_Vtbl, 0x59c7966b_ae52_5283_ad7f_a1b9e9678add);
impl windows_core::RuntimeType for IGuidHelperStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGuidHelperStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateNewGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Empty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(*mut core::ffi::c_void, &windows_core::GUID, &windows_core::GUID, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMemoryBuffer, IMemoryBuffer_Vtbl, 0xfbc4dd2a_245b_11e4_af98_689423260cf8);
impl windows_core::RuntimeType for IMemoryBuffer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IMemoryBuffer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IMemoryBuffer, IClosable);
impl IMemoryBuffer {
    pub fn CreateReference(&self) -> windows_core::Result<IMemoryBufferReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for IMemoryBuffer {
    const NAME: &'static str = "Windows.Foundation.IMemoryBuffer";
}
pub trait IMemoryBuffer_Impl: IClosable_Impl {
    fn CreateReference(&self) -> windows_core::Result<IMemoryBufferReference>;
}
impl IMemoryBuffer_Vtbl {
    pub const fn new<Identity: IMemoryBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateReference<Identity: IMemoryBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMemoryBuffer, OFFSET>(), CreateReference: CreateReference::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMemoryBuffer as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IMemoryBuffer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMemoryBufferFactory, IMemoryBufferFactory_Vtbl, 0xfbc4dd2b_245b_11e4_af98_689423260cf8);
impl windows_core::RuntimeType for IMemoryBufferFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IMemoryBufferFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMemoryBufferReference, IMemoryBufferReference_Vtbl, 0xfbc4dd29_245b_11e4_af98_689423260cf8);
impl windows_core::RuntimeType for IMemoryBufferReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IMemoryBufferReference, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IMemoryBufferReference, IClosable);
impl IMemoryBufferReference {
    pub fn Capacity(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Closed<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<TypedEventHandler<IMemoryBufferReference, windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Closed)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveClosed(&self, cookie: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveClosed)(windows_core::Interface::as_raw(this), cookie).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for IMemoryBufferReference {
    const NAME: &'static str = "Windows.Foundation.IMemoryBufferReference";
}
pub trait IMemoryBufferReference_Impl: IClosable_Impl {
    fn Capacity(&self) -> windows_core::Result<u32>;
    fn Closed(&self, handler: windows_core::Ref<'_, TypedEventHandler<IMemoryBufferReference, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveClosed(&self, cookie: i64) -> windows_core::Result<()>;
}
impl IMemoryBufferReference_Vtbl {
    pub const fn new<Identity: IMemoryBufferReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Capacity<Identity: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMemoryBufferReference_Impl::Capacity(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Closed<Identity: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMemoryBufferReference_Impl::Closed(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveClosed<Identity: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMemoryBufferReference_Impl::RemoveClosed(this, cookie).into()
            }
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
#[repr(C)]
pub struct IMemoryBufferReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Capacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyValue, IPropertyValue_Vtbl, 0x4bd682dd_7554_40e9_9a9b_82654ede7e62);
impl windows_core::RuntimeType for IPropertyValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPropertyValue, windows_core::IUnknown, windows_core::IInspectable);
impl IPropertyValue {
    pub fn Type(&self) -> windows_core::Result<PropertyType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsNumericScalar(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNumericScalar)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt8(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt8)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetInt16(&self) -> windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt16(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetInt32(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt32(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetInt64(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt64(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetSingle(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSingle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDouble(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDouble)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetChar16(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChar16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetBoolean(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBoolean)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDateTime(&self) -> windows_core::Result<DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDateTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetTimeSpan(&self) -> windows_core::Result<TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTimeSpan)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetPoint(&self) -> windows_core::Result<Point> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetSize(&self) -> windows_core::Result<Size> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetRect(&self) -> windows_core::Result<Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt8Array(&self, value: &mut windows_core::Array<u8>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetUInt8Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt16Array(&self, value: &mut windows_core::Array<i16>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetInt16Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetUInt16Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt32Array(&self, value: &mut windows_core::Array<i32>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetInt32Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt32Array(&self, value: &mut windows_core::Array<u32>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetUInt32Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt64Array(&self, value: &mut windows_core::Array<i64>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetInt64Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt64Array(&self, value: &mut windows_core::Array<u64>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetUInt64Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSingleArray(&self, value: &mut windows_core::Array<f32>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetSingleArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDoubleArray(&self, value: &mut windows_core::Array<f64>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetDoubleArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetChar16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetChar16Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetBooleanArray(&self, value: &mut windows_core::Array<bool>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetBooleanArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetStringArray(&self, value: &mut windows_core::Array<windows_core::HSTRING>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetStringArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInspectableArray(&self, value: &mut windows_core::Array<windows_core::IInspectable>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetInspectableArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetGuidArray(&self, value: &mut windows_core::Array<windows_core::GUID>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetGuidArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDateTimeArray(&self, value: &mut windows_core::Array<DateTime>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetDateTimeArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetTimeSpanArray(&self, value: &mut windows_core::Array<TimeSpan>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetTimeSpanArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetPointArray(&self, value: &mut windows_core::Array<Point>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetPointArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSizeArray(&self, value: &mut windows_core::Array<Size>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetSizeArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetRectArray(&self, value: &mut windows_core::Array<Rect>) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).GetRectArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
impl windows_core::RuntimeName for IPropertyValue {
    const NAME: &'static str = "Windows.Foundation.IPropertyValue";
}
pub trait IPropertyValue_Impl: windows_core::IUnknownImpl {
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
impl IPropertyValue_Vtbl {
    pub const fn new<Identity: IPropertyValue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Type<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PropertyType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::Type(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsNumericScalar<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::IsNumericScalar(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUInt8<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetUInt8(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInt16<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetInt16(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUInt16<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetUInt16(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInt32<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetInt32(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUInt32<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetUInt32(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInt64<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetInt64(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUInt64<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetUInt64(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSingle<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetSingle(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDouble<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetDouble(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChar16<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetChar16(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBoolean<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetBoolean(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetString<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn GetGuid<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetGuid(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDateTime<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut DateTime) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetDateTime(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTimeSpan<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetTimeSpan(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPoint<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetPoint(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSize<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Size) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetSize(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRect<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertyValue_Impl::GetRect(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUInt8Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetUInt8Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetInt16Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetInt16Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetUInt16Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetUInt16Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetInt32Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetInt32Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetUInt32Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetUInt32Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetInt64Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetInt64Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetUInt64Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetUInt64Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetSingleArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetSingleArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetDoubleArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetDoubleArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetChar16Array<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetChar16Array(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetBooleanArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetBooleanArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetStringArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut windows_core::HSTRING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetStringArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetInspectableArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut windows_core::IInspectable) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetInspectableArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetGuidArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetGuidArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetDateTimeArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut DateTime) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetDateTimeArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetTimeSpanArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetTimeSpanArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetPointArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetPointArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetSizeArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Size) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetSizeArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
        }
        unsafe extern "system" fn GetRectArray<Identity: IPropertyValue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: *mut u32, value: *mut *mut Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertyValue_Impl::GetRectArray(this, windows_core::ArrayProxy::from_raw_parts(core::mem::transmute_copy(&value), value_array_size).as_array()).into()
            }
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
#[repr(C)]
pub struct IPropertyValue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PropertyType) -> windows_core::HRESULT,
    pub IsNumericScalar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetUInt8: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub GetInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub GetUInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetUInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetInt64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub GetUInt64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetSingle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetDouble: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub GetChar16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DateTime) -> windows_core::HRESULT,
    pub GetTimeSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TimeSpan) -> windows_core::HRESULT,
    pub GetPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Point) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Size) -> windows_core::HRESULT,
    pub GetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Rect) -> windows_core::HRESULT,
    pub GetUInt8Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetInt16Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut i16) -> windows_core::HRESULT,
    pub GetUInt16Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u16) -> windows_core::HRESULT,
    pub GetInt32Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut i32) -> windows_core::HRESULT,
    pub GetUInt32Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u32) -> windows_core::HRESULT,
    pub GetInt64Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut i64) -> windows_core::HRESULT,
    pub GetUInt64Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u64) -> windows_core::HRESULT,
    pub GetSingleArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut f32) -> windows_core::HRESULT,
    pub GetDoubleArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut f64) -> windows_core::HRESULT,
    pub GetChar16Array: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u16) -> windows_core::HRESULT,
    pub GetBooleanArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut bool) -> windows_core::HRESULT,
    pub GetStringArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::HSTRING) -> windows_core::HRESULT,
    pub GetInspectableArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::IInspectable) -> windows_core::HRESULT,
    pub GetGuidArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetDateTimeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut DateTime) -> windows_core::HRESULT,
    pub GetTimeSpanArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut TimeSpan) -> windows_core::HRESULT,
    pub GetPointArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut Point) -> windows_core::HRESULT,
    pub GetSizeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut Size) -> windows_core::HRESULT,
    pub GetRectArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertyValueStatics, IPropertyValueStatics_Vtbl, 0x629bdbc8_d932_4ff4_96b9_8d96c5c1e858);
impl windows_core::RuntimeType for IPropertyValueStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IPropertyValueStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateEmpty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUInt8: unsafe extern "system" fn(*mut core::ffi::c_void, u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInt16: unsafe extern "system" fn(*mut core::ffi::c_void, i16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUInt16: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInt32: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUInt32: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInt64: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUInt64: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSingle: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDouble: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateChar16: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInspectable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGuid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTimeSpan: unsafe extern "system" fn(*mut core::ffi::c_void, TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePoint: unsafe extern "system" fn(*mut core::ffi::c_void, Point, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSize: unsafe extern "system" fn(*mut core::ffi::c_void, Size, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRect: unsafe extern "system" fn(*mut core::ffi::c_void, Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUInt8Array: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInt16Array: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUInt16Array: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInt32Array: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUInt32Array: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInt64Array: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateUInt64Array: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSingleArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDoubleArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateChar16Array: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBooleanArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateStringArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::HSTRING, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInspectableArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::IInspectable, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGuidArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDateTimeArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DateTime, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTimeSpanArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const TimeSpan, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePointArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const Point, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateSizeArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const Size, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateRectArray: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IReference<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IReference<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IReference<T> {}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IReference<T> {
    type Vtable = IReference_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IReference<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({61c17706-2d65-11e0-9ae8-d48564015472}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IPropertyValue> for IReference<T> {
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IReference<T> {
    pub fn Value(&self) -> windows_core::Result<T> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Type(&self) -> windows_core::Result<PropertyType> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsNumericScalar(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNumericScalar)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt8(&self) -> windows_core::Result<u8> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt8)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetInt16(&self) -> windows_core::Result<i16> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt16(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetInt32(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt32(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetInt64(&self) -> windows_core::Result<i64> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt64(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetSingle(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSingle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDouble(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDouble)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetChar16(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChar16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetBoolean(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBoolean)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDateTime(&self) -> windows_core::Result<DateTime> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDateTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetTimeSpan(&self) -> windows_core::Result<TimeSpan> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTimeSpan)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetPoint(&self) -> windows_core::Result<Point> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetSize(&self) -> windows_core::Result<Size> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetRect(&self) -> windows_core::Result<Rect> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt8Array(&self, value: &mut windows_core::Array<u8>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetUInt8Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt16Array(&self, value: &mut windows_core::Array<i16>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetInt16Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetUInt16Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt32Array(&self, value: &mut windows_core::Array<i32>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetInt32Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt32Array(&self, value: &mut windows_core::Array<u32>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetUInt32Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt64Array(&self, value: &mut windows_core::Array<i64>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetInt64Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt64Array(&self, value: &mut windows_core::Array<u64>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetUInt64Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSingleArray(&self, value: &mut windows_core::Array<f32>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetSingleArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDoubleArray(&self, value: &mut windows_core::Array<f64>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetDoubleArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetChar16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetChar16Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetBooleanArray(&self, value: &mut windows_core::Array<bool>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetBooleanArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetStringArray(&self, value: &mut windows_core::Array<windows_core::HSTRING>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetStringArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInspectableArray(&self, value: &mut windows_core::Array<windows_core::IInspectable>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetInspectableArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetGuidArray(&self, value: &mut windows_core::Array<windows_core::GUID>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetGuidArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDateTimeArray(&self, value: &mut windows_core::Array<DateTime>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetDateTimeArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetTimeSpanArray(&self, value: &mut windows_core::Array<TimeSpan>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetTimeSpanArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetPointArray(&self, value: &mut windows_core::Array<Point>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetPointArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSizeArray(&self, value: &mut windows_core::Array<Size>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetSizeArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetRectArray(&self, value: &mut windows_core::Array<Rect>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetRectArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IReference<T> {
    const NAME: &'static str = "Windows.Foundation.IReference";
}
pub trait IReference_Impl<T>: IPropertyValue_Impl
where
    T: windows_core::RuntimeType + 'static,
{
    fn Value(&self) -> windows_core::Result<T>;
}
impl<T: windows_core::RuntimeType + 'static> IReference_Vtbl<T> {
    pub const fn new<Identity: IReference_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<T: windows_core::RuntimeType + 'static, Identity: IReference_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::AbiType<T>) -> windows_core::HRESULT {
            unsafe {
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
#[repr(C)]
pub struct IReference_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::AbiType<T>) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IReferenceArray<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IUnknown> for IReferenceArray<T> {}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<windows_core::IInspectable> for IReferenceArray<T> {}
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for IReferenceArray<T> {
    type Vtable = IReferenceArray_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for IReferenceArray<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({61c17707-2d65-11e0-9ae8-d48564015472}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> windows_core::imp::CanInto<IPropertyValue> for IReferenceArray<T> {
    const QUERY: bool = true;
}
impl<T: windows_core::RuntimeType + 'static> IReferenceArray<T> {
    pub fn Value(&self) -> windows_core::Result<windows_core::Array<T>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), windows_core::Array::<T>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn Type(&self) -> windows_core::Result<PropertyType> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Type)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsNumericScalar(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsNumericScalar)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt8(&self) -> windows_core::Result<u8> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt8)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetInt16(&self) -> windows_core::Result<i16> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt16(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetInt32(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt32(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetInt64(&self) -> windows_core::Result<i64> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt64(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetUInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetSingle(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSingle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDouble(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDouble)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetChar16(&self) -> windows_core::Result<u16> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChar16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetBoolean(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBoolean)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetGuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetDateTime(&self) -> windows_core::Result<DateTime> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDateTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetTimeSpan(&self) -> windows_core::Result<TimeSpan> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTimeSpan)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetPoint(&self) -> windows_core::Result<Point> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetSize(&self) -> windows_core::Result<Size> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetRect(&self) -> windows_core::Result<Rect> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetUInt8Array(&self, value: &mut windows_core::Array<u8>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetUInt8Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt16Array(&self, value: &mut windows_core::Array<i16>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetInt16Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetUInt16Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt32Array(&self, value: &mut windows_core::Array<i32>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetInt32Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt32Array(&self, value: &mut windows_core::Array<u32>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetUInt32Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInt64Array(&self, value: &mut windows_core::Array<i64>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetInt64Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetUInt64Array(&self, value: &mut windows_core::Array<u64>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetUInt64Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSingleArray(&self, value: &mut windows_core::Array<f32>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetSingleArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDoubleArray(&self, value: &mut windows_core::Array<f64>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetDoubleArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetChar16Array(&self, value: &mut windows_core::Array<u16>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetChar16Array)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetBooleanArray(&self, value: &mut windows_core::Array<bool>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetBooleanArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetStringArray(&self, value: &mut windows_core::Array<windows_core::HSTRING>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetStringArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetInspectableArray(&self, value: &mut windows_core::Array<windows_core::IInspectable>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetInspectableArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetGuidArray(&self, value: &mut windows_core::Array<windows_core::GUID>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetGuidArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetDateTimeArray(&self, value: &mut windows_core::Array<DateTime>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetDateTimeArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetTimeSpanArray(&self, value: &mut windows_core::Array<TimeSpan>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetTimeSpanArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetPointArray(&self, value: &mut windows_core::Array<Point>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetPointArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetSizeArray(&self, value: &mut windows_core::Array<Size>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetSizeArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    pub fn GetRectArray(&self, value: &mut windows_core::Array<Rect>) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (windows_core::Interface::vtable(this).GetRectArray)(windows_core::Interface::as_raw(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeName for IReferenceArray<T> {
    const NAME: &'static str = "Windows.Foundation.IReferenceArray";
}
pub trait IReferenceArray_Impl<T>: IPropertyValue_Impl
where
    T: windows_core::RuntimeType + 'static,
{
    fn Value(&self) -> windows_core::Result<windows_core::Array<T>>;
}
impl<T: windows_core::RuntimeType + 'static> IReferenceArray_Vtbl<T> {
    pub const fn new<Identity: IReferenceArray_Impl<T>, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Value<T: windows_core::RuntimeType + 'static, Identity: IReferenceArray_Impl<T>, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut windows_core::AbiType<T>) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReferenceArray_Impl::Value(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[repr(C)]
pub struct IReferenceArray_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    pub base__: windows_core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut windows_core::AbiType<T>) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
windows_core::imp::define_interface!(IStringable, IStringable_Vtbl, 0x96369f54_8eb6_48f0_abce_c1b211e627c3);
impl windows_core::RuntimeType for IStringable {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IStringable, windows_core::IUnknown, windows_core::IInspectable);
impl IStringable {
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IStringable {
    const NAME: &'static str = "Windows.Foundation.IStringable";
}
pub trait IStringable_Impl: windows_core::IUnknownImpl {
    fn ToString(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IStringable_Vtbl {
    pub const fn new<Identity: IStringable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ToString<Identity: IStringable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IStringable, OFFSET>(), ToString: ToString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStringable as windows_core::Interface>::IID
    }
}
#[repr(C)]
pub struct IStringable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ToString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUriEscapeStatics, IUriEscapeStatics_Vtbl, 0xc1d432ba_c824_4452_a7fd_512bc3bbe9a1);
impl windows_core::RuntimeType for IUriEscapeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUriEscapeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnescapeComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EscapeComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUriRuntimeClass, IUriRuntimeClass_Vtbl, 0x9e365e57_48b2_4160_956f_c7385120bbfc);
impl windows_core::RuntimeType for IUriRuntimeClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUriRuntimeClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AbsoluteUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Extension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Fragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Host: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub QueryParsed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QueryParsed: usize,
    pub RawUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SchemeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Port: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Suspicious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CombineUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUriRuntimeClassFactory, IUriRuntimeClassFactory_Vtbl, 0x44a9796f_723e_4fdf_a218_033e75b0c084);
impl windows_core::RuntimeType for IUriRuntimeClassFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUriRuntimeClassFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithRelativeUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUriRuntimeClassWithAbsoluteCanonicalUri, IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl, 0x758d9661_221c_480f_a339_50656673f46f);
impl windows_core::RuntimeType for IUriRuntimeClassWithAbsoluteCanonicalUri {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AbsoluteCanonicalUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayIri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWwwFormUrlDecoderEntry, IWwwFormUrlDecoderEntry_Vtbl, 0x125e7431_f678_4e8e_b670_20a9b06c512d);
impl windows_core::RuntimeType for IWwwFormUrlDecoderEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IWwwFormUrlDecoderEntry, windows_core::IUnknown, windows_core::IInspectable);
impl IWwwFormUrlDecoderEntry {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IWwwFormUrlDecoderEntry {
    const NAME: &'static str = "Windows.Foundation.IWwwFormUrlDecoderEntry";
}
pub trait IWwwFormUrlDecoderEntry_Impl: windows_core::IUnknownImpl {
    fn Name(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Value(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IWwwFormUrlDecoderEntry_Vtbl {
    pub const fn new<Identity: IWwwFormUrlDecoderEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Name<Identity: IWwwFormUrlDecoderEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn Value<Identity: IWwwFormUrlDecoderEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
#[repr(C)]
pub struct IWwwFormUrlDecoderEntry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::define_interface!(IWwwFormUrlDecoderRuntimeClass, IWwwFormUrlDecoderRuntimeClass_Vtbl, 0xd45a0451_f225_4542_9296_0e1df5d254df);
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for IWwwFormUrlDecoderRuntimeClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[cfg(feature = "Foundation_Collections")]
#[repr(C)]
pub struct IWwwFormUrlDecoderRuntimeClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFirstValueByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWwwFormUrlDecoderRuntimeClassFactory, IWwwFormUrlDecoderRuntimeClassFactory_Vtbl, 0x5b8c6b3d_24ae_41b5_a1bf_f0c3d544845b);
impl windows_core::RuntimeType for IWwwFormUrlDecoderRuntimeClassFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IWwwFormUrlDecoderRuntimeClassFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateWwwFormUrlDecoder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateWwwFormUrlDecoder: usize,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryBuffer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MemoryBuffer, windows_core::IUnknown, windows_core::IInspectable, IMemoryBuffer);
windows_core::imp::required_hierarchy!(MemoryBuffer, IClosable);
impl MemoryBuffer {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateReference(&self) -> windows_core::Result<IMemoryBufferReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateReference)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Create(capacity: u32) -> windows_core::Result<MemoryBuffer> {
        Self::IMemoryBufferFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), capacity, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IMemoryBufferFactory<R, F: FnOnce(&IMemoryBufferFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MemoryBuffer, IMemoryBufferFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MemoryBuffer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMemoryBuffer>();
}
unsafe impl windows_core::Interface for MemoryBuffer {
    type Vtable = <IMemoryBuffer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMemoryBuffer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MemoryBuffer {
    const NAME: &'static str = "Windows.Foundation.MemoryBuffer";
}
unsafe impl Send for MemoryBuffer {}
unsafe impl Sync for MemoryBuffer {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub X: f32,
    pub Y: f32,
}
impl windows_core::TypeKind for Point {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Point {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Point;f4;f4)");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PropertyType(pub i32);
impl PropertyType {
    pub const Empty: Self = Self(0i32);
    pub const UInt8: Self = Self(1i32);
    pub const Int16: Self = Self(2i32);
    pub const UInt16: Self = Self(3i32);
    pub const Int32: Self = Self(4i32);
    pub const UInt32: Self = Self(5i32);
    pub const Int64: Self = Self(6i32);
    pub const UInt64: Self = Self(7i32);
    pub const Single: Self = Self(8i32);
    pub const Double: Self = Self(9i32);
    pub const Char16: Self = Self(10i32);
    pub const Boolean: Self = Self(11i32);
    pub const String: Self = Self(12i32);
    pub const Inspectable: Self = Self(13i32);
    pub const DateTime: Self = Self(14i32);
    pub const TimeSpan: Self = Self(15i32);
    pub const Guid: Self = Self(16i32);
    pub const Point: Self = Self(17i32);
    pub const Size: Self = Self(18i32);
    pub const Rect: Self = Self(19i32);
    pub const OtherType: Self = Self(20i32);
    pub const UInt8Array: Self = Self(1025i32);
    pub const Int16Array: Self = Self(1026i32);
    pub const UInt16Array: Self = Self(1027i32);
    pub const Int32Array: Self = Self(1028i32);
    pub const UInt32Array: Self = Self(1029i32);
    pub const Int64Array: Self = Self(1030i32);
    pub const UInt64Array: Self = Self(1031i32);
    pub const SingleArray: Self = Self(1032i32);
    pub const DoubleArray: Self = Self(1033i32);
    pub const Char16Array: Self = Self(1034i32);
    pub const BooleanArray: Self = Self(1035i32);
    pub const StringArray: Self = Self(1036i32);
    pub const InspectableArray: Self = Self(1037i32);
    pub const DateTimeArray: Self = Self(1038i32);
    pub const TimeSpanArray: Self = Self(1039i32);
    pub const GuidArray: Self = Self(1040i32);
    pub const PointArray: Self = Self(1041i32);
    pub const SizeArray: Self = Self(1042i32);
    pub const RectArray: Self = Self(1043i32);
    pub const OtherTypeArray: Self = Self(1044i32);
}
impl windows_core::TypeKind for PropertyType {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for PropertyType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Foundation.PropertyType;i4)");
}
pub struct PropertyValue;
impl PropertyValue {
    pub fn CreateEmpty() -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateEmpty)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateUInt8(value: u8) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUInt8)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInt16(value: i16) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInt16)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateUInt16(value: u16) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUInt16)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInt32(value: i32) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInt32)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateUInt32(value: u32) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUInt32)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInt64(value: i64) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInt64)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateUInt64(value: u64) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUInt64)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateSingle(value: f32) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSingle)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateDouble(value: f64) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDouble)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateChar16(value: u16) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateChar16)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateBoolean(value: bool) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBoolean)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateString(value: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInspectable<P0>(value: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInspectable)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateGuid(value: windows_core::GUID) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateGuid)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateDateTime(value: DateTime) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDateTime)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateTimeSpan(value: TimeSpan) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTimeSpan)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreatePoint(value: Point) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePoint)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateSize(value: Size) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSize)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateRect(value: Rect) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateRect)(windows_core::Interface::as_raw(this), value, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateUInt8Array(value: &[u8]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUInt8Array)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInt16Array(value: &[i16]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInt16Array)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateUInt16Array(value: &[u16]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUInt16Array)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInt32Array(value: &[i32]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInt32Array)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateUInt32Array(value: &[u32]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUInt32Array)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInt64Array(value: &[i64]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInt64Array)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateUInt64Array(value: &[u64]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUInt64Array)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateSingleArray(value: &[f32]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSingleArray)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateDoubleArray(value: &[f64]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDoubleArray)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateChar16Array(value: &[u16]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateChar16Array)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateBooleanArray(value: &[bool]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateBooleanArray)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateStringArray(value: &[windows_core::HSTRING]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateStringArray)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), core::mem::transmute(value.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateInspectableArray(value: &[Option<windows_core::IInspectable>]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInspectableArray)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), core::mem::transmute(value.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateGuidArray(value: &[windows_core::GUID]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateGuidArray)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateDateTimeArray(value: &[DateTime]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDateTimeArray)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateTimeSpanArray(value: &[TimeSpan]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTimeSpanArray)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreatePointArray(value: &[Point]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePointArray)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateSizeArray(value: &[Size]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSizeArray)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateRectArray(value: &[Rect]) -> windows_core::Result<windows_core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateRectArray)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IPropertyValueStatics<R, F: FnOnce(&IPropertyValueStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PropertyValue, IPropertyValueStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for PropertyValue {
    const NAME: &'static str = "Windows.Foundation.PropertyValue";
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
impl windows_core::TypeKind for Rect {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Rect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Rect;f4;f4;f4;f4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub Width: f32,
    pub Height: f32,
}
impl windows_core::TypeKind for Size {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Size {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Size;f4;f4)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TimeSpan {
    pub Duration: i64,
}
impl windows_core::TypeKind for TimeSpan {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TimeSpan {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.TimeSpan;i8)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedEventHandler<TSender, TResult>(windows_core::IUnknown, core::marker::PhantomData<TSender>, core::marker::PhantomData<TResult>)
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static;
unsafe impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static> windows_core::Interface for TypedEventHandler<TSender, TResult> {
    type Vtable = TypedEventHandler_Vtbl<TSender, TResult>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static> windows_core::RuntimeType for TypedEventHandler<TSender, TResult> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({9de1c534-6ae1-11e0-84e1-18a905bcc53f}").push_slice(b";").push_other(TSender::SIGNATURE).push_slice(b";").push_other(TResult::SIGNATURE).push_slice(b")");
}
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static> TypedEventHandler<TSender, TResult> {
    pub fn new<F: FnMut(windows_core::Ref<'_, TSender>, windows_core::Ref<'_, TResult>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = TypedEventHandlerBox { vtable: &TypedEventHandlerBox::<TSender, TResult, F>::VTABLE, count: windows_core::imp::RefCount::new(1), invoke };
        unsafe { core::mem::transmute(windows_core::imp::Box::new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<TSender>,
        P1: windows_core::Param<TResult>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Invoke)(windows_core::Interface::as_raw(this), sender.param().abi(), args.param().abi()).ok() }
    }
}
#[repr(C)]
pub struct TypedEventHandler_Vtbl<TSender, TResult>
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: windows_core::AbiType<TSender>, args: windows_core::AbiType<TResult>) -> windows_core::HRESULT,
    TSender: core::marker::PhantomData<TSender>,
    TResult: core::marker::PhantomData<TResult>,
}
#[repr(C)]
struct TypedEventHandlerBox<TSender, TResult, F: FnMut(windows_core::Ref<'_, TSender>, windows_core::Ref<'_, TResult>) -> windows_core::Result<()> + Send + 'static>
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static,
{
    vtable: *const TypedEventHandler_Vtbl<TSender, TResult>,
    invoke: F,
    count: windows_core::imp::RefCount,
}
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static, F: FnMut(windows_core::Ref<'_, TSender>, windows_core::Ref<'_, TResult>) -> windows_core::Result<()> + Send + 'static> TypedEventHandlerBox<TSender, TResult, F> {
    const VTABLE: TypedEventHandler_Vtbl<TSender, TResult> = TypedEventHandler_Vtbl::<TSender, TResult> {
        base__: windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
        TSender: core::marker::PhantomData::<TSender>,
        TResult: core::marker::PhantomData::<TResult>,
    };
    unsafe extern "system" fn QueryInterface(this: *mut core::ffi::c_void, iid: *const windows_core::GUID, interface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            if iid.is_null() || interface.is_null() {
                return windows_core::HRESULT(-2147467261);
            }
            *interface = if *iid == <TypedEventHandler<TSender, TResult> as windows_core::Interface>::IID || *iid == <windows_core::IUnknown as windows_core::Interface>::IID || *iid == <windows_core::imp::IAgileObject as windows_core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { core::ptr::null_mut() };
            if (*interface).is_null() {
                windows_core::HRESULT(-2147467262)
            } else {
                (*this).count.add_ref();
                windows_core::HRESULT(0)
            }
        }
    }
    unsafe extern "system" fn AddRef(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            (*this).count.add_ref()
        }
    }
    unsafe extern "system" fn Release(this: *mut core::ffi::c_void) -> u32 {
        unsafe {
            let this = this as *mut *mut core::ffi::c_void as *mut Self;
            let remaining = (*this).count.release();
            if remaining == 0 {
                let _ = windows_core::imp::Box::from_raw(this);
            }
            remaining
        }
    }
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: windows_core::AbiType<TSender>, args: windows_core::AbiType<TResult>) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut Self);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&args)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uri(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Uri, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Uri, IStringable);
impl Uri {
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn UnescapeComponent(tounescape: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnescapeComponent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tounescape), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn EscapeComponent(toescape: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EscapeComponent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(toescape), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn AbsoluteUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteUri)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayUri)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Domain(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Domain)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Extension(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Extension)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Fragment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Fragment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Host(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Host)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Password(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Password)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Query(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Query)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn QueryParsed(&self) -> windows_core::Result<WwwFormUrlDecoder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).QueryParsed)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RawUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RawUri)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SchemeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SchemeName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn UserName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UserName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Port(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Port)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Suspicious(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Suspicious)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Equals<P0>(&self, puri: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<Uri>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Equals)(windows_core::Interface::as_raw(this), puri.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn CombineUri(&self, relativeuri: &windows_core::HSTRING) -> windows_core::Result<Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CombineUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(relativeuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateUri(uri: &windows_core::HSTRING) -> windows_core::Result<Uri> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(uri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateWithRelativeUri(baseuri: &windows_core::HSTRING, relativeuri: &windows_core::HSTRING) -> windows_core::Result<Uri> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithRelativeUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(baseuri), core::mem::transmute_copy(relativeuri), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn AbsoluteCanonicalUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteCanonicalUri)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayIri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayIri)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IUriEscapeStatics<R, F: FnOnce(&IUriEscapeStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Uri, IUriEscapeStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IUriRuntimeClassFactory<R, F: FnOnce(&IUriRuntimeClassFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Uri, IUriRuntimeClassFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Uri {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUriRuntimeClass>();
}
unsafe impl windows_core::Interface for Uri {
    type Vtable = <IUriRuntimeClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUriRuntimeClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Uri {
    const NAME: &'static str = "Windows.Foundation.Uri";
}
unsafe impl Send for Uri {}
unsafe impl Sync for Uri {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WwwFormUrlDecoder(windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::interface_hierarchy!(WwwFormUrlDecoder, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(WwwFormUrlDecoder, Collections::IIterable<IWwwFormUrlDecoderEntry>, Collections::IVectorView<IWwwFormUrlDecoderEntry>);
#[cfg(feature = "Foundation_Collections")]
impl WwwFormUrlDecoder {
    pub fn First(&self) -> windows_core::Result<Collections::IIterator<IWwwFormUrlDecoderEntry>> {
        let this = &windows_core::Interface::cast::<Collections::IIterable<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<IWwwFormUrlDecoderEntry> {
        let this = &windows_core::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<IWwwFormUrlDecoderEntry>,
    {
        let this = &windows_core::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<IWwwFormUrlDecoderEntry>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn GetFirstValueByName(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFirstValueByName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateWwwFormUrlDecoder(query: &windows_core::HSTRING) -> windows_core::Result<WwwFormUrlDecoder> {
        Self::IWwwFormUrlDecoderRuntimeClassFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWwwFormUrlDecoder)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(query), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IWwwFormUrlDecoderRuntimeClassFactory<R, F: FnOnce(&IWwwFormUrlDecoderRuntimeClassFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WwwFormUrlDecoder, IWwwFormUrlDecoderRuntimeClassFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeType for WwwFormUrlDecoder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWwwFormUrlDecoderRuntimeClass>();
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl windows_core::Interface for WwwFormUrlDecoder {
    type Vtable = <IWwwFormUrlDecoderRuntimeClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWwwFormUrlDecoderRuntimeClass as windows_core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for WwwFormUrlDecoder {
    const NAME: &'static str = "Windows.Foundation.WwwFormUrlDecoder";
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Send for WwwFormUrlDecoder {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl Sync for WwwFormUrlDecoder {}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WwwFormUrlDecoderEntry(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WwwFormUrlDecoderEntry, windows_core::IUnknown, windows_core::IInspectable, IWwwFormUrlDecoderEntry);
impl WwwFormUrlDecoderEntry {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for WwwFormUrlDecoderEntry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWwwFormUrlDecoderEntry>();
}
unsafe impl windows_core::Interface for WwwFormUrlDecoderEntry {
    type Vtable = <IWwwFormUrlDecoderEntry as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWwwFormUrlDecoderEntry as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WwwFormUrlDecoderEntry {
    const NAME: &'static str = "Windows.Foundation.WwwFormUrlDecoderEntry";
}
unsafe impl Send for WwwFormUrlDecoderEntry {}
unsafe impl Sync for WwwFormUrlDecoderEntry {}
