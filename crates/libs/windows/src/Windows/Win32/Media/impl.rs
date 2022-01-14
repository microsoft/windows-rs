#[cfg(feature = "Win32_Foundation")]
pub trait IReferenceClock_Impl: Sized {
    fn GetTime(&mut self) -> ::windows::core::Result<i64>;
    fn AdviseTime(&mut self, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE) -> ::windows::core::Result<usize>;
    fn AdvisePeriodic(&mut self, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE) -> ::windows::core::Result<usize>;
    fn Unadvise(&mut self, dwadvisecookie: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IReferenceClock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceClock_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceClock_Vtbl {
        unsafe extern "system" fn GetTime<Impl: IReferenceClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTime() {
                ::core::result::Result::Ok(ok__) => {
                    *ptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdviseTime<Impl: IReferenceClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdviseTime(::core::mem::transmute_copy(&basetime), ::core::mem::transmute_copy(&streamtime), ::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwadvisecookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvisePeriodic<Impl: IReferenceClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AdvisePeriodic(::core::mem::transmute_copy(&starttime), ::core::mem::transmute_copy(&periodtime), ::core::mem::transmute_copy(&hsemaphore)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwadvisecookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Impl: IReferenceClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwadvisecookie: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unadvise(::core::mem::transmute_copy(&dwadvisecookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetTime: GetTime::<Impl, IMPL_OFFSET>,
            AdviseTime: AdviseTime::<Impl, IMPL_OFFSET>,
            AdvisePeriodic: AdvisePeriodic::<Impl, IMPL_OFFSET>,
            Unadvise: Unadvise::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceClock as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IReferenceClock2_Impl: Sized + IReferenceClock_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl IReferenceClock2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceClock2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceClock2_Vtbl {
        Self { base: IReferenceClock_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceClock2 as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceClockTimerControl_Impl: Sized {
    fn SetDefaultTimerResolution(&mut self, timerresolution: i64) -> ::windows::core::Result<()>;
    fn GetDefaultTimerResolution(&mut self) -> ::windows::core::Result<i64>;
}
impl IReferenceClockTimerControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceClockTimerControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceClockTimerControl_Vtbl {
        unsafe extern "system" fn SetDefaultTimerResolution<Impl: IReferenceClockTimerControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timerresolution: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDefaultTimerResolution(::core::mem::transmute_copy(&timerresolution)).into()
        }
        unsafe extern "system" fn GetDefaultTimerResolution<Impl: IReferenceClockTimerControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimerresolution: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultTimerResolution() {
                ::core::result::Result::Ok(ok__) => {
                    *ptimerresolution = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDefaultTimerResolution: SetDefaultTimerResolution::<Impl, IMPL_OFFSET>,
            GetDefaultTimerResolution: GetDefaultTimerResolution::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceClockTimerControl as ::windows::core::Interface>::IID
    }
}
