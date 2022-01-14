#[cfg(feature = "Win32_System_Ole")]
pub trait IADesktopP2_Impl: Sized {
    fn ReReadWallpaper(&mut self) -> ::windows::core::Result<()>;
    fn GetADObjectFlags(&mut self, pdwflags: *mut u32, dwmask: u32) -> ::windows::core::Result<()>;
    fn UpdateAllDesktopSubscriptions(&mut self) -> ::windows::core::Result<()>;
    fn MakeDynamicChanges(&mut self, poleobj: &::core::option::Option<super::super::System::Ole::IOleObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl IADesktopP2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADesktopP2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IADesktopP2_Vtbl {
        unsafe extern "system" fn ReReadWallpaper<Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReReadWallpaper().into()
        }
        unsafe extern "system" fn GetADObjectFlags<Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetADObjectFlags(::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UpdateAllDesktopSubscriptions<Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateAllDesktopSubscriptions().into()
        }
        unsafe extern "system" fn MakeDynamicChanges<Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poleobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeDynamicChanges(::core::mem::transmute(&poleobj)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ReReadWallpaper: ReReadWallpaper::<Impl, IMPL_OFFSET>,
            GetADObjectFlags: GetADObjectFlags::<Impl, IMPL_OFFSET>,
            UpdateAllDesktopSubscriptions: UpdateAllDesktopSubscriptions::<Impl, IMPL_OFFSET>,
            MakeDynamicChanges: MakeDynamicChanges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADesktopP2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IActiveDesktopP_Impl: Sized {
    fn SetSafeMode(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn EnsureUpdateHTML(&mut self) -> ::windows::core::Result<()>;
    fn SetScheme(&mut self, pwszschemename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetScheme(&mut self, pwszschemename: super::super::Foundation::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IActiveDesktopP_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveDesktopP_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActiveDesktopP_Vtbl {
        unsafe extern "system" fn SetSafeMode<Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSafeMode(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn EnsureUpdateHTML<Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnsureUpdateHTML().into()
        }
        unsafe extern "system" fn SetScheme<Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszschemename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScheme(::core::mem::transmute_copy(&pwszschemename), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetScheme<Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszschemename: super::super::Foundation::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetScheme(::core::mem::transmute_copy(&pwszschemename), ::core::mem::transmute_copy(&pdwcchbuffer), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetSafeMode: SetSafeMode::<Impl, IMPL_OFFSET>,
            EnsureUpdateHTML: EnsureUpdateHTML::<Impl, IMPL_OFFSET>,
            SetScheme: SetScheme::<Impl, IMPL_OFFSET>,
            GetScheme: GetScheme::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveDesktopP as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBriefcaseInitiator_Impl: Sized {
    fn IsMonikerInBriefcase(&mut self, pmk: &::core::option::Option<super::super::System::Com::IMoniker>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IBriefcaseInitiator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBriefcaseInitiator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBriefcaseInitiator_Vtbl {
        unsafe extern "system" fn IsMonikerInBriefcase<Impl: IBriefcaseInitiator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsMonikerInBriefcase(::core::mem::transmute(&pmk)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IsMonikerInBriefcase: IsMonikerInBriefcase::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBriefcaseInitiator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IEmptyVolumeCache_Impl: Sized {
    fn Initialize(&mut self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: super::super::Foundation::PWSTR, ppwszdisplayname: *mut super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32) -> ::windows::core::Result<()>;
    fn GetSpaceUsed(&mut self, pdwlspaceused: *mut u64, picb: &::core::option::Option<IEmptyVolumeCacheCallBack>) -> ::windows::core::Result<()>;
    fn Purge(&mut self, dwlspacetofree: u64, picb: &::core::option::Option<IEmptyVolumeCacheCallBack>) -> ::windows::core::Result<()>;
    fn ShowProperties(&mut self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Deactivate(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IEmptyVolumeCache_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCache_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmptyVolumeCache_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: super::super::Foundation::PWSTR, ppwszdisplayname: *mut super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&hkregkey), ::core::mem::transmute_copy(&pcwszvolume), ::core::mem::transmute_copy(&ppwszdisplayname), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetSpaceUsed<Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlspaceused: *mut u64, picb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSpaceUsed(::core::mem::transmute_copy(&pdwlspaceused), ::core::mem::transmute(&picb)).into()
        }
        unsafe extern "system" fn Purge<Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspacetofree: u64, picb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Purge(::core::mem::transmute_copy(&dwlspacetofree), ::core::mem::transmute(&picb)).into()
        }
        unsafe extern "system" fn ShowProperties<Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShowProperties(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn Deactivate<Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deactivate() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetSpaceUsed: GetSpaceUsed::<Impl, IMPL_OFFSET>,
            Purge: Purge::<Impl, IMPL_OFFSET>,
            ShowProperties: ShowProperties::<Impl, IMPL_OFFSET>,
            Deactivate: Deactivate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmptyVolumeCache as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IEmptyVolumeCache2_Impl: Sized + IEmptyVolumeCache_Impl {
    fn InitializeEx(&mut self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: super::super::Foundation::PWSTR, pcwszkeyname: super::super::Foundation::PWSTR, ppwszdisplayname: *mut super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, ppwszbtntext: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IEmptyVolumeCache2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCache2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmptyVolumeCache2_Vtbl {
        unsafe extern "system" fn InitializeEx<Impl: IEmptyVolumeCache2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: super::super::Foundation::PWSTR, pcwszkeyname: super::super::Foundation::PWSTR, ppwszdisplayname: *mut super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, ppwszbtntext: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeEx(::core::mem::transmute_copy(&hkregkey), ::core::mem::transmute_copy(&pcwszvolume), ::core::mem::transmute_copy(&pcwszkeyname), ::core::mem::transmute_copy(&ppwszdisplayname), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&ppwszbtntext), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base: IEmptyVolumeCache_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), InitializeEx: InitializeEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmptyVolumeCache2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEmptyVolumeCacheCallBack_Impl: Sized {
    fn ScanProgress(&mut self, dwlspaceused: u64, dwflags: u32, pcwszstatus: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn PurgeProgress(&mut self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEmptyVolumeCacheCallBack_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCacheCallBack_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEmptyVolumeCacheCallBack_Vtbl {
        unsafe extern "system" fn ScanProgress<Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspaceused: u64, dwflags: u32, pcwszstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ScanProgress(::core::mem::transmute_copy(&dwlspaceused), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pcwszstatus)).into()
        }
        unsafe extern "system" fn PurgeProgress<Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PurgeProgress(::core::mem::transmute_copy(&dwlspacefreed), ::core::mem::transmute_copy(&dwlspacetofree), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pcwszstatus)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ScanProgress: ScanProgress::<Impl, IMPL_OFFSET>,
            PurgeProgress: PurgeProgress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmptyVolumeCacheCallBack as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IReconcilableObject_Impl: Sized {
    fn Reconcile(&mut self, pinitiator: &::core::option::Option<IReconcileInitiator>, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut ::core::option::Option<super::super::System::Com::IMoniker>, ploutindex: *mut i32, pstgnewresidues: &::core::option::Option<super::super::System::Com::StructuredStorage::IStorage>, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetProgressFeedbackMaxEstimate(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IReconcilableObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReconcilableObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReconcilableObject_Vtbl {
        unsafe extern "system" fn Reconcile<Impl: IReconcilableObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiator: ::windows::core::RawPtr, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut ::windows::core::RawPtr, ploutindex: *mut i32, pstgnewresidues: ::windows::core::RawPtr, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reconcile(::core::mem::transmute(&pinitiator), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute_copy(&hwndprogressfeedback), ::core::mem::transmute_copy(&ulcinput), ::core::mem::transmute_copy(&rgpmkotherinput), ::core::mem::transmute_copy(&ploutindex), ::core::mem::transmute(&pstgnewresidues), ::core::mem::transmute_copy(&pvreserved)).into()
        }
        unsafe extern "system" fn GetProgressFeedbackMaxEstimate<Impl: IReconcilableObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulprogressmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProgressFeedbackMaxEstimate() {
                ::core::result::Result::Ok(ok__) => {
                    *pulprogressmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Reconcile: Reconcile::<Impl, IMPL_OFFSET>,
            GetProgressFeedbackMaxEstimate: GetProgressFeedbackMaxEstimate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReconcilableObject as ::windows::core::Interface>::IID
    }
}
pub trait IReconcileInitiator_Impl: Sized {
    fn SetAbortCallback(&mut self, punkforabort: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetProgressFeedback(&mut self, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::Result<()>;
}
impl IReconcileInitiator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReconcileInitiator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReconcileInitiator_Vtbl {
        unsafe extern "system" fn SetAbortCallback<Impl: IReconcileInitiator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkforabort: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAbortCallback(::core::mem::transmute(&punkforabort)).into()
        }
        unsafe extern "system" fn SetProgressFeedback<Impl: IReconcileInitiator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProgressFeedback(::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAbortCallback: SetAbortCallback::<Impl, IMPL_OFFSET>,
            SetProgressFeedback: SetProgressFeedback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReconcileInitiator as ::windows::core::Interface>::IID
    }
}
