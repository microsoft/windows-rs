#[cfg(feature = "System_Threading_Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct IThreadPoolStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IThreadPoolStatics {
    type Vtable = IThreadPoolStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6bf67dd_84bd_44f8_ac1c_93ebcb9dba91);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadPoolStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RunAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RunWithPriorityAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, priority: WorkItemPriority, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunWithPriorityAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RunWithPriorityAndOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, priority: WorkItemPriority, options: WorkItemOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunWithPriorityAndOptionsAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IThreadPoolTimer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IThreadPoolTimer {
    type Vtable = IThreadPoolTimer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x594ebe78_55ea_4a88_a50d_3402ae1f9cf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadPoolTimer_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Period: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Period: usize,
    #[cfg(feature = "Foundation")]
    pub Delay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Delay: usize,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IThreadPoolTimerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IThreadPoolTimerStatics {
    type Vtable = IThreadPoolTimerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a8a9d02_e482_461b_b8c7_8efad1cce590);
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadPoolTimerStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreatePeriodicTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, period: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePeriodicTimer: usize,
    #[cfg(feature = "Foundation")]
    pub CreateTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, delay: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateTimer: usize,
    #[cfg(feature = "Foundation")]
    pub CreatePeriodicTimerWithCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, period: super::super::Foundation::TimeSpan, destroyed: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreatePeriodicTimerWithCompletion: usize,
    #[cfg(feature = "Foundation")]
    pub CreateTimerWithCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, delay: super::super::Foundation::TimeSpan, destroyed: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateTimerWithCompletion: usize,
}
#[doc = "*Required features: `\"System_Threading\"`*"]
pub struct ThreadPool;
impl ThreadPool {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RunAsync<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, WorkItemHandler>>,
    {
        Self::IThreadPoolStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RunAsync)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RunWithPriorityAsync<'a, P0>(handler: P0, priority: WorkItemPriority) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, WorkItemHandler>>,
    {
        Self::IThreadPoolStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RunWithPriorityAsync)(::windows::core::Interface::as_raw(this), handler.into().abi(), priority, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RunWithPriorityAndOptionsAsync<'a, P0>(handler: P0, priority: WorkItemPriority, options: WorkItemOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, WorkItemHandler>>,
    {
        Self::IThreadPoolStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RunWithPriorityAndOptionsAsync)(::windows::core::Interface::as_raw(this), handler.into().abi(), priority, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IThreadPoolStatics<R, F: FnOnce(&IThreadPoolStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ThreadPool, IThreadPoolStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for ThreadPool {
    const NAME: &'static str = "Windows.System.Threading.ThreadPool";
}
#[doc = "*Required features: `\"System_Threading\"`*"]
#[repr(transparent)]
pub struct ThreadPoolTimer(::windows::core::IUnknown);
impl ThreadPoolTimer {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Period(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Period)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Delay)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Cancel)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePeriodicTimer<'a, P0>(handler: P0, period: super::super::Foundation::TimeSpan) -> ::windows::core::Result<ThreadPoolTimer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, TimerElapsedHandler>>,
    {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreatePeriodicTimer)(::windows::core::Interface::as_raw(this), handler.into().abi(), period, result__.as_mut_ptr()).from_abi::<ThreadPoolTimer>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateTimer<'a, P0>(handler: P0, delay: super::super::Foundation::TimeSpan) -> ::windows::core::Result<ThreadPoolTimer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, TimerElapsedHandler>>,
    {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateTimer)(::windows::core::Interface::as_raw(this), handler.into().abi(), delay, result__.as_mut_ptr()).from_abi::<ThreadPoolTimer>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreatePeriodicTimerWithCompletion<'a, P0, P1>(handler: P0, period: super::super::Foundation::TimeSpan, destroyed: P1) -> ::windows::core::Result<ThreadPoolTimer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, TimerElapsedHandler>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, TimerDestroyedHandler>>,
    {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreatePeriodicTimerWithCompletion)(::windows::core::Interface::as_raw(this), handler.into().abi(), period, destroyed.into().abi(), result__.as_mut_ptr()).from_abi::<ThreadPoolTimer>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateTimerWithCompletion<'a, P0, P1>(handler: P0, delay: super::super::Foundation::TimeSpan, destroyed: P1) -> ::windows::core::Result<ThreadPoolTimer>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, TimerElapsedHandler>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, TimerDestroyedHandler>>,
    {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateTimerWithCompletion)(::windows::core::Interface::as_raw(this), handler.into().abi(), delay, destroyed.into().abi(), result__.as_mut_ptr()).from_abi::<ThreadPoolTimer>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IThreadPoolTimerStatics<R, F: FnOnce(&IThreadPoolTimerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<ThreadPoolTimer, IThreadPoolTimerStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for ThreadPoolTimer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ThreadPoolTimer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ThreadPoolTimer {}
impl ::core::fmt::Debug for ThreadPoolTimer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ThreadPoolTimer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ThreadPoolTimer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Threading.ThreadPoolTimer;{594ebe78-55ea-4a88-a50d-3402ae1f9cf2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ThreadPoolTimer {
    type Vtable = IThreadPoolTimer_Vtbl;
    const IID: ::windows::core::GUID = <IThreadPoolTimer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ThreadPoolTimer {
    const NAME: &'static str = "Windows.System.Threading.ThreadPoolTimer";
}
impl ::core::convert::From<ThreadPoolTimer> for ::windows::core::IUnknown {
    fn from(value: ThreadPoolTimer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ThreadPoolTimer> for ::windows::core::IUnknown {
    fn from(value: &ThreadPoolTimer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ThreadPoolTimer> for &::windows::core::IUnknown {
    fn from(value: &ThreadPoolTimer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ThreadPoolTimer> for ::windows::core::IInspectable {
    fn from(value: ThreadPoolTimer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ThreadPoolTimer> for ::windows::core::IInspectable {
    fn from(value: &ThreadPoolTimer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ThreadPoolTimer> for &::windows::core::IInspectable {
    fn from(value: &ThreadPoolTimer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for ThreadPoolTimer {}
unsafe impl ::core::marker::Sync for ThreadPoolTimer {}
#[doc = "*Required features: `\"System_Threading\"`*"]
#[repr(transparent)]
pub struct TimerDestroyedHandler(pub ::windows::core::IUnknown);
impl TimerDestroyedHandler {
    pub fn new<F: FnMut(&::core::option::Option<ThreadPoolTimer>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = TimerDestroyedHandlerBox::<F> { vtable: &TimerDestroyedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0>(&self, timer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ThreadPoolTimer>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), timer.into().abi()).ok() }
    }
}
#[repr(C)]
struct TimerDestroyedHandlerBox<F: FnMut(&::core::option::Option<ThreadPoolTimer>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const TimerDestroyedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<ThreadPoolTimer>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> TimerDestroyedHandlerBox<F> {
    const VTABLE: TimerDestroyedHandler_Vtbl = TimerDestroyedHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<TimerDestroyedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, timer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&timer)).into()
    }
}
impl ::core::clone::Clone for TimerDestroyedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimerDestroyedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimerDestroyedHandler {}
impl ::core::fmt::Debug for TimerDestroyedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimerDestroyedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for TimerDestroyedHandler {
    type Vtable = TimerDestroyedHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34ed19fa_8384_4eb9_8209_fb5094eeec35);
}
unsafe impl ::windows::core::RuntimeType for TimerDestroyedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{34ed19fa-8384-4eb9-8209-fb5094eeec35}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct TimerDestroyedHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"System_Threading\"`*"]
#[repr(transparent)]
pub struct TimerElapsedHandler(pub ::windows::core::IUnknown);
impl TimerElapsedHandler {
    pub fn new<F: FnMut(&::core::option::Option<ThreadPoolTimer>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = TimerElapsedHandlerBox::<F> { vtable: &TimerElapsedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0>(&self, timer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ThreadPoolTimer>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), timer.into().abi()).ok() }
    }
}
#[repr(C)]
struct TimerElapsedHandlerBox<F: FnMut(&::core::option::Option<ThreadPoolTimer>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const TimerElapsedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<ThreadPoolTimer>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> TimerElapsedHandlerBox<F> {
    const VTABLE: TimerElapsedHandler_Vtbl = TimerElapsedHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<TimerElapsedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, timer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&timer)).into()
    }
}
impl ::core::clone::Clone for TimerElapsedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for TimerElapsedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for TimerElapsedHandler {}
impl ::core::fmt::Debug for TimerElapsedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimerElapsedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for TimerElapsedHandler {
    type Vtable = TimerElapsedHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfaaea667_fbeb_49cb_adb2_71184c556e43);
}
unsafe impl ::windows::core::RuntimeType for TimerElapsedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{faaea667-fbeb-49cb-adb2-71184c556e43}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct TimerElapsedHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"System_Threading\"`, `\"Foundation\"`*"]
#[cfg(feature = "Foundation")]
#[repr(transparent)]
pub struct WorkItemHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "Foundation")]
impl WorkItemHandler {
    pub fn new<F: FnMut(&::core::option::Option<super::super::Foundation::IAsyncAction>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = WorkItemHandlerBox::<F> { vtable: &WorkItemHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Invoke<'a, P0, E0>(&self, operation: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::IAsyncAction>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), operation.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
#[cfg(feature = "Foundation")]
#[repr(C)]
struct WorkItemHandlerBox<F: FnMut(&::core::option::Option<super::super::Foundation::IAsyncAction>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const WorkItemHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "Foundation")]
impl<F: FnMut(&::core::option::Option<super::super::Foundation::IAsyncAction>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> WorkItemHandlerBox<F> {
    const VTABLE: WorkItemHandler_Vtbl = WorkItemHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<WorkItemHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, operation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&operation)).into()
    }
}
#[cfg(feature = "Foundation")]
impl ::core::clone::Clone for WorkItemHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::PartialEq for WorkItemHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation")]
impl ::core::cmp::Eq for WorkItemHandler {}
#[cfg(feature = "Foundation")]
impl ::core::fmt::Debug for WorkItemHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WorkItemHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::Interface for WorkItemHandler {
    type Vtable = WorkItemHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d1a8b8b_fa66_414f_9cbd_b65fc99d17fa);
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::core::RuntimeType for WorkItemHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1d1a8b8b-fa66-414f-9cbd-b65fc99d17fa}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation")]
#[repr(C)]
#[doc(hidden)]
pub struct WorkItemHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Foundation")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, operation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WorkItemOptions(pub u32);
impl WorkItemOptions {
    pub const None: Self = Self(0u32);
    pub const TimeSliced: Self = Self(1u32);
}
impl ::core::marker::Copy for WorkItemOptions {}
impl ::core::clone::Clone for WorkItemOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WorkItemOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WorkItemOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for WorkItemOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WorkItemOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WorkItemOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WorkItemOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WorkItemOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WorkItemOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WorkItemOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for WorkItemOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Threading.WorkItemOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"System_Threading\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WorkItemPriority(pub i32);
impl WorkItemPriority {
    pub const Low: Self = Self(-1i32);
    pub const Normal: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for WorkItemPriority {}
impl ::core::clone::Clone for WorkItemPriority {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WorkItemPriority {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WorkItemPriority {
    type Abi = Self;
}
impl ::core::fmt::Debug for WorkItemPriority {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WorkItemPriority").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WorkItemPriority {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.System.Threading.WorkItemPriority;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
