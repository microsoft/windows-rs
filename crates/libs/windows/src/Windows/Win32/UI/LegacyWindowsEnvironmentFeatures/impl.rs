#[cfg(feature = "Win32_System_Ole")]
pub trait IADesktopP2Impl: Sized {
    fn ReReadWallpaper();
    fn GetADObjectFlags();
    fn UpdateAllDesktopSubscriptions();
    fn MakeDynamicChanges();
}
#[cfg(feature = "Win32_System_Ole")]
impl IADesktopP2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADesktopP2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADesktopP2Vtbl {
        unsafe extern "system" fn ReReadWallpaper<Impl: IADesktopP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetADObjectFlags<Impl: IADesktopP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateAllDesktopSubscriptions<Impl: IADesktopP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MakeDynamicChanges<Impl: IADesktopP2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poleobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ReReadWallpaper::<Impl, IMPL_OFFSET>, GetADObjectFlags::<Impl, IMPL_OFFSET>, UpdateAllDesktopSubscriptions::<Impl, IMPL_OFFSET>, MakeDynamicChanges::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADesktopP2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IActiveDesktopPImpl: Sized {
    fn SetSafeMode();
    fn EnsureUpdateHTML();
    fn SetScheme();
    fn GetScheme();
}
#[cfg(feature = "Win32_Foundation")]
impl IActiveDesktopPVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveDesktopPImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActiveDesktopPVtbl {
        unsafe extern "system" fn SetSafeMode<Impl: IActiveDesktopPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnsureUpdateHTML<Impl: IActiveDesktopPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetScheme<Impl: IActiveDesktopPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszschemename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScheme<Impl: IActiveDesktopPImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszschemename: super::super::Foundation::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetSafeMode::<Impl, IMPL_OFFSET>, EnsureUpdateHTML::<Impl, IMPL_OFFSET>, SetScheme::<Impl, IMPL_OFFSET>, GetScheme::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveDesktopP as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBriefcaseInitiatorImpl: Sized {
    fn IsMonikerInBriefcase();
}
#[cfg(feature = "Win32_System_Com")]
impl IBriefcaseInitiatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBriefcaseInitiatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBriefcaseInitiatorVtbl {
        unsafe extern "system" fn IsMonikerInBriefcase<Impl: IBriefcaseInitiatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsMonikerInBriefcase::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBriefcaseInitiator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IEmptyVolumeCacheImpl: Sized {
    fn Initialize();
    fn GetSpaceUsed();
    fn Purge();
    fn ShowProperties();
    fn Deactivate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IEmptyVolumeCacheVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCacheImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmptyVolumeCacheVtbl {
        unsafe extern "system" fn Initialize<Impl: IEmptyVolumeCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: super::super::Foundation::PWSTR, ppwszdisplayname: *mut super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpaceUsed<Impl: IEmptyVolumeCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlspaceused: *mut u64, picb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Purge<Impl: IEmptyVolumeCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspacetofree: u64, picb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShowProperties<Impl: IEmptyVolumeCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Deactivate<Impl: IEmptyVolumeCacheImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, GetSpaceUsed::<Impl, IMPL_OFFSET>, Purge::<Impl, IMPL_OFFSET>, ShowProperties::<Impl, IMPL_OFFSET>, Deactivate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmptyVolumeCache as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IEmptyVolumeCache2Impl: Sized + IEmptyVolumeCacheImpl {
    fn InitializeEx();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IEmptyVolumeCache2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCache2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmptyVolumeCache2Vtbl {
        unsafe extern "system" fn InitializeEx<Impl: IEmptyVolumeCache2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: super::super::Foundation::PWSTR, pcwszkeyname: super::super::Foundation::PWSTR, ppwszdisplayname: *mut super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, ppwszbtntext: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, GetSpaceUsed::<Impl, IMPL_OFFSET>, Purge::<Impl, IMPL_OFFSET>, ShowProperties::<Impl, IMPL_OFFSET>, Deactivate::<Impl, IMPL_OFFSET>, InitializeEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmptyVolumeCache2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEmptyVolumeCacheCallBackImpl: Sized {
    fn ScanProgress();
    fn PurgeProgress();
}
#[cfg(feature = "Win32_Foundation")]
impl IEmptyVolumeCacheCallBackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCacheCallBackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmptyVolumeCacheCallBackVtbl {
        unsafe extern "system" fn ScanProgress<Impl: IEmptyVolumeCacheCallBackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspaceused: u64, dwflags: u32, pcwszstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PurgeProgress<Impl: IEmptyVolumeCacheCallBackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ScanProgress::<Impl, IMPL_OFFSET>, PurgeProgress::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmptyVolumeCacheCallBack as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IReconcilableObjectImpl: Sized {
    fn Reconcile();
    fn GetProgressFeedbackMaxEstimate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IReconcilableObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReconcilableObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReconcilableObjectVtbl {
        unsafe extern "system" fn Reconcile<Impl: IReconcilableObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiator: ::windows::core::RawPtr, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut ::windows::core::RawPtr, ploutindex: *mut i32, pstgnewresidues: ::windows::core::RawPtr, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProgressFeedbackMaxEstimate<Impl: IReconcilableObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulprogressmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Reconcile::<Impl, IMPL_OFFSET>, GetProgressFeedbackMaxEstimate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReconcilableObject as ::windows::core::Interface>::IID
    }
}
pub trait IReconcileInitiatorImpl: Sized {
    fn SetAbortCallback();
    fn SetProgressFeedback();
}
impl IReconcileInitiatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReconcileInitiatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReconcileInitiatorVtbl {
        unsafe extern "system" fn SetAbortCallback<Impl: IReconcileInitiatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkforabort: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProgressFeedback<Impl: IReconcileInitiatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAbortCallback::<Impl, IMPL_OFFSET>, SetProgressFeedback::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReconcileInitiator as ::windows::core::Interface>::IID
    }
}
