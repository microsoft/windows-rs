#[cfg(feature = "Win32_Foundation")]
pub trait IReferenceClockImpl: Sized {
    fn GetTime();
    fn AdviseTime();
    fn AdvisePeriodic();
    fn Unadvise();
}
#[cfg(feature = "Win32_Foundation")]
impl IReferenceClockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceClockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceClockVtbl {
        unsafe extern "system" fn GetTime<Impl: IReferenceClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdviseTime<Impl: IReferenceClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AdvisePeriodic<Impl: IReferenceClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unadvise<Impl: IReferenceClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwadvisecookie: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTime::<Impl, IMPL_OFFSET>, AdviseTime::<Impl, IMPL_OFFSET>, AdvisePeriodic::<Impl, IMPL_OFFSET>, Unadvise::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceClock as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IReferenceClock2Impl: Sized + IReferenceClockImpl {}
#[cfg(feature = "Win32_Foundation")]
impl IReferenceClock2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceClock2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceClock2Vtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTime::<Impl, IMPL_OFFSET>, AdviseTime::<Impl, IMPL_OFFSET>, AdvisePeriodic::<Impl, IMPL_OFFSET>, Unadvise::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceClock2 as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceClockTimerControlImpl: Sized {
    fn SetDefaultTimerResolution();
    fn GetDefaultTimerResolution();
}
impl IReferenceClockTimerControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceClockTimerControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceClockTimerControlVtbl {
        unsafe extern "system" fn SetDefaultTimerResolution<Impl: IReferenceClockTimerControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timerresolution: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultTimerResolution<Impl: IReferenceClockTimerControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimerresolution: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetDefaultTimerResolution::<Impl, IMPL_OFFSET>, GetDefaultTimerResolution::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceClockTimerControl as ::windows::core::Interface>::IID
    }
}
