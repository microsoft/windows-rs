#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IThreadPoolStaticsImpl: Sized {
    fn RunAsync(&mut self, handler: &::core::option::Option<WorkItemHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RunWithPriorityAsync(&mut self, handler: &::core::option::Option<WorkItemHandler>, priority: WorkItemPriority) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RunWithPriorityAndOptionsAsync(&mut self, handler: &::core::option::Option<WorkItemHandler>, priority: WorkItemPriority, options: WorkItemOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IThreadPoolStatics {
    const NAME: &'static str = "Windows.System.Threading.IThreadPoolStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IThreadPoolStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThreadPoolStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThreadPoolStaticsVtbl {
        unsafe extern "system" fn RunAsync<Impl: IThreadPoolStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunAsync(&*(&handler as *const <WorkItemHandler as ::windows::core::Abi>::Abi as *const <WorkItemHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunWithPriorityAsync<Impl: IThreadPoolStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, priority: WorkItemPriority, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunWithPriorityAsync(&*(&handler as *const <WorkItemHandler as ::windows::core::Abi>::Abi as *const <WorkItemHandler as ::windows::core::DefaultType>::DefaultType), priority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunWithPriorityAndOptionsAsync<Impl: IThreadPoolStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, priority: WorkItemPriority, options: WorkItemOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RunWithPriorityAndOptionsAsync(&*(&handler as *const <WorkItemHandler as ::windows::core::Abi>::Abi as *const <WorkItemHandler as ::windows::core::DefaultType>::DefaultType), priority, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IThreadPoolStatics, BASE_OFFSET>(),
            RunAsync: RunAsync::<Impl, IMPL_OFFSET>,
            RunWithPriorityAsync: RunWithPriorityAsync::<Impl, IMPL_OFFSET>,
            RunWithPriorityAndOptionsAsync: RunWithPriorityAndOptionsAsync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThreadPoolStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IThreadPoolTimerImpl: Sized {
    fn Period(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Delay(&mut self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IThreadPoolTimer {
    const NAME: &'static str = "Windows.System.Threading.IThreadPoolTimer";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IThreadPoolTimerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThreadPoolTimerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThreadPoolTimerVtbl {
        unsafe extern "system" fn Period<Impl: IThreadPoolTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Period() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delay<Impl: IThreadPoolTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IThreadPoolTimerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IThreadPoolTimer, BASE_OFFSET>(),
            Period: Period::<Impl, IMPL_OFFSET>,
            Delay: Delay::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThreadPoolTimer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IThreadPoolTimerStaticsImpl: Sized {
    fn CreatePeriodicTimer(&mut self, handler: &::core::option::Option<TimerElapsedHandler>, period: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<ThreadPoolTimer>;
    fn CreateTimer(&mut self, handler: &::core::option::Option<TimerElapsedHandler>, delay: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<ThreadPoolTimer>;
    fn CreatePeriodicTimerWithCompletion(&mut self, handler: &::core::option::Option<TimerElapsedHandler>, period: &super::super::Foundation::TimeSpan, destroyed: &::core::option::Option<TimerDestroyedHandler>) -> ::windows::core::Result<ThreadPoolTimer>;
    fn CreateTimerWithCompletion(&mut self, handler: &::core::option::Option<TimerElapsedHandler>, delay: &super::super::Foundation::TimeSpan, destroyed: &::core::option::Option<TimerDestroyedHandler>) -> ::windows::core::Result<ThreadPoolTimer>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IThreadPoolTimerStatics {
    const NAME: &'static str = "Windows.System.Threading.IThreadPoolTimerStatics";
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
impl IThreadPoolTimerStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThreadPoolTimerStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThreadPoolTimerStaticsVtbl {
        unsafe extern "system" fn CreatePeriodicTimer<Impl: IThreadPoolTimerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, period: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePeriodicTimer(&*(&handler as *const <TimerElapsedHandler as ::windows::core::Abi>::Abi as *const <TimerElapsedHandler as ::windows::core::DefaultType>::DefaultType), &*(&period as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTimer<Impl: IThreadPoolTimerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, delay: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTimer(&*(&handler as *const <TimerElapsedHandler as ::windows::core::Abi>::Abi as *const <TimerElapsedHandler as ::windows::core::DefaultType>::DefaultType), &*(&delay as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePeriodicTimerWithCompletion<Impl: IThreadPoolTimerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, period: super::super::Foundation::TimeSpan, destroyed: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePeriodicTimerWithCompletion(
                &*(&handler as *const <TimerElapsedHandler as ::windows::core::Abi>::Abi as *const <TimerElapsedHandler as ::windows::core::DefaultType>::DefaultType),
                &*(&period as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                &*(&destroyed as *const <TimerDestroyedHandler as ::windows::core::Abi>::Abi as *const <TimerDestroyedHandler as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTimerWithCompletion<Impl: IThreadPoolTimerStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, delay: super::super::Foundation::TimeSpan, destroyed: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTimerWithCompletion(
                &*(&handler as *const <TimerElapsedHandler as ::windows::core::Abi>::Abi as *const <TimerElapsedHandler as ::windows::core::DefaultType>::DefaultType),
                &*(&delay as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                &*(&destroyed as *const <TimerDestroyedHandler as ::windows::core::Abi>::Abi as *const <TimerDestroyedHandler as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IThreadPoolTimerStatics, BASE_OFFSET>(),
            CreatePeriodicTimer: CreatePeriodicTimer::<Impl, IMPL_OFFSET>,
            CreateTimer: CreateTimer::<Impl, IMPL_OFFSET>,
            CreatePeriodicTimerWithCompletion: CreatePeriodicTimerWithCompletion::<Impl, IMPL_OFFSET>,
            CreateTimerWithCompletion: CreateTimerWithCompletion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThreadPoolTimerStatics as ::windows::core::Interface>::IID
    }
}
