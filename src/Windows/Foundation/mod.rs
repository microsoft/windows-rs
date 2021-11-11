#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Foundation_Collections")]
pub mod Collections;
#[cfg(feature = "Foundation_Diagnostics")]
pub mod Diagnostics;
#[cfg(feature = "Foundation_Metadata")]
pub mod Metadata;
#[cfg(feature = "Foundation_Numerics")]
pub mod Numerics;
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncActionCompletedHandler(::windows::core::IUnknown);
impl AsyncActionCompletedHandler {
    pub fn new<F: FnMut(&::core::option::Option<IAsyncAction>, AsyncStatus) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = AsyncActionCompletedHandler_box::<F> {
            vtable: &AsyncActionCompletedHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, IAsyncAction>>(&self, asyncinfo: Param0, asyncstatus: AsyncStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), asyncinfo.into_param().abi(), asyncstatus).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for AsyncActionCompletedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({a4ed5c81-76c9-40bd-8be6-b1d90fb20ae7})");
}
unsafe impl ::windows::core::Interface for AsyncActionCompletedHandler {
    type Vtable = AsyncActionCompletedHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4ed5c81_76c9_40bd_8be6_b1d90fb20ae7);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncActionCompletedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, asyncinfo: ::windows::core::RawPtr, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct AsyncActionCompletedHandler_box<F: FnMut(&::core::option::Option<IAsyncAction>, AsyncStatus) -> ::windows::core::Result<()> + 'static> {
    vtable: *const AsyncActionCompletedHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<IAsyncAction>, AsyncStatus) -> ::windows::core::Result<()> + 'static> AsyncActionCompletedHandler_box<F> {
    const VTABLE: AsyncActionCompletedHandler_abi = AsyncActionCompletedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<AsyncActionCompletedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, asyncinfo: ::windows::core::RawPtr, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&asyncinfo as *const <IAsyncAction as ::windows::core::Abi>::Abi as *const <IAsyncAction as ::windows::core::DefaultType>::DefaultType), asyncstatus).into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncActionProgressHandler<TProgress>(::windows::core::IUnknown, ::core::marker::PhantomData<TProgress>)
where
    TProgress: ::windows::core::RuntimeType + 'static;
impl<TProgress: ::windows::core::RuntimeType + 'static> AsyncActionProgressHandler<TProgress> {
    pub fn new<F: FnMut(&::core::option::Option<IAsyncActionWithProgress<TProgress>>, &<TProgress as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = AsyncActionProgressHandler_box::<TProgress, F> {
            vtable: &AsyncActionProgressHandler_box::<TProgress, F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, IAsyncActionWithProgress<TProgress>>, Param1: ::windows::core::IntoParam<'a, TProgress>>(&self, asyncinfo: Param0, progressinfo: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), asyncinfo.into_param().abi(), progressinfo.into_param().abi()).ok() }
    }
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for AsyncActionProgressHandler<TProgress> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{6d844858-0cff-4590-ae89-95a5a5c8b4b8}").push_slice(b";").push_other(<TProgress as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for AsyncActionProgressHandler<TProgress> {
    type Vtable = AsyncActionProgressHandler_abi<TProgress>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<AsyncActionProgressHandler<TProgress> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncActionProgressHandler_abi<TProgress>(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, asyncinfo: ::windows::core::RawPtr, progressinfo: <TProgress as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub ::core::marker::PhantomData<TProgress>,
)
where
    TProgress: ::windows::core::RuntimeType + 'static;
#[repr(C)]
struct AsyncActionProgressHandler_box<TProgress, F: FnMut(&::core::option::Option<IAsyncActionWithProgress<TProgress>>, &<TProgress as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()> + 'static>
where
    TProgress: ::windows::core::RuntimeType + 'static,
{
    vtable: *const AsyncActionProgressHandler_abi<TProgress>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<TProgress: ::windows::core::RuntimeType + 'static, F: FnMut(&::core::option::Option<IAsyncActionWithProgress<TProgress>>, &<TProgress as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()> + 'static> AsyncActionProgressHandler_box<TProgress, F> {
    const VTABLE: AsyncActionProgressHandler_abi<TProgress> = AsyncActionProgressHandler_abi::<TProgress>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::core::marker::PhantomData::<TProgress>);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<AsyncActionProgressHandler<TProgress> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, asyncinfo: ::windows::core::RawPtr, progressinfo: <TProgress as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&asyncinfo as *const <IAsyncActionWithProgress<TProgress> as ::windows::core::Abi>::Abi as *const <IAsyncActionWithProgress<TProgress> as ::windows::core::DefaultType>::DefaultType),
            &*(&progressinfo as *const <TProgress as ::windows::core::Abi>::Abi as *const <TProgress as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncActionWithProgressCompletedHandler<TProgress>(::windows::core::IUnknown, ::core::marker::PhantomData<TProgress>)
where
    TProgress: ::windows::core::RuntimeType + 'static;
impl<TProgress: ::windows::core::RuntimeType + 'static> AsyncActionWithProgressCompletedHandler<TProgress> {
    pub fn new<F: FnMut(&::core::option::Option<IAsyncActionWithProgress<TProgress>>, AsyncStatus) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = AsyncActionWithProgressCompletedHandler_box::<TProgress, F> {
            vtable: &AsyncActionWithProgressCompletedHandler_box::<TProgress, F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, IAsyncActionWithProgress<TProgress>>>(&self, asyncinfo: Param0, asyncstatus: AsyncStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), asyncinfo.into_param().abi(), asyncstatus).ok() }
    }
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for AsyncActionWithProgressCompletedHandler<TProgress> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9c029f91-cc84-44fd-ac26-0a6c4e555281}").push_slice(b";").push_other(<TProgress as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for AsyncActionWithProgressCompletedHandler<TProgress> {
    type Vtable = AsyncActionWithProgressCompletedHandler_abi<TProgress>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<AsyncActionWithProgressCompletedHandler<TProgress> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncActionWithProgressCompletedHandler_abi<TProgress>(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, asyncinfo: ::windows::core::RawPtr, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT,
    pub ::core::marker::PhantomData<TProgress>,
)
where
    TProgress: ::windows::core::RuntimeType + 'static;
#[repr(C)]
struct AsyncActionWithProgressCompletedHandler_box<TProgress, F: FnMut(&::core::option::Option<IAsyncActionWithProgress<TProgress>>, AsyncStatus) -> ::windows::core::Result<()> + 'static>
where
    TProgress: ::windows::core::RuntimeType + 'static,
{
    vtable: *const AsyncActionWithProgressCompletedHandler_abi<TProgress>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<TProgress: ::windows::core::RuntimeType + 'static, F: FnMut(&::core::option::Option<IAsyncActionWithProgress<TProgress>>, AsyncStatus) -> ::windows::core::Result<()> + 'static> AsyncActionWithProgressCompletedHandler_box<TProgress, F> {
    const VTABLE: AsyncActionWithProgressCompletedHandler_abi<TProgress> = AsyncActionWithProgressCompletedHandler_abi::<TProgress>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::core::marker::PhantomData::<TProgress>);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<AsyncActionWithProgressCompletedHandler<TProgress> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, asyncinfo: ::windows::core::RawPtr, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&asyncinfo as *const <IAsyncActionWithProgress<TProgress> as ::windows::core::Abi>::Abi as *const <IAsyncActionWithProgress<TProgress> as ::windows::core::DefaultType>::DefaultType), asyncstatus).into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncOperationCompletedHandler<TResult>(::windows::core::IUnknown, ::core::marker::PhantomData<TResult>)
where
    TResult: ::windows::core::RuntimeType + 'static;
impl<TResult: ::windows::core::RuntimeType + 'static> AsyncOperationCompletedHandler<TResult> {
    pub fn new<F: FnMut(&::core::option::Option<IAsyncOperation<TResult>>, AsyncStatus) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = AsyncOperationCompletedHandler_box::<TResult, F> {
            vtable: &AsyncOperationCompletedHandler_box::<TResult, F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, IAsyncOperation<TResult>>>(&self, asyncinfo: Param0, asyncstatus: AsyncStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), asyncinfo.into_param().abi(), asyncstatus).ok() }
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for AsyncOperationCompletedHandler<TResult> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{fcdcf02c-e5d8-4478-915a-4d90b74b83a5}").push_slice(b";").push_other(<TResult as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for AsyncOperationCompletedHandler<TResult> {
    type Vtable = AsyncOperationCompletedHandler_abi<TResult>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<AsyncOperationCompletedHandler<TResult> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncOperationCompletedHandler_abi<TResult>(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, asyncinfo: ::windows::core::RawPtr, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT,
    pub ::core::marker::PhantomData<TResult>,
)
where
    TResult: ::windows::core::RuntimeType + 'static;
