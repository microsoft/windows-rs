#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "System_Threading_Core")]
pub mod Core;
#[repr(transparent)]
#[doc(hidden)]
pub struct IThreadPoolStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IThreadPoolStatics {
    type Vtable = IThreadPoolStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6bf67dd_84bd_44f8_ac1c_93ebcb9dba91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadPoolStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, priority: WorkItemPriority, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, priority: WorkItemPriority, options: WorkItemOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IThreadPoolTimer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IThreadPoolTimer {
    type Vtable = IThreadPoolTimer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594ebe78_55ea_4a88_a50d_3402ae1f9cf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadPoolTimer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IThreadPoolTimerStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IThreadPoolTimerStatics {
    type Vtable = IThreadPoolTimerStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a8a9d02_e482_461b_b8c7_8efad1cce590);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadPoolTimerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, period: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, delay: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, period: super::super::Foundation::TimeSpan, destroyed: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, delay: super::super::Foundation::TimeSpan, destroyed: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc = "*Required features: `System_Threading`*"]
pub struct ThreadPool {}
impl ThreadPool {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Threading`, `Foundation`*"]
    pub fn RunAsync<'a, Param0: ::windows::core::IntoParam<'a, WorkItemHandler>>(handler: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IThreadPoolStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Threading`, `Foundation`*"]
    pub fn RunWithPriorityAsync<'a, Param0: ::windows::core::IntoParam<'a, WorkItemHandler>>(handler: Param0, priority: WorkItemPriority) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IThreadPoolStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), priority, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Threading`, `Foundation`*"]
    pub fn RunWithPriorityAndOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, WorkItemHandler>>(handler: Param0, priority: WorkItemPriority, options: WorkItemOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IThreadPoolStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), priority, options, &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IThreadPoolStatics<R, F: FnOnce(&IThreadPoolStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ThreadPool, IThreadPoolStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for ThreadPool {
    const NAME: &'static str = "Windows.System.Threading.ThreadPool";
}
#[doc = "*Required features: `System_Threading`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ThreadPoolTimer(pub ::windows::core::IInspectable);
impl ThreadPoolTimer {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Threading`, `Foundation`*"]
    pub fn Period(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Threading`, `Foundation`*"]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `System_Threading`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Threading`, `Foundation`*"]
    pub fn CreatePeriodicTimer<'a, Param0: ::windows::core::IntoParam<'a, TimerElapsedHandler>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(handler: Param0, period: Param1) -> ::windows::core::Result<ThreadPoolTimer> {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), period.into_param().abi(), &mut result__).from_abi::<ThreadPoolTimer>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Threading`, `Foundation`*"]
    pub fn CreateTimer<'a, Param0: ::windows::core::IntoParam<'a, TimerElapsedHandler>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>>(handler: Param0, delay: Param1) -> ::windows::core::Result<ThreadPoolTimer> {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), handler.into_param().abi(), delay.into_param().abi(), &mut result__).from_abi::<ThreadPoolTimer>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Threading`, `Foundation`*"]
    pub fn CreatePeriodicTimerWithCompletion<'a, Param0: ::windows::core::IntoParam<'a, TimerElapsedHandler>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param2: ::windows::core::IntoParam<'a, TimerDestroyedHandler>>(handler: Param0, period: Param1, destroyed: Param2) -> ::windows::core::Result<ThreadPoolTimer> {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), period.into_param().abi(), destroyed.into_param().abi(), &mut result__).from_abi::<ThreadPoolTimer>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Threading`, `Foundation`*"]
    pub fn CreateTimerWithCompletion<'a, Param0: ::windows::core::IntoParam<'a, TimerElapsedHandler>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::TimeSpan>, Param2: ::windows::core::IntoParam<'a, TimerDestroyedHandler>>(handler: Param0, delay: Param1, destroyed: Param2) -> ::windows::core::Result<ThreadPoolTimer> {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), handler.into_param().abi(), delay.into_param().abi(), destroyed.into_param().abi(), &mut result__).from_abi::<ThreadPoolTimer>(result__)
        })
    }
    pub fn IThreadPoolTimerStatics<R, F: FnOnce(&IThreadPoolTimerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ThreadPoolTimer, IThreadPoolTimerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for ThreadPoolTimer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Threading.ThreadPoolTimer;{594ebe78-55ea-4a88-a50d-3402ae1f9cf2})");
}
unsafe impl ::windows::core::Interface for ThreadPoolTimer {
    type Vtable = IThreadPoolTimer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594ebe78_55ea_4a88_a50d_3402ae1f9cf2);
}
impl ::windows::core::RuntimeName for ThreadPoolTimer {
    const NAME: &'static str = "Windows.System.Threading.ThreadPoolTimer";
}
impl ::core::convert::From<ThreadPoolTimer> for ::windows::core::IUnknown {
    fn from(value: ThreadPoolTimer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ThreadPoolTimer> for ::windows::core::IUnknown {
    fn from(value: &ThreadPoolTimer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ThreadPoolTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ThreadPoolTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ThreadPoolTimer> for ::windows::core::IInspectable {
    fn from(value: ThreadPoolTimer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ThreadPoolTimer> for ::windows::core::IInspectable {
    fn from(value: &ThreadPoolTimer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ThreadPoolTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ThreadPoolTimer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for ThreadPoolTimer {}
unsafe impl ::core::marker::Sync for ThreadPoolTimer {}
#[doc = "*Required features: `System_Threading`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TimerDestroyedHandler(::windows::core::IUnknown);
impl TimerDestroyedHandler {
    pub fn new<F: FnMut(&::core::option::Option<ThreadPoolTimer>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = TimerDestroyedHandler_box::<F> {
            vtable: &TimerDestroyedHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `System_Threading`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ThreadPoolTimer>>(&self, timer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), timer.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for TimerDestroyedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({34ed19fa-8384-4eb9-8209-fb5094eeec35})");
}
unsafe impl ::windows::core::Interface for TimerDestroyedHandler {
    type Vtable = TimerDestroyedHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34ed19fa_8384_4eb9_8209_fb5094eeec35);
}
#[repr(C)]
#[doc(hidden)]
pub struct TimerDestroyedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct TimerDestroyedHandler_box<F: FnMut(&::core::option::Option<ThreadPoolTimer>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const TimerDestroyedHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<ThreadPoolTimer>) -> ::windows::core::Result<()> + 'static> TimerDestroyedHandler_box<F> {
    const VTABLE: TimerDestroyedHandler_abi = TimerDestroyedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<TimerDestroyedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, timer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&timer as *const <ThreadPoolTimer as ::windows::core::Abi>::Abi as *const <ThreadPoolTimer as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[doc = "*Required features: `System_Threading`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TimerElapsedHandler(::windows::core::IUnknown);
impl TimerElapsedHandler {
    pub fn new<F: FnMut(&::core::option::Option<ThreadPoolTimer>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = TimerElapsedHandler_box::<F> {
            vtable: &TimerElapsedHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `System_Threading`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ThreadPoolTimer>>(&self, timer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), timer.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for TimerElapsedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({faaea667-fbeb-49cb-adb2-71184c556e43})");
}
unsafe impl ::windows::core::Interface for TimerElapsedHandler {
    type Vtable = TimerElapsedHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaaea667_fbeb_49cb_adb2_71184c556e43);
}
#[repr(C)]
#[doc(hidden)]
pub struct TimerElapsedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, timer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct TimerElapsedHandler_box<F: FnMut(&::core::option::Option<ThreadPoolTimer>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const TimerElapsedHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<ThreadPoolTimer>) -> ::windows::core::Result<()> + 'static> TimerElapsedHandler_box<F> {
    const VTABLE: TimerElapsedHandler_abi = TimerElapsedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<TimerElapsedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, timer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&timer as *const <ThreadPoolTimer as ::windows::core::Abi>::Abi as *const <ThreadPoolTimer as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[cfg(feature = "Foundation")]
#[doc = "*Required features: `System_Threading`, `Foundation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct WorkItemHandler(::windows::core::IUnknown);
#[cfg(feature = "Foundation")]
impl WorkItemHandler {
    pub fn new<F: FnMut(&::core::option::Option<super::super::Foundation::IAsyncAction>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = WorkItemHandler_box::<F> {
            vtable: &WorkItemHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_Threading`, `Foundation`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::IAsyncAction>>(&self, operation: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), operation.into_param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for WorkItemHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({1d1a8b8b-fa66-414f-9cbd-b65fc99d17fa})");
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for WorkItemHandler {
    type Vtable = WorkItemHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d1a8b8b_fa66_414f_9cbd_b65fc99d17fa);
}
#[cfg(feature = "Foundation")]
#[repr(C)]
#[doc(hidden)]
pub struct WorkItemHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, operation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[cfg(feature = "Foundation")]
#[repr(C)]
struct WorkItemHandler_box<F: FnMut(&::core::option::Option<super::super::Foundation::IAsyncAction>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const WorkItemHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Foundation")]
impl<F: FnMut(&::core::option::Option<super::super::Foundation::IAsyncAction>) -> ::windows::core::Result<()> + 'static> WorkItemHandler_box<F> {
    const VTABLE: WorkItemHandler_abi = WorkItemHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<WorkItemHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, operation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&operation as *const <super::super::Foundation::IAsyncAction as ::windows::core::Abi>::Abi as *const <super::super::Foundation::IAsyncAction as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[doc = "*Required features: `System_Threading`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WorkItemOptions(pub u32);
impl WorkItemOptions {
    pub const None: WorkItemOptions = WorkItemOptions(0u32);
    pub const TimeSliced: WorkItemOptions = WorkItemOptions(1u32);
}
impl ::core::convert::From<u32> for WorkItemOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WorkItemOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WorkItemOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Threading.WorkItemOptions;u4)");
}
impl ::windows::core::DefaultType for WorkItemOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for WorkItemOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for WorkItemOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for WorkItemOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for WorkItemOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for WorkItemOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `System_Threading`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct WorkItemPriority(pub i32);
impl WorkItemPriority {
    pub const Low: WorkItemPriority = WorkItemPriority(-1i32);
    pub const Normal: WorkItemPriority = WorkItemPriority(0i32);
    pub const High: WorkItemPriority = WorkItemPriority(1i32);
}
impl ::core::convert::From<i32> for WorkItemPriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for WorkItemPriority {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for WorkItemPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Threading.WorkItemPriority;i4)");
}
impl ::windows::core::DefaultType for WorkItemPriority {
    type DefaultType = Self;
}
