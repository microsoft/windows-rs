#[doc = "*Required features: `\"Win32_Media\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IReferenceClock_Impl: Sized {
    fn GetTime(&self) -> ::windows::core::Result<i64>;
    fn AdviseTime(&self, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE) -> ::windows::core::Result<usize>;
    fn AdvisePeriodic(&self, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE) -> ::windows::core::Result<usize>;
    fn Unadvise(&self, dwadvisecookie: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IReferenceClock {}
#[cfg(feature = "Win32_Foundation")]
impl IReferenceClock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: isize>() -> IReferenceClock_Vtbl {
        unsafe extern "system" fn GetTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdviseTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdviseTime(::core::mem::transmute_copy(&basetime), ::core::mem::transmute_copy(&streamtime), ::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwadvisecookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AdvisePeriodic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AdvisePeriodic(::core::mem::transmute_copy(&starttime), ::core::mem::transmute_copy(&periodtime), ::core::mem::transmute_copy(&hsemaphore)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwadvisecookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwadvisecookie: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unadvise(::core::mem::transmute_copy(&dwadvisecookie)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTime: GetTime::<Identity, Impl, OFFSET>,
            AdviseTime: AdviseTime::<Identity, Impl, OFFSET>,
            AdvisePeriodic: AdvisePeriodic::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceClock as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IReferenceClock2_Impl: Sized + IReferenceClock_Impl {}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IReferenceClock2 {}
#[cfg(feature = "Win32_Foundation")]
impl IReferenceClock2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClock2_Impl, const OFFSET: isize>() -> IReferenceClock2_Vtbl {
        Self { base__: IReferenceClock_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceClock2 as ::windows::core::ComInterface>::IID || iid == &<IReferenceClock as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media\"`, `\"implement\"`*"]
pub trait IReferenceClockTimerControl_Impl: Sized {
    fn SetDefaultTimerResolution(&self, timerresolution: i64) -> ::windows::core::Result<()>;
    fn GetDefaultTimerResolution(&self) -> ::windows::core::Result<i64>;
}
impl ::windows::core::RuntimeName for IReferenceClockTimerControl {}
impl IReferenceClockTimerControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClockTimerControl_Impl, const OFFSET: isize>() -> IReferenceClockTimerControl_Vtbl {
        unsafe extern "system" fn SetDefaultTimerResolution<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClockTimerControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timerresolution: i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDefaultTimerResolution(::core::mem::transmute_copy(&timerresolution)).into()
        }
        unsafe extern "system" fn GetDefaultTimerResolution<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReferenceClockTimerControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimerresolution: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultTimerResolution() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptimerresolution, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDefaultTimerResolution: SetDefaultTimerResolution::<Identity, Impl, OFFSET>,
            GetDefaultTimerResolution: GetDefaultTimerResolution::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceClockTimerControl as ::windows::core::ComInterface>::IID
    }
}