#[repr(C)]
struct AsyncOperationCompletedHandler_box<TResult, F: FnMut(&::core::option::Option<IAsyncOperation<TResult>>, AsyncStatus) -> ::windows::core::Result<()> + 'static>
where
    TResult: ::windows::core::RuntimeType + 'static,
{
    vtable: *const AsyncOperationCompletedHandler_abi<TResult>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<TResult: ::windows::core::RuntimeType + 'static, F: FnMut(&::core::option::Option<IAsyncOperation<TResult>>, AsyncStatus) -> ::windows::core::Result<()> + 'static> AsyncOperationCompletedHandler_box<TResult, F> {
    const VTABLE: AsyncOperationCompletedHandler_abi<TResult> = AsyncOperationCompletedHandler_abi::<TResult>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::core::marker::PhantomData::<TResult>);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<AsyncOperationCompletedHandler<TResult> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, asyncinfo: ::windows::core::RawPtr, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&asyncinfo as *const <IAsyncOperation<TResult> as ::windows::core::Abi>::Abi as *const <IAsyncOperation<TResult> as ::windows::core::DefaultType>::DefaultType), asyncstatus).into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncOperationProgressHandler<TResult, TProgress>(::windows::core::IUnknown, ::core::marker::PhantomData<TResult>, ::core::marker::PhantomData<TProgress>)
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static;
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> AsyncOperationProgressHandler<TResult, TProgress> {
    pub fn new<F: FnMut(&::core::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, &<TProgress as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = AsyncOperationProgressHandler_box::<TResult, TProgress, F> {
            vtable: &AsyncOperationProgressHandler_box::<TResult, TProgress, F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, IAsyncOperationWithProgress<TResult, TProgress>>, Param1: ::windows::core::IntoParam<'a, TProgress>>(&self, asyncinfo: Param0, progressinfo: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), asyncinfo.into_param().abi(), progressinfo.into_param().abi()).ok() }
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for AsyncOperationProgressHandler<TResult, TProgress> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{55690902-0aab-421a-8778-f8ce5026d758}").push_slice(b";").push_other(<TResult as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<TProgress as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for AsyncOperationProgressHandler<TResult, TProgress> {
    type Vtable = AsyncOperationProgressHandler_abi<TResult, TProgress>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<AsyncOperationProgressHandler<TResult, TProgress> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncOperationProgressHandler_abi<TResult, TProgress>(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, asyncinfo: ::windows::core::RawPtr, progressinfo: <TProgress as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub ::core::marker::PhantomData<TResult>,
    pub ::core::marker::PhantomData<TProgress>,
)
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static;
#[repr(C)]
struct AsyncOperationProgressHandler_box<TResult, TProgress, F: FnMut(&::core::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, &<TProgress as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()> + 'static>
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static,
{
    vtable: *const AsyncOperationProgressHandler_abi<TResult, TProgress>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, F: FnMut(&::core::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, &<TProgress as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()> + 'static> AsyncOperationProgressHandler_box<TResult, TProgress, F> {
    const VTABLE: AsyncOperationProgressHandler_abi<TResult, TProgress> = AsyncOperationProgressHandler_abi::<TResult, TProgress>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::core::marker::PhantomData::<TResult>, ::core::marker::PhantomData::<TProgress>);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<AsyncOperationProgressHandler<TResult, TProgress> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, asyncinfo: ::windows::core::RawPtr, progressinfo: <TProgress as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&asyncinfo as *const <IAsyncOperationWithProgress<TResult, TProgress> as ::windows::core::Abi>::Abi as *const <IAsyncOperationWithProgress<TResult, TProgress> as ::windows::core::DefaultType>::DefaultType),
            &*(&progressinfo as *const <TProgress as ::windows::core::Abi>::Abi as *const <TProgress as ::windows::core::DefaultType>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct AsyncOperationWithProgressCompletedHandler<TResult, TProgress>(::windows::core::IUnknown, ::core::marker::PhantomData<TResult>, ::core::marker::PhantomData<TProgress>)
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static;
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    pub fn new<F: FnMut(&::core::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, AsyncStatus) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = AsyncOperationWithProgressCompletedHandler_box::<TResult, TProgress, F> {
            vtable: &AsyncOperationWithProgressCompletedHandler_box::<TResult, TProgress, F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, IAsyncOperationWithProgress<TResult, TProgress>>>(&self, asyncinfo: Param0, asyncstatus: AsyncStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), asyncinfo.into_param().abi(), asyncstatus).ok() }
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{e85df41d-6aa7-46e3-a8e2-f009d840c627}").push_slice(b";").push_other(<TResult as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<TProgress as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    type Vtable = AsyncOperationWithProgressCompletedHandler_abi<TResult, TProgress>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<AsyncOperationWithProgressCompletedHandler<TResult, TProgress> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncOperationWithProgressCompletedHandler_abi<TResult, TProgress>(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, asyncinfo: ::windows::core::RawPtr, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT,
    pub ::core::marker::PhantomData<TResult>,
    pub ::core::marker::PhantomData<TProgress>,
)
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static;
#[repr(C)]
struct AsyncOperationWithProgressCompletedHandler_box<TResult, TProgress, F: FnMut(&::core::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, AsyncStatus) -> ::windows::core::Result<()> + 'static>
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static,
{
    vtable: *const AsyncOperationWithProgressCompletedHandler_abi<TResult, TProgress>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static, F: FnMut(&::core::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, AsyncStatus) -> ::windows::core::Result<()> + 'static> AsyncOperationWithProgressCompletedHandler_box<TResult, TProgress, F> {
    const VTABLE: AsyncOperationWithProgressCompletedHandler_abi<TResult, TProgress> = AsyncOperationWithProgressCompletedHandler_abi::<TResult, TProgress>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::core::marker::PhantomData::<TResult>, ::core::marker::PhantomData::<TProgress>);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<AsyncOperationWithProgressCompletedHandler<TResult, TProgress> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, asyncinfo: ::windows::core::RawPtr, asyncstatus: AsyncStatus) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&asyncinfo as *const <IAsyncOperationWithProgress<TResult, TProgress> as ::windows::core::Abi>::Abi as *const <IAsyncOperationWithProgress<TResult, TProgress> as ::windows::core::DefaultType>::DefaultType), asyncstatus).into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: AsyncStatus = AsyncStatus(2i32);
    pub const Completed: AsyncStatus = AsyncStatus(1i32);
    pub const Error: AsyncStatus = AsyncStatus(3i32);
    pub const Started: AsyncStatus = AsyncStatus(0i32);
}
impl ::core::convert::From<i32> for AsyncStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for AsyncStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for AsyncStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.AsyncStatus;i4)");
}
impl ::windows::core::DefaultType for AsyncStatus {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Foundation`*"]
pub struct DateTime {
    pub UniversalTime: i64,
}
impl DateTime {}
impl ::core::default::Default for DateTime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DateTime {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DateTime").field("UniversalTime", &self.UniversalTime).finish()
    }
}
impl ::core::cmp::PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.UniversalTime == other.UniversalTime
    }
}
impl ::core::cmp::Eq for DateTime {}
unsafe impl ::windows::core::Abi for DateTime {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DateTime {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.DateTime;i8)");
}
impl ::windows::core::DefaultType for DateTime {
    type DefaultType = Self;
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Deferral(pub ::windows::core::IInspectable);
impl Deferral {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, DeferralCompletedHandler>>(handler: Param0) -> ::windows::core::Result<Deferral> {
        Self::IDeferralFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<Deferral>(result__)
        })
    }
    pub fn IDeferralFactory<R, F: FnOnce(&IDeferralFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Deferral, IDeferralFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Deferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Deferral;{d6269732-3b7f-46a7-b40b-4fdca2a2c693})");
}
unsafe impl ::windows::core::Interface for Deferral {
    type Vtable = IDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6269732_3b7f_46a7_b40b_4fdca2a2c693);
}
impl ::windows::core::RuntimeName for Deferral {
    const NAME: &'static str = "Windows.Foundation.Deferral";
}
impl ::core::convert::From<Deferral> for ::windows::core::IUnknown {
    fn from(value: Deferral) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Deferral> for ::windows::core::IUnknown {
    fn from(value: &Deferral) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Deferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Deferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Deferral> for ::windows::core::IInspectable {
    fn from(value: Deferral) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Deferral> for ::windows::core::IInspectable {
    fn from(value: &Deferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Deferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Deferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<Deferral> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: Deferral) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Deferral> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &Deferral) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IClosable> for Deferral {
    fn into_param(self) -> ::windows::core::Param<'a, IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IClosable> for &Deferral {
    fn into_param(self) -> ::windows::core::Param<'a, IClosable> {
        ::core::convert::TryInto::<IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Deferral {}
unsafe impl ::core::marker::Sync for Deferral {}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct DeferralCompletedHandler(::windows::core::IUnknown);
impl DeferralCompletedHandler {
    pub fn new<F: FnMut() -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = DeferralCompletedHandler_box::<F> {
            vtable: &DeferralCompletedHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for DeferralCompletedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({ed32a372-f3c8-4faa-9cfb-470148da3888})");
}
unsafe impl ::windows::core::Interface for DeferralCompletedHandler {
    type Vtable = DeferralCompletedHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed32a372_f3c8_4faa_9cfb_470148da3888);
}
#[repr(C)]
#[doc(hidden)]
pub struct DeferralCompletedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct DeferralCompletedHandler_box<F: FnMut() -> ::windows::core::Result<()> + 'static> {
    vtable: *const DeferralCompletedHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + 'static> DeferralCompletedHandler_box<F> {
    const VTABLE: DeferralCompletedHandler_abi = DeferralCompletedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<DeferralCompletedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)().into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct EventHandler<T>(::windows::core::IUnknown, ::core::marker::PhantomData<T>)
where
    T: ::windows::core::RuntimeType + 'static;
impl<T: ::windows::core::RuntimeType + 'static> EventHandler<T> {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &<T as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = EventHandler_box::<T, F> {
            vtable: &EventHandler_box::<T, F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, T>>(&self, sender: Param0, args: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for EventHandler<T> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9de1c535-6ae1-11e0-84e1-18a905bcc53f}").push_slice(b";").push_other(<T as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for EventHandler<T> {
    type Vtable = EventHandler_abi<T>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<EventHandler<T> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct EventHandler_abi<T>(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, args: <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub ::core::marker::PhantomData<T>,
)
where
    T: ::windows::core::RuntimeType + 'static;
#[repr(C)]
struct EventHandler_box<T, F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &<T as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()> + 'static>
where
    T: ::windows::core::RuntimeType + 'static,
{
    vtable: *const EventHandler_abi<T>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<T: ::windows::core::RuntimeType + 'static, F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &<T as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()> + 'static> EventHandler_box<T, F> {
    const VTABLE: EventHandler_abi<T> = EventHandler_abi::<T>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::core::marker::PhantomData::<T>);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<EventHandler<T> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: ::windows::core::RawPtr, args: <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <T as ::windows::core::Abi>::Abi as *const <T as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Foundation`*"]
pub struct EventRegistrationToken {
    pub Value: i64,
}
impl EventRegistrationToken {}
impl ::core::default::Default for EventRegistrationToken {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for EventRegistrationToken {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("EventRegistrationToken").field("Value", &self.Value).finish()
    }
}
impl ::core::cmp::PartialEq for EventRegistrationToken {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for EventRegistrationToken {}
unsafe impl ::windows::core::Abi for EventRegistrationToken {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for EventRegistrationToken {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.EventRegistrationToken;i8)");
}
impl ::windows::core::DefaultType for EventRegistrationToken {
    type DefaultType = Self;
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct FoundationContract(pub u8);
#[doc = "*Required features: `Foundation`*"]
pub struct GuidHelper {}
impl GuidHelper {
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateNewGuid() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Empty() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Equals<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(target: Param0, value: Param1) -> ::windows::core::Result<bool> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &target.into_param().abi(), &value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IGuidHelperStatics<R, F: FnOnce(&IGuidHelperStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GuidHelper, IGuidHelperStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for GuidHelper {
    const NAME: &'static str = "Windows.Foundation.GuidHelper";
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IAsyncAction(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAsyncAction {
    type Vtable = IAsyncAction_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5a648006_843a_4da9_865b_9d26e5dfad7b);
}
impl IAsyncAction {
    #[doc = "*Required features: `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::core::IntoParam<'a, AsyncActionCompletedHandler>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Completed(&self) -> ::windows::core::Result<AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AsyncActionCompletedHandler>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Status(&self) -> ::windows::core::Result<AsyncStatus> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: AsyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::core::Result<()> {
        if self.Status()? == AsyncStatus::Started {
            let (waiter, signaler) = ::windows::core::Waiter::new();
            self.SetCompleted(AsyncActionCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
unsafe impl ::windows::core::RuntimeType for IAsyncAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{5a648006-843a-4da9-865b-9d26e5dfad7b}");
}
#[cfg(feature = "std")]
impl ::std::future::Future for IAsyncAction {
    type Output = ::windows::core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(AsyncActionCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
impl ::core::convert::From<IAsyncAction> for ::windows::core::IUnknown {
    fn from(value: IAsyncAction) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAsyncAction> for ::windows::core::IUnknown {
    fn from(value: &IAsyncAction) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAsyncAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAsyncAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAsyncAction> for ::windows::core::IInspectable {
    fn from(value: IAsyncAction) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAsyncAction> for ::windows::core::IInspectable {
    fn from(value: &IAsyncAction) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAsyncAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAsyncAction {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IAsyncAction> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: IAsyncAction) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IAsyncAction> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAsyncAction) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAsyncInfo> for IAsyncAction {
    fn into_param(self) -> ::windows::core::Param<'a, IAsyncInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IAsyncInfo> for &IAsyncAction {
    fn into_param(self) -> ::windows::core::Param<'a, IAsyncInfo> {
        ::core::convert::TryInto::<IAsyncInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for IAsyncAction {}
unsafe impl ::core::marker::Sync for IAsyncAction {}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncAction_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IAsyncActionWithProgress<TProgress>(pub ::windows::core::IInspectable, ::core::marker::PhantomData<TProgress>)
where
    TProgress: ::windows::core::RuntimeType + 'static;
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for IAsyncActionWithProgress<TProgress> {
    type Vtable = IAsyncActionWithProgress_abi<TProgress>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<IAsyncActionWithProgress<TProgress> as ::windows::core::RuntimeType>::SIGNATURE);
}
impl<TProgress: ::windows::core::RuntimeType + 'static> IAsyncActionWithProgress<TProgress> {
    #[doc = "*Required features: `Foundation`*"]
    pub fn SetProgress<'a, Param0: ::windows::core::IntoParam<'a, AsyncActionProgressHandler<TProgress>>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Progress(&self) -> ::windows::core::Result<AsyncActionProgressHandler<TProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AsyncActionProgressHandler<TProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::core::IntoParam<'a, AsyncActionWithProgressCompletedHandler<TProgress>>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Completed(&self) -> ::windows::core::Result<AsyncActionWithProgressCompletedHandler<TProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AsyncActionWithProgressCompletedHandler<TProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Status(&self) -> ::windows::core::Result<AsyncStatus> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: AsyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::core::Result<()> {
        if self.Status()? == AsyncStatus::Started {
            let (waiter, signaler) = ::windows::core::Waiter::new();
            self.SetCompleted(AsyncActionWithProgressCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for IAsyncActionWithProgress<TProgress> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{1f6db258-e803-48a1-9546-eb7353398884}").push_slice(b";").push_other(<TProgress as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
#[cfg(feature = "std")]
impl<TProgress: ::windows::core::RuntimeType + 'static> ::std::future::Future for IAsyncActionWithProgress<TProgress> {
    type Output = ::windows::core::Result<()>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(AsyncActionWithProgressCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<IAsyncActionWithProgress<TProgress>> for ::windows::core::IUnknown {
    fn from(value: IAsyncActionWithProgress<TProgress>) -> Self {
        value.0 .0
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IAsyncActionWithProgress<TProgress>> for ::windows::core::IUnknown {
    fn from(value: &IAsyncActionWithProgress<TProgress>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAsyncActionWithProgress<TProgress> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAsyncActionWithProgress<TProgress> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<IAsyncActionWithProgress<TProgress>> for ::windows::core::IInspectable {
    fn from(value: IAsyncActionWithProgress<TProgress>) -> Self {
        value.0
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IAsyncActionWithProgress<TProgress>> for ::windows::core::IInspectable {
    fn from(value: &IAsyncActionWithProgress<TProgress>) -> Self {
        value.0.clone()
    }
}
impl<'a, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAsyncActionWithProgress<TProgress> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAsyncActionWithProgress<TProgress> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<IAsyncActionWithProgress<TProgress>> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: IAsyncActionWithProgress<TProgress>) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IAsyncActionWithProgress<TProgress>> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAsyncActionWithProgress<TProgress>) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, IAsyncInfo> for IAsyncActionWithProgress<TProgress> {
    fn into_param(self) -> ::windows::core::Param<'a, IAsyncInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, IAsyncInfo> for &IAsyncActionWithProgress<TProgress> {
    fn into_param(self) -> ::windows::core::Param<'a, IAsyncInfo> {
        ::core::convert::TryInto::<IAsyncInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::marker::Send for IAsyncActionWithProgress<TProgress> {}
unsafe impl<TProgress: ::windows::core::RuntimeType + 'static> ::core::marker::Sync for IAsyncActionWithProgress<TProgress> {}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncActionWithProgress_abi<TProgress>(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ::core::marker::PhantomData<TProgress>,
)
where
    TProgress: ::windows::core::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IAsyncInfo(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IAsyncInfo {
    type Vtable = IAsyncInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000036_0000_0000_c000_000000000046);
}
impl IAsyncInfo {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Status(&self) -> ::windows::core::Result<AsyncStatus> {
        let this = self;
        unsafe {
            let mut result__: AsyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IAsyncInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{00000036-0000-0000-c000-000000000046}");
}
impl ::core::convert::From<IAsyncInfo> for ::windows::core::IUnknown {
    fn from(value: IAsyncInfo) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IAsyncInfo> for ::windows::core::IUnknown {
    fn from(value: &IAsyncInfo) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAsyncInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAsyncInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IAsyncInfo> for ::windows::core::IInspectable {
    fn from(value: IAsyncInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IAsyncInfo> for ::windows::core::IInspectable {
    fn from(value: &IAsyncInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAsyncInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAsyncInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut AsyncStatus) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IAsyncOperation<TResult>(pub ::windows::core::IInspectable, ::core::marker::PhantomData<TResult>)
where
    TResult: ::windows::core::RuntimeType + 'static;
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for IAsyncOperation<TResult> {
    type Vtable = IAsyncOperation_abi<TResult>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<IAsyncOperation<TResult> as ::windows::core::RuntimeType>::SIGNATURE);
}
impl<TResult: ::windows::core::RuntimeType + 'static> IAsyncOperation<TResult> {
    #[doc = "*Required features: `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::core::IntoParam<'a, AsyncOperationCompletedHandler<TResult>>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Completed(&self) -> ::windows::core::Result<AsyncOperationCompletedHandler<TResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AsyncOperationCompletedHandler<TResult>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::core::Result<TResult> {
        let this = self;
        unsafe {
            let mut result__: <TResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TResult>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Status(&self) -> ::windows::core::Result<AsyncStatus> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: AsyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::core::Result<TResult> {
        if self.Status()? == AsyncStatus::Started {
            let (waiter, signaler) = ::windows::core::Waiter::new();
            self.SetCompleted(AsyncOperationCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for IAsyncOperation<TResult> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9fc2b0bb-e446-44e2-aa61-9cab8f636af2}").push_slice(b";").push_other(<TResult as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
#[cfg(feature = "std")]
impl<TResult: ::windows::core::RuntimeType + 'static> ::std::future::Future for IAsyncOperation<TResult> {
    type Output = ::windows::core::Result<TResult>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(AsyncOperationCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::convert::From<IAsyncOperation<TResult>> for ::windows::core::IUnknown {
    fn from(value: IAsyncOperation<TResult>) -> Self {
        value.0 .0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IAsyncOperation<TResult>> for ::windows::core::IUnknown {
    fn from(value: &IAsyncOperation<TResult>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAsyncOperation<TResult> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAsyncOperation<TResult> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::convert::From<IAsyncOperation<TResult>> for ::windows::core::IInspectable {
    fn from(value: IAsyncOperation<TResult>) -> Self {
        value.0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IAsyncOperation<TResult>> for ::windows::core::IInspectable {
    fn from(value: &IAsyncOperation<TResult>) -> Self {
        value.0.clone()
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAsyncOperation<TResult> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAsyncOperation<TResult> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<IAsyncOperation<TResult>> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: IAsyncOperation<TResult>) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IAsyncOperation<TResult>> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAsyncOperation<TResult>) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, IAsyncInfo> for IAsyncOperation<TResult> {
    fn into_param(self) -> ::windows::core::Param<'a, IAsyncInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, IAsyncInfo> for &IAsyncOperation<TResult> {
    fn into_param(self) -> ::windows::core::Param<'a, IAsyncInfo> {
        ::core::convert::TryInto::<IAsyncInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::core::marker::Send for IAsyncOperation<TResult> {}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static> ::core::marker::Sync for IAsyncOperation<TResult> {}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncOperation_abi<TResult>(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub ::core::marker::PhantomData<TResult>,
)
where
    TResult: ::windows::core::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IAsyncOperationWithProgress<TResult, TProgress>(pub ::windows::core::IInspectable, ::core::marker::PhantomData<TResult>, ::core::marker::PhantomData<TProgress>)
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static;
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for IAsyncOperationWithProgress<TResult, TProgress> {
    type Vtable = IAsyncOperationWithProgress_abi<TResult, TProgress>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<IAsyncOperationWithProgress<TResult, TProgress> as ::windows::core::RuntimeType>::SIGNATURE);
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> IAsyncOperationWithProgress<TResult, TProgress> {
    #[doc = "*Required features: `Foundation`*"]
    pub fn SetProgress<'a, Param0: ::windows::core::IntoParam<'a, AsyncOperationProgressHandler<TResult, TProgress>>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Progress(&self) -> ::windows::core::Result<AsyncOperationProgressHandler<TResult, TProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AsyncOperationProgressHandler<TResult, TProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::core::IntoParam<'a, AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>>(&self, handler: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Completed(&self) -> ::windows::core::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::core::Result<TResult> {
        let this = self;
        unsafe {
            let mut result__: <TResult as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TResult>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Status(&self) -> ::windows::core::Result<AsyncStatus> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: AsyncStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::core::HRESULT = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::core::Result<TResult> {
        if self.Status()? == AsyncStatus::Started {
            let (waiter, signaler) = ::windows::core::Waiter::new();
            self.SetCompleted(AsyncOperationWithProgressCompletedHandler::new(move |_sender, _args| {
                unsafe {
                    signaler.signal();
                }
                Ok(())
            }))?;
        }
        self.GetResults()
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for IAsyncOperationWithProgress<TResult, TProgress> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{b5d036d7-e297-498f-ba60-0289e76e23dd}").push_slice(b";").push_other(<TResult as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<TProgress as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
#[cfg(feature = "std")]
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::std::future::Future for IAsyncOperationWithProgress<TResult, TProgress> {
    type Output = ::windows::core::Result<TResult>;
    fn poll(self: ::std::pin::Pin<&mut Self>, context: &mut ::std::task::Context) -> ::std::task::Poll<Self::Output> {
        if self.Status()? == AsyncStatus::Started {
            let waker = context.waker().clone();
            let _ = self.SetCompleted(AsyncOperationWithProgressCompletedHandler::new(move |_sender, _args| {
                waker.wake_by_ref();
                Ok(())
            }));
            ::std::task::Poll::Pending
        } else {
            ::std::task::Poll::Ready(self.GetResults())
        }
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<IAsyncOperationWithProgress<TResult, TProgress>> for ::windows::core::IUnknown {
    fn from(value: IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
        value.0 .0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IAsyncOperationWithProgress<TResult, TProgress>> for ::windows::core::IUnknown {
    fn from(value: &IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAsyncOperationWithProgress<TResult, TProgress> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAsyncOperationWithProgress<TResult, TProgress> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<IAsyncOperationWithProgress<TResult, TProgress>> for ::windows::core::IInspectable {
    fn from(value: IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
        value.0
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IAsyncOperationWithProgress<TResult, TProgress>> for ::windows::core::IInspectable {
    fn from(value: &IAsyncOperationWithProgress<TResult, TProgress>) -> Self {
        value.0.clone()
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAsyncOperationWithProgress<TResult, TProgress> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAsyncOperationWithProgress<TResult, TProgress> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<IAsyncOperationWithProgress<TResult, TProgress>> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: IAsyncOperationWithProgress<TResult, TProgress>) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IAsyncOperationWithProgress<TResult, TProgress>> for IAsyncInfo {
    type Error = ::windows::core::Error;
    fn try_from(value: &IAsyncOperationWithProgress<TResult, TProgress>) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, IAsyncInfo> for IAsyncOperationWithProgress<TResult, TProgress> {
    fn into_param(self) -> ::windows::core::Param<'a, IAsyncInfo> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a, TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, IAsyncInfo> for &IAsyncOperationWithProgress<TResult, TProgress> {
    fn into_param(self) -> ::windows::core::Param<'a, IAsyncInfo> {
        ::core::convert::TryInto::<IAsyncInfo>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::marker::Send for IAsyncOperationWithProgress<TResult, TProgress> {}
unsafe impl<TResult: ::windows::core::RuntimeType + 'static, TProgress: ::windows::core::RuntimeType + 'static> ::core::marker::Sync for IAsyncOperationWithProgress<TResult, TProgress> {}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncOperationWithProgress_abi<TResult, TProgress>(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub ::core::marker::PhantomData<TResult>,
    pub ::core::marker::PhantomData<TProgress>,
)
where
    TResult: ::windows::core::RuntimeType + 'static,
    TProgress: ::windows::core::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IClosable(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IClosable {
    type Vtable = IClosable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30d5a829_7fa4_4026_83bb_d75bae4ea99e);
}
impl IClosable {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IClosable {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{30d5a829-7fa4-4026-83bb-d75bae4ea99e}");
}
impl ::core::convert::From<IClosable> for ::windows::core::IUnknown {
    fn from(value: IClosable) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IClosable> for ::windows::core::IUnknown {
    fn from(value: &IClosable) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IClosable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IClosable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IClosable> for ::windows::core::IInspectable {
    fn from(value: IClosable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IClosable> for ::windows::core::IInspectable {
    fn from(value: &IClosable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IClosable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IClosable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosable_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeferral(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeferral {
    type Vtable = IDeferral_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6269732_3b7f_46a7_b40b_4fdca2a2c693);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDeferralFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IDeferralFactory {
    type Vtable = IDeferralFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x65a1ecc5_3fb5_4832_8ca9_f061b281d13a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeferralFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IGetActivationFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGetActivationFactory {
    type Vtable = IGetActivationFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4edb8ee2_96dd_49a7_94f7_4607ddab8e3c);
}
impl IGetActivationFactory {
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetActivationFactory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, activatableclassid: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), activatableclassid.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IGetActivationFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4edb8ee2-96dd-49a7-94f7-4607ddab8e3c}");
}
impl ::core::convert::From<IGetActivationFactory> for ::windows::core::IUnknown {
    fn from(value: IGetActivationFactory) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IGetActivationFactory> for ::windows::core::IUnknown {
    fn from(value: &IGetActivationFactory) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGetActivationFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGetActivationFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IGetActivationFactory> for ::windows::core::IInspectable {
    fn from(value: IGetActivationFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGetActivationFactory> for ::windows::core::IInspectable {
    fn from(value: &IGetActivationFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGetActivationFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGetActivationFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetActivationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, activatableclassid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IGuidHelperStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IGuidHelperStatics {
    type Vtable = IGuidHelperStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x59c7966b_ae52_5283_ad7f_a1b9e9678add);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, target: &::windows::core::GUID, value: &::windows::core::GUID, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IMemoryBuffer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMemoryBuffer {
    type Vtable = IMemoryBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc4dd2a_245b_11e4_af98_689423260cf8);
}
impl IMemoryBuffer {
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::core::Result<IMemoryBufferReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IMemoryBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fbc4dd2a-245b-11e4-af98-689423260cf8}");
}
impl ::core::convert::From<IMemoryBuffer> for ::windows::core::IUnknown {
    fn from(value: IMemoryBuffer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IMemoryBuffer> for ::windows::core::IUnknown {
    fn from(value: &IMemoryBuffer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IMemoryBuffer> for ::windows::core::IInspectable {
    fn from(value: IMemoryBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMemoryBuffer> for ::windows::core::IInspectable {
    fn from(value: &IMemoryBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IMemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IMemoryBuffer> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IMemoryBuffer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IMemoryBuffer> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IMemoryBuffer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IClosable> for IMemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IClosable> for &IMemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IClosable> {
        ::core::convert::TryInto::<IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMemoryBufferFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMemoryBufferFactory {
    type Vtable = IMemoryBufferFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc4dd2b_245b_11e4_af98_689423260cf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBufferFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, capacity: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IMemoryBufferReference(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMemoryBufferReference {
    type Vtable = IMemoryBufferReference_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc4dd29_245b_11e4_af98_689423260cf8);
}
impl IMemoryBufferReference {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Capacity(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::core::IntoParam<'a, TypedEventHandler<IMemoryBufferReference, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::core::IntoParam<'a, EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IMemoryBufferReference {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fbc4dd29-245b-11e4-af98-689423260cf8}");
}
impl ::core::convert::From<IMemoryBufferReference> for ::windows::core::IUnknown {
    fn from(value: IMemoryBufferReference) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IMemoryBufferReference> for ::windows::core::IUnknown {
    fn from(value: &IMemoryBufferReference) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMemoryBufferReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMemoryBufferReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IMemoryBufferReference> for ::windows::core::IInspectable {
    fn from(value: IMemoryBufferReference) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IMemoryBufferReference> for ::windows::core::IInspectable {
    fn from(value: &IMemoryBufferReference) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMemoryBufferReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IMemoryBufferReference {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IMemoryBufferReference> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IMemoryBufferReference) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IMemoryBufferReference> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IMemoryBufferReference) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IClosable> for IMemoryBufferReference {
    fn into_param(self) -> ::windows::core::Param<'a, IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IClosable> for &IMemoryBufferReference {
    fn into_param(self) -> ::windows::core::Param<'a, IClosable> {
        ::core::convert::TryInto::<IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBufferReference_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut EventRegistrationToken) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: EventRegistrationToken) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IPropertyValue(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPropertyValue {
    type Vtable = IPropertyValue_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bd682dd_7554_40e9_9a9b_82654ede7e62);
}
impl IPropertyValue {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Type(&self) -> ::windows::core::Result<PropertyType> {
        let this = self;
        unsafe {
            let mut result__: PropertyType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyType>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn IsNumericScalar(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt8(&self) -> ::windows::core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt16(&self) -> ::windows::core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt16(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt32(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt32(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt64(&self) -> ::windows::core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt64(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSingle(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDouble(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetChar16(&self) -> ::windows::core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDateTime(&self) -> ::windows::core::Result<DateTime> {
        let this = self;
        unsafe {
            let mut result__: DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetTimeSpan(&self) -> ::windows::core::Result<TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetPoint(&self) -> ::windows::core::Result<Point> {
        let this = self;
        unsafe {
            let mut result__: Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Point>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSize(&self) -> ::windows::core::Result<Size> {
        let this = self;
        unsafe {
            let mut result__: Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Size>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetRect(&self) -> ::windows::core::Result<Rect> {
        let this = self;
        unsafe {
            let mut result__: Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Rect>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt8Array(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt16Array(&self, value: &mut ::windows::core::Array<i16>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt32Array(&self, value: &mut ::windows::core::Array<i32>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt32Array(&self, value: &mut ::windows::core::Array<u32>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt64Array(&self, value: &mut ::windows::core::Array<i64>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt64Array(&self, value: &mut ::windows::core::Array<u64>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSingleArray(&self, value: &mut ::windows::core::Array<f32>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDoubleArray(&self, value: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetChar16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetBooleanArray(&self, value: &mut ::windows::core::Array<bool>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetStringArray(&self, value: &mut ::windows::core::Array<::windows::core::HSTRING>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInspectableArray(&self, value: &mut ::windows::core::Array<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetGuidArray(&self, value: &mut ::windows::core::Array<::windows::core::GUID>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDateTimeArray(&self, value: &mut ::windows::core::Array<DateTime>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetTimeSpanArray(&self, value: &mut ::windows::core::Array<TimeSpan>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetPointArray(&self, value: &mut ::windows::core::Array<Point>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSizeArray(&self, value: &mut ::windows::core::Array<Size>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetRectArray(&self, value: &mut ::windows::core::Array<Rect>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IPropertyValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4bd682dd-7554-40e9-9a9b-82654ede7e62}");
}
impl ::core::convert::From<IPropertyValue> for ::windows::core::IUnknown {
    fn from(value: IPropertyValue) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IPropertyValue> for ::windows::core::IUnknown {
    fn from(value: &IPropertyValue) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPropertyValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPropertyValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IPropertyValue> for ::windows::core::IInspectable {
    fn from(value: IPropertyValue) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPropertyValue> for ::windows::core::IInspectable {
    fn from(value: &IPropertyValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IPropertyValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IPropertyValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyValue_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PropertyType) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut DateTime) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut TimeSpan) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut Point) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut Size) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut Rect) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut i16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut DateTime) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut TimeSpan) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut Point) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut Size) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: *mut u32, value: *mut *mut Rect) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPropertyValueStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPropertyValueStatics {
    type Vtable = IPropertyValueStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x629bdbc8_d932_4ff4_96b9_8d96c5c1e858);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: i64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const u8, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const i16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const i64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const f64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const u16, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const bool, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const ::windows::core::GUID, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const Size, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value_array_size: u32, value: *const Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IReference<T>(pub ::windows::core::IInspectable, ::core::marker::PhantomData<T>)
where
    T: ::windows::core::RuntimeType + 'static;
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for IReference<T> {
    type Vtable = IReference_abi<T>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<IReference<T> as ::windows::core::RuntimeType>::SIGNATURE);
}
impl<T: ::windows::core::RuntimeType + 'static> IReference<T> {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Value(&self) -> ::windows::core::Result<T> {
        let this = self;
        unsafe {
            let mut result__: <T as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<T>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Type(&self) -> ::windows::core::Result<PropertyType> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: PropertyType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyType>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn IsNumericScalar(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt8(&self) -> ::windows::core::Result<u8> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt16(&self) -> ::windows::core::Result<i16> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt16(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt32(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt32(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt64(&self) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt64(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSingle(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDouble(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetChar16(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDateTime(&self) -> ::windows::core::Result<DateTime> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetTimeSpan(&self) -> ::windows::core::Result<TimeSpan> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetPoint(&self) -> ::windows::core::Result<Point> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Point>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSize(&self) -> ::windows::core::Result<Size> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Size>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetRect(&self) -> ::windows::core::Result<Rect> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Rect>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt8Array(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt16Array(&self, value: &mut ::windows::core::Array<i16>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt32Array(&self, value: &mut ::windows::core::Array<i32>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt32Array(&self, value: &mut ::windows::core::Array<u32>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt64Array(&self, value: &mut ::windows::core::Array<i64>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt64Array(&self, value: &mut ::windows::core::Array<u64>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSingleArray(&self, value: &mut ::windows::core::Array<f32>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDoubleArray(&self, value: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetChar16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetBooleanArray(&self, value: &mut ::windows::core::Array<bool>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetStringArray(&self, value: &mut ::windows::core::Array<::windows::core::HSTRING>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInspectableArray(&self, value: &mut ::windows::core::Array<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetGuidArray(&self, value: &mut ::windows::core::Array<::windows::core::GUID>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDateTimeArray(&self, value: &mut ::windows::core::Array<DateTime>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetTimeSpanArray(&self, value: &mut ::windows::core::Array<TimeSpan>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetPointArray(&self, value: &mut ::windows::core::Array<Point>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSizeArray(&self, value: &mut ::windows::core::Array<Size>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetRectArray(&self, value: &mut ::windows::core::Array<Rect>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for IReference<T> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{61c17706-2d65-11e0-9ae8-d48564015472}").push_slice(b";").push_other(<T as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<IReference<T>> for ::windows::core::IUnknown {
    fn from(value: IReference<T>) -> Self {
        value.0 .0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IReference<T>> for ::windows::core::IUnknown {
    fn from(value: &IReference<T>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReference<T> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReference<T> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<IReference<T>> for ::windows::core::IInspectable {
    fn from(value: IReference<T>) -> Self {
        value.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IReference<T>> for ::windows::core::IInspectable {
    fn from(value: &IReference<T>) -> Self {
        value.0.clone()
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IReference<T> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IReference<T> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<IReference<T>> for IPropertyValue {
    type Error = ::windows::core::Error;
    fn try_from(value: IReference<T>) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IReference<T>> for IPropertyValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &IReference<T>) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, IPropertyValue> for IReference<T> {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, IPropertyValue> for &IReference<T> {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyValue> {
        ::core::convert::TryInto::<IPropertyValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReference_abi<T>(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub ::core::marker::PhantomData<T>,
)
where
    T: ::windows::core::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IReferenceArray<T>(pub ::windows::core::IInspectable, ::core::marker::PhantomData<T>)
where
    T: ::windows::core::RuntimeType + 'static;
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for IReferenceArray<T> {
    type Vtable = IReferenceArray_abi<T>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<IReferenceArray<T> as ::windows::core::RuntimeType>::SIGNATURE);
}
impl<T: ::windows::core::RuntimeType + 'static> IReferenceArray<T> {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::Array<T>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<T> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::windows::core::Array::<T>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Type(&self) -> ::windows::core::Result<PropertyType> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: PropertyType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PropertyType>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn IsNumericScalar(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt8(&self) -> ::windows::core::Result<u8> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u8 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt16(&self) -> ::windows::core::Result<i16> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt16(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt32(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt32(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt64(&self) -> ::windows::core::Result<i64> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt64(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSingle(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDouble(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetChar16(&self) -> ::windows::core::Result<u16> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u16 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetBoolean(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDateTime(&self) -> ::windows::core::Result<DateTime> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: DateTime = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetTimeSpan(&self) -> ::windows::core::Result<TimeSpan> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), &mut result__).from_abi::<TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetPoint(&self) -> ::windows::core::Result<Point> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Point>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSize(&self) -> ::windows::core::Result<Size> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Size>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetRect(&self) -> ::windows::core::Result<Rect> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Rect>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt8Array(&self, value: &mut ::windows::core::Array<u8>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt16Array(&self, value: &mut ::windows::core::Array<i16>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt32Array(&self, value: &mut ::windows::core::Array<i32>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt32Array(&self, value: &mut ::windows::core::Array<u32>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt64Array(&self, value: &mut ::windows::core::Array<i64>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt64Array(&self, value: &mut ::windows::core::Array<u64>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSingleArray(&self, value: &mut ::windows::core::Array<f32>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDoubleArray(&self, value: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetChar16Array(&self, value: &mut ::windows::core::Array<u16>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetBooleanArray(&self, value: &mut ::windows::core::Array<bool>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetStringArray(&self, value: &mut ::windows::core::Array<::windows::core::HSTRING>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInspectableArray(&self, value: &mut ::windows::core::Array<::windows::core::IInspectable>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetGuidArray(&self, value: &mut ::windows::core::Array<::windows::core::GUID>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDateTimeArray(&self, value: &mut ::windows::core::Array<DateTime>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetTimeSpanArray(&self, value: &mut ::windows::core::Array<TimeSpan>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetPointArray(&self, value: &mut ::windows::core::Array<Point>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSizeArray(&self, value: &mut ::windows::core::Array<Size>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetRectArray(&self, value: &mut ::windows::core::Array<Rect>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
unsafe impl<T: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for IReferenceArray<T> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{61c17707-2d65-11e0-9ae8-d48564015472}").push_slice(b";").push_other(<T as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<IReferenceArray<T>> for ::windows::core::IUnknown {
    fn from(value: IReferenceArray<T>) -> Self {
        value.0 .0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IReferenceArray<T>> for ::windows::core::IUnknown {
    fn from(value: &IReferenceArray<T>) -> Self {
        value.0 .0.clone()
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IReferenceArray<T> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IReferenceArray<T> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<IReferenceArray<T>> for ::windows::core::IInspectable {
    fn from(value: IReferenceArray<T>) -> Self {
        value.0
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::From<&IReferenceArray<T>> for ::windows::core::IInspectable {
    fn from(value: &IReferenceArray<T>) -> Self {
        value.0.clone()
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IReferenceArray<T> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IReferenceArray<T> {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<IReferenceArray<T>> for IPropertyValue {
    type Error = ::windows::core::Error;
    fn try_from(value: IReferenceArray<T>) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl<T: ::windows::core::RuntimeType + 'static> ::core::convert::TryFrom<&IReferenceArray<T>> for IPropertyValue {
    type Error = ::windows::core::Error;
    fn try_from(value: &IReferenceArray<T>) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, IPropertyValue> for IReferenceArray<T> {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyValue> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a, T: ::windows::core::RuntimeType + 'static> ::windows::core::IntoParam<'a, IPropertyValue> for &IReferenceArray<T> {
    fn into_param(self) -> ::windows::core::Param<'a, IPropertyValue> {
        ::core::convert::TryInto::<IPropertyValue>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceArray_abi<T>(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut <T as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub ::core::marker::PhantomData<T>,
)
where
    T: ::windows::core::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IStringable(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IStringable {
    type Vtable = IStringable_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96369f54_8eb6_48f0_abce_c1b211e627c3);
}
impl IStringable {
    #[doc = "*Required features: `Foundation`*"]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IStringable {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{96369f54-8eb6-48f0-abce-c1b211e627c3}");
}
impl ::core::convert::From<IStringable> for ::windows::core::IUnknown {
    fn from(value: IStringable) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IStringable> for ::windows::core::IUnknown {
    fn from(value: &IStringable) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStringable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStringable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IStringable> for ::windows::core::IInspectable {
    fn from(value: IStringable) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IStringable> for ::windows::core::IInspectable {
    fn from(value: &IStringable) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStringable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStringable {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStringable_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUriEscapeStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUriEscapeStatics {
    type Vtable = IUriEscapeStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1d432ba_c824_4452_a7fd_512bc3bbe9a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriEscapeStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tounescape: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, toescape: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUriRuntimeClass(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUriRuntimeClass {
    type Vtable = IUriRuntimeClass_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e365e57_48b2_4160_956f_c7385120bbfc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClass_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, puri: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, relativeuri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUriRuntimeClassFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUriRuntimeClassFactory {
    type Vtable = IUriRuntimeClassFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44a9796f_723e_4fdf_a218_033e75b0c084);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClassFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, baseuri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, relativeuri: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUriRuntimeClassWithAbsoluteCanonicalUri {
    type Vtable = IUriRuntimeClassWithAbsoluteCanonicalUri_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x758d9661_221c_480f_a339_50656673f46f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IWwwFormUrlDecoderEntry(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWwwFormUrlDecoderEntry {
    type Vtable = IWwwFormUrlDecoderEntry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x125e7431_f678_4e8e_b670_20a9b06c512d);
}
impl IWwwFormUrlDecoderEntry {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IWwwFormUrlDecoderEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{125e7431-f678-4e8e-b670-20a9b06c512d}");
}
impl ::core::convert::From<IWwwFormUrlDecoderEntry> for ::windows::core::IUnknown {
    fn from(value: IWwwFormUrlDecoderEntry) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IWwwFormUrlDecoderEntry> for ::windows::core::IUnknown {
    fn from(value: &IWwwFormUrlDecoderEntry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWwwFormUrlDecoderEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWwwFormUrlDecoderEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IWwwFormUrlDecoderEntry> for ::windows::core::IInspectable {
    fn from(value: IWwwFormUrlDecoderEntry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWwwFormUrlDecoderEntry> for ::windows::core::IInspectable {
    fn from(value: &IWwwFormUrlDecoderEntry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWwwFormUrlDecoderEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWwwFormUrlDecoderEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderEntry_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderRuntimeClass(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWwwFormUrlDecoderRuntimeClass {
    type Vtable = IWwwFormUrlDecoderRuntimeClass_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd45a0451_f225_4542_9296_0e1df5d254df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderRuntimeClass_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderRuntimeClassFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IWwwFormUrlDecoderRuntimeClassFactory {
    type Vtable = IWwwFormUrlDecoderRuntimeClassFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b8c6b3d_24ae_41b5_a1bf_f0c3d544845b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderRuntimeClassFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, query: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MemoryBuffer(pub ::windows::core::IInspectable);
impl MemoryBuffer {
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::core::Result<IMemoryBufferReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Create(capacity: u32) -> ::windows::core::Result<MemoryBuffer> {
        Self::IMemoryBufferFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), capacity, &mut result__).from_abi::<MemoryBuffer>(result__)
        })
    }
    pub fn IMemoryBufferFactory<R, F: FnOnce(&IMemoryBufferFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MemoryBuffer, IMemoryBufferFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MemoryBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.MemoryBuffer;{fbc4dd2a-245b-11e4-af98-689423260cf8})");
}
unsafe impl ::windows::core::Interface for MemoryBuffer {
    type Vtable = IMemoryBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbc4dd2a_245b_11e4_af98_689423260cf8);
}
impl ::windows::core::RuntimeName for MemoryBuffer {
    const NAME: &'static str = "Windows.Foundation.MemoryBuffer";
}
impl ::core::convert::From<MemoryBuffer> for ::windows::core::IUnknown {
    fn from(value: MemoryBuffer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MemoryBuffer> for ::windows::core::IUnknown {
    fn from(value: &MemoryBuffer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MemoryBuffer> for ::windows::core::IInspectable {
    fn from(value: MemoryBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MemoryBuffer> for ::windows::core::IInspectable {
    fn from(value: &MemoryBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<MemoryBuffer> for IMemoryBuffer {
    fn from(value: MemoryBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MemoryBuffer> for IMemoryBuffer {
    fn from(value: &MemoryBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMemoryBuffer> for MemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IMemoryBuffer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IMemoryBuffer> for &MemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IMemoryBuffer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<MemoryBuffer> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: MemoryBuffer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&MemoryBuffer> for IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &MemoryBuffer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IClosable> for MemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IClosable> for &MemoryBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, IClosable> {
        ::core::convert::TryInto::<IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for MemoryBuffer {}
unsafe impl ::core::marker::Sync for MemoryBuffer {}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Foundation`*"]
pub struct Point {
    pub X: f32,
    pub Y: f32,
}
impl Point {}
impl ::core::default::Default for Point {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for Point {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("Point").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::core::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for Point {}
unsafe impl ::windows::core::Abi for Point {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Point {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.Point;f4;f4)");
}
impl ::windows::core::DefaultType for Point {
    type DefaultType = Self;
}
#[doc = "*Required features: `Foundation`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PropertyType(pub i32);
impl PropertyType {
    pub const Empty: PropertyType = PropertyType(0i32);
    pub const UInt8: PropertyType = PropertyType(1i32);
    pub const Int16: PropertyType = PropertyType(2i32);
    pub const UInt16: PropertyType = PropertyType(3i32);
    pub const Int32: PropertyType = PropertyType(4i32);
    pub const UInt32: PropertyType = PropertyType(5i32);
    pub const Int64: PropertyType = PropertyType(6i32);
    pub const UInt64: PropertyType = PropertyType(7i32);
    pub const Single: PropertyType = PropertyType(8i32);
    pub const Double: PropertyType = PropertyType(9i32);
    pub const Char16: PropertyType = PropertyType(10i32);
    pub const Boolean: PropertyType = PropertyType(11i32);
    pub const String: PropertyType = PropertyType(12i32);
    pub const Inspectable: PropertyType = PropertyType(13i32);
    pub const DateTime: PropertyType = PropertyType(14i32);
    pub const TimeSpan: PropertyType = PropertyType(15i32);
    pub const Guid: PropertyType = PropertyType(16i32);
    pub const Point: PropertyType = PropertyType(17i32);
    pub const Size: PropertyType = PropertyType(18i32);
    pub const Rect: PropertyType = PropertyType(19i32);
    pub const OtherType: PropertyType = PropertyType(20i32);
    pub const UInt8Array: PropertyType = PropertyType(1025i32);
    pub const Int16Array: PropertyType = PropertyType(1026i32);
    pub const UInt16Array: PropertyType = PropertyType(1027i32);
    pub const Int32Array: PropertyType = PropertyType(1028i32);
    pub const UInt32Array: PropertyType = PropertyType(1029i32);
    pub const Int64Array: PropertyType = PropertyType(1030i32);
    pub const UInt64Array: PropertyType = PropertyType(1031i32);
    pub const SingleArray: PropertyType = PropertyType(1032i32);
    pub const DoubleArray: PropertyType = PropertyType(1033i32);
    pub const Char16Array: PropertyType = PropertyType(1034i32);
    pub const BooleanArray: PropertyType = PropertyType(1035i32);
    pub const StringArray: PropertyType = PropertyType(1036i32);
    pub const InspectableArray: PropertyType = PropertyType(1037i32);
    pub const DateTimeArray: PropertyType = PropertyType(1038i32);
    pub const TimeSpanArray: PropertyType = PropertyType(1039i32);
    pub const GuidArray: PropertyType = PropertyType(1040i32);
    pub const PointArray: PropertyType = PropertyType(1041i32);
    pub const SizeArray: PropertyType = PropertyType(1042i32);
    pub const RectArray: PropertyType = PropertyType(1043i32);
    pub const OtherTypeArray: PropertyType = PropertyType(1044i32);
}
impl ::core::convert::From<i32> for PropertyType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PropertyType {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PropertyType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Foundation.PropertyType;i4)");
}
impl ::windows::core::DefaultType for PropertyType {
    type DefaultType = Self;
}
#[doc = "*Required features: `Foundation`*"]
pub struct PropertyValue {}
impl PropertyValue {
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateEmpty() -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt8(value: u8) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInt16(value: i16) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt16(value: u16) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInt32(value: i32) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt32(value: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInt64(value: i64) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt64(value: u64) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateSingle(value: f32) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateDouble(value: f64) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateChar16(value: u16) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateBoolean(value: bool) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(value: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInspectable<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(value: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateGuid<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(value: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateDateTime<'a, Param0: ::windows::core::IntoParam<'a, DateTime>>(value: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateTimeSpan<'a, Param0: ::windows::core::IntoParam<'a, TimeSpan>>(value: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreatePoint<'a, Param0: ::windows::core::IntoParam<'a, Point>>(value: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).23)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateSize<'a, Param0: ::windows::core::IntoParam<'a, Size>>(value: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).24)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateRect<'a, Param0: ::windows::core::IntoParam<'a, Rect>>(value: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).25)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt8Array(value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).26)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInt16Array(value: &[<i16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).27)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt16Array(value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).28)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInt32Array(value: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).29)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt32Array(value: &[<u32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).30)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInt64Array(value: &[<i64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).31)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt64Array(value: &[<u64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).32)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateSingleArray(value: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).33)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateDoubleArray(value: &[<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).34)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateChar16Array(value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).35)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateBooleanArray(value: &[<bool as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).36)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateStringArray(value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).37)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInspectableArray(value: &[<::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).38)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateGuidArray(value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).39)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateDateTimeArray(value: &[<DateTime as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).40)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateTimeSpanArray(value: &[<TimeSpan as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).41)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreatePointArray(value: &[<Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).42)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateSizeArray(value: &[<Size as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).43)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateRectArray(value: &[<Rect as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<::windows::core::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).44)(::core::mem::transmute_copy(this), value.len() as u32, ::core::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        })
    }
    pub fn IPropertyValueStatics<R, F: FnOnce(&IPropertyValueStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PropertyValue, IPropertyValueStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for PropertyValue {
    const NAME: &'static str = "Windows.Foundation.PropertyValue";
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Foundation`*"]
pub struct Rect {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
impl Rect {}
impl ::core::default::Default for Rect {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for Rect {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("Rect").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::cmp::PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for Rect {}
unsafe impl ::windows::core::Abi for Rect {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Rect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.Rect;f4;f4;f4;f4)");
}
impl ::windows::core::DefaultType for Rect {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Foundation`*"]
pub struct Size {
    pub Width: f32,
    pub Height: f32,
}
impl Size {}
impl ::core::default::Default for Size {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for Size {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("Size").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::cmp::PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for Size {}
unsafe impl ::windows::core::Abi for Size {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Size {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.Size;f4;f4)");
}
impl ::windows::core::DefaultType for Size {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Foundation`*"]
pub struct TimeSpan {
    pub Duration: i64,
}
impl TimeSpan {}
impl ::core::default::Default for TimeSpan {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for TimeSpan {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("TimeSpan").field("Duration", &self.Duration).finish()
    }
}
impl ::core::cmp::PartialEq for TimeSpan {
    fn eq(&self, other: &Self) -> bool {
        self.Duration == other.Duration
    }
}
impl ::core::cmp::Eq for TimeSpan {}
unsafe impl ::windows::core::Abi for TimeSpan {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TimeSpan {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Foundation.TimeSpan;i8)");
}
impl ::windows::core::DefaultType for TimeSpan {
    type DefaultType = Self;
}
impl ::core::convert::From<::core::time::Duration> for TimeSpan {
    fn from(value: ::core::time::Duration) -> Self {
        Self { Duration: (value.as_nanos() / 100) as i64 }
    }
}
impl ::core::convert::From<TimeSpan> for ::core::time::Duration {
    fn from(value: TimeSpan) -> Self {
        ::core::time::Duration::from_nanos((value.Duration * 100) as u64)
    }
}
impl<'a> ::windows::core::IntoParam<'a, TimeSpan> for ::core::time::Duration {
    fn into_param(self) -> ::windows::core::Param<'a, TimeSpan> {
        ::windows::core::Param::Owned(self.into())
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TypedEventHandler<TSender, TResult>(::windows::core::IUnknown, ::core::marker::PhantomData<TSender>, ::core::marker::PhantomData<TResult>)
where
    TSender: ::windows::core::RuntimeType + 'static,
    TResult: ::windows::core::RuntimeType + 'static;
impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> TypedEventHandler<TSender, TResult> {
    pub fn new<F: FnMut(&<TSender as ::windows::core::DefaultType>::DefaultType, &<TResult as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = TypedEventHandler_box::<TSender, TResult, F> {
            vtable: &TypedEventHandler_box::<TSender, TResult, F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, TSender>, Param1: ::windows::core::IntoParam<'a, TResult>>(&self, sender: Param0, args: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
}
unsafe impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> ::windows::core::RuntimeType for TypedEventHandler<TSender, TResult> {
    const SIGNATURE: ::windows::core::ConstBuffer = { ::windows::core::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9de1c534-6ae1-11e0-84e1-18a905bcc53f}").push_slice(b";").push_other(<TSender as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b";").push_other(<TResult as ::windows::core::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static> ::windows::core::Interface for TypedEventHandler<TSender, TResult> {
    type Vtable = TypedEventHandler_abi<TSender, TResult>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<TypedEventHandler<TSender, TResult> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct TypedEventHandler_abi<TSender, TResult>(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sender: <TSender as ::windows::core::Abi>::Abi, args: <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT,
    pub ::core::marker::PhantomData<TSender>,
    pub ::core::marker::PhantomData<TResult>,
)
where
    TSender: ::windows::core::RuntimeType + 'static,
    TResult: ::windows::core::RuntimeType + 'static;
#[repr(C)]
struct TypedEventHandler_box<TSender, TResult, F: FnMut(&<TSender as ::windows::core::DefaultType>::DefaultType, &<TResult as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()> + 'static>
where
    TSender: ::windows::core::RuntimeType + 'static,
    TResult: ::windows::core::RuntimeType + 'static,
{
    vtable: *const TypedEventHandler_abi<TSender, TResult>,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<TSender: ::windows::core::RuntimeType + 'static, TResult: ::windows::core::RuntimeType + 'static, F: FnMut(&<TSender as ::windows::core::DefaultType>::DefaultType, &<TResult as ::windows::core::DefaultType>::DefaultType) -> ::windows::core::Result<()> + 'static> TypedEventHandler_box<TSender, TResult, F> {
    const VTABLE: TypedEventHandler_abi<TSender, TResult> = TypedEventHandler_abi::<TSender, TResult>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::core::marker::PhantomData::<TSender>, ::core::marker::PhantomData::<TResult>);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<TypedEventHandler<TSender, TResult> as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, sender: <TSender as ::windows::core::Abi>::Abi, args: <TResult as ::windows::core::Abi>::Abi) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <TSender as ::windows::core::Abi>::Abi as *const <TSender as ::windows::core::DefaultType>::DefaultType), &*(&args as *const <TResult as ::windows::core::Abi>::Abi as *const <TResult as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[repr(C)]
#[derive(:: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug, :: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy)]
pub struct UniversalApiContract(pub u8);
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct Uri(pub ::windows::core::IInspectable);
impl Uri {
    #[doc = "*Required features: `Foundation`*"]
    pub fn AbsoluteUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn DisplayUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Domain(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Extension(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Fragment(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Host(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Password(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Query(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn QueryParsed(&self) -> ::windows::core::Result<WwwFormUrlDecoder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WwwFormUrlDecoder>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn RawUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn SchemeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn UserName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Port(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Suspicious(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Equals<'a, Param0: ::windows::core::IntoParam<'a, Uri>>(&self, puri: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), puri.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CombineUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, relativeuri: Param0) -> ::windows::core::Result<Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), relativeuri.into_param().abi(), &mut result__).from_abi::<Uri>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn ToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IStringable>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn AbsoluteCanonicalUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn DisplayIri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn UnescapeComponent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(tounescape: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), tounescape.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn EscapeComponent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(toescape: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), toescape.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(uri: Param0) -> ::windows::core::Result<Uri> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<Uri>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateWithRelativeUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(baseuri: Param0, relativeuri: Param1) -> ::windows::core::Result<Uri> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), baseuri.into_param().abi(), relativeuri.into_param().abi(), &mut result__).from_abi::<Uri>(result__)
        })
    }
    pub fn IUriEscapeStatics<R, F: FnOnce(&IUriEscapeStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Uri, IUriEscapeStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUriRuntimeClassFactory<R, F: FnOnce(&IUriRuntimeClassFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Uri, IUriRuntimeClassFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for Uri {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.Uri;{9e365e57-48b2-4160-956f-c7385120bbfc})");
}
unsafe impl ::windows::core::Interface for Uri {
    type Vtable = IUriRuntimeClass_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e365e57_48b2_4160_956f_c7385120bbfc);
}
impl ::windows::core::RuntimeName for Uri {
    const NAME: &'static str = "Windows.Foundation.Uri";
}
impl ::core::convert::From<Uri> for ::windows::core::IUnknown {
    fn from(value: Uri) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&Uri> for ::windows::core::IUnknown {
    fn from(value: &Uri) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Uri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Uri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<Uri> for ::windows::core::IInspectable {
    fn from(value: Uri) -> Self {
        value.0
    }
}
impl ::core::convert::From<&Uri> for ::windows::core::IInspectable {
    fn from(value: &Uri) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Uri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Uri {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<Uri> for IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: Uri) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&Uri> for IStringable {
    type Error = ::windows::core::Error;
    fn try_from(value: &Uri) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStringable> for Uri {
    fn into_param(self) -> ::windows::core::Param<'a, IStringable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStringable> for &Uri {
    fn into_param(self) -> ::windows::core::Param<'a, IStringable> {
        ::core::convert::TryInto::<IStringable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for Uri {}
unsafe impl ::core::marker::Sync for Uri {}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WwwFormUrlDecoder(pub ::windows::core::IInspectable);
impl WwwFormUrlDecoder {
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetFirstValueByName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Foundation`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::core::Result<Collections::IIterator<IWwwFormUrlDecoderEntry>> {
        let this = &::windows::core::Interface::cast::<Collections::IIterable<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Collections::IIterator<IWwwFormUrlDecoderEntry>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Foundation`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<IWwwFormUrlDecoderEntry> {
        let this = &::windows::core::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<IWwwFormUrlDecoderEntry>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Foundation`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Foundation`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, IWwwFormUrlDecoderEntry>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Foundation`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<IWwwFormUrlDecoderEntry as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateWwwFormUrlDecoder<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(query: Param0) -> ::windows::core::Result<WwwFormUrlDecoder> {
        Self::IWwwFormUrlDecoderRuntimeClassFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), query.into_param().abi(), &mut result__).from_abi::<WwwFormUrlDecoder>(result__)
        })
    }
    pub fn IWwwFormUrlDecoderRuntimeClassFactory<R, F: FnOnce(&IWwwFormUrlDecoderRuntimeClassFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WwwFormUrlDecoder, IWwwFormUrlDecoderRuntimeClassFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for WwwFormUrlDecoder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.WwwFormUrlDecoder;{d45a0451-f225-4542-9296-0e1df5d254df})");
}
unsafe impl ::windows::core::Interface for WwwFormUrlDecoder {
    type Vtable = IWwwFormUrlDecoderRuntimeClass_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd45a0451_f225_4542_9296_0e1df5d254df);
}
impl ::windows::core::RuntimeName for WwwFormUrlDecoder {
    const NAME: &'static str = "Windows.Foundation.WwwFormUrlDecoder";
}
impl ::core::convert::From<WwwFormUrlDecoder> for ::windows::core::IUnknown {
    fn from(value: WwwFormUrlDecoder) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WwwFormUrlDecoder> for ::windows::core::IUnknown {
    fn from(value: &WwwFormUrlDecoder) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WwwFormUrlDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WwwFormUrlDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WwwFormUrlDecoder> for ::windows::core::IInspectable {
    fn from(value: WwwFormUrlDecoder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WwwFormUrlDecoder> for ::windows::core::IInspectable {
    fn from(value: &WwwFormUrlDecoder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WwwFormUrlDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WwwFormUrlDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<WwwFormUrlDecoder> for Collections::IIterable<IWwwFormUrlDecoderEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: WwwFormUrlDecoder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&WwwFormUrlDecoder> for Collections::IIterable<IWwwFormUrlDecoderEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WwwFormUrlDecoder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, Collections::IIterable<IWwwFormUrlDecoderEntry>> for WwwFormUrlDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, Collections::IIterable<IWwwFormUrlDecoderEntry>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, Collections::IIterable<IWwwFormUrlDecoderEntry>> for &WwwFormUrlDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, Collections::IIterable<IWwwFormUrlDecoderEntry>> {
        ::core::convert::TryInto::<Collections::IIterable<IWwwFormUrlDecoderEntry>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<WwwFormUrlDecoder> for Collections::IVectorView<IWwwFormUrlDecoderEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: WwwFormUrlDecoder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&WwwFormUrlDecoder> for Collections::IVectorView<IWwwFormUrlDecoderEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WwwFormUrlDecoder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, Collections::IVectorView<IWwwFormUrlDecoderEntry>> for WwwFormUrlDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, Collections::IVectorView<IWwwFormUrlDecoderEntry>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, Collections::IVectorView<IWwwFormUrlDecoderEntry>> for &WwwFormUrlDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, Collections::IVectorView<IWwwFormUrlDecoderEntry>> {
        ::core::convert::TryInto::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WwwFormUrlDecoder {}
unsafe impl ::core::marker::Sync for WwwFormUrlDecoder {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        Collections::VectorViewIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WwwFormUrlDecoderEntry(pub ::windows::core::IInspectable);
impl WwwFormUrlDecoderEntry {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for WwwFormUrlDecoderEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Foundation.WwwFormUrlDecoderEntry;{125e7431-f678-4e8e-b670-20a9b06c512d})");
}
unsafe impl ::windows::core::Interface for WwwFormUrlDecoderEntry {
    type Vtable = IWwwFormUrlDecoderEntry_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x125e7431_f678_4e8e_b670_20a9b06c512d);
}
impl ::windows::core::RuntimeName for WwwFormUrlDecoderEntry {
    const NAME: &'static str = "Windows.Foundation.WwwFormUrlDecoderEntry";
}
impl ::core::convert::From<WwwFormUrlDecoderEntry> for ::windows::core::IUnknown {
    fn from(value: WwwFormUrlDecoderEntry) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&WwwFormUrlDecoderEntry> for ::windows::core::IUnknown {
    fn from(value: &WwwFormUrlDecoderEntry) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WwwFormUrlDecoderEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WwwFormUrlDecoderEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<WwwFormUrlDecoderEntry> for ::windows::core::IInspectable {
    fn from(value: WwwFormUrlDecoderEntry) -> Self {
        value.0
    }
}
impl ::core::convert::From<&WwwFormUrlDecoderEntry> for ::windows::core::IInspectable {
    fn from(value: &WwwFormUrlDecoderEntry) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WwwFormUrlDecoderEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WwwFormUrlDecoderEntry {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<WwwFormUrlDecoderEntry> for IWwwFormUrlDecoderEntry {
    fn from(value: WwwFormUrlDecoderEntry) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WwwFormUrlDecoderEntry> for IWwwFormUrlDecoderEntry {
    fn from(value: &WwwFormUrlDecoderEntry) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWwwFormUrlDecoderEntry> for WwwFormUrlDecoderEntry {
    fn into_param(self) -> ::windows::core::Param<'a, IWwwFormUrlDecoderEntry> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWwwFormUrlDecoderEntry> for &WwwFormUrlDecoderEntry {
    fn into_param(self) -> ::windows::core::Param<'a, IWwwFormUrlDecoderEntry> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WwwFormUrlDecoderEntry {}
unsafe impl ::core::marker::Sync for WwwFormUrlDecoderEntry {}
