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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncActionCompletedHandler(::windows::runtime::IUnknown);
impl AsyncActionCompletedHandler {
    pub fn new<F: FnMut(&::std::option::Option<IAsyncAction>, AsyncStatus) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = AsyncActionCompletedHandler_box::<F> {
            vtable: &AsyncActionCompletedHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, IAsyncAction>>(&self, asyncinfo: Param0, asyncstatus: AsyncStatus) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), asyncinfo.into_param().abi(), asyncstatus).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for AsyncActionCompletedHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({a4ed5c81-76c9-40bd-8be6-b1d90fb20ae7})");
}
unsafe impl ::windows::runtime::Interface for AsyncActionCompletedHandler {
    type Vtable = AsyncActionCompletedHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2767019137, 30409, 16573, [139, 230, 177, 217, 15, 178, 10, 231]);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncActionCompletedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, asyncinfo: ::windows::runtime::RawPtr, asyncstatus: AsyncStatus) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct AsyncActionCompletedHandler_box<F: FnMut(&::std::option::Option<IAsyncAction>, AsyncStatus) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const AsyncActionCompletedHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<IAsyncAction>, AsyncStatus) -> ::windows::runtime::Result<()> + 'static> AsyncActionCompletedHandler_box<F> {
    const VTABLE: AsyncActionCompletedHandler_abi = AsyncActionCompletedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<AsyncActionCompletedHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, asyncinfo: ::windows::runtime::RawPtr, asyncstatus: AsyncStatus) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&asyncinfo as *const <IAsyncAction as ::windows::runtime::Abi>::Abi as *const <IAsyncAction as ::windows::runtime::Abi>::DefaultType), asyncstatus).into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncActionProgressHandler<TProgress>(::windows::runtime::IUnknown, ::std::marker::PhantomData<TProgress>)
where
    TProgress: ::windows::runtime::RuntimeType + 'static;
impl<TProgress: ::windows::runtime::RuntimeType + 'static> AsyncActionProgressHandler<TProgress> {
    pub fn new<F: FnMut(&::std::option::Option<IAsyncActionWithProgress<TProgress>>, &<TProgress as ::windows::runtime::Abi>::DefaultType) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = AsyncActionProgressHandler_box::<TProgress, F> {
            vtable: &AsyncActionProgressHandler_box::<TProgress, F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, IAsyncActionWithProgress<TProgress>>, Param1: ::windows::runtime::IntoParam<'a, TProgress>>(&self, asyncinfo: Param0, progressinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), asyncinfo.into_param().abi(), progressinfo.into_param().abi()).ok() }
    }
}
unsafe impl<TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for AsyncActionProgressHandler<TProgress> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{6d844858-0cff-4590-ae89-95a5a5c8b4b8}").push_slice(b";").push_other(<TProgress as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for AsyncActionProgressHandler<TProgress> {
    type Vtable = AsyncActionProgressHandler_abi<TProgress>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<AsyncActionProgressHandler<TProgress> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncActionProgressHandler_abi<TProgress>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, asyncinfo: ::windows::runtime::RawPtr, progressinfo: <TProgress as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub ::std::marker::PhantomData<TProgress>,
)
where
    TProgress: ::windows::runtime::RuntimeType + 'static;
#[repr(C)]
struct AsyncActionProgressHandler_box<TProgress, F: FnMut(&::std::option::Option<IAsyncActionWithProgress<TProgress>>, &<TProgress as ::windows::runtime::Abi>::DefaultType) -> ::windows::runtime::Result<()> + 'static>
where
    TProgress: ::windows::runtime::RuntimeType + 'static,
{
    vtable: *const AsyncActionProgressHandler_abi<TProgress>,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<TProgress: ::windows::runtime::RuntimeType + 'static, F: FnMut(&::std::option::Option<IAsyncActionWithProgress<TProgress>>, &<TProgress as ::windows::runtime::Abi>::DefaultType) -> ::windows::runtime::Result<()> + 'static> AsyncActionProgressHandler_box<TProgress, F> {
    const VTABLE: AsyncActionProgressHandler_abi<TProgress> = AsyncActionProgressHandler_abi::<TProgress>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::std::marker::PhantomData::<TProgress>);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<AsyncActionProgressHandler<TProgress> as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, asyncinfo: ::windows::runtime::RawPtr, progressinfo: <TProgress as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&asyncinfo as *const <IAsyncActionWithProgress<TProgress> as ::windows::runtime::Abi>::Abi as *const <IAsyncActionWithProgress<TProgress> as ::windows::runtime::Abi>::DefaultType),
            &*(&progressinfo as *const <TProgress as ::windows::runtime::Abi>::Abi as *const <TProgress as ::windows::runtime::Abi>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncActionWithProgressCompletedHandler<TProgress>(::windows::runtime::IUnknown, ::std::marker::PhantomData<TProgress>)
where
    TProgress: ::windows::runtime::RuntimeType + 'static;
impl<TProgress: ::windows::runtime::RuntimeType + 'static> AsyncActionWithProgressCompletedHandler<TProgress> {
    pub fn new<F: FnMut(&::std::option::Option<IAsyncActionWithProgress<TProgress>>, AsyncStatus) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = AsyncActionWithProgressCompletedHandler_box::<TProgress, F> {
            vtable: &AsyncActionWithProgressCompletedHandler_box::<TProgress, F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, IAsyncActionWithProgress<TProgress>>>(&self, asyncinfo: Param0, asyncstatus: AsyncStatus) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), asyncinfo.into_param().abi(), asyncstatus).ok() }
    }
}
unsafe impl<TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for AsyncActionWithProgressCompletedHandler<TProgress> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9c029f91-cc84-44fd-ac26-0a6c4e555281}").push_slice(b";").push_other(<TProgress as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for AsyncActionWithProgressCompletedHandler<TProgress> {
    type Vtable = AsyncActionWithProgressCompletedHandler_abi<TProgress>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<AsyncActionWithProgressCompletedHandler<TProgress> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncActionWithProgressCompletedHandler_abi<TProgress>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, asyncinfo: ::windows::runtime::RawPtr, asyncstatus: AsyncStatus) -> ::windows::runtime::HRESULT,
    pub ::std::marker::PhantomData<TProgress>,
)
where
    TProgress: ::windows::runtime::RuntimeType + 'static;
#[repr(C)]
struct AsyncActionWithProgressCompletedHandler_box<TProgress, F: FnMut(&::std::option::Option<IAsyncActionWithProgress<TProgress>>, AsyncStatus) -> ::windows::runtime::Result<()> + 'static>
where
    TProgress: ::windows::runtime::RuntimeType + 'static,
{
    vtable: *const AsyncActionWithProgressCompletedHandler_abi<TProgress>,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<TProgress: ::windows::runtime::RuntimeType + 'static, F: FnMut(&::std::option::Option<IAsyncActionWithProgress<TProgress>>, AsyncStatus) -> ::windows::runtime::Result<()> + 'static> AsyncActionWithProgressCompletedHandler_box<TProgress, F> {
    const VTABLE: AsyncActionWithProgressCompletedHandler_abi<TProgress> = AsyncActionWithProgressCompletedHandler_abi::<TProgress>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::std::marker::PhantomData::<TProgress>);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<AsyncActionWithProgressCompletedHandler<TProgress> as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, asyncinfo: ::windows::runtime::RawPtr, asyncstatus: AsyncStatus) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&asyncinfo as *const <IAsyncActionWithProgress<TProgress> as ::windows::runtime::Abi>::Abi as *const <IAsyncActionWithProgress<TProgress> as ::windows::runtime::Abi>::DefaultType), asyncstatus).into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncOperationCompletedHandler<TResult>(::windows::runtime::IUnknown, ::std::marker::PhantomData<TResult>)
where
    TResult: ::windows::runtime::RuntimeType + 'static;
impl<TResult: ::windows::runtime::RuntimeType + 'static> AsyncOperationCompletedHandler<TResult> {
    pub fn new<F: FnMut(&::std::option::Option<IAsyncOperation<TResult>>, AsyncStatus) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = AsyncOperationCompletedHandler_box::<TResult, F> {
            vtable: &AsyncOperationCompletedHandler_box::<TResult, F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, IAsyncOperation<TResult>>>(&self, asyncinfo: Param0, asyncstatus: AsyncStatus) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), asyncinfo.into_param().abi(), asyncstatus).ok() }
    }
}
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for AsyncOperationCompletedHandler<TResult> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{fcdcf02c-e5d8-4478-915a-4d90b74b83a5}").push_slice(b";").push_other(<TResult as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for AsyncOperationCompletedHandler<TResult> {
    type Vtable = AsyncOperationCompletedHandler_abi<TResult>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<AsyncOperationCompletedHandler<TResult> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncOperationCompletedHandler_abi<TResult>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, asyncinfo: ::windows::runtime::RawPtr, asyncstatus: AsyncStatus) -> ::windows::runtime::HRESULT,
    pub ::std::marker::PhantomData<TResult>,
)
where
    TResult: ::windows::runtime::RuntimeType + 'static;
