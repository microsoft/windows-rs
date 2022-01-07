pub trait IReferenceClockImpl: Sized {
    fn GetTime();
    fn AdviseTime();
    fn AdvisePeriodic();
    fn Unadvise();
}
impl ::windows::core::RuntimeName for IReferenceClock {
    const NAME: &'static str = "Windows.Win32.Media.IReferenceClock";
}
impl IReferenceClockVtbl {
    pub const fn new<Impl: IReferenceClockImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReferenceClockVtbl {
        unsafe extern "system" fn GetTime<Impl: IReferenceClockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTime(::core::mem::transmute_copy(&ptime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdviseTime<Impl: IReferenceClockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdviseTime(basetime, streamtime, &*(&hevent as *const <super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwadvisecookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvisePeriodic<Impl: IReferenceClockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AdvisePeriodic(starttime, periodtime, &*(&hsemaphore as *const <super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwadvisecookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IReferenceClockImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwadvisecookie: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unadvise(dwadvisecookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReferenceClock>, base.5, GetTime::<Impl, OFFSET>, AdviseTime::<Impl, OFFSET>, AdvisePeriodic::<Impl, OFFSET>, Unadvise::<Impl, OFFSET>)
    }
}
pub trait IReferenceClock2Impl: Sized + IReferenceClockImpl {}
impl ::windows::core::RuntimeName for IReferenceClock2 {
    const NAME: &'static str = "Windows.Win32.Media.IReferenceClock2";
}
impl IReferenceClock2Vtbl {
    pub const fn new<Impl: IReferenceClock2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReferenceClock2Vtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReferenceClock2>, base.5)
    }
}
pub trait IReferenceClockTimerControlImpl: Sized {
    fn SetDefaultTimerResolution();
    fn GetDefaultTimerResolution();
}
impl ::windows::core::RuntimeName for IReferenceClockTimerControl {
    const NAME: &'static str = "Windows.Win32.Media.IReferenceClockTimerControl";
}
impl IReferenceClockTimerControlVtbl {
    pub const fn new<Impl: IReferenceClockTimerControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReferenceClockTimerControlVtbl {
        unsafe extern "system" fn SetDefaultTimerResolution<Impl: IReferenceClockTimerControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timerresolution: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDefaultTimerResolution(timerresolution) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultTimerResolution<Impl: IReferenceClockTimerControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptimerresolution: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDefaultTimerResolution(::core::mem::transmute_copy(&ptimerresolution)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReferenceClockTimerControl>, base.5, SetDefaultTimerResolution::<Impl, OFFSET>, GetDefaultTimerResolution::<Impl, OFFSET>)
    }
}
