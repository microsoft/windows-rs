#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
pub trait IADesktopP2_Impl: Sized {
    fn ReReadWallpaper(&self) -> ::windows_core::Result<()>;
    fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> ::windows_core::Result<()>;
    fn UpdateAllDesktopSubscriptions(&self) -> ::windows_core::Result<()>;
    fn MakeDynamicChanges(&self, poleobj: ::core::option::Option<&super::super::System::Ole::IOleObject>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows_core::RuntimeName for IADesktopP2 {}
#[cfg(feature = "Win32_System_Ole")]
impl IADesktopP2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: isize>() -> IADesktopP2_Vtbl {
        unsafe extern "system" fn ReReadWallpaper<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReReadWallpaper().into()
        }
        unsafe extern "system" fn GetADObjectFlags<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32, dwmask: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetADObjectFlags(::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UpdateAllDesktopSubscriptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateAllDesktopSubscriptions().into()
        }
        unsafe extern "system" fn MakeDynamicChanges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poleobj: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MakeDynamicChanges(::windows_core::from_raw_borrowed(&poleobj)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReReadWallpaper: ReReadWallpaper::<Identity, Impl, OFFSET>,
            GetADObjectFlags: GetADObjectFlags::<Identity, Impl, OFFSET>,
            UpdateAllDesktopSubscriptions: UpdateAllDesktopSubscriptions::<Identity, Impl, OFFSET>,
            MakeDynamicChanges: MakeDynamicChanges::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IADesktopP2 as ::windows_core::ComInterface>::IID
    }
}
pub trait IActiveDesktopP_Impl: Sized {
    fn SetSafeMode(&self, dwflags: u32) -> ::windows_core::Result<()>;
    fn EnsureUpdateHTML(&self) -> ::windows_core::Result<()>;
    fn SetScheme(&self, pwszschemename: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetScheme(&self, pwszschemename: ::windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IActiveDesktopP {}
impl IActiveDesktopP_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: isize>() -> IActiveDesktopP_Vtbl {
        unsafe extern "system" fn SetSafeMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSafeMode(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn EnsureUpdateHTML<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnsureUpdateHTML().into()
        }
        unsafe extern "system" fn SetScheme<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszschemename: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScheme(::core::mem::transmute(&pwszschemename), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetScheme<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszschemename: ::windows_core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScheme(::core::mem::transmute_copy(&pwszschemename), ::core::mem::transmute_copy(&pdwcchbuffer), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSafeMode: SetSafeMode::<Identity, Impl, OFFSET>,
            EnsureUpdateHTML: EnsureUpdateHTML::<Identity, Impl, OFFSET>,
            SetScheme: SetScheme::<Identity, Impl, OFFSET>,
            GetScheme: GetScheme::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IActiveDesktopP as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IBriefcaseInitiator_Impl: Sized {
    fn IsMonikerInBriefcase(&self, pmk: ::core::option::Option<&super::super::System::Com::IMoniker>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IBriefcaseInitiator {}
#[cfg(feature = "Win32_System_Com")]
impl IBriefcaseInitiator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBriefcaseInitiator_Impl, const OFFSET: isize>() -> IBriefcaseInitiator_Vtbl {
        unsafe extern "system" fn IsMonikerInBriefcase<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBriefcaseInitiator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsMonikerInBriefcase(::windows_core::from_raw_borrowed(&pmk)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsMonikerInBriefcase: IsMonikerInBriefcase::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IBriefcaseInitiator as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IEmptyVolumeCache_Impl: Sized {
    fn Initialize(&self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: &::windows_core::PCWSTR, ppwszdisplayname: *mut ::windows_core::PWSTR, ppwszdescription: *mut ::windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows_core::Result<()>;
    fn GetSpaceUsed(&self, pdwlspaceused: *mut u64, picb: ::core::option::Option<&IEmptyVolumeCacheCallBack>) -> ::windows_core::Result<()>;
    fn Purge(&self, dwlspacetofree: u64, picb: ::core::option::Option<&IEmptyVolumeCacheCallBack>) -> ::windows_core::Result<()>;
    fn ShowProperties(&self, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn Deactivate(&self) -> ::windows_core::Result<EMPTY_VOLUME_CACHE_FLAGS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows_core::RuntimeName for IEmptyVolumeCache {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IEmptyVolumeCache_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>() -> IEmptyVolumeCache_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows_core::PCWSTR, ppwszdisplayname: *mut ::windows_core::PWSTR, ppwszdescription: *mut ::windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&hkregkey), ::core::mem::transmute(&pcwszvolume), ::core::mem::transmute_copy(&ppwszdisplayname), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetSpaceUsed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlspaceused: *mut u64, picb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSpaceUsed(::core::mem::transmute_copy(&pdwlspaceused), ::windows_core::from_raw_borrowed(&picb)).into()
        }
        unsafe extern "system" fn Purge<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspacetofree: u64, picb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Purge(::core::mem::transmute_copy(&dwlspacetofree), ::windows_core::from_raw_borrowed(&picb)).into()
        }
        unsafe extern "system" fn ShowProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowProperties(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Deactivate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetSpaceUsed: GetSpaceUsed::<Identity, Impl, OFFSET>,
            Purge: Purge::<Identity, Impl, OFFSET>,
            ShowProperties: ShowProperties::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IEmptyVolumeCache as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IEmptyVolumeCache2_Impl: Sized + IEmptyVolumeCache_Impl {
    fn InitializeEx(&self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: &::windows_core::PCWSTR, pcwszkeyname: &::windows_core::PCWSTR, ppwszdisplayname: *mut ::windows_core::PWSTR, ppwszdescription: *mut ::windows_core::PWSTR, ppwszbtntext: *mut ::windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows_core::RuntimeName for IEmptyVolumeCache2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IEmptyVolumeCache2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache2_Impl, const OFFSET: isize>() -> IEmptyVolumeCache2_Vtbl {
        unsafe extern "system" fn InitializeEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows_core::PCWSTR, pcwszkeyname: ::windows_core::PCWSTR, ppwszdisplayname: *mut ::windows_core::PWSTR, ppwszdescription: *mut ::windows_core::PWSTR, ppwszbtntext: *mut ::windows_core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeEx(::core::mem::transmute_copy(&hkregkey), ::core::mem::transmute(&pcwszvolume), ::core::mem::transmute(&pcwszkeyname), ::core::mem::transmute_copy(&ppwszdisplayname), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&ppwszbtntext), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base__: IEmptyVolumeCache_Vtbl::new::<Identity, Impl, OFFSET>(), InitializeEx: InitializeEx::<Identity, Impl, OFFSET> }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IEmptyVolumeCache2 as ::windows_core::ComInterface>::IID || *iid == <IEmptyVolumeCache as ::windows_core::ComInterface>::IID
    }
}
pub trait IEmptyVolumeCacheCallBack_Impl: Sized {
    fn ScanProgress(&self, dwlspaceused: u64, dwflags: u32, pcwszstatus: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn PurgeProgress(&self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEmptyVolumeCacheCallBack {}
impl IEmptyVolumeCacheCallBack_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>() -> IEmptyVolumeCacheCallBack_Vtbl {
        unsafe extern "system" fn ScanProgress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspaceused: u64, dwflags: u32, pcwszstatus: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScanProgress(::core::mem::transmute_copy(&dwlspaceused), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcwszstatus)).into()
        }
        unsafe extern "system" fn PurgeProgress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PurgeProgress(::core::mem::transmute_copy(&dwlspacefreed), ::core::mem::transmute_copy(&dwlspacetofree), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcwszstatus)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ScanProgress: ScanProgress::<Identity, Impl, OFFSET>,
            PurgeProgress: PurgeProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IEmptyVolumeCacheCallBack as ::windows_core::ComInterface>::IID
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IReconcilableObject_Impl: Sized {
    fn Reconcile(&self, pinitiator: ::core::option::Option<&IReconcileInitiator>, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut ::core::option::Option<super::super::System::Com::IMoniker>, ploutindex: *mut i32, pstgnewresidues: ::core::option::Option<&super::super::System::Com::StructuredStorage::IStorage>, pvreserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetProgressFeedbackMaxEstimate(&self) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::RuntimeName for IReconcilableObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IReconcilableObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReconcilableObject_Impl, const OFFSET: isize>() -> IReconcilableObject_Vtbl {
        unsafe extern "system" fn Reconcile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReconcilableObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiator: *mut ::core::ffi::c_void, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut *mut ::core::ffi::c_void, ploutindex: *mut i32, pstgnewresidues: *mut ::core::ffi::c_void, pvreserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reconcile(::windows_core::from_raw_borrowed(&pinitiator), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute_copy(&hwndprogressfeedback), ::core::mem::transmute_copy(&ulcinput), ::core::mem::transmute_copy(&rgpmkotherinput), ::core::mem::transmute_copy(&ploutindex), ::windows_core::from_raw_borrowed(&pstgnewresidues), ::core::mem::transmute_copy(&pvreserved)).into()
        }
        unsafe extern "system" fn GetProgressFeedbackMaxEstimate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReconcilableObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulprogressmax: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProgressFeedbackMaxEstimate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulprogressmax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reconcile: Reconcile::<Identity, Impl, OFFSET>,
            GetProgressFeedbackMaxEstimate: GetProgressFeedbackMaxEstimate::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IReconcilableObject as ::windows_core::ComInterface>::IID
    }
}
pub trait IReconcileInitiator_Impl: Sized {
    fn SetAbortCallback(&self, punkforabort: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetProgressFeedback(&self, ulprogress: u32, ulprogressmax: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IReconcileInitiator {}
impl IReconcileInitiator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReconcileInitiator_Impl, const OFFSET: isize>() -> IReconcileInitiator_Vtbl {
        unsafe extern "system" fn SetAbortCallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReconcileInitiator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkforabort: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAbortCallback(::windows_core::from_raw_borrowed(&punkforabort)).into()
        }
        unsafe extern "system" fn SetProgressFeedback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IReconcileInitiator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProgressFeedback(::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAbortCallback: SetAbortCallback::<Identity, Impl, OFFSET>,
            SetProgressFeedback: SetProgressFeedback::<Identity, Impl, OFFSET>,
        }
    }
    pub unsafe fn matches(iid: *const ::windows_core::GUID) -> bool {
        *iid == <IReconcileInitiator as ::windows_core::ComInterface>::IID
    }
}