#[repr(C)]
struct AsyncOperationCompletedHandler_box<TResult, F: FnMut(&::std::option::Option<IAsyncOperation<TResult>>, AsyncStatus) -> ::windows::runtime::Result<()> + 'static>
where
    TResult: ::windows::runtime::RuntimeType + 'static,
{
    vtable: *const AsyncOperationCompletedHandler_abi<TResult>,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<TResult: ::windows::runtime::RuntimeType + 'static, F: FnMut(&::std::option::Option<IAsyncOperation<TResult>>, AsyncStatus) -> ::windows::runtime::Result<()> + 'static> AsyncOperationCompletedHandler_box<TResult, F> {
    const VTABLE: AsyncOperationCompletedHandler_abi<TResult> = AsyncOperationCompletedHandler_abi::<TResult>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::std::marker::PhantomData::<TResult>);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<AsyncOperationCompletedHandler<TResult> as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, asyncinfo: ::windows::runtime::RawPtr, asyncstatus: AsyncStatus) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&asyncinfo as *const <IAsyncOperation<TResult> as ::windows::runtime::Abi>::Abi as *const <IAsyncOperation<TResult> as ::windows::runtime::Abi>::DefaultType), asyncstatus).into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncOperationProgressHandler<TResult, TProgress>(::windows::runtime::IUnknown, ::std::marker::PhantomData<TResult>, ::std::marker::PhantomData<TProgress>)
where
    TResult: ::windows::runtime::RuntimeType + 'static,
    TProgress: ::windows::runtime::RuntimeType + 'static;
impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> AsyncOperationProgressHandler<TResult, TProgress> {
    pub fn new<F: FnMut(&::std::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, &<TProgress as ::windows::runtime::Abi>::DefaultType) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = AsyncOperationProgressHandler_box::<TResult, TProgress, F> {
            vtable: &AsyncOperationProgressHandler_box::<TResult, TProgress, F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, IAsyncOperationWithProgress<TResult, TProgress>>, Param1: ::windows::runtime::IntoParam<'a, TProgress>>(&self, asyncinfo: Param0, progressinfo: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), asyncinfo.into_param().abi(), progressinfo.into_param().abi()).ok() }
    }
}
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for AsyncOperationProgressHandler<TResult, TProgress> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = {
        ::windows::runtime::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(b"{55690902-0aab-421a-8778-f8ce5026d758}")
            .push_slice(b";")
            .push_other(<TResult as ::windows::runtime::RuntimeType>::SIGNATURE)
            .push_slice(b";")
            .push_other(<TProgress as ::windows::runtime::RuntimeType>::SIGNATURE)
            .push_slice(b")")
    };
}
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for AsyncOperationProgressHandler<TResult, TProgress> {
    type Vtable = AsyncOperationProgressHandler_abi<TResult, TProgress>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<AsyncOperationProgressHandler<TResult, TProgress> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncOperationProgressHandler_abi<TResult, TProgress>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, asyncinfo: ::windows::runtime::RawPtr, progressinfo: <TProgress as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub ::std::marker::PhantomData<TResult>,
    pub ::std::marker::PhantomData<TProgress>,
)
where
    TResult: ::windows::runtime::RuntimeType + 'static,
    TProgress: ::windows::runtime::RuntimeType + 'static;
#[repr(C)]
struct AsyncOperationProgressHandler_box<TResult, TProgress, F: FnMut(&::std::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, &<TProgress as ::windows::runtime::Abi>::DefaultType) -> ::windows::runtime::Result<()> + 'static>
where
    TResult: ::windows::runtime::RuntimeType + 'static,
    TProgress: ::windows::runtime::RuntimeType + 'static,
{
    vtable: *const AsyncOperationProgressHandler_abi<TResult, TProgress>,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static, F: FnMut(&::std::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, &<TProgress as ::windows::runtime::Abi>::DefaultType) -> ::windows::runtime::Result<()> + 'static> AsyncOperationProgressHandler_box<TResult, TProgress, F> {
    const VTABLE: AsyncOperationProgressHandler_abi<TResult, TProgress> = AsyncOperationProgressHandler_abi::<TResult, TProgress>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::std::marker::PhantomData::<TResult>, ::std::marker::PhantomData::<TProgress>);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<AsyncOperationProgressHandler<TResult, TProgress> as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, asyncinfo: ::windows::runtime::RawPtr, progressinfo: <TProgress as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&asyncinfo as *const <IAsyncOperationWithProgress<TResult, TProgress> as ::windows::runtime::Abi>::Abi as *const <IAsyncOperationWithProgress<TResult, TProgress> as ::windows::runtime::Abi>::DefaultType),
            &*(&progressinfo as *const <TProgress as ::windows::runtime::Abi>::Abi as *const <TProgress as ::windows::runtime::Abi>::DefaultType),
        )
        .into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct AsyncOperationWithProgressCompletedHandler<TResult, TProgress>(::windows::runtime::IUnknown, ::std::marker::PhantomData<TResult>, ::std::marker::PhantomData<TProgress>)
where
    TResult: ::windows::runtime::RuntimeType + 'static,
    TProgress: ::windows::runtime::RuntimeType + 'static;
impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    pub fn new<F: FnMut(&::std::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, AsyncStatus) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = AsyncOperationWithProgressCompletedHandler_box::<TResult, TProgress, F> {
            vtable: &AsyncOperationWithProgressCompletedHandler_box::<TResult, TProgress, F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, IAsyncOperationWithProgress<TResult, TProgress>>>(&self, asyncinfo: Param0, asyncstatus: AsyncStatus) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), asyncinfo.into_param().abi(), asyncstatus).ok() }
    }
}
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = {
        ::windows::runtime::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(b"{e85df41d-6aa7-46e3-a8e2-f009d840c627}")
            .push_slice(b";")
            .push_other(<TResult as ::windows::runtime::RuntimeType>::SIGNATURE)
            .push_slice(b";")
            .push_other(<TProgress as ::windows::runtime::RuntimeType>::SIGNATURE)
            .push_slice(b")")
    };
}
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for AsyncOperationWithProgressCompletedHandler<TResult, TProgress> {
    type Vtable = AsyncOperationWithProgressCompletedHandler_abi<TResult, TProgress>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<AsyncOperationWithProgressCompletedHandler<TResult, TProgress> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncOperationWithProgressCompletedHandler_abi<TResult, TProgress>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, asyncinfo: ::windows::runtime::RawPtr, asyncstatus: AsyncStatus) -> ::windows::runtime::HRESULT,
    pub ::std::marker::PhantomData<TResult>,
    pub ::std::marker::PhantomData<TProgress>,
)
where
    TResult: ::windows::runtime::RuntimeType + 'static,
    TProgress: ::windows::runtime::RuntimeType + 'static;
