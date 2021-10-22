#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "System_Threading_Core")]
pub mod Core;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IThreadPoolStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IThreadPoolStatics {
    type Vtable = IThreadPoolStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3065997277,
        33981,
        17656,
        [172, 28, 147, 235, 203, 157, 186, 145],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadPoolStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        priority: WorkItemPriority,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        priority: WorkItemPriority,
        options: WorkItemOptions,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IThreadPoolTimer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IThreadPoolTimer {
    type Vtable = IThreadPoolTimer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1498332792,
        21994,
        19080,
        [165, 13, 52, 2, 174, 31, 156, 242],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadPoolTimer_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::TimeSpan,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IThreadPoolTimerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IThreadPoolTimerStatics {
    type Vtable = IThreadPoolTimerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        445291778,
        58498,
        17947,
        [184, 199, 142, 250, 209, 204, 229, 144],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IThreadPoolTimerStatics_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        period: super::super::Foundation::TimeSpan,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        delay: super::super::Foundation::TimeSpan,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        period: super::super::Foundation::TimeSpan,
        destroyed: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        delay: super::super::Foundation::TimeSpan,
        destroyed: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
pub struct ThreadPool {}
impl ThreadPool {
    #[cfg(feature = "Foundation")]
    pub fn RunAsync<'a, Param0: ::windows::runtime::IntoParam<'a, WorkItemHandler>>(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IThreadPoolStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RunWithPriorityAsync<'a, Param0: ::windows::runtime::IntoParam<'a, WorkItemHandler>>(
        handler: Param0,
        priority: WorkItemPriority,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IThreadPoolStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                priority,
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RunWithPriorityAndOptionsAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, WorkItemHandler>,
    >(
        handler: Param0,
        priority: WorkItemPriority,
        options: WorkItemOptions,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        Self::IThreadPoolStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                priority,
                options,
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    pub fn IThreadPoolStatics<
        R,
        F: FnOnce(&IThreadPoolStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ThreadPool, IThreadPoolStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ThreadPool {
    const NAME: &'static str = "Windows.System.Threading.ThreadPool";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ThreadPoolTimer(::windows::runtime::IInspectable);
impl ThreadPoolTimer {
    #[cfg(feature = "Foundation")]
    pub fn Period(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Delay(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    pub fn Cancel(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreatePeriodicTimer<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, TimerElapsedHandler>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
    >(
        handler: Param0,
        period: Param1,
    ) -> ::windows::runtime::Result<ThreadPoolTimer> {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                period.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ThreadPoolTimer>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateTimer<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, TimerElapsedHandler>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
    >(
        handler: Param0,
        delay: Param1,
    ) -> ::windows::runtime::Result<ThreadPoolTimer> {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                delay.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ThreadPoolTimer>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreatePeriodicTimerWithCompletion<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, TimerElapsedHandler>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
        Param2: ::windows::runtime::IntoParam<'a, TimerDestroyedHandler>,
    >(
        handler: Param0,
        period: Param1,
        destroyed: Param2,
    ) -> ::windows::runtime::Result<ThreadPoolTimer> {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                period.into_param().abi(),
                destroyed.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ThreadPoolTimer>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateTimerWithCompletion<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, TimerElapsedHandler>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::TimeSpan>,
        Param2: ::windows::runtime::IntoParam<'a, TimerDestroyedHandler>,
    >(
        handler: Param0,
        delay: Param1,
        destroyed: Param2,
    ) -> ::windows::runtime::Result<ThreadPoolTimer> {
        Self::IThreadPoolTimerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                delay.into_param().abi(),
                destroyed.into_param().abi(),
                &mut result__,
            )
            .from_abi::<ThreadPoolTimer>(result__)
        })
    }
    pub fn IThreadPoolTimerStatics<
        R,
        F: FnOnce(&IThreadPoolTimerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ThreadPoolTimer,
            IThreadPoolTimerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ThreadPoolTimer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.System.Threading.ThreadPoolTimer;{594ebe78-55ea-4a88-a50d-3402ae1f9cf2})",
    );
}
unsafe impl ::windows::runtime::Interface for ThreadPoolTimer {
    type Vtable = IThreadPoolTimer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1498332792,
        21994,
        19080,
        [165, 13, 52, 2, 174, 31, 156, 242],
    );
}
impl ::windows::runtime::RuntimeName for ThreadPoolTimer {
    const NAME: &'static str = "Windows.System.Threading.ThreadPoolTimer";
}
impl ::std::convert::From<ThreadPoolTimer> for ::windows::runtime::IUnknown {
    fn from(value: ThreadPoolTimer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ThreadPoolTimer> for ::windows::runtime::IUnknown {
    fn from(value: &ThreadPoolTimer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ThreadPoolTimer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ThreadPoolTimer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ThreadPoolTimer> for ::windows::runtime::IInspectable {
    fn from(value: ThreadPoolTimer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ThreadPoolTimer> for ::windows::runtime::IInspectable {
    fn from(value: &ThreadPoolTimer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ThreadPoolTimer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ThreadPoolTimer
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for ThreadPoolTimer {}
unsafe impl ::std::marker::Sync for ThreadPoolTimer {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TimerDestroyedHandler(::windows::runtime::IUnknown);
impl TimerDestroyedHandler {
    pub fn new<
        F: FnMut(&::std::option::Option<ThreadPoolTimer>) -> ::windows::runtime::Result<()> + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = TimerDestroyedHandler_box::<F> {
            vtable: &TimerDestroyedHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ThreadPoolTimer>>(
        &self,
        timer: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                timer.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TimerDestroyedHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({34ed19fa-8384-4eb9-8209-fb5094eeec35})",
    );
}
unsafe impl ::windows::runtime::Interface for TimerDestroyedHandler {
    type Vtable = TimerDestroyedHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        887953914,
        33668,
        20153,
        [130, 9, 251, 80, 148, 238, 236, 53],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct TimerDestroyedHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timer: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct TimerDestroyedHandler_box<
    F: FnMut(&::std::option::Option<ThreadPoolTimer>) -> ::windows::runtime::Result<()> + 'static,
> {
    vtable: *const TimerDestroyedHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<
        F: FnMut(&::std::option::Option<ThreadPoolTimer>) -> ::windows::runtime::Result<()> + 'static,
    > TimerDestroyedHandler_box<F>
{
    const VTABLE: TimerDestroyedHandler_abi = TimerDestroyedHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<TimerDestroyedHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
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
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        timer: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&timer as *const <ThreadPoolTimer as ::windows::runtime::Abi>::Abi
                as *const <ThreadPoolTimer as ::windows::runtime::Abi>::DefaultType),
        )
        .into()
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct TimerElapsedHandler(::windows::runtime::IUnknown);
impl TimerElapsedHandler {
    pub fn new<
        F: FnMut(&::std::option::Option<ThreadPoolTimer>) -> ::windows::runtime::Result<()> + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = TimerElapsedHandler_box::<F> {
            vtable: &TimerElapsedHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, ThreadPoolTimer>>(
        &self,
        timer: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                timer.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TimerElapsedHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({faaea667-fbeb-49cb-adb2-71184c556e43})",
    );
}
unsafe impl ::windows::runtime::Interface for TimerElapsedHandler {
    type Vtable = TimerElapsedHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4205749863,
        64491,
        18891,
        [173, 178, 113, 24, 76, 85, 110, 67],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct TimerElapsedHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        timer: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct TimerElapsedHandler_box<
    F: FnMut(&::std::option::Option<ThreadPoolTimer>) -> ::windows::runtime::Result<()> + 'static,
> {
    vtable: *const TimerElapsedHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<
        F: FnMut(&::std::option::Option<ThreadPoolTimer>) -> ::windows::runtime::Result<()> + 'static,
    > TimerElapsedHandler_box<F>
{
    const VTABLE: TimerElapsedHandler_abi = TimerElapsedHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<TimerElapsedHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
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
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        timer: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(
            &*(&timer as *const <ThreadPoolTimer as ::windows::runtime::Abi>::Abi
                as *const <ThreadPoolTimer as ::windows::runtime::Abi>::DefaultType),
        )
        .into()
    }
}
#[cfg(feature = "Foundation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WorkItemHandler(::windows::runtime::IUnknown);
#[cfg(feature = "Foundation")]
impl WorkItemHandler {
    pub fn new<
        F: FnMut(
                &::std::option::Option<super::super::Foundation::IAsyncAction>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = WorkItemHandler_box::<F> {
            vtable: &WorkItemHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[cfg(feature = "Foundation")]
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::IAsyncAction>,
    >(
        &self,
        operation: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                operation.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::RuntimeType for WorkItemHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({1d1a8b8b-fa66-414f-9cbd-b65fc99d17fa})",
    );
}
#[cfg(feature = "Foundation")]
unsafe impl ::windows::runtime::Interface for WorkItemHandler {
    type Vtable = WorkItemHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        488278923,
        64102,
        16719,
        [156, 189, 182, 95, 201, 157, 23, 250],
    );
}
#[cfg(feature = "Foundation")]
#[repr(C)]
#[doc(hidden)]
pub struct WorkItemHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        operation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[cfg(feature = "Foundation")]
#[repr(C)]
struct WorkItemHandler_box<
    F: FnMut(
            &::std::option::Option<super::super::Foundation::IAsyncAction>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const WorkItemHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
#[cfg(feature = "Foundation")]
impl<
        F: FnMut(
                &::std::option::Option<super::super::Foundation::IAsyncAction>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > WorkItemHandler_box<F>
{
    const VTABLE: WorkItemHandler_abi = WorkItemHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<WorkItemHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
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
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        operation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & operation as * const < super::super::Foundation:: IAsyncAction as :: windows :: runtime :: Abi > :: Abi as * const < super::super::Foundation:: IAsyncAction as :: windows :: runtime :: Abi > :: DefaultType ) , ) . into ( )
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WorkItemOptions(pub u32);
impl WorkItemOptions {
    pub const None: WorkItemOptions = WorkItemOptions(0u32);
    pub const TimeSliced: WorkItemOptions = WorkItemOptions(1u32);
}
impl ::std::convert::From<u32> for WorkItemOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WorkItemOptions {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WorkItemOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.Threading.WorkItemOptions;u4)",
    );
}
impl ::std::ops::BitOr for WorkItemOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for WorkItemOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for WorkItemOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for WorkItemOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for WorkItemOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WorkItemPriority(pub i32);
impl WorkItemPriority {
    pub const Low: WorkItemPriority = WorkItemPriority(-1i32);
    pub const Normal: WorkItemPriority = WorkItemPriority(0i32);
    pub const High: WorkItemPriority = WorkItemPriority(1i32);
}
impl ::std::convert::From<i32> for WorkItemPriority {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WorkItemPriority {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for WorkItemPriority {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.Threading.WorkItemPriority;i4)",
    );
}
