#[cfg(feature = "Win32_System_Ole")]
pub trait IADesktopP2_Impl: Sized {
    fn ReReadWallpaper(&self) -> ::windows::core::Result<()>;
    fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> ::windows::core::Result<()>;
    fn UpdateAllDesktopSubscriptions(&self) -> ::windows::core::Result<()>;
    fn MakeDynamicChanges(&self, poleobj: &::core::option::Option<super::super::System::Ole::IOleObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl IADesktopP2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IADesktopP2_Impl, const OFFSET: isize>() -> IADesktopP2_Vtbl {
        unsafe extern "system" fn ReReadWallpaper<Identity: ::windows::core::IUnknownImpl, Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReReadWallpaper().into()
        }
        unsafe extern "system" fn GetADObjectFlags<Identity: ::windows::core::IUnknownImpl, Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetADObjectFlags(::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UpdateAllDesktopSubscriptions<Identity: ::windows::core::IUnknownImpl, Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateAllDesktopSubscriptions().into()
        }
        unsafe extern "system" fn MakeDynamicChanges<Identity: ::windows::core::IUnknownImpl, Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poleobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MakeDynamicChanges(::core::mem::transmute(&poleobj)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ReReadWallpaper: ReReadWallpaper::<Identity, Impl, OFFSET>,
            GetADObjectFlags: GetADObjectFlags::<Identity, Impl, OFFSET>,
            UpdateAllDesktopSubscriptions: UpdateAllDesktopSubscriptions::<Identity, Impl, OFFSET>,
            MakeDynamicChanges: MakeDynamicChanges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADesktopP2 as ::windows::core::Interface>::IID
    }
}
pub trait IActiveDesktopP_Impl: Sized {
    fn SetSafeMode(&self, dwflags: u32) -> ::windows::core::Result<()>;
    fn EnsureUpdateHTML(&self) -> ::windows::core::Result<()>;
    fn SetScheme(&self, pwszschemename: &::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetScheme(&self, pwszschemename: ::windows::core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
}
impl IActiveDesktopP_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActiveDesktopP_Impl, const OFFSET: isize>() -> IActiveDesktopP_Vtbl {
        unsafe extern "system" fn SetSafeMode<Identity: ::windows::core::IUnknownImpl, Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSafeMode(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn EnsureUpdateHTML<Identity: ::windows::core::IUnknownImpl, Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnsureUpdateHTML().into()
        }
        unsafe extern "system" fn SetScheme<Identity: ::windows::core::IUnknownImpl, Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszschemename: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScheme(::core::mem::transmute(&pwszschemename), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetScheme<Identity: ::windows::core::IUnknownImpl, Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszschemename: ::windows::core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetScheme(::core::mem::transmute_copy(&pwszschemename), ::core::mem::transmute_copy(&pdwcchbuffer), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetSafeMode: SetSafeMode::<Identity, Impl, OFFSET>,
            EnsureUpdateHTML: EnsureUpdateHTML::<Identity, Impl, OFFSET>,
            SetScheme: SetScheme::<Identity, Impl, OFFSET>,
            GetScheme: GetScheme::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveDesktopP as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBriefcaseInitiator_Impl: Sized {
    fn IsMonikerInBriefcase(&self, pmk: &::core::option::Option<super::super::System::Com::IMoniker>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IBriefcaseInitiator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBriefcaseInitiator_Impl, const OFFSET: isize>() -> IBriefcaseInitiator_Vtbl {
        unsafe extern "system" fn IsMonikerInBriefcase<Identity: ::windows::core::IUnknownImpl, Impl: IBriefcaseInitiator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsMonikerInBriefcase(::core::mem::transmute(&pmk)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IsMonikerInBriefcase: IsMonikerInBriefcase::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBriefcaseInitiator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IEmptyVolumeCache_Impl: Sized {
    fn Initialize(&self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: &::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::Result<()>;
    fn GetSpaceUsed(&self, pdwlspaceused: *mut u64, picb: &::core::option::Option<IEmptyVolumeCacheCallBack>) -> ::windows::core::Result<()>;
    fn Purge(&self, dwlspacetofree: u64, picb: &::core::option::Option<IEmptyVolumeCacheCallBack>) -> ::windows::core::Result<()>;
    fn ShowProperties(&self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Deactivate(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IEmptyVolumeCache_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>() -> IEmptyVolumeCache_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&hkregkey), ::core::mem::transmute(&pcwszvolume), ::core::mem::transmute_copy(&ppwszdisplayname), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetSpaceUsed<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlspaceused: *mut u64, picb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSpaceUsed(::core::mem::transmute_copy(&pdwlspaceused), ::core::mem::transmute(&picb)).into()
        }
        unsafe extern "system" fn Purge<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspacetofree: u64, picb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Purge(::core::mem::transmute_copy(&dwlspacetofree), ::core::mem::transmute(&picb)).into()
        }
        unsafe extern "system" fn ShowProperties<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShowProperties(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Deactivate() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetSpaceUsed: GetSpaceUsed::<Identity, Impl, OFFSET>,
            Purge: Purge::<Identity, Impl, OFFSET>,
            ShowProperties: ShowProperties::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmptyVolumeCache as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IEmptyVolumeCache2_Impl: Sized + IEmptyVolumeCache_Impl {
    fn InitializeEx(&self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: &::windows::core::PCWSTR, pcwszkeyname: &::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, ppwszbtntext: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IEmptyVolumeCache2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCache2_Impl, const OFFSET: isize>() -> IEmptyVolumeCache2_Vtbl {
        unsafe extern "system" fn InitializeEx<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCache2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows::core::PCWSTR, pcwszkeyname: ::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, ppwszbtntext: *mut ::windows::core::PWSTR, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitializeEx(::core::mem::transmute_copy(&hkregkey), ::core::mem::transmute(&pcwszvolume), ::core::mem::transmute(&pcwszkeyname), ::core::mem::transmute_copy(&ppwszdisplayname), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&ppwszbtntext), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base: IEmptyVolumeCache_Vtbl::new::<Identity, Impl, OFFSET>(), InitializeEx: InitializeEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmptyVolumeCache2 as ::windows::core::Interface>::IID || iid == &<IEmptyVolumeCache as ::windows::core::Interface>::IID
    }
}
pub trait IEmptyVolumeCacheCallBack_Impl: Sized {
    fn ScanProgress(&self, dwlspaceused: u64, dwflags: u32, pcwszstatus: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn PurgeProgress(&self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl IEmptyVolumeCacheCallBack_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>() -> IEmptyVolumeCacheCallBack_Vtbl {
        unsafe extern "system" fn ScanProgress<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspaceused: u64, dwflags: u32, pcwszstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ScanProgress(::core::mem::transmute_copy(&dwlspaceused), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcwszstatus)).into()
        }
        unsafe extern "system" fn PurgeProgress<Identity: ::windows::core::IUnknownImpl, Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PurgeProgress(::core::mem::transmute_copy(&dwlspacefreed), ::core::mem::transmute_copy(&dwlspacetofree), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcwszstatus)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ScanProgress: ScanProgress::<Identity, Impl, OFFSET>,
            PurgeProgress: PurgeProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmptyVolumeCacheCallBack as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IReconcilableObject_Impl: Sized {
    fn Reconcile(&self, pinitiator: &::core::option::Option<IReconcileInitiator>, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut ::core::option::Option<super::super::System::Com::IMoniker>, ploutindex: *mut i32, pstgnewresidues: &::core::option::Option<super::super::System::Com::StructuredStorage::IStorage>, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetProgressFeedbackMaxEstimate(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IReconcilableObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReconcilableObject_Impl, const OFFSET: isize>() -> IReconcilableObject_Vtbl {
        unsafe extern "system" fn Reconcile<Identity: ::windows::core::IUnknownImpl, Impl: IReconcilableObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiator: ::windows::core::RawPtr, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut ::windows::core::RawPtr, ploutindex: *mut i32, pstgnewresidues: ::windows::core::RawPtr, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reconcile(::core::mem::transmute(&pinitiator), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute_copy(&hwndprogressfeedback), ::core::mem::transmute_copy(&ulcinput), ::core::mem::transmute_copy(&rgpmkotherinput), ::core::mem::transmute_copy(&ploutindex), ::core::mem::transmute(&pstgnewresidues), ::core::mem::transmute_copy(&pvreserved)).into()
        }
        unsafe extern "system" fn GetProgressFeedbackMaxEstimate<Identity: ::windows::core::IUnknownImpl, Impl: IReconcilableObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulprogressmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProgressFeedbackMaxEstimate() {
                ::core::result::Result::Ok(ok__) => {
                    *pulprogressmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Reconcile: Reconcile::<Identity, Impl, OFFSET>,
            GetProgressFeedbackMaxEstimate: GetProgressFeedbackMaxEstimate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReconcilableObject as ::windows::core::Interface>::IID
    }
}
pub trait IReconcileInitiator_Impl: Sized {
    fn SetAbortCallback(&self, punkforabort: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetProgressFeedback(&self, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::Result<()>;
}
impl IReconcileInitiator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReconcileInitiator_Impl, const OFFSET: isize>() -> IReconcileInitiator_Vtbl {
        unsafe extern "system" fn SetAbortCallback<Identity: ::windows::core::IUnknownImpl, Impl: IReconcileInitiator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkforabort: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAbortCallback(::core::mem::transmute(&punkforabort)).into()
        }
        unsafe extern "system" fn SetProgressFeedback<Identity: ::windows::core::IUnknownImpl, Impl: IReconcileInitiator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProgressFeedback(::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetAbortCallback: SetAbortCallback::<Identity, Impl, OFFSET>,
            SetProgressFeedback: SetProgressFeedback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReconcileInitiator as ::windows::core::Interface>::IID
    }
}