#[repr(C)]
struct AsyncOperationWithProgressCompletedHandler_box<TResult, TProgress, F: FnMut(&::std::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, AsyncStatus) -> ::windows::runtime::Result<()> + 'static>
where
    TResult: ::windows::runtime::RuntimeType + 'static,
    TProgress: ::windows::runtime::RuntimeType + 'static,
{
    vtable: *const AsyncOperationWithProgressCompletedHandler_abi<TResult, TProgress>,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static, F: FnMut(&::std::option::Option<IAsyncOperationWithProgress<TResult, TProgress>>, AsyncStatus) -> ::windows::runtime::Result<()> + 'static> AsyncOperationWithProgressCompletedHandler_box<TResult, TProgress, F> {
    const VTABLE: AsyncOperationWithProgressCompletedHandler_abi<TResult, TProgress> = AsyncOperationWithProgressCompletedHandler_abi::<TResult, TProgress>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::std::marker::PhantomData::<TResult>, ::std::marker::PhantomData::<TProgress>);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<AsyncOperationWithProgressCompletedHandler<TResult, TProgress> as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, asyncinfo: ::windows::runtime::RawPtr, asyncstatus: AsyncStatus) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&asyncinfo as *const <IAsyncOperationWithProgress<TResult, TProgress> as ::windows::runtime::Abi>::Abi as *const <IAsyncOperationWithProgress<TResult, TProgress> as ::windows::runtime::Abi>::DefaultType), asyncstatus).into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: AsyncStatus = AsyncStatus(2i32);
    pub const Completed: AsyncStatus = AsyncStatus(1i32);
    pub const Error: AsyncStatus = AsyncStatus(3i32);
    pub const Started: AsyncStatus = AsyncStatus(0i32);
}
impl ::std::convert::From<i32> for AsyncStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AsyncStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for AsyncStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.AsyncStatus;i4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Foundation`*"]
pub struct DateTime {
    pub UniversalTime: i64,
}
impl DateTime {}
impl ::std::default::Default for DateTime {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DateTime {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DateTime").field("UniversalTime", &self.UniversalTime).finish()
    }
}
impl ::std::cmp::PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.UniversalTime == other.UniversalTime
    }
}
impl ::std::cmp::Eq for DateTime {}
unsafe impl ::windows::runtime::Abi for DateTime {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DateTime {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.DateTime;i8)");
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct Deferral(::windows::runtime::IInspectable);
impl Deferral {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, DeferralCompletedHandler>>(handler: Param0) -> ::windows::runtime::Result<Deferral> {
        Self::IDeferralFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<Deferral>(result__)
        })
    }
    pub fn IDeferralFactory<R, F: FnOnce(&IDeferralFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Deferral, IDeferralFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Deferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Deferral;{d6269732-3b7f-46a7-b40b-4fdca2a2c693})");
}
unsafe impl ::windows::runtime::Interface for Deferral {
    type Vtable = IDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3592853298, 15231, 18087, [180, 11, 79, 220, 162, 162, 198, 147]);
}
impl ::windows::runtime::RuntimeName for Deferral {
    const NAME: &'static str = "Windows.Foundation.Deferral";
}
impl ::std::convert::TryFrom<Deferral> for IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Deferral) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&Deferral> for IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Deferral) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IClosable> for Deferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IClosable> for &Deferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, IClosable> {
        ::std::convert::TryInto::<IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for Deferral {}
unsafe impl ::std::marker::Sync for Deferral {}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct DeferralCompletedHandler(::windows::runtime::IUnknown);
impl DeferralCompletedHandler {
    pub fn new<F: FnMut() -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = DeferralCompletedHandler_box::<F> {
            vtable: &DeferralCompletedHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DeferralCompletedHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({ed32a372-f3c8-4faa-9cfb-470148da3888})");
}
unsafe impl ::windows::runtime::Interface for DeferralCompletedHandler {
    type Vtable = DeferralCompletedHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3979518834, 62408, 20394, [156, 251, 71, 1, 72, 218, 56, 136]);
}
#[repr(C)]
#[doc(hidden)]
pub struct DeferralCompletedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct DeferralCompletedHandler_box<F: FnMut() -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const DeferralCompletedHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut() -> ::windows::runtime::Result<()> + 'static> DeferralCompletedHandler_box<F> {
    const VTABLE: DeferralCompletedHandler_abi = DeferralCompletedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<DeferralCompletedHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)().into()
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct EventHandler<T>(::windows::runtime::IUnknown, ::std::marker::PhantomData<T>)
where
    T: ::windows::runtime::RuntimeType + 'static;
impl<T: ::windows::runtime::RuntimeType + 'static> EventHandler<T> {
    pub fn new<F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &<T as ::windows::runtime::Abi>::DefaultType) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = EventHandler_box::<T, F> {
            vtable: &EventHandler_box::<T, F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param1: ::windows::runtime::IntoParam<'a, T>>(&self, sender: Param0, args: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
}
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for EventHandler<T> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9de1c535-6ae1-11e0-84e1-18a905bcc53f}").push_slice(b";").push_other(<T as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for EventHandler<T> {
    type Vtable = EventHandler_abi<T>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<EventHandler<T> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct EventHandler_abi<T>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, args: <T as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub ::std::marker::PhantomData<T>,
)
where
    T: ::windows::runtime::RuntimeType + 'static;
#[repr(C)]
struct EventHandler_box<T, F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &<T as ::windows::runtime::Abi>::DefaultType) -> ::windows::runtime::Result<()> + 'static>
where
    T: ::windows::runtime::RuntimeType + 'static,
{
    vtable: *const EventHandler_abi<T>,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<T: ::windows::runtime::RuntimeType + 'static, F: FnMut(&::std::option::Option<::windows::runtime::IInspectable>, &<T as ::windows::runtime::Abi>::DefaultType) -> ::windows::runtime::Result<()> + 'static> EventHandler_box<T, F> {
    const VTABLE: EventHandler_abi<T> = EventHandler_abi::<T>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::std::marker::PhantomData::<T>);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<EventHandler<T> as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: ::windows::runtime::RawPtr, args: <T as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi as *const <::windows::runtime::IInspectable as ::windows::runtime::Abi>::DefaultType), &*(&args as *const <T as ::windows::runtime::Abi>::Abi as *const <T as ::windows::runtime::Abi>::DefaultType)).into()
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Foundation`*"]
pub struct EventRegistrationToken {
    pub Value: i64,
}
impl EventRegistrationToken {}
impl ::std::default::Default for EventRegistrationToken {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EventRegistrationToken {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EventRegistrationToken").field("Value", &self.Value).finish()
    }
}
impl ::std::cmp::PartialEq for EventRegistrationToken {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::std::cmp::Eq for EventRegistrationToken {}
unsafe impl ::windows::runtime::Abi for EventRegistrationToken {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for EventRegistrationToken {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.EventRegistrationToken;i8)");
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct FoundationContract(pub u8);
#[doc = "*Required features: `Foundation`*"]
pub struct GuidHelper {}
impl GuidHelper {
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateNewGuid() -> ::windows::runtime::Result<::windows::runtime::GUID> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Empty() -> ::windows::runtime::Result<::windows::runtime::GUID> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Equals<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(target: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        Self::IGuidHelperStatics(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &target.into_param().abi(), &value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        })
    }
    pub fn IGuidHelperStatics<R, F: FnOnce(&IGuidHelperStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<GuidHelper, IGuidHelperStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for GuidHelper {
    const NAME: &'static str = "Windows.Foundation.GuidHelper";
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation`*"]
pub struct IAsyncAction(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAsyncAction {
    type Vtable = IAsyncAction_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1516535814, 33850, 19881, [134, 91, 157, 38, 229, 223, 173, 123]);
}
impl IAsyncAction {
    #[doc = "*Required features: `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, AsyncActionCompletedHandler>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Completed(&self) -> ::windows::runtime::Result<AsyncActionCompletedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AsyncActionCompletedHandler>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<()> {
        if self.Status()? == AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
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
unsafe impl ::windows::runtime::RuntimeType for IAsyncAction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{5a648006-843a-4da9-865b-9d26e5dfad7b}");
}
impl ::std::future::Future for IAsyncAction {
    type Output = ::windows::runtime::Result<()>;
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
impl ::std::convert::TryFrom<IAsyncAction> for IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAsyncAction) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IAsyncAction> for IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAsyncAction) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAsyncInfo> for IAsyncAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IAsyncInfo> for &IAsyncAction {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAsyncInfo> {
        ::std::convert::TryInto::<IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for IAsyncAction {}
unsafe impl ::std::marker::Sync for IAsyncAction {}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncAction_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IAsyncActionWithProgress<TProgress>(::windows::runtime::IInspectable, ::std::marker::PhantomData<TProgress>)
where
    TProgress: ::windows::runtime::RuntimeType + 'static;
unsafe impl<TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IAsyncActionWithProgress<TProgress> {
    type Vtable = IAsyncActionWithProgress_abi<TProgress>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IAsyncActionWithProgress<TProgress> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<TProgress: ::windows::runtime::RuntimeType + 'static> IAsyncActionWithProgress<TProgress> {
    #[doc = "*Required features: `Foundation`*"]
    pub fn SetProgress<'a, Param0: ::windows::runtime::IntoParam<'a, AsyncActionProgressHandler<TProgress>>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Progress(&self) -> ::windows::runtime::Result<AsyncActionProgressHandler<TProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AsyncActionProgressHandler<TProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, AsyncActionWithProgressCompletedHandler<TProgress>>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Completed(&self) -> ::windows::runtime::Result<AsyncActionWithProgressCompletedHandler<TProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AsyncActionWithProgressCompletedHandler<TProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<()> {
        if self.Status()? == AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
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
unsafe impl<TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IAsyncActionWithProgress<TProgress> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{1f6db258-e803-48a1-9546-eb7353398884}").push_slice(b";").push_other(<TProgress as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<TProgress: ::windows::runtime::RuntimeType + 'static> ::std::future::Future for IAsyncActionWithProgress<TProgress> {
    type Output = ::windows::runtime::Result<()>;
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
impl<TProgress: ::windows::runtime::RuntimeType + 'static> ::std::convert::TryFrom<IAsyncActionWithProgress<TProgress>> for IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAsyncActionWithProgress<TProgress>) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl<TProgress: ::windows::runtime::RuntimeType + 'static> ::std::convert::TryFrom<&IAsyncActionWithProgress<TProgress>> for IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAsyncActionWithProgress<TProgress>) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a, TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IAsyncInfo> for IAsyncActionWithProgress<TProgress> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a, TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IAsyncInfo> for &IAsyncActionWithProgress<TProgress> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAsyncInfo> {
        ::std::convert::TryInto::<IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl<TProgress: ::windows::runtime::RuntimeType + 'static> ::std::marker::Send for IAsyncActionWithProgress<TProgress> {}
unsafe impl<TProgress: ::windows::runtime::RuntimeType + 'static> ::std::marker::Sync for IAsyncActionWithProgress<TProgress> {}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncActionWithProgress_abi<TProgress>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub ::std::marker::PhantomData<TProgress>,
)
where
    TProgress: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation`*"]
pub struct IAsyncInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAsyncInfo {
    type Vtable = IAsyncInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(54, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
impl IAsyncInfo {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<AsyncStatus> {
        let this = self;
        unsafe {
            let mut result__: AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IAsyncInfo {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{00000036-0000-0000-c000-000000000046}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut AsyncStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IAsyncOperation<TResult>(::windows::runtime::IInspectable, ::std::marker::PhantomData<TResult>)
where
    TResult: ::windows::runtime::RuntimeType + 'static;
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IAsyncOperation<TResult> {
    type Vtable = IAsyncOperation_abi<TResult>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IAsyncOperation<TResult> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<TResult: ::windows::runtime::RuntimeType + 'static> IAsyncOperation<TResult> {
    #[doc = "*Required features: `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, AsyncOperationCompletedHandler<TResult>>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Completed(&self) -> ::windows::runtime::Result<AsyncOperationCompletedHandler<TResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AsyncOperationCompletedHandler<TResult>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::runtime::Result<TResult> {
        let this = self;
        unsafe {
            let mut result__: <TResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TResult>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<TResult> {
        if self.Status()? == AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
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
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IAsyncOperation<TResult> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{9fc2b0bb-e446-44e2-aa61-9cab8f636af2}").push_slice(b";").push_other(<TResult as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<TResult: ::windows::runtime::RuntimeType + 'static> ::std::future::Future for IAsyncOperation<TResult> {
    type Output = ::windows::runtime::Result<TResult>;
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
impl<TResult: ::windows::runtime::RuntimeType + 'static> ::std::convert::TryFrom<IAsyncOperation<TResult>> for IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAsyncOperation<TResult>) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl<TResult: ::windows::runtime::RuntimeType + 'static> ::std::convert::TryFrom<&IAsyncOperation<TResult>> for IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAsyncOperation<TResult>) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a, TResult: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IAsyncInfo> for IAsyncOperation<TResult> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a, TResult: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IAsyncInfo> for &IAsyncOperation<TResult> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAsyncInfo> {
        ::std::convert::TryInto::<IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static> ::std::marker::Send for IAsyncOperation<TResult> {}
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static> ::std::marker::Sync for IAsyncOperation<TResult> {}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncOperation_abi<TResult>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut <TResult as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub ::std::marker::PhantomData<TResult>,
)
where
    TResult: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IAsyncOperationWithProgress<TResult, TProgress>(::windows::runtime::IInspectable, ::std::marker::PhantomData<TResult>, ::std::marker::PhantomData<TProgress>)
where
    TResult: ::windows::runtime::RuntimeType + 'static,
    TProgress: ::windows::runtime::RuntimeType + 'static;
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IAsyncOperationWithProgress<TResult, TProgress> {
    type Vtable = IAsyncOperationWithProgress_abi<TResult, TProgress>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IAsyncOperationWithProgress<TResult, TProgress> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> IAsyncOperationWithProgress<TResult, TProgress> {
    #[doc = "*Required features: `Foundation`*"]
    pub fn SetProgress<'a, Param0: ::windows::runtime::IntoParam<'a, AsyncOperationProgressHandler<TResult, TProgress>>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Progress(&self) -> ::windows::runtime::Result<AsyncOperationProgressHandler<TResult, TProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AsyncOperationProgressHandler<TResult, TProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn SetCompleted<'a, Param0: ::windows::runtime::IntoParam<'a, AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>>(&self, handler: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Completed(&self) -> ::windows::runtime::Result<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AsyncOperationWithProgressCompletedHandler<TResult, TProgress>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetResults(&self) -> ::windows::runtime::Result<TResult> {
        let this = self;
        unsafe {
            let mut result__: <TResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TResult>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Status(&self) -> ::windows::runtime::Result<AsyncStatus> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: AsyncStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<AsyncStatus>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn ErrorCode(&self) -> ::windows::runtime::Result<::windows::runtime::HRESULT> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::HRESULT = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HRESULT>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IAsyncInfo>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn get(&self) -> ::windows::runtime::Result<TResult> {
        if self.Status()? == AsyncStatus::Started {
            let (waiter, signaler) = ::windows::runtime::Waiter::new();
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
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IAsyncOperationWithProgress<TResult, TProgress> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = {
        ::windows::runtime::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(b"{b5d036d7-e297-498f-ba60-0289e76e23dd}")
            .push_slice(b";")
            .push_other(<TResult as ::windows::runtime::RuntimeType>::SIGNATURE)
            .push_slice(b";")
            .push_other(<TProgress as ::windows::runtime::RuntimeType>::SIGNATURE)
            .push_slice(b")")
    };
}
impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> ::std::future::Future for IAsyncOperationWithProgress<TResult, TProgress> {
    type Output = ::windows::runtime::Result<TResult>;
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
impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> ::std::convert::TryFrom<IAsyncOperationWithProgress<TResult, TProgress>> for IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IAsyncOperationWithProgress<TResult, TProgress>) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> ::std::convert::TryFrom<&IAsyncOperationWithProgress<TResult, TProgress>> for IAsyncInfo {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IAsyncOperationWithProgress<TResult, TProgress>) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a, TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IAsyncInfo> for IAsyncOperationWithProgress<TResult, TProgress> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAsyncInfo> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a, TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IAsyncInfo> for &IAsyncOperationWithProgress<TResult, TProgress> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IAsyncInfo> {
        ::std::convert::TryInto::<IAsyncInfo>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> ::std::marker::Send for IAsyncOperationWithProgress<TResult, TProgress> {}
unsafe impl<TResult: ::windows::runtime::RuntimeType + 'static, TProgress: ::windows::runtime::RuntimeType + 'static> ::std::marker::Sync for IAsyncOperationWithProgress<TResult, TProgress> {}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncOperationWithProgress_abi<TResult, TProgress>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut <TResult as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub ::std::marker::PhantomData<TResult>,
    pub ::std::marker::PhantomData<TProgress>,
)
where
    TResult: ::windows::runtime::RuntimeType + 'static,
    TProgress: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation`*"]
pub struct IClosable(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClosable {
    type Vtable = IClosable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(819308585, 32676, 16422, [131, 187, 215, 91, 174, 78, 169, 158]);
}
impl IClosable {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IClosable {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{30d5a829-7fa4-4026-83bb-d75bae4ea99e}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeferral {
    type Vtable = IDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3592853298, 15231, 18087, [180, 11, 79, 220, 162, 162, 198, 147]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeferral_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IDeferralFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDeferralFactory {
    type Vtable = IDeferralFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1705110725, 16309, 18482, [140, 169, 240, 97, 178, 129, 209, 58]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeferralFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation`*"]
pub struct IGetActivationFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGetActivationFactory {
    type Vtable = IGetActivationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1323011810, 38621, 18855, [148, 247, 70, 7, 221, 171, 142, 60]);
}
impl IGetActivationFactory {
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetActivationFactory<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, activatableclassid: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), activatableclassid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IGetActivationFactory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{4edb8ee2-96dd-49a7-94f7-4607ddab8e3c}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetActivationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activatableclassid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IGuidHelperStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGuidHelperStatics {
    type Vtable = IGuidHelperStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1506252395, 44626, 21123, [173, 127, 161, 185, 233, 103, 138, 221]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGuidHelperStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: &::windows::runtime::GUID, value: &::windows::runtime::GUID, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation`*"]
pub struct IMemoryBuffer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMemoryBuffer {
    type Vtable = IMemoryBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4223982890, 9307, 4580, [175, 152, 104, 148, 35, 38, 12, 248]);
}
impl IMemoryBuffer {
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<IMemoryBufferReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IMemoryBuffer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{fbc4dd2a-245b-11e4-af98-689423260cf8}");
}
impl ::std::convert::TryFrom<IMemoryBuffer> for IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IMemoryBuffer) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IMemoryBuffer> for IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IMemoryBuffer) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IClosable> for IMemoryBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IClosable> for &IMemoryBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IClosable> {
        ::std::convert::TryInto::<IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IMemoryBufferFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMemoryBufferFactory {
    type Vtable = IMemoryBufferFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4223982891, 9307, 4580, [175, 152, 104, 148, 35, 38, 12, 248]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBufferFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capacity: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation`*"]
pub struct IMemoryBufferReference(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMemoryBufferReference {
    type Vtable = IMemoryBufferReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4223982889, 9307, 4580, [175, 152, 104, 148, 35, 38, 12, 248]);
}
impl IMemoryBufferReference {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Capacity(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Closed<'a, Param0: ::windows::runtime::IntoParam<'a, TypedEventHandler<IMemoryBufferReference, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn RemoveClosed<'a, Param0: ::windows::runtime::IntoParam<'a, EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IMemoryBufferReference {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{fbc4dd29-245b-11e4-af98-689423260cf8}");
}
impl ::std::convert::TryFrom<IMemoryBufferReference> for IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IMemoryBufferReference) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IMemoryBufferReference> for IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IMemoryBufferReference) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IClosable> for IMemoryBufferReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IClosable> for &IMemoryBufferReference {
    fn into_param(self) -> ::windows::runtime::Param<'a, IClosable> {
        ::std::convert::TryInto::<IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBufferReference_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut EventRegistrationToken) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: EventRegistrationToken) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation`*"]
pub struct IPropertyValue(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPropertyValue {
    type Vtable = IPropertyValue_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1272349405, 30036, 16617, [154, 155, 130, 101, 78, 222, 126, 98]);
}
impl IPropertyValue {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<PropertyType> {
        let this = self;
        unsafe {
            let mut result__: PropertyType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PropertyType>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn IsNumericScalar(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt8(&self) -> ::windows::runtime::Result<u8> {
        let this = self;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt16(&self) -> ::windows::runtime::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt16(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt32(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt32(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt64(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt64(&self) -> ::windows::runtime::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSingle(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDouble(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetChar16(&self) -> ::windows::runtime::Result<u16> {
        let this = self;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetBoolean(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetGuid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDateTime(&self) -> ::windows::runtime::Result<DateTime> {
        let this = self;
        unsafe {
            let mut result__: DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetTimeSpan(&self) -> ::windows::runtime::Result<TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetPoint(&self) -> ::windows::runtime::Result<Point> {
        let this = self;
        unsafe {
            let mut result__: Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Point>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSize(&self) -> ::windows::runtime::Result<Size> {
        let this = self;
        unsafe {
            let mut result__: Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Size>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetRect(&self) -> ::windows::runtime::Result<Rect> {
        let this = self;
        unsafe {
            let mut result__: Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Rect>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt8Array(&self, value: &mut ::windows::runtime::Array<u8>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt16Array(&self, value: &mut ::windows::runtime::Array<i16>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt16Array(&self, value: &mut ::windows::runtime::Array<u16>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt32Array(&self, value: &mut ::windows::runtime::Array<i32>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt32Array(&self, value: &mut ::windows::runtime::Array<u32>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt64Array(&self, value: &mut ::windows::runtime::Array<i64>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt64Array(&self, value: &mut ::windows::runtime::Array<u64>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSingleArray(&self, value: &mut ::windows::runtime::Array<f32>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDoubleArray(&self, value: &mut ::windows::runtime::Array<f64>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetChar16Array(&self, value: &mut ::windows::runtime::Array<u16>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetBooleanArray(&self, value: &mut ::windows::runtime::Array<bool>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetStringArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::HSTRING>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInspectableArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetGuidArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::GUID>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDateTimeArray(&self, value: &mut ::windows::runtime::Array<DateTime>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetTimeSpanArray(&self, value: &mut ::windows::runtime::Array<TimeSpan>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetPointArray(&self, value: &mut ::windows::runtime::Array<Point>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSizeArray(&self, value: &mut ::windows::runtime::Array<Size>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetRectArray(&self, value: &mut ::windows::runtime::Array<Rect>) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IPropertyValue {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{4bd682dd-7554-40e9-9a9b-82654ede7e62}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyValue_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PropertyType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut DateTime) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut TimeSpan) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Point) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Size) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Rect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut DateTime) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut TimeSpan) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut Point) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut Size) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: *mut u32, value: *mut *mut Rect) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IPropertyValueStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPropertyValueStatics {
    type Vtable = IPropertyValueStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1654381512, 55602, 20468, [150, 185, 141, 150, 197, 193, 232, 88]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyValueStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: DateTime, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Point, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Size, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: Rect, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u8, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const i16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const i64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const f32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const f64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const u16, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const bool, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const DateTime, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const TimeSpan, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const Point, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const Size, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value_array_size: u32, value: *const Rect, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IReference<T>(::windows::runtime::IInspectable, ::std::marker::PhantomData<T>)
where
    T: ::windows::runtime::RuntimeType + 'static;
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IReference<T> {
    type Vtable = IReference_abi<T>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IReference<T> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<T: ::windows::runtime::RuntimeType + 'static> IReference<T> {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<T> {
        let this = self;
        unsafe {
            let mut result__: <T as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<T>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<PropertyType> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: PropertyType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PropertyType>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn IsNumericScalar(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt8(&self) -> ::windows::runtime::Result<u8> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt16(&self) -> ::windows::runtime::Result<i16> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: i16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt16(&self) -> ::windows::runtime::Result<u16> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt32(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt32(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt64(&self) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt64(&self) -> ::windows::runtime::Result<u64> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSingle(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDouble(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetChar16(&self) -> ::windows::runtime::Result<u16> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetBoolean(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetGuid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDateTime(&self) -> ::windows::runtime::Result<DateTime> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetTimeSpan(&self) -> ::windows::runtime::Result<TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetPoint(&self) -> ::windows::runtime::Result<Point> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Point>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSize(&self) -> ::windows::runtime::Result<Size> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Size>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetRect(&self) -> ::windows::runtime::Result<Rect> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Rect>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt8Array(&self, value: &mut ::windows::runtime::Array<u8>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt16Array(&self, value: &mut ::windows::runtime::Array<i16>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt16Array(&self, value: &mut ::windows::runtime::Array<u16>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt32Array(&self, value: &mut ::windows::runtime::Array<i32>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt32Array(&self, value: &mut ::windows::runtime::Array<u32>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt64Array(&self, value: &mut ::windows::runtime::Array<i64>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt64Array(&self, value: &mut ::windows::runtime::Array<u64>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSingleArray(&self, value: &mut ::windows::runtime::Array<f32>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDoubleArray(&self, value: &mut ::windows::runtime::Array<f64>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetChar16Array(&self, value: &mut ::windows::runtime::Array<u16>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetBooleanArray(&self, value: &mut ::windows::runtime::Array<bool>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetStringArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::HSTRING>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInspectableArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetGuidArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::GUID>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDateTimeArray(&self, value: &mut ::windows::runtime::Array<DateTime>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetTimeSpanArray(&self, value: &mut ::windows::runtime::Array<TimeSpan>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetPointArray(&self, value: &mut ::windows::runtime::Array<Point>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSizeArray(&self, value: &mut ::windows::runtime::Array<Size>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetRectArray(&self, value: &mut ::windows::runtime::Array<Rect>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IReference<T> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{61c17706-2d65-11e0-9ae8-d48564015472}").push_slice(b";").push_other(<T as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::std::convert::TryFrom<IReference<T>> for IPropertyValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IReference<T>) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::std::convert::TryFrom<&IReference<T>> for IPropertyValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IReference<T>) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IPropertyValue> for IReference<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IPropertyValue> for &IReference<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyValue> {
        ::std::convert::TryInto::<IPropertyValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReference_abi<T>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut <T as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub ::std::marker::PhantomData<T>,
)
where
    T: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `Foundation`*"]
pub struct IReferenceArray<T>(::windows::runtime::IInspectable, ::std::marker::PhantomData<T>)
where
    T: ::windows::runtime::RuntimeType + 'static;
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for IReferenceArray<T> {
    type Vtable = IReferenceArray_abi<T>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<IReferenceArray<T> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
impl<T: ::windows::runtime::RuntimeType + 'static> IReferenceArray<T> {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::Array<T>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::Array<T> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), ::windows::runtime::Array::<T>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Type(&self) -> ::windows::runtime::Result<PropertyType> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: PropertyType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PropertyType>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn IsNumericScalar(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt8(&self) -> ::windows::runtime::Result<u8> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u8 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u8>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt16(&self) -> ::windows::runtime::Result<i16> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: i16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt16(&self) -> ::windows::runtime::Result<u16> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt32(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt32(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt64(&self) -> ::windows::runtime::Result<i64> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt64(&self) -> ::windows::runtime::Result<u64> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSingle(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDouble(&self) -> ::windows::runtime::Result<f64> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetChar16(&self) -> ::windows::runtime::Result<u16> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: u16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetBoolean(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetGuid(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDateTime(&self) -> ::windows::runtime::Result<DateTime> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DateTime>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetTimeSpan(&self) -> ::windows::runtime::Result<TimeSpan> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), &mut result__).from_abi::<TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetPoint(&self) -> ::windows::runtime::Result<Point> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Point>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSize(&self) -> ::windows::runtime::Result<Size> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Size>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetRect(&self) -> ::windows::runtime::Result<Rect> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe {
            let mut result__: Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Rect>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt8Array(&self, value: &mut ::windows::runtime::Array<u8>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt16Array(&self, value: &mut ::windows::runtime::Array<i16>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt16Array(&self, value: &mut ::windows::runtime::Array<u16>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt32Array(&self, value: &mut ::windows::runtime::Array<i32>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt32Array(&self, value: &mut ::windows::runtime::Array<u32>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInt64Array(&self, value: &mut ::windows::runtime::Array<i64>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetUInt64Array(&self, value: &mut ::windows::runtime::Array<u64>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSingleArray(&self, value: &mut ::windows::runtime::Array<f32>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDoubleArray(&self, value: &mut ::windows::runtime::Array<f64>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetChar16Array(&self, value: &mut ::windows::runtime::Array<u16>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetBooleanArray(&self, value: &mut ::windows::runtime::Array<bool>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetStringArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::HSTRING>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetInspectableArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::IInspectable>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetGuidArray(&self, value: &mut ::windows::runtime::Array<::windows::runtime::GUID>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetDateTimeArray(&self, value: &mut ::windows::runtime::Array<DateTime>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetTimeSpanArray(&self, value: &mut ::windows::runtime::Array<TimeSpan>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetPointArray(&self, value: &mut ::windows::runtime::Array<Point>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetSizeArray(&self, value: &mut ::windows::runtime::Array<Size>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetRectArray(&self, value: &mut ::windows::runtime::Array<Rect>) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPropertyValue>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), value.set_abi_len(), value as *mut _ as _).ok() }
    }
}
unsafe impl<T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for IReferenceArray<T> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = { ::windows::runtime::ConstBuffer::new().push_slice(b"pinterface(").push_slice(b"{61c17707-2d65-11e0-9ae8-d48564015472}").push_slice(b";").push_other(<T as ::windows::runtime::RuntimeType>::SIGNATURE).push_slice(b")") };
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::std::convert::TryFrom<IReferenceArray<T>> for IPropertyValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IReferenceArray<T>) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl<T: ::windows::runtime::RuntimeType + 'static> ::std::convert::TryFrom<&IReferenceArray<T>> for IPropertyValue {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IReferenceArray<T>) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IPropertyValue> for IReferenceArray<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyValue> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a, T: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::IntoParam<'a, IPropertyValue> for &IReferenceArray<T> {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPropertyValue> {
        ::std::convert::TryInto::<IPropertyValue>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceArray_abi<T>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result_size__: *mut u32, result__: *mut *mut <T as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub ::std::marker::PhantomData<T>,
)
where
    T: ::windows::runtime::RuntimeType + 'static;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation`*"]
pub struct IStringable(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IStringable {
    type Vtable = IStringable_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2520162132, 36534, 18672, [171, 206, 193, 178, 17, 230, 39, 195]);
}
impl IStringable {
    #[doc = "*Required features: `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IStringable {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{96369f54-8eb6-48f0-abce-c1b211e627c3}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IStringable_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IUriEscapeStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUriEscapeStatics {
    type Vtable = IUriEscapeStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3251909306, 51236, 17490, [167, 253, 81, 43, 195, 187, 233, 161]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriEscapeStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tounescape: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, toescape: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IUriRuntimeClass(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUriRuntimeClass {
    type Vtable = IUriRuntimeClass_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2654363223, 18610, 16736, [149, 111, 199, 56, 81, 32, 187, 252]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClass_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, puri: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, relativeuri: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IUriRuntimeClassFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUriRuntimeClassFactory {
    type Vtable = IUriRuntimeClassFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1151957359, 29246, 20447, [162, 24, 3, 62, 117, 176, 192, 132]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClassFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uri: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baseuri: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, relativeuri: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUriRuntimeClassWithAbsoluteCanonicalUri {
    type Vtable = IUriRuntimeClassWithAbsoluteCanonicalUri_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1972213345, 8732, 18447, [163, 57, 80, 101, 102, 115, 244, 111]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation`*"]
pub struct IWwwFormUrlDecoderEntry(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWwwFormUrlDecoderEntry {
    type Vtable = IWwwFormUrlDecoderEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(308180017, 63096, 20110, [182, 112, 32, 169, 176, 108, 81, 45]);
}
impl IWwwFormUrlDecoderEntry {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IWwwFormUrlDecoderEntry {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{125e7431-f678-4e8e-b670-20a9b06c512d}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderEntry_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderRuntimeClass(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWwwFormUrlDecoderRuntimeClass {
    type Vtable = IWwwFormUrlDecoderRuntimeClass_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3562669137, 61989, 17730, [146, 150, 14, 29, 245, 210, 84, 223]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderRuntimeClass_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderRuntimeClassFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWwwFormUrlDecoderRuntimeClassFactory {
    type Vtable = IWwwFormUrlDecoderRuntimeClassFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1535929149, 9390, 16821, [161, 191, 240, 195, 213, 68, 132, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWwwFormUrlDecoderRuntimeClassFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, query: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MemoryBuffer(::windows::runtime::IInspectable);
impl MemoryBuffer {
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateReference(&self) -> ::windows::runtime::Result<IMemoryBufferReference> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IMemoryBufferReference>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Create(capacity: u32) -> ::windows::runtime::Result<MemoryBuffer> {
        Self::IMemoryBufferFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), capacity, &mut result__).from_abi::<MemoryBuffer>(result__)
        })
    }
    pub fn IMemoryBufferFactory<R, F: FnOnce(&IMemoryBufferFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MemoryBuffer, IMemoryBufferFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MemoryBuffer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.MemoryBuffer;{fbc4dd2a-245b-11e4-af98-689423260cf8})");
}
unsafe impl ::windows::runtime::Interface for MemoryBuffer {
    type Vtable = IMemoryBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4223982890, 9307, 4580, [175, 152, 104, 148, 35, 38, 12, 248]);
}
impl ::windows::runtime::RuntimeName for MemoryBuffer {
    const NAME: &'static str = "Windows.Foundation.MemoryBuffer";
}
impl ::std::convert::From<MemoryBuffer> for IMemoryBuffer {
    fn from(value: MemoryBuffer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&MemoryBuffer> for IMemoryBuffer {
    fn from(value: &MemoryBuffer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMemoryBuffer> for MemoryBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMemoryBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMemoryBuffer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IMemoryBuffer> for &MemoryBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IMemoryBuffer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IMemoryBuffer>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<MemoryBuffer> for IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: MemoryBuffer) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&MemoryBuffer> for IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &MemoryBuffer) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IClosable> for MemoryBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IClosable> for &MemoryBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IClosable> {
        ::std::convert::TryInto::<IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for MemoryBuffer {}
unsafe impl ::std::marker::Sync for MemoryBuffer {}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Foundation`*"]
pub struct Point {
    pub X: f32,
    pub Y: f32,
}
impl Point {}
impl ::std::default::Default for Point {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Point {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Point").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::std::cmp::Eq for Point {}
unsafe impl ::windows::runtime::Abi for Point {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Point {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Point;f4;f4)");
}
#[doc = "*Required features: `Foundation`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PropertyType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PropertyType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PropertyType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.PropertyType;i4)");
}
#[doc = "*Required features: `Foundation`*"]
pub struct PropertyValue {}
impl PropertyValue {
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateEmpty() -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt8(value: u8) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInt16(value: i16) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt16(value: u16) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInt32(value: i32) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt32(value: u32) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInt64(value: i64) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt64(value: u64) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateSingle(value: f32) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateDouble(value: f64) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateChar16(value: u16) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateBoolean(value: bool) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInspectable<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateGuid<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateDateTime<'a, Param0: ::windows::runtime::IntoParam<'a, DateTime>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateTimeSpan<'a, Param0: ::windows::runtime::IntoParam<'a, TimeSpan>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreatePoint<'a, Param0: ::windows::runtime::IntoParam<'a, Point>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateSize<'a, Param0: ::windows::runtime::IntoParam<'a, Size>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateRect<'a, Param0: ::windows::runtime::IntoParam<'a, Rect>>(value: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt8Array(value: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInt16Array(value: &[<i16 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt16Array(value: &[<u16 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInt32Array(value: &[<i32 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt32Array(value: &[<u32 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInt64Array(value: &[<i64 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUInt64Array(value: &[<u64 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateSingleArray(value: &[<f32 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateDoubleArray(value: &[<f64 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateChar16Array(value: &[<u16 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateBooleanArray(value: &[<bool as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateStringArray(value: &[<::windows::runtime::HSTRING as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateInspectableArray(value: &[<::windows::runtime::IInspectable as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateGuidArray(value: &[<::windows::runtime::GUID as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateDateTimeArray(value: &[<DateTime as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateTimeSpanArray(value: &[<TimeSpan as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreatePointArray(value: &[<Point as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateSizeArray(value: &[<Size as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateRectArray(value: &[<Rect as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        Self::IPropertyValueStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), value.len() as u32, ::std::mem::transmute(value.as_ptr()), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        })
    }
    pub fn IPropertyValueStatics<R, F: FnOnce(&IPropertyValueStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PropertyValue, IPropertyValueStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for PropertyValue {
    const NAME: &'static str = "Windows.Foundation.PropertyValue";
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Foundation`*"]
pub struct Rect {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
impl Rect {}
impl ::std::default::Default for Rect {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Rect {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Rect").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::std::cmp::PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::std::cmp::Eq for Rect {}
unsafe impl ::windows::runtime::Abi for Rect {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Rect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Rect;f4;f4;f4;f4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Foundation`*"]
pub struct Size {
    pub Width: f32,
    pub Height: f32,
}
impl Size {}
impl ::std::default::Default for Size {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for Size {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("Size").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::std::cmp::PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::std::cmp::Eq for Size {}
unsafe impl ::windows::runtime::Abi for Size {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Size {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.Size;f4;f4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Foundation`*"]
pub struct TimeSpan {
    pub Duration: i64,
}
impl TimeSpan {}
impl ::std::default::Default for TimeSpan {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TimeSpan {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TimeSpan").field("Duration", &self.Duration).finish()
    }
}
impl ::std::cmp::PartialEq for TimeSpan {
    fn eq(&self, other: &Self) -> bool {
        self.Duration == other.Duration
    }
}
impl ::std::cmp::Eq for TimeSpan {}
unsafe impl ::windows::runtime::Abi for TimeSpan {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for TimeSpan {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Foundation.TimeSpan;i8)");
}
impl ::std::convert::From<::std::time::Duration> for TimeSpan {
    fn from(value: ::std::time::Duration) -> Self {
        Self { Duration: (value.as_nanos() / 100) as i64 }
    }
}
impl ::std::convert::From<TimeSpan> for ::std::time::Duration {
    fn from(value: TimeSpan) -> Self {
        ::std::time::Duration::from_nanos((value.Duration * 100) as u64)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, TimeSpan> for ::std::time::Duration {
    fn into_param(self) -> ::windows::runtime::Param<'a, TimeSpan> {
        ::windows::runtime::Param::Owned(self.into())
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct TypedEventHandler<TSender, TResult>(::windows::runtime::IUnknown, ::std::marker::PhantomData<TSender>, ::std::marker::PhantomData<TResult>)
where
    TSender: ::windows::runtime::RuntimeType + 'static,
    TResult: ::windows::runtime::RuntimeType + 'static;
impl<TSender: ::windows::runtime::RuntimeType + 'static, TResult: ::windows::runtime::RuntimeType + 'static> TypedEventHandler<TSender, TResult> {
    pub fn new<F: FnMut(&<TSender as ::windows::runtime::Abi>::DefaultType, &<TResult as ::windows::runtime::Abi>::DefaultType) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = TypedEventHandler_box::<TSender, TResult, F> {
            vtable: &TypedEventHandler_box::<TSender, TResult, F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, TSender>, Param1: ::windows::runtime::IntoParam<'a, TResult>>(&self, sender: Param0, args: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), sender.into_param().abi(), args.into_param().abi()).ok() }
    }
}
unsafe impl<TSender: ::windows::runtime::RuntimeType + 'static, TResult: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::RuntimeType for TypedEventHandler<TSender, TResult> {
    const SIGNATURE: ::windows::runtime::ConstBuffer = {
        ::windows::runtime::ConstBuffer::new()
            .push_slice(b"pinterface(")
            .push_slice(b"{9de1c534-6ae1-11e0-84e1-18a905bcc53f}")
            .push_slice(b";")
            .push_other(<TSender as ::windows::runtime::RuntimeType>::SIGNATURE)
            .push_slice(b";")
            .push_other(<TResult as ::windows::runtime::RuntimeType>::SIGNATURE)
            .push_slice(b")")
    };
}
unsafe impl<TSender: ::windows::runtime::RuntimeType + 'static, TResult: ::windows::runtime::RuntimeType + 'static> ::windows::runtime::Interface for TypedEventHandler<TSender, TResult> {
    type Vtable = TypedEventHandler_abi<TSender, TResult>;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_signature(<TypedEventHandler<TSender, TResult> as ::windows::runtime::RuntimeType>::SIGNATURE);
}
#[repr(C)]
#[doc(hidden)]
pub struct TypedEventHandler_abi<TSender, TResult>(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sender: <TSender as ::windows::runtime::Abi>::Abi, args: <TResult as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT,
    pub ::std::marker::PhantomData<TSender>,
    pub ::std::marker::PhantomData<TResult>,
)
where
    TSender: ::windows::runtime::RuntimeType + 'static,
    TResult: ::windows::runtime::RuntimeType + 'static;
#[repr(C)]
struct TypedEventHandler_box<TSender, TResult, F: FnMut(&<TSender as ::windows::runtime::Abi>::DefaultType, &<TResult as ::windows::runtime::Abi>::DefaultType) -> ::windows::runtime::Result<()> + 'static>
where
    TSender: ::windows::runtime::RuntimeType + 'static,
    TResult: ::windows::runtime::RuntimeType + 'static,
{
    vtable: *const TypedEventHandler_abi<TSender, TResult>,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<TSender: ::windows::runtime::RuntimeType + 'static, TResult: ::windows::runtime::RuntimeType + 'static, F: FnMut(&<TSender as ::windows::runtime::Abi>::DefaultType, &<TResult as ::windows::runtime::Abi>::DefaultType) -> ::windows::runtime::Result<()> + 'static> TypedEventHandler_box<TSender, TResult, F> {
    const VTABLE: TypedEventHandler_abi<TSender, TResult> = TypedEventHandler_abi::<TSender, TResult>(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke, ::std::marker::PhantomData::<TSender>, ::std::marker::PhantomData::<TResult>);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<TypedEventHandler<TSender, TResult> as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, sender: <TSender as ::windows::runtime::Abi>::Abi, args: <TResult as ::windows::runtime::Abi>::Abi) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&sender as *const <TSender as ::windows::runtime::Abi>::Abi as *const <TSender as ::windows::runtime::Abi>::DefaultType), &*(&args as *const <TResult as ::windows::runtime::Abi>::Abi as *const <TResult as ::windows::runtime::Abi>::DefaultType)).into()
    }
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct UniversalApiContract(pub u8);
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct Uri(::windows::runtime::IInspectable);
impl Uri {
    #[doc = "*Required features: `Foundation`*"]
    pub fn AbsoluteUri(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn DisplayUri(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Domain(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Extension(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Fragment(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Host(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Password(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Path(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Query(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn QueryParsed(&self) -> ::windows::runtime::Result<WwwFormUrlDecoder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<WwwFormUrlDecoder>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn RawUri(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn SchemeName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn UserName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Port(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Suspicious(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Equals<'a, Param0: ::windows::runtime::IntoParam<'a, Uri>>(&self, puri: Param0) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), puri.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CombineUri<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, relativeuri: Param0) -> ::windows::runtime::Result<Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), relativeuri.into_param().abi(), &mut result__).from_abi::<Uri>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn ToString(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IStringable>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn AbsoluteCanonicalUri(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn DisplayIri(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn UnescapeComponent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(tounescape: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), tounescape.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn EscapeComponent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(toescape: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), toescape.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateUri<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(uri: Param0) -> ::windows::runtime::Result<Uri> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<Uri>(result__)
        })
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateWithRelativeUri<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(baseuri: Param0, relativeuri: Param1) -> ::windows::runtime::Result<Uri> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), baseuri.into_param().abi(), relativeuri.into_param().abi(), &mut result__).from_abi::<Uri>(result__)
        })
    }
    pub fn IUriEscapeStatics<R, F: FnOnce(&IUriEscapeStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Uri, IUriEscapeStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IUriRuntimeClassFactory<R, F: FnOnce(&IUriRuntimeClassFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<Uri, IUriRuntimeClassFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for Uri {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Uri;{9e365e57-48b2-4160-956f-c7385120bbfc})");
}
unsafe impl ::windows::runtime::Interface for Uri {
    type Vtable = IUriRuntimeClass_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2654363223, 18610, 16736, [149, 111, 199, 56, 81, 32, 187, 252]);
}
impl ::windows::runtime::RuntimeName for Uri {
    const NAME: &'static str = "Windows.Foundation.Uri";
}
impl ::std::convert::TryFrom<Uri> for IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: Uri) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&Uri> for IStringable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &Uri) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStringable> for Uri {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStringable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStringable> for &Uri {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStringable> {
        ::std::convert::TryInto::<IStringable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for Uri {}
unsafe impl ::std::marker::Sync for Uri {}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WwwFormUrlDecoder(::windows::runtime::IInspectable);
impl WwwFormUrlDecoder {
    #[doc = "*Required features: `Foundation`*"]
    pub fn GetFirstValueByName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Foundation`, `Foundation_Collections`*"]
    pub fn First(&self) -> ::windows::runtime::Result<Collections::IIterator<IWwwFormUrlDecoderEntry>> {
        let this = &::windows::runtime::Interface::cast::<Collections::IIterable<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<Collections::IIterator<IWwwFormUrlDecoderEntry>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Foundation`, `Foundation_Collections`*"]
    pub fn GetAt(&self, index: u32) -> ::windows::runtime::Result<IWwwFormUrlDecoderEntry> {
        let this = &::windows::runtime::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), index, &mut result__).from_abi::<IWwwFormUrlDecoderEntry>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Foundation`, `Foundation_Collections`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Foundation`, `Foundation_Collections`*"]
    pub fn IndexOf<'a, Param0: ::windows::runtime::IntoParam<'a, IWwwFormUrlDecoderEntry>>(&self, value: Param0, index: &mut u32) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `Foundation`, `Foundation_Collections`*"]
    pub fn GetMany(&self, startindex: u32, items: &mut [<IWwwFormUrlDecoderEntry as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), startindex, items.len() as u32, ::std::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn CreateWwwFormUrlDecoder<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(query: Param0) -> ::windows::runtime::Result<WwwFormUrlDecoder> {
        Self::IWwwFormUrlDecoderRuntimeClassFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), query.into_param().abi(), &mut result__).from_abi::<WwwFormUrlDecoder>(result__)
        })
    }
    pub fn IWwwFormUrlDecoderRuntimeClassFactory<R, F: FnOnce(&IWwwFormUrlDecoderRuntimeClassFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WwwFormUrlDecoder, IWwwFormUrlDecoderRuntimeClassFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WwwFormUrlDecoder {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.WwwFormUrlDecoder;{d45a0451-f225-4542-9296-0e1df5d254df})");
}
unsafe impl ::windows::runtime::Interface for WwwFormUrlDecoder {
    type Vtable = IWwwFormUrlDecoderRuntimeClass_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3562669137, 61989, 17730, [146, 150, 14, 29, 245, 210, 84, 223]);
}
impl ::windows::runtime::RuntimeName for WwwFormUrlDecoder {
    const NAME: &'static str = "Windows.Foundation.WwwFormUrlDecoder";
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<WwwFormUrlDecoder> for Collections::IIterable<IWwwFormUrlDecoderEntry> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WwwFormUrlDecoder) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&WwwFormUrlDecoder> for Collections::IIterable<IWwwFormUrlDecoderEntry> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WwwFormUrlDecoder) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, Collections::IIterable<IWwwFormUrlDecoderEntry>> for WwwFormUrlDecoder {
    fn into_param(self) -> ::windows::runtime::Param<'a, Collections::IIterable<IWwwFormUrlDecoderEntry>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, Collections::IIterable<IWwwFormUrlDecoderEntry>> for &WwwFormUrlDecoder {
    fn into_param(self) -> ::windows::runtime::Param<'a, Collections::IIterable<IWwwFormUrlDecoderEntry>> {
        ::std::convert::TryInto::<Collections::IIterable<IWwwFormUrlDecoderEntry>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<WwwFormUrlDecoder> for Collections::IVectorView<IWwwFormUrlDecoderEntry> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WwwFormUrlDecoder) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::std::convert::TryFrom<&WwwFormUrlDecoder> for Collections::IVectorView<IWwwFormUrlDecoderEntry> {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WwwFormUrlDecoder) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, Collections::IVectorView<IWwwFormUrlDecoderEntry>> for WwwFormUrlDecoder {
    fn into_param(self) -> ::windows::runtime::Param<'a, Collections::IVectorView<IWwwFormUrlDecoderEntry>> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::runtime::IntoParam<'a, Collections::IVectorView<IWwwFormUrlDecoderEntry>> for &WwwFormUrlDecoder {
    fn into_param(self) -> ::windows::runtime::Param<'a, Collections::IVectorView<IWwwFormUrlDecoderEntry>> {
        ::std::convert::TryInto::<Collections::IVectorView<IWwwFormUrlDecoderEntry>>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for WwwFormUrlDecoder {}
unsafe impl ::std::marker::Sync for WwwFormUrlDecoder {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::std::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::std::iter::IntoIterator for &WwwFormUrlDecoder {
    type Item = IWwwFormUrlDecoderEntry;
    type IntoIter = Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        Collections::VectorViewIterator::new(::std::convert::TryInto::try_into(self).ok())
    }
}
#[doc = "*Required features: `Foundation`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct WwwFormUrlDecoderEntry(::windows::runtime::IInspectable);
impl WwwFormUrlDecoderEntry {
    #[doc = "*Required features: `Foundation`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation`*"]
    pub fn Value(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WwwFormUrlDecoderEntry {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.WwwFormUrlDecoderEntry;{125e7431-f678-4e8e-b670-20a9b06c512d})");
}
unsafe impl ::windows::runtime::Interface for WwwFormUrlDecoderEntry {
    type Vtable = IWwwFormUrlDecoderEntry_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(308180017, 63096, 20110, [182, 112, 32, 169, 176, 108, 81, 45]);
}
impl ::windows::runtime::RuntimeName for WwwFormUrlDecoderEntry {
    const NAME: &'static str = "Windows.Foundation.WwwFormUrlDecoderEntry";
}
impl ::std::convert::From<WwwFormUrlDecoderEntry> for IWwwFormUrlDecoderEntry {
    fn from(value: WwwFormUrlDecoderEntry) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WwwFormUrlDecoderEntry> for IWwwFormUrlDecoderEntry {
    fn from(value: &WwwFormUrlDecoderEntry) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWwwFormUrlDecoderEntry> for WwwFormUrlDecoderEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWwwFormUrlDecoderEntry> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWwwFormUrlDecoderEntry>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWwwFormUrlDecoderEntry> for &WwwFormUrlDecoderEntry {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWwwFormUrlDecoderEntry> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWwwFormUrlDecoderEntry>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for WwwFormUrlDecoderEntry {}
unsafe impl ::std::marker::Sync for WwwFormUrlDecoderEntry {}
