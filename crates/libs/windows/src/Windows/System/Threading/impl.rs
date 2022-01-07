#[cfg(feature = "implement_exclusive")]
pub trait IThreadPoolStaticsImpl: Sized {
    fn RunAsync(&self, handler: &::core::option::Option<WorkItemHandler>) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RunWithPriorityAsync(&self, handler: &::core::option::Option<WorkItemHandler>, priority: WorkItemPriority) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn RunWithPriorityAndOptionsAsync(&self, handler: &::core::option::Option<WorkItemHandler>, priority: WorkItemPriority, options: WorkItemOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThreadPoolStatics {
    const NAME: &'static str = "Windows.System.Threading.IThreadPoolStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IThreadPoolStaticsVtbl {
    pub const fn new<Impl: IThreadPoolStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IThreadPoolStaticsVtbl {
        unsafe extern "system" fn RunAsync<Impl: IThreadPoolStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunAsync(&*(&handler as *const <WorkItemHandler as ::windows::core::Abi>::Abi as *const <WorkItemHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunWithPriorityAsync<Impl: IThreadPoolStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, priority: WorkItemPriority, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunWithPriorityAsync(&*(&handler as *const <WorkItemHandler as ::windows::core::Abi>::Abi as *const <WorkItemHandler as ::windows::core::DefaultType>::DefaultType), priority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RunWithPriorityAndOptionsAsync<Impl: IThreadPoolStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, priority: WorkItemPriority, options: WorkItemOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RunWithPriorityAndOptionsAsync(&*(&handler as *const <WorkItemHandler as ::windows::core::Abi>::Abi as *const <WorkItemHandler as ::windows::core::DefaultType>::DefaultType), priority, options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IThreadPoolStatics>, base.5, RunAsync::<Impl, OFFSET>, RunWithPriorityAsync::<Impl, OFFSET>, RunWithPriorityAndOptionsAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThreadPoolTimerImpl: Sized {
    fn Period(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Delay(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThreadPoolTimer {
    const NAME: &'static str = "Windows.System.Threading.IThreadPoolTimer";
}
#[cfg(feature = "implement_exclusive")]
impl IThreadPoolTimerVtbl {
    pub const fn new<Impl: IThreadPoolTimerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IThreadPoolTimerVtbl {
        unsafe extern "system" fn Period<Impl: IThreadPoolTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Period() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delay<Impl: IThreadPoolTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delay() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IThreadPoolTimerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IThreadPoolTimer>, base.5, Period::<Impl, OFFSET>, Delay::<Impl, OFFSET>, Cancel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IThreadPoolTimerStaticsImpl: Sized {
    fn CreatePeriodicTimer(&self, handler: &::core::option::Option<TimerElapsedHandler>, period: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<ThreadPoolTimer>;
    fn CreateTimer(&self, handler: &::core::option::Option<TimerElapsedHandler>, delay: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<ThreadPoolTimer>;
    fn CreatePeriodicTimerWithCompletion(&self, handler: &::core::option::Option<TimerElapsedHandler>, period: &super::super::Foundation::TimeSpan, destroyed: &::core::option::Option<TimerDestroyedHandler>) -> ::windows::core::Result<ThreadPoolTimer>;
    fn CreateTimerWithCompletion(&self, handler: &::core::option::Option<TimerElapsedHandler>, delay: &super::super::Foundation::TimeSpan, destroyed: &::core::option::Option<TimerDestroyedHandler>) -> ::windows::core::Result<ThreadPoolTimer>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IThreadPoolTimerStatics {
    const NAME: &'static str = "Windows.System.Threading.IThreadPoolTimerStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IThreadPoolTimerStaticsVtbl {
    pub const fn new<Impl: IThreadPoolTimerStaticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IThreadPoolTimerStaticsVtbl {
        unsafe extern "system" fn CreatePeriodicTimer<Impl: IThreadPoolTimerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, period: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePeriodicTimer(&*(&handler as *const <TimerElapsedHandler as ::windows::core::Abi>::Abi as *const <TimerElapsedHandler as ::windows::core::DefaultType>::DefaultType), &*(&period as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTimer<Impl: IThreadPoolTimerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, delay: super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTimer(&*(&handler as *const <TimerElapsedHandler as ::windows::core::Abi>::Abi as *const <TimerElapsedHandler as ::windows::core::DefaultType>::DefaultType), &*(&delay as *const <super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePeriodicTimerWithCompletion<Impl: IThreadPoolTimerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, period: super::super::Foundation::TimeSpan, destroyed: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        unsafe extern "system" fn CreateTimerWithCompletion<Impl: IThreadPoolTimerStaticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, delay: super::super::Foundation::TimeSpan, destroyed: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
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
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IThreadPoolTimerStatics>, base.5, CreatePeriodicTimer::<Impl, OFFSET>, CreateTimer::<Impl, OFFSET>, CreatePeriodicTimerWithCompletion::<Impl, OFFSET>, CreateTimerWithCompletion::<Impl, OFFSET>)
    }
}
